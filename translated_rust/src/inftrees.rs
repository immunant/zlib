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

fn table_entry_for_symbol(
    type_0: crate::src::inftrees::codetype,
    symbol: u16,
    bits: u8,
) -> Option<crate::src::inftrees::code> {
    let (base, extra, match_symbol): (&[u16], &[u16], u16) = match type_0 {
        crate::src::inftrees::CODES => (&[], &[], 20),
        crate::src::inftrees::LENS => (&LBASE, &LEXT, 257),
        crate::src::inftrees::DISTS => (&DBASE, &DEXT, 0),
        _ => return None,
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
    table_cursor: &mut usize,
    bits: &mut u32,
    work: &mut [u16],
) -> ::core::ffi::c_int {
    let codes = lens.len();
    if codes > u16::MAX as usize || work.len() < codes || *table_cursor > table.len() {
        return 1;
    }

    if !matches!(
        type_0,
        crate::src::inftrees::CODES | crate::src::inftrees::LENS | crate::src::inftrees::DISTS
    ) {
        return -1;
    }

    let mut count = [0u16; MAXBITS as usize + 1];
    for &length in lens {
        if length as usize > MAXBITS as usize {
            return -1;
        }
        count[length as usize] = count[length as usize].wrapping_add(1);
    }

    let mut root = *bits;
    let mut max = MAXBITS as u32;
    while max >= 1 && count[max as usize] == 0 {
        max -= 1;
    }
    if root > max {
        root = max;
    }
    if max == 0 {
        let Some(end) = (*table_cursor).checked_add(2) else {
            return 1;
        };
        if end > table.len() {
            return 1;
        }
        let here = crate::src::inftrees::code {
            op: 64,
            bits: 1,
            val: 0,
        };
        let Some(entries) = table.get_mut(*table_cursor..end) else {
            return 1;
        };
        entries.copy_from_slice(&[here; 2]);
        *table_cursor = end;
        *bits = 1;
        return 0;
    }

    let mut min = 1u32;
    while min < max && count[min as usize] == 0 {
        min += 1;
    }
    if root < min {
        root = min;
    }

    let mut left = 1i32;
    for length in 1..=MAXBITS as usize {
        left <<= 1;
        left -= count[length] as i32;
        if left < 0 {
            return -1;
        }
    }
    if left > 0 && (type_0 == CODES || max != 1) {
        return -1;
    }

    let mut offs = [0u16; MAXBITS as usize + 1];
    for length in 1..MAXBITS as usize {
        offs[length + 1] = offs[length].wrapping_add(count[length]);
    }
    for (symbol, &length) in lens.iter().enumerate() {
        if length != 0 {
            let offset = offs[length as usize] as usize;
            let Some(entry) = work.get_mut(offset) else {
                return 1;
            };
            *entry = symbol as u16;
            offs[length as usize] = offs[length as usize].wrapping_add(1);
        }
    }

    let table_start = *table_cursor;
    let mut huff = 0u32;
    let mut symbol = 0usize;
    let mut length = min;
    let mut next = 0usize;
    let mut curr = root;
    let mut drop_bits = 0u32;
    let mut low = u32::MAX;
    let mut used = 1u32 << root;
    let mask = used - 1;
    if (type_0 == LENS && used > ENOUGH_LENS as u32)
        || (type_0 == DISTS && used > ENOUGH_DISTS as u32)
        || table_start
            .checked_add(used as usize)
            .map_or(true, |end| end > table.len())
    {
        return 1;
    }

    loop {
        let Some(&work_code) = work.get(symbol) else {
            return 1;
        };
        let Some(here) = table_entry_for_symbol(type_0, work_code, (length - drop_bits) as u8)
        else {
            return -1;
        };

        let increment = 1u32 << (length - drop_bits);
        let mut fill = 1u32 << curr;
        let next_table_size = fill;
        loop {
            fill -= increment;
            let index = table_start + next + ((huff >> drop_bits) + fill) as usize;
            let Some(entry) = table.get_mut(index) else {
                return 1;
            };
            *entry = here;
            if fill == 0 {
                break;
            }
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

        symbol += 1;
        count[length as usize] = count[length as usize].wrapping_sub(1);
        if count[length as usize] == 0 {
            if length == max {
                break;
            }
            let Some(&next_symbol) = work.get(symbol) else {
                return 1;
            };
            let Some(&next_length) = lens.get(next_symbol as usize) else {
                return -1;
            };
            length = next_length as u32;
        }

        if length > root && huff & mask != low {
            if drop_bits == 0 {
                drop_bits = root;
            }
            next += next_table_size as usize;
            curr = length - drop_bits;
            left = 1i32 << curr;
            while curr + drop_bits < max {
                left -= count[(curr + drop_bits) as usize] as i32;
                if left <= 0 {
                    break;
                }
                curr += 1;
                left <<= 1;
            }
            used += 1u32 << curr;
            if (type_0 == LENS && used > ENOUGH_LENS as u32)
                || (type_0 == DISTS && used > ENOUGH_DISTS as u32)
                || table_start
                    .checked_add(used as usize)
                    .map_or(true, |end| end > table.len())
            {
                return 1;
            }
            low = huff & mask;
            let Some(entry) = table.get_mut(table_start + low as usize) else {
                return 1;
            };
            entry.op = curr as u8;
            entry.bits = root as u8;
            entry.val = next as u16;
        }
    }

    if huff != 0 {
        let Some(entry) = table.get_mut(table_start + next + huff as usize) else {
            return 1;
        };
        *entry = crate::src::inftrees::code {
            op: 64,
            bits: (length - drop_bits) as u8,
            val: 0,
        };
    }
    *table_cursor = table_start + used as usize;
    *bits = root;
    0
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
    if table_out.is_null() || bits.is_null() || (codes != 0 && (lens.is_null() || work.is_null())) {
        return -1;
    }

    let table_capacity = match type_0 {
        CODES => 128,
        LENS => ENOUGH_LENS as usize,
        DISTS => ENOUGH_DISTS as usize,
        _ => return -1,
    };
    let table_start = *table_out;
    if table_start.is_null() {
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
        *table_out = table_start.add(table_cursor);
    }
    result
}

pub use inflate_table_ffi as inflate_table;
pub(crate) fn inflate_fixed(state: &mut crate::src::inflate::inflate_state) {
    state.lencode = lenfix.as_ptr();
    state.lenbits = 9;
    state.distcode = distfix.as_ptr();
    state.distbits = 5;
}
#[export_name = "inflate_fixed"]

pub unsafe extern "C" fn inflate_fixed_ffi(mut state: *mut crate::src::inflate::inflate_state) {
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
    fn table_entry_for_symbol_classifies_code_entries() {
        assert_table_entry(table_entry_for_symbol(CODES, 18, 7), 0, 7, 18);
        assert_table_entry(table_entry_for_symbol(CODES, 19, 7), 96, 7, 0);
        assert!(table_entry_for_symbol(CODES, 20, 7).is_none());
    }

    #[test]
    fn table_entry_for_symbol_classifies_length_and_distance_entries() {
        assert_table_entry(
            table_entry_for_symbol(LENS, 257, 4),
            LEXT[0] as u8,
            4,
            LBASE[0],
        );
        assert!(table_entry_for_symbol(LENS, 288, 4).is_none());
        assert_table_entry(
            table_entry_for_symbol(DISTS, 29, 3),
            DEXT[29] as u8,
            3,
            DBASE[29],
        );
        assert!(table_entry_for_symbol(DISTS, 32, 3).is_none());
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
