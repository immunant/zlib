pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::{off64_t, off_t, uInt, uLong, z_size_t, Bytef};

pub const BASE: ::core::ffi::c_uint = 65521;
pub const NMAX: ::core::ffi::c_int = 5552;

fn adler32_block(adler: &mut uLong, sum2: &mut uLong, bytes: &[Bytef]) {
    for &byte in bytes {
        *adler = adler.wrapping_add(byte as uLong);
        *sum2 = sum2.wrapping_add(*adler);
    }
}

pub fn adler32_z(adler: uLong, buf: Option<&[Bytef]>) -> uLong {
    let Some(buf) = buf else {
        return 1;
    };

    let mut sum2 = (adler >> 16) & 0xffff;
    let mut adler = adler & 0xffff;

    if buf.len() == 1 {
        adler = adler.wrapping_add(buf[0] as uLong);
        if adler >= BASE as uLong {
            adler = adler.wrapping_sub(BASE as uLong);
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= BASE as uLong {
            sum2 = sum2.wrapping_sub(BASE as uLong);
        }
        return adler | (sum2 << 16);
    }

    for block in buf.chunks(NMAX as usize) {
        adler32_block(&mut adler, &mut sum2, block);
        adler %= BASE as uLong;
        sum2 %= BASE as uLong;
    }
    adler | (sum2 << 16)
}

pub fn adler32(adler: uLong, buf: &[Bytef]) -> uLong {
    adler32_z(adler, Some(buf))
}

fn adler32_combine_(adler1: uLong, adler2: uLong, len2: off64_t) -> uLong {
    if len2 < 0 {
        return 0xffffffff;
    }
    let rem = (len2 % BASE as off64_t) as uLong;
    let mut sum1 = adler1 & 0xffff;
    let mut sum2 = rem.wrapping_mul(sum1) % BASE as uLong;
    sum1 = sum1.wrapping_add(
        (adler2 & 0xffff)
            .wrapping_add(BASE as uLong)
            .wrapping_sub(1),
    );
    sum2 = sum2.wrapping_add(
        (adler1 >> 16 & 0xffff)
            .wrapping_add(adler2 >> 16 & 0xffff)
            .wrapping_add(BASE as uLong)
            .wrapping_sub(rem),
    );
    if sum1 >= BASE as uLong {
        sum1 = sum1.wrapping_sub(BASE as uLong);
    }
    if sum1 >= BASE as uLong {
        sum1 = sum1.wrapping_sub(BASE as uLong);
    }
    if sum2 >= (BASE as uLong) << 1 {
        sum2 = sum2.wrapping_sub((BASE as uLong) << 1);
    }
    if sum2 >= BASE as uLong {
        sum2 = sum2.wrapping_sub(BASE as uLong);
    }
    sum1 | (sum2 << 16)
}

pub fn adler32_combine(adler1: uLong, adler2: uLong, len2: off_t) -> uLong {
    adler32_combine_(adler1, adler2, len2 as off64_t)
}

pub fn adler32_combine64(adler1: uLong, adler2: uLong, len2: off64_t) -> uLong {
    adler32_combine_(adler1, adler2, len2)
}

#[export_name = "adler32_z"]
pub unsafe extern "C" fn adler32_z_ffi(adler: uLong, buf: *const Bytef, len: z_size_t) -> uLong {
    let buf = if buf.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(buf, len) })
    };
    adler32_z(adler, buf)
}

#[export_name = "adler32"]
pub unsafe extern "C" fn adler32_ffi(adler: uLong, buf: *const Bytef, len: uInt) -> uLong {
    let buf = if buf.is_null() {
        None
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(buf, len as usize) })
    };
    adler32_z(adler, buf)
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
