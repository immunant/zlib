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

// Codec calls see short-lived slices of gzip's two owned buffers.  Keep the
// cursor validation and output-capacity check in pointer-free views so later
// codec projections can carry slices instead of ABI `z_stream` cursors.
pub(crate) struct GzCodecInputView<'a> {
    bytes: &'a [u8],
}

pub(crate) struct GzCodecOutputView<'a> {
    bytes: &'a mut [u8],
}

impl<'a> GzBufferedCursor<'a> {
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

impl<'a> GzCodecInputView<'a> {
    // A codec input cursor is valid only when its entire advertised range is
    // within gzip's owned input allocation.  Preserve the zero-length case:
    // it may carry a null ABI cursor and therefore has no address to check.
    pub(crate) fn from_owned_buffer(
        buffer: &'a [u8],
        cursor_address: usize,
        available: u32,
    ) -> Option<Self> {
        if available == 0 {
            return Some(Self { bytes: &[] });
        }
        let start = cursor_address.checked_sub(buffer.as_ptr().addr())?;
        let end = start.checked_add(available as usize)?;
        Some(Self {
            bytes: buffer.get(start..end)?,
        })
    }

    pub(crate) fn bytes(&self) -> &'a [u8] {
        self.bytes
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

// The decompression loop mutates only these scalar gzip fields in response to
// an inflate result.  Keep that transition with the bounded output accounting
// so an eventual owned gzip codec can run the loop without borrowing the ABI
// `gz_state`; the current boundary only snapshots and republishes the values.
pub(crate) struct GzDecompState {
    output: GzCodecOutput,
    input_available: crate::stdlib::uInt,
    junk: ::core::ffi::c_int,
    eof: ::core::ffi::c_int,
    how: ::core::ffi::c_int,
}

impl GzDecompState {
    pub(crate) fn new(
        output_capacity: usize,
        input_available: crate::stdlib::uInt,
        junk: ::core::ffi::c_int,
        eof: ::core::ffi::c_int,
        how: ::core::ffi::c_int,
    ) -> Option<Self> {
        Some(Self {
            output: GzCodecOutput::new(output_capacity)?,
            input_available,
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
        self.input_available == 0
    }

    pub(crate) fn record_input_available(&mut self, available: crate::stdlib::uInt) {
        self.input_available = available;
    }

    pub(crate) fn output_available(&self) -> crate::stdlib::uInt {
        self.output.available()
    }

    pub(crate) fn record_inflate(
        &mut self,
        result: ::core::ffi::c_int,
        available: crate::stdlib::uInt,
    ) -> GzDecompAction {
        self.output.record_available(available);
        let produced_output = self.output.has_output();
        if produced_output {
            self.junk = 0;
        }
        let step = gz_decomp_step(result, available, produced_output, self.junk);
        if matches!(step.action, GzDecompAction::Junk) {
            self.eof = 1;
            self.how = crate::gzguts_h::LOOK;
        }
        step.action
    }

    pub(crate) fn finish(&mut self, result: ::core::ffi::c_int) -> ::core::ffi::c_int {
        if result == crate::zlib_h::Z_STREAM_END {
            self.junk = 0;
            self.how = crate::gzguts_h::LOOK;
            0
        } else if result != crate::zlib_h::Z_OK {
            -1
        } else {
            0
        }
    }

    pub(crate) fn written(&self) -> crate::stdlib::uInt {
        self.output.written()
    }

    pub(crate) fn fields(&self) -> (::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int) {
        (self.junk, self.eof, self.how)
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

unsafe fn gz_open(path: &[u8], fd: ::core::ffi::c_int, mode: &[u8]) -> crate::zlib_h::gzFile {
    // The gzip handle is opaque at the ABI.  Keep its allocation owned until
    // the handle is successfully returned, rather than using malloc/free for
    // the state record itself.
    let mut state_owner = Vec::new();
    if state_owner.try_reserve_exact(1).is_err() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let Some(mode) = parse_gz_open_mode(mode).and_then(GzOpenMode::normalize) else {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    };
    let Some(config) = GzOpenConfig::new(path, mode) else {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    };
    let initial = if fd == -1 as ::core::ffi::c_int {
        let Some(initial) = config.open(path) else {
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        };
        initial
    } else {
        let oflag = config.open_flags();
        let fd = <rustix::fd::OwnedFd as rustix::fd::FromRawFd>::from_raw_fd(fd);
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
        config.into_open_state(fd)
    };
    state_owner.push(crate::gzguts_h::gz_state {
        x: crate::zlib_h::gzFile_s {
            have: initial.reset.have,
            next: ::core::ptr::null_mut(),
            pos: initial.reset.pos,
        },
        mode: initial.reset.mode,
        fd: Some(initial.fd),
        path: Some(initial.path),
        size: initial.size,
        want: initial.want,
        in_0: None,
        out: None,
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
            state: ::core::ptr::null_mut(),
            zalloc: None,
            zfree: None,
            opaque: ::core::ptr::null_mut(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        },
    });
    let state = state_owner.as_mut_ptr();
    ::core::mem::forget(state_owner);
    return state as crate::zlib_h::gzFile;
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
        ::core::ffi::CStr::from_ptr(path).to_bytes(),
        -1 as ::core::ffi::c_int,
        ::core::ffi::CStr::from_ptr(mode).to_bytes(),
    )
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
        ::core::ffi::CStr::from_ptr(path).to_bytes(),
        -1 as ::core::ffi::c_int,
        ::core::ffi::CStr::from_ptr(mode).to_bytes(),
    )
}
unsafe fn gzdopen(fd: ::core::ffi::c_int, mode: &[u8]) -> crate::zlib_h::gzFile {
    if fd == -1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let path = GzFdPath::new(fd);
    gz_open(path.as_bytes(), fd, mode)
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    if fd == -1 as ::core::ffi::c_int || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    gzdopen(fd, ::core::ffi::CStr::from_ptr(mode).to_bytes())
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
    gzbuffer(state.mode, state.size, &mut state.want, size)
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

fn gzseek64_state<'a>(
    mut state: GzSeekState<'a>,
    buffered: Option<&[u8]>,
    mut offset: crate::stdlib::off64_t,
    whence: ::core::ffi::c_int,
) -> (crate::stdlib::off64_t, GzSeekState<'a>, usize) {
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
        return (-1, state, 0);
    };
    if plan.clear_skip {
        state.reset.skip = 0;
    }
    offset = match plan.action {
        GzSeekAction::Direct { seek_by, position } => {
            let Some(fd) = state.fd else {
                return (-1, state, 0);
            };
            if rustix::fs::seek(fd, rustix::fs::SeekFrom::Current(seek_by as i64)).is_err() {
                return (-1, state, 0);
            }
            state.reset.have = 0;
            state.reset.eof = 0;
            state.reset.past = 0;
            state.reset.skip = 0;
            gz_clear_error(&mut state.reset.msg, &mut state.reset.err);
            state.reset.codec.reset_input();
            state.reset.pos = position;
            return (state.reset.pos, state, 0);
        }
        GzSeekAction::Rewind { offset } => {
            let Some(fd) = state.fd else {
                return (-1, state, 0);
            };
            if rustix::fs::seek(fd, rustix::fs::SeekFrom::Start(state.start as u64)).is_err() {
                return (-1, state, 0);
            }
            state.reset.apply_reset();
            offset
        }
        GzSeekAction::Skip { offset } => offset,
        GzSeekAction::Reject => return (-1, state, 0),
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
                return (-1, state, 0);
            };
            if buffered.get(..n as usize).is_none() {
                return (-1, state, 0);
            }
        }
        state.reset.have = state.reset.have.wrapping_sub(n);
        state.reset.pos += n as crate::stdlib::off64_t;
        offset -= n as crate::stdlib::off64_t;
        consumed = n as usize;
    }
    state.reset.skip = offset;
    (state.reset.pos + offset, state, consumed)
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
        },
    })
}
pub unsafe extern "C" fn gzseek64(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    // `x.next` is an ABI cursor, not owned storage.  When data is buffered,
    // prove both the cursor's provenance and its advertised remaining length
    // against the owned output allocation before handing a slice to the safe
    // seek transition.
    let buffered = if state.x.have == 0 {
        None
    } else {
        let Some(buffer) = state.out.as_deref() else {
            return -1 as crate::stdlib::off64_t;
        };
        let Some(buffered) =
            GzBufferedCursor::from_owned_buffer(buffer, state.x.next.addr(), state.x.have)
        else {
            return -1 as crate::stdlib::off64_t;
        };
        Some(buffered.unread())
    };
    let (result, reset, consumed) = gzseek64_state(
        GzSeekState {
            reset: GzResetState {
                mode: state.mode,
                have: state.x.have,
                eof: state.eof,
                past: state.past,
                how: state.how,
                junk: state.junk,
                reset: state.reset,
                again: state.again,
                skip: state.skip,
                err: state.err,
                msg: state.msg.take(),
                pos: state.x.pos,
                codec: GzCodecCounters {
                    available_input: state.strm.avail_in,
                    available_output: state.strm.avail_out,
                    total_in: state.strm.total_in,
                    total_out: state.strm.total_out,
                },
            },
            fd: state.fd.as_ref(),
            start: state.start,
        },
        buffered,
        offset,
        whence,
    );
    store_gz_reset_target(
        GzResetTarget {
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
        },
        reset.reset,
    );
    if consumed != 0 {
        state.x.next = state.x.next.wrapping_add(consumed);
    }
    result
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    gzseek64(file, offset, whence)
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
    gzseek_result(gzseek64(file, offset, whence))
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
pub unsafe extern "C" fn gz_error(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    let state = &mut *state;
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
#[export_name = "gz_error"]

pub unsafe extern "C" fn gz_error_ffi(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    gz_error(state, err, msg)
}
pub fn gz_intmax() -> ::core::ffi::c_uint {
    return crate::limits_h::INT_MAX as ::core::ffi::c_uint;
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
