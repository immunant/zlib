pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZIP;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::LOOK;
pub use crate::src::gzlib::gz_intmax;
pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::internal_state;
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

use crate::src::gzlib::{
    GzCodecInput, GzCodecResult, GzEmbeddedInflateCall,
};

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

// This is the complete pointer-free projection needed by LOOK, COPY, and
// GZIP fetches.  The public gzip handle and its ABI cursor remain at the
// callers that construct this view; the fetch loop itself cannot reach the
// embedded `z_stream` or `gzFile_s::next`.
struct GzFetchOwner<'a> {
    buffers: &'a mut crate::gzguts_h::GzBuffers,
    want: ::core::ffi::c_uint,
    direct: &'a mut ::core::ffi::c_int,
    junk: &'a mut ::core::ffi::c_int,
    how: &'a mut ::core::ffi::c_int,
    again: &'a mut ::core::ffi::c_int,
    eof: &'a mut ::core::ffi::c_int,
    err: &'a mut ::core::ffi::c_int,
    message: &'a mut Option<Box<[u8]>>,
    have: &'a mut ::core::ffi::c_uint,
    fd: &'a rustix::fd::OwnedFd,
    path: Option<&'a [u8]>,
    codec_available_input: &'a mut crate::stdlib::uInt,
    codec_available_output: &'a mut crate::stdlib::uInt,
    codec_total_in: &'a mut crate::stdlib::uLong,
    codec_total_out: &'a mut crate::stdlib::uLong,
}

impl<'a> GzFetchOwner<'a> {
    fn new(
        buffers: &'a mut crate::gzguts_h::GzBuffers,
        want: ::core::ffi::c_uint,
        direct: &'a mut ::core::ffi::c_int,
        junk: &'a mut ::core::ffi::c_int,
        how: &'a mut ::core::ffi::c_int,
        again: &'a mut ::core::ffi::c_int,
        eof: &'a mut ::core::ffi::c_int,
        err: &'a mut ::core::ffi::c_int,
        message: &'a mut Option<Box<[u8]>>,
        have: &'a mut ::core::ffi::c_uint,
        fd: &'a rustix::fd::OwnedFd,
        path: Option<&'a [u8]>,
        codec_available_input: &'a mut crate::stdlib::uInt,
        codec_available_output: &'a mut crate::stdlib::uInt,
        codec_total_in: &'a mut crate::stdlib::uLong,
        codec_total_out: &'a mut crate::stdlib::uLong,
    ) -> Self {
        Self {
            buffers,
            want,
            direct,
            junk,
            how,
            again,
            eof,
            err,
            message,
            have,
            fd,
            path,
            codec_available_input,
            codec_available_output,
            codec_total_in,
            codec_total_out,
        }
    }
}

enum GzFetchAction {
    Look,
    Copy,
    Gzip,
    Corrupt,
}

// `gzdirect()` has one observable state-machine transition: an unread LOOK
// stream must classify its input before reporting whether it is direct. Keep
// that selection scalar-only so the eventual gzip owner can make it without
// exposing the ABI-shaped state or its cursor.
struct GzDirectState {
    mode: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
}

enum GzDirectAction {
    Look,
    Report,
}

fn gz_direct_action(state: GzDirectState) -> GzDirectAction {
    if state.mode == crate::gzguts_h::GZ_READ
        && state.how == crate::gzguts_h::LOOK
        && state.have == 0
    {
        GzDirectAction::Look
    } else {
        GzDirectAction::Report
    }
}

fn gz_direct_result(direct: ::core::ffi::c_int) -> ::core::ffi::c_int {
    (direct == 1) as ::core::ffi::c_int
}

// `gzdirect()` needs the same bounded fetch capability as LOOK, but none of
// the ABI handle's cursor fields.  Keeping that capability in this facade
// lets the direct-query transition remain a safe implementation operation.
struct GzDirectOwner<'a> {
    state: GzDirectState,
    fetch: GzFetchOwner<'a>,
}

// `gzdirect()` has no independent ABI work once its possible LOOK transition
// is represented by the pointer-free fetch owner.  The ABI projection that
// constructs this owner remains at its caller.
fn gzdirect(mut owner: GzDirectOwner<'_>) -> ::core::ffi::c_int {
    match gz_direct_action(owner.state) {
        GzDirectAction::Look => {
            let _ = gz_look(&mut owner.fetch);
            gz_direct_result(*owner.fetch.direct)
        }
        GzDirectAction::Report => gz_direct_result(*owner.fetch.direct),
    }
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
fn gz_fetch(
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

// `gzgets()` owns every read-side field it needs between fetches.  The ABI
// cursor is rebuilt only while the adapter dispatches `gz_fetch()` and after
// this facade has finished, so text reads themselves work entirely with
// checked indices and owned buffers.
// The buffered byte APIs share this complete pointer-free read facade.  The
// ABI state only lends it the buffer transaction and scalar snapshot around a
// fetch or direct codec dispatch; copying, skipping, and caller-output
// progress never need `gzFile_s::next` or the embedded `z_stream`.
struct GzReadState {
    buffers: crate::gzguts_h::GzBuffers,
    have: crate::stdlib::uInt,
    pos: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
    how: ::core::ffi::c_int,
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
}

enum GzReadAction {
    Fetch,
    Copy,
    Decompress,
}

struct GzReadStep {
    count: crate::stdlib::uInt,
    failed: bool,
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
    mut decomp: crate::src::gzlib::GzDecompState,
    state: &mut GzDecompLoopState<'_>,
    output: &mut [u8],
    mut inflate: impl FnMut(GzEmbeddedInflateCall<'_, '_>) -> Option<GzCodecResult>,
) -> crate::src::gzlib::GzDecompFinish {
    let mut result = crate::zlib_h::Z_OK;
    loop {
        if decomp.needs_input() && state.refill(&mut decomp).is_err() {
            result = *state.err;
            break;
        }
        if decomp.needs_input() {
            if *state.again == 0 {
                state.set_error(crate::zlib_h::Z_BUF_ERROR, b"unexpected end of file");
            }
            break;
        }
        let Some(call) = state.input.as_deref().and_then(|input| {
            decomp
                .embedded_inflate_call(input, &mut *output)
                .and_then(&mut inflate)
        }) else {
            result = -1;
            break;
        };
        result = call.result;
        decomp.record_input(&call.input);
        match decomp.record_inflate(&call) {
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

fn gz_look(owner: &mut GzFetchOwner<'_>) -> ::core::ffi::c_int {
    if owner.buffers.size == 0 as ::core::ffi::c_uint {
        let Some(buffers) = crate::gzguts_h::GzBuffers::allocate_read(owner.want) else {
            crate::src::gzlib::GzErrorState {
                message: owner.message,
                error: owner.err,
                buffered: owner.have,
                again: *owner.again,
                path: owner.path,
            }
            .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
            return -1 as ::core::ffi::c_int;
        };
        *owner.buffers = buffers;
        *owner.codec_available_input = 0;
        owner.buffers.inflate_state = Some(
            crate::src::gzlib::GzEmbeddedInflateState::from_stream_fields(
                0,
                0,
                0,
                0,
            ),
        );
    }
    // This reset is intentionally before refill: opening a normal gzip read
    // starts with `junk == 0`, and the original ordering resets the embedded
    // codec before it attempts any file I/O.  Both this path and a detected
    // gzip header perform the exact same reset, so carry the action to one
    // dispatch site below rather than duplicating the codec boundary.
    let action = if *owner.direct == -1 || *owner.junk == 0 {
        let action = gz_look_step(GzLookState {
            direct: owner.direct,
            junk: owner.junk,
            how: owner.how,
            again: *owner.again,
            input: &[],
            output: None,
        });
        if !matches!(action, Ok(GzLookAction::ResetGzip)) {
            return -1;
        }
        action
    } else {
        let Some(mut input_cursor) = owner.buffers.input_cursor.take() else {
            return -1 as ::core::ffi::c_int;
        };
        let refill = gz_avail(GzAvailState {
            err: owner.err,
            eof: owner.eof,
            input_cursor: &mut input_cursor,
            size: owner.buffers.size as usize,
            input: &mut owner.buffers.input,
            fd: owner.fd,
            again: owner.again,
            message: owner.message,
            buffered: owner.have,
            path: owner.path,
        });
        owner.buffers.input_cursor = Some(input_cursor);
        if refill.is_none() {
            return -1 as ::core::ffi::c_int;
        }
        let input_cursor = owner.buffers.input_cursor.as_ref().unwrap();
        *owner.codec_available_input = input_cursor.available();
        // The successful refill above leaves a checked owner cursor.  Retain that
        // index through copy detection instead of rebuilding a view from the ABI
        // stream pointer that is published only for the subsequent codec call.
        let Some(input) = owner
            .buffers
            .input
            .as_deref()
            .and_then(|buffer| input_cursor.bytes(buffer))
        else {
            return -1 as ::core::ffi::c_int;
        };
        gz_look_step(GzLookState {
            direct: owner.direct,
            junk: owner.junk,
            how: owner.how,
            again: *owner.again,
            input,
            output: owner.buffers.output.as_deref_mut(),
        })
    };
    match action {
        Ok(GzLookAction::ResetGzip) => {
            if let Some(inflate) = owner.buffers.inflate_state.as_mut() {
                inflate.update(crate::src::gzlib::GzCodecCounters::from_stream_fields(
                    *owner.codec_available_input,
                    *owner.codec_available_output,
                    *owner.codec_total_in,
                    *owner.codec_total_out,
                ));
                inflate.reset();
                let counters = inflate.counters();
                *owner.codec_available_input = counters.available_input();
                *owner.codec_available_output = counters.available_output();
                *owner.codec_total_in = counters.total_in();
                *owner.codec_total_out = counters.total_out();
            }
            0
        }
        Ok(GzLookAction::NeedInput) => 0,
        Ok(GzLookAction::Copy { have }) => {
            let Some(output) = owner.buffers.output.as_deref_mut() else {
                return -1;
            };
            let Some(cursor) = crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(
                output,
                0,
                have as ::core::ffi::c_uint,
            ) else {
                return -1;
            };
            *owner.have = have as ::core::ffi::c_uint;
            owner.buffers.set_output_cursor(cursor);
            *owner.codec_available_input = 0;
            owner.buffers.input_cursor = Some(GzCodecInput::empty());
            0
        }
        Err(()) => -1,
    }
}

fn gz_decomp(owner: &mut GzFetchOwner<'_>) -> ::core::ffi::c_int {
    let output_len = (owner.buffers.size << 1 as ::core::ffi::c_int) as usize;
    // The loop receives the owned output allocation itself.  Each embedded
    // request narrows it to the checked prefix available to this inflate
    // pass, so this boundary no longer publishes an output cursor before a
    // request exists.
    let Some(output) = owner.buffers.output.as_deref_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    let Some(input) = owner.buffers.input_cursor.as_ref() else {
        return -1 as ::core::ffi::c_int;
    };
    let Some(mut decomp) = crate::src::gzlib::GzDecompState::new(
        output_len,
        input,
        owner
            .buffers
            .inflate_state
            .as_ref()
            .map(crate::src::gzlib::GzEmbeddedInflateState::counters)
            .unwrap_or_else(|| crate::src::gzlib::GzCodecCounters::from_stream_fields(
                *owner.codec_available_input,
                *owner.codec_available_output,
                *owner.codec_total_in,
                *owner.codec_total_out,
            )),
        *owner.junk,
        *owner.eof,
        *owner.how,
    ) else {
        return -1 as ::core::ffi::c_int;
    };
    let finish = {
        let mut loop_state = GzDecompLoopState {
            err: owner.err,
            eof: owner.eof,
            size: owner.buffers.size as usize,
            input: &mut owner.buffers.input,
            fd: owner.fd,
            again: owner.again,
            message: owner.message,
            buffered: owner.have,
            path: owner.path,
        };
        let Some(inflate) = owner.buffers.inflate_state.as_mut() else {
            return -1;
        };
        gz_decomp_loop(decomp, &mut loop_state, output, |call| inflate.inflate(call))
    };
    // The core transition returns the checked start of its owned output span,
    // not the ABI cursor that `inflate()` advanced. Rebuild that cursor only
    // while publishing the completed result back to the handle.
    let output_have = finish.output.have();
    // Keep the completed, bounds-checked cursor with the output allocation.
    // `x.next` is still the ABI publication for current callers; later read
    // transitions can consume this owned cursor instead of revalidating it
    // from that raw pointer.
    owner.buffers.set_output_cursor(finish.output);
    *owner.have = output_have;
    owner.buffers.input_cursor = Some(finish.input);
    *owner.codec_available_input = finish.codec.available_input();
    *owner.codec_available_output = finish.codec.available_output();
    *owner.codec_total_in = finish.codec.total_in();
    *owner.codec_total_out = finish.codec.total_out();
    if let Some(inflate) = owner.buffers.inflate_state.as_mut() {
        inflate.update(finish.codec);
    }
    *owner.junk = finish.junk;
    *owner.eof = finish.eof;
    *owner.how = finish.how;
    finish.result
}

fn gz_fetch_from_state(owner: &mut GzFetchOwner<'_>) -> ::core::ffi::c_int {
    let result = gz_fetch(
        GzFetchState::new(
            *owner.how,
            *owner.have,
            *owner.eof,
            *owner.codec_available_input,
        ),
        |action| {
            match action {
                GzFetchAction::Look => {
                    if gz_look(owner) == -1 as ::core::ffi::c_int {
                        return Err(());
                    }
                }
                GzFetchAction::Copy => {
                    let (ret, have) = match gz_copy_load(GzCopyLoadState {
                        output: &mut owner.buffers.output,
                        fd: owner.fd,
                        target: GzLoadTarget {
                            again: owner.again,
                            eof: owner.eof,
                            message: owner.message,
                            error: owner.err,
                            buffered: owner.have,
                            path: owner.path,
                        },
                    }) {
                        Ok(have) => (0, have),
                        Err(have) => (-1, have),
                    };
                    *owner.have = have;
                    if ret == -1 as ::core::ffi::c_int {
                        return Err(());
                    }
                    let Some(output) = owner.buffers.output.as_deref_mut() else {
                        return Err(());
                    };
                    let Some(cursor) =
                        crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(output, 0, have)
                    else {
                        return Err(());
                    };
                    owner.buffers.set_output_cursor(cursor);
                }
                GzFetchAction::Gzip => {
                    if gz_decomp(owner) == -1 as ::core::ffi::c_int {
                        return Err(());
                    }
                }
                GzFetchAction::Corrupt => {
                    crate::src::gzlib::gz_set_error(
                        owner.message,
                        owner.err,
                        owner.have,
                        *owner.again,
                        owner.path,
                        crate::zlib_h::Z_STREAM_ERROR,
                        Some(b"state corrupt"),
                    );
                    return Err(());
                }
            }
            Ok(GzFetchState::new(
                *owner.how,
                *owner.have,
                *owner.eof,
                *owner.codec_available_input,
            ))
        },
    );
    if result.is_ok() {
        0
    } else {
        -1
    }
}

// The byte-copy/skip state machine works entirely over the pointer-free read
// owner.  The ABI state is projected only by its callers when they must drive
// the embedded codec.
fn gz_read(
    state: &mut GzReadState,
    output: &mut [u8],
    mut dispatch: impl FnMut(GzReadAction, &mut GzReadState, &mut [u8]) -> GzReadStep,
) -> crate::stdlib::z_size_t {
    let mut got: crate::stdlib::z_size_t = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut len = output.len() as crate::stdlib::z_size_t;
    if len == 0 as crate::stdlib::z_size_t {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 {
        loop {
            let cursor = if state.have == 0 {
                0
            } else {
                let Some(cursor) = state.buffers.output_cursor().map(|cursor| cursor.start())
                else {
                    return 0;
                };
                cursor
            };
            let mut skip = GzSkipState {
                buffer: state.buffers.output.as_deref(),
                cursor,
                have: state.have,
                pos: state.pos,
                skip: state.skip,
                eof: state.eof,
                avail_in: state.avail_in,
            };
            match gz_skip_step(&mut skip) {
                Ok(GzSkipStep::Fetch) => {
                    if dispatch(GzReadAction::Fetch, state, &mut []).failed {
                        return 0;
                    }
                }
                Ok(step @ (GzSkipStep::Advanced | GzSkipStep::Done)) => {
                    let have = skip.have;
                    let cursor_index = skip.cursor;
                    let pos = skip.pos;
                    let skip_remaining = skip.skip;
                    drop(skip);
                    if have != 0 {
                        let Some(buffer) = state.buffers.output.as_deref() else {
                            return 0;
                        };
                        let Some(cursor) =
                            crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(
                                buffer,
                                cursor_index,
                                have,
                            )
                        else {
                            return 0;
                        };
                        state.buffers.set_output_cursor(cursor);
                    } else {
                        state.buffers.clear_output_cursor();
                    }
                    state.have = have;
                    state.pos = pos;
                    state.skip = skip_remaining;
                    if matches!(step, GzSkipStep::Done) {
                        break;
                    }
                }
                Err(()) => return 0,
            }
        }
    }
    got = 0 as crate::stdlib::z_size_t;
    err = 0 as ::core::ffi::c_int;
    's_140: loop {
        n = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
        if n as crate::stdlib::z_size_t > len {
            n = len as ::core::ffi::c_uint;
        }
        if state.have != 0 {
            if state.have < n {
                n = state.have;
            }
            let Some(destination) = output.get_mut(got as usize..got as usize + n as usize) else {
                return got;
            };
            // The output allocation retains the checked cursor created
            // by LOOK/COPY/inflate. Consume that owner cursor rather
            // than rebuilding a range from the ABI publication.
            let Some(next_cursor) = (|| {
                let buffer = state.buffers.output.as_deref()?;
                let cursor = state.buffers.output_cursor()?;
                let buffered = cursor.buffered(buffer)?;
                buffered.copy_into(destination)?;
                cursor.advance(n as usize)
            })() else {
                return got;
            };
            state.have = next_cursor.have();
            state.buffers.set_output_cursor(next_cursor);
            if state.err != crate::zlib_h::Z_OK {
                err = -1 as ::core::ffi::c_int;
            }
        } else {
            if state.eof != 0 && state.avail_in == 0 as crate::stdlib::uInt {
                break 's_140;
            }
            if state.how == crate::gzguts_h::LOOK
                || n < state.buffers.size << 1 as ::core::ffi::c_int
            {
                if dispatch(GzReadAction::Fetch, state, &mut []).failed
                    && state.have == 0 as ::core::ffi::c_uint
                {
                    err = -1 as ::core::ffi::c_int;
                }
                if err != 0 {
                    break 's_140;
                }
                continue 's_140;
            } else if state.how == crate::gzguts_h::COPY {
                let Some(destination) = output.get_mut(got as usize..got as usize + n as usize)
                else {
                    return got;
                };
                let step = dispatch(GzReadAction::Copy, state, destination);
                n = step.count;
                err = -(step.failed as ::core::ffi::c_int);
            } else {
                let Some(destination) = output.get_mut(got as usize..got as usize + n as usize)
                else {
                    return got;
                };
                let step = dispatch(GzReadAction::Decompress, state, destination);
                err = -(step.failed as ::core::ffi::c_int);
                n = step.count;
                state.have = 0 as ::core::ffi::c_uint;
                state.buffers.clear_output_cursor();
            }
        }
        len = len.wrapping_sub(n as crate::stdlib::z_size_t);
        got = got.wrapping_add(n as crate::stdlib::z_size_t);
        state.pos += n as crate::stdlib::off64_t;
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
    let mut read = GzReadState {
        buffers: ::core::mem::replace(&mut state.buffers, crate::gzguts_h::GzBuffers::empty()),
        have: state.x.have,
        pos: state.x.pos,
        skip: state.skip,
        how: state.how,
        eof: state.eof,
        past: state.past,
        err: state.err,
        avail_in: state.strm.avail_in,
    };
    let len = gz_read(&mut read, output, |action, read, destination| {
        state.buffers =
            ::core::mem::replace(&mut read.buffers, crate::gzguts_h::GzBuffers::empty());
        state.x.have = read.have;
        state.x.pos = read.pos;
        state.skip = read.skip;
        state.how = read.how;
        state.eof = read.eof;
        state.past = read.past;
        state.err = read.err;
        state.strm.avail_in = read.avail_in;
        let cursor_is_valid = match state
            .buffers
            .output_cursor()
            .map(|cursor| (cursor.start(), cursor.have()))
        {
            Some((start, have)) if have == state.x.have => state
                .buffers
                .output
                .as_deref_mut()
                .and_then(|buffer| buffer.get_mut(start..))
                .map(|buffer| state.x.next = buffer.as_mut_ptr())
                .is_some(),
            None if state.x.have == 0 => {
                state.x.next = ::core::ptr::null_mut();
                true
            }
            _ => false,
        };
        let step = if !cursor_is_valid {
            GzReadStep {
                count: 0,
                failed: true,
            }
        } else {
            match action {
                GzReadAction::Fetch => GzReadStep {
                    count: 0,
                    failed: gz_fetch_from_state(&mut GzFetchOwner::new(
                        &mut state.buffers,
                        state.want,
                        &mut state.direct,
                        &mut state.junk,
                        &mut state.how,
                        &mut state.again,
                        &mut state.eof,
                        &mut state.err,
                        &mut state.msg,
                        &mut state.x.have,
                        state.fd.as_ref().expect("gzip state has an open file"),
                        state.path.as_deref(),
                        &mut state.strm.avail_in,
                        &mut state.strm.avail_out,
                        &mut state.strm.total_in,
                        &mut state.strm.total_out,
                    )) == -1,
                },
                GzReadAction::Copy => match gz_copy_load_into(
                    state.fd.as_ref().expect("gzip state has an open file"),
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
                    Ok(count) => GzReadStep {
                        count,
                        failed: false,
                    },
                    Err(count) => GzReadStep {
                        count,
                        failed: true,
                    },
                },
                GzReadAction::Decompress => {
                    GzReadStep {
                        count: state.x.have,
                        failed: gz_decomp(&mut GzFetchOwner::new(
                            &mut state.buffers,
                            state.want,
                            &mut state.direct,
                            &mut state.junk,
                            &mut state.how,
                            &mut state.again,
                            &mut state.eof,
                            &mut state.err,
                            &mut state.msg,
                            &mut state.x.have,
                            state.fd.as_ref().expect("gzip state has an open file"),
                            state.path.as_deref(),
                            &mut state.strm.avail_in,
                            &mut state.strm.avail_out,
                            &mut state.strm.total_in,
                            &mut state.strm.total_out,
                        )) == -1,
                    }
                }
            }
        };
        read.buffers =
            ::core::mem::replace(&mut state.buffers, crate::gzguts_h::GzBuffers::empty());
        read.have = state.x.have;
        read.pos = state.x.pos;
        read.skip = state.skip;
        read.how = state.how;
        read.eof = state.eof;
        read.past = state.past;
        read.err = state.err;
        read.avail_in = state.strm.avail_in;
        step
    }) as ::core::ffi::c_uint;
    state.buffers = ::core::mem::replace(&mut read.buffers, crate::gzguts_h::GzBuffers::empty());
    state.x.have = read.have;
    state.x.pos = read.pos;
    state.skip = read.skip;
    state.how = read.how;
    state.eof = read.eof;
    state.past = read.past;
    state.err = read.err;
    state.strm.avail_in = read.avail_in;
    match state
        .buffers
        .output_cursor()
        .map(|cursor| (cursor.start(), cursor.have()))
    {
        Some((start, have)) if have == state.x.have => {
            let Some(buffer) = state.buffers.output.as_deref_mut() else {
                return 0;
            };
            let Some(buffer) = buffer.get_mut(start..) else {
                return 0;
            };
            state.x.next = buffer.as_mut_ptr();
        }
        None if state.x.have == 0 => state.x.next = ::core::ptr::null_mut(),
        _ => return 0,
    }
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
        gzread(state, output).max(0) as crate::stdlib::z_size_t / size
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
        let Some((byte, next_cursor)) = (|| {
            let buffer = state.buffers.output.as_deref()?;
            let cursor = state.buffers.output_cursor()?;
            let (byte, _) = cursor.buffered(buffer)?.consume_one()?;
            Some((byte, cursor.advance(1)?))
        })() else {
            return -1 as ::core::ffi::c_int;
        };
        let Some(buffer) = state.buffers.output.as_deref() else {
            return -1 as ::core::ffi::c_int;
        };
        state.x.have = next_cursor.have();
        state.x.pos += 1;
        state.x.next = buffer.as_ptr().wrapping_add(next_cursor.start()).cast_mut();
        state.buffers.set_output_cursor(next_cursor);
        return byte as ::core::ffi::c_int;
    }
    return if gzread(state, &mut buf) < 1 as ::core::ffi::c_int {
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
        gz_look(&mut GzFetchOwner::new(
            &mut state.buffers,
            state.want,
            &mut state.direct,
            &mut state.junk,
            &mut state.how,
            &mut state.again,
            &mut state.eof,
            &mut state.err,
            &mut state.msg,
            &mut state.x.have,
            state.fd.as_ref().expect("gzip state has an open file"),
            state.path.as_deref(),
            &mut state.strm.avail_in,
            &mut state.strm.avail_out,
            &mut state.strm.total_in,
            &mut state.strm.total_out,
        ));
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
    if state.skip != 0 {
        loop {
            let cursor = if state.x.have == 0 {
                0
            } else {
                let Some(cursor) = state.buffers.output_cursor().map(|cursor| cursor.start())
                else {
                    return -1;
                };
                cursor
            };
            let mut skip = GzSkipState {
                buffer: state.buffers.output.as_deref(),
                cursor,
                have: state.x.have,
                pos: state.x.pos,
                skip: state.skip,
                eof: state.eof,
                avail_in: state.strm.avail_in,
            };
            match gz_skip_step(&mut skip) {
                Ok(GzSkipStep::Fetch) => {
                    if gz_fetch_from_state(&mut GzFetchOwner::new(
                        &mut state.buffers,
                        state.want,
                        &mut state.direct,
                        &mut state.junk,
                        &mut state.how,
                        &mut state.again,
                        &mut state.eof,
                        &mut state.err,
                        &mut state.msg,
                        &mut state.x.have,
                        state.fd.as_ref().expect("gzip state has an open file"),
                        state.path.as_deref(),
                        &mut state.strm.avail_in,
                        &mut state.strm.avail_out,
                        &mut state.strm.total_in,
                        &mut state.strm.total_out,
                    )) == -1 as ::core::ffi::c_int {
                        return -1;
                    }
                }
                Ok(step @ (GzSkipStep::Advanced | GzSkipStep::Done)) => {
                    let have = skip.have;
                    let cursor_index = skip.cursor;
                    let pos = skip.pos;
                    let skip_remaining = skip.skip;
                    drop(skip);
                    if have != 0 {
                        let Some(buffer) = state.buffers.output.as_deref() else {
                            return -1;
                        };
                        let Some(cursor) =
                            crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(
                                buffer,
                                cursor_index,
                                have,
                            )
                        else {
                            return -1;
                        };
                        state.x.next = buffer.as_ptr().wrapping_add(cursor_index).cast_mut();
                        state.buffers.set_output_cursor(cursor);
                    } else {
                        state.buffers.clear_output_cursor();
                    }
                    state.x.have = have;
                    state.x.pos = pos;
                    state.skip = skip_remaining;
                    if matches!(step, GzSkipStep::Done) {
                        break;
                    }
                }
                Err(()) => return -1,
            }
        }
    }
    if c < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if state.x.have != 0 && state.x.have == state.buffers.size << 1 as ::core::ffi::c_int {
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
    let size = state.buffers.size as usize;
    let Some(capacity) = size.checked_mul(2) else {
        return -1 as ::core::ffi::c_int;
    };
    let existing_cursor = state
        .buffers
        .output_cursor()
        .map(|cursor| (cursor.start(), cursor.have()));
    let Some(buffer) = state.buffers.output.as_deref_mut() else {
        return -1;
    };
    let Some(buffer) = buffer.get_mut(..capacity) else {
        return -1;
    };
    let Some(mut cursor) = existing_cursor
        .map(|(start, have)| {
            crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(buffer, start, have)
        })
        .unwrap_or_else(|| crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(buffer, 0, 0))
    else {
        return -1;
    };
    if cursor.prepend(buffer, c as ::core::ffi::c_uchar).is_none() {
        return -1 as ::core::ffi::c_int;
    }
    state.x.have = cursor.have();
    state.x.next = buffer.as_mut_ptr().wrapping_add(cursor.start());
    state.buffers.set_output_cursor(cursor);
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
fn gzgets(
    state: &mut GzReadState,
    output: &mut [u8],
    mut fetch: impl FnMut(&mut GzReadState) -> Result<(), ()>,
) -> bool {
    if output.is_empty() {
        return false;
    }
    if state.skip != 0 {
        loop {
            let cursor = if state.have == 0 {
                0
            } else {
                let Some(cursor) = state.buffers.output_cursor().map(|cursor| cursor.start())
                else {
                    return false;
                };
                cursor
            };
            let mut skip = GzSkipState {
                buffer: state.buffers.output.as_deref(),
                cursor,
                have: state.have,
                pos: state.pos,
                skip: state.skip,
                eof: state.eof,
                avail_in: state.avail_in,
            };
            match gz_skip_step(&mut skip) {
                Ok(GzSkipStep::Fetch) => {
                    if fetch(state).is_err() {
                        return false;
                    }
                }
                Ok(step @ (GzSkipStep::Advanced | GzSkipStep::Done)) => {
                    let have = skip.have;
                    let cursor_index = skip.cursor;
                    let pos = skip.pos;
                    let skip_remaining = skip.skip;
                    drop(skip);
                    if have != 0 {
                        let Some(buffer) = state.buffers.output.as_deref() else {
                            return false;
                        };
                        let Some(cursor) =
                            crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(
                                buffer,
                                cursor_index,
                                have,
                            )
                        else {
                            return false;
                        };
                        state.buffers.set_output_cursor(cursor);
                    } else {
                        state.buffers.clear_output_cursor();
                    }
                    state.have = have;
                    state.pos = pos;
                    state.skip = skip_remaining;
                    if matches!(step, GzSkipStep::Done) {
                        break;
                    }
                }
                Err(()) => return false,
            }
        }
    }
    let mut left = output.len() - 1;
    let mut written = 0;
    if left != 0 {
        loop {
            if state.have == 0 && fetch(state).is_err() {
                break;
            }
            if state.have == 0 {
                state.past = 1 as ::core::ffi::c_int;
                break;
            } else {
                let mut n = if state.have as usize > left {
                    left
                } else {
                    state.have as usize
                };
                let Some((copied, found_newline, next_cursor)) = (|| {
                    let buffer = state.buffers.output.as_deref()?;
                    let cursor = state.buffers.output_cursor()?;
                    let (input, _) = cursor.buffered(buffer)?.consume(n)?;
                    let (copied, found_newline) =
                        copy_through_newline(input, &mut output[written..written + n]);
                    Some((copied, found_newline, cursor.advance(copied)?))
                })() else {
                    return false;
                };
                n = copied;
                state.have = next_cursor.have();
                state.buffers.set_output_cursor(next_cursor);
                state.pos += n as crate::stdlib::off64_t;
                left -= n;
                written += n;
                if !(left != 0 && !found_newline) {
                    break;
                }
            }
        }
    }
    if written == 0 {
        return false;
    }
    output[written] = 0;
    true
}

// This is the only bridge between the pointer-free `gzgets` facade and the
// existing embedded-codec state.  It transfers the owned buffer transaction
// to the ABI state for one fetch, then snapshots the scalar result back into
// the facade before the core resumes.
unsafe fn gzgets_from_state(
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [u8],
) -> *mut ::core::ffi::c_char {
    if output.is_empty() {
        return ::core::ptr::null_mut();
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
        return ::core::ptr::null_mut();
    }
    drop(error);
    let mut read = GzReadState {
        buffers: ::core::mem::replace(&mut state.buffers, crate::gzguts_h::GzBuffers::empty()),
        have: state.x.have,
        pos: state.x.pos,
        skip: state.skip,
        how: state.how,
        eof: state.eof,
        past: state.past,
        err: state.err,
        avail_in: state.strm.avail_in,
    };
    let result = gzgets(&mut read, output, |read| {
        state.buffers =
            ::core::mem::replace(&mut read.buffers, crate::gzguts_h::GzBuffers::empty());
        state.x.have = read.have;
        state.x.pos = read.pos;
        state.skip = read.skip;
        state.how = read.how;
        state.eof = read.eof;
        state.past = read.past;
        state.err = read.err;
        state.strm.avail_in = read.avail_in;
        let cursor = state
            .buffers
            .output_cursor()
            .map(|cursor| (cursor.start(), cursor.have()));
        match cursor {
            Some((start, have)) if have == state.x.have => {
                let Some(output) = state.buffers.output.as_deref_mut() else {
                    read.buffers = ::core::mem::replace(
                        &mut state.buffers,
                        crate::gzguts_h::GzBuffers::empty(),
                    );
                    return Err(());
                };
                let Some(output) = output.get_mut(start..) else {
                    read.buffers = ::core::mem::replace(
                        &mut state.buffers,
                        crate::gzguts_h::GzBuffers::empty(),
                    );
                    return Err(());
                };
                state.x.next = output.as_mut_ptr();
            }
            None if state.x.have == 0 => state.x.next = ::core::ptr::null_mut(),
            _ => {
                read.buffers =
                    ::core::mem::replace(&mut state.buffers, crate::gzguts_h::GzBuffers::empty());
                return Err(());
            }
        }
        let fetched = gz_fetch_from_state(&mut GzFetchOwner::new(
            &mut state.buffers,
            state.want,
            &mut state.direct,
            &mut state.junk,
            &mut state.how,
            &mut state.again,
            &mut state.eof,
            &mut state.err,
            &mut state.msg,
            &mut state.x.have,
            state.fd.as_ref().expect("gzip state has an open file"),
            state.path.as_deref(),
            &mut state.strm.avail_in,
            &mut state.strm.avail_out,
            &mut state.strm.total_in,
            &mut state.strm.total_out,
        ));
        read.buffers =
            ::core::mem::replace(&mut state.buffers, crate::gzguts_h::GzBuffers::empty());
        read.have = state.x.have;
        read.pos = state.x.pos;
        read.skip = state.skip;
        read.how = state.how;
        read.eof = state.eof;
        read.past = state.past;
        read.err = state.err;
        read.avail_in = state.strm.avail_in;
        if fetched == -1 {
            Err(())
        } else {
            Ok(())
        }
    });
    state.buffers = ::core::mem::replace(&mut read.buffers, crate::gzguts_h::GzBuffers::empty());
    state.x.have = read.have;
    state.x.pos = read.pos;
    state.skip = read.skip;
    state.how = read.how;
    state.eof = read.eof;
    state.past = read.past;
    state.err = read.err;
    state.strm.avail_in = read.avail_in;
    let cursor = state
        .buffers
        .output_cursor()
        .map(|cursor| (cursor.start(), cursor.have()));
    match cursor {
        Some((start, have)) if have == state.x.have => {
            let Some(output) = state.buffers.output.as_deref_mut() else {
                return ::core::ptr::null_mut();
            };
            let Some(output) = output.get_mut(start..) else {
                return ::core::ptr::null_mut();
            };
            state.x.next = output.as_mut_ptr();
        }
        None if state.x.have == 0 => state.x.next = ::core::ptr::null_mut(),
        _ => return ::core::ptr::null_mut(),
    }
    if result {
        output.as_mut_ptr().cast()
    } else {
        ::core::ptr::null_mut()
    }
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
    // The opaque handle is checked and borrowed at the ABI boundary.  The
    // implementation receives only that validated state plus the bounded
    // caller output, so its read/LOOK transitions remain out of this wrapper.
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let output = ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as usize);
    gzgets_from_state(state, output)
}
fn gzdirect_from_state(owner: GzDirectOwner<'_>) -> ::core::ffi::c_int {
    gzdirect(owner)
}

// Keep the ABI-shaped gzip handle at this projection boundary.  Once its
// disjoint scalar and owned-buffer fields have been borrowed, the direct
// query itself receives only the pointer-free owner above.
unsafe fn gzdirect_from_abi_state(
    state: &mut crate::gzguts_h::gz_state,
) -> ::core::ffi::c_int {
    let crate::gzguts_h::gz_state {
        x,
        mode,
        fd,
        path,
        want,
        buffers,
        direct,
        junk,
        how,
        again,
        eof,
        err,
        msg,
        strm,
        ..
    } = state;
    gzdirect_from_state(GzDirectOwner {
        state: GzDirectState {
            mode: *mode,
            how: *how,
            have: x.have,
        },
        fetch: GzFetchOwner::new(
            buffers,
            *want,
            direct,
            junk,
            how,
            again,
            eof,
            err,
            msg,
            &mut x.have,
            fd.as_ref().expect("gzip state has an open file"),
            path.as_deref(),
            &mut strm.avail_in,
            &mut strm.avail_out,
            &mut strm.total_in,
            &mut strm.total_out,
        ),
    })
}
#[export_name = "gzdirect"]

pub unsafe extern "C" fn gzdirect_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return 0 as ::core::ffi::c_int;
    };
    gzdirect_from_abi_state(state.as_mut())
}
pub unsafe fn gzclose_r(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if state.mode != crate::gzguts_h::GZ_READ {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.buffers.size != 0 {
        // The read-side inflater is owned by `GzBuffers`; dropping that
        // pointer-free owner is its complete teardown.  It was never
        // published through the embedded ABI stream.
        state.buffers.inflate_state = None;
        state.buffers.output = None;
        state.buffers.input = None;
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
        Some(fd) => crate::src::gzlib::gz_close_fd(fd)
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
