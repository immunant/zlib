pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_intmax;
pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::deflate;
pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit2_;
pub use crate::src::deflate::internal_state;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpc;
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
pub use crate::zlib_h::Z_BLOCK;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_ERRNO;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;

fn gz_write_error(state: &mut crate::gzguts_h::gz_state, error: rustix::io::Errno) {
    let message = ::std::ffi::CString::new(::std::io::Error::from(error).to_string()).ok();
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_ERRNO, message.as_deref());
}

fn gz_set_errno(error: rustix::io::Errno) {
    errno::set_errno(errno::Errno(error.raw_os_error()));
}

fn gz_clear_errno() {
    errno::set_errno(errno::Errno(0));
}

fn gz_buffer(len: usize) -> Option<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(len).ok()?;
    buffer.resize(len, 0);
    Some(buffer)
}

fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let strm = &mut state.strm;
    let Some(input_len) = (state.want as usize).checked_mul(2) else {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_MEM_ERROR,
            b"out of memory\0",
        );
        return -1 as ::core::ffi::c_int;
    };
    let Some(input) = gz_buffer(input_len) else {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_MEM_ERROR,
            b"out of memory\0",
        );
        return -1 as ::core::ffi::c_int;
    };
    let mut output = None;
    if state.direct == 0 {
        let Some(buffer) = gz_buffer(state.want as usize) else {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        };
        output = Some(buffer);
        (*strm).zalloc = None;
        (*strm).zfree = None;
        (*strm).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = crate::src::deflate::deflateInit_(
            Some(strm),
            state.level,
            Some(&crate::zlib_h::ZLIB_VERSION[0]),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
            crate::src::deflate::DeflateInitMode::Gzip {
                strategy: state.strategy,
            },
        );
        if ret != crate::zlib_h::Z_OK {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        (*strm).next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    state.in_0 = ::core::mem::ManuallyDrop::new(input);
    if let Some(output) = output {
        state.out = ::core::mem::ManuallyDrop::new(output);
    }
    state.size = state.want;
    if state.direct == 0 {
        strm.avail_out = state.size as crate::stdlib::uInt;
        strm.next_out = state.out.as_mut_ptr() as *mut crate::stdlib::Bytef;
        state.x.next = strm.next_out as *mut ::core::ffi::c_uchar;
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_direct_write(
    fd: &std::os::fd::OwnedFd,
    input: &[u8],
    again: &mut ::core::ffi::c_int,
) -> Result<usize, (usize, rustix::io::Errno)> {
    let max = ((-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2) + 1) as usize;
    let mut written = 0usize;
    while written != input.len() {
        let end = written.saturating_add(max).min(input.len());
        gz_clear_errno();
        *again = 0 as ::core::ffi::c_int;
        match rustix::io::write(fd, &input[written..end]) {
            Ok(count) => written += count,
            Err(error) => {
                if error == rustix::io::Errno::AGAIN || error == rustix::io::Errno::WOULDBLOCK {
                    *again = 1 as ::core::ffi::c_int;
                }
                return Err((written, error));
            }
        }
    }
    Ok(written)
}

fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    if state.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.direct != 0 {
        while state.strm.avail_in != 0 {
            put = if state.strm.avail_in > max {
                max
            } else {
                state.strm.avail_in as ::core::ffi::c_uint
            };
            let Some(cursor) = state
                .strm
                .next_in
                .addr()
                .checked_sub(state.in_0.as_ptr().addr())
            else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal write buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            };
            let Some(end) = cursor.checked_add(put as usize) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal write buffer corrupt\0",
                );
                return -1 as ::core::ffi::c_int;
            };
            let write_result = {
                let Some(fd) = state.fd.as_ref() else {
                    state.again = 0 as ::core::ffi::c_int;
                    gz_write_error(state, rustix::io::Errno::BADF);
                    gz_set_errno(rustix::io::Errno::BADF);
                    return -1 as ::core::ffi::c_int;
                };
                let Some(input) = state.in_0.get(cursor..end) else {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal write buffer corrupt\0",
                    );
                    return -1 as ::core::ffi::c_int;
                };
                gz_direct_write(fd, input, &mut state.again)
            };
            writ = match write_result {
                Ok(written) => written as ::core::ffi::c_int,
                Err((written, error)) => {
                    state.strm.avail_in = state
                        .strm
                        .avail_in
                        .wrapping_sub(written as ::core::ffi::c_uint);
                    state.strm.next_in = state.in_0.as_mut_ptr().wrapping_add(cursor + written);
                    gz_write_error(state, error);
                    gz_set_errno(error);
                    return -1 as ::core::ffi::c_int;
                }
            };
            state.strm.avail_in = state.strm.avail_in.wrapping_sub(writ as ::core::ffi::c_uint);
            state.strm.next_in = state.in_0.as_mut_ptr().wrapping_add(cursor + writ as usize);
        }
        return 0 as ::core::ffi::c_int;
    }
    let strm = &mut state.strm;
    if state.reset != 0 {
        if strm.avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
            return 0 as ::core::ffi::c_int;
        }
        // `strm` is the initialized stream held by this gzip state.
        let reset_state = unsafe {
            &mut *(strm.state as *mut crate::src::deflate::deflate_state)
        };
        if reset_state.head.is_null() {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        let head = unsafe {
            ::core::slice::from_raw_parts_mut(reset_state.head, reset_state.hash_size as usize)
        };
        if crate::src::deflate::deflate_reset(strm, reset_state, head)
            != crate::zlib_h::Z_OK
        {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        state.reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    if state.fd.is_none() {
        state.again = 0 as ::core::ffi::c_int;
        gz_write_error(state, rustix::io::Errno::BADF);
        gz_set_errno(rustix::io::Errno::BADF);
        return -1 as ::core::ffi::c_int;
    }
    // `fd` was checked above and remains owned by `state` for this call.
    let fd = state.fd.as_ref().expect("checked descriptor");
    loop {
        if strm.avail_out == 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
        {
            loop {
                let output_len = state.out.len();
                let output_start = state.out.as_ptr().addr();
                let Some(cursor) = state.x.next.addr().checked_sub(output_start) else {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal write buffer corrupt\0",
                    );
                    return -1 as ::core::ffi::c_int;
                };
                let Some(produced) = strm.next_out.addr().checked_sub(output_start) else {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal write buffer corrupt\0",
                    );
                    return -1 as ::core::ffi::c_int;
                };
                if cursor > produced || produced > output_len {
                    crate::src::gzlib::gz_static_error(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        b"internal write buffer corrupt\0",
                    );
                    return -1 as ::core::ffi::c_int;
                }
                let pending = produced - cursor;
                if pending == 0 {
                    break;
                }
                gz_clear_errno();
                state.again = 0 as ::core::ffi::c_int;
                put = pending.min(max as usize) as ::core::ffi::c_uint;
                // The pending range is bounded by the gzip output buffer.
                let write_result = rustix::io::write(
                    fd,
                    &state.out[cursor..cursor + put as usize],
                );
                writ = match write_result {
                    Ok(written) => written as ::core::ffi::c_int,
                    Err(error) => {
                        if error == rustix::io::Errno::AGAIN
                            || error == rustix::io::Errno::WOULDBLOCK
                        {
                            state.again = 1 as ::core::ffi::c_int;
                        }
                        gz_write_error(state, error);
                        gz_set_errno(error);
                        return -1 as ::core::ffi::c_int;
                    }
                };
                state.x.next = state.out.as_mut_ptr().wrapping_add(cursor + writ as usize);
            }
            if strm.avail_out == 0 as crate::stdlib::uInt {
                strm.avail_out = state.size as crate::stdlib::uInt;
                strm.next_out = state.out.as_mut_ptr() as *mut crate::stdlib::Bytef;
                state.x.next = state.out.as_mut_ptr();
            }
        }
        have = strm.avail_out as ::core::ffi::c_uint;
        // `strm` is the initialized stream held by this gzip state.
        ret = unsafe { crate::src::deflate::deflate(strm, flush) };
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            crate::src::gzlib::gz_static_error(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0",
            );
            return -1 as ::core::ffi::c_int;
        }
        have = have.wrapping_sub(strm.avail_out as ::core::ffi::c_uint);
        if have == 0 {
            break;
        }
    }
    if flush == crate::zlib_h::Z_FINISH {
        state.reset = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_zero(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    if state.strm.avail_in != 0
        && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        n = if ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && state.size > crate::src::gzlib::gz_intmax()
            || state.size as crate::stdlib::off64_t > state.skip
        {
            state.skip as ::core::ffi::c_uint
        } else {
            state.size
        };
        if first != 0 {
            state.in_0[..n as usize].fill(0);
            first = 0 as ::core::ffi::c_int;
        }
        state.strm.avail_in = n as crate::stdlib::uInt;
        state.strm.next_in = state.in_0.as_mut_ptr() as *mut crate::stdlib::Bytef;
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        n = n.wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
        state.x.pos += n as crate::stdlib::off64_t;
        state.skip -= n as crate::stdlib::off64_t;
        if ret == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if state.skip == 0 {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_write(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: &[u8],
) -> crate::stdlib::z_size_t {
    let put = buf.len();
    if buf.is_empty() {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    // Compressed writes must keep any unconsumed input in the state-owned
    // buffer: `gz_comp` may return after a short output write and resume on a
    // later API call.  Only transparent direct writes can use the caller
    // slice synchronously.
    if state.direct == 0 || buf.len() < state.size as crate::stdlib::z_size_t {
        loop {
            if state.strm.avail_in == 0 as crate::stdlib::uInt {
                state.strm.next_in = state.in_0.as_mut_ptr() as *mut crate::stdlib::Bytef;
            }
            let Some(cursor) = state
                .strm
                .next_in
                .addr()
                .checked_sub(state.in_0.as_ptr().addr())
            else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal write buffer corrupt\0",
                );
                return 0 as crate::stdlib::z_size_t;
            };
            let Some(have_len) = cursor.checked_add(state.strm.avail_in as usize) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal write buffer corrupt\0",
                );
                return 0 as crate::stdlib::z_size_t;
            };
            let Some(remaining) = (state.size as usize).checked_sub(have_len) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal write buffer corrupt\0",
                );
                return 0 as crate::stdlib::z_size_t;
            };
            let copy_len = remaining.min(buf.len());
            let Some(end) = have_len.checked_add(copy_len) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal write buffer corrupt\0",
                );
                return 0 as crate::stdlib::z_size_t;
            };
            let Some(in_0) = state.in_0.get_mut(..state.size as usize) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal write buffer corrupt\0",
                );
                return 0 as crate::stdlib::z_size_t;
            };
            let Some(dest) = in_0.get_mut(have_len..end) else {
                crate::src::gzlib::gz_static_error(
                    state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal write buffer corrupt\0",
                );
                return 0 as crate::stdlib::z_size_t;
            };
            for (dest, source) in dest.iter_mut().zip(&buf[..copy_len]) {
                *dest = *source;
            }
            let copy = copy_len as ::core::ffi::c_uint;
            state.strm.avail_in = state.strm.avail_in.wrapping_add(copy);
            state.x.pos += copy as crate::stdlib::off64_t;
            buf = &buf[copy as usize..];
            if buf.is_empty() {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return if state.again != 0 {
                    put.wrapping_sub(buf.len())
                } else {
                    0 as crate::stdlib::z_size_t
                };
            }
        }
    } else {
        if state.strm.avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
        }
        let write_result = {
            let Some(fd) = state.fd.as_ref() else {
                state.again = 0 as ::core::ffi::c_int;
                gz_write_error(state, rustix::io::Errno::BADF);
                gz_set_errno(rustix::io::Errno::BADF);
                return 0 as crate::stdlib::z_size_t;
            };
            gz_direct_write(fd, buf, &mut state.again)
        };
        let written = match write_result {
            Ok(written) => written,
            Err((written, error)) => {
                state.x.pos += written as crate::stdlib::off64_t;
                gz_write_error(state, error);
                gz_set_errno(error);
                return if state.again != 0 {
                    written
                } else {
                    0 as crate::stdlib::z_size_t
                };
            }
        };
        state.x.pos += written as crate::stdlib::off64_t;
        return written;
    }
    return put;
}
fn gzwrite(
    state: &mut crate::gzguts_h::gz_state,
    buf: &[u8],
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if buf.len() > crate::limits_h::INT_MAX as usize {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            b"requested length does not fit in int\0",
        );
        return 0 as ::core::ffi::c_int;
    }
    return gz_write(state, buf) as ::core::ffi::c_int;
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0 as ::core::ffi::c_int;
    };
    let buf = if len == 0 {
        &[]
    } else {
        if buf.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        ::core::slice::from_raw_parts(buf as *const u8, len as usize)
    };
    gzwrite(state, buf)
}
fn gzfwrite(
    state: &mut crate::gzguts_h::gz_state,
    buf: Result<&[u8], ()>,
    mut size: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
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
        gz_write(state, buf).wrapping_div(size)
    } else {
        0 as crate::stdlib::z_size_t
    };
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0 as crate::stdlib::z_size_t;
    };
    let buf = match size.checked_mul(nitems) {
        Some(0) => Ok(&[][..]),
        Some(len) if !buf.is_null() => Ok(::core::slice::from_raw_parts(buf as *const u8, len)),
        Some(_) => Err(()),
        None => Err(()),
    };
    gzfwrite(state, buf, size)
}
fn gzputc(
    state: &mut crate::gzguts_h::gz_state,
    c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let buf = [c as ::core::ffi::c_uchar];
    // `state` is the validated handle supplied by the FFI wrapper, and `buf`
    // lives for the entire synchronous write.
    if gzwrite(state, &buf) != 1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzputc(state, c)
}
fn gzputs(
    state: &mut crate::gzguts_h::gz_state,
    s: &::std::ffi::CStr,
) -> ::core::ffi::c_int {
    let mut len: crate::stdlib::z_size_t = 0;
    let mut put: crate::stdlib::z_size_t = 0;
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    len = s.to_bytes().len();
    if len > crate::limits_h::INT_MAX as usize
        || len as ::core::ffi::c_uint as crate::stdlib::z_size_t != len
    {
        crate::src::gzlib::gz_static_error(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"string length does not fit in int\0",
        );
        return -1 as ::core::ffi::c_int;
    }
    put = gz_write(state, s.to_bytes());
    return if len != 0 && put == 0 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        put as ::core::ffi::c_int
    };
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    if s.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    gzputs(state, ::std::ffi::CStr::from_ptr(s))
}
fn gzflush(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if flush < 0 as ::core::ffi::c_int || flush > crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    gz_comp(state, flush);
    return state.err;
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzflush(state, flush)
}
fn gzsetparams(
    state: &mut crate::gzguts_h::gz_state,
    deflate_state: Option<&mut crate::src::deflate::deflate_state>,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
        || state.direct != 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if level == state.level && strategy == state.strategy {
        return crate::zlib_h::Z_OK;
    }
    if state.skip != 0
        && gz_zero(state) == -1 as ::core::ffi::c_int
    {
        return state.err;
    }
    if state.size != 0 {
        if state.strm.avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
        {
            return state.err;
        }
        let Some(deflate_state) = deflate_state else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let _ = crate::src::deflate::deflateParams(
            &mut state.strm,
            deflate_state,
            level,
            strategy,
        );
    }
    state.level = level;
    state.strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let deflate_state =
        (state.strm.state as *mut crate::src::deflate::deflate_state).as_mut();
    gzsetparams(state, deflate_state, level, strategy)
}
fn gzclose_w_cleanup(state: &mut crate::gzguts_h::gz_state) {
    if state.size != 0 {
        if state.direct == 0 {
            let output = ::core::mem::replace(
                &mut state.out,
                ::core::mem::ManuallyDrop::new(Vec::new()),
            );
            drop(::core::mem::ManuallyDrop::into_inner(output));
        }
        let input = ::core::mem::replace(
            &mut state.in_0,
            ::core::mem::ManuallyDrop::new(Vec::new()),
        );
        drop(::core::mem::ManuallyDrop::into_inner(input));
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    let path = ::core::mem::replace(
        &mut state.path,
        ::core::mem::ManuallyDrop::new(None),
    );
    drop(::core::mem::ManuallyDrop::into_inner(path));
}
pub fn gzclose_w(mut allocation: Box<[crate::gzguts_h::gz_state]>) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    // `gz_open` allocates exactly one state.  Keep the C error path's
    // non-consuming behavior for a mismatched close entry point.
    if allocation.len() != 1 || allocation[0].mode != crate::gzguts_h::GZ_WRITE {
        ::core::mem::forget(allocation);
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let fd = {
        let state = &mut allocation[0];
        if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
            ret = state.err;
        }
        if gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
            ret = state.err;
        }
        if state.size != 0 && state.direct == 0 {
            // The initialized gzip state owns this stream until close.
            unsafe { crate::src::deflate::deflateEnd(&raw mut state.strm) };
        }
        gzclose_w_cleanup(state);
        state.fd.take()
    };
    let close_result = match fd {
        Some(fd) => unsafe { crate::stdlib::close(std::os::fd::IntoRawFd::into_raw_fd(fd)) },
        None => -1,
    };
    if close_result == -1 as ::core::ffi::c_int {
        ret = crate::zlib_h::Z_ERRNO;
    }
    return ret;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let allocation = Box::from_raw(::core::ptr::slice_from_raw_parts_mut(
        file as crate::gzguts_h::gz_statep,
        1,
    ));
    gzclose_w(allocation)
}
