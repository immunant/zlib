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
use std::os::fd::OwnedFd;

fn gz_load_error(state: &mut crate::gzguts_h::gz_state, error: rustix::io::Errno) {
    errno::set_errno(errno::Errno(error.raw_os_error()));
    let message = ::std::ffi::CString::new(::std::io::Error::from(error).to_string()).ok();
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_ERRNO, message.as_deref());
}

fn gz_current_errno_error(state: &mut crate::gzguts_h::gz_state) {
    let error = errno::errno();
    let message =
        ::std::ffi::CString::new(::std::io::Error::from_raw_os_error(error.0).to_string()).ok();
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_ERRNO, message.as_deref());
}

fn gz_load(
    fd: &OwnedFd,
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

fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
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
            let Some(offset) = state.strm.next_in.addr().checked_sub(input.as_ptr().addr()) else {
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
        let Some(fd) = state.fd.as_ref() else {
            gz_load_error(state, rustix::io::Errno::BADF);
            return -1 as ::core::ffi::c_int;
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
        state.strm.avail_in = state
            .strm
            .avail_in
            .wrapping_add(read as ::core::ffi::c_uint);
        state.strm.next_in = state.in_0.as_mut_ptr() as *mut crate::stdlib::Bytef;
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_input_range(state: &crate::gzguts_h::gz_state) -> Option<::core::ops::Range<usize>> {
    let available = state.strm.avail_in as usize;
    if available == 0 {
        return Some(0..0);
    }
    if state.strm.next_in.is_null() {
        return None;
    }
    let offset = state
        .strm
        .next_in
        .addr()
        .checked_sub(state.in_0.as_ptr().addr())?;
    let end = offset.checked_add(available)?;
    (end <= state.in_0.len()).then_some(offset..end)
}

fn gz_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 as ::core::ffi::c_uint {
        let want = state.want as usize;
        let Some(output_len) = want.checked_mul(2) else {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        };
        let mut input = Vec::new();
        let mut output = Vec::new();
        if input.try_reserve_exact(want).is_err() || output.try_reserve_exact(output_len).is_err() {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        input.resize(want, 0);
        output.resize(output_len, 0);
        state.in_0 = ::core::mem::ManuallyDrop::new(input);
        state.out = ::core::mem::ManuallyDrop::new(output);
        state.size = state.want;
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        state.strm.avail_in = 0 as crate::stdlib::uInt;
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit_(
            Some(&mut state.strm),
            Some(&crate::zlib_h::ZLIB_VERSION[0]),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
            crate::src::inflate::InflateInitMode::Gzip,
        ) != crate::zlib_h::Z_OK
        {
            let output =
                ::core::mem::replace(&mut state.out, ::core::mem::ManuallyDrop::new(Vec::new()));
            drop(::core::mem::ManuallyDrop::into_inner(output));
            let input =
                ::core::mem::replace(&mut state.in_0, ::core::mem::ManuallyDrop::new(Vec::new()));
            drop(::core::mem::ManuallyDrop::into_inner(input));
            state.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    let reset_junk =
        if state.direct == -1 as ::core::ffi::c_int || state.junk == 0 as ::core::ffi::c_int {
            (state.junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int
        } else {
            if gz_avail(state) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if state.strm.avail_in == 0 as crate::stdlib::uInt
                || state.again != 0 && state.strm.avail_in < 4 as crate::stdlib::uInt
            {
                return 0 as ::core::ffi::c_int;
            }
            let Some(input) = gz_input_range(state) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            };
            if input.len() > 3
                && state.in_0[input.start] == 31
                && state.in_0[input.start + 1] == 139
                && state.in_0[input.start + 2] == 8
                && state.in_0[input.start + 3] < 32
            {
                1 as ::core::ffi::c_int
            } else {
                let (output, have) = {
                    let Some(output) = state.out.get_mut(..input.len()) else {
                        crate::src::gzlib::gz_static_error(
                            state,
                            crate::zlib_h::Z_STREAM_ERROR,
                            b"internal read buffer corrupt\0",
                        );
                        return -1 as ::core::ffi::c_int;
                    };
                    output.copy_from_slice(&state.in_0[input]);
                    (output.as_mut_ptr(), output.len() as ::core::ffi::c_uint)
                };
                state.x.next = output;
                state.x.have = have;
                state.strm.avail_in = 0 as crate::stdlib::uInt;
                state.how = crate::gzguts_h::COPY;
                return 0 as ::core::ffi::c_int;
            }
        };
    // `inflateInit_` above created this stream.  Check its allocator pair and
    // handle before following the state pointer, so a malformed gzip handle
    // is rejected before the one remaining raw conversion.
    if state.strm.zalloc.is_none() || state.strm.zfree.is_none() || state.strm.state.is_null() {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"internal inflate stream corrupt\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    let inflate_state =
        unsafe { &mut *(state.strm.state as *mut crate::src::inflate::inflate_state) };
    if crate::src::inflate::inflateReset(&mut state.strm, inflate_state) != crate::zlib_h::Z_OK {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"internal inflate stream corrupt\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    state.how = crate::gzguts_h::GZIP;
    state.junk = reset_junk;
    state.direct = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}

fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut had: ::core::ffi::c_uint = 0;
    had = state.strm.avail_out as ::core::ffi::c_uint;
    loop {
        if state.strm.avail_in == 0 as crate::stdlib::uInt
            && gz_avail(state) == -1 as ::core::ffi::c_int
        {
            ret = state.err;
            break;
        } else if state.strm.avail_in == 0 as crate::stdlib::uInt {
            if state.again == 0 {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_BUF_ERROR,
                    b"unexpected end of file\0",
                );
            }
            break;
        } else {
            // `strm` is the initialized stream held by this gzip state.
            ret = crate::src::inflate::inflate(&mut state.strm, crate::zlib_h::Z_NO_FLUSH);
            if state.strm.avail_out < had {
                state.junk = 0 as ::core::ffi::c_int;
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0",
                );
                break;
            } else if ret == crate::zlib_h::Z_MEM_ERROR {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0",
                );
                break;
            } else if ret == crate::zlib_h::Z_DATA_ERROR {
                if state.junk == 1 as ::core::ffi::c_int {
                    state.strm.avail_in = 0 as crate::stdlib::uInt;
                    state.eof = 1 as ::core::ffi::c_int;
                    state.how = crate::gzguts_h::LOOK;
                    ret = crate::zlib_h::Z_OK;
                    break;
                } else {
                    let message =
                        crate::src::inflate::inflate_error_message(&state.strm).or_else(|| {
                            ::std::ffi::CStr::from_bytes_with_nul(b"compressed data error\0").ok()
                        });
                    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_DATA_ERROR, message);
                    break;
                }
            } else if !(state.strm.avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END) {
                break;
            }
        }
    }
    state.x.have =
        (had as crate::stdlib::uInt).wrapping_sub(state.strm.avail_out) as ::core::ffi::c_uint;
    state.x.next =
        state.strm.next_out.wrapping_sub(state.x.have as usize) as *mut ::core::ffi::c_uchar;
    if ret == crate::zlib_h::Z_STREAM_END {
        state.junk = 0 as ::core::ffi::c_int;
        state.how = crate::gzguts_h::LOOK;
        return 0 as ::core::ffi::c_int;
    }
    return if ret != crate::zlib_h::Z_OK {
        -1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}

fn gz_fetch(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
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
                let fd = if state.fd.is_none() {
                    gz_load_error(state, rustix::io::Errno::BADF);
                    return -1 as ::core::ffi::c_int;
                } else {
                    state.fd.as_ref().expect("checked descriptor")
                };
                let Some(output_len) = (state.size as usize).checked_mul(2) else {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal read buffer corrupt\0",
                    );
                    return -1 as ::core::ffi::c_int;
                };
                if state.out.len() != output_len {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal read buffer corrupt\0",
                    );
                    return -1 as ::core::ffi::c_int;
                }
                let output = &mut state.out[..];
                let read = match gz_load(fd, output, &mut state.again, &mut state.eof) {
                    Ok(read) => read,
                    Err((_, error)) => {
                        gz_load_error(state, error);
                        return -1 as ::core::ffi::c_int;
                    }
                };
                state.x.have = read as ::core::ffi::c_uint;
                state.x.next = state.out.as_mut_ptr();
                return 0 as ::core::ffi::c_int;
            }
            crate::gzguts_h::GZIP => {
                let Some(output_len) = (state.size as usize).checked_mul(2) else {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal read buffer corrupt\0",
                    );
                    return -1 as ::core::ffi::c_int;
                };
                if state.out.len() != output_len {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal read buffer corrupt\0",
                    );
                    return -1 as ::core::ffi::c_int;
                }
                state.strm.avail_out =
                    (state.size << 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                state.strm.next_out = state.out.as_mut_ptr() as *mut crate::stdlib::Bytef;
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"state corrupt\0",
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

fn gz_skip(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
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
            let Some(capacity) = (state.size as usize).checked_mul(2) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            };
            let Some(start) = state.x.next.addr().checked_sub(state.out.as_ptr().addr()) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            };
            let Some(end) = start.checked_add(state.x.have as usize) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            };
            if state.out.len() != capacity || end > capacity {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal read buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            }
            state.x.have = state.x.have.wrapping_sub(n);
            state.x.next = state.out.as_mut_ptr().wrapping_add(start + n as usize);
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

fn gz_read(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: &mut [crate::stdlib::Bytef],
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = buf.len() as crate::stdlib::z_size_t;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
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
            if state.x.have != 0 {
                if state.x.have < n {
                    n = state.x.have;
                }
                let Some(capacity) = (state.size as usize).checked_mul(2) else {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal read buffer corrupt\0",
                    );
                    return got;
                };
                let Some(start) = state.x.next.addr().checked_sub(state.out.as_ptr().addr()) else {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal read buffer corrupt\0",
                    );
                    return got;
                };
                let Some(have_end) = start.checked_add(state.x.have as usize) else {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal read buffer corrupt\0",
                    );
                    return got;
                };
                if state.out.len() != capacity || have_end > capacity {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal read buffer corrupt\0",
                    );
                    return got;
                }
                let end = start + n as usize;
                buf[..n as usize].copy_from_slice(&state.out[start..end]);
                state.x.next = state.out.as_mut_ptr().wrapping_add(end);
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
                    if state.fd.is_none() {
                        gz_load_error(state, rustix::io::Errno::BADF);
                        err = -1;
                        n = 0;
                    } else {
                        let fd = state.fd.as_ref().expect("checked descriptor");
                        match gz_load(fd, &mut buf[..n as usize], &mut state.again, &mut state.eof)
                        {
                            Ok(read) => n = read as ::core::ffi::c_uint,
                            Err((read, error)) => {
                                gz_load_error(state, error);
                                err = -1;
                                n = read as ::core::ffi::c_uint;
                            }
                        }
                    }
                } else {
                    state.strm.avail_out = n as crate::stdlib::uInt;
                    state.strm.next_out = buf.as_mut_ptr();
                    err = gz_decomp(state);
                    n = state.x.have;
                    state.x.have = 0 as ::core::ffi::c_uint;
                }
            }
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            buf = &mut buf[n as usize..];
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
    let len = gz_read(state, buf) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if state.again != 0 {
            gz_current_errno_error(state);
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
        gz_read(state, buf).wrapping_div(size)
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
    return if gz_read(state, &mut buf) < 1 as crate::stdlib::z_size_t {
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
fn gzungetc(
    c: ::core::ffi::c_int,
    state: Option<&mut crate::gzguts_h::gz_state>,
) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return -1 as ::core::ffi::c_int;
    };
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
    let Some(capacity) = (state.size as usize).checked_mul(2) else {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"internal read buffer corrupt\0",
        );
        return -1 as ::core::ffi::c_int;
    };
    if state.out.len() != capacity || capacity == 0 {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"internal read buffer corrupt\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    let have = state.x.have as usize;
    if have == capacity {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            b"out of room to push characters\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    let mut start = if have == 0 {
        capacity
    } else {
        let Some(start) = state.x.next.addr().checked_sub(state.out.as_ptr().addr()) else {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal read buffer corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        };
        let Some(end) = start.checked_add(have) else {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal read buffer corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        };
        if end > capacity {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal read buffer corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        if start == 0 {
            let moved = capacity - have;
            state.out.copy_within(0..have, moved);
            moved
        } else {
            start
        }
    };
    start -= 1;
    state.out[start] = c as ::core::ffi::c_uchar;
    state.x.have = state.x.have.wrapping_add(1);
    state.x.next = state.out.as_mut_ptr().wrapping_add(start);
    state.x.pos -= 1;
    state.past = 0 as ::core::ffi::c_int;
    return c;
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    gzungetc(c, (file as crate::gzguts_h::gz_statep).as_mut())
}
fn gzgets(state: &mut crate::gzguts_h::gz_state, buf: &mut [::core::ffi::c_char]) -> bool {
    if state.mode != crate::gzguts_h::GZ_READ {
        return false;
    }
    if state.err != crate::zlib_h::Z_OK
        && state.err != crate::zlib_h::Z_BUF_ERROR
        && state.again == 0
    {
        return false;
    }
    let mut written = 0usize;
    while written + 1 < buf.len() {
        let mut byte = [0u8; 1];
        if gzread(state, &mut byte) != 1 {
            break;
        }
        buf[written] = byte[0] as ::core::ffi::c_char;
        written += 1;
        if byte[0] == b'\n' {
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
        gz_look(state);
    }
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzdirect((file as crate::gzguts_h::gz_statep).as_mut())
}
fn gzclose_r_cleanup(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size != 0 {
        let output =
            ::core::mem::replace(&mut state.out, ::core::mem::ManuallyDrop::new(Vec::new()));
        drop(::core::mem::ManuallyDrop::into_inner(output));
        let input =
            ::core::mem::replace(&mut state.in_0, ::core::mem::ManuallyDrop::new(Vec::new()));
        drop(::core::mem::ManuallyDrop::into_inner(input));
    }
    let err = if state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    let path = ::core::mem::replace(&mut state.path, ::core::mem::ManuallyDrop::new(None));
    drop(::core::mem::ManuallyDrop::into_inner(path));
    err
}
pub fn gzclose_r(mut allocation: Box<[crate::gzguts_h::gz_state]>) -> ::core::ffi::c_int {
    // `gz_open` allocates exactly one state.  Keep the C error path's
    // non-consuming behavior for a mismatched close entry point.
    if allocation.len() != 1 || allocation[0].mode != crate::gzguts_h::GZ_READ {
        ::core::mem::forget(allocation);
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let (fd, err) = {
        let state = &mut allocation[0];
        if state.size != 0 {
            // The initialized gzip state owns this stream until close.
            crate::src::inflate::inflateEnd(&mut state.strm);
        }
        let err = gzclose_r_cleanup(state);
        (state.fd.take(), err)
    };
    let ret = match fd {
        Some(fd) => unsafe { crate::stdlib::close(std::os::fd::IntoRawFd::into_raw_fd(fd)) },
        None => -1,
    };
    return if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    };
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let allocation = Box::from_raw(::core::ptr::slice_from_raw_parts_mut(
        file as crate::gzguts_h::gz_statep,
        1,
    ));
    gzclose_r(allocation)
}
