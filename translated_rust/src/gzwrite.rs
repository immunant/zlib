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
    state.out_pending = 0;
    if state.direct == 0 {
        state.strm.avail_out = state.size as crate::stdlib::uInt;
        state.strm.next_out = state.out;
        state.x.next = state.strm.next_out;
    }
}

fn gz_init_stream_defaults(strm: &mut crate::zlib_h::z_stream) {
    strm.zalloc = None;
    strm.zfree = None;
    strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
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

enum GzZeroAction {
    Error,
    Done,
    Continue,
}

fn gz_zero_action(ret: ::core::ffi::c_int, has_skip: bool) -> GzZeroAction {
    if ret == -1 as ::core::ffi::c_int {
        GzZeroAction::Error
    } else if !has_skip {
        GzZeroAction::Done
    } else {
        GzZeroAction::Continue
    }
}

fn gzclose_w_result(
    zero_error: Option<::core::ffi::c_int>,
    finish_error: Option<::core::ffi::c_int>,
    close_failed: bool,
) -> ::core::ffi::c_int {
    let mut ret = crate::zlib_h::Z_OK;
    if let Some(error) = zero_error {
        ret = error;
    }
    if let Some(error) = finish_error {
        ret = error;
    }
    if close_failed {
        ret = crate::zlib_h::Z_ERRNO;
    }
    ret
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

fn gzsetparams_state_is_usable(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
) -> bool {
    gz_write_state_is_usable(mode, err, again) && direct == 0
}

fn gzflush_mode_is_valid(flush: ::core::ffi::c_int) -> bool {
    flush >= 0 && flush <= crate::zlib_h::Z_FINISH
}

fn gzfwrite_result(
    size: crate::stdlib::z_size_t,
    len: crate::stdlib::z_size_t,
    written: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if len == 0 {
        0
    } else {
        written.wrapping_div(size)
    }
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

fn gz_write_is_empty(len: crate::stdlib::z_size_t) -> bool {
    len == 0
}

fn gz_has_pending_input(avail_in: crate::stdlib::uInt) -> bool {
    avail_in != 0
}

fn gz_has_pending_skip(skip: crate::stdlib::off64_t) -> bool {
    skip != 0
}

fn gz_buffer_is_initialized(size: ::core::ffi::c_uint) -> bool {
    size != 0
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

fn gz_write_chunk_len(remaining: crate::stdlib::z_size_t) -> ::core::ffi::c_uint {
    if ::core::ffi::c_uint::MAX as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        ::core::ffi::c_uint::MAX
    }
}

fn gz_write_consumed(
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    chunk_len.wrapping_sub(remaining_avail_in as ::core::ffi::c_uint)
}

fn gz_write_apply_chunk_progress(
    pos: &mut crate::stdlib::off64_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    let consumed = gz_write_consumed(chunk_len, remaining_avail_in);
    *pos = gz_write_advanced_pos(*pos, consumed);
    consumed
}

fn gz_write_advanced_pos(
    pos: crate::stdlib::off64_t,
    consumed: ::core::ffi::c_uint,
) -> crate::stdlib::off64_t {
    pos + consumed as crate::stdlib::off64_t
}

fn gz_write_remaining_after_consumption(
    remaining: crate::stdlib::z_size_t,
    consumed: ::core::ffi::c_uint,
) -> crate::stdlib::z_size_t {
    remaining.wrapping_sub(consumed as crate::stdlib::z_size_t)
}

#[derive(Debug, Eq, PartialEq)]
struct GzWriteDirectProgress {
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
}

fn gz_write_direct_progress(
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
) -> GzWriteDirectProgress {
    let consumed = gz_write_consumed(chunk_len, remaining_avail_in);

    GzWriteDirectProgress {
        pos: gz_write_advanced_pos(pos, consumed),
        remaining: gz_write_remaining_after_consumption(remaining, consumed),
    }
}

enum GzWriteDirectAction {
    Error,
    Done,
    Continue,
}

fn gz_write_direct_action(
    ret: ::core::ffi::c_int,
    remaining: crate::stdlib::z_size_t,
) -> GzWriteDirectAction {
    if ret == -1 as ::core::ffi::c_int {
        GzWriteDirectAction::Error
    } else if gz_write_is_empty(remaining) {
        GzWriteDirectAction::Done
    } else {
        GzWriteDirectAction::Continue
    }
}

fn gz_zero_apply_progress(
    pos: &mut crate::stdlib::off64_t,
    skip: &mut crate::stdlib::off64_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
) -> bool {
    let consumed = gz_write_apply_chunk_progress(pos, chunk_len, remaining_avail_in);
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

#[derive(Debug, Eq, PartialEq)]
enum GzPutcWriteAction {
    Error,
    ReturnByte,
}

fn gzputc_write_action(written: crate::stdlib::z_size_t) -> GzPutcWriteAction {
    if written == 1 {
        GzPutcWriteAction::ReturnByte
    } else {
        GzPutcWriteAction::Error
    }
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

enum GzCompResetAction {
    Skip,
    Reset,
    Continue,
}

fn gz_comp_reset_action(
    reset: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> GzCompResetAction {
    if gz_comp_skips_empty_flush(reset, avail_in, flush) {
        GzCompResetAction::Skip
    } else if reset != 0 {
        GzCompResetAction::Reset
    } else {
        GzCompResetAction::Continue
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

enum GzCompWriteFailure {
    Retryable,
    Fatal,
}

fn gz_comp_write_failure(errno: ::core::ffi::c_int) -> GzCompWriteFailure {
    if gz_write_errno_is_retryable(errno) {
        GzCompWriteFailure::Retryable
    } else {
        GzCompWriteFailure::Fatal
    }
}

fn gz_comp_write_again(errno: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match gz_comp_write_failure(errno) {
        GzCompWriteFailure::Retryable => 1,
        GzCompWriteFailure::Fatal => 0,
    }
}

fn gz_comp_write_chunk_len(available: usize, max: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    if available > max as usize {
        max
    } else {
        available as ::core::ffi::c_uint
    }
}

fn gz_comp_output_write_chunk_len(
    pending: crate::stdlib::uInt,
    max: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if pending != 0 {
        Some(gz_comp_write_chunk_len(pending as usize, max))
    } else {
        None
    }
}

struct GzCompDirectWriteProgress {
    remaining_input: crate::stdlib::uInt,
    cursor_advance: usize,
}

fn gz_comp_direct_write_progress(
    avail_in: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
) -> GzCompDirectWriteProgress {
    GzCompDirectWriteProgress {
        remaining_input: avail_in.wrapping_sub(written as crate::stdlib::uInt),
        cursor_advance: written as usize,
    }
}

struct GzCompOutputWriteProgress {
    remaining_pending: crate::stdlib::uInt,
    cursor_advance: usize,
}

fn gz_comp_output_write_progress(
    pending: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
) -> GzCompOutputWriteProgress {
    GzCompOutputWriteProgress {
        remaining_pending: gz_comp_pending_after_write(pending, written),
        cursor_advance: written as usize,
    }
}

fn gz_comp_output_produced(
    avail_out_before: ::core::ffi::c_uint,
    avail_out_after: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    avail_out_before.wrapping_sub(avail_out_after)
}

fn gz_comp_pending_after_output(
    pending: crate::stdlib::uInt,
    produced: ::core::ffi::c_uint,
) -> crate::stdlib::uInt {
    pending.wrapping_add(produced as crate::stdlib::uInt)
}

fn gz_comp_pending_after_write(
    pending: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
) -> crate::stdlib::uInt {
    pending.wrapping_sub(written as crate::stdlib::uInt)
}

fn gz_comp_write_failed(written: ::core::ffi::c_int) -> bool {
    written < 0
}

fn gz_comp_has_output(produced: ::core::ffi::c_uint) -> bool {
    produced != 0
}

fn gz_comp_deflate_stream_is_corrupt(ret: ::core::ffi::c_int) -> bool {
    ret == crate::zlib_h::Z_STREAM_ERROR
}

#[derive(Debug, Eq, PartialEq)]
struct GzWriteBufferedProgress {
    copy: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
}

fn gz_write_buffered_progress(
    size: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
) -> GzWriteBufferedProgress {
    let copy = gz_write_buffered_copy_len(size, have, remaining);

    GzWriteBufferedProgress {
        copy,
        avail_in: avail_in.wrapping_add(copy),
        have: have.wrapping_add(copy),
        pos: gz_write_advanced_pos(pos, copy),
        remaining: gz_write_remaining_after_consumption(remaining, copy),
    }
}

unsafe fn gz_init(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
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
        gz_init_stream_defaults(&mut state.strm);
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
    }
    gz_init_core(state);
    0 as ::core::ffi::c_int
}

unsafe fn gz_comp(
    mut state: crate::gzguts_h::gz_statep,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = gz_comp_max_write_chunk();
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if !gz_buffer_is_initialized((*state).size) && gz_init(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let out_pending = &mut (*state).out_pending;
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
            if gz_comp_write_failed(writ) {
                let errno = *crate::stdlib::__errno_location();
                (*state).again = gz_comp_write_again(errno);
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_ERRNO,
                    crate::stdlib::strerror(errno),
                );
                return -1 as ::core::ffi::c_int;
            }
            let progress = gz_comp_direct_write_progress((*strm).avail_in, writ);
            (*strm).avail_in = progress.remaining_input;
            (*strm).next_in = (*strm).next_in.wrapping_add(progress.cursor_advance);
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut reset = (*state).reset;
    match gz_comp_reset_action(reset, (*strm).avail_in, flush) {
        GzCompResetAction::Skip => return 0 as ::core::ffi::c_int,
        GzCompResetAction::Reset => {
            crate::src::deflate::deflateReset(strm as *mut crate::zlib_h::z_stream_s);
            (*state).reset = 0 as ::core::ffi::c_int;
            reset = 0 as ::core::ffi::c_int;
        }
        GzCompResetAction::Continue => {}
    }
    ret = crate::zlib_h::Z_OK;
    loop {
        if gz_comp_needs_output_write((*strm).avail_out, flush, ret) {
            while let Some(chunk_len) = gz_comp_output_write_chunk_len(*out_pending, max) {
                *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                (*state).again = 0 as ::core::ffi::c_int;
                put = chunk_len;
                writ = crate::stdlib::write(
                    (*state).fd,
                    (*state).x.next as *const ::core::ffi::c_void,
                    put as crate::__stddef_size_t_h::size_t,
                ) as ::core::ffi::c_int;
                if gz_comp_write_failed(writ) {
                    let errno = *crate::stdlib::__errno_location();
                    (*state).again = gz_comp_write_again(errno);
                    crate::src::gzlib::gz_error(
                        state as *mut crate::gzguts_h::gz_state,
                        crate::zlib_h::Z_ERRNO,
                        crate::stdlib::strerror(errno),
                    );
                    return -1 as ::core::ffi::c_int;
                }
                let progress = gz_comp_output_write_progress(*out_pending, writ);
                (*state).x.next = (*state).x.next.wrapping_add(progress.cursor_advance);
                *out_pending = progress.remaining_pending;
            }
            if gz_comp_needs_output_buffer_reset((*strm).avail_out) {
                (*strm).avail_out = (*state).size as crate::stdlib::uInt;
                (*strm).next_out = (*state).out;
                (*state).x.next = (*state).out;
                *out_pending = 0;
            }
        }
        have = (*strm).avail_out as ::core::ffi::c_uint;
        ret = crate::src::deflate::deflate(strm as *mut crate::zlib_h::z_stream_s, flush);
        if gz_comp_deflate_stream_is_corrupt(ret) {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        have = gz_comp_output_produced(have, (*strm).avail_out as ::core::ffi::c_uint);
        *out_pending = gz_comp_pending_after_output(*out_pending, have);
        if !gz_comp_has_output(have) {
            break;
        }
    }
    (*state).reset = gz_comp_reset_after_flush(flush, reset);
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_zero(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if gz_has_pending_input((*strm).avail_in)
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
        (*strm).next_in = (*state).in_0;
        ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        let remaining_avail_in = (*strm).avail_in;
        let has_skip = gz_zero_apply_progress(
            &mut (*state).x.pos,
            &mut (*state).skip,
            n,
            remaining_avail_in,
        );
        match gz_zero_action(ret, has_skip) {
            GzZeroAction::Error => return -1 as ::core::ffi::c_int,
            GzZeroAction::Done => break,
            GzZeroAction::Continue => {}
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_write(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: crate::stdlib::voidpc,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut put: crate::stdlib::z_size_t = len;
    let mut ret: ::core::ffi::c_int = 0;
    if gz_write_is_empty(len) {
        return 0 as crate::stdlib::z_size_t;
    }
    if !gz_buffer_is_initialized((*state).size) && gz_init(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if gz_has_pending_skip((*state).skip) && gz_zero(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    if gz_write_uses_buffered_path(len, (*state).size) {
        loop {
            let mut have: ::core::ffi::c_uint = 0;
            let mut copy: ::core::ffi::c_uint = 0;
            if !gz_has_pending_input((*state).strm.avail_in) {
                (*state).strm.next_in = (*state).in_0;
                (*state).x.have = 0;
            }
            have = (*state).x.have;
            let progress = gz_write_buffered_progress(
                (*state).size,
                have,
                (*state).strm.avail_in,
                (*state).x.pos,
                len,
            );
            copy = progress.copy;
            (*state).strm.avail_in = progress.avail_in;
            (*state).x.have = progress.have;
            (*state).x.pos = progress.pos;
            len = progress.remaining;
            crate::stdlib::memcpy(
                (*state).in_0.wrapping_add(have as usize) as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                copy as crate::__stddef_size_t_h::size_t,
            );
            buf = (buf as *mut crate::stdlib::Bytef).wrapping_add(copy as usize)
                as crate::stdlib::voidpc;
            if gz_write_is_empty(len) {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return gz_write_error_result((*state).again, put, len);
            }
        }
    } else {
        let state = &mut *state;
        if gz_has_pending_input(state.strm.avail_in)
            && gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
        }
        state.strm.next_in = buf as *mut crate::stdlib::Bytef;
        loop {
            let n = gz_write_chunk_len(len);
            state.strm.avail_in = n as crate::stdlib::uInt;
            ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            let progress = gz_write_direct_progress(state.x.pos, len, n, state.strm.avail_in);
            state.x.pos = progress.pos;
            len = progress.remaining;
            match gz_write_direct_action(ret, len) {
                GzWriteDirectAction::Error => return gz_write_error_result(state.again, put, len),
                GzWriteDirectAction::Done => break,
                GzWriteDirectAction::Continue => {}
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzSetParamsBufferAction {
    SetOnly,
    DeflateOnly,
    FlushThenDeflate,
}

fn gzsetparams_buffer_action(
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
) -> GzSetParamsBufferAction {
    if !gz_buffer_is_initialized(size) {
        GzSetParamsBufferAction::SetOnly
    } else if gz_has_pending_input(avail_in) {
        GzSetParamsBufferAction::FlushThenDeflate
    } else {
        GzSetParamsBufferAction::DeflateOnly
    }
}

fn gzclose_mode_is_writable(mode: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_WRITE
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
    if !gz_write_state_is_usable((*state).mode, (*state).err, (*state).again) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(len) = crate::src::gzlib::gz_request_len(size, nitems) else {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    };
    return gzfwrite_result(size, len, gz_write(state, buf, len));
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
    let buf = [c as ::core::ffi::c_uchar];
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_write_state_is_usable((*state).mode, (*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let written = gz_write(
        state,
        buf.as_ptr() as crate::stdlib::voidpc,
        1 as crate::stdlib::z_size_t,
    );
    match gzputc_write_action(written) {
        GzPutcWriteAction::Error => return -1 as ::core::ffi::c_int,
        GzPutcWriteAction::ReturnByte => {}
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
    if !gz_write_state_is_usable((*state).mode, (*state).err, (*state).again) {
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
    if !gz_write_state_is_usable((*state).mode, (*state).err, (*state).again) {
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
    if gz_has_pending_skip((*state).skip) && gz_zero(state) == -1 as ::core::ffi::c_int {
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
    if !gzsetparams_state_is_usable((*state).mode, (*state).err, (*state).again, (*state).direct) {
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
    if gz_has_pending_skip((*state).skip) && gz_zero(state) == -1 as ::core::ffi::c_int {
        return (*state).err;
    }
    let action = gzsetparams_buffer_action((*state).size, (*strm).avail_in);
    if matches!(action, GzSetParamsBufferAction::FlushThenDeflate)
        && gz_comp(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
    {
        return (*state).err;
    }
    if !matches!(action, GzSetParamsBufferAction::SetOnly) {
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
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gzclose_mode_is_writable((*state).mode) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let zero_error =
        if gz_has_pending_skip((*state).skip) && gz_zero(state) == -1 as ::core::ffi::c_int {
            Some((*state).err)
        } else {
            None
        };
    let finish_error = if gz_comp(state, crate::zlib_h::Z_FINISH) == -1 as ::core::ffi::c_int {
        Some((*state).err)
    } else {
        None
    };
    if gz_buffer_is_initialized((*state).size) {
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
    let close_failed = crate::stdlib::close((*state).fd) == -1 as ::core::ffi::c_int;
    crate::stdlib::free(state as *mut ::core::ffi::c_void);
    return gzclose_w_result(zero_error, finish_error, close_failed);
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_w(file)
}

#[cfg(test)]
mod tests {
    use super::{
        gz_buffer_is_initialized, gz_comp_deflate_stream_is_corrupt, gz_comp_direct_write_progress,
        gz_comp_has_output, gz_comp_max_write_chunk, gz_comp_needs_output_buffer_reset,
        gz_comp_needs_output_write, gz_comp_needs_reset, gz_comp_output_produced,
        gz_comp_output_write_chunk_len, gz_comp_output_write_progress,
        gz_comp_pending_after_output, gz_comp_pending_after_write, gz_comp_reset_action,
        gz_comp_reset_after_flush, gz_comp_skips_empty_flush, gz_comp_write_again,
        gz_comp_write_chunk_len, gz_comp_write_failed, gz_comp_write_failure, gz_has_pending_input,
        gz_has_pending_skip, gz_init_stream_defaults, gz_write_advanced_pos,
        gz_write_apply_chunk_progress, gz_write_buffered_copy_len, gz_write_buffered_progress,
        gz_write_chunk_len, gz_write_consumed, gz_write_direct_action, gz_write_direct_progress,
        gz_write_errno_is_retryable, gz_write_error_result, gz_write_is_empty,
        gz_write_remaining_after_consumption, gz_write_state_is_usable,
        gz_write_uses_buffered_path, gz_zero_action, gz_zero_apply_progress, gz_zero_chunk_len,
        gz_zero_needs_initialization, gzclose_mode_is_writable, gzclose_w_result,
        gzflush_mode_is_valid, gzfwrite_result, gzputc_result, gzputc_write_action,
        gzputs_len_fits_int, gzputs_result, gzsetparams_buffer_action, gzsetparams_settings_match,
        gzsetparams_state_is_usable, gzwrite_len_fits_int, GzCompResetAction, GzCompWriteFailure,
        GzPutcWriteAction, GzSetParamsBufferAction, GzWriteDirectAction, GzZeroAction,
    };

    #[test]
    fn gzclose_w_result_returns_success_without_errors() {
        assert_eq!(gzclose_w_result(None, None, false), crate::zlib_h::Z_OK);
    }

    #[test]
    fn gzclose_w_result_returns_zero_error() {
        assert_eq!(gzclose_w_result(Some(-10), None, false), -10);
    }

    #[test]
    fn gzclose_w_result_returns_finish_error() {
        assert_eq!(gzclose_w_result(None, Some(-11), false), -11);
    }

    #[test]
    fn gzclose_w_result_prefers_finish_error() {
        assert_eq!(gzclose_w_result(Some(-10), Some(-11), false), -11);
    }

    #[test]
    fn gzclose_w_result_prefers_close_failure() {
        assert_eq!(
            gzclose_w_result(Some(-10), Some(-11), true),
            crate::zlib_h::Z_ERRNO
        );
    }

    #[test]
    fn gzclose_mode_is_writable_only_for_write_mode() {
        assert!(gzclose_mode_is_writable(crate::gzguts_h::GZ_WRITE));
        assert!(!gzclose_mode_is_writable(crate::gzguts_h::GZ_WRITE + 1));
    }

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
    fn gz_has_pending_input_only_for_buffered_input() {
        assert!(!gz_has_pending_input(0));
        assert!(gz_has_pending_input(1));
        assert!(gz_has_pending_input(crate::stdlib::uInt::MAX));
    }

    #[test]
    fn gz_has_pending_skip_detects_nonzero_offsets() {
        assert!(!gz_has_pending_skip(0));
        assert!(gz_has_pending_skip(1));
        assert!(gz_has_pending_skip(-1));
    }

    #[test]
    fn gz_buffer_is_initialized_requires_nonzero_size() {
        assert!(!gz_buffer_is_initialized(0));
        assert!(gz_buffer_is_initialized(1));
        assert!(gz_buffer_is_initialized(::core::ffi::c_uint::MAX));
    }

    #[test]
    fn gz_init_stream_defaults_clears_callbacks_and_input() {
        let mut strm = crate::zlib_h::z_stream {
            next_in: ::core::ptr::null_mut(),
            avail_in: 0,
            total_in: 0,
            next_out: ::core::ptr::null_mut(),
            avail_out: 0,
            total_out: 0,
            msg: ::core::ptr::null_mut(),
            state: ::core::ptr::null_mut(),
            zalloc: Some(crate::src::zutil::zcalloc_ffi),
            zfree: Some(crate::src::zutil::zcfree_ffi),
            opaque: ::core::ptr::null_mut(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        };

        gz_init_stream_defaults(&mut strm);

        assert!(strm.zalloc.is_none());
        assert!(strm.zfree.is_none());
        assert!(strm.opaque.is_null());
        assert!(strm.next_in.is_null());
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
    fn gzsetparams_buffer_action_preserves_buffer_and_pending_input_cases() {
        assert_eq!(
            gzsetparams_buffer_action(0, 1),
            GzSetParamsBufferAction::SetOnly
        );
        assert_eq!(
            gzsetparams_buffer_action(1, 0),
            GzSetParamsBufferAction::DeflateOnly
        );
        assert_eq!(
            gzsetparams_buffer_action(1, 1),
            GzSetParamsBufferAction::FlushThenDeflate
        );
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
    fn gzputc_write_action_requires_exactly_one_written_byte() {
        assert_eq!(gzputc_write_action(0), GzPutcWriteAction::Error);
        assert_eq!(gzputc_write_action(1), GzPutcWriteAction::ReturnByte);
        assert_eq!(gzputc_write_action(2), GzPutcWriteAction::Error);
    }

    #[test]
    fn gz_comp_write_failure_classifies_retryable_and_fatal_errors() {
        assert!(matches!(
            gz_comp_write_failure(crate::stdlib::EAGAIN),
            GzCompWriteFailure::Retryable
        ));
        assert!(matches!(
            gz_comp_write_failure(crate::stdlib::EWOULDBLOCK),
            GzCompWriteFailure::Retryable
        ));
        assert!(matches!(
            gz_comp_write_failure(1),
            GzCompWriteFailure::Fatal
        ));
    }

    #[test]
    fn gz_comp_write_again_sets_retryable_errors_only() {
        assert_eq!(gz_comp_write_again(crate::stdlib::EAGAIN), 1);
        assert_eq!(gz_comp_write_again(crate::stdlib::EWOULDBLOCK), 1);
        assert_eq!(gz_comp_write_again(0), 0);
        assert_eq!(gz_comp_write_again(1), 0);
    }

    #[test]
    fn gz_comp_write_again_rejects_non_retryable_extreme_errors() {
        assert_eq!(gz_comp_write_again(::core::ffi::c_int::MIN), 0);
        assert_eq!(gz_comp_write_again(::core::ffi::c_int::MAX), 0);
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
        assert!(matches!(
            gz_comp_reset_action(1, 0, crate::zlib_h::Z_NO_FLUSH),
            GzCompResetAction::Skip
        ));
        assert!(matches!(
            gz_comp_reset_action(0, 0, crate::zlib_h::Z_NO_FLUSH),
            GzCompResetAction::Continue
        ));
        assert!(matches!(
            gz_comp_reset_action(1, 1, crate::zlib_h::Z_NO_FLUSH),
            GzCompResetAction::Reset
        ));
        assert!(matches!(
            gz_comp_reset_action(1, 0, crate::zlib_h::Z_BLOCK),
            GzCompResetAction::Reset
        ));
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
    fn gz_comp_output_write_chunk_len_requires_pending_output() {
        assert_eq!(gz_comp_output_write_chunk_len(0, 64), None);
    }

    #[test]
    fn gz_comp_output_write_chunk_len_uses_pending_output_length() {
        assert_eq!(gz_comp_output_write_chunk_len(24, 64), Some(24));
        assert_eq!(gz_comp_output_write_chunk_len(100, 64), Some(64));
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
    fn gz_comp_direct_write_progress_tracks_remaining_input_and_cursor() {
        let progress = gz_comp_direct_write_progress(1024, 24);

        assert_eq!(progress.remaining_input, 1000);
        assert_eq!(progress.cursor_advance, 24);
    }

    #[test]
    fn gz_comp_direct_write_progress_preserves_wrapping_accounting() {
        let progress = gz_comp_direct_write_progress(0, 1);

        assert_eq!(progress.remaining_input, crate::stdlib::uInt::MAX);
        assert_eq!(progress.cursor_advance, 1);
    }

    #[test]
    fn gz_comp_output_write_progress_tracks_pending_bytes_and_cursor() {
        let progress = gz_comp_output_write_progress(100, 24);

        assert_eq!(progress.remaining_pending, 76);
        assert_eq!(progress.cursor_advance, 24);
    }

    #[test]
    fn gz_comp_output_write_progress_preserves_wrapping_accounting() {
        let progress = gz_comp_output_write_progress(0, 1);

        assert_eq!(progress.remaining_pending, crate::stdlib::uInt::MAX);
        assert_eq!(progress.cursor_advance, 1);
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
    fn gz_comp_pending_output_tracks_produced_and_written_bytes() {
        let pending = gz_comp_pending_after_output(0, 100);
        assert_eq!(pending, 100);
        assert_eq!(gz_comp_pending_after_write(pending, 20), 80);
    }

    #[test]
    fn gz_comp_pending_output_preserves_wrapping_accounting() {
        assert_eq!(gz_comp_pending_after_output(crate::stdlib::uInt::MAX, 1), 0);
        assert_eq!(gz_comp_pending_after_write(0, 1), crate::stdlib::uInt::MAX);
    }

    #[test]
    fn gz_comp_write_failed_only_for_negative_results() {
        assert!(gz_comp_write_failed(-1));
        assert!(!gz_comp_write_failed(0));
        assert!(!gz_comp_write_failed(::core::ffi::c_int::MAX));
    }

    #[test]
    fn gz_comp_has_output_only_for_nonzero_production() {
        assert!(!gz_comp_has_output(0));
        assert!(gz_comp_has_output(1));
        assert!(gz_comp_has_output(::core::ffi::c_uint::MAX));
    }

    #[test]
    fn gz_comp_deflate_stream_is_corrupt_only_for_stream_errors() {
        assert!(gz_comp_deflate_stream_is_corrupt(
            crate::zlib_h::Z_STREAM_ERROR
        ));
        assert!(!gz_comp_deflate_stream_is_corrupt(crate::zlib_h::Z_OK));
        assert!(!gz_comp_deflate_stream_is_corrupt(
            crate::zlib_h::Z_STREAM_END
        ));
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
    fn gzfwrite_result_converts_written_bytes_to_items() {
        assert_eq!(gzfwrite_result(4, 12, 12), 3);
        assert_eq!(gzfwrite_result(4, 12, 11), 2);
    }

    #[test]
    fn gzfwrite_result_handles_zero_size_requests_without_division() {
        assert_eq!(gzfwrite_result(0, 0, 0), 0);
    }

    #[test]
    fn gzsetparams_state_is_usable_rejects_direct_and_invalid_write_states() {
        assert!(gzsetparams_state_is_usable(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_OK,
            0,
            0
        ));
        assert!(!gzsetparams_state_is_usable(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_OK,
            0,
            1
        ));
        assert!(!gzsetparams_state_is_usable(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_ERRNO,
            0,
            0
        ));
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
    fn gz_write_is_empty_only_for_zero_length() {
        assert!(gz_write_is_empty(0));
        assert!(!gz_write_is_empty(1));
        assert!(!gz_write_is_empty(crate::stdlib::z_size_t::MAX));
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
    fn gz_write_buffered_progress_tracks_all_buffer_bookkeeping() {
        let progress = gz_write_buffered_progress(1024, 1000, 17, 10, 99);

        assert_eq!(progress.copy, 24);
        assert_eq!(progress.avail_in, 41);
        assert_eq!(progress.have, 1024);
        assert_eq!(progress.pos, 34);
        assert_eq!(progress.remaining, 75);
    }

    #[test]
    fn gz_write_buffered_progress_uses_shared_position_advance() {
        let progress = gz_write_buffered_progress(2, 0, 0, -1, 1);

        assert_eq!(progress.pos, 0);
        assert_eq!(progress.remaining, 0);
    }

    #[test]
    fn gz_write_buffered_progress_preserves_wrapping_accounting() {
        let progress = gz_write_buffered_progress(0, ::core::ffi::c_uint::MAX, 1, 0, 5);

        assert_eq!(progress.copy, 1);
        assert_eq!(progress.avail_in, 2);
        assert_eq!(progress.have, 0);
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
    fn gz_write_apply_chunk_progress_consumes_and_advances_position() {
        let mut pos = 100;

        assert_eq!(gz_write_apply_chunk_progress(&mut pos, 1024, 24), 1000);
        assert_eq!(pos, 1100);

        assert_eq!(gz_write_apply_chunk_progress(&mut pos, 1024, 1024), 0);
        assert_eq!(pos, 1100);
    }

    #[test]
    fn gz_write_consumed_preserves_partial_and_wrapping_subtraction() {
        assert_eq!(gz_write_consumed(80, 0), 80);
        assert_eq!(gz_write_consumed(80, 20), 60);
        assert_eq!(gz_write_consumed(0, 1), ::core::ffi::c_uint::MAX);
    }

    #[test]
    fn gz_write_apply_chunk_progress_preserves_wrapping_accounting() {
        let mut pos = 0;

        assert_eq!(
            gz_write_apply_chunk_progress(&mut pos, 0, 1),
            ::core::ffi::c_uint::MAX
        );
        assert_eq!(pos, ::core::ffi::c_uint::MAX as crate::stdlib::off64_t);
    }

    #[test]
    fn gz_write_advanced_pos_advances_by_consumed_input() {
        assert_eq!(gz_write_advanced_pos(100, 24), 124);
        assert_eq!(gz_write_advanced_pos(-1, 1), 0);
    }

    #[test]
    fn gz_write_remaining_after_consumption_preserves_wrapping_subtraction() {
        assert_eq!(gz_write_remaining_after_consumption(100, 24), 76);
        assert_eq!(
            gz_write_remaining_after_consumption(0, 1),
            crate::stdlib::z_size_t::MAX
        );
    }

    #[test]
    fn gz_write_direct_progress_accounts_for_partial_consumption() {
        let progress = gz_write_direct_progress(10, 100, 80, 20);

        assert_eq!(progress.pos, 70);
        assert_eq!(progress.remaining, 40);
    }

    #[test]
    fn gz_write_direct_progress_reports_input_exhaustion() {
        let progress = gz_write_direct_progress(10, 80, 80, 0);

        assert_eq!(progress.pos, 90);
        assert_eq!(progress.remaining, 0);
    }

    #[test]
    fn gz_write_direct_progress_preserves_wrapping_accounting() {
        let progress =
            gz_write_direct_progress(0, ::core::ffi::c_uint::MAX as crate::stdlib::z_size_t, 0, 1);

        assert_eq!(
            progress.pos,
            ::core::ffi::c_uint::MAX as crate::stdlib::off64_t
        );
        assert_eq!(progress.remaining, 0);
    }

    #[test]
    fn gz_write_direct_action_prioritizes_errors_over_completion() {
        assert!(matches!(
            gz_write_direct_action(-1, 0),
            GzWriteDirectAction::Error
        ));
    }

    #[test]
    fn gz_write_direct_action_finishes_after_all_input_is_consumed() {
        assert!(matches!(
            gz_write_direct_action(0, 0),
            GzWriteDirectAction::Done
        ));
    }

    #[test]
    fn gz_write_direct_action_continues_after_successful_partial_write() {
        assert!(matches!(
            gz_write_direct_action(0, 1),
            GzWriteDirectAction::Continue
        ));
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

    #[test]
    fn gz_zero_action_prioritizes_error_over_skip_exhaustion() {
        assert!(matches!(gz_zero_action(-1, false), GzZeroAction::Error));
    }

    #[test]
    fn gz_zero_action_finishes_when_skip_is_exhausted() {
        assert!(matches!(gz_zero_action(0, false), GzZeroAction::Done));
    }

    #[test]
    fn gz_zero_action_continues_for_other_statuses_with_skip_remaining() {
        assert!(matches!(gz_zero_action(0, true), GzZeroAction::Continue));
        assert!(matches!(gz_zero_action(1, true), GzZeroAction::Continue));
    }
}
