use crate::stdlib::{off64_t, off_t, Bytef, uInt, uLong, z_size_t};

pub const BASE: uLong = 65_521;
pub const NMAX: usize = 5_552;

/// The checksum returned by zlib's `adler32(0, NULL, 0)` initializer query.
pub const ADLER32_INITIAL: uLong = 1;

/// Update an Adler-32 checksum with `buf`.
///
/// The input is processed in zlib's `NMAX`-sized blocks so that the running
/// sums stay bounded before each modular reduction.
pub fn adler32_z(mut adler: uLong, buf: &[Bytef]) -> uLong {
    let mut sum2 = (adler >> 16) & 0xffff;
    adler &= 0xffff;

    if buf.len() == 1 {
        adler = adler.wrapping_add(buf[0] as uLong);
        if adler >= BASE {
            adler = adler.wrapping_sub(BASE);
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= BASE {
            sum2 = sum2.wrapping_sub(BASE);
        }
        return adler | (sum2 << 16);
    }

    for block in buf.chunks(NMAX) {
        for &byte in block {
            adler = adler.wrapping_add(byte as uLong);
            sum2 = sum2.wrapping_add(adler);
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    adler | (sum2 << 16)
}

pub fn adler32(adler: uLong, buf: &[Bytef]) -> uLong {
    adler32_z(adler, buf)
}

#[export_name = "adler32_z"]
pub unsafe extern "C" fn adler32_z_ffi(adler: uLong, buf: *const Bytef, len: z_size_t) -> uLong {
    if buf.is_null() {
        return ADLER32_INITIAL;
    }
    adler32_z(adler, ::core::slice::from_raw_parts(buf, len))
}

#[export_name = "adler32"]
pub unsafe extern "C" fn adler32_ffi(adler: uLong, buf: *const Bytef, len: uInt) -> uLong {
    if buf.is_null() {
        return ADLER32_INITIAL;
    }
    adler32(adler, ::core::slice::from_raw_parts(buf, len as usize))
}

fn adler32_combine_(adler1: uLong, adler2: uLong, len2: off64_t) -> uLong {
    if len2 < 0 {
        return 0xffff_ffff;
    }

    let rem = (len2 % BASE as off64_t) as uLong;
    let mut sum1 = adler1 & 0xffff;
    let mut sum2 = rem.wrapping_mul(sum1) % BASE;
    sum1 = sum1.wrapping_add((adler2 & 0xffff).wrapping_add(BASE).wrapping_sub(1));
    sum2 = sum2.wrapping_add(
        ((adler1 >> 16) & 0xffff)
            .wrapping_add((adler2 >> 16) & 0xffff)
            .wrapping_add(BASE)
            .wrapping_sub(rem),
    );
    if sum1 >= BASE {
        sum1 = sum1.wrapping_sub(BASE);
    }
    if sum1 >= BASE {
        sum1 = sum1.wrapping_sub(BASE);
    }
    if sum2 >= BASE << 1 {
        sum2 = sum2.wrapping_sub(BASE << 1);
    }
    if sum2 >= BASE {
        sum2 = sum2.wrapping_sub(BASE);
    }
    sum1 | (sum2 << 16)
}

pub fn adler32_combine(adler1: uLong, adler2: uLong, len2: off_t) -> uLong {
    adler32_combine_(adler1, adler2, len2 as off64_t)
}

#[export_name = "adler32_combine"]
pub unsafe extern "C" fn adler32_combine_ffi(adler1: uLong, adler2: uLong, len2: off_t) -> uLong {
    adler32_combine(adler1, adler2, len2)
}

pub fn adler32_combine64(adler1: uLong, adler2: uLong, len2: off64_t) -> uLong {
    adler32_combine_(adler1, adler2, len2)
}

#[export_name = "adler32_combine64"]
pub unsafe extern "C" fn adler32_combine64_ffi(
    adler1: uLong,
    adler2: uLong,
    len2: off64_t,
) -> uLong {
    adler32_combine64(adler1, adler2, len2)
}
