pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_clamped_uint;
pub use crate::src::gzlib::gz_errno_is_retryable;
pub use crate::src::gzlib::gz_error;
pub(crate) use crate::src::gzlib::gz_error_with_os_error;
pub(crate) use crate::src::gzlib::gz_file_completed_items;
pub(crate) use crate::src::gzlib::gz_file_request_len;
pub use crate::src::gzlib::gz_io_chunk_len;
pub use crate::src::gzlib::gz_io_chunk_limit;
pub(crate) use crate::src::gzlib::gz_remove_owned_buffers;
pub(crate) use crate::src::gzlib::gz_remove_owned_file_fd;
pub(crate) use crate::src::gzlib::gz_store_owned_buffers;
pub use crate::src::gzlib::gz_uInt_fits_int;
pub(crate) use crate::src::gzlib::gz_with_file_mut;
pub(crate) use crate::src::gzlib::gz_with_input_buffer_mut;
pub(crate) use crate::src::gzlib::gz_with_output_buffer_mut;
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

fn gz_alloc_byte_vec(len: usize) -> Option<Vec<crate::stdlib::Bytef>> {
    let mut buf = Vec::new();
    buf.try_reserve_exact(len).ok()?;
    buf.resize(len, 0);
    Some(buf)
}

macro_rules! gz_init {
    ($state:expr) => {{
        let state: &mut crate::gzguts_h::gz_state = &mut *$state;
        'gz_init_result: {
            let input_len = (state.want << 1 as ::core::ffi::c_int) as usize;
            let Some(mut input_buf) = gz_alloc_byte_vec(input_len) else {
                crate::src::gzlib::gz_error_static(
                    state,
                    crate::zlib_h::Z_MEM_ERROR,
                    b"out of memory\0",
                );
                break 'gz_init_result -1 as ::core::ffi::c_int;
            };
            let mut output_buf = None;
            if state.direct == 0 {
                let Some(buf) = gz_alloc_byte_vec(state.want as usize) else {
                    crate::src::gzlib::gz_error_static(
                        state,
                        crate::zlib_h::Z_MEM_ERROR,
                        b"out of memory\0",
                    );
                    break 'gz_init_result -1 as ::core::ffi::c_int;
                };
                output_buf = Some(buf);
                state.strm.zalloc = None;
                state.strm.zfree = None;
                state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
                let ret = unsafe {
                    crate::src::deflate::deflateInit2__ffi(
                        &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                        state.level,
                        8 as ::core::ffi::c_int,
                        15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        state.strategy,
                        crate::zlib_h::ZLIB_VERSION.as_ptr(),
                        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
                    )
                };
                if ret != crate::zlib_h::Z_OK {
                    crate::src::gzlib::gz_error_static(
                        state,
                        crate::zlib_h::Z_MEM_ERROR,
                        b"out of memory\0",
                    );
                    break 'gz_init_result -1 as ::core::ffi::c_int;
                }
                state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
            }
            state.in_0 = input_buf.as_mut_ptr();
            let output_buf = if let Some(mut output_buf) = output_buf {
                state.out = output_buf.as_mut_ptr();
                output_buf
            } else {
                Vec::new()
            };
            gz_store_owned_buffers(state, input_buf, output_buf);
            state.size = state.want;
            if state.direct == 0 {
                gz_reset_write_output(state);
            }
            break 'gz_init_result 0 as ::core::ffi::c_int;
        }
    }};
}

macro_rules! gz_deflate_reset_if_needed {
    ($state:expr, $flush:expr, $will_have_input:expr) => {{
        let will_have_input = $will_have_input;
        let state: &mut crate::gzguts_h::gz_state = &mut *$state;
        if state.direct == 0 && state.reset != 0 {
            if !(state.strm.avail_in == 0 as crate::stdlib::uInt
                && !will_have_input
                && $flush == crate::zlib_h::Z_NO_FLUSH)
            {
                unsafe {
                    crate::src::deflate::deflateReset_ffi(
                        &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                    );
                }
                state.reset = 0 as ::core::ffi::c_int;
            }
        }
    }};
}

#[derive(Copy, Clone)]
struct GzDeflateAction {
    flush: ::core::ffi::c_int,
    before_avail_out: crate::stdlib::uInt,
}

#[derive(Copy, Clone)]
enum GzCompStatus {
    Done(::core::ffi::c_int),
    NeedDeflate(GzDeflateAction),
}

enum GzStepStatus<T> {
    Done(T),
    NeedDeflate(GzDeflateAction),
}

enum GzPendingCompPoll {
    Idle,
    Finished(::core::ffi::c_int),
    NeedDeflate(GzDeflateAction),
}

fn gz_pending_comp_poll(pending: &mut Option<GzCompStatus>) -> GzPendingCompPoll {
    let Some(status) = pending.take() else {
        return GzPendingCompPoll::Idle;
    };
    match status {
        GzCompStatus::Done(ret) => GzPendingCompPoll::Finished(ret),
        GzCompStatus::NeedDeflate(action) => {
            *pending = Some(GzCompStatus::NeedDeflate(action));
            GzPendingCompPoll::NeedDeflate(action)
        }
    }
}

fn gz_pending_comp_after_deflate(
    state: &mut crate::gzguts_h::gz_state,
    pending: &mut Option<GzCompStatus>,
    action: GzDeflateAction,
    ret: ::core::ffi::c_int,
) {
    *pending = Some(gz_comp_after_deflate(state, action, ret));
}

fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
    direct_input: Option<&[crate::stdlib::Bytef]>,
) -> GzCompStatus {
    let mut put: ::core::ffi::c_uint = 0;
    if state.size == 0 as ::core::ffi::c_uint {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"internal error: write buffers not initialized\0",
        );
        return GzCompStatus::Done(-1 as ::core::ffi::c_int);
    }
    if state.direct != 0 {
        let mut input_offset = 0usize;
        let input = direct_input.unwrap_or(&[]);
        if input.len() < state.strm.avail_in as usize {
            crate::src::gzlib::gz_error_static(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: direct write buffer missing\0",
            );
            return GzCompStatus::Done(-1 as ::core::ffi::c_int);
        }
        while state.strm.avail_in != 0 {
            state.again = 0 as ::core::ffi::c_int;
            put = gz_io_chunk_len(state.strm.avail_in);
            let write_result =
                gz_write_file(state, &input[input_offset..input_offset + put as usize]);
            match write_result {
                GzWriteSyscallResult::Wrote(written) => {
                    state.strm.avail_in = state
                        .strm
                        .avail_in
                        .wrapping_sub(written as ::core::ffi::c_uint);
                    input_offset = input_offset.wrapping_add(written as usize);
                    state.strm.next_in =
                        input[input_offset..].as_ptr() as *mut crate::stdlib::Bytef;
                }
                GzWriteSyscallResult::Error { errno, again } => {
                    state.again = again;
                    gz_error_with_os_error(state, crate::zlib_h::Z_ERRNO, errno);
                    return GzCompStatus::Done(-1 as ::core::ffi::c_int);
                }
            }
        }
        return GzCompStatus::Done(0 as ::core::ffi::c_int);
    }
    match gz_comp_reset_action(state.reset, state.strm.avail_in, flush) {
        GzCompResetAction::None => {}
        GzCompResetAction::ReturnOk => {
            return GzCompStatus::Done(0 as ::core::ffi::c_int);
        }
        GzCompResetAction::ResetStream => {
            crate::src::gzlib::gz_error_static(
                state,
                crate::zlib_h::Z_STREAM_ERROR,
                b"internal error: deflate reset not performed at boundary\0",
            );
            return GzCompStatus::Done(-1 as ::core::ffi::c_int);
        }
    }
    gz_comp_continue(state, flush, crate::zlib_h::Z_OK)
}

fn gz_comp_continue(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> GzCompStatus {
    let max = gz_io_chunk_limit();
    loop {
        if gz_comp_should_write_pending(state.strm.avail_out, flush, ret) {
            while let Some(chunk) = gz_pending_output_chunk(state, max) {
                state.again = 0 as ::core::ffi::c_int;
                let write_result = gz_with_output_buffer_mut(state, |state, output_buf| {
                    let next_offset =
                        (state.x.next as usize).wrapping_sub(output_buf.as_ptr() as usize);
                    gz_write_file(
                        state,
                        &output_buf[next_offset..next_offset + chunk as usize],
                    )
                });
                match write_result {
                    GzWriteSyscallResult::Wrote(written) => {
                        state.x.next = state.x.next.wrapping_add(written as usize);
                    }
                    GzWriteSyscallResult::Error { errno, again } => {
                        state.again = again;
                        gz_error_with_os_error(state, crate::zlib_h::Z_ERRNO, errno);
                        return GzCompStatus::Done(-1 as ::core::ffi::c_int);
                    }
                }
            }
            if state.strm.avail_out == 0 as crate::stdlib::uInt {
                gz_reset_write_output(state);
            }
        }
        return GzCompStatus::NeedDeflate(GzDeflateAction {
            flush,
            before_avail_out: state.strm.avail_out,
        });
    }
}

fn gz_comp_after_deflate(
    state: &mut crate::gzguts_h::gz_state,
    action: GzDeflateAction,
    ret: ::core::ffi::c_int,
) -> GzCompStatus {
    if ret == crate::zlib_h::Z_STREAM_ERROR {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"internal error: deflate stream corrupt\0",
        );
        return GzCompStatus::Done(-1 as ::core::ffi::c_int);
    }
    let have = action.before_avail_out.wrapping_sub(state.strm.avail_out);
    if have != 0 {
        return gz_comp_continue(state, action.flush, ret);
    }
    if action.flush == crate::zlib_h::Z_FINISH {
        state.reset = 1 as ::core::ffi::c_int;
    }
    GzCompStatus::Done(0 as ::core::ffi::c_int)
}

fn gz_direct_input_slice<'a>(
    state: &crate::gzguts_h::gz_state,
    input_buf: &'a [crate::stdlib::Bytef],
) -> &'a [crate::stdlib::Bytef] {
    let next_offset = (state.strm.next_in as usize).wrapping_sub(input_buf.as_ptr() as usize);
    &input_buf[next_offset..next_offset + state.strm.avail_in as usize]
}

fn gz_comp_with_state_input(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
) -> GzCompStatus {
    if state.direct != 0
        && state.size != 0 as ::core::ffi::c_uint
        && state.strm.avail_in != 0 as crate::stdlib::uInt
    {
        gz_with_input_buffer_mut(state, |state, input_buf| {
            let input_buf = &input_buf[..state.size as usize];
            gz_comp(state, flush, Some(gz_direct_input_slice(state, input_buf)))
        })
    } else {
        gz_comp(state, flush, None)
    }
}

struct GzZeroCursor {
    pending_comp: Option<GzCompStatus>,
    pending_n: crate::stdlib::uInt,
    draining_initial: bool,
    filled: bool,
}

impl GzZeroCursor {
    fn new() -> Self {
        Self {
            pending_comp: None,
            pending_n: 0 as crate::stdlib::uInt,
            draining_initial: true,
            filled: false,
        }
    }

    fn after_deflate(
        &mut self,
        state: &mut crate::gzguts_h::gz_state,
        action: GzDeflateAction,
        ret: ::core::ffi::c_int,
    ) {
        gz_pending_comp_after_deflate(state, &mut self.pending_comp, action, ret);
    }
}

fn gz_zero_step(
    state: &mut crate::gzguts_h::gz_state,
    input_buf: &mut [crate::stdlib::Bytef],
    cursor: &mut GzZeroCursor,
) -> GzStepStatus<::core::ffi::c_int> {
    loop {
        match gz_pending_comp_poll(&mut cursor.pending_comp) {
            GzPendingCompPoll::Idle => {}
            GzPendingCompPoll::NeedDeflate(action) => {
                return GzStepStatus::NeedDeflate(action);
            }
            GzPendingCompPoll::Finished(ret) => {
                if cursor.pending_n != 0 {
                    let consumed = gz_note_input_consumed(state, cursor.pending_n);
                    state.skip -= consumed as crate::stdlib::off64_t;
                    cursor.pending_n = 0 as crate::stdlib::uInt;
                } else {
                    cursor.draining_initial = false;
                }
                if ret == -1 as ::core::ffi::c_int {
                    return GzStepStatus::Done(-1 as ::core::ffi::c_int);
                }
                if state.skip == 0 {
                    return GzStepStatus::Done(0 as ::core::ffi::c_int);
                }
            }
        }

        if cursor.draining_initial {
            if state.strm.avail_in != 0 {
                cursor.pending_comp = Some(gz_comp(
                    state,
                    crate::zlib_h::Z_NO_FLUSH,
                    Some(gz_direct_input_slice(state, input_buf)),
                ));
                continue;
            }
            cursor.draining_initial = false;
        }

        let n = gz_clamped_uint(state.size, state.skip);
        if !cursor.filled {
            gz_fill_zero(&mut input_buf[..n as usize]);
            cursor.filled = true;
        }
        state.strm.avail_in = n as crate::stdlib::uInt;
        state.strm.next_in = input_buf.as_mut_ptr();
        cursor.pending_n = n;
        cursor.pending_comp = Some(gz_comp(
            state,
            crate::zlib_h::Z_NO_FLUSH,
            Some(gz_direct_input_slice(state, input_buf)),
        ));
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum GzWritePhase {
    Start,
    Zero,
    Choose,
    Small,
    LargeDrain,
    LargeChunks,
}

#[derive(Copy, Clone)]
enum GzWritePendingKind {
    None,
    Small,
    LargeDrain,
    LargeChunk { requested: crate::stdlib::uInt },
}

struct GzWriteCursor {
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
    input_offset: usize,
    phase: GzWritePhase,
    zero: GzZeroCursor,
    pending_comp: Option<GzCompStatus>,
    pending_kind: GzWritePendingKind,
}

impl GzWriteCursor {
    fn new(len: crate::stdlib::z_size_t) -> Self {
        Self {
            requested: len,
            remaining: len,
            input_offset: 0,
            phase: GzWritePhase::Start,
            zero: GzZeroCursor::new(),
            pending_comp: None,
            pending_kind: GzWritePendingKind::None,
        }
    }

    fn after_deflate(
        &mut self,
        state: &mut crate::gzguts_h::gz_state,
        action: GzDeflateAction,
        ret: ::core::ffi::c_int,
    ) {
        if self.phase == GzWritePhase::Zero {
            self.zero.after_deflate(state, action, ret);
        } else {
            gz_pending_comp_after_deflate(state, &mut self.pending_comp, action, ret);
        }
    }
}

fn gz_write_step(
    state: &mut crate::gzguts_h::gz_state,
    input_buf: &mut [crate::stdlib::Bytef],
    input: &[crate::stdlib::Bytef],
    cursor: &mut GzWriteCursor,
) -> GzStepStatus<crate::stdlib::z_size_t> {
    loop {
        match gz_pending_comp_poll(&mut cursor.pending_comp) {
            GzPendingCompPoll::Idle => {}
            GzPendingCompPoll::NeedDeflate(action) => {
                return GzStepStatus::NeedDeflate(action);
            }
            GzPendingCompPoll::Finished(ret) => match cursor.pending_kind {
                GzWritePendingKind::None => {}
                GzWritePendingKind::Small => {
                    cursor.pending_kind = GzWritePendingKind::None;
                    if ret == -1 as ::core::ffi::c_int {
                        return GzStepStatus::Done(gz_write_error_return(
                            state.again,
                            cursor.requested,
                            cursor.remaining,
                        ));
                    }
                }
                GzWritePendingKind::LargeDrain => {
                    cursor.pending_kind = GzWritePendingKind::None;
                    cursor.phase = GzWritePhase::LargeChunks;
                    if ret == -1 as ::core::ffi::c_int {
                        return GzStepStatus::Done(0 as crate::stdlib::z_size_t);
                    }
                }
                GzWritePendingKind::LargeChunk { requested } => {
                    cursor.pending_kind = GzWritePendingKind::None;
                    let consumed = gz_note_input_consumed(state, requested);
                    cursor.remaining = cursor
                        .remaining
                        .wrapping_sub(consumed as crate::stdlib::z_size_t);
                    cursor.input_offset = cursor.input_offset.wrapping_add(consumed as usize);
                    if ret == -1 as ::core::ffi::c_int {
                        return GzStepStatus::Done(gz_write_error_return(
                            state.again,
                            cursor.requested,
                            cursor.remaining,
                        ));
                    }
                    if cursor.remaining == 0 as crate::stdlib::z_size_t {
                        return GzStepStatus::Done(cursor.requested);
                    }
                }
            },
        }

        match cursor.phase {
            GzWritePhase::Start => {
                if cursor.requested == 0 as crate::stdlib::z_size_t {
                    return GzStepStatus::Done(0 as crate::stdlib::z_size_t);
                }
                cursor.phase = if state.skip != 0 {
                    GzWritePhase::Zero
                } else {
                    GzWritePhase::Choose
                };
            }
            GzWritePhase::Zero => match gz_zero_step(state, input_buf, &mut cursor.zero) {
                GzStepStatus::NeedDeflate(action) => return GzStepStatus::NeedDeflate(action),
                GzStepStatus::Done(ret) => {
                    if ret == -1 as ::core::ffi::c_int {
                        return GzStepStatus::Done(0 as crate::stdlib::z_size_t);
                    }
                    cursor.phase = GzWritePhase::Choose;
                }
            },
            GzWritePhase::Choose => {
                cursor.phase = if cursor.remaining < state.size as crate::stdlib::z_size_t
                    || state.direct != 0
                {
                    GzWritePhase::Small
                } else {
                    GzWritePhase::LargeDrain
                };
            }
            GzWritePhase::Small => {
                let have = if state.strm.avail_in == 0 as crate::stdlib::uInt {
                    state.strm.next_in = input_buf.as_mut_ptr();
                    0 as crate::stdlib::uInt
                } else {
                    gz_buffered_input_used(state)
                };
                let copy = gz_buffered_write_copy_len(state.size, have, cursor.remaining);
                input_buf[have as usize..have.wrapping_add(copy) as usize].copy_from_slice(
                    &input[cursor.input_offset..cursor.input_offset + copy as usize],
                );
                gz_note_buffered_input(state, copy);
                cursor.input_offset = cursor.input_offset.wrapping_add(copy as usize);
                cursor.remaining = cursor
                    .remaining
                    .wrapping_sub(copy as crate::stdlib::z_size_t);
                if cursor.remaining == 0 as crate::stdlib::z_size_t {
                    return GzStepStatus::Done(cursor.requested);
                }
                cursor.pending_kind = GzWritePendingKind::Small;
                cursor.pending_comp = Some(gz_comp(
                    state,
                    crate::zlib_h::Z_NO_FLUSH,
                    Some(gz_direct_input_slice(state, input_buf)),
                ));
            }
            GzWritePhase::LargeDrain => {
                if state.strm.avail_in != 0 {
                    cursor.pending_kind = GzWritePendingKind::LargeDrain;
                    cursor.pending_comp = Some(gz_comp(state, crate::zlib_h::Z_NO_FLUSH, None));
                } else {
                    cursor.phase = GzWritePhase::LargeChunks;
                }
            }
            GzWritePhase::LargeChunks => {
                if cursor.remaining == 0 as crate::stdlib::z_size_t {
                    return GzStepStatus::Done(cursor.requested);
                }
                let n = gz_z_size_to_uInt_chunk(cursor.remaining);
                state.strm.next_in =
                    input[cursor.input_offset..].as_ptr() as *mut crate::stdlib::Bytef;
                state.strm.avail_in = n as crate::stdlib::uInt;
                cursor.pending_kind = GzWritePendingKind::LargeChunk { requested: n };
                cursor.pending_comp = Some(gz_comp(state, crate::zlib_h::Z_NO_FLUSH, None));
            }
        }
    }
}
fn gz_write_state_ready(state: &crate::gzguts_h::gz_state) -> bool {
    state.mode == crate::gzguts_h::GZ_WRITE
        && (state.err == crate::zlib_h::Z_OK || state.again != 0)
}

fn gz_write_params_ready(state: &crate::gzguts_h::gz_state) -> bool {
    gz_write_state_ready(state) && state.direct == 0
}

fn gz_write_error_return(
    again: ::core::ffi::c_int,
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if again != 0 {
        requested.wrapping_sub(remaining)
    } else {
        0 as crate::stdlib::z_size_t
    }
}

fn gz_write_errno_again(errno: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if gz_errno_is_retryable(errno) {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }
}

enum GzWriteSyscallResult {
    Wrote(::core::ffi::c_int),
    Error {
        errno: ::core::ffi::c_int,
        again: ::core::ffi::c_int,
    },
}

fn gz_write_file(
    state: &crate::gzguts_h::gz_state,
    buf: &[crate::stdlib::Bytef],
) -> GzWriteSyscallResult {
    let write_result = gz_with_file_mut(state, |file| ::std::io::Write::write(file, buf));
    match write_result {
        Ok(written) => GzWriteSyscallResult::Wrote(written as ::core::ffi::c_int),
        Err(err) => {
            let errno = err.raw_os_error().unwrap_or(0 as ::core::ffi::c_int);
            GzWriteSyscallResult::Error {
                errno,
                again: gz_write_errno_again(errno),
            }
        }
    }
}

fn gz_comp_should_write_pending(
    avail_out: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> bool {
    avail_out == 0 as crate::stdlib::uInt
        || flush != crate::zlib_h::Z_NO_FLUSH
            && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
}

enum GzCompResetAction {
    None,
    ReturnOk,
    ResetStream,
}

fn gz_comp_reset_action(
    reset: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> GzCompResetAction {
    if reset == 0 {
        GzCompResetAction::None
    } else if avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
        GzCompResetAction::ReturnOk
    } else {
        GzCompResetAction::ResetStream
    }
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

fn gz_reset_write_output(state: &mut crate::gzguts_h::gz_state) {
    state.strm.avail_out = state.size as crate::stdlib::uInt;
    state.strm.next_out = state.out as *mut crate::stdlib::Bytef;
    state.x.next = state.out;
}

fn gz_buffered_input_used(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_uint {
    gz_buffered_input_used_addrs(
        state.strm.next_in as usize,
        state.strm.avail_in,
        state.in_0 as usize,
    )
}

fn gz_buffered_input_used_addrs(
    next_in_addr: usize,
    avail_in: crate::stdlib::uInt,
    base_addr: usize,
) -> ::core::ffi::c_uint {
    next_in_addr
        .wrapping_add(avail_in as usize)
        .wrapping_sub(base_addr) as ::core::ffi::c_uint
}

fn gz_buffered_write_copy_len(
    size: crate::stdlib::uInt,
    used: crate::stdlib::uInt,
    len: crate::stdlib::z_size_t,
) -> crate::stdlib::uInt {
    let space = size.wrapping_sub(used);
    if space as crate::stdlib::z_size_t > len {
        len as crate::stdlib::uInt
    } else {
        space
    }
}

fn gz_pending_output_chunk(
    state: &crate::gzguts_h::gz_state,
    max: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    gz_pending_output_chunk_addrs(state.strm.next_out as usize, state.x.next as usize, max)
}

fn gz_pending_output_chunk_addrs(
    next_out_addr: usize,
    next_addr: usize,
    max: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if next_out_addr <= next_addr {
        return None;
    }
    let pending = next_out_addr - next_addr;
    Some(if pending > max as usize {
        max
    } else {
        pending as ::core::ffi::c_uint
    })
}

fn gz_fill_zero(buf: &mut [crate::stdlib::Bytef]) {
    buf.fill(0);
}

fn gz_store_buffered_byte(
    buf: &mut [crate::stdlib::Bytef],
    offset: ::core::ffi::c_uint,
    c: ::core::ffi::c_int,
) {
    buf[offset as usize] = c as ::core::ffi::c_uchar;
}

fn gzputc_buffered(
    state: &mut crate::gzguts_h::gz_state,
    buf: &mut [crate::stdlib::Bytef],
    c: ::core::ffi::c_int,
) -> Option<::core::ffi::c_int> {
    if state.size == 0 {
        return None;
    }
    if state.strm.avail_in == 0 as crate::stdlib::uInt {
        state.strm.next_in = buf.as_mut_ptr();
    }
    let have = gz_buffered_input_used(state);
    if have < state.size {
        gz_store_buffered_byte(buf, have, c);
        gz_note_buffered_input(state, 1 as ::core::ffi::c_uint);
        Some(c & 0xff as ::core::ffi::c_int)
    } else {
        None
    }
}

fn gzwrite_len_fits_int(len: ::core::ffi::c_uint) -> bool {
    gz_uInt_fits_int(len)
}

fn gzputc_impl(c: ::core::ffi::c_int, put: crate::stdlib::z_size_t) -> ::core::ffi::c_int {
    if put != 1 as crate::stdlib::z_size_t {
        return -1 as ::core::ffi::c_int;
    }
    c & 0xff as ::core::ffi::c_int
}

#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return 0 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    if !gzwrite_len_fits_int(len) {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_DATA_ERROR,
            b"requested length does not fit in int\0",
        );
        return 0 as ::core::ffi::c_int;
    }
    let input = if len == 0 {
        &[] as &[crate::stdlib::Bytef]
    } else {
        ::core::slice::from_raw_parts(buf as *const crate::stdlib::Bytef, len as usize)
    };
    if !input.is_empty()
        && state.size == 0 as ::core::ffi::c_uint
        && gz_init!(state) == -1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if !input.is_empty() {
        gz_deflate_reset_if_needed!(state, crate::zlib_h::Z_NO_FLUSH, true);
    }
    let mut cursor = GzWriteCursor::new(input.len() as crate::stdlib::z_size_t);
    loop {
        let step = if state.size == 0 {
            let mut empty_input_buf = [];
            gz_write_step(state, &mut empty_input_buf[..], input, &mut cursor)
        } else {
            gz_with_input_buffer_mut(state, |state, input_buf| {
                let input_buf = &mut input_buf[..state.size as usize];
                gz_write_step(state, input_buf, input, &mut cursor)
            })
        };
        match step {
            GzStepStatus::Done(put) => return put as ::core::ffi::c_int,
            GzStepStatus::NeedDeflate(action) => {
                let ret = crate::src::deflate::deflate_ffi(
                    &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                    action.flush,
                );
                cursor.after_deflate(state, action, ret);
            }
        }
    }
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    if file.is_null() {
        return 0 as crate::stdlib::z_size_t;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return 0 as crate::stdlib::z_size_t;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    let Some(len) = gz_file_request_len(size, nitems) else {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"request does not fit in a size_t\0",
        );
        return 0 as crate::stdlib::z_size_t;
    };
    let completed = if len != 0 {
        let input = ::core::slice::from_raw_parts(buf as *const crate::stdlib::Bytef, len);
        if state.size == 0 as ::core::ffi::c_uint && gz_init!(state) == -1 as ::core::ffi::c_int {
            return 0 as crate::stdlib::z_size_t;
        }
        gz_deflate_reset_if_needed!(state, crate::zlib_h::Z_NO_FLUSH, true);
        let mut cursor = GzWriteCursor::new(input.len() as crate::stdlib::z_size_t);
        loop {
            let step = gz_with_input_buffer_mut(state, |state, input_buf| {
                let input_buf = &mut input_buf[..state.size as usize];
                gz_write_step(state, input_buf, input, &mut cursor)
            });
            match step {
                GzStepStatus::Done(put) => break put,
                GzStepStatus::NeedDeflate(action) => {
                    let ret = crate::src::deflate::deflate_ffi(
                        &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                        action.flush,
                    );
                    cursor.after_deflate(state, action, ret);
                }
            }
        }
    } else {
        0 as crate::stdlib::z_size_t
    };
    return gz_file_completed_items(len, size, completed);
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    if state.size == 0 as ::core::ffi::c_uint && gz_init!(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    gz_deflate_reset_if_needed!(state, crate::zlib_h::Z_NO_FLUSH, true);
    if state.skip != 0 {
        let mut zero = GzZeroCursor::new();
        loop {
            let step = gz_with_input_buffer_mut(state, |state, input_buf| {
                let input_buf = &mut input_buf[..state.size as usize];
                gz_zero_step(state, input_buf, &mut zero)
            });
            match step {
                GzStepStatus::Done(ret) => {
                    if ret == -1 as ::core::ffi::c_int {
                        return -1 as ::core::ffi::c_int;
                    }
                    break;
                }
                GzStepStatus::NeedDeflate(action) => {
                    let ret = crate::src::deflate::deflate_ffi(
                        &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                        action.flush,
                    );
                    zero.after_deflate(state, action, ret);
                }
            }
        }
    }
    let buffered = gz_with_input_buffer_mut(state, |state, input_buf| {
        let input_buf = &mut input_buf[..state.size as usize];
        gzputc_buffered(state, input_buf, c)
    });
    if let Some(ret) = buffered {
        return ret;
    }
    let buf: [::core::ffi::c_uchar; 1] = [c as ::core::ffi::c_uchar; 1];
    let mut cursor = GzWriteCursor::new(1 as crate::stdlib::z_size_t);
    loop {
        let step = gz_with_input_buffer_mut(state, |state, input_buf| {
            let input_buf = &mut input_buf[..state.size as usize];
            gz_write_step(state, input_buf, &buf, &mut cursor)
        });
        match step {
            GzStepStatus::Done(put) => return gzputc_impl(c, put),
            GzStepStatus::NeedDeflate(action) => {
                let ret = crate::src::deflate::deflate_ffi(
                    &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                    action.flush,
                );
                cursor.after_deflate(state, action, ret);
            }
        }
    }
}
fn gzputs_len_fits_int(len: crate::stdlib::z_size_t) -> bool {
    (len as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int
        && len as ::core::ffi::c_uint as crate::stdlib::z_size_t == len
}

fn gzputs_return_value(
    len: crate::stdlib::z_size_t,
    put: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if len != 0 && put == 0 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        put as ::core::ffi::c_int
    }
}

fn gzputs_len_or_error(
    state: &mut crate::gzguts_h::gz_state,
    input: &[crate::stdlib::Bytef],
) -> Option<crate::stdlib::z_size_t> {
    let len = input.len() as crate::stdlib::z_size_t;
    if !gzputs_len_fits_int(len) {
        crate::src::gzlib::gz_error_static(
            state,
            crate::zlib_h::Z_STREAM_ERROR,
            b"string length does not fit in int\0",
        );
        None
    } else {
        Some(len)
    }
}

fn gzputs_impl(len: crate::stdlib::z_size_t, put: crate::stdlib::z_size_t) -> ::core::ffi::c_int {
    gzputs_return_value(len, put)
}

#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return -1 as ::core::ffi::c_int;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    let input = ::core::ffi::CStr::from_ptr(s).to_bytes();
    let Some(len) = gzputs_len_or_error(state, input) else {
        return -1 as ::core::ffi::c_int;
    };
    if !input.is_empty()
        && state.size == 0 as ::core::ffi::c_uint
        && gz_init!(state) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    if !input.is_empty() {
        gz_deflate_reset_if_needed!(state, crate::zlib_h::Z_NO_FLUSH, true);
    }
    let mut cursor = GzWriteCursor::new(input.len() as crate::stdlib::z_size_t);
    loop {
        let step = if state.size == 0 {
            let mut empty_input_buf = [];
            gz_write_step(state, &mut empty_input_buf[..], input, &mut cursor)
        } else {
            gz_with_input_buffer_mut(state, |state, input_buf| {
                let input_buf = &mut input_buf[..state.size as usize];
                gz_write_step(state, input_buf, input, &mut cursor)
            })
        };
        match step {
            GzStepStatus::Done(put) => return gzputs_impl(len, put),
            GzStepStatus::NeedDeflate(action) => {
                let ret = crate::src::deflate::deflate_ffi(
                    &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                    action.flush,
                );
                cursor.after_deflate(state, action, ret);
            }
        }
    }
}
fn gzflush_valid_flush(flush: ::core::ffi::c_int) -> bool {
    flush >= 0 as ::core::ffi::c_int && flush <= crate::zlib_h::Z_FINISH
}

#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_state_ready(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    if !gzflush_valid_flush(flush) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 {
        if state.size == 0 as ::core::ffi::c_uint && gz_init!(state) == -1 as ::core::ffi::c_int {
            return state.err;
        }
        gz_deflate_reset_if_needed!(state, crate::zlib_h::Z_NO_FLUSH, true);
        let mut zero = GzZeroCursor::new();
        loop {
            let step = gz_with_input_buffer_mut(state, |state, input_buf| {
                let input_buf = &mut input_buf[..state.size as usize];
                gz_zero_step(state, input_buf, &mut zero)
            });
            match step {
                GzStepStatus::Done(ret) => {
                    if ret == -1 as ::core::ffi::c_int {
                        return state.err;
                    }
                    break;
                }
                GzStepStatus::NeedDeflate(action) => {
                    let ret = crate::src::deflate::deflate_ffi(
                        &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                        action.flush,
                    );
                    zero.after_deflate(state, action, ret);
                }
            }
        }
    }
    if state.size == 0 as ::core::ffi::c_uint && gz_init!(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    gz_deflate_reset_if_needed!(
        state,
        flush,
        state.strm.avail_in != 0 as crate::stdlib::uInt
    );
    let mut pending = Some(gz_comp_with_state_input(state, flush));
    loop {
        match gz_pending_comp_poll(&mut pending) {
            GzPendingCompPoll::Idle => break,
            GzPendingCompPoll::Finished(_) => break,
            GzPendingCompPoll::NeedDeflate(action) => {
                let ret = crate::src::deflate::deflate_ffi(
                    &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                    action.flush,
                );
                gz_pending_comp_after_deflate(state, &mut pending, action, ret);
            }
        }
    }
    return state.err;
}
fn gzsetparams_unchanged(
    state: &crate::gzguts_h::gz_state,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> bool {
    level == state.level && strategy == state.strategy
}

fn gzclose_w_after_step(
    ret: ::core::ffi::c_int,
    failed: bool,
    state_err: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if failed {
        state_err
    } else {
        ret
    }
}

fn gzclose_w_final_status(ret: ::core::ffi::c_int, close_failed: bool) -> ::core::ffi::c_int {
    if close_failed {
        crate::zlib_h::Z_ERRNO
    } else {
        ret
    }
}

#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if !gz_write_params_ready(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    if gzsetparams_unchanged(state, level, strategy) {
        return crate::zlib_h::Z_OK;
    }
    if state.skip != 0 {
        if state.size == 0 as ::core::ffi::c_uint && gz_init!(state) == -1 as ::core::ffi::c_int {
            return state.err;
        }
        gz_deflate_reset_if_needed!(state, crate::zlib_h::Z_NO_FLUSH, true);
        let mut zero = GzZeroCursor::new();
        loop {
            let step = gz_with_input_buffer_mut(state, |state, input_buf| {
                let input_buf = &mut input_buf[..state.size as usize];
                gz_zero_step(state, input_buf, &mut zero)
            });
            match step {
                GzStepStatus::Done(ret) => {
                    if ret == -1 as ::core::ffi::c_int {
                        return state.err;
                    }
                    break;
                }
                GzStepStatus::NeedDeflate(action) => {
                    let ret = crate::src::deflate::deflate_ffi(
                        &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                        action.flush,
                    );
                    zero.after_deflate(state, action, ret);
                }
            }
        }
    }
    if state.size != 0 {
        if state.strm.avail_in != 0 {
            gz_deflate_reset_if_needed!(state, crate::zlib_h::Z_BLOCK, true);
            let mut pending = Some(gz_comp(state, crate::zlib_h::Z_BLOCK, None));
            let comp_ret = loop {
                match gz_pending_comp_poll(&mut pending) {
                    GzPendingCompPoll::Idle => break 0 as ::core::ffi::c_int,
                    GzPendingCompPoll::Finished(ret) => break ret,
                    GzPendingCompPoll::NeedDeflate(action) => {
                        let ret = crate::src::deflate::deflate_ffi(
                            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                            action.flush,
                        );
                        gz_pending_comp_after_deflate(state, &mut pending, action, ret);
                    }
                }
            };
            if comp_ret == -1 as ::core::ffi::c_int {
                return state.err;
            }
        }
        crate::src::deflate::deflateParams_ffi(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            level,
            strategy,
        );
    }
    state.level = level;
    state.strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "gzclose_w"]
pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = crate::zlib_h::Z_OK;
    if file.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_WRITE {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let zero_failed = if state.skip != 0 {
        if state.size == 0 as ::core::ffi::c_uint && gz_init!(state) == -1 as ::core::ffi::c_int {
            true
        } else {
            gz_deflate_reset_if_needed!(state, crate::zlib_h::Z_NO_FLUSH, true);
            let mut zero = GzZeroCursor::new();
            loop {
                let step = gz_with_input_buffer_mut(state, |state, input_buf| {
                    let input_buf = &mut input_buf[..state.size as usize];
                    gz_zero_step(state, input_buf, &mut zero)
                });
                match step {
                    GzStepStatus::Done(ret) => break ret == -1 as ::core::ffi::c_int,
                    GzStepStatus::NeedDeflate(action) => {
                        let ret = crate::src::deflate::deflate_ffi(
                            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                            action.flush,
                        );
                        zero.after_deflate(state, action, ret);
                    }
                }
            }
        }
    } else {
        false
    };
    ret = gzclose_w_after_step(ret, zero_failed, state.err);
    let init_failed =
        state.size == 0 as ::core::ffi::c_uint && gz_init!(state) == -1 as ::core::ffi::c_int;
    if init_failed {
        ret = gzclose_w_after_step(ret, true, state.err);
    }
    if !init_failed {
        gz_deflate_reset_if_needed!(
            state,
            crate::zlib_h::Z_FINISH,
            state.strm.avail_in != 0 as crate::stdlib::uInt
        );
    }
    let comp_failed = if init_failed {
        true
    } else {
        let mut pending = Some(gz_comp_with_state_input(state, crate::zlib_h::Z_FINISH));
        loop {
            match gz_pending_comp_poll(&mut pending) {
                GzPendingCompPoll::Idle => break false,
                GzPendingCompPoll::Finished(ret) => break ret == -1 as ::core::ffi::c_int,
                GzPendingCompPoll::NeedDeflate(action) => {
                    let ret = crate::src::deflate::deflate_ffi(
                        &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
                        action.flush,
                    );
                    gz_pending_comp_after_deflate(state, &mut pending, action, ret);
                }
            }
        }
    };
    ret = gzclose_w_after_step(ret, comp_failed, state.err);
    if state.size != 0 {
        if state.direct == 0 {
            crate::src::deflate::deflateEnd_ffi(
                &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            );
        }
        gz_remove_owned_buffers(state);
    }
    crate::src::gzlib::gz_error_clear(state, crate::zlib_h::Z_OK);
    crate::src::gzlib::gz_remove_error_info(state);
    let close_fd = gz_remove_owned_file_fd(state).unwrap_or(state.fd);
    ret = gzclose_w_final_status(
        ret,
        crate::stdlib::close(close_fd) == -1 as ::core::ffi::c_int,
    );
    crate::stdlib::free(file as *mut ::core::ffi::c_void);
    return ret;
}
