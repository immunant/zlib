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

fn compact_buffered_input(buffer: &mut [crate::stdlib::Bytef], source_start: usize, len: usize) {
    buffer.copy_within(source_start..source_start + len, 0);
}

fn is_gzip_header(input: &[u8]) -> bool {
    input.len() >= 4 && input[0] == 31 && input[1] == 139 && input[2] == 8 && input[3] < 32
}

fn copy_buffered_input(input: &[u8], output: &mut [u8]) {
    output[..input.len()].copy_from_slice(input);
}

fn copy_through_newline(input: &[u8], output: &mut [u8]) -> (usize, bool) {
    let copied = input
        .iter()
        .position(|&byte| byte == b'\n')
        .map_or(input.len(), |newline| newline + 1);
    output[..copied].copy_from_slice(&input[..copied]);
    (copied, copied != input.len())
}

fn pushback_empty(buffer: &mut [u8], byte: u8) -> Option<usize> {
    let next = buffer.len().checked_sub(1)?;
    buffer[next] = byte;
    Some(next)
}

fn pushback_buffer(
    buffer: &mut [u8],
    next: usize,
    have: usize,
    byte: u8,
) -> Option<usize> {
    if next > buffer.len() || have >= buffer.len() {
        return None;
    }
    let next = if next == 0 {
        let shifted = buffer.len().checked_sub(have)?;
        buffer.copy_within(0..have, shifted);
        shifted
    } else {
        next
    };
    let next = next.checked_sub(1)?;
    buffer[next] = byte;
    Some(next)
}

unsafe extern "C" fn gz_load(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
    mut have: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut get: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    let state = &mut *state;
    let have = &mut *have;
    let errno = &mut *crate::stdlib::__errno_location();
    state.again = 0 as ::core::ffi::c_int;
    *errno = 0 as ::core::ffi::c_int;
    *have = 0 as ::core::ffi::c_uint;
    loop {
        get = len.wrapping_sub(*have);
        if get > max {
            get = max;
        }
        ret = crate::stdlib::read(
            state.fd,
            buf.offset(*have as isize) as *mut ::core::ffi::c_void,
            get as crate::__stddef_size_t_h::size_t,
        ) as ::core::ffi::c_int;
        if ret <= 0 as ::core::ffi::c_int {
            break;
        }
        *have = (*have).wrapping_add(ret as ::core::ffi::c_uint);
        if *have >= len {
            break;
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        let errno_value = *errno;
        if errno_value == crate::stdlib::EAGAIN
            || errno_value == crate::stdlib::EWOULDBLOCK
        {
            state.again = 1 as ::core::ffi::c_int;
            if *have != 0 as ::core::ffi::c_uint {
                return 0 as ::core::ffi::c_int;
            }
        }
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_ERRNO,
            crate::stdlib::strerror(errno_value),
        );
        return -1 as ::core::ffi::c_int;
    }
    if ret == 0 as ::core::ffi::c_int {
        state.eof = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_avail(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_uint = 0;
    let state = &mut *state;
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if state.eof == 0 as ::core::ffi::c_int {
        {
            let strm = &mut state.strm;
            if strm.avail_in != 0 {
                let buffer = state.in_0.as_deref_mut().unwrap();
                let p = buffer.as_mut_ptr();
                let q: *const ::core::ffi::c_uchar = strm.next_in;
                if q != p as *const ::core::ffi::c_uchar {
                    let n = strm.avail_in as usize;
                    let size = state.size as usize;
                    if p.is_null() || q.is_null() || n > size {
                        return -1 as ::core::ffi::c_int;
                    }
                    // `strm.next_in` is always a cursor in `in_0`: gz_load()
                    // installs the buffer, and inflate only advances that cursor.
                    let Some(source_start) = q.addr().checked_sub(p.addr()) else {
                        return -1 as ::core::ffi::c_int;
                    };
                    if source_start <= size && n <= size.wrapping_sub(source_start) {
                        compact_buffered_input(buffer, source_start, n);
                    } else {
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
        }
        let avail_in = state.strm.avail_in;
        if gz_load(
            state as *mut crate::gzguts_h::gz_state,
            state.in_0.as_deref_mut().unwrap().as_mut_ptr().add(avail_in as usize),
            state.size.wrapping_sub(avail_in as ::core::ffi::c_uint),
            &raw mut got,
        ) == -1 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        state.strm.avail_in = state.strm.avail_in.wrapping_add(got);
        state.strm.next_in = state.in_0.as_deref_mut().unwrap().as_mut_ptr();
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_look(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let state = &mut *state;
    if state.size == 0 as ::core::ffi::c_uint {
        state.in_0 = crate::src::gzlib::gz_buffer(state.want);
        state.out = crate::src::gzlib::gz_buffer(state.want << 1);
        if state.in_0.is_none() || state.out.is_none() {
            state.out = None;
            state.in_0 = None;
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        state.size = state.want;
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        state.strm.avail_in = 0 as crate::stdlib::uInt;
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2_(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            state.out = None;
            state.in_0 = None;
            state.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflateReset(&raw mut state.strm as *mut crate::zlib_h::z_stream_s);
        state.how = crate::gzguts_h::GZIP;
        state.junk = (state.junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        state.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.strm.avail_in == 0 as crate::stdlib::uInt
        || state.again != 0 && state.strm.avail_in < 4 as crate::stdlib::uInt
    {
        return 0 as ::core::ffi::c_int;
    }
    let avail_in = state.strm.avail_in as usize;
    let next_in = state.strm.next_in;
    // `gz_avail()` installs `next_in` in this owned buffer, and inflate only
    // advances that cursor. Validate it before deriving the input view.
    let Some(input) = state.in_0.as_deref().and_then(|buffer| {
        let start = next_in.addr().checked_sub(buffer.as_ptr().addr())?;
        let end = start.checked_add(avail_in)?;
        buffer.get(start..end)
    }) else {
        return -1 as ::core::ffi::c_int;
    };
    if is_gzip_header(input) {
        crate::src::inflate::inflateReset(&raw mut state.strm as *mut crate::zlib_h::z_stream_s);
        state.how = crate::gzguts_h::GZIP;
        state.junk = 1 as ::core::ffi::c_int;
        state.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    let Some(output) = state
        .out
        .as_deref_mut()
        .and_then(|buffer| buffer.get_mut(..avail_in))
    else {
        return -1 as ::core::ffi::c_int;
    };
    copy_buffered_input(input, output);
    state.x.next = output.as_mut_ptr();
    state.x.have = avail_in as ::core::ffi::c_uint;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
    state.how = crate::gzguts_h::COPY;
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
    (*state).x.next = (*strm)
        .next_out
        .wrapping_sub((*state).x.have as usize) as *mut ::core::ffi::c_uchar;
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
                    (*state).out.as_deref_mut().unwrap().as_mut_ptr(),
                    (*state).size << 1 as ::core::ffi::c_int,
                    &raw mut (*state).x.have,
                ) == -1 as ::core::ffi::c_int
                {
                    return -1 as ::core::ffi::c_int;
                }
                (*state).x.next = (*state).out.as_deref_mut().unwrap().as_mut_ptr();
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                (*strm).avail_out =
                    ((*state).size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                (*strm).next_out = (*state).out.as_deref_mut().unwrap().as_mut_ptr();
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

unsafe extern "C" fn gz_skip(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0;
    loop {
        if (*state).x.have != 0 {
            n = if ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<crate::stdlib::off64_t>()
                && (*state).x.have > crate::src::gzlib::gz_intmax()
                || (*state).x.have as crate::stdlib::off64_t > (*state).skip
            {
                (*state).skip as ::core::ffi::c_uint
            } else {
                (*state).x.have
            };
            (*state).x.have = (*state).x.have.wrapping_sub(n);
            (*state).x.next = (*state).x.next.wrapping_add(n as usize);
            (*state).x.pos += n as crate::stdlib::off64_t;
            (*state).skip -= n as crate::stdlib::off64_t;
        } else {
            if (*state).eof != 0 && (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if (*state).skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
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
    's_140: loop {
        n = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
        if n as crate::stdlib::z_size_t > len {
            n = len as ::core::ffi::c_uint;
        }
        's_28: {
            if (*state).x.have != 0 {
                if (*state).x.have < n {
                    n = (*state).x.have;
                }
                let input = ::core::slice::from_raw_parts((*state).x.next, n as usize);
                let output = ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), n as usize);
                copy_buffered_input(input, output);
                (*state).x.next = (*state).x.next.wrapping_add(n as usize);
                (*state).x.have = (*state).x.have.wrapping_sub(n);
                if (*state).err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            } else {
                if (*state).eof != 0 && (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                    break 's_140;
                }
                if (*state).how == crate::gzguts_h::LOOK
                    || n < (*state).size << 1 as ::core::ffi::c_int
                {
                    if gz_fetch(state) == -1 as ::core::ffi::c_int
                        && (*state).x.have == 0 as ::core::ffi::c_uint
                    {
                        err = -1 as ::core::ffi::c_int;
                    }
                    break 's_28;
                } else if (*state).how == crate::gzguts_h::COPY {
                    err = gz_load(state, buf as *mut ::core::ffi::c_uchar, n, &raw mut n);
                } else {
                    (*state).strm.avail_out = n as crate::stdlib::uInt;
                    (*state).strm.next_out =
                        buf as *mut ::core::ffi::c_uchar as *mut crate::stdlib::Bytef;
                    err = gz_decomp(state);
                    n = (*state).x.have;
                    (*state).x.have = 0 as ::core::ffi::c_uint;
                }
            }
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            buf = (buf as *mut u8).wrapping_add(n as usize) as crate::stdlib::voidp;
            got = got.wrapping_add(n as crate::stdlib::z_size_t);
            (*state).x.pos += n as crate::stdlib::off64_t;
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
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    len = gz_read(state, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if state.again != 0 {
            crate::src::gzlib::gz_error(
                state,
                crate::zlib_h::Z_ERRNO,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    return len as ::core::ffi::c_int;
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
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    }
    return if len != 0 {
        gz_read(state, buf, len).wrapping_div(size)
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
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if state.x.have != 0 {
        // `x.next` is an ABI cursor into the owned output buffer here.  Do
        // not construct a raw slice from its advertised `have` length: a
        // corrupt cursor must not extend the view past that allocation.
        let cursor = state.x.next;
        let Some(byte) = state.out.as_deref().and_then(|buffer| {
            let offset = cursor.addr().checked_sub(buffer.as_ptr().addr())?;
            buffer.get(offset).copied()
        }) else {
            return -1 as ::core::ffi::c_int;
        };
        state.x.have = state.x.have.wrapping_sub(1);
        state.x.pos += 1;
        state.x.next = state.x.next.wrapping_add(1);
        return byte as ::core::ffi::c_int;
    }
    return if gz_read(
        state,
        &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidp,
        1 as crate::stdlib::z_size_t,
    ) < 1 as crate::stdlib::z_size_t
    {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as usize] as ::core::ffi::c_int
    };
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc(file)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc(file)
}
pub unsafe extern "C" fn gzungetc(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.how == crate::gzguts_h::LOOK && state.x.have == 0 as ::core::ffi::c_uint {
        gz_look(state);
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.x.have == 0 as ::core::ffi::c_uint {
        let size = state.size as usize;
        let Some(capacity) = size.checked_mul(2) else {
            return -1 as ::core::ffi::c_int;
        };
        let buffer = &mut state.out.as_deref_mut().unwrap()[..capacity];
        let Some(next) = pushback_empty(buffer, c as ::core::ffi::c_uchar) else {
            return -1 as ::core::ffi::c_int;
        };
        state.x.have = 1 as ::core::ffi::c_uint;
        state.x.next = buffer.as_mut_ptr().wrapping_add(next);
        state.x.pos -= 1;
        state.past = 0 as ::core::ffi::c_int;
        return c;
    }
    if state.x.have == state.size << 1 as ::core::ffi::c_int {
        crate::src::gzlib::gz_error(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            b"out of room to push characters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    let size = state.size as usize;
    let Some(capacity) = size.checked_mul(2) else {
        return -1 as ::core::ffi::c_int;
    };
    let buffer = &mut state.out.as_deref_mut().unwrap()[..capacity];
    let out = buffer.as_mut_ptr();
    let cursor = state.x.next;
    if cursor.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let Some(next) = cursor.addr().checked_sub(out.addr()) else {
        return -1 as ::core::ffi::c_int;
    };
    let Some(next) = pushback_buffer(buffer, next, state.x.have as usize, c as ::core::ffi::c_uchar) else {
        return -1 as ::core::ffi::c_int;
    };
    state.x.have = state.x.have.wrapping_add(1);
    state.x.next = out.wrapping_add(next);
    state.x.pos -= 1;
    state.past = 0 as ::core::ffi::c_int;
    return c;
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    gzungetc(c, file)
}
unsafe fn gzgets(
    mut state: Option<::core::ptr::NonNull<crate::gzguts_h::gz_state>>,
    output: &mut [u8],
) -> *mut ::core::ffi::c_char {
    if output.is_empty() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let Some(mut state) = state else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let state = state.as_mut();
    if state.mode != crate::gzguts_h::GZ_READ {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let str = output.as_mut_ptr().cast::<::core::ffi::c_char>();
    let mut left = output.len() - 1;
    let mut written = 0;
    if left != 0 {
        while !(state.x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(state) == -1 as ::core::ffi::c_int)
        {
            if state.x.have == 0 as ::core::ffi::c_uint {
                state.past = 1 as ::core::ffi::c_int;
                break;
            } else {
                let mut n = if state.x.have as usize > left {
                    left
                } else {
                    state.x.have as usize
                };
                // `x.next` is a cursor in the owned output buffer whenever
                // `x.have` is nonzero.  Rebuild that view with a checked
                // range, so a corrupt cursor cannot extend a raw slice past
                // the allocation's remaining capacity.
                let cursor = state.x.next;
                let Some(input) = state.out.as_deref().and_then(|buffer| {
                    let start = cursor.addr().checked_sub(buffer.as_ptr().addr())?;
                    let end = start.checked_add(n)?;
                    buffer.get(start..end)
                }) else {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                };
                let (copied, found_newline) =
                    copy_through_newline(input, &mut output[written..written + n]);
                n = copied;
                state.x.have = state.x.have.wrapping_sub(n as ::core::ffi::c_uint);
                state.x.next = state.x.next.wrapping_add(n);
                state.x.pos += n as crate::stdlib::off64_t;
                left -= n;
                written += n;
                if !(left != 0 && !found_newline) {
                    break;
                }
            }
        }
    }
    if written == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    output[written] = 0;
    return str;
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if file.is_null() || buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let output = ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as usize);
    gzgets(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        output,
    )
}
pub unsafe extern "C" fn gzdirect(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0 as ::core::ffi::c_uint
    {
        gz_look(state);
    }
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzdirect(file)
}
pub unsafe extern "C" fn gzclose_r(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_ptr = file as crate::gzguts_h::gz_statep;
    if (*state_ptr).mode != crate::gzguts_h::GZ_READ {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state_ptr;
    if state.size != 0 {
        crate::src::inflate::inflateEnd(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        state.out = None;
        state.in_0 = None;
    }
    err = if state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    state.path = None;
    state.msg = None;
    ret = crate::stdlib::close(state.fd);
    drop(Vec::from_raw_parts(state_ptr, 1, 1));
    return if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    };
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_r(file)
}
