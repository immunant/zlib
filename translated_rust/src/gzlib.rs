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
struct GzOpenOptions {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
}

impl GzOpenOptions {
    fn new() -> Self {
        Self {
            mode: crate::gzguts_h::GZ_NONE,
            level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
            strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
            direct: 0,
            oflag: 0,
            exclusive: 0,
        }
    }
}

/// Apply one non-NUL gzip mode byte.  `None` is the legacy rejection of
/// update mode (`+`); all other unrecognized bytes are ignored.
fn gz_open_option_byte(mut options: GzOpenOptions, byte: u8) -> Option<GzOpenOptions> {
    if byte.is_ascii_digit() {
        options.level = (byte - b'0') as ::core::ffi::c_int;
        return Some(options);
    }
    match byte {
        b'r' => options.mode = crate::gzguts_h::GZ_READ,
        b'w' => options.mode = crate::gzguts_h::GZ_WRITE,
        b'a' => options.mode = crate::gzguts_h::GZ_APPEND,
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

/// Reject impossible mode/direct combinations and normalize the default
/// read mode to transparent-operation probing, matching zlib's open path.
fn gz_open_options_finalize(mut options: GzOpenOptions) -> Option<GzOpenOptions> {
    if options.mode == crate::gzguts_h::GZ_NONE {
        return None;
    }
    if options.mode == crate::gzguts_h::GZ_READ {
        if options.direct == 1 {
            return None;
        }
        if options.direct == 0 {
            options.direct = 1;
        }
    } else if options.direct == -1 {
        return None;
    }
    Some(options)
}

/// Add the access/create flags after mode parsing without touching a file
/// descriptor or opaque gzip state.
fn gz_open_descriptor_flags(options: GzOpenOptions) -> ::core::ffi::c_int {
    options.oflag
        | crate::stdlib::O_LARGEFILE
        | if options.mode == crate::gzguts_h::GZ_READ {
            crate::stdlib::O_RDONLY
        } else {
            crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | if options.exclusive != 0 {
                    crate::stdlib::O_EXCL
                } else {
                    0
                }
                | if options.mode == crate::gzguts_h::GZ_WRITE {
                    crate::stdlib::O_TRUNC
                } else {
                    crate::stdlib::O_APPEND
                }
        }
}

unsafe extern "C" fn gz_open(
    mut path: *const ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut len: crate::stdlib::z_size_t = 0;
    let mut options = GzOpenOptions::new();
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state = crate::stdlib::malloc(::core::mem::size_of::<crate::gzguts_h::gz_state>())
        as crate::gzguts_h::gz_statep;
    if state.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    (*state).size = 0 as ::core::ffi::c_uint;
    (*state).want = crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint;
    (*state).err = crate::zlib_h::Z_OK;
    (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while *mode != 0 {
        options = match gz_open_option_byte(options, *mode as u8) {
            Some(options) => options,
            None => {
                crate::stdlib::free(state as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
            }
        };
        mode = mode.offset(1);
    }
    options = match gz_open_options_finalize(options) {
        Some(options) => options,
        None => {
            crate::stdlib::free(state as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        }
    };
    (*state).mode = options.mode;
    (*state).level = options.level;
    (*state).strategy = options.strategy;
    (*state).direct = options.direct;
    len = crate::stdlib::strlen(path as *const ::core::ffi::c_char) as crate::stdlib::z_size_t;
    (*state).path = crate::stdlib::malloc(
        (len as crate::__stddef_size_t_h::size_t)
            .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
    ) as *mut ::core::ffi::c_char;
    if (*state).path.is_null() {
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    crate::stdlib::snprintf(
        (*state).path,
        (len as crate::__stddef_size_t_h::size_t)
            .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        path as *const ::core::ffi::c_char,
    );
    let oflag = gz_open_descriptor_flags(options);
    if fd == -1 as ::core::ffi::c_int {
        (*state).fd = crate::stdlib::open(
            path as *const ::core::ffi::c_char,
            oflag,
            0o666 as ::core::ffi::c_int,
        );
    } else {
        if oflag & crate::stdlib::O_NONBLOCK != 0 {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFL,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFL) | crate::stdlib::O_NONBLOCK,
            );
        }
        if oflag & crate::stdlib::O_CLOEXEC != 0 {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFD,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFD) | crate::stdlib::O_CLOEXEC,
            );
        }
        (*state).fd = fd;
    }
    if (*state).fd == -1 as ::core::ffi::c_int {
        crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let state = &mut *state;
    if state.mode == crate::gzguts_h::GZ_APPEND {
        crate::stdlib::lseek64(
            state.fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_END,
        );
        state.mode = crate::gzguts_h::GZ_WRITE;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.start = crate::stdlib::lseek64(
            state.fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t;
        if state.start == -1 as crate::stdlib::off64_t {
            state.start = 0 as crate::stdlib::off64_t;
        }
    }
    gz_reset_state(state);
    gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return state as *mut crate::gzguts_h::gz_state as crate::zlib_h::gzFile;
}
pub unsafe extern "C" fn gzopen(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    return gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    );
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzopen(path, mode)
}
pub unsafe extern "C" fn gzopen64(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    return gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    );
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzopen64(path, mode)
}
pub unsafe extern "C" fn gzdopen(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gz: crate::zlib_h::gzFile = ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    if fd == -1 as ::core::ffi::c_int || {
        path = crate::stdlib::malloc(
            (7 as crate::__stddef_size_t_h::size_t).wrapping_add(
                (3 as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
            ),
        ) as *mut ::core::ffi::c_char;
        path.is_null()
    } {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    crate::stdlib::snprintf(
        path,
        (7 as crate::__stddef_size_t_h::size_t).wrapping_add(
            (3 as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
        ),
        b"<fd:%d>\0".as_ptr() as *const ::core::ffi::c_char,
        fd,
    );
    gz = gz_open(path as *const ::core::ffi::c_void, fd, mode);
    crate::stdlib::free(path as *mut ::core::ffi::c_void);
    return gz;
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzdopen(fd, mode)
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

pub unsafe extern "C" fn gzrewind(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gzrewind_is_valid(state.mode, state.err) {
        return -1 as ::core::ffi::c_int;
    }
    if crate::stdlib::lseek64(
        state.fd,
        state.start as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_SET,
    ) == -1 as crate::stdlib::__off64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    gz_reset_state(state);
    gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzrewind(file)
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

pub unsafe extern "C" fn gzseek64(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    let mut ret: crate::stdlib::off64_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state_ref = &mut *state;
    let Some((normalized_offset, clear_skip)) = gzseek_offset_state(
        state_ref.mode,
        state_ref.err,
        whence,
        state_ref.x.pos,
        state_ref.past,
        state_ref.skip,
        offset,
    ) else {
        return -1 as crate::stdlib::off64_t;
    };
    offset = normalized_offset;
    if clear_skip {
        (*state).skip = 0 as crate::stdlib::off64_t;
    }
    if let Some((descriptor_offset, next_pos)) = gzseek_copy_plan(
        (*state).mode,
        (*state).how,
        (*state).x.pos,
        (*state).x.have,
        offset,
    ) {
        ret = crate::stdlib::lseek64(
            (*state).fd,
            descriptor_offset as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t;
        if ret == -1 as crate::stdlib::off64_t {
            return -1 as crate::stdlib::off64_t;
        }
        gzseek_copy_commit_state(state_ref, next_pos);
        gz_error(
            state,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return next_pos;
    }
    if offset < 0 as crate::stdlib::off64_t {
        let Some(rewind_offset) = gzseek_rewind_offset_state((*state).mode, (*state).x.pos, offset)
        else {
            return -1 as crate::stdlib::off64_t;
        };
        offset = rewind_offset;
        if gzrewind(file) == -1 as ::core::ffi::c_int {
            return -1 as crate::stdlib::off64_t;
        }
    }
    if (*state).mode == crate::gzguts_h::GZ_READ {
        let Some((consume, remaining_offset)) = gzseek_read_buffer_plan((*state).x.have, offset)
        else {
            return -1 as crate::stdlib::off64_t;
        };
        (*state).x.next = (*state).x.next.offset(consume as isize);
        if !gzseek_read_buffer_commit_state(state_ref, consume) {
            return -1 as crate::stdlib::off64_t;
        }
        offset = remaining_offset;
    }
    gzseek_schedule_state(state_ref, offset)
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    gzseek64(file, offset, whence)
}
pub unsafe extern "C" fn gzseek(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzseek64(file, offset, whence);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    gzseek(file, offset, whence)
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
fn gzerror_state<'a>(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    message: Option<&'a ::std::ffi::CStr>,
) -> Option<(::core::ffi::c_int, &'a ::std::ffi::CStr)> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    if err == crate::zlib_h::Z_MEM_ERROR {
        return Some((
            err,
            ::std::ffi::CStr::from_bytes_with_nul(b"out of memory\0")
                .expect("static message is NUL-terminated"),
        ));
    }
    Some((
        err,
        message.unwrap_or_else(|| {
            ::std::ffi::CStr::from_bytes_with_nul(b"\0").expect("static message is NUL-terminated")
        }),
    ))
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
    let message = if state.msg.is_null() {
        None
    } else {
        Some(::std::ffi::CStr::from_ptr(state.msg))
    };
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
        gz_error(
            state,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
}
pub unsafe extern "C" fn gz_error(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    if !(*state).msg.is_null() {
        if (*state).err != crate::zlib_h::Z_MEM_ERROR {
            crate::stdlib::free((*state).msg as *mut ::core::ffi::c_void);
        }
        (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && (*state).again == 0 {
        (*state).x.have = 0 as ::core::ffi::c_uint;
    }
    (*state).err = err;
    if msg.is_null() {
        return;
    }
    if err == crate::zlib_h::Z_MEM_ERROR {
        return;
    }
    (*state).msg = crate::stdlib::malloc(
        crate::stdlib::strlen((*state).path)
            .wrapping_add(crate::stdlib::strlen(msg))
            .wrapping_add(3 as crate::__stddef_size_t_h::size_t),
    ) as *mut ::core::ffi::c_char;
    if (*state).msg.is_null() {
        (*state).err = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    crate::stdlib::snprintf(
        (*state).msg,
        crate::stdlib::strlen((*state).path)
            .wrapping_add(crate::stdlib::strlen(msg))
            .wrapping_add(3 as crate::__stddef_size_t_h::size_t),
        b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*state).path,
        b": \0".as_ptr() as *const ::core::ffi::c_char,
        msg,
    );
}
#[export_name = "gz_error"]

pub unsafe extern "C" fn gz_error_ffi(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    gz_error(state, err, msg)
}
pub fn gz_intmax() -> ::core::ffi::c_uint {
    crate::limits_h::INT_MAX as ::core::ffi::c_uint
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
