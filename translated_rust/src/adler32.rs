pub use crate::__stddef_size_t_h::size_t;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::Z_NULL;

pub const BASE: ::core::ffi::c_uint = 65_521;
pub const NMAX: ::core::ffi::c_int = 5_552;

const BASE_U64: u64 = BASE as u64;
const NMAX_USIZE: usize = NMAX as usize;

fn reduce(adler: u64, sum2: u64) -> (u64, u64) {
    (adler % BASE_U64, sum2 % BASE_U64)
}

fn accumulate_block(mut sum1: u64, mut sum2: u64, block: &[Bytef]) -> (u64, u64) {
    for &byte in block {
        sum1 += byte as u64;
        sum2 += sum1;
    }

    reduce(sum1, sum2)
}

fn reduce_combine_sums(mut sum1: u64, mut sum2: u64) -> (u64, u64) {
    for _ in 0..2 {
        if sum1 >= BASE_U64 {
            sum1 -= BASE_U64;
        }
    }
    if sum2 >= BASE_U64 << 1 {
        sum2 -= BASE_U64 << 1;
    }
    if sum2 >= BASE_U64 {
        sum2 -= BASE_U64;
    }

    (sum1, sum2)
}

pub fn adler32_z(adler: uLong, buf: &[Bytef]) -> uLong {
    let adler = adler as u64;
    let mut sum2 = (adler >> 16) & 0xffff;
    let mut adler = adler & 0xffff;

    for block in buf.chunks(NMAX_USIZE) {
        (adler, sum2) = accumulate_block(adler, sum2, block);
    }

    (adler, sum2) = reduce(adler, sum2);

    (adler | sum2 << 16) as uLong
}

pub fn adler32(adler: uLong, buf: &[Bytef]) -> uLong {
    adler32_z(adler, buf)
}

fn adler32_ffi_input(adler: uLong, input: Option<&[Bytef]>) -> uLong {
    input.map_or(1, |input| adler32_z(adler, input))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FfiInputKind {
    Null,
    Empty,
    NonEmpty,
}

fn classify_ffi_input(buf_is_null: bool, len: usize) -> FfiInputKind {
    if buf_is_null {
        FfiInputKind::Null
    } else if len == 0 {
        FfiInputKind::Empty
    } else {
        FfiInputKind::NonEmpty
    }
}

fn adler32_combine_(adler1: uLong, adler2: uLong, len2: off64_t) -> uLong {
    if len2 < 0 {
        return 0xffff_ffff;
    }

    let rem = (len2 % BASE as off64_t) as u64;
    let mut sum1 = adler1 as u64 & 0xffff;
    let mut sum2 = rem * sum1 % BASE_U64;

    sum1 += (adler2 as u64 & 0xffff) + BASE_U64 - 1;
    sum2 += ((adler1 as u64 >> 16) & 0xffff) + ((adler2 as u64 >> 16) & 0xffff) + BASE_U64 - rem;

    (sum1, sum2) = reduce_combine_sums(sum1, sum2);

    (sum1 | sum2 << 16) as uLong
}

pub fn adler32_combine(adler1: uLong, adler2: uLong, len2: off_t) -> uLong {
    adler32_combine_(adler1, adler2, len2 as off64_t)
}

pub fn adler32_combine64(adler1: uLong, adler2: uLong, len2: off64_t) -> uLong {
    adler32_combine_(adler1, adler2, len2)
}

#[export_name = "adler32_z"]
pub unsafe extern "C" fn adler32_z_ffi(adler: uLong, buf: *const Bytef, len: z_size_t) -> uLong {
    let input: Option<&[Bytef]> = match classify_ffi_input(buf.is_null(), len) {
        FfiInputKind::Null => None,
        FfiInputKind::Empty => Some(&[]),
        FfiInputKind::NonEmpty => Some(unsafe { core::slice::from_raw_parts(buf, len) }),
    };

    adler32_ffi_input(adler, input)
}

#[export_name = "adler32"]
pub unsafe extern "C" fn adler32_ffi(adler: uLong, buf: *const Bytef, len: uInt) -> uLong {
    let len = len as usize;
    let input: Option<&[Bytef]> = match classify_ffi_input(buf.is_null(), len) {
        FfiInputKind::Null => None,
        FfiInputKind::Empty => Some(&[]),
        FfiInputKind::NonEmpty => Some(unsafe { core::slice::from_raw_parts(buf, len) }),
    };

    adler32_ffi_input(adler, input)
}

#[export_name = "adler32_combine"]
pub unsafe extern "C" fn adler32_combine_ffi(adler1: uLong, adler2: uLong, len2: off_t) -> uLong {
    adler32_combine(adler1, adler2, len2)
}

#[export_name = "adler32_combine64"]
pub unsafe extern "C" fn adler32_combine64_ffi(
    adler1: uLong,
    adler2: uLong,
    len2: off64_t,
) -> uLong {
    adler32_combine64(adler1, adler2, len2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_adler32(adler: uLong, buf: &[Bytef]) -> uLong {
        let mut sum1 = adler as u64 & 0xffff;
        let mut sum2 = (adler as u64 >> 16) & 0xffff;

        for &byte in buf {
            sum1 = (sum1 + byte as u64) % BASE_U64;
            sum2 = (sum2 + sum1) % BASE_U64;
        }

        (sum1 | sum2 << 16) as uLong
    }

    #[test]
    fn ffi_empty_non_null_input_avoids_a_raw_slice() {
        let pointer = core::ptr::NonNull::<Bytef>::dangling().as_ptr();
        let seed = 0x1234_5678;

        assert_eq!(
            unsafe { adler32_z_ffi(seed, pointer, 0) },
            adler32_z(seed, &[])
        );
        assert_eq!(unsafe { adler32_ffi(seed, pointer, 0) }, adler32(seed, &[]));
        assert_eq!(unsafe { adler32_z_ffi(seed, core::ptr::null(), 0) }, 1);
        assert_eq!(unsafe { adler32_ffi(seed, core::ptr::null(), 0) }, 1);
    }

    #[test]
    fn ffi_input_preserves_null_and_slice_results() {
        let seed = 0x1234_5678;
        let input = b"input";

        assert_eq!(adler32_ffi_input(seed, None), 1);
        assert_eq!(adler32_ffi_input(seed, Some(input)), adler32_z(seed, input));
    }

    #[test]
    fn classifies_ffi_input_before_creating_a_raw_slice() {
        assert_eq!(classify_ffi_input(true, 0), FfiInputKind::Null);
        assert_eq!(classify_ffi_input(true, 1), FfiInputKind::Null);
        assert_eq!(classify_ffi_input(false, 0), FfiInputKind::Empty);
        assert_eq!(classify_ffi_input(false, 1), FfiInputKind::NonEmpty);
    }

    #[test]
    fn matches_known_vectors() {
        assert_eq!(adler32_z(1, b""), 1);
        assert_eq!(adler32_z(1, b"Wikipedia"), 0x11e6_0398);
        assert_eq!(adler32(1, b"123456789"), 0x091e_01de);
    }

    #[test]
    fn accumulates_a_block_from_seed_sums() {
        let seed = 0x1234_5678 as uLong;
        let block = b"block input";
        let sum1 = seed as u64 & 0xffff;
        let sum2 = (seed as u64 >> 16) & 0xffff;
        let (sum1, sum2) = accumulate_block(sum1, sum2, block);

        assert_eq!((sum1 | sum2 << 16) as uLong, reference_adler32(seed, block));
    }

    #[test]
    fn matches_reference_across_nmax_boundaries() {
        let input: Vec<Bytef> = (0..(NMAX_USIZE * 2 + 1))
            .map(|index| index.wrapping_mul(37).wrapping_add(11) as Bytef)
            .collect();
        let seed = 0x1234_5678 as uLong;

        for len in [0, NMAX_USIZE - 1, NMAX_USIZE, NMAX_USIZE + 1, input.len()] {
            let buf = &input[..len];
            assert_eq!(adler32_z(seed, buf), reference_adler32(seed, buf));
        }
    }

    #[test]
    fn reduces_combine_sums_across_conditional_boundaries() {
        assert_eq!(
            reduce_combine_sums(BASE_U64 - 1, BASE_U64 - 1),
            (BASE_U64 - 1, BASE_U64 - 1)
        );
        assert_eq!(reduce_combine_sums(BASE_U64, BASE_U64), (0, 0));
        assert_eq!(reduce_combine_sums(BASE_U64 << 1, BASE_U64 << 1), (0, 0));
        assert_eq!(
            reduce_combine_sums((BASE_U64 << 1) - 1, (BASE_U64 << 1) - 1),
            (BASE_U64 - 1, BASE_U64 - 1),
        );
        assert_eq!(reduce_combine_sums(BASE_U64 << 1, BASE_U64 * 3), (0, 0));
    }

    #[test]
    fn reduce_combine_sums_subtracts_sum1_at_most_twice() {
        assert_eq!(reduce_combine_sums(BASE_U64 * 3, 0).0, BASE_U64);
    }

    #[test]
    fn combines_checksums_for_concatenated_input() {
        let first = b"first part";
        let second = b" and second part";
        let mut combined_input = first.to_vec();
        combined_input.extend_from_slice(second);

        let first_adler = adler32(1, first);
        let second_adler = adler32(1, second);
        let expected = adler32(1, &combined_input);

        assert_eq!(
            adler32_combine(first_adler, second_adler, second.len() as off_t),
            expected
        );
        assert_eq!(
            adler32_combine64(first_adler, second_adler, second.len() as off64_t),
            expected
        );
    }

    #[test]
    fn rejects_negative_combine_lengths() {
        assert_eq!(adler32_combine64(1, 1, -1), 0xffff_ffff);
    }
}
