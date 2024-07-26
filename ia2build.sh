# TODO: Fix 

BUILD_DIR=${PWD}/build

mkdir $BUILD_DIR

# Generate `compile_commands.json` from the make build.
if [ ! -f compile_commands.json ]; then
    make clean
    bear -- make zpipesh
    cp compile_commands.json $BUILD_DIR/
fi

ZLIB_ROOT=`realpath .`

# TODO: Allow `--root-directory` to directly contain the source files. Currently
# the rewriter appears to expect that the root dir contains a directory that in
# turn contains the source files, and recreates this directory structure in the
# output dir. This makes things slighly weird for zlib, which puts all of its
# source files in the root dir.
#
# TODO: Allow `--root-directory` to be a relative path. At least from my testing
# it seems like it needs to be an absolute paths otherwise the rewriter gets
# confused.
ZLIB_PARENT=`realpath $ZLIB_ROOT/..`

pushd $BUILD_DIR/

# TODO: Don't assume location of IA2 build dir.
~/IA2-Phase2/build/tools/rewriter/ia2-rewriter \
    --output-prefix=wrapper \
    --root-directory=$ZLIB_PARENT \
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
