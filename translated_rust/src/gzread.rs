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

use crate::src::gzlib::GzCodecInput;

fn is_gzip_header(input: &[u8]) -> bool {
    input.len() >= 4 && input[0] == 31 && input[1] == 139 && input[2] == 8 && input[3] < 32
}

fn copy_buffered_input(input: &[u8], output: &mut [u8]) {
    output[..input.len()].copy_from_slice(input);
}

fn copy_through_newline(input: &[u8], output: &mut [u8]) -> (usize, bool) {
    let copied = input
        .iter()
        .position(|&byte| byte == b'\n')
        .map_or(input.len(), |newline| newline + 1);
    output[..copied].copy_from_slice(&input[..copied]);
    (copied, copied != input.len())
}

// Admission to a gzip read operation depends only on scalar state.  Keep
// that decision pointer-free so the eventual gzip owner can reuse it without
// exposing the ABI stream or its cursors to read APIs.
struct GzReadPolicy {
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
}

// The public read entry points all begin with the same scalar admission and
// error-reset transition.  Keep that transition over the pointer-free error
// view, so the eventual gzip owner can reuse it without exposing an ABI
// cursor or embedded stream to the request layer.
struct GzReadRequest {
    policy: GzReadPolicy,
}

// Allocate the read side's paired buffers before changing the ABI-shaped
// state.  The eventual gzip owner can take this transaction directly, while
// this boundary still performs the existing stream/cursor projection.
struct GzReadBuffers {
    input: Box<[u8]>,
    output: Box<[u8]>,
    size: ::core::ffi::c_uint,
}

impl GzReadBuffers {
    fn allocate(want: ::core::ffi::c_uint) -> Option<Self> {
        let input = crate::src::gzlib::gz_buffer(want)?;
        let output = crate::src::gzlib::gz_buffer(want << 1)?;
        Some(Self {
            input,
            output,
            size: want,
        })
    }
}

impl GzReadPolicy {
    fn accepts_read(&self) -> bool {
        self.mode == crate::gzguts_h::GZ_READ
            && (self.err == crate::zlib_h::Z_OK
                || self.err == crate::zlib_h::Z_BUF_ERROR
                || self.again != 0)
    }
}

impl GzReadRequest {
    fn new(mode: ::core::ffi::c_int, err: ::core::ffi::c_int, again: ::core::ffi::c_int) -> Self {
        Self {
            policy: GzReadPolicy { mode, err, again },
        }
    }

    fn begin(&self, error: &mut crate::src::gzlib::GzErrorState<'_>) -> bool {
        if !self.policy.accepts_read() {
            return false;
        }
        error.clear();
        true
    }
}

enum GzLoad {
    Loaded {
        have: ::core::ffi::c_uint,
        eof: bool,
        again: bool,
    },
    Error {
        have: ::core::ffi::c_uint,
        errno_value: ::core::ffi::c_int,
        again: bool,
    },
}

// The load result touches only scalar status and owned error storage. Keeping
// those fields in a separate view lets applying it stay independent of the
// ABI-shaped gzip state.
struct GzLoadTarget<'a> {
    again: &'a mut ::core::ffi::c_int,
    eof: &'a mut ::core::ffi::c_int,
    message: &'a mut Option<Box<[u8]>>,
    error: &'a mut ::core::ffi::c_int,
    buffered: &'a mut ::core::ffi::c_uint,
    path: Option<&'a [u8]>,
}

// Copy-mode reads share the same owned-buffer transaction whether the bytes
// are staged for `gz_fetch()` or sent directly to the caller.  Keep that
// transaction pointer-free so the eventual gzip owner can move both paths
// out of the ABI-shaped state together.
struct GzCopyLoadState<'a> {
    output: &'a mut Option<Box<[u8]>>,
    fd: &'a rustix::fd::OwnedFd,
    target: GzLoadTarget<'a>,
}

// The refill transition needs only owned storage and scalar fields.  In
// particular, the ABI stream cursor is deliberately not part of this view:
// callers publish the buffer base as `next_in` only after this operation has
// restored the owned input allocation and succeeded.
struct GzAvailState<'a> {
    err: &'a mut ::core::ffi::c_int,
    eof: &'a mut ::core::ffi::c_int,
    input_cursor: &'a mut GzCodecInput,
    size: usize,
    input: &'a mut Option<Box<[u8]>>,
    fd: &'a rustix::fd::OwnedFd,
    again: &'a mut ::core::ffi::c_int,
    message: &'a mut Option<Box<[u8]>>,
    buffered: &'a mut ::core::ffi::c_uint,
    path: Option<&'a [u8]>,
}

// This owns the pointer-free portion of one gzip inflate loop.  The ABI
// stream remains projected by its caller only for the actual codec dispatch;
// refill, result handling, and gzip error state need no raw cursor or stream.
struct GzDecompLoopState<'a> {
    err: &'a mut ::core::ffi::c_int,
    eof: &'a mut ::core::ffi::c_int,
    size: usize,
    input: &'a mut Option<Box<[u8]>>,
    fd: &'a rustix::fd::OwnedFd,
    again: &'a mut ::core::ffi::c_int,
    message: &'a mut Option<Box<[u8]>>,
    buffered: &'a mut ::core::ffi::c_uint,
    path: Option<&'a [u8]>,
}

struct GzInflateResult {
    result: ::core::ffi::c_int,
    input: GzCodecInput,
    output_available: crate::stdlib::uInt,
    data_error_message: Option<&'static [u8]>,
}

// LOOK mode has a complete pointer-free transition once the boundary has
// refilled and validated its owned input cursor.  Keep header detection and
// the raw-copy fallback here so the eventual gzip owner need only project an
// ABI stream for the reset/codec calls themselves.
struct GzLookState<'a> {
    direct: &'a mut ::core::ffi::c_int,
    junk: &'a mut ::core::ffi::c_int,
    how: &'a mut ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    input: &'a [u8],
    output: Option<&'a mut [u8]>,
}

enum GzLookAction {
    ResetGzip,
    NeedInput,
    Copy { have: usize },
}

fn gz_look_step(state: GzLookState<'_>) -> Result<GzLookAction, ()> {
    let GzLookState {
        direct,
        junk,
        how,
        again,
        input,
        output,
    } = state;
    if *direct == -1 || *junk == 0 {
        *how = crate::gzguts_h::GZIP;
        *junk = (*junk != -1) as ::core::ffi::c_int;
        *direct = 0;
        return Ok(GzLookAction::ResetGzip);
    }
    if input.is_empty() || again != 0 && input.len() < 4 {
        return Ok(GzLookAction::NeedInput);
    }
    if is_gzip_header(input) {
        *how = crate::gzguts_h::GZIP;
        *junk = 1;
        *direct = 0;
        return Ok(GzLookAction::ResetGzip);
    }
    let Some(output) = output else {
        return Err(());
    };
    let Some(destination) = output.get_mut(..input.len()) else {
        return Err(());
    };
    copy_buffered_input(input, destination);
    *how = crate::gzguts_h::COPY;
    Ok(GzLookAction::Copy { have: input.len() })
}

// Fetch dispatch is a scalar gzip state-machine decision.  Keep it outside
// the ABI-shaped state so a later gzip owner can drive LOOK/COPY/GZIP without
// exposing its stream or buffered cursor to the dispatch layer.
struct GzFetchState {
    how: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
}

enum GzFetchAction {
    Look,
    Copy,
    Gzip,
    Corrupt,
}

impl GzFetchState {
    fn new(
        how: ::core::ffi::c_int,
        have: ::core::ffi::c_uint,
        eof: ::core::ffi::c_int,
        avail_in: crate::stdlib::uInt,
    ) -> Self {
        Self {
            how,
            have,
            eof,
            avail_in,
        }
    }

    fn action(&self) -> GzFetchAction {
        match self.how {
            crate::gzguts_h::LOOK => GzFetchAction::Look,
            crate::gzguts_h::COPY => GzFetchAction::Copy,
            crate::gzguts_h::GZIP => GzFetchAction::Gzip,
            _ => GzFetchAction::Corrupt,
        }
    }

    // A fetch continues only when the selected operation produced no buffered
    // bytes and either needs another read or still has codec input to consume.
    // This preserves the original check while keeping it independent of the
    // ABI stream projection.
    fn needs_more(&self) -> bool {
        self.have == 0 && (self.eof == 0 || self.avail_in != 0)
    }
}

// The complete fetch loop is independent of the ABI-shaped gzip state once
// each selected operation returns its resulting scalar snapshot.  This keeps
// the state-machine ordering (in particular COPY's immediate return) in the
// pointer-free core, leaving the current boundary responsible only for
// projecting and publishing the selected operation.
fn gz_fetch_loop(
    mut fetch: GzFetchState,
    mut dispatch: impl FnMut(GzFetchAction) -> Result<GzFetchState, ()>,
) -> Result<(), ()> {
    loop {
        match fetch.action() {
            GzFetchAction::Look => {
                fetch = dispatch(GzFetchAction::Look)?;
                if fetch.how == crate::gzguts_h::LOOK {
                    return Ok(());
                }
            }
            GzFetchAction::Copy => {
                dispatch(GzFetchAction::Copy)?;
                return Ok(());
            }
            GzFetchAction::Gzip => {
                fetch = dispatch(GzFetchAction::Gzip)?;
            }
            GzFetchAction::Corrupt => {
                dispatch(GzFetchAction::Corrupt)?;
                return Err(());
            }
        }
        if !fetch.needs_more() {
            return Ok(());
        }
    }
}

// Skipping buffered gzip output needs only a checked buffer offset and scalar
// progress.  Keep that transition independent of the ABI cursor so the
// eventual gzip owner can reuse it after the cursor becomes an offset rather
// than a raw pointer.
struct GzSkipState<'a> {
    buffer: Option<&'a [u8]>,
    cursor: usize,
    have: ::core::ffi::c_uint,
    pos: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
    eof: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
}

enum GzSkipStep {
    Advanced,
    Fetch,
    Done,
}

fn gz_skip_step(state: &mut GzSkipState<'_>) -> Result<GzSkipStep, ()> {
    if state.have == 0 {
        return Ok(if state.eof != 0 && state.avail_in == 0 {
            GzSkipStep::Done
        } else {
            GzSkipStep::Fetch
        });
    }
    let n = if ::core::mem::size_of::<::core::ffi::c_int>()
        == ::core::mem::size_of::<crate::stdlib::off64_t>()
        && state.have > crate::src::gzlib::gz_intmax()
        || state.have as crate::stdlib::off64_t > state.skip
    {
        state.skip as ::core::ffi::c_uint
    } else {
        state.have
    };
    if n > state.have {
        return Err(());
    }
    let end = state.cursor.checked_add(state.have as usize).ok_or(())?;
    let Some(buffer) = state.buffer else {
        return Err(());
    };
    if buffer.get(state.cursor..end).is_none() {
        return Err(());
    }
    state.cursor = state.cursor.checked_add(n as usize).ok_or(())?;
    state.have = state.have.wrapping_sub(n);
    state.pos += n as crate::stdlib::off64_t;
    state.skip -= n as crate::stdlib::off64_t;
    Ok(if state.skip == 0 {
        GzSkipStep::Done
    } else {
        GzSkipStep::Advanced
    })
}

// Reading an owned gzip buffer does not require the ABI-shaped state.  Keep
// the I/O loop pointer-free and return every state transition for the caller
// to apply at its existing boundary.
fn gz_load(fd: &rustix::fd::OwnedFd, buf: &mut [u8]) -> GzLoad {
    let mut have: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    loop {
        // `have` is incremented only by bytes read into this same slice and
        // stops at its length, so it remains a valid suffix boundary.
        let output = &mut buf[have as usize..];
        let mut get = output.len() as ::core::ffi::c_uint;
        if get > max {
            get = max;
        }
        match rustix::io::read(fd, &mut output[..get as usize]) {
            Ok(0) => {
                return GzLoad::Loaded {
                    have,
                    eof: true,
                    again: false,
                };
            }
            Ok(read) => {
                have = have.wrapping_add(read as ::core::ffi::c_uint);
                if have as usize >= buf.len() {
                    return GzLoad::Loaded {
                        have,
                        eof: false,
                        again: false,
                    };
                }
            }
            Err(error) => {
                let errno_value = error.raw_os_error();
                let again = errno_value == crate::stdlib::EAGAIN
                    || errno_value == crate::stdlib::EWOULDBLOCK;
                if again && have != 0 {
                    return GzLoad::Loaded {
                        have,
                        eof: false,
                        again: true,
                    };
                }
                return GzLoad::Error {
                    have,
                    errno_value,
                    again,
                };
            }
        }
    }
}

fn apply_gz_load(
    target: GzLoadTarget<'_>,
    result: GzLoad,
) -> Result<::core::ffi::c_uint, ::core::ffi::c_uint> {
    let GzLoadTarget {
        again: state_again,
        eof: state_eof,
        message: stored_message,
        error,
        buffered,
        path,
    } = target;
    match result {
        GzLoad::Loaded { have, eof, again } => {
            *state_again = again as ::core::ffi::c_int;
            if eof {
                *state_eof = 1;
            }
            Ok(have)
        }
        GzLoad::Error {
            have,
            errno_value,
            again,
        } => {
            errno::set_errno(errno::Errno(errno_value));
            *state_again = again as ::core::ffi::c_int;
            let message = errno::Errno(errno_value).to_string();
            crate::src::gzlib::gz_set_error(
                stored_message,
                error,
                buffered,
                *state_again,
                path,
                crate::zlib_h::Z_ERRNO,
                Some(message.as_bytes()),
            );
            Err(have)
        }
    }
}

fn gz_copy_load_into(
    fd: &rustix::fd::OwnedFd,
    output: &mut [u8],
    target: GzLoadTarget<'_>,
) -> Result<::core::ffi::c_uint, ::core::ffi::c_uint> {
    errno::set_errno(errno::Errno(0));
    apply_gz_load(target, gz_load(fd, output))
}

fn gz_copy_load(state: GzCopyLoadState<'_>) -> Result<::core::ffi::c_uint, ::core::ffi::c_uint> {
    let GzCopyLoadState { output, fd, target } = state;
    let Some(mut buffer) = output.take() else {
        return Err(0);
    };
    let result = gz_copy_load_into(fd, buffer.as_mut(), target);
    *output = Some(buffer);
    result
}

fn gz_avail(state: GzAvailState<'_>) -> Option<()> {
    let GzAvailState {
        err,
        eof,
        input_cursor,
        size,
        input,
        fd,
        again,
        message,
        buffered,
        path,
    } = state;
    if *err != crate::zlib_h::Z_OK && *err != crate::zlib_h::Z_BUF_ERROR {
        return None;
    }
    if *eof == 0 {
        let pending = input_cursor.available();
        let Some(mut buffer) = input.take() else {
            return None;
        };
        errno::set_errno(errno::Errno(0));
        let ret = (|| {
            // A zero count intentionally uses an empty owner cursor.  A
            // nonzero count was range-checked when this pointer-free cursor
            // was built at the ABI boundary.
            let Some(mut input) = (if pending == 0 {
                crate::src::gzlib::GzBufferedInput::empty(buffer.as_mut(), size)
            } else {
                crate::src::gzlib::GzBufferedInput::from_index(
                    buffer.as_mut(),
                    input_cursor.cursor(),
                    pending,
                )
            }) else {
                return None;
            };
            let Some(target) = input.refill_target() else {
                return None;
            };
            let load = gz_load(fd, target);
            let added = match load {
                GzLoad::Loaded { have, .. } | GzLoad::Error { have, .. } => have as usize,
            };
            if input.extend(added).is_none() {
                return None;
            }
            match apply_gz_load(
                GzLoadTarget {
                    again,
                    eof,
                    message,
                    error: err,
                    buffered,
                    path,
                },
                load,
            ) {
                Ok(_) => {
                    let Some((cursor, available)) = input.cursor() else {
                        return None;
                    };
                    input_cursor.update(cursor, available);
                    Some(())
                }
                Err(_) => None,
            }
        })();
        *input = Some(buffer);
        if ret.is_none() {
            return None;
        }
    }
    Some(())
}

impl GzDecompLoopState<'_> {
    fn refill(&mut self, decomp: &mut crate::src::gzlib::GzDecompState) -> Result<(), ()> {
        gz_avail(GzAvailState {
            err: self.err,
            eof: self.eof,
            input_cursor: decomp.input_mut(),
            size: self.size,
            input: self.input,
            fd: self.fd,
            again: self.again,
            message: self.message,
            buffered: self.buffered,
            path: self.path,
        })
        .ok_or(())
    }

    fn set_error(&mut self, error: ::core::ffi::c_int, message: &'static [u8]) {
        crate::src::gzlib::GzErrorState {
            message: self.message,
            error: self.err,
            buffered: self.buffered,
            again: *self.again,
            path: self.path,
        }
        .set(error, Some(message));
    }
}

// The loop is entirely over checked cursors, owned buffers, and scalar gzip
// state.  `inflate` is injected so the only ABI stream projection stays in
// the caller until the codec owner/view split can remove it as well.
fn gz_decomp_loop(
    decomp: &mut crate::src::gzlib::GzDecompState,
    state: &mut GzDecompLoopState<'_>,
    mut inflate: impl FnMut(&[u8], &GzCodecInput, crate::stdlib::uInt) -> Option<GzInflateResult>,
) -> ::core::ffi::c_int {
    let mut result = crate::zlib_h::Z_OK;
    loop {
        if decomp.needs_input() && state.refill(decomp).is_err() {
            result = *state.err;
            break;
        }
        if decomp.needs_input() {
            if *state.again == 0 {
                state.set_error(crate::zlib_h::Z_BUF_ERROR, b"unexpected end of file");
            }
            break;
        }
        let Some(call) = state
            .input
            .as_deref()
            .and_then(|input| inflate(input, decomp.input(), decomp.output_available()))
        else {
            result = -1;
            break;
        };
        result = call.result;
        decomp.record_input(call.input);
        match decomp.record_inflate(result, call.output_available) {
            crate::src::gzlib::GzDecompAction::Continue => {}
            crate::src::gzlib::GzDecompAction::Stop => break,
            crate::src::gzlib::GzDecompAction::Junk => {
                result = crate::zlib_h::Z_OK;
                break;
            }
            crate::src::gzlib::GzDecompAction::StreamError => {
                state.set_error(
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt",
                );
                break;
            }
            crate::src::gzlib::GzDecompAction::MemoryError => {
                state.set_error(crate::zlib_h::Z_MEM_ERROR, b"out of memory");
                break;
            }
            crate::src::gzlib::GzDecompAction::DataError => {
                state.set_error(
                    crate::zlib_h::Z_DATA_ERROR,
                    call.data_error_message.unwrap_or(b"compressed data error"),
                );
                break;
            }
        }
    }
    decomp.finish(result)
}

unsafe fn gz_look(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.size == 0 as ::core::ffi::c_uint {
        let Some(buffers) = GzReadBuffers::allocate(state.want) else {
            crate::src::gzlib::GzErrorState {
                message: &mut state.msg,
                error: &mut state.err,
                buffered: &mut state.x.have,
                again: state.again,
                path: state.path.as_deref(),
            }
            .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
            return -1 as ::core::ffi::c_int;
        };
        state.in_0 = Some(buffers.input);
        state.out = Some(buffers.output);
        state.size = buffers.size;
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        state.strm.avail_in = 0 as crate::stdlib::uInt;
        state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
        if crate::src::inflate::inflateInit2_(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        ) != crate::zlib_h::Z_OK
        {
            state.out = None;
            state.in_0 = None;
            state.size = 0 as ::core::ffi::c_uint;
            crate::src::gzlib::GzErrorState {
                message: &mut state.msg,
                error: &mut state.err,
                buffered: &mut state.x.have,
                again: state.again,
                path: state.path.as_deref(),
            }
            .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
            return -1 as ::core::ffi::c_int;
        }
    }
    // This reset is intentionally before refill: opening a normal gzip read
    // starts with `junk == 0`, and the original ordering resets the embedded
    // codec before it attempts any file I/O.
    if state.direct == -1 || state.junk == 0 {
        let action = gz_look_step(GzLookState {
            direct: &mut state.direct,
            junk: &mut state.junk,
            how: &mut state.how,
            again: state.again,
            input: &[],
            output: None,
        });
        if !matches!(action, Ok(GzLookAction::ResetGzip)) {
            return -1;
        }
        crate::src::inflate::inflateReset(&raw mut state.strm as *mut crate::zlib_h::z_stream_s);
        return 0;
    }
    let Some(mut input_cursor) = state.in_0.as_deref().and_then(|buffer| {
        GzCodecInput::from_owned_buffer(buffer, state.strm.next_in.addr(), state.strm.avail_in)
    }) else {
        return -1 as ::core::ffi::c_int;
    };
    if gz_avail(GzAvailState {
        err: &mut state.err,
        eof: &mut state.eof,
        input_cursor: &mut input_cursor,
        size: state.size as usize,
        input: &mut state.in_0,
        fd: state.fd.as_ref().expect("gzip state has an open file"),
        again: &mut state.again,
        message: &mut state.msg,
        buffered: &mut state.x.have,
        path: state.path.as_deref(),
    })
    .is_none()
    {
        return -1 as ::core::ffi::c_int;
    }
    state.strm.next_in = state.in_0.as_deref_mut().unwrap().as_mut_ptr();
    state.strm.avail_in = input_cursor.available();
    // The successful refill above leaves a checked owner cursor.  Retain that
    // index through copy detection instead of rebuilding a view from the ABI
    // stream pointer that is published only for the subsequent codec call.
    let Some(input) = state
        .in_0
        .as_deref()
        .and_then(|buffer| input_cursor.bytes(buffer))
    else {
        return -1 as ::core::ffi::c_int;
    };
    let action = gz_look_step(GzLookState {
        direct: &mut state.direct,
        junk: &mut state.junk,
        how: &mut state.how,
        again: state.again,
        input,
        output: state.out.as_deref_mut(),
    });
    match action {
        Ok(GzLookAction::ResetGzip) => {
            crate::src::inflate::inflateReset(
                &raw mut state.strm as *mut crate::zlib_h::z_stream_s,
            );
            0
        }
        Ok(GzLookAction::NeedInput) => 0,
        Ok(GzLookAction::Copy { have }) => {
            let Some(output) = state.out.as_deref_mut() else {
                return -1;
            };
            state.x.next = output.as_mut_ptr();
            state.x.have = have as ::core::ffi::c_uint;
            state.strm.avail_in = 0;
            0
        }
        Err(()) => -1,
    }
}

unsafe fn gz_decomp(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let output_len = (state.size << 1 as ::core::ffi::c_int) as usize;
    // Publish the ABI codec cursor only after a pointer-free view proves that
    // its complete advertised output range lies in the owned gzip buffer.
    let Some(next_out) = state.out.as_deref_mut().and_then(|buffer| {
        crate::src::gzlib::GzCodecOutputView::prefix(buffer, output_len)
            .map(|mut output| output.bytes_mut().as_mut_ptr())
    }) else {
        return -1 as ::core::ffi::c_int;
    };
    let Some(input) = state.in_0.as_deref().and_then(|buffer| {
        GzCodecInput::from_owned_buffer(buffer, state.strm.next_in.addr(), state.strm.avail_in)
    }) else {
        return -1 as ::core::ffi::c_int;
    };
    let Some(mut decomp) =
        crate::src::gzlib::GzDecompState::new(output_len, input, state.junk, state.eof, state.how)
    else {
        return -1 as ::core::ffi::c_int;
    };
    state.strm.avail_out = decomp.output_available();
    state.strm.next_out = next_out;
    // `inflate()` advances `next_out`, but gzip's buffered cursor must point
    // at the beginning of this output span. Retain that boundary value rather
    // than recovering it later with raw-pointer arithmetic.
    let output_start = state.strm.next_out;
    let result = {
        // The stream itself is embedded in the state we already exclusively
        // own. Its cursor projection and the unsafe codec call stay in this
        // small closure; the loop around it is pointer-free.
        let strm = &mut state.strm;
        let mut loop_state = GzDecompLoopState {
            err: &mut state.err,
            eof: &mut state.eof,
            size: state.size as usize,
            input: &mut state.in_0,
            fd: state.fd.as_ref().expect("gzip state has an open file"),
            again: &mut state.again,
            message: &mut state.msg,
            buffered: &mut state.x.have,
            path: state.path.as_deref(),
        };
        gz_decomp_loop(
            &mut decomp,
            &mut loop_state,
            |input, input_cursor, available_out| {
                strm.next_in = input
                    .as_ptr()
                    .wrapping_add(input_cursor.cursor())
                    .cast_mut();
                strm.avail_in = input_cursor.available();
                strm.avail_out = available_out;
                let result = crate::src::inflate::inflate(
                    strm as *mut crate::zlib_h::z_stream_s,
                    crate::zlib_h::Z_NO_FLUSH,
                );
                let input =
                    GzCodecInput::from_owned_buffer(input, strm.next_in.addr(), strm.avail_in)?;
                // `inflate()` owns every diagnostic it publishes through
                // `strm.msg`. Match that known static storage by address rather
                // than dereferencing the ABI pointer. The safe loop receives only
                // the selected byte slice.
                let data_error_message = if result == crate::zlib_h::Z_DATA_ERROR {
                    Some(
                        crate::src::inflate::INFLATE_ERROR_MESSAGES
                            .iter()
                            .find(|known| known.as_ptr().cast::<::core::ffi::c_char>() == strm.msg)
                            .map(|known| &known[..known.len() - 1])
                            .unwrap_or(b"compressed data error"),
                    )
                } else {
                    None
                };
                Some(GzInflateResult {
                    result,
                    input,
                    output_available: strm.avail_out,
                    data_error_message,
                })
            },
        )
    };
    state.x.have = decomp.written() as ::core::ffi::c_uint;
    state.x.next = output_start;
    let input = decomp.input();
    let Some(buffer) = state.in_0.as_deref_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    state.strm.next_in = buffer.as_mut_ptr().wrapping_add(input.cursor());
    state.strm.avail_in = input.available();
    let (junk, eof, how) = decomp.fields();
    state.junk = junk;
    state.eof = eof;
    state.how = how;
    result
}

unsafe fn gz_fetch(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let result = gz_fetch_loop(
        GzFetchState::new(state.how, state.x.have, state.eof, state.strm.avail_in),
        |action| {
            match action {
                GzFetchAction::Look => {
                    if gz_look(state) == -1 as ::core::ffi::c_int {
                        return Err(());
                    }
                }
                GzFetchAction::Copy => {
                    let (ret, have) = match gz_copy_load(GzCopyLoadState {
                        output: &mut state.out,
                        fd: state.fd.as_ref().unwrap(),
                        target: GzLoadTarget {
                            again: &mut state.again,
                            eof: &mut state.eof,
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            path: state.path.as_deref(),
                        },
                    }) {
                        Ok(have) => (0, have),
                        Err(have) => (-1, have),
                    };
                    state.x.have = have;
                    if ret == -1 as ::core::ffi::c_int {
                        return Err(());
                    }
                    state.x.next = state.out.as_deref_mut().unwrap().as_mut_ptr();
                }
                GzFetchAction::Gzip => {
                    if gz_decomp(state) == -1 as ::core::ffi::c_int {
                        return Err(());
                    }
                }
                GzFetchAction::Corrupt => {
                    crate::src::gzlib::gz_set_error(
                        &mut state.msg,
                        &mut state.err,
                        &mut state.x.have,
                        state.again,
                        state.path.as_deref(),
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(b"state corrupt"),
                    );
                    return Err(());
                }
            }
            Ok(GzFetchState::new(
                state.how,
                state.x.have,
                state.eof,
                state.strm.avail_in,
            ))
        },
    );
    if result.is_ok() {
        0
    } else {
        -1
    }
}

unsafe fn gz_skip(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    loop {
        let cursor = if state.x.have == 0 {
            0
        } else {
            let Some(cursor) = state.out.as_deref().and_then(|buffer| {
                crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                    buffer,
                    state.x.next.addr(),
                    state.x.have,
                )
                .and_then(|cursor| cursor.advance(0))
                .map(|(next, _)| next)
            }) else {
                return -1 as ::core::ffi::c_int;
            };
            cursor
        };
        let mut skip = GzSkipState {
            buffer: state.out.as_deref(),
            cursor,
            have: state.x.have,
            pos: state.x.pos,
            skip: state.skip,
            eof: state.eof,
            avail_in: state.strm.avail_in,
        };
        match gz_skip_step(&mut skip) {
            Ok(GzSkipStep::Fetch) => {
                if gz_fetch(state) == -1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
            Ok(step @ (GzSkipStep::Advanced | GzSkipStep::Done)) => {
                if skip.have != 0 {
                    let Some(buffer) = state.out.as_deref() else {
                        return -1 as ::core::ffi::c_int;
                    };
                    state.x.next = buffer.as_ptr().wrapping_add(skip.cursor).cast_mut();
                }
                state.x.have = skip.have;
                state.x.pos = skip.pos;
                state.skip = skip.skip;
                if matches!(step, GzSkipStep::Done) {
                    break;
                }
            }
            Err(()) => return -1 as ::core::ffi::c_int,
        }
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_read(
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [u8],
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = output.len() as crate::stdlib::z_size_t;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    's_140: loop {
        n = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
        if n as crate::stdlib::z_size_t > len {
            n = len as ::core::ffi::c_uint;
        }
        's_28: {
            if state.x.have != 0 {
                if state.x.have < n {
                    n = state.x.have;
                }
                // `x.next` is a cursor in the owned output buffer whenever
                // `x.have` is nonzero here. Rebuild the buffered input with
                // a checked range so a corrupt cursor cannot extend a raw
                // slice beyond that allocation.
                let Some((input, next)) = state.out.as_deref().and_then(|buffer| {
                    crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                        buffer,
                        state.x.next.addr(),
                        state.x.have,
                    )
                    .and_then(|cursor| cursor.consume(n as usize))
                }) else {
                    return got;
                };
                let Some(destination) = output.get_mut(got as usize..got as usize + n as usize)
                else {
                    return got;
                };
                copy_buffered_input(input, destination);
                let Some(buffer) = state.out.as_deref() else {
                    return got;
                };
                state.x.next = buffer.as_ptr().wrapping_add(next).cast_mut();
                state.x.have = state.x.have.wrapping_sub(n);
                if state.err != crate::zlib_h::Z_OK {
                    err = -1 as ::core::ffi::c_int;
                }
            } else {
                if state.eof != 0 && state.strm.avail_in == 0 as crate::stdlib::uInt {
                    break 's_140;
                }
                if state.how == crate::gzguts_h::LOOK || n < state.size << 1 as ::core::ffi::c_int {
                    if gz_fetch(state) == -1 as ::core::ffi::c_int
                        && state.x.have == 0 as ::core::ffi::c_uint
                    {
                        err = -1 as ::core::ffi::c_int;
                    }
                    break 's_28;
                } else if state.how == crate::gzguts_h::COPY {
                    let Some(destination) = output.get_mut(got as usize..got as usize + n as usize)
                    else {
                        return got;
                    };
                    match gz_copy_load_into(
                        state.fd.as_ref().unwrap(),
                        destination,
                        GzLoadTarget {
                            again: &mut state.again,
                            eof: &mut state.eof,
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            path: state.path.as_deref(),
                        },
                    ) {
                        Ok(have) => n = have,
                        Err(have) => {
                            n = have;
                            err = -1;
                        }
                    }
                } else {
                    let Some(destination) = output.get_mut(got as usize..got as usize + n as usize)
                    else {
                        return got;
                    };
                    state.strm.avail_out = n as crate::stdlib::uInt;
                    state.strm.next_out = destination.as_mut_ptr();
                    err = gz_decomp(state);
                    n = state.x.have;
                    state.x.have = 0 as ::core::ffi::c_uint;
                }
            }
            len = len.wrapping_sub(n as crate::stdlib::z_size_t);
            got = got.wrapping_add(n as crate::stdlib::z_size_t);
            state.x.pos += n as crate::stdlib::off64_t;
        }
        if !(len != 0 && err == 0) {
            break;
        }
    }
    if len != 0 && state.eof != 0 {
        state.past = 1 as ::core::ffi::c_int;
    }
    return got;
}
unsafe fn gzread(state: &mut crate::gzguts_h::gz_state, output: &mut [u8]) -> ::core::ffi::c_int {
    let request = GzReadRequest::new(state.mode, state.err, state.again);
    let mut error = crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    };
    if !request.begin(&mut error) {
        return -1 as ::core::ffi::c_int;
    }
    if (output.len() as ::core::ffi::c_uint as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        error.set(
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in an int"),
        );
        return -1 as ::core::ffi::c_int;
    }
    drop(error);
    let len = gz_read(state, output) as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as ::core::ffi::c_int;
        }
        if state.again != 0 {
            let errno_value = errno::errno().0;
            let message = errno::Errno(errno_value).to_string();
            crate::src::gzlib::gz_set_error(
                &mut state.msg,
                &mut state.err,
                &mut state.x.have,
                state.again,
                state.path.as_deref(),
                crate::zlib_h::Z_ERRNO,
                Some(message.as_bytes()),
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    return len as ::core::ffi::c_int;
}
#[export_name = "gzread"]

pub unsafe extern "C" fn gzread_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidp,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let output = if len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as usize)
    };
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as ::core::ffi::c_int;
    };
    gzread(state.as_mut(), output)
}
unsafe fn gzfread(
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [u8],
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut len: crate::stdlib::z_size_t = 0;
    let request = GzReadRequest::new(state.mode, state.err, state.again);
    let mut error = crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    };
    if !request.begin(&mut error) {
        return 0 as crate::stdlib::z_size_t;
    }
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        error.set(
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in a size_t"),
        );
        return 0 as crate::stdlib::z_size_t;
    }
    drop(error);
    return if len != 0 {
        gz_read(state, output).wrapping_div(size)
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
    let output = match size.checked_mul(nitems) {
        Some(0) | None => &mut [],
        Some(len) => ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len),
    };
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return 0 as crate::stdlib::z_size_t;
    };
    gzfread(state.as_mut(), output, size, nitems)
}
unsafe fn gzgetc(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let request = GzReadRequest::new(state.mode, state.err, state.again);
    let mut error = crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    };
    if !request.begin(&mut error) {
        return -1 as ::core::ffi::c_int;
    }
    drop(error);
    if state.x.have != 0 {
        // Convert the ABI cursor once at this boundary.  The cursor view
        // checks the complete advertised unread range before the safe read
        // policy consumes its first byte.
        let Some((byte, next)) = state.out.as_deref().and_then(|buffer| {
            crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                buffer,
                state.x.next.addr(),
                state.x.have,
            )
            .and_then(|cursor| cursor.consume_one())
        }) else {
            return -1 as ::core::ffi::c_int;
        };
        let Some(buffer) = state.out.as_deref() else {
            return -1 as ::core::ffi::c_int;
        };
        state.x.have = state.x.have.wrapping_sub(1);
        state.x.pos += 1;
        state.x.next = buffer.as_ptr().wrapping_add(next).cast_mut();
        return byte as ::core::ffi::c_int;
    }
    return if gz_read(state, &mut buf) < 1 as crate::stdlib::z_size_t {
        -1 as ::core::ffi::c_int
    } else {
        buf[0 as usize] as ::core::ffi::c_int
    };
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzgetc(state)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzgetc(state)
}
unsafe fn gzungetc(
    mut c: ::core::ffi::c_int,
    state: &mut crate::gzguts_h::gz_state,
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ {
        return -1 as ::core::ffi::c_int;
    }
    if state.how == crate::gzguts_h::LOOK && state.x.have == 0 as ::core::ffi::c_uint {
        gz_look(state);
    }
    let request = GzReadRequest::new(state.mode, state.err, state.again);
    let mut error = crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    };
    if !request.begin(&mut error) {
        return -1 as ::core::ffi::c_int;
    }
    drop(error);
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.x.have != 0 && state.x.have == state.size << 1 as ::core::ffi::c_int {
        crate::src::gzlib::gz_set_error(
            &mut state.msg,
            &mut state.err,
            &mut state.x.have,
            state.again,
            state.path.as_deref(),
            crate::zlib_h::Z_DATA_ERROR,
            Some(b"out of room to push characters"),
        );
        return -1 as ::core::ffi::c_int;
    }
    let size = state.size as usize;
    let Some(capacity) = size.checked_mul(2) else {
        return -1 as ::core::ffi::c_int;
    };
    let buffer = &mut state.out.as_deref_mut().unwrap()[..capacity];
    let Some((next, have)) = crate::src::gzlib::GzBufferedCursor::prepend(
        buffer,
        state.x.next.addr(),
        state.x.have,
        c as ::core::ffi::c_uchar,
    ) else {
        return -1 as ::core::ffi::c_int;
    };
    state.x.have = have;
    state.x.next = buffer.as_mut_ptr().wrapping_add(next);
    state.x.pos -= 1;
    state.past = 0 as ::core::ffi::c_int;
    return c;
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzungetc(c, state)
}
unsafe fn gzgets(
    mut state: Option<::core::ptr::NonNull<crate::gzguts_h::gz_state>>,
    output: &mut [u8],
) -> *mut ::core::ffi::c_char {
    if output.is_empty() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let Some(mut state) = state else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let state = state.as_mut();
    let policy = GzReadPolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
    };
    if !policy.accepts_read() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    }
    .clear();
    if state.skip != 0 && gz_skip(state) == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let str = output.as_mut_ptr().cast::<::core::ffi::c_char>();
    let mut left = output.len() - 1;
    let mut written = 0;
    if left != 0 {
        while !(state.x.have == 0 as ::core::ffi::c_uint
            && gz_fetch(state) == -1 as ::core::ffi::c_int)
        {
            if state.x.have == 0 as ::core::ffi::c_uint {
                state.past = 1 as ::core::ffi::c_int;
                break;
            } else {
                let mut n = if state.x.have as usize > left {
                    left
                } else {
                    state.x.have as usize
                };
                // `x.next` is a cursor in the owned output buffer whenever
                // `x.have` is nonzero. Validate the complete advertised
                // unread range before borrowing its requested prefix, so a
                // corrupt cursor or `x.have` cannot extend a raw slice past
                // the allocation's remaining capacity.
                let Some((input, _)) = state.out.as_deref().and_then(|buffer| {
                    crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                        buffer,
                        state.x.next.addr(),
                        state.x.have,
                    )
                    .and_then(|cursor| cursor.consume(n))
                }) else {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                };
                let (copied, found_newline) =
                    copy_through_newline(input, &mut output[written..written + n]);
                n = copied;
                state.x.have = state.x.have.wrapping_sub(n as ::core::ffi::c_uint);
                state.x.next = state.x.next.wrapping_add(n);
                state.x.pos += n as crate::stdlib::off64_t;
                left -= n;
                written += n;
                if !(left != 0 && !found_newline) {
                    break;
                }
            }
        }
    }
    if written == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    output[written] = 0;
    return str;
}
#[export_name = "gzgets"]

pub unsafe extern "C" fn gzgets_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if file.is_null() || buf.is_null() || len < 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let output = ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as usize);
    gzgets(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        output,
    )
}
unsafe fn gzdirect(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.x.have == 0 as ::core::ffi::c_uint
    {
        gz_look(state);
    }
    return (state.direct == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return 0 as ::core::ffi::c_int;
    };
    gzdirect(state.as_mut())
}
pub unsafe fn gzclose_r(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if state.mode != crate::gzguts_h::GZ_READ {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.size != 0 {
        crate::src::inflate::inflateEnd(
            &raw mut state.strm as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
        state.out = None;
        state.in_0 = None;
    }
    err = if state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_clear_error(&mut state.msg, &mut state.err);
    state.path = None;
    state.msg = None;
    ret = match state.fd.take() {
        Some(fd) => unsafe {
            rustix::io::try_close(<rustix::fd::OwnedFd as rustix::fd::IntoRawFd>::into_raw_fd(
                fd,
            ))
        }
        .map(|()| 0 as ::core::ffi::c_int)
        .unwrap_or(-1 as ::core::ffi::c_int),
        None => -1 as ::core::ffi::c_int,
    };
    return if ret != 0 {
        crate::zlib_h::Z_ERRNO
    } else {
        err
    };
}
#[export_name = "gzclose_r"]

pub unsafe extern "C" fn gzclose_r_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    crate::src::gzclose::gzclose(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        crate::src::gzclose::GzCloseTarget::Read,
    )
}
