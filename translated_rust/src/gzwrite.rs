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

fn gz_zero_needs_initialization(first: ::core::ffi::c_int) -> bool {
    first != 0
}

fn gzputs_len_fits_int(len: crate::stdlib::z_size_t) -> bool {
    (len as ::core::ffi::c_int) >= 0 && len as ::core::ffi::c_uint as crate::stdlib::z_size_t == len
}

fn gzwrite_len_fits_int(len: ::core::ffi::c_uint) -> bool {
    (len as ::core::ffi::c_int) >= 0
}

fn gz_write_state_is_usable(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> bool {
    mode == crate::gzguts_h::GZ_WRITE && (err == crate::zlib_h::Z_OK || again != 0)
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

fn gz_write_needs_pending_flush(avail_in: crate::stdlib::uInt) -> bool {
    avail_in != 0
}

fn gz_zero_needs_pending_flush(avail_in: crate::stdlib::uInt) -> bool {
    avail_in != 0
}

fn gz_write_buffered_copy_len(
    size: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    remaining: crate::stdlib::z_size_t,
) -> ::core::ffi::c_uint {
    let available = size.wrapping_sub(have);
    if available as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        available
    }
}

fn gz_buffered_have(
    buffer_address: usize,
    next_in_address: usize,
    avail_in: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    next_in_address
        .wrapping_sub(buffer_address)
        .wrapping_add(avail_in as usize) as ::core::ffi::c_uint
}

fn gz_write_chunk_len(remaining: crate::stdlib::z_size_t) -> ::core::ffi::c_uint {
    if ::core::ffi::c_uint::MAX as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        ::core::ffi::c_uint::MAX
    }
}

fn gz_write_chunk_consumed_len(
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    chunk_len.wrapping_sub(remaining_avail_in as ::core::ffi::c_uint)
}

fn gz_write_apply_direct_progress(
    pos: &mut crate::stdlib::off64_t,
    remaining: &mut crate::stdlib::z_size_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
) -> bool {
    let consumed = gz_write_chunk_consumed_len(chunk_len, remaining_avail_in);
    *pos += consumed as crate::stdlib::off64_t;
    *remaining = remaining.wrapping_sub(consumed as crate::stdlib::z_size_t);
    *remaining != 0
}

fn gz_zero_apply_progress(
    pos: &mut crate::stdlib::off64_t,
    skip: &mut crate::stdlib::off64_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
) -> bool {
    let consumed = gz_write_chunk_consumed_len(chunk_len, remaining_avail_in);
    *pos += consumed as crate::stdlib::off64_t;
    *skip -= consumed as crate::stdlib::off64_t;
    *skip != 0
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

fn gzputc_result(c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    c & 0xff as ::core::ffi::c_int
}

fn gz_comp_needs_output_write(
    avail_out: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> bool {
    avail_out == 0
        || flush != crate::zlib_h::Z_NO_FLUSH
            && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
}

fn gz_comp_needs_output_buffer_reset(avail_out: crate::stdlib::uInt) -> bool {
    avail_out == 0
}

fn gz_comp_needs_reset(avail_in: crate::stdlib::uInt, flush: ::core::ffi::c_int) -> bool {
    avail_in != 0 || flush != crate::zlib_h::Z_NO_FLUSH
}

fn gz_comp_skips_empty_flush(
    reset: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> bool {
    reset != 0 && avail_in == 0 && flush == crate::zlib_h::Z_NO_FLUSH
}

fn gz_comp_reset_action(
    reset: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if gz_comp_skips_empty_flush(reset, avail_in, flush) {
        -1
    } else if reset != 0 {
        1
    } else {
        0
    }
}

fn gz_comp_reset_after_flush(
    flush: ::core::ffi::c_int,
    current_reset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if flush == crate::zlib_h::Z_FINISH {
        1
    } else {
        current_reset
    }
}

fn gz_comp_max_write_chunk() -> ::core::ffi::c_uint {
    (-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint)
}

fn gz_write_errno_is_retryable(errno: ::core::ffi::c_int) -> bool {
    errno == crate::stdlib::EAGAIN || errno == crate::stdlib::EWOULDBLOCK
}

fn gz_comp_write_chunk_len(available: usize, max: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    if available > max as usize {
        max
    } else {
        available as ::core::ffi::c_uint
    }
}

fn gz_comp_remaining_direct_input(
    avail_in: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
) -> crate::stdlib::uInt {
    avail_in.wrapping_sub(written as crate::stdlib::uInt)
}

fn gz_comp_output_produced(
    avail_out_before: ::core::ffi::c_uint,
    avail_out_after: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    avail_out_before.wrapping_sub(avail_out_after)
}

fn gz_write_buffered_progress(
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
    copy: ::core::ffi::c_uint,
) -> (crate::stdlib::off64_t, crate::stdlib::z_size_t) {
    (
        pos + copy as crate::stdlib::off64_t,
        remaining.wrapping_sub(copy as crate::stdlib::z_size_t),
    )
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
    let mut max: ::core::ffi::c_uint = gz_comp_max_write_chunk();
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if (*state).direct != 0 {
        while (*strm).avail_in != 0 {
            *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
            (*state).again = 0 as ::core::ffi::c_int;
            put = gz_comp_write_chunk_len((*strm).avail_in as usize, max);
            writ = crate::stdlib::write(
                (*state).fd,
                (*strm).next_in as *const ::core::ffi::c_void,
                put as crate::__stddef_size_t_h::size_t,
            ) as ::core::ffi::c_int;
            if writ < 0 as ::core::ffi::c_int {
                if gz_write_errno_is_retryable(*crate::stdlib::__errno_location()) {
                    (*state).again = 1 as ::core::ffi::c_int;
                }
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_ERRNO,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                return -1 as ::core::ffi::c_int;
            }
            (*strm).avail_in = gz_comp_remaining_direct_input((*strm).avail_in, writ);
            (*strm).next_in = (*strm).next_in.offset(writ as isize);
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut reset = (*state).reset;
    let reset_action = gz_comp_reset_action(reset, (*strm).avail_in, flush);
    if reset_action < 0 {
        return 0 as ::core::ffi::c_int;
    }
    if reset_action != 0 {
        crate::src::deflate::deflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).reset = 0 as ::core::ffi::c_int;
        reset = 0 as ::core::ffi::c_int;
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if gz_comp_needs_output_write((*strm).avail_out, flush, ret) {
            while (*strm).next_out > (*state).x.next {
                *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                (*state).again = 0 as ::core::ffi::c_int;
                put = gz_comp_write_chunk_len(
                    (*strm).next_out.offset_from((*state).x.next) as usize,
                    max,
                );
                writ = crate::stdlib::write(
                    (*state).fd,
                    (*state).x.next as *const ::core::ffi::c_void,
                    put as crate::__stddef_size_t_h::size_t,
                ) as ::core::ffi::c_int;
                if writ < 0 as ::core::ffi::c_int {
                    if gz_write_errno_is_retryable(*crate::stdlib::__errno_location()) {
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
            if gz_comp_needs_output_buffer_reset((*strm).avail_out) {
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
        have = gz_comp_output_produced(have, (*strm).avail_out as ::core::ffi::c_uint);
        if !(have != 0) {
            break;
        }
    }
    (*state).reset = gz_comp_reset_after_flush(flush, reset);
    return 0 as ::core::ffi::c_int;
}

unsafe extern "C" fn gz_zero(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if gz_zero_needs_pending_flush((*strm).avail_in)
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
        if gz_zero_needs_initialization(first) {
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
        let remaining_avail_in = (*strm).avail_in;
        let has_skip = gz_zero_apply_progress(
            &mut (*state).x.pos,
            &mut (*state).skip,
            n,
            remaining_avail_in,
        );
        if ret == -1 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if !has_skip {
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
            have = gz_buffered_have(
                (*state).in_0 as usize,
                (*state).strm.next_in as usize,
                (*state).strm.avail_in,
            );
            copy = gz_write_buffered_copy_len((*state).size, have, len);
            crate::stdlib::memcpy(
                (*state).in_0.offset(have as isize) as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                copy as crate::__stddef_size_t_h::size_t,
            );
            (*state).strm.avail_in = (*state).strm.avail_in.wrapping_add(copy);
            let (next_pos, next_len) = gz_write_buffered_progress((*state).x.pos, len, copy);
            (*state).x.pos = next_pos;
            buf =
                (buf as *const ::core::ffi::c_char).offset(copy as isize) as crate::stdlib::voidpc;
            len = next_len;
            if len == 0 as crate::stdlib::z_size_t {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return gz_write_error_result((*state).again, put, len);
            }
        }
    } else {
        if gz_write_needs_pending_flush((*state).strm.avail_in)
            && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
        }
        (*state).strm.next_in = buf as *mut crate::stdlib::Bytef;
        loop {
            let n = gz_write_chunk_len(len);
            (*state).strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            let has_remaining = gz_write_apply_direct_progress(
                &mut (*state).x.pos,
                &mut len,
                n,
                (*state).strm.avail_in,
            );
            if ret == -1 as ::core::ffi::c_int {
                return gz_write_error_result((*state).again, put, len);
            }
            if !has_remaining {
                break;
            }
        }
    }
    return put;
}

fn gzsetparams_settings_match(
    requested_level: ::core::ffi::c_int,
    current_level: ::core::ffi::c_int,
    requested_strategy: ::core::ffi::c_int,
    current_strategy: ::core::ffi::c_int,
) -> bool {
    requested_level == current_level && requested_strategy == current_strategy
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
    if !gz_write_state_is_usable((*state).mode, (*state).err, (*state).again) {
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
        have = gz_buffered_have(
            (*state).in_0 as usize,
            (*strm).next_in as usize,
            (*strm).avail_in,
        );
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as ::core::ffi::c_uchar;
            (*strm).avail_in = (*strm).avail_in.wrapping_add(1);
            (*state).x.pos += 1;
            return gzputc_result(c);
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
    return gzputc_result(c);
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
    if gzsetparams_settings_match(level, (*state).level, strategy, (*state).strategy) {
        return crate::zlib_h::Z_OK;
    }
    if (*state).skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return (*state).err;
    }
    if (*state).size != 0 {
        if gz_write_needs_pending_flush((*strm).avail_in)
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
        gz_buffered_have, gz_comp_max_write_chunk, gz_comp_needs_output_buffer_reset,
        gz_comp_needs_output_write, gz_comp_needs_reset, gz_comp_output_produced,
        gz_comp_remaining_direct_input, gz_comp_reset_action, gz_comp_reset_after_flush,
        gz_comp_skips_empty_flush, gz_comp_write_chunk_len, gz_write_apply_direct_progress,
        gz_write_buffered_copy_len, gz_write_buffered_progress, gz_write_chunk_consumed_len,
        gz_write_chunk_len, gz_write_errno_is_retryable, gz_write_error_result,
        gz_write_needs_pending_flush, gz_write_state_is_usable, gz_write_uses_buffered_path,
        gz_zero_apply_progress, gz_zero_chunk_len, gz_zero_needs_initialization,
        gz_zero_needs_pending_flush, gzflush_mode_is_valid, gzfwrite_len, gzputc_result,
        gzputs_len_fits_int, gzputs_result, gzsetparams_settings_match, gzwrite_len_fits_int,
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
    fn gz_zero_needs_initialization_only_on_the_first_pass() {
        assert!(gz_zero_needs_initialization(1));
        assert!(gz_zero_needs_initialization(-1));
        assert!(!gz_zero_needs_initialization(0));
    }

    #[test]
    fn gz_zero_needs_pending_flush_only_for_buffered_input() {
        assert!(!gz_zero_needs_pending_flush(0));
        assert!(gz_zero_needs_pending_flush(1));
        assert!(gz_zero_needs_pending_flush(crate::stdlib::uInt::MAX));
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
    fn gzsetparams_settings_match_requires_both_settings_to_match() {
        assert!(gzsetparams_settings_match(1, 1, 2, 2));
        assert!(!gzsetparams_settings_match(1, 2, 2, 2));
        assert!(!gzsetparams_settings_match(1, 1, 2, 3));
        assert!(!gzsetparams_settings_match(1, 2, 3, 4));
    }

    #[test]
    fn gzsetparams_settings_match_compares_extreme_values_exactly() {
        assert!(gzsetparams_settings_match(
            ::core::ffi::c_int::MIN,
            ::core::ffi::c_int::MIN,
            ::core::ffi::c_int::MAX,
            ::core::ffi::c_int::MAX
        ));
        assert!(!gzsetparams_settings_match(
            ::core::ffi::c_int::MIN,
            ::core::ffi::c_int::MAX,
            ::core::ffi::c_int::MAX,
            ::core::ffi::c_int::MAX
        ));
    }

    #[test]
    fn gzputs_result_returns_written_count() {
        assert_eq!(gzputs_result(5, 5), 5);
        assert_eq!(gzputs_result(5, 3), 3);
    }

    #[test]
    fn gzputc_result_returns_the_low_byte() {
        assert_eq!(gzputc_result(0), 0);
        assert_eq!(gzputc_result(0x7f), 0x7f);
        assert_eq!(gzputc_result(0x123), 0x23);
        assert_eq!(gzputc_result(-1), 0xff);
    }

    #[test]
    fn gz_write_errno_is_retryable_for_nonblocking_write_errors() {
        assert!(gz_write_errno_is_retryable(crate::stdlib::EAGAIN));
        assert!(gz_write_errno_is_retryable(crate::stdlib::EWOULDBLOCK));
        assert!(!gz_write_errno_is_retryable(0));
        assert!(!gz_write_errno_is_retryable(1));
    }

    #[test]
    fn gz_comp_needs_output_write_when_output_buffer_is_full() {
        assert!(gz_comp_needs_output_write(
            0,
            crate::zlib_h::Z_NO_FLUSH,
            crate::zlib_h::Z_OK
        ));
    }

    #[test]
    fn gz_comp_needs_output_buffer_reset_when_buffer_is_exhausted() {
        assert!(gz_comp_needs_output_buffer_reset(0));
    }

    #[test]
    fn gz_comp_needs_output_buffer_reset_preserves_available_buffer() {
        assert!(!gz_comp_needs_output_buffer_reset(1));
        assert!(!gz_comp_needs_output_buffer_reset(crate::stdlib::uInt::MAX));
    }

    #[test]
    fn gz_comp_needs_output_write_for_non_finish_flushes() {
        assert!(gz_comp_needs_output_write(
            1,
            crate::zlib_h::Z_BLOCK,
            crate::zlib_h::Z_OK
        ));
    }

    #[test]
    fn gz_comp_needs_output_write_only_finishes_at_stream_end() {
        assert!(!gz_comp_needs_output_write(
            1,
            crate::zlib_h::Z_FINISH,
            crate::zlib_h::Z_OK
        ));
        assert!(gz_comp_needs_output_write(
            1,
            crate::zlib_h::Z_FINISH,
            crate::zlib_h::Z_STREAM_END
        ));
    }

    #[test]
    fn gz_comp_needs_output_write_skips_idle_no_flush_calls() {
        assert!(!gz_comp_needs_output_write(
            1,
            crate::zlib_h::Z_NO_FLUSH,
            crate::zlib_h::Z_OK
        ));
    }

    #[test]
    fn gz_comp_needs_reset_skips_idle_no_flush_calls() {
        assert!(!gz_comp_needs_reset(0, crate::zlib_h::Z_NO_FLUSH));
    }

    #[test]
    fn gz_comp_needs_reset_handles_input_and_flush_requests() {
        assert!(gz_comp_needs_reset(1, crate::zlib_h::Z_NO_FLUSH));
        assert!(gz_comp_needs_reset(0, crate::zlib_h::Z_BLOCK));
    }

    #[test]
    fn gz_comp_skips_empty_no_flush_when_reset_is_pending() {
        assert!(gz_comp_skips_empty_flush(1, 0, crate::zlib_h::Z_NO_FLUSH));
        assert!(gz_comp_skips_empty_flush(-1, 0, crate::zlib_h::Z_NO_FLUSH));
    }

    #[test]
    fn gz_comp_reset_action_preserves_skip_and_reset_priority() {
        assert_eq!(gz_comp_reset_action(1, 0, crate::zlib_h::Z_NO_FLUSH), -1);
        assert_eq!(gz_comp_reset_action(0, 0, crate::zlib_h::Z_NO_FLUSH), 0);
        assert_eq!(gz_comp_reset_action(1, 1, crate::zlib_h::Z_NO_FLUSH), 1);
        assert_eq!(gz_comp_reset_action(1, 0, crate::zlib_h::Z_BLOCK), 1);
    }

    #[test]
    fn gz_comp_reset_after_flush_only_marks_finished_streams() {
        assert_eq!(gz_comp_reset_after_flush(crate::zlib_h::Z_FINISH, 0), 1);
        assert_eq!(gz_comp_reset_after_flush(crate::zlib_h::Z_FINISH, -1), 1);
        assert_eq!(gz_comp_reset_after_flush(crate::zlib_h::Z_NO_FLUSH, -1), -1);
        assert_eq!(gz_comp_reset_after_flush(crate::zlib_h::Z_BLOCK, 0), 0);
    }

    #[test]
    fn gz_comp_max_write_chunk_matches_the_quarter_range_boundary() {
        assert_eq!(
            gz_comp_max_write_chunk(),
            (::core::ffi::c_uint::MAX >> 2).wrapping_add(1)
        );
    }

    #[test]
    fn gz_comp_max_write_chunk_caps_large_available_input() {
        assert_eq!(
            gz_comp_write_chunk_len(usize::MAX, gz_comp_max_write_chunk()),
            gz_comp_max_write_chunk()
        );
    }

    #[test]
    fn gz_comp_does_not_skip_when_input_or_flush_requires_work() {
        assert!(!gz_comp_skips_empty_flush(0, 0, crate::zlib_h::Z_NO_FLUSH));
        assert!(!gz_comp_skips_empty_flush(1, 1, crate::zlib_h::Z_NO_FLUSH));
        assert!(!gz_comp_skips_empty_flush(1, 0, crate::zlib_h::Z_BLOCK));
    }

    #[test]
    fn gz_comp_write_chunk_len_keeps_lengths_within_cap() {
        assert_eq!(gz_comp_write_chunk_len(0, 4096), 0);
        assert_eq!(gz_comp_write_chunk_len(1024, 4096), 1024);
        assert_eq!(gz_comp_write_chunk_len(4096, 4096), 4096);
    }

    #[test]
    fn gz_comp_write_chunk_len_caps_lengths_above_limit() {
        assert_eq!(gz_comp_write_chunk_len(4097, 4096), 4096);
        assert_eq!(gz_comp_write_chunk_len(usize::MAX, 4096), 4096);
    }

    #[test]
    fn gz_comp_remaining_direct_input_subtracts_written_bytes() {
        assert_eq!(gz_comp_remaining_direct_input(1024, 24), 1000);
    }

    #[test]
    fn gz_comp_remaining_direct_input_preserves_wrapping_accounting() {
        assert_eq!(
            gz_comp_remaining_direct_input(0, 1),
            crate::stdlib::uInt::MAX
        );
    }

    #[test]
    fn gz_comp_output_produced_subtracts_remaining_output_space() {
        assert_eq!(gz_comp_output_produced(1024, 24), 1000);
        assert_eq!(gz_comp_output_produced(1024, 1024), 0);
    }

    #[test]
    fn gz_comp_output_produced_preserves_wrapping_accounting() {
        assert_eq!(gz_comp_output_produced(0, 1), ::core::ffi::c_uint::MAX);
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
    fn gz_write_state_is_usable_for_writable_healthy_or_retryable_states() {
        assert!(gz_write_state_is_usable(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_OK,
            0
        ));
        assert!(gz_write_state_is_usable(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_ERRNO,
            1
        ));
    }

    #[test]
    fn gz_write_state_is_usable_rejects_wrong_mode_and_unretryable_errors() {
        assert!(!gz_write_state_is_usable(
            crate::gzguts_h::GZ_WRITE + 1,
            crate::zlib_h::Z_OK,
            1
        ));
        assert!(!gz_write_state_is_usable(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_ERRNO,
            0
        ));
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
    fn gz_write_needs_pending_flush_only_for_buffered_input() {
        assert!(!gz_write_needs_pending_flush(0));
        assert!(gz_write_needs_pending_flush(1));
        assert!(gz_write_needs_pending_flush(crate::stdlib::uInt::MAX));
    }

    #[test]
    fn gz_write_uses_buffered_path_only_for_short_writes() {
        assert!(gz_write_uses_buffered_path(0, 1));
        assert!(gz_write_uses_buffered_path(1023, 1024));
        assert!(!gz_write_uses_buffered_path(1024, 1024));
        assert!(!gz_write_uses_buffered_path(1025, 1024));
    }

    #[test]
    fn gz_write_buffered_copy_len_uses_available_buffer_space() {
        assert_eq!(gz_write_buffered_copy_len(1024, 1000, 99), 24);
        assert_eq!(gz_write_buffered_copy_len(1024, 1024, 1), 0);
    }

    #[test]
    fn gz_write_buffered_copy_len_limits_to_remaining_input() {
        assert_eq!(gz_write_buffered_copy_len(1024, 1000, 12), 12);
    }

    #[test]
    fn gz_write_buffered_copy_len_preserves_wrapping_accounting() {
        assert_eq!(gz_write_buffered_copy_len(0, 1, 5), 5);
    }

    #[test]
    fn gz_write_buffered_progress_updates_position_and_remaining_input() {
        assert_eq!(gz_write_buffered_progress(10, 100, 60), (70, 40));
    }

    #[test]
    fn gz_write_buffered_progress_preserves_wrapping_remaining_input() {
        assert_eq!(
            gz_write_buffered_progress(0, 0, 1),
            (1, crate::stdlib::z_size_t::MAX)
        );
    }

    #[test]
    fn gz_buffered_have_counts_buffered_bytes() {
        let buffer = [0_u8; 8];
        assert_eq!(
            gz_buffered_have(buffer.as_ptr() as usize, buffer.as_ptr() as usize, 0),
            0
        );
        assert_eq!(
            gz_buffered_have(
                buffer.as_ptr() as usize,
                buffer.as_ptr().wrapping_add(3) as usize,
                2,
            ),
            5
        );
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

    #[test]
    fn gz_write_chunk_consumed_len_subtracts_unconsumed_input() {
        assert_eq!(gz_write_chunk_consumed_len(1024, 24), 1000);
        assert_eq!(gz_write_chunk_consumed_len(1024, 1024), 0);
    }

    #[test]
    fn gz_write_chunk_consumed_len_preserves_wrapping_accounting() {
        assert_eq!(gz_write_chunk_consumed_len(0, 1), ::core::ffi::c_uint::MAX);
    }

    #[test]
    fn gz_write_apply_direct_progress_accounts_for_partial_consumption() {
        let mut pos = 10;
        let mut remaining = 100;

        assert!(gz_write_apply_direct_progress(
            &mut pos,
            &mut remaining,
            80,
            20
        ));
        assert_eq!(pos, 70);
        assert_eq!(remaining, 40);
    }

    #[test]
    fn gz_write_apply_direct_progress_reports_input_exhaustion() {
        let mut pos = 10;
        let mut remaining = 80;

        assert!(!gz_write_apply_direct_progress(
            &mut pos,
            &mut remaining,
            80,
            0
        ));
        assert_eq!(pos, 90);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn gz_write_apply_direct_progress_preserves_wrapping_accounting() {
        let mut pos = 0;
        let mut remaining = ::core::ffi::c_uint::MAX as crate::stdlib::z_size_t;

        assert!(!gz_write_apply_direct_progress(
            &mut pos,
            &mut remaining,
            0,
            1
        ));
        assert_eq!(pos, ::core::ffi::c_uint::MAX as crate::stdlib::off64_t);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn gz_zero_apply_progress_accounts_for_partial_consumption() {
        let mut pos = 10;
        let mut skip = 100;

        assert!(gz_zero_apply_progress(&mut pos, &mut skip, 80, 20));
        assert_eq!(pos, 70);
        assert_eq!(skip, 40);
    }

    #[test]
    fn gz_zero_apply_progress_keeps_state_for_zero_consumption() {
        let mut pos = 10;
        let mut skip = 100;

        assert!(gz_zero_apply_progress(&mut pos, &mut skip, 80, 80));
        assert_eq!(pos, 10);
        assert_eq!(skip, 100);
    }

    #[test]
    fn gz_zero_apply_progress_reports_exact_skip_exhaustion() {
        let mut pos = 10;
        let mut skip = 80;

        assert!(!gz_zero_apply_progress(&mut pos, &mut skip, 80, 0));
        assert_eq!(pos, 90);
        assert_eq!(skip, 0);
    }

    #[test]
    fn gz_zero_apply_progress_preserves_wrapping_consumed_behavior() {
        let mut pos = 0;
        let mut skip = ::core::ffi::c_uint::MAX as crate::stdlib::off64_t;

        assert!(!gz_zero_apply_progress(&mut pos, &mut skip, 0, 1));
        assert_eq!(pos, ::core::ffi::c_uint::MAX as crate::stdlib::off64_t);
        assert_eq!(skip, 0);
    }
}
