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

fn gz_init_deflate_failed(result: ::core::ffi::c_int) -> bool {
    result != crate::zlib_h::Z_OK
}

fn gz_init_failed(result: ::core::ffi::c_int) -> bool {
    result == -1 as ::core::ffi::c_int
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GzInitMode {
    Direct,
    Compressed,
}

fn gz_comp_is_direct(direct: ::core::ffi::c_int) -> bool {
    direct != 0
}

#[derive(Debug, Eq, PartialEq)]
enum GzCompDirectLoopAction {
    Write,
    Done,
}

fn gz_comp_direct_loop_action(avail_in: crate::stdlib::uInt) -> GzCompDirectLoopAction {
    if gz_has_pending_input(avail_in) {
        GzCompDirectLoopAction::Write
    } else {
        GzCompDirectLoopAction::Done
    }
}

fn gz_init_mode(direct: ::core::ffi::c_int) -> GzInitMode {
    if gz_comp_is_direct(direct) {
        GzInitMode::Direct
    } else {
        GzInitMode::Compressed
    }
}

#[derive(Debug, Eq, PartialEq)]
struct GzInitAllocationPlan {
    input_len: crate::stdlib::z_size_t,
    output_len: Option<crate::stdlib::z_size_t>,
}

fn gz_init_allocation_plan(
    want: ::core::ffi::c_uint,
    direct: ::core::ffi::c_int,
) -> GzInitAllocationPlan {
    GzInitAllocationPlan {
        input_len: want.wrapping_shl(1) as crate::stdlib::z_size_t,
        output_len: (gz_init_mode(direct) == GzInitMode::Compressed)
            .then_some(want as crate::stdlib::z_size_t),
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

fn gz_zero_initialize_buffer(buffer: &mut [crate::stdlib::Byte]) {
    buffer.fill(0);
}

fn gz_zero_initialize_chunk_buffer(
    first: &mut ::core::ffi::c_int,
    buffer: &mut [crate::stdlib::Byte],
) {
    gz_zero_initialize_buffer(buffer);
    *first = 0;
}

#[derive(Debug, Eq, PartialEq)]
struct GzZeroChunkLimits {
    int_and_off64_are_same_size: bool,
    int_max: ::core::ffi::c_uint,
}

fn gz_zero_chunk_limits() -> GzZeroChunkLimits {
    GzZeroChunkLimits {
        int_and_off64_are_same_size: ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>(),
        int_max: crate::src::gzlib::gz_intmax(),
    }
}

enum GzZeroStep {
    FlushPending,
    WriteChunk {
        len: ::core::ffi::c_uint,
        initialize_buffer: bool,
    },
}

fn gz_zero_pending_step(pending_input: crate::stdlib::uInt) -> GzZeroStep {
    if gz_has_pending_input(pending_input) {
        GzZeroStep::FlushPending
    } else {
        GzZeroStep::WriteChunk {
            len: 0,
            initialize_buffer: false,
        }
    }
}

fn gz_zero_chunk_step(
    first: ::core::ffi::c_int,
    size: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
    int_and_off64_are_same_size: bool,
    int_max: ::core::ffi::c_uint,
) -> GzZeroStep {
    GzZeroStep::WriteChunk {
        len: gz_zero_chunk_len(size, skip, int_and_off64_are_same_size, int_max),
        initialize_buffer: gz_zero_needs_initialization(first),
    }
}

fn gz_zero_initial_step(
    pending_input: crate::stdlib::uInt,
    first: ::core::ffi::c_int,
    size: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
    int_and_off64_are_same_size: bool,
    int_max: ::core::ffi::c_uint,
) -> GzZeroStep {
    match gz_zero_pending_step(pending_input) {
        GzZeroStep::FlushPending => GzZeroStep::FlushPending,
        GzZeroStep::WriteChunk { .. } => {
            gz_zero_chunk_step(first, size, skip, int_and_off64_are_same_size, int_max)
        }
    }
}

enum GzZeroAction {
    Error,
    Done,
    Continue,
}

struct GzZeroProgress {
    pos: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
    action: GzZeroAction,
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

fn gzclose_operation_error(
    result: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
) -> Option<::core::ffi::c_int> {
    (result == -1 as ::core::ffi::c_int).then_some(err)
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

fn gzwrite_request(len: ::core::ffi::c_uint) -> Option<crate::stdlib::z_size_t> {
    ((len as ::core::ffi::c_int) >= 0).then_some(len as crate::stdlib::z_size_t)
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

enum GzFlushAction {
    Compress,
    ReturnStateError,
}

fn gzflush_action(zero_result: Option<::core::ffi::c_int>) -> GzFlushAction {
    if zero_result == Some(-1) {
        GzFlushAction::ReturnStateError
    } else {
        GzFlushAction::Compress
    }
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

fn gz_write_comp_failed(result: ::core::ffi::c_int) -> bool {
    result == -1 as ::core::ffi::c_int
}

fn gz_write_buffered_comp_result(
    result: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    gz_write_comp_failed(result).then(|| gz_write_error_result(again, requested, remaining))
}

fn gz_write_uses_buffered_path(len: crate::stdlib::z_size_t, size: ::core::ffi::c_uint) -> bool {
    len < size as crate::stdlib::z_size_t
}

fn gz_write_is_empty(len: crate::stdlib::z_size_t) -> bool {
    len == 0
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzWritePreparation {
    Empty,
    Initialize,
    ZeroSkip,
    Ready,
}

fn gz_write_preparation(
    len: crate::stdlib::z_size_t,
    size: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
) -> GzWritePreparation {
    if gz_write_is_empty(len) {
        GzWritePreparation::Empty
    } else if !gz_buffer_is_initialized(size) {
        GzWritePreparation::Initialize
    } else if gz_has_pending_skip(skip) {
        GzWritePreparation::ZeroSkip
    } else {
        GzWritePreparation::Ready
    }
}

fn gz_has_pending_input(avail_in: crate::stdlib::uInt) -> bool {
    avail_in != 0
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzWriteBufferedInputAction {
    ResetBuffer,
    KeepPending,
}

fn gz_write_buffered_input_action(avail_in: crate::stdlib::uInt) -> GzWriteBufferedInputAction {
    if gz_has_pending_input(avail_in) {
        GzWriteBufferedInputAction::KeepPending
    } else {
        GzWriteBufferedInputAction::ResetBuffer
    }
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
struct GzWriteProgress {
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
}

fn gz_write_progress(
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
) -> GzWriteProgress {
    let consumed = gz_write_consumed(chunk_len, remaining_avail_in);

    GzWriteProgress {
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

fn gz_write_apply_direct_progress(
    pos: &mut crate::stdlib::off64_t,
    remaining: &mut crate::stdlib::z_size_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
    ret: ::core::ffi::c_int,
) -> GzWriteDirectAction {
    let progress = gz_write_progress(*pos, *remaining, chunk_len, remaining_avail_in);
    *pos = progress.pos;
    *remaining = progress.remaining;
    gz_write_direct_action(ret, *remaining)
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

fn gz_zero_progress(
    mut pos: crate::stdlib::off64_t,
    mut skip: crate::stdlib::off64_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
    ret: ::core::ffi::c_int,
) -> GzZeroProgress {
    let has_skip = gz_zero_apply_progress(&mut pos, &mut skip, chunk_len, remaining_avail_in);
    GzZeroProgress {
        pos,
        skip,
        action: gz_zero_action(ret, has_skip),
    }
}

fn gz_zero_apply_comp_progress(
    pos: &mut crate::stdlib::off64_t,
    skip: &mut crate::stdlib::off64_t,
    chunk_len: ::core::ffi::c_uint,
    remaining_avail_in: crate::stdlib::uInt,
    ret: ::core::ffi::c_int,
) -> GzZeroAction {
    let progress = gz_zero_progress(*pos, *skip, chunk_len, remaining_avail_in, ret);
    *pos = progress.pos;
    *skip = progress.skip;
    progress.action
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

#[derive(Debug, Eq, PartialEq)]
enum GzCompOutputBufferAction {
    Keep,
    Reset,
}

fn gz_comp_output_buffer_action(avail_out: crate::stdlib::uInt) -> GzCompOutputBufferAction {
    if avail_out == 0 {
        GzCompOutputBufferAction::Reset
    } else {
        GzCompOutputBufferAction::Keep
    }
}

#[derive(Debug, Eq, PartialEq)]
struct GzCompOutputBufferProgress {
    avail_out: crate::stdlib::uInt,
    out_pending: crate::stdlib::uInt,
    reset_cursor: bool,
}

fn gz_comp_output_buffer_progress(
    avail_out: crate::stdlib::uInt,
    size: ::core::ffi::c_uint,
    out_pending: crate::stdlib::uInt,
) -> GzCompOutputBufferProgress {
    match gz_comp_output_buffer_action(avail_out) {
        GzCompOutputBufferAction::Keep => GzCompOutputBufferProgress {
            avail_out,
            out_pending,
            reset_cursor: false,
        },
        GzCompOutputBufferAction::Reset => GzCompOutputBufferProgress {
            avail_out: size as crate::stdlib::uInt,
            out_pending: 0,
            reset_cursor: true,
        },
    }
}

fn gz_comp_apply_output_buffer_progress(
    avail_out: &mut crate::stdlib::uInt,
    out_pending: &mut crate::stdlib::uInt,
    size: ::core::ffi::c_uint,
) -> bool {
    let progress = gz_comp_output_buffer_progress(*avail_out, size, *out_pending);
    *avail_out = progress.avail_out;
    *out_pending = progress.out_pending;
    progress.reset_cursor
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

fn gz_comp_reset_value(
    action: &GzCompResetAction,
    current_reset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if matches!(action, GzCompResetAction::Reset) {
        0
    } else {
        current_reset
    }
}

fn gz_comp_reset_transition(
    reset: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> (GzCompResetAction, ::core::ffi::c_int) {
    let action = gz_comp_reset_action(reset, avail_in, flush);
    let next_reset = gz_comp_reset_value(&action, reset);
    (action, next_reset)
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
    crate::src::gzlib::gz_errno_is_retryable(errno)
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

#[derive(Debug, Eq, PartialEq)]
enum GzCompWriteResult {
    Written(::core::ffi::c_int),
    Error { again: ::core::ffi::c_int },
}

fn gz_comp_write_result(
    written: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> GzCompWriteResult {
    if gz_comp_write_failed(written) {
        GzCompWriteResult::Error {
            again: gz_comp_write_again(errno),
        }
    } else {
        GzCompWriteResult::Written(written)
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

#[derive(Debug, Eq, PartialEq)]
enum GzCompOutputFlushStep {
    Write { len: ::core::ffi::c_uint },
    Complete(GzCompOutputBufferProgress),
}

fn gz_comp_output_flush_step(
    avail_out: crate::stdlib::uInt,
    size: ::core::ffi::c_uint,
    pending: crate::stdlib::uInt,
    max: ::core::ffi::c_uint,
) -> GzCompOutputFlushStep {
    match gz_comp_output_write_chunk_len(pending, max) {
        Some(len) => GzCompOutputFlushStep::Write { len },
        None => GzCompOutputFlushStep::Complete(gz_comp_output_buffer_progress(
            avail_out, size, pending,
        )),
    }
}

#[derive(Debug, Eq, PartialEq)]
struct GzCompDirectWriteProgress {
    remaining_input: crate::stdlib::uInt,
    cursor_advance: usize,
}

struct GzCompWriteProgress {
    remaining: crate::stdlib::uInt,
    cursor_advance: usize,
}

fn gz_comp_write_progress(
    available: crate::stdlib::uInt,
    consumed: crate::stdlib::uInt,
    cursor_advance: usize,
) -> GzCompWriteProgress {
    GzCompWriteProgress {
        remaining: available.wrapping_sub(consumed),
        cursor_advance,
    }
}

fn gz_comp_direct_write_progress(
    avail_in: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
) -> GzCompDirectWriteProgress {
    let progress =
        gz_comp_write_progress(avail_in, written as crate::stdlib::uInt, written as usize);
    GzCompDirectWriteProgress {
        remaining_input: progress.remaining,
        cursor_advance: progress.cursor_advance,
    }
}

#[derive(Debug, Eq, PartialEq)]
enum GzCompDirectWriteResult {
    Error { again: ::core::ffi::c_int },
    Progress(GzCompDirectWriteProgress),
}

fn gz_comp_direct_write_result(
    avail_in: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> GzCompDirectWriteResult {
    match gz_comp_write_result(written, errno) {
        GzCompWriteResult::Error { again } => GzCompDirectWriteResult::Error { again },
        GzCompWriteResult::Written(written) => {
            GzCompDirectWriteResult::Progress(gz_comp_direct_write_progress(avail_in, written))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct GzCompOutputWriteProgress {
    remaining_pending: crate::stdlib::uInt,
    cursor_advance: usize,
}

fn gz_comp_output_write_progress(
    pending: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
) -> GzCompOutputWriteProgress {
    let progress =
        gz_comp_write_progress(pending, written as crate::stdlib::uInt, written as usize);
    GzCompOutputWriteProgress {
        remaining_pending: progress.remaining,
        cursor_advance: progress.cursor_advance,
    }
}

#[derive(Debug, Eq, PartialEq)]
enum GzCompOutputWriteResult {
    Error { again: ::core::ffi::c_int },
    Progress(GzCompOutputWriteProgress),
}

fn gz_comp_output_write_result(
    pending: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> GzCompOutputWriteResult {
    match gz_comp_write_result(written, errno) {
        GzCompWriteResult::Error { again } => GzCompOutputWriteResult::Error { again },
        GzCompWriteResult::Written(written) => {
            GzCompOutputWriteResult::Progress(gz_comp_output_write_progress(pending, written))
        }
    }
}

fn gz_comp_output_produced(
    avail_out_before: ::core::ffi::c_uint,
    avail_out_after: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    avail_out_before.wrapping_sub(avail_out_after)
}

struct GzCompDeflateProgress {
    produced: ::core::ffi::c_uint,
    pending: crate::stdlib::uInt,
}

fn gz_comp_deflate_progress(
    avail_out_before: ::core::ffi::c_uint,
    avail_out_after: ::core::ffi::c_uint,
    pending: crate::stdlib::uInt,
) -> GzCompDeflateProgress {
    let produced = gz_comp_output_produced(avail_out_before, avail_out_after);
    GzCompDeflateProgress {
        produced,
        pending: pending.wrapping_add(produced as crate::stdlib::uInt),
    }
}

fn gz_comp_apply_deflate_progress(
    pending: &mut crate::stdlib::uInt,
    avail_out_before: ::core::ffi::c_uint,
    avail_out_after: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let progress = gz_comp_deflate_progress(avail_out_before, avail_out_after, *pending);
    *pending = progress.pending;
    progress.produced
}

fn gz_comp_pending_after_write(
    pending: crate::stdlib::uInt,
    written: ::core::ffi::c_int,
) -> crate::stdlib::uInt {
    gz_comp_write_progress(pending, written as crate::stdlib::uInt, written as usize).remaining
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
enum GzCompDeflateAction {
    Error,
    Done,
    Continue,
}

fn gz_comp_deflate_action(
    ret: ::core::ffi::c_int,
    produced: ::core::ffi::c_uint,
) -> GzCompDeflateAction {
    if gz_comp_deflate_stream_is_corrupt(ret) {
        GzCompDeflateAction::Error
    } else if !gz_comp_has_output(produced) {
        GzCompDeflateAction::Done
    } else {
        GzCompDeflateAction::Continue
    }
}

fn gz_comp_apply_deflate_result(
    pending: &mut crate::stdlib::uInt,
    avail_out_before: ::core::ffi::c_uint,
    avail_out_after: ::core::ffi::c_uint,
    ret: ::core::ffi::c_int,
) -> GzCompDeflateAction {
    let produced = gz_comp_output_produced(avail_out_before, avail_out_after);
    let action = gz_comp_deflate_action(ret, produced);
    if matches!(action, GzCompDeflateAction::Continue) {
        gz_comp_apply_deflate_progress(pending, avail_out_before, avail_out_after);
    }
    action
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
    let write_progress = gz_write_progress(pos, remaining, copy, 0);

    GzWriteBufferedProgress {
        copy,
        avail_in: avail_in.wrapping_add(copy),
        have: have.wrapping_add(copy),
        pos: write_progress.pos,
        remaining: write_progress.remaining,
    }
}

fn gz_write_apply_buffered_progress(
    size: ::core::ffi::c_uint,
    avail_in: &mut crate::stdlib::uInt,
    have: &mut ::core::ffi::c_uint,
    pos: &mut crate::stdlib::off64_t,
    remaining: &mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_uint {
    let progress = gz_write_buffered_progress(size, *have, *avail_in, *pos, *remaining);
    *avail_in = progress.avail_in;
    *have = progress.have;
    *pos = progress.pos;
    *remaining = progress.remaining;
    progress.copy
}

#[derive(Debug, Eq, PartialEq)]
struct GzWriteBufferedCopyPlan {
    destination_offset: usize,
    copy: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
    reset_input_cursor: bool,
}

fn gz_write_buffered_copy_plan(
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    remaining: crate::stdlib::z_size_t,
) -> GzWriteBufferedCopyPlan {
    let reset_input_cursor = matches!(
        gz_write_buffered_input_action(avail_in),
        GzWriteBufferedInputAction::ResetBuffer
    );
    let mut avail_in = avail_in;
    let mut have = if reset_input_cursor { 0 } else { have };
    let mut pos = pos;
    let mut remaining = remaining;
    let copy =
        gz_write_apply_buffered_progress(size, &mut avail_in, &mut have, &mut pos, &mut remaining);

    GzWriteBufferedCopyPlan {
        destination_offset: if reset_input_cursor {
            0
        } else {
            have.wrapping_sub(copy) as usize
        },
        copy,
        avail_in,
        have,
        pos,
        remaining,
        reset_input_cursor,
    }
}

fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let allocation = gz_init_allocation_plan(state.want, state.direct);
    state.in_0 = unsafe {
        crate::stdlib::malloc(allocation.input_len as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar
    };
    if state.in_0.is_null() {
        unsafe {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return -1 as ::core::ffi::c_int;
    }
    if let Some(output_len) = allocation.output_len {
        state.out = unsafe {
            crate::stdlib::malloc(output_len as crate::__stddef_size_t_h::size_t)
                as *mut ::core::ffi::c_uchar
        };
        if state.out.is_null() {
            unsafe {
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        gz_init_stream_defaults(&mut state.strm);
        if gz_init_deflate_failed(unsafe {
            crate::src::deflate::deflateInit2_(
                &mut state.strm as *mut crate::zlib_h::z_stream_s,
                state.level,
                8 as ::core::ffi::c_int,
                15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
                state.strategy,
                crate::zlib_h::ZLIB_VERSION.as_ptr(),
                ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
            )
        }) {
            unsafe {
                crate::stdlib::free(state.out as *mut ::core::ffi::c_void);
                crate::stdlib::free(state.in_0 as *mut ::core::ffi::c_void);
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
    }
    gz_init_core(state);
    0 as ::core::ffi::c_int
}

unsafe fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = gz_comp_max_write_chunk();
    if !gz_buffer_is_initialized(state.size) {
        if gz_init_failed(gz_init(state)) {
            return -1 as ::core::ffi::c_int;
        }
    }
    if gz_comp_is_direct(state.direct) {
        while matches!(
            gz_comp_direct_loop_action(state.strm.avail_in),
            GzCompDirectLoopAction::Write
        ) {
            *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
            state.again = 0 as ::core::ffi::c_int;
            put = gz_comp_write_chunk_len(state.strm.avail_in as usize, max);
            writ = crate::stdlib::write(
                state.fd,
                state.strm.next_in as *const ::core::ffi::c_void,
                put as crate::__stddef_size_t_h::size_t,
            ) as ::core::ffi::c_int;
            let errno = *crate::stdlib::__errno_location();
            match gz_comp_direct_write_result(state.strm.avail_in, writ, errno) {
                GzCompDirectWriteResult::Error { again } => {
                    state.again = again;
                    crate::src::gzlib::gz_error(
                        state as *mut crate::gzguts_h::gz_state,
                        crate::zlib_h::Z_ERRNO,
                        crate::stdlib::strerror(errno),
                    );
                    return -1 as ::core::ffi::c_int;
                }
                GzCompDirectWriteResult::Progress(progress) => {
                    state.strm.avail_in = progress.remaining_input;
                    state.strm.next_in = state.strm.next_in.wrapping_add(progress.cursor_advance);
                }
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    let (reset_action, next_reset) =
        gz_comp_reset_transition(state.reset, state.strm.avail_in, flush);
    match reset_action {
        GzCompResetAction::Skip => return 0 as ::core::ffi::c_int,
        GzCompResetAction::Reset => {
            crate::src::deflate::deflateReset(&mut state.strm as *mut crate::zlib_h::z_stream_s);
        }
        GzCompResetAction::Continue => {}
    }
    state.reset = next_reset;
    ret = crate::zlib_h::Z_OK;
    loop {
        if gz_comp_needs_output_write(state.strm.avail_out, flush, ret) {
            loop {
                match gz_comp_output_flush_step(
                    state.strm.avail_out,
                    state.size,
                    state.out_pending,
                    max,
                ) {
                    GzCompOutputFlushStep::Write { len } => {
                        *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
                        state.again = 0 as ::core::ffi::c_int;
                        put = len;
                        writ = crate::stdlib::write(
                            state.fd,
                            state.x.next as *const ::core::ffi::c_void,
                            put as crate::__stddef_size_t_h::size_t,
                        ) as ::core::ffi::c_int;
                        let errno = *crate::stdlib::__errno_location();
                        match gz_comp_output_write_result(state.out_pending, writ, errno) {
                            GzCompOutputWriteResult::Error { again } => {
                                state.again = again;
                                crate::src::gzlib::gz_error(
                                    state as *mut crate::gzguts_h::gz_state,
                                    crate::zlib_h::Z_ERRNO,
                                    crate::stdlib::strerror(errno),
                                );
                                return -1 as ::core::ffi::c_int;
                            }
                            GzCompOutputWriteResult::Progress(progress) => {
                                state.x.next = state.x.next.wrapping_add(progress.cursor_advance);
                                state.out_pending = progress.remaining_pending;
                            }
                        }
                    }
                    GzCompOutputFlushStep::Complete(progress) => {
                        state.strm.avail_out = progress.avail_out;
                        state.out_pending = progress.out_pending;
                        if progress.reset_cursor {
                            state.strm.next_out = state.out;
                            state.x.next = state.out;
                        }
                        break;
                    }
                }
            }
        }
        have = state.strm.avail_out as ::core::ffi::c_uint;
        ret =
            crate::src::deflate::deflate(&mut state.strm as *mut crate::zlib_h::z_stream_s, flush);
        match gz_comp_apply_deflate_result(
            &mut state.out_pending,
            have,
            state.strm.avail_out as ::core::ffi::c_uint,
            ret,
        ) {
            GzCompDeflateAction::Error => {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: deflate stream corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            GzCompDeflateAction::Done => break,
            GzCompDeflateAction::Continue => {}
        }
    }
    state.reset = gz_comp_reset_after_flush(flush, state.reset);
    return 0 as ::core::ffi::c_int;
}

#[derive(Debug, Eq, PartialEq)]
struct GzZeroPreparedChunk {
    len: ::core::ffi::c_uint,
    initialize_buffer: bool,
}

fn gz_zero_chunk_plan(
    first: ::core::ffi::c_int,
    size: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
) -> GzZeroPreparedChunk {
    let limits = gz_zero_chunk_limits();
    let GzZeroStep::WriteChunk {
        len,
        initialize_buffer,
    } = gz_zero_chunk_step(
        first,
        size,
        skip,
        limits.int_and_off64_are_same_size,
        limits.int_max,
    )
    else {
        unreachable!();
    };

    GzZeroPreparedChunk {
        len,
        initialize_buffer,
    }
}

fn gz_zero_prepare_chunk(
    state: &mut crate::gzguts_h::gz_state,
    first: ::core::ffi::c_int,
) -> GzZeroPreparedChunk {
    let chunk = gz_zero_chunk_plan(first, state.size, state.skip);
    state.strm.avail_in = chunk.len as crate::stdlib::uInt;
    state.strm.next_in = state.in_0;

    chunk
}

fn gz_zero_prepare_and_initialize_chunk(
    state: &mut crate::gzguts_h::gz_state,
    buffer: &mut [crate::stdlib::Byte],
    first: &mut ::core::ffi::c_int,
) -> GzZeroPreparedChunk {
    let chunk = gz_zero_prepare_chunk(state, *first);
    if chunk.initialize_buffer {
        gz_zero_initialize_chunk_buffer(first, &mut buffer[..chunk.len as usize]);
    }
    chunk
}

unsafe fn gz_zero(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let buffer = if state.size == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(state.in_0, state.size as usize)
    };
    let mut first: ::core::ffi::c_int = 0;
    let limits = gz_zero_chunk_limits();
    match gz_zero_initial_step(
        state.strm.avail_in,
        first,
        state.size,
        state.skip,
        limits.int_and_off64_are_same_size,
        limits.int_max,
    ) {
        GzZeroStep::FlushPending => {
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH) == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        GzZeroStep::WriteChunk { .. } => {}
    }
    first = 1 as ::core::ffi::c_int;
    loop {
        let chunk = gz_zero_prepare_and_initialize_chunk(state, buffer, &mut first);
        let ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
        match gz_zero_apply_comp_progress(
            &mut state.x.pos,
            &mut state.skip,
            chunk.len,
            state.strm.avail_in,
            ret,
        ) {
            GzZeroAction::Error => return -1 as ::core::ffi::c_int,
            GzZeroAction::Done => break,
            GzZeroAction::Continue => {}
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_write(
    state: &mut crate::gzguts_h::gz_state,
    mut buf: crate::stdlib::voidpc,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let put: crate::stdlib::z_size_t = len;
    match gz_write_preparation(len, state.size, state.skip) {
        GzWritePreparation::Empty => return 0 as crate::stdlib::z_size_t,
        GzWritePreparation::Initialize => {
            if gz_init_failed(gz_init(state)) {
                return 0 as crate::stdlib::z_size_t;
            }
        }
        GzWritePreparation::ZeroSkip => {
            if gz_zero(state) == -1 as ::core::ffi::c_int {
                return 0 as crate::stdlib::z_size_t;
            }
        }
        GzWritePreparation::Ready => {}
    }
    if gz_write_uses_buffered_path(len, state.size) {
        loop {
            let plan = gz_write_buffered_copy_plan(
                state.size,
                state.strm.avail_in,
                state.x.have,
                state.x.pos,
                len,
            );
            if plan.reset_input_cursor {
                state.strm.next_in = state.in_0;
            }
            state.strm.avail_in = plan.avail_in;
            state.x.have = plan.have;
            state.x.pos = plan.pos;
            len = plan.remaining;
            crate::stdlib::memcpy(
                state.in_0.wrapping_add(plan.destination_offset) as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                plan.copy as crate::__stddef_size_t_h::size_t,
            );
            buf = (buf as *mut crate::stdlib::Bytef).wrapping_add(plan.copy as usize)
                as crate::stdlib::voidpc;
            if gz_write_is_empty(len) {
                break;
            }
            if let Some(result) = gz_write_buffered_comp_result(
                gz_comp(state, crate::zlib_h::Z_NO_FLUSH),
                state.again,
                put,
                len,
            ) {
                return result;
            }
        }
    } else {
        if gz_has_pending_input(state.strm.avail_in)
            && gz_write_comp_failed(gz_comp(state, crate::zlib_h::Z_NO_FLUSH))
        {
            return 0 as crate::stdlib::z_size_t;
        }
        state.strm.next_in = buf as *mut crate::stdlib::Bytef;
        loop {
            let n = gz_write_chunk_len(len);
            state.strm.avail_in = n as crate::stdlib::uInt;
            let ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH);
            match gz_write_apply_direct_progress(
                &mut state.x.pos,
                &mut len,
                n,
                state.strm.avail_in,
                ret,
            ) {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzSetParamsAction {
    ReturnOk,
    SetOnly,
    DeflateOnly,
    FlushThenDeflate,
}

fn gzsetparams_action(
    requested_level: ::core::ffi::c_int,
    current_level: ::core::ffi::c_int,
    requested_strategy: ::core::ffi::c_int,
    current_strategy: ::core::ffi::c_int,
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
) -> GzSetParamsAction {
    if gzsetparams_settings_match(
        requested_level,
        current_level,
        requested_strategy,
        current_strategy,
    ) {
        GzSetParamsAction::ReturnOk
    } else {
        match gzsetparams_buffer_action(size, avail_in) {
            GzSetParamsBufferAction::SetOnly => GzSetParamsAction::SetOnly,
            GzSetParamsBufferAction::DeflateOnly => GzSetParamsAction::DeflateOnly,
            GzSetParamsBufferAction::FlushThenDeflate => GzSetParamsAction::FlushThenDeflate,
        }
    }
}

fn gzsetparams_requires_deflate(action: GzSetParamsAction) -> bool {
    !matches!(
        action,
        GzSetParamsAction::ReturnOk | GzSetParamsAction::SetOnly
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzSetParamsZeroAction {
    Skip,
    Zero,
}

fn gzsetparams_zero_action(has_skip: bool) -> GzSetParamsZeroAction {
    if has_skip {
        GzSetParamsZeroAction::Zero
    } else {
        GzSetParamsZeroAction::Skip
    }
}

fn gzclose_mode_is_writable(mode: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_WRITE
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzCloseBufferAction {
    Keep,
    FreeBuffers { end_deflate: bool },
}

fn gzclose_buffer_action(
    size: ::core::ffi::c_uint,
    direct: ::core::ffi::c_int,
) -> GzCloseBufferAction {
    if !gz_buffer_is_initialized(size) {
        GzCloseBufferAction::Keep
    } else {
        GzCloseBufferAction::FreeBuffers {
            end_deflate: direct == 0,
        }
    }
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
    let state = &mut *state;
    if !gz_write_state_is_usable(state.mode, state.err, state.again) {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(len) = gzwrite_request(len) else {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_DATA_ERROR,
            b"requested length does not fit in int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    };
    return gz_write(state, buf, len) as ::core::ffi::c_int;
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
    let state = &mut *state;
    if !gz_write_state_is_usable(state.mode, state.err, state.again) {
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
    gzfwrite_result(size, len, gz_write(state, buf, len))
}
unsafe fn gzputc(mut file: crate::zlib_h::gzFile, mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let buf = [c as ::core::ffi::c_uchar];
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state = &mut *state;
    if !gz_write_state_is_usable(state.mode, state.err, state.again) {
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
unsafe fn gzputs(
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
    let state = &mut *state;
    if !gz_write_state_is_usable(state.mode, state.err, state.again) {
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
    let state = &mut *state;
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
    let zero_result = if gz_has_pending_skip((*state).skip) {
        Some(gz_zero(state))
    } else {
        None
    };
    if matches!(gzflush_action(zero_result), GzFlushAction::ReturnStateError) {
        return (*state).err;
    }
    gz_comp(state, flush);
    return (*state).err;
}
unsafe fn gzsetparams(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state = &mut *state;
    if !gzsetparams_state_is_usable(state.mode, state.err, state.again, state.direct) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let action = gzsetparams_action(
        level,
        state.level,
        strategy,
        state.strategy,
        state.size,
        state.strm.avail_in,
    );
    if matches!(action, GzSetParamsAction::ReturnOk) {
        return crate::zlib_h::Z_OK;
    }
    if matches!(
        gzsetparams_zero_action(gz_has_pending_skip(state.skip)),
        GzSetParamsZeroAction::Zero
    ) && gz_zero(state) == -1 as ::core::ffi::c_int
    {
        return state.err;
    }
    if matches!(action, GzSetParamsAction::FlushThenDeflate)
        && gz_comp(state, crate::zlib_h::Z_BLOCK) == -1 as ::core::ffi::c_int
    {
        return state.err;
    }
    if gzsetparams_requires_deflate(action) {
        crate::src::deflate::deflateParams(
            &mut state.strm as *mut crate::zlib_h::z_stream_s,
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
    gzsetparams(file, level, strategy)
}
pub unsafe extern "C" fn gzclose_w(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    let state = &mut *state;
    if !gzclose_mode_is_writable(state.mode) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let zero_error = if gz_has_pending_skip((*state).skip) {
        let result = gz_zero(state);
        gzclose_operation_error(result, (*state).err)
    } else {
        None
    };
    let result = gz_comp(state, crate::zlib_h::Z_FINISH);
    let finish_error = gzclose_operation_error(result, (*state).err);
    match gzclose_buffer_action((*state).size, (*state).direct) {
        GzCloseBufferAction::Keep => {}
        GzCloseBufferAction::FreeBuffers { end_deflate } => {
            if end_deflate {
                crate::src::deflate::deflateEnd(
                    &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                );
                crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            }
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
        }
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
    let close_failed = crate::stdlib::close((*state).fd) == -1 as ::core::ffi::c_int;
    crate::stdlib::free(state as *mut crate::gzguts_h::gz_state as *mut ::core::ffi::c_void);
    return gzclose_w_result(zero_error, finish_error, close_failed);
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_w(file)
}

#[cfg(test)]
mod tests {
    use super::{
        gz_buffer_is_initialized, gz_comp_apply_deflate_progress, gz_comp_apply_deflate_result,
        gz_comp_apply_output_buffer_progress, gz_comp_deflate_action, gz_comp_deflate_progress,
        gz_comp_deflate_stream_is_corrupt, gz_comp_direct_loop_action,
        gz_comp_direct_write_progress, gz_comp_direct_write_result, gz_comp_has_output,
        gz_comp_is_direct, gz_comp_max_write_chunk, gz_comp_needs_output_write,
        gz_comp_needs_reset, gz_comp_output_buffer_action, gz_comp_output_buffer_progress,
        gz_comp_output_flush_step, gz_comp_output_produced, gz_comp_output_write_chunk_len,
        gz_comp_output_write_progress, gz_comp_output_write_result, gz_comp_pending_after_write,
        gz_comp_reset_action, gz_comp_reset_after_flush, gz_comp_reset_transition,
        gz_comp_reset_value, gz_comp_skips_empty_flush, gz_comp_write_again,
        gz_comp_write_chunk_len, gz_comp_write_failed, gz_comp_write_failure,
        gz_comp_write_progress, gz_comp_write_result, gz_has_pending_input, gz_has_pending_skip,
        gz_init_allocation_plan, gz_init_deflate_failed, gz_init_failed, gz_init_mode,
        gz_init_stream_defaults, gz_write_advanced_pos, gz_write_apply_buffered_progress,
        gz_write_apply_chunk_progress, gz_write_apply_direct_progress,
        gz_write_buffered_comp_result, gz_write_buffered_copy_len, gz_write_buffered_copy_plan,
        gz_write_buffered_input_action, gz_write_buffered_progress, gz_write_chunk_len,
        gz_write_comp_failed, gz_write_consumed, gz_write_direct_action,
        gz_write_errno_is_retryable, gz_write_error_result, gz_write_is_empty,
        gz_write_preparation, gz_write_progress, gz_write_remaining_after_consumption,
        gz_write_state_is_usable, gz_write_uses_buffered_path, gz_zero_action,
        gz_zero_apply_comp_progress, gz_zero_apply_progress, gz_zero_chunk_len,
        gz_zero_chunk_limits, gz_zero_chunk_plan, gz_zero_chunk_step, gz_zero_initial_step,
        gz_zero_initialize_buffer, gz_zero_initialize_chunk_buffer, gz_zero_needs_initialization,
        gz_zero_pending_step, gz_zero_prepare_and_initialize_chunk, gz_zero_progress,
        gzclose_buffer_action, gzclose_mode_is_writable, gzclose_operation_error, gzclose_w_result,
        gzflush_action, gzflush_mode_is_valid, gzfwrite_result, gzputc_result, gzputc_write_action,
        gzputs_len_fits_int, gzputs_result, gzsetparams_action, gzsetparams_buffer_action,
        gzsetparams_requires_deflate, gzsetparams_settings_match, gzsetparams_state_is_usable,
        gzsetparams_zero_action, gzwrite_request, GzCloseBufferAction, GzCompDeflateAction,
        GzCompDirectLoopAction, GzCompDirectWriteProgress, GzCompDirectWriteResult,
        GzCompOutputBufferAction, GzCompOutputBufferProgress, GzCompOutputFlushStep,
        GzCompOutputWriteProgress, GzCompOutputWriteResult, GzCompResetAction, GzCompWriteFailure,
        GzCompWriteResult, GzFlushAction, GzInitAllocationPlan, GzInitMode, GzPutcWriteAction,
        GzSetParamsAction, GzSetParamsBufferAction, GzSetParamsZeroAction, GzWriteBufferedCopyPlan,
        GzWriteBufferedInputAction, GzWriteDirectAction, GzWritePreparation, GzZeroAction,
        GzZeroChunkLimits, GzZeroPreparedChunk, GzZeroStep,
    };

    #[test]
    fn gz_write_buffered_input_action_resets_when_no_pending_input() {
        assert_eq!(
            gz_write_buffered_input_action(0),
            GzWriteBufferedInputAction::ResetBuffer
        );
    }

    #[test]
    fn gz_write_buffered_input_action_keeps_pending_input() {
        assert_eq!(
            gz_write_buffered_input_action(1),
            GzWriteBufferedInputAction::KeepPending
        );
        assert_eq!(
            gz_write_buffered_input_action(crate::stdlib::uInt::MAX),
            GzWriteBufferedInputAction::KeepPending
        );
    }

    #[test]
    fn gzflush_action_returns_state_error_only_for_zero_fill_failure() {
        assert!(matches!(gzflush_action(None), GzFlushAction::Compress));
        assert!(matches!(
            gzflush_action(Some(-1)),
            GzFlushAction::ReturnStateError
        ));
        assert!(matches!(gzflush_action(Some(0)), GzFlushAction::Compress));
        assert!(matches!(
            gzflush_action(Some(::core::ffi::c_int::MIN)),
            GzFlushAction::Compress
        ));
    }

    #[test]
    fn gzclose_operation_error_captures_only_failure_sentinel() {
        assert_eq!(
            gzclose_operation_error(-1, crate::zlib_h::Z_ERRNO),
            Some(crate::zlib_h::Z_ERRNO)
        );
        assert_eq!(gzclose_operation_error(0, crate::zlib_h::Z_ERRNO), None);
        assert_eq!(
            gzclose_operation_error(crate::zlib_h::Z_STREAM_END, crate::zlib_h::Z_ERRNO),
            None
        );
    }

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
    fn gzclose_buffer_action_keeps_uninitialized_buffers() {
        assert_eq!(gzclose_buffer_action(0, 0), GzCloseBufferAction::Keep);
        assert_eq!(gzclose_buffer_action(0, 1), GzCloseBufferAction::Keep);
    }

    #[test]
    fn gzclose_buffer_action_frees_only_input_for_direct_writes() {
        assert_eq!(
            gzclose_buffer_action(1, 1),
            GzCloseBufferAction::FreeBuffers { end_deflate: false }
        );
    }

    #[test]
    fn gzclose_buffer_action_ends_deflate_for_buffered_writes() {
        assert_eq!(
            gzclose_buffer_action(1, 0),
            GzCloseBufferAction::FreeBuffers { end_deflate: true }
        );
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
    fn gz_zero_initialize_buffer_clears_every_byte() {
        let mut buffer = [0xff; 4];

        gz_zero_initialize_buffer(&mut buffer);

        assert_eq!(buffer, [0; 4]);
    }

    #[test]
    fn gz_zero_initialize_chunk_buffer_clears_buffer_and_marks_follow_up_chunks() {
        let mut first = 1;
        let mut buffer = [0xff; 4];

        gz_zero_initialize_chunk_buffer(&mut first, &mut buffer);

        assert_eq!(first, 0);
        assert_eq!(buffer, [0; 4]);
    }

    #[test]
    fn gz_zero_initialize_chunk_buffer_marks_follow_up_chunks_for_empty_buffers() {
        let mut first = ::core::ffi::c_int::MAX;
        let mut buffer = [];

        gz_zero_initialize_chunk_buffer(&mut first, &mut buffer);

        assert_eq!(first, 0);
    }

    #[test]
    fn gz_zero_chunk_limits_match_current_platform() {
        assert_eq!(
            gz_zero_chunk_limits(),
            GzZeroChunkLimits {
                int_and_off64_are_same_size: ::core::mem::size_of::<::core::ffi::c_int>()
                    == ::core::mem::size_of::<crate::stdlib::off64_t>(),
                int_max: crate::src::gzlib::gz_intmax(),
            }
        );
    }

    #[test]
    fn gz_zero_chunk_limits_drive_zero_chunk_setup() {
        let limits = gz_zero_chunk_limits();

        assert!(matches!(
            gz_zero_chunk_step(
                1,
                1024,
                99,
                limits.int_and_off64_are_same_size,
                limits.int_max,
            ),
            GzZeroStep::WriteChunk {
                len: 99,
                initialize_buffer: true,
            }
        ));
    }

    #[test]
    fn gz_zero_chunk_plan_preserves_first_pass_and_remaining_skip() {
        assert_eq!(
            gz_zero_chunk_plan(1, 1024, 99),
            GzZeroPreparedChunk {
                len: 99,
                initialize_buffer: true,
            }
        );
        assert_eq!(
            gz_zero_chunk_plan(0, 1024, 99),
            GzZeroPreparedChunk {
                len: 99,
                initialize_buffer: false,
            }
        );
    }

    #[test]
    fn gz_zero_pending_step_flushes_only_for_pending_input() {
        assert!(matches!(gz_zero_pending_step(1), GzZeroStep::FlushPending));
        assert!(matches!(
            gz_zero_pending_step(0),
            GzZeroStep::WriteChunk {
                len: 0,
                initialize_buffer: false,
            }
        ));
    }

    #[test]
    fn gz_zero_chunk_step_plans_initial_and_follow_up_chunks() {
        assert!(matches!(
            gz_zero_chunk_step(1, 1024, 99, false, 0),
            GzZeroStep::WriteChunk {
                len: 99,
                initialize_buffer: true,
            }
        ));
        assert!(matches!(
            gz_zero_chunk_step(0, 1024, 2048, false, 0),
            GzZeroStep::WriteChunk {
                len: 1024,
                initialize_buffer: false,
            }
        ));
    }

    #[test]
    fn gz_zero_initial_step_prioritizes_pending_input_over_chunk_setup() {
        assert!(matches!(
            gz_zero_initial_step(1, 1, 1024, 99, false, 0),
            GzZeroStep::FlushPending
        ));
        assert!(matches!(
            gz_zero_initial_step(0, 1, 1024, 99, false, 0),
            GzZeroStep::WriteChunk {
                len: 99,
                initialize_buffer: true,
            }
        ));
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
    fn gz_init_deflate_failed_accepts_only_z_ok() {
        assert!(!gz_init_deflate_failed(crate::zlib_h::Z_OK));
        assert!(gz_init_deflate_failed(crate::zlib_h::Z_STREAM_ERROR));
        assert!(gz_init_deflate_failed(crate::zlib_h::Z_MEM_ERROR));
    }

    #[test]
    fn gz_init_failed_accepts_only_the_failure_sentinel() {
        assert!(gz_init_failed(-1));
        assert!(!gz_init_failed(0));
        assert!(!gz_init_failed(crate::zlib_h::Z_MEM_ERROR));
    }

    #[test]
    fn gz_init_mode_treats_only_zero_as_compressed() {
        assert_eq!(gz_init_mode(0), GzInitMode::Compressed);
        assert_eq!(gz_init_mode(1), GzInitMode::Direct);
        assert_eq!(gz_init_mode(-1), GzInitMode::Direct);
        assert_eq!(gz_init_mode(::core::ffi::c_int::MIN), GzInitMode::Direct);
        assert_eq!(gz_init_mode(::core::ffi::c_int::MAX), GzInitMode::Direct);
    }

    #[test]
    fn gz_comp_is_direct_treats_only_zero_as_compressed() {
        assert!(!gz_comp_is_direct(0));
        assert!(gz_comp_is_direct(1));
        assert!(gz_comp_is_direct(-1));
        assert!(gz_comp_is_direct(::core::ffi::c_int::MIN));
        assert!(gz_comp_is_direct(::core::ffi::c_int::MAX));
    }

    #[test]
    fn gz_init_allocation_plan_allocates_only_input_for_direct_writes() {
        assert_eq!(
            gz_init_allocation_plan(4096, -1),
            GzInitAllocationPlan {
                input_len: 8192,
                output_len: None,
            }
        );
    }

    #[test]
    fn gz_init_allocation_plan_allocates_input_and_output_for_compressed_writes() {
        assert_eq!(
            gz_init_allocation_plan(4096, 0),
            GzInitAllocationPlan {
                input_len: 8192,
                output_len: Some(4096),
            }
        );
    }

    #[test]
    fn gz_init_allocation_plan_preserves_input_size_wrapping() {
        assert_eq!(
            gz_init_allocation_plan(::core::ffi::c_uint::MAX, 0),
            GzInitAllocationPlan {
                input_len: ::core::ffi::c_uint::MAX.wrapping_shl(1) as crate::stdlib::z_size_t,
                output_len: Some(::core::ffi::c_uint::MAX as crate::stdlib::z_size_t),
            }
        );
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
    fn gzsetparams_action_prioritizes_matching_settings() {
        assert_eq!(
            gzsetparams_action(1, 1, 2, 2, 1, 1),
            GzSetParamsAction::ReturnOk
        );
    }

    #[test]
    fn gzsetparams_action_selects_the_required_buffer_handling() {
        assert_eq!(
            gzsetparams_action(1, 2, 3, 4, 0, 1),
            GzSetParamsAction::SetOnly
        );
        assert_eq!(
            gzsetparams_action(1, 2, 3, 4, 1, 0),
            GzSetParamsAction::DeflateOnly
        );
        assert_eq!(
            gzsetparams_action(1, 2, 3, 4, 1, 1),
            GzSetParamsAction::FlushThenDeflate
        );
    }

    #[test]
    fn gzsetparams_requires_deflate_only_for_deflate_actions() {
        assert!(!gzsetparams_requires_deflate(GzSetParamsAction::ReturnOk));
        assert!(!gzsetparams_requires_deflate(GzSetParamsAction::SetOnly));
        assert!(gzsetparams_requires_deflate(GzSetParamsAction::DeflateOnly));
        assert!(gzsetparams_requires_deflate(
            GzSetParamsAction::FlushThenDeflate
        ));
    }

    #[test]
    fn gzsetparams_zero_action_only_zero_fills_pending_skip() {
        assert_eq!(gzsetparams_zero_action(false), GzSetParamsZeroAction::Skip);
        assert_eq!(gzsetparams_zero_action(true), GzSetParamsZeroAction::Zero);
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
    fn gz_comp_direct_loop_action_writes_only_with_pending_input() {
        assert_eq!(gz_comp_direct_loop_action(0), GzCompDirectLoopAction::Done);
        assert_eq!(gz_comp_direct_loop_action(1), GzCompDirectLoopAction::Write);
        assert_eq!(
            gz_comp_direct_loop_action(crate::stdlib::uInt::MAX),
            GzCompDirectLoopAction::Write
        );
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
    fn gz_comp_write_result_preserves_successful_write_counts() {
        assert_eq!(
            gz_comp_write_result(0, crate::stdlib::EAGAIN),
            GzCompWriteResult::Written(0)
        );
        assert_eq!(gz_comp_write_result(24, 0), GzCompWriteResult::Written(24));
    }

    #[test]
    fn gz_comp_write_result_maps_retryable_and_fatal_errors() {
        assert_eq!(
            gz_comp_write_result(-1, crate::stdlib::EAGAIN),
            GzCompWriteResult::Error { again: 1 }
        );
        assert_eq!(
            gz_comp_write_result(-1, 1),
            GzCompWriteResult::Error { again: 0 }
        );
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
    fn gz_comp_output_buffer_action_resets_exhausted_buffers() {
        assert_eq!(
            gz_comp_output_buffer_action(0),
            GzCompOutputBufferAction::Reset
        );
    }

    #[test]
    fn gz_comp_output_buffer_action_keeps_available_buffers() {
        assert_eq!(
            gz_comp_output_buffer_action(1),
            GzCompOutputBufferAction::Keep
        );
        assert_eq!(
            gz_comp_output_buffer_action(crate::stdlib::uInt::MAX),
            GzCompOutputBufferAction::Keep
        );
    }

    #[test]
    fn gz_comp_output_buffer_progress_preserves_available_output() {
        assert_eq!(
            gz_comp_output_buffer_progress(17, 128, 23),
            GzCompOutputBufferProgress {
                avail_out: 17,
                out_pending: 23,
                reset_cursor: false,
            }
        );
    }

    #[test]
    fn gz_comp_output_buffer_progress_resets_exhausted_output() {
        assert_eq!(
            gz_comp_output_buffer_progress(0, ::core::ffi::c_uint::MAX, 23),
            GzCompOutputBufferProgress {
                avail_out: ::core::ffi::c_uint::MAX,
                out_pending: 0,
                reset_cursor: true,
            }
        );
    }

    #[test]
    fn gz_comp_apply_output_buffer_progress_preserves_available_output() {
        let mut avail_out = 17;
        let mut out_pending = 23;

        assert!(!gz_comp_apply_output_buffer_progress(
            &mut avail_out,
            &mut out_pending,
            128,
        ));
        assert_eq!(avail_out, 17);
        assert_eq!(out_pending, 23);
    }

    #[test]
    fn gz_comp_apply_output_buffer_progress_resets_exhausted_output() {
        let mut avail_out = 0;
        let mut out_pending = 23;

        assert!(gz_comp_apply_output_buffer_progress(
            &mut avail_out,
            &mut out_pending,
            ::core::ffi::c_uint::MAX,
        ));
        assert_eq!(avail_out, ::core::ffi::c_uint::MAX);
        assert_eq!(out_pending, 0);
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
    fn gz_comp_reset_transition_preserves_reset_state_and_action() {
        assert!(matches!(
            gz_comp_reset_transition(1, 0, crate::zlib_h::Z_NO_FLUSH),
            (GzCompResetAction::Skip, 1)
        ));
        assert!(matches!(
            gz_comp_reset_transition(-1, 0, crate::zlib_h::Z_NO_FLUSH),
            (GzCompResetAction::Skip, -1)
        ));
        assert!(matches!(
            gz_comp_reset_transition(1, 1, crate::zlib_h::Z_NO_FLUSH),
            (GzCompResetAction::Reset, 0)
        ));
        assert!(matches!(
            gz_comp_reset_transition(1, 0, crate::zlib_h::Z_BLOCK),
            (GzCompResetAction::Reset, 0)
        ));
        assert!(matches!(
            gz_comp_reset_transition(0, 0, crate::zlib_h::Z_NO_FLUSH),
            (GzCompResetAction::Continue, 0)
        ));
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
    fn gz_comp_reset_value_clears_only_after_reset_action() {
        assert_eq!(gz_comp_reset_value(&GzCompResetAction::Reset, 1), 0);
        assert_eq!(gz_comp_reset_value(&GzCompResetAction::Reset, -1), 0);
        assert_eq!(gz_comp_reset_value(&GzCompResetAction::Continue, 1), 1);
        assert_eq!(gz_comp_reset_value(&GzCompResetAction::Skip, -1), -1);
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
    fn gz_comp_output_flush_step_writes_pending_output_before_buffer_transition() {
        assert_eq!(
            gz_comp_output_flush_step(0, 1024, 100, 64),
            GzCompOutputFlushStep::Write { len: 64 }
        );
    }

    #[test]
    fn gz_comp_output_flush_step_resets_exhausted_output_buffer_after_writes() {
        assert_eq!(
            gz_comp_output_flush_step(0, 1024, 0, 64),
            GzCompOutputFlushStep::Complete(GzCompOutputBufferProgress {
                avail_out: 1024,
                out_pending: 0,
                reset_cursor: true,
            })
        );
    }

    #[test]
    fn gz_comp_output_flush_step_preserves_partial_output_buffer_without_pending_bytes() {
        assert_eq!(
            gz_comp_output_flush_step(24, 1024, 0, 64),
            GzCompOutputFlushStep::Complete(GzCompOutputBufferProgress {
                avail_out: 24,
                out_pending: 0,
                reset_cursor: false,
            })
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
    fn gz_comp_write_progress_preserves_wrapping_accounting_and_cursor() {
        let progress = gz_comp_write_progress(0, 1, 24);

        assert_eq!(progress.remaining, crate::stdlib::uInt::MAX);
        assert_eq!(progress.cursor_advance, 24);
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
    fn gz_comp_direct_write_result_preserves_zero_write_retry_behavior() {
        assert_eq!(
            gz_comp_direct_write_result(1024, 0, crate::stdlib::EAGAIN),
            GzCompDirectWriteResult::Progress(GzCompDirectWriteProgress {
                remaining_input: 1024,
                cursor_advance: 0,
            })
        );
    }

    #[test]
    fn gz_comp_direct_write_result_composes_retryable_error_and_progress() {
        assert_eq!(
            gz_comp_direct_write_result(1024, -1, crate::stdlib::EAGAIN),
            GzCompDirectWriteResult::Error { again: 1 }
        );
        assert_eq!(
            gz_comp_direct_write_result(1024, 24, 0),
            GzCompDirectWriteResult::Progress(GzCompDirectWriteProgress {
                remaining_input: 1000,
                cursor_advance: 24,
            })
        );
        assert_eq!(
            gz_comp_direct_write_result(0, 1, 0),
            GzCompDirectWriteResult::Progress(GzCompDirectWriteProgress {
                remaining_input: crate::stdlib::uInt::MAX,
                cursor_advance: 1,
            })
        );
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
    fn gz_comp_output_write_result_composes_errors_and_progress() {
        assert_eq!(
            gz_comp_output_write_result(1024, -1, crate::stdlib::EAGAIN),
            GzCompOutputWriteResult::Error { again: 1 }
        );
        assert_eq!(
            gz_comp_output_write_result(1024, -1, 0),
            GzCompOutputWriteResult::Error { again: 0 }
        );
        assert_eq!(
            gz_comp_output_write_result(1024, 0, crate::stdlib::EAGAIN),
            GzCompOutputWriteResult::Progress(GzCompOutputWriteProgress {
                remaining_pending: 1024,
                cursor_advance: 0,
            })
        );
        assert_eq!(
            gz_comp_output_write_result(1024, 24, 0),
            GzCompOutputWriteResult::Progress(GzCompOutputWriteProgress {
                remaining_pending: 1000,
                cursor_advance: 24,
            })
        );
        assert_eq!(
            gz_comp_output_write_result(0, 1, 0),
            GzCompOutputWriteResult::Progress(GzCompOutputWriteProgress {
                remaining_pending: crate::stdlib::uInt::MAX,
                cursor_advance: 1,
            })
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
    fn gz_comp_deflate_progress_tracks_produced_and_pending_bytes() {
        let progress = gz_comp_deflate_progress(1024, 24, 100);

        assert_eq!(progress.produced, 1000);
        assert_eq!(progress.pending, 1100);
        assert_eq!(gz_comp_pending_after_write(progress.pending, 20), 1080);
    }

    #[test]
    fn gz_comp_apply_deflate_progress_updates_pending_and_returns_produced_bytes() {
        let mut pending = 100;

        assert_eq!(gz_comp_apply_deflate_progress(&mut pending, 1024, 24), 1000);
        assert_eq!(pending, 1100);
    }

    #[test]
    fn gz_comp_apply_deflate_progress_preserves_wrapping_accounting() {
        let mut pending = crate::stdlib::uInt::MAX;

        assert_eq!(
            gz_comp_apply_deflate_progress(&mut pending, 0, 1),
            ::core::ffi::c_uint::MAX
        );
        assert_eq!(pending, crate::stdlib::uInt::MAX.wrapping_sub(1));
    }

    #[test]
    fn gz_comp_apply_deflate_result_accounts_only_continuing_output() {
        let mut pending = 100;

        assert_eq!(
            gz_comp_apply_deflate_result(&mut pending, 1024, 24, crate::zlib_h::Z_OK),
            GzCompDeflateAction::Continue
        );
        assert_eq!(pending, 1100);
    }

    #[test]
    fn gz_comp_apply_deflate_result_preserves_pending_when_done_or_corrupt() {
        let mut pending = 100;

        assert_eq!(
            gz_comp_apply_deflate_result(&mut pending, 1024, 1024, crate::zlib_h::Z_OK),
            GzCompDeflateAction::Done
        );
        assert_eq!(pending, 100);

        assert_eq!(
            gz_comp_apply_deflate_result(&mut pending, 1024, 24, crate::zlib_h::Z_STREAM_ERROR),
            GzCompDeflateAction::Error
        );
        assert_eq!(pending, 100);
    }

    #[test]
    fn gz_comp_deflate_progress_handles_zero_and_wrapping_output() {
        let empty = gz_comp_deflate_progress(1024, 1024, 9);
        assert_eq!(empty.produced, 0);
        assert_eq!(empty.pending, 9);

        let wrapping = gz_comp_deflate_progress(0, 1, crate::stdlib::uInt::MAX);
        assert_eq!(wrapping.produced, ::core::ffi::c_uint::MAX);
        assert_eq!(wrapping.pending, crate::stdlib::uInt::MAX.wrapping_sub(1));
    }

    #[test]
    fn gz_comp_deflate_progress_preserves_pending_wrapping_accounting() {
        assert_eq!(
            gz_comp_deflate_progress(1, 0, crate::stdlib::uInt::MAX).pending,
            0
        );
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
    fn gz_comp_deflate_action_prioritizes_stream_errors_over_output() {
        assert_eq!(
            gz_comp_deflate_action(crate::zlib_h::Z_STREAM_ERROR, 1),
            GzCompDeflateAction::Error
        );
    }

    #[test]
    fn gz_comp_deflate_action_finishes_without_output() {
        assert_eq!(
            gz_comp_deflate_action(crate::zlib_h::Z_OK, 0),
            GzCompDeflateAction::Done
        );
        assert_eq!(
            gz_comp_deflate_action(crate::zlib_h::Z_STREAM_END, 0),
            GzCompDeflateAction::Done
        );
    }

    #[test]
    fn gz_comp_deflate_action_continues_when_output_is_available() {
        assert_eq!(
            gz_comp_deflate_action(crate::zlib_h::Z_OK, 1),
            GzCompDeflateAction::Continue
        );
        assert_eq!(
            gz_comp_deflate_action(crate::zlib_h::Z_STREAM_END, ::core::ffi::c_uint::MAX),
            GzCompDeflateAction::Continue
        );
    }

    #[test]
    fn gzwrite_request_converts_only_signed_int_representable_lengths() {
        let largest_valid = ::core::ffi::c_int::MAX as ::core::ffi::c_uint;

        assert_eq!(gzwrite_request(0), Some(0));
        assert_eq!(gzwrite_request(largest_valid), Some(largest_valid as _));
        assert_eq!(gzwrite_request(largest_valid.wrapping_add(1)), None);
        assert_eq!(gzwrite_request(::core::ffi::c_uint::MAX), None);
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
    fn gz_write_comp_failed_matches_only_the_failure_sentinel() {
        assert!(gz_write_comp_failed(-1));
        assert!(!gz_write_comp_failed(0));
        assert!(!gz_write_comp_failed(1));
        assert!(!gz_write_comp_failed(::core::ffi::c_int::MIN));
    }

    #[test]
    fn gz_write_buffered_comp_result_returns_partial_count_only_on_retryable_failures() {
        assert_eq!(gz_write_buffered_comp_result(-1, 1, 10, 4), Some(6));
        assert_eq!(gz_write_buffered_comp_result(-1, 0, 10, 4), Some(0));
    }

    #[test]
    fn gz_write_buffered_comp_result_continues_after_successful_compression() {
        assert_eq!(gz_write_buffered_comp_result(0, 1, 10, 4), None);
        assert_eq!(gz_write_buffered_comp_result(1, 0, 10, 4), None);
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
    fn gz_write_preparation_preserves_initialization_and_skip_order() {
        assert_eq!(gz_write_preparation(0, 0, 1), GzWritePreparation::Empty);
        assert_eq!(
            gz_write_preparation(1, 0, 1),
            GzWritePreparation::Initialize
        );
        assert_eq!(gz_write_preparation(1, 1, 1), GzWritePreparation::ZeroSkip);
        assert_eq!(gz_write_preparation(1, 1, 0), GzWritePreparation::Ready);
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
    fn gz_write_apply_buffered_progress_updates_all_scalars() {
        let mut avail_in = 17;
        let mut have = 1000;
        let mut pos = 10;
        let mut remaining = 99;

        let copy = gz_write_apply_buffered_progress(
            1024,
            &mut avail_in,
            &mut have,
            &mut pos,
            &mut remaining,
        );

        assert_eq!(copy, 24);
        assert_eq!(avail_in, 41);
        assert_eq!(have, 1024);
        assert_eq!(pos, 34);
        assert_eq!(remaining, 75);
    }

    #[test]
    fn gz_write_apply_buffered_progress_preserves_wrapping_accounting() {
        let mut avail_in = 1;
        let mut have = ::core::ffi::c_uint::MAX;
        let mut pos = 0;
        let mut remaining = 5;

        let copy =
            gz_write_apply_buffered_progress(0, &mut avail_in, &mut have, &mut pos, &mut remaining);

        assert_eq!(copy, 1);
        assert_eq!(avail_in, 2);
        assert_eq!(have, 0);
        assert_eq!(pos, 1);
        assert_eq!(remaining, 4);
    }

    #[test]
    fn gz_write_buffered_copy_plan_resets_empty_input_buffer() {
        assert_eq!(
            gz_write_buffered_copy_plan(1024, 0, 1000, 10, 99),
            GzWriteBufferedCopyPlan {
                destination_offset: 0,
                copy: 99,
                avail_in: 99,
                have: 99,
                pos: 109,
                remaining: 0,
                reset_input_cursor: true,
            }
        );
    }

    #[test]
    fn gz_write_buffered_copy_plan_preserves_pending_input() {
        assert_eq!(
            gz_write_buffered_copy_plan(1024, 17, 1000, 10, 99),
            GzWriteBufferedCopyPlan {
                destination_offset: 1000,
                copy: 24,
                avail_in: 41,
                have: 1024,
                pos: 34,
                remaining: 75,
                reset_input_cursor: false,
            }
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
    fn gz_write_progress_accounts_for_partial_consumption() {
        let progress = gz_write_progress(10, 100, 80, 20);

        assert_eq!(progress.pos, 70);
        assert_eq!(progress.remaining, 40);
    }

    #[test]
    fn gz_write_progress_reports_input_exhaustion() {
        let progress = gz_write_progress(10, 80, 80, 0);

        assert_eq!(progress.pos, 90);
        assert_eq!(progress.remaining, 0);
    }

    #[test]
    fn gz_write_progress_preserves_wrapping_accounting() {
        let progress =
            gz_write_progress(0, ::core::ffi::c_uint::MAX as crate::stdlib::z_size_t, 0, 1);

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
    fn gz_write_apply_direct_progress_updates_state_before_reporting_errors() {
        let mut pos = 10;
        let mut remaining = 100;

        assert!(matches!(
            gz_write_apply_direct_progress(&mut pos, &mut remaining, 80, 20, -1),
            GzWriteDirectAction::Error
        ));
        assert_eq!(pos, 70);
        assert_eq!(remaining, 40);
    }

    #[test]
    fn gz_write_apply_direct_progress_reports_completion_after_consumption() {
        let mut pos = 10;
        let mut remaining = 80;

        assert!(matches!(
            gz_write_apply_direct_progress(&mut pos, &mut remaining, 80, 0, 0),
            GzWriteDirectAction::Done
        ));
        assert_eq!(pos, 90);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn gz_write_apply_direct_progress_continues_for_partial_writes() {
        let mut pos = 10;
        let mut remaining = 100;

        assert!(matches!(
            gz_write_apply_direct_progress(&mut pos, &mut remaining, 80, 20, 0),
            GzWriteDirectAction::Continue
        ));
        assert_eq!(pos, 70);
        assert_eq!(remaining, 40);
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
    fn gz_zero_progress_applies_state_before_selecting_the_next_action() {
        let partial = gz_zero_progress(10, 100, 80, 20, 0);
        assert_eq!(partial.pos, 70);
        assert_eq!(partial.skip, 40);
        assert!(matches!(partial.action, GzZeroAction::Continue));

        let exhausted = gz_zero_progress(10, 80, 80, 0, 0);
        assert_eq!(exhausted.pos, 90);
        assert_eq!(exhausted.skip, 0);
        assert!(matches!(exhausted.action, GzZeroAction::Done));

        let failed = gz_zero_progress(10, 80, 80, 0, -1);
        assert_eq!(failed.pos, 90);
        assert_eq!(failed.skip, 0);
        assert!(matches!(failed.action, GzZeroAction::Error));
    }

    #[test]
    fn gz_zero_apply_comp_progress_updates_state_and_continues() {
        let mut pos = 10;
        let mut skip = 100;

        assert!(matches!(
            gz_zero_apply_comp_progress(&mut pos, &mut skip, 80, 20, 0),
            GzZeroAction::Continue
        ));
        assert_eq!(pos, 70);
        assert_eq!(skip, 40);
    }

    #[test]
    fn gz_zero_apply_comp_progress_reports_completion_after_updating_state() {
        let mut pos = 10;
        let mut skip = 80;

        assert!(matches!(
            gz_zero_apply_comp_progress(&mut pos, &mut skip, 80, 0, 0),
            GzZeroAction::Done
        ));
        assert_eq!(pos, 90);
        assert_eq!(skip, 0);
    }

    #[test]
    fn gz_zero_apply_comp_progress_prioritizes_errors_after_updating_state() {
        let mut pos = 10;
        let mut skip = 80;

        assert!(matches!(
            gz_zero_apply_comp_progress(&mut pos, &mut skip, 80, 0, -1),
            GzZeroAction::Error
        ));
        assert_eq!(pos, 90);
        assert_eq!(skip, 0);
    }

    #[test]
    fn gz_zero_prepare_and_initialize_chunk_zero_fills_once() {
        let mut buffer = [0xff; 4];
        let mut state = crate::gzguts_h::gz_state {
            x: crate::zlib_h::gzFile_s {
                have: 0,
                next: ::core::ptr::null_mut(),
                pos: 10,
            },
            mode: 0,
            fd: 0,
            path: ::core::ptr::null_mut(),
            size: buffer.len() as ::core::ffi::c_uint,
            want: 0,
            in_0: buffer.as_mut_ptr(),
            out: ::core::ptr::null_mut(),
            direct: 0,
            junk: 0,
            how: 0,
            again: 0,
            start: 0,
            eof: 0,
            past: 0,
            level: 0,
            strategy: 0,
            reset: 0,
            skip: 6,
            err: 0,
            msg: ::core::ptr::null_mut(),
            strm: crate::zlib_h::z_stream {
                next_in: ::core::ptr::null_mut(),
                avail_in: 0,
                total_in: 0,
                next_out: ::core::ptr::null_mut(),
                avail_out: 0,
                total_out: 0,
                msg: ::core::ptr::null_mut(),
                state: ::core::ptr::null_mut(),
                zalloc: None,
                zfree: None,
                opaque: ::core::ptr::null_mut(),
                data_type: 0,
                adler: 0,
                reserved: 0,
            },
            out_pending: 0,
        };
        let mut first = 1;

        let initial = gz_zero_prepare_and_initialize_chunk(&mut state, &mut buffer, &mut first);
        assert_eq!(initial.len, 4);
        assert!(initial.initialize_buffer);
        assert_eq!(buffer, [0; 4]);
        assert_eq!(first, 0);
        assert_eq!(state.strm.avail_in, 4);
        assert_eq!(state.strm.next_in, state.in_0);

        state.skip = 2;
        let final_chunk = gz_zero_prepare_and_initialize_chunk(&mut state, &mut buffer, &mut first);
        assert_eq!(final_chunk.len, 2);
        assert!(!final_chunk.initialize_buffer);
        assert_eq!(buffer, [0; 4]);
        assert_eq!(state.strm.avail_in, 2);
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
