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
use std::os::fd::BorrowedFd;

fn gz_load_error(state: &mut crate::gzguts_h::gz_state, error: rustix::io::Errno) {
    let message = ::std::ffi::CString::new(::std::io::Error::from(error).to_string()).ok();
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_ERRNO, message.as_deref());
}

fn gz_load(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    again: &mut ::core::ffi::c_int,
    eof: &mut ::core::ffi::c_int,
) -> Result<usize, (usize, rustix::io::Errno)> {
    let mut have = 0usize;
    let max = ((-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2) + 1) as usize;
    *again = 0;
    while have != buf.len() {
        let get = (buf.len() - have).min(max);
        match rustix::io::read(fd, &mut buf[have..have + get]) {
            Ok(0) => {
                *eof = 1;
                break;
            }
            Ok(read) => have += read,
            Err(error) => {
                if error == rustix::io::Errno::AGAIN || error == rustix::io::Errno::WOULDBLOCK {
                    *again = 1;
                    if have != 0 {
                        return Ok(have);
                    }
                }
                return Err((have, error));
            }
        }
    }
    Ok(have)
}

unsafe fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return -1 as ::core::ffi::c_int;
    }
    if state.eof == 0 as ::core::ffi::c_int {
        if state.in_0.len() != state.size as usize {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal read buffer corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        let available = state.strm.avail_in as usize;
        if available > state.size as usize {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal read buffer corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        let (input, again, eof) = (&mut state.in_0, &mut state.again, &mut state.eof);
        let input = &mut input[..];
        if available != 0 {
            if state.strm.next_in.is_null() {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            }
            let Some(offset) = state
                .strm
                .next_in
                .addr()
                .checked_sub(input.as_ptr().addr())
            else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            };
            let Some(end) = offset.checked_add(available) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            };
            if end > input.len() {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            }
            input.copy_within(offset..end, 0);
        }
        let fd = if state.fd < 0 {
            gz_load_error(state, rustix::io::Errno::BADF);
            return -1 as ::core::ffi::c_int;
        } else {
            BorrowedFd::borrow_raw(state.fd)
        };
        let Some(input) = input.get_mut(available..) else {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal read buffer corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        };
        let read = match gz_load(fd, input, again, eof) {
            Ok(read) => read,
            Err((_, error)) => {
                gz_load_error(state, error);
                return -1 as ::core::ffi::c_int;
            }
        };
        state.strm.avail_in = state.strm.avail_in.wrapping_add(read as ::core::ffi::c_uint);
        state.strm.next_in = state.in_0.as_mut_ptr() as *mut crate::stdlib::Bytef;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_look(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint {
        let want = (*state).want as usize;
        let Some(output_len) = want.checked_mul(2) else {
            crate::src::gzlib::gz_static_error(
                &mut *state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        };
        let mut input = Vec::new();
        let mut output = Vec::new();
        if input.try_reserve_exact(want).is_err()
            || output.try_reserve_exact(output_len).is_err()
        {
            crate::src::gzlib::gz_static_error(
                &mut *state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        input.resize(want, 0);
        output.resize(output_len, 0);
        (*state).in_0 = ::core::mem::ManuallyDrop::new(input);
        (*state).out = ::core::mem::ManuallyDrop::new(output);
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
            ::core::mem::ManuallyDrop::drop(&mut (*state).out);
            ::core::mem::ManuallyDrop::drop(&mut (*state).in_0);
            (*state).size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_static_error(
                &mut *state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
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
    if gz_avail(&mut *state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*strm).avail_in == 0 as crate::stdlib::uInt
        || (*state).again != 0 && (*strm).avail_in < 4 as crate::stdlib::uInt
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*strm).avail_in > 3 as crate::stdlib::uInt
        && *(*strm).next_in.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 31 as ::core::ffi::c_int
        && *(*strm).next_in.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 139 as ::core::ffi::c_int
        && *(*strm).next_in.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 8 as ::core::ffi::c_int
        && (*(*strm).next_in.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            < 32 as ::core::ffi::c_int
    {
        crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).how = crate::gzguts_h::GZIP;
        (*state).junk = 1 as ::core::ffi::c_int;
        (*state).direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    (*state).x.next = (*state).out.as_mut_ptr();
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
            && gz_avail(&mut *state) == -1 as ::core::ffi::c_int
        {
            ret = (*state).err;
            break;
        } else if (*strm).avail_in == 0 as crate::stdlib::uInt {
            if (*state).again == 0 {
                crate::src::gzlib::gz_static_error(
                    &mut *state,
                    crate::zlib_h::Z_BUF_ERROR,
                    b"unexpected end of file\0",
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
                crate::src::gzlib::gz_static_error(
                    &mut *state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0",
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_static_error(
                    &mut *state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0",
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
                        ::std::ffi::CStr::from_bytes_with_nul(b"compressed data error\0").ok()
                    } else {
                        Some(::std::ffi::CStr::from_ptr((*strm).msg))
                    };
                    crate::src::gzlib::gz_error_state(
                        &mut *state,
                        crate::zlib_h::Z_DATA_ERROR,
                        message,
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
                let fd = if (*state).fd < 0 {
                    gz_load_error(&mut *state, rustix::io::Errno::BADF);
                    return -1 as ::core::ffi::c_int;
                } else {
                    BorrowedFd::borrow_raw((*state).fd)
                };
                let output = ::core::slice::from_raw_parts_mut(
                    (*state).out.as_mut_ptr(),
                    ((*state).size << 1) as usize,
                );
                let read = match gz_load(fd, output, &mut (*state).again, &mut (*state).eof) {
                    Ok(read) => read,
                    Err((_, error)) => {
                        gz_load_error(&mut *state, error);
                        return -1 as ::core::ffi::c_int;
                    }
                };
                (*state).x.have = read as ::core::ffi::c_uint;
                (*state).x.next = (*state).out.as_mut_ptr();
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                (*strm).avail_out =
                    ((*state).size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                (*strm).next_out = (*state).out.as_mut_ptr() as *mut crate::stdlib::Bytef;
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_static_error(
                    &mut *state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"state corrupt\0",
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
            (*state).x.next = (*state).x.next.offset(n as isize);
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

unsafe fn gz_read(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: &mut [crate::stdlib::Bytef],
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = buf.len() as crate::stdlib::z_size_t;
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
                let source = ::core::slice::from_raw_parts((*state).x.next, n as usize);
                for (dest, source) in buf[..n as usize].iter_mut().zip(source) {
                    *dest = *source;
                }
                (*state).x.next = (*state).x.next.offset(n as isize);
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
                    if (*state).fd < 0 {
                        gz_load_error(&mut *state, rustix::io::Errno::BADF);
                        err = -1;
                        n = 0;
                    } else {
                        let fd = BorrowedFd::borrow_raw((*state).fd);
                        match gz_load(fd, &mut buf[..n as usize], &mut (*state).again, &mut (*state).eof) {
                            Ok(read) => n = read as ::core::ffi::c_uint,
                            Err((read, error)) => {
                                gz_load_error(&mut *state, error);
                                err = -1;
                                n = read as ::core::ffi::c_uint;
                            }
                        }
                    }
                } else {
                    (*state).strm.avail_out = n as crate::stdlib::uInt;
                    (*state).strm.next_out = buf.as_mut_ptr();
                    err = gz_decomp(state);
                    n = (*state).x.have;
                    (*state).x.have = 0 as ::core::ffi::c_uint;
                }
            }
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            buf = &mut buf[n as usize..];
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
fn gzread(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let len = buf.len() as ::core::ffi::c_uint;
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    let len = unsafe {
        gz_read(
            state,
            buf,
        ) as ::core::ffi::c_uint
    };
    if len == 0 as ::core::ffi::c_uint {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if state.again != 0 {
            unsafe {
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_ERRNO,
                    Some(::std::ffi::CStr::from_ptr(crate::stdlib::strerror(
                        *crate::stdlib::__errno_location(),
                    ))),
                );
            }
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
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    let buf = if len == 0 {
        &mut []
    } else {
        if buf.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        ::core::slice::from_raw_parts_mut(buf as *mut crate::stdlib::Bytef, len as usize)
    };
    gzread(state, buf)
}
fn gzfread(
    state: &mut crate::gzguts_h::gz_state,
    buf: Result<&mut [crate::stdlib::Bytef], ()>,
    size: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if state.mode != crate::gzguts_h::GZ_READ {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    let Ok(buf) = buf else {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0",
        );
        return 0 as crate::stdlib::z_size_t;
    };
    return if !buf.is_empty() {
        unsafe {
            gz_read(state, buf)
        }
        .wrapping_div(size)
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
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0 as crate::stdlib::z_size_t;
    };
    let buf = match size.checked_mul(nitems) {
        Some(0) => Ok(&mut [][..]),
        Some(len) if !buf.is_null() => Ok(::core::slice::from_raw_parts_mut(
            buf as *mut crate::stdlib::Bytef,
            len,
        )),
        Some(_) | None => Err(()),
    };
    gzfread(state, buf, size)
}
fn gzgetc(state: Option<&mut crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let Some(state) = state else {
        return -1 as ::core::ffi::c_int;
    };
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    return if unsafe { gz_read(state, &mut buf) } < 1 as crate::stdlib::z_size_t
    {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    };
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc((file as crate::gzguts_h::gz_statep).as_mut())
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc((file as crate::gzguts_h::gz_statep).as_mut())
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
    if (*state).err != crate::zlib_h::Z_OK
        && (*state).err != crate::zlib_h::Z_BUF_ERROR
        && (*state).again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(&mut *state, crate::zlib_h::Z_OK, None);
    if (*state).skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).x.have == 0 as ::core::ffi::c_uint {
        (*state).x.have = 1 as ::core::ffi::c_uint;
        (*state).x.next = (*state)
            .out
            .as_mut_ptr()
            .offset(((*state).size << 1 as ::core::ffi::c_int) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        *(*state).x.next.offset(0 as ::core::ffi::c_int as isize) = c as ::core::ffi::c_uchar;
        (*state).x.pos -= 1;
        (*state).past = 0 as ::core::ffi::c_int;
        return c;
    }
    if (*state).x.have == (*state).size << 1 as ::core::ffi::c_int {
        crate::src::gzlib::gz_static_error(
            &mut *state,
            crate::zlib_h::Z_DATA_ERROR,
            b"out of room to push characters\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*state).x.next == (*state).out.as_mut_ptr() {
        let mut src: *mut ::core::ffi::c_uchar = (*state)
            .out
            .as_mut_ptr()
            .offset((*state).x.have as isize);
        let mut dest: *mut ::core::ffi::c_uchar = (*state)
            .out
            .as_mut_ptr()
            .offset(((*state).size << 1 as ::core::ffi::c_int) as isize);
        while src > (*state).out.as_mut_ptr() {
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
unsafe fn gzgets(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [::core::ffi::c_char],
) -> bool {
    if state.mode != crate::gzguts_h::GZ_READ {
        return false;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return false;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return false;
    }
    let mut written = 0usize;
    while written + 1 < buf.len()
        && !(state.x.have == 0 && gz_fetch(state) == -1 as ::core::ffi::c_int)
    {
        if state.x.have == 0 {
            state.past = 1 as ::core::ffi::c_int;
            break;
        }
        let available = (buf.len() - written - 1).min(state.x.have as usize);
        let source = ::core::slice::from_raw_parts(state.x.next, available);
        let count = match source.iter().position(|&byte| byte == b'\n') {
            Some(newline) => newline + 1,
            None => available,
        };
        for (dest, source) in buf[written..written + count].iter_mut().zip(source) {
            *dest = *source as ::core::ffi::c_char;
        }
        state.x.have = state.x.have.wrapping_sub(count as ::core::ffi::c_uint);
        state.x.next = state.x.next.wrapping_add(count);
        state.x.pos += count as crate::stdlib::off64_t;
        written += count;
        if count < available {
            break;
        }
    }
    if written == 0 {
        return false;
    }
    buf[written] = 0;
    true
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    if buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let buf = ::core::slice::from_raw_parts_mut(buf, len as usize);
    if gzgets(state, buf) {
        buf.as_mut_ptr()
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }
}
pub fn gzdirect(state: Option<&mut crate::gzguts_h::gz_state>) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return 0 as ::core::ffi::c_int;
    };
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0 as ::core::ffi::c_uint
    {
        unsafe {
            gz_look(state);
        }
    }
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzdirect((file as crate::gzguts_h::gz_statep).as_mut())
}
pub unsafe extern "C" fn gzclose_r(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
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
        ::core::mem::ManuallyDrop::drop(&mut (*state).out);
        ::core::mem::ManuallyDrop::drop(&mut (*state).in_0);
    }
    err = if (*state).err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_error_state(&mut *state, crate::zlib_h::Z_OK, None);
    ::core::mem::ManuallyDrop::drop(&mut (*state).path);
    ret = crate::stdlib::close((*state).fd);
    drop(Box::from_raw(::core::ptr::slice_from_raw_parts_mut(state, 1)));
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
