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

#[derive(Copy, Clone)]
struct GzOpenMode {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
}

fn gz_parse_open_mode(mode: &[u8]) -> Option<GzOpenMode> {
    let mut parsed = GzOpenMode {
        mode: crate::gzguts_h::GZ_NONE,
        level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
        strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
        direct: 0 as ::core::ffi::c_int,
        oflag: 0 as ::core::ffi::c_int,
        exclusive: 0 as ::core::ffi::c_int,
    };

    for &byte in mode {
        if byte >= b'0' && byte <= b'9' {
            parsed.level = byte as ::core::ffi::c_int - b'0' as ::core::ffi::c_int;
        } else {
            match byte {
                b'r' => parsed.mode = crate::gzguts_h::GZ_READ,
                b'w' => parsed.mode = crate::gzguts_h::GZ_WRITE,
                b'a' => parsed.mode = crate::gzguts_h::GZ_APPEND,
                b'+' => return None,
                b'e' => parsed.oflag |= crate::stdlib::O_CLOEXEC,
                b'x' => parsed.exclusive = 1 as ::core::ffi::c_int,
                b'f' => parsed.strategy = crate::zlib_h::Z_FILTERED,
                b'h' => parsed.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
                b'R' => parsed.strategy = crate::zlib_h::Z_RLE,
                b'F' => parsed.strategy = crate::zlib_h::Z_FIXED,
                b'G' => parsed.direct = -1 as ::core::ffi::c_int,
                b'N' => parsed.oflag |= crate::stdlib::O_NONBLOCK,
                b'T' => parsed.direct = 1 as ::core::ffi::c_int,
                b'b' | _ => {}
            }
        }
    }

    if parsed.mode == crate::gzguts_h::GZ_NONE {
        return None;
    }
    if parsed.mode == crate::gzguts_h::GZ_READ {
        if parsed.direct == 1 as ::core::ffi::c_int {
            return None;
        }
        if parsed.direct == 0 as ::core::ffi::c_int {
            parsed.direct = 1 as ::core::ffi::c_int;
        }
    } else if parsed.direct == -1 as ::core::ffi::c_int {
        return None;
    }

    Some(parsed)
}

fn gz_open_oflag_for_mode(
    mut oflag: ::core::ffi::c_int,
    mode: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    oflag |= crate::stdlib::O_LARGEFILE
        | if mode == crate::gzguts_h::GZ_READ {
            crate::stdlib::O_RDONLY
        } else {
            crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | if exclusive != 0 {
                    crate::stdlib::O_EXCL
                } else {
                    0 as ::core::ffi::c_int
                }
                | if mode == crate::gzguts_h::GZ_WRITE {
                    crate::stdlib::O_TRUNC
                } else {
                    crate::stdlib::O_APPEND
                }
        };
    oflag
}

fn gz_reset_before_error(state: &mut crate::gzguts_h::gz_state) {
    state.x.have = 0 as ::core::ffi::c_uint;
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0 as ::core::ffi::c_int;
        state.past = 0 as ::core::ffi::c_int;
        state.how = crate::gzguts_h::LOOK;
        state.junk = -1 as ::core::ffi::c_int;
    } else {
        state.reset = 0 as ::core::ffi::c_int;
    }
    state.again = 0 as ::core::ffi::c_int;
    state.skip = 0 as crate::stdlib::off64_t;
}

fn gz_reset_after_error(state: &mut crate::gzguts_h::gz_state) {
    state.x.pos = 0 as crate::stdlib::off64_t;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
}

unsafe fn gz_open(
    mut path: *const ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
    parsed_mode: GzOpenMode,
) -> crate::zlib_h::gzFile {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut len: crate::stdlib::z_size_t = 0;
    let mut oflag: ::core::ffi::c_int;
    let exclusive: ::core::ffi::c_int;
    if path.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state = crate::stdlib::malloc(
        ::core::mem::size_of::<crate::gzguts_h::gz_state>() as crate::__stddef_size_t_h::size_t
    ) as crate::gzguts_h::gz_statep;
    if state.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    (*state).size = 0 as ::core::ffi::c_uint;
    (*state).want = crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint;
    (*state).err = crate::zlib_h::Z_OK;
    (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).mode = crate::gzguts_h::GZ_NONE;
    (*state).level = crate::zlib_h::Z_DEFAULT_COMPRESSION;
    (*state).strategy = crate::zlib_h::Z_DEFAULT_STRATEGY;
    (*state).direct = 0 as ::core::ffi::c_int;
    (*state).mode = parsed_mode.mode;
    (*state).level = parsed_mode.level;
    (*state).strategy = parsed_mode.strategy;
    (*state).direct = parsed_mode.direct;
    oflag = parsed_mode.oflag;
    exclusive = parsed_mode.exclusive;
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
    oflag = gz_open_oflag_for_mode(oflag, (*state).mode, exclusive);
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
    {
        let state_ref = &mut *state;
        if state_ref.fd == -1 as ::core::ffi::c_int {
            crate::stdlib::free(state_ref.path as *mut ::core::ffi::c_void);
            crate::stdlib::free(state as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        }
        if state_ref.mode == crate::gzguts_h::GZ_APPEND {
            crate::stdlib::lseek64(
                state_ref.fd,
                0 as crate::stdlib::__off64_t,
                crate::stdlib::SEEK_END,
            );
            state_ref.mode = crate::gzguts_h::GZ_WRITE;
        }
        if state_ref.mode == crate::gzguts_h::GZ_READ {
            state_ref.start = crate::stdlib::lseek64(
                state_ref.fd,
                0 as crate::stdlib::__off64_t,
                crate::stdlib::SEEK_CUR,
            ) as crate::stdlib::off64_t;
            if state_ref.start == -1 as ::core::ffi::c_int as crate::stdlib::off64_t {
                state_ref.start = 0 as crate::stdlib::off64_t;
            }
        }
        gz_reset_before_error(state_ref);
    }
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    {
        let state_ref = &mut *state;
        gz_reset_after_error(state_ref);
    }
    return state as crate::zlib_h::gzFile;
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let Some(parsed_mode) = gz_parse_open_mode(::core::ffi::CStr::from_ptr(mode).to_bytes()) else {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    };
    return gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        parsed_mode,
    );
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let Some(parsed_mode) = gz_parse_open_mode(::core::ffi::CStr::from_ptr(mode).to_bytes()) else {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    };
    return gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        parsed_mode,
    );
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gz: crate::zlib_h::gzFile = ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    if fd == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    if mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let Some(parsed_mode) = gz_parse_open_mode(::core::ffi::CStr::from_ptr(mode).to_bytes()) else {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    };
    if {
        path = crate::stdlib::malloc(
            (7 as crate::__stddef_size_t_h::size_t).wrapping_add(
                (3 as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                        as crate::__stddef_size_t_h::size_t),
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
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()
                    as crate::__stddef_size_t_h::size_t),
        ),
        b"<fd:%d>\0".as_ptr() as *const ::core::ffi::c_char,
        fd,
    );
    gz = gz_open(path as *const ::core::ffi::c_void, fd, parsed_mode);
    crate::stdlib::free(path as *mut ::core::ffi::c_void);
    return gz;
}
fn gz_state_open(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_READ || state.mode == crate::gzguts_h::GZ_WRITE
}

fn gzbuffer_normalized_size(size: crate::stdlib::uInt) -> Option<crate::stdlib::uInt> {
    if (size << 1 as ::core::ffi::c_int) < size {
        None
    } else if size < 8 as crate::stdlib::uInt {
        Some(8 as crate::stdlib::uInt)
    } else {
        Some(size)
    }
}

pub fn gzbuffer(
    state: &mut crate::gzguts_h::gz_state,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if !gz_state_open(state) {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    let Some(normalized_size) = gzbuffer_normalized_size(size) else {
        return -1 as ::core::ffi::c_int;
    };
    size = normalized_size;
    state.want = size;
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzbuffer(&mut *(file as crate::gzguts_h::gz_statep), size)
}
pub fn gz_clamped_uint(
    value: crate::stdlib::uInt,
    limit: crate::stdlib::off64_t,
) -> crate::stdlib::uInt {
    if ::core::mem::size_of::<::core::ffi::c_int>() as usize
        == ::core::mem::size_of::<crate::stdlib::off64_t>() as usize
        && value > gz_intmax()
        || value as crate::stdlib::off64_t > limit
    {
        limit as crate::stdlib::uInt
    } else {
        value
    }
}

pub fn gz_consume_buffered_read(
    have: &mut crate::stdlib::uInt,
    pos: &mut crate::stdlib::off64_t,
    amount: &mut crate::stdlib::off64_t,
) -> crate::stdlib::uInt {
    let n = gz_clamped_uint(*have, *amount);
    *have = (*have).wrapping_sub(n);
    *pos += n as crate::stdlib::off64_t;
    *amount -= n as crate::stdlib::off64_t;
    n
}

pub fn gz_consume_buffered_read_cursor(
    state: &mut crate::gzguts_h::gz_state,
    amount: &mut crate::stdlib::off64_t,
) -> crate::stdlib::uInt {
    let n = gz_consume_buffered_read(&mut state.x.have, &mut state.x.pos, amount);
    state.x.next = state.x.next.wrapping_add(n as usize);
    n
}

pub fn gz_seek64_normalize(
    mut offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> (crate::stdlib::off64_t, bool) {
    if whence == crate::stdlib::SEEK_SET {
        offset -= pos;
        (offset, false)
    } else {
        offset += if past != 0 {
            0 as crate::stdlib::off64_t
        } else {
            skip
        };
        (offset, true)
    }
}

pub fn gz_io_chunk_limit() -> ::core::ffi::c_uint {
    (-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint)
}

pub fn gz_io_chunk_len(len: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let max = gz_io_chunk_limit();
    if len > max {
        max
    } else {
        len
    }
}

pub fn gz_uInt_fits_int(len: crate::stdlib::uInt) -> bool {
    (len as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int
}

pub fn gz_z_size_to_uInt_chunk(len: crate::stdlib::z_size_t) -> ::core::ffi::c_uint {
    let max = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    if max as crate::stdlib::z_size_t > len {
        len as ::core::ffi::c_uint
    } else {
        max
    }
}

pub(crate) fn gz_file_request_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    size.checked_mul(nitems)
}

pub fn gz_errno_is_retryable(errno: ::core::ffi::c_int) -> bool {
    errno == crate::stdlib::EAGAIN || errno == crate::stdlib::EWOULDBLOCK
}

fn gz_rewind_state_ready(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_READ
        && (state.err == crate::zlib_h::Z_OK || state.err == crate::zlib_h::Z_BUF_ERROR)
}

fn gz_seek64_state_ready(state: &crate::gzguts_h::gz_state, whence: ::core::ffi::c_int) -> bool {
    (state.mode == crate::gzguts_h::GZ_READ || state.mode == crate::gzguts_h::GZ_WRITE)
        && (state.err == crate::zlib_h::Z_OK || state.err == crate::zlib_h::Z_BUF_ERROR)
        && (whence == crate::stdlib::SEEK_SET || whence == crate::stdlib::SEEK_CUR)
}

fn gz_seek64_can_seek_copy(
    state: &crate::gzguts_h::gz_state,
    offset: crate::stdlib::off64_t,
) -> bool {
    state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::COPY
        && state.x.pos + offset >= 0 as crate::stdlib::off64_t
}

fn gz_seek64_prepare_copy_seek(state: &mut crate::gzguts_h::gz_state) {
    state.x.have = 0 as ::core::ffi::c_uint;
    state.eof = 0 as ::core::ffi::c_int;
    state.past = 0 as ::core::ffi::c_int;
    state.skip = 0 as crate::stdlib::off64_t;
}

fn gz_seek64_finish_copy_seek(
    state: &mut crate::gzguts_h::gz_state,
    offset: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    state.strm.avail_in = 0 as crate::stdlib::uInt;
    state.x.pos += offset;
    state.x.pos
}

#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state_ref = &mut *state;
    if !gz_rewind_state_ready(state_ref) {
        return -1 as ::core::ffi::c_int;
    }
    if crate::stdlib::lseek64(
        state_ref.fd,
        state_ref.start as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_SET,
    ) == -1 as ::core::ffi::c_int as crate::stdlib::__off64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    gz_reset_before_error(state_ref);
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    gz_reset_after_error(state_ref);
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    let mut ret: crate::stdlib::off64_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state_ref = &mut *state;
    if !gz_seek64_state_ready(state_ref, whence) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    let clear_skip = {
        let normalized = gz_seek64_normalize(
            offset,
            whence,
            state_ref.x.pos,
            state_ref.past,
            state_ref.skip,
        );
        offset = normalized.0;
        normalized.1
    };
    if clear_skip {
        state_ref.skip = 0 as crate::stdlib::off64_t;
    }
    if gz_seek64_can_seek_copy(state_ref, offset) {
        ret = crate::stdlib::lseek64(
            state_ref.fd,
            offset as crate::stdlib::__off64_t - state_ref.x.have as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t;
        if ret == -1 as ::core::ffi::c_int as crate::stdlib::off64_t {
            return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
        }
        gz_seek64_prepare_copy_seek(state_ref);
        gz_error(
            state,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return gz_seek64_finish_copy_seek(&mut *state, offset);
    }
    if offset < 0 as crate::stdlib::off64_t {
        if state_ref.mode != crate::gzguts_h::GZ_READ {
            return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
        }
        offset += state_ref.x.pos;
        if offset < 0 as crate::stdlib::off64_t {
            return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
        }
        if gzrewind_ffi(file) == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
        }
    }
    let state_ref = &mut *(file as crate::gzguts_h::gz_statep);
    if state_ref.mode == crate::gzguts_h::GZ_READ {
        gz_consume_buffered_read_cursor(state_ref, &mut offset);
    }
    state_ref.skip = offset;
    return state_ref.x.pos + offset;
}
#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzseek64_ffi(file, offset, whence);
    return if ret == ret {
        ret
    } else {
        -1 as ::core::ffi::c_int as crate::stdlib::off_t
    };
}
pub fn gztell64(state: &crate::gzguts_h::gz_state) -> crate::stdlib::off64_t {
    return state.x.pos
        + (if state.past != 0 {
            0 as crate::stdlib::off64_t
        } else {
            state.skip
        });
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gz_state_open(state) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    gztell64(state)
}
pub fn gztell(state: &crate::gzguts_h::gz_state) -> crate::stdlib::off_t {
    return gztell64(state) as crate::stdlib::off_t;
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off_t;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gz_state_open(state) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off_t;
    }
    gztell(state)
}
pub fn gzoffset64(
    state: &crate::gzguts_h::gz_state,
    mut offset: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    if state.mode == crate::gzguts_h::GZ_READ {
        offset -= state.strm.avail_in as crate::stdlib::off64_t;
    }
    return offset;
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gz_state_open(state) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    let offset = crate::stdlib::lseek64(
        state.fd,
        0 as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_CUR,
    ) as crate::stdlib::off64_t;
    if offset == -1 as ::core::ffi::c_int as crate::stdlib::off64_t {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    gzoffset64(state, offset)
}
pub fn gzoffset(
    state: &crate::gzguts_h::gz_state,
    offset: crate::stdlib::off64_t,
) -> crate::stdlib::off_t {
    let ret = gzoffset64(state, offset);
    return if ret == ret {
        ret
    } else {
        -1 as ::core::ffi::c_int as crate::stdlib::off_t
    };
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off_t;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gz_state_open(state) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off_t;
    }
    let offset = crate::stdlib::lseek64(
        state.fd,
        0 as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_CUR,
    ) as crate::stdlib::off64_t;
    if offset == -1 as ::core::ffi::c_int as crate::stdlib::off64_t {
        return -1 as ::core::ffi::c_int as crate::stdlib::off_t;
    }
    gzoffset(state, offset)
}
pub fn gzeof(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    return if state.mode == crate::gzguts_h::GZ_READ {
        state.past
    } else {
        0 as ::core::ffi::c_int
    };
}
#[export_name = "gzeof"]

pub unsafe extern "C" fn gzeof_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gz_state_open(state) {
        return 0 as ::core::ffi::c_int;
    }
    gzeof(state)
}
enum GzErrorMessage {
    OutOfMemory,
    Empty,
    StateMessage,
}

fn gzerror(state: &crate::gzguts_h::gz_state) -> (::core::ffi::c_int, GzErrorMessage) {
    let message = if state.err == crate::zlib_h::Z_MEM_ERROR {
        GzErrorMessage::OutOfMemory
    } else if state.msg.is_null() {
        GzErrorMessage::Empty
    } else {
        GzErrorMessage::StateMessage
    };
    (state.err, message)
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if file.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    if !gz_state_open(state) {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let (err, message) = gzerror(state);
    if !errnum.is_null() {
        *errnum = err;
    }
    match message {
        GzErrorMessage::OutOfMemory => b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        GzErrorMessage::Empty => b"\0".as_ptr() as *const ::core::ffi::c_char,
        GzErrorMessage::StateMessage => state.msg as *const ::core::ffi::c_char,
    }
}
pub fn gzclearerr(state: &mut crate::gzguts_h::gz_state) {
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0 as ::core::ffi::c_int;
        state.past = 0 as ::core::ffi::c_int;
    }
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    if file.is_null() {
        return;
    }
    let state_ptr = file as crate::gzguts_h::gz_statep;
    let state = &mut *state_ptr;
    if !gz_state_open(state) {
        return;
    }
    gzclearerr(state);
    gz_error(
        state_ptr,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
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
    if gz_error_should_clear_buffer(err, (*state).again) {
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

fn gz_error_should_clear_buffer(err: ::core::ffi::c_int, again: ::core::ffi::c_int) -> bool {
    err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && again == 0
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
    return crate::limits_h::INT_MAX as ::core::ffi::c_uint;
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
