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

fn update_byte(adler: &mut u64, sum2: &mut u64, byte: Bytef) {
    *adler = adler.wrapping_add(byte as u64);
    *sum2 = sum2.wrapping_add(*adler);
}

fn update_bounded_block(adler: &mut u64, sum2: &mut u64, block: &[Bytef]) {
    debug_assert!(block.len() <= NMAX_USIZE);

    let mut groups = block.chunks_exact(16);
    for group in &mut groups {
        for &byte in group {
            update_byte(adler, sum2, byte);
        }
    }
    for &byte in groups.remainder() {
        update_byte(adler, sum2, byte);
    }
}

fn reduce(adler: &mut u64, sum2: &mut u64) {
    *adler %= BASE_U64;
    *sum2 %= BASE_U64;
}

pub fn adler32_z(adler: uLong, buf: &[Bytef]) -> uLong {
    let adler = adler as u64;
    let mut sum2 = (adler >> 16) & 0xffff;
    let mut adler = adler & 0xffff;

    for block in buf.chunks(NMAX_USIZE) {
        update_bounded_block(&mut adler, &mut sum2, block);
        reduce(&mut adler, &mut sum2);
    }

    reduce(&mut adler, &mut sum2);

    (adler | sum2 << 16) as uLong
}

pub fn adler32(adler: uLong, buf: &[Bytef]) -> uLong {
    adler32_z(adler, buf)
}

fn adler32_combine_(adler1: uLong, adler2: uLong, len2: off64_t) -> uLong {
    if len2 < 0 {
        return 0xffff_ffff;
    }

    let rem = (len2 % BASE as off64_t) as u64;
    let mut sum1 = adler1 as u64 & 0xffff;
    let mut sum2 = rem.wrapping_mul(sum1) % BASE_U64;

    sum1 = sum1.wrapping_add((adler2 as u64 & 0xffff).wrapping_add(BASE_U64 - 1));
    sum2 = sum2.wrapping_add(
        ((adler1 as u64 >> 16) & 0xffff)
            .wrapping_add((adler2 as u64 >> 16) & 0xffff)
            .wrapping_add(BASE_U64 - rem),
    );

    if sum1 >= BASE_U64 {
        sum1 -= BASE_U64;
    }
    if sum1 >= BASE_U64 {
        sum1 -= BASE_U64;
    }
    if sum2 >= BASE_U64 << 1 {
        sum2 -= BASE_U64 << 1;
    }
    if sum2 >= BASE_U64 {
        sum2 -= BASE_U64;
    }

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
    if buf.is_null() {
        return 1;
    }

    let input = unsafe { core::slice::from_raw_parts(buf, len) };
    adler32_z(adler, input)
}

#[export_name = "adler32"]
pub unsafe extern "C" fn adler32_ffi(adler: uLong, buf: *const Bytef, len: uInt) -> uLong {
    if buf.is_null() {
        return 1;
    }

    let input = unsafe { core::slice::from_raw_parts(buf, len as usize) };
    adler32(adler, input)
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
    fn matches_known_vectors() {
        assert_eq!(adler32_z(1, b""), 1);
        assert_eq!(adler32_z(1, b"Wikipedia"), 0x11e6_0398);
        assert_eq!(adler32(1, b"123456789"), 0x091e_01de);
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
}
