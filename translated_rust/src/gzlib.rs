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
pub use crate::stdlib::__O_CLOEXEC;
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

// `OwnedFd` normally closes on drop, but gzip close reports a close failure
// to its caller.  Consume the owner in this pointer-free boundary so the
// read and write close paths preserve that observable result without each
// repeating a raw-FD handoff.
pub(crate) fn gz_close_fd(fd: rustix::fd::OwnedFd) -> rustix::io::Result<()> {
    unsafe {
        rustix::io::try_close(<rustix::fd::OwnedFd as rustix::fd::IntoRawFd>::into_raw_fd(
            fd,
        ))
    }
}

// A write handle's paired allocations are owned by `GzBuffers`, but embedded
// deflate setup needs only the bounded compressed-output allocation and its
// checked size.  Keeping that hand-off separate from the input allocation
// lets `gz_comp()` later retain an independent bounded input borrow while it
// constructs the complete request owner.
pub(crate) struct GzWriteOutputView<'a> {
    size: crate::stdlib::uInt,
    output: &'a mut [u8],
}

impl<'a> GzWriteOutputView<'a> {
    pub(crate) fn size(&self) -> crate::stdlib::uInt {
        self.size
    }

    pub(crate) fn output_mut(&mut self) -> &mut [u8] {
        self.output
    }
}

impl crate::gzguts_h::GzBuffers {
    // Allocate gzip write storage as the same single owner transaction used
    // by the read side.  Direct writes intentionally retain only the doubled
    // input buffer; compressed writes add the output buffer before publishing
    // either allocation to the ABI-shaped state.
    pub(crate) fn allocate_write(
        want: ::core::ffi::c_uint,
        direct: ::core::ffi::c_int,
    ) -> Option<Self> {
        let input = gz_buffer(want << 1)?;
        let output = if direct == 0 {
            Some(gz_buffer(want)?)
        } else {
            None
        };
        Some(Self {
            size: want,
            input: Some(input),
            output,
            input_cursor: None,
            deflate_state: None,
            output_cursor: None,
        })
    }

    // Allocate gzip read storage as one owner transaction.  Keeping the
    // paired buffers together ensures that a failed second allocation drops
    // the first immediately, and lets read initialization publish the owner
    // in one assignment before projecting ABI cursors.
    pub(crate) fn allocate_read(want: ::core::ffi::c_uint) -> Option<Self> {
        let input = gz_buffer(want)?;
        let output = gz_buffer(want << 1)?;
        Some(Self {
            size: want,
            input: Some(input),
            output: Some(output),
            input_cursor: Some(GzCodecInput::empty()),
            deflate_state: None,
            output_cursor: None,
        })
    }

    // Construct the compressed-output view only after both write allocations
    // have been installed.  Direct handles intentionally have no output
    // allocation, whereas compressed handles must obtain one before an ABI
    // deflate cursor is published.  Do not include the input allocation: the
    // next facade step needs to borrow it separately as the bounded request.
    pub(crate) fn write_output_view(&mut self) -> Option<GzWriteOutputView<'_>> {
        let size = self.size;
        self.input.as_deref()?;
        let output = self.output.as_deref_mut()?;
        Some(GzWriteOutputView { size, output })
    }

    // Reset the complete allocation transaction after codec initialization
    // fails.  Do not leave one side of the paired owner installed: later
    // retries must follow the same allocation path as a fresh handle.
    pub(crate) fn clear(&mut self) {
        self.input = None;
        self.output = None;
        self.input_cursor = None;
        self.deflate_state = None;
        self.output_cursor = None;
        self.size = 0;
    }

    // The completed read cursor belongs to the output allocation.  Keep the
    // owner as the source of truth between buffered-read operations; `x` is
    // only the ABI projection published for the public gzgetc macro.
    pub(crate) fn output_cursor(&self) -> Option<&GzCodecOutputCursor> {
        self.output_cursor.as_ref()
    }

    pub(crate) fn set_output_cursor(&mut self, cursor: GzCodecOutputCursor) {
        self.output_cursor = Some(cursor);
    }

    pub(crate) fn clear_output_cursor(&mut self) {
        self.output_cursor = None;
    }
}

// This is the scalar portion of a gzip handle that position queries need.
// Keep it pointer-free so the query rules can move out of the ABI state before
// the resource-owning gzip facade is introduced.
struct GzPosition {
    mode: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
}

// The offset query needs one resource in addition to the scalar position
// state.  Borrow it separately so the query implementation stays independent
// of the ABI-shaped gzip handle.
struct GzOffsetQuery<'a> {
    position: GzPosition,
    fd: Option<&'a rustix::fd::OwnedFd>,
    buffered_input: crate::stdlib::uInt,
}

// A checked, pointer-free view of unread bytes in gzip's owned output buffer.
// The ABI cursor is converted to its address at the state boundary, leaving
// read/seek policy with an index and a slice only.  Keep this small view in
// gzlib so the remaining buffered read paths can adopt the same proof without
// teaching their cores about `gzFile_s::next`.
pub(crate) struct GzBufferedCursor<'a> {
    buffer: &'a [u8],
    start: usize,
    have: usize,
}

// The write side uses the same cursor representation, but needs exclusive
// access to append bytes after the already-buffered input.  Keep that proof in
// a pointer-free view so callers only publish the resulting byte count back to
// the ABI stream fields.
pub(crate) struct GzBufferedInput<'a> {
    buffer: &'a mut [u8],
    start: usize,
    have: usize,
}

// The codec owner keeps its unread input as a checked index and byte count.
// ABI boundaries convert a stream cursor to this form for the duration of a
// call, while refill and decompression state keep only this pointer-free
// representation.
pub struct GzCodecInput {
    cursor: usize,
    available: crate::stdlib::uInt,
}

// This is the complete pointer-free request made to gzip's embedded inflate
// codec.  It owns the checked input cursor needed to account for consumption,
// so a future embedded-codec owner can consume the request and return a
// `GzCodecResult` without borrowing an ABI `z_stream` or a separate gzip
// state-machine request object.
pub(crate) struct GzEmbeddedInflateCall<'input, 'output> {
    input: &'input [u8],
    input_cursor: GzCodecInput,
    output: GzCodecOutputView<'output>,
}

// Own one complete bounded gzip-to-inflate request while the current ABI
// adapter projects it into `z_stream`.  The owner deliberately contains no
// stream or raw cursor: it is the hand-off seam for moving the embedded codec
// itself out of gzip state once the inflate-state owner is available.
pub(crate) struct GzEmbeddedInflateDispatch<'input, 'output> {
    call: GzEmbeddedInflateCall<'input, 'output>,
}

// The write-side counterpart starts with the one part of an embedded deflate
// call that gzip already owns outright: its bounded compressed-output buffer.
// Keep that borrow in an owner rather than reconstructing a cursor from
// `gz_state` in setup.  `call()` extends it with the bounded input request
// and the scalar input count used by deflate, so the next step can move a
// complete embedded-deflate dispatch out of the ABI-shaped gzip state.
pub(crate) struct GzEmbeddedDeflateSetup<'a> {
    output: GzCodecOutputView<'a>,
}

// A complete pointer-free request for gzip's embedded deflate codec.  The
// current adapter still publishes these views to an ABI `z_stream`, but this
// owner keeps both cursor bounds with the request that supplied them.
pub(crate) struct GzEmbeddedDeflateCall<'input, 'output> {
    input: &'input [u8],
    input_available: crate::stdlib::uInt,
    output: GzCodecOutputView<'output>,
}

// Keep the complete bounded request together with the scalar codec state it
// will replace.  This is the write-side handoff boundary for a future
// embedded-deflate owner: its ABI adapter can publish this one object, then
// return the checked next state without reopening either gzip buffer cursor.
pub(crate) struct GzEmbeddedDeflateDispatch<'input, 'output> {
    state: GzEmbeddedDeflateState,
    call: GzEmbeddedDeflateCall<'input, 'output>,
}

// Snapshot the scalar result immediately after the temporary ABI stream
// projection.  Keeping it pointer-free mirrors the inflate-side result owner
// and avoids a future deflate loop having to inspect advanced raw cursors.
pub(crate) struct GzEmbeddedDeflateResult {
    pub(crate) result: ::core::ffi::c_int,
    pub(crate) remaining_input: crate::stdlib::uInt,
    pub(crate) output_available: crate::stdlib::uInt,
    pub(crate) total_in: crate::stdlib::uLong,
    pub(crate) total_out: crate::stdlib::uLong,
}

// A completed deflate request retains both the codec's scalar counters and
// the checked amount of each borrowed buffer it consumed.  This is the
// write-side equivalent of `GzCodecResult`: the next owner can apply this
// value without revisiting ABI cursors after the temporary stream projection
// has ended.
pub(crate) struct GzEmbeddedDeflateProgress {
    pub(crate) result: ::core::ffi::c_int,
    pub(crate) remaining_input: crate::stdlib::uInt,
    pub(crate) output_available: crate::stdlib::uInt,
    pub(crate) input_used: crate::stdlib::uInt,
    pub(crate) output_used: crate::stdlib::uInt,
    pub(crate) total_in: crate::stdlib::uLong,
    pub(crate) total_out: crate::stdlib::uLong,
}

// The scalar portion of gzip's embedded-deflate stream.  Keep it with the
// bounded request/result handoff rather than having gzip state recompute
// cursor progress after every ABI projection.  A future codec owner can hold
// this directly while its boundary adapter alone publishes `z_stream`.
#[derive(Clone, Copy)]
pub(crate) struct GzEmbeddedDeflateState {
    input_available: crate::stdlib::uInt,
    output_available: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
}

pub(crate) struct GzCodecOutputView<'a> {
    bytes: &'a mut [u8],
}

impl<'a> GzBufferedCursor<'a> {
    pub(crate) fn from_index(buffer: &'a [u8], start: usize, have: u32) -> Option<Self> {
        let have = have as usize;
        let end = start.checked_add(have)?;
        buffer.get(start..end)?;
        Some(Self {
            buffer,
            start,
            have,
        })
    }

    pub(crate) fn from_owned_buffer(
        buffer: &'a [u8],
        cursor_address: usize,
        have: u32,
    ) -> Option<Self> {
        let start = cursor_address.checked_sub(buffer.as_ptr().addr())?;
        let have = have as usize;
        let end = start.checked_add(have)?;
        buffer.get(start..end)?;
        Some(Self {
            buffer,
            start,
            have,
        })
    }

    fn unread(&self) -> &'a [u8] {
        // Construction checked this exact range against `buffer`.
        &self.buffer[self.start..self.start + self.have]
    }

    // Borrow a checked prefix of the unread buffer and return the index that
    // follows it.  Read-side callers can use this rather than rebuilding a
    // range from the ABI cursor for every buffered copy.
    pub(crate) fn consume(&self, len: usize) -> Option<(&'a [u8], usize)> {
        let bytes = self.unread().get(..len)?;
        Some((bytes, self.start.checked_add(len)?))
    }

    // Copy a bounded prefix of the checked unread range and return the
    // pointer-free cursor/count projection for the remaining bytes.  This
    // keeps buffered gzip reads from doing their own slice and accounting
    // work after the ABI cursor has been validated at the boundary.
    pub(crate) fn copy_into(&self, output: &mut [u8]) -> Option<(usize, u32)> {
        let (input, next) = self.consume(output.len())?;
        output.copy_from_slice(input);
        let have = self.have.checked_sub(output.len())?;
        Some((next, u32::try_from(have).ok()?))
    }

    // Return one buffered byte together with the next checked buffer index.
    // Keeping cursor advancement as an index lets read-side policy consume
    // buffered output without retaining the ABI cursor pointer.
    pub(crate) fn consume_one(&self) -> Option<(u8, usize)> {
        let byte = *self.unread().first()?;
        Some((byte, self.start.checked_add(1)?))
    }

    // Advance within the checked unread range.  This keeps skip paths from
    // doing arithmetic on the ABI cursor after it has been validated.
    pub(crate) fn advance(&self, len: usize) -> Option<(usize, u32)> {
        let (_, next) = self.consume(len)?;
        let have = self.have.checked_sub(len)?;
        Some((next, u32::try_from(have).ok()?))
    }

    // Prepend a byte to the owned output buffer.  For a nonempty cursor,
    // validate the entire advertised unread range before shifting or writing;
    // the caller only has to project the resulting checked index back to the
    // ABI cursor.
    pub(crate) fn prepend(
        buffer: &mut [u8],
        cursor_address: usize,
        have: u32,
        byte: u8,
    ) -> Option<(usize, u32)> {
        if have == 0 {
            let next = buffer.len().checked_sub(1)?;
            buffer[next] = byte;
            return Some((next, 1));
        }

        let start = {
            let cursor = GzBufferedCursor::from_owned_buffer(&*buffer, cursor_address, have)?;
            cursor.start
        };
        let have_usize = have as usize;
        if have_usize >= buffer.len() {
            return None;
        }
        let next = if start == 0 {
            let shifted = buffer.len().checked_sub(have_usize)?;
            buffer.copy_within(0..have_usize, shifted);
            shifted
        } else {
            start
        };
        let next = next.checked_sub(1)?;
        buffer[next] = byte;
        Some((next, have.checked_add(1)?))
    }
}

impl<'a> GzBufferedInput<'a> {
    // An empty cursor is valid even when the ABI cursor is null.  Limit the
    // visible storage to the caller's initialized capacity before any later
    // refill or compaction operation uses it.
    pub(crate) fn empty(buffer: &'a mut [u8], capacity: usize) -> Option<Self> {
        let buffer = buffer.get_mut(..capacity)?;
        Some(Self {
            buffer,
            start: 0,
            have: 0,
        })
    }

    pub(crate) fn from_owned_buffer(
        buffer: &'a mut [u8],
        cursor_address: usize,
        have: u32,
    ) -> Option<Self> {
        let start = cursor_address.checked_sub(buffer.as_ptr().addr())?;
        let have = have as usize;
        let end = start.checked_add(have)?;
        buffer.get(start..end)?;
        Some(Self {
            buffer,
            start,
            have,
        })
    }

    // The gzip owner stores an input cursor as a checked index.  Keep this
    // constructor alongside the ABI-cursor conversion above so refill code
    // can operate entirely on the owner representation once the boundary has
    // made that conversion.
    pub(crate) fn from_index(buffer: &'a mut [u8], start: usize, have: u32) -> Option<Self> {
        let have = have as usize;
        let end = start.checked_add(have)?;
        buffer.get(start..end)?;
        Some(Self {
            buffer,
            start,
            have,
        })
    }

    // Compact the checked unread range and expose the remaining initialized
    // suffix for a refill.  Read-side code can then keep its byte count as an
    // index instead of retaining an ABI stream cursor between I/O calls.
    pub(crate) fn refill_target(&mut self) -> Option<&mut [u8]> {
        if self.start != 0 {
            let end = self.start.checked_add(self.have)?;
            self.buffer.copy_within(self.start..end, 0);
            self.start = 0;
        }
        self.buffer.get_mut(self.have..)
    }

    // Publish bytes added through the refill target only after checking that
    // they still fit in this owned buffer.  The returned u32 is suitable for
    // the ABI avail_in field at the boundary.
    pub(crate) fn extend(&mut self, added: usize) -> Option<u32> {
        let have = self.have.checked_add(added)?;
        if have > self.buffer.len() {
            return None;
        }
        self.have = have;
        u32::try_from(have).ok()
    }

    pub(crate) fn append(&mut self, input: &[u8]) -> usize {
        let end = match self.start.checked_add(self.have) {
            Some(end) => end,
            None => return 0,
        };
        let copy = input.len().min(self.buffer.len().saturating_sub(end));
        self.buffer[end..end + copy].copy_from_slice(&input[..copy]);
        self.have += copy;
        copy
    }

    pub(crate) fn have(&self) -> Option<u32> {
        u32::try_from(self.have).ok()
    }

    // Return the checked owner representation after compaction/refill.  The
    // caller may publish it to an ABI cursor at the boundary, but core code
    // should retain only this index and count.
    pub(crate) fn cursor(&self) -> Option<(usize, u32)> {
        Some((self.start, u32::try_from(self.have).ok()?))
    }
}

impl GzCodecInput {
    pub(crate) fn empty() -> Self {
        Self {
            cursor: 0,
            available: 0,
        }
    }

    // A codec input cursor is valid only when its entire advertised range is
    // within gzip's owned input allocation. Preserve the zero-length case:
    // it may carry a null ABI cursor and therefore has no address to check.
    pub(crate) fn from_owned_buffer(
        buffer: &[u8],
        cursor_address: usize,
        available: u32,
    ) -> Option<Self> {
        if available == 0 {
            return Some(Self::empty());
        }
        let start = cursor_address.checked_sub(buffer.as_ptr().addr())?;
        let end = start.checked_add(available as usize)?;
        buffer.get(start..end)?;
        Some(Self {
            cursor: start,
            available,
        })
    }

    // Buffered writers already know their owned-buffer offset.  Preserve
    // that pointer-free proof instead of publishing a temporary ABI cursor
    // solely to recover it before the next embedded-deflate request.
    pub(crate) fn from_index(buffer: &[u8], cursor: usize, available: u32) -> Option<Self> {
        let end = cursor.checked_add(available as usize)?;
        buffer.get(cursor..end)?;
        Some(Self { cursor, available })
    }

    pub(crate) fn available(&self) -> u32 {
        self.available
    }

    pub(crate) fn cursor(&self) -> usize {
        self.cursor
    }

    pub(crate) fn bytes<'a>(&self, buffer: &'a [u8]) -> Option<&'a [u8]> {
        let end = self.cursor.checked_add(self.available as usize)?;
        buffer.get(self.cursor..end)
    }

    pub(crate) fn update(&mut self, cursor: usize, available: u32) {
        self.cursor = cursor;
        self.available = available;
    }

    // The codec reports consumption by reducing `avail_in`.  The call was
    // built from this checked cursor, so a remaining count within the
    // advertised range is enough to retain a pointer-free post-call cursor.
    // This avoids deriving the next cursor from the ABI stream's raw pointer.
    pub(crate) fn after_codec(&self, remaining: u32) -> Option<Self> {
        let consumed = self.available.checked_sub(remaining)?;
        Some(Self {
            cursor: self.cursor.checked_add(consumed as usize)?,
            available: remaining,
        })
    }
}

impl<'input, 'output> GzEmbeddedInflateCall<'input, 'output> {
    pub(crate) fn input(&self) -> &'input [u8] {
        self.input
    }

    pub(crate) fn input_available(&self) -> crate::stdlib::uInt {
        self.input_cursor.available()
    }

    pub(crate) fn output_available(&self) -> crate::stdlib::uInt {
        self.output.bytes.len() as crate::stdlib::uInt
    }

    // The request owns the bounded writable output view for this codec pass.
    // The ABI projection may turn it into a cursor for `inflate()`, but no
    // gzip state-machine code has to construct that cursor from a capacity.
    pub(crate) fn output_mut(&mut self) -> &mut [u8] {
        self.output.bytes_mut()
    }

    // Consume the bounded request when publishing the scalar codec result.
    // This keeps checked input-progress accounting with the request that was
    // actually dispatched, rather than reconstructing it from an ABI cursor.
    pub(crate) fn into_codec_result(
        self,
        snapshot: GzEmbeddedInflateResult,
    ) -> Option<GzCodecResult> {
        Some(GzCodecResult {
            result: snapshot.result,
            input: self.input_cursor.after_codec(snapshot.remaining_input)?,
            output_available: snapshot.output_available,
            total_in: snapshot.total_in,
            total_out: snapshot.total_out,
            data_error_message: if snapshot.result == crate::zlib_h::Z_DATA_ERROR {
                snapshot.data_error_message()
            } else {
                None
            },
        })
    }
}

impl<'input, 'output> GzEmbeddedInflateDispatch<'input, 'output> {
    pub(crate) fn new(call: GzEmbeddedInflateCall<'input, 'output>) -> Self {
        Self { call }
    }

    pub(crate) fn input(&self) -> &'input [u8] {
        self.call.input()
    }

    pub(crate) fn input_available(&self) -> crate::stdlib::uInt {
        self.call.input_available()
    }

    pub(crate) fn output_available(&self) -> crate::stdlib::uInt {
        self.call.output_available()
    }

    pub(crate) fn output_mut(&mut self) -> &mut [u8] {
        self.call.output_mut()
    }

    // Finishing consumes the exact request that supplied the stream cursors,
    // so the result can retain checked input progress without observing the
    // ABI stream after the projection has ended.
    pub(crate) fn finish(self, snapshot: GzEmbeddedInflateResult) -> Option<GzCodecResult> {
        self.call.into_codec_result(snapshot)
    }
}

impl<'a> GzEmbeddedDeflateSetup<'a> {
    // The paired write allocation established by `GzBuffers` is the only
    // source of this output view.  Direct handles have no compressed-output
    // allocation and therefore cannot construct an embedded-deflate setup.
    pub(crate) fn from_write_buffers(buffers: &'a mut GzWriteOutputView<'a>) -> Option<Self> {
        let size = buffers.size();
        Self::from_output(buffers.output_mut(), size)
    }

    // `gz_comp()` borrows the paired input and output allocations separately
    // for one complete codec request.  It therefore needs the same checked
    // setup without first borrowing all of `GzBuffers` through
    // `write_output_view()`.
    pub(crate) fn from_output(output: &'a mut [u8], size: crate::stdlib::uInt) -> Option<Self> {
        let available = usize::try_from(size).ok()?;
        Some(Self {
            output: GzCodecOutputView::prefix(output, available)?,
        })
    }

    pub(crate) fn output_available(&self) -> crate::stdlib::uInt {
        self.output.bytes.len() as crate::stdlib::uInt
    }

    // Publishing the ABI cursor remains at the codec boundary.  All callers
    // of this owner receive the checked, allocation-backed slice first.
    pub(crate) fn output_mut(&mut self) -> &mut [u8] {
        self.output.bytes_mut()
    }

    pub(crate) fn call<'input>(
        self,
        input: &'input [u8],
        input_available: crate::stdlib::uInt,
    ) -> Option<GzEmbeddedDeflateCall<'input, 'a>> {
        let output_available = crate::stdlib::uInt::try_from(self.output.bytes.len()).ok()?;
        let output_cursor = self.output.bytes.as_ptr().addr();
        self.call_at_output_cursor(input, input_available, output_cursor, output_available)
    }

    // A deflate call can resume partway through the owned output allocation.
    // Validate that ABI cursor projection as an address-relative bounded range
    // before handing the request to the codec, so the later gzip owner need
    // retain only this request rather than a raw `next_out` cursor.
    pub(crate) fn call_at_output_cursor<'input>(
        mut self,
        input: &'input [u8],
        input_available: crate::stdlib::uInt,
        output_cursor: usize,
        output_available: crate::stdlib::uInt,
    ) -> Option<GzEmbeddedDeflateCall<'input, 'a>> {
        let input_len = usize::try_from(input_available).ok()?;
        let output_start = output_cursor.checked_sub(self.output.bytes.as_ptr().addr())?;
        let output_end = output_start.checked_add(usize::try_from(output_available).ok()?)?;
        let output = GzCodecOutputView {
            bytes: self.output.bytes.get_mut(output_start..output_end)?,
        };
        Some(GzEmbeddedDeflateCall {
            input: input.get(..input_len)?,
            input_available,
            output,
        })
    }
}

impl<'input, 'output> GzEmbeddedDeflateCall<'input, 'output> {
    pub(crate) fn input(&self) -> &'input [u8] {
        self.input
    }

    pub(crate) fn input_available(&self) -> crate::stdlib::uInt {
        self.input_available
    }

    pub(crate) fn output_available(&self) -> crate::stdlib::uInt {
        self.output.bytes.len() as crate::stdlib::uInt
    }

    pub(crate) fn output_mut(&mut self) -> &mut [u8] {
        self.output.bytes_mut()
    }

    // The ABI codec reports progress only as remaining input/output counts.
    // The request's checked bounds turn those counters into a complete,
    // pointer-free result for the surrounding gzip state machine.
    pub(crate) fn finish(
        self,
        snapshot: GzEmbeddedDeflateResult,
    ) -> Option<GzEmbeddedDeflateProgress> {
        let remaining_input = usize::try_from(snapshot.remaining_input).ok()?;
        let output_available = usize::try_from(snapshot.output_available).ok()?;
        (remaining_input <= self.input.len() && output_available <= self.output.bytes.len())
            .then_some(GzEmbeddedDeflateProgress {
                result: snapshot.result,
                remaining_input: snapshot.remaining_input,
                output_available: snapshot.output_available,
                input_used: self.input_available.wrapping_sub(snapshot.remaining_input),
                output_used: (self.output.bytes.len() as crate::stdlib::uInt)
                    .wrapping_sub(snapshot.output_available),
                total_in: snapshot.total_in,
                total_out: snapshot.total_out,
            })
    }

    // Consume both the bounded request and its pre-call scalar stream state.
    // The resulting state is entirely pointer-free, so callers do not need
    // to infer consumption from advanced ABI cursors once this projection
    // ends.
    pub(crate) fn finish_state(
        self,
        state: GzEmbeddedDeflateState,
        snapshot: GzEmbeddedDeflateResult,
    ) -> Option<(GzEmbeddedDeflateState, GzEmbeddedDeflateProgress)> {
        let progress = self.finish(snapshot)?;
        let input_available = state.input_available.checked_sub(progress.input_used)?;
        (input_available == progress.remaining_input).then_some((
            GzEmbeddedDeflateState {
                input_available,
                output_available: progress.output_available,
                total_in: progress.total_in,
                total_out: progress.total_out,
            },
            progress,
        ))
    }
}

impl<'input, 'output> GzEmbeddedDeflateDispatch<'input, 'output> {
    pub(crate) fn new(
        state: GzEmbeddedDeflateState,
        call: GzEmbeddedDeflateCall<'input, 'output>,
    ) -> Self {
        Self { state, call }
    }

    pub(crate) fn input(&self) -> &'input [u8] {
        self.call.input()
    }

    pub(crate) fn input_available(&self) -> crate::stdlib::uInt {
        self.call.input_available()
    }

    pub(crate) fn output_available(&self) -> crate::stdlib::uInt {
        self.call.output_available()
    }

    pub(crate) fn output_mut(&mut self) -> &mut [u8] {
        self.call.output_mut()
    }

    pub(crate) fn finish(
        self,
        snapshot: GzEmbeddedDeflateResult,
    ) -> Option<(GzEmbeddedDeflateState, GzEmbeddedDeflateProgress)> {
        self.call.finish_state(self.state, snapshot)
    }

    // Keep the temporary ABI projection behind the complete bounded request.
    // The callback receives only the two checked buffers and their advertised
    // extents, then returns scalar stream fields.  This lets the gzip write
    // loop consume the pointer-free result without reopening either cursor;
    // replacing the callback with an owned deflate core will not change the
    // surrounding state machine.
    pub(crate) fn dispatch(
        self,
        invoke: impl FnOnce(
            &'input [u8],
            crate::stdlib::uInt,
            &mut [u8],
            crate::stdlib::uInt,
        ) -> GzEmbeddedDeflateResult,
    ) -> Option<(GzEmbeddedDeflateState, GzEmbeddedDeflateProgress)> {
        let GzEmbeddedDeflateDispatch { state, call } = self;
        let GzEmbeddedDeflateCall {
            input,
            input_available,
            mut output,
        } = call;
        let output_available = crate::stdlib::uInt::try_from(output.bytes.len()).ok()?;
        let snapshot = invoke(input, input_available, output.bytes_mut(), output_available);
        let remaining_input = usize::try_from(snapshot.remaining_input).ok()?;
        let remaining_output = usize::try_from(snapshot.output_available).ok()?;
        if remaining_input > input.len() || remaining_output > output.bytes.len() {
            return None;
        }
        let input_used = input_available.checked_sub(snapshot.remaining_input)?;
        let output_used = output_available.checked_sub(snapshot.output_available)?;
        let input_available = state.input_available.checked_sub(input_used)?;
        (input_available == snapshot.remaining_input).then_some((
            GzEmbeddedDeflateState {
                input_available,
                output_available: snapshot.output_available,
                total_in: snapshot.total_in,
                total_out: snapshot.total_out,
            },
            GzEmbeddedDeflateProgress {
                result: snapshot.result,
                remaining_input: snapshot.remaining_input,
                output_available: snapshot.output_available,
                input_used,
                output_used,
                total_in: snapshot.total_in,
                total_out: snapshot.total_out,
            },
        ))
    }
}

impl GzEmbeddedDeflateState {
    pub(crate) fn new(
        input_available: crate::stdlib::uInt,
        output_available: crate::stdlib::uInt,
        total_in: crate::stdlib::uLong,
        total_out: crate::stdlib::uLong,
    ) -> Self {
        Self {
            input_available,
            output_available,
            total_in,
            total_out,
        }
    }

    pub(crate) fn input_available(&self) -> crate::stdlib::uInt {
        self.input_available
    }

    pub(crate) fn output_available(&self) -> crate::stdlib::uInt {
        self.output_available
    }

    pub(crate) fn total_in(&self) -> crate::stdlib::uLong {
        self.total_in
    }

    pub(crate) fn total_out(&self) -> crate::stdlib::uLong {
        self.total_out
    }
}

impl<'a> GzCodecOutputView<'a> {
    // The caller chooses the exact initialized prefix a codec operation may
    // write.  Keeping this bounded view separate prevents later transitions
    // from rebuilding a raw output range from `avail_out`.
    pub(crate) fn prefix(buffer: &'a mut [u8], available: usize) -> Option<Self> {
        Some(Self {
            bytes: buffer.get_mut(..available)?,
        })
    }

    pub(crate) fn bytes_mut(&mut self) -> &mut [u8] {
        self.bytes
    }
}

impl GzPosition {
    fn active(&self) -> bool {
        self.mode == crate::gzguts_h::GZ_READ || self.mode == crate::gzguts_h::GZ_WRITE
    }

    fn tell(&self) -> crate::stdlib::off64_t {
        if !self.active() {
            return -1 as crate::stdlib::off64_t;
        }
        self.pos
            + if self.past != 0 {
                0 as crate::stdlib::off64_t
            } else {
                self.skip
            }
    }

    fn offset(
        &self,
        current: crate::stdlib::off64_t,
        buffered_input: crate::stdlib::uInt,
    ) -> crate::stdlib::off64_t {
        if !self.active() || current == -1 as crate::stdlib::off64_t {
            return -1 as crate::stdlib::off64_t;
        }
        if self.mode == crate::gzguts_h::GZ_READ {
            current - buffered_input as crate::stdlib::off64_t
        } else {
            current
        }
    }
}

// Pointer-free seek policy.  The ABI-facing implementation applies this plan
// to the gzip state and performs I/O; a future owner facade can reuse the
// same checked transition without borrowing the raw state.
enum GzSeekAction {
    Direct {
        seek_by: crate::stdlib::off64_t,
        position: crate::stdlib::off64_t,
    },
    Rewind {
        offset: crate::stdlib::off64_t,
    },
    Skip {
        offset: crate::stdlib::off64_t,
    },
    Reject,
}

struct GzSeekPlan {
    clear_skip: bool,
    action: GzSeekAction,
}

fn gzseek_plan(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    pos: crate::stdlib::off64_t,
    past: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    how: ::core::ffi::c_int,
    have: crate::stdlib::uInt,
    mut offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
) -> Option<GzSeekPlan> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR {
        return None;
    }
    if whence != crate::stdlib::SEEK_SET && whence != crate::stdlib::SEEK_CUR {
        return None;
    }
    let clear_skip = whence == crate::stdlib::SEEK_CUR;
    if whence == crate::stdlib::SEEK_SET {
        offset -= pos;
    } else {
        offset += if past != 0 { 0 } else { skip };
    }
    if mode == crate::gzguts_h::GZ_READ && how == crate::gzguts_h::COPY && pos + offset >= 0 {
        return Some(GzSeekPlan {
            clear_skip,
            action: GzSeekAction::Direct {
                seek_by: offset - have as crate::stdlib::off64_t,
                position: pos + offset,
            },
        });
    }
    if offset < 0 {
        if mode != crate::gzguts_h::GZ_READ {
            return Some(GzSeekPlan {
                clear_skip,
                action: GzSeekAction::Reject,
            });
        }
        offset += pos;
        if offset < 0 {
            return Some(GzSeekPlan {
                clear_skip,
                action: GzSeekAction::Reject,
            });
        }
        return Some(GzSeekPlan {
            clear_skip,
            action: GzSeekAction::Rewind { offset },
        });
    }
    Some(GzSeekPlan {
        clear_skip,
        action: GzSeekAction::Skip { offset },
    })
}

pub fn gzeof(mode: ::core::ffi::c_int, past: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if mode == crate::gzguts_h::GZ_READ {
        past
    } else {
        0 as ::core::ffi::c_int
    }
}

pub(crate) fn gz_clear_error(message: &mut Option<Box<[u8]>>, error: &mut ::core::ffi::c_int) {
    *message = None;
    *error = crate::zlib_h::Z_OK;
}

// Error publication belongs to gzip's safe state machine, not to the
// ABI-shaped handle.  Keep all of the mutable error fields in one
// pointer-free view so read and write operations can share the transition
// while a later owner facade removes their direct dependency on `gz_state`.
pub(crate) struct GzErrorState<'a> {
    pub(crate) message: &'a mut Option<Box<[u8]>>,
    pub(crate) error: &'a mut ::core::ffi::c_int,
    pub(crate) buffered: &'a mut ::core::ffi::c_uint,
    pub(crate) again: ::core::ffi::c_int,
    pub(crate) path: Option<&'a [u8]>,
}

impl GzErrorState<'_> {
    pub(crate) fn clear(&mut self) {
        gz_clear_error(self.message, self.error);
    }

    pub(crate) fn set(&mut self, err: ::core::ffi::c_int, message: Option<&[u8]>) {
        *self.message = None;
        if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && self.again == 0 {
            *self.buffered = 0;
        }
        *self.error = err;
        let Some(message) = message else {
            return;
        };
        if err == crate::zlib_h::Z_MEM_ERROR {
            return;
        }
        let Some(len) = self
            .path
            .and_then(|path| path.len().checked_add(message.len()))
            .and_then(|len| len.checked_add(3))
        else {
            *self.error = crate::zlib_h::Z_MEM_ERROR;
            return;
        };
        let mut text = Vec::new();
        if text.try_reserve_exact(len).is_err() {
            *self.error = crate::zlib_h::Z_MEM_ERROR;
            return;
        }
        // A message is only formatted when a gzip path was successfully
        // retained by the owner, matching the existing error contract.
        text.extend_from_slice(self.path.unwrap());
        text.extend_from_slice(b": ");
        text.extend_from_slice(message);
        text.push(0);
        *self.message = Some(text.into_boxed_slice());
    }
}

// Keep the error-state transition independent of the ABI-shaped gzip handle.
// The callers that still hold that handle only provide the scalar fields and
// owned byte views; a later gzip owner facade can use this directly.
pub(crate) fn gz_set_error(
    stored_message: &mut Option<Box<[u8]>>,
    error: &mut ::core::ffi::c_int,
    buffered: &mut ::core::ffi::c_uint,
    again: ::core::ffi::c_int,
    path: Option<&[u8]>,
    err: ::core::ffi::c_int,
    message: Option<&[u8]>,
) {
    GzErrorState {
        message: stored_message,
        error,
        buffered,
        again,
        path,
    }
    .set(err, message);
}

fn gzbuffer_want(
    mode: ::core::ffi::c_int,
    current_size: ::core::ffi::c_uint,
    requested_size: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    if current_size != 0 as ::core::ffi::c_uint {
        return None;
    }
    if requested_size.wrapping_shl(1) < requested_size {
        return None;
    }
    Some(if requested_size < 8 as ::core::ffi::c_uint {
        8 as ::core::ffi::c_uint
    } else {
        requested_size
    })
}

pub(crate) fn gz_buffer(size: ::core::ffi::c_uint) -> Option<Box<[u8]>> {
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size as usize).ok()?;
    bytes.resize(size as usize, 0);
    Some(bytes.into_boxed_slice())
}

struct GzReadResetFields {
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
}

// This is the pointer-free scalar part of gzip's embedded codec stream that
// reset and seek transitions currently own.  Keeping it as a separate value
// starts the stream-owner split without letting an ABI `z_stream` leak into
// those safe transitions.  Carry both availability counters here: reset
// clears only pending input, while output capacity remains owned by the
// active codec operation.
pub(crate) struct GzCodecCounters {
    available_input: crate::stdlib::uInt,
    available_output: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
}

impl GzCodecCounters {
    pub(crate) fn from_stream_fields(
        available_input: crate::stdlib::uInt,
        available_output: crate::stdlib::uInt,
        total_in: crate::stdlib::uLong,
        total_out: crate::stdlib::uLong,
    ) -> Self {
        Self {
            available_input,
            available_output,
            total_in,
            total_out,
        }
    }

    // zlib reset clears pending input but deliberately retains the stream's
    // cumulative counters.  Keep that distinction in the safe projection so
    // a future owned codec state can preserve the same observable values.
    fn reset_input(&mut self) {
        self.available_input = 0;
    }

    fn record_input(&mut self, input: &GzCodecInput) {
        self.available_input = input.available();
    }

    fn record_output(&mut self, available: crate::stdlib::uInt) {
        self.available_output = available;
    }

    pub(crate) fn available_output(&self) -> crate::stdlib::uInt {
        self.available_output
    }

    pub(crate) fn available_input(&self) -> crate::stdlib::uInt {
        self.available_input
    }

    pub(crate) fn total_in(&self) -> crate::stdlib::uLong {
        self.total_in
    }

    pub(crate) fn total_out(&self) -> crate::stdlib::uLong {
        self.total_out
    }
}

// A codec operation reports output progress by decreasing `avail_out`. Keep
// that accounting separate from the ABI cursor so gzip can eventually hand a
// bounded output view directly to its owned codec state. The subtraction is
// deliberately wrapping: this mirrors the translated stream accounting until
// the owner/view split can reject a malformed embedded stream earlier.
pub(crate) struct GzCodecOutput {
    capacity: crate::stdlib::uInt,
    available: crate::stdlib::uInt,
}

// A completed codec pass describes its output using the owned buffer's
// checked index and byte count.  Keep this separate from the ABI pointer so
// the decompression transition never needs to retain `strm.next_out` after
// the codec call returns.
pub(crate) struct GzCodecOutputCursor {
    start: usize,
    have: crate::stdlib::uInt,
}

impl GzCodecOutput {
    pub(crate) fn new(capacity: usize) -> Option<Self> {
        let capacity = crate::stdlib::uInt::try_from(capacity).ok()?;
        Some(Self {
            capacity,
            available: capacity,
        })
    }

    pub(crate) fn available(&self) -> crate::stdlib::uInt {
        self.available
    }

    pub(crate) fn record_available(&mut self, available: crate::stdlib::uInt) {
        self.available = available;
    }

    pub(crate) fn has_output(&self) -> bool {
        self.available < self.capacity
    }

    pub(crate) fn written(&self) -> crate::stdlib::uInt {
        self.capacity.wrapping_sub(self.available)
    }

    fn completed_cursor(&self) -> GzCodecOutputCursor {
        GzCodecOutputCursor {
            start: 0,
            have: self.written(),
        }
    }
}

impl GzCodecOutputCursor {
    pub(crate) fn from_owned_buffer(buffer: &[u8], start: usize, have: u32) -> Option<Self> {
        GzBufferedCursor::from_index(buffer, start, have)?;
        Some(Self { start, have })
    }

    pub(crate) fn start(&self) -> usize {
        self.start
    }

    pub(crate) fn have(&self) -> crate::stdlib::uInt {
        self.have
    }

    pub(crate) fn buffered<'a>(&self, buffer: &'a [u8]) -> Option<GzBufferedCursor<'a>> {
        GzBufferedCursor::from_index(buffer, self.start, self.have)
    }

    pub(crate) fn advance(&self, len: usize) -> Option<Self> {
        let start = self.start.checked_add(len)?;
        let have = (self.have as usize).checked_sub(len)?;
        Some(Self {
            start,
            have: u32::try_from(have).ok()?,
        })
    }

    pub(crate) fn prepend(&mut self, buffer: &mut [u8], byte: u8) -> Option<()> {
        if self.have == 0 {
            self.start = buffer.len().checked_sub(1)?;
            buffer[self.start] = byte;
            self.have = 1;
            return Some(());
        }
        GzBufferedCursor::from_index(buffer, self.start, self.have)?;
        let have = self.have as usize;
        if have >= buffer.len() {
            return None;
        }
        if self.start == 0 {
            let shifted = buffer.len().checked_sub(have)?;
            buffer.copy_within(0..have, shifted);
            self.start = shifted;
        }
        self.start = self.start.checked_sub(1)?;
        buffer[self.start] = byte;
        self.have = self.have.checked_add(1)?;
        Some(())
    }
}

// The result of one embedded inflate call is entirely scalar.  Keep this
// state-machine decision separate from the ABI stream so gzip can eventually
// drive it from an owned codec facade instead of inspecting `z_stream` after
// each call.  In particular, the raw-copy fallback (`Junk`) is distinct from
// a corrupt compressed member even though inflate reports Z_DATA_ERROR for
// both.
pub(crate) enum GzDecompAction {
    Continue,
    Stop,
    Junk,
    StreamError,
    MemoryError,
    DataError,
}

pub(crate) struct GzDecompStep {
    pub(crate) produced_output: bool,
    pub(crate) action: GzDecompAction,
}

// Completing a gzip inflate pass publishes all of the pointer-free state that
// the ABI boundary must mirror back to `gz_state`.  Keep this as one owned
// snapshot so the eventual gzip owner can finish a codec operation without
// exposing individual stream cursor fields to the read layer.
pub(crate) struct GzDecompFinish {
    pub(crate) result: ::core::ffi::c_int,
    pub(crate) input: GzCodecInput,
    pub(crate) output: GzCodecOutputCursor,
    pub(crate) codec: GzCodecCounters,
    pub(crate) junk: ::core::ffi::c_int,
    pub(crate) eof: ::core::ffi::c_int,
    pub(crate) how: ::core::ffi::c_int,
}

// An embedded inflate pass returns this pointer-free snapshot to gzip's read
// state machine.  The small ABI projection that invokes inflate constructs it
// at the boundary; refill, result handling, and output accounting therefore
// never need to inspect a `z_stream` themselves.
pub(crate) struct GzCodecResult {
    pub(crate) result: ::core::ffi::c_int,
    pub(crate) input: GzCodecInput,
    pub(crate) output_available: crate::stdlib::uInt,
    pub(crate) total_in: crate::stdlib::uLong,
    pub(crate) total_out: crate::stdlib::uLong,
    pub(crate) data_error_message: Option<&'static [u8]>,
}

// This is the complete pointer-free result of one embedded inflate dispatch.
// The gzip boundary may still need an ABI `z_stream` to invoke inflate today,
// but it can immediately snapshot that projection here and leave all later
// cursor/result handling in the owned gzip state machine.  Keeping the
// diagnostic address as an integer is intentional: the stable known-message
// lookup never dereferences the ABI pointer.
pub(crate) struct GzEmbeddedInflateResult {
    result: ::core::ffi::c_int,
    remaining_input: crate::stdlib::uInt,
    output_available: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    data_error_message_address: Option<usize>,
}

impl GzEmbeddedInflateResult {
    pub(crate) fn from_stream_fields(
        result: ::core::ffi::c_int,
        remaining_input: crate::stdlib::uInt,
        output_available: crate::stdlib::uInt,
        total_in: crate::stdlib::uLong,
        total_out: crate::stdlib::uLong,
        data_error_message_address: Option<usize>,
    ) -> Self {
        Self {
            result,
            remaining_input,
            output_available,
            total_in,
            total_out,
            data_error_message_address,
        }
    }

    fn data_error_message(&self) -> Option<&'static [u8]> {
        self.data_error_message_address.and_then(|address| {
            crate::src::inflate::INFLATE_ERROR_MESSAGES
                .iter()
                .find(|known| known.as_ptr().addr() == address)
                .map(|known| &known[..known.len() - 1])
        })
    }
}

// The decompression loop mutates only these scalar gzip fields in response to
// an inflate result.  Keep that transition with the bounded output accounting
// so an eventual owned gzip codec can run the loop without borrowing the ABI
// `gz_state`; the current boundary only snapshots and republishes the values.
pub(crate) struct GzDecompState {
    output: GzCodecOutput,
    input: GzCodecInput,
    codec: GzCodecCounters,
    junk: ::core::ffi::c_int,
    eof: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
}

impl GzDecompState {
    pub(crate) fn new(
        output_capacity: usize,
        input: &GzCodecInput,
        mut codec: GzCodecCounters,
        junk: ::core::ffi::c_int,
        eof: ::core::ffi::c_int,
        how: ::core::ffi::c_int,
    ) -> Option<Self> {
        let output = GzCodecOutput::new(output_capacity)?;
        codec.record_input(input);
        codec.record_output(output.available());
        Some(Self {
            output,
            input: GzCodecInput {
                cursor: input.cursor,
                available: input.available,
            },
            codec,
            junk,
            eof,
            how,
        })
    }

    // The owning codec loop needs to decide when to refill without inspecting
    // an ABI stream. Keep that scalar cursor state with the decompression
    // transition; the current boundary only snapshots it around each codec
    // call until the stream owner/view split is complete.
    pub(crate) fn needs_input(&self) -> bool {
        self.input.available() == 0
    }

    pub(crate) fn record_input(&mut self, input: &GzCodecInput) {
        self.input.update(input.cursor(), input.available());
        self.codec.record_input(&self.input);
    }

    pub(crate) fn input(&self) -> &GzCodecInput {
        &self.input
    }

    pub(crate) fn input_mut(&mut self) -> &mut GzCodecInput {
        &mut self.input
    }

    pub(crate) fn output_available(&self) -> crate::stdlib::uInt {
        self.codec.available_output()
    }

    // Build the complete pointer-free input/output view for one codec pass.
    // The slice bounds validate the stored cursor before any ABI stream
    // cursor is published by the caller.
    pub(crate) fn embedded_inflate_call<'input, 'output>(
        &self,
        input: &'input [u8],
        output: &'output mut [u8],
    ) -> Option<GzEmbeddedInflateCall<'input, 'output>> {
        let output = GzCodecOutputView::prefix(output, self.output_available() as usize)?;
        Some(GzEmbeddedInflateCall {
            input: self.input.bytes(input)?,
            input_cursor: GzCodecInput {
                cursor: self.input.cursor,
                available: self.input.available,
            },
            output,
        })
    }

    pub(crate) fn record_inflate(&mut self, result: &GzCodecResult) -> GzDecompAction {
        self.output.record_available(result.output_available);
        self.codec.record_output(result.output_available);
        self.codec.total_in = result.total_in;
        self.codec.total_out = result.total_out;
        let produced_output = self.output.has_output();
        if produced_output {
            self.junk = 0;
        }
        let step = gz_decomp_step(
            result.result,
            result.output_available,
            produced_output,
            self.junk,
        );
        if matches!(step.action, GzDecompAction::Junk) {
            self.eof = 1;
            self.how = crate::gzguts_h::LOOK;
        }
        step.action
    }

    pub(crate) fn finish(mut self, result: ::core::ffi::c_int) -> GzDecompFinish {
        if result == crate::zlib_h::Z_STREAM_END {
            self.junk = 0;
            self.how = crate::gzguts_h::LOOK;
            GzDecompFinish {
                result: 0,
                input: self.input,
                output: self.output.completed_cursor(),
                codec: self.codec,
                junk: self.junk,
                eof: self.eof,
                how: self.how,
            }
        } else if result != crate::zlib_h::Z_OK {
            GzDecompFinish {
                result: -1,
                input: self.input,
                output: self.output.completed_cursor(),
                codec: self.codec,
                junk: self.junk,
                eof: self.eof,
                how: self.how,
            }
        } else {
            GzDecompFinish {
                result: 0,
                input: self.input,
                output: self.output.completed_cursor(),
                codec: self.codec,
                junk: self.junk,
                eof: self.eof,
                how: self.how,
            }
        }
    }
}

pub(crate) fn gz_decomp_step(
    result: ::core::ffi::c_int,
    output_available: crate::stdlib::uInt,
    produced_output: bool,
    junk: ::core::ffi::c_int,
) -> GzDecompStep {
    let action = if result == crate::zlib_h::Z_STREAM_ERROR || result == crate::zlib_h::Z_NEED_DICT
    {
        GzDecompAction::StreamError
    } else if result == crate::zlib_h::Z_MEM_ERROR {
        GzDecompAction::MemoryError
    } else if result == crate::zlib_h::Z_DATA_ERROR {
        if junk == 1 {
            GzDecompAction::Junk
        } else {
            GzDecompAction::DataError
        }
    } else if output_available == 0 || result == crate::zlib_h::Z_STREAM_END {
        GzDecompAction::Stop
    } else {
        GzDecompAction::Continue
    };
    GzDecompStep {
        produced_output,
        action,
    }
}

struct GzResetFields {
    have: ::core::ffi::c_uint,
    read: Option<GzReadResetFields>,
    reset: Option<::core::ffi::c_int>,
    again: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    pos: crate::stdlib::off64_t,
}

// The reset transition deliberately excludes the ABI cursors themselves.
// Keeping this view pointer-free lets the state-machine update live in safe
// code while the opaque-handle projection remains at the boundary.
struct GzResetState {
    mode: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
    eof: ::core::ffi::c_int,
    past: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
    junk: ::core::ffi::c_int,
    reset: ::core::ffi::c_int,
    again: ::core::ffi::c_int,
    skip: crate::stdlib::off64_t,
    err: ::core::ffi::c_int,
    msg: Option<Box<[u8]>>,
    pos: crate::stdlib::off64_t,
    codec: GzCodecCounters,
}

// This is the mutable, pointer-free portion of a gzip reset projection.  It
// deliberately contains only scalar fields and owned error storage, so the
// reset transition can be shared by the temporary ABI state and a future
// owned gzip handle without retaining a raw state reference.
struct GzResetTarget<'a> {
    have: &'a mut ::core::ffi::c_uint,
    eof: &'a mut ::core::ffi::c_int,
    past: &'a mut ::core::ffi::c_int,
    how: &'a mut ::core::ffi::c_int,
    junk: &'a mut ::core::ffi::c_int,
    reset: &'a mut ::core::ffi::c_int,
    again: &'a mut ::core::ffi::c_int,
    skip: &'a mut crate::stdlib::off64_t,
    err: &'a mut ::core::ffi::c_int,
    msg: &'a mut Option<Box<[u8]>>,
    pos: &'a mut crate::stdlib::off64_t,
    codec_available_input: &'a mut crate::stdlib::uInt,
    codec_available_output: &'a mut crate::stdlib::uInt,
    codec_total_in: &'a mut crate::stdlib::uLong,
    codec_total_out: &'a mut crate::stdlib::uLong,
    input_cursor: &'a mut Option<GzCodecInput>,
}

// `gzrewind` needs the same reset projection as the other gzip state
// transitions, plus only the owned descriptor and scalar seek state.  Keep
// that view pointer-free so the rewind implementation itself needs no unsafe
// operations or ABI-shaped state reference.
struct GzRewindState<'a> {
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    start: crate::stdlib::off64_t,
    fd: &'a rustix::fd::OwnedFd,
    reset: GzResetTarget<'a>,
    output_cursor: &'a mut Option<GzCodecOutputCursor>,
}

impl GzResetState {
    fn apply_reset(&mut self) {
        let fields = gz_reset_fields(self.mode);
        self.have = fields.have;
        if let Some(read) = fields.read {
            self.eof = read.eof;
            self.past = read.past;
            self.how = read.how;
            self.junk = read.junk;
        }
        if let Some(reset) = fields.reset {
            self.reset = reset;
        }
        self.again = fields.again;
        self.skip = fields.skip;
        gz_clear_error(&mut self.msg, &mut self.err);
        self.pos = fields.pos;
        self.codec.reset_input();
    }
}

fn reset_gz_target(mode: ::core::ffi::c_int, target: GzResetTarget<'_>) {
    let reset = gz_reset(GzResetState {
        mode,
        have: *target.have,
        eof: *target.eof,
        past: *target.past,
        how: *target.how,
        junk: *target.junk,
        reset: *target.reset,
        again: *target.again,
        skip: *target.skip,
        err: *target.err,
        msg: target.msg.take(),
        pos: *target.pos,
        codec: GzCodecCounters::from_stream_fields(
            *target.codec_available_input,
            *target.codec_available_output,
            *target.codec_total_in,
            *target.codec_total_out,
        ),
    });
    store_gz_reset_target(target, reset);
}

// Apply a pointer-free reset projection back to the ABI state fields.  The
// cursor itself deliberately stays outside this projection: callers can only
// publish a new cursor after validating the owned buffer it refers to.
fn store_gz_reset_target(target: GzResetTarget<'_>, reset: GzResetState) {
    *target.have = reset.have;
    *target.eof = reset.eof;
    *target.past = reset.past;
    *target.how = reset.how;
    *target.junk = reset.junk;
    *target.reset = reset.reset;
    *target.again = reset.again;
    *target.skip = reset.skip;
    *target.err = reset.err;
    *target.msg = reset.msg;
    *target.pos = reset.pos;
    *target.codec_available_input = reset.codec.available_input;
    *target.codec_available_output = reset.codec.available_output;
    *target.codec_total_in = reset.codec.total_in;
    *target.codec_total_out = reset.codec.total_out;
    // Seek-state snapshots use this owner cursor for reads, but buffered
    // writers now retain pending input here as well.  A write-side seek only
    // records `skip`; it must not discard bytes that still need deflating.
    if reset.mode == crate::gzguts_h::GZ_READ {
        *target.input_cursor = Some(GzCodecInput::empty());
    }
}

fn gz_reset_fields(mode: ::core::ffi::c_int) -> GzResetFields {
    let read = mode == crate::gzguts_h::GZ_READ;
    GzResetFields {
        have: 0,
        read: read.then_some(GzReadResetFields {
            eof: 0,
            past: 0,
            how: crate::gzguts_h::LOOK,
            junk: -1,
        }),
        reset: (!read).then_some(0),
        again: 0,
        skip: 0,
        pos: 0,
    }
}

struct GzOpenMode {
    mode: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    direct: ::core::ffi::c_int,
    oflag: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
}

// This is the pointer-free input for constructing an opaque gzip owner.  The
// ABI handle is still assembled at the boundary, but parsing, mode
// normalization, path ownership, and open-flag selection no longer need to
// borrow its embedded ABI stream.  A later C0 owner split can construct its
// safe state directly from this value.
struct GzOpenConfig {
    mode: GzOpenMode,
    path: Box<[u8]>,
}

// `gzdopen()` needs a byte-exact synthetic pathname for diagnostics, but that
// formatting is independent of both the ABI handle and raw-FD adoption.  Keep
// it as an owned, pointer-free value so the future owned-open constructor can
// use the same label without rebuilding it at an unsafe boundary.
struct GzFdPath {
    bytes: [u8; 7 + 3 * ::core::mem::size_of::<::core::ffi::c_int>()],
    len: usize,
}

// Keep path selection in the implementation layer. In particular, the FFI
// wrapper for gzdopen() must not assemble its diagnostic path itself.
enum GzOpenPath<'a> {
    Path(&'a [u8]),
    Descriptor(::core::ffi::c_int),
}

impl GzFdPath {
    fn new(fd: ::core::ffi::c_int) -> Self {
        // This is the same bound used by the C implementation: enough for
        // the literal label, every decimal digit of a C int, its sign, and
        // the NUL.
        let mut path = Self {
            bytes: [0; 7 + 3 * ::core::mem::size_of::<::core::ffi::c_int>()],
            len: 4,
        };
        path.bytes[..4].copy_from_slice(b"<fd:");
        if fd < 0 as ::core::ffi::c_int {
            path.bytes[path.len] = b'-';
            path.len += 1;
        }
        let mut digits = [0u8; 10];
        let mut value = fd.unsigned_abs();
        let mut count = 0usize;
        loop {
            digits[count] = (value % 10) as u8;
            count += 1;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        while count != 0 {
            count -= 1;
            path.bytes[path.len] = b'0' + digits[count];
            path.len += 1;
        }
        path.bytes[path.len] = b'>';
        path.len += 1;
        path
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

// The resource-owning portion of a freshly opened gzip handle.  This is the
// first complete pointer-free owner projection for gzip open: the ABI cursor
// and embedded codec stream are assembled only at the boundary below.  Keep
// the initial reset values here as well, so a later opaque gzip owner can use
// this constructor without reintroducing ABI-shaped state mutation.
struct GzOpenState {
    fd: rustix::fd::OwnedFd,
    path: Box<[u8]>,
    size: ::core::ffi::c_uint,
    want: ::core::ffi::c_uint,
    direct: ::core::ffi::c_int,
    start: crate::stdlib::off64_t,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    // A fresh handle starts at exactly the same pointer-free reset state that
    // rewind and seek use.  Keeping this as one owned transition avoids
    // re-expanding ABI cursor/stream fields during the eventual gzip owner
    // split.
    reset: GzResetState,
}

// Opening by path and adopting an already-open descriptor share the same
// pointer-free owner construction.  Keep the distinction explicit so raw-FD
// adoption remains at the ABI boundary, while flag adjustment and all gzip
// state initialization are reusable by the eventual owned-handle facade.
enum GzOpenSource {
    Path,
    Adopted(rustix::fd::OwnedFd),
}

impl GzOpenConfig {
    // Callers normalize the parsed mode before path allocation, matching
    // gz_open's existing failure ordering for invalid mode combinations.
    fn new(path: &[u8], mode: GzOpenMode) -> Option<Self> {
        let mut path_bytes = Vec::new();
        path_bytes.try_reserve_exact(path.len()).ok()?;
        path_bytes.extend_from_slice(path);
        Some(Self {
            mode,
            path: path_bytes.into_boxed_slice(),
        })
    }

    fn open_flags(&self) -> ::core::ffi::c_int {
        self.mode.oflag
            | crate::stdlib::O_LARGEFILE
            | if self.mode.mode == crate::gzguts_h::GZ_READ {
                crate::stdlib::O_RDONLY
            } else {
                crate::stdlib::O_WRONLY
                    | crate::stdlib::O_CREAT
                    | if self.mode.exclusive != 0 {
                        crate::stdlib::O_EXCL
                    } else {
                        0
                    }
                    | if self.mode.mode == crate::gzguts_h::GZ_WRITE {
                        crate::stdlib::O_TRUNC
                    } else {
                        crate::stdlib::O_APPEND
                    }
            }
    }

    // Opening a path is entirely within the pointer-free owner boundary.  In
    // particular, keep this separate from descriptor adoption: converting a
    // caller-owned raw descriptor remains an ABI-boundary operation, whereas
    // this path can already construct the eventual gzip owner directly.
    fn open(self, path: &[u8]) -> Option<GzOpenState> {
        let oflag = self.open_flags();
        let fd = match rustix::fs::open(
            path,
            rustix::fs::OFlags::from_bits_retain(oflag as u32),
            rustix::fs::Mode::from_raw_mode(0o666),
        ) {
            Ok(opened) => opened,
            Err(error) => {
                errno::set_errno(errno::Errno(error.raw_os_error()));
                return None;
            }
        };
        Some(self.into_open_state(fd))
    }

    fn into_open_state(mut self, fd: rustix::fd::OwnedFd) -> GzOpenState {
        if self.mode.mode == crate::gzguts_h::GZ_APPEND {
            let _ = rustix::fs::seek(&fd, rustix::fs::SeekFrom::End(0));
            self.mode.mode = crate::gzguts_h::GZ_WRITE;
        }
        let start = if self.mode.mode == crate::gzguts_h::GZ_READ {
            rustix::fs::tell(&fd)
                .map(|position| position as crate::stdlib::off64_t)
                .unwrap_or(0 as crate::stdlib::off64_t)
        } else {
            0 as crate::stdlib::off64_t
        };
        let reset = gz_reset(GzResetState {
            mode: self.mode.mode,
            have: 0,
            eof: 0,
            past: 0,
            how: crate::gzguts_h::LOOK,
            junk: 0,
            reset: 0,
            again: 0,
            skip: 0,
            err: crate::zlib_h::Z_OK,
            msg: None,
            pos: 0,
            codec: GzCodecCounters {
                available_input: 0,
                available_output: 0,
                total_in: 0,
                total_out: 0,
            },
        });
        GzOpenState {
            fd,
            path: self.path,
            size: 0,
            want: crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint,
            direct: self.mode.direct,
            start,
            level: self.mode.level,
            strategy: self.mode.strategy,
            reset,
        }
    }
}

fn gz_open_state(config: GzOpenConfig, path: &[u8], source: GzOpenSource) -> Option<GzOpenState> {
    match source {
        GzOpenSource::Path => config.open(path),
        GzOpenSource::Adopted(fd) => {
            let oflag = config.open_flags();
            if oflag & crate::stdlib::O_NONBLOCK != 0 {
                if let Ok(flags) = rustix::fs::fcntl_getfl(&fd) {
                    let _ = rustix::fs::fcntl_setfl(&fd, flags | rustix::fs::OFlags::NONBLOCK);
                }
            }
            if oflag & crate::stdlib::O_CLOEXEC != 0 {
                if let Ok(flags) = rustix::io::fcntl_getfd(&fd) {
                    let _ = rustix::io::fcntl_setfd(&fd, flags | rustix::io::FdFlags::CLOEXEC);
                }
            }
            Some(config.into_open_state(fd))
        }
    }
}

impl GzOpenMode {
    // Normalize the state selected by the mode string before allocating a
    // path or adopting a descriptor.  The result contains no ABI cursor or
    // raw stream state.
    fn normalize(mut self) -> Option<Self> {
        if self.mode == crate::gzguts_h::GZ_NONE {
            return None;
        }
        if self.mode == crate::gzguts_h::GZ_READ {
            if self.direct == 1 {
                return None;
            }
            if self.direct == 0 {
                self.direct = 1;
            }
        } else if self.direct == -1 {
            return None;
        }
        Some(self)
    }
}

fn parse_gz_open_mode(mode: &[u8]) -> Option<GzOpenMode> {
    let mut parsed = GzOpenMode {
        mode: crate::gzguts_h::GZ_NONE,
        level: crate::zlib_h::Z_DEFAULT_COMPRESSION,
        strategy: crate::zlib_h::Z_DEFAULT_STRATEGY,
        direct: 0,
        oflag: 0,
        exclusive: 0,
    };
    for &option in mode {
        if option.is_ascii_digit() {
            parsed.level = (option - b'0') as ::core::ffi::c_int;
            continue;
        }
        match option {
            b'r' => parsed.mode = crate::gzguts_h::GZ_READ,
            b'w' => parsed.mode = crate::gzguts_h::GZ_WRITE,
            b'a' => parsed.mode = crate::gzguts_h::GZ_APPEND,
            b'+' => return None,
            b'e' => parsed.oflag |= crate::stdlib::O_CLOEXEC,
            b'x' => parsed.exclusive = 1,
            b'f' => parsed.strategy = crate::zlib_h::Z_FILTERED,
            b'h' => parsed.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
            b'R' => parsed.strategy = crate::zlib_h::Z_RLE,
            b'F' => parsed.strategy = crate::zlib_h::Z_FIXED,
            b'G' => parsed.direct = -1,
            b'N' => parsed.oflag |= crate::stdlib::O_NONBLOCK,
            b'T' => parsed.direct = 1,
            _ => {}
        }
    }
    Some(parsed)
}

fn gz_reset(mut reset: GzResetState) -> GzResetState {
    reset.apply_reset();
    reset
}

// Build the opaque handle while it is still owned. The FFI entry points are
// solely responsible for publishing this Box as the C handle. Raw-FD
// adoption remains here, after mode/path validation, for gzdopen().
unsafe fn gz_open(path: GzOpenPath<'_>, mode: &[u8]) -> Option<Box<crate::gzguts_h::gz_state>> {
    // The gzip handle is opaque at the ABI.  Keep its allocation owned until
    // the handle is successfully returned, rather than using malloc/free for
    // the state record itself.
    // Reserve the exact one-state allocation before parsing or opening, as
    // the former Vec owner did, so allocation failure has the same ordering.
    // `Box::write()` publishes the initialized state without an intermediate
    // raw handle; close can now reclaim the matching one-state Box directly.
    let fd_path;
    let (path, fd) = match path {
        GzOpenPath::Path(path) => (path, -1 as ::core::ffi::c_int),
        GzOpenPath::Descriptor(fd) => {
            fd_path = GzFdPath::new(fd);
            (fd_path.as_bytes(), fd)
        }
    };
    let state_owner = match Box::<crate::gzguts_h::gz_state>::try_new_uninit() {
        Ok(owner) => owner,
        Err(_) => return None,
    };
    let Some(mode) = parse_gz_open_mode(mode).and_then(GzOpenMode::normalize) else {
        return None;
    };
    let Some(config) = GzOpenConfig::new(path, mode) else {
        return None;
    };
    let source = if fd == -1 as ::core::ffi::c_int {
        GzOpenSource::Path
    } else {
        // This is the only raw-FD adoption. Keep it after mode/path
        // validation so invalid gzdopen() modes do not consume the caller's
        // descriptor, matching the original failure ordering.
        let fd = <rustix::fd::OwnedFd as rustix::fd::FromRawFd>::from_raw_fd(fd);
        GzOpenSource::Adopted(fd)
    };
    let Some(initial) = gz_open_state(config, path, source) else {
        return None;
    };
    let state_owner = Box::write(
        state_owner,
        crate::gzguts_h::gz_state {
            x: crate::zlib_h::gzFile_s {
                have: initial.reset.have,
                next: ::core::ptr::null_mut(),
                pos: initial.reset.pos,
            },
            mode: initial.reset.mode,
            fd: Some(initial.fd),
            path: Some(initial.path),
            want: initial.want,
            buffers: crate::gzguts_h::GzBuffers {
                size: initial.size,
                input: None,
                output: None,
                input_cursor: None,
                deflate_state: None,
                output_cursor: None,
            },
            direct: initial.direct,
            junk: initial.reset.junk,
            how: initial.reset.how,
            again: initial.reset.again,
            start: initial.start,
            eof: initial.reset.eof,
            past: initial.reset.past,
            level: initial.level,
            strategy: initial.strategy,
            reset: initial.reset.reset,
            skip: initial.reset.skip,
            err: initial.reset.err,
            msg: None,
            strm: crate::zlib_h::z_stream {
                next_in: ::core::ptr::null_mut(),
                avail_in: initial.reset.codec.available_input,
                total_in: initial.reset.codec.total_in,
                next_out: ::core::ptr::null_mut(),
                avail_out: initial.reset.codec.available_output,
                total_out: initial.reset.codec.total_out,
                msg: ::core::ptr::null_mut(),
                state: None,
                zalloc: None,
                zfree: None,
                opaque: ::core::ptr::null_mut(),
                data_type: 0,
                adler: 0,
                reserved: 0,
            },
        },
    );
    Some(state_owner)
}

#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    gz_open(
        GzOpenPath::Path(::core::ffi::CStr::from_ptr(path).to_bytes()),
        ::core::ffi::CStr::from_ptr(mode).to_bytes(),
    )
    .map_or(::core::ptr::null_mut(), |state| {
        Box::into_raw(state).cast::<crate::zlib_h::gzFile_s>()
    })
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    gz_open(
        GzOpenPath::Path(::core::ffi::CStr::from_ptr(path).to_bytes()),
        ::core::ffi::CStr::from_ptr(mode).to_bytes(),
    )
    .map_or(::core::ptr::null_mut(), |state| {
        Box::into_raw(state).cast::<crate::zlib_h::gzFile_s>()
    })
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if fd == -1 as ::core::ffi::c_int || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    gz_open(
        GzOpenPath::Descriptor(fd),
        ::core::ffi::CStr::from_ptr(mode).to_bytes(),
    )
    .map_or(::core::ptr::null_mut(), |state| {
        Box::into_raw(state).cast::<crate::zlib_h::gzFile_s>()
    })
}
fn gzbuffer(
    mode: ::core::ffi::c_int,
    current_size: ::core::ffi::c_uint,
    want: &mut ::core::ffi::c_uint,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    match gzbuffer_want(mode, current_size, size) {
        Some(requested_want) => {
            *want = requested_want;
            0 as ::core::ffi::c_int
        }
        None => -1 as ::core::ffi::c_int,
    }
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzbuffer(state.mode, state.buffers.size, &mut state.want, size)
}
fn gzrewind(state: GzRewindState<'_>) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ
        || state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR
    {
        return -1 as ::core::ffi::c_int;
    }
    if rustix::fs::seek(state.fd, rustix::fs::SeekFrom::Start(state.start as u64)).is_err() {
        return -1 as ::core::ffi::c_int;
    }
    reset_gz_target(state.mode, state.reset);
    // The ABI cursor is empty after a rewind, so retire the owner cursor as
    // well.  A later refill will install a newly checked cursor.
    *state.output_cursor = None;
    return 0 as ::core::ffi::c_int;
}

// This is the complete pointer-free state required by gzip seeking.  The
// ABI-facing adapter validates and projects its buffered cursor separately,
// leaving this transition independent of raw state or cursor pointers.
struct GzSeekState<'a> {
    reset: GzResetState,
    fd: Option<&'a rustix::fd::OwnedFd>,
    start: crate::stdlib::off64_t,
}

// The seek transition borrows only scalar gzip fields, owned error storage,
// and the descriptor.  The ABI wrapper validates and converts its cursor
// before building this pointer-free target.  The core then synchronizes that
// checked cursor with the owned buffer before applying seek policy, so a
// public `gzgetc` macro advance cannot leave the owner stale.
struct GzSeekTarget<'a> {
    mode: ::core::ffi::c_int,
    reset: GzResetTarget<'a>,
    output_cursor: &'a mut Option<GzCodecOutputCursor>,
    validated_output_cursor: Option<GzCodecOutputCursor>,
    fd: Option<&'a rustix::fd::OwnedFd>,
    start: crate::stdlib::off64_t,
}

// Seeking either discards the completed output span (a direct seek or
// rewind), advances within it, or leaves it alone while scheduling a future
// skip.  Keep that decision in the pointer-free seek implementation so the
// ABI wrapper only supplies validated views and publishes its cursor.
enum GzSeekCursorAction {
    Clear,
    Advance(usize),
    Keep,
}

fn gzseek64_state<'a>(
    mut state: GzSeekState<'a>,
    buffered: Option<&[u8]>,
    mut offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
) -> (crate::stdlib::off64_t, GzSeekState<'a>, GzSeekCursorAction) {
    let plan = gzseek_plan(
        state.reset.mode,
        state.reset.err,
        state.reset.pos,
        state.reset.past,
        state.reset.skip,
        state.reset.how,
        state.reset.have,
        offset,
        whence,
    );
    let Some(plan) = plan else {
        return (-1, state, GzSeekCursorAction::Keep);
    };
    if plan.clear_skip {
        state.reset.skip = 0;
    }
    let mut clear_output_cursor = false;
    offset = match plan.action {
        GzSeekAction::Direct { seek_by, position } => {
            let Some(fd) = state.fd else {
                return (-1, state, GzSeekCursorAction::Keep);
            };
            if rustix::fs::seek(fd, rustix::fs::SeekFrom::Current(seek_by as i64)).is_err() {
                return (-1, state, GzSeekCursorAction::Keep);
            }
            state.reset.have = 0;
            state.reset.eof = 0;
            state.reset.past = 0;
            state.reset.skip = 0;
            gz_clear_error(&mut state.reset.msg, &mut state.reset.err);
            state.reset.codec.reset_input();
            state.reset.pos = position;
            return (state.reset.pos, state, GzSeekCursorAction::Clear);
        }
        GzSeekAction::Rewind { offset } => {
            let Some(fd) = state.fd else {
                return (-1, state, GzSeekCursorAction::Keep);
            };
            if rustix::fs::seek(fd, rustix::fs::SeekFrom::Start(state.start as u64)).is_err() {
                return (-1, state, GzSeekCursorAction::Keep);
            }
            state.reset.apply_reset();
            clear_output_cursor = true;
            offset
        }
        GzSeekAction::Skip { offset } => offset,
        GzSeekAction::Reject => return (-1, state, GzSeekCursorAction::Keep),
    };
    let mut consumed = 0;
    if state.reset.mode == crate::gzguts_h::GZ_READ {
        let n = if ::core::mem::size_of::<::core::ffi::c_int>()
            == ::core::mem::size_of::<crate::stdlib::off64_t>()
            && state.reset.have > gz_intmax()
            || state.reset.have as crate::stdlib::off64_t > offset
        {
            offset as ::core::ffi::c_uint
        } else {
            state.reset.have
        };
        if n != 0 {
            // The boundary constructs `buffered` only after checking that the
            // ABI cursor and advertised length lie within the owned output
            // allocation.  Keep the same proof here before consuming it.
            let Some(buffered) = buffered else {
                return (-1, state, GzSeekCursorAction::Keep);
            };
            if buffered.get(..n as usize).is_none() {
                return (-1, state, GzSeekCursorAction::Keep);
            }
        }
        state.reset.have = state.reset.have.wrapping_sub(n);
        state.reset.pos += n as crate::stdlib::off64_t;
        offset -= n as crate::stdlib::off64_t;
        consumed = n as usize;
    }
    state.reset.skip = offset;
    (
        state.reset.pos + offset,
        state,
        if clear_output_cursor {
            GzSeekCursorAction::Clear
        } else if consumed == 0 {
            GzSeekCursorAction::Keep
        } else {
            GzSeekCursorAction::Advance(consumed)
        },
    )
}

#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as ::core::ffi::c_int;
    };
    gzrewind(GzRewindState {
        mode: state.mode,
        err: state.err,
        start: state.start,
        fd: state.fd.as_ref().unwrap(),
        reset: GzResetTarget {
            have: &mut state.x.have,
            eof: &mut state.eof,
            past: &mut state.past,
            how: &mut state.how,
            junk: &mut state.junk,
            reset: &mut state.reset,
            again: &mut state.again,
            skip: &mut state.skip,
            err: &mut state.err,
            msg: &mut state.msg,
            pos: &mut state.x.pos,
            codec_available_input: &mut state.strm.avail_in,
            codec_available_output: &mut state.strm.avail_out,
            codec_total_in: &mut state.strm.total_in,
            codec_total_out: &mut state.strm.total_out,
            input_cursor: &mut state.buffers.input_cursor,
        },
        output_cursor: &mut state.buffers.output_cursor,
    })
}
fn gzseek64(
    target: GzSeekTarget<'_>,
    buffered: Option<&[u8]>,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> (crate::stdlib::off64_t, usize) {
    // The ABI cursor is exposed through the public `gzgetc` macro, so it can
    // advance without an intervening Rust call.  The wrapper proved this
    // cursor is within the owned output allocation; make it authoritative
    // before this safe state transition consults or advances the owner.
    *target.output_cursor = target.validated_output_cursor;
    let reset = GzResetState {
        mode: target.mode,
        have: *target.reset.have,
        eof: *target.reset.eof,
        past: *target.reset.past,
        how: *target.reset.how,
        junk: *target.reset.junk,
        reset: *target.reset.reset,
        again: *target.reset.again,
        skip: *target.reset.skip,
        err: *target.reset.err,
        msg: target.reset.msg.take(),
        pos: *target.reset.pos,
        codec: GzCodecCounters::from_stream_fields(
            *target.reset.codec_available_input,
            *target.reset.codec_available_output,
            *target.reset.codec_total_in,
            *target.reset.codec_total_out,
        ),
    };
    let (result, reset, cursor_action) = gzseek64_state(
        GzSeekState {
            reset,
            fd: target.fd,
            start: target.start,
        },
        buffered,
        offset,
        whence,
    );
    store_gz_reset_target(target.reset, reset.reset);
    let consumed = match cursor_action {
        GzSeekCursorAction::Clear => {
            *target.output_cursor = None;
            0
        }
        GzSeekCursorAction::Advance(consumed) => {
            let Some(next) = target
                .output_cursor
                .as_ref()
                .and_then(|cursor| cursor.advance(consumed))
            else {
                return (-1, 0);
            };
            *target.output_cursor = Some(next);
            consumed
        }
        GzSeekCursorAction::Keep => 0,
    };
    (result, consumed)
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as crate::stdlib::off64_t;
    };
    // `x.next` is an ABI cursor, not owned storage.  When data is buffered,
    // prove both the cursor's provenance and its advertised remaining length
    // against the owned output allocation before dispatching to the safe seek
    // transition.
    let (buffered, validated_output_cursor) = if state.x.have == 0 {
        (None, None)
    } else {
        let Some(buffer) = state.buffers.output.as_deref() else {
            return -1 as crate::stdlib::off64_t;
        };
        let Some(buffered) =
            GzBufferedCursor::from_owned_buffer(buffer, state.x.next.addr(), state.x.have)
        else {
            return -1 as crate::stdlib::off64_t;
        };
        let Some(cursor) =
            GzCodecOutputCursor::from_owned_buffer(buffer, buffered.start, state.x.have)
        else {
            return -1 as crate::stdlib::off64_t;
        };
        (Some(buffered.unread()), Some(cursor))
    };
    let (result, consumed) = gzseek64(
        GzSeekTarget {
            mode: state.mode,
            reset: GzResetTarget {
                have: &mut state.x.have,
                eof: &mut state.eof,
                past: &mut state.past,
                how: &mut state.how,
                junk: &mut state.junk,
                reset: &mut state.reset,
                again: &mut state.again,
                skip: &mut state.skip,
                err: &mut state.err,
                msg: &mut state.msg,
                pos: &mut state.x.pos,
                codec_available_input: &mut state.strm.avail_in,
                codec_available_output: &mut state.strm.avail_out,
                codec_total_in: &mut state.strm.total_in,
                codec_total_out: &mut state.strm.total_out,
                input_cursor: &mut state.buffers.input_cursor,
            },
            output_cursor: &mut state.buffers.output_cursor,
            validated_output_cursor,
            fd: state.fd.as_ref(),
            start: state.start,
        },
        buffered,
        offset,
        whence,
    );
    if consumed != 0 {
        state.x.next = state.x.next.wrapping_add(consumed);
    }
    result
}
fn gzseek_result(ret: crate::stdlib::off64_t) -> crate::stdlib::off_t {
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let Some(state) = (file as crate::gzguts_h::gz_statep).as_mut() else {
        return -1 as crate::stdlib::off_t;
    };
    let (buffered, validated_output_cursor) = if state.x.have == 0 {
        (None, None)
    } else {
        let Some(buffer) = state.buffers.output.as_deref() else {
            return -1 as crate::stdlib::off_t;
        };
        let Some(buffered) =
            GzBufferedCursor::from_owned_buffer(buffer, state.x.next.addr(), state.x.have)
        else {
            return -1 as crate::stdlib::off_t;
        };
        let Some(cursor) =
            GzCodecOutputCursor::from_owned_buffer(buffer, buffered.start, state.x.have)
        else {
            return -1 as crate::stdlib::off_t;
        };
        (Some(buffered.unread()), Some(cursor))
    };
    let (result, consumed) = gzseek64(
        GzSeekTarget {
            mode: state.mode,
            reset: GzResetTarget {
                have: &mut state.x.have,
                eof: &mut state.eof,
                past: &mut state.past,
                how: &mut state.how,
                junk: &mut state.junk,
                reset: &mut state.reset,
                again: &mut state.again,
                skip: &mut state.skip,
                err: &mut state.err,
                msg: &mut state.msg,
                pos: &mut state.x.pos,
                codec_available_input: &mut state.strm.avail_in,
                codec_available_output: &mut state.strm.avail_out,
                codec_total_in: &mut state.strm.total_in,
                codec_total_out: &mut state.strm.total_out,
                input_cursor: &mut state.buffers.input_cursor,
            },
            output_cursor: &mut state.buffers.output_cursor,
            validated_output_cursor,
            fd: state.fd.as_ref(),
            start: state.start,
        },
        buffered,
        offset,
        whence,
    );
    if consumed != 0 {
        state.x.next = state.x.next.wrapping_add(consumed);
    }
    gzseek_result(result)
}
fn gztell64(position: &GzPosition) -> crate::stdlib::off64_t {
    position.tell()
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as crate::stdlib::off64_t;
    };
    let state = state.as_ref();
    gztell64(&GzPosition {
        mode: state.mode,
        pos: state.x.pos,
        past: state.past,
        skip: state.skip,
    })
}
fn gztell(position: &GzPosition) -> crate::stdlib::off_t {
    let ret = gztell64(position);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as crate::stdlib::off_t;
    };
    let state = state.as_ref();
    gztell(&GzPosition {
        mode: state.mode,
        pos: state.x.pos,
        past: state.past,
        skip: state.skip,
    })
}
fn gzoffset64(query: GzOffsetQuery<'_>) -> crate::stdlib::off64_t {
    if !query.position.active() {
        return -1 as crate::stdlib::off64_t;
    }
    let Some(fd) = query.fd else {
        return -1 as crate::stdlib::off64_t;
    };
    let Ok(offset) = rustix::fs::tell(fd) else {
        return -1 as crate::stdlib::off64_t;
    };
    let offset = offset as crate::stdlib::off64_t;
    query.position.offset(offset, query.buffered_input)
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as crate::stdlib::off64_t;
    };
    let state = state.as_ref();
    gzoffset64(GzOffsetQuery {
        position: GzPosition {
            mode: state.mode,
            pos: 0 as crate::stdlib::off64_t,
            past: 0 as ::core::ffi::c_int,
            skip: 0 as crate::stdlib::off64_t,
        },
        fd: state.fd.as_ref(),
        buffered_input: state.strm.avail_in,
    })
}
fn gzoffset(query: GzOffsetQuery<'_>) -> crate::stdlib::off_t {
    let ret = gzoffset64(query);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return -1 as crate::stdlib::off_t;
    };
    let state = state.as_ref();
    gzoffset(GzOffsetQuery {
        position: GzPosition {
            mode: state.mode,
            pos: 0 as crate::stdlib::off64_t,
            past: 0 as ::core::ffi::c_int,
            skip: 0 as crate::stdlib::off64_t,
        },
        fd: state.fd.as_ref(),
        buffered_input: state.strm.avail_in,
    })
}
#[export_name = "gzeof"]

pub unsafe extern "C" fn gzeof_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return 0 as ::core::ffi::c_int;
    };
    let state = state.as_ref();
    gzeof(state.mode, state.past)
}
enum GzErrorMessage {
    OutOfMemory,
    Empty,
    State,
}

fn gzerror(
    mode: ::core::ffi::c_int,
    err: ::core::ffi::c_int,
    has_message: bool,
    errnum: Option<&mut ::core::ffi::c_int>,
) -> Option<GzErrorMessage> {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return None;
    }
    if let Some(errnum) = errnum {
        *errnum = err;
    }
    if err == crate::zlib_h::Z_MEM_ERROR {
        Some(GzErrorMessage::OutOfMemory)
    } else if has_message {
        Some(GzErrorMessage::State)
    } else {
        Some(GzErrorMessage::Empty)
    }
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let Some(state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return ::core::ptr::null::<::core::ffi::c_char>();
    };
    let state = state.as_ref();
    let errnum = ::core::ptr::NonNull::new(errnum).map(|mut errnum| errnum.as_mut());
    match gzerror(state.mode, state.err, state.msg.is_some(), errnum) {
        None => ::core::ptr::null::<::core::ffi::c_char>(),
        Some(GzErrorMessage::OutOfMemory) => b"out of memory\0".as_ptr().cast(),
        Some(GzErrorMessage::Empty) => b"\0".as_ptr().cast(),
        Some(GzErrorMessage::State) => state
            .msg
            .as_deref()
            .map_or(::core::ptr::null(), |msg| msg.as_ptr().cast()),
    }
}
fn gzclearerr(
    mode: ::core::ffi::c_int,
    eof: &mut ::core::ffi::c_int,
    past: &mut ::core::ffi::c_int,
    message: &mut Option<Box<[u8]>>,
    error: &mut ::core::ffi::c_int,
) {
    if mode != crate::gzguts_h::GZ_READ && mode != crate::gzguts_h::GZ_WRITE {
        return;
    }
    if mode == crate::gzguts_h::GZ_READ {
        *eof = 0 as ::core::ffi::c_int;
        *past = 0 as ::core::ffi::c_int;
    }
    gz_clear_error(message, error);
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    let Some(mut state) = ::core::ptr::NonNull::new(file as crate::gzguts_h::gz_statep) else {
        return;
    };
    let state = state.as_mut();
    gzclearerr(
        state.mode,
        &mut state.eof,
        &mut state.past,
        &mut state.msg,
        &mut state.err,
    )
}
#[export_name = "gz_error"]

pub unsafe extern "C" fn gz_error_ffi(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    // This callback is the ABI boundary for both pointers.  Convert them
    // before dispatching the error policy so no implementation function
    // carries an ABI-shaped state or message pointer.
    let Some(state) = state.as_mut() else {
        return;
    };
    let message = (!msg.is_null()).then(|| ::core::ffi::CStr::from_ptr(msg).to_bytes());
    gz_set_error(
        &mut state.msg,
        &mut state.err,
        &mut state.x.have,
        state.again,
        state.path.as_deref(),
        err,
        message,
    );
}
pub fn gz_intmax() -> ::core::ffi::c_uint {
    return crate::limits_h::INT_MAX as ::core::ffi::c_uint;
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
