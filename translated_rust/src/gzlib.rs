pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__O_LARGEFILE;

pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZBUFSIZE;
pub use crate::gzguts_h::GZ_APPEND;
pub use crate::gzguts_h::GZ_NONE;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::gzguts_h::LOOK;
pub use crate::internal::__INT_MAX__;
pub use crate::limits_h::INT_MAX;
pub use crate::stdlib::fcntl;

pub use crate::stdlib::open;

pub use crate::stdlib::F_GETFD;
pub use crate::stdlib::F_GETFL;
pub use crate::stdlib::F_SETFD;
pub use crate::stdlib::F_SETFL;
pub use crate::stdlib::O_APPEND;
pub use crate::stdlib::O_CLOEXEC;
pub use crate::stdlib::O_CREAT;
pub use crate::stdlib::O_EXCL;
pub use crate::stdlib::O_LARGEFILE;
pub use crate::stdlib::O_NONBLOCK;
pub use crate::stdlib::O_RDONLY;
pub use crate::stdlib::O_TRUNC;
pub use crate::stdlib::O_WRONLY;
pub use crate::stdlib::SEEK_CUR;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;
pub use crate::stdlib::__O_CLOEXEC;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;

pub use crate::stdlib::off64_t;
pub use crate::stdlib::off_t;

pub use crate::src::deflate::internal_state;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
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
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DEFAULT_COMPRESSION;
pub use crate::zlib_h::Z_DEFAULT_STRATEGY;
pub use crate::zlib_h::Z_FILTERED;
pub use crate::zlib_h::Z_FIXED;
pub use crate::zlib_h::Z_HUFFMAN_ONLY;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_RLE;

fn gz_is_read_or_write_mode(mode: ::core::ffi::c_int) -> bool {
    gz_is_read_mode(mode) || mode == crate::gzguts_h::GZ_WRITE
}

fn gz_is_read_mode(mode: ::core::ffi::c_int) -> bool {
    mode == crate::gzguts_h::GZ_READ
}

fn gz_open_has_required_inputs(path_present: bool, mode_present: bool) -> bool {
    path_present && mode_present
}

pub(crate) fn gz_request_len(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    size.checked_mul(nitems)
}

pub(crate) fn gz_errno_is_retryable(errno: ::core::ffi::c_int) -> bool {
    errno == crate::stdlib::EAGAIN || errno == crate::stdlib::EWOULDBLOCK
}

fn gz_clear_read_flags(eof: &mut ::core::ffi::c_int, past: &mut ::core::ffi::c_int) {
    *eof = 0;
    *past = 0;
}

fn gzseek_fast_forward_reset(state: &mut crate::gzguts_h::gz_state) {
    state.x.have = 0;
    gz_clear_read_flags(&mut state.eof, &mut state.past);
    state.skip = 0;
}

fn gzseek_finish_fast_forward(
    state: &mut crate::gzguts_h::gz_state,
    offset: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    state.strm.avail_in = 0;
    state.x.pos += offset;
    state.x.pos
}

fn gzclearerr_core(
    mode: ::core::ffi::c_int,
    eof: &mut ::core::ffi::c_int,
    past: &mut ::core::ffi::c_int,
) -> bool {
    if gz_is_read_or_write_mode(mode) {
        if gz_is_read_mode(mode) {
            gz_clear_read_flags(eof, past);
        }
        true
    } else {
        false
    }
}

fn gz_error_clears_buffer(err: ::core::ffi::c_int, again: ::core::ffi::c_int) -> bool {
    err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && again == 0
}

fn gz_error_needs_message_allocation(err: ::core::ffi::c_int, has_message: bool) -> bool {
    has_message && err != crate::zlib_h::Z_MEM_ERROR
}

fn gz_error_message_allocation_len(
    path_len: crate::__stddef_size_t_h::size_t,
    message_len: crate::__stddef_size_t_h::size_t,
) -> crate::__stddef_size_t_h::size_t {
    path_len.wrapping_add(message_len).wrapping_add(3)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzErrorPlan {
    free_previous_message: bool,
    clear_buffer: bool,
    err: ::core::ffi::c_int,
    allocate_message: bool,
}

fn gz_error_plan(
    has_previous_message: bool,
    previous_err: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    has_message: bool,
) -> GzErrorPlan {
    GzErrorPlan {
        free_previous_message: has_previous_message && previous_err != crate::zlib_h::Z_MEM_ERROR,
        clear_buffer: gz_error_clears_buffer(err, again),
        err,
        allocate_message: gz_error_needs_message_allocation(err, has_message),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzResetFields {
    mode: ::core::ffi::c_int,
    have: crate::stdlib::uInt,
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
    reset: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    pos: crate::stdlib::off64_t,
    avail_in: crate::stdlib::uInt,
}

fn gz_reset_core(fields: &mut GzResetFields) {
    fields.have = 0;
    if gz_is_read_mode(fields.mode) {
        gz_clear_read_flags(&mut fields.eof, &mut fields.past);
        fields.how = crate::gzguts_h::LOOK;
        fields.junk = -1;
    } else {
        fields.reset = 0;
    }
    fields.again = 0;
    fields.skip = 0;
    fields.pos = 0;
    fields.avail_in = 0;
}

fn gz_reset_state(state: &mut crate::gzguts_h::gz_state) {
    let mut fields = GzResetFields {
        mode: state.mode,
        have: state.x.have,
        eof: state.eof,
        past: state.past,
        how: state.how,
        junk: state.junk,
        reset: state.reset,
        again: state.again,
        skip: state.skip,
        pos: state.x.pos,
        avail_in: state.strm.avail_in,
    };
    gz_reset_core(&mut fields);
    state.x.have = fields.have;
    state.eof = fields.eof;
    state.past = fields.past;
    state.how = fields.how;
    state.junk = fields.junk;
    state.reset = fields.reset;
    state.again = fields.again;
    state.skip = fields.skip;
    state.x.pos = fields.pos;
    state.strm.avail_in = fields.avail_in;
}

fn gz_open_defaults(state: &mut crate::gzguts_h::gz_state) {
    state.size = 0;
    state.want = crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint;
    state.err = crate::zlib_h::Z_OK;
    state.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    state.mode = crate::gzguts_h::GZ_NONE;
    state.level = crate::zlib_h::Z_DEFAULT_COMPRESSION;
    state.strategy = crate::zlib_h::Z_DEFAULT_STRATEGY;
    state.direct = 0;
}

fn gzseek_read_buffer_consumed(
    avail_in: crate::stdlib::uInt,
    offset: crate::stdlib::off64_t,
    int_and_off64_same_width: bool,
    int_max: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    if gzseek_read_buffer_uses_requested_offset(avail_in, offset, int_and_off64_same_width, int_max)
    {
        offset as crate::stdlib::uInt
    } else {
        avail_in
    }
}

fn gzseek_read_buffer_uses_requested_offset(
    avail_in: crate::stdlib::uInt,
    offset: crate::stdlib::off64_t,
    int_and_off64_same_width: bool,
    int_max: crate::stdlib::uInt,
) -> bool {
    (int_and_off64_same_width && avail_in > int_max) || avail_in as crate::stdlib::off64_t > offset
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzSeekReadBufferPlan {
    consumed: crate::stdlib::uInt,
    remaining_offset: crate::stdlib::off64_t,
}

fn gzseek_plan_read_buffer_consumption(
    avail_in: crate::stdlib::uInt,
    offset: crate::stdlib::off64_t,
    int_and_off64_same_width: bool,
    int_max: crate::stdlib::uInt,
) -> GzSeekReadBufferPlan {
    let consumed = gzseek_read_buffer_consumed(avail_in, offset, int_and_off64_same_width, int_max);
    GzSeekReadBufferPlan {
        consumed,
        remaining_offset: offset - consumed as crate::stdlib::off64_t,
    }
}

fn gzseek_read_buffer_plan_for_mode(
    mode: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    offset: crate::stdlib::off64_t,
    int_and_off64_same_width: bool,
    int_max: crate::stdlib::uInt,
) -> Option<GzSeekReadBufferPlan> {
    if !gzseek_uses_read_buffer(mode) {
        return None;
    }

    Some(gzseek_plan_read_buffer_consumption(
        avail_in,
        offset,
        int_and_off64_same_width,
        int_max,
    ))
}

fn gzseek_apply_read_buffer_plan(
    state: &mut crate::gzguts_h::gz_state,
    plan: GzSeekReadBufferPlan,
) {
    state.x.have = state.x.have.wrapping_sub(plan.consumed);
    state.x.next = state.x.next.wrapping_add(plan.consumed as usize);
    state.x.pos = state
        .x
        .pos
        .wrapping_add(plan.consumed as crate::stdlib::off64_t);
}

fn gzseek_request_is_valid(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    whence: ::core::ffi::c_int,
) -> bool {
    gz_is_read_or_write_mode(mode)
        && gzseek_error_allows_positioning(err)
        && (whence == crate::stdlib::SEEK_SET || whence == crate::stdlib::SEEK_CUR)
}

fn gzrewind_request_is_valid(mode: ::core::ffi::c_int, err: ::core::ffi::c_int) -> bool {
    gz_is_read_mode(mode) && gzseek_error_allows_positioning(err)
}

fn gz_lseek_succeeded(result: crate::stdlib::__off64_t) -> bool {
    result != -1 as ::core::ffi::c_int as crate::stdlib::__off64_t
}

fn gzseek_error_allows_positioning(err: ::core::ffi::c_int) -> bool {
    err == crate::zlib_h::Z_OK || err == crate::zlib_h::Z_BUF_ERROR
}

fn gzseek_adjust_offset(
    offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
    position: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    if whence == crate::stdlib::SEEK_SET {
        offset - position
    } else {
        offset + gzseek_effective_skip(past, skip)
    }
}

fn gzseek_clears_pending_skip(whence: ::core::ffi::c_int) -> bool {
    whence == crate::stdlib::SEEK_CUR
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct GzSeekRequestPlan {
    offset: crate::stdlib::off64_t,
    clear_pending_skip: bool,
}

fn gzseek_plan_request(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    whence: ::core::ffi::c_int,
    requested_offset: crate::stdlib::off64_t,
    position: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> Option<GzSeekRequestPlan> {
    if !gzseek_request_is_valid(mode, err, whence) {
        return None;
    }

    Some(GzSeekRequestPlan {
        offset: gzseek_adjust_offset(requested_offset, whence, position, past, skip),
        clear_pending_skip: gzseek_clears_pending_skip(whence),
    })
}

fn gzseek_uses_read_buffer(mode: ::core::ffi::c_int) -> bool {
    gz_is_read_mode(mode)
}

fn gzseek_effective_skip(
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    if past != 0 {
        0
    } else {
        skip
    }
}

fn gz_position_after_skip(
    position: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    position + skip
}

fn gzseek_can_fast_forward(
    mode: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    position: crate::stdlib::off64_t,
    offset: crate::stdlib::off64_t,
) -> bool {
    gz_is_read_mode(mode)
        && how == crate::gzguts_h::COPY
        && position + offset >= 0 as crate::stdlib::off64_t
}

fn gzseek_fast_forward_lseek_offset(
    offset: crate::stdlib::off64_t,
    buffered_input: crate::stdlib::uInt,
) -> crate::stdlib::off64_t {
    offset - buffered_input as crate::stdlib::off64_t
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzSeekFastForwardPlan {
    lseek_offset: crate::stdlib::off64_t,
    position: crate::stdlib::off64_t,
}

fn gzseek_plan_fast_forward(
    mode: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    position: crate::stdlib::off64_t,
    offset: crate::stdlib::off64_t,
    buffered_input: crate::stdlib::uInt,
) -> Option<GzSeekFastForwardPlan> {
    if !gzseek_can_fast_forward(mode, how, position, offset) {
        return None;
    }

    Some(GzSeekFastForwardPlan {
        lseek_offset: gzseek_fast_forward_lseek_offset(offset, buffered_input),
        position: gz_position_after_skip(position, offset),
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzSeekOffsetPlan {
    offset: crate::stdlib::off64_t,
    rewind: bool,
}

fn gzseek_plan_remaining_offset(
    mode: ::core::ffi::c_int,
    position: crate::stdlib::off64_t,
    offset: crate::stdlib::off64_t,
) -> Option<GzSeekOffsetPlan> {
    if offset >= 0 {
        return Some(GzSeekOffsetPlan {
            offset,
            rewind: false,
        });
    }
    if !gz_is_read_mode(mode) {
        return None;
    }

    let offset = offset + position;
    if offset < 0 {
        return None;
    }

    Some(GzSeekOffsetPlan {
        offset,
        rewind: true,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzOpenOptions {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzOpenPlan {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
}

fn gz_parse_open_mode(mode: &[u8]) -> Option<GzOpenOptions> {
    let mut options = GzOpenOptions {
        mode: crate::gzguts_h::GZ_NONE,
        level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
        strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
        direct: 0,
        oflag: 0,
        exclusive: 0,
    };

    for &option in mode {
        if option.is_ascii_digit() {
            options.level = (option - b'0') as ::core::ffi::c_int;
            continue;
        }

        match option {
            b'r' => options.mode = crate::gzguts_h::GZ_READ,
            b'w' => options.mode = crate::gzguts_h::GZ_WRITE,
            b'a' => options.mode = crate::gzguts_h::GZ_APPEND,
            b'+' => return None,
            b'e' => options.oflag |= crate::stdlib::O_CLOEXEC,
            b'x' => options.exclusive = 1,
            b'f' => options.strategy = crate::zlib_h::Z_FILTERED,
            b'h' => options.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
            b'R' => options.strategy = crate::zlib_h::Z_RLE,
            b'F' => options.strategy = crate::zlib_h::Z_FIXED,
            b'G' => options.direct = -1,
            b'N' => options.oflag |= crate::stdlib::O_NONBLOCK,
            b'T' => options.direct = 1,
            b'b' | _ => {}
        }
    }

    Some(options)
}

fn gz_prepare_open(options: GzOpenOptions) -> Option<GzOpenPlan> {
    if options.mode == crate::gzguts_h::GZ_NONE {
        return None;
    }

    let direct = if gz_is_read_mode(options.mode) {
        if options.direct == 1 {
            return None;
        }
        if options.direct == 0 {
            1
        } else {
            options.direct
        }
    } else {
        if options.direct == -1 {
            return None;
        }
        options.direct
    };
    let oflag = options.oflag
        | crate::stdlib::O_LARGEFILE
        | if gz_is_read_mode(options.mode) {
            crate::stdlib::O_RDONLY
        } else {
            crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | if options.exclusive != 0 {
                    crate::stdlib::O_EXCL
                } else {
                    0
                }
                | if options.mode == crate::gzguts_h::GZ_WRITE {
                    crate::stdlib::O_TRUNC
                } else {
                    crate::stdlib::O_APPEND
                }
        };

    Some(GzOpenPlan {
        mode: options.mode,
        level: options.level,
        strategy: options.strategy,
        direct,
        oflag,
    })
}

fn gz_apply_open_plan(state: &mut crate::gzguts_h::gz_state, plan: GzOpenPlan) {
    state.mode = plan.mode;
    state.level = plan.level;
    state.strategy = plan.strategy;
    state.direct = plan.direct;
}

fn gz_open_initialize_state(
    state: &mut crate::gzguts_h::gz_state,
    mode: &[u8],
) -> Option<GzOpenPlan> {
    gz_open_defaults(state);
    let plan = gz_parse_open_mode(mode).and_then(gz_prepare_open)?;
    gz_apply_open_plan(state, plan);
    Some(plan)
}

fn gz_post_open_metadata(
    mode: ::core::ffi::c_int,
    current_offset: crate::stdlib::off64_t,
) -> (::core::ffi::c_int, Option<crate::stdlib::off64_t>) {
    if mode == crate::gzguts_h::GZ_APPEND {
        (crate::gzguts_h::GZ_WRITE, None)
    } else if gz_is_read_mode(mode) {
        (
            mode,
            Some(if current_offset == -1 {
                0
            } else {
                current_offset
            }),
        )
    } else {
        (mode, None)
    }
}

fn gz_apply_post_open_metadata(
    state: &mut crate::gzguts_h::gz_state,
    current_offset: crate::stdlib::off64_t,
) {
    let (mode, start) = gz_post_open_metadata(state.mode, current_offset);
    state.mode = mode;
    if let Some(start) = start {
        state.start = start;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GzOpenOffsetPlan {
    whence: ::core::ffi::c_int,
    record_offset: bool,
}

fn gz_open_offset_plan(mode: ::core::ffi::c_int) -> Option<GzOpenOffsetPlan> {
    if mode == crate::gzguts_h::GZ_APPEND {
        Some(GzOpenOffsetPlan {
            whence: crate::stdlib::SEEK_END,
            record_offset: false,
        })
    } else if gz_is_read_mode(mode) {
        Some(GzOpenOffsetPlan {
            whence: crate::stdlib::SEEK_CUR,
            record_offset: true,
        })
    } else {
        None
    }
}

fn gz_open_recorded_offset(
    record_offset: bool,
    current_offset: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    if record_offset {
        current_offset
    } else {
        0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzOpenFdPlan {
    OpenPath,
    AdoptFd {
        set_nonblocking: bool,
        set_close_on_exec: bool,
    },
}

fn gz_open_fd_plan(fd: ::core::ffi::c_int, oflag: ::core::ffi::c_int) -> GzOpenFdPlan {
    if fd == -1 as ::core::ffi::c_int {
        GzOpenFdPlan::OpenPath
    } else {
        GzOpenFdPlan::AdoptFd {
            set_nonblocking: oflag & crate::stdlib::O_NONBLOCK != 0,
            set_close_on_exec: oflag & crate::stdlib::O_CLOEXEC != 0,
        }
    }
}

fn gz_open_fd_succeeded(fd: ::core::ffi::c_int) -> bool {
    fd != -1 as ::core::ffi::c_int
}

fn gz_open_path_buffer_len(len: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    (len as crate::stdlib::z_size_t).wrapping_add(1 as crate::stdlib::z_size_t)
}

fn gz_finish_open(state: &mut crate::gzguts_h::gz_state, current_offset: crate::stdlib::off64_t) {
    gz_apply_post_open_metadata(state, current_offset);
    gz_reset_state(state);
}

unsafe fn gz_open(
    mut path: *const ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut len: crate::stdlib::z_size_t = 0;
    if !gz_open_has_required_inputs(!path.is_null(), !mode.is_null()) {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state = crate::stdlib::malloc(
        ::core::mem::size_of::<crate::gzguts_h::gz_state>() as crate::__stddef_size_t_h::size_t
    ) as crate::gzguts_h::gz_statep;
    if state.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let state = &mut *state;
    let plan = match gz_open_initialize_state(state, ::core::ffi::CStr::from_ptr(mode).to_bytes()) {
        Some(plan) => plan,
        None => {
            crate::stdlib::free(
                state as *mut crate::gzguts_h::gz_state as *mut ::core::ffi::c_void,
            );
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        }
    };
    len = crate::stdlib::strlen(path as *const ::core::ffi::c_char) as crate::stdlib::z_size_t;
    state.path = crate::stdlib::malloc(gz_open_path_buffer_len(len)) as *mut ::core::ffi::c_char;
    if state.path.is_null() {
        crate::stdlib::free(state as *mut crate::gzguts_h::gz_state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    crate::stdlib::snprintf(
        state.path,
        gz_open_path_buffer_len(len),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        path as *const ::core::ffi::c_char,
    );
    match gz_open_fd_plan(fd, plan.oflag) {
        GzOpenFdPlan::OpenPath => {
            state.fd = crate::stdlib::open(
                path as *const ::core::ffi::c_char,
                plan.oflag,
                0o666 as ::core::ffi::c_int,
            );
        }
        GzOpenFdPlan::AdoptFd {
            set_nonblocking,
            set_close_on_exec,
        } => {
            if set_nonblocking {
                crate::stdlib::fcntl(
                    fd,
                    crate::stdlib::F_SETFL,
                    crate::stdlib::fcntl(fd, crate::stdlib::F_GETFL) | crate::stdlib::O_NONBLOCK,
                );
            }
            if set_close_on_exec {
                crate::stdlib::fcntl(
                    fd,
                    crate::stdlib::F_SETFD,
                    crate::stdlib::fcntl(fd, crate::stdlib::F_GETFD) | crate::stdlib::O_CLOEXEC,
                );
            }
            state.fd = fd;
        }
    }
    if !gz_open_fd_succeeded(state.fd) {
        crate::stdlib::free(state.path as *mut ::core::ffi::c_void);
        crate::stdlib::free(state as *mut crate::gzguts_h::gz_state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let current_offset = match gz_open_offset_plan(state.mode) {
        Some(plan) => {
            let offset =
                crate::stdlib::lseek64(state.fd, 0 as crate::stdlib::__off64_t, plan.whence)
                    as crate::stdlib::off64_t;
            gz_open_recorded_offset(plan.record_offset, offset)
        }
        None => 0,
    };
    gz_finish_open(state, current_offset);
    gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return state as *mut crate::gzguts_h::gz_state as crate::zlib_h::gzFile;
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    )
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    )
}
fn gzdopen_has_valid_descriptor(fd: ::core::ffi::c_int) -> bool {
    fd != -1 as ::core::ffi::c_int
}

fn gzdopen_path_buffer_len() -> crate::__stddef_size_t_h::size_t {
    (7 as crate::__stddef_size_t_h::size_t).wrapping_add(
        (3 as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(
                ::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::size_t
            ),
    )
}

pub unsafe extern "C" fn gzdopen(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gz: crate::zlib_h::gzFile = ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    if !gzdopen_has_valid_descriptor(fd) {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    path = crate::stdlib::malloc(gzdopen_path_buffer_len()) as *mut ::core::ffi::c_char;
    if path.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    crate::stdlib::snprintf(
        path,
        gzdopen_path_buffer_len(),
        b"<fd:%d>\0".as_ptr() as *const ::core::ffi::c_char,
        fd,
    );
    gz = gz_open(path as *const ::core::ffi::c_void, fd, mode);
    crate::stdlib::free(path as *mut ::core::ffi::c_void);
    return gz;
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzdopen(fd, mode)
}
fn gzbuffer_normalized_want(size: ::core::ffi::c_uint) -> Option<::core::ffi::c_uint> {
    size.checked_mul(2)?;
    Some(size.max(8 as ::core::ffi::c_uint))
}

fn gzbuffer_can_set_want(mode: ::core::ffi::c_int, allocated_size: ::core::ffi::c_uint) -> bool {
    gz_is_read_or_write_mode(mode) && allocated_size == 0 as ::core::ffi::c_uint
}

fn gzbuffer_core(
    state: &mut crate::gzguts_h::gz_state,
    size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if !gzbuffer_can_set_want(state.mode, state.size) {
        return -1 as ::core::ffi::c_int;
    }
    let Some(size) = gzbuffer_normalized_want(size) else {
        return -1 as ::core::ffi::c_int;
    };
    state.want = size;
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    file: crate::zlib_h::gzFile,
    size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }

    gzbuffer_core(&mut *(file as crate::gzguts_h::gz_statep), size)
}
#[export_name = "gzrewind"]
pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    state = file as crate::gzguts_h::gz_statep;
    {
        let state_ref = &mut *state;
        if !gzrewind_request_is_valid(state_ref.mode, state_ref.err) {
            return -1 as ::core::ffi::c_int;
        }
        if !gz_lseek_succeeded(crate::stdlib::lseek64(
            state_ref.fd,
            state_ref.start as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_SET,
        )) {
            return -1 as ::core::ffi::c_int;
        }
        gz_reset_state(state_ref);
    }
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn gzseek64(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }

    let state_ptr = file as crate::gzguts_h::gz_statep;
    let request_plan = {
        let state = &mut *state_ptr;
        match gzseek_plan_request(
            state.mode,
            state.err,
            whence,
            offset,
            state.x.pos,
            state.past,
            state.skip,
        ) {
            Some(plan) => plan,
            None => return -1 as ::core::ffi::c_int as crate::stdlib::off64_t,
        }
    };
    offset = request_plan.offset;
    let fast_forward_plan = {
        let state = &mut *state_ptr;
        if request_plan.clear_pending_skip {
            state.skip = 0 as crate::stdlib::off64_t;
        }
        if let Some(plan) =
            gzseek_plan_fast_forward(state.mode, state.how, state.x.pos, offset, state.x.have)
        {
            let ret = crate::stdlib::lseek64(
                state.fd,
                plan.lseek_offset as crate::stdlib::__off64_t,
                crate::stdlib::SEEK_CUR,
            ) as crate::stdlib::off64_t;
            if !gz_lseek_succeeded(ret as crate::stdlib::__off64_t) {
                return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
            }
            gzseek_fast_forward_reset(state);
            Some(plan)
        } else {
            None
        }
    };
    if let Some(plan) = fast_forward_plan {
        gz_error(
            state_ptr,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let position = gzseek_finish_fast_forward(&mut *state_ptr, offset);
        debug_assert_eq!(position, plan.position);
        return position;
    }
    let seek_plan = {
        let state = &mut *state_ptr;
        match gzseek_plan_remaining_offset(state.mode, state.x.pos, offset) {
            Some(plan) => plan,
            None => return -1 as ::core::ffi::c_int as crate::stdlib::off64_t,
        }
    };
    offset = seek_plan.offset;
    if seek_plan.rewind && gzrewind_ffi(file) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }
    let state = &mut *state_ptr;
    if let Some(read_buffer_plan) = gzseek_read_buffer_plan_for_mode(
        state.mode,
        state.x.have,
        offset,
        ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>(),
        gz_intmax(),
    ) {
        gzseek_apply_read_buffer_plan(state, read_buffer_plan);
        offset = read_buffer_plan.remaining_offset;
    }
    state.skip = offset;
    return gz_position_after_skip(state.x.pos, offset);
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    gzseek64(file, offset, whence)
}
fn gz_legacy_offset_result(ret: crate::stdlib::off64_t) -> crate::stdlib::off_t {
    if ret == ret {
        ret
    } else {
        -1 as ::core::ffi::c_int as crate::stdlib::off_t
    }
}

#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    gz_legacy_offset_result(gzseek64(file, offset, whence))
}

fn gztell64_core(
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    gz_position_after_skip(pos, gzseek_effective_skip(past, skip))
}

fn gztell64_result(
    mode: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    if !gz_is_read_or_write_mode(mode) {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }

    gztell64_core(pos, past, skip)
}

fn gztell64_state_result(state: &crate::gzguts_h::gz_state) -> crate::stdlib::off64_t {
    gztell64_result(state.mode, state.x.pos, state.past, state.skip)
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }

    let state = unsafe { &*(file as *const crate::gzguts_h::gz_state) };
    gztell64_state_result(state)
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    if file.is_null() {
        return gz_legacy_offset_result(-1 as ::core::ffi::c_int as crate::stdlib::off64_t);
    }

    let state = unsafe { &*(file as *const crate::gzguts_h::gz_state) };
    gz_legacy_offset_result(gztell64_state_result(state))
}
fn gzoffset64_adjust_for_buffered_read(
    offset: crate::stdlib::off64_t,
    mode: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
) -> crate::stdlib::off64_t {
    if gz_is_read_mode(mode) {
        offset.wrapping_sub(avail_in as crate::stdlib::off64_t)
    } else {
        offset
    }
}

fn gzoffset64_result(
    mode: ::core::ffi::c_int,
    offset: crate::stdlib::off64_t,
    avail_in: crate::stdlib::uInt,
) -> crate::stdlib::off64_t {
    if !gz_is_read_or_write_mode(mode) || offset == -1 {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }

    gzoffset64_adjust_for_buffered_read(offset, mode, avail_in)
}

fn gzoffset64_state_result(
    state: &crate::gzguts_h::gz_state,
    offset: crate::stdlib::off64_t,
) -> crate::stdlib::off64_t {
    gzoffset64_result(state.mode, offset, state.strm.avail_in)
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as ::core::ffi::c_int as crate::stdlib::off64_t;
    }

    let state = unsafe { &*(file as *const crate::gzguts_h::gz_state) };
    let offset = unsafe {
        crate::stdlib::lseek64(
            state.fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t
    };
    gzoffset64_state_result(state, offset)
}
#[export_name = "gzoffset"]
pub unsafe extern "C" fn gzoffset_ffi(file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    if file.is_null() {
        return gz_legacy_offset_result(-1 as ::core::ffi::c_int as crate::stdlib::off64_t);
    }

    let state = unsafe { &*(file as *const crate::gzguts_h::gz_state) };
    let offset = unsafe {
        crate::stdlib::lseek64(
            state.fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t
    };
    gz_legacy_offset_result(gzoffset64_state_result(state, offset))
}
fn gzeof_result(mode: ::core::ffi::c_int, past: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if gz_is_read_mode(mode) {
        past
    } else {
        0 as ::core::ffi::c_int
    }
}

fn gzeof_state_result(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    gzeof_result(state.mode, state.past)
}

#[export_name = "gzeof"]
pub unsafe extern "C" fn gzeof_ffi(file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }

    let state = unsafe { &*(file as crate::gzguts_h::gz_statep) };
    gzeof_state_result(state)
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GzErrorMessage {
    Empty,
    OutOfMemory,
    Stored,
}

fn gzerror_core(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    has_message: bool,
) -> Option<GzErrorMessage> {
    if !gz_is_read_or_write_mode(mode) {
        return None;
    }

    if err == crate::zlib_h::Z_MEM_ERROR {
        Some(GzErrorMessage::OutOfMemory)
    } else if has_message {
        Some(GzErrorMessage::Stored)
    } else {
        Some(GzErrorMessage::Empty)
    }
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    file: crate::zlib_h::gzFile,
    errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if file.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }

    let state = &*(file as crate::gzguts_h::gz_statep);
    let Some(message) = gzerror_core(state.mode, state.err, !state.msg.is_null()) else {
        return ::core::ptr::null::<::core::ffi::c_char>();
    };

    if !errnum.is_null() {
        *errnum = state.err;
    }

    match message {
        GzErrorMessage::OutOfMemory => b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
        GzErrorMessage::Empty => b"\0".as_ptr() as *const ::core::ffi::c_char,
        GzErrorMessage::Stored => state.msg as *const ::core::ffi::c_char,
    }
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    if file.is_null() {
        return;
    }

    let state = unsafe { &mut *(file as crate::gzguts_h::gz_statep) };
    if !gzclearerr_core(state.mode, &mut state.eof, &mut state.past) {
        return;
    }
    unsafe {
        gz_error(
            state,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
}
pub unsafe extern "C" fn gz_error(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    let state = &mut *state;
    let previous_message = state.msg;
    let plan = gz_error_plan(
        !previous_message.is_null(),
        state.err,
        err,
        state.again,
        !msg.is_null(),
    );
    if !previous_message.is_null() {
        if plan.free_previous_message {
            crate::stdlib::free(previous_message as *mut ::core::ffi::c_void);
        }
        state.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if plan.clear_buffer {
        state.x.have = 0 as ::core::ffi::c_uint;
    }
    state.err = plan.err;
    if !plan.allocate_message {
        return;
    }
    let message_allocation_len = gz_error_message_allocation_len(
        crate::stdlib::strlen(state.path),
        crate::stdlib::strlen(msg),
    );
    state.msg = crate::stdlib::malloc(message_allocation_len) as *mut ::core::ffi::c_char;
    if state.msg.is_null() {
        state.err = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    crate::stdlib::snprintf(
        state.msg,
        message_allocation_len,
        b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        state.path,
        b": \0".as_ptr() as *const ::core::ffi::c_char,
        msg,
    );
}
#[export_name = "gz_error"]

pub unsafe extern "C" fn gz_error_ffi(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    gz_error(state, err, msg)
}
pub fn gz_intmax() -> ::core::ffi::c_uint {
    crate::limits_h::INT_MAX as ::core::ffi::c_uint
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}

#[cfg(test)]
mod tests {
    use super::{
        gz_clear_read_flags, gz_errno_is_retryable, gz_error_clears_buffer,
        gz_error_message_allocation_len, gz_error_needs_message_allocation, gz_error_plan,
        gz_is_read_mode, gz_is_read_or_write_mode, gz_legacy_offset_result, gz_lseek_succeeded,
        gz_open_fd_plan, gz_open_fd_succeeded, gz_open_has_required_inputs, gz_open_offset_plan,
        gz_open_path_buffer_len, gz_open_recorded_offset, gz_parse_open_mode,
        gz_position_after_skip, gz_post_open_metadata, gz_prepare_open, gz_request_len,
        gz_reset_core, gzbuffer_can_set_want, gzbuffer_normalized_want, gzclearerr_core,
        gzdopen_has_valid_descriptor, gzdopen_path_buffer_len, gzeof_result, gzerror_core,
        gzoffset64_adjust_for_buffered_read, gzoffset64_result, gzrewind_request_is_valid,
        gzseek_adjust_offset, gzseek_can_fast_forward, gzseek_clears_pending_skip,
        gzseek_effective_skip, gzseek_error_allows_positioning, gzseek_fast_forward_lseek_offset,
        gzseek_fast_forward_reset, gzseek_finish_fast_forward, gzseek_plan_fast_forward,
        gzseek_plan_read_buffer_consumption, gzseek_plan_remaining_offset, gzseek_plan_request,
        gzseek_read_buffer_consumed, gzseek_read_buffer_plan_for_mode,
        gzseek_read_buffer_uses_requested_offset, gzseek_request_is_valid, gzseek_uses_read_buffer,
        gztell64_core, gztell64_result, GzErrorMessage, GzErrorPlan, GzOpenFdPlan,
        GzOpenOffsetPlan, GzResetFields, GzSeekFastForwardPlan, GzSeekOffsetPlan,
        GzSeekReadBufferPlan, GzSeekRequestPlan,
    };

    #[test]
    fn clearing_read_flags_resets_both_values() {
        let mut eof = 1;
        let mut past = 1;
        gz_clear_read_flags(&mut eof, &mut past);
        assert_eq!((eof, past), (0, 0));
    }

    #[test]
    fn gz_open_requires_both_path_and_mode() {
        assert!(gz_open_has_required_inputs(true, true));
        assert!(!gz_open_has_required_inputs(false, true));
        assert!(!gz_open_has_required_inputs(true, false));
        assert!(!gz_open_has_required_inputs(false, false));
    }

    #[test]
    fn gz_request_len_handles_zero_and_representable_products() {
        assert_eq!(gz_request_len(0, 5), Some(0));
        assert_eq!(gz_request_len(5, 0), Some(0));
        assert_eq!(
            gz_request_len(crate::stdlib::z_size_t::MAX, 1),
            Some(crate::stdlib::z_size_t::MAX)
        );
        assert_eq!(gz_request_len(4, 7), Some(28));
    }

    #[test]
    fn gz_request_len_rejects_overflow() {
        assert_eq!(gz_request_len(crate::stdlib::z_size_t::MAX, 2), None);
    }

    #[test]
    fn gz_errno_is_retryable_accepts_only_would_block_errors() {
        assert!(gz_errno_is_retryable(crate::stdlib::EAGAIN));
        assert!(gz_errno_is_retryable(crate::stdlib::EWOULDBLOCK));
        assert!(!gz_errno_is_retryable(0));
        assert!(!gz_errno_is_retryable(::core::ffi::c_int::MIN));
    }

    #[test]
    fn gz_lseek_succeeded_rejects_only_the_lseek_failure_sentinel() {
        assert!(!gz_lseek_succeeded(-1));
        assert!(gz_lseek_succeeded(0));
        assert!(gz_lseek_succeeded(17));
    }

    #[test]
    fn gzseek_fast_forward_completion_resets_and_updates_cursor_fields() {
        let mut state = crate::gzguts_h::gz_state {
            x: crate::zlib_h::gzFile_s {
                have: 7,
                next: ::core::ptr::null_mut(),
                pos: 101,
            },
            mode: crate::gzguts_h::GZ_READ,
            fd: 0,
            path: ::core::ptr::null_mut(),
            size: 0,
            want: 0,
            in_0: ::core::ptr::null_mut(),
            out: ::core::ptr::null_mut(),
            direct: 0,
            junk: 0,
            how: crate::gzguts_h::COPY,
            again: 23,
            start: 0,
            eof: 1,
            past: 1,
            level: 0,
            strategy: 0,
            reset: 0,
            skip: 29,
            err: crate::zlib_h::Z_BUF_ERROR,
            msg: ::core::ptr::null_mut(),
            strm: crate::zlib_h::z_stream_s {
                next_in: ::core::ptr::null_mut(),
                avail_in: 31,
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

        gzseek_fast_forward_reset(&mut state);
        let position = gzseek_finish_fast_forward(&mut state, 11);

        assert_eq!(state.x.have, 0);
        assert_eq!((state.eof, state.past, state.skip), (0, 0, 0));
        assert_eq!(position, 112);
        assert_eq!(state.x.pos, 112);
        assert_eq!(state.strm.avail_in, 0);
        assert_eq!(state.err, crate::zlib_h::Z_BUF_ERROR);
        assert_eq!(state.again, 23);
    }

    #[test]
    fn gz_reset_core_resets_read_state_without_touching_write_reset() {
        let mut fields = GzResetFields {
            mode: crate::gzguts_h::GZ_READ,
            have: 1,
            eof: 1,
            past: 1,
            how: crate::gzguts_h::COPY,
            junk: 0,
            reset: 1,
            again: 1,
            skip: 1,
            pos: 1,
            avail_in: 1,
        };

        gz_reset_core(&mut fields);

        assert_eq!(fields.have, 0);
        assert_eq!((fields.eof, fields.past), (0, 0));
        assert_eq!((fields.how, fields.junk), (crate::gzguts_h::LOOK, -1));
        assert_eq!(fields.reset, 1);
        assert_eq!((fields.again, fields.skip), (0, 0));
        assert_eq!((fields.pos, fields.avail_in), (0, 0));
    }

    #[test]
    fn gz_reset_core_resets_write_state_without_touching_read_fields() {
        let mut fields = GzResetFields {
            mode: crate::gzguts_h::GZ_WRITE,
            have: 1,
            eof: 1,
            past: 1,
            how: crate::gzguts_h::COPY,
            junk: 0,
            reset: 1,
            again: 1,
            skip: 1,
            pos: 1,
            avail_in: 1,
        };

        gz_reset_core(&mut fields);

        assert_eq!(fields.have, 0);
        assert_eq!((fields.eof, fields.past), (1, 1));
        assert_eq!((fields.how, fields.junk), (crate::gzguts_h::COPY, 0));
        assert_eq!(fields.reset, 0);
        assert_eq!((fields.again, fields.skip), (0, 0));
        assert_eq!((fields.pos, fields.avail_in), (0, 0));
    }

    #[test]
    fn gzclearerr_core_clears_read_flags_for_active_read_mode_only() {
        let mut eof = 1;
        let mut past = 1;
        assert!(gzclearerr_core(
            crate::gzguts_h::GZ_READ,
            &mut eof,
            &mut past
        ));
        assert_eq!((eof, past), (0, 0));

        eof = 1;
        past = 1;
        assert!(gzclearerr_core(
            crate::gzguts_h::GZ_WRITE,
            &mut eof,
            &mut past
        ));
        assert_eq!((eof, past), (1, 1));

        assert!(!gzclearerr_core(
            crate::gzguts_h::GZ_NONE,
            &mut eof,
            &mut past
        ));
    }

    #[test]
    fn read_or_write_mode_validation_accepts_active_modes_only() {
        assert!(gz_is_read_or_write_mode(crate::gzguts_h::GZ_READ));
        assert!(gz_is_read_or_write_mode(crate::gzguts_h::GZ_WRITE));
        assert!(!gz_is_read_or_write_mode(crate::gzguts_h::GZ_NONE));
        assert!(!gz_is_read_or_write_mode(crate::gzguts_h::GZ_APPEND));
    }

    #[test]
    fn read_mode_validation_accepts_only_read_mode() {
        assert!(gz_is_read_mode(crate::gzguts_h::GZ_READ));
        assert!(!gz_is_read_mode(crate::gzguts_h::GZ_WRITE));
        assert!(!gz_is_read_mode(crate::gzguts_h::GZ_APPEND));
        assert!(!gz_is_read_mode(crate::gzguts_h::GZ_NONE));
    }

    #[test]
    fn gzbuffer_normalizes_small_requested_sizes() {
        assert_eq!(gzbuffer_normalized_want(0), Some(8));
        assert_eq!(gzbuffer_normalized_want(7), Some(8));
    }

    #[test]
    fn gzbuffer_preserves_valid_requested_sizes() {
        assert_eq!(gzbuffer_normalized_want(8), Some(8));
        assert_eq!(gzbuffer_normalized_want(9), Some(9));
    }

    #[test]
    fn gzbuffer_admission_requires_active_unallocated_state() {
        assert!(gzbuffer_can_set_want(crate::gzguts_h::GZ_READ, 0));
        assert!(gzbuffer_can_set_want(crate::gzguts_h::GZ_WRITE, 0));
        assert!(!gzbuffer_can_set_want(crate::gzguts_h::GZ_NONE, 0));
        assert!(!gzbuffer_can_set_want(123, 0));
        assert!(!gzbuffer_can_set_want(crate::gzguts_h::GZ_READ, 1));
        assert!(!gzbuffer_can_set_want(
            crate::gzguts_h::GZ_WRITE,
            ::core::ffi::c_uint::MAX
        ));
    }

    #[test]
    fn gzbuffer_rejects_sizes_that_overflow_when_doubled() {
        assert_eq!(gzbuffer_normalized_want(::core::ffi::c_uint::MAX), None);
    }

    #[test]
    fn gzbuffer_normalization_checks_doubling_boundary() {
        let largest_doublable = ::core::ffi::c_uint::MAX / 2;

        assert_eq!(
            gzbuffer_normalized_want(largest_doublable),
            Some(largest_doublable)
        );
        assert_eq!(gzbuffer_normalized_want(largest_doublable + 1), None);
    }

    #[test]
    fn gz_error_clears_buffer_only_for_nonrecoverable_errors_without_retry() {
        assert!(gz_error_clears_buffer(crate::zlib_h::Z_MEM_ERROR, 0));
        assert!(!gz_error_clears_buffer(crate::zlib_h::Z_OK, 0));
        assert!(!gz_error_clears_buffer(crate::zlib_h::Z_BUF_ERROR, 0));
        assert!(!gz_error_clears_buffer(crate::zlib_h::Z_MEM_ERROR, 1));
    }

    #[test]
    fn gz_error_allocates_messages_only_for_non_memory_errors_with_text() {
        assert!(gz_error_needs_message_allocation(
            crate::zlib_h::Z_DATA_ERROR,
            true
        ));
        assert!(!gz_error_needs_message_allocation(
            crate::zlib_h::Z_DATA_ERROR,
            false
        ));
        assert!(!gz_error_needs_message_allocation(
            crate::zlib_h::Z_MEM_ERROR,
            true
        ));
    }

    #[test]
    fn gz_error_message_allocation_length_preserves_wrapping_arithmetic() {
        assert_eq!(gz_error_message_allocation_len(4, 6), 13);
        assert_eq!(
            gz_error_message_allocation_len(crate::__stddef_size_t_h::size_t::MAX, 1),
            3
        );
    }

    #[test]
    fn gz_error_plan_frees_and_replaces_regular_messages() {
        assert_eq!(
            gz_error_plan(
                true,
                crate::zlib_h::Z_DATA_ERROR,
                crate::zlib_h::Z_DATA_ERROR,
                0,
                true,
            ),
            GzErrorPlan {
                free_previous_message: true,
                clear_buffer: true,
                err: crate::zlib_h::Z_DATA_ERROR,
                allocate_message: true,
            }
        );
    }

    #[test]
    fn gz_error_plan_retains_out_of_memory_messages_without_allocation() {
        assert_eq!(
            gz_error_plan(
                true,
                crate::zlib_h::Z_MEM_ERROR,
                crate::zlib_h::Z_MEM_ERROR,
                1,
                true,
            ),
            GzErrorPlan {
                free_previous_message: false,
                clear_buffer: false,
                err: crate::zlib_h::Z_MEM_ERROR,
                allocate_message: false,
            }
        );
    }

    #[test]
    fn gzerror_core_rejects_invalid_modes() {
        assert_eq!(gzerror_core(0, 0, false), None);
    }

    #[test]
    fn gzerror_core_prioritizes_out_of_memory_message() {
        assert_eq!(
            gzerror_core(crate::gzguts_h::GZ_READ, crate::zlib_h::Z_MEM_ERROR, true),
            Some(GzErrorMessage::OutOfMemory)
        );
    }

    #[test]
    fn gzerror_core_selects_empty_or_stored_message() {
        assert_eq!(
            gzerror_core(crate::gzguts_h::GZ_WRITE, crate::zlib_h::Z_OK, false),
            Some(GzErrorMessage::Empty)
        );
        assert_eq!(
            gzerror_core(crate::gzguts_h::GZ_WRITE, crate::zlib_h::Z_OK, true),
            Some(GzErrorMessage::Stored)
        );
    }

    #[test]
    fn gztell64_core_includes_pending_skip_before_eof() {
        assert_eq!(gztell64_core(42, 0, 7), 49);
    }

    #[test]
    fn gztell64_core_ignores_skip_after_eof() {
        assert_eq!(gztell64_core(42, 1, 7), 42);
    }

    #[test]
    fn position_after_skip_preserves_signed_cursor_arithmetic() {
        assert_eq!(gz_position_after_skip(42, 7), 49);
        assert_eq!(gz_position_after_skip(42, -7), 35);
    }

    #[test]
    fn gztell64_result_rejects_inactive_modes() {
        assert_eq!(gztell64_result(crate::gzguts_h::GZ_NONE, 42, 0, 7), -1);
        assert_eq!(gztell64_result(crate::gzguts_h::GZ_APPEND, 42, 0, 7), -1);
    }

    #[test]
    fn gztell64_result_preserves_active_mode_position_rules() {
        assert_eq!(gztell64_result(crate::gzguts_h::GZ_READ, 42, 0, 7), 49);
        assert_eq!(gztell64_result(crate::gzguts_h::GZ_WRITE, 42, 1, 7), 42);
    }

    #[test]
    fn legacy_offset_result_preserves_signed_offsets() {
        assert_eq!(gz_legacy_offset_result(27), 27 as crate::stdlib::off_t);
        assert_eq!(gz_legacy_offset_result(-1), -1 as crate::stdlib::off_t);
    }

    #[test]
    fn gzoffset64_adjusts_for_unconsumed_read_input() {
        assert_eq!(
            gzoffset64_adjust_for_buffered_read(42, crate::gzguts_h::GZ_READ, 7),
            35
        );
    }

    #[test]
    fn gzoffset64_buffered_read_adjustment_wraps_at_signed_minimum() {
        assert_eq!(
            gzoffset64_adjust_for_buffered_read(
                crate::stdlib::off64_t::MIN,
                crate::gzguts_h::GZ_READ,
                1,
            ),
            crate::stdlib::off64_t::MAX,
        );
    }

    #[test]
    fn gzoffset64_preserves_offset_outside_read_mode() {
        assert_eq!(
            gzoffset64_adjust_for_buffered_read(42, crate::gzguts_h::GZ_WRITE, 7),
            42
        );
    }

    #[test]
    fn gzoffset64_result_rejects_inactive_modes_and_seek_errors() {
        assert_eq!(gzoffset64_result(crate::gzguts_h::GZ_NONE, 42, 7), -1);
        assert_eq!(gzoffset64_result(crate::gzguts_h::GZ_READ, -1, 7), -1);
    }

    #[test]
    fn gzoffset64_result_adjusts_active_read_offsets() {
        assert_eq!(gzoffset64_result(crate::gzguts_h::GZ_READ, 42, 7), 35);
        assert_eq!(gzoffset64_result(crate::gzguts_h::GZ_WRITE, 42, 7), 42);
    }

    #[test]
    fn gzseek_consumes_all_buffered_input_with_sufficient_offset() {
        assert_eq!(gzseek_read_buffer_consumed(7, 7, false, 0), 7);
        assert_eq!(gzseek_read_buffer_consumed(7, 9, false, 0), 7);
    }

    #[test]
    fn gzseek_consumes_only_requested_buffered_input() {
        assert_eq!(gzseek_read_buffer_consumed(7, 3, false, 0), 3);
        assert_eq!(gzseek_read_buffer_consumed(7, 0, false, 0), 0);
    }

    #[test]
    fn gzseek_read_buffer_limit_detects_offset_and_matching_width_boundaries() {
        assert!(gzseek_read_buffer_uses_requested_offset(7, 6, false, 0));
        assert!(!gzseek_read_buffer_uses_requested_offset(7, 7, false, 0));
        assert!(gzseek_read_buffer_uses_requested_offset(9, 20, true, 8));
        assert!(!gzseek_read_buffer_uses_requested_offset(8, 20, true, 8));
    }

    #[test]
    fn gzseek_preserves_matching_width_large_buffer_rule() {
        assert_eq!(gzseek_read_buffer_consumed(9, 20, true, 8), 20);
        assert_eq!(gzseek_read_buffer_consumed(8, 20, true, 8), 8);
    }

    #[test]
    fn gzseek_read_buffer_plan_preserves_consumed_bytes_and_remaining_offset() {
        assert_eq!(
            gzseek_plan_read_buffer_consumption(7, 3, false, 0),
            GzSeekReadBufferPlan {
                consumed: 3,
                remaining_offset: 0,
            }
        );
        assert_eq!(
            gzseek_plan_read_buffer_consumption(7, 9, false, 0),
            GzSeekReadBufferPlan {
                consumed: 7,
                remaining_offset: 2,
            }
        );
    }

    #[test]
    fn gzseek_read_buffer_plan_preserves_matching_width_large_buffer_quirk() {
        assert_eq!(
            gzseek_plan_read_buffer_consumption(9, 20, true, 8),
            GzSeekReadBufferPlan {
                consumed: 20,
                remaining_offset: 0,
            }
        );
    }

    #[test]
    fn gzseek_read_buffer_plan_for_mode_plans_read_cursor_consumption() {
        assert_eq!(
            gzseek_read_buffer_plan_for_mode(crate::gzguts_h::GZ_READ, 7, 3, false, 0),
            Some(GzSeekReadBufferPlan {
                consumed: 3,
                remaining_offset: 0,
            })
        );
    }

    #[test]
    fn gzseek_read_buffer_plan_for_mode_skips_non_read_modes() {
        for mode in [
            crate::gzguts_h::GZ_WRITE,
            crate::gzguts_h::GZ_NONE,
            crate::gzguts_h::GZ_APPEND,
        ] {
            assert_eq!(gzseek_read_buffer_plan_for_mode(mode, 7, 3, false, 0), None);
        }
    }

    #[test]
    fn gzseek_uses_read_buffer_only_for_read_mode() {
        assert!(gzseek_uses_read_buffer(crate::gzguts_h::GZ_READ));
        assert!(!gzseek_uses_read_buffer(crate::gzguts_h::GZ_WRITE));
        assert!(!gzseek_uses_read_buffer(crate::gzguts_h::GZ_NONE));
        assert!(!gzseek_uses_read_buffer(crate::gzguts_h::GZ_APPEND));
    }

    #[test]
    fn gzeof_result_returns_past_only_for_read_mode() {
        assert_eq!(gzeof_result(crate::gzguts_h::GZ_READ, 1), 1);
        assert_eq!(gzeof_result(crate::gzguts_h::GZ_READ, 0), 0);
        assert_eq!(gzeof_result(crate::gzguts_h::GZ_WRITE, 1), 0);
        assert_eq!(gzeof_result(crate::gzguts_h::GZ_NONE, 1), 0);
        assert_eq!(gzeof_result(crate::gzguts_h::GZ_APPEND, 1), 0);
    }

    #[test]
    fn gzseek_read_buffer_plan_updates_buffered_cursor_state() {
        let mut state = crate::gzguts_h::gz_state {
            x: crate::zlib_h::gzFile_s {
                have: 7,
                next: ::core::ptr::null_mut(),
                pos: 101,
            },
            mode: crate::gzguts_h::GZ_READ,
            fd: 0,
            path: ::core::ptr::null_mut(),
            size: 0,
            want: 0,
            in_0: ::core::ptr::null_mut(),
            out: ::core::ptr::null_mut(),
            direct: 0,
            junk: 0,
            how: crate::gzguts_h::COPY,
            again: 0,
            start: 0,
            eof: 0,
            past: 0,
            level: 0,
            strategy: 0,
            reset: 0,
            skip: 0,
            err: crate::zlib_h::Z_OK,
            msg: ::core::ptr::null_mut(),
            strm: crate::zlib_h::z_stream_s {
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

        super::gzseek_apply_read_buffer_plan(
            &mut state,
            GzSeekReadBufferPlan {
                consumed: 3,
                remaining_offset: 4,
            },
        );

        assert_eq!(state.x.have, 4);
        assert_eq!(state.x.next, ::core::ptr::null_mut::<u8>().wrapping_add(3));
        assert_eq!(state.x.pos, 104);
    }

    #[test]
    fn gzseek_read_buffer_plan_wraps_position_and_updates_buffered_cursor_state() {
        let mut state = crate::gzguts_h::gz_state {
            x: crate::zlib_h::gzFile_s {
                have: 7,
                next: ::core::ptr::null_mut(),
                pos: crate::stdlib::off64_t::MAX,
            },
            mode: crate::gzguts_h::GZ_READ,
            fd: 0,
            path: ::core::ptr::null_mut(),
            size: 0,
            want: 0,
            in_0: ::core::ptr::null_mut(),
            out: ::core::ptr::null_mut(),
            direct: 0,
            junk: 0,
            how: crate::gzguts_h::COPY,
            again: 0,
            start: 0,
            eof: 0,
            past: 0,
            level: 0,
            strategy: 0,
            reset: 0,
            skip: 0,
            err: crate::zlib_h::Z_OK,
            msg: ::core::ptr::null_mut(),
            strm: crate::zlib_h::z_stream_s {
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

        super::gzseek_apply_read_buffer_plan(
            &mut state,
            GzSeekReadBufferPlan {
                consumed: 3,
                remaining_offset: 4,
            },
        );

        assert_eq!(state.x.have, 4);
        assert_eq!(state.x.next, ::core::ptr::null_mut::<u8>().wrapping_add(3));
        assert_eq!(state.x.pos, crate::stdlib::off64_t::MIN.wrapping_add(2));
    }

    #[test]
    fn gzseek_request_validation_requires_active_mode_recoverable_error_and_supported_whence() {
        assert!(gzseek_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_OK,
            crate::stdlib::SEEK_SET
        ));
        assert!(gzseek_request_is_valid(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_BUF_ERROR,
            crate::stdlib::SEEK_CUR
        ));
        assert!(!gzseek_request_is_valid(
            crate::gzguts_h::GZ_NONE,
            crate::zlib_h::Z_OK,
            crate::stdlib::SEEK_SET
        ));
        assert!(!gzseek_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_MEM_ERROR,
            crate::stdlib::SEEK_SET
        ));
        assert!(!gzseek_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_OK,
            crate::stdlib::SEEK_END
        ));
    }

    #[test]
    fn gzseek_request_plan_preserves_validation_skip_and_offset_rules() {
        assert_eq!(
            gzseek_plan_request(
                crate::gzguts_h::GZ_READ,
                crate::zlib_h::Z_OK,
                crate::stdlib::SEEK_SET,
                30,
                12,
                0,
                7,
            ),
            Some(GzSeekRequestPlan {
                offset: 18,
                clear_pending_skip: false,
            })
        );
        assert_eq!(
            gzseek_plan_request(
                crate::gzguts_h::GZ_WRITE,
                crate::zlib_h::Z_BUF_ERROR,
                crate::stdlib::SEEK_CUR,
                30,
                12,
                0,
                7,
            ),
            Some(GzSeekRequestPlan {
                offset: 37,
                clear_pending_skip: true,
            })
        );
        assert_eq!(
            gzseek_plan_request(
                crate::gzguts_h::GZ_WRITE,
                crate::zlib_h::Z_BUF_ERROR,
                crate::stdlib::SEEK_CUR,
                30,
                12,
                1,
                7,
            ),
            Some(GzSeekRequestPlan {
                offset: 30,
                clear_pending_skip: true,
            })
        );
        assert_eq!(
            gzseek_plan_request(
                crate::gzguts_h::GZ_NONE,
                crate::zlib_h::Z_OK,
                crate::stdlib::SEEK_SET,
                0,
                0,
                0,
                0,
            ),
            None
        );
        assert_eq!(
            gzseek_plan_request(
                crate::gzguts_h::GZ_READ,
                crate::zlib_h::Z_MEM_ERROR,
                crate::stdlib::SEEK_SET,
                0,
                0,
                0,
                0,
            ),
            None
        );
        assert_eq!(
            gzseek_plan_request(
                crate::gzguts_h::GZ_READ,
                crate::zlib_h::Z_OK,
                crate::stdlib::SEEK_END,
                0,
                0,
                0,
                0,
            ),
            None
        );
    }

    #[test]
    fn gzseek_positioning_errors_match_rewind_and_seek_requirements() {
        assert!(gzseek_error_allows_positioning(crate::zlib_h::Z_OK));
        assert!(gzseek_error_allows_positioning(crate::zlib_h::Z_BUF_ERROR));
        assert!(!gzseek_error_allows_positioning(crate::zlib_h::Z_MEM_ERROR));
    }

    #[test]
    fn gzrewind_request_validation_requires_read_mode_and_recoverable_error() {
        assert!(gzrewind_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_OK
        ));
        assert!(gzrewind_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_BUF_ERROR
        ));
        assert!(!gzrewind_request_is_valid(
            crate::gzguts_h::GZ_WRITE,
            crate::zlib_h::Z_OK
        ));
        assert!(!gzrewind_request_is_valid(
            crate::gzguts_h::GZ_READ,
            crate::zlib_h::Z_MEM_ERROR
        ));
    }

    #[test]
    fn gzseek_remaining_offset_plan_keeps_nonnegative_offsets_in_place() {
        assert_eq!(
            gzseek_plan_remaining_offset(crate::gzguts_h::GZ_WRITE, 12, 5),
            Some(GzSeekOffsetPlan {
                offset: 5,
                rewind: false,
            })
        );
    }

    #[test]
    fn gzseek_remaining_offset_plan_rewinds_reads_for_valid_negative_targets() {
        assert_eq!(
            gzseek_plan_remaining_offset(crate::gzguts_h::GZ_READ, 12, -5),
            Some(GzSeekOffsetPlan {
                offset: 7,
                rewind: true,
            })
        );
    }

    #[test]
    fn gzseek_remaining_offset_plan_rejects_invalid_negative_targets() {
        assert_eq!(
            gzseek_plan_remaining_offset(crate::gzguts_h::GZ_WRITE, 12, -1),
            None
        );
        assert_eq!(
            gzseek_plan_remaining_offset(crate::gzguts_h::GZ_READ, 12, -13),
            None
        );
    }

    #[test]
    fn gzseek_adjusts_set_and_current_offsets_using_pending_skip_only_before_eof() {
        assert_eq!(
            gzseek_adjust_offset(30, crate::stdlib::SEEK_SET, 12, 0, 7),
            18
        );
        assert_eq!(
            gzseek_adjust_offset(30, crate::stdlib::SEEK_CUR, 12, 0, 7),
            37
        );
        assert_eq!(
            gzseek_adjust_offset(30, crate::stdlib::SEEK_CUR, 12, 1, 7),
            30
        );
    }

    #[test]
    fn gzseek_effective_skip_preserves_pending_skip_before_past_end() {
        assert_eq!(gzseek_effective_skip(0, 7), 7);
    }

    #[test]
    fn gzseek_effective_skip_clears_pending_skip_after_past_end() {
        assert_eq!(gzseek_effective_skip(1, 7), 0);
    }

    #[test]
    fn gzseek_clears_pending_skip_only_for_current_relative_seeks() {
        assert!(gzseek_clears_pending_skip(crate::stdlib::SEEK_CUR));
        assert!(!gzseek_clears_pending_skip(crate::stdlib::SEEK_SET));
        assert!(!gzseek_clears_pending_skip(crate::stdlib::SEEK_END));
    }

    #[test]
    fn gzseek_fast_forward_requires_read_copy_mode_and_nonnegative_target() {
        assert!(gzseek_can_fast_forward(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::COPY,
            12,
            -12
        ));
        assert!(!gzseek_can_fast_forward(
            crate::gzguts_h::GZ_WRITE,
            crate::gzguts_h::COPY,
            12,
            0
        ));
        assert!(!gzseek_can_fast_forward(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::LOOK,
            12,
            0
        ));
        assert!(!gzseek_can_fast_forward(
            crate::gzguts_h::GZ_READ,
            crate::gzguts_h::COPY,
            12,
            -13
        ));
    }

    #[test]
    fn gzseek_fast_forward_lseek_offset_accounts_for_buffered_input() {
        assert_eq!(gzseek_fast_forward_lseek_offset(19, 7), 12);
        assert_eq!(gzseek_fast_forward_lseek_offset(-3, 7), -10);
        assert_eq!(gzseek_fast_forward_lseek_offset(0, 0), 0);
    }

    #[test]
    fn gzseek_fast_forward_plan_preserves_seek_and_position_calculations() {
        assert_eq!(
            gzseek_plan_fast_forward(crate::gzguts_h::GZ_READ, crate::gzguts_h::COPY, 12, 19, 7),
            Some(GzSeekFastForwardPlan {
                lseek_offset: 12,
                position: 31,
            })
        );
        assert_eq!(
            gzseek_plan_fast_forward(crate::gzguts_h::GZ_READ, crate::gzguts_h::LOOK, 12, 19, 7),
            None
        );
        assert_eq!(
            gzseek_plan_fast_forward(crate::gzguts_h::GZ_READ, crate::gzguts_h::COPY, 12, -13, 7),
            None
        );
    }

    #[test]
    fn parsing_open_mode_collects_mode_flags_and_compression_options() {
        let options = gz_parse_open_mode(b"w9exfNT").unwrap();

        assert_eq!(options.mode, crate::gzguts_h::GZ_WRITE);
        assert_eq!(options.level, 9);
        assert_eq!(options.strategy, crate::zlib_h::Z_FILTERED);
        assert_eq!(options.direct, 1);
        assert_eq!(
            options.oflag,
            crate::stdlib::O_CLOEXEC | crate::stdlib::O_NONBLOCK
        );
        assert_eq!(options.exclusive, 1);
    }

    #[test]
    fn parsing_open_mode_rejects_update_mode() {
        assert_eq!(gz_parse_open_mode(b"rb+"), None);
    }

    #[test]
    fn parsing_open_mode_uses_the_last_conflicting_option() {
        let options = gz_parse_open_mode(b"rawhRFGT2").unwrap();

        assert_eq!(options.mode, crate::gzguts_h::GZ_WRITE);
        assert_eq!(options.level, 2);
        assert_eq!(options.strategy, crate::zlib_h::Z_FIXED);
        assert_eq!(options.direct, 1);
    }

    #[test]
    fn preparing_read_open_normalizes_default_direct_mode_and_flags() {
        let plan = gz_prepare_open(gz_parse_open_mode(b"r").unwrap()).unwrap();

        assert_eq!(plan.mode, crate::gzguts_h::GZ_READ);
        assert_eq!(plan.direct, 1);
        assert_eq!(
            plan.oflag,
            crate::stdlib::O_LARGEFILE | crate::stdlib::O_RDONLY
        );
    }

    #[test]
    fn preparing_write_open_combines_descriptor_flags() {
        let plan = gz_prepare_open(gz_parse_open_mode(b"axNe").unwrap()).unwrap();

        assert_eq!(plan.mode, crate::gzguts_h::GZ_APPEND);
        assert_eq!(plan.direct, 0);
        assert_eq!(
            plan.oflag,
            crate::stdlib::O_LARGEFILE
                | crate::stdlib::O_NONBLOCK
                | crate::stdlib::O_CLOEXEC
                | crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | crate::stdlib::O_EXCL
                | crate::stdlib::O_APPEND
        );
    }

    #[test]
    fn preparing_open_rejects_missing_mode_and_invalid_direct_modes() {
        assert_eq!(gz_prepare_open(gz_parse_open_mode(b"9").unwrap()), None);
        assert_eq!(gz_prepare_open(gz_parse_open_mode(b"rT").unwrap()), None);
        assert_eq!(gz_prepare_open(gz_parse_open_mode(b"wG").unwrap()), None);
    }

    #[test]
    fn post_open_metadata_converts_append_without_setting_read_start() {
        assert_eq!(
            gz_post_open_metadata(crate::gzguts_h::GZ_APPEND, 91),
            (crate::gzguts_h::GZ_WRITE, None)
        );
    }

    #[test]
    fn post_open_metadata_uses_zero_when_read_offset_is_unavailable() {
        assert_eq!(
            gz_post_open_metadata(crate::gzguts_h::GZ_READ, -1),
            (crate::gzguts_h::GZ_READ, Some(0))
        );
        assert_eq!(
            gz_post_open_metadata(crate::gzguts_h::GZ_READ, 19),
            (crate::gzguts_h::GZ_READ, Some(19))
        );
    }
    #[test]
    fn gz_open_offset_plan_seeks_to_end_without_recording_append_offsets() {
        assert_eq!(
            gz_open_offset_plan(crate::gzguts_h::GZ_APPEND),
            Some(GzOpenOffsetPlan {
                whence: crate::stdlib::SEEK_END,
                record_offset: false,
            })
        );
    }

    #[test]
    fn gz_open_offset_plan_records_current_read_offset() {
        assert_eq!(
            gz_open_offset_plan(crate::gzguts_h::GZ_READ),
            Some(GzOpenOffsetPlan {
                whence: crate::stdlib::SEEK_CUR,
                record_offset: true,
            })
        );
    }

    #[test]
    fn gz_open_offset_plan_skips_seeking_for_write_mode() {
        assert_eq!(gz_open_offset_plan(crate::gzguts_h::GZ_WRITE), None);
    }

    #[test]
    fn gz_open_fd_plan_opens_a_path_only_for_the_missing_descriptor_sentinel() {
        assert_eq!(gz_open_fd_plan(-1, 0), GzOpenFdPlan::OpenPath);
        assert_ne!(gz_open_fd_plan(-2, 0), GzOpenFdPlan::OpenPath);
        assert_ne!(gz_open_fd_plan(0, 0), GzOpenFdPlan::OpenPath);
        assert_ne!(gz_open_fd_plan(17, 0), GzOpenFdPlan::OpenPath);
    }

    #[test]
    fn gz_open_fd_succeeded_rejects_only_the_open_failure_sentinel() {
        assert!(!gz_open_fd_succeeded(-1));
        assert!(gz_open_fd_succeeded(-2));
        assert!(gz_open_fd_succeeded(0));
        assert!(gz_open_fd_succeeded(17));
    }

    #[test]
    fn gz_open_fd_plan_applies_requested_flags_only_to_adopted_descriptors() {
        assert_eq!(
            gz_open_fd_plan(17, crate::stdlib::O_NONBLOCK | crate::stdlib::O_CLOEXEC),
            GzOpenFdPlan::AdoptFd {
                set_nonblocking: true,
                set_close_on_exec: true,
            }
        );
        assert_eq!(
            gz_open_fd_plan(17, crate::stdlib::O_NONBLOCK),
            GzOpenFdPlan::AdoptFd {
                set_nonblocking: true,
                set_close_on_exec: false,
            }
        );
        assert_eq!(
            gz_open_fd_plan(17, crate::stdlib::O_CLOEXEC),
            GzOpenFdPlan::AdoptFd {
                set_nonblocking: false,
                set_close_on_exec: true,
            }
        );
        assert_eq!(
            gz_open_fd_plan(17, 0),
            GzOpenFdPlan::AdoptFd {
                set_nonblocking: false,
                set_close_on_exec: false,
            }
        );
    }

    #[test]
    fn gz_open_path_buffer_len_includes_terminator_for_empty_path() {
        assert_eq!(gz_open_path_buffer_len(0), 1);
    }

    #[test]
    fn gz_open_path_buffer_len_includes_terminator_for_normal_path() {
        assert_eq!(gz_open_path_buffer_len(42), 43);
    }

    #[test]
    fn gz_open_path_buffer_len_wraps_at_z_size_t_max() {
        assert_eq!(gz_open_path_buffer_len(crate::stdlib::z_size_t::MAX), 0);
    }

    #[test]
    fn gzdopen_rejects_only_the_missing_descriptor_sentinel() {
        assert!(!gzdopen_has_valid_descriptor(-1));
        assert!(gzdopen_has_valid_descriptor(-2));
        assert!(gzdopen_has_valid_descriptor(0));
        assert!(gzdopen_has_valid_descriptor(17));
    }

    #[test]
    fn gzdopen_path_buffer_len_matches_fd_identifier_bound() {
        assert_eq!(
            gzdopen_path_buffer_len(),
            7 + 3 * ::core::mem::size_of::<::core::ffi::c_int>()
        );
    }

    #[test]
    fn gz_open_recorded_offset_keeps_only_requested_offsets() {
        assert_eq!(gz_open_recorded_offset(true, 37), 37);
        assert_eq!(gz_open_recorded_offset(false, 37), 0);
    }
}
