pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_error;
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

fn gz_init_core(state: &mut crate::gzguts_h::gz_state) {
    state.size = state.want;
    if state.direct == 0 {
        state.strm.avail_out = state.size as crate::stdlib::uInt;
        state.strm.next_out = state.out as *mut crate::stdlib::Bytef;
        state.x.next = state.strm.next_out as *mut ::core::ffi::c_uchar;
    }
}

fn gz_zero_chunk_len(
    size: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
    int_and_off64_are_same_size: bool,
    int_max: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if (int_and_off64_are_same_size && size > int_max) || size as crate::stdlib::off64_t > skip {
        skip as ::core::ffi::c_uint
    } else {
        size
    }
}

fn gzputs_len_fits_int(len: crate::stdlib::z_size_t) -> bool {
    (len as ::core::ffi::c_int) >= 0 && len as ::core::ffi::c_uint as crate::stdlib::z_size_t == len
}

fn gzwrite_len_fits_int(len: ::core::ffi::c_uint) -> bool {
    (len as ::core::ffi::c_int) >= 0
}

fn gzflush_mode_is_valid(flush: ::core::ffi::c_int) -> bool {
    flush >= 0 && flush <= crate::zlib_h::Z_FINISH
}

fn gzfwrite_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    size.checked_mul(nitems)
}

fn gz_write_error_result(
    again: ::core::ffi::c_int,
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if again != 0 {
        requested.wrapping_sub(remaining)
    } else {
        0
    }
}

fn gz_write_uses_buffered_path(len: crate::stdlib::z_size_t, size: ::core::ffi::c_uint) -> bool {
    len < size as crate::stdlib::z_size_t
}

fn gz_write_chunk_len(remaining: crate::stdlib::z_size_t) -> ::core::ffi::c_uint {
    if ::core::ffi::c_uint::MAX as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        ::core::ffi::c_uint::MAX
    }
}

fn gzputs_result(
    requested: crate::stdlib::z_size_t,
    written: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if requested != 0 && written == 0 {
        -1
    } else {
        written as ::core::ffi::c_int
    }
}

unsafe extern "C" fn gz_init(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let state = &mut *state;
    state.in_0 = crate::stdlib::malloc(
        (state.want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
    ) as *mut ::core::ffi::c_uchar;
    if state.in_0.is_null() {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_MEM_ERROR,
            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if state.direct == 0 {
        state.out = crate::stdlib::malloc(state.want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        if state.out.is_null() {
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if crate::src::deflate::deflateInit2_(
            &mut state.strm as *mut crate::zlib_h::z_stream_s,
            state.level,
            8 as ::core::ffi::c_int,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            state.strategy,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
            crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    gz_init_core(state);
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn gz_comp(
    mut state: crate::gzguts_h::gz_statep,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).direct != 0 {
        while (*strm).avail_in != 0 {
            *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
            (*state).again = 0 as ::core::ffi::c_int;
            put = if (*strm).avail_in > max {
                max
            } else {
                (*strm).avail_in as ::core::ffi::c_uint
            };
            writ = crate::stdlib::write(
                (*state).fd,
                (*strm).next_in as *const ::core::ffi::c_void,
                put as crate::__stddef_size_t_h::size_t,
            ) as ::core::ffi::c_int;
            if writ < 0 as ::core::ffi::c_int {
                if *crate::stdlib::__errno_location() == crate::stdlib::EAGAIN
                    || *crate::stdlib::__errno_location() == crate::stdlib::EWOULDBLOCK
                {
                    (*state).again = 1 as ::core::ffi::c_int;
                }
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_ERRNO,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                return -1 as ::core::ffi::c_int;
            }
            (*strm).avail_in = (*strm).avail_in.wrapping_sub(writ as ::core::ffi::c_uint);
            (*strm).next_in = (*strm).next_in.offset(writ as isize);
        }
        return 0 as ::core::ffi::c_int;
    }
    if (*state).reset != 0 {
        if (*strm).avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
            return 0 as ::core::ffi::c_int;
        }
        crate::src::deflate::deflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if (*strm).avail_out == 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
        {
            while (*strm).next_out > (*state).x.next {
                *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                (*state).again = 0 as ::core::ffi::c_int;
                put = if (*strm).next_out.offset_from((*state).x.next) as ::core::ffi::c_long
                    > max as ::core::ffi::c_int as ::core::ffi::c_long
                {
                    max
                } else {
                    (*strm).next_out.offset_from((*state).x.next) as ::core::ffi::c_long
                        as ::core::ffi::c_uint
                };
                writ = crate::stdlib::write(
                    (*state).fd,
                    (*state).x.next as *const ::core::ffi::c_void,
                    put as crate::__stddef_size_t_h::size_t,
                ) as ::core::ffi::c_int;
                if writ < 0 as ::core::ffi::c_int {
                    if *crate::stdlib::__errno_location() == crate::stdlib::EAGAIN
                        || *crate::stdlib::__errno_location() == crate::stdlib::EWOULDBLOCK
                    {
                        (*state).again = 1 as ::core::ffi::c_int;
                    }
                    crate::src::gzlib::gz_error(
                        state as *mut crate::gzguts_h::gz_state,
                        crate::zlib_h::Z_ERRNO,
                        crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                    );
                    return -1 as ::core::ffi::c_int;
                }
                (*state).x.next = (*state).x.next.offset(writ as isize);
            }
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                (*strm).avail_out = (*state).size as crate::stdlib::uInt;
                (*strm).next_out = (*state).out as *mut crate::stdlib::Bytef;
                (*state).x.next = (*state).out;
            }
        }
        have = (*strm).avail_out as ::core::ffi::c_uint;
        ret = crate::src::deflate::deflate(strm as *mut crate::zlib_h::z_stream_s, flush);
        if ret == crate::zlib_h::Z_STREAM_ERROR {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        have = have.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
        if !(have != 0) {
            break;
        }
    }
    if flush == crate::zlib_h::Z_FINISH {
        (*state).reset = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_zero(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*strm).avail_in != 0
        && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        n = gz_zero_chunk_len(
            (*state).size,
            (*state).skip,
            ::core::mem::size_of::<::core::ffi::c_int>()
                == ::core::mem::size_of::<crate::stdlib::off64_t>(),
            crate::src::gzlib::gz_intmax(),
        );
        if first != 0 {
            crate::stdlib::memset(
                (*state).in_0 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                n as crate::__stddef_size_t_h::size_t,
            );
            first = 0 as ::core::ffi::c_int;
        }
        (*strm).avail_in = n as crate::stdlib::uInt;
        (*strm).next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        n = n.wrapping_sub((*strm).avail_in as ::core::ffi::c_uint);
        (*state).x.pos += n as crate::stdlib::off64_t;
        (*state).skip -= n as crate::stdlib::off64_t;
        if ret == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if !((*state).skip != 0) {
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
    if (*state).size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if gz_write_uses_buffered_path(len, (*state).size) {
        loop {
            let mut have: ::core::ffi::c_uint = 0;
            let mut copy: ::core::ffi::c_uint = 0;
            if (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                (*state).strm.next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
            }
            have = (*state)
                .strm
                .next_in
                .offset((*state).strm.avail_in as isize)
                .offset_from((*state).in_0) as ::core::ffi::c_long
                as ::core::ffi::c_uint;
            copy = (*state).size.wrapping_sub(have);
            if copy as crate::stdlib::z_size_t > len {
                copy = len as ::core::ffi::c_uint;
            }
            crate::stdlib::memcpy(
                (*state).in_0.offset(have as isize) as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                copy as crate::__stddef_size_t_h::size_t,
            );
            (*state).strm.avail_in = (*state).strm.avail_in.wrapping_add(copy);
            (*state).x.pos += copy as crate::stdlib::off64_t;
            buf =
                (buf as *const ::core::ffi::c_char).offset(copy as isize) as crate::stdlib::voidpc;
            len = len.wrapping_sub(copy as crate::stdlib::z_size_t);
            if len == 0 as crate::stdlib::z_size_t {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return gz_write_error_result((*state).again, put, len);
            }
        }
    } else {
        if (*state).strm.avail_in != 0
            && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
        }
        (*state).strm.next_in = buf as *mut crate::stdlib::Bytef;
        loop {
            let mut n = gz_write_chunk_len(len);
            (*state).strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            n = n.wrapping_sub((*state).strm.avail_in as ::core::ffi::c_uint);
            (*state).x.pos += n as crate::stdlib::off64_t;
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            if ret == -1 as ::core::ffi::c_int {
                return gz_write_error_result((*state).again, put, len);
            }
            if !(len != 0) {
                break;
            }
        }
    }
    return put;
}
pub unsafe extern "C" fn gzwrite(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_WRITE
        || (*state).err != crate::zlib_h::Z_OK && (*state).again == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !gzwrite_len_fits_int(len) {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_DATA_ERROR,
            b"requested length does not fit in int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return gz_write(state, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_int;
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzwrite(file, buf, len)
}
pub unsafe extern "C" fn gzfwrite(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
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
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(len) = gzfwrite_len(size, nitems) else {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    };
    return if len != 0 {
        gz_write(state, buf, len).wrapping_div(size)
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
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).size != 0 {
        if (*strm).avail_in == 0 as crate::stdlib::uInt {
            (*strm).next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
        }
        have = (*strm)
            .next_in
            .offset((*strm).avail_in as isize)
            .offset_from((*state).in_0) as ::core::ffi::c_long
            as ::core::ffi::c_uint;
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as ::core::ffi::c_uchar;
            (*strm).avail_in = (*strm).avail_in.wrapping_add(1);
            (*state).x.pos += 1;
            return c & 0xff as ::core::ffi::c_int;
        }
    }
    buf[0 as ::core::ffi::c_int as usize] = c as ::core::ffi::c_uchar;
    if gz_write(
        state,
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
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    len = crate::stdlib::strlen(s) as crate::stdlib::z_size_t;
    if !gzputs_len_fits_int(len) {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"string length does not fit in int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    put = gz_write(state, s as crate::stdlib::voidpc, len);
    return gzputs_result(len, put);
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
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !gzflush_mode_is_valid(flush) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
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
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if level == (*state).level && strategy == (*state).strategy {
        return crate::zlib_h::Z_OK;
    }
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
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
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
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
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
        }
        crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
    if crate::stdlib::close((*state).fd) == -1 as ::core::ffi::c_int {
        ret = crate::zlib_h::Z_ERRNO;
    }
    crate::stdlib::free(state as *mut ::core::ffi::c_void);
    return ret;
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_w(file)
}

#[cfg(test)]
mod tests {
    use super::{
        gz_write_chunk_len, gz_write_error_result, gz_write_uses_buffered_path, gz_zero_chunk_len,
        gzflush_mode_is_valid, gzfwrite_len, gzputs_len_fits_int, gzputs_result,
        gzwrite_len_fits_int,
    };

    #[test]
    fn gz_zero_chunk_len_limits_to_remaining_skip() {
        assert_eq!(gz_zero_chunk_len(1024, 99, false, 0), 99);
    }

    #[test]
    fn gz_zero_chunk_len_uses_buffer_size_when_skip_is_sufficient() {
        assert_eq!(gz_zero_chunk_len(1024, 1024, false, 0), 1024);
        assert_eq!(gz_zero_chunk_len(1024, 2048, false, 0), 1024);
    }

    #[test]
    fn gz_zero_chunk_len_limits_large_buffers_on_matching_widths() {
        assert_eq!(gz_zero_chunk_len(1024, 4096, true, 1023), 4096);
    }

    #[test]
    fn gz_zero_chunk_len_ignores_int_limit_on_different_widths() {
        assert_eq!(gz_zero_chunk_len(1024, 4096, false, 1023), 1024);
    }

    #[test]
    fn gzputs_len_fits_int_accepts_c_int_range() {
        assert!(gzputs_len_fits_int(0));
        assert!(gzputs_len_fits_int(1));
        assert!(gzputs_len_fits_int(
            ::core::ffi::c_int::MAX as crate::stdlib::z_size_t
        ));
    }

    #[test]
    fn gzputs_len_fits_int_rejects_values_outside_c_int_range() {
        assert!(!gzputs_len_fits_int(
            (::core::ffi::c_int::MAX as crate::stdlib::z_size_t) + 1
        ));
    }

    #[test]
    fn gzputs_result_allows_empty_writes() {
        assert_eq!(gzputs_result(0, 0), 0);
    }

    #[test]
    fn gzputs_result_reports_nonempty_write_failures() {
        assert_eq!(gzputs_result(1, 0), -1);
    }

    #[test]
    fn gzputs_result_returns_written_count() {
        assert_eq!(gzputs_result(5, 5), 5);
        assert_eq!(gzputs_result(5, 3), 3);
    }

    #[test]
    fn gzwrite_len_fits_int_accepts_c_int_range() {
        assert!(gzwrite_len_fits_int(0));
        assert!(gzwrite_len_fits_int(1));
        assert!(gzwrite_len_fits_int(
            ::core::ffi::c_int::MAX as ::core::ffi::c_uint
        ));
    }

    #[test]
    fn gzwrite_len_fits_int_rejects_values_outside_c_int_range() {
        assert!(!gzwrite_len_fits_int(
            (::core::ffi::c_int::MAX as ::core::ffi::c_uint) + 1
        ));
        assert!(!gzwrite_len_fits_int(::core::ffi::c_uint::MAX));
    }

    #[test]
    fn gzflush_mode_is_valid_accepts_supported_range() {
        assert!(gzflush_mode_is_valid(0));
        assert!(gzflush_mode_is_valid(crate::zlib_h::Z_FINISH));
    }

    #[test]
    fn gzflush_mode_is_valid_rejects_values_outside_supported_range() {
        assert!(!gzflush_mode_is_valid(-1));
        assert!(!gzflush_mode_is_valid(crate::zlib_h::Z_FINISH + 1));
    }

    #[test]
    fn gzfwrite_len_returns_requested_byte_count() {
        assert_eq!(gzfwrite_len(4, 3), Some(12));
        assert_eq!(gzfwrite_len(0, crate::stdlib::z_size_t::MAX), Some(0));
    }

    #[test]
    fn gzfwrite_len_rejects_overflow() {
        assert_eq!(gzfwrite_len(crate::stdlib::z_size_t::MAX, 2), None);
    }

    #[test]
    fn gz_write_error_result_returns_partial_count_when_retryable() {
        assert_eq!(gz_write_error_result(1, 10, 4), 6);
        assert_eq!(gz_write_error_result(-1, 10, 4), 6);
    }

    #[test]
    fn gz_write_error_result_discards_partial_count_when_not_retryable() {
        assert_eq!(gz_write_error_result(0, 10, 4), 0);
    }

    #[test]
    fn gz_write_uses_buffered_path_only_for_short_writes() {
        assert!(gz_write_uses_buffered_path(0, 1));
        assert!(gz_write_uses_buffered_path(1023, 1024));
        assert!(!gz_write_uses_buffered_path(1024, 1024));
        assert!(!gz_write_uses_buffered_path(1025, 1024));
    }

    #[test]
    fn gz_write_chunk_len_caps_input_at_c_uint_max() {
        assert_eq!(gz_write_chunk_len(0), 0);
        assert_eq!(gz_write_chunk_len(123), 123);
        assert_eq!(
            gz_write_chunk_len(::core::ffi::c_uint::MAX as crate::stdlib::z_size_t),
            ::core::ffi::c_uint::MAX
        );
    }

    #[test]
    fn gz_write_chunk_len_handles_sizes_above_c_uint_max() {
        if crate::stdlib::z_size_t::MAX > ::core::ffi::c_uint::MAX as crate::stdlib::z_size_t {
            assert_eq!(
                gz_write_chunk_len((::core::ffi::c_uint::MAX as crate::stdlib::z_size_t) + 1),
                ::core::ffi::c_uint::MAX
            );
        }
    }
}
