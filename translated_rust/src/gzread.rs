pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
pub use crate::src::gzlib::gz_error;
pub use crate::src::gzlib::gz_intmax;

pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
pub use crate::src::inflate::inflateEnd_ffi as inflateEnd;
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

#[derive(Debug, PartialEq, Eq)]
struct GzLoadState {
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    more: bool,
    error: Option<::core::ffi::c_int>,
}

struct GzLoadResult {
    have: ::core::ffi::c_uint,
    failed: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum GzLoadTransition {
    Error {
        have: ::core::ffi::c_uint,
        errno: ::core::ffi::c_int,
    },
    Continue {
        have: ::core::ffi::c_uint,
    },
    Complete {
        have: ::core::ffi::c_uint,
    },
}

#[derive(Debug, PartialEq, Eq)]
struct GzLoadError {
    have: ::core::ffi::c_uint,
    errno: ::core::ffi::c_int,
}

fn gz_load_checked_have(
    have: ::core::ffi::c_uint,
    failed: bool,
) -> Result<::core::ffi::c_uint, ()> {
    if failed {
        Err(())
    } else {
        Ok(have)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum GzAvailLoadTransition {
    Error,
    ResetInput { avail_in: crate::stdlib::uInt },
}

fn gz_avail_load_transition(
    prior_avail_in: crate::stdlib::uInt,
    load: &GzLoadResult,
) -> GzAvailLoadTransition {
    match gz_load_checked_have(load.have, load.failed) {
        Err(()) => GzAvailLoadTransition::Error,
        Ok(have) => GzAvailLoadTransition::ResetInput {
            avail_in: prior_avail_in.wrapping_add(have),
        },
    }
}

fn gz_avail_apply_load_transition(
    avail_in: &mut crate::stdlib::uInt,
    transition: GzAvailLoadTransition,
) -> Option<GzAvailNextInAction> {
    match transition {
        GzAvailLoadTransition::Error => None,
        GzAvailLoadTransition::ResetInput {
            avail_in: committed_avail_in,
        } => {
            *avail_in = committed_avail_in;
            Some(GzAvailNextInAction::ResetToInputStart)
        }
    }
}

fn gz_avail_finish_refill(
    prior_avail_in: crate::stdlib::uInt,
    load: &GzLoadResult,
    avail_in: &mut crate::stdlib::uInt,
) -> Result<GzAvailNextInAction, ()> {
    gz_avail_apply_load_transition(avail_in, gz_avail_load_transition(prior_avail_in, load))
        .ok_or(())
}

#[derive(Debug, PartialEq, Eq)]
enum GzAvailNextInAction {
    ResetToInputStart,
}

fn gz_load_with_reader<F>(
    len: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    _again: ::core::ffi::c_int,
    read: F,
) -> GzLoadState
where
    F: FnOnce() -> Result<::core::ffi::c_uint, ::core::ffi::c_int>,
{
    match read() {
        Ok(0) => GzLoadState {
            have,
            eof: 1,
            again: 0,
            more: false,
            error: None,
        },
        Ok(got) => {
            let have = have.wrapping_add(got);
            GzLoadState {
                have,
                eof,
                again: 0,
                more: have < len,
                error: None,
            }
        }
        Err(errno) => {
            let again = crate::src::gzlib::gz_errno_is_retryable(errno);
            GzLoadState {
                have,
                eof,
                again: again as ::core::ffi::c_int,
                more: false,
                error: if again && have != 0 {
                    None
                } else {
                    Some(errno)
                },
            }
        }
    }
}

fn gz_load_apply_state(
    eof: &mut ::core::ffi::c_int,
    again: &mut ::core::ffi::c_int,
    load: &GzLoadState,
) -> Result<(), ::core::ffi::c_int> {
    *eof = load.eof;
    *again = load.again;
    load.error.map_or(Ok(()), Err)
}

fn gz_load_transition(
    len: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    eof: &mut ::core::ffi::c_int,
    again: &mut ::core::ffi::c_int,
    read: Result<::core::ffi::c_uint, ::core::ffi::c_int>,
) -> GzLoadTransition {
    let load = gz_load_with_reader(len, have, *eof, *again, || read);
    match gz_load_apply_state(eof, again, &load) {
        Err(errno) => GzLoadTransition::Error {
            have: load.have,
            errno,
        },
        Ok(()) if load.more => GzLoadTransition::Continue { have: load.have },
        Ok(()) => GzLoadTransition::Complete { have: load.have },
    }
}

fn gz_load_read_loop<F>(
    len: ::core::ffi::c_uint,
    eof: &mut ::core::ffi::c_int,
    again: &mut ::core::ffi::c_int,
    mut read: F,
) -> Result<::core::ffi::c_uint, GzLoadError>
where
    F: FnMut(
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
    ) -> Result<::core::ffi::c_uint, ::core::ffi::c_int>,
{
    let max = gz_load_max_read_len();
    let mut have = 0;
    loop {
        let get = gz_load_read_len(len, have, max);
        match gz_load_transition(len, have, eof, again, read(have, get)) {
            GzLoadTransition::Error { have, errno } => {
                return Err(GzLoadError { have, errno });
            }
            GzLoadTransition::Continue { have: next_have } => have = next_have,
            GzLoadTransition::Complete { have } => return Ok(have),
        }
    }
}

fn gz_load_read_result(
    ret: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> Result<::core::ffi::c_uint, ::core::ffi::c_int> {
    if ret < 0 {
        Err(errno)
    } else {
        Ok(ret as ::core::ffi::c_uint)
    }
}

fn gz_load_errno(
    ret: ::core::ffi::c_int,
    os_error: Option<::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    if ret < 0 {
        os_error.unwrap_or(0)
    } else {
        0
    }
}

fn gz_load_read_len(
    len: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    max: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let get = len.wrapping_sub(have);
    get.min(max)
}

fn gz_load_max_read_len() -> ::core::ffi::c_uint {
    (1 as ::core::ffi::c_uint) << (::core::ffi::c_uint::BITS - 2)
}

fn gz_avail_can_load(err: ::core::ffi::c_int) -> bool {
    err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR
}

#[derive(Debug, PartialEq, Eq)]
enum GzAvailAction {
    Error,
    Refill { compact_input: bool },
    Done,
}

fn gz_avail_action(
    err: ::core::ffi::c_int,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
) -> GzAvailAction {
    if !gz_avail_can_load(err) {
        GzAvailAction::Error
    } else if eof != 0 {
        GzAvailAction::Done
    } else {
        GzAvailAction::Refill {
            compact_input: avail_in != 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GzAvailRefillPlan {
    input_offset: usize,
    read_len: ::core::ffi::c_uint,
    prior_avail_in: crate::stdlib::uInt,
}

fn gz_avail_refill_plan(
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
) -> GzAvailRefillPlan {
    GzAvailRefillPlan {
        input_offset: avail_in as usize,
        read_len: size.wrapping_sub(avail_in as ::core::ffi::c_uint),
        prior_avail_in: avail_in,
    }
}

fn gz_avail_should_compact(compact_input: bool, input_is_buffer_start: bool) -> bool {
    compact_input && !input_is_buffer_start
}

#[derive(Debug, PartialEq, Eq)]
struct GzAvailRefillStep {
    compact_input: bool,
    plan: GzAvailRefillPlan,
}

fn gz_avail_refill_step(
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    compact_input: bool,
    input_is_buffer_start: bool,
) -> GzAvailRefillStep {
    GzAvailRefillStep {
        compact_input: gz_avail_should_compact(compact_input, input_is_buffer_start),
        plan: gz_avail_refill_plan(size, avail_in),
    }
}

fn gz_avail_prepare_refill(
    input: &mut [crate::stdlib::Byte],
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    compact_input: bool,
    input_offset: usize,
) -> Option<GzAvailRefillPlan> {
    let step = gz_avail_refill_step(size, avail_in, compact_input, input_offset == 0);
    let plan = step.plan;
    let refill_end = plan.input_offset.checked_add(plan.read_len as usize)?;
    if refill_end > input.len() {
        return None;
    }
    if step.compact_input {
        let input_end = input_offset.checked_add(plan.prior_avail_in as usize)?;
        if input_end > input.len() {
            return None;
        }
        input.copy_within(input_offset..input_end, 0);
    }
    Some(plan)
}

struct GzAvailInputRefill<'a> {
    buffer: &'a mut [crate::stdlib::Byte],
    prior_avail_in: crate::stdlib::uInt,
}

fn gz_avail_input_refill(
    input: &mut [crate::stdlib::Byte],
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    compact_input: bool,
    input_offset: usize,
) -> Option<GzAvailInputRefill<'_>> {
    let plan = gz_avail_prepare_refill(input, size, avail_in, compact_input, input_offset)?;
    let refill_end = plan.input_offset.checked_add(plan.read_len as usize)?;
    let buffer = input.get_mut(plan.input_offset..refill_end)?;
    Some(GzAvailInputRefill {
        buffer,
        prior_avail_in: plan.prior_avail_in,
    })
}

fn gzread_request(len: ::core::ffi::c_uint) -> Option<crate::stdlib::z_size_t> {
    ((len as ::core::ffi::c_int) >= 0).then_some(len as crate::stdlib::z_size_t)
}

fn gz_is_read_mode(mode: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_READ
}

fn gz_read_error_is_recoverable(err: ::core::ffi::c_int, again: ::core::ffi::c_int) -> bool {
    err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR || again != 0
}

fn gz_read_state_is_usable(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> bool {
    gz_is_read_mode(mode) && gz_read_error_is_recoverable(err, again)
}

enum GzreadOutcome {
    Read(::core::ffi::c_int),
    Error,
    Again,
}

fn gzread_outcome(
    len: ::core::ffi::c_uint,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> GzreadOutcome {
    if len != 0 {
        GzreadOutcome::Read(len as ::core::ffi::c_int)
    } else if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR {
        GzreadOutcome::Error
    } else if again != 0 {
        GzreadOutcome::Again
    } else {
        GzreadOutcome::Read(0)
    }
}

fn gz_fread_items_read(
    size: crate::stdlib::z_size_t,
    bytes_read: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if size == 0 {
        0
    } else {
        bytes_read / size
    }
}

#[derive(Debug, Eq, PartialEq)]
enum GzFreadAction {
    ReturnZero,
    Read,
}

fn gz_fread_action(len: crate::stdlib::z_size_t) -> GzFreadAction {
    if gz_read_request_is_empty(len) {
        GzFreadAction::ReturnZero
    } else {
        GzFreadAction::Read
    }
}

#[derive(Debug, Eq, PartialEq)]
enum GzgetcAction {
    ConsumeBuffered,
    Read,
}

fn gzgetc_action(buffered_have: ::core::ffi::c_uint) -> GzgetcAction {
    if buffered_have != 0 {
        GzgetcAction::ConsumeBuffered
    } else {
        GzgetcAction::Read
    }
}

fn gzgetc_read_result(
    bytes_read: crate::stdlib::z_size_t,
    byte: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    if bytes_read < 1 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        byte as ::core::ffi::c_int
    }
}

fn gzgetc_buffered_result(
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    byte: ::core::ffi::c_uchar,
) -> (
    ::core::ffi::c_uint,
    crate::stdlib::off64_t,
    ::core::ffi::c_int,
) {
    (
        have.wrapping_sub(1),
        gz_cursor_advance(pos, 1),
        byte as ::core::ffi::c_int,
    )
}

fn gz_read_chunk_len(
    len: crate::stdlib::z_size_t,
    buffered: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut chunk = ::core::ffi::c_uint::MAX;
    if chunk as crate::stdlib::z_size_t > len {
        chunk = len as ::core::ffi::c_uint;
    }
    if buffered != 0 && buffered < chunk {
        chunk = buffered;
    }
    chunk
}

struct GzReadDrainPlan {
    remaining_have: ::core::ffi::c_uint,
    err: ::core::ffi::c_int,
    next_advance: usize,
}

fn gz_read_apply_drain_plan(
    have: &mut ::core::ffi::c_uint,
    err: &mut ::core::ffi::c_int,
    plan: &GzReadDrainPlan,
) {
    *have = plan.remaining_have;
    *err = plan.err;
}

fn gz_read_drain_plan(
    have: ::core::ffi::c_uint,
    state_err: ::core::ffi::c_int,
    chunk_len: ::core::ffi::c_uint,
) -> GzReadDrainPlan {
    GzReadDrainPlan {
        remaining_have: have.wrapping_sub(chunk_len),
        err: if state_err == crate::zlib_h::Z_OK {
            0
        } else {
            -1
        },
        next_advance: chunk_len as usize,
    }
}

fn gz_read_needs_fetch(
    how: ::core::ffi::c_int,
    chunk_len: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> bool {
    how == crate::gzguts_h::LOOK || !gz_read_has_full_output_chunk(chunk_len, size)
}

fn gz_read_has_full_output_chunk(
    chunk_len: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> bool {
    chunk_len >= gz_output_buffer_len(size)
}

fn gz_read_stops_at_eof(eof: ::core::ffi::c_int, avail_in: crate::stdlib::uInt) -> bool {
    eof != 0 && avail_in == 0
}

enum GzReadAction {
    DrainBuffered,
    StopAtEof,
    Fetch,
    Load,
    Decompress,
}

struct GzReadStep {
    chunk_len: ::core::ffi::c_uint,
    action: GzReadAction,
}

fn gz_read_action(
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    how: ::core::ffi::c_int,
    chunk_len: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> GzReadAction {
    if have != 0 {
        GzReadAction::DrainBuffered
    } else if gz_read_stops_at_eof(eof, avail_in) {
        GzReadAction::StopAtEof
    } else if gz_read_needs_fetch(how, chunk_len, size) {
        GzReadAction::Fetch
    } else if how == crate::gzguts_h::COPY {
        GzReadAction::Load
    } else {
        GzReadAction::Decompress
    }
}

fn gz_read_action_advances_output(action: &GzReadAction) -> bool {
    matches!(
        action,
        GzReadAction::DrainBuffered | GzReadAction::Load | GzReadAction::Decompress
    )
}

fn gz_read_step(
    len: crate::stdlib::z_size_t,
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    how: ::core::ffi::c_int,
    size: ::core::ffi::c_uint,
) -> GzReadStep {
    let chunk_len = gz_read_chunk_len(len, have);
    GzReadStep {
        chunk_len,
        action: gz_read_action(have, eof, avail_in, how, chunk_len, size),
    }
}

fn gz_read_fetch_failed_without_buffer(
    fetch_result: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
) -> bool {
    fetch_result == -1 as ::core::ffi::c_int && have == 0
}

fn gz_read_fetch_error(
    fetch_result: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_int> {
    if gz_read_fetch_failed_without_buffer(fetch_result, have) {
        Some(-1)
    } else {
        None
    }
}

fn gz_read_load_status(load_failed: bool) -> ::core::ffi::c_int {
    if load_failed {
        -1
    } else {
        0
    }
}

fn gz_read_take_decompressed(
    have: ::core::ffi::c_uint,
) -> (::core::ffi::c_uint, ::core::ffi::c_uint) {
    (have, 0)
}

fn gz_cursor_advance(
    pos: crate::stdlib::off64_t,
    consumed: ::core::ffi::c_uint,
) -> crate::stdlib::off64_t {
    pos.wrapping_add(consumed as crate::stdlib::off64_t)
}

fn gz_cursor_rewind(
    pos: crate::stdlib::off64_t,
    pushed_back: ::core::ffi::c_uint,
) -> crate::stdlib::off64_t {
    pos.wrapping_sub(pushed_back as crate::stdlib::off64_t)
}

fn gz_read_progress(
    len: crate::stdlib::z_size_t,
    got: crate::stdlib::z_size_t,
    pos: crate::stdlib::off64_t,
    chunk_len: ::core::ffi::c_uint,
) -> (
    crate::stdlib::z_size_t,
    crate::stdlib::z_size_t,
    crate::stdlib::off64_t,
) {
    (
        len.wrapping_sub(chunk_len as crate::stdlib::z_size_t),
        got.wrapping_add(chunk_len as crate::stdlib::z_size_t),
        gz_cursor_advance(pos, chunk_len),
    )
}

fn gz_read_should_continue(len: crate::stdlib::z_size_t, err: ::core::ffi::c_int) -> bool {
    len != 0 && err == 0
}

#[derive(Debug, PartialEq, Eq)]
struct GzReadLoopProgress {
    len: crate::stdlib::z_size_t,
    got: crate::stdlib::z_size_t,
    pos: crate::stdlib::off64_t,
    should_continue: bool,
}

fn gz_read_loop_progress(
    advance: bool,
    len: crate::stdlib::z_size_t,
    got: crate::stdlib::z_size_t,
    pos: crate::stdlib::off64_t,
    chunk_len: ::core::ffi::c_uint,
    err: ::core::ffi::c_int,
) -> GzReadLoopProgress {
    let (len, got, pos) = if advance {
        gz_read_progress(len, got, pos, chunk_len)
    } else {
        (len, got, pos)
    };
    GzReadLoopProgress {
        len,
        got,
        pos,
        should_continue: gz_read_should_continue(len, err),
    }
}

fn gz_read_request_is_empty(len: crate::stdlib::z_size_t) -> bool {
    len == 0 as crate::stdlib::z_size_t
}

#[derive(Debug, PartialEq, Eq)]
enum GzReadSetup {
    ReturnEmpty,
    Skip,
    Read,
}

fn gz_read_setup(len: crate::stdlib::z_size_t, skip: crate::stdlib::off64_t) -> GzReadSetup {
    if gz_read_request_is_empty(len) {
        GzReadSetup::ReturnEmpty
    } else if gz_read_has_pending_skip(skip) {
        GzReadSetup::Skip
    } else {
        GzReadSetup::Read
    }
}

fn gz_read_marks_past_eof(len: crate::stdlib::z_size_t, eof: ::core::ffi::c_int) -> bool {
    len != 0 && eof != 0
}

fn gz_read_note_past_eof(
    past: &mut ::core::ffi::c_int,
    len: crate::stdlib::z_size_t,
    eof: ::core::ffi::c_int,
) {
    if gz_read_marks_past_eof(len, eof) {
        *past = 1;
    }
}

fn gzdirect_result(direct: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (direct == 1) as ::core::ffi::c_int
}

fn gz_read_needs_look(
    mode: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
) -> bool {
    mode == crate::gzguts_h::GZ_READ && how == crate::gzguts_h::LOOK && have == 0
}

fn gz_read_has_pending_skip(skip: crate::stdlib::off64_t) -> bool {
    skip != 0
}

fn gzgets_request_has_capacity(len: ::core::ffi::c_int) -> bool {
    len >= 1
}

fn gzgets_has_valid_inputs(
    file_present: bool,
    buffer_present: bool,
    len: ::core::ffi::c_int,
) -> bool {
    file_present && buffer_present && gzgets_request_has_capacity(len)
}

fn gzgets_remaining_capacity(len: ::core::ffi::c_int) -> ::core::ffi::c_uint {
    (len as ::core::ffi::c_uint).wrapping_sub(1)
}

fn gzgets_copied_any(initial_left: ::core::ffi::c_uint, left: ::core::ffi::c_uint) -> bool {
    initial_left != left
}

fn gzgets_needs_fetch(have: ::core::ffi::c_uint) -> bool {
    have == 0
}

#[derive(Debug, Eq, PartialEq)]
enum GzgetsPostFetchDecision {
    Stop,
    MarkPastAndStop,
    Copy,
}

fn gzgets_post_fetch_decision(
    have: ::core::ffi::c_uint,
    fetch: ::core::ffi::c_int,
) -> GzgetsPostFetchDecision {
    if have != 0 {
        GzgetsPostFetchDecision::Copy
    } else if fetch == -1 {
        GzgetsPostFetchDecision::Stop
    } else {
        GzgetsPostFetchDecision::MarkPastAndStop
    }
}

fn gzgets_apply_post_fetch_decision(
    past: &mut ::core::ffi::c_int,
    decision: GzgetsPostFetchDecision,
) -> bool {
    match decision {
        GzgetsPostFetchDecision::Stop => false,
        GzgetsPostFetchDecision::MarkPastAndStop => {
            *past = 1;
            false
        }
        GzgetsPostFetchDecision::Copy => true,
    }
}

enum GzUngetcBufferState {
    Empty,
    Full,
    Pushable,
}

#[derive(Debug, Eq, PartialEq)]
enum GzUngetcAction {
    Empty { write_index: usize },
    Full,
    Pushable { compact: bool },
}

fn gz_ungetc_buffer_state(
    have: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> GzUngetcBufferState {
    if have == 0 {
        GzUngetcBufferState::Empty
    } else if have >= gz_output_buffer_len(size) {
        GzUngetcBufferState::Full
    } else {
        GzUngetcBufferState::Pushable
    }
}

fn gz_ungetc_action(
    have: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
    next_is_out: bool,
) -> GzUngetcAction {
    match gz_ungetc_buffer_state(have, size) {
        GzUngetcBufferState::Empty => GzUngetcAction::Empty {
            write_index: gz_output_buffer_len(size).wrapping_sub(1) as usize,
        },
        GzUngetcBufferState::Full => GzUngetcAction::Full,
        GzUngetcBufferState::Pushable => GzUngetcAction::Pushable {
            compact: next_is_out,
        },
    }
}

fn gz_ungetc_next_have(have: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    have.wrapping_add(1)
}

fn gz_ungetc_accepts_byte(c: ::core::ffi::c_int) -> bool {
    c >= 0
}

fn gz_ungetc_progress(
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
) -> (
    ::core::ffi::c_uint,
    crate::stdlib::off64_t,
    ::core::ffi::c_int,
) {
    (gz_ungetc_next_have(have), gz_cursor_rewind(pos, 1), 0)
}

#[derive(Debug, Eq, PartialEq)]
struct GzUngetcCompactPlan {
    dest_index: usize,
    len: usize,
}

fn gz_ungetc_compact_plan(
    have: ::core::ffi::c_uint,
    size: ::core::ffi::c_uint,
) -> GzUngetcCompactPlan {
    GzUngetcCompactPlan {
        dest_index: gz_output_buffer_len(size).wrapping_sub(have) as usize,
        len: have as usize,
    }
}

unsafe fn gz_load(
    state: &mut crate::gzguts_h::gz_state,
    buf: *mut ::core::ffi::c_uchar,
    len: ::core::ffi::c_uint,
) -> GzLoadResult {
    let mut have = 0;
    loop {
        let get = gz_load_read_len(len, have, gz_load_max_read_len());
        let ret = crate::stdlib::read(
            state.fd,
            buf.wrapping_add(have as usize) as *mut ::core::ffi::c_void,
            get as crate::__stddef_size_t_h::size_t,
        ) as ::core::ffi::c_int;
        let errno = gz_load_errno(ret, std::io::Error::last_os_error().raw_os_error());
        match gz_load_transition(
            len,
            have,
            &mut state.eof,
            &mut state.again,
            gz_load_read_result(ret, errno),
        ) {
            GzLoadTransition::Error { have, errno } => {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_ERRNO,
                    crate::stdlib::strerror(errno),
                );
                return GzLoadResult { have, failed: true };
            }
            GzLoadTransition::Continue { have: next_have } => have = next_have,
            GzLoadTransition::Complete { have } => {
                return GzLoadResult {
                    have,
                    failed: false,
                };
            }
        }
    }
}

unsafe fn gz_avail(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let action = gz_avail_action(state.err, state.eof, state.strm.avail_in);
    match action {
        GzAvailAction::Error => return -1 as ::core::ffi::c_int,
        GzAvailAction::Done => return 0 as ::core::ffi::c_int,
        GzAvailAction::Refill { compact_input } => {
            let (buf, len, prior_avail_in) = {
                let p = state.in_0;
                let q = state.strm.next_in;
                if p.is_null() {
                    return -1 as ::core::ffi::c_int;
                }
                let input = core::slice::from_raw_parts_mut(p, state.size as usize);
                let input_offset = (q as usize).wrapping_sub(p as usize);
                let refill = match gz_avail_input_refill(
                    input,
                    state.size,
                    state.strm.avail_in,
                    compact_input,
                    input_offset,
                ) {
                    Some(plan) => plan,
                    None => return -1 as ::core::ffi::c_int,
                };
                (
                    refill.buffer.as_mut_ptr(),
                    refill.buffer.len() as ::core::ffi::c_uint,
                    refill.prior_avail_in,
                )
            };
            let load = gz_load(state, buf, len);
            match gz_avail_finish_refill(prior_avail_in, &load, &mut state.strm.avail_in) {
                Err(()) => return -1 as ::core::ffi::c_int,
                Ok(GzAvailNextInAction::ResetToInputStart) => {
                    state.strm.next_in = state.in_0;
                }
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}

fn gz_is_gzip_header(
    first: ::core::ffi::c_uchar,
    second: ::core::ffi::c_uchar,
    third: ::core::ffi::c_uchar,
    fourth: ::core::ffi::c_uchar,
) -> bool {
    first == 31 && second == 139 && third == 8 && fourth < 32
}

fn gz_look_header_is_available(avail_in: crate::stdlib::uInt) -> bool {
    avail_in > 3
}

fn gz_look_forces_gzip(direct: ::core::ffi::c_int, junk: ::core::ffi::c_int) -> bool {
    direct == -1 || junk == 0
}

fn gz_look_needs_more_input(avail_in: crate::stdlib::uInt, again: ::core::ffi::c_int) -> bool {
    avail_in == 0 || again != 0 && avail_in < 4
}

fn gz_output_buffer_len(size: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    size << 1 as ::core::ffi::c_int
}

fn gz_look_window_bits() -> ::core::ffi::c_int {
    15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int
}

fn gz_look_allocations_failed(input_allocated: bool, output_allocated: bool) -> bool {
    !input_allocated || !output_allocated
}

enum GzLookGzipSource {
    Forced { junk_is_known: bool },
    Header,
}

#[derive(Debug, PartialEq, Eq)]
struct GzLookGzipState {
    how: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
}

fn gz_look_gzip_state(source: GzLookGzipSource) -> GzLookGzipState {
    GzLookGzipState {
        how: crate::gzguts_h::GZIP,
        junk: match source {
            GzLookGzipSource::Forced { junk_is_known } => junk_is_known as ::core::ffi::c_int,
            GzLookGzipSource::Header => 1 as ::core::ffi::c_int,
        },
        direct: 0 as ::core::ffi::c_int,
    }
}

fn gz_look_apply_gzip_state(
    how: &mut ::core::ffi::c_int,
    junk: &mut ::core::ffi::c_int,
    direct: &mut ::core::ffi::c_int,
    gzip_state: GzLookGzipState,
) {
    *how = gzip_state.how;
    *junk = gzip_state.junk;
    *direct = gzip_state.direct;
}

#[derive(Debug, PartialEq, Eq)]
struct GzLookTransparentCopyPlan {
    have: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    how: ::core::ffi::c_int,
}

fn gz_look_transparent_copy_plan(avail_in: crate::stdlib::uInt) -> GzLookTransparentCopyPlan {
    GzLookTransparentCopyPlan {
        have: avail_in as ::core::ffi::c_uint,
        avail_in: 0,
        how: crate::gzguts_h::COPY,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GzLookAction {
    NeedMoreInput,
    Gzip,
    TransparentCopy,
}

fn gz_look_action(
    avail_in: crate::stdlib::uInt,
    again: ::core::ffi::c_int,
    header: Option<[::core::ffi::c_uchar; 4]>,
) -> GzLookAction {
    if gz_look_needs_more_input(avail_in, again) {
        GzLookAction::NeedMoreInput
    } else if header.is_some_and(|[first, second, third, fourth]| {
        gz_is_gzip_header(first, second, third, fourth)
    }) {
        GzLookAction::Gzip
    } else {
        GzLookAction::TransparentCopy
    }
}

unsafe fn gz_look(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint {
        (*state).in_0 = crate::stdlib::malloc((*state).want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        (*state).out = crate::stdlib::malloc(
            gz_output_buffer_len((*state).want) as crate::__stddef_size_t_h::size_t
        ) as *mut ::core::ffi::c_uchar;
        if gz_look_allocations_failed(!(*state).in_0.is_null(), !(*state).out.is_null()) {
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        (*state).size = (*state).want;
        (*state).strm.zalloc = None;
        (*state).strm.zfree = None;
        (*state).strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*state).strm.avail_in = 0 as crate::stdlib::uInt;
        (*state).strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2_(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            gz_look_window_bits(),
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
            crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
            (*state).size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_MEM_ERROR,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    let junk = (*state).junk;
    if gz_look_forces_gzip((*state).direct, junk) {
        crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
        let gzip_state = gz_look_gzip_state(GzLookGzipSource::Forced {
            junk_is_known: junk != -1 as ::core::ffi::c_int,
        });
        let state_ref = &mut *state;
        gz_look_apply_gzip_state(
            &mut state_ref.how,
            &mut state_ref.junk,
            &mut state_ref.direct,
            gzip_state,
        );
        return 0 as ::core::ffi::c_int;
    }
    let (avail, again) = {
        let state_ref = &mut *state;
        (gz_avail(state_ref), state_ref.again)
    };
    if avail == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let header = if gz_look_header_is_available((*strm).avail_in) {
        let next_in = (*strm).next_in;
        Some([
            *next_in,
            *next_in.wrapping_add(1),
            *next_in.wrapping_add(2),
            *next_in.wrapping_add(3),
        ])
    } else {
        None
    };
    match gz_look_action((*strm).avail_in, again, header) {
        GzLookAction::NeedMoreInput => return 0 as ::core::ffi::c_int,
        GzLookAction::Gzip => {
            crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
            let gzip_state = gz_look_gzip_state(GzLookGzipSource::Header);
            let state_ref = &mut *state;
            gz_look_apply_gzip_state(
                &mut state_ref.how,
                &mut state_ref.junk,
                &mut state_ref.direct,
                gzip_state,
            );
            return 0 as ::core::ffi::c_int;
        }
        GzLookAction::TransparentCopy => {}
    }
    (*state).x.next = (*state).out;
    crate::stdlib::memcpy(
        (*state).x.next as *mut ::core::ffi::c_void,
        (*strm).next_in as *const ::core::ffi::c_void,
        (*strm).avail_in as crate::__stddef_size_t_h::size_t,
    );
    let plan = gz_look_transparent_copy_plan((*strm).avail_in);
    (*state).x.have = plan.have;
    (*strm).avail_in = plan.avail_in;
    (*state).how = plan.how;
    return 0 as ::core::ffi::c_int;
}

#[derive(Debug, PartialEq, Eq)]
enum GzDecompAction {
    InternalError,
    MemoryError,
    TrailingJunk,
    DataError,
    Stop,
    Continue,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GzDecompDataErrorMessage {
    Generic,
    Inflate,
}

fn gz_decomp_data_error_message(inflate_message_present: bool) -> GzDecompDataErrorMessage {
    if inflate_message_present {
        GzDecompDataErrorMessage::Inflate
    } else {
        GzDecompDataErrorMessage::Generic
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GzDecompDecision {
    clear_junk: bool,
    action: GzDecompAction,
}

#[derive(Debug, PartialEq, Eq)]
enum GzDecompInputAction {
    InputError,
    UnexpectedEof,
    Inflate,
}

fn gz_decomp_input_action(load_failed: bool, avail_in: crate::stdlib::uInt) -> GzDecompInputAction {
    if load_failed {
        GzDecompInputAction::InputError
    } else if avail_in == 0 {
        GzDecompInputAction::UnexpectedEof
    } else {
        GzDecompInputAction::Inflate
    }
}

fn gz_decomp_needs_input_load(avail_in: crate::stdlib::uInt) -> bool {
    avail_in == 0
}

fn gz_decomp_input_load_failed(result: ::core::ffi::c_int) -> bool {
    result == -1 as ::core::ffi::c_int
}

#[derive(Debug, Eq, PartialEq)]
struct GzDecompStreamState {
    avail_in: crate::stdlib::uInt,
    avail_out: crate::stdlib::uInt,
    inflate_message_present: bool,
}

fn gz_decomp_stream_state(stream: &crate::zlib_h::z_stream) -> GzDecompStreamState {
    GzDecompStreamState {
        avail_in: stream.avail_in,
        avail_out: stream.avail_out,
        inflate_message_present: !stream.msg.is_null(),
    }
}

fn gz_decomp_produced_output(
    prior_avail_out: ::core::ffi::c_uint,
    avail_out: crate::stdlib::uInt,
) -> bool {
    avail_out < prior_avail_out
}

fn gz_decomp_reports_unexpected_eof(again: ::core::ffi::c_int) -> bool {
    again == 0
}

#[derive(Debug, Eq, PartialEq)]
struct GzDecompOutputProgress {
    have: ::core::ffi::c_uint,
    rewind_len: usize,
}

fn gz_decomp_output_progress(
    had: ::core::ffi::c_uint,
    avail_out: crate::stdlib::uInt,
) -> GzDecompOutputProgress {
    let have = (had as crate::stdlib::uInt).wrapping_sub(avail_out) as ::core::ffi::c_uint;
    GzDecompOutputProgress {
        have,
        rewind_len: have as usize,
    }
}

fn gz_decomp_apply_output_progress(
    output: &mut crate::zlib_h::gzFile_s,
    stream: &crate::zlib_h::z_stream,
    progress: &GzDecompOutputProgress,
) {
    output.have = progress.have;
    output.next = stream.next_out.wrapping_sub(progress.rewind_len);
}

fn gz_decomp_should_continue(ret: ::core::ffi::c_int, avail_out: crate::stdlib::uInt) -> bool {
    avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END
}

fn gz_decomp_apply_result(
    how: &mut ::core::ffi::c_int,
    junk: &mut ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ret == crate::zlib_h::Z_STREAM_END {
        *junk = 0;
        *how = crate::gzguts_h::LOOK;
        0
    } else if ret != crate::zlib_h::Z_OK {
        -1
    } else {
        0
    }
}

fn gz_decomp_decision(
    ret: ::core::ffi::c_int,
    produced_output: bool,
    junk: ::core::ffi::c_int,
    avail_out: crate::stdlib::uInt,
) -> GzDecompDecision {
    let clear_junk = produced_output;
    let junk = gz_decomp_junk_after_output(junk, clear_junk);
    let action = if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
        GzDecompAction::InternalError
    } else if ret == crate::zlib_h::Z_MEM_ERROR {
        GzDecompAction::MemoryError
    } else if ret == crate::zlib_h::Z_DATA_ERROR && junk == 1 {
        GzDecompAction::TrailingJunk
    } else if ret == crate::zlib_h::Z_DATA_ERROR {
        GzDecompAction::DataError
    } else if gz_decomp_should_continue(ret, avail_out) {
        GzDecompAction::Continue
    } else {
        GzDecompAction::Stop
    };

    GzDecompDecision { clear_junk, action }
}

fn gz_decomp_junk_after_output(
    junk: ::core::ffi::c_int,
    produced_output: bool,
) -> ::core::ffi::c_int {
    if produced_output {
        0
    } else {
        junk
    }
}

#[derive(Debug, Eq, PartialEq)]
struct GzDecompTrailingJunkPlan {
    avail_in: crate::stdlib::uInt,
    eof: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
}

fn gz_decomp_trailing_junk_plan() -> GzDecompTrailingJunkPlan {
    GzDecompTrailingJunkPlan {
        avail_in: 0,
        eof: 1,
        how: crate::gzguts_h::LOOK,
    }
}

fn gz_decomp_apply_trailing_junk_plan(
    avail_in: &mut crate::stdlib::uInt,
    eof: &mut ::core::ffi::c_int,
    how: &mut ::core::ffi::c_int,
    plan: &GzDecompTrailingJunkPlan,
) {
    *avail_in = plan.avail_in;
    *eof = plan.eof;
    *how = plan.how;
}

unsafe fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut strm: crate::zlib_h::z_streamp = &raw mut state.strm;
    let had = gz_decomp_stream_state(&state.strm).avail_out as ::core::ffi::c_uint;
    loop {
        let needs_input_load =
            gz_decomp_needs_input_load(gz_decomp_stream_state(&state.strm).avail_in);
        let load_failed = needs_input_load && gz_decomp_input_load_failed(gz_avail(state));
        let stream_state = gz_decomp_stream_state(&state.strm);
        match gz_decomp_input_action(load_failed, stream_state.avail_in) {
            GzDecompInputAction::InputError => {
                ret = state.err;
                break;
            }
            GzDecompInputAction::UnexpectedEof => {
                if gz_decomp_reports_unexpected_eof(state.again) {
                    crate::src::gzlib::gz_error(
                        state as *mut crate::gzguts_h::gz_state,
                        crate::zlib_h::Z_BUF_ERROR,
                        b"unexpected end of file\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                break;
            }
            GzDecompInputAction::Inflate => {}
        }
        ret = crate::src::inflate::inflate(
            strm as *mut crate::zlib_h::z_stream_s,
            crate::zlib_h::Z_NO_FLUSH,
        );
        let stream_state = gz_decomp_stream_state(&state.strm);
        let decision = gz_decomp_decision(
            ret,
            gz_decomp_produced_output(had, stream_state.avail_out),
            state.junk,
            stream_state.avail_out,
        );
        state.junk = gz_decomp_junk_after_output(state.junk, decision.clear_junk);
        match decision.action {
            GzDecompAction::InternalError => {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                break;
            }
            GzDecompAction::MemoryError => {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                break;
            }
            GzDecompAction::TrailingJunk => {
                let plan = gz_decomp_trailing_junk_plan();
                gz_decomp_apply_trailing_junk_plan(
                    &mut state.strm.avail_in,
                    &mut state.eof,
                    &mut state.how,
                    &plan,
                );
                ret = crate::zlib_h::Z_OK;
                break;
            }
            GzDecompAction::DataError => {
                let message =
                    match gz_decomp_data_error_message(stream_state.inflate_message_present) {
                        GzDecompDataErrorMessage::Generic => {
                            b"compressed data error\0".as_ptr() as *const ::core::ffi::c_char
                        }
                        GzDecompDataErrorMessage::Inflate => {
                            state.strm.msg as *const ::core::ffi::c_char
                        }
                    };
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_DATA_ERROR,
                    message,
                );
                break;
            }
            GzDecompAction::Stop => break,
            GzDecompAction::Continue => {}
        }
    }
    let progress = gz_decomp_output_progress(had, gz_decomp_stream_state(&state.strm).avail_out);
    gz_decomp_apply_output_progress(&mut state.x, &state.strm, &progress);
    gz_decomp_apply_result(&mut state.how, &mut state.junk, ret)
}

unsafe fn gz_fetch(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    loop {
        let action = gz_fetch_action(state.how);
        match action {
            GzFetchAction::Look => {
                if gz_fetch_look_failed(gz_look(state as *mut crate::gzguts_h::gz_state)) {
                    return -1 as ::core::ffi::c_int;
                }
            }
            GzFetchAction::Copy => {
                let out = state.out;
                let size = state.size;
                let load = gz_load(state, out, gz_output_buffer_len(size));
                if !gz_fetch_apply_copy_load(&mut state.x.have, &load) {
                    return -1 as ::core::ffi::c_int;
                }
                state.x.next = state.out;
            }
            GzFetchAction::Gzip => {
                state.strm.avail_out = gz_fetch_output_capacity(state.size) as crate::stdlib::uInt;
                state.strm.next_out = state.out as *mut crate::stdlib::Bytef;
                if gz_decomp(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            GzFetchAction::StateCorrupt => {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"state corrupt\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if gz_fetch_post_action(
            action,
            state.how,
            state.x.have,
            state.eof,
            state.strm.avail_in,
        ) == GzFetchPostAction::Return
        {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GzFetchAction {
    Look,
    Copy,
    Gzip,
    StateCorrupt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GzFetchPostAction {
    Return,
    Continue,
}

fn gz_fetch_action(how: ::core::ffi::c_int) -> GzFetchAction {
    match how {
        crate::gzguts_h::LOOK => GzFetchAction::Look,
        crate::gzguts_h::COPY => GzFetchAction::Copy,
        crate::gzguts_h::GZIP => GzFetchAction::Gzip,
        _ => GzFetchAction::StateCorrupt,
    }
}

fn gz_fetch_output_capacity(size: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    gz_output_buffer_len(size)
}

fn gz_fetch_post_action(
    action: GzFetchAction,
    how: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
) -> GzFetchPostAction {
    match action {
        GzFetchAction::Copy | GzFetchAction::StateCorrupt => GzFetchPostAction::Return,
        GzFetchAction::Look if how == crate::gzguts_h::LOOK => GzFetchPostAction::Return,
        GzFetchAction::Look | GzFetchAction::Gzip if have == 0 && (eof == 0 || avail_in != 0) => {
            GzFetchPostAction::Continue
        }
        GzFetchAction::Look | GzFetchAction::Gzip => GzFetchPostAction::Return,
    }
}

fn gz_fetch_look_failed(result: ::core::ffi::c_int) -> bool {
    result == -1 as ::core::ffi::c_int
}

fn gz_fetch_apply_copy_load(have: &mut ::core::ffi::c_uint, load: &GzLoadResult) -> bool {
    *have = load.have;
    gz_load_checked_have(load.have, load.failed).is_ok()
}

#[derive(Debug, Eq, PartialEq)]
struct GzSkipProgress {
    remaining_have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    remaining_skip: crate::stdlib::off64_t,
    consumed: ::core::ffi::c_uint,
}

fn gz_skip_progress(
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
    intmax: ::core::ffi::c_uint,
) -> GzSkipProgress {
    let n = gz_skip_len(have, skip, intmax);
    gz_skip_consume_progress(have, pos, skip, n)
}

fn gz_skip_remaining(
    skip: crate::stdlib::off64_t,
    consumed: ::core::ffi::c_uint,
) -> crate::stdlib::off64_t {
    skip.wrapping_sub(consumed as crate::stdlib::off64_t)
}

fn gz_skip_consume_progress(
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
    n: ::core::ffi::c_uint,
) -> GzSkipProgress {
    GzSkipProgress {
        remaining_have: have.wrapping_sub(n),
        pos: gz_cursor_advance(pos, n),
        remaining_skip: gz_skip_remaining(skip, n),
        consumed: n,
    }
}

fn gz_skip_apply_progress(
    have: &mut ::core::ffi::c_uint,
    pos: &mut crate::stdlib::off64_t,
    skip: &mut crate::stdlib::off64_t,
    progress: &GzSkipProgress,
) {
    *have = progress.remaining_have;
    *pos = progress.pos;
    *skip = progress.remaining_skip;
}

fn gz_skip_is_limited_by_remaining(
    have: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
    intmax: ::core::ffi::c_uint,
) -> bool {
    let same_width = ::core::mem::size_of::<::core::ffi::c_int>()
        == ::core::mem::size_of::<crate::stdlib::off64_t>();
    (same_width && have > intmax) || have as crate::stdlib::off64_t > skip
}

fn gz_skip_len(
    have: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
    intmax: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if gz_skip_is_limited_by_remaining(have, skip, intmax) {
        skip as ::core::ffi::c_uint
    } else {
        have
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzSkipAction {
    ConsumeBuffered,
    StopAtEof,
    Fetch,
}

#[derive(Debug, Eq, PartialEq)]
enum GzSkipStep {
    ConsumeBuffered(GzSkipProgress),
    StopAtEof,
    Fetch,
}

#[derive(Debug, Eq, PartialEq)]
enum GzSkipLoopDecision {
    Error,
    Done,
    Continue,
}

fn gz_skip_action(
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
) -> GzSkipAction {
    if have != 0 {
        GzSkipAction::ConsumeBuffered
    } else if eof != 0 && avail_in == 0 {
        GzSkipAction::StopAtEof
    } else {
        GzSkipAction::Fetch
    }
}

fn gz_skip_step(
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    intmax: ::core::ffi::c_uint,
) -> GzSkipStep {
    match gz_skip_action(have, eof, avail_in) {
        GzSkipAction::ConsumeBuffered => {
            GzSkipStep::ConsumeBuffered(gz_skip_progress(have, pos, skip, intmax))
        }
        GzSkipAction::StopAtEof => GzSkipStep::StopAtEof,
        GzSkipAction::Fetch => GzSkipStep::Fetch,
    }
}

fn gz_skip_loop_decision(
    action: GzSkipAction,
    fetch_failed: bool,
    skip: crate::stdlib::off64_t,
) -> GzSkipLoopDecision {
    match action {
        GzSkipAction::StopAtEof => GzSkipLoopDecision::Done,
        GzSkipAction::Fetch if fetch_failed => GzSkipLoopDecision::Error,
        GzSkipAction::ConsumeBuffered | GzSkipAction::Fetch if skip != 0 => {
            GzSkipLoopDecision::Continue
        }
        GzSkipAction::ConsumeBuffered | GzSkipAction::Fetch => GzSkipLoopDecision::Done,
    }
}

fn gz_skip_fetch_failed(action: &GzSkipAction, fetch_result: Option<::core::ffi::c_int>) -> bool {
    matches!(action, GzSkipAction::Fetch) && fetch_result == Some(-1 as ::core::ffi::c_int)
}

fn gzgets_copy_len(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    newline_offset: Option<usize>,
) -> ::core::ffi::c_uint {
    let limit = if have > left { left } else { have };
    match newline_offset {
        Some(offset) if offset < limit as usize => (offset as ::core::ffi::c_uint).wrapping_add(1),
        _ => limit,
    }
}

fn gzgets_progress(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    copied: ::core::ffi::c_uint,
) -> (
    ::core::ffi::c_uint,
    ::core::ffi::c_uint,
    crate::stdlib::off64_t,
) {
    (
        have.wrapping_sub(copied),
        left.wrapping_sub(copied),
        gz_cursor_advance(pos, copied),
    )
}

fn gzgets_should_continue(left: ::core::ffi::c_uint, found_eol: bool) -> bool {
    left != 0 && !found_eol
}

#[derive(Debug, Eq, PartialEq)]
struct GzgetsCopyProgress {
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    should_continue: bool,
}

fn gzgets_copy_progress(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    copied: ::core::ffi::c_uint,
    found_eol: bool,
) -> GzgetsCopyProgress {
    let (have, left, pos) = gzgets_progress(have, left, pos, copied);
    GzgetsCopyProgress {
        have,
        left,
        pos,
        should_continue: gzgets_should_continue(left, found_eol),
    }
}

fn gzclose_r_result(
    stream_err: ::core::ffi::c_int,
    close_ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let err = if stream_err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    if close_ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    }
}

fn gz_skip_apply_step(state: &mut crate::gzguts_h::gz_state) -> GzSkipAction {
    match gz_skip_step(
        state.x.have,
        state.x.pos,
        state.skip,
        state.eof,
        state.strm.avail_in,
        crate::src::gzlib::gz_intmax(),
    ) {
        GzSkipStep::ConsumeBuffered(progress) => {
            gz_skip_apply_progress(
                &mut state.x.have,
                &mut state.x.pos,
                &mut state.skip,
                &progress,
            );
            state.x.next = state.x.next.wrapping_add(progress.consumed as usize);
            GzSkipAction::ConsumeBuffered
        }
        GzSkipStep::StopAtEof => GzSkipAction::StopAtEof,
        GzSkipStep::Fetch => GzSkipAction::Fetch,
    }
}

macro_rules! gz_skip {
    ($state:expr) => {{
        loop {
            let action = gz_skip_apply_step($state);
            let fetch_result = match action {
                GzSkipAction::Fetch => Some(gz_fetch($state)),
                _ => None,
            };
            let fetch_failed = gz_skip_fetch_failed(&action, fetch_result);
            match gz_skip_loop_decision(action, fetch_failed, $state.skip) {
                GzSkipLoopDecision::Error => break true,
                GzSkipLoopDecision::Done => break false,
                GzSkipLoopDecision::Continue => {}
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gz_load_read_result_preserves_signed_read_results() {
        assert_eq!(gz_load_read_result(-1, 5), Err(5));
        assert_eq!(gz_load_read_result(0, 0), Ok(0));
        assert_eq!(gz_load_read_result(17, 0), Ok(17));
        assert_eq!(
            gz_load_read_result(::core::ffi::c_int::MAX, 0),
            Ok(::core::ffi::c_int::MAX as ::core::ffi::c_uint),
        );
    }

    #[test]
    fn gz_load_with_reader_completes_partial_read() {
        let read = Ok(2);

        assert_eq!(
            gz_load_with_reader(5, 3, 0, -1, || read),
            GzLoadState {
                have: 5,
                eof: 0,
                again: 0,
                more: false,
                error: None,
            }
        );
    }

    #[test]
    fn gz_load_with_reader_marks_eof_after_partial_read() {
        let read = Ok(0);

        assert_eq!(
            gz_load_with_reader(8, 3, 0, 0, || read),
            GzLoadState {
                have: 3,
                eof: 1,
                again: 0,
                more: false,
                error: None,
            }
        );
    }

    #[test]
    fn gz_load_with_reader_preserves_partial_data_on_retryable_error() {
        let read = Err(crate::stdlib::EAGAIN);

        assert_eq!(
            gz_load_with_reader(8, 3, -1, 0, || read),
            GzLoadState {
                have: 3,
                eof: -1,
                again: 1,
                more: false,
                error: None,
            }
        );
    }

    #[test]
    fn gz_load_with_reader_preserves_errno_and_have_on_nonretryable_error() {
        let read = Err(5);

        assert_eq!(
            gz_load_with_reader(8, 3, -1, -1, || read),
            GzLoadState {
                have: 3,
                eof: -1,
                again: 0,
                more: false,
                error: Some(5),
            }
        );
    }

    #[test]
    fn gz_avail_load_transition_rejects_failed_loads_with_data() {
        let load = GzLoadResult {
            have: 3,
            failed: true,
        };

        assert_eq!(
            gz_avail_load_transition(4, &load),
            GzAvailLoadTransition::Error
        );
    }

    #[test]
    fn gz_avail_load_transition_resets_input_after_successful_empty_load() {
        let load = GzLoadResult {
            have: 0,
            failed: false,
        };

        assert_eq!(
            gz_avail_load_transition(4, &load),
            GzAvailLoadTransition::ResetInput { avail_in: 4 }
        );
    }

    #[test]
    fn gz_avail_load_transition_adds_loaded_input_and_resets_input() {
        let load = GzLoadResult {
            have: 3,
            failed: false,
        };

        assert_eq!(
            gz_avail_load_transition(4, &load),
            GzAvailLoadTransition::ResetInput { avail_in: 7 }
        );
    }

    #[test]
    fn gz_avail_load_transition_wraps_input_count() {
        let load = GzLoadResult {
            have: 1,
            failed: false,
        };

        assert_eq!(
            gz_avail_load_transition(::core::ffi::c_uint::MAX, &load),
            GzAvailLoadTransition::ResetInput { avail_in: 0 }
        );
    }

    #[test]
    fn gz_avail_apply_load_transition_preserves_input_after_error() {
        let mut avail_in = 4;

        assert_eq!(
            gz_avail_apply_load_transition(&mut avail_in, GzAvailLoadTransition::Error),
            None
        );
        assert_eq!(avail_in, 4);
    }

    #[test]
    fn gz_avail_apply_load_transition_commits_and_requests_input_reset() {
        let mut avail_in = 4;

        assert_eq!(
            gz_avail_apply_load_transition(
                &mut avail_in,
                GzAvailLoadTransition::ResetInput { avail_in: 7 },
            ),
            Some(GzAvailNextInAction::ResetToInputStart)
        );
        assert_eq!(avail_in, 7);
    }

    #[test]
    fn gz_avail_apply_load_transition_commits_wrapped_input_count() {
        let mut avail_in = 1;

        assert_eq!(
            gz_avail_apply_load_transition(
                &mut avail_in,
                GzAvailLoadTransition::ResetInput { avail_in: 0 },
            ),
            Some(GzAvailNextInAction::ResetToInputStart)
        );
        assert_eq!(avail_in, 0);
    }

    #[test]
    fn gz_avail_finish_refill_preserves_input_after_failed_load() {
        let load = GzLoadResult {
            have: 3,
            failed: true,
        };
        let mut avail_in = 4;

        assert_eq!(gz_avail_finish_refill(4, &load, &mut avail_in), Err(()));
        assert_eq!(avail_in, 4);
    }

    #[test]
    fn gz_avail_finish_refill_commits_input_and_requests_reset() {
        let load = GzLoadResult {
            have: 3,
            failed: false,
        };
        let mut avail_in = 4;

        assert_eq!(
            gz_avail_finish_refill(4, &load, &mut avail_in),
            Ok(GzAvailNextInAction::ResetToInputStart)
        );
        assert_eq!(avail_in, 7);
    }

    #[test]
    fn gz_load_checked_have_preserves_load_failure_status() {
        assert_eq!(gz_load_checked_have(3, true), Err(()));
        assert_eq!(gz_load_checked_have(0, false), Ok(0));
    }

    #[test]
    fn gz_load_errno_uses_os_error_only_for_failed_reads() {
        assert_eq!(gz_load_errno(0, Some(crate::stdlib::EAGAIN)), 0);
        assert_eq!(
            gz_load_errno(-1, Some(crate::stdlib::EAGAIN)),
            crate::stdlib::EAGAIN
        );
        assert_eq!(gz_load_errno(-1, None), 0);
    }

    #[test]
    fn gz_fetch_apply_copy_load_commits_count_before_returning_status() {
        let mut have = 0;

        assert!(!gz_fetch_apply_copy_load(
            &mut have,
            &GzLoadResult {
                have: 3,
                failed: true,
            },
        ));
        assert_eq!(have, 3);

        assert!(gz_fetch_apply_copy_load(
            &mut have,
            &GzLoadResult {
                have: 5,
                failed: false,
            },
        ));
        assert_eq!(have, 5);
    }

    #[test]
    fn gz_avail_can_load_accepts_only_refillable_stream_errors() {
        assert!(gz_avail_can_load(crate::zlib_h::Z_OK));
        assert!(gz_avail_can_load(crate::zlib_h::Z_BUF_ERROR));
        assert!(!gz_avail_can_load(crate::zlib_h::Z_DATA_ERROR));
    }

    #[test]
    fn gz_avail_action_rejects_non_refillable_errors_before_eof() {
        assert_eq!(
            gz_avail_action(crate::zlib_h::Z_DATA_ERROR, 1, 4),
            GzAvailAction::Error
        );
    }

    #[test]
    fn gz_avail_compacts_only_moved_refill_input() {
        assert!(!gz_avail_should_compact(false, false));
        assert!(!gz_avail_should_compact(false, true));
        assert!(gz_avail_should_compact(true, false));
        assert!(!gz_avail_should_compact(true, true));
    }

    #[test]
    fn gz_avail_prepare_refill_compacts_overlapping_input_and_plans_read() {
        let mut input = *b"abcdefgh";
        assert_eq!(
            gz_avail_prepare_refill(&mut input, 8, 6, true, 2),
            Some(GzAvailRefillPlan {
                input_offset: 6,
                read_len: 2,
                prior_avail_in: 6,
            })
        );
        assert_eq!(&input[..6], b"cdefgh");
    }

    #[test]
    fn gz_avail_prepare_refill_keeps_uncompacted_input_and_validates_capacity() {
        let mut input = *b"abcdefgh";
        assert_eq!(
            gz_avail_prepare_refill(&mut input, 8, 0, false, 0),
            Some(GzAvailRefillPlan {
                input_offset: 0,
                read_len: 8,
                prior_avail_in: 0,
            })
        );
        assert_eq!(input, *b"abcdefgh");
        assert_eq!(gz_avail_prepare_refill(&mut input, 8, 6, true, 3), None);
    }

    #[test]
    fn gz_avail_input_refill_returns_only_the_validated_read_region() {
        let mut input = *b"abcdefgh";
        let refill = gz_avail_input_refill(&mut input, 8, 6, true, 2).unwrap();

        assert_eq!(refill.prior_avail_in, 6);
        assert_eq!(refill.buffer, b"gh");
        refill.buffer.copy_from_slice(b"12");
        assert_eq!(input, *b"cdefgh12");

        assert!(gz_avail_input_refill(&mut input, 8, 6, true, 3).is_none());
    }

    #[test]
    fn gz_avail_action_leaves_eof_input_untouched() {
        assert_eq!(
            gz_avail_action(crate::zlib_h::Z_OK, 1, 4),
            GzAvailAction::Done
        );
    }

    #[test]
    fn gz_avail_action_refills_with_or_without_input_compaction() {
        assert_eq!(
            gz_avail_action(crate::zlib_h::Z_BUF_ERROR, 0, 0),
            GzAvailAction::Refill {
                compact_input: false
            }
        );
        assert_eq!(
            gz_avail_action(crate::zlib_h::Z_OK, 0, 1),
            GzAvailAction::Refill {
                compact_input: true
            }
        );
    }

    #[test]
    fn gz_avail_refill_plan_preserves_refill_layout_and_wrapping() {
        assert_eq!(
            gz_avail_refill_plan(16, 4),
            GzAvailRefillPlan {
                input_offset: 4,
                read_len: 12,
                prior_avail_in: 4,
            }
        );
        assert_eq!(
            gz_avail_refill_plan(0, 1),
            GzAvailRefillPlan {
                input_offset: 1,
                read_len: ::core::ffi::c_uint::MAX,
                prior_avail_in: 1,
            }
        );
    }

    #[test]
    fn gz_avail_refill_step_keeps_compaction_and_refill_layout_together() {
        assert_eq!(
            gz_avail_refill_step(16, 4, true, false),
            GzAvailRefillStep {
                compact_input: true,
                plan: GzAvailRefillPlan {
                    input_offset: 4,
                    read_len: 12,
                    prior_avail_in: 4,
                },
            }
        );
        assert_eq!(
            gz_avail_refill_step(16, 0, false, true),
            GzAvailRefillStep {
                compact_input: false,
                plan: GzAvailRefillPlan {
                    input_offset: 0,
                    read_len: 16,
                    prior_avail_in: 0,
                },
            }
        );
    }

    #[test]
    fn gz_decomp_input_action_prioritizes_load_failure() {
        assert_eq!(
            gz_decomp_input_action(true, 1),
            GzDecompInputAction::InputError
        );
    }

    #[test]
    fn gz_look_gzip_state_preserves_forced_lookup_state() {
        for (junk, expected_junk) in [(-1, 0), (0, 1), (1, 1)] {
            assert_eq!(
                gz_look_gzip_state(GzLookGzipSource::Forced {
                    junk_is_known: junk != -1,
                }),
                GzLookGzipState {
                    how: crate::gzguts_h::GZIP,
                    junk: expected_junk,
                    direct: 0,
                }
            );
        }
    }

    #[test]
    fn gz_look_gzip_state_marks_detected_headers_as_junk() {
        assert_eq!(
            gz_look_gzip_state(GzLookGzipSource::Header),
            GzLookGzipState {
                how: crate::gzguts_h::GZIP,
                junk: 1,
                direct: 0,
            }
        );
    }

    #[test]
    fn gz_decomp_input_action_distinguishes_eof_from_input_to_inflate() {
        assert_eq!(
            gz_decomp_input_action(false, 0),
            GzDecompInputAction::UnexpectedEof
        );
        assert_eq!(
            gz_decomp_input_action(false, 1),
            GzDecompInputAction::Inflate
        );
    }

    #[test]
    fn gz_decomp_needs_input_load_only_when_input_is_empty() {
        assert!(gz_decomp_needs_input_load(0));
        assert!(!gz_decomp_needs_input_load(1));
        assert!(!gz_decomp_needs_input_load(crate::stdlib::uInt::MAX));
    }

    #[test]
    fn gz_fetch_look_failed_matches_only_gz_look_failure() {
        assert!(gz_fetch_look_failed(-1));
        assert!(!gz_fetch_look_failed(0));
        assert!(!gz_fetch_look_failed(1));
        assert!(!gz_fetch_look_failed(-2));
    }

    #[test]
    fn gz_decomp_input_load_failed_matches_only_gz_avail_failure() {
        assert!(gz_decomp_input_load_failed(-1));
        assert!(!gz_decomp_input_load_failed(0));
        assert!(!gz_decomp_input_load_failed(1));
        assert!(!gz_decomp_input_load_failed(::core::ffi::c_int::MIN));
    }

    #[test]
    fn gz_decomp_stream_state_extracts_input_output_and_message_state() {
        let stream = crate::zlib_h::z_stream_s {
            next_in: ::core::ptr::null_mut(),
            avail_in: 3,
            total_in: 0,
            next_out: ::core::ptr::null_mut(),
            avail_out: 7,
            total_out: 0,
            msg: b"inflate failed\0".as_ptr() as *mut ::core::ffi::c_char,
            state: ::core::ptr::null_mut(),
            zalloc: None,
            zfree: None,
            opaque: ::core::ptr::null_mut(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        };

        assert_eq!(
            gz_decomp_stream_state(&stream),
            GzDecompStreamState {
                avail_in: 3,
                avail_out: 7,
                inflate_message_present: true,
            }
        );
    }

    #[test]
    fn gz_decomp_stream_state_reports_absent_inflate_message() {
        let stream = crate::zlib_h::z_stream_s {
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
        };

        assert_eq!(
            gz_decomp_stream_state(&stream),
            GzDecompStreamState {
                avail_in: 0,
                avail_out: 0,
                inflate_message_present: false,
            }
        );
    }

    #[test]
    fn gz_decomp_produced_output_requires_available_output_to_decrease() {
        assert!(gz_decomp_produced_output(10, 9));
        assert!(!gz_decomp_produced_output(10, 10));
        assert!(!gz_decomp_produced_output(10, 11));
    }

    #[test]
    fn gz_look_window_bits_selects_the_gzip_wrapper() {
        assert_eq!(gz_look_window_bits(), 31);
    }

    #[test]
    fn gz_look_allocations_failed_requires_both_buffers() {
        assert!(!gz_look_allocations_failed(true, true));
        assert!(gz_look_allocations_failed(false, true));
        assert!(gz_look_allocations_failed(true, false));
        assert!(gz_look_allocations_failed(false, false));
    }

    #[test]
    fn gz_look_transparent_copy_plan_preserves_input_count_and_copy_mode() {
        for avail_in in [0, 4, crate::stdlib::uInt::MAX] {
            assert_eq!(
                gz_look_transparent_copy_plan(avail_in),
                GzLookTransparentCopyPlan {
                    have: avail_in as ::core::ffi::c_uint,
                    avail_in: 0,
                    how: crate::gzguts_h::COPY,
                }
            );
        }
    }

    #[test]
    fn gz_decomp_reports_unexpected_eof_only_without_retry_state() {
        assert!(gz_decomp_reports_unexpected_eof(0));
        assert!(!gz_decomp_reports_unexpected_eof(1));
        assert!(!gz_decomp_reports_unexpected_eof(-1));
    }

    #[test]
    fn gz_decomp_output_progress_tracks_produced_bytes_and_rewind() {
        assert_eq!(
            gz_decomp_output_progress(10, 4),
            GzDecompOutputProgress {
                have: 6,
                rewind_len: 6,
            }
        );
    }

    #[test]
    fn gz_decomp_output_progress_preserves_unsigned_wrapping() {
        assert_eq!(
            gz_decomp_output_progress(0, 1),
            GzDecompOutputProgress {
                have: ::core::ffi::c_uint::MAX,
                rewind_len: ::core::ffi::c_uint::MAX as usize,
            }
        );
    }

    #[test]
    fn gz_decomp_apply_output_progress_rewinds_stream_output_cursor() {
        let mut buffer = [0; 10];
        let mut output = crate::zlib_h::gzFile_s {
            have: 0,
            next: buffer.as_mut_ptr(),
            pos: 0,
        };
        let stream = crate::zlib_h::z_stream_s {
            next_in: ::core::ptr::null_mut(),
            avail_in: 0,
            total_in: 0,
            next_out: buffer.as_mut_ptr().wrapping_add(8),
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
        };

        gz_decomp_apply_output_progress(
            &mut output,
            &stream,
            &GzDecompOutputProgress {
                have: 3,
                rewind_len: 3,
            },
        );

        assert_eq!(output.have, 3);
        assert_eq!(output.next, buffer.as_mut_ptr().wrapping_add(5));
    }

    #[test]
    fn gz_decomp_should_continue_requires_remaining_output_space() {
        assert!(!gz_decomp_should_continue(crate::zlib_h::Z_OK, 0));
        assert!(!gz_decomp_should_continue(crate::zlib_h::Z_BUF_ERROR, 0));
    }

    #[test]
    fn gz_decomp_should_continue_stops_at_stream_end() {
        assert!(!gz_decomp_should_continue(crate::zlib_h::Z_STREAM_END, 1));
        assert!(!gz_decomp_should_continue(
            crate::zlib_h::Z_STREAM_END,
            crate::stdlib::uInt::MAX,
        ));
    }

    #[test]
    fn gz_decomp_should_continue_for_nonterminal_results_with_space() {
        assert!(gz_decomp_should_continue(crate::zlib_h::Z_OK, 1));
        assert!(gz_decomp_should_continue(
            crate::zlib_h::Z_BUF_ERROR,
            crate::stdlib::uInt::MAX,
        ));
    }

    #[test]
    fn gz_decomp_apply_result_restores_look_only_at_stream_end() {
        let mut how = crate::gzguts_h::GZIP;
        let mut junk = 1;
        assert_eq!(
            gz_decomp_apply_result(&mut how, &mut junk, crate::zlib_h::Z_STREAM_END),
            0
        );
        assert_eq!(how, crate::gzguts_h::LOOK);
        assert_eq!(junk, 0);
    }

    #[test]
    fn gz_decomp_apply_result_preserves_state_for_ok_and_errors() {
        for ret in [
            crate::zlib_h::Z_OK,
            crate::zlib_h::Z_DATA_ERROR,
            crate::zlib_h::Z_STREAM_ERROR,
        ] {
            let mut how = crate::gzguts_h::GZIP;
            let mut junk = 1;
            assert_eq!(
                gz_decomp_apply_result(&mut how, &mut junk, ret),
                if ret == crate::zlib_h::Z_OK { 0 } else { -1 }
            );
            assert_eq!(how, crate::gzguts_h::GZIP);
            assert_eq!(junk, 1);
        }
    }

    #[test]
    fn gz_decomp_decision_maps_stream_and_dictionary_errors_to_internal_error() {
        for ret in [crate::zlib_h::Z_STREAM_ERROR, crate::zlib_h::Z_NEED_DICT] {
            assert_eq!(
                gz_decomp_decision(ret, false, 0, 1),
                GzDecompDecision {
                    clear_junk: false,
                    action: GzDecompAction::InternalError,
                }
            );
        }
    }

    #[test]
    fn gz_decomp_decision_maps_memory_errors() {
        assert_eq!(
            gz_decomp_decision(crate::zlib_h::Z_MEM_ERROR, false, 0, 1),
            GzDecompDecision {
                clear_junk: false,
                action: GzDecompAction::MemoryError,
            }
        );
    }

    #[test]
    fn gz_decomp_decision_detects_trailing_junk() {
        assert_eq!(
            gz_decomp_decision(crate::zlib_h::Z_DATA_ERROR, false, 1, 1),
            GzDecompDecision {
                clear_junk: false,
                action: GzDecompAction::TrailingJunk,
            }
        );
    }

    #[test]
    fn gz_decomp_trailing_junk_plan_discards_input_and_restarts_look() {
        assert_eq!(
            gz_decomp_trailing_junk_plan(),
            GzDecompTrailingJunkPlan {
                avail_in: 0,
                eof: 1,
                how: crate::gzguts_h::LOOK,
            }
        );
    }

    #[test]
    fn gz_decomp_apply_trailing_junk_plan_updates_only_planned_state() {
        let plan = gz_decomp_trailing_junk_plan();
        let mut avail_in = 12;
        let mut eof = 0;
        let mut how = crate::gzguts_h::GZIP;

        gz_decomp_apply_trailing_junk_plan(&mut avail_in, &mut eof, &mut how, &plan);

        assert_eq!(avail_in, 0);
        assert_eq!(eof, 1);
        assert_eq!(how, crate::gzguts_h::LOOK);
    }

    #[test]
    fn gz_decomp_decision_maps_data_errors_without_trailing_junk() {
        assert_eq!(
            gz_decomp_decision(crate::zlib_h::Z_DATA_ERROR, false, 0, 1),
            GzDecompDecision {
                clear_junk: false,
                action: GzDecompAction::DataError,
            }
        );
    }

    #[test]
    fn gz_decomp_data_error_message_selects_inflate_text_only_when_present() {
        assert_eq!(
            gz_decomp_data_error_message(false),
            GzDecompDataErrorMessage::Generic
        );
        assert_eq!(
            gz_decomp_data_error_message(true),
            GzDecompDataErrorMessage::Inflate
        );
    }

    #[test]
    fn gz_decomp_decision_clears_junk_before_classifying_data_errors() {
        assert_eq!(
            gz_decomp_decision(crate::zlib_h::Z_DATA_ERROR, true, 1, 1),
            GzDecompDecision {
                clear_junk: true,
                action: GzDecompAction::DataError,
            }
        );
    }

    #[test]
    fn gz_decomp_junk_after_output_clears_only_when_output_was_produced() {
        assert_eq!(gz_decomp_junk_after_output(7, true), 0);
        assert_eq!(gz_decomp_junk_after_output(-1, true), 0);
        assert_eq!(gz_decomp_junk_after_output(7, false), 7);
        assert_eq!(gz_decomp_junk_after_output(-1, false), -1);
    }

    #[test]
    fn gz_decomp_decision_continues_only_with_output_space_and_no_stream_end() {
        assert_eq!(
            gz_decomp_decision(crate::zlib_h::Z_OK, false, 0, 1).action,
            GzDecompAction::Continue
        );
        assert_eq!(
            gz_decomp_decision(crate::zlib_h::Z_BUF_ERROR, false, 0, 1).action,
            GzDecompAction::Continue
        );
        assert_eq!(
            gz_decomp_decision(crate::zlib_h::Z_OK, false, 0, 0).action,
            GzDecompAction::Stop
        );
        assert_eq!(
            gz_decomp_decision(crate::zlib_h::Z_STREAM_END, false, 0, 1).action,
            GzDecompAction::Stop
        );
    }

    #[test]
    fn gz_load_read_len_uses_remaining_bytes_below_cap() {
        assert_eq!(gz_load_read_len(10, 4, 8), 6);
        assert_eq!(gz_load_read_len(4, 4, 8), 0);
    }

    #[test]
    fn gz_load_read_len_caps_large_requests() {
        assert_eq!(gz_load_read_len(20, 4, 8), 8);
        assert_eq!(gz_load_read_len(12, 4, 8), 8);
    }

    #[test]
    fn gz_load_read_len_preserves_unsigned_wrapping_before_capping() {
        assert_eq!(gz_load_read_len(0, 1, 8), 8);
    }

    #[test]
    fn gz_load_max_read_len_is_one_quarter_of_the_unsigned_range() {
        assert_eq!(
            gz_load_max_read_len(),
            (1 as ::core::ffi::c_uint) << (::core::ffi::c_uint::BITS - 2)
        );
    }

    #[test]
    fn gz_load_max_read_len_caps_full_unsigned_requests() {
        let max = gz_load_max_read_len();

        assert_eq!(gz_load_read_len(::core::ffi::c_uint::MAX, 0, max), max);
    }

    #[test]
    fn gz_fread_action_skips_empty_requests() {
        assert_eq!(gz_fread_action(0), GzFreadAction::ReturnZero);
    }

    #[test]
    fn gz_fread_action_reads_nonempty_requests() {
        assert_eq!(gz_fread_action(1), GzFreadAction::Read);
        assert_eq!(
            gz_fread_action(crate::stdlib::z_size_t::MAX),
            GzFreadAction::Read
        );
    }

    #[test]
    fn gzgetc_action_consumes_available_buffered_data() {
        assert_eq!(gzgetc_action(1), GzgetcAction::ConsumeBuffered);
        assert_eq!(
            gzgetc_action(::core::ffi::c_uint::MAX),
            GzgetcAction::ConsumeBuffered
        );
    }

    #[test]
    fn gzgetc_action_reads_when_no_buffered_data_is_available() {
        assert_eq!(gzgetc_action(0), GzgetcAction::Read);
    }

    #[test]
    fn gzgetc_read_result_returns_error_when_no_byte_was_read() {
        assert_eq!(gzgetc_read_result(0, 42), -1);
    }

    #[test]
    fn gzgetc_read_result_returns_the_read_byte() {
        assert_eq!(gzgetc_read_result(1, 0), 0);
        assert_eq!(gzgetc_read_result(1, ::core::ffi::c_uchar::MAX), 255);
    }

    #[test]
    fn gzgetc_buffered_result_advances_state_and_returns_byte() {
        assert_eq!(gzgetc_buffered_result(3, 42, 255), (2, 43, 255));
        assert_eq!(
            gzgetc_buffered_result(1, crate::stdlib::off64_t::MAX, 0),
            (0, crate::stdlib::off64_t::MIN, 0)
        );
    }

    #[test]
    fn gz_load_apply_state_commits_state_without_error() {
        let load = GzLoadState {
            have: 3,
            eof: 1,
            again: 0,
            more: false,
            error: None,
        };
        let mut eof = 0;
        let mut again = 1;

        assert_eq!(gz_load_apply_state(&mut eof, &mut again, &load), Ok(()));
        assert_eq!(eof, 1);
        assert_eq!(again, 0);
    }

    #[test]
    fn gz_load_apply_state_commits_state_before_returning_error() {
        let load = GzLoadState {
            have: 0,
            eof: -1,
            again: 1,
            more: false,
            error: Some(crate::stdlib::EAGAIN),
        };
        let mut eof = 0;
        let mut again = 0;

        assert_eq!(
            gz_load_apply_state(&mut eof, &mut again, &load),
            Err(crate::stdlib::EAGAIN)
        );
        assert_eq!(eof, -1);
        assert_eq!(again, 1);
    }

    #[test]
    fn gz_load_transition_commits_partial_read_and_continues() {
        let mut eof = 0;
        let mut again = 1;

        assert_eq!(
            gz_load_transition(5, 2, &mut eof, &mut again, Ok(1)),
            GzLoadTransition::Continue { have: 3 }
        );
        assert_eq!(eof, 0);
        assert_eq!(again, 0);
    }

    #[test]
    fn gz_load_transition_completes_on_eof() {
        let mut eof = 0;
        let mut again = 1;

        assert_eq!(
            gz_load_transition(5, 2, &mut eof, &mut again, Ok(0)),
            GzLoadTransition::Complete { have: 2 }
        );
        assert_eq!(eof, 1);
        assert_eq!(again, 0);
    }

    #[test]
    fn gz_load_transition_commits_retryable_error_before_returning_it() {
        let mut eof = 0;
        let mut again = 0;

        assert_eq!(
            gz_load_transition(5, 0, &mut eof, &mut again, Err(crate::stdlib::EAGAIN)),
            GzLoadTransition::Error {
                have: 0,
                errno: crate::stdlib::EAGAIN,
            }
        );
        assert_eq!(eof, 0);
        assert_eq!(again, 1);
    }

    #[test]
    fn gz_load_read_loop_preserves_partial_data_on_retryable_error() {
        let mut eof = 0;
        let mut again = 0;
        let mut reads = [Ok(3), Err(crate::stdlib::EAGAIN)].into_iter();
        let mut requests = Vec::new();

        assert_eq!(
            gz_load_read_loop(8, &mut eof, &mut again, |have, get| {
                requests.push((have, get));
                reads.next().unwrap()
            }),
            Ok(3)
        );
        assert_eq!(requests, vec![(0, 8), (3, 5)]);
        assert_eq!(eof, 0);
        assert_eq!(again, 1);
    }

    #[test]
    fn gz_load_read_loop_marks_eof_after_partial_reads() {
        let mut eof = 0;
        let mut again = 1;
        let mut reads = [Ok(3), Ok(0)].into_iter();
        let mut requests = Vec::new();

        assert_eq!(
            gz_load_read_loop(8, &mut eof, &mut again, |have, get| {
                requests.push((have, get));
                reads.next().unwrap()
            }),
            Ok(3)
        );
        assert_eq!(requests, vec![(0, 8), (3, 5)]);
        assert_eq!(eof, 1);
        assert_eq!(again, 0);
    }

    #[test]
    fn gzread_request_converts_only_signed_int_representable_lengths() {
        let largest_valid = ::core::ffi::c_int::MAX as ::core::ffi::c_uint;

        assert_eq!(gzread_request(0), Some(0));
        assert_eq!(gzread_request(largest_valid), Some(largest_valid as _));
        assert_eq!(gzread_request(largest_valid.wrapping_add(1)), None);
        assert_eq!(gzread_request(::core::ffi::c_uint::MAX), None);
    }

    #[test]
    fn gz_read_error_is_recoverable_accepts_ok_and_buffer_errors() {
        assert!(gz_read_error_is_recoverable(crate::zlib_h::Z_OK, 0));
        assert!(gz_read_error_is_recoverable(crate::zlib_h::Z_BUF_ERROR, 0));
    }

    #[test]
    fn gz_read_error_is_recoverable_accepts_retryable_errors() {
        assert!(gz_read_error_is_recoverable(crate::zlib_h::Z_DATA_ERROR, 1));
    }

    #[test]
    fn gz_read_error_is_recoverable_rejects_non_retryable_errors() {
        assert!(!gz_read_error_is_recoverable(
            crate::zlib_h::Z_DATA_ERROR,
            0
        ));
    }

    #[test]
    fn gz_is_read_mode_accepts_only_read_mode() {
        assert!(gz_is_read_mode(crate::gzguts_h::GZ_READ));
        assert!(!gz_is_read_mode(crate::gzguts_h::GZ_WRITE));
        assert!(!gz_is_read_mode(::core::ffi::c_int::MIN));
        assert!(!gz_is_read_mode(::core::ffi::c_int::MAX));
    }

    #[test]
    fn gz_read_state_is_usable_requires_read_mode_and_recoverable_errors() {
        assert!(gz_read_state_is_usable(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_OK,
            0
        ));
        assert!(gz_read_state_is_usable(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_DATA_ERROR,
            1
        ));
        assert!(!gz_read_state_is_usable(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_OK,
            0
        ));
        assert!(!gz_read_state_is_usable(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_DATA_ERROR,
            0
        ));
    }

    #[test]
    fn gzread_outcome_returns_nonzero_reads_even_with_state_flags() {
        assert!(matches!(
            gzread_outcome(5, crate::zlib_h::Z_DATA_ERROR, 1),
            GzreadOutcome::Read(5)
        ));
    }

    #[test]
    fn gzread_outcome_classifies_empty_reads() {
        assert!(matches!(
            gzread_outcome(0, crate::zlib_h::Z_DATA_ERROR, 0),
            GzreadOutcome::Error
        ));
        assert!(matches!(
            gzread_outcome(0, crate::zlib_h::Z_BUF_ERROR, 1),
            GzreadOutcome::Again
        ));
        assert!(matches!(
            gzread_outcome(0, crate::zlib_h::Z_OK, 0),
            GzreadOutcome::Read(0)
        ));
    }

    #[test]
    fn gz_fread_items_read_counts_only_complete_items() {
        assert_eq!(gz_fread_items_read(4, 11), 2);
        assert_eq!(gz_fread_items_read(4, 12), 3);
    }

    #[test]
    fn gz_fread_items_read_handles_zero_item_size() {
        assert_eq!(gz_fread_items_read(0, 12), 0);
    }

    #[test]
    fn gz_read_chunk_len_limits_requests_to_remaining_length() {
        assert_eq!(gz_read_chunk_len(17, 0), 17);
    }

    #[test]
    fn gz_read_chunk_len_limits_requests_to_buffered_data() {
        assert_eq!(gz_read_chunk_len(17, 5), 5);
        assert_eq!(gz_read_chunk_len(5, 17), 5);
    }

    #[test]
    fn gz_read_chunk_len_caps_requests_at_uint_max() {
        let max = ::core::ffi::c_uint::MAX;
        let request = (max as crate::stdlib::z_size_t).checked_add(1);

        assert_eq!(
            gz_read_chunk_len(request.unwrap_or(max as crate::stdlib::z_size_t), 0),
            max
        );
    }

    #[test]
    fn gz_read_drain_plan_updates_healthy_buffer_state() {
        let plan = gz_read_drain_plan(7, crate::zlib_h::Z_OK, 3);
        assert_eq!(plan.remaining_have, 4);
        assert_eq!(plan.err, 0);
        assert_eq!(plan.next_advance, 3);
    }

    #[test]
    fn gz_read_apply_drain_plan_commits_buffer_and_error_scalars() {
        let plan = GzReadDrainPlan {
            remaining_have: 6,
            err: crate::zlib_h::Z_DATA_ERROR,
            next_advance: 4,
        };
        let mut have = 10;
        let mut err = crate::zlib_h::Z_OK;

        gz_read_apply_drain_plan(&mut have, &mut err, &plan);

        assert_eq!(have, 6);
        assert_eq!(err, crate::zlib_h::Z_DATA_ERROR);
    }

    #[test]
    fn gz_read_drain_plan_wraps_and_reports_state_errors() {
        let plan = gz_read_drain_plan(0, crate::zlib_h::Z_DATA_ERROR, 1);
        assert_eq!(plan.remaining_have, ::core::ffi::c_uint::MAX);
        assert_eq!(plan.err, -1);
        assert_eq!(plan.next_advance, 1);
    }

    #[test]
    fn gz_read_take_decompressed_returns_output_and_clears_buffered_bytes() {
        assert_eq!(gz_read_take_decompressed(17), (17, 0));
        assert_eq!(
            gz_read_take_decompressed(::core::ffi::c_uint::MAX),
            (::core::ffi::c_uint::MAX, 0)
        );
    }

    #[test]
    fn gz_read_needs_fetch_for_look_state() {
        assert!(gz_read_needs_fetch(crate::gzguts_h::LOOK, 16, 8));
    }

    #[test]
    fn gz_read_needs_fetch_for_small_chunks_only() {
        assert!(gz_read_needs_fetch(crate::gzguts_h::COPY, 15, 8));
        assert!(!gz_read_needs_fetch(crate::gzguts_h::COPY, 16, 8));
    }

    #[test]
    fn gz_read_has_full_output_chunk_uses_the_wrapping_output_threshold() {
        assert!(!gz_read_has_full_output_chunk(15, 8));
        assert!(gz_read_has_full_output_chunk(16, 8));
        assert!(!gz_read_has_full_output_chunk(
            ::core::ffi::c_uint::MAX - 2,
            ::core::ffi::c_uint::MAX,
        ));
        assert!(gz_read_has_full_output_chunk(
            ::core::ffi::c_uint::MAX - 1,
            ::core::ffi::c_uint::MAX,
        ));
    }

    #[test]
    fn gz_read_stops_at_eof_requires_eof_without_input() {
        assert!(gz_read_stops_at_eof(1, 0));
        assert!(!gz_read_stops_at_eof(0, 0));
        assert!(!gz_read_stops_at_eof(1, 1));
    }

    #[test]
    fn gz_read_action_prioritizes_buffered_data() {
        assert!(matches!(
            gz_read_action(1, 1, 0, crate::gzguts_h::LOOK, 1, 8),
            GzReadAction::DrainBuffered
        ));
    }

    #[test]
    fn gz_read_action_stops_only_after_eof_with_no_input() {
        assert!(matches!(
            gz_read_action(0, 1, 0, crate::gzguts_h::COPY, 16, 8),
            GzReadAction::StopAtEof
        ));
        assert!(matches!(
            gz_read_action(0, 1, 1, crate::gzguts_h::COPY, 16, 8),
            GzReadAction::Load
        ));
    }

    #[test]
    fn gz_read_action_fetches_for_look_or_small_requests() {
        assert!(matches!(
            gz_read_action(0, 0, 0, crate::gzguts_h::LOOK, 16, 8),
            GzReadAction::Fetch
        ));
        assert!(matches!(
            gz_read_action(0, 0, 0, crate::gzguts_h::COPY, 15, 8),
            GzReadAction::Fetch
        ));
    }

    #[test]
    fn gz_read_action_selects_load_or_decompression() {
        assert!(matches!(
            gz_read_action(0, 0, 0, crate::gzguts_h::COPY, 16, 8),
            GzReadAction::Load
        ));
        assert!(matches!(
            gz_read_action(0, 0, 0, crate::gzguts_h::GZIP, 16, 8),
            GzReadAction::Decompress
        ));
    }

    #[test]
    fn gz_read_action_advances_output_only_for_writing_actions() {
        assert!(gz_read_action_advances_output(&GzReadAction::DrainBuffered));
        assert!(!gz_read_action_advances_output(&GzReadAction::StopAtEof));
        assert!(!gz_read_action_advances_output(&GzReadAction::Fetch));
        assert!(gz_read_action_advances_output(&GzReadAction::Load));
        assert!(gz_read_action_advances_output(&GzReadAction::Decompress));
    }

    #[test]
    fn gz_read_step_pairs_chunk_length_with_its_control_action() {
        let buffered = gz_read_step(17, 5, 1, 0, crate::gzguts_h::LOOK, 8);
        assert_eq!(buffered.chunk_len, 5);
        assert!(matches!(buffered.action, GzReadAction::DrainBuffered));

        let unbuffered = gz_read_step(17, 0, 0, 0, crate::gzguts_h::COPY, 8);
        assert_eq!(unbuffered.chunk_len, 17);
        assert!(matches!(unbuffered.action, GzReadAction::Load));
    }

    #[test]
    fn gz_read_progress_updates_remaining_total_and_position() {
        assert_eq!(gz_read_progress(10, 4, 42, 3), (7, 7, 45));
    }

    #[test]
    fn gz_read_progress_preserves_wrapping_byte_counts() {
        assert_eq!(
            gz_read_progress(0, crate::stdlib::z_size_t::MAX, 0, 1),
            (crate::stdlib::z_size_t::MAX, 0, 1)
        );
    }

    #[test]
    fn gz_cursor_advance_preserves_forward_wrapping() {
        assert_eq!(gz_cursor_advance(42, 3), 45);
        assert_eq!(
            gz_cursor_advance(crate::stdlib::off64_t::MAX, 1),
            crate::stdlib::off64_t::MIN
        );
    }

    #[test]
    fn gz_cursor_rewind_preserves_backward_wrapping() {
        assert_eq!(gz_cursor_rewind(42, 3), 39);
        assert_eq!(
            gz_cursor_rewind(crate::stdlib::off64_t::MIN, 1),
            crate::stdlib::off64_t::MAX
        );
    }

    #[test]
    fn gz_read_progress_wraps_position_at_off64_t_boundary() {
        assert_eq!(
            gz_read_progress(1, 0, crate::stdlib::off64_t::MAX, 1),
            (0, 1, crate::stdlib::off64_t::MIN)
        );
    }

    #[test]
    fn gz_read_fetch_failed_without_buffer_requires_both_conditions() {
        assert!(gz_read_fetch_failed_without_buffer(-1, 0));
        assert!(!gz_read_fetch_failed_without_buffer(-1, 1));
    }

    #[test]
    fn gz_read_fetch_failed_without_buffer_ignores_successful_fetches() {
        assert!(!gz_read_fetch_failed_without_buffer(0, 0));
        assert!(!gz_read_fetch_failed_without_buffer(1, 0));
    }

    #[test]
    fn gz_read_fetch_error_reports_only_unbuffered_fetch_failures() {
        assert_eq!(gz_read_fetch_error(-1, 0), Some(-1));
        assert_eq!(gz_read_fetch_error(-1, 1), None);
        assert_eq!(gz_read_fetch_error(0, 0), None);
    }

    #[test]
    fn gz_read_should_continue_requires_remaining_output_without_errors() {
        assert!(gz_read_should_continue(1, 0));
        assert!(!gz_read_should_continue(0, 0));
        assert!(!gz_read_should_continue(1, -1));
    }

    #[test]
    fn gz_read_loop_progress_advances_scalars_and_continues() {
        assert_eq!(
            gz_read_loop_progress(true, 10, 4, 42, 3, 0),
            GzReadLoopProgress {
                len: 7,
                got: 7,
                pos: 45,
                should_continue: true,
            }
        );
    }

    #[test]
    fn gz_read_loop_progress_preserves_scalars_without_advance() {
        assert_eq!(
            gz_read_loop_progress(false, 10, 4, 42, 3, 0),
            GzReadLoopProgress {
                len: 10,
                got: 4,
                pos: 42,
                should_continue: true,
            }
        );
    }

    #[test]
    fn gz_read_loop_progress_stops_after_error() {
        assert_eq!(
            gz_read_loop_progress(true, 10, 4, 42, 3, -1),
            GzReadLoopProgress {
                len: 7,
                got: 7,
                pos: 45,
                should_continue: false,
            }
        );
    }

    #[test]
    fn gz_read_request_is_empty_matches_only_zero_length_requests() {
        assert!(gz_read_request_is_empty(0));
        assert!(!gz_read_request_is_empty(1));
    }

    #[test]
    fn gz_read_request_is_empty_handles_large_unsigned_lengths() {
        assert!(!gz_read_request_is_empty(crate::stdlib::z_size_t::MAX));
    }

    #[test]
    fn gz_read_setup_prioritizes_empty_requests_over_pending_skip() {
        assert_eq!(gz_read_setup(0, 7), GzReadSetup::ReturnEmpty);
    }

    #[test]
    fn gz_read_setup_runs_pending_skip_before_reading() {
        assert_eq!(gz_read_setup(1, -1), GzReadSetup::Skip);
    }

    #[test]
    fn gz_read_setup_reads_without_pending_skip() {
        assert_eq!(gz_read_setup(1, 0), GzReadSetup::Read);
    }

    #[test]
    fn gz_ungetc_buffer_state_prioritizes_empty_buffer() {
        assert!(matches!(
            gz_ungetc_buffer_state(0, 8),
            GzUngetcBufferState::Empty
        ));
    }

    #[test]
    fn gz_ungetc_buffer_state_treats_full_or_overfull_buffers_as_full() {
        assert!(matches!(
            gz_ungetc_buffer_state(15, 8),
            GzUngetcBufferState::Pushable
        ));
        assert!(matches!(
            gz_ungetc_buffer_state(16, 8),
            GzUngetcBufferState::Full
        ));
        assert!(matches!(
            gz_ungetc_buffer_state(17, 8),
            GzUngetcBufferState::Full
        ));
    }

    #[test]
    fn gz_ungetc_action_preserves_empty_full_and_compaction_decisions() {
        assert_eq!(
            gz_ungetc_action(0, 8, true),
            GzUngetcAction::Empty { write_index: 15 }
        );
        assert_eq!(gz_ungetc_action(16, 8, false), GzUngetcAction::Full);
        assert_eq!(gz_ungetc_action(17, 8, true), GzUngetcAction::Full);
        assert_eq!(
            gz_ungetc_action(15, 8, true),
            GzUngetcAction::Pushable { compact: true }
        );
        assert_eq!(
            gz_ungetc_action(15, 8, false),
            GzUngetcAction::Pushable { compact: false }
        );
    }

    #[test]
    fn gz_ungetc_compact_plan_moves_buffered_bytes_to_the_output_tail() {
        assert_eq!(
            gz_ungetc_compact_plan(1, 8),
            GzUngetcCompactPlan {
                dest_index: 15,
                len: 1,
            }
        );
        assert_eq!(
            gz_ungetc_compact_plan(15, 8),
            GzUngetcCompactPlan {
                dest_index: 1,
                len: 15,
            }
        );
    }

    #[test]
    fn gz_ungetc_next_have_wraps_buffered_count() {
        assert_eq!(gz_ungetc_next_have(0), 1);
        assert_eq!(gz_ungetc_next_have(::core::ffi::c_uint::MAX), 0);
    }

    #[test]
    fn gz_ungetc_accepts_only_nonnegative_input_bytes() {
        assert!(!gz_ungetc_accepts_byte(::core::ffi::c_int::MIN));
        assert!(!gz_ungetc_accepts_byte(-1));
        assert!(gz_ungetc_accepts_byte(0));
        assert!(gz_ungetc_accepts_byte(
            ::core::ffi::c_uchar::MAX as ::core::ffi::c_int
        ));
    }

    #[test]
    fn gz_ungetc_progress_updates_buffer_position_and_past() {
        assert_eq!(gz_ungetc_progress(4, 42), (5, 41, 0));
        assert_eq!(gz_ungetc_progress(::core::ffi::c_uint::MAX, 0), (0, -1, 0));
    }

    #[test]
    fn gz_ungetc_progress_wraps_minimum_position_and_buffered_count() {
        assert_eq!(
            gz_ungetc_progress(::core::ffi::c_uint::MAX, crate::stdlib::off64_t::MIN,),
            (0, crate::stdlib::off64_t::MAX, 0)
        );
    }

    #[test]
    fn gz_is_gzip_header_accepts_valid_header() {
        assert!(gz_is_gzip_header(31, 139, 8, 31));
    }

    #[test]
    fn gz_is_gzip_header_rejects_invalid_magic_or_flags() {
        assert!(!gz_is_gzip_header(30, 139, 8, 0));
        assert!(!gz_is_gzip_header(31, 139, 8, 32));
    }

    #[test]
    fn gz_look_header_is_available_requires_all_four_header_bytes() {
        assert!(!gz_look_header_is_available(3));
        assert!(gz_look_header_is_available(4));
        assert!(gz_look_header_is_available(crate::stdlib::uInt::MAX));
    }

    #[test]
    fn gz_look_forces_gzip_for_direct_or_completed_members() {
        assert!(gz_look_forces_gzip(-1, 1));
        assert!(gz_look_forces_gzip(0, 0));
        assert!(!gz_look_forces_gzip(0, 1));
    }

    #[test]
    fn gz_look_needs_more_input_requires_data_for_initial_probe() {
        assert!(gz_look_needs_more_input(0, 0));
        assert!(!gz_look_needs_more_input(4, 0));
    }

    #[test]
    fn gz_look_needs_more_input_retries_until_header_is_wide_enough() {
        assert!(gz_look_needs_more_input(3, 1));
        assert!(!gz_look_needs_more_input(4, 1));
    }

    #[test]
    fn gz_output_buffer_len_doubles_with_unsigned_wrapping() {
        assert_eq!(gz_output_buffer_len(8), 16);
        assert_eq!(
            gz_output_buffer_len(::core::ffi::c_uint::MAX),
            ::core::ffi::c_uint::MAX - 1
        );
    }

    #[test]
    fn gz_look_apply_gzip_state_updates_all_gzip_fields() {
        let mut how = crate::gzguts_h::LOOK;
        let mut junk = -1;
        let mut direct = -1;

        gz_look_apply_gzip_state(
            &mut how,
            &mut junk,
            &mut direct,
            gz_look_gzip_state(GzLookGzipSource::Forced {
                junk_is_known: false,
            }),
        );

        assert_eq!(how, crate::gzguts_h::GZIP);
        assert_eq!(junk, 0);
        assert_eq!(direct, 0);
    }

    #[test]
    fn gz_look_action_preserves_partial_and_transparent_input_rules() {
        assert_eq!(gz_look_action(0, 0, None), GzLookAction::NeedMoreInput);
        assert_eq!(gz_look_action(3, 1, None), GzLookAction::NeedMoreInput);
        assert_eq!(
            gz_look_action(4, 0, Some([31, 139, 8, 31])),
            GzLookAction::Gzip
        );
        assert_eq!(
            gz_look_action(4, 0, Some([31, 139, 8, 32])),
            GzLookAction::TransparentCopy
        );
        assert_eq!(gz_look_action(3, 0, None), GzLookAction::TransparentCopy);
    }

    #[test]
    fn gz_fetch_action_dispatches_exactly_known_read_modes() {
        assert_eq!(gz_fetch_action(crate::gzguts_h::LOOK), GzFetchAction::Look);
        assert_eq!(gz_fetch_action(crate::gzguts_h::COPY), GzFetchAction::Copy);
        assert_eq!(gz_fetch_action(crate::gzguts_h::GZIP), GzFetchAction::Gzip);
    }

    #[test]
    fn gz_fetch_action_rejects_unknown_read_modes() {
        assert_eq!(gz_fetch_action(-1), GzFetchAction::StateCorrupt);
        assert_eq!(gz_fetch_action(99), GzFetchAction::StateCorrupt);
    }

    #[test]
    fn gz_fetch_post_action_returns_when_look_remains_selected() {
        assert_eq!(
            gz_fetch_post_action(GzFetchAction::Look, crate::gzguts_h::LOOK, 0, 0, 0),
            GzFetchPostAction::Return
        );
    }

    #[test]
    fn gz_fetch_post_action_repeats_look_or_gzip_only_with_work_remaining() {
        assert_eq!(
            gz_fetch_post_action(GzFetchAction::Look, crate::gzguts_h::GZIP, 0, 0, 0),
            GzFetchPostAction::Continue
        );
        assert_eq!(
            gz_fetch_post_action(GzFetchAction::Gzip, crate::gzguts_h::GZIP, 0, 1, 1),
            GzFetchPostAction::Continue
        );
        assert_eq!(
            gz_fetch_post_action(GzFetchAction::Gzip, crate::gzguts_h::GZIP, 1, 0, 1),
            GzFetchPostAction::Return
        );
        assert_eq!(
            gz_fetch_post_action(GzFetchAction::Gzip, crate::gzguts_h::GZIP, 0, 1, 0),
            GzFetchPostAction::Return
        );
    }

    #[test]
    fn gz_fetch_output_capacity_doubles_sizes_with_unsigned_wrapping() {
        assert_eq!(gz_fetch_output_capacity(0), 0);
        assert_eq!(gz_fetch_output_capacity(8), 16);
        assert_eq!(
            gz_fetch_output_capacity(::core::ffi::c_uint::MAX),
            ::core::ffi::c_uint::MAX - 1
        );
    }

    #[test]
    fn gz_fetch_post_action_stops_copy_and_corrupt_state_actions() {
        assert_eq!(
            gz_fetch_post_action(GzFetchAction::Copy, crate::gzguts_h::COPY, 0, 0, 0),
            GzFetchPostAction::Return
        );
        assert_eq!(
            gz_fetch_post_action(GzFetchAction::StateCorrupt, 99, 0, 0, 0),
            GzFetchPostAction::Return
        );
    }

    #[test]
    fn gz_skip_len_limits_consumption_by_skip_and_buffered_input() {
        assert_eq!(gz_skip_len(10, 3, 5), 3);
        assert_eq!(gz_skip_len(10, 10, 5), 10);
        assert_eq!(gz_skip_len(10, 15, 5), 10);
    }

    #[test]
    fn gz_skip_is_limited_by_remaining_clamps_and_preserves_equality() {
        assert!(gz_skip_is_limited_by_remaining(
            10,
            3,
            ::core::ffi::c_uint::MAX
        ));
        assert!(!gz_skip_is_limited_by_remaining(
            10,
            10,
            ::core::ffi::c_uint::MAX
        ));
    }

    #[test]
    fn gz_skip_is_limited_by_remaining_checks_architecture_intmax_boundary() {
        let intmax = crate::src::gzlib::gz_intmax();
        let same_width = ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>();

        assert!(!gz_skip_is_limited_by_remaining(
            intmax,
            crate::stdlib::off64_t::MAX,
            intmax
        ));
        assert_eq!(
            gz_skip_is_limited_by_remaining(intmax + 1, crate::stdlib::off64_t::MAX, intmax),
            same_width
        );
    }

    #[test]
    fn gz_skip_progress_updates_buffered_state_and_cursor() {
        assert_eq!(
            gz_skip_progress(10, 42, 3, 5),
            GzSkipProgress {
                remaining_have: 7,
                pos: 45,
                remaining_skip: 0,
                consumed: 3,
            }
        );
    }

    #[test]
    fn gz_skip_progress_wraps_signed_position_at_boundary() {
        assert_eq!(
            gz_skip_progress(1, crate::stdlib::off64_t::MAX, 1, ::core::ffi::c_uint::MAX),
            GzSkipProgress {
                remaining_have: 0,
                pos: crate::stdlib::off64_t::MIN,
                remaining_skip: 0,
                consumed: 1,
            }
        );
    }

    #[test]
    fn gz_skip_apply_progress_commits_buffered_skip_scalars() {
        let progress = GzSkipProgress {
            remaining_have: 7,
            pos: 45,
            remaining_skip: 0,
            consumed: 3,
        };
        let mut have = 10;
        let mut pos = 42;
        let mut skip = 3;

        gz_skip_apply_progress(&mut have, &mut pos, &mut skip, &progress);

        assert_eq!(have, 7);
        assert_eq!(pos, 45);
        assert_eq!(skip, 0);
    }

    #[test]
    fn gz_skip_apply_progress_preserves_wrapping_progress_values() {
        let progress = GzSkipProgress {
            remaining_have: ::core::ffi::c_uint::MAX,
            pos: crate::stdlib::off64_t::MIN,
            remaining_skip: crate::stdlib::off64_t::MAX,
            consumed: 1,
        };
        let mut have = 0;
        let mut pos = crate::stdlib::off64_t::MAX;
        let mut skip = crate::stdlib::off64_t::MIN;

        gz_skip_apply_progress(&mut have, &mut pos, &mut skip, &progress);

        assert_eq!(have, ::core::ffi::c_uint::MAX);
        assert_eq!(pos, crate::stdlib::off64_t::MIN);
        assert_eq!(skip, crate::stdlib::off64_t::MAX);
    }

    #[test]
    fn gz_skip_remaining_preserves_signed_wrapping() {
        assert_eq!(gz_skip_remaining(3, 1), 2);
        assert_eq!(
            gz_skip_remaining(crate::stdlib::off64_t::MIN, 1),
            crate::stdlib::off64_t::MAX
        );
    }

    #[test]
    fn gz_skip_action_prioritizes_buffered_data_over_eof() {
        assert!(matches!(
            gz_skip_action(1, 1, 0),
            GzSkipAction::ConsumeBuffered
        ));
    }

    #[test]
    fn gz_skip_action_stops_only_at_eof_without_input() {
        assert!(matches!(gz_skip_action(0, 1, 0), GzSkipAction::StopAtEof));
        assert!(matches!(gz_skip_action(0, 1, 1), GzSkipAction::Fetch));
    }

    #[test]
    fn gz_skip_step_pairs_buffered_input_with_its_progress() {
        assert_eq!(
            gz_skip_step(10, 42, 3, 1, 0, 5),
            GzSkipStep::ConsumeBuffered(GzSkipProgress {
                remaining_have: 7,
                pos: 45,
                remaining_skip: 0,
                consumed: 3,
            })
        );
    }

    #[test]
    fn gz_skip_step_preserves_eof_and_fetch_boundaries() {
        assert_eq!(gz_skip_step(0, 42, 3, 1, 0, 5), GzSkipStep::StopAtEof);
        assert_eq!(gz_skip_step(0, 42, 3, 0, 0, 5), GzSkipStep::Fetch);
    }

    #[test]
    fn gz_skip_loop_decision_preserves_buffered_skip_progress() {
        assert_eq!(
            gz_skip_loop_decision(GzSkipAction::ConsumeBuffered, false, 1),
            GzSkipLoopDecision::Continue
        );
        assert_eq!(
            gz_skip_loop_decision(GzSkipAction::ConsumeBuffered, false, 0),
            GzSkipLoopDecision::Done
        );
    }

    #[test]
    fn gz_skip_loop_decision_stops_at_eof_before_fetch_status() {
        assert_eq!(
            gz_skip_loop_decision(GzSkipAction::StopAtEof, true, 1),
            GzSkipLoopDecision::Done
        );
    }

    #[test]
    fn gz_skip_loop_decision_reports_fetch_failure_or_remaining_work() {
        assert_eq!(
            gz_skip_loop_decision(GzSkipAction::Fetch, true, 1),
            GzSkipLoopDecision::Error
        );
        assert_eq!(
            gz_skip_loop_decision(GzSkipAction::Fetch, false, 1),
            GzSkipLoopDecision::Continue
        );
        assert_eq!(
            gz_skip_loop_decision(GzSkipAction::Fetch, false, 0),
            GzSkipLoopDecision::Done
        );
    }

    #[test]
    fn gz_skip_fetch_failed_accepts_only_failed_fetches() {
        assert!(gz_skip_fetch_failed(
            &GzSkipAction::Fetch,
            Some(-1 as ::core::ffi::c_int),
        ));
        assert!(!gz_skip_fetch_failed(&GzSkipAction::Fetch, Some(0)));
        assert!(!gz_skip_fetch_failed(&GzSkipAction::Fetch, None));
    }

    #[test]
    fn gz_skip_fetch_failed_ignores_results_for_non_fetch_actions() {
        assert!(!gz_skip_fetch_failed(
            &GzSkipAction::ConsumeBuffered,
            Some(-1 as ::core::ffi::c_int),
        ));
        assert!(!gz_skip_fetch_failed(
            &GzSkipAction::StopAtEof,
            Some(-1 as ::core::ffi::c_int),
        ));
    }

    #[test]
    fn gzgets_copy_len_prefers_smaller_buffer_limit() {
        assert_eq!(gzgets_copy_len(10, 4, None), 4);
        assert_eq!(gzgets_copy_len(4, 10, None), 4);
    }

    #[test]
    fn gzgets_copy_len_stops_after_newline_within_limit() {
        assert_eq!(gzgets_copy_len(10, 8, Some(0)), 1);
        assert_eq!(gzgets_copy_len(10, 8, Some(4)), 5);
    }

    #[test]
    fn gzgets_copy_len_ignores_newline_past_copy_limit() {
        assert_eq!(gzgets_copy_len(10, 4, Some(4)), 4);
        assert_eq!(gzgets_copy_len(3, 8, Some(9)), 3);
    }

    #[test]
    fn gzgets_progress_updates_buffer_request_and_position() {
        assert_eq!(gzgets_progress(10, 8, 42, 3), (7, 5, 45));
    }

    #[test]
    fn gzgets_progress_preserves_wrapping_byte_counts() {
        assert_eq!(
            gzgets_progress(0, 0, 42, 1),
            (::core::ffi::c_uint::MAX, ::core::ffi::c_uint::MAX, 43)
        );
    }

    #[test]
    fn gzgets_should_continue_requires_space_without_a_newline() {
        assert!(!gzgets_should_continue(0, false));
        assert!(!gzgets_should_continue(0, true));
        assert!(gzgets_should_continue(1, false));
        assert!(!gzgets_should_continue(1, true));
        assert!(gzgets_should_continue(::core::ffi::c_uint::MAX, false));
    }

    #[test]
    fn gzgets_apply_post_fetch_decision_only_marks_past_at_eof() {
        let mut past = 0;

        assert!(!gzgets_apply_post_fetch_decision(
            &mut past,
            GzgetsPostFetchDecision::Stop,
        ));
        assert_eq!(past, 0);

        assert!(gzgets_apply_post_fetch_decision(
            &mut past,
            GzgetsPostFetchDecision::Copy,
        ));
        assert_eq!(past, 0);

        assert!(!gzgets_apply_post_fetch_decision(
            &mut past,
            GzgetsPostFetchDecision::MarkPastAndStop,
        ));
        assert_eq!(past, 1);
    }

    #[test]
    fn gzgets_copy_progress_updates_state_and_loop_decision() {
        assert_eq!(
            gzgets_copy_progress(10, 8, 42, 3, false),
            GzgetsCopyProgress {
                have: 7,
                left: 5,
                pos: 45,
                should_continue: true,
            }
        );
        assert_eq!(
            gzgets_copy_progress(10, 8, 42, 3, true),
            GzgetsCopyProgress {
                have: 7,
                left: 5,
                pos: 45,
                should_continue: false,
            }
        );
        assert!(!gzgets_copy_progress(3, 3, 42, 3, false).should_continue);
    }

    #[test]
    fn gzgets_copy_progress_preserves_wrapping_counts() {
        assert_eq!(
            gzgets_copy_progress(0, 0, 42, 1, false),
            GzgetsCopyProgress {
                have: ::core::ffi::c_uint::MAX,
                left: ::core::ffi::c_uint::MAX,
                pos: 43,
                should_continue: true,
            }
        );
    }

    #[test]
    fn gz_read_needs_look_only_for_empty_read_look_state() {
        assert!(gz_read_needs_look(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::LOOK,
            0
        ));
        assert!(!gz_read_needs_look(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::COPY,
            0
        ));
        assert!(!gz_read_needs_look(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::LOOK,
            1
        ));
        assert!(!gz_read_needs_look(
            crate::gzguts_h::GZ_WRITE,
            crate::gzguts_h::LOOK,
            0
        ));
    }

    #[test]
    fn gz_read_has_pending_skip_requires_nonzero_skip() {
        assert!(!gz_read_has_pending_skip(0));
        assert!(gz_read_has_pending_skip(1));
        assert!(gz_read_has_pending_skip(-1));
    }

    #[test]
    fn gzgets_post_fetch_decision_stops_after_an_empty_failed_fetch() {
        assert_eq!(
            gzgets_post_fetch_decision(0, -1),
            GzgetsPostFetchDecision::Stop
        );
    }

    #[test]
    fn gzgets_post_fetch_decision_marks_past_after_other_empty_fetches() {
        assert_eq!(
            gzgets_post_fetch_decision(0, -2),
            GzgetsPostFetchDecision::MarkPastAndStop
        );
        assert_eq!(
            gzgets_post_fetch_decision(0, 0),
            GzgetsPostFetchDecision::MarkPastAndStop
        );
        assert_eq!(
            gzgets_post_fetch_decision(0, ::core::ffi::c_int::MAX),
            GzgetsPostFetchDecision::MarkPastAndStop
        );
    }

    #[test]
    fn gzgets_post_fetch_decision_copies_buffered_data_regardless_of_fetch() {
        assert_eq!(
            gzgets_post_fetch_decision(1, -1),
            GzgetsPostFetchDecision::Copy
        );
        assert_eq!(
            gzgets_post_fetch_decision(1, 0),
            GzgetsPostFetchDecision::Copy
        );
        assert_eq!(
            gzgets_post_fetch_decision(1, ::core::ffi::c_int::MAX),
            GzgetsPostFetchDecision::Copy
        );
    }

    #[test]
    fn gzgets_request_has_capacity_requires_space_for_a_terminator() {
        assert!(!gzgets_request_has_capacity(-1));
        assert!(!gzgets_request_has_capacity(0));
        assert!(gzgets_request_has_capacity(1));
    }

    #[test]
    fn gzgets_has_valid_inputs_rejects_missing_file() {
        assert!(!gzgets_has_valid_inputs(false, true, 1));
    }

    #[test]
    fn gzgets_has_valid_inputs_rejects_missing_buffer() {
        assert!(!gzgets_has_valid_inputs(true, false, 1));
    }

    #[test]
    fn gzgets_has_valid_inputs_rejects_negative_and_zero_lengths() {
        assert!(!gzgets_has_valid_inputs(true, true, -1));
        assert!(!gzgets_has_valid_inputs(true, true, 0));
    }

    #[test]
    fn gzgets_has_valid_inputs_accepts_minimum_and_maximum_lengths() {
        assert!(gzgets_has_valid_inputs(true, true, 1));
        assert!(gzgets_has_valid_inputs(true, true, ::core::ffi::c_int::MAX));
    }

    #[test]
    fn gzgets_remaining_capacity_reserves_the_terminator() {
        assert_eq!(gzgets_remaining_capacity(1), 0);
        assert_eq!(gzgets_remaining_capacity(2), 1);
        assert_eq!(
            gzgets_remaining_capacity(::core::ffi::c_int::MAX),
            (::core::ffi::c_int::MAX as ::core::ffi::c_uint) - 1
        );
    }

    #[test]
    fn gzgets_copied_any_tracks_remaining_capacity_cursor() {
        assert!(!gzgets_copied_any(8, 8));
        assert!(gzgets_copied_any(8, 7));
        assert!(gzgets_copied_any(::core::ffi::c_uint::MAX, 0));
    }

    #[test]
    fn gzgets_needs_fetch_only_without_buffered_bytes() {
        assert!(gzgets_needs_fetch(0));
        assert!(!gzgets_needs_fetch(1));
        assert!(!gzgets_needs_fetch(::core::ffi::c_uint::MAX));
    }

    #[test]
    fn gz_read_marks_past_eof_only_for_unfilled_eof_requests() {
        assert!(gz_read_marks_past_eof(1, 1));
        assert!(!gz_read_marks_past_eof(0, 1));
        assert!(!gz_read_marks_past_eof(1, 0));
    }

    #[test]
    fn gz_read_note_past_eof_updates_only_for_unfilled_eof_requests() {
        let mut past = 0;
        gz_read_note_past_eof(&mut past, 1, 1);
        assert_eq!(past, 1);

        past = -1;
        gz_read_note_past_eof(&mut past, 0, 1);
        assert_eq!(past, -1);

        gz_read_note_past_eof(&mut past, 1, 0);
        assert_eq!(past, -1);
    }

    #[test]
    fn gz_read_load_status_maps_load_success_and_failure() {
        assert_eq!(gz_read_load_status(false), 0);
        assert_eq!(gz_read_load_status(true), -1);
    }

    #[test]
    fn gzdirect_result_accepts_only_direct_mode() {
        assert_eq!(gzdirect_result(1), 1);
        assert_eq!(gzdirect_result(0), 0);
        assert_eq!(gzdirect_result(-1), 0);
        assert_eq!(gzdirect_result(::core::ffi::c_int::MIN), 0);
        assert_eq!(gzdirect_result(::core::ffi::c_int::MAX), 0);
    }

    #[test]
    fn gzclose_r_result_preserves_buffer_error_on_clean_close() {
        assert_eq!(
            gzclose_r_result(crate::zlib_h::Z_BUF_ERROR, 0),
            crate::zlib_h::Z_BUF_ERROR
        );
        assert_eq!(
            gzclose_r_result(crate::zlib_h::Z_DATA_ERROR, 0),
            crate::zlib_h::Z_OK
        );
    }

    #[test]
    fn gzclose_r_result_prioritizes_close_failures() {
        assert_eq!(
            gzclose_r_result(crate::zlib_h::Z_BUF_ERROR, -1),
            crate::zlib_h::Z_ERRNO
        );
    }
}

unsafe fn gz_read(
    state: crate::gzguts_h::gz_statep,
    mut buf: crate::stdlib::voidp,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let state_ref = &mut *state;
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    match gz_read_setup(len, state_ref.skip) {
        GzReadSetup::ReturnEmpty => return 0 as crate::stdlib::z_size_t,
        GzReadSetup::Skip => {
            if gz_skip!(state_ref) {
                return 0 as crate::stdlib::z_size_t;
            }
        }
        GzReadSetup::Read => {}
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    loop {
        let step = gz_read_step(
            len,
            state_ref.x.have,
            state_ref.eof,
            state_ref.strm.avail_in,
            state_ref.how,
            state_ref.size,
        );
        n = step.chunk_len;
        let advance = gz_read_action_advances_output(&step.action);
        match step.action {
            GzReadAction::DrainBuffered => {
                let next = state_ref.x.next;
                let have = state_ref.x.have;
                let state_err = state_ref.err;
                let plan = gz_read_drain_plan(have, state_err, n);
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                state_ref.x.next = state_ref.x.next.wrapping_add(plan.next_advance);
                gz_read_apply_drain_plan(&mut state_ref.x.have, &mut err, &plan);
            }
            GzReadAction::StopAtEof => break,
            GzReadAction::Fetch => {
                if let Some(fetch_error) =
                    gz_read_fetch_error(gz_fetch(state_ref), state_ref.x.have)
                {
                    err = fetch_error;
                }
            }
            GzReadAction::Load => {
                let load = gz_load(state_ref, buf as *mut ::core::ffi::c_uchar, n);
                n = load.have;
                err = gz_read_load_status(load.failed);
            }
            GzReadAction::Decompress => {
                state_ref.strm.avail_out = n as crate::stdlib::uInt;
                state_ref.strm.next_out =
                    buf as *mut ::core::ffi::c_uchar as *mut crate::stdlib::Bytef;
                err = gz_decomp(state_ref);
                (n, state_ref.x.have) = gz_read_take_decompressed(state_ref.x.have);
            }
        }
        let progress = gz_read_loop_progress(advance, len, got, state_ref.x.pos, n, err);
        len = progress.len;
        got = progress.got;
        state_ref.x.pos = progress.pos;
        if advance {
            buf =
                (buf as *mut ::core::ffi::c_char).wrapping_add(n as usize) as crate::stdlib::voidp;
        }
        if !progress.should_continue {
            break;
        }
    }
    gz_read_note_past_eof(&mut state_ref.past, len, state_ref.eof);
    return got;
}

#[export_name = "gzread"]
pub unsafe extern "C" fn gzread_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_read_state_is_usable((*state).mode, (*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(request_len) = gzread_request(len) else {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    };
    len = gz_read(state, buf, request_len) as ::core::ffi::c_uint;
    match gzread_outcome(len, (*state).err, (*state).again) {
        GzreadOutcome::Read(read) => read,
        GzreadOutcome::Error => -1 as ::core::ffi::c_int,
        GzreadOutcome::Again => {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_ERRNO,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            -1 as ::core::ffi::c_int
        }
    }
}
#[export_name = "gzfread"]
pub unsafe extern "C" fn gzfread_ffi(
    mut buf: crate::stdlib::voidp,
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
    if !gz_read_state_is_usable((*state).mode, (*state).err, (*state).again) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let Some(request_len) = crate::src::gzlib::gz_request_len(size, nitems) else {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as crate::stdlib::z_size_t;
    };
    len = request_len;
    return match gz_fread_action(len) {
        GzFreadAction::ReturnZero => 0 as crate::stdlib::z_size_t,
        GzFreadAction::Read => gz_fread_items_read(size, gz_read(state, buf, len)),
    };
}
#[export_name = "gzgetc"]
pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_read_state_is_usable((*state).mode, (*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let state_ref = &mut *state;
    match gzgetc_action(state_ref.x.have) {
        GzgetcAction::ConsumeBuffered => {
            let buffered = &mut state_ref.x;
            let next = buffered.next;
            let (have, pos, result) = gzgetc_buffered_result(buffered.have, buffered.pos, *next);
            buffered.have = have;
            buffered.pos = pos;
            buffered.next = next.wrapping_add(1);
            return result;
        }
        GzgetcAction::Read => {}
    }
    return gzgetc_read_result(
        gz_read(
            state_ref,
            &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidp,
            1 as crate::stdlib::z_size_t,
        ),
        buf[0 as ::core::ffi::c_int as usize],
    );
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc_ffi(file)
}
#[export_name = "gzungetc"]
pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_is_read_mode((*state).mode) {
        return -1 as ::core::ffi::c_int;
    }
    if gz_read_needs_look(crate::gzguts_h::GZ_READ, (*state).how, (*state).x.have) {
        gz_look(state);
    }
    if !gz_read_error_is_recoverable((*state).err, (*state).again) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let state_ref = &mut *state;
    if gz_read_has_pending_skip(state_ref.skip) && gz_skip!(state_ref) {
        return -1 as ::core::ffi::c_int;
    }
    if !gz_ungetc_accepts_byte(c) {
        return -1 as ::core::ffi::c_int;
    }
    match gz_ungetc_action(
        state_ref.x.have,
        state_ref.size,
        state_ref.x.next == state_ref.out,
    ) {
        GzUngetcAction::Empty { write_index } => {
            let (have, pos, past) = gz_ungetc_progress(state_ref.x.have, state_ref.x.pos);
            state_ref.x.have = have;
            state_ref.x.next = state_ref.out.wrapping_add(write_index);
            *state_ref.x.next = c as ::core::ffi::c_uchar;
            state_ref.x.pos = pos;
            state_ref.past = past;
            return c;
        }
        GzUngetcAction::Full => {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_DATA_ERROR,
                b"out of room to push characters\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        GzUngetcAction::Pushable { compact } => {
            if compact {
                let plan = gz_ungetc_compact_plan(state_ref.x.have, state_ref.size);
                let mut remaining = plan.len;
                while remaining != 0 {
                    remaining -= 1;
                    *state_ref.out.wrapping_add(plan.dest_index + remaining) =
                        *state_ref.out.wrapping_add(remaining);
                }
                state_ref.x.next = state_ref.out.wrapping_add(plan.dest_index);
            }
        }
    }
    let (have, pos, past) = gz_ungetc_progress(state_ref.x.have, state_ref.x.pos);
    state_ref.x.have = have;
    state_ref.x.next = state_ref.x.next.wrapping_sub(1);
    *state_ref.x.next = c as ::core::ffi::c_uchar;
    state_ref.x.pos = pos;
    state_ref.past = past;
    return c;
}
#[export_name = "gzgets"]
pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut left: ::core::ffi::c_uint = 0;
    let initial_left: ::core::ffi::c_uint;
    let mut n: ::core::ffi::c_uint = 0;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut eol: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if !gzgets_has_valid_inputs(!file.is_null(), !buf.is_null(), len) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_read_state_is_usable((*state).mode, (*state).err, (*state).again) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let state_ref = &mut *state;
    if gz_read_has_pending_skip(state_ref.skip) && gz_skip!(state_ref) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    str = buf;
    initial_left = gzgets_remaining_capacity(len);
    left = initial_left;
    if left != 0 {
        loop {
            let fetch = if gzgets_needs_fetch(state_ref.x.have) {
                gz_fetch(state_ref)
            } else {
                0
            };
            if !gzgets_apply_post_fetch_decision(
                &mut state_ref.past,
                gzgets_post_fetch_decision(state_ref.x.have, fetch),
            ) {
                break;
            }
            n = gzgets_copy_len(state_ref.x.have, left, None);
            eol = crate::stdlib::memchr(
                state_ref.x.next as *const ::core::ffi::c_void,
                '\n' as i32,
                n as crate::__stddef_size_t_h::size_t,
            ) as *mut ::core::ffi::c_uchar;
            if !eol.is_null() {
                n = gzgets_copy_len(
                    state_ref.x.have,
                    left,
                    Some(eol.offset_from(state_ref.x.next) as usize),
                );
            }
            crate::stdlib::memcpy(
                buf as *mut ::core::ffi::c_void,
                state_ref.x.next as *const ::core::ffi::c_void,
                n as crate::__stddef_size_t_h::size_t,
            );
            let progress =
                gzgets_copy_progress(state_ref.x.have, left, state_ref.x.pos, n, !eol.is_null());
            state_ref.x.have = progress.have;
            left = progress.left;
            state_ref.x.pos = progress.pos;
            state_ref.x.next = state_ref.x.next.wrapping_add(n as usize);
            buf = buf.wrapping_add(n as usize);
            if !progress.should_continue {
                break;
            }
        }
    }
    if !gzgets_copied_any(initial_left, left) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *buf = 0 as ::core::ffi::c_char;
    return str;
}
#[export_name = "gzdirect"]
pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let state = file as crate::gzguts_h::gz_statep;
    if gz_read_needs_look((*state).mode, (*state).how, (*state).x.have) {
        gz_look(state);
    }
    gzdirect_result((*state).direct)
}
#[export_name = "gzclose_r"]
pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let stream_err: ::core::ffi::c_int;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if !gz_is_read_mode((*state).mode) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).size != 0 {
        crate::src::inflate::inflateEnd_ffi(
            &raw mut (*state).strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        crate::stdlib::free((*state).out as *mut ::core::ffi::c_void);
        crate::stdlib::free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    stream_err = (*state).err;
    crate::src::gzlib::gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    crate::stdlib::free((*state).path as *mut ::core::ffi::c_void);
    ret = crate::stdlib::close((*state).fd);
    crate::stdlib::free(state as *mut ::core::ffi::c_void);
    return gzclose_r_result(stream_err, ret);
}
