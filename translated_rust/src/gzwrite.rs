pub use crate::__stddef_null_h::NULL;
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
pub use crate::src::deflate::deflateReset;
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

fn gz_write_fd(
    fd: &std::os::fd::OwnedFd,
    buffer: &[u8],
) -> Result<usize, rustix::io::Errno> {
    rustix::io::write(fd, buffer)
}

unsafe fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let strm: &mut crate::zlib_h::z_stream = &mut state.strm;
    let Some(input_len) = (state.want as usize).checked_mul(2) else {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_MEM_ERROR,
            Some(c"out of memory"),
        );
        return -1 as ::core::ffi::c_int;
    };
    if state.in_0.try_reserve_exact(input_len).is_err() {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_MEM_ERROR,
            Some(c"out of memory"),
        );
        return -1 as ::core::ffi::c_int;
    }
    state.in_0.resize(input_len, 0);
    if state.direct == 0 {
        let output_len = state.want as usize;
        if state.out.try_reserve_exact(output_len).is_ok() {
            state.out.resize(output_len, 0);
        }
        if state.out.len() != output_len {
            state.in_0.clear();
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        }
        strm.zalloc = None;
        strm.zfree = None;
        strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = crate::src::deflate::deflateInit2_(
            strm,
            state.level,
            8 as ::core::ffi::c_int,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            state.strategy,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
        if ret != crate::zlib_h::Z_OK {
            state.in_0.clear();
            state.out.clear();
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_MEM_ERROR,
                Some(c"out of memory"),
            );
            return -1 as ::core::ffi::c_int;
        }
        strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    state.size = state.want;
    if state.direct == 0 {
        strm.avail_out = state.size as crate::stdlib::uInt;
        strm.next_out = state.out.as_mut_ptr();
        state.x.next = strm.next_out as *mut ::core::ffi::c_uchar;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    if state.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.direct != 0 {
        if state.strm.avail_in == 0 {
            return 0 as ::core::ffi::c_int;
        }
        let mut input = ::core::slice::from_raw_parts(
            state.strm.next_in as *const u8,
            state.strm.avail_in as usize,
        );
        while !input.is_empty() {
            state.again = 0 as ::core::ffi::c_int;
            put = if input.len() > max as usize {
                max
            } else {
                input.len() as ::core::ffi::c_uint
            };
            let Some(fd) = state.fd.as_ref() else {
                return -1;
            };
            let written = match gz_write_fd(fd, &input[..put as usize]) {
                Ok(written) => written,
                Err(error) => {
                    if error == rustix::io::Errno::AGAIN
                        || error == rustix::io::Errno::WOULDBLOCK
                    {
                        state.again = 1 as ::core::ffi::c_int;
                    }
                    let error = std::io::Error::from_raw_os_error(error.raw_os_error());
                    let message = std::ffi::CString::new(error.to_string()).ok();
                    crate::src::gzlib::gz_error_state(
                        state,
                        crate::zlib_h::Z_ERRNO,
                        message.as_deref(),
                    );
                    return -1 as ::core::ffi::c_int;
                }
            };
            state.strm.avail_in = state
                .strm
                .avail_in
                .wrapping_sub(written as ::core::ffi::c_uint);
            input = &input[written..];
            state.strm.next_in = input.as_ptr() as *mut crate::stdlib::Bytef;
        }
        return 0 as ::core::ffi::c_int;
    }
    if state.reset != 0 {
        if state.strm.avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
            return 0 as ::core::ffi::c_int;
        }
        crate::src::deflate::deflateReset(&mut state.strm);
        state.reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if state.strm.avail_out == 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
        {
            while state.strm.next_out > state.x.next {
                state.again = 0 as ::core::ffi::c_int;
                let start = state.x.next.addr().checked_sub(state.out.as_ptr().addr());
                let end = state
                    .strm
                    .next_out
                    .addr()
                    .checked_sub(state.out.as_ptr().addr());
                let Some((start, end)) = start
                    .zip(end)
                    .filter(|(start, end)| *start <= *end && *end <= state.out.len())
                else {
                    crate::src::gzlib::gz_error_state(
                        state,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(c"internal error: output buffer corrupt"),
                    );
                    return -1 as ::core::ffi::c_int;
                };
                let pending = end - start;
                put = pending.min(max as usize) as ::core::ffi::c_uint;
                let Some(fd) = state.fd.as_ref() else {
                    return -1;
                };
                let written = match gz_write_fd(fd, &state.out[start..start + put as usize]) {
                    Ok(written) => written,
                    Err(error) => {
                        if error == rustix::io::Errno::AGAIN
                            || error == rustix::io::Errno::WOULDBLOCK
                        {
                            state.again = 1 as ::core::ffi::c_int;
                        }
                        let error = std::io::Error::from_raw_os_error(error.raw_os_error());
                        let message = std::ffi::CString::new(error.to_string()).ok();
                        crate::src::gzlib::gz_error_state(
                            state,
                            crate::zlib_h::Z_ERRNO,
                            message.as_deref(),
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                };
                state.x.next = state.out[start + written..].as_mut_ptr();
            }
            if state.strm.avail_out == 0 as crate::stdlib::uInt {
                state.strm.avail_out = state.size as crate::stdlib::uInt;
                state.strm.next_out = state.out.as_mut_ptr();
                state.x.next = state.out.as_mut_ptr();
            }
        }
        have = state.strm.avail_out as ::core::ffi::c_uint;
        ret = crate::src::deflate::deflate(&mut state.strm, flush);
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            crate::src::gzlib::gz_error_state(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                Some(c"internal error: deflate stream corrupt"),
            );
            return -1 as ::core::ffi::c_int;
        }
        have = have.wrapping_sub(state.strm.avail_out as ::core::ffi::c_uint);
        if have == 0 {
            break;
        }
    }
    if flush == crate::zlib_h::Z_FINISH {
        state.reset = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_zero(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
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
            // `gz_init` sizes this staging buffer before `size` becomes
            // non-zero, and `n` is bounded by that size above.
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

unsafe fn gz_write(
    state: &mut crate::gzguts_h::gz_state,
    input: &[u8],
) -> crate::stdlib::z_size_t {
    let len = input.len();
    let mut put: crate::stdlib::z_size_t = len;
    let mut ret: ::core::ffi::c_int = 0;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    let Ok(capacity) = usize::try_from(state.size) else {
        return 0 as crate::stdlib::z_size_t;
    };
    if capacity == 0 || state.in_0.len() < capacity {
        return 0 as crate::stdlib::z_size_t;
    }
    let mut consumed = 0usize;
    if input.len() < capacity {
        loop {
            if state.strm.avail_in == 0 as crate::stdlib::uInt {
                state.strm.next_in = state.in_0.as_mut_ptr() as *mut crate::stdlib::Bytef;
            }
            let Some(have) = state
                .strm
                .next_in
                .addr()
                .checked_sub(state.in_0.as_ptr().addr())
                .and_then(|offset| offset.checked_add(state.strm.avail_in as usize))
            else {
                return 0 as crate::stdlib::z_size_t;
            };
            if have > capacity {
                return 0 as crate::stdlib::z_size_t;
            }
            let copy = (capacity - have).min(input.len() - consumed);
            state.in_0[have..have + copy].copy_from_slice(&input[consumed..consumed + copy]);
            state.strm.avail_in = state
                .strm
                .avail_in
                .wrapping_add(copy as crate::stdlib::uInt);
            state.x.pos += copy as crate::stdlib::off64_t;
            consumed += copy;
            if consumed == input.len() {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return if state.again != 0 {
                    put.wrapping_sub((input.len() - consumed) as crate::stdlib::z_size_t)
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
        loop {
            let mut n: ::core::ffi::c_uint = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
            let remaining = input.len() - consumed;
            if n as usize > remaining {
                n = remaining as ::core::ffi::c_uint;
            }
            state.strm.next_in = input[consumed..].as_ptr() as *mut crate::stdlib::Bytef;
            state.strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            n = n.wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
            state.x.pos += n as crate::stdlib::off64_t;
            consumed += n as usize;
            if ret == -1 as ::core::ffi::c_int {
                return if state.again != 0 {
                    put.wrapping_sub((input.len() - consumed) as crate::stdlib::z_size_t)
                } else {
                    0 as crate::stdlib::z_size_t
                };
            }
            if consumed == input.len() {
                break;
            }
        }
    }
    return put;
}
unsafe fn gzwrite(state: &mut crate::gzguts_h::gz_state, buf: &[u8]) -> ::core::ffi::c_int {
    let len = buf.len() as ::core::ffi::c_uint;
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            Some(c"requested length does not fit in int"),
        );
        return 0 as ::core::ffi::c_int;
    }
    gz_write(state, buf) as ::core::ffi::c_int
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0;
    };
    let buf = if len == 0 {
        &[]
    } else {
        if buf.is_null() {
            return 0;
        }
        ::core::slice::from_raw_parts(buf as *const u8, len as usize)
    };
    gzwrite(state, buf)
}
unsafe fn gzfwrite(
    state: &mut crate::gzguts_h::gz_state,
    input: Option<&[u8]>,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
    {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    let Some(len) = nitems.checked_mul(size) else {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"request does not fit in a size_t"),
        );
        return 0 as crate::stdlib::z_size_t;
    };
    if len == 0 {
        0 as crate::stdlib::z_size_t
    } else {
        let Some(input) = input.filter(|input| input.len() == len) else {
            return 0;
        };
        gz_write(state, input).wrapping_div(size)
    }
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    buf: crate::stdlib::voidpc,
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
    file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0;
    };
    let input = match nitems.checked_mul(size) {
        None => None,
        Some(0) => Some(&[][..]),
        Some(_) if buf.is_null() => None,
        Some(len) => Some(::core::slice::from_raw_parts(buf as *const u8, len)),
    };
    gzfwrite(state, input, size, nitems)
}
pub unsafe extern "C" fn gzputc(
    state: &mut crate::gzguts_h::gz_state,
    c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    let buf = [c as u8];
    if gzwrite(state, &buf) != 1 {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    file: crate::zlib_h::gzFile,
    c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1;
    };
    gzputc(state, c)
}
unsafe fn gzputs(
    state: &mut crate::gzguts_h::gz_state,
    input: &[u8],
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_WRITE
        || state.err != crate::zlib_h::Z_OK && state.again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    let len = input.len();
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || len as ::core::ffi::c_uint as crate::stdlib::z_size_t != len
    {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            Some(c"string length does not fit in int"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let put = gz_write(state, input);
    return if len != 0 && put == 0 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        put as ::core::ffi::c_int
    };
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    file: crate::zlib_h::gzFile,
    s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if file.is_null() || s.is_null() {
        return -1;
    }
    let input = std::ffi::CStr::from_ptr(s).to_bytes();
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1;
    };
    gzputs(state, input)
}
pub unsafe fn gzflush(
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
    file: crate::zlib_h::gzFile,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzflush(state, flush)
}
fn gzsetparams_impl<Zero, Comp, Params>(
    state: &mut crate::gzguts_h::gz_state,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    mut zero: Zero,
    mut comp: Comp,
    mut params: Params,
) -> ::core::ffi::c_int
where
    Zero: FnMut(&mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int,
    Comp: FnMut(&mut crate::gzguts_h::gz_state, ::core::ffi::c_int) -> ::core::ffi::c_int,
    Params: FnMut(
        &mut crate::zlib_h::z_stream,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
{
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
    if state.skip != 0 && zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    if state.size != 0 {
        if state.strm.avail_in != 0
            && comp(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
        {
            return state.err;
        }
        params(&mut state.strm, level, strategy);
    }
    state.level = level;
    state.strategy = strategy;
    return crate::zlib_h::Z_OK;
}

unsafe fn gzsetparams(
    state: &mut crate::gzguts_h::gz_state,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzsetparams_impl(
        state,
        level,
        strategy,
        |state| gz_zero(state),
        |state, flush| gz_comp(state, flush),
        |strm, level, strategy| crate::src::deflate::deflateParams(strm, level, strategy),
    )
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
    gzsetparams(state, level, strategy)
}
/// Finish the write stream before releasing the already-owned gzip state.
/// The opaque-handle conversion is confined to the exported boundary.
pub unsafe fn gzclose_w(
    mut owned: Box<crate::gzguts_h::gz_state>,
) -> ::core::ffi::c_int {
    let ret = {
        let state: &mut crate::gzguts_h::gz_state = &mut owned;
        let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
        if state.mode != crate::gzguts_h::GZ_WRITE {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
            ret = state.err;
        }
        if gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
            ret = state.err;
        }
        if state.size != 0 {
            if state.direct == 0 {
                crate::src::deflate::deflateEnd(&mut state.strm);
            }
            state.in_0.clear();
        }
        crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
        let close_failed = match state.fd.take() {
            // `OwnedFd` closes exactly once when dropped.  This state owns the
            // descriptor, so no raw descriptor transfer is required here.
            Some(fd) => {
                drop(fd);
                false
            }
            None => true,
        };
        if close_failed {
            ret = crate::zlib_h::Z_ERRNO;
        }
        ret
    };
    ret
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let owned = Box::from_raw(file.cast::<crate::gzguts_h::gz_state>());
    gzclose_w(owned)
}
