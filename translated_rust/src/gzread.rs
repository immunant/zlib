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

use crate::src::gzlib::{GzCodecInput, GzCodecResult, GzEmbeddedInflateCall};

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

// This is the complete resource owner for one gzip read operation.  It owns
// the descriptor, path, paired buffers, embedded inflate owner, and every
// scalar the read loop can change.  The ABI `gzFile_s` prefix and embedded
// `z_stream` are deliberately absent: their `have`/`pos` and counter mirrors
// are imported and republished only by the existing opaque-handle boundary.
//
// Keeping the complete item-read resource here is important: `gzfread()`
// differs from `gzread()` only in request overflow and item-count policy, not
// in how it owns or advances gzip state.  Later byte APIs can therefore reuse
// this owner without rebuilding an ABI-shaped dispatch facade.
struct GzReadOwner {
    mode: ::core::ffi::c_int,
    fd: rustix::fd::OwnedFd,
    path: Option<Box<[u8]>>,
    want: ::core::ffi::c_uint,
    buffers: crate::gzguts_h::GzBuffers,
    direct: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    err: ::core::ffi::c_int,
    message: Option<Box<[u8]>>,
    have: crate::stdlib::uInt,
    pos: crate::stdlib::off64_t,
    avail_in: crate::stdlib::uInt,
    avail_out: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
}

// Importing a gzip read transaction from its opaque ABI state is deliberately
// split into a pointer-free snapshot and the outer publication boundary.  The
// snapshot owns every resource that a complete read may change; in
// particular, it carries the descriptor and buffers together so a later
// request kind cannot accidentally leave either behind in the ABI state.
//
// `gzFile_s::next` is intentionally absent.  That cursor is an ABI mirror of
// the owned output cursor and must be rebuilt only after the transaction has
// returned its buffers to the opaque state.
struct GzReadOwnerSnapshot {
    mode: ::core::ffi::c_int,
    fd: rustix::fd::OwnedFd,
    path: Option<Box<[u8]>>,
    want: ::core::ffi::c_uint,
    buffers: crate::gzguts_h::GzBuffers,
    direct: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    err: ::core::ffi::c_int,
    message: Option<Box<[u8]>>,
    have: crate::stdlib::uInt,
    pos: crate::stdlib::off64_t,
    avail_in: crate::stdlib::uInt,
    avail_out: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
}

impl GzReadOwnerSnapshot {
    fn into_owner(self) -> GzReadOwner {
        GzReadOwner {
            mode: self.mode,
            fd: self.fd,
            path: self.path,
            want: self.want,
            buffers: self.buffers,
            direct: self.direct,
            junk: self.junk,
            how: self.how,
            again: self.again,
            eof: self.eof,
            past: self.past,
            skip: self.skip,
            err: self.err,
            message: self.message,
            have: self.have,
            pos: self.pos,
            avail_in: self.avail_in,
            avail_out: self.avail_out,
            total_in: self.total_in,
            total_out: self.total_out,
        }
    }
}

// The public `gzFile_s::next` mirror is published after a bounded read but
// before gzlib converts a zero-byte EAGAIN result into its stored error.  Keep
// that ordering explicit without giving the safe owner an ABI cursor.
enum GzFreadOutcome {
    Complete(crate::stdlib::z_size_t),
    PublishThenFinish {
        read_len: ::core::ffi::c_uint,
        size: crate::stdlib::z_size_t,
    },
}

// Both exported byte-read variants enter through the same opaque-handle
// boundary.  The request and result tags keep their distinct C return
// policies out of the FFI wrappers without giving either pointer-free core
// an ABI-shaped state.
enum GzReadAbiRequest {
    Bytes,
    Gets,
    Unget(::core::ffi::c_int),
    Items {
        size: crate::stdlib::z_size_t,
        nitems: crate::stdlib::z_size_t,
    },
}

enum GzReadAbiResult {
    Bytes(::core::ffi::c_int),
    Gets(bool),
    Unget(::core::ffi::c_int),
    Items(crate::stdlib::z_size_t),
}

impl GzReadAbiResult {
    fn bytes(self) -> ::core::ffi::c_int {
        match self {
            Self::Bytes(result) => result,
            Self::Gets(_) | Self::Unget(_) | Self::Items(_) => unreachable!(),
        }
    }

    fn items(self) -> crate::stdlib::z_size_t {
        match self {
            Self::Items(result) => result,
            Self::Bytes(_) | Self::Gets(_) | Self::Unget(_) => unreachable!(),
        }
    }

    // `gzgetc()` is the one-byte form of the item-read transaction.  Keep
    // its return conversion in this pointer-free result owner instead of
    // reintroducing a second opaque-state adapter just to expose the byte.
    fn getc(self, byte: ::core::ffi::c_uchar) -> ::core::ffi::c_int {
        match self {
            Self::Items(1) => byte as ::core::ffi::c_int,
            Self::Items(_) => -1,
            Self::Bytes(_) | Self::Gets(_) | Self::Unget(_) => unreachable!(),
        }
    }

    fn unget(self) -> ::core::ffi::c_int {
        match self {
            Self::Unget(result) => result,
            Self::Bytes(_) | Self::Gets(_) | Self::Items(_) => unreachable!(),
        }
    }

    fn gets(self) -> bool {
        match self {
            Self::Gets(result) => result,
            Self::Bytes(_) | Self::Unget(_) | Self::Items(_) => unreachable!(),
        }
    }
}

// This is the persistent half of the read action boundary.  It contains all
// fields an action may update, but deliberately excludes `gzFile_s::next` and
// the embedded stream's raw cursors.  The outer ABI adapter publishes those
// only after the bounded read has finished, while individual Fetch/COPY/GZIP
// actions can exchange this pointer-free owner with `GzReadState`.
struct GzReadDispatch<'a> {
    buffers: &'a mut crate::gzguts_h::GzBuffers,
    have: &'a mut crate::stdlib::uInt,
    pos: &'a mut crate::stdlib::off64_t,
    skip: &'a mut crate::stdlib::off64_t,
    how: &'a mut ::core::ffi::c_int,
    eof: &'a mut ::core::ffi::c_int,
    past: &'a mut ::core::ffi::c_int,
    err: &'a mut ::core::ffi::c_int,
    want: ::core::ffi::c_uint,
    direct: &'a mut ::core::ffi::c_int,
    junk: &'a mut ::core::ffi::c_int,
    again: &'a mut ::core::ffi::c_int,
    message: &'a mut Option<Box<[u8]>>,
    fd: Option<&'a rustix::fd::OwnedFd>,
    path: Option<&'a [u8]>,
    avail_in: &'a mut crate::stdlib::uInt,
    avail_out: &'a mut crate::stdlib::uInt,
    total_in: &'a mut crate::stdlib::uLong,
    total_out: &'a mut crate::stdlib::uLong,
}

impl GzReadDispatch<'_> {
    fn begin(&mut self, request: &GzReadRequest, read: &mut GzReadState) -> bool {
        let mut error = crate::src::gzlib::GzErrorState {
            message: &mut *self.message,
            error: &mut read.err,
            buffered: &mut read.have,
            again: *self.again,
            path: self.path,
        };
        request.begin(&mut error)
    }

    // Move the owned buffer transaction and its scalar snapshot into the
    // action facade.  No ABI pointer is consulted to validate the output
    // cursor: the owned cursor is the authoritative proof of that range.
    fn project_from_read(&mut self, read: &mut GzReadState) {
        *self.buffers =
            ::core::mem::replace(&mut read.buffers, crate::gzguts_h::GzBuffers::empty());
        *self.have = read.have;
        *self.pos = read.pos;
        *self.skip = read.skip;
        *self.how = read.how;
        *self.eof = read.eof;
        *self.past = read.past;
        *self.err = read.err;
        *self.avail_in = read.avail_in;
    }

    // Return the post-action transaction to the pointer-free state machine.
    fn snapshot_into_read(&mut self, read: &mut GzReadState) {
        read.buffers = ::core::mem::replace(self.buffers, crate::gzguts_h::GzBuffers::empty());
        read.have = *self.have;
        read.pos = *self.pos;
        read.skip = *self.skip;
        read.how = *self.how;
        read.eof = *self.eof;
        read.past = *self.past;
        read.err = *self.err;
        read.avail_in = *self.avail_in;
    }

    fn output_cursor_is_valid(&mut self) -> bool {
        match self
            .buffers
            .output_cursor()
            .map(|cursor| (cursor.start(), cursor.have()))
        {
            Some((start, have)) if have == *self.have => self
                .buffers
                .output
                .as_deref()
                .and_then(|buffer| buffer.get(start..))
                .is_some(),
            None if *self.have == 0 => true,
            _ => false,
        }
    }

    fn dispatch(
        &mut self,
        action: GzReadAction,
        read: &mut GzReadState,
        destination: &mut [u8],
    ) -> GzReadStep {
        self.project_from_read(read);
        let step = if !self.output_cursor_is_valid() || self.fd.is_none() {
            GzReadStep {
                count: 0,
                failed: true,
            }
        } else {
            let fd = self.fd.expect("checked gzip descriptor");
            match action {
                GzReadAction::Look => GzReadStep {
                    count: 0,
                    failed: gz_look(&mut GzFetchOwner::new(
                        self.buffers,
                        self.want,
                        self.direct,
                        self.junk,
                        self.how,
                        self.again,
                        self.eof,
                        self.err,
                        self.message,
                        self.have,
                        fd,
                        self.path,
                        self.avail_in,
                        self.avail_out,
                        self.total_in,
                        self.total_out,
                    )) == -1,
                },
                GzReadAction::Fetch => GzReadStep {
                    count: 0,
                    failed: gz_fetch_from_state(&mut GzFetchOwner::new(
                        self.buffers,
                        self.want,
                        self.direct,
                        self.junk,
                        self.how,
                        self.again,
                        self.eof,
                        self.err,
                        self.message,
                        self.have,
                        fd,
                        self.path,
                        self.avail_in,
                        self.avail_out,
                        self.total_in,
                        self.total_out,
                    )) == -1,
                },
                GzReadAction::Copy => match gz_copy_load_into(
                    fd,
                    destination,
                    GzLoadTarget {
                        again: self.again,
                        eof: self.eof,
                        message: self.message,
                        error: self.err,
                        buffered: self.have,
                        path: self.path,
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
                GzReadAction::Decompress => GzReadStep {
                    count: *self.have,
                    failed: gz_decomp(&mut GzFetchOwner::new(
                        self.buffers,
                        self.want,
                        self.direct,
                        self.junk,
                        self.how,
                        self.again,
                        self.eof,
                        self.err,
                        self.message,
                        self.have,
                        fd,
                        self.path,
                        self.avail_in,
                        self.avail_out,
                        self.total_in,
                        self.total_out,
                    )) == -1,
                },
            }
        };
        self.snapshot_into_read(read);
        step
    }
}

enum GzReadAction {
    Look,
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
        owner.buffers.inflate_state =
            Some(crate::src::gzlib::GzEmbeddedInflateState::from_stream_fields(0, 0, 0, 0));
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
            .unwrap_or_else(|| {
                crate::src::gzlib::GzCodecCounters::from_stream_fields(
                    *owner.codec_available_input,
                    *owner.codec_available_output,
                    *owner.codec_total_in,
                    *owner.codec_total_out,
                )
            }),
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
        gz_decomp_loop(decomp, &mut loop_state, output, |call| {
            inflate.inflate(call)
        })
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
fn gzread(
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

// A complete read request runs over the pointer-free read owner and its
// pointer-free action dispatcher.  The ABI adapters construct and republish
// those views around this transaction; keeping the loop here lets byte and
// item reads share it without one adapter calling another.
fn gzread_with_dispatch(
    read: &mut GzReadState,
    output: &mut [u8],
    dispatch: &mut GzReadDispatch<'_>,
) -> ::core::ffi::c_uint {
    let len = gzread(read, output, |action, read, destination| {
        dispatch.dispatch(action, read, destination)
    });
    dispatch.project_from_read(read);
    len as ::core::ffi::c_uint
}

// The ABI-shaped state is projected exactly once for either exported read
// request.  Keep that projection under this established adapter; `gzread()`
// and `gzfread()` themselves are pointer-free owner loops once it completes.
unsafe fn gzread_from_state(
    state: &mut crate::gzguts_h::gz_state,
    output: &mut [u8],
    request: GzReadAbiRequest,
) -> GzReadAbiResult {
    if let GzReadAbiRequest::Items { size, nitems } = request {
        let Some(fd) = state.fd.take() else {
            return GzReadAbiResult::Items(0);
        };
        let mut owner = GzReadOwnerSnapshot {
            mode: state.mode,
            fd,
            path: state.path.take(),
            want: state.want,
            buffers: ::core::mem::replace(&mut state.buffers, crate::gzguts_h::GzBuffers::empty()),
            direct: state.direct,
            junk: state.junk,
            how: state.how,
            again: state.again,
            eof: state.eof,
            past: state.past,
            skip: state.skip,
            err: state.err,
            message: state.msg.take(),
            have: state.x.have,
            pos: state.x.pos,
            avail_in: state.strm.avail_in,
            avail_out: state.strm.avail_out,
            total_in: state.strm.total_in,
            total_out: state.strm.total_out,
        }
        .into_owner();
        let outcome = gzfread(&mut owner, output, size, nitems);
        let result = match outcome {
            GzFreadOutcome::Complete(result) => result,
            GzFreadOutcome::PublishThenFinish { read_len, size } => {
                let published = match owner
                    .buffers
                    .output_cursor()
                    .map(|cursor| (cursor.start(), cursor.have()))
                {
                    Some((start, have)) if have == owner.have => owner
                        .buffers
                        .output
                        .as_deref_mut()
                        .and_then(|buffer| buffer.get_mut(start..))
                        .map(|buffer| {
                            state.x.next = buffer.as_mut_ptr();
                        })
                        .is_some(),
                    None if owner.have == 0 => {
                        state.x.next = ::core::ptr::null_mut();
                        true
                    }
                    _ => false,
                };
                if !published {
                    0
                } else {
                    gzfread_finish(&mut owner, read_len, size)
                }
            }
        };
        state.mode = owner.mode;
        state.fd = Some(owner.fd);
        state.path = owner.path;
        state.want = owner.want;
        state.buffers = owner.buffers;
        state.direct = owner.direct;
        state.junk = owner.junk;
        state.how = owner.how;
        state.again = owner.again;
        state.eof = owner.eof;
        state.past = owner.past;
        state.skip = owner.skip;
        state.err = owner.err;
        state.msg = owner.message;
        state.x.have = owner.have;
        state.x.pos = owner.pos;
        state.strm.avail_in = owner.avail_in;
        state.strm.avail_out = owner.avail_out;
        state.strm.total_in = owner.total_in;
        state.strm.total_out = owner.total_out;
        return GzReadAbiResult::Items(result);
    }
    if let GzReadAbiRequest::Unget(c) = request {
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
        let mode = state.mode;
        let again = state.again;
        let path = state.path.as_deref();
        let mut dispatch = GzReadDispatch {
            buffers: &mut state.buffers,
            have: &mut state.x.have,
            pos: &mut state.x.pos,
            skip: &mut state.skip,
            how: &mut state.how,
            eof: &mut state.eof,
            past: &mut state.past,
            err: &mut state.err,
            want: state.want,
            direct: &mut state.direct,
            junk: &mut state.junk,
            again: &mut state.again,
            message: &mut state.msg,
            fd: state.fd.as_ref(),
            path,
            avail_in: &mut state.strm.avail_in,
            avail_out: &mut state.strm.avail_out,
            total_in: &mut state.strm.total_in,
            total_out: &mut state.strm.total_out,
        };
        let outcome = match gzungetc_begin(&mut read, mode, again, |action, read| {
            !dispatch.dispatch(action, read, &mut []).failed
        }) {
            Some(request) => {
                let accepted = dispatch.begin(&request, &mut read);
                if accepted {
                    gzungetc(c, &mut read, |action, read| {
                        !dispatch.dispatch(action, read, &mut []).failed
                    })
                } else {
                    GzUngetOutcome::Result(-1)
                }
            }
            None => GzUngetOutcome::Result(-1),
        };
        dispatch.project_from_read(&mut read);
        drop(dispatch);
        state.x.have = read.have;
        state.x.pos = read.pos;
        state.skip = read.skip;
        state.how = read.how;
        state.eof = read.eof;
        state.past = read.past;
        state.err = read.err;
        state.strm.avail_in = read.avail_in;
        let published = match state
            .buffers
            .output_cursor()
            .map(|cursor| (cursor.start(), cursor.have()))
        {
            Some((start, have)) if have == state.x.have => state
                .buffers
                .output
                .as_deref_mut()
                .and_then(|buffer| buffer.get_mut(start..))
                .map(|buffer| {
                    state.x.next = buffer.as_mut_ptr();
                })
                .is_some(),
            None if state.x.have == 0 => {
                state.x.next = ::core::ptr::null_mut();
                true
            }
            _ => false,
        };
        if !published {
            return GzReadAbiResult::Unget(-1);
        }
        return GzReadAbiResult::Unget(match outcome {
            GzUngetOutcome::Result(result) => result,
            GzUngetOutcome::OutOfRoom => {
                crate::src::gzlib::gz_set_error(
                    &mut state.msg,
                    &mut state.err,
                    &mut state.x.have,
                    state.again,
                    state.path.as_deref(),
                    crate::zlib_h::Z_DATA_ERROR,
                    Some(b"out of room to push characters"),
                );
                -1
            }
        });
    }
    if matches!(request, GzReadAbiRequest::Gets) {
        if output.is_empty() {
            return GzReadAbiResult::Gets(false);
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
            return GzReadAbiResult::Gets(false);
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
                    read.buffers = ::core::mem::replace(
                        &mut state.buffers,
                        crate::gzguts_h::GzBuffers::empty(),
                    );
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
        let published = match state
            .buffers
            .output_cursor()
            .map(|cursor| (cursor.start(), cursor.have()))
        {
            Some((start, have)) if have == state.x.have => state
                .buffers
                .output
                .as_deref_mut()
                .and_then(|output| output.get_mut(start..))
                .map(|output| state.x.next = output.as_mut_ptr())
                .is_some(),
            None if state.x.have == 0 => {
                state.x.next = ::core::ptr::null_mut();
                true
            }
            _ => false,
        };
        return GzReadAbiResult::Gets(result && published);
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
        return GzReadAbiResult::Bytes(-1 as ::core::ffi::c_int);
    }
    if (output.len() as ::core::ffi::c_uint as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        error.set(
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in an int"),
        );
        return GzReadAbiResult::Bytes(-1 as ::core::ffi::c_int);
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
    let len = {
        let mut dispatch = GzReadDispatch {
            buffers: &mut state.buffers,
            have: &mut state.x.have,
            pos: &mut state.x.pos,
            skip: &mut state.skip,
            how: &mut state.how,
            eof: &mut state.eof,
            past: &mut state.past,
            err: &mut state.err,
            want: state.want,
            direct: &mut state.direct,
            junk: &mut state.junk,
            again: &mut state.again,
            message: &mut state.msg,
            fd: Some(state.fd.as_ref().expect("gzip state has an open file")),
            path: state.path.as_deref(),
            avail_in: &mut state.strm.avail_in,
            avail_out: &mut state.strm.avail_out,
            total_in: &mut state.strm.total_in,
            total_out: &mut state.strm.total_out,
        };
        gzread_with_dispatch(&mut read, output, &mut dispatch)
    };
    match state
        .buffers
        .output_cursor()
        .map(|cursor| (cursor.start(), cursor.have()))
    {
        Some((start, have)) if have == state.x.have => {
            let Some(buffer) = state.buffers.output.as_deref_mut() else {
                return GzReadAbiResult::Bytes(0);
            };
            let Some(buffer) = buffer.get_mut(start..) else {
                return GzReadAbiResult::Bytes(0);
            };
            state.x.next = buffer.as_mut_ptr();
        }
        None if state.x.have == 0 => state.x.next = ::core::ptr::null_mut(),
        _ => return GzReadAbiResult::Bytes(0),
    }
    if len == 0 as ::core::ffi::c_uint {
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return GzReadAbiResult::Bytes(-1 as ::core::ffi::c_int);
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
            return GzReadAbiResult::Bytes(-1 as ::core::ffi::c_int);
        }
    }
    GzReadAbiResult::Bytes(len as ::core::ffi::c_int)
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
    gzread_from_state(state.as_mut(), output, GzReadAbiRequest::Bytes).bytes()
}
// The actual bounded byte transaction is shared by item and byte getters.
// Its input and dispatch capability are wholly owned, so it never needs an
// ABI cursor while consuming gzip state.
fn gzread_owner(owner: &mut GzReadOwner, output: &mut [u8]) -> ::core::ffi::c_uint {
    let mut read = GzReadState {
        buffers: ::core::mem::replace(&mut owner.buffers, crate::gzguts_h::GzBuffers::empty()),
        have: owner.have,
        pos: owner.pos,
        skip: owner.skip,
        how: owner.how,
        eof: owner.eof,
        past: owner.past,
        err: owner.err,
        avail_in: owner.avail_in,
    };
    {
        let mut dispatch = GzReadDispatch {
            buffers: &mut owner.buffers,
            have: &mut owner.have,
            pos: &mut owner.pos,
            skip: &mut owner.skip,
            how: &mut owner.how,
            eof: &mut owner.eof,
            past: &mut owner.past,
            err: &mut owner.err,
            want: owner.want,
            direct: &mut owner.direct,
            junk: &mut owner.junk,
            again: &mut owner.again,
            message: &mut owner.message,
            fd: Some(&owner.fd),
            path: owner.path.as_deref(),
            avail_in: &mut owner.avail_in,
            avail_out: &mut owner.avail_out,
            total_in: &mut owner.total_in,
            total_out: &mut owner.total_out,
        };
        gzread_with_dispatch(&mut read, output, &mut dispatch)
    }
}

fn gzfread(
    owner: &mut GzReadOwner,
    output: &mut [u8],
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
) -> GzFreadOutcome {
    let mut len: crate::stdlib::z_size_t = 0;
    let request = GzReadRequest::new(owner.mode, owner.err, owner.again);
    let mut error = crate::src::gzlib::GzErrorState {
        message: &mut owner.message,
        error: &mut owner.err,
        buffered: &mut owner.have,
        again: owner.again,
        path: owner.path.as_deref(),
    };
    if !request.begin(&mut error) {
        return GzFreadOutcome::Complete(0);
    }
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        error.set(
            crate::zlib_h::Z_STREAM_ERROR,
            Some(b"request does not fit in a size_t"),
        );
        return GzFreadOutcome::Complete(0);
    }
    drop(error);
    if len == 0 {
        return GzFreadOutcome::Complete(0);
    }

    // The item-read policy now runs over the complete pointer-free gzip
    // resource.  The opaque-handle adapter imports and republishes the ABI
    // cursor separately, so item counts never need an ABI-shaped state.
    let read_len = gzread_owner(owner, output);
    GzFreadOutcome::PublishThenFinish { read_len, size }
}

fn gzfread_finish(owner: &mut GzReadOwner, read_len: ::core::ffi::c_uint, size: usize) -> usize {
    if read_len == 0 {
        if owner.err != crate::zlib_h::Z_OK && owner.err != crate::zlib_h::Z_BUF_ERROR {
            return 0;
        }
        if owner.again != 0 {
            let errno_value = errno::errno().0;
            let message = errno::Errno(errno_value).to_string();
            crate::src::gzlib::gz_set_error(
                &mut owner.message,
                &mut owner.err,
                &mut owner.have,
                owner.again,
                owner.path.as_deref(),
                crate::zlib_h::Z_ERRNO,
                Some(message.as_bytes()),
            );
            return 0;
        }
    }
    read_len as usize / size
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
    gzread_from_state(
        state.as_mut(),
        output,
        GzReadAbiRequest::Items { size, nitems },
    )
    .items()
}
#[export_name = "gzgetc"]

pub unsafe extern "C" fn gzgetc_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    let mut byte = 0;
    gzread_from_state(
        state,
        ::core::slice::from_mut(&mut byte),
        GzReadAbiRequest::Items { size: 1, nitems: 1 },
    )
    .getc(byte)
}
#[export_name = "gzgetc_"]

pub unsafe extern "C" fn gzgetc__ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    let mut byte = 0;
    gzread_from_state(
        state,
        ::core::slice::from_mut(&mut byte),
        GzReadAbiRequest::Items { size: 1, nitems: 1 },
    )
    .getc(byte)
}
// The unget byte policy is wholly owned by the buffered read facade.  LOOK
// classification and refills are requested through the supplied action
// visitor; that visitor is the only part that needs the ABI gzip state.
enum GzUngetOutcome {
    Result(::core::ffi::c_int),
    OutOfRoom,
}

fn gzungetc_begin(
    state: &mut GzReadState,
    mode: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    mut dispatch: impl FnMut(GzReadAction, &mut GzReadState) -> bool,
) -> Option<GzReadRequest> {
    if mode != crate::gzguts_h::GZ_READ {
        return None;
    }
    if state.how == crate::gzguts_h::LOOK && state.have == 0 {
        let _ = dispatch(GzReadAction::Look, state);
    }
    Some(GzReadRequest::new(mode, state.err, again))
}

fn gzungetc(
    c: ::core::ffi::c_int,
    state: &mut GzReadState,
    mut dispatch: impl FnMut(GzReadAction, &mut GzReadState) -> bool,
) -> GzUngetOutcome {
    if state.skip != 0 {
        loop {
            let cursor = if state.have == 0 {
                0
            } else {
                let Some(cursor) = state.buffers.output_cursor().map(|cursor| cursor.start())
                else {
                    return GzUngetOutcome::Result(-1);
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
                    if !dispatch(GzReadAction::Fetch, state) {
                        return GzUngetOutcome::Result(-1);
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
                            return GzUngetOutcome::Result(-1);
                        };
                        let Some(cursor) =
                            crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(
                                buffer,
                                cursor_index,
                                have,
                            )
                        else {
                            return GzUngetOutcome::Result(-1);
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
                Err(()) => return GzUngetOutcome::Result(-1),
            }
        }
    }
    if c < 0 as ::core::ffi::c_int {
        return GzUngetOutcome::Result(-1);
    }
    if state.have != 0 && state.have == state.buffers.size << 1 as ::core::ffi::c_int {
        return GzUngetOutcome::OutOfRoom;
    }
    let size = state.buffers.size as usize;
    let Some(capacity) = size.checked_mul(2) else {
        return GzUngetOutcome::Result(-1);
    };
    let existing_cursor = state
        .buffers
        .output_cursor()
        .map(|cursor| (cursor.start(), cursor.have()));
    let Some(buffer) = state.buffers.output.as_deref_mut() else {
        return GzUngetOutcome::Result(-1);
    };
    let Some(buffer) = buffer.get_mut(..capacity) else {
        return GzUngetOutcome::Result(-1);
    };
    let Some(mut cursor) = existing_cursor
        .map(|(start, have)| {
            crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(buffer, start, have)
        })
        .unwrap_or_else(|| crate::src::gzlib::GzCodecOutputCursor::from_owned_buffer(buffer, 0, 0))
    else {
        return GzUngetOutcome::Result(-1);
    };
    if cursor.prepend(buffer, c as ::core::ffi::c_uchar).is_none() {
        return GzUngetOutcome::Result(-1);
    }
    state.have = cursor.have();
    state.buffers.set_output_cursor(cursor);
    state.pos -= 1;
    state.past = 0 as ::core::ffi::c_int;
    GzUngetOutcome::Result(c)
}
#[export_name = "gzungetc"]

pub unsafe extern "C" fn gzungetc_ffi(
    mut c: ::core::ffi::c_int,
    mut file: crate::zlib_h::gzFile,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzread_from_state(state, &mut [], GzReadAbiRequest::Unget(c)).unget()
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
    // shared read adapter retains the state projection and its LOOK/fetch
    // transitions; this wrapper only supplies the bounded caller output.
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
    let output = ::core::slice::from_raw_parts_mut(buf.cast::<u8>(), len as usize);
    if gzread_from_state(state, output, GzReadAbiRequest::Gets).gets() {
        output.as_mut_ptr().cast()
    } else {
        ::core::ptr::null_mut()
    }
}
fn gzdirect_from_state(owner: GzDirectOwner<'_>) -> ::core::ffi::c_int {
    gzdirect(owner)
}

// Keep the ABI-shaped gzip handle at this projection boundary.  Once its
// disjoint scalar and owned-buffer fields have been borrowed, the direct
// query itself receives only the pointer-free owner above.
unsafe fn gzdirect_from_abi_state(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
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
// The read close path has no need for the ABI cursor prefix or embedded
// stream.  Keep its complete resource transaction over owned buffers,
// descriptor, and error/path storage so the opaque-handle boundary can select
// it without making close policy itself unsafe.
pub(crate) struct GzReadCloseState<'a> {
    pub(crate) mode: ::core::ffi::c_int,
    pub(crate) buffers: &'a mut crate::gzguts_h::GzBuffers,
    pub(crate) err: &'a mut ::core::ffi::c_int,
    pub(crate) msg: &'a mut Option<Box<[u8]>>,
    pub(crate) path: &'a mut Option<Box<[u8]>>,
    pub(crate) fd: &'a mut Option<rustix::fd::OwnedFd>,
}

pub(crate) fn gzclose_r(state: GzReadCloseState<'_>) -> ::core::ffi::c_int {
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
    let err = if *state.err == crate::zlib_h::Z_BUF_ERROR {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
    crate::src::gzlib::gz_clear_error(state.msg, state.err);
    *state.path = None;
    *state.msg = None;
    let ret = match state.fd.take() {
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
