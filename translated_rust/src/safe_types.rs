//! Fixed-width values used by safe implementation code.
//!
//! These aliases deliberately do not mirror the C ABI types.  Boundary
//! wrappers convert to and from the ABI only after validating that a value is
//! representable in the type required by the safe core.

pub(crate) type ByteCount = usize;
pub(crate) type BitCount = u32;
pub(crate) type Checksum = u32;
pub(crate) type StreamOffset = i64;

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
        stream_offset_from_off64, uint_from_byte_count, ulong_from_checksum, BitCount,
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
}
