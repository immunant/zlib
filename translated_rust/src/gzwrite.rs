pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::src::gzlib::gz_intmax;
pub use crate::stdlib::EAGAIN;
pub use crate::stdlib::EWOULDBLOCK;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::ssize_t;

pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit2_;
pub use crate::src::deflate::deflate_dispatch_from_abi_stream as deflate;
pub use crate::src::deflate::internal_state;
use crate::src::deflate::DeflateResetKind;
use crate::src::deflate::{deflate_scalar_from_abi_stream, DeflateScalarAction};
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

// Keep the errno-dependent write failure policy separate from the ABI-shaped
// gzip state.  A later FD/write facade can return this pointer-free result
// directly instead of making the state machine inspect errno itself.
struct GzWriteFailure {
    errno_value: ::core::ffi::c_int,
    would_block: bool,
}

// This is the pointer-free portion of the gzip write state that determines
// whether an operation may proceed.  Keep the policy independent from the
// ABI-shaped owner: the eventual gzip-state facade can construct this directly
// and leave all handle conversion at the boundary.
#[derive(Clone, Copy)]
struct GzWritePolicy {
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
}

impl GzWritePolicy {
    fn accepts_write(&self) -> bool {
        self.mode == crate::gzguts_h::GZ_WRITE
            && (self.err == crate::zlib_h::Z_OK || self.again != 0)
    }

    fn accepts_params(&self) -> bool {
        self.accepts_write() && self.direct == 0
    }
}

// Flush has the same codec lifetime as a byte write, but no caller buffer.
// Keep its admission and error-clearing order in a pointer-free action so the
// shared gzip adapter is the only boundary that reaches the embedded deflater.
enum GzFlushPlan {
    Reject,
    ClearAndReject,
    Dispatch { flush: ::core::ffi::c_int },
}

fn gzflush_plan(policy: &GzWritePolicy, flush: ::core::ffi::c_int) -> GzFlushPlan {
    if !policy.accepts_write() {
        GzFlushPlan::Reject
    } else if !(0..=crate::zlib_h::Z_FINISH).contains(&flush) {
        GzFlushPlan::ClearAndReject
    } else {
        GzFlushPlan::Dispatch { flush }
    }
}

// Retuning has a small pointer-free admission phase before it reaches the
// embedded deflater.  In particular, a rejected request must preserve the
// previous error, while an accepted no-op clears it without materializing a
// deferred seek or touching the codec.
enum GzSetParamsPlan {
    Reject,
    NoChange,
    Change { materialize_skip: bool },
}

fn gzsetparams_plan(
    policy: &GzWritePolicy,
    current_level: ::core::ffi::c_int,
    current_strategy: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
) -> GzSetParamsPlan {
    if !policy.accepts_params() {
        GzSetParamsPlan::Reject
    } else if level == current_level && strategy == current_strategy {
        GzSetParamsPlan::NoChange
    } else {
        GzSetParamsPlan::Change {
            materialize_skip: skip != 0,
        }
    }
}

// The zero-fill step may initialize the write buffers, so form this second
// plan only after it has completed.  The existing compressor adapter then
// decides whether it must drain buffered input before applying this scalar
// transition.
struct GzDeflateRetune {
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
}

fn gzsetparams_needs_retune(buffers_size: crate::stdlib::uInt) -> bool {
    buffers_size != 0
}

// Closing a writer has a small, but externally visible, error-precedence
// policy: both pending zero-fill and the final codec flush are attempted, and
// a later descriptor-close failure wins over either codec result.  Keep those
// scalar decisions independent from the embedded stream/resource boundary so
// the eventual gzip resource owner can reuse them without retaining ABI
// pointers.
struct GzWriteCloseResult {
    result: ::core::ffi::c_int,
}

impl GzWriteCloseResult {
    fn begin(mode: ::core::ffi::c_int) -> Option<Self> {
        (mode == crate::gzguts_h::GZ_WRITE).then_some(Self {
            result: crate::zlib_h::Z_OK,
        })
    }

    fn record_codec_result(&mut self, status: ::core::ffi::c_int, error: ::core::ffi::c_int) {
        if status == -1 {
            self.result = error;
        }
    }

    fn finish(self, close_failed: bool) -> ::core::ffi::c_int {
        if close_failed {
            crate::zlib_h::Z_ERRNO
        } else {
            self.result
        }
    }
}

// The close-only compressor request carries neither ABI cursors nor codec
// storage.  It keeps the two codec-status observations and the consumed
// lifecycle proof together while `gz_comp()` performs the entire final
// zero-fill/finish/teardown cluster.
struct GzWriteCloseCodec<'a> {
    result: &'a mut GzWriteCloseResult,
    deflater_closed: &'a mut bool,
}

// This is the pointer-free proof that the embedded deflater was initialized
// and therefore needs exactly one matching teardown.  It is deliberately
// detached from the paired buffer owner before the ABI stream is borrowed for
// `deflateEnd()`: resource release can then consume this lifecycle fact
// without inferring it from allocation state or stream counters.
struct GzEmbeddedDeflaterClose {
    initialized: bool,
}

impl GzEmbeddedDeflaterClose {
    fn take(buffers: &mut crate::gzguts_h::GzBuffers) -> Self {
        Self {
            initialized: buffers.take_embedded_deflater().is_some(),
        }
    }

    fn needs_teardown(&self) -> bool {
        self.initialized
    }
}

// Once the final embedded-deflater request has ended, the remaining writer
// resources have no ABI cursors or callback-backed state.  Move them as one
// owner so close ordering and descriptor-error precedence can eventually be
// shared with a pointer-free gzip resource facade.
struct GzWriteCloseResources {
    buffers: crate::gzguts_h::GzBuffers,
    fd: Option<rustix::fd::OwnedFd>,
    path: Option<Box<[u8]>>,
    message: Option<Box<[u8]>>,
    error: ::core::ffi::c_int,
}

impl GzWriteCloseResources {
    fn take(
        buffers: &mut crate::gzguts_h::GzBuffers,
        fd: &mut Option<rustix::fd::OwnedFd>,
        path: &mut Option<Box<[u8]>>,
        message: &mut Option<Box<[u8]>>,
        error: &mut ::core::ffi::c_int,
    ) -> Self {
        Self {
            buffers: ::core::mem::replace(buffers, crate::gzguts_h::GzBuffers::empty()),
            fd: fd.take(),
            path: path.take(),
            message: message.take(),
            error: ::core::mem::replace(error, crate::zlib_h::Z_OK),
        }
    }

    fn release_write_buffers(&mut self, had_embedded_deflater: bool) {
        if had_embedded_deflater {
            self.buffers.output = None;
        }
        if self.buffers.size != 0 {
            self.buffers.input = None;
        }
    }

    fn finish(mut self) -> bool {
        crate::src::gzlib::gz_clear_error(&mut self.message, &mut self.error);
        self.path = None;
        self.message = None;
        self.fd
            .take()
            .map(crate::src::gzlib::gz_close_fd)
            .transpose()
            .is_err()
    }
}

fn gzwrite_length_fits_int(len: usize) -> bool {
    (len as ::core::ffi::c_uint as ::core::ffi::c_int) >= 0
}

fn gzputs_length_fits_int(len: crate::stdlib::z_size_t) -> bool {
    (len as ::core::ffi::c_int) >= 0 && len as ::core::ffi::c_uint as crate::stdlib::z_size_t == len
}

// The compressed large-write path is bounded by zlib's uInt input cursor,
// independently of the caller slice. Keep that chunk selection pointer-free
// so a future gzip write owner can carry this cursor policy without an ABI
// stream.
struct GzCompressionChunk {
    input_len: crate::stdlib::uInt,
}

impl GzCompressionChunk {
    fn next(remaining: crate::stdlib::z_size_t) -> Self {
        let mut input_len = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
        if input_len as crate::stdlib::z_size_t > remaining {
            input_len = remaining as crate::stdlib::uInt;
        }
        Self { input_len }
    }

    fn consumed(&self, remaining: crate::stdlib::uInt) -> crate::stdlib::uInt {
        self.input_len.wrapping_sub(remaining)
    }
}

// Keep one caller write request as a pointer-free owner while it crosses the
// buffered, direct, and compressed paths.  In particular, partial-write
// accounting comes from the bounded caller slice rather than an ABI stream
// cursor.  The embedded-deflate boundary still consumes individual chunks,
// but it reports progress back into this owner before the next request is
// formed.
struct GzWriteInput<'a> {
    input: &'a [u8],
    total: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
}

// A complete gzip write starts with only caller-owned input.  Keep that
// request pointer-free so admission and byte accounting can be shared by
// every write entry point; the ABI-shaped state adapter below is solely
// responsible for initialization, seek materialization, and codec dispatch.
struct GzWriteTransaction<'a> {
    request: GzWriteInput<'a>,
}

fn gz_write(input: &[u8]) -> Option<GzWriteTransaction<'_>> {
    (!input.is_empty()).then_some(GzWriteTransaction {
        request: GzWriteInput::new(input),
    })
}

impl<'a> GzWriteInput<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            total: input.len() as crate::stdlib::z_size_t,
            remaining: input.len() as crate::stdlib::z_size_t,
        }
    }

    fn remaining(&self) -> &'a [u8] {
        self.input
    }

    fn len(&self) -> crate::stdlib::z_size_t {
        self.remaining
    }

    fn is_empty(&self) -> bool {
        self.remaining == 0
    }

    fn advance(&mut self, count: usize) -> Option<()> {
        self.input = self.input.get(count..)?;
        self.remaining = self
            .remaining
            .checked_sub(count as crate::stdlib::z_size_t)?;
        Some(())
    }

    fn partial_or_zero(&self, again: ::core::ffi::c_int) -> crate::stdlib::z_size_t {
        if again != 0 {
            self.total.wrapping_sub(self.remaining)
        } else {
            0
        }
    }
}

fn gzfwrite_length(
    size: crate::stdlib::z_size_t,
    nitems: crate::stdlib::z_size_t,
) -> Option<crate::stdlib::z_size_t> {
    let len = nitems.wrapping_mul(size);
    (size == 0 || len.wrapping_div(size) == nitems).then_some(len)
}

// All public gzip write entry points differ only in their pointer-free
// admission and return conventions. Keep those conventions together so the
// gzip-state adapter remains the single boundary that reaches the embedded
// deflater.
pub(crate) enum GzWriteFlavor {
    Bytes,
    Items {
        size: crate::stdlib::z_size_t,
        nitems: crate::stdlib::z_size_t,
    },
    Byte {
        value: ::core::ffi::c_int,
    },
    Text,
}

#[derive(Clone, Copy)]
enum GzWriteResult {
    Bytes,
    Items { size: crate::stdlib::z_size_t },
    Byte { value: ::core::ffi::c_int },
    Text { len: crate::stdlib::z_size_t },
}

impl GzWriteResult {
    fn finish(self, written: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
        match self {
            Self::Bytes => written,
            Self::Items { size } => written.wrapping_div(size),
            Self::Byte { value } => {
                if written == 1 {
                    (value & 0xff) as crate::stdlib::z_size_t
                } else {
                    (-1 as ::core::ffi::c_int) as crate::stdlib::z_size_t
                }
            }
            Self::Text { len } => {
                if len != 0 && written == 0 {
                    (-1 as ::core::ffi::c_int) as crate::stdlib::z_size_t
                } else {
                    written
                }
            }
        }
    }
}

enum GzWritePlan<'a> {
    Return(crate::stdlib::z_size_t),
    Dispatch {
        transaction: GzWriteTransaction<'a>,
        result: GzWriteResult,
    },
}

// Every gzip writer request crosses one persistent codec owner. A flush and
// close have no caller input cursor, while a write retains its bounded caller
// slice until the adapter has accounted for it.
pub(crate) enum GzWriteOperation<'a> {
    Write {
        input: &'a [u8],
        flavor: GzWriteFlavor,
    },
    Flush {
        flush: ::core::ffi::c_int,
    },
    SetParams {
        level: ::core::ffi::c_int,
        strategy: ::core::ffi::c_int,
    },
    Close,
}

// Admission reads only scalar gzip state.  Keep that snapshot separate from
// the ABI-shaped state adapter so the later persistent write owner can take
// over operation selection without retaining a `gz_state` reference or any
// of its callback-backed codec fields.
#[derive(Clone, Copy)]
struct GzWriteStateSnapshot {
    policy: GzWritePolicy,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
}

enum GzWriteAdmission<'a> {
    Close,
    Flush(GzFlushPlan),
    SetParams {
        level: ::core::ffi::c_int,
        strategy: ::core::ffi::c_int,
        plan: GzSetParamsPlan,
    },
    Write {
        input: &'a [u8],
        flavor: GzWriteFlavor,
        policy: GzWritePolicy,
    },
}

// This is deliberately limited to selection.  Clearing errors and mutating
// gzip/codec state remain at the existing adapter until the write owner also
// carries the complete embedded-deflater lifecycle.
fn gzip_write_admission<'input>(
    operation: GzWriteOperation<'input>,
    state: GzWriteStateSnapshot,
) -> GzWriteAdmission<'input> {
    match operation {
        GzWriteOperation::Close => GzWriteAdmission::Close,
        GzWriteOperation::Flush { flush } => {
            GzWriteAdmission::Flush(gzflush_plan(&state.policy, flush))
        }
        GzWriteOperation::SetParams { level, strategy } => GzWriteAdmission::SetParams {
            level,
            strategy,
            plan: gzsetparams_plan(
                &state.policy,
                state.level,
                state.strategy,
                level,
                strategy,
                state.skip,
            ),
        },
        GzWriteOperation::Write { input, flavor } => GzWriteAdmission::Write {
            input,
            flavor,
            policy: state.policy,
        },
    }
}

fn gzwrite_plan<'input>(
    input: &'input [u8],
    flavor: GzWriteFlavor,
    policy: GzWritePolicy,
    mut error: crate::src::gzlib::GzErrorState<'_>,
) -> GzWritePlan<'input> {
    if !policy.accepts_write() {
        return GzWritePlan::Return(match flavor {
            GzWriteFlavor::Text => (-1 as ::core::ffi::c_int) as crate::stdlib::z_size_t,
            _ => 0,
        });
    }
    error.clear();

    let result = match flavor {
        GzWriteFlavor::Bytes => {
            if !gzwrite_length_fits_int(input.len()) {
                error.set(
                    crate::zlib_h::Z_DATA_ERROR,
                    Some(b"requested length does not fit in int"),
                );
                return GzWritePlan::Return(0);
            }
            GzWriteResult::Bytes
        }
        GzWriteFlavor::Items { size, nitems } => {
            let Some(len) = gzfwrite_length(size, nitems) else {
                error.set(
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"request does not fit in a size_t"),
                );
                return GzWritePlan::Return(0);
            };
            if len == 0 {
                return GzWritePlan::Return(0);
            }
            GzWriteResult::Items { size }
        }
        GzWriteFlavor::Byte { value } => GzWriteResult::Byte { value },
        GzWriteFlavor::Text => {
            let len = input.len() as crate::stdlib::z_size_t;
            if !gzputs_length_fits_int(len) {
                error.set(
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"string length does not fit in int"),
                );
                return GzWritePlan::Return((-1 as ::core::ffi::c_int) as crate::stdlib::z_size_t);
            }
            if len == 0 {
                return GzWritePlan::Return(0);
            }
            GzWriteResult::Text { len }
        }
    };

    match gz_write(input) {
        Some(transaction) => GzWritePlan::Dispatch {
            transaction,
            result,
        },
        None => GzWritePlan::Return(0),
    }
}

fn gz_write_failure(errno_value: ::core::ffi::c_int) -> GzWriteFailure {
    GzWriteFailure {
        errno_value,
        would_block: errno_value == crate::stdlib::EAGAIN
            || errno_value == crate::stdlib::EWOULDBLOCK,
    }
}

fn gz_direct_write(fd: &rustix::fd::OwnedFd, input: &[u8]) -> Result<usize, GzWriteFailure> {
    errno::set_errno(errno::Errno(0));
    rustix::io::write(fd, input).map_err(|error| gz_write_failure(error.raw_os_error()))
}

fn clear_buffered_input(buffer: &mut [u8]) {
    buffer.fill(0);
}

// A forward seek on a write handle is materialized as zero-filled input fed
// through the normal compression path.  Keep the byte-range proof and the
// scalar accounting outside the ABI-shaped gzip state so the eventual owned
// gzip facade can drive the same transaction without retaining stream
// pointers.  `input_len` deliberately follows zlib's c_int/off64_t chunk
// selection, including its truncating cast when the remaining skip is less
// than one buffer.
struct GzZeroStep {
    input_len: crate::stdlib::uInt,
}

struct GzZeroProgress {
    position: crate::stdlib::off64_t,
    skip: crate::stdlib::off64_t,
}

impl GzZeroStep {
    fn new(size: crate::stdlib::uInt, skip: crate::stdlib::off64_t) -> Self {
        let input_len = if ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && size > crate::src::gzlib::gz_intmax()
            || size as crate::stdlib::off64_t > skip
        {
            skip as crate::stdlib::uInt
        } else {
            size
        };
        Self { input_len }
    }

    fn zero_input<'a>(&self, input: &'a mut [u8]) -> Option<&'a mut [u8]> {
        let input = input.get_mut(..self.input_len as usize)?;
        clear_buffered_input(input);
        Some(input)
    }

    fn finish(
        &self,
        remaining: crate::stdlib::uInt,
        position: crate::stdlib::off64_t,
        skip: crate::stdlib::off64_t,
    ) -> GzZeroProgress {
        let written = self.input_len.wrapping_sub(remaining);
        GzZeroProgress {
            position: position + written as crate::stdlib::off64_t,
            skip: skip - written as crate::stdlib::off64_t,
        }
    }
}

// Zero-fill can either complete a deferred seek by itself or precede a normal
// compressor request.  Keep that distinction in the existing compressor
// boundary so callers do not need a second unsafe state adapter just to stage
// zero bytes.
enum GzSkipMaterialization {
    None,
    Continue,
    Only,
    InitializeOnly,
}

// One staged compressor pass carries only caller input and scalar transition
// choices.  The ABI stream and its callback-backed state stay at `gz_comp()`;
// this request can therefore drive admission, buffering, and result policy
// without itself owning an unsafe deflate boundary.
struct GzCompRequest<'a> {
    flush: ::core::ffi::c_int,
    external_input: Option<&'a [u8]>,
    retune: Option<GzDeflateRetune>,
    close: Option<GzWriteCloseCodec<'a>>,
}

// A compressor pass retains only owned buffers, checked cursor offsets, and
// scalar gzip state.  `gz_comp()` is the sole ABI adapter that imports and
// publishes the `z_stream` cursors around this owner.
struct GzCompOwner<'a> {
    buffers: &'a mut crate::gzguts_h::GzBuffers,
    fd: Option<&'a rustix::fd::OwnedFd>,
    path: Option<&'a [u8]>,
    message: &'a mut Option<Box<[u8]>>,
    error: &'a mut ::core::ffi::c_int,
    buffered: &'a mut crate::stdlib::uInt,
    direct: ::core::ffi::c_int,
    again: &'a mut ::core::ffi::c_int,
    reset: &'a mut ::core::ffi::c_int,
    input_available: crate::stdlib::uInt,
    input_cursor: Option<usize>,
    output_available: crate::stdlib::uInt,
    output_cursor: Option<usize>,
    output_next: Option<usize>,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
}

impl GzCompOwner<'_> {
    fn set_error(&mut self, error: ::core::ffi::c_int, message: &[u8]) {
        crate::src::gzlib::GzErrorState {
            message: self.message,
            error: self.error,
            buffered: self.buffered,
            again: *self.again,
            path: self.path,
        }
        .set(error, Some(message));
    }

    fn direct_input(&self) -> Option<&[u8]> {
        let cursor = self.input_cursor?;
        let buffer = self.buffers.input.as_deref()?;
        crate::src::gzlib::GzCodecInput::from_index(buffer, cursor, self.input_available)
            .and_then(|input| input.bytes(buffer))
    }

    fn pending_output(&self) -> Option<&[u8]> {
        let start = self.output_next?;
        let end = self.output_cursor?;
        self.buffers.output.as_deref()?.get(start..end)
    }

    fn reset_output_cursor(&mut self) -> Option<()> {
        let size = usize::try_from(self.buffers.size).ok()?;
        self.output_available = self.buffers.size;
        self.output_cursor = Some(0);
        self.output_next = Some(0);
        self.buffers.output.as_deref()?.get(..size)?;
        Some(())
    }

    fn advance_input(&mut self, consumed: usize) -> Option<()> {
        let available = usize::try_from(self.input_available).ok()?;
        let cursor = self.input_cursor?;
        let next = cursor.checked_add(consumed)?;
        self.buffers
            .input
            .as_deref()?
            .get(next..next.checked_add(available.checked_sub(consumed)?)?)?;
        self.input_cursor = Some(next);
        self.input_available = self
            .input_available
            .checked_sub(u32::try_from(consumed).ok()?)?;
        Some(())
    }

    fn advance_output(&mut self, written: usize) -> Option<()> {
        let next = self.output_next?.checked_add(written)?;
        self.buffers
            .output
            .as_deref()?
            .get(next..self.output_cursor?)?;
        self.output_next = Some(next);
        Some(())
    }

    fn set_codec_result(
        &mut self,
        codec_state: crate::src::gzlib::GzEmbeddedDeflateState,
        progress: crate::src::gzlib::GzEmbeddedDeflateProgress,
        external_input: bool,
    ) -> Option<()> {
        self.input_available = codec_state.input_available();
        self.output_available = codec_state.output_available();
        self.total_in = codec_state.total_in();
        self.total_out = codec_state.total_out();
        self.output_cursor = self
            .output_cursor?
            .checked_add(progress.output_used as usize);
        self.buffers.write_owner.as_mut()?.set_deflater(codec_state);
        if !external_input {
            let cursor = self
                .buffers
                .write_owner
                .as_ref()?
                .input()
                .after_codec(progress.remaining_input)?;
            self.buffers.write_owner.as_mut()?.set_input(cursor);
            self.input_cursor = Some(self.buffers.write_owner.as_ref()?.input().cursor());
        }
        Some(())
    }
}

enum GzCompCodecAction<'input, 'output> {
    End,
    Reset,
    Dispatch {
        flush: ::core::ffi::c_int,
        dispatch: crate::src::gzlib::GzEmbeddedDeflateDispatch<'input, 'output>,
    },
    Retune(GzDeflateRetune),
}

enum GzCompCodecResult {
    Complete,
    Dispatch(
        Option<(
            crate::src::gzlib::GzEmbeddedDeflateState,
            crate::src::gzlib::GzEmbeddedDeflateProgress,
        )>,
    ),
}

// Keep deferred-zero materialization in the compressor boundary, but schedule
// its individual no-flush requests iteratively.  This avoids recursively
// re-entering the ABI-shaped adapter while preserving the old ordering: drain
// pre-existing buffered input first, then each zero chunk, and finally the
// caller's requested operation.
unsafe fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
    external_input: Option<&[u8]>,
    mut retune: Option<GzDeflateRetune>,
    mut close: Option<GzWriteCloseCodec<'_>>,
    skip_materialization: GzSkipMaterialization,
) -> ::core::ffi::c_int {
    let initialize_only = matches!(skip_materialization, GzSkipMaterialization::InitializeOnly);
    let zero_only = matches!(skip_materialization, GzSkipMaterialization::Only);
    let mut materializing = zero_only
        || (state.skip != 0
            && (matches!(skip_materialization, GzSkipMaterialization::Continue)
                || close.is_some()));
    let mut needs_compress = state.strm.avail_in != 0;
    let mut staged_zero: Option<GzZeroStep> = None;
    let mut first_zero = true;

    loop {
        if materializing && !needs_compress {
            let step = GzZeroStep::new(state.buffers.size, state.skip);
            if first_zero {
                if state
                    .buffers
                    .input
                    .as_deref_mut()
                    .and_then(|input| step.zero_input(input))
                    .is_none()
                {
                    return -1;
                }
                first_zero = false;
            }
            state.strm.avail_in = step.input_len;
            let Some(input) = state.buffers.input.as_deref() else {
                return -1;
            };
            let Some(cursor) =
                crate::src::gzlib::GzCodecInput::from_index(input, 0, step.input_len)
            else {
                return -1;
            };
            state
                .buffers
                .write_owner
                .as_mut()
                .unwrap()
                .set_input(cursor);
            staged_zero = Some(step);
            needs_compress = true;
        }

        let materialization_request = materializing && needs_compress;
        let (request_flush, request_input, request_retune, request_close) =
            if materialization_request {
                (crate::zlib_h::Z_NO_FLUSH, None, None, None)
            } else {
                (flush, external_input, retune.take(), close.take())
            };
        let uses_external_input = request_input.is_some();
        // Initialization is part of this established compressor boundary:
        // allocation, embedded-deflater setup, and cursor publication are one
        // callback-backed lifecycle transaction.  Keeping it here removes a
        // second unsafe state adapter while all later requests see only
        // bounded buffers, checked indices, and scalar state.
        let initialization_failed = if state.buffers.size != 0 {
            false
        } else {
            'initialize: {
                let Some(buffers) =
                    crate::gzguts_h::GzBuffers::allocate_write(state.want, state.direct)
                else {
                    crate::src::gzlib::GzErrorState {
                        message: &mut state.msg,
                        error: &mut state.err,
                        buffered: &mut state.x.have,
                        again: state.again,
                        path: state.path.as_deref(),
                    }
                    .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
                    break 'initialize true;
                };
                state.buffers = buffers;
                // The write-side input allocation is owned storage. Retain its
                // empty cursor as an index/count rather than recovering it
                // from the embedded stream's temporary `next_in` projection.
                state.buffers.write_owner = Some(crate::src::gzlib::GzWriteOwner::new());
                if state.direct == 0 {
                    state.strm.zalloc = None;
                    state.strm.zfree = None;
                    state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
                    let ret = crate::src::deflate::deflateInit2_(
                        Some(&mut state.strm),
                        state.level,
                        8 as ::core::ffi::c_int,
                        15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        state.strategy,
                        crate::zlib_h::ZLIB_VERSION.as_ptr(),
                        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
                    );
                    if ret != crate::zlib_h::Z_OK {
                        state.buffers.clear();
                        crate::src::gzlib::GzErrorState {
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            again: state.again,
                            path: state.path.as_deref(),
                        }
                        .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
                        break 'initialize true;
                    }
                    // Successful setup, rather than buffer allocation alone,
                    // is close's lifecycle proof.
                    state.buffers.write_owner.as_mut().unwrap().set_deflater(
                        crate::src::gzlib::GzEmbeddedDeflateState::new(
                            0,
                            0,
                            state.strm.total_in,
                            state.strm.total_out,
                        ),
                    );
                    state.strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                }
                if state.direct == 0 {
                    let Some(mut buffers) = state.buffers.write_output_view() else {
                        state.buffers.clear();
                        crate::src::gzlib::GzErrorState {
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            again: state.again,
                            path: state.path.as_deref(),
                        }
                        .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
                        break 'initialize true;
                    };
                    let Some(setup) =
                        crate::src::gzlib::GzEmbeddedDeflateSetup::from_write_buffers(&mut buffers)
                    else {
                        state.buffers.clear();
                        crate::src::gzlib::GzErrorState {
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            again: state.again,
                            path: state.path.as_deref(),
                        }
                        .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
                        break 'initialize true;
                    };
                    // Initialization has no input, but its output publication
                    // uses the same complete bounded request shape as later
                    // `gz_comp()` dispatches.
                    let Some(mut call) = setup.call(&[], 0) else {
                        state.buffers.clear();
                        crate::src::gzlib::GzErrorState {
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            again: state.again,
                            path: state.path.as_deref(),
                        }
                        .set(crate::zlib_h::Z_MEM_ERROR, Some(b"out of memory"));
                        break 'initialize true;
                    };
                    state.strm.avail_out = call.output_available();
                    state.strm.next_out = call.output_mut().as_mut_ptr();
                    state.x.next = state.strm.next_out as *mut ::core::ffi::c_uchar;
                    state.buffers.write_owner.as_mut().unwrap().set_deflater(
                        crate::src::gzlib::GzEmbeddedDeflateState::new(
                            0,
                            state.strm.avail_out,
                            state.strm.total_in,
                            state.strm.total_out,
                        ),
                    );
                }
                false
            }
        };
        if initialization_failed {
            if let Some(close) = close.as_mut() {
                close.result.record_codec_result(-1, state.err);
            }
            return -1;
        }
        if initialize_only {
            return 0;
        }
        let input_cursor = if state.direct != 0 && state.strm.avail_in != 0 {
            state
                .buffers
                .input
                .as_deref()
                .and_then(|buffer| {
                    crate::src::gzlib::GzCodecInput::from_owned_buffer(
                        buffer,
                        state.strm.next_in.addr(),
                        state.strm.avail_in,
                    )
                })
                .map(|cursor| cursor.cursor())
        } else {
            state
                .buffers
                .write_owner
                .as_ref()
                .map(|owner| owner.input().cursor())
        };
        let output_cursor = state.buffers.output.as_deref().and_then(|buffer| {
            crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                buffer,
                state.strm.next_out.addr(),
                0,
            )
            .and_then(|cursor| cursor.consume(0).map(|(_, index)| index))
        });
        let output_next = state.buffers.output.as_deref().and_then(|buffer| {
            crate::src::gzlib::GzBufferedCursor::from_owned_buffer(buffer, state.x.next.addr(), 0)
                .and_then(|cursor| cursor.consume(0).map(|(_, index)| index))
        });
        let x = &mut state.x;
        let (buffers, fd, path, message, error, buffered, direct, again, reset, strm) = (
            &mut state.buffers,
            state.fd.as_ref(),
            state.path.as_deref(),
            &mut state.msg,
            &mut state.err,
            &mut x.have,
            state.direct,
            &mut state.again,
            &mut state.reset,
            &mut state.strm,
        );
        let mut owner = GzCompOwner {
            buffers,
            fd,
            path,
            message,
            error,
            buffered,
            direct,
            again,
            reset,
            input_available: strm.avail_in,
            input_cursor,
            output_available: strm.avail_out,
            output_cursor,
            output_next,
            total_in: strm.total_in,
            total_out: strm.total_out,
        };
        let status = gz_comp_request(
            &mut owner,
            GzCompRequest {
                flush: request_flush,
                external_input: request_input,
                retune: request_retune,
                close: request_close,
            },
            |action| match action {
                GzCompCodecAction::End => {
                    crate::src::deflate::deflateEnd(::core::ptr::NonNull::from(&mut *strm));
                    GzCompCodecResult::Complete
                }
                GzCompCodecAction::Reset => {
                    deflate_scalar_from_abi_stream(
                        strm,
                        DeflateScalarAction::Reset(DeflateResetKind::Full),
                    );
                    GzCompCodecResult::Complete
                }
                GzCompCodecAction::Retune(retune) => {
                    deflate_scalar_from_abi_stream(
                        strm,
                        DeflateScalarAction::Params {
                            level: retune.level,
                            strategy: retune.strategy,
                        },
                    );
                    GzCompCodecResult::Complete
                }
                GzCompCodecAction::Dispatch { flush, dispatch } => GzCompCodecResult::Dispatch(
                    dispatch.dispatch(|input, input_available, output, output_available| {
                        strm.next_in = input.as_ptr().cast_mut();
                        strm.avail_in = input_available;
                        strm.next_out = output.as_mut_ptr();
                        strm.avail_out = output_available;
                        let result = crate::src::deflate::deflate_dispatch_from_abi_stream(
                            strm, flush, None,
                        );
                        crate::src::gzlib::GzEmbeddedDeflateResult {
                            result,
                            remaining_input: strm.avail_in,
                            output_available: strm.avail_out,
                            total_in: strm.total_in,
                            total_out: strm.total_out,
                        }
                    }),
                ),
            },
        );
        strm.avail_in = owner.input_available;
        strm.avail_out = owner.output_available;
        strm.total_in = owner.total_in;
        strm.total_out = owner.total_out;
        if !uses_external_input {
            if let Some(cursor) = owner.input_cursor {
                if let Some(buffer) = owner.buffers.input.as_deref_mut() {
                    strm.next_in = buffer.as_mut_ptr().wrapping_add(cursor);
                }
            }
        }
        if let (Some(cursor), Some(buffer)) =
            (owner.output_cursor, owner.buffers.output.as_deref_mut())
        {
            strm.next_out = buffer.as_mut_ptr().wrapping_add(cursor);
        }
        if let (Some(next), Some(buffer)) = (owner.output_next, owner.buffers.output.as_deref_mut())
        {
            x.next = buffer.as_mut_ptr().wrapping_add(next);
        }

        if !materialization_request {
            return status;
        }
        needs_compress = false;
        if let Some(step) = staged_zero.take() {
            let progress = step.finish(state.strm.avail_in, state.x.pos, state.skip);
            state.x.pos = progress.position;
            state.skip = progress.skip;
        }
        if status == -1 as ::core::ffi::c_int {
            if let Some(close) = close.as_mut() {
                close.result.record_codec_result(status, state.err);
            }
            if zero_only || matches!(skip_materialization, GzSkipMaterialization::Continue) {
                return -1;
            }
            materializing = false;
            continue;
        }
        if !first_zero && state.skip == 0 {
            if zero_only {
                return 0;
            }
            materializing = false;
        }
    }
}

// This request executes one already-staged compressor pass.  Its caller owns
// the deferred-zero state machine above, so this body never needs to recurse
// through the ABI-shaped gzip state.
fn gz_comp_request<'request>(
    owner: &mut GzCompOwner<'_>,
    request: GzCompRequest<'request>,
    codec: impl for<'input, 'output> FnMut(GzCompCodecAction<'input, 'output>) -> GzCompCodecResult,
) -> ::core::ffi::c_int {
    let GzCompRequest {
        mut flush,
        external_input,
        retune,
        mut close,
    } = request;
    let codec = ::core::cell::RefCell::new(codec);
    // A close lends this existing codec boundary a pointer-free slot for the
    // single-use lifecycle proof.  Every exit below reaches this completion,
    // so a failed final write still tears the embedded deflater down before
    // the separate resource owner releases its buffers and descriptor.
    let mut finish_close = |owner: &mut GzCompOwner<'_>, status: ::core::ffi::c_int| {
        if let Some(close) = close.as_mut() {
            close.result.record_codec_result(status, *owner.error);
            let deflater = GzEmbeddedDeflaterClose::take(owner.buffers);
            if deflater.needs_teardown() {
                let GzCompCodecResult::Complete = (codec.borrow_mut())(GzCompCodecAction::End)
                else {
                    return status;
                };
                *close.deflater_closed = true;
            }
        }
        status
    };
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    // `gzsetparams()` only needs a block flush when the persistent input
    // owner has a pending prefix.  An empty owner bypasses the codec request
    // below, then reaches the shared scalar retune completion at the end.
    let retune_empty_owner = retune.is_some()
        && owner
            .buffers
            .write_owner
            .as_ref()
            .is_some_and(|owner| owner.input().available() == 0);
    if !retune_empty_owner {
        if owner.direct != 0 {
            while owner.input_available != 0 {
                *owner.again = 0;
                put = if owner.input_available > max {
                    max
                } else {
                    owner.input_available as ::core::ffi::c_uint
                };
                let write = {
                    let Some(input) = owner
                        .direct_input()
                        .and_then(|input| input.get(..put as usize))
                    else {
                        return finish_close(owner, -1);
                    };
                    let Some(fd) = owner.fd else {
                        return finish_close(owner, -1);
                    };
                    gz_direct_write(fd, input)
                };
                match write {
                    Ok(written) => writ = written as ::core::ffi::c_int,
                    Err(failure) => {
                        if failure.would_block {
                            *owner.again = 1;
                        }
                        let message = errno::Errno(failure.errno_value).to_string();
                        owner.set_error(crate::zlib_h::Z_ERRNO, message.as_bytes());
                        return finish_close(owner, -1);
                    }
                }
                if owner.advance_input(writ as usize).is_none() {
                    return finish_close(owner, -1);
                }
            }
            return finish_close(owner, 0);
        }
        if *owner.reset != 0 {
            if owner.input_available == 0 && flush == crate::zlib_h::Z_NO_FLUSH {
                return finish_close(owner, 0 as ::core::ffi::c_int);
            }
            let GzCompCodecResult::Complete = (codec.borrow_mut())(GzCompCodecAction::Reset) else {
                return finish_close(owner, -1);
            };
            *owner.reset = 0;
        }
        ret = crate::zlib_h::Z_OK;
        loop {
            if owner.output_available == 0
                || flush != crate::zlib_h::Z_NO_FLUSH
                    && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
            {
                while owner.output_cursor > owner.output_next {
                    *owner.again = 0;
                    let write = {
                        let Some(input) = owner.pending_output() else {
                            return finish_close(owner, -1);
                        };
                        put = u32::try_from(input.len())
                            .ok()
                            .map_or(max, |len| len.min(max));
                        let Some(fd) = owner.fd else {
                            return finish_close(owner, -1);
                        };
                        gz_direct_write(fd, &input[..put as usize])
                    };
                    match write {
                        Ok(written) => {
                            writ = written as ::core::ffi::c_int;
                            if owner.advance_output(written).is_none() {
                                return finish_close(owner, -1);
                            }
                        }
                        Err(failure) => {
                            if failure.would_block {
                                *owner.again = 1;
                            }
                            let message = errno::Errno(failure.errno_value).to_string();
                            owner.set_error(crate::zlib_h::Z_ERRNO, message.as_bytes());
                            return finish_close(owner, -1);
                        }
                    }
                }
                if owner.output_available == 0 {
                    if owner.reset_output_cursor().is_none() {
                        return finish_close(owner, -1);
                    }
                }
            }
            // Form one complete bounded embedded-deflate request before
            // publishing its cursors to the ABI stream.  Buffered callers borrow
            // the owned input allocation; large writes pass their caller slice
            // explicitly, so this layer never reconstructs that range from an
            // unchecked state cursor.
            let input_available = owner.input_available;
            let output_available = owner.output_available;
            let Some(output_cursor) = owner.output_cursor else {
                return finish_close(owner, -1);
            };
            // Availability belongs to this request, since callers may have
            // staged fresh input since the last pass. Totals persist with the
            // paired gzip buffers so later write policy need not trust ABI
            // counters between calls.
            // Initialization installs this tag immediately after
            // `deflateInit2_()` succeeds. Do not recreate it from allocation
            // state here: that would allow failed setup to masquerade as an
            // initialized codec.
            let Some(persisted) = owner
                .buffers
                .write_owner
                .as_ref()
                .and_then(crate::src::gzlib::GzWriteOwner::deflater)
            else {
                return finish_close(owner, -1);
            };
            let codec_state = crate::src::gzlib::GzEmbeddedDeflateState::new(
                input_available,
                output_available,
                persisted.total_in(),
                persisted.total_out(),
            );
            let input = if input_available == 0 {
                // zlib permits a flush/finalization pass with no current input;
                // `next_in` may then still be null, so no cursor validation is
                // meaningful or required for the empty request.
                &[]
            } else {
                match external_input {
                    Some(input) => match input.get(..input_available as usize) {
                        Some(input) => input,
                        None => return finish_close(owner, -1),
                    },
                    None => {
                        let Some(cursor) = owner
                            .buffers
                            .write_owner
                            .as_ref()
                            .map(crate::src::gzlib::GzWriteOwner::input)
                        else {
                            return finish_close(owner, -1);
                        };
                        let Some(buffer) = owner.buffers.input.as_deref() else {
                            return finish_close(owner, -1);
                        };
                        let Some(input) = cursor.bytes(buffer) else {
                            return finish_close(owner, -1);
                        };
                        input
                    }
                }
            };
            let output_size = owner.buffers.size;
            let Some(output) = owner.buffers.output.as_deref_mut() else {
                return finish_close(owner, -1);
            };
            let Some(setup) =
                crate::src::gzlib::GzEmbeddedDeflateSetup::from_output(output, output_size)
            else {
                return finish_close(owner, -1);
            };
            let Some(call) =
                setup.call_at_output_index(input, input_available, output_cursor, output_available)
            else {
                return finish_close(owner, -1);
            };
            let dispatch = crate::src::gzlib::GzEmbeddedDeflateDispatch::new(codec_state, call);
            let GzCompCodecResult::Dispatch(Some((codec_state, snapshot))) =
                (codec.borrow_mut())(GzCompCodecAction::Dispatch { flush, dispatch })
            else {
                return finish_close(owner, -1);
            };
            ret = snapshot.result;
            have = snapshot.output_used;
            if owner
                .set_codec_result(codec_state, snapshot, external_input.is_some())
                .is_none()
            {
                return finish_close(owner, -1);
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR {
                owner.set_error(
                    crate::zlib_h::Z_STREAM_ERROR,
                    b"internal error: deflate stream corrupt",
                );
                return finish_close(owner, -1);
            }
            if have == 0 {
                break;
            }
        }
        if flush == crate::zlib_h::Z_FINISH {
            *owner.reset = 1;
        }
    }
    if let Some(retune) = retune {
        let GzCompCodecResult::Complete = (codec.borrow_mut())(GzCompCodecAction::Retune(retune))
        else {
            return finish_close(owner, -1);
        };
    }
    finish_close(owner, 0)
}

// This is the only ABI-shaped write adapter.  The persistent `GzWriteOwner`
// supplies the owned input cursor and deflater lifecycle; the adapter merely
// projects those bounded requests through the legacy gzip/deflate state.
pub(crate) unsafe fn gzip_write_state_adapter(
    state: &mut crate::gzguts_h::gz_state,
    operation: GzWriteOperation<'_>,
) -> crate::stdlib::z_size_t {
    // Take a fresh scalar snapshot for every public operation.  Compressor
    // requests can initialize, retune, or materialize a deferred seek, so a
    // snapshot must never survive a request and be reused by the next one.
    let snapshot = GzWriteStateSnapshot {
        policy: GzWritePolicy {
            mode: state.mode,
            err: state.err,
            again: state.again,
            direct: state.direct,
        },
        level: state.level,
        strategy: state.strategy,
        skip: state.skip,
    };
    let (transaction, result) = match gzip_write_admission(operation, snapshot) {
        // Close is a write-side codec request followed by the pointer-free
        // resource transaction.  Keep it in this established state adapter
        // so the close path does not need a second ABI-shaped gzip facade.
        GzWriteAdmission::Close => {
            let Some(mut result) = GzWriteCloseResult::begin(snapshot.policy.mode) else {
                return crate::zlib_h::Z_STREAM_ERROR as crate::stdlib::z_size_t;
            };
            let mut deflater_closed = false;
            gz_comp(
                state,
                crate::zlib_h::Z_FINISH,
                None,
                None,
                Some(GzWriteCloseCodec {
                    result: &mut result,
                    deflater_closed: &mut deflater_closed,
                }),
                GzSkipMaterialization::None,
            );
            let mut resources = GzWriteCloseResources::take(
                &mut state.buffers,
                &mut state.fd,
                &mut state.path,
                &mut state.msg,
                &mut state.err,
            );
            resources.release_write_buffers(deflater_closed);
            return result.finish(resources.finish()) as crate::stdlib::z_size_t;
        }
        GzWriteAdmission::Flush(plan) => {
            if matches!(plan, GzFlushPlan::Reject) {
                return crate::zlib_h::Z_STREAM_ERROR as crate::stdlib::z_size_t;
            }
            crate::src::gzlib::GzErrorState {
                message: &mut state.msg,
                error: &mut state.err,
                buffered: &mut state.x.have,
                again: state.again,
                path: state.path.as_deref(),
            }
            .clear();
            let GzFlushPlan::Dispatch { flush } = plan else {
                return crate::zlib_h::Z_STREAM_ERROR as crate::stdlib::z_size_t;
            };
            if gz_comp(
                state,
                flush,
                None,
                None,
                None,
                GzSkipMaterialization::Continue,
            ) == -1
            {
                return state.err as crate::stdlib::z_size_t;
            }
            return state.err as crate::stdlib::z_size_t;
        }
        GzWriteAdmission::SetParams {
            level,
            strategy,
            plan,
        } => {
            if matches!(plan, GzSetParamsPlan::Reject) {
                return crate::zlib_h::Z_STREAM_ERROR as crate::stdlib::z_size_t;
            }
            crate::src::gzlib::GzErrorState {
                message: &mut state.msg,
                error: &mut state.err,
                buffered: &mut state.x.have,
                again: state.again,
                path: state.path.as_deref(),
            }
            .clear();
            let GzSetParamsPlan::Change { materialize_skip } = plan else {
                return crate::zlib_h::Z_OK as crate::stdlib::z_size_t;
            };
            if materialize_skip
                && gz_comp(
                    state,
                    crate::zlib_h::Z_NO_FLUSH,
                    None,
                    None,
                    None,
                    GzSkipMaterialization::Only,
                ) == -1 as ::core::ffi::c_int
            {
                return state.err as crate::stdlib::z_size_t;
            }
            if gzsetparams_needs_retune(state.buffers.size)
                && gz_comp(
                    state,
                    crate::zlib_h::Z_BLOCK,
                    None,
                    Some(GzDeflateRetune { level, strategy }),
                    None,
                    GzSkipMaterialization::None,
                ) == -1 as ::core::ffi::c_int
            {
                return state.err as crate::stdlib::z_size_t;
            }
            state.level = level;
            state.strategy = strategy;
            return crate::zlib_h::Z_OK as crate::stdlib::z_size_t;
        }
        GzWriteAdmission::Write {
            input,
            flavor,
            policy,
        } => {
            let plan = gzwrite_plan(
                input,
                flavor,
                policy,
                crate::src::gzlib::GzErrorState {
                    message: &mut state.msg,
                    error: &mut state.err,
                    buffered: &mut state.x.have,
                    again: state.again,
                    path: state.path.as_deref(),
                },
            );
            let GzWritePlan::Dispatch {
                transaction,
                result,
            } = plan
            else {
                let GzWritePlan::Return(result) = plan else {
                    unreachable!("write plan must dispatch or return");
                };
                return result;
            };
            (transaction, result)
        }
    };
    let mut request = transaction.request;
    let mut ret: ::core::ffi::c_int = 0;
    if state.buffers.size == 0 as ::core::ffi::c_uint
        && gz_comp(
            state,
            crate::zlib_h::Z_NO_FLUSH,
            None,
            None,
            None,
            GzSkipMaterialization::InitializeOnly,
        ) == -1 as ::core::ffi::c_int
    {
        return result.finish(0);
    }
    if state.skip != 0
        && gz_comp(
            state,
            crate::zlib_h::Z_NO_FLUSH,
            None,
            None,
            None,
            GzSkipMaterialization::Only,
        ) == -1 as ::core::ffi::c_int
    {
        return result.finish(0);
    }
    if request.len() < state.buffers.size as crate::stdlib::z_size_t {
        loop {
            let (copy, cursor) = {
                let buffer =
                    &mut state.buffers.input.as_deref_mut().unwrap()[..state.buffers.size as usize];
                let cursor = state
                    .buffers
                    .write_owner
                    .as_mut()
                    .map(crate::src::gzlib::GzWriteOwner::take_input)
                    .unwrap_or_else(crate::src::gzlib::GzCodecInput::empty);
                let Some(mut buffered) = crate::src::gzlib::GzBufferedInput::from_index(
                    buffer,
                    cursor.cursor(),
                    cursor.available(),
                ) else {
                    return result.finish(0);
                };
                let copy = buffered.append(request.remaining());
                let Some((start, have)) = buffered.cursor() else {
                    return result.finish(0);
                };
                let Some(cursor) = crate::src::gzlib::GzCodecInput::from_index(buffer, start, have)
                else {
                    return result.finish(0);
                };
                (copy, cursor)
            };
            state.strm.avail_in = cursor.available();
            state
                .buffers
                .write_owner
                .as_mut()
                .unwrap()
                .set_input(cursor);
            state.x.pos += copy as crate::stdlib::off64_t;
            if request.advance(copy).is_none() {
                return result.finish(0);
            }
            if request.is_empty() {
                break;
            }
            if gz_comp(
                state,
                crate::zlib_h::Z_NO_FLUSH,
                None,
                None,
                None,
                GzSkipMaterialization::None,
            ) == -1 as ::core::ffi::c_int
            {
                return result.finish(request.partial_or_zero(state.again));
            }
        }
    } else {
        if state
            .buffers
            .write_owner
            .as_ref()
            .is_some_and(|owner| owner.input().available() != 0)
            && gz_comp(
                state,
                crate::zlib_h::Z_NO_FLUSH,
                None,
                None,
                None,
                GzSkipMaterialization::None,
            ) == -1 as ::core::ffi::c_int
        {
            return result.finish(0);
        }
        if state.direct != 0 {
            while !request.is_empty() {
                let max = ((-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2).wrapping_add(1))
                    as usize;
                let count = request.remaining().len().min(max);
                state.again = 0;
                match gz_direct_write(state.fd.as_ref().unwrap(), &request.remaining()[..count]) {
                    Ok(written) => {
                        state.x.pos += written as crate::stdlib::off64_t;
                        if request.advance(written).is_none() {
                            return result.finish(0);
                        }
                    }
                    Err(failure) => {
                        if failure.would_block {
                            state.again = 1;
                        }
                        let message = errno::Errno(failure.errno_value).to_string();
                        crate::src::gzlib::GzErrorState {
                            message: &mut state.msg,
                            error: &mut state.err,
                            buffered: &mut state.x.have,
                            again: state.again,
                            path: state.path.as_deref(),
                        }
                        .set(crate::zlib_h::Z_ERRNO, Some(message.as_bytes()));
                        return result.finish(request.partial_or_zero(state.again));
                    }
                }
            }
            return result.finish(request.total);
        }
        loop {
            let chunk = GzCompressionChunk::next(request.len());
            state.strm.avail_in = chunk.input_len;
            ret = gz_comp(
                state,
                crate::zlib_h::Z_NO_FLUSH,
                Some(request.remaining()),
                None,
                None,
                GzSkipMaterialization::None,
            );
            let consumed = chunk.consumed(state.strm.avail_in);
            state.x.pos += consumed as crate::stdlib::off64_t;
            if request.advance(consumed as usize).is_none() {
                return result.finish(0);
            }
            if ret == -1 as ::core::ffi::c_int {
                return result.finish(request.partial_or_zero(state.again));
            }
            if request.is_empty() {
                break;
            }
        }
    }
    result.finish(request.total)
}
#[export_name = "gzwrite"]

pub unsafe extern "C" fn gzwrite_ffi(
    mut file: crate::zlib_h::gzFile,
    mut buf: crate::stdlib::voidpc,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    // Validate the opaque handle before touching the caller buffer.  Besides
    // keeping this wrapper a boundary conversion only, this preserves the
    // null/invalid-handle return without constructing a slice from `buf`.
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0 as ::core::ffi::c_int;
    };
    let input = if len == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(buf.cast::<u8>(), len as usize)
    };
    gzip_write_state_adapter(
        state,
        GzWriteOperation::Write {
            input,
            flavor: GzWriteFlavor::Bytes,
        },
    ) as ::core::ffi::c_int
}
#[export_name = "gzfwrite"]

pub unsafe extern "C" fn gzfwrite_ffi(
    mut buf: crate::stdlib::voidpc,
    mut size: crate::stdlib::z_size_t,
    mut nitems: crate::stdlib::z_size_t,
    mut file: crate::zlib_h::gzFile,
) -> crate::stdlib::z_size_t {
    // As above, establish the opaque handle before forming any caller-buffer
    // view.  The implementation retains the size/item validation policy.
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return 0 as crate::stdlib::z_size_t;
    };
    let input = match size.checked_mul(nitems) {
        Some(0) | None => &[],
        Some(len) => ::core::slice::from_raw_parts(buf.cast::<u8>(), len),
    };
    gzip_write_state_adapter(
        state,
        GzWriteOperation::Write {
            input,
            flavor: GzWriteFlavor::Items { size, nitems },
        },
    )
}
#[export_name = "gzputc"]

pub unsafe extern "C" fn gzputc_ffi(
    mut file: crate::zlib_h::gzFile,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    let byte = [c as ::core::ffi::c_uchar];
    gzip_write_state_adapter(
        state,
        GzWriteOperation::Write {
            input: &byte,
            flavor: GzWriteFlavor::Byte { value: c },
        },
    ) as ::core::ffi::c_int
}
#[export_name = "gzputs"]

pub unsafe extern "C" fn gzputs_ffi(
    mut file: crate::zlib_h::gzFile,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if file.is_null() || s.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let text = ::core::ffi::CStr::from_ptr(s).to_bytes();
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzip_write_state_adapter(
        state,
        GzWriteOperation::Write {
            input: text,
            flavor: GzWriteFlavor::Text,
        },
    ) as ::core::ffi::c_int
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzip_write_state_adapter(state, GzWriteOperation::Flush { flush }) as ::core::ffi::c_int
}
#[export_name = "gzsetparams"]

pub unsafe extern "C" fn gzsetparams_ffi(
    mut file: crate::zlib_h::gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzip_write_state_adapter(state, GzWriteOperation::SetParams { level, strategy })
        as ::core::ffi::c_int
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    crate::src::gzclose::gzclose(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        crate::src::gzclose::GzCloseTarget::Write,
    )
}
