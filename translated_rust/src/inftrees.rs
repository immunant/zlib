// =============== BEGIN inftrees_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: ::core::ffi::c_uchar,
    pub bits: ::core::ffi::c_uchar,
    pub val: ::core::ffi::c_ushort,
}

pub const ENOUGH_LENS: ::core::ffi::c_int = 852 as ::core::ffi::c_int;

pub const ENOUGH_DISTS: ::core::ffi::c_int = 592 as ::core::ffi::c_int;

pub const ENOUGH: ::core::ffi::c_int =
    crate::src::inftrees::ENOUGH_LENS + crate::src::inftrees::ENOUGH_DISTS;

pub type codetype = ::core::ffi::c_uint;

pub const CODES: crate::src::inftrees::codetype = 0;

pub const LENS: crate::src::inftrees::codetype = 1;

pub const DISTS: crate::src::inftrees::codetype = 2;

pub mod inffixed_h {

    pub static lenfix: [crate::src::inftrees::code; 512] = [
        crate::src::inftrees::code {
            op: 96 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 80 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 16 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 115 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 112 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 48 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 192 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 96 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 32 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 160 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 128 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 64 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 224 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 88 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 24 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 144 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 120 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 56 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 208 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 104 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 40 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 176 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 136 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 72 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 240 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 84 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 20 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 227 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 116 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 52 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 200 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 100 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 36 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 168 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 132 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 68 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 232 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 92 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 28 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 152 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 124 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 60 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 216 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 108 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 44 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 184 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 12 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 140 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 76 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 248 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 82 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 18 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 163 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 114 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 50 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 196 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 98 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 34 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 164 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 2 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 130 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 66 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 228 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 90 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 26 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 148 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 122 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 58 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 212 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 106 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 42 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 180 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 138 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 74 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 244 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 86 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 22 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 118 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 54 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 204 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 102 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 38 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 172 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 134 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 70 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 236 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 94 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 30 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 156 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 126 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 62 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 220 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 110 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 46 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 188 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 14 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 142 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 78 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 252 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 96 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 81 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 131 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 113 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 49 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 194 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 97 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 33 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 162 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 1 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 129 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 65 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 226 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 89 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 25 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 146 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 121 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 57 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 210 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 105 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 41 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 178 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 137 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 73 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 242 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 85 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 21 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 258 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 117 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 53 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 202 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 101 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 37 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 170 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 133 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 69 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 234 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 93 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 29 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 154 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 125 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 61 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 218 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 109 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 45 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 186 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 141 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 77 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 250 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 195 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 115 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 198 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 166 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 131 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 230 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 91 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 150 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 123 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 214 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 107 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 182 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 139 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 75 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 246 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 87 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 119 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 55 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 206 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 103 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 39 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 174 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 135 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 71 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 238 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 95 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 158 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 127 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 63 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 222 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 111 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 47 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 190 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 143 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 79 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 254 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 96 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 80 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 16 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 115 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 112 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 48 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 193 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 96 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 32 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 161 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 128 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 64 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 225 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 88 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 24 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 145 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 120 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 56 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 209 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 104 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 40 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 177 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 136 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 72 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 241 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 84 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 20 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 227 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 116 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 52 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 201 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 100 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 36 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 169 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 132 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 68 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 233 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 92 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 28 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 153 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 124 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 60 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 217 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 108 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 44 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 185 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 12 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 140 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 76 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 249 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 82 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 18 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 163 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 114 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 50 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 197 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 98 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 34 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 165 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 2 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 130 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 66 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 229 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 90 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 26 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 149 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 122 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 58 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 213 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 106 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 42 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 181 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 138 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 74 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 245 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 86 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 22 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 118 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 54 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 205 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 102 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 38 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 173 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 134 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 70 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 237 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 94 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 30 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 157 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 126 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 62 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 221 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 110 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 46 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 189 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 14 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 142 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 78 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 253 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 96 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 81 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 131 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 113 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 49 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 195 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 97 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 33 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 163 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 1 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 129 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 65 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 227 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 89 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 25 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 147 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 121 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 57 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 211 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 105 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 41 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 179 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 137 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 73 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 243 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 85 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 21 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 258 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 117 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 53 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 203 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 101 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 37 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 171 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 133 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 69 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 235 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 93 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 29 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 155 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 125 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 61 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 219 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 109 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 45 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 187 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 141 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 77 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 251 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 195 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 115 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 199 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 167 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 131 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 231 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 91 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 151 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 123 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 215 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 107 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 183 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 139 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 75 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 247 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 87 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 119 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 55 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 207 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 103 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 39 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 175 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 135 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 71 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 239 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 95 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 159 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 127 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 63 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 223 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 111 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 47 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 191 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 143 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 79 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 255 as ::core::ffi::c_ushort,
        },
    ];

    pub static distfix: [crate::src::inftrees::code; 32] = [
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 1 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 23 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 257 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 27 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 4097 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 25 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 1025 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 65 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 29 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 16385 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 24 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 513 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 33 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 28 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 8193 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 26 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 2049 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 22 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 129 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 2 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 23 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 385 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 25 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 27 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 6145 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 25 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 1537 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 97 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 29 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 24577 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 24 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 769 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 49 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 28 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 12289 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 26 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 3073 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 22 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 193 as ::core::ffi::c_ushort,
        },
        crate::src::inftrees::code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
    ];
}

pub use crate::__stddef_null_h::NULL;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate_mode;
pub use crate::src::inflate::inflate_state;
pub use crate::src::inflate::BAD;
pub use crate::src::inflate::CHECK;
pub use crate::src::inflate::CODELENS;
pub use crate::src::inflate::COMMENT;
pub use crate::src::inflate::COPY_;
pub use crate::src::inflate::COPY_1;
pub use crate::src::inflate::DICT;
pub use crate::src::inflate::DICTID;
pub use crate::src::inflate::DIST;
pub use crate::src::inflate::DISTEXT;
pub use crate::src::inflate::DONE;
pub use crate::src::inflate::EXLEN;
pub use crate::src::inflate::EXTRA;
pub use crate::src::inflate::FLAGS;
pub use crate::src::inflate::HCRC;
pub use crate::src::inflate::HEAD;
pub use crate::src::inflate::LEN;
pub use crate::src::inflate::LENEXT;
pub use crate::src::inflate::LENGTH;
pub use crate::src::inflate::LENLENS;
pub use crate::src::inflate::LEN_;
pub use crate::src::inflate::LIT;
pub use crate::src::inflate::MATCH;
pub use crate::src::inflate::MEM;
pub use crate::src::inflate::NAME;
pub use crate::src::inflate::OS;
pub use crate::src::inflate::STORED;
pub use crate::src::inflate::SYNC;
pub use crate::src::inflate::TABLE;
pub use crate::src::inflate::TIME;
pub use crate::src::inflate::TYPE;
pub use crate::src::inflate::TYPEDO;
pub use crate::src::inftrees::inffixed_h::distfix;
pub use crate::src::inftrees::inffixed_h::lenfix;
pub(crate) fn fixed_len_code(index: usize) -> crate::src::inftrees::code {
    lenfix
        .get(index)
        .copied()
        .unwrap_or(crate::src::inflate::INVALID_DECODE_CODE)
}

pub(crate) fn fixed_dist_code(index: usize) -> crate::src::inftrees::code {
    distfix
        .get(index)
        .copied()
        .unwrap_or(crate::src::inflate::INVALID_DECODE_CODE)
}
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gz_header;
pub use crate::zlib_h::gz_header_s;
pub use crate::zlib_h::gz_headerp;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;

pub const MAXBITS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
#[no_mangle]
pub static inflate_copyright: [u8; 49] = *b" inflate 1.3.2.1 Copyright 1995-2026 Mark Adler \0";
const LBASE: [u16; 31] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258, 0, 0,
];
const LEXT: [u16; 31] = [
    16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20, 20, 20,
    21, 21, 21, 21, 16, 68, 193,
];
const DBASE: [u16; 32] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577, 0, 0,
];
const DEXT: [u16; 32] = [
    16, 16, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26,
    27, 27, 28, 28, 29, 29, 64, 64,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CodeType {
    Codes,
    Lens,
    Dists,
}

impl CodeType {
    fn from_raw(type_0: crate::src::inftrees::codetype) -> Option<Self> {
        match type_0 {
            crate::src::inftrees::CODES => Some(Self::Codes),
            crate::src::inftrees::LENS => Some(Self::Lens),
            crate::src::inftrees::DISTS => Some(Self::Dists),
            _ => None,
        }
    }

    fn table_capacity(self) -> usize {
        match self {
            Self::Codes => 128,
            Self::Lens => ENOUGH_LENS as usize,
            Self::Dists => ENOUGH_DISTS as usize,
        }
    }

    fn maximum_used_entries(self) -> Option<u32> {
        match self {
            Self::Codes => None,
            Self::Lens => Some(ENOUGH_LENS as u32),
            Self::Dists => Some(ENOUGH_DISTS as u32),
        }
    }
}

fn code_type(type_0: crate::src::inftrees::codetype) -> Option<CodeType> {
    CodeType::from_raw(type_0)
}

fn table_capacity_for_type(type_0: crate::src::inftrees::codetype) -> Option<usize> {
    code_type(type_0).map(CodeType::table_capacity)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TableCursor {
    start: usize,
    table_len: usize,
}

impl TableCursor {
    fn new(start: usize, table_len: usize) -> Option<Self> {
        (start <= table_len).then_some(Self { start, table_len })
    }

    fn index(self, table_offset: usize, entry_offset: usize) -> Option<usize> {
        let index = self
            .start
            .checked_add(table_offset)?
            .checked_add(entry_offset)?;
        (index < self.table_len).then_some(index)
    }

    fn entry_mut(
        self,
        table: &mut [crate::src::inftrees::code],
        table_offset: usize,
        entry_offset: usize,
    ) -> Option<&mut crate::src::inftrees::code> {
        table.get_mut(self.index(table_offset, entry_offset)?)
    }

    fn write_entries(
        self,
        table: &mut [crate::src::inftrees::code],
        table_offset: usize,
        entries: &[crate::src::inftrees::code],
    ) -> Option<usize> {
        let end_offset = table_offset.checked_add(entries.len())?;
        let end = self.end(end_offset)?;
        let start = self.start.checked_add(table_offset)?;
        table.get_mut(start..end)?.copy_from_slice(entries);
        Some(end)
    }

    fn end(self, table_offset: usize) -> Option<usize> {
        let end = self.start.checked_add(table_offset)?;
        (end <= self.table_len).then_some(end)
    }

    fn advance(self, table_offset: usize, table_size: u32) -> Option<usize> {
        let next_offset = table_offset.checked_add(table_size as usize)?;
        self.end(next_offset).map(|_| next_offset)
    }
}

fn table_inputs_fit(codes: usize, work_len: usize) -> bool {
    codes <= u16::MAX as usize && work_len >= codes
}

fn code_input_buffers_are_valid(
    codes: ::core::ffi::c_uint,
    lens_present: bool,
    work_present: bool,
) -> bool {
    codes == 0 || (lens_present && work_present)
}

fn ffi_table_capacity(
    type_0: crate::src::inftrees::codetype,
    codes: ::core::ffi::c_uint,
    table_out_present: bool,
    bits_present: bool,
    lens_present: bool,
    work_present: bool,
) -> Option<usize> {
    (table_out_present
        && bits_present
        && code_input_buffers_are_valid(codes, lens_present, work_present))
    .then(|| table_capacity_for_type(type_0))?
}

fn table_usage_fits(type_0: CodeType, used: u32, table_cursor: TableCursor) -> bool {
    let within_type_capacity = type_0
        .maximum_used_entries()
        .map_or(true, |maximum| used <= maximum);
    within_type_capacity && table_cursor.end(used as usize).is_some()
}

fn subtable_is_needed(length: u32, root: u32, huff: u32, mask: u32, low: u32) -> bool {
    length > root && (huff & mask) != low
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SubtableLayout {
    drop_bits: u32,
    curr: u32,
    used: u32,
    low: u32,
}

fn subtable_layout(
    length: u32,
    root: u32,
    huff: u32,
    mask: u32,
    low: u32,
    drop_bits: u32,
    max: u32,
    counts: &[u16; MAXBITS as usize + 1],
    used: u32,
) -> Option<SubtableLayout> {
    if !subtable_is_needed(length, root, huff, mask, low) {
        return None;
    }

    let drop_bits = if drop_bits == 0 { root } else { drop_bits };
    let mut curr = length - drop_bits;
    let mut left = 1i32 << curr;
    while curr + drop_bits < max {
        left -= counts[(curr + drop_bits) as usize] as i32;
        if left <= 0 {
            break;
        }
        curr += 1;
        left <<= 1;
    }

    Some(SubtableLayout {
        drop_bits,
        curr,
        used: used + (1u32 << curr),
        low: huff & mask,
    })
}

fn next_huffman_code(mut huff: u32, length: u32) -> u32 {
    if !(1..=MAXBITS as u32).contains(&length) {
        return huff;
    }
    let mut increment = 1u32 << (length - 1);
    while huff & increment != 0 {
        increment >>= 1;
    }
    if increment != 0 {
        huff &= increment - 1;
        huff += increment;
    } else {
        huff = 0;
    }
    huff
}

fn write_replicated_table_entries(
    table_cursor: TableCursor,
    table: &mut [crate::src::inftrees::code],
    table_offset: usize,
    huff: u32,
    drop_bits: u32,
    curr: u32,
    length: u32,
    here: crate::src::inftrees::code,
) -> Result<u32, ::core::ffi::c_int> {
    let bits = length.checked_sub(drop_bits).ok_or(1)?;
    let increment = 1u32.checked_shl(bits).ok_or(1)?;
    let table_size = 1u32.checked_shl(curr).ok_or(1)?;
    let huff_offset = huff >> drop_bits;
    let mut fill = table_size;

    loop {
        fill = fill.checked_sub(increment).ok_or(1)?;
        let entry_offset =
            usize::try_from(huff_offset.checked_add(fill).ok_or(1)?).map_err(|_| 1)?;
        let entry = table_cursor
            .entry_mut(table, table_offset, entry_offset)
            .ok_or(1)?;
        *entry = here;
        if fill == 0 {
            return Ok(table_size);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SymbolAdvance {
    Next { symbol: usize, length: u32 },
    Complete,
}

fn advance_symbol(
    symbol: usize,
    length: u32,
    max: u32,
    count: &mut [u16],
    work: &[u16],
    lens: &[u16],
) -> Result<SymbolAdvance, ::core::ffi::c_int> {
    let next_symbol = symbol.checked_add(1).ok_or(1)?;
    let length_index = usize::try_from(length).map_err(|_| 1)?;
    let remaining = count.get_mut(length_index).ok_or(1)?;
    *remaining = remaining.wrapping_sub(1);
    if *remaining != 0 {
        return Ok(SymbolAdvance::Next {
            symbol: next_symbol,
            length,
        });
    }
    if length == max {
        return Ok(SymbolAdvance::Complete);
    }

    let next_work_code = *work.get(next_symbol).ok_or(1)?;
    let next_length = *lens.get(usize::from(next_work_code)).ok_or(-1)?;
    Ok(SymbolAdvance::Next {
        symbol: next_symbol,
        length: u32::from(next_length),
    })
}

fn table_entry_for_symbol(
    type_0: CodeType,
    symbol: u16,
    bits: u8,
) -> Option<crate::src::inftrees::code> {
    let (base, extra, match_symbol): (&[u16], &[u16], u16) = match type_0 {
        CodeType::Codes => (&[], &[], 20),
        CodeType::Lens => (&LBASE, &LEXT, 257),
        CodeType::Dists => (&DBASE, &DEXT, 0),
    };
    if u32::from(symbol) + 1 < u32::from(match_symbol) {
        Some(crate::src::inftrees::code {
            op: 0,
            bits,
            val: symbol,
        })
    } else if symbol >= match_symbol {
        let index = (symbol - match_symbol) as usize;
        Some(crate::src::inftrees::code {
            op: *extra.get(index)? as u8,
            bits,
            val: *base.get(index)?,
        })
    } else {
        Some(crate::src::inftrees::code {
            op: 96,
            bits,
            val: 0,
        })
    }
}

fn normalized_root_bits(requested_root: u32, min: u32, max: u32) -> u32 {
    requested_root.min(max).max(min)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LengthState {
    counts: [u16; MAXBITS as usize + 1],
    min: u32,
    max: u32,
    root: u32,
}

impl LengthState {
    fn try_new(
        type_0: CodeType,
        lens: &[u16],
        requested_root: u32,
    ) -> Result<Option<Self>, ::core::ffi::c_int> {
        let mut counts = [0u16; MAXBITS as usize + 1];
        for &length in lens {
            if length as usize > MAXBITS as usize {
                return Err(-1);
            }
            counts[length as usize] = counts[length as usize].wrapping_add(1);
        }

        let mut max = MAXBITS as u32;
        while max >= 1 && counts[max as usize] == 0 {
            max -= 1;
        }
        if max == 0 {
            return Ok(None);
        }

        let mut min = 1u32;
        while min < max && counts[min as usize] == 0 {
            min += 1;
        }

        let root = normalized_root_bits(requested_root, min, max);

        let mut left = 1i32;
        for length in 1..=MAXBITS as usize {
            left <<= 1;
            left -= counts[length] as i32;
            if left < 0 {
                return Err(-1);
            }
        }
        if left > 0 && (type_0 == CodeType::Codes || max != 1) {
            return Err(-1);
        }

        Ok(Some(Self {
            counts,
            min,
            max,
            root,
        }))
    }

    fn write_symbol_order(self, lens: &[u16], work: &mut [u16]) -> Result<(), ::core::ffi::c_int> {
        let mut offs = [0u16; MAXBITS as usize + 1];
        for length in 1..MAXBITS as usize {
            offs[length + 1] = offs[length].wrapping_add(self.counts[length]);
        }
        for (symbol, &length) in lens.iter().enumerate() {
            if length != 0 {
                let offset = offs[length as usize] as usize;
                let Some(entry) = work.get_mut(offset) else {
                    return Err(1);
                };
                *entry = symbol as u16;
                offs[length as usize] = offs[length as usize].wrapping_add(1);
            }
        }
        Ok(())
    }
}

fn inflate_table_core(
    type_0: crate::src::inftrees::codetype,
    lens: &[u16],
    table: &mut [crate::src::inftrees::code],
    bits: &mut u32,
    work: &mut [u16],
) -> Result<usize, ::core::ffi::c_int> {
    let table_cursor = TableCursor {
        start: 0,
        table_len: table.len(),
    };

    let Some(type_0) = code_type(type_0) else {
        return Err(-1);
    };

    let length_state = match LengthState::try_new(type_0, lens, *bits) {
        Err(error) => return Err(error),
        Ok(None) => {
            let here = crate::src::inftrees::code {
                op: 64,
                bits: 1,
                val: 0,
            };
            if table_cursor.write_entries(table, 0, &[here; 2]).is_none() {
                return Err(1);
            };
            *bits = 1;
            return Ok(2);
        }
        Ok(Some(length_state)) => length_state,
    };
    let mut count = length_state.counts;
    let min = length_state.min;
    let max = length_state.max;
    let root = length_state.root;
    if length_state.write_symbol_order(lens, work).is_err() {
        return Err(1);
    }

    let mut huff = 0u32;
    let mut symbol = 0usize;
    let mut length = min;
    let mut next = 0usize;
    let mut curr = root;
    let mut drop_bits = 0u32;
    let mut low = u32::MAX;
    let mut used = 1u32 << root;
    let mask = used - 1;
    if !table_usage_fits(type_0, used, table_cursor) {
        return Err(1);
    }

    loop {
        let Some(&work_code) = work.get(symbol) else {
            return Err(1);
        };
        let Some(here) = table_entry_for_symbol(type_0, work_code, (length - drop_bits) as u8)
        else {
            return Err(-1);
        };

        let next_table_size = match write_replicated_table_entries(
            table_cursor,
            table,
            next,
            huff,
            drop_bits,
            curr,
            length,
            here,
        ) {
            Ok(table_size) => table_size,
            Err(error) => return Err(error),
        };

        huff = next_huffman_code(huff, length);

        match advance_symbol(symbol, length, max, &mut count, work, lens) {
            Err(error) => return Err(error),
            Ok(SymbolAdvance::Complete) => break,
            Ok(SymbolAdvance::Next {
                symbol: next_symbol,
                length: next_length,
            }) => {
                symbol = next_symbol;
                length = next_length;
            }
        }

        if let Some(layout) =
            subtable_layout(length, root, huff, mask, low, drop_bits, max, &count, used)
        {
            drop_bits = layout.drop_bits;
            let Some(next_cursor) = table_cursor.advance(next, next_table_size) else {
                return Err(1);
            };
            next = next_cursor;
            curr = layout.curr;
            used = layout.used;
            if !table_usage_fits(type_0, used, table_cursor) {
                return Err(1);
            }
            low = layout.low;
            let Some(entry) = table_cursor.entry_mut(table, 0, low as usize) else {
                return Err(1);
            };
            entry.op = curr as u8;
            entry.bits = root as u8;
            entry.val = next as u16;
        }
    }

    if huff != 0 {
        let Some(entry) = table_cursor.entry_mut(table, next, huff as usize) else {
            return Err(1);
        };
        *entry = crate::src::inftrees::code {
            op: 64,
            bits: (length - drop_bits) as u8,
            val: 0,
        };
    }
    if table_cursor.end(used as usize).is_none() {
        return Err(1);
    };
    *bits = root;
    Ok(used as usize)
}

/// Builds an inflate decoding table using bounded Rust slices.
///
/// `table_cursor` is the index at which the table starts and is advanced by
/// the number of entries used on success. `work` must have at least
/// `lens.len()` elements. As in zlib, `0` is success, `-1` reports an invalid
/// code-length set, and `1` reports insufficient table or workspace capacity.
pub fn inflate_table_safe(
    type_0: crate::src::inftrees::codetype,
    lens: &[u16],
    table: &mut [crate::src::inftrees::code],
    table_cursor_out: &mut usize,
    bits: &mut u32,
    work: &mut [u16],
) -> ::core::ffi::c_int {
    if !table_inputs_fit(lens.len(), work.len()) {
        return 1;
    }
    let Some(table_cursor) = TableCursor::new(*table_cursor_out, table.len()) else {
        return 1;
    };
    let Some(table) = table.get_mut(table_cursor.start..) else {
        return 1;
    };

    match inflate_table_core(type_0, lens, table, bits, work) {
        Ok(used) => {
            let Some(end) = table_cursor.end(used) else {
                return 1;
            };
            *table_cursor_out = end;
            0
        }
        Err(error) => error,
    }
}

#[export_name = "inflate_table"]
pub unsafe extern "C" fn inflate_table_ffi(
    type_0: crate::src::inftrees::codetype,
    lens: *mut ::core::ffi::c_ushort,
    codes: ::core::ffi::c_uint,
    table_out: *mut *mut crate::src::inftrees::code,
    bits: *mut ::core::ffi::c_uint,
    work: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    // Check alignment before dereferencing the caller's output pointers or
    // converting its buffers to Rust references.  A non-null, misaligned C
    // pointer is invalid for this API, but rejecting it here keeps the FFI
    // boundary from invoking Rust's alignment requirements on it.
    if table_out.align_offset(core::mem::align_of::<*mut crate::src::inftrees::code>()) != 0
        || bits.align_offset(core::mem::align_of::<::core::ffi::c_uint>()) != 0
    {
        return -1;
    }
    if codes != 0
        && (lens.align_offset(core::mem::align_of::<::core::ffi::c_ushort>()) != 0
            || work.align_offset(core::mem::align_of::<::core::ffi::c_ushort>()) != 0)
    {
        return -1;
    }
    let Some(table_capacity) = ffi_table_capacity(
        type_0,
        codes,
        !table_out.is_null(),
        !bits.is_null(),
        !lens.is_null(),
        !work.is_null(),
    ) else {
        return -1;
    };
    let table_start = *table_out;
    if table_start.is_null()
        || table_start.align_offset(core::mem::align_of::<crate::src::inftrees::code>()) != 0
    {
        return -1;
    }

    let codes = codes as usize;
    let lens = if codes == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(lens, codes)
    };
    let work = if codes == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(work, codes)
    };
    let table = ::core::slice::from_raw_parts_mut(table_start, table_capacity);
    let mut table_cursor = 0;
    let result = inflate_table_safe(type_0, lens, table, &mut table_cursor, &mut *bits, work);
    if result == 0 {
        let table_tail = table
            .get_mut(table_cursor..)
            .expect("successful table build keeps cursor in bounds");
        *table_out = table_tail.as_mut_ptr();
    }
    result
}

pub use inflate_table_ffi as inflate_table;
pub(crate) fn inflate_fixed(state: &mut crate::src::inflate::inflate_state) {
    state.lencode = crate::src::inflate::DecodeTableLocation::fixed_lens();
    state.lenbits = 9;
    state.distcode = crate::src::inflate::DecodeTableLocation::fixed_dists();
    state.distbits = 5;
}
#[export_name = "inflate_fixed"]

pub unsafe extern "C" fn inflate_fixed_ffi(mut state: *mut crate::src::inflate::inflate_state) {
    // `inflate_fixed` is a safe state-only core.  Establish its reference at
    // this exported boundary only after rejecting the invalid C pointers that
    // would violate Rust's reference requirements.
    if state.is_null()
        || state.align_offset(core::mem::align_of::<crate::src::inflate::inflate_state>()) != 0
    {
        return;
    }
    inflate_fixed(&mut *state)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_table_entry(entry: Option<code>, op: u8, bits: u8, val: u16) {
        let entry = entry.expect("expected a table entry");
        assert_eq!(entry.op, op);
        assert_eq!(entry.bits, bits);
        assert_eq!(entry.val, val);
    }

    #[test]
    fn subtable_gate_respects_length_and_mask_boundaries() {
        assert!(!subtable_is_needed(5, 5, 0b1101, 0b0111, 0b0101));
        assert!(!subtable_is_needed(6, 5, 0b1101, 0b0111, 0b0101));
        assert!(subtable_is_needed(6, 5, 0b1101, 0b0111, 0b0100));
    }

    #[test]
    fn subtable_layout_keeps_existing_root_table_when_not_needed() {
        let counts = [0u16; MAXBITS as usize + 1];

        assert_eq!(
            subtable_layout(8, 7, 0b101_0011, 0b111_1111, 0b101_0011, 7, 12, &counts, 128),
            None
        );
    }

    #[test]
    fn subtable_layout_selects_width_and_tracks_table_usage() {
        let mut counts = [0u16; MAXBITS as usize + 1];
        counts[10] = 1;
        counts[11] = 1;

        assert_eq!(
            subtable_layout(10, 7, 0b101_0011, 0b111_1111, 0, 0, 12, &counts, 128),
            Some(SubtableLayout {
                drop_bits: 7,
                curr: 5,
                used: 160,
                low: 0b101_0011,
            })
        );
    }

    #[test]
    fn next_huffman_code_reverses_the_canonical_increment() {
        assert_eq!(next_huffman_code(0b0000, 3), 0b0100);
        assert_eq!(next_huffman_code(0b0100, 3), 0b0010);
        assert_eq!(next_huffman_code(0b0110, 3), 0b0001);
        assert_eq!(next_huffman_code(0b0111, 3), 0);
    }

    #[test]
    fn next_huffman_code_leaves_invalid_lengths_unchanged() {
        assert_eq!(next_huffman_code(0b1010, 0), 0b1010);
        assert_eq!(next_huffman_code(0b1010, 32), 0b1010);
    }

    #[test]
    fn replicated_entries_fill_each_matching_decode_slot() {
        let sentinel = code {
            op: 7,
            bits: 8,
            val: 9,
        };
        let here = code {
            op: 3,
            bits: 2,
            val: 42,
        };
        let mut table = [sentinel; 6];
        let cursor = TableCursor::new(0, table.len()).expect("valid cursor");

        assert_eq!(
            write_replicated_table_entries(cursor, &mut table, 0, 0b110, 1, 2, 2, here),
            Ok(4)
        );
        assert_eq!(table[3].op, here.op);
        assert_eq!(table[3].bits, here.bits);
        assert_eq!(table[3].val, here.val);
        assert_eq!(table[5].op, here.op);
        assert_eq!(table[5].bits, here.bits);
        assert_eq!(table[5].val, here.val);
        for &index in &[0, 1, 2, 4] {
            assert_eq!(table[index].op, sentinel.op);
            assert_eq!(table[index].bits, sentinel.bits);
            assert_eq!(table[index].val, sentinel.val);
        }
    }

    #[test]
    fn replicated_entries_reject_an_out_of_bounds_first_slot() {
        let sentinel = code {
            op: 7,
            bits: 8,
            val: 9,
        };
        let mut table = [sentinel; 6];
        let cursor = TableCursor::new(0, table.len()).expect("valid cursor");

        assert_eq!(
            write_replicated_table_entries(cursor, &mut table, 0, 0b110, 0, 2, 2, sentinel),
            Err(1)
        );
        for entry in table {
            assert_eq!(entry.op, sentinel.op);
            assert_eq!(entry.bits, sentinel.bits);
            assert_eq!(entry.val, sentinel.val);
        }
    }

    #[test]
    fn symbol_advance_preserves_wrapping_count_and_current_length() {
        let mut count = [0u16; MAXBITS as usize + 1];
        let work = [0u16, 1];
        let lens = [3u16, 4];

        assert_eq!(
            advance_symbol(0, 3, 4, &mut count, &work, &lens),
            Ok(SymbolAdvance::Next {
                symbol: 1,
                length: 3,
            })
        );
        assert_eq!(count[3], u16::MAX);
    }

    #[test]
    fn symbol_advance_selects_next_length_or_completes() {
        let mut count = [0u16; MAXBITS as usize + 1];
        count[3] = 1;
        let work = [0u16, 1];
        let lens = [3u16, 5];

        assert_eq!(
            advance_symbol(0, 3, 5, &mut count, &work, &lens),
            Ok(SymbolAdvance::Next {
                symbol: 1,
                length: 5,
            })
        );
        assert_eq!(count[3], 0);

        count[5] = 1;
        assert_eq!(
            advance_symbol(1, 5, 5, &mut count, &work, &lens),
            Ok(SymbolAdvance::Complete)
        );
        assert_eq!(count[5], 0);
    }

    #[test]
    fn symbol_advance_reports_missing_next_symbol_or_length() {
        let mut count = [0u16; MAXBITS as usize + 1];
        count[3] = 1;
        assert_eq!(advance_symbol(0, 3, 4, &mut count, &[0], &[3]), Err(1));

        count[3] = 1;
        assert_eq!(advance_symbol(0, 3, 4, &mut count, &[0, 2], &[3]), Err(-1));
    }

    #[test]
    fn table_cursor_enforces_table_bounds() {
        let cursor = TableCursor::new(2, 6).expect("cursor starts within table");
        let mut table = [code {
            op: 0,
            bits: 0,
            val: 0,
        }; 6];

        assert_eq!(cursor.index(1, 2), Some(5));
        assert_eq!(cursor.end(4), Some(6));
        assert_eq!(cursor.advance(1, 3), Some(4));
        assert_eq!(cursor.index(4, 0), None);
        assert_eq!(cursor.end(5), None);
        assert_eq!(cursor.advance(4, 1), None);
        let entries = [
            code {
                op: 1,
                bits: 2,
                val: 3,
            },
            code {
                op: 4,
                bits: 5,
                val: 6,
            },
        ];
        assert_eq!(cursor.write_entries(&mut table, 0, &entries), Some(4));
        assert_eq!(table[2].val, 3);
        assert_eq!(table[3].val, 6);
        assert_eq!(cursor.write_entries(&mut table, 4, &entries), None);
        cursor
            .entry_mut(&mut table, 1, 2)
            .expect("entry lies within cursor bounds")
            .val = 42;
        assert_eq!(table[5].val, 42);
        assert!(cursor.entry_mut(&mut table, 4, 0).is_none());
        assert_eq!(TableCursor::new(7, 6), None);
    }

    #[test]
    fn table_entry_for_symbol_classifies_code_entries() {
        assert_table_entry(table_entry_for_symbol(CodeType::Codes, 18, 7), 0, 7, 18);
        assert_table_entry(table_entry_for_symbol(CodeType::Codes, 19, 7), 96, 7, 0);
        assert!(table_entry_for_symbol(CodeType::Codes, 20, 7).is_none());
    }

    #[test]
    fn table_capacity_for_type_accepts_known_table_kinds() {
        assert_eq!(table_capacity_for_type(CODES), Some(128));
        assert_eq!(table_capacity_for_type(LENS), Some(ENOUGH_LENS as usize));
        assert_eq!(table_capacity_for_type(DISTS), Some(ENOUGH_DISTS as usize));
    }

    #[test]
    fn table_capacity_for_type_rejects_unknown_table_kinds() {
        assert_eq!(table_capacity_for_type(3), None);
    }

    #[test]
    fn table_inputs_fit_enforces_code_workspace_bounds() {
        assert!(table_inputs_fit(u16::MAX as usize, u16::MAX as usize));
        assert!(!table_inputs_fit(u16::MAX as usize + 1, usize::MAX));
        assert!(!table_inputs_fit(2, 1));
    }

    #[test]
    fn code_input_buffers_are_required_only_for_nonempty_codes() {
        assert!(code_input_buffers_are_valid(0, false, false));
        assert!(code_input_buffers_are_valid(0, true, true));
        assert!(code_input_buffers_are_valid(1, true, true));
        assert!(!code_input_buffers_are_valid(1, false, true));
        assert!(!code_input_buffers_are_valid(1, true, false));
        assert!(!code_input_buffers_are_valid(1, false, false));
    }

    #[test]
    fn ffi_table_capacity_validates_scalar_ffi_inputs() {
        assert_eq!(
            ffi_table_capacity(CODES, 0, true, true, false, false),
            Some(128)
        );
        assert_eq!(
            ffi_table_capacity(LENS, 1, true, true, true, true),
            Some(ENOUGH_LENS as usize)
        );
        assert_eq!(
            ffi_table_capacity(CODES, 0, false, true, false, false),
            None
        );
        assert_eq!(
            ffi_table_capacity(CODES, 0, true, false, false, false),
            None
        );
        assert_eq!(ffi_table_capacity(CODES, 1, true, true, false, true), None);
        assert_eq!(ffi_table_capacity(3, 0, true, true, false, false), None);
    }

    #[test]
    fn ffi_table_rejects_misaligned_pointers_before_creating_views() {
        #[repr(align(8))]
        struct AlignedBytes([u8; 16]);

        let mut table = [code {
            op: 0,
            bits: 0,
            val: 0,
        }; 128];
        let mut table_out = table.as_mut_ptr();
        let mut bits = 7u32;
        let mut bytes = AlignedBytes([0; 16]);

        let misaligned_table_out = bytes.0[1..].as_mut_ptr().cast::<*mut code>();
        assert_eq!(
            unsafe {
                inflate_table_ffi(
                    CODES,
                    core::ptr::null_mut(),
                    0,
                    misaligned_table_out,
                    &mut bits,
                    core::ptr::null_mut(),
                )
            },
            -1
        );

        let misaligned_bits = bytes.0[1..].as_mut_ptr().cast::<u32>();
        assert_eq!(
            unsafe {
                inflate_table_ffi(
                    CODES,
                    core::ptr::null_mut(),
                    0,
                    &mut table_out,
                    misaligned_bits,
                    core::ptr::null_mut(),
                )
            },
            -1
        );

        let misaligned_lens = bytes.0[1..].as_mut_ptr().cast::<u16>();
        assert_eq!(
            unsafe {
                inflate_table_ffi(
                    CODES,
                    misaligned_lens,
                    1,
                    &mut table_out,
                    &mut bits,
                    core::ptr::null_mut(),
                )
            },
            -1
        );
    }

    #[test]
    fn ffi_fixed_rejects_null_or_misaligned_state_before_creating_a_reference() {
        unsafe { inflate_fixed_ffi(core::ptr::null_mut()) };

        #[repr(align(8))]
        struct AlignedBytes([u8; 256]);

        let mut bytes = AlignedBytes([0; 256]);
        let misaligned = bytes.0[1..]
            .as_mut_ptr()
            .cast::<crate::src::inflate::inflate_state>();
        unsafe { inflate_fixed_ffi(misaligned) };
    }

    #[test]
    fn ffi_fixed_initializes_decode_table_locations() {
        let mut state = crate::src::inflate::inflate_state::newly_allocated();

        unsafe { inflate_fixed_ffi(&mut state) };

        assert_eq!(
            state.lencode,
            crate::src::inflate::DecodeTableLocation::fixed_lens()
        );
        assert_eq!(state.lenbits, 9);
        assert_eq!(
            state.distcode,
            crate::src::inflate::DecodeTableLocation::fixed_dists()
        );
        assert_eq!(state.distbits, 5);
    }

    #[test]
    fn table_usage_fits_enforces_type_and_slice_boundaries() {
        assert!(table_usage_fits(
            CodeType::Codes,
            129,
            TableCursor::new(2, 131).expect("valid cursor"),
        ));
        assert!(table_usage_fits(
            CodeType::Lens,
            ENOUGH_LENS as u32,
            TableCursor::new(0, ENOUGH_LENS as usize).expect("valid cursor"),
        ));
        assert!(!table_usage_fits(
            CodeType::Lens,
            ENOUGH_LENS as u32 + 1,
            TableCursor::new(0, ENOUGH_LENS as usize + 1).expect("valid cursor"),
        ));
        assert!(table_usage_fits(
            CodeType::Dists,
            ENOUGH_DISTS as u32,
            TableCursor::new(4, ENOUGH_DISTS as usize + 4).expect("valid cursor"),
        ));
        assert!(!table_usage_fits(
            CodeType::Dists,
            1,
            TableCursor::new(4, 4).expect("valid cursor"),
        ));
    }

    #[test]
    fn table_usage_fits_rejects_cursor_overflow() {
        let cursor = TableCursor::new(usize::MAX, usize::MAX).expect("valid cursor");
        assert!(!table_usage_fits(CodeType::Codes, 1, cursor));
    }

    #[test]
    fn table_entry_for_symbol_classifies_length_and_distance_entries() {
        assert_table_entry(
            table_entry_for_symbol(CodeType::Lens, 257, 4),
            LEXT[0] as u8,
            4,
            LBASE[0],
        );
        assert!(table_entry_for_symbol(CodeType::Lens, 288, 4).is_none());
        assert_table_entry(
            table_entry_for_symbol(CodeType::Dists, 29, 3),
            DEXT[29] as u8,
            3,
            DBASE[29],
        );
        assert!(table_entry_for_symbol(CodeType::Dists, 32, 3).is_none());
    }

    #[test]
    fn normalized_root_bits_clamps_to_observed_length_range() {
        assert_eq!(normalized_root_bits(2, 3, 7), 3);
        assert_eq!(normalized_root_bits(5, 3, 7), 5);
        assert_eq!(normalized_root_bits(9, 3, 7), 7);
    }

    #[test]
    fn length_state_normalizes_root_and_orders_symbols() {
        let lens = [3u16, 1, 3, 2];
        let state = LengthState::try_new(CodeType::Lens, &lens, 9)
            .expect("valid lengths")
            .expect("non-empty alphabet");
        let mut work = [0u16; 4];

        assert_eq!(state.min, 1);
        assert_eq!(state.max, 3);
        assert_eq!(state.root, 3);
        state
            .write_symbol_order(&lens, &mut work)
            .expect("workspace fits");
        assert_eq!(work, [1, 3, 0, 2]);
    }

    #[test]
    fn length_state_reports_empty_alphabet() {
        assert_eq!(
            LengthState::try_new(CodeType::Codes, &[0u16, 0], 7).expect("empty alphabet is valid"),
            None
        );
    }

    #[test]
    fn copyright_export_has_stable_bytes() {
        assert_eq!(
            inflate_copyright,
            *b" inflate 1.3.2.1 Copyright 1995-2026 Mark Adler \0"
        );
    }

    #[test]
    fn safe_table_builds_a_codes_table() {
        let lens = [1u16, 1];
        let mut table = [code {
            op: 0,
            bits: 0,
            val: 0,
        }; 2];
        let mut cursor = 0;
        let mut bits = 7;
        let mut work = [0u16; 2];

        assert_eq!(
            inflate_table_safe(CODES, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            0
        );
        assert_eq!(cursor, 2);
        assert_eq!(bits, 1);
        assert_eq!(table[0].val, 0);
        assert_eq!(table[1].val, 1);
    }

    #[test]
    fn safe_table_builds_after_cursor_without_touching_prefix() {
        let lens = [1u16, 1];
        let prefix = code {
            op: 7,
            bits: 8,
            val: 9,
        };
        let mut table = [prefix; 3];
        let mut cursor = 1;
        let mut bits = 7;
        let mut work = [0u16; 2];

        assert_eq!(
            inflate_table_safe(CODES, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            0
        );
        assert_eq!(cursor, 3);
        assert_eq!(bits, 1);
        assert_eq!(table[0].op, prefix.op);
        assert_eq!(table[0].bits, prefix.bits);
        assert_eq!(table[0].val, prefix.val);
        assert_eq!(table[1].val, 0);
        assert_eq!(table[2].val, 1);
    }

    #[test]
    fn table_core_reports_entries_used_from_slice_start() {
        let lens = [1u16, 1];
        let mut table = [code {
            op: 0,
            bits: 0,
            val: 0,
        }; 2];
        let mut bits = 7;
        let mut work = [0u16; 2];

        assert_eq!(
            inflate_table_core(CODES, &lens, &mut table, &mut bits, &mut work),
            Ok(2)
        );
        assert_eq!(bits, 1);
        assert_eq!(table[0].val, 0);
        assert_eq!(table[1].val, 1);
    }

    #[test]
    fn safe_table_builds_empty_alphabet_at_exact_table_boundary() {
        let lens = [0u16; 3];
        let mut table = [code {
            op: 7,
            bits: 8,
            val: 9,
        }; 2];
        let mut cursor = 0;
        let mut bits = MAXBITS as u32;
        let mut work = [0u16; 3];

        assert_eq!(
            inflate_table_safe(CODES, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            0
        );
        assert_eq!(cursor, table.len());
        assert_eq!(bits, 1);
        for entry in table {
            assert_eq!(entry.op, 64);
            assert_eq!(entry.bits, 1);
            assert_eq!(entry.val, 0);
        }
    }

    #[test]
    fn safe_table_rejects_insufficient_table_space() {
        let lens = [1u16, 1];
        let mut table = [];
        let mut cursor = 0;
        let mut bits = 7;
        let mut work = [0u16; 2];

        assert_eq!(
            inflate_table_safe(CODES, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            1
        );
    }

    #[test]
    fn safe_table_rejects_insufficient_workspace_before_writing_output() {
        let lens = [1u16, 1];
        let original_entry = code {
            op: 7,
            bits: 8,
            val: 9,
        };
        let mut table = [original_entry; 2];
        let mut cursor = 0;
        let mut bits = 7;
        let mut work = [0u16; 1];

        assert_eq!(
            inflate_table_safe(CODES, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            1
        );
        for entry in table {
            assert_eq!(entry.op, original_entry.op);
            assert_eq!(entry.bits, original_entry.bits);
            assert_eq!(entry.val, original_entry.val);
        }
        assert_eq!(cursor, 0);
        assert_eq!(bits, 7);
    }

    #[test]
    fn safe_table_rejects_malformed_lengths_before_writing_output() {
        let original_entry = code {
            op: 7,
            bits: 8,
            val: 9,
        };

        for lens in [&[16u16][..], &[1u16, 1, 1][..], &[2u16, 2][..]] {
            let mut table = [original_entry; 4];
            let mut cursor = 0;
            let mut bits = 7;
            let mut work = [0u16; 3];

            assert_eq!(
                inflate_table_safe(CODES, lens, &mut table, &mut cursor, &mut bits, &mut work),
                -1,
                "lens={lens:?}"
            );
            for entry in table {
                assert_eq!(entry.op, original_entry.op, "lens={lens:?}");
                assert_eq!(entry.bits, original_entry.bits, "lens={lens:?}");
                assert_eq!(entry.val, original_entry.val, "lens={lens:?}");
            }
            assert_eq!(cursor, 0, "lens={lens:?}");
            assert_eq!(bits, 7, "lens={lens:?}");
        }
    }

    #[test]
    fn safe_table_rejects_invalid_type_before_writing_output() {
        let lens = [1u16, 1];
        let original_entry = code {
            op: 7,
            bits: 8,
            val: 9,
        };
        let mut table = [original_entry; 2];
        let mut cursor = 0;
        let mut bits = 7;
        let mut work = [0u16; 2];

        assert_eq!(
            inflate_table_safe(3, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            -1
        );
        for entry in table {
            assert_eq!(entry.op, original_entry.op);
            assert_eq!(entry.bits, original_entry.bits);
            assert_eq!(entry.val, original_entry.val);
        }
        assert_eq!(cursor, 0);
        assert_eq!(bits, 7);
    }

    #[test]
    fn safe_table_rejects_cursor_beyond_table_without_writing_output() {
        let lens = [1u16, 1];
        let original_entry = code {
            op: 7,
            bits: 8,
            val: 9,
        };
        let mut table = [original_entry; 2];
        let mut cursor = table.len() + 1;
        let mut bits = 7;
        let mut work = [0u16; 2];

        assert_eq!(
            inflate_table_safe(CODES, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            1
        );
        for entry in table {
            assert_eq!(entry.op, original_entry.op);
            assert_eq!(entry.bits, original_entry.bits);
            assert_eq!(entry.val, original_entry.val);
        }
        assert_eq!(cursor, 3);
        assert_eq!(bits, 7);
    }

    #[test]
    fn safe_table_builds_fixed_lens_table_and_advances_cursor() {
        let mut lens = [8u16; 288];
        lens[144..256].fill(9);
        lens[256..280].fill(7);
        let mut table = [code {
            op: 0,
            bits: 0,
            val: 0,
        }; ENOUGH_LENS as usize];
        let mut cursor = 0;
        let mut bits = 9;
        let mut work = [0u16; 288];

        assert_eq!(
            inflate_table_safe(LENS, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            0
        );
        assert_eq!(cursor, lenfix.len());
        assert_eq!(bits, 9);
        for (actual, expected) in table[..cursor].iter().zip(lenfix.iter()) {
            if expected.op == 64 {
                assert_ne!(actual.op & 64, 0);
            } else {
                assert_eq!(actual.op, expected.op);
                assert_eq!(actual.val, expected.val);
            }
            assert_eq!(actual.bits, expected.bits);
        }
    }

    #[test]
    fn safe_table_builds_fixed_dists_table_and_advances_cursor() {
        let lens = [5u16; 32];
        let mut table = [code {
            op: 0,
            bits: 0,
            val: 0,
        }; ENOUGH_DISTS as usize];
        let mut cursor = 0;
        let mut bits = 6;
        let mut work = [0u16; 32];

        assert_eq!(
            inflate_table_safe(DISTS, &lens, &mut table, &mut cursor, &mut bits, &mut work),
            0
        );
        assert_eq!(cursor, distfix.len());
        assert_eq!(bits, 5);
        for (actual, expected) in table[..cursor].iter().zip(distfix.iter()) {
            assert_eq!(actual.op, expected.op);
            assert_eq!(actual.bits, expected.bits);
            assert_eq!(actual.val, expected.val);
        }
    }
}
