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
pub use crate::src::deflate::deflate_params_from_stream as deflateParams;
use crate::src::deflate::deflate_reset_keep_from_stream;
pub use crate::src::deflate::internal_state;
use crate::src::deflate::DeflateResetKind;
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
enum GzWriteFlavor {
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

unsafe fn gz_init(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let Some(buffers) = crate::gzguts_h::GzBuffers::allocate_write(state.want, state.direct) else {
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
    state.buffers = buffers;
    // The write-side input allocation is owned storage.  Retain its empty
    // cursor as an index/count now, so later buffered writes need not recover
    // it from the embedded stream's temporary `next_in` projection.
    state.buffers.write_owner = Some(crate::src::gzlib::GzWriteOwner::new());
    if state.direct == 0 {
        state.strm.zalloc = None;
        state.strm.zfree = None;
        state.strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = crate::src::deflate::deflateInit2_(
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
            return -1 as ::core::ffi::c_int;
        }
        // Successful initialization, rather than buffer allocation or
        // compressed mode, is the lifecycle proof needed by close.  Install
        // the pointer-free owner before any later setup can publish cursors.
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
        // The allocation owner proves that this is the bounded compressed
        // output buffer before its address is published to the ABI stream.
        // Keep the view separate from `gz_state` so an embedded-deflate owner
        // can replace this one cursor projection without changing setup.
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
            return -1;
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
            return -1;
        };
        // Initialization has no input, but use the complete bounded request
        // shape already.  Later `gz_comp()` dispatches can use the same owner
        // for non-empty input without rebuilding a cursor from gzip state.
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
            return -1;
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
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_comp(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
    external_input: Option<&[u8]>,
    retune: Option<GzDeflateRetune>,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    if state.buffers.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    // `gzsetparams()` only needs a block flush when the persistent input
    // owner has a pending prefix.  An empty owner bypasses the codec request
    // below, then reaches the shared scalar retune completion at the end.
    let retune_empty_owner = retune.is_some()
        && state
            .buffers
            .write_owner
            .as_ref()
            .is_some_and(|owner| owner.input().available() == 0);
    if !retune_empty_owner {
        if state.direct != 0 {
            while state.strm.avail_in != 0 {
                state.again = 0;
                put = if state.strm.avail_in > max {
                    max
                } else {
                    state.strm.avail_in as ::core::ffi::c_uint
                };
                let write = {
                    let Some(buffer) = state.buffers.input.as_deref() else {
                        return -1;
                    };
                    let Some(buffered) = crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                        buffer,
                        state.strm.next_in.addr(),
                        state.strm.avail_in,
                    ) else {
                        return -1;
                    };
                    let Some((input, _)) = buffered.consume(put as usize) else {
                        return -1;
                    };
                    gz_direct_write(state.fd.as_ref().unwrap(), input)
                };
                match write {
                    Ok(written) => writ = written as ::core::ffi::c_int,
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
                        return -1;
                    }
                }
                state.strm.avail_in = state
                    .strm
                    .avail_in
                    .wrapping_sub(writ as crate::stdlib::uInt);
                state.strm.next_in = state.strm.next_in.wrapping_add(writ as usize);
            }
            return 0;
        }
        if state.reset != 0 {
            if state.strm.avail_in == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return 0 as ::core::ffi::c_int;
            }
            deflate_reset_keep_from_stream(&mut state.strm, DeflateResetKind::Full);
            state.reset = 0 as ::core::ffi::c_int;
        }
        ret = crate::zlib_h::Z_OK;
        loop {
            if state.strm.avail_out == 0 as crate::stdlib::uInt
                || flush != crate::zlib_h::Z_NO_FLUSH
                    && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END)
            {
                while state.strm.next_out > state.x.next {
                    state.again = 0 as ::core::ffi::c_int;
                    let write = {
                        let Some(buffer) = state.buffers.output.as_deref() else {
                            return -1;
                        };
                        let Some(buffered_len) =
                            state.strm.next_out.addr().checked_sub(state.x.next.addr())
                        else {
                            return -1;
                        };
                        let Some(buffered_len) = ::core::ffi::c_uint::try_from(buffered_len).ok()
                        else {
                            return -1;
                        };
                        let Some(buffered) = crate::src::gzlib::GzBufferedCursor::from_owned_buffer(
                            buffer,
                            state.x.next.addr(),
                            buffered_len,
                        ) else {
                            return -1;
                        };
                        put = buffered_len.min(max);
                        let Some((input, _)) = buffered.consume(put as usize) else {
                            return -1;
                        };
                        let result = gz_direct_write(state.fd.as_ref().unwrap(), input);
                        result
                    };
                    match write {
                        Ok(written) => {
                            writ = written as ::core::ffi::c_int;
                            state.x.next = state.x.next.wrapping_add(written);
                        }
                        Err(failure) => {
                            if failure.would_block {
                                state.again = 1 as ::core::ffi::c_int;
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
                            return -1 as ::core::ffi::c_int;
                        }
                    }
                }
                if state.strm.avail_out == 0 as crate::stdlib::uInt {
                    state.strm.avail_out = state.buffers.size as crate::stdlib::uInt;
                    state.strm.next_out = state.buffers.output.as_deref_mut().unwrap().as_mut_ptr();
                    state.x.next = state.strm.next_out;
                }
            }
            // Form one complete bounded embedded-deflate request before
            // publishing its cursors to the ABI stream.  Buffered callers borrow
            // the owned input allocation; large writes pass their caller slice
            // explicitly, so this layer never reconstructs that range from an
            // unchecked state cursor.
            let input_available = state.strm.avail_in;
            let output_available = state.strm.avail_out;
            let output_cursor = state.strm.next_out.addr();
            // Availability belongs to this request, since callers may have
            // staged fresh input since the last pass. Totals persist with the
            // paired gzip buffers so later write policy need not trust ABI
            // counters between calls.
            // `gz_init()` installs this tag immediately after `deflateInit2_()`
            // succeeds.  Do not recreate it from allocation state here: that
            // would allow a failed setup to masquerade as an initialized codec.
            let Some(persisted) = state
                .buffers
                .write_owner
                .as_ref()
                .and_then(crate::src::gzlib::GzWriteOwner::deflater)
            else {
                return -1;
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
                        None => return -1,
                    },
                    None => {
                        let Some(cursor) = state
                            .buffers
                            .write_owner
                            .as_ref()
                            .map(crate::src::gzlib::GzWriteOwner::input)
                        else {
                            return -1;
                        };
                        let Some(buffer) = state.buffers.input.as_deref() else {
                            return -1;
                        };
                        let Some(input) = cursor.bytes(buffer) else {
                            return -1;
                        };
                        input
                    }
                }
            };
            let output_size = state.buffers.size;
            let Some(output) = state.buffers.output.as_deref_mut() else {
                return -1;
            };
            let Some(setup) =
                crate::src::gzlib::GzEmbeddedDeflateSetup::from_output(output, output_size)
            else {
                return -1;
            };
            let Some(call) = setup.call_at_output_cursor(
                input,
                input_available,
                output_cursor,
                output_available,
            ) else {
                return -1;
            };
            let dispatch = crate::src::gzlib::GzEmbeddedDeflateDispatch::new(codec_state, call);
            let Some((codec_state, snapshot)) =
                dispatch.dispatch(|input, input_available, output, output_available| {
                    let strm = &mut state.strm;
                    strm.next_in = input.as_ptr().cast_mut();
                    strm.avail_in = input_available;
                    strm.next_out = output.as_mut_ptr();
                    strm.avail_out = output_available;
                    let result = crate::src::deflate::deflate_dispatch_from_abi_stream(strm, flush);
                    crate::src::gzlib::GzEmbeddedDeflateResult {
                        result,
                        remaining_input: strm.avail_in,
                        output_available: strm.avail_out,
                        total_in: strm.total_in,
                        total_out: strm.total_out,
                    }
                })
            else {
                return -1;
            };
            ret = snapshot.result;
            state.strm.avail_in = codec_state.input_available();
            state.strm.avail_out = codec_state.output_available();
            state.strm.total_in = codec_state.total_in();
            state.strm.total_out = codec_state.total_out();
            state
                .buffers
                .write_owner
                .as_mut()
                .unwrap()
                .set_deflater(codec_state);
            if external_input.is_none() {
                let Some(cursor) = state
                    .buffers
                    .write_owner
                    .as_ref()
                    .map(crate::src::gzlib::GzWriteOwner::input)
                else {
                    return -1;
                };
                let Some(cursor) = cursor.after_codec(snapshot.remaining_input) else {
                    return -1;
                };
                state
                    .buffers
                    .write_owner
                    .as_mut()
                    .unwrap()
                    .set_input(cursor);
            }
            if ret == crate::zlib_h::Z_STREAM_ERROR {
                crate::src::gzlib::GzErrorState {
                    message: &mut state.msg,
                    error: &mut state.err,
                    buffered: &mut state.x.have,
                    again: state.again,
                    path: state.path.as_deref(),
                }
                .set(
                    crate::zlib_h::Z_STREAM_ERROR,
                    Some(b"internal error: deflate stream corrupt"),
                );
                return -1 as ::core::ffi::c_int;
            }
            have = snapshot.output_used;
            if have == 0 {
                break;
            }
        }
        if flush == crate::zlib_h::Z_FINISH {
            state.reset = 1 as ::core::ffi::c_int;
        }
    }
    if let Some(retune) = retune {
        crate::src::deflate::deflate_params_from_stream(
            &mut state.strm,
            retune.level,
            retune.strategy,
        );
    }
    return 0 as ::core::ffi::c_int;
}

unsafe fn gz_zero(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    // Treat pre-existing buffered input and each staged zero chunk as the
    // same pending compressor request.  This keeps the zero-fill policy in
    // the pointer-free `GzZeroStep`/`GzZeroProgress` pair and leaves a single
    // compressor boundary below this ABI-shaped state adapter.
    let mut needs_compress = state.strm.avail_in != 0;
    let mut staged_zero: Option<GzZeroStep> = None;
    let mut first = true;
    loop {
        if needs_compress {
            let ret = gz_comp(state, crate::zlib_h::Z_NO_FLUSH, None, None);
            if let Some(step) = staged_zero.take() {
                let progress = step.finish(state.strm.avail_in, state.x.pos, state.skip);
                state.x.pos = progress.position;
                state.skip = progress.skip;
            }
            if ret == -1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if !first && state.skip == 0 {
                return 0;
            }
        }
        let step = GzZeroStep::new(state.buffers.size, state.skip);
        if first {
            if state
                .buffers
                .input
                .as_deref_mut()
                .and_then(|input| step.zero_input(input))
                .is_none()
            {
                return -1;
            }
            first = false;
        }
        state.strm.avail_in = step.input_len;
        let Some(input) = state.buffers.input.as_deref() else {
            return -1;
        };
        let Some(cursor) = crate::src::gzlib::GzCodecInput::from_index(input, 0, step.input_len)
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
}

// This is the only ABI-shaped write adapter.  The persistent `GzWriteOwner`
// supplies the owned input cursor and deflater lifecycle; the adapter merely
// projects those bounded requests through the legacy gzip/deflate state.
unsafe fn gzip_write_state_adapter(
    state: &mut crate::gzguts_h::gz_state,
    transaction: GzWriteTransaction<'_>,
) -> crate::stdlib::z_size_t {
    let mut request = transaction.request;
    let mut ret: ::core::ffi::c_int = 0;
    if state.buffers.size == 0 as ::core::ffi::c_uint && gz_init(state) == -1 as ::core::ffi::c_int
    {
        return 0 as crate::stdlib::z_size_t;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return 0 as crate::stdlib::z_size_t;
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
                    return 0 as crate::stdlib::z_size_t;
                };
                let copy = buffered.append(request.remaining());
                let Some((start, have)) = buffered.cursor() else {
                    return 0 as crate::stdlib::z_size_t;
                };
                let Some(cursor) = crate::src::gzlib::GzCodecInput::from_index(buffer, start, have)
                else {
                    return 0 as crate::stdlib::z_size_t;
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
                return 0;
            }
            if request.is_empty() {
                break;
            }
            if gz_comp(state, crate::zlib_h::Z_NO_FLUSH, None, None) == -1 as ::core::ffi::c_int {
                return request.partial_or_zero(state.again);
            }
        }
    } else {
        if state
            .buffers
            .write_owner
            .as_ref()
            .is_some_and(|owner| owner.input().available() != 0)
            && gz_comp(state, crate::zlib_h::Z_NO_FLUSH, None, None) == -1 as ::core::ffi::c_int
        {
            return 0 as crate::stdlib::z_size_t;
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
                            return 0;
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
                        return request.partial_or_zero(state.again);
                    }
                }
            }
            return request.total;
        }
        loop {
            let chunk = GzCompressionChunk::next(request.len());
            state.strm.avail_in = chunk.input_len;
            ret = gz_comp(
                state,
                crate::zlib_h::Z_NO_FLUSH,
                Some(request.remaining()),
                None,
            );
            let consumed = chunk.consumed(state.strm.avail_in);
            state.x.pos += consumed as crate::stdlib::off64_t;
            if request.advance(consumed as usize).is_none() {
                return 0;
            }
            if ret == -1 as ::core::ffi::c_int {
                return request.partial_or_zero(state.again);
            }
            if request.is_empty() {
                break;
            }
        }
    }
    return request.total;
}
unsafe fn gzwrite(
    state: &mut crate::gzguts_h::gz_state,
    input: &[u8],
    flavor: GzWriteFlavor,
) -> crate::stdlib::z_size_t {
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
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
    match plan {
        GzWritePlan::Return(result) => result,
        GzWritePlan::Dispatch {
            transaction,
            result,
        } => result.finish(gzip_write_state_adapter(state, transaction)),
    }
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
    gzwrite(state, input, GzWriteFlavor::Bytes) as ::core::ffi::c_int
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
    gzwrite(state, input, GzWriteFlavor::Items { size, nitems })
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
    gzwrite(state, &byte, GzWriteFlavor::Byte { value: c }) as ::core::ffi::c_int
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
    gzwrite(state, text, GzWriteFlavor::Text) as ::core::ffi::c_int
}
unsafe fn gzflush(
    state: &mut crate::gzguts_h::gz_state,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
    if !policy.accepts_write() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::src::gzlib::GzErrorState {
        message: &mut state.msg,
        error: &mut state.err,
        buffered: &mut state.x.have,
        again: state.again,
        path: state.path.as_deref(),
    }
    .clear();
    if flush < 0 as ::core::ffi::c_int || flush > crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.skip != 0 && gz_zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    gz_comp(state, flush, None, None);
    return state.err;
}
#[export_name = "gzflush"]

pub unsafe extern "C" fn gzflush_ffi(
    mut file: crate::zlib_h::gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzflush(state, flush)
}
unsafe fn gzsetparams(
    state: &mut crate::gzguts_h::gz_state,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let policy = GzWritePolicy {
        mode: state.mode,
        err: state.err,
        again: state.again,
        direct: state.direct,
    };
    let plan = gzsetparams_plan(
        &policy,
        state.level,
        state.strategy,
        level,
        strategy,
        state.skip,
    );
    if matches!(plan, GzSetParamsPlan::Reject) {
        return crate::zlib_h::Z_STREAM_ERROR;
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
        return crate::zlib_h::Z_OK;
    };
    if materialize_skip && gz_zero(state) == -1 as ::core::ffi::c_int {
        return state.err;
    }
    if gzsetparams_needs_retune(state.buffers.size) {
        if gz_comp(
            state,
            crate::zlib_h::Z_BLOCK,
            None,
            Some(GzDeflateRetune { level, strategy }),
        ) == -1 as ::core::ffi::c_int
        {
            return state.err;
        }
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
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    gzsetparams(state, level, strategy)
}
pub unsafe fn gzclose_w(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    let Some(mut result) = GzWriteCloseResult::begin(state.mode) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if state.skip != 0 {
        let status = gz_zero(state);
        result.record_codec_result(status, state.err);
    }
    let status = gz_comp(state, crate::zlib_h::Z_FINISH, None, None);
    result.record_codec_result(status, state.err);
    // The lifecycle tag, not mode or allocated-buffer size, authorizes the
    // matching codec end operation.  Consume it before either buffer is
    // detached so a partial setup cannot erase an initialized deflater.
    let had_embedded_deflater = state.buffers.take_embedded_deflater().is_some();
    if had_embedded_deflater {
        crate::src::deflate::deflateEnd(::core::ptr::NonNull::from(&mut state.strm));
    }
    let mut resources = GzWriteCloseResources::take(
        &mut state.buffers,
        &mut state.fd,
        &mut state.path,
        &mut state.msg,
        &mut state.err,
    );
    resources.release_write_buffers(had_embedded_deflater);
    result.finish(resources.finish())
}
#[export_name = "gzclose_w"]

pub unsafe extern "C" fn gzclose_w_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    crate::src::gzclose::gzclose(
        ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep),
        crate::src::gzclose::GzCloseTarget::Write,
    )
}
