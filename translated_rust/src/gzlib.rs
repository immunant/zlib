pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__O_LARGEFILE;

pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZBUFSIZE;
pub use crate::gzguts_h::GZ_APPEND;
pub use crate::gzguts_h::GZ_NONE;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::gzguts_h::LOOK;
pub use crate::internal::__INT_MAX__;
pub use crate::limits_h::INT_MAX;
pub use crate::stdlib::fcntl;

pub use crate::stdlib::open;

pub use crate::stdlib::__O_CLOEXEC;
pub use crate::stdlib::F_GETFD;
pub use crate::stdlib::F_GETFL;
pub use crate::stdlib::F_SETFD;
pub use crate::stdlib::F_SETFL;
pub use crate::stdlib::O_APPEND;
pub use crate::stdlib::O_CLOEXEC;
pub use crate::stdlib::O_CREAT;
pub use crate::stdlib::O_EXCL;
pub use crate::stdlib::O_LARGEFILE;
pub use crate::stdlib::O_NONBLOCK;
pub use crate::stdlib::O_RDONLY;
pub use crate::stdlib::O_TRUNC;
pub use crate::stdlib::O_WRONLY;
pub use crate::stdlib::SEEK_CUR;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::off_t;

pub use crate::src::deflate::internal_state;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gzFile;
pub use crate::zlib_h::gzFile_s;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DEFAULT_COMPRESSION;
pub use crate::zlib_h::Z_DEFAULT_STRATEGY;
pub use crate::zlib_h::Z_FILTERED;
pub use crate::zlib_h::Z_FIXED;
pub use crate::zlib_h::Z_HUFFMAN_ONLY;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_RLE;

/// Reset the scalar gzip state after the caller has restored its descriptor
/// position.  Error-message ownership remains at the raw boundary.
fn gz_reset_state(state: &mut crate::gzguts_h::gz_state) {
    state.x.have = 0;
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0;
        state.past = 0;
        state.how = crate::gzguts_h::LOOK;
        state.junk = -1;
    } else {
        state.reset = 0;
    }
    state.again = 0;
    state.skip = 0;
    state.x.pos = 0;
    state.strm.avail_in = 0;
}

/// Copy a boundary C string into fallibly reserved owned storage before it is
/// retained in opaque gzip state.  In particular, do not let Rust's infallible
/// collection growth turn zlib's normal open-allocation failure into a panic.
fn gz_owned_c_string(value: &::std::ffi::CStr) -> Option<::std::ffi::CString> {
    let bytes = value.to_bytes_with_nul();
    let mut owned = Vec::new();
    owned.try_reserve_exact(bytes.len()).ok()?;
    owned.extend_from_slice(bytes);
    ::std::ffi::CString::from_vec_with_nul(owned).ok()
}

/// Check whether a gzip byte length fits the signed `int` result range used
/// by the legacy API.  The FFI adapters retain their raw buffers and error
/// reporting; this is only the shared scalar admission rule.
pub(crate) fn gz_len_fits_int(len: crate::stdlib::z_size_t) -> bool {
    len <= gz_intmax() as crate::stdlib::z_size_t
}

/// Check whether a public gzip read or write request fits the signed `int`
/// result range used by the legacy API.
pub(crate) fn gz_request_len_fits_int(len: ::core::ffi::c_uint) -> bool {
    gz_len_fits_int(len as crate::stdlib::z_size_t)
}

/// The scalar portion of gzip open-mode parsing.  The C-string traversal,
/// allocations, and descriptor setup remain at the ABI boundary.
#[derive(Clone, Copy)]
enum GzOpenMode {
    Read,
    Write,
    Append,
}

impl GzOpenMode {
    fn as_raw(self) -> ::core::ffi::c_int {
        match self {
            Self::Read => crate::gzguts_h::GZ_READ,
            Self::Write => crate::gzguts_h::GZ_WRITE,
            Self::Append => crate::gzguts_h::GZ_APPEND,
        }
    }
}

/// The scalar portion of gzip open-mode parsing.  The C-string traversal,
/// allocations, and descriptor setup remain at the ABI boundary.
#[derive(Clone, Copy)]
struct GzOpenOptions {
    mode: Option<GzOpenMode>,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
}

impl GzOpenOptions {
    fn new() -> Self {
        Self {
            mode: None,
            level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
            strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
            direct: 0,
            oflag: 0,
            exclusive: 0,
        }
    }
}

/// Everything the raw open boundary needs after it has borrowed the mode
/// C-string.  Keeping the parsed state and descriptor flags together avoids
/// having separate copies of the mode interpretation at the allocation and
/// descriptor steps.
#[derive(Clone, Copy)]
struct GzOpenPlan {
    options: GzOpenOptions,
    mode: GzOpenMode,
    descriptor_flags: ::core::ffi::c_int,
}

/// Apply one non-NUL gzip mode byte.  `None` is the legacy rejection of
/// update mode (`+`); all other unrecognized bytes are ignored.
fn gz_open_option_byte(mut options: GzOpenOptions, byte: u8) -> Option<GzOpenOptions> {
    if byte.is_ascii_digit() {
        options.level = (byte - b'0') as ::core::ffi::c_int;
        return Some(options);
    }
    match byte {
        b'r' => options.mode = Some(GzOpenMode::Read),
        b'w' => options.mode = Some(GzOpenMode::Write),
        b'a' => options.mode = Some(GzOpenMode::Append),
        b'+' => return None,
        b'e' => options.oflag |= crate::stdlib::O_CLOEXEC,
        b'x' => options.exclusive = 1,
        b'f' => options.strategy = crate::zlib_h::Z_FILTERED,
        b'h' => options.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
        b'R' => options.strategy = crate::zlib_h::Z_RLE,
        b'F' => options.strategy = crate::zlib_h::Z_FIXED,
        b'G' => options.direct = -1,
        b'N' => options.oflag |= crate::stdlib::O_NONBLOCK,
        b'T' => options.direct = 1,
        _ => {}
    }
    Some(options)
}

/// Parse the complete, NUL-free mode payload before the FFI boundary touches
/// the descriptor or opaque gzip state.  This keeps the legacy permissive
/// handling of unknown mode bytes while making the rejection and defaulting
/// rules independently testable from raw C-string traversal.
fn gz_open_options(mode: &[u8]) -> Option<GzOpenOptions> {
    let mut options = GzOpenOptions::new();
    for &byte in mode {
        options = gz_open_option_byte(options, byte)?;
    }
    gz_open_options_finalize(options)
}

/// Reject impossible mode/direct combinations and normalize the default
/// read mode to transparent-operation probing, matching zlib's open path.
fn gz_open_options_finalize(mut options: GzOpenOptions) -> Option<GzOpenOptions> {
    match options.mode? {
        GzOpenMode::Read => {
            if options.direct == 1 {
                return None;
            }
            if options.direct == 0 {
                options.direct = 1;
            }
        }
        GzOpenMode::Write | GzOpenMode::Append if options.direct == -1 => return None,
        GzOpenMode::Write | GzOpenMode::Append => {}
    }
    Some(options)
}

/// Add the access/create flags after mode parsing without touching a file
/// descriptor or opaque gzip state.
fn gz_open_descriptor_flags(options: GzOpenOptions, mode: GzOpenMode) -> ::core::ffi::c_int {
    options.oflag
        | crate::stdlib::O_LARGEFILE
        | match mode {
            GzOpenMode::Read => crate::stdlib::O_RDONLY,
            GzOpenMode::Write | GzOpenMode::Append => {
                crate::stdlib::O_WRONLY
                    | crate::stdlib::O_CREAT
                    | if options.exclusive != 0 {
                        crate::stdlib::O_EXCL
                    } else {
                        0
                    }
                    | if matches!(mode, GzOpenMode::Write) {
                        crate::stdlib::O_TRUNC
                    } else {
                        crate::stdlib::O_APPEND
                    }
            }
        }
}

/// Parse a gzip mode exactly once and derive all descriptor-facing flags
/// before the FFI boundary allocates state or adopts/opens a descriptor.
fn gz_open_plan(mode: &[u8]) -> Option<GzOpenPlan> {
    let options = gz_open_options(mode)?;
    let mode = options.mode?;
    Some(GzOpenPlan {
        descriptor_flags: gz_open_descriptor_flags(options, mode),
        options,
        mode,
    })
}

/// Extract the scalar state settings from a parsed plan before the raw open
/// boundary writes them to its newly allocated ABI state.
fn gz_open_option_values(
    plan: GzOpenPlan,
) -> (
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
) {
    (
        plan.mode.as_raw(),
        plan.options.level,
        plan.options.strategy,
        plan.options.direct,
    )
}

/// Record the result of the descriptor positioning performed while opening a
/// gzip stream.  `Append` ignores the seek result just as zlib does, whereas
/// an unseekable read descriptor starts at logical offset zero.
enum GzOpenDescriptorPosition {
    Append,
    Read(crate::stdlib::off64_t),
    Unchanged,
}

fn gz_open_apply_descriptor_position(
    state: &mut crate::gzguts_h::gz_state,
    position: GzOpenDescriptorPosition,
) {
    match position {
        GzOpenDescriptorPosition::Append => state.mode = crate::gzguts_h::GZ_WRITE,
        GzOpenDescriptorPosition::Read(start) => {
            state.start = if start == -1 { 0 } else { start };
        }
        GzOpenDescriptorPosition::Unchanged => {}
    }
}

/// The synthetic path reported by `gzdopen`.  It is kept in fixed storage so
/// constructing it cannot introduce a second allocation failure before the
/// gzip state itself is opened.
struct GzFdPath {
    bytes: [u8; 7 + 3 * ::core::mem::size_of::<::core::ffi::c_int>()],
}

impl GzFdPath {
    fn new(fd: ::core::ffi::c_int) -> Self {
        let mut bytes = [0; 7 + 3 * ::core::mem::size_of::<::core::ffi::c_int>()];
        bytes[..4].copy_from_slice(b"<fd:");

        let value = fd as i64;
        let magnitude = if value < 0 {
            bytes[4] = b'-';
            -value
        } else {
            value
        } as u64;
        let mut digits = [0; 20];
        let mut count = 0;
        let mut remaining = magnitude;
        loop {
            digits[count] = b'0' + (remaining % 10) as u8;
            count += 1;
            remaining /= 10;
            if remaining == 0 {
                break;
            }
        }

        let start = if value < 0 { 5 } else { 4 };
        for (index, digit) in digits[..count].iter().rev().enumerate() {
            bytes[start + index] = *digit;
        }
        bytes[start + count] = b'>';
        Self { bytes }
    }

    fn as_c_str(&self) -> &::std::ffi::CStr {
        ::std::ffi::CStr::from_bytes_until_nul(&self.bytes).unwrap_or_default()
    }
}

// This expands only in exported entry points.  It intentionally keeps C
// string conversion, allocation, descriptor adoption, and raw ABI writes at
// that boundary; the mode and scalar state decisions are handled above by
// pointer-free helpers.
macro_rules! gz_open_at_boundary {
    ($path:expr, $fd:expr, $mode:expr) => {{
        let path_ptr: *const ::core::ffi::c_char = $path;
        let supplied_fd: ::core::ffi::c_int = $fd;
        let mode_ptr: *const ::core::ffi::c_char = $mode;
        'open: {
            if path_ptr.is_null() || mode_ptr.is_null() {
                break 'open ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
            }

            let path = ::std::ffi::CStr::from_ptr(path_ptr);
            let mode = ::std::ffi::CStr::from_ptr(mode_ptr);
            let path = match gz_owned_c_string(path) {
                Some(path) => path,
                None => break 'open ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>(),
            };
            let state = crate::stdlib::malloc(::core::mem::size_of::<crate::gzguts_h::gz_state>())
                as crate::gzguts_h::gz_statep;
            if state.is_null() {
                break 'open ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
            }

            // Do not make a Rust reference to malloc-backed storage until it
            // has a valid value for every ABI field.
            state.write(crate::gzguts_h::gz_state {
                x: crate::zlib_h::gzFile_s {
                    have: 0,
                    next: ::core::ptr::null_mut(),
                    pos: 0,
                },
                mode: crate::gzguts_h::GZ_NONE,
                fd: -1,
                file: None,
                path,
                size: 0,
                want: crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint,
                buffers: None,
                direct: 0,
                junk: 0,
                how: crate::gzguts_h::LOOK,
                again: 0,
                start: 0,
                eof: 0,
                past: 0,
                level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
                strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
                reset: 0,
                skip: 0,
                err: crate::zlib_h::Z_OK,
                msg: None,
                strm: crate::zlib_h::z_stream_s {
                    next_in: ::core::ptr::null_mut(),
                    avail_in: 0,
                    total_in: 0,
                    next_out: ::core::ptr::null_mut(),
                    avail_out: 0,
                    total_out: 0,
                    msg: ::core::ptr::null_mut(),
                    state: ::core::ptr::null_mut(),
                    zalloc: None,
                    zfree: None,
                    opaque: ::core::ptr::null_mut(),
                    data_type: 0,
                    adler: 0,
                    reserved: 0,
                },
            });
            let state = &mut *state;

            let plan = match gz_open_plan(mode.to_bytes()) {
                Some(plan) => plan,
                None => {
                    ::core::ptr::drop_in_place(state);
                    crate::stdlib::free(
                        state as *mut crate::gzguts_h::gz_state as *mut ::core::ffi::c_void,
                    );
                    break 'open ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
                }
            };
            let (mode, level, strategy, direct) = gz_open_option_values(plan);
            state.mode = mode;
            state.level = level;
            state.strategy = strategy;
            state.direct = direct;

            if supplied_fd == -1 {
                state.fd = crate::stdlib::open(path_ptr, plan.descriptor_flags, 0o666);
            } else {
                if plan.descriptor_flags & crate::stdlib::O_NONBLOCK != 0 {
                    crate::stdlib::fcntl(
                        supplied_fd,
                        crate::stdlib::F_SETFL,
                        crate::stdlib::fcntl(supplied_fd, crate::stdlib::F_GETFL)
                            | crate::stdlib::O_NONBLOCK,
                    );
                }
                if plan.descriptor_flags & crate::stdlib::O_CLOEXEC != 0 {
                    crate::stdlib::fcntl(
                        supplied_fd,
                        crate::stdlib::F_SETFD,
                        crate::stdlib::fcntl(supplied_fd, crate::stdlib::F_GETFD)
                            | crate::stdlib::O_CLOEXEC,
                    );
                }
                state.fd = supplied_fd;
            }
            if state.fd == -1 {
                ::core::ptr::drop_in_place(state);
                crate::stdlib::free(
                    state as *mut crate::gzguts_h::gz_state as *mut ::core::ffi::c_void,
                );
                break 'open ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
            }

            let descriptor_position = if state.mode == crate::gzguts_h::GZ_APPEND {
                crate::stdlib::lseek64(state.fd, 0, crate::stdlib::SEEK_END);
                GzOpenDescriptorPosition::Append
            } else if state.mode == crate::gzguts_h::GZ_READ {
                GzOpenDescriptorPosition::Read(crate::stdlib::lseek64(
                    state.fd,
                    0,
                    crate::stdlib::SEEK_CUR,
                ) as crate::stdlib::off64_t)
            } else {
                GzOpenDescriptorPosition::Unchanged
            };
            gz_open_apply_descriptor_position(state, descriptor_position);
            gz_reset_state(state);
            state.file = Some(<::std::fs::File as ::std::os::fd::FromRawFd>::from_raw_fd(
                state.fd,
            ));
            state as *mut crate::gzguts_h::gz_state as crate::zlib_h::gzFile
        }
    }};
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open_at_boundary!(path, -1, mode)
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open_at_boundary!(path, -1, mode)
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if fd == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let path = GzFdPath::new(fd);
    gz_open_at_boundary!(path.as_c_str().as_ptr(), fd, mode)
}
fn gzbuffer_want(
    mode: ::core::ffi::c_int,
    allocated_size: ::core::ffi::c_uint,
    requested_size: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if (mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE)
        || allocated_size != 0
        || requested_size > ::core::ffi::c_uint::MAX / 2
    {
        return None;
    }
    Some(requested_size.max(8))
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    match gzbuffer_want(state.mode, state.size, size) {
        Some(want) => {
            state.want = want;
            0
        }
        None => -1,
    }
}

/// Determine whether a gzip stream can be rewound before accessing its
/// descriptor or mutating its state.
fn gzrewind_is_valid(mode: ::core::ffi::c_int, err: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_READ
        && (err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR)
}

/// Commit a rewind after the FFI boundary has restored the descriptor to the
/// saved stream start.  Keeping this state update separate makes it usable by
/// seek without routing through an exported entry point.
fn gzrewind_state(state: &mut crate::gzguts_h::gz_state) -> bool {
    if !gzrewind_is_valid(state.mode, state.err) {
        return false;
    }
    gz_reset_state(state);
    true
}

#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzrewind_is_valid(state.mode, state.err)
        || crate::stdlib::lseek64(
            state.fd,
            state.start as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_SET,
        ) == -1
        || !gzrewind_state(state)
    {
        return -1;
    }
    state.msg = None;
    state.err = crate::zlib_h::Z_OK;
    0
}
/// Validate and normalize a gzip seek request without touching the opaque
/// handle or descriptor.  The boolean records the `SEEK_CUR` side effect of
/// consuming a previously scheduled skip.
fn gzseek_offset_state(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    whence: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    offset: crate::stdlib::off64_t,
) -> Option<(crate::stdlib::off64_t, bool)> {
    if (mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE)
        || (err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR)
    {
        return None;
    }
    match whence {
        crate::stdlib::SEEK_SET => Some((offset.wrapping_sub(pos), false)),
        crate::stdlib::SEEK_CUR => {
            Some((offset.wrapping_add(if past != 0 { 0 } else { skip }), true))
        }
        _ => None,
    }
}

/// Plan the direct descriptor seek used by a transparent read stream.  This
/// deliberately leaves the descriptor operation and state mutation at the
/// boundary, where the opaque handle is already validated.
fn gzseek_copy_plan(
    mode: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    have: crate::stdlib::uInt,
    offset: crate::stdlib::off64_t,
) -> Option<(crate::stdlib::off64_t, crate::stdlib::off64_t)> {
    if mode != crate::gzguts_h::GZ_READ
        || how != crate::gzguts_h::COPY
        || pos.wrapping_add(offset) < 0
    {
        return None;
    }
    Some((
        offset.wrapping_sub(have as crate::stdlib::off64_t),
        pos.wrapping_add(offset),
    ))
}

/// Commit the scalar reset after a transparent-copy seek has succeeded.  The
/// descriptor operation and error-message ownership remain at the boundary.
fn gzseek_copy_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    next_pos: crate::stdlib::off64_t,
) {
    state.x.have = 0;
    state.eof = 0;
    state.past = 0;
    state.skip = 0;
    state.strm.avail_in = 0;
    state.x.pos = next_pos;
}

/// Determine how much already-buffered read data a seek can consume.  A
/// negative offset has already been handled by rewind before this step.
fn gzseek_read_buffer_plan(
    have: crate::stdlib::uInt,
    offset: crate::stdlib::off64_t,
) -> Option<(crate::stdlib::uInt, crate::stdlib::off64_t)> {
    if offset < 0 {
        return None;
    }
    let consume = if (::core::mem::size_of::<::core::ffi::c_int>()
        == ::core::mem::size_of::<crate::stdlib::off64_t>()
        && have > gz_intmax())
        || have as crate::stdlib::off64_t > offset
    {
        offset as crate::stdlib::uInt
    } else {
        have
    };
    Some((
        consume,
        offset.wrapping_sub(consume as crate::stdlib::off64_t),
    ))
}

/// Commit consumption of bytes already buffered by a read seek.  Advancing
/// the ABI cursor itself remains at the boundary, since it is a raw pointer.
fn gzseek_read_buffer_commit_state(
    state: &mut crate::gzguts_h::gz_state,
    consume: crate::stdlib::uInt,
) -> bool {
    if consume > state.x.have {
        return false;
    }
    state.x.have = state.x.have.wrapping_sub(consume);
    state.x.pos = state.x.pos.wrapping_add(consume as crate::stdlib::off64_t);
    true
}

/// Normalize a negative seek through the read-stream rewind path.  The
/// descriptor rewind itself remains at the boundary; this only validates the
/// mode and computes the post-rewind logical offset with zlib's signed
/// wrapping arithmetic.
fn gzseek_rewind_offset_state(
    mode: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    offset: crate::stdlib::off64_t,
) -> Option<crate::stdlib::off64_t> {
    if mode != crate::gzguts_h::GZ_READ {
        return None;
    }
    let offset = offset.wrapping_add(pos);
    (offset >= 0).then_some(offset)
}

/// Record a pending seek once all descriptor and buffered-output work has
/// completed.  The return value deliberately uses the legacy wrapping
/// signed arithmetic for the reported logical position.
fn gzseek_schedule_state(
    state: &mut crate::gzguts_h::gz_state,
    offset: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    state.skip = offset;
    state.x.pos.wrapping_add(offset)
}

/// A descriptor operation selected entirely from gzip scalar state.  Both
/// variants are issued by the exported seek boundary, keeping the selection
/// safe while retaining exactly one raw descriptor call there.
#[derive(Clone, Copy)]
enum GzSeekDescriptorAction {
    Copy {
        offset: crate::stdlib::off64_t,
        next_pos: crate::stdlib::off64_t,
    },
    Rewind {
        start: crate::stdlib::off64_t,
    },
}

/// The boundary work required after the safe seek state machine has selected
/// a descriptor action.  `advance` is deliberately a byte count rather than
/// an adjusted pointer: the exported wrapper alone advances the public ABI
/// cursor after validating the opaque handle.
#[derive(Clone, Copy)]
struct GzSeekResult {
    position: crate::stdlib::off64_t,
    advance: crate::stdlib::uInt,
    clear_error: bool,
}

#[derive(Clone, Copy)]
struct GzSeekPending {
    action: GzSeekDescriptorAction,
    offset: crate::stdlib::off64_t,
}

/// Complete the logical seek after any direct descriptor operation has
/// succeeded.  This updates only scalar ABI-mirror fields; pointer movement
/// remains in the exported wrapper.
fn gzseek_finish(
    state: &mut crate::gzguts_h::gz_state,
    mut offset: crate::stdlib::off64_t,
    clear_error: bool,
) -> Option<GzSeekResult> {
    let mut advance = 0;
    if state.mode == crate::gzguts_h::GZ_READ {
        let (consume, remaining_offset) = gzseek_read_buffer_plan(state.x.have, offset)?;
        if !gzseek_read_buffer_commit_state(state, consume) {
            return None;
        }
        advance = consume;
        offset = remaining_offset;
    }
    Some(GzSeekResult {
        position: gzseek_schedule_state(state, offset),
        advance,
        clear_error,
    })
}

/// Apply the scalar changes following a successful direct descriptor seek.
/// The caller performs the descriptor call at the FFI boundary and then uses
/// this safe helper to commit the corresponding gzip state transition.
fn gzseek_after_descriptor(
    state: &mut crate::gzguts_h::gz_state,
    action: GzSeekDescriptorAction,
    offset: crate::stdlib::off64_t,
) -> Option<GzSeekResult> {
    match action {
        GzSeekDescriptorAction::Copy { next_pos, .. } => {
            gzseek_copy_commit_state(state, next_pos);
            Some(GzSeekResult {
                position: next_pos,
                advance: 0,
                clear_error: true,
            })
        }
        GzSeekDescriptorAction::Rewind { .. } => {
            if !gzrewind_state(state) {
                return None;
            }
            gzseek_finish(state, offset, true)
        }
    }
}

/// Validate and prepare a gzip seek without dereferencing its opaque handle,
/// moving its ABI cursor, or invoking the descriptor.  The returned action,
/// if any, is issued only by the exported boundary adapter.
fn gzseek64(
    state: &mut crate::gzguts_h::gz_state,
    mut offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
) -> Option<Result<GzSeekResult, GzSeekPending>> {
    let Some((normalized_offset, clear_skip)) = gzseek_offset_state(
        state.mode,
        state.err,
        whence,
        state.x.pos,
        state.past,
        state.skip,
        offset,
    ) else {
        return None;
    };
    offset = normalized_offset;
    if clear_skip {
        state.skip = 0;
    }
    let descriptor_action = if let Some((descriptor_offset, next_pos)) =
        gzseek_copy_plan(state.mode, state.how, state.x.pos, state.x.have, offset)
    {
        Some(GzSeekDescriptorAction::Copy {
            offset: descriptor_offset,
            next_pos,
        })
    } else if offset < 0 {
        let Some(rewind_offset) = gzseek_rewind_offset_state(state.mode, state.x.pos, offset)
        else {
            return None;
        };
        offset = rewind_offset;
        Some(GzSeekDescriptorAction::Rewind { start: state.start })
    } else {
        None
    };
    match descriptor_action {
        Some(action) => Some(Err(GzSeekPending { action, offset })),
        None => gzseek_finish(state, offset, false).map(Ok),
    }
}

// This expands only in the exported seek entry points.  Pointer conversion,
// cursor adjustment, descriptor access, and raw error storage consequently
// remain at the boundary while `gzseek64` owns the safe state machine.
macro_rules! gzseek_at_boundary {
    ($file:expr, $offset:expr, $whence:expr) => {{
        let file = $file;
        if file.is_null() {
            -1
        } else {
            let state = &mut *(file as crate::gzguts_h::gz_statep);
            let result = match gzseek64(state, $offset, $whence) {
                Some(Ok(result)) => result,
                Some(Err(pending)) => {
                    let (descriptor_offset, descriptor_whence) = match pending.action {
                        GzSeekDescriptorAction::Copy { offset, .. } => {
                            (offset, crate::stdlib::SEEK_CUR)
                        }
                        GzSeekDescriptorAction::Rewind { start } => {
                            (start, crate::stdlib::SEEK_SET)
                        }
                    };
                    if crate::stdlib::lseek64(
                        state.fd,
                        descriptor_offset as crate::stdlib::__off64_t,
                        descriptor_whence,
                    ) == -1
                    {
                        return -1;
                    }
                    match gzseek_after_descriptor(state, pending.action, pending.offset) {
                        Some(result) => result,
                        None => return -1,
                    }
                }
                None => return -1,
            };
            if result.advance != 0 {
                state.x.next = state.x.next.wrapping_add(result.advance as usize);
            }
            if result.clear_error {
                state.msg = None;
                state.err = crate::zlib_h::Z_OK;
            }
            result.position
        }
    }};
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    gzseek_at_boundary!(file, offset, whence)
}
#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let ret = gzseek_at_boundary!(file, offset, whence);
    if ret == ret {
        ret
    } else {
        -1
    }
}
/// Return the logical gzip position once the boundary has validated the
/// opaque handle.  A read stream with `past` set deliberately ignores a
/// previously scheduled skip, matching zlib's EOF-position semantics.
fn gztell64_state(
    mode: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> Option<crate::stdlib::off64_t> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    // zlib's position accounting uses the platform signed offset type.  Keep
    // that legacy wrapping behavior explicit instead of allowing a debug
    // build overflow panic to change an otherwise valid query result.
    Some(pos.wrapping_add(if past != 0 { 0 } else { skip }))
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    gztell64_state(state.mode, state.x.pos, state.past, state.skip).unwrap_or(-1)
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    if file.is_null() {
        return -1;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    match gztell64_state(state.mode, state.x.pos, state.past, state.skip) {
        Some(ret) if ret == ret => ret,
        _ => -1,
    }
}
/// Adjust the descriptor position to the gzip stream position after the
/// boundary has sampled the opaque state and queried the descriptor.
fn gzoffset_mode_is_valid(mode: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_READ || mode == crate::gzguts_h::GZ_WRITE
}

fn gzoffset_state(
    mode: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    descriptor_offset: crate::stdlib::off64_t,
) -> Option<crate::stdlib::off64_t> {
    if !gzoffset_mode_is_valid(mode) || descriptor_offset == -1 {
        return None;
    }
    Some(if mode == crate::gzguts_h::GZ_READ {
        // `avail_in` was already consumed from the descriptor position by
        // the inflater.  Mirror zlib's offset arithmetic without making this
        // read-only query panic for extreme values.
        descriptor_offset.wrapping_sub(avail_in as crate::stdlib::off64_t)
    } else {
        descriptor_offset
    })
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gzoffset_mode_is_valid(state.mode) {
        return -1;
    }
    let descriptor_offset = crate::stdlib::lseek64(state.fd, 0, crate::stdlib::SEEK_CUR);
    gzoffset_state(state.mode, state.strm.avail_in, descriptor_offset).unwrap_or(-1)
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    if file.is_null() {
        return -1;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gzoffset_mode_is_valid(state.mode) {
        return -1;
    }
    let descriptor_offset = crate::stdlib::lseek64(state.fd, 0, crate::stdlib::SEEK_CUR);
    gzoffset_state(state.mode, state.strm.avail_in, descriptor_offset).unwrap_or(-1)
}
fn gzeof_state(mode: ::core::ffi::c_int, past: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if mode == crate::gzguts_h::GZ_READ {
        past
    } else {
        0
    }
}
#[export_name = "gzeof"]

pub unsafe extern "C" fn gzeof_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return 0;
    }
    gzeof_state(state.mode, state.past)
}
fn gzerror_fixed_message(bytes: &'static [u8]) -> &'static ::std::ffi::CStr {
    ::std::ffi::CStr::from_bytes_with_nul(bytes).unwrap_or_default()
}

fn gzerror_state<'a>(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    message: Option<&'a ::std::ffi::CStr>,
) -> Option<(::core::ffi::c_int, &'a ::std::ffi::CStr)> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    if err == crate::zlib_h::Z_MEM_ERROR {
        return Some((err, gzerror_fixed_message(b"out of memory\0")));
    }
    Some((err, message.unwrap_or_else(|| gzerror_fixed_message(b"\0"))))
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if file.is_null() {
        return ::core::ptr::null();
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    let message = state.msg.as_deref();
    let Some((err, message)) = gzerror_state(state.mode, state.err, message) else {
        return ::core::ptr::null();
    };
    if !errnum.is_null() {
        *errnum = err;
    }
    message.as_ptr()
}
/// Clear the read-side EOF markers after the boundary has validated the
/// opaque gzip state.  The caller remains responsible for releasing any
/// boundary-owned error message.
fn gzclearerr(state: &mut crate::gzguts_h::gz_state) -> bool {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return false;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0;
        state.past = 0;
    }
    true
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    if file.is_null() {
        return;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if gzclearerr(state) {
        state.msg = None;
        state.err = crate::zlib_h::Z_OK;
    }
}

/// The pointer-free portion of gzip error replacement.  This is deliberately
/// separate from the ABI state: the caller owns the old C allocation, and the
/// exported boundary is the only place that can release it or install the new
/// C pointer.
#[derive(Clone, Copy)]
struct GzErrorState {
    available: ::core::ffi::c_uint,
    again: ::core::ffi::c_int,
}

struct GzErrorUpdate {
    available: ::core::ffi::c_uint,
    err: ::core::ffi::c_int,
    message: Option<::std::ffi::CString>,
}

/// Construct the legacy `"path: message"` text before the raw error-storage
/// boundary allocates or writes it.  Overflow and allocation failure are
/// reported to that boundary as an out-of-memory condition rather than
/// allowing a wrapped C allocation size.
fn gz_error_message(
    path: &::std::ffi::CStr,
    message: &::std::ffi::CStr,
) -> Option<::std::ffi::CString> {
    let path = path.to_bytes();
    let message = message.to_bytes();
    let capacity = path
        .len()
        .checked_add(2)?
        .checked_add(message.len())?
        .checked_add(1)?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(capacity).ok()?;
    bytes.extend_from_slice(path);
    bytes.extend_from_slice(b": ");
    bytes.extend_from_slice(message);
    bytes.push(0);
    ::std::ffi::CString::from_vec_with_nul(bytes).ok()
}

/// Compute an error replacement without accessing ABI pointers or making a
/// foreign allocation.  In particular, message-construction failure is
/// normalized to `Z_MEM_ERROR` before the boundary mutates the C mirror.
fn gz_error_update(
    state: GzErrorState,
    err: ::core::ffi::c_int,
    path: Option<&::std::ffi::CStr>,
    message: Option<&::std::ffi::CStr>,
) -> GzErrorUpdate {
    let available =
        if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && state.again == 0 {
            0
        } else {
            state.available
        };
    let compose_message = err != crate::zlib_h::Z_MEM_ERROR && message.is_some();
    let message = if compose_message {
        path.zip(message)
            .and_then(|(path, message)| gz_error_message(path, message))
    } else {
        None
    };
    let err = if compose_message && message.is_none() {
        crate::zlib_h::Z_MEM_ERROR
    } else {
        err
    };
    GzErrorUpdate {
        available,
        err,
        message,
    }
}

/// Apply a gzip error to owned opaque state.  Boundary adapters provide an
/// already-validated message view, while this core preserves the historical
/// `x.have` and retryable-error rules without touching C storage.
pub(crate) fn gz_error_update_state(
    state: &mut crate::gzguts_h::gz_state,
    err: ::core::ffi::c_int,
    message: Option<&::std::ffi::CStr>,
) {
    let prior = GzErrorState {
        available: state.x.have,
        again: state.again,
    };
    let path = if message.is_some() {
        Some(state.path.as_c_str())
    } else {
        None
    };
    let update = gz_error_update(prior, err, path, message);

    state.msg = None;
    state.x.have = update.available;
    state.err = update.err;
    state.msg = update.message;
}

/// Clear a prior gzip error without converting an ABI string pointer.  This is
/// the common operation at the start of a public read or write request.
pub(crate) fn gz_error_clear(state: &mut crate::gzguts_h::gz_state) {
    gz_error_update_state(state, crate::zlib_h::Z_OK, None);
}

/// Store one of zlib's fixed diagnostic strings using the owned state path.
/// Descriptor and codec diagnostics that arrive as foreign pointers still
/// need a boundary conversion before they can use this core.
pub(crate) fn gz_error_static(
    state: &mut crate::gzguts_h::gz_state,
    err: ::core::ffi::c_int,
    message: &'static [u8],
) {
    match ::std::ffi::CStr::from_bytes_with_nul(message) {
        Ok(message) => gz_error_update_state(state, err, Some(message)),
        Err(_) => gz_error_update_state(state, crate::zlib_h::Z_MEM_ERROR, None),
    }
}

/// Record a descriptor failure without converting a foreign `strerror()`
/// pointer.  Rust's OS error text has the same platform source; remove its
/// added numeric suffix so `gzerror()` keeps the C-facing diagnostic form.
pub(crate) fn gz_error_io(state: &mut crate::gzguts_h::gz_state, code: ::core::ffi::c_int) {
    let message = ::std::io::Error::from_raw_os_error(code).to_string();
    let message = message
        .split_once(" (os error ")
        .map(|(message, _)| message)
        .unwrap_or(&message);
    match ::std::ffi::CString::new(message) {
        Ok(message) => gz_error_update_state(state, crate::zlib_h::Z_ERRNO, Some(&message)),
        Err(_) => gz_error_update_state(state, crate::zlib_h::Z_MEM_ERROR, None),
    }
}

#[export_name = "gz_error"]

pub unsafe extern "C" fn gz_error_ffi(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    if !state.is_null() {
        let state = &mut *state;
        let message = if msg.is_null() {
            None
        } else {
            Some(::std::ffi::CStr::from_ptr(msg))
        };
        gz_error_update_state(state, err, message);
    }
}
pub fn gz_intmax() -> ::core::ffi::c_uint {
    crate::limits_h::INT_MAX as ::core::ffi::c_uint
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
