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

pub const BASE: ::core::ffi::c_uint = 65521 as ::core::ffi::c_uint;

pub const NMAX: ::core::ffi::c_int = 5552 as ::core::ffi::c_int;

/// Adler-32's documented value for an empty input stream.
pub(crate) const ADLER32_INITIAL: crate::stdlib::uLong = 1;

pub(crate) fn adler32_slice(
    mut adler: crate::stdlib::uLong,
    buf: &[crate::stdlib::Bytef],
) -> crate::stdlib::uLong {
    let base = BASE as crate::stdlib::uLong;
    let mut sum2 = adler >> 16 & 0xffff;
    adler &= 0xffff;

    if buf.len() == 1 {
        adler = adler.wrapping_add(buf[0] as crate::stdlib::uLong);
        if adler >= base {
            adler = adler.wrapping_sub(base);
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= base {
            sum2 = sum2.wrapping_sub(base);
        }
        return adler | sum2 << 16;
    }

    for block in buf.chunks(NMAX as usize) {
        for &byte in block {
            adler = adler.wrapping_add(byte as crate::stdlib::uLong);
            sum2 = sum2.wrapping_add(adler);
        }
        adler %= base;
        sum2 %= base;
    }

    adler | sum2 << 16
}

pub unsafe extern "C" fn adler32_z(
    adler: crate::stdlib::uLong,
    buf: *const crate::stdlib::Bytef,
    len: crate::stdlib::z_size_t,
) -> crate::stdlib::uLong {
    if buf.is_null() {
        return 1;
    }
    adler32_slice(adler, unsafe { core::slice::from_raw_parts(buf, len) })
}

#[export_name = "adler32_z"]

pub unsafe extern "C" fn adler32_z_ffi(
    adler: crate::stdlib::uLong,
    buf: *const crate::stdlib::Bytef,
    len: crate::stdlib::z_size_t,
) -> crate::stdlib::uLong {
    unsafe { adler32_z(adler, buf, len) }
}
#[export_name = "adler32"]

pub unsafe extern "C" fn adler32_ffi(
    adler: crate::stdlib::uLong,
    buf: *const crate::stdlib::Bytef,
    len: crate::stdlib::uInt,
) -> crate::stdlib::uLong {
    unsafe { adler32_z(adler, buf, len as crate::stdlib::z_size_t) }
}
fn adler32_combine_(
    adler1: crate::stdlib::uLong,
    adler2: crate::stdlib::uLong,
    len2: crate::stdlib::off64_t,
) -> crate::stdlib::uLong {
    if len2 < 0 {
        return 0xffffffff as crate::stdlib::uLong;
    }
    let base = BASE as crate::stdlib::uLong;
    let rem = (len2 % BASE as crate::stdlib::off64_t) as crate::stdlib::uLong;
    let mut sum1 = adler1 & 0xffff;
    let mut sum2 = rem.wrapping_mul(sum1) % base;
    sum1 = sum1.wrapping_add((adler2 & 0xffff).wrapping_add(base).wrapping_sub(1));
    sum2 = sum2.wrapping_add(
        (adler1 >> 16 & 0xffff)
            .wrapping_add(adler2 >> 16 & 0xffff)
            .wrapping_add(base)
            .wrapping_sub(rem),
    );
    if sum1 >= base {
        sum1 = sum1.wrapping_sub(base);
    }
    if sum1 >= base {
        sum1 = sum1.wrapping_sub(base);
    }
    if sum2 >= base << 1 {
        sum2 = sum2.wrapping_sub(base << 1);
    }
    if sum2 >= base {
        sum2 = sum2.wrapping_sub(base);
    }
    sum1 | sum2 << 16
}
pub fn adler32_combine(
    adler1: crate::stdlib::uLong,
    adler2: crate::stdlib::uLong,
    len2: crate::stdlib::off_t,
) -> crate::stdlib::uLong {
    adler32_combine_(adler1, adler2, len2 as crate::stdlib::off64_t)
}
#[export_name = "adler32_combine"]

pub unsafe extern "C" fn adler32_combine_ffi(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off_t,
) -> crate::stdlib::uLong {
    adler32_combine(adler1, adler2, len2)
}
pub fn adler32_combine64(
    adler1: crate::stdlib::uLong,
    adler2: crate::stdlib::uLong,
    len2: crate::stdlib::off64_t,
) -> crate::stdlib::uLong {
    adler32_combine_(adler1, adler2, len2)
}
#[export_name = "adler32_combine64"]

pub unsafe extern "C" fn adler32_combine64_ffi(
    mut adler1: crate::stdlib::uLong,
    mut adler2: crate::stdlib::uLong,
    mut len2: crate::stdlib::off64_t,
) -> crate::stdlib::uLong {
    adler32_combine64(adler1, adler2, len2)
}
