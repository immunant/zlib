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

#[derive(Debug, PartialEq, Eq)]
struct GzLoadDecision {
    have: ::core::ffi::c_uint,
    eof: bool,
    again: bool,
    more: bool,
    error: Option<::core::ffi::c_int>,
}

struct GzLoadResult {
    have: ::core::ffi::c_uint,
    failed: bool,
}

fn gz_load_decision(
    have: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
    read: Result<::core::ffi::c_uint, ::core::ffi::c_int>,
) -> GzLoadDecision {
    match read {
        Ok(0) => GzLoadDecision {
            have,
            eof: true,
            again: false,
            more: false,
            error: None,
        },
        Ok(got) => {
            let have = have.wrapping_add(got);
            GzLoadDecision {
                have,
                eof: false,
                again: false,
                more: have < len,
                error: None,
            }
        }
        Err(errno) => {
            let again = errno == crate::stdlib::EAGAIN || errno == crate::stdlib::EWOULDBLOCK;
            GzLoadDecision {
                have,
                eof: false,
                again,
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

fn gz_load_read_len(
    len: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    max: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let get = len.wrapping_sub(have);
    if get > max {
        max
    } else {
        get
    }
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

fn gz_avail_refill_len(
    size: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    size.wrapping_sub(avail_in as ::core::ffi::c_uint)
}

fn gz_avail_should_compact(compact_input: bool, input_is_buffer_start: bool) -> bool {
    compact_input && !input_is_buffer_start
}

fn gzread_request_fits_int(len: ::core::ffi::c_uint) -> bool {
    (len as ::core::ffi::c_int) >= 0
}

fn gz_read_error_is_recoverable(err: ::core::ffi::c_int, again: ::core::ffi::c_int) -> bool {
    err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR || again != 0
}

fn gz_read_state_is_usable(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
) -> bool {
    mode == crate::gzguts_h::GZ_READ && gz_read_error_is_recoverable(err, again)
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
    how == crate::gzguts_h::LOOK || chunk_len < gz_output_buffer_len(size)
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

fn gz_read_fetch_failed_without_buffer(
    fetch_result: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
) -> bool {
    fetch_result == -1 as ::core::ffi::c_int && have == 0
}

fn gz_cursor_advance(
    pos: crate::stdlib::off64_t,
    consumed: ::core::ffi::c_uint,
) -> crate::stdlib::off64_t {
    pos.wrapping_add(consumed as crate::stdlib::off64_t)
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

fn gz_read_request_is_empty(len: crate::stdlib::z_size_t) -> bool {
    len == 0 as crate::stdlib::z_size_t
}

fn gz_read_marks_past_eof(len: crate::stdlib::z_size_t, eof: ::core::ffi::c_int) -> bool {
    len != 0 && eof != 0
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
    (gz_ungetc_next_have(have), pos.wrapping_sub(1), 0)
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
    state: crate::gzguts_h::gz_statep,
    buf: *mut ::core::ffi::c_uchar,
    len: ::core::ffi::c_uint,
) -> GzLoadResult {
    let max = gz_load_max_read_len();
    let state_ref = &mut *state;
    let mut have = 0 as ::core::ffi::c_uint;
    state_ref.again = 0 as ::core::ffi::c_int;
    *crate::stdlib::__errno_location() = 0 as ::core::ffi::c_int;
    loop {
        let get = gz_load_read_len(len, have, max);
        let ret = crate::stdlib::read(
            state_ref.fd,
            buf.wrapping_add(have as usize) as *mut ::core::ffi::c_void,
            get as crate::__stddef_size_t_h::size_t,
        ) as ::core::ffi::c_int;
        let read = if ret < 0 {
            Err(*crate::stdlib::__errno_location())
        } else {
            Ok(ret as ::core::ffi::c_uint)
        };
        let decision = gz_load_decision(have, len, read);
        if decision.eof {
            state_ref.eof = 1 as ::core::ffi::c_int;
        }
        if decision.again {
            state_ref.again = 1 as ::core::ffi::c_int;
        }
        have = decision.have;
        if let Some(errno) = decision.error {
            crate::src::gzlib::gz_error(
                state as *mut crate::gzguts_h::gz_state,
                crate::zlib_h::Z_ERRNO,
                crate::stdlib::strerror(errno),
            );
            return GzLoadResult { have, failed: true };
        }
        if !decision.more {
            return GzLoadResult {
                have,
                failed: false,
            };
        }
    }
}

unsafe extern "C" fn gz_avail(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let action = {
        let state_ref = &mut *state;
        gz_avail_action(state_ref.err, state_ref.eof, state_ref.strm.avail_in)
    };
    match action {
        GzAvailAction::Error => return -1 as ::core::ffi::c_int,
        GzAvailAction::Done => return 0 as ::core::ffi::c_int,
        GzAvailAction::Refill { compact_input } => {
            let state_ref = &mut *state;
            let p = state_ref.in_0;
            let q = state_ref.strm.next_in;
            if gz_avail_should_compact(compact_input, q == p) {
                core::ptr::copy(q, p, state_ref.strm.avail_in as usize);
            }
            let (buf, len) = {
                let state_ref = &*state;
                (
                    state_ref
                        .in_0
                        .wrapping_add(state_ref.strm.avail_in as usize),
                    gz_avail_refill_len(state_ref.size, state_ref.strm.avail_in),
                )
            };
            let load = gz_load(state, buf, len);
            if load.failed {
                return -1 as ::core::ffi::c_int;
            }
            let state_ref = &mut *state;
            state_ref.strm.avail_in = state_ref.strm.avail_in.wrapping_add(load.have);
            state_ref.strm.next_in = state_ref.in_0 as *mut crate::stdlib::Bytef;
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

fn gz_look_forces_gzip(direct: ::core::ffi::c_int, junk: ::core::ffi::c_int) -> bool {
    direct == -1 || junk == 0
}

fn gz_look_needs_more_input(avail_in: crate::stdlib::uInt, again: ::core::ffi::c_int) -> bool {
    avail_in == 0 || again != 0 && avail_in < 4
}

fn gz_output_buffer_len(size: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    size << 1 as ::core::ffi::c_int
}

enum GzLookGzipSource {
    Forced,
    Header,
}

fn gz_look_gzip_junk(source: GzLookGzipSource, junk: &mut ::core::ffi::c_int) {
    *junk = match source {
        GzLookGzipSource::Forced => (*junk != -1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        GzLookGzipSource::Header => 1 as ::core::ffi::c_int,
    };
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

unsafe extern "C" fn gz_look(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint {
        (*state).in_0 = crate::stdlib::malloc((*state).want as crate::__stddef_size_t_h::size_t)
            as *mut ::core::ffi::c_uchar;
        (*state).out = crate::stdlib::malloc(
            gz_output_buffer_len((*state).want) as crate::__stddef_size_t_h::size_t
        ) as *mut ::core::ffi::c_uchar;
        if (*state).in_0.is_null() || (*state).out.is_null() {
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
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
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
    if gz_look_forces_gzip((*state).direct, (*state).junk) {
        crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
        (*state).how = crate::gzguts_h::GZIP;
        gz_look_gzip_junk(GzLookGzipSource::Forced, &mut (*state).junk);
        (*state).direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if gz_avail(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    let header = if (*strm).avail_in > 3 as crate::stdlib::uInt {
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
    match gz_look_action((*strm).avail_in, (*state).again, header) {
        GzLookAction::NeedMoreInput => return 0 as ::core::ffi::c_int,
        GzLookAction::Gzip => {
            crate::src::inflate::inflateReset(strm as *mut crate::zlib_h::z_stream_s);
            (*state).how = crate::gzguts_h::GZIP;
            gz_look_gzip_junk(GzLookGzipSource::Header, &mut (*state).junk);
            (*state).direct = 0 as ::core::ffi::c_int;
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
    (*state).x.have = (*strm).avail_in as ::core::ffi::c_uint;
    (*strm).avail_in = 0 as crate::stdlib::uInt;
    (*state).how = crate::gzguts_h::COPY;
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

fn gz_decomp_output_len(
    had: ::core::ffi::c_uint,
    avail_out: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    (had as crate::stdlib::uInt).wrapping_sub(avail_out) as ::core::ffi::c_uint
}

fn gz_decomp_output_rewind_len(have: ::core::ffi::c_uint) -> usize {
    have as usize
}

enum GzDecompResult {
    RestartLook,
    Error,
    Ok,
}

fn gz_decomp_result(ret: ::core::ffi::c_int) -> GzDecompResult {
    if ret == crate::zlib_h::Z_STREAM_END {
        GzDecompResult::RestartLook
    } else if ret != crate::zlib_h::Z_OK {
        GzDecompResult::Error
    } else {
        GzDecompResult::Ok
    }
}

fn gz_decomp_decision(
    ret: ::core::ffi::c_int,
    produced_output: bool,
    junk: ::core::ffi::c_int,
    avail_out: crate::stdlib::uInt,
) -> GzDecompDecision {
    let clear_junk = produced_output;
    let junk = if clear_junk { 0 } else { junk };
    let action = if ret == crate::zlib_h::Z_STREAM_ERROR || ret == crate::zlib_h::Z_NEED_DICT {
        GzDecompAction::InternalError
    } else if ret == crate::zlib_h::Z_MEM_ERROR {
        GzDecompAction::MemoryError
    } else if ret == crate::zlib_h::Z_DATA_ERROR && junk == 1 {
        GzDecompAction::TrailingJunk
    } else if ret == crate::zlib_h::Z_DATA_ERROR {
        GzDecompAction::DataError
    } else if avail_out != 0 && ret != crate::zlib_h::Z_STREAM_END {
        GzDecompAction::Continue
    } else {
        GzDecompAction::Stop
    };

    GzDecompDecision { clear_junk, action }
}

unsafe extern "C" fn gz_decomp(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    let mut had: ::core::ffi::c_uint = 0;
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    had = (*strm).avail_out as ::core::ffi::c_uint;
    loop {
        let load_failed = if (*strm).avail_in == 0 as crate::stdlib::uInt {
            gz_avail(state) == -1 as ::core::ffi::c_int
        } else {
            false
        };
        match gz_decomp_input_action(load_failed, (*strm).avail_in) {
            GzDecompInputAction::InputError => {
                ret = (*state).err;
                break;
            }
            GzDecompInputAction::UnexpectedEof => {
                if (*state).again == 0 {
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
        let decision = gz_decomp_decision(
            ret,
            (*strm).avail_out < had,
            (*state).junk,
            (*strm).avail_out,
        );
        if decision.clear_junk {
            (*state).junk = 0 as ::core::ffi::c_int;
        }
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
                (*strm).avail_in = 0 as crate::stdlib::uInt;
                (*state).eof = 1 as ::core::ffi::c_int;
                (*state).how = crate::gzguts_h::LOOK;
                ret = crate::zlib_h::Z_OK;
                break;
            }
            GzDecompAction::DataError => {
                crate::src::gzlib::gz_error(
                    state as *mut crate::gzguts_h::gz_state,
                    crate::zlib_h::Z_DATA_ERROR,
                    if (*strm).msg.is_null() {
                        b"compressed data error\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        (*strm).msg as *const ::core::ffi::c_char
                    },
                );
                break;
            }
            GzDecompAction::Stop => break,
            GzDecompAction::Continue => {}
        }
    }
    (*state).x.have = gz_decomp_output_len(had, (*strm).avail_out);
    (*state).x.next = (*strm)
        .next_out
        .wrapping_sub(gz_decomp_output_rewind_len((*state).x.have));
    match gz_decomp_result(ret) {
        GzDecompResult::RestartLook => {
            (*state).junk = 0 as ::core::ffi::c_int;
            (*state).how = crate::gzguts_h::LOOK;
            0 as ::core::ffi::c_int
        }
        GzDecompResult::Error => -1 as ::core::ffi::c_int,
        GzDecompResult::Ok => 0 as ::core::ffi::c_int,
    }
}

unsafe extern "C" fn gz_fetch(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    let mut strm: crate::zlib_h::z_streamp = &raw mut (*state).strm;
    loop {
        match gz_fetch_action((*state).how) {
            GzFetchAction::Look => {
                if gz_look(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                if (*state).how == crate::gzguts_h::LOOK {
                    return 0 as ::core::ffi::c_int;
                }
            }
            GzFetchAction::Copy => {
                let load = gz_load(state, (*state).out, gz_output_buffer_len((*state).size));
                (*state).x.have = load.have;
                if load.failed {
                    return -1 as ::core::ffi::c_int;
                }
                (*state).x.next = (*state).out;
                return 0 as ::core::ffi::c_int;
            }
            GzFetchAction::Gzip => {
                (*strm).avail_out = gz_output_buffer_len((*state).size) as crate::stdlib::uInt;
                (*strm).next_out = (*state).out as *mut crate::stdlib::Bytef;
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
        if !gz_fetch_should_continue((*state).x.have, (*state).eof, (*strm).avail_in) {
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

fn gz_fetch_action(how: ::core::ffi::c_int) -> GzFetchAction {
    match how {
        crate::gzguts_h::LOOK => GzFetchAction::Look,
        crate::gzguts_h::COPY => GzFetchAction::Copy,
        crate::gzguts_h::GZIP => GzFetchAction::Gzip,
        _ => GzFetchAction::StateCorrupt,
    }
}

fn gz_fetch_should_continue(
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
) -> bool {
    have == 0 && (eof == 0 || avail_in != 0)
}

fn gz_skip_core(
    have: &mut ::core::ffi::c_uint,
    pos: &mut crate::stdlib::off64_t,
    skip: &mut crate::stdlib::off64_t,
    intmax: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let n = gz_skip_len(*have, *skip, intmax);
    *have = have.wrapping_sub(n);
    *pos = gz_cursor_advance(*pos, n);
    *skip -= n as crate::stdlib::off64_t;
    n
}

fn gz_skip_len(
    have: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
    intmax: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if ::core::mem::size_of::<::core::ffi::c_int>() as usize
        == ::core::mem::size_of::<crate::stdlib::off64_t>() as usize
        && have > intmax
        || have as crate::stdlib::off64_t > skip
    {
        skip as ::core::ffi::c_uint
    } else {
        have
    }
}

enum GzSkipAction {
    ConsumeBuffered,
    StopAtEof,
    Fetch,
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

fn gz_skip_should_continue(skip: crate::stdlib::off64_t) -> bool {
    skip != 0
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

unsafe extern "C" fn gz_skip(mut state: crate::gzguts_h::gz_statep) -> ::core::ffi::c_int {
    loop {
        let action = {
            let state_ref = &mut *state;
            gz_skip_action(state_ref.x.have, state_ref.eof, state_ref.strm.avail_in)
        };
        match action {
            GzSkipAction::ConsumeBuffered => {
                let state_ref = &mut *state;
                let n = gz_skip_core(
                    &mut state_ref.x.have,
                    &mut state_ref.x.pos,
                    &mut state_ref.skip,
                    crate::src::gzlib::gz_intmax(),
                );
                state_ref.x.next = state_ref.x.next.wrapping_add(n as usize);
            }
            GzSkipAction::StopAtEof => break,
            GzSkipAction::Fetch => {
                if gz_fetch(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
        }
        let should_continue = {
            let state_ref = &*state;
            gz_skip_should_continue(state_ref.skip)
        };
        if !should_continue {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn gz_avail_input_compaction_preserves_overlapping_tail() {
        let mut input = *b"abcdefgh";
        input.copy_within(2..8, 0);
        assert_eq!(&input[..6], b"cdefgh");
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
    fn gz_avail_refill_len_preserves_remaining_buffer_wrapping() {
        assert_eq!(gz_avail_refill_len(16, 4), 12);
        assert_eq!(gz_avail_refill_len(0, 1), ::core::ffi::c_uint::MAX);
    }

    #[test]
    fn gz_decomp_input_action_prioritizes_load_failure() {
        assert_eq!(
            gz_decomp_input_action(true, 1),
            GzDecompInputAction::InputError
        );
    }

    #[test]
    fn gz_look_gzip_junk_preserves_forced_lookup_state() {
        let mut junk = -1;
        gz_look_gzip_junk(GzLookGzipSource::Forced, &mut junk);
        assert_eq!(junk, 0);

        junk = 0;
        gz_look_gzip_junk(GzLookGzipSource::Forced, &mut junk);
        assert_eq!(junk, 1);

        junk = 1;
        gz_look_gzip_junk(GzLookGzipSource::Forced, &mut junk);
        assert_eq!(junk, 1);
    }

    #[test]
    fn gz_look_gzip_junk_marks_detected_headers_as_junk() {
        let mut junk = -1;
        gz_look_gzip_junk(GzLookGzipSource::Header, &mut junk);
        assert_eq!(junk, 1);

        junk = 0;
        gz_look_gzip_junk(GzLookGzipSource::Header, &mut junk);
        assert_eq!(junk, 1);
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
    fn gz_decomp_output_len_tracks_produced_bytes_with_wrapping() {
        assert_eq!(gz_decomp_output_len(10, 4), 6);
        assert_eq!(gz_decomp_output_len(0, 1), ::core::ffi::c_uint::MAX);
    }

    #[test]
    fn gz_decomp_output_rewind_len_preserves_the_cursor_without_output() {
        assert_eq!(gz_decomp_output_rewind_len(0), 0);
    }

    #[test]
    fn gz_decomp_output_rewind_len_matches_produced_output() {
        assert_eq!(gz_decomp_output_rewind_len(1), 1);
        assert_eq!(gz_decomp_output_rewind_len(42), 42);
        assert_eq!(
            gz_decomp_output_rewind_len(::core::ffi::c_uint::MAX),
            ::core::ffi::c_uint::MAX as usize,
        );
    }

    #[test]
    fn gz_decomp_result_restores_look_only_at_stream_end() {
        assert!(matches!(
            gz_decomp_result(crate::zlib_h::Z_STREAM_END),
            GzDecompResult::RestartLook
        ));
        assert!(matches!(
            gz_decomp_result(crate::zlib_h::Z_DATA_ERROR),
            GzDecompResult::Error
        ));
        assert!(matches!(
            gz_decomp_result(crate::zlib_h::Z_OK),
            GzDecompResult::Ok
        ));
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
    fn gz_load_decision_marks_eof_without_requesting_more_data() {
        assert_eq!(
            gz_load_decision(4, 8, Ok(0)),
            GzLoadDecision {
                have: 4,
                eof: true,
                again: false,
                more: false,
                error: None,
            }
        );
    }

    #[test]
    fn gz_load_decision_requests_more_only_for_partial_reads() {
        assert_eq!(
            gz_load_decision(2, 8, Ok(3)),
            GzLoadDecision {
                have: 5,
                eof: false,
                again: false,
                more: true,
                error: None,
            }
        );
        assert_eq!(
            gz_load_decision(2, 5, Ok(3)),
            GzLoadDecision {
                have: 5,
                eof: false,
                again: false,
                more: false,
                error: None,
            }
        );
    }

    #[test]
    fn gz_load_decision_preserves_buffered_data_on_retryable_errors() {
        assert_eq!(
            gz_load_decision(3, 8, Err(crate::stdlib::EAGAIN)),
            GzLoadDecision {
                have: 3,
                eof: false,
                again: true,
                more: false,
                error: None,
            }
        );
        assert_eq!(
            gz_load_decision(0, 8, Err(crate::stdlib::EWOULDBLOCK)),
            GzLoadDecision {
                have: 0,
                eof: false,
                again: true,
                more: false,
                error: Some(crate::stdlib::EWOULDBLOCK),
            }
        );
    }

    #[test]
    fn gz_load_decision_reports_nonretryable_errors() {
        assert_eq!(
            gz_load_decision(3, 8, Err(5)),
            GzLoadDecision {
                have: 3,
                eof: false,
                again: false,
                more: false,
                error: Some(5),
            }
        );
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
    fn gzread_request_fits_int_checks_signed_int_boundary() {
        let largest_valid = ::core::ffi::c_int::MAX as ::core::ffi::c_uint;

        assert!(gzread_request_fits_int(largest_valid));
        assert!(!gzread_request_fits_int(largest_valid.wrapping_add(1)));
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
    fn gz_read_drain_plan_wraps_and_reports_state_errors() {
        let plan = gz_read_drain_plan(0, crate::zlib_h::Z_DATA_ERROR, 1);
        assert_eq!(plan.remaining_have, ::core::ffi::c_uint::MAX);
        assert_eq!(plan.err, -1);
        assert_eq!(plan.next_advance, 1);
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
    fn gz_read_should_continue_requires_remaining_output_without_errors() {
        assert!(gz_read_should_continue(1, 0));
        assert!(!gz_read_should_continue(0, 0));
        assert!(!gz_read_should_continue(1, -1));
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
    fn gz_fetch_should_continue_only_without_output_and_with_work_remaining() {
        assert!(gz_fetch_should_continue(0, 0, 0));
        assert!(gz_fetch_should_continue(0, 1, 1));
        assert!(!gz_fetch_should_continue(1, 0, 1));
        assert!(!gz_fetch_should_continue(0, 1, 0));
    }

    #[test]
    fn gz_skip_len_limits_consumption_by_skip_and_buffered_input() {
        assert_eq!(gz_skip_len(10, 3, 5), 3);
        assert_eq!(gz_skip_len(10, 10, 5), 10);
        assert_eq!(gz_skip_len(10, 15, 5), 10);
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
    fn gz_skip_should_continue_requires_remaining_skip() {
        assert!(!gz_skip_should_continue(0));
        assert!(gz_skip_should_continue(1));
        assert!(gz_skip_should_continue(-1));
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

unsafe extern "C" fn gz_read(
    mut state: crate::gzguts_h::gz_statep,
    mut buf: crate::stdlib::voidp,
    mut len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    if gz_read_request_is_empty(len) {
        return 0 as crate::stdlib::z_size_t;
    }
    if gz_read_has_pending_skip((*state).skip) && gz_skip(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    loop {
        n = gz_read_chunk_len(len, (*state).x.have);
        let advance = match gz_read_action(
            (*state).x.have,
            (*state).eof,
            (*state).strm.avail_in,
            (*state).how,
            n,
            (*state).size,
        ) {
            GzReadAction::DrainBuffered => {
                let (next, have, state_err) = {
                    let state_ref = &mut *state;
                    (state_ref.x.next, state_ref.x.have, state_ref.err)
                };
                let plan = gz_read_drain_plan(have, state_err, n);
                crate::stdlib::memcpy(
                    buf as *mut ::core::ffi::c_void,
                    next as *const ::core::ffi::c_void,
                    n as crate::__stddef_size_t_h::size_t,
                );
                let state_ref = &mut *state;
                state_ref.x.next = state_ref.x.next.wrapping_add(plan.next_advance);
                state_ref.x.have = plan.remaining_have;
                err = plan.err;
                true
            }
            GzReadAction::StopAtEof => break,
            GzReadAction::Fetch => {
                if gz_read_fetch_failed_without_buffer(gz_fetch(state), (*state).x.have) {
                    err = -1 as ::core::ffi::c_int;
                }
                false
            }
            GzReadAction::Load => {
                let load = gz_load(state, buf as *mut ::core::ffi::c_uchar, n);
                n = load.have;
                err = if load.failed { -1 } else { 0 };
                true
            }
            GzReadAction::Decompress => {
                (*state).strm.avail_out = n as crate::stdlib::uInt;
                (*state).strm.next_out =
                    buf as *mut ::core::ffi::c_uchar as *mut crate::stdlib::Bytef;
                err = gz_decomp(state);
                n = (*state).x.have;
                (*state).x.have = 0 as ::core::ffi::c_uint;
                true
            }
        };
        if advance {
            (len, got, (*state).x.pos) = gz_read_progress(len, got, (*state).x.pos, n);
            buf =
                (buf as *mut ::core::ffi::c_char).wrapping_add(n as usize) as crate::stdlib::voidp;
        }
        if !gz_read_should_continue(len, err) {
            break;
        }
    }
    if gz_read_marks_past_eof(len, (*state).eof) {
        (*state).past = 1 as ::core::ffi::c_int;
    }
    return got;
}
pub unsafe extern "C" fn gzread(
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
    if !gzread_request_fits_int(len) {
        crate::src::gzlib::gz_error(
            state as *mut crate::gzguts_h::gz_state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in an int\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    len = gz_read(state, buf, len as crate::stdlib::z_size_t) as ::core::ffi::c_uint;
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
#[export_name = "gzread"]

pub unsafe extern "C" fn gzread_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzread(file, buf, len)
}
pub unsafe extern "C" fn gzfread(
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
    return if !gz_read_request_is_empty(len) {
        gz_fread_items_read(size, gz_read(state, buf, len))
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
    gzfread(buf, size, nitems, file)
}
pub unsafe extern "C" fn gzgetc(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
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
    let buffered = &mut (*state).x;
    if buffered.have != 0 {
        let next = buffered.next;
        let (have, pos, result) = gzgetc_buffered_result(buffered.have, buffered.pos, *next);
        buffered.have = have;
        buffered.pos = pos;
        buffered.next = next.wrapping_add(1);
        return result;
    }
    return gzgetc_read_result(
        gz_read(
            state,
            &raw mut buf as *mut ::core::ffi::c_uchar as crate::stdlib::voidp,
            1 as crate::stdlib::z_size_t,
        ),
        buf[0 as ::core::ffi::c_int as usize],
    );
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc(file)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzgetc(file)
}
pub unsafe extern "C" fn gzungetc(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
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
    if gz_read_has_pending_skip((*state).skip) && gz_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if !gz_ungetc_accepts_byte(c) {
        return -1 as ::core::ffi::c_int;
    }
    match gz_ungetc_action(
        (*state).x.have,
        (*state).size,
        (*state).x.next == (*state).out,
    ) {
        GzUngetcAction::Empty { write_index } => {
            let (have, pos, past) = gz_ungetc_progress((*state).x.have, (*state).x.pos);
            (*state).x.have = have;
            (*state).x.next = (*state).out.wrapping_add(write_index);
            *(*state).x.next = c as ::core::ffi::c_uchar;
            (*state).x.pos = pos;
            (*state).past = past;
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
                let plan = gz_ungetc_compact_plan((*state).x.have, (*state).size);
                let mut remaining = plan.len;
                while remaining != 0 {
                    remaining -= 1;
                    *(*state).out.wrapping_add(plan.dest_index + remaining) =
                        *(*state).out.wrapping_add(remaining);
                }
                (*state).x.next = (*state).out.wrapping_add(plan.dest_index);
            }
        }
    }
    let (have, pos, past) = gz_ungetc_progress((*state).x.have, (*state).x.pos);
    (*state).x.have = have;
    (*state).x.next = (*state).x.next.wrapping_sub(1);
    *(*state).x.next = c as ::core::ffi::c_uchar;
    (*state).x.pos = pos;
    (*state).past = past;
    return c;
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    gzungetc(c, file)
}
pub unsafe extern "C" fn gzgets(
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
    if file.is_null() || buf.is_null() || !gzgets_request_has_capacity(len) {
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
    if gz_read_has_pending_skip((*state).skip) && gz_skip(state) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    str = buf;
    initial_left = gzgets_remaining_capacity(len);
    left = initial_left;
    if left != 0 {
        loop {
            let fetch = if gzgets_needs_fetch((*state).x.have) {
                gz_fetch(state)
            } else {
                0
            };
            match gzgets_post_fetch_decision((*state).x.have, fetch) {
                GzgetsPostFetchDecision::Stop => break,
                GzgetsPostFetchDecision::MarkPastAndStop => {
                    (*state).past = 1 as ::core::ffi::c_int;
                    break;
                }
                GzgetsPostFetchDecision::Copy => {}
            }
            n = gzgets_copy_len((*state).x.have, left, None);
            eol = crate::stdlib::memchr(
                (*state).x.next as *const ::core::ffi::c_void,
                '\n' as i32,
                n as crate::__stddef_size_t_h::size_t,
            ) as *mut ::core::ffi::c_uchar;
            if !eol.is_null() {
                n = gzgets_copy_len(
                    (*state).x.have,
                    left,
                    Some(eol.offset_from((*state).x.next) as usize),
                );
            }
            crate::stdlib::memcpy(
                buf as *mut ::core::ffi::c_void,
                (*state).x.next as *const ::core::ffi::c_void,
                n as crate::__stddef_size_t_h::size_t,
            );
            ((*state).x.have, left, (*state).x.pos) =
                gzgets_progress((*state).x.have, left, (*state).x.pos, n);
            (*state).x.next = (*state).x.next.wrapping_add(n as usize);
            buf = buf.wrapping_add(n as usize);
            if !gzgets_should_continue(left, !eol.is_null()) {
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
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    gzgets(file, buf, len)
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
pub unsafe extern "C" fn gzclose_r(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let stream_err: ::core::ffi::c_int;
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = file as crate::gzguts_h::gz_statep;
    if (*state).mode != crate::gzguts_h::GZ_READ {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).size != 0 {
        crate::src::inflate::inflateEnd(
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
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzclose_r(file)
}
