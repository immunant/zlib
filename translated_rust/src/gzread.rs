pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
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

enum GzLoadBuffer<'a> {
    Slice(&'a mut [u8]),
    Output,
}

unsafe fn gz_load(
    state: &mut crate::gzguts_h::gz_state,
    buffer: GzLoadBuffer<'_>,
) -> Result<usize, ()> {
    let buf = match buffer {
        GzLoadBuffer::Slice(buf) => buf,
        GzLoadBuffer::Output => {
            let Some(len) = (state.size as usize).checked_mul(2) else {
                return Err(());
            };
            if len > state.out.len() {
                return Err(());
            }
            &mut state.out[..len]
        }
    };
    let mut ret: ::core::ffi::c_int = 0;
    let mut have = 0usize;
    let max = ((-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2) + 1) as usize;
    state.again = 0 as ::core::ffi::c_int;
    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
    loop {
        let get = (buf.len() - have).min(max);
        ret = crate::stdlib::read(
            state.fd,
            buf[have..].as_mut_ptr() as *mut ::core::ffi::c_void,
            get,
        ) as ::core::ffi::c_int;
        if ret <= 0 as ::core::ffi::c_int {
            break;
        }
        have += ret as usize;
        if have >= buf.len() {
            break;
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        if *crate::stdlib::__errno_location() == crate::stdlib::EAGAIN
            || *crate::stdlib::__errno_location() == crate::stdlib::EWOULDBLOCK
        {
            state.again = 1 as ::core::ffi::c_int;
            if have != 0 {
                return Ok(have);
            }
        }
        let message = crate::stdlib::strerror(*crate::stdlib::__errno_location());
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_ERRNO,
            (!message.is_null()).then(|| ::core::ffi::CStr::from_ptr(message)),
        );
        return Err(());
    }
    if ret == 0 as ::core::ffi::c_int {
        state.eof = 1 as ::core::ffi::c_int;
    }
    Ok(have)
}

unsafe fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if state.eof == 0 as ::core::ffi::c_int {
        let available = state.strm.avail_in as usize;
        let capacity = state.size as usize;
        if available > capacity || state.in_0.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        let input_start = state.strm.next_in.addr().wrapping_sub(state.in_0.addr());
        if available != 0 && (input_start > capacity || available > capacity - input_start) {
            return -1 as ::core::ffi::c_int;
        }
        let input = ::core::slice::from_raw_parts_mut(state.in_0, capacity);
        if available != 0 && input_start != 0 {
            input.copy_within(input_start..input_start + available, 0);
        }
        let got = match gz_load(state, GzLoadBuffer::Slice(&mut input[available..])) {
            Ok(got) => got,
            Err(()) => return -1 as ::core::ffi::c_int,
        };
        state.strm.avail_in = (available + got) as crate::stdlib::uInt;
        state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 as ::core::ffi::c_uint {
        state.in_0 = crate::stdlib::malloc(state.want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        let Some(output_len) = (state.want as usize).checked_mul(2) else {
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        };
        if state.out.try_reserve_exact(output_len).is_ok() {
            state.out.resize(output_len, 0);
        }
        if state.in_0.is_null() || state.out.len() != output_len {
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
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
            &mut state.strm,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            state.out.clear();
            state.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
        crate::src::inflate::inflateReset(&mut state.strm);
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
    let input = ::core::slice::from_raw_parts(
        state.strm.next_in as *const u8,
        state.strm.avail_in as usize,
    );
    if input.len() > 3 && input[0] == 31 && input[1] == 139 && input[2] == 8 && input[3] < 32 {
        crate::src::inflate::inflateReset(&mut state.strm);
        state.how = crate::gzguts_h::GZIP;
        state.junk = 1 as ::core::ffi::c_int;
        state.direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if input.len() > state.out.len() {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    }
    state.out[..input.len()].copy_from_slice(input);
    state.x.next = state.out.as_mut_ptr();
    state.x.have = state.strm.avail_in as ::core::ffi::c_uint;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
    state.how = crate::gzguts_h::COPY;
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut had: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut state.strm;
    had = (*strm).avail_out as ::core::ffi::c_uint;
    loop {
        if (*strm).avail_in == 0 as crate::stdlib::uInt
            && gz_avail(state) == -1 as ::core::ffi::c_int
        {
            ret = state.err;
            break;
        } else if (*strm).avail_in == 0 as crate::stdlib::uInt {
            if (*state).again == 0 {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_BUF_ERROR,
                    Some(c"unexpected end of file"),
                );
            }
            break;
        } else {
            ret = crate::src::inflate::inflate(&mut state.strm, crate::zlib_h::Z_NO_FLUSH);
            if (*strm).avail_out < had {
                (*state).junk = 0 as ::core::ffi::c_int;
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"internal error: inflate stream corrupt"),
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    Some(c"out of memory"),
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
                    let message = if (*strm).msg.is_null() {
                        c"compressed data error"
                    } else {
                        ::core::ffi::CStr::from_ptr((*strm).msg)
                    };
                    crate::src::gzlib::gz_error_state(
                        state,
                        crate::zlib_h::Z_DATA_ERROR,
                        Some(message),
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

unsafe fn gz_fetch(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    loop {
        match state.how {
            crate::gzguts_h::LOOK => {
                if gz_look(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if state.how == crate::gzguts_h::LOOK {
                    return 0 as ::core::ffi::c_int;
                }
            }
            crate::gzguts_h::COPY => {
                state.x.have = match gz_load(state, GzLoadBuffer::Output) {
                    Ok(got) => got as ::core::ffi::c_uint,
                    Err(()) => return -1 as ::core::ffi::c_int,
                };
                state.x.next = state.out.as_mut_ptr();
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                state.strm.avail_out =
                    (state.size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                state.strm.next_out = state.out.as_mut_ptr();
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(c"state corrupt"),
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if !(state.x.have == 0 as ::core::ffi::c_uint
            && (state.eof == 0 || state.strm.avail_in != 0))
        {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_skip(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0;
    loop {
        if state.x.have != 0 {
            n = if ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<crate::stdlib::off64_t>()
                && state.x.have > crate::src::gzlib::gz_intmax()
                || state.x.have as crate::stdlib::off64_t > state.skip
            {
                state.skip as ::core::ffi::c_uint
            } else {
                state.x.have
            };
            state.x.have = state.x.have.wrapping_sub(n);
            state.x.next = state.x.next.offset(n as isize);
            state.x.pos += n as crate::stdlib::off64_t;
            state.skip -= n as crate::stdlib::off64_t;
        } else {
            if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                break;
            }
            if gz_fetch(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        if state.skip == 0 {
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
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    gz_read_impl(
        &mut *state,
        ::core::slice::from_raw_parts_mut(buf as *mut u8, len),
    )
}

unsafe fn gz_read_impl(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [u8],
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = buf.len();
    let mut out = 0usize;
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    's_140: loop {
        n = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
        if n as usize > len {
            n = len as ::core::ffi::c_uint;
        }
        's_28: {
            if state.x.have != 0 {
                if state.x.have < n {
                    n = state.x.have;
                }
                let count = n as usize;
                let source = ::core::slice::from_raw_parts(state.x.next, count);
                buf[out..out + count].copy_from_slice(source);
                state.x.next = state.x.next.wrapping_add(count);
                state.x.have = state.x.have.wrapping_sub(n);
                if state.err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            } else {
                if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                    break 's_140;
                }
                if state.how == crate::gzguts_h::LOOK || n < state.size << 1 as ::core::ffi::c_int {
                    if gz_fetch(state) == -1 as ::core::ffi::c_int
                        && state.x.have == 0 as ::core::ffi::c_uint
                    {
                        err = -1 as ::core::ffi::c_int;
                    }
                    break 's_28;
                } else if state.how == crate::gzguts_h::COPY {
                    match gz_load(state, GzLoadBuffer::Slice(&mut buf[out..out + n as usize])) {
                        Ok(got) => n = got as ::core::ffi::c_uint,
                        Err(()) => err = -1 as ::core::ffi::c_int,
                    }
                } else {
                    state.strm.avail_out = n as crate::stdlib::uInt;
                    state.strm.next_out = buf[out..].as_mut_ptr() as *mut crate::stdlib::Bytef;
                    err = gz_decomp(state);
                    n = state.x.have;
                    state.x.have = 0 as ::core::ffi::c_uint;
                }
            }
            len -= n as usize;
            out += n as usize;
            got = got.wrapping_add(n as crate::stdlib::z_size_t);
            state.x.pos += n as crate::stdlib::off64_t;
        }
        if !(len != 0 && err == 0) {
            break;
        }
    }
    if len != 0 && state.eof != 0 {
        state.past = 1 as ::core::ffi::c_int;
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
    if (*state).err != crate::zlib_h::Z_OK
        && (*state).err != crate::zlib_h::Z_BUF_ERROR
        && (*state).again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *state;
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"request does not fit in an int"),
        );
        return -1 as ::core::ffi::c_int;
    }
    len = gz_read(state as *mut _, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        if (*state).err != crate::zlib_h::Z_OK && (*state).err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if (*state).again != 0 {
            let message = crate::stdlib::strerror(*crate::stdlib::__errno_location());
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_ERRNO,
                (!message.is_null()).then(|| ::core::ffi::CStr::from_ptr(message)),
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
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return 0 as crate::stdlib::z_size_t;
    }
    if (*state).err != crate::zlib_h::Z_OK
        && (*state).err != crate::zlib_h::Z_BUF_ERROR
        && (*state).again == 0
    {
        return 0 as crate::stdlib::z_size_t;
    }
    let state = &mut *state;
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"request does not fit in a size_t"),
        );
        return 0 as crate::stdlib::z_size_t;
    }
    return if len != 0 {
        gz_read(state as *mut _, buf, len).wrapping_div(size)
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
    if (*state).err != crate::zlib_h::Z_OK
        && (*state).err != crate::zlib_h::Z_BUF_ERROR
        && (*state).again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *state;
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if (*state).x.have != 0 {
        (*state).x.have = (*state).x.have.wrapping_sub(1);
        (*state).x.pos += 1;
        let c2rust_fresh2 = (*state).x.next;
        (*state).x.next = (*state).x.next.offset(1);
        return *c2rust_fresh2 as ::core::ffi::c_int;
    }
    return if gz_read(
        state as *mut _,
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
pub unsafe extern "C" fn gzgetc_(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    return gzgetc(file);
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_(file)
}
unsafe fn gzungetc(
    c: ::core::ffi::c_int,
    state: &mut crate::gzguts_h::gz_state,
) -> ::core::ffi::c_int {
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
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let Some(output_len) = (state.size as usize).checked_mul(2) else {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    };
    if state.out.len() != output_len {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    }
    if state.x.have == 0 as ::core::ffi::c_uint {
        let Some(last) = output_len.checked_sub(1) else {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"state corrupt"),
            );
            return -1 as ::core::ffi::c_int;
        };
        state.x.have = 1 as ::core::ffi::c_uint;
        state.x.next = state.out.as_mut_ptr().wrapping_add(last);
        state.out[last] = c as ::core::ffi::c_uchar;
        state.x.pos -= 1;
        state.past = 0 as ::core::ffi::c_int;
        return c;
    }
    if state.x.have as usize == output_len {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            Some(c"out of room to push characters"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let have = state.x.have as usize;
    if have > output_len {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let Some(mut next) = state.x.next.addr().checked_sub(state.out.as_ptr().addr()) else {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    };
    if next > output_len || have > output_len - next {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    }
    if next == 0 {
        let moved = output_len - have;
        state.out.copy_within(..have, moved);
        next = moved;
    }
    state.x.have = state.x.have.wrapping_add(1);
    let Some(next_byte) = next.checked_sub(1) else {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"state corrupt"),
        );
        return -1 as ::core::ffi::c_int;
    };
    state.x.next = state.out.as_mut_ptr().wrapping_add(next_byte);
    state.out[next_byte] = c as ::core::ffi::c_uchar;
    state.x.pos -= 1;
    state.past = 0 as ::core::ffi::c_int;
    return c;
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzungetc(c, state)
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
    let state = &mut *state;
    if state.mode != crate::gzguts_h::GZ_READ {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    str = buf;
    left = (len as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint);
    if left != 0 {
        while !(state.x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(state) == -1 as ::core::ffi::c_int)
        {
            if state.x.have == 0 as ::core::ffi::c_uint {
                state.past = 1 as ::core::ffi::c_int;
                break;
            } else {
                n = if state.x.have > left {
                    left
                } else {
                    state.x.have
                };
                eol = crate::stdlib::memchr(
                    state.x.next as *const ::core::ffi::c_void,
                    '\n' as ::core::ffi::c_int,
                    n as crate::__stddef_size_t_h::size_t,
                ) as *mut ::core::ffi::c_uchar;
                if !eol.is_null() {
                    n = (eol.offset_from(state.x.next) as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint);
                }
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    state.x.next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                state.x.have = state.x.have.wrapping_sub(n);
                state.x.next = state.x.next.offset(n as isize);
                state.x.pos += n as crate::stdlib::off64_t;
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
    *buf.offset(0 as isize) = 0 as ::core::ffi::c_char;
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
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.size != 0 {
        crate::src::inflate::inflateEnd(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
    }
    let err = if state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    let ret = crate::stdlib::close(state.fd);
    drop(Box::from_raw(state));
    if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    }
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_r(file)
}
