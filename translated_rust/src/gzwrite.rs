pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_clamped_uint;
pub use crate::src::gzlib::gz_error;
pub use crate::src::gzlib::gz_io_chunk_limit;
pub use crate::src::gzlib::gz_z_size_to_uInt_chunk;

pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::deflateEnd_ffi;
pub use crate::src::deflate::deflateInit2__ffi;
pub use crate::src::deflate::deflateReset_ffi;
pub use crate::src::deflate::deflate_ffi;
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

unsafe extern "C" fn gz_init(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    (*state).in_0 = crate::stdlib::malloc(
        ((*state).want << 1 as ::core::ffi::c_int) as crate::__stddef_size_t_h::size_t,
    ) as *mut ::core::ffi::c_uchar;
    if (*state).in_0.is_null() {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_MEM_ERROR,
            b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if (*state).direct == 0 {
        (*state).out = crate::stdlib::malloc((*state).want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        if (*state).out.is_null() {
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*strm).zalloc = None;
        (*strm).zfree = None;
        (*strm).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = crate::src::deflate::deflateInit2__ffi(
            strm as *mut crate::zlib_h::z_stream_s,
            (*state).level,
            8 as ::core::ffi::c_int,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (*state).strategy,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
        if ret != crate::zlib_h::Z_OK {
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*strm).next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    }
    (*state).size = (*state).want;
    if (*state).direct == 0 {
        (*strm).avail_out = (*state).size as crate::stdlib::uInt;
        (*strm).next_out = (*state).out as *mut crate::stdlib::Bytef;
        (*state).x.next = (*strm).next_out as *mut ::core::ffi::c_uchar;
    }
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_comp(
    mut state: crate::gzguts_h::gz_statep,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = gz_io_chunk_limit();
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
            gz_note_direct_input_written(&mut *strm, writ);
        }
        return 0 as ::core::ffi::c_int;
    }
    if (*state).reset != 0 {
        if (*strm).avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
            return 0 as ::core::ffi::c_int;
        }
        crate::src::deflate::deflateReset_ffi(strm as *mut crate::zlib_h::z_stream_s);
        (*state).reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if (*strm).avail_out == 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
        {
            while let Some(chunk) = gz_pending_output_chunk(&*state, max) {
                *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                (*state).again = 0 as ::core::ffi::c_int;
                put = chunk;
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
                gz_note_pending_output_written(&mut *state, writ);
            }
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                (*strm).avail_out = (*state).size as crate::stdlib::uInt;
                (*strm).next_out = (*state).out as *mut crate::stdlib::Bytef;
                (*state).x.next = (*state).out;
            }
        }
        have = (*strm).avail_out as ::core::ffi::c_uint;
        ret = crate::src::deflate::deflate_ffi(strm as *mut crate::zlib_h::z_stream_s, flush);
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
        n = gz_clamped_uint((*state).size, (*state).skip);
        if first != 0 {
            let zero_buf = ::core::slice::from_raw_parts_mut((*state).in_0, n as usize);
            gz_fill_zero(zero_buf);
            first = 0 as ::core::ffi::c_int;
        }
        (*strm).avail_in = n as crate::stdlib::uInt;
        (*strm).next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        n = gz_note_input_consumed(&mut *state, n);
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
    if len < (*state).size as crate::stdlib::z_size_t {
        loop {
            let mut have: ::core::ffi::c_uint = 0;
            let mut copy: ::core::ffi::c_uint = 0;
            if (*state).strm.avail_in == 0 as crate::stdlib::uInt {
                (*state).strm.next_in = (*state).in_0 as *mut crate::stdlib::Bytef;
            }
            have = gz_buffered_input_used(&*state);
            copy = (*state).size.wrapping_sub(have);
            if copy as crate::stdlib::z_size_t > len {
                copy = len as ::core::ffi::c_uint;
            }
            crate::stdlib::memcpy(
                (*state).in_0.offset(have as isize) as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                copy as crate::__stddef_size_t_h::size_t,
            );
            gz_note_buffered_input(&mut *state, copy);
            buf =
                (buf as *const ::core::ffi::c_char).offset(copy as isize) as crate::stdlib::voidpc;
            len = len.wrapping_sub(copy as crate::stdlib::z_size_t);
            if len == 0 as crate::stdlib::z_size_t {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return if (*state).again != 0 {
                    put.wrapping_sub(len)
                } else {
                    0 as crate::stdlib::z_size_t
                };
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
            let mut n: ::core::ffi::c_uint = gz_z_size_to_uInt_chunk(len);
            (*state).strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            n = gz_note_input_consumed(&mut *state, n);
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            if ret == -1 as ::core::ffi::c_int {
                return if (*state).again != 0 {
                    put.wrapping_sub(len)
                } else {
                    0 as crate::stdlib::z_size_t
                };
            }
            if !(len != 0) {
                break;
            }
        }
    }
    return put;
}
fn gz_write_state_ready(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_WRITE
        && (state.err == crate::zlib_h::Z_OK || state.again != 0)
}

fn gz_write_params_ready(state: &crate::gzguts_h::gz_state) -> bool {
    gz_write_state_ready(state) && state.direct == 0
}

fn gz_note_buffered_input(state: &mut crate::gzguts_h::gz_state, count: ::core::ffi::c_uint) {
    state.strm.avail_in = state.strm.avail_in.wrapping_add(count);
    state.x.pos += count as crate::stdlib::off64_t;
}

fn gz_note_input_consumed(
    state: &mut crate::gzguts_h::gz_state,
    requested: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let consumed = requested.wrapping_sub(state.strm.avail_in as ::core::ffi::c_uint);
    state.x.pos += consumed as crate::stdlib::off64_t;
    consumed
}

fn gz_note_direct_input_written(strm: &mut crate::zlib_h::z_stream, written: ::core::ffi::c_int) {
    strm.avail_in = strm.avail_in.wrapping_sub(written as ::core::ffi::c_uint);
    strm.next_in = strm.next_in.wrapping_add(written as usize);
}

fn gz_note_pending_output_written(
    state: &mut crate::gzguts_h::gz_state,
    written: ::core::ffi::c_int,
) {
    state.x.next = state.x.next.wrapping_add(written as usize);
}

fn gz_buffered_input_used(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_uint {
    (state.strm.next_in as usize)
        .wrapping_add(state.strm.avail_in as usize)
        .wrapping_sub(state.in_0 as usize) as ::core::ffi::c_uint
}

fn gz_pending_output_chunk(
    state: &crate::gzguts_h::gz_state,
    max: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    let next_out = state.strm.next_out as usize;
    let next = state.x.next as usize;
    if next_out <= next {
        return None;
    }
    let pending = next_out - next;
    Some(if pending > max as usize {
        max
    } else {
        pending as ::core::ffi::c_uint
    })
}

fn gz_fill_zero(buf: &mut [crate::stdlib::Bytef]) {
    buf.fill(0);
}

fn gzwrite_len_fits_int(len: ::core::ffi::c_uint) -> bool {
    (len as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int
}

#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
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
    if !gz_write_state_ready(&*state) {
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
fn gzf_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    size.checked_mul(nitems)
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
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
    if !gz_write_state_ready(&*state) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(len) = gzf_len(size, nitems) else {
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
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
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
    if !gz_write_state_ready(&*state) {
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
        have = gz_buffered_input_used(&*state);
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as ::core::ffi::c_uchar;
            gz_note_buffered_input(&mut *state, 1 as ::core::ffi::c_uint);
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
fn gzputs_len_fits_int(len: crate::stdlib::z_size_t) -> bool {
    (len as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int
        && len as ::core::ffi::c_uint as crate::stdlib::z_size_t == len
}

#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
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
    if !gz_write_state_ready(&*state) {
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
    return if len != 0 && put == 0 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        put as ::core::ffi::c_int
    };
}
fn gzflush_valid_flush(flush: ::core::ffi::c_int) -> bool {
    flush >= 0 as ::core::ffi::c_int && flush <= crate::zlib_h::Z_FINISH
}

#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_write_state_ready(&*state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if !gzflush_valid_flush(flush) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return (*state).err;
    }
    gz_comp(state, flush);
    return (*state).err;
}
fn gzsetparams_unchanged(
    state: &crate::gzguts_h::gz_state,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> bool {
    level == state.level && strategy == state.strategy
}

#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
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
    if !gz_write_params_ready(&*state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if gzsetparams_unchanged(&*state, level, strategy) {
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
        crate::src::deflate::deflateParams_ffi(
            strm as *mut crate::zlib_h::z_stream_s,
            level,
            strategy,
        );
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "gzclose_w"]
pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
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
            crate::src::deflate::deflateEnd_ffi(
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
