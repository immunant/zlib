# Get the path to is IA2-Phase2 checkout. Currently assumed to be next to the
# zlib checkout.
# 
# TODO: Don't assume IA2 is located next to the zlib checkout.
IA2_PATH=`realpath ../IA2-Phase2`

./configure

# Inject IA2 compiler flags for the initial (IA2-disabled) build.
#
# TODO: Specify PKEY in a less hacky way. This approach will define `PKEY=2` for
# all modules, including zpipesh which should instead have `PKEY=1`. We probably
# need to customize the Makefile futher to more directly support IA2 flags.
IA2FLAGS="\
    -I $IA2_PATH/runtime/libia2/include \
    -I $IA2_PATH/runtime/partition-alloc/include \
    -DPKEY=2"
sed < Makefile "
/^IA2FLAGS *=/s#=.*#=$IA2FLAGS#
" > Makefile.tmp && mv Makefile.tmp Makefile


# Generate `compile_commands.json` from the make build.
if [ ! -f compile_commands.json ]; then
    make clean
    bear -- make zpipesh
else
    echo "compile_commands.json already exists, skipping initial build"
fi

ZLIB_ROOT=`realpath .`
BUILD_DIR=${PWD}/ia2

# Run the rewriter if the build dir is not already present.
if [ ! -d "$BUILD_DIR" ]; then
    mkdir -p $BUILD_DIR
    pushd $BUILD_DIR
    # TODO: Allow `--root-directory` to be a relative path. At least from my testing
    # it seems like it needs to be an absolute paths otherwise the rewriter gets
    # confused.
    #
    # TODO: Why do we need to specify the source files manually? Isn't that info
    # in `compile_commands.json`?
    $IA2_PATH/build/tools/rewriter/ia2-rewriter \
        --output-prefix=wrapper \
        --root-directory=$ZLIB_ROOT \
        --output-directory=$BUILD_DIR \
        $ZLIB_ROOT/examples/zpipe.c \
        $ZLIB_ROOT/adler32.c \
        $ZLIB_ROOT/crc32.c \
        $ZLIB_ROOT/deflate.c \
        $ZLIB_ROOT/infback.c \
        $ZLIB_ROOT/inffast.c \
        $ZLIB_ROOT/inflate.c \
        $ZLIB_ROOT/inftrees.c \
        $ZLIB_ROOT/trees.c \
        $ZLIB_ROOT/zutil.c \
        $ZLIB_ROOT/compress.c \
        $ZLIB_ROOT/uncompr.c \
        $ZLIB_ROOT/gzclose.c \
        $ZLIB_ROOT/gzlib.c \
        $ZLIB_ROOT/gzread.c \
        $ZLIB_ROOT/gzwrite.c \
        $ZLIB_ROOT/adler32.c \
        $ZLIB_ROOT/crc32.c \
        $ZLIB_ROOT/deflate.c \
        $ZLIB_ROOT/infback.c \
        $ZLIB_ROOT/inffast.c \
        $ZLIB_ROOT/inflate.c \
        $ZLIB_ROOT/inftrees.c \
        $ZLIB_ROOT/trees.c \
        $ZLIB_ROOT/zutil.c \
        $ZLIB_ROOT/compress.c \
        $ZLIB_ROOT/uncompr.c \
        $ZLIB_ROOT/gzclose.c \
        $ZLIB_ROOT/gzlib.c \
        $ZLIB_ROOT/gzread.c \
        $ZLIB_ROOT/gzwrite.c
    popd
else
    echo "Build dir ($BUILD_DIR) already exists, skipping rewriter"
fi

# Copy additional files needed by the build.
cp -t $BUILD_DIR Makefile

pushd $BUILD_DIR

# Re-run `configure` with additional flags for compiling the compartment.
#
# TODO: Currently we're hard coding the `LIBIA2_X86_64` define. We should probably
# detecing the current platform to determine what define should be set. This
# might need to be done in `configure`.
#
# -Werror=incompatible-pointer-types \
IA2FLAGS="$IA2FLAGS \
    -fPIC \
    -DIA2_ENABLE=1 \
    -DLIBIA2_X86_64=1 \
    -include wrapper.h \
    -Wl,--wrap=pthread_create \
    -pthread \
    -Wl,-z,now \
    -Wl,-z,relro \
    -Wl,-T$IA2_PATH/runtime/libia2/padding.ld \
    -Wl,@wrapper_2.ld"

IA2MAINFLAGS="\
    -Wl,--wrap=main \
    -Wl,--wrap=calloc  \
    -Wl,--wrap=malloc  \
    -Wl,--wrap=realloc \
    -Wl,--wrap=free \
    -Wl,--dynamic-list=$IA2_PATH/runtime/libia2/dynsym.syms \
    -Wl,--export-dynamic \
    -Wl,@wrapper_1.ld"

sed < Makefile "
/^IA2FLAGS *=/s#=.*#=$IA2FLAGS#
/^IA2MAINFLAGS *=/s#=.*#=$IA2MAINFLAGS#
/^IA2PATH *=/s#=.*#=$IA2_PATH#
" > Makefile.tmp && mv Makefile.tmp Makefile

# Copy libc.so and run pad-tls on it.
#
# TODO: Don't assume path to `libc.so`. I'm not sure how to properly detect that.
cp /lib/x86_64-linux-gnu/libc.so.6 .
$IA2_PATH/build/tools/pad-tls/pad-tls libc.so.6

popd

# Manual rewriting notes
#
# - Function pointer calls where the pointer was being dereferenced before the
#   call (i.e. `(*fn_ptr)(args)`) fail to compile once the pointer has been
#   replaced with a struct. I had to manually remove the extra dereference
#   operation.
# - `IA2_ADDR` for null checks is incorrect and needed to be manually rewritten
#   to check against 0.
# - Assignments to fn ptr fields break, e.g. `foo.field = 0;` where `field` is a
#   fn ptr.
# - Removed `zlib.map` version script from the build in order to make `zcalloc`
#   and `zcfree` publicly exported in order to fix linker errors publicly
#   exported in order to fix linker errors.
# - Currently no way to mark a function pointer type that shouldn't be rewritten
#   (e.g. a function pointer like `deflate_fast` that is entirely internal and
#   will never cross compartments). We have `IA2_BEGIN_NO_WRAP` but currently
#   that only applies to direct function calls that should not be rewritten.
#   We'll need to update the rewriter to correctly handle this case. For zlib I
#   manually edited the rewritten file to remove the rewritten pointer type.
# - Need to change `zpipe.c` to allocate `strm` on the shared heap since IA2
#   can't handle shared memory on the stack.
#
# Build notes
#
# - Linking in `liblibia2.a` is not mentioned in the usage docs.
# - Need to also set `LIBIA2_X86_64=1` via CLI flag.
# - Rewriter-generated wrapper for `static` fn is broken, we get a linker error
#   in e.g. `deflate_fast` when using version symbols (defined in `zlib.map`).
# - Rewriter doesn't need to wrap fn pointers if they never escape the
#   compartment, but can't detect that at compile time. Would be useful to have
#   a configuration option to say "don't tranlate this fn pointer". Maybe
#   `IA2_IGNORE` is sufficient for that?
# - `zlib.map` is marking `zcfree` and `zcalloc` local, which effectively makes
#   them `static` as well, but we can't see that in the source code. This leads
#   to linker errors when the generated wrappers in `libcallgates.so` try to
#   link against the symbols in `zlib.so`, and can't find them because they're
#   internal.
# - Need to include partion-alloc and wrap `malloc` and friends. Not documented
#   anywhere.
# - Need to run `pad-tls` on `libc.so` and `libz.so` since they use TLS. Not
#   documented anywhere.
