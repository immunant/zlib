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
pub use crate::src::deflate::deflateParams;
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

unsafe fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let strm: &mut crate::zlib_h::z_stream = &mut state.strm;
    state.in_0 = crate::stdlib::malloc(
        (state.want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
    ) as *mut ::core::ffi::c_uchar;
    if state.in_0.is_null() {
        crate::src::gzlib::gz_error_state(
            state,
            crate::zlib_h::Z_MEM_ERROR,
            Some(c"out of memory"),
        );
        return -1 as ::core::ffi::c_int;
    }
    if state.direct == 0 {
        let output_len = state.want as usize;
        if state.out.try_reserve_exact(output_len).is_ok() {
            state.out.resize(output_len, 0);
        }
        if state.out.len() != output_len {
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
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
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
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
            *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
            state.again = 0 as ::core::ffi::c_int;
            put = if state.strm.avail_in > max {
                max
            } else {
                state.strm.avail_in as ::core::ffi::c_uint
            };
            writ = crate::stdlib::write(
                state.fd,
                state.strm.next_in as *const ::core::ffi::c_void,
                put as crate::__stddef_size_t_h::size_t,
            ) as ::core::ffi::c_int;
            if writ < 0 as ::core::ffi::c_int {
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() == Some(crate::stdlib::EAGAIN)
                    || error.raw_os_error() == Some(crate::stdlib::EWOULDBLOCK)
                {
                    state.again = 1 as ::core::ffi::c_int;
                }
                let message = std::ffi::CString::new(error.to_string()).ok();
                crate::src::gzlib::gz_error_state(
                    state,
                    crate::zlib_h::Z_ERRNO,
                    message.as_deref(),
                );
                return -1 as ::core::ffi::c_int;
            }
            state.strm.avail_in = state
                .strm
                .avail_in
                .wrapping_sub(writ as ::core::ffi::c_uint);
            state.strm.next_in = state.strm.next_in.wrapping_add(writ as usize);
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
                *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                state.again = 0 as ::core::ffi::c_int;
                let pending = state
                    .strm
                    .next_out
                    .addr()
                    .saturating_sub(state.x.next.addr());
                put = pending.min(max as usize) as ::core::ffi::c_uint;
                writ = crate::stdlib::write(
                    state.fd,
                    state.x.next as *const ::core::ffi::c_void,
                    put as crate::__stddef_size_t_h::size_t,
                ) as ::core::ffi::c_int;
                if writ < 0 as ::core::ffi::c_int {
                    let error = std::io::Error::last_os_error();
                    if error.raw_os_error() == Some(crate::stdlib::EAGAIN)
                        || error.raw_os_error() == Some(crate::stdlib::EWOULDBLOCK)
                    {
                        state.again = 1 as ::core::ffi::c_int;
                    }
                    let message = std::ffi::CString::new(error.to_string()).ok();
                    crate::src::gzlib::gz_error_state(
                        state,
                        crate::zlib_h::Z_ERRNO,
                        message.as_deref(),
                    );
                    return -1 as ::core::ffi::c_int;
                }
                state.x.next = state.x.next.wrapping_add(writ as usize);
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

unsafe fn gz_zero(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let state = &mut *state;
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
            // `gz_init` allocates at least `state.size` bytes for `in_0` before
            // `size` becomes non-zero, and `n` is bounded by that size above.
            ::core::slice::from_raw_parts_mut(state.in_0, n as usize).fill(0);
            first = 0 as ::core::ffi::c_int;
        }
        state.strm.avail_in = n as crate::stdlib::uInt;
        state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
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

unsafe extern "C" fn gz_write(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: crate::stdlib::voidpc,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut put: crate::stdlib::z_size_t = len;
    let mut ret: ::core::ffi::c_int = 0;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.is_null() || buf.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    let input_len = len as usize;
    if input_len as crate::stdlib::z_size_t != len {
        return 0 as crate::stdlib::z_size_t;
    }
    // The caller supplies `len` readable bytes at `buf`.  Convert that pair
    // once, before any cursor arithmetic, so the rest of this routine can
    // track progress as a checked slice offset.
    let input = ::core::slice::from_raw_parts(buf as *const u8, input_len);
    let state = &mut *state;
    if state.size == 0 as ::core::ffi::c_uint
        && gz_init(state) == -1 as ::core::ffi::c_int
    {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0
        && gz_zero(state as *mut crate::gzguts_h::gz_state) == -1 as ::core::ffi::c_int
    {
        return 0 as crate::stdlib::z_size_t;
    }
    let Ok(capacity) = usize::try_from(state.size) else {
        return 0 as crate::stdlib::z_size_t;
    };
    if capacity == 0 || state.in_0.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    let mut consumed = 0usize;
    if input.len() < capacity {
        loop {
            if state.strm.avail_in == 0 as crate::stdlib::uInt {
                state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
            }
            let Some(have) = state
                .strm
                .next_in
                .addr()
                .checked_sub(state.in_0.addr())
                .and_then(|offset| offset.checked_add(state.strm.avail_in as usize))
            else {
                return 0 as crate::stdlib::z_size_t;
            };
            if have > capacity {
                return 0 as crate::stdlib::z_size_t;
            }
            let copy = (capacity - have).min(input.len() - consumed);
            // `have` and `copy` are bounded by `capacity`, which is the
            // initialized input allocation's usable prefix.
            ::core::slice::from_raw_parts_mut(state.in_0, capacity)[have..have + copy]
                .copy_from_slice(&input[consumed..consumed + copy]);
            state.strm.avail_in = state.strm.avail_in.wrapping_add(copy as crate::stdlib::uInt);
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
unsafe fn gzwrite(
    state: &mut crate::gzguts_h::gz_state,
    buf: &[u8],
) -> ::core::ffi::c_int {
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
    gz_write(
        state as *mut crate::gzguts_h::gz_state,
        buf.as_ptr() as crate::stdlib::voidpc,
        len as crate::stdlib::z_size_t,
    ) as ::core::ffi::c_int
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
pub unsafe extern "C" fn gzfwrite(
    mut buf: crate::stdlib::voidpc,
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
    if (*state).mode != crate::gzguts_h::GZ_WRITE
        || (*state).err != crate::zlib_h::Z_OK && (*state).again == 0
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
        gz_write(state as *mut _, buf, len).wrapping_div(size)
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
    gzfwrite(buf, size, nitems, file)
}
pub unsafe extern "C" fn gzputc(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut have: ::core::ffi::c_uint = 0;
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut strm: crate::zlib_h::z_streamp = ::core::ptr::null_mut::<crate::zlib_h::z_stream>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    strm = &raw mut (*state).strm as crate::zlib_h::z_streamp;
    if (*state).mode != crate::gzguts_h::GZ_WRITE
        || (*state).err != crate::zlib_h::Z_OK && (*state).again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *state;
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if (*state).skip != 0 && gz_zero(state as *mut _) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).size != 0 {
        if (*strm).avail_in == 0 as crate::stdlib::uInt {
            (*strm).next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
        }
        have = (*strm)
            .next_in
            .offset((*strm).avail_in as isize)
            .offset_from((*state).in_0) as ::core::ffi::c_uint;
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as ::core::ffi::c_uchar;
            (*strm).avail_in = (*strm).avail_in.wrapping_add(1);
            (*state).x.pos += 1;
            return c & 0xff as ::core::ffi::c_int;
        }
    }
    buf[0 as usize] = c as ::core::ffi::c_uchar;
    if gz_write(
        state as *mut _,
        &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidpc,
        1 as crate::stdlib::z_size_t,
    ) != 1 as crate::stdlib::z_size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    return c & 0xff as ::core::ffi::c_int;
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzputc(file, c)
}
pub unsafe extern "C" fn gzputs(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: crate::stdlib::z_size_t = 0;
    let mut put: crate::stdlib::z_size_t = 0;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_WRITE
        || (*state).err != crate::zlib_h::Z_OK && (*state).again == 0
    {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *state;
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    len = crate::stdlib::strlen(s) as crate::stdlib::z_size_t;
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
    put = gz_write(state as *mut _, s as crate::stdlib::voidpc, len);
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
    gzputs(file, s)
}
pub unsafe extern "C" fn gzflush(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_WRITE
        || (*state).err != crate::zlib_h::Z_OK && (*state).again == 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state;
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if flush < 0 as ::core::ffi::c_int || flush > crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).skip != 0 && gz_zero(state as *mut _) == -1 as ::core::ffi::c_int {
        return (*state).err;
    }
    gz_comp(state, flush);
    return (*state).err;
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzflush(file, flush)
}
pub unsafe extern "C" fn gzsetparams(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut strm: crate::zlib_h::z_streamp = ::core::ptr::null_mut::<crate::zlib_h::z_stream>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    strm = &raw mut (*state).strm as crate::zlib_h::z_streamp;
    if (*state).mode != crate::gzguts_h::GZ_WRITE
        || (*state).err != crate::zlib_h::Z_OK && (*state).again == 0
        || (*state).direct != 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state;
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if level == (*state).level && strategy == (*state).strategy {
        return crate::zlib_h::Z_OK;
    }
    if (*state).skip != 0 && gz_zero(state as *mut _) == -1 as ::core::ffi::c_int {
        return (*state).err;
    }
    if (*state).size != 0 {
        if (*strm).avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
        {
            return (*state).err;
        }
        crate::src::deflate::deflateParams(strm as *mut crate::zlib_h::z_stream_s, level, strategy);
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    gzsetparams(file, level, strategy)
}
pub unsafe extern "C" fn gzclose_w(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_WRITE {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state;
    if (*state).skip != 0 && gz_zero(state as *mut _) == -1 as ::core::ffi::c_int {
        ret = (*state).err;
    }
    if gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
        ret = (*state).err;
    }
    if (*state).size != 0 {
        if (*state).direct == 0 {
            crate::src::deflate::deflateEnd(
                &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            );
        }
        crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    crate::src::gzlib::gz_error_state(state, crate::zlib_h::Z_OK, None);
    if crate::stdlib::close((*state).fd) == -1 as ::core::ffi::c_int {
        ret = crate::zlib_h::Z_ERRNO;
    }
    drop(Box::from_raw(state as *mut _));
    return ret;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_w(file)
}
