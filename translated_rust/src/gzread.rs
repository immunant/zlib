pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
pub use crate::src::gzlib::gz_error;
pub use crate::src::gzlib::gz_intmax;

pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
pub use crate::src::inflate::inflateEnd;
pub use crate::src::inflate::inflateInit2_;
pub use crate::src::inflate::inflateReset;

pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidp;
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
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_ERRNO;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;

fn gz_load_core(
    state: &mut crate::gzguts_h::gz_state,
    have: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
    read: Result<::core::ffi::c_uint, ::core::ffi::c_int>,
) -> (::core::ffi::c_uint, bool, Option<::core::ffi::c_int>) {
    match read {
        Ok(0) => {
            state.eof = 1 as ::core::ffi::c_int;
            (have, false, None)
        }
        Ok(got) => {
            let have = have.wrapping_add(got);
            (have, have < len, None)
        }
        Err(errno) => {
            if errno == crate::stdlib::EAGAIN || errno == crate::stdlib::EWOULDBLOCK {
                state.again = 1 as ::core::ffi::c_int;
                if have != 0 {
                    return (have, false, None);
                }
            }
            (have, false, Some(errno))
        }
    }
}

fn gz_load_read_len(
    len: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    max: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let get = len.wrapping_sub(have);
    if get > max {
        max
    } else {
        get
    }
}

fn gz_fread_request_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    size.checked_mul(nitems)
}

fn gzread_request_fits_int(len: ::core::ffi::c_uint) -> bool {
    (len as ::core::ffi::c_int) >= 0
}

fn gz_read_error_is_recoverable(err: ::core::ffi::c_int, again: ::core::ffi::c_int) -> bool {
    err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR || again != 0
}

enum GzreadOutcome {
    Read(::core::ffi::c_int),
    Error,
    Again,
}

fn gzread_outcome(
    len: ::core::ffi::c_uint,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> GzreadOutcome {
    if len != 0 {
        GzreadOutcome::Read(len as ::core::ffi::c_int)
    } else if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR {
        GzreadOutcome::Error
    } else if again != 0 {
        GzreadOutcome::Again
    } else {
        GzreadOutcome::Read(0)
    }
}

fn gz_fread_items_read(
    size: crate::stdlib::z_size_t,
    bytes_read: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if size == 0 {
        0
    } else {
        bytes_read / size
    }
}

fn gz_read_chunk_len(
    len: crate::stdlib::z_size_t,
    buffered: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut chunk = ::core::ffi::c_uint::MAX;
    if chunk as crate::stdlib::z_size_t > len {
        chunk = len as ::core::ffi::c_uint;
    }
    if buffered != 0 && buffered < chunk {
        chunk = buffered;
    }
    chunk
}

fn gz_read_needs_fetch(
    how: ::core::ffi::c_int,
    chunk_len: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> bool {
    how == crate::gzguts_h::LOOK || chunk_len < size << 1 as ::core::ffi::c_int
}

enum GzReadAction {
    DrainBuffered,
    StopAtEof,
    Fetch,
    Load,
    Decompress,
}

fn gz_read_action(
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    how: ::core::ffi::c_int,
    chunk_len: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> GzReadAction {
    if have != 0 {
        GzReadAction::DrainBuffered
    } else if eof != 0 && avail_in == 0 {
        GzReadAction::StopAtEof
    } else if gz_read_needs_fetch(how, chunk_len, size) {
        GzReadAction::Fetch
    } else if how == crate::gzguts_h::COPY {
        GzReadAction::Load
    } else {
        GzReadAction::Decompress
    }
}

fn gz_read_progress(
    len: crate::stdlib::z_size_t,
    got: crate::stdlib::z_size_t,
    pos: crate::stdlib::off64_t,
    chunk_len: ::core::ffi::c_uint,
) -> (
    crate::stdlib::z_size_t,
    crate::stdlib::z_size_t,
    crate::stdlib::off64_t,
) {
    (
        len.wrapping_sub(chunk_len as crate::stdlib::z_size_t),
        got.wrapping_add(chunk_len as crate::stdlib::z_size_t),
        pos + chunk_len as crate::stdlib::off64_t,
    )
}

enum GzUngetcBufferState {
    Empty,
    Full,
    Pushable,
}

fn gz_ungetc_buffer_state(
    have: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> GzUngetcBufferState {
    if have == 0 {
        GzUngetcBufferState::Empty
    } else if have == size << 1 as ::core::ffi::c_int {
        GzUngetcBufferState::Full
    } else {
        GzUngetcBufferState::Pushable
    }
}

unsafe extern "C" fn gz_load(
    state: crate::gzguts_h::gz_statep,
    buf: *mut ::core::ffi::c_uchar,
    len: ::core::ffi::c_uint,
    have: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let max = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    (*state).again = 0 as ::core::ffi::c_int;
    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
    *have = 0 as ::core::ffi::c_uint;
    loop {
        let get = gz_load_read_len(len, *have, max);
        let ret = crate::stdlib::read(
            (*state).fd,
            buf.offset(*have as isize) as *mut ::core::ffi::c_void,
            get as crate::__stddef_size_t_h::size_t,
        ) as ::core::ffi::c_int;
        let read = if ret < 0 {
            Err(*crate::stdlib::__errno_location())
        } else {
            Ok(ret as ::core::ffi::c_uint)
        };
        let (got, more, error) = gz_load_core(&mut *state, *have, len, read);
        *have = got;
        if let Some(errno) = error {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_ERRNO,
                crate::stdlib::strerror(errno),
            );
            return -1 as ::core::ffi::c_int;
        }
        if !more {
            return 0 as ::core::ffi::c_int;
        }
    }
}

unsafe extern "C" fn gz_avail(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).err != crate::zlib_h::Z_OK && (*state).err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).eof == 0 as ::core::ffi::c_int {
        if (*strm).avail_in != 0 {
            let mut p: *mut ::core::ffi::c_uchar = (*state).in_0;
            let mut q: *const ::core::ffi::c_uchar = (*strm).next_in;
            if q != p as *const ::core::ffi::c_uchar {
                let mut n: ::core::ffi::c_uint = (*strm).avail_in as ::core::ffi::c_uint;
                loop {
                    let c2rust_fresh0 = q;
                    q = q.offset(1);
                    let c2rust_fresh1 = p;
                    p = p.offset(1);
                    *c2rust_fresh1 = *c2rust_fresh0;
                    n = n.wrapping_sub(1);
                    if !(n != 0) {
                        break;
                    }
                }
            }
        }
        if gz_load(
            state,
            (*state).in_0.offset((*strm).avail_in as isize),
            (*state)
                .size
                .wrapping_sub((*strm).avail_in as ::core::ffi::c_uint),
            &raw mut got,
        ) == -1 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        (*strm).avail_in = (*strm).avail_in.wrapping_add(got);
        (*strm).next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_is_gzip_header(
    first: ::core::ffi::c_uchar,
    second: ::core::ffi::c_uchar,
    third: ::core::ffi::c_uchar,
    fourth: ::core::ffi::c_uchar,
) -> bool {
    first == 31 && second == 139 && third == 8 && fourth < 32
}

fn gz_look_needs_more_input(avail_in: crate::stdlib::uInt, again: ::core::ffi::c_int) -> bool {
    avail_in == 0 || again != 0 && avail_in < 4
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GzLookAction {
    NeedMoreInput,
    Gzip,
    TransparentCopy,
}

fn gz_look_action(
    avail_in: crate::stdlib::uInt,
    again: ::core::ffi::c_int,
    header: Option<[::core::ffi::c_uchar; 4]>,
) -> GzLookAction {
    if gz_look_needs_more_input(avail_in, again) {
        GzLookAction::NeedMoreInput
    } else if header.is_some_and(|[first, second, third, fourth]| {
        gz_is_gzip_header(first, second, third, fourth)
    }) {
        GzLookAction::Gzip
    } else {
        GzLookAction::TransparentCopy
    }
}

unsafe extern "C" fn gz_look(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint {
        (*state).in_0 = crate::stdlib::malloc((*state).want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        (*state).out = crate::stdlib::malloc(
            ((*state).want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
        ) as *mut ::core::ffi::c_uchar;
        if (*state).in_0.is_null() || (*state).out.is_null() {
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*state).size = (*state).want;
        (*state).strm.zalloc = None;
        (*state).strm.zfree = None;
        (*state).strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*state).strm.avail_in = 0 as crate::stdlib::uInt;
        (*state).strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2_(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            (*state).size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if (*state).direct == -1 as ::core::ffi::c_int || (*state).junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).how = crate::gzguts_h::GZIP;
        (*state).junk = ((*state).junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        (*state).direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let header = if (*strm).avail_in > 3 as crate::stdlib::uInt {
        Some([
            *(*strm).next_in.offset(0 as ::core::ffi::c_int as isize),
            *(*strm).next_in.offset(1 as ::core::ffi::c_int as isize),
            *(*strm).next_in.offset(2 as ::core::ffi::c_int as isize),
            *(*strm).next_in.offset(3 as ::core::ffi::c_int as isize),
        ])
    } else {
        None
    };
    match gz_look_action((*strm).avail_in, (*state).again, header) {
        GzLookAction::NeedMoreInput => return 0 as ::core::ffi::c_int,
        GzLookAction::Gzip => {
            crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
            (*state).how = crate::gzguts_h::GZIP;
            (*state).junk = 1 as ::core::ffi::c_int;
            (*state).direct = 0 as ::core::ffi::c_int;
            return 0 as ::core::ffi::c_int;
        }
        GzLookAction::TransparentCopy => {}
    }
    (*state).x.next = (*state).out;
    crate::stdlib::memcpy(
        (*state).x.next as *mut ::core::ffi::c_void,
        (*strm).next_in as *const ::core::ffi::c_void,
        (*strm).avail_in as crate::__stddef_size_t_h::size_t,
    );
    (*state).x.have = (*strm).avail_in as ::core::ffi::c_uint;
    (*strm).avail_in = 0 as crate::stdlib::uInt;
    (*state).how = crate::gzguts_h::COPY;
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_decomp(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut had: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    had = (*strm).avail_out as ::core::ffi::c_uint;
    loop {
        if (*strm).avail_in == 0 as crate::stdlib::uInt
            && gz_avail(state) == -1 as ::core::ffi::c_int
        {
            ret = (*state).err;
            break;
        } else if (*strm).avail_in == 0 as crate::stdlib::uInt {
            if (*state).again == 0 {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_BUF_ERROR,
                    b"unexpected end of file\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            break;
        } else {
            ret = crate::src::inflate::inflate(
                strm as *mut crate::zlib_h::z_stream_s,
                crate::zlib_h::Z_NO_FLUSH,
            );
            if (*strm).avail_out < had {
                (*state).junk = 0 as ::core::ffi::c_int;
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break;
            } else if ret == crate::zlib_h::Z_DATA_ERROR {
                if (*state).junk == 1 as ::core::ffi::c_int {
                    (*strm).avail_in = 0 as crate::stdlib::uInt;
                    (*state).eof = 1 as ::core::ffi::c_int;
                    (*state).how = crate::gzguts_h::LOOK;
                    ret = crate::zlib_h::Z_OK;
                    break;
                } else {
                    crate::src::gzlib::gz_error(
                        state as *mut crate::gzguts_h::gz_state,
                        crate::zlib_h::Z_DATA_ERROR,
                        if (*strm).msg.is_null() {
                            b"compressed data error\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            (*strm).msg as *const ::core::ffi::c_char
                        },
                    );
                    break;
                }
            } else if !((*strm).avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END) {
                break;
            }
        }
    }
    (*state).x.have =
        (had as crate::stdlib::uInt).wrapping_sub((*strm).avail_out) as ::core::ffi::c_uint;
    (*state).x.next =
        (*strm).next_out.offset(-((*state).x.have as isize)) as *mut ::core::ffi::c_uchar;
    if ret == crate::zlib_h::Z_STREAM_END {
        (*state).junk = 0 as ::core::ffi::c_int;
        (*state).how = crate::gzguts_h::LOOK;
        return 0 as ::core::ffi::c_int;
    }
    return if ret != crate::zlib_h::Z_OK {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}

unsafe extern "C" fn gz_fetch(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    loop {
        match (*state).how {
            crate::gzguts_h::LOOK => {
                if gz_look(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if (*state).how == crate::gzguts_h::LOOK {
                    return 0 as ::core::ffi::c_int;
                }
            }
            crate::gzguts_h::COPY => {
                if gz_load(
                    state,
                    (*state).out,
                    (*state).size << 1 as ::core::ffi::c_int,
                    &raw mut (*state).x.have,
                ) == -1 as ::core::ffi::c_int
                {
                    return -1 as ::core::ffi::c_int;
                }
                (*state).x.next = (*state).out;
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                (*strm).avail_out =
                    ((*state).size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                (*strm).next_out = (*state).out as *mut crate::stdlib::Bytef;
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"state corrupt\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if !((*state).x.have == 0 as ::core::ffi::c_uint
            && ((*state).eof == 0 || (*strm).avail_in != 0))
        {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_skip_core(
    have: &mut ::core::ffi::c_uint,
    pos: &mut crate::stdlib::off64_t,
    skip: &mut crate::stdlib::off64_t,
    intmax: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let n = if ::core::mem::size_of::<::core::ffi::c_int>() as usize
        == ::core::mem::size_of::<crate::stdlib::off64_t>() as usize
        && *have > intmax
        || *have as crate::stdlib::off64_t > *skip
    {
        *skip as ::core::ffi::c_uint
    } else {
        *have
    };
    *have = have.wrapping_sub(n);
    *pos += n as crate::stdlib::off64_t;
    *skip -= n as crate::stdlib::off64_t;
    n
}

fn gzgets_copy_len(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    newline_offset: Option<usize>,
) -> ::core::ffi::c_uint {
    let limit = if have > left { left } else { have };
    match newline_offset {
        Some(offset) if offset < limit as usize => (offset as ::core::ffi::c_uint).wrapping_add(1),
        _ => limit,
    }
}

fn gzclose_r_result(
    stream_err: ::core::ffi::c_int,
    close_ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let err = if stream_err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    if close_ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    }
}

unsafe extern "C" fn gz_skip(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    loop {
        if (*state).x.have != 0 {
            let n = gz_skip_core(
                &mut (*state).x.have,
                &mut (*state).x.pos,
                &mut (*state).skip,
                crate::src::gzlib::gz_intmax(),
            );
            (*state).x.next = (*state).x.next.offset(n as isize);
        } else {
            if (*state).eof != 0 && (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if !((*state).skip != 0) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gz_load_read_len_uses_remaining_bytes_below_cap() {
        assert_eq!(gz_load_read_len(10, 4, 8), 6);
    }

    #[test]
    fn gz_load_read_len_caps_large_requests() {
        assert_eq!(gz_load_read_len(20, 4, 8), 8);
        assert_eq!(gz_load_read_len(12, 4, 8), 8);
    }

    #[test]
    fn gz_load_read_len_preserves_unsigned_wrapping_before_capping() {
        assert_eq!(gz_load_read_len(0, 1, 8), 8);
    }

    #[test]
    fn gz_fread_request_len_handles_zero_operands() {
        assert_eq!(gz_fread_request_len(0, 5), Some(0));
        assert_eq!(gz_fread_request_len(5, 0), Some(0));
    }

    #[test]
    fn gz_fread_request_len_accepts_representable_products() {
        assert_eq!(
            gz_fread_request_len(crate::stdlib::z_size_t::MAX, 1),
            Some(crate::stdlib::z_size_t::MAX)
        );
        assert_eq!(gz_fread_request_len(4, 7), Some(28));
    }

    #[test]
    fn gz_fread_request_len_rejects_overflow() {
        assert_eq!(gz_fread_request_len(crate::stdlib::z_size_t::MAX, 2), None);
    }

    #[test]
    fn gzread_request_fits_int_checks_signed_int_boundary() {
        let largest_valid = ::core::ffi::c_int::MAX as ::core::ffi::c_uint;

        assert!(gzread_request_fits_int(largest_valid));
        assert!(!gzread_request_fits_int(largest_valid.wrapping_add(1)));
    }

    #[test]
    fn gz_read_error_is_recoverable_accepts_ok_and_buffer_errors() {
        assert!(gz_read_error_is_recoverable(crate::zlib_h::Z_OK, 0));
        assert!(gz_read_error_is_recoverable(crate::zlib_h::Z_BUF_ERROR, 0));
    }

    #[test]
    fn gz_read_error_is_recoverable_accepts_retryable_errors() {
        assert!(gz_read_error_is_recoverable(crate::zlib_h::Z_DATA_ERROR, 1));
    }

    #[test]
    fn gz_read_error_is_recoverable_rejects_non_retryable_errors() {
        assert!(!gz_read_error_is_recoverable(
            crate::zlib_h::Z_DATA_ERROR,
            0
        ));
    }

    #[test]
    fn gzread_outcome_returns_nonzero_reads_even_with_state_flags() {
        assert!(matches!(
            gzread_outcome(5, crate::zlib_h::Z_DATA_ERROR, 1),
            GzreadOutcome::Read(5)
        ));
    }

    #[test]
    fn gzread_outcome_classifies_empty_reads() {
        assert!(matches!(
            gzread_outcome(0, crate::zlib_h::Z_DATA_ERROR, 0),
            GzreadOutcome::Error
        ));
        assert!(matches!(
            gzread_outcome(0, crate::zlib_h::Z_BUF_ERROR, 1),
            GzreadOutcome::Again
        ));
        assert!(matches!(
            gzread_outcome(0, crate::zlib_h::Z_OK, 0),
            GzreadOutcome::Read(0)
        ));
    }

    #[test]
    fn gz_fread_items_read_counts_only_complete_items() {
        assert_eq!(gz_fread_items_read(4, 11), 2);
        assert_eq!(gz_fread_items_read(4, 12), 3);
    }

    #[test]
    fn gz_fread_items_read_handles_zero_item_size() {
        assert_eq!(gz_fread_items_read(0, 12), 0);
    }

    #[test]
    fn gz_read_chunk_len_limits_requests_to_remaining_length() {
        assert_eq!(gz_read_chunk_len(17, 0), 17);
    }

    #[test]
    fn gz_read_chunk_len_limits_requests_to_buffered_data() {
        assert_eq!(gz_read_chunk_len(17, 5), 5);
        assert_eq!(gz_read_chunk_len(5, 17), 5);
    }

    #[test]
    fn gz_read_chunk_len_caps_requests_at_uint_max() {
        let max = ::core::ffi::c_uint::MAX;
        let request = (max as crate::stdlib::z_size_t).checked_add(1);

        assert_eq!(
            gz_read_chunk_len(request.unwrap_or(max as crate::stdlib::z_size_t), 0),
            max
        );
    }

    #[test]
    fn gz_read_needs_fetch_for_look_state() {
        assert!(gz_read_needs_fetch(crate::gzguts_h::LOOK, 16, 8));
    }

    #[test]
    fn gz_read_needs_fetch_for_small_chunks_only() {
        assert!(gz_read_needs_fetch(crate::gzguts_h::COPY, 15, 8));
        assert!(!gz_read_needs_fetch(crate::gzguts_h::COPY, 16, 8));
    }

    #[test]
    fn gz_read_action_prioritizes_buffered_data() {
        assert!(matches!(
            gz_read_action(1, 1, 0, crate::gzguts_h::LOOK, 1, 8),
            GzReadAction::DrainBuffered
        ));
    }

    #[test]
    fn gz_read_action_stops_only_after_eof_with_no_input() {
        assert!(matches!(
            gz_read_action(0, 1, 0, crate::gzguts_h::COPY, 16, 8),
            GzReadAction::StopAtEof
        ));
        assert!(matches!(
            gz_read_action(0, 1, 1, crate::gzguts_h::COPY, 16, 8),
            GzReadAction::Load
        ));
    }

    #[test]
    fn gz_read_action_fetches_for_look_or_small_requests() {
        assert!(matches!(
            gz_read_action(0, 0, 0, crate::gzguts_h::LOOK, 16, 8),
            GzReadAction::Fetch
        ));
        assert!(matches!(
            gz_read_action(0, 0, 0, crate::gzguts_h::COPY, 15, 8),
            GzReadAction::Fetch
        ));
    }

    #[test]
    fn gz_read_action_selects_load_or_decompression() {
        assert!(matches!(
            gz_read_action(0, 0, 0, crate::gzguts_h::COPY, 16, 8),
            GzReadAction::Load
        ));
        assert!(matches!(
            gz_read_action(0, 0, 0, crate::gzguts_h::GZIP, 16, 8),
            GzReadAction::Decompress
        ));
    }

    #[test]
    fn gz_read_progress_updates_remaining_total_and_position() {
        assert_eq!(gz_read_progress(10, 4, 42, 3), (7, 7, 45));
    }

    #[test]
    fn gz_read_progress_preserves_wrapping_byte_counts() {
        assert_eq!(
            gz_read_progress(0, crate::stdlib::z_size_t::MAX, 0, 1),
            (crate::stdlib::z_size_t::MAX, 0, 1)
        );
    }

    #[test]
    fn gz_ungetc_buffer_state_prioritizes_empty_buffer() {
        assert!(matches!(
            gz_ungetc_buffer_state(0, 8),
            GzUngetcBufferState::Empty
        ));
    }

    #[test]
    fn gz_ungetc_buffer_state_requires_exact_double_size_to_be_full() {
        assert!(matches!(
            gz_ungetc_buffer_state(15, 8),
            GzUngetcBufferState::Pushable
        ));
        assert!(matches!(
            gz_ungetc_buffer_state(16, 8),
            GzUngetcBufferState::Full
        ));
        assert!(matches!(
            gz_ungetc_buffer_state(17, 8),
            GzUngetcBufferState::Pushable
        ));
    }

    #[test]
    fn gz_is_gzip_header_accepts_valid_header() {
        assert!(gz_is_gzip_header(31, 139, 8, 31));
    }

    #[test]
    fn gz_is_gzip_header_rejects_invalid_magic_or_flags() {
        assert!(!gz_is_gzip_header(30, 139, 8, 0));
        assert!(!gz_is_gzip_header(31, 139, 8, 32));
    }

    #[test]
    fn gz_look_needs_more_input_requires_data_for_initial_probe() {
        assert!(gz_look_needs_more_input(0, 0));
        assert!(!gz_look_needs_more_input(4, 0));
    }

    #[test]
    fn gz_look_needs_more_input_retries_until_header_is_wide_enough() {
        assert!(gz_look_needs_more_input(3, 1));
        assert!(!gz_look_needs_more_input(4, 1));
    }

    #[test]
    fn gz_look_action_preserves_partial_and_transparent_input_rules() {
        assert_eq!(gz_look_action(0, 0, None), GzLookAction::NeedMoreInput);
        assert_eq!(gz_look_action(3, 1, None), GzLookAction::NeedMoreInput);
        assert_eq!(
            gz_look_action(4, 0, Some([31, 139, 8, 31])),
            GzLookAction::Gzip
        );
        assert_eq!(
            gz_look_action(4, 0, Some([31, 139, 8, 32])),
            GzLookAction::TransparentCopy
        );
        assert_eq!(gz_look_action(3, 0, None), GzLookAction::TransparentCopy);
    }

    #[test]
    fn gz_skip_core_consumes_only_remaining_skip() {
        let mut have = 10;
        let mut pos = 42;
        let mut skip = 3;

        let consumed = gz_skip_core(
            &mut have,
            &mut pos,
            &mut skip,
            crate::src::gzlib::gz_intmax(),
        );

        assert_eq!(consumed, 3);
        assert_eq!(have, 7);
        assert_eq!(pos, 45);
        assert_eq!(skip, 0);
    }

    #[test]
    fn gz_skip_core_consumes_available_buffer() {
        let mut have = 10;
        let mut pos = 42;
        let mut skip = 15;

        let consumed = gz_skip_core(
            &mut have,
            &mut pos,
            &mut skip,
            crate::src::gzlib::gz_intmax(),
        );

        assert_eq!(consumed, 10);
        assert_eq!(have, 0);
        assert_eq!(pos, 52);
        assert_eq!(skip, 5);
    }

    #[test]
    fn gzgets_copy_len_prefers_smaller_buffer_limit() {
        assert_eq!(gzgets_copy_len(10, 4, None), 4);
        assert_eq!(gzgets_copy_len(4, 10, None), 4);
    }

    #[test]
    fn gzgets_copy_len_stops_after_newline_within_limit() {
        assert_eq!(gzgets_copy_len(10, 8, Some(0)), 1);
        assert_eq!(gzgets_copy_len(10, 8, Some(4)), 5);
    }

    #[test]
    fn gzgets_copy_len_ignores_newline_past_copy_limit() {
        assert_eq!(gzgets_copy_len(10, 4, Some(4)), 4);
        assert_eq!(gzgets_copy_len(3, 8, Some(9)), 3);
    }

    #[test]
    fn gzclose_r_result_preserves_buffer_error_on_clean_close() {
        assert_eq!(
            gzclose_r_result(crate::zlib_h::Z_BUF_ERROR, 0),
            crate::zlib_h::Z_BUF_ERROR
        );
        assert_eq!(
            gzclose_r_result(crate::zlib_h::Z_DATA_ERROR, 0),
            crate::zlib_h::Z_OK
        );
    }

    #[test]
    fn gzclose_r_result_prioritizes_close_failures() {
        assert_eq!(
            gzclose_r_result(crate::zlib_h::Z_BUF_ERROR, -1),
            crate::zlib_h::Z_ERRNO
        );
    }
}

unsafe extern "C" fn gz_read(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: crate::stdlib::voidp,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if (*state).skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    loop {
        n = gz_read_chunk_len(len, (*state).x.have);
        let advance = match gz_read_action(
            (*state).x.have,
            (*state).eof,
            (*state).strm.avail_in,
            (*state).how,
            n,
            (*state).size,
        ) {
            GzReadAction::DrainBuffered => {
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (*state).x.next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                (*state).x.next = (*state).x.next.offset(n as isize);
                (*state).x.have = (*state).x.have.wrapping_sub(n);
                if (*state).err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
                true
            }
            GzReadAction::StopAtEof => break,
            GzReadAction::Fetch => {
                if gz_fetch(state) == -1 as ::core::ffi::c_int
                    && (*state).x.have == 0 as ::core::ffi::c_uint
                {
                    err = -1 as ::core::ffi::c_int;
                }
                false
            }
            GzReadAction::Load => {
                err = gz_load(state, buf as *mut ::core::ffi::c_uchar, n, &raw mut n);
                true
            }
            GzReadAction::Decompress => {
                (*state).strm.avail_out = n as crate::stdlib::uInt;
                (*state).strm.next_out =
                    buf as *mut ::core::ffi::c_uchar as *mut crate::stdlib::Bytef;
                err = gz_decomp(state);
                n = (*state).x.have;
                (*state).x.have = 0 as ::core::ffi::c_uint;
                true
            }
        };
        if advance {
            (len, got, (*state).x.pos) = gz_read_progress(len, got, (*state).x.pos, n);
            buf = (buf as *mut ::core::ffi::c_char).offset(n as isize) as crate::stdlib::voidp;
        }
        if !(len != 0 && err == 0) {
            break;
        }
    }
    if len != 0 && (*state).eof != 0 {
        (*state).past = 1 as ::core::ffi::c_int;
    }
    return got;
}
pub unsafe extern "C" fn gzread(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if !gz_read_error_is_recoverable((*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !gzread_request_fits_int(len) {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    len = gz_read(state, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_uint;
    match gzread_outcome(len, (*state).err, (*state).again) {
        GzreadOutcome::Read(read) => read,
        GzreadOutcome::Error => -1 as ::core::ffi::c_int,
        GzreadOutcome::Again => {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_ERRNO,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            -1 as ::core::ffi::c_int
        }
    }
}
#[export_name = "gzread"]

pub unsafe extern "C" fn gzread_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzread(file, buf, len)
}
pub unsafe extern "C" fn gzfread(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let mut len: crate::stdlib::z_size_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return 0 as crate::stdlib::z_size_t;
    }
    if !gz_read_error_is_recoverable((*state).err, (*state).again) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(request_len) = gz_fread_request_len(size, nitems) else {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    };
    len = request_len;
    return if len != 0 {
        gz_fread_items_read(size, gz_read(state, buf, len))
    } else {
        0 as crate::stdlib::z_size_t
    };
}
#[export_name = "gzfread"]

pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    gzfread(buf, size, nitems, file)
}
pub unsafe extern "C" fn gzgetc(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if !gz_read_error_is_recoverable((*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).x.have != 0 {
        (*state).x.have = (*state).x.have.wrapping_sub(1);
        (*state).x.pos += 1;
        let c2rust_fresh2 = (*state).x.next;
        (*state).x.next = (*state).x.next.offset(1);
        return *c2rust_fresh2 as ::core::ffi::c_int;
    }
    return if gz_read(
        state,
        &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidp,
        1 as crate::stdlib::z_size_t,
    ) < 1 as crate::stdlib::z_size_t
    {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    };
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc(file)
}
pub unsafe extern "C" fn gzgetc_(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    return gzgetc(file);
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_(file)
}
pub unsafe extern "C" fn gzungetc(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).how == crate::gzguts_h::LOOK && (*state).x.have == 0 as ::core::ffi::c_uint {
        gz_look(state);
    }
    if !gz_read_error_is_recoverable((*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    match gz_ungetc_buffer_state((*state).x.have, (*state).size) {
        GzUngetcBufferState::Empty => {
            (*state).x.have = 1 as ::core::ffi::c_uint;
            (*state).x.next = (*state)
                .out
                .offset(((*state).size << 1 as ::core::ffi::c_int) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize));
            *(*state).x.next.offset(0 as ::core::ffi::c_int as isize) = c as ::core::ffi::c_uchar;
            (*state).x.pos -= 1;
            (*state).past = 0 as ::core::ffi::c_int;
            return c;
        }
        GzUngetcBufferState::Full => {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_DATA_ERROR,
                b"out of room to push characters\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        GzUngetcBufferState::Pushable => {}
    }
    if (*state).x.next == (*state).out {
        let mut src: *mut ::core::ffi::c_uchar = (*state).out.offset((*state).x.have as isize);
        let mut dest: *mut ::core::ffi::c_uchar = (*state)
            .out
            .offset(((*state).size << 1 as ::core::ffi::c_int) as isize);
        while src > (*state).out {
            src = src.offset(-1);
            dest = dest.offset(-1);
            *dest = *src;
        }
        (*state).x.next = dest;
    }
    (*state).x.have = (*state).x.have.wrapping_add(1);
    (*state).x.next = (*state).x.next.offset(-1);
    *(*state).x.next.offset(0 as ::core::ffi::c_int as isize) = c as ::core::ffi::c_uchar;
    (*state).x.pos -= 1;
    (*state).past = 0 as ::core::ffi::c_int;
    return c;
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    gzungetc(c, file)
}
pub unsafe extern "C" fn gzgets(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut left: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut eol: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() || buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !gz_read_error_is_recoverable((*state).err, (*state).again) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    str = buf;
    left = (len as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
    if left != 0 {
        while !((*state).x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(state) == -1 as ::core::ffi::c_int)
        {
            if (*state).x.have == 0 as ::core::ffi::c_uint {
                (*state).past = 1 as ::core::ffi::c_int;
                break;
            } else {
                n = gzgets_copy_len((*state).x.have, left, None);
                eol = crate::stdlib::memchr(
                    (*state).x.next as *const ::core::ffi::c_void,
                    '\n' as i32,
                    n as crate::__stddef_size_t_h::size_t,
                ) as *mut ::core::ffi::c_uchar;
                if !eol.is_null() {
                    n = gzgets_copy_len(
                        (*state).x.have,
                        left,
                        Some(eol.offset_from((*state).x.next) as usize),
                    );
                }
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    (*state).x.next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                (*state).x.have = (*state).x.have.wrapping_sub(n);
                (*state).x.next = (*state).x.next.offset(n as isize);
                (*state).x.pos += n as crate::stdlib::off64_t;
                left = left.wrapping_sub(n);
                buf = buf.offset(n as isize);
                if !(left != 0 && eol.is_null()) {
                    break;
                }
            }
        }
    }
    if buf == str {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *buf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    return str;
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    gzgets(file, buf, len)
}
pub unsafe extern "C" fn gzdirect(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode == crate::gzguts_h::GZ_READ
        && (*state).how == crate::gzguts_h::LOOK
        && (*state).x.have == 0 as ::core::ffi::c_uint
    {
        gz_look(state);
    }
    return ((*state).direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzdirect(file)
}
pub unsafe extern "C" fn gzclose_r(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let stream_err: ::core::ffi::c_int;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).size != 0 {
        crate::src::inflate::inflateEnd(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
        crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    stream_err = (*state).err;
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
    ret = crate::stdlib::close((*state).fd);
    crate::stdlib::free(state as *mut ::core::ffi::c_void);
    return gzclose_r_result(stream_err, ret);
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_r(file)
}
