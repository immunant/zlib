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
pub static inflate_copyright: [::core::ffi::c_char; 49] = [
    b' ' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'n' as ::core::ffi::c_char,
    b'f' as ::core::ffi::c_char,
    b'l' as ::core::ffi::c_char,
    b'a' as ::core::ffi::c_char,
    b't' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'1' as ::core::ffi::c_char,
    b'.' as ::core::ffi::c_char,
    b'3' as ::core::ffi::c_char,
    b'.' as ::core::ffi::c_char,
    b'2' as ::core::ffi::c_char,
    b'.' as ::core::ffi::c_char,
    b'1' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'C' as ::core::ffi::c_char,
    b'o' as ::core::ffi::c_char,
    b'p' as ::core::ffi::c_char,
    b'y' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'i' as ::core::ffi::c_char,
    b'g' as ::core::ffi::c_char,
    b'h' as ::core::ffi::c_char,
    b't' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'1' as ::core::ffi::c_char,
    b'9' as ::core::ffi::c_char,
    b'9' as ::core::ffi::c_char,
    b'5' as ::core::ffi::c_char,
    b'-' as ::core::ffi::c_char,
    b'2' as ::core::ffi::c_char,
    b'0' as ::core::ffi::c_char,
    b'2' as ::core::ffi::c_char,
    b'6' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'M' as ::core::ffi::c_char,
    b'a' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b'k' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    b'A' as ::core::ffi::c_char,
    b'd' as ::core::ffi::c_char,
    b'l' as ::core::ffi::c_char,
    b'e' as ::core::ffi::c_char,
    b'r' as ::core::ffi::c_char,
    b' ' as ::core::ffi::c_char,
    0,
];
pub unsafe extern "C" fn inflate_table(
    mut type_0: crate::src::inftrees::codetype,
    mut lens: *mut ::core::ffi::c_ushort,
    mut codes: ::core::ffi::c_uint,
    mut table: *mut *mut crate::src::inftrees::code,
    mut bits: *mut ::core::ffi::c_uint,
    mut work: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_uint = 0;
    let mut sym: ::core::ffi::c_uint = 0;
    let mut min: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = 0;
    let mut root: ::core::ffi::c_uint = 0;
    let mut curr: ::core::ffi::c_uint = 0;
    let mut drop_0: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_int = 0;
    let mut used: ::core::ffi::c_uint = 0;
    let mut huff: ::core::ffi::c_uint = 0;
    let mut incr: ::core::ffi::c_uint = 0;
    let mut fill: ::core::ffi::c_uint = 0;
    let mut low: ::core::ffi::c_uint = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut here: crate::src::inftrees::code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut next: *mut crate::src::inftrees::code =
        ::core::ptr::null_mut::<crate::src::inftrees::code>();
    let mut base: &[::core::ffi::c_ushort] = &[];
    let mut extra: &[::core::ffi::c_ushort] = &[];
    let mut match_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut count: [::core::ffi::c_ushort; 16] = [0; 16];
    let mut offs: [::core::ffi::c_ushort; 16] = [0; 16];
    static LBASE: [::core::ffi::c_ushort; 31] = [
        3 as ::core::ffi::c_ushort,
        4 as ::core::ffi::c_ushort,
        5 as ::core::ffi::c_ushort,
        6 as ::core::ffi::c_ushort,
        7 as ::core::ffi::c_ushort,
        8 as ::core::ffi::c_ushort,
        9 as ::core::ffi::c_ushort,
        10 as ::core::ffi::c_ushort,
        11 as ::core::ffi::c_ushort,
        13 as ::core::ffi::c_ushort,
        15 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_ushort,
        23 as ::core::ffi::c_ushort,
        27 as ::core::ffi::c_ushort,
        31 as ::core::ffi::c_ushort,
        35 as ::core::ffi::c_ushort,
        43 as ::core::ffi::c_ushort,
        51 as ::core::ffi::c_ushort,
        59 as ::core::ffi::c_ushort,
        67 as ::core::ffi::c_ushort,
        83 as ::core::ffi::c_ushort,
        99 as ::core::ffi::c_ushort,
        115 as ::core::ffi::c_ushort,
        131 as ::core::ffi::c_ushort,
        163 as ::core::ffi::c_ushort,
        195 as ::core::ffi::c_ushort,
        227 as ::core::ffi::c_ushort,
        258 as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_ushort,
    ];
    static LEXT: [::core::ffi::c_ushort; 31] = [
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        68 as ::core::ffi::c_ushort,
        193 as ::core::ffi::c_ushort,
    ];
    static DBASE: [::core::ffi::c_ushort; 32] = [
        1 as ::core::ffi::c_ushort,
        2 as ::core::ffi::c_ushort,
        3 as ::core::ffi::c_ushort,
        4 as ::core::ffi::c_ushort,
        5 as ::core::ffi::c_ushort,
        7 as ::core::ffi::c_ushort,
        9 as ::core::ffi::c_ushort,
        13 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        25 as ::core::ffi::c_ushort,
        33 as ::core::ffi::c_ushort,
        49 as ::core::ffi::c_ushort,
        65 as ::core::ffi::c_ushort,
        97 as ::core::ffi::c_ushort,
        129 as ::core::ffi::c_ushort,
        193 as ::core::ffi::c_ushort,
        257 as ::core::ffi::c_ushort,
        385 as ::core::ffi::c_ushort,
        513 as ::core::ffi::c_ushort,
        769 as ::core::ffi::c_ushort,
        1025 as ::core::ffi::c_ushort,
        1537 as ::core::ffi::c_ushort,
        2049 as ::core::ffi::c_ushort,
        3073 as ::core::ffi::c_ushort,
        4097 as ::core::ffi::c_ushort,
        6145 as ::core::ffi::c_ushort,
        8193 as ::core::ffi::c_ushort,
        12289 as ::core::ffi::c_ushort,
        16385 as ::core::ffi::c_ushort,
        24577 as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_ushort,
    ];
    static DEXT: [::core::ffi::c_ushort; 32] = [
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        16 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_ushort,
        19 as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_ushort,
        20 as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_ushort,
        21 as ::core::ffi::c_ushort,
        22 as ::core::ffi::c_ushort,
        22 as ::core::ffi::c_ushort,
        23 as ::core::ffi::c_ushort,
        23 as ::core::ffi::c_ushort,
        24 as ::core::ffi::c_ushort,
        24 as ::core::ffi::c_ushort,
        25 as ::core::ffi::c_ushort,
        25 as ::core::ffi::c_ushort,
        26 as ::core::ffi::c_ushort,
        26 as ::core::ffi::c_ushort,
        27 as ::core::ffi::c_ushort,
        27 as ::core::ffi::c_ushort,
        28 as ::core::ffi::c_ushort,
        28 as ::core::ffi::c_ushort,
        29 as ::core::ffi::c_ushort,
        29 as ::core::ffi::c_ushort,
        64 as ::core::ffi::c_ushort,
        64 as ::core::ffi::c_ushort,
    ];
    len = 0 as ::core::ffi::c_uint;
    while len <= MAXBITS as ::core::ffi::c_uint {
        count[len as usize] = 0 as ::core::ffi::c_ushort;
        len = len.wrapping_add(1);
    }
    sym = 0 as ::core::ffi::c_uint;
    while sym < codes {
        count[*lens.offset(sym as isize) as usize] =
            count[*lens.offset(sym as isize) as usize].wrapping_add(1);
        sym = sym.wrapping_add(1);
    }
    root = *bits;
    max = MAXBITS as ::core::ffi::c_uint;
    while max >= 1 as ::core::ffi::c_uint {
        if count[max as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            break;
        }
        max = max.wrapping_sub(1);
    }
    if root > max {
        root = max;
    }
    if max == 0 as ::core::ffi::c_uint {
        here.op = 64 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        here.bits = 1 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        here.val = 0 as ::core::ffi::c_int as ::core::ffi::c_ushort;
        let c2rust_fresh0 = *table;
        *table = (*table).offset(1);
        *c2rust_fresh0 = here;
        let c2rust_fresh1 = *table;
        *table = (*table).offset(1);
        *c2rust_fresh1 = here;
        *bits = 1 as ::core::ffi::c_uint;
        return 0 as ::core::ffi::c_int;
    }
    min = 1 as ::core::ffi::c_uint;
    while min < max {
        if count[min as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            break;
        }
        min = min.wrapping_add(1);
    }
    if root < min {
        root = min;
    }
    left = 1 as ::core::ffi::c_int;
    len = 1 as ::core::ffi::c_uint;
    while len <= MAXBITS as ::core::ffi::c_uint {
        left <<= 1 as ::core::ffi::c_int;
        left -= count[len as usize] as ::core::ffi::c_int;
        if left < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        len = len.wrapping_add(1);
    }
    if left > 0 as ::core::ffi::c_int
        && (type_0 as ::core::ffi::c_uint
            == crate::src::inftrees::CODES as ::core::ffi::c_int as ::core::ffi::c_uint
            || max != 1 as ::core::ffi::c_uint)
    {
        return -1 as ::core::ffi::c_int;
    }
    offs[1 as usize] = 0 as ::core::ffi::c_ushort;
    len = 1 as ::core::ffi::c_uint;
    while len < MAXBITS as ::core::ffi::c_uint {
        offs[len.wrapping_add(1 as ::core::ffi::c_uint) as usize] =
            (offs[len as usize] as ::core::ffi::c_int + count[len as usize] as ::core::ffi::c_int)
                as ::core::ffi::c_ushort;
        len = len.wrapping_add(1);
    }
    sym = 0 as ::core::ffi::c_uint;
    while sym < codes {
        if *lens.offset(sym as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            let c2rust_fresh2 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize) as usize] =
                offs[*lens.offset(sym as isize) as usize].wrapping_add(1);
            *work.offset(c2rust_fresh2 as isize) = sym as ::core::ffi::c_ushort;
        }
        sym = sym.wrapping_add(1);
    }
    match type_0 as ::core::ffi::c_uint {
        0 => {
            match_0 = 20 as ::core::ffi::c_uint;
        }
        1 => {
            base = &LBASE;
            extra = &LEXT;
            match_0 = 257 as ::core::ffi::c_uint;
        }
        2 => {
            base = &DBASE;
            extra = &DEXT;
        }
        _ => {}
    }
    huff = 0 as ::core::ffi::c_uint;
    sym = 0 as ::core::ffi::c_uint;
    len = min;
    next = *table;
    curr = root;
    drop_0 = 0 as ::core::ffi::c_uint;
    low = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    used = (1 as ::core::ffi::c_uint) << root;
    mask = used.wrapping_sub(1 as ::core::ffi::c_uint);
    if type_0 as ::core::ffi::c_uint
        == crate::src::inftrees::LENS as ::core::ffi::c_int as ::core::ffi::c_uint
        && used > crate::src::inftrees::ENOUGH_LENS as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint
            == crate::src::inftrees::DISTS as ::core::ffi::c_int as ::core::ffi::c_uint
            && used > crate::src::inftrees::ENOUGH_DISTS as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        here.bits = len.wrapping_sub(drop_0) as ::core::ffi::c_uchar;
        if (*work.offset(sym as isize) as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
            < match_0
        {
            here.op = 0 as ::core::ffi::c_int as ::core::ffi::c_uchar;
            here.val = *work.offset(sym as isize);
        } else if *work.offset(sym as isize) as ::core::ffi::c_uint >= match_0 {
            let index =
                (*work.offset(sym as isize) as ::core::ffi::c_uint).wrapping_sub(match_0) as usize;
            here.op = extra[index] as ::core::ffi::c_uchar;
            here.val = base[index];
        } else {
            here.op = (32 as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            here.val = 0 as ::core::ffi::c_ushort;
        }
        incr = (1 as ::core::ffi::c_uint) << len.wrapping_sub(drop_0);
        fill = (1 as ::core::ffi::c_uint) << curr;
        min = fill;
        loop {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = here;
            if fill == 0 as ::core::ffi::c_uint {
                break;
            }
        }
        incr = (1 as ::core::ffi::c_uint) << len.wrapping_sub(1 as ::core::ffi::c_uint);
        while huff & incr != 0 {
            incr >>= 1 as ::core::ffi::c_int;
        }
        if incr != 0 as ::core::ffi::c_uint {
            huff &= incr.wrapping_sub(1 as ::core::ffi::c_uint);
            huff = huff.wrapping_add(incr);
        } else {
            huff = 0 as ::core::ffi::c_uint;
        }
        sym = sym.wrapping_add(1);
        count[len as usize] = count[len as usize].wrapping_sub(1);
        if count[len as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if len == max {
                break;
            }
            len = *lens.offset(*work.offset(sym as isize) as isize) as ::core::ffi::c_uint;
        }
        if len > root && huff & mask != low {
            if drop_0 == 0 as ::core::ffi::c_uint {
                drop_0 = root;
            }
            next = next.offset(min as isize);
            curr = len.wrapping_sub(drop_0);
            left = (1 as ::core::ffi::c_int) << curr;
            while curr.wrapping_add(drop_0) < max {
                left -= count[curr.wrapping_add(drop_0) as usize] as ::core::ffi::c_int;
                if left <= 0 as ::core::ffi::c_int {
                    break;
                }
                curr = curr.wrapping_add(1);
                left <<= 1 as ::core::ffi::c_int;
            }
            used = used.wrapping_add((1 as ::core::ffi::c_uint) << curr);
            if type_0 as ::core::ffi::c_uint
                == crate::src::inftrees::LENS as ::core::ffi::c_int as ::core::ffi::c_uint
                && used > crate::src::inftrees::ENOUGH_LENS as ::core::ffi::c_uint
                || type_0 as ::core::ffi::c_uint
                    == crate::src::inftrees::DISTS as ::core::ffi::c_int as ::core::ffi::c_uint
                    && used > crate::src::inftrees::ENOUGH_DISTS as ::core::ffi::c_uint
            {
                return 1 as ::core::ffi::c_int;
            }
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as ::core::ffi::c_uchar;
            (*(*table).offset(low as isize)).bits = root as ::core::ffi::c_uchar;
            (*(*table).offset(low as isize)).val =
                (next.addr().wrapping_sub((*table).addr())
                    / ::core::mem::size_of::<crate::src::inftrees::code>())
                    as ::core::ffi::c_ushort;
        }
    }
    if huff != 0 as ::core::ffi::c_uint {
        here.op = 64 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        here.bits = len.wrapping_sub(drop_0) as ::core::ffi::c_uchar;
        here.val = 0 as ::core::ffi::c_int as ::core::ffi::c_ushort;
        *next.offset(huff as isize) = here;
    }
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0 as ::core::ffi::c_int;
}

/// Build a Huffman decode table in a bounded state-owned arena.
///
/// The exported `inflate_table` ABI keeps its pointer cursor for C callers.
/// Rust decoders own their lens, work, and code arenas, so they can use this
/// checked cursor form instead.  `next` is advanced exactly as the ABI table
/// pointer would be.
pub fn inflate_table_slice(
    type_0: crate::src::inftrees::codetype,
    lens: &[::core::ffi::c_ushort],
    codes: usize,
    table: &mut [crate::src::inftrees::code],
    next: &mut usize,
    bits: &mut ::core::ffi::c_uint,
    work: &mut [::core::ffi::c_ushort],
) -> ::core::ffi::c_int {
    const LBASE: [::core::ffi::c_ushort; 31] = [
        3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51,
        59, 67, 83, 99, 115, 131, 163, 195, 227, 258, 0, 0,
    ];
    const LEXT: [::core::ffi::c_ushort; 31] = [
        16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18, 19,
        19, 19, 19, 20, 20, 20, 20, 21, 21, 21, 21, 16, 68, 193,
    ];
    const DBASE: [::core::ffi::c_ushort; 32] = [
        1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385,
        513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385,
        24577, 0, 0,
    ];
    const DEXT: [::core::ffi::c_ushort; 32] = [
        16, 16, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23,
        23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 29, 29, 64, 64,
    ];

    if codes > lens.len() || codes > work.len() || *next > table.len() || *bits > MAXBITS as u32 {
        return 1;
    }

    let mut count = [0u16; 16];
    for &length in &lens[..codes] {
        let length = length as usize;
        if length > MAXBITS as usize {
            return -1;
        }
        count[length] = count[length].wrapping_add(1);
    }

    let mut root = *bits;
    let mut max = MAXBITS as u32;
    while max != 0 && count[max as usize] == 0 {
        max -= 1;
    }
    root = root.min(max);
    if max == 0 {
        let Some(end) = next.checked_add(2) else {
            return 1;
        };
        let Some(entries) = table.get_mut(*next..end) else {
            return 1;
        };
        let here = crate::src::inftrees::code { op: 64, bits: 1, val: 0 };
        entries[0] = here;
        entries[1] = here;
        *next = end;
        *bits = 1;
        return 0;
    }

    let mut min = 1u32;
    while min < max && count[min as usize] == 0 {
        min += 1;
    }
    root = root.max(min);

    let mut left = 1i32;
    for length in 1..=MAXBITS as usize {
        left = (left << 1) - count[length] as i32;
        if left < 0 {
            return -1;
        }
    }
    if left > 0 && (type_0 == CODES || max != 1) {
        return -1;
    }

    let mut offs = [0u16; 16];
    for length in 1..MAXBITS as usize {
        offs[length + 1] = offs[length].wrapping_add(count[length]);
    }
    for (symbol, &length) in lens[..codes].iter().enumerate() {
        if length != 0 {
            let slot = length as usize;
            let offset = offs[slot] as usize;
            let Some(destination) = work.get_mut(offset) else {
                return 1;
            };
            *destination = symbol as u16;
            offs[slot] = offs[slot].wrapping_add(1);
        }
    }

    let (base, extra, match_symbol): (&[u16], &[u16], u32) = match type_0 {
        CODES => (&[], &[], 20),
        LENS => (&LBASE, &LEXT, 257),
        DISTS => (&DBASE, &DEXT, 0),
        _ => (&[], &[], 0),
    };

    let start = *next;
    let mut table_next = start;
    let mut used = 1u32 << root;
    let capacity = match type_0 {
        LENS if used > ENOUGH_LENS as u32 => return 1,
        DISTS if used > ENOUGH_DISTS as u32 => return 1,
        _ => used,
    };
    if start.checked_add(capacity as usize).is_none_or(|end| end > table.len()) {
        return 1;
    }
    let mask = used - 1;
    let mut huff = 0u32;
    let mut symbol = 0usize;
    let mut length = min;
    let mut curr = root;
    let mut drop = 0u32;
    let mut low = u32::MAX;

    loop {
        let work_symbol = match work.get(symbol) {
            Some(symbol) => *symbol as usize,
            None => return 1,
        };
        let mut here = crate::src::inftrees::code {
            op: 0,
            bits: length.wrapping_sub(drop) as u8,
            val: 0,
        };
        if (work_symbol as u32).wrapping_add(1) < match_symbol {
            here.val = work_symbol as u16;
        } else if work_symbol as u32 >= match_symbol {
            let index = work_symbol.wrapping_sub(match_symbol as usize);
            let (Some(&op), Some(&val)) = (extra.get(index), base.get(index)) else {
                return 1;
            };
            here.op = op as u8;
            here.val = val;
        } else {
            here.op = 96;
        }

        let increment = 1u32 << length.wrapping_sub(drop);
        let mut fill = 1u32 << curr;
        let minimum = fill;
        loop {
            fill -= increment;
            let index = table_next + ((huff >> drop).wrapping_add(fill) as usize);
            let Some(entry) = table.get_mut(index) else {
                return 1;
            };
            *entry = here;
            if fill == 0 {
                break;
            }
        }

        let mut increment = 1u32 << length.wrapping_sub(1);
        while huff & increment != 0 {
            increment >>= 1;
        }
        if increment != 0 {
            huff &= increment - 1;
            huff = huff.wrapping_add(increment);
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
                return 1;
            };
            length = next_length as u32;
        }

        if length > root && huff & mask != low {
            if drop == 0 {
                drop = root;
            }
            table_next = match table_next.checked_add(minimum as usize) {
                Some(next) => next,
                None => return 1,
            };
            curr = length - drop;
            left = 1i32 << curr;
            while curr + drop < max {
                left -= count[(curr + drop) as usize] as i32;
                if left <= 0 {
                    break;
                }
                curr += 1;
                left <<= 1;
            }
            used = used.wrapping_add(1u32 << curr);
            if (type_0 == LENS && used > ENOUGH_LENS as u32)
                || (type_0 == DISTS && used > ENOUGH_DISTS as u32)
            {
                return 1;
            }
            if start.checked_add(used as usize).is_none_or(|end| end > table.len()) {
                return 1;
            }
            low = huff & mask;
            let Some(entry) = table.get_mut(start + low as usize) else {
                return 1;
            };
            entry.op = curr as u8;
            entry.bits = root as u8;
            entry.val = (table_next - start) as u16;
        }
    }

    if huff != 0 {
        let Some(entry) = table.get_mut(table_next + huff as usize) else {
            return 1;
        };
        *entry = crate::src::inftrees::code {
            op: 64,
            bits: length.wrapping_sub(drop) as u8,
            val: 0,
        };
    }
    let Some(end) = start.checked_add(used as usize) else {
        return 1;
    };
    if end > table.len() {
        return 1;
    }
    *next = end;
    *bits = root;
    0
}
#[export_name = "inflate_table"]

pub unsafe extern "C" fn inflate_table_ffi(
    mut type_0: crate::src::inftrees::codetype,
    mut lens: *mut ::core::ffi::c_ushort,
    mut codes: ::core::ffi::c_uint,
    mut table: *mut *mut crate::src::inftrees::code,
    mut bits: *mut ::core::ffi::c_uint,
    mut work: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    inflate_table(type_0, lens, codes, table, bits, work)
}
pub fn inflate_fixed(
    lencode: &mut crate::src::inflate::length_table,
    lenbits: &mut ::core::ffi::c_uint,
    distcode: &mut crate::src::inflate::distance_table,
    distbits: &mut ::core::ffi::c_uint,
) {
    *lencode = crate::src::inflate::length_table::Fixed;
    *lenbits = 9 as ::core::ffi::c_uint;
    *distcode = crate::src::inflate::distance_table::Fixed;
    *distbits = 5 as ::core::ffi::c_uint;
}
#[export_name = "inflate_fixed"]

pub unsafe extern "C" fn inflate_fixed_ffi(mut state: *mut crate::src::inflate::inflate_state) {
    let Some(state) = state.as_mut() else {
        return;
    };
    inflate_fixed(
        &mut state.lencode,
        &mut state.lenbits,
        &mut state.distcode,
        &mut state.distbits,
    )
}
