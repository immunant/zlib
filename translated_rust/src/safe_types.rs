//! Fixed-width values used by safe implementation code.
//!
//! These aliases deliberately do not mirror the C ABI types.  Boundary
//! wrappers convert to and from the ABI only after validating that a value is
//! representable in the type required by the safe core.

pub(crate) type ByteCount = usize;
pub(crate) type BitCount = u32;
pub(crate) type Checksum = u32;
pub(crate) type StreamOffset = i64;

/// Classifies a foreign input buffer before an FFI wrapper creates a slice.
///
/// A null input pointer has a distinct meaning for zlib's checksum APIs,
/// including when its advertised length is non-zero.  A non-null empty input,
/// on the other hand, is a valid empty slice and must not require a raw slice
/// construction.  Keeping this distinction in safe code makes the boundary
/// rule shared by checksum wrappers explicit and testable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FfiInputKind {
    Null,
    Empty,
    NonEmpty,
}

impl FfiInputKind {
    pub(crate) fn from_nullable_parts(pointer_is_null: bool, len: ByteCount) -> Self {
        if pointer_is_null {
            Self::Null
        } else if len == 0 {
            Self::Empty
        } else {
            Self::NonEmpty
        }
    }
}

/// A checked, forward-only view of input supplied to a safe implementation.
///
/// The cursor owns no storage and never exposes raw pointers.  Callers can
/// retain it across state-machine steps without retaining a borrow of data
/// that was not established at an FFI boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct InputCursor<'a> {
    bytes: &'a [u8],
    position: ByteCount,
}

impl<'a> InputCursor<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub(crate) fn position(&self) -> ByteCount {
        self.position
    }

    pub(crate) fn remaining(&self) -> ByteCount {
        self.bytes.len() - self.position
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    pub(crate) fn as_remaining(&self) -> &'a [u8] {
        &self.bytes[self.position..]
    }

    pub(crate) fn advance(&mut self, count: ByteCount) -> bool {
        let Some(next) = self.position.checked_add(count) else {
            return false;
        };
        if next > self.bytes.len() {
            return false;
        }

        self.position = next;
        true
    }

    pub(crate) fn read_byte(&mut self) -> Option<u8> {
        let byte = *self.as_remaining().first()?;
        debug_assert!(self.advance(1));
        Some(byte)
    }

    pub(crate) fn take(&mut self, count: ByteCount) -> Option<&'a [u8]> {
        let start = self.position;
        if !self.advance(count) {
            return None;
        }
        Some(&self.bytes[start..self.position])
    }
}

/// A bounded, least-significant-bit-first reader over an input cursor.
///
/// Deflate streams consume bits from the low end of each byte.  The reader
/// keeps that ordering explicit and never shifts by a value that could exceed
/// the width of its accumulator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BitReader<'a> {
    input: InputCursor<'a>,
    hold: u64,
    bits: BitCount,
}

impl<'a> BitReader<'a> {
    /// Deflate never needs more than 32 bits in one decode request.
    pub(crate) const MAX_REQUEST_BITS: BitCount = 32;

    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self {
            input: InputCursor::new(bytes),
            hold: 0,
            bits: 0,
        }
    }

    pub(crate) fn input_position(&self) -> ByteCount {
        self.input.position()
    }

    pub(crate) fn remaining_input(&self) -> ByteCount {
        self.input.remaining()
    }

    pub(crate) fn buffered_bits(&self) -> BitCount {
        self.bits
    }

    fn ensure_bits(&mut self, count: BitCount) -> bool {
        if count > Self::MAX_REQUEST_BITS {
            return false;
        }

        while self.bits < count {
            let Some(byte) = self.input.read_byte() else {
                return false;
            };
            self.hold |= u64::from(byte) << self.bits;
            self.bits += 8;
        }
        true
    }

    pub(crate) fn peek_bits(&mut self, count: BitCount) -> Option<u32> {
        if !self.ensure_bits(count) {
            return None;
        }

        let mask = if count == 0 { 0 } else { (1u64 << count) - 1 };
        u32::try_from(self.hold & mask).ok()
    }

    pub(crate) fn drop_bits(&mut self, count: BitCount) -> bool {
        if count > self.bits {
            return false;
        }

        self.hold >>= count;
        self.bits -= count;
        true
    }

    pub(crate) fn read_bits(&mut self, count: BitCount) -> Option<u32> {
        let bits = self.peek_bits(count)?;
        debug_assert!(self.drop_bits(count));
        Some(bits)
    }
}

/// A checked, forward-only output cursor.
///
/// `copy_from_history()` deliberately copies one byte at a time.  When the
/// requested length exceeds the distance, each newly written byte becomes
/// source for the next byte, matching deflate's overlapping match semantics.
#[derive(Debug)]
pub(crate) struct OutputCursor<'a> {
    bytes: &'a mut [u8],
    position: ByteCount,
}

impl<'a> OutputCursor<'a> {
    pub(crate) fn new(bytes: &'a mut [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub(crate) fn position(&self) -> ByteCount {
        self.position
    }

    pub(crate) fn remaining(&self) -> ByteCount {
        self.bytes.len() - self.position
    }

    pub(crate) fn is_full(&self) -> bool {
        self.remaining() == 0
    }

    pub(crate) fn written(&self) -> &[u8] {
        &self.bytes[..self.position]
    }

    pub(crate) fn write_byte(&mut self, byte: u8) -> bool {
        if self.is_full() {
            return false;
        }

        self.bytes[self.position] = byte;
        self.position += 1;
        true
    }

    pub(crate) fn write_from(&mut self, input: &[u8]) -> bool {
        if input.len() > self.remaining() {
            return false;
        }

        let end = self.position + input.len();
        self.bytes[self.position..end].copy_from_slice(input);
        self.position = end;
        true
    }

    pub(crate) fn copy_from_history(&mut self, distance: ByteCount, count: ByteCount) -> bool {
        if distance == 0 || distance > self.position || count > self.remaining() {
            return false;
        }

        let start = self.position;
        for offset in 0..count {
            let byte = self.bytes[start - distance + offset];
            self.bytes[start + offset] = byte;
        }
        self.position += count;
        true
    }
}

/// A bounded, least-significant-bit-first writer over an output cursor.
///
/// `write_bits()` checks all output capacity before it mutates either cursor
/// or its bit buffer.  That lets state machines stop at an output boundary
/// without manufacturing a partially written symbol.
#[derive(Debug)]
pub(crate) struct BitWriter<'a> {
    output: OutputCursor<'a>,
    hold: u64,
    bits: BitCount,
}

impl<'a> BitWriter<'a> {
    pub(crate) const MAX_REQUEST_BITS: BitCount = 32;

    pub(crate) fn new(bytes: &'a mut [u8]) -> Self {
        Self {
            output: OutputCursor::new(bytes),
            hold: 0,
            bits: 0,
        }
    }

    pub(crate) fn buffered_bits(&self) -> BitCount {
        self.bits
    }

    pub(crate) fn written(&self) -> &[u8] {
        self.output.written()
    }

    pub(crate) fn write_bits(&mut self, value: u32, count: BitCount) -> bool {
        if count > Self::MAX_REQUEST_BITS {
            return false;
        }

        let bytes_to_flush = (self.bits + count) / 8;
        let Ok(bytes_to_flush) = usize::try_from(bytes_to_flush) else {
            return false;
        };
        if bytes_to_flush > self.output.remaining() {
            return false;
        }

        let mask = if count == 0 { 0 } else { (1u64 << count) - 1 };
        self.hold |= (u64::from(value) & mask) << self.bits;
        self.bits += count;
        while self.bits >= 8 {
            debug_assert!(self.output.write_byte(self.hold as u8));
            self.hold >>= 8;
            self.bits -= 8;
        }
        true
    }

    /// Flush a final partial byte, padding its unused high bits with zero.
    pub(crate) fn flush_partial_byte(&mut self) -> bool {
        if self.bits == 0 {
            return true;
        }
        if self.output.is_full() {
            return false;
        }

        debug_assert!(self.output.write_byte(self.hold as u8));
        self.hold = 0;
        self.bits = 0;
        true
    }
}

pub(crate) fn byte_count_from_uint(value: crate::stdlib::uInt) -> Option<ByteCount> {
    ByteCount::try_from(value).ok()
}

pub(crate) fn uint_from_byte_count(value: ByteCount) -> Option<crate::stdlib::uInt> {
    crate::stdlib::uInt::try_from(value).ok()
}

pub(crate) fn checksum_from_ulong(value: crate::stdlib::uLong) -> Option<Checksum> {
    Checksum::try_from(value).ok()
}

pub(crate) fn ulong_from_checksum(value: Checksum) -> Option<crate::stdlib::uLong> {
    crate::stdlib::uLong::try_from(value).ok()
}

pub(crate) fn stream_offset_from_off64(value: crate::stdlib::off64_t) -> Option<StreamOffset> {
    StreamOffset::try_from(value).ok()
}

pub(crate) fn off64_from_stream_offset(value: StreamOffset) -> Option<crate::stdlib::off64_t> {
    crate::stdlib::off64_t::try_from(value).ok()
}

#[cfg(test)]
mod tests {
    use super::{
        byte_count_from_uint, checksum_from_ulong, off64_from_stream_offset,
        stream_offset_from_off64, uint_from_byte_count, ulong_from_checksum, BitCount, BitReader,
        BitWriter, FfiInputKind, InputCursor, OutputCursor,
    };

    #[test]
    fn uint_and_byte_count_conversions_are_checked() {
        assert_eq!(byte_count_from_uint(42), Some(42));
        assert_eq!(uint_from_byte_count(42), Some(42));
        assert_eq!(
            uint_from_byte_count((crate::stdlib::uInt::MAX as usize).saturating_add(1)),
            None
        );
    }

    #[test]
    fn checksum_conversion_rejects_values_outside_the_safe_core_width() {
        assert_eq!(checksum_from_ulong(0xffff_ffff), Some(0xffff_ffff));
        assert_eq!(ulong_from_checksum(0xffff_ffff), Some(0xffff_ffff));

        if core::mem::size_of::<crate::stdlib::uLong>() > core::mem::size_of::<u32>() {
            assert_eq!(checksum_from_ulong(0x1_0000_0000), None);
        }
    }

    #[test]
    fn stream_offset_conversions_preserve_signed_values() {
        assert_eq!(stream_offset_from_off64(-1), Some(-1));
        assert_eq!(off64_from_stream_offset(-1), Some(-1));
    }

    #[test]
    fn bit_count_is_explicitly_fixed_width() {
        assert_eq!(core::mem::size_of::<BitCount>(), 4);
    }

    #[test]
    fn ffi_input_kind_distinguishes_null_empty_and_nonempty_inputs() {
        assert_eq!(
            FfiInputKind::from_nullable_parts(true, 0),
            FfiInputKind::Null
        );
        assert_eq!(
            FfiInputKind::from_nullable_parts(true, 1),
            FfiInputKind::Null
        );
        assert_eq!(
            FfiInputKind::from_nullable_parts(false, 0),
            FfiInputKind::Empty
        );
        assert_eq!(
            FfiInputKind::from_nullable_parts(false, 1),
            FfiInputKind::NonEmpty
        );
    }

    #[test]
    fn input_cursor_rejects_advances_past_the_slice_without_moving() {
        let mut cursor = InputCursor::new(b"abc");

        assert_eq!(cursor.read_byte(), Some(b'a'));
        assert_eq!(cursor.take(2), Some(&b"bc"[..]));
        assert!(cursor.is_empty());
        assert!(!cursor.advance(1));
        assert_eq!(cursor.position(), 3);
        assert_eq!(cursor.take(1), None);
        assert_eq!(cursor.position(), 3);
    }

    #[test]
    fn bit_reader_consumes_deflate_bits_from_the_low_end_first() {
        let mut reader = BitReader::new(&[0b1010_0110, 0b0000_0011]);

        assert_eq!(reader.read_bits(3), Some(0b110));
        assert_eq!(reader.peek_bits(5), Some(0b10100));
        assert_eq!(reader.buffered_bits(), 5);
        assert!(reader.drop_bits(5));
        assert_eq!(reader.read_bits(2), Some(0b11));
        assert_eq!(reader.input_position(), 2);
        assert_eq!(reader.remaining_input(), 0);
    }

    #[test]
    fn bit_reader_keeps_available_bits_when_a_larger_request_runs_out_of_input() {
        let mut reader = BitReader::new(&[0x5a]);

        assert_eq!(reader.peek_bits(12), None);
        assert_eq!(reader.buffered_bits(), 8);
        assert_eq!(reader.input_position(), 1);
        assert_eq!(reader.read_bits(8), Some(0x5a));
        assert_eq!(reader.peek_bits(BitReader::MAX_REQUEST_BITS + 1), None);
    }

    #[test]
    fn output_cursor_tracks_writes_and_leaves_state_unchanged_on_overflow() {
        let mut storage = [0_u8; 4];
        let mut cursor = OutputCursor::new(&mut storage);

        assert!(cursor.write_byte(b'a'));
        assert!(cursor.write_from(b"bc"));
        assert_eq!(cursor.written(), b"abc");
        assert!(!cursor.write_from(b"de"));
        assert_eq!(cursor.position(), 3);
        assert_eq!(cursor.written(), b"abc");
    }

    #[test]
    fn output_cursor_preserves_overlapping_lz_copy_semantics() {
        let mut storage = [0_u8; 10];
        let mut cursor = OutputCursor::new(&mut storage);

        assert!(cursor.write_from(b"ab"));
        assert!(cursor.copy_from_history(2, 6));
        assert_eq!(cursor.written(), b"abababab");

        assert!(!cursor.copy_from_history(0, 1));
        assert!(!cursor.copy_from_history(9, 1));
        assert!(!cursor.copy_from_history(2, 3));
        assert_eq!(cursor.written(), b"abababab");
    }

    #[test]
    fn bit_writer_emits_deflate_bits_from_the_low_end_first() {
        let mut storage = [0_u8; 2];
        let mut writer = BitWriter::new(&mut storage);

        assert!(writer.write_bits(0b110, 3));
        assert!(writer.write_bits(0b10100, 5));
        assert!(writer.write_bits(0b11, 2));
        assert_eq!(writer.written(), &[0b1010_0110]);
        assert_eq!(writer.buffered_bits(), 2);
        assert!(writer.flush_partial_byte());
        assert_eq!(writer.written(), &[0b1010_0110, 0b0000_0011]);
    }

    #[test]
    fn bit_writer_rejects_a_write_that_cannot_flush_without_changing_state() {
        let mut storage = [0_u8; 1];
        let mut writer = BitWriter::new(&mut storage);

        assert!(writer.write_bits(0b111, 3));
        assert!(writer.write_bits(0xff, 8));
        assert_eq!(writer.written(), &[0xff]);
        assert_eq!(writer.buffered_bits(), 3);
        assert!(!writer.write_bits(0b1_1111, 5));
        assert_eq!(writer.buffered_bits(), 3);
        assert!(!writer.flush_partial_byte());
    }
}
