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

/// Build the table entry for one canonical symbol using immutable lookup
/// tables.  Keeping this decision separate from the raw table cursor lets the
/// table builder treat an out-of-range base/extra lookup as zlib's existing
/// invalid-code marker instead of ever indexing an unchecked table.
fn inflate_table_symbol(
    symbol: ::core::ffi::c_ushort,
    match_symbol: ::core::ffi::c_uint,
    base: Option<&[::core::ffi::c_ushort]>,
    extra: Option<&[::core::ffi::c_ushort]>,
) -> crate::src::inftrees::code {
    let symbol = symbol as ::core::ffi::c_uint;
    if symbol.wrapping_add(1) < match_symbol {
        crate::src::inftrees::code {
            op: 0,
            bits: 0,
            val: symbol as ::core::ffi::c_ushort,
        }
    } else if symbol >= match_symbol {
        let index = symbol.wrapping_sub(match_symbol) as usize;
        if let Some((&base, &extra)) =
            base.and_then(|base| extra.and_then(|extra| base.get(index).zip(extra.get(index))))
        {
            crate::src::inftrees::code {
                op: extra as ::core::ffi::c_uchar,
                bits: 0,
                val: base,
            }
        } else {
            crate::src::inftrees::code {
                op: 64,
                bits: 0,
                val: 0,
            }
        }
    } else {
        crate::src::inftrees::code {
            op: 32 + 64,
            bits: 0,
            val: 0,
        }
    }
}

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
struct InflateTableBuild {
    entries: Vec<crate::src::inftrees::code>,
    work: Vec<::core::ffi::c_ushort>,
    root: ::core::ffi::c_uint,
}

/// Construct a Huffman decode table using only validated Rust slices and
/// indices. The FFI adapter below is deliberately limited to copying the
/// legacy inputs and committing this completed table to the caller's storage.
fn inflate_table_build(
    type_0: crate::src::inftrees::codetype,
    lens: &[::core::ffi::c_ushort],
    mut root: ::core::ffi::c_uint,
) -> Result<InflateTableBuild, ::core::ffi::c_int> {
    let mut len: ::core::ffi::c_uint = 0;
    let mut sym: ::core::ffi::c_uint = 0;
    let mut min: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = 0;
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
    let mut next_index: ::core::ffi::c_uint = 0;
    // The base and extra tables are immutable local data.  Keep them as
    // slices so table selection and lookup do not manufacture raw pointers.
    let mut base: Option<&[::core::ffi::c_ushort]> = None;
    let mut extra: Option<&[::core::ffi::c_ushort]> = None;
    let mut match_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut count: [::core::ffi::c_ushort; 16] = [0; 16];
    let mut offs: [::core::ffi::c_ushort; 16] = [0; 16];
    static lbase: [::core::ffi::c_ushort; 31] = [
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
    static lext: [::core::ffi::c_ushort; 31] = [
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
    static dbase: [::core::ffi::c_ushort; 32] = [
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
    static dext: [::core::ffi::c_ushort; 32] = [
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
    for &length in lens {
        let length = length as usize;
        if length > MAXBITS as usize {
            return Err(-1);
        }
        count[length] = count[length].wrapping_add(1);
    }
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
        return Ok(InflateTableBuild {
            entries: vec![here; 2],
            work: vec![0; lens.len()],
            root: 1,
        });
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
            return Err(-1);
        }
        len = len.wrapping_add(1);
    }
    if left > 0 as ::core::ffi::c_int
        && (type_0 as ::core::ffi::c_uint
            == crate::src::inftrees::CODES as ::core::ffi::c_int as ::core::ffi::c_uint
            || max != 1 as ::core::ffi::c_uint)
    {
        return Err(-1);
    }
    offs[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_ushort;
    len = 1 as ::core::ffi::c_uint;
    while len < MAXBITS as ::core::ffi::c_uint {
        offs[len.wrapping_add(1 as ::core::ffi::c_uint) as usize] =
            (offs[len as usize] as ::core::ffi::c_int + count[len as usize] as ::core::ffi::c_int)
                as ::core::ffi::c_ushort;
        len = len.wrapping_add(1);
    }
    let mut work = vec![0; lens.len()];
    for (symbol, &length) in lens.iter().enumerate() {
        if length != 0 {
            let offset = offs[length as usize] as usize;
            let Some(slot) = work.get_mut(offset) else {
                return Err(-1);
            };
            *slot = symbol as ::core::ffi::c_ushort;
            offs[length as usize] = offs[length as usize].wrapping_add(1);
        }
    }
    match type_0 as ::core::ffi::c_uint {
        0 => {
            match_0 = 20 as ::core::ffi::c_uint;
        }
        1 => {
            base = Some(&lbase);
            extra = Some(&lext);
            match_0 = 257 as ::core::ffi::c_uint;
        }
        2 => {
            base = Some(&dbase);
            extra = Some(&dext);
        }
        _ => {}
    }
    huff = 0 as ::core::ffi::c_uint;
    sym = 0 as ::core::ffi::c_uint;
    len = min;
    next_index = 0;
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
        return Err(1);
    }
    let mut entries = vec![
        crate::src::inftrees::code {
            op: 0,
            bits: 0,
            val: 0,
        };
        crate::src::inftrees::ENOUGH as usize
    ];
    loop {
        here.bits = len.wrapping_sub(drop_0) as ::core::ffi::c_uchar;
        let Some(&symbol) = work.get(sym as usize) else {
            return Err(-1);
        };
        let entry = inflate_table_symbol(symbol, match_0, base, extra);
        here.op = entry.op;
        here.val = entry.val;
        incr = (1 as ::core::ffi::c_uint) << len.wrapping_sub(drop_0);
        fill = (1 as ::core::ffi::c_uint) << curr;
        min = fill;
        loop {
            fill = fill.wrapping_sub(incr);
            let index = next_index.wrapping_add((huff >> drop_0).wrapping_add(fill)) as usize;
            let Some(slot) = entries.get_mut(index) else {
                return Err(1);
            };
            *slot = here;
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
            let Some(&symbol) = work.get(sym as usize) else {
                return Err(-1);
            };
            let Some(&next_len) = lens.get(symbol as usize) else {
                return Err(-1);
            };
            len = next_len as ::core::ffi::c_uint;
        }
        if len > root && huff & mask != low {
            if drop_0 == 0 as ::core::ffi::c_uint {
                drop_0 = root;
            }
            next_index = next_index.wrapping_add(min);
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
                return Err(1);
            }
            low = huff & mask;
            let Some(slot) = entries.get_mut(low as usize) else {
                return Err(1);
            };
            slot.op = curr as ::core::ffi::c_uchar;
            slot.bits = root as ::core::ffi::c_uchar;
            slot.val = next_index as ::core::ffi::c_ushort;
        }
    }
    if huff != 0 as ::core::ffi::c_uint {
        here.op = 64 as ::core::ffi::c_int as ::core::ffi::c_uchar;
        here.bits = len.wrapping_sub(drop_0) as ::core::ffi::c_uchar;
        here.val = 0 as ::core::ffi::c_int as ::core::ffi::c_ushort;
        let index = next_index.wrapping_add(huff) as usize;
        let Some(slot) = entries.get_mut(index) else {
            return Err(1);
        };
        *slot = here;
    }
    entries.truncate(used as usize);
    Ok(InflateTableBuild {
        entries,
        work,
        root,
    })
}

/// Legacy raw cursor adapter. All table construction occurs in
/// `inflate_table_build`; this layer only copies bounded C arrays in and
/// commits the resulting entries and cursors back out.
pub unsafe extern "C" fn inflate_table(
    type_0: crate::src::inftrees::codetype,
    lens: *mut ::core::ffi::c_ushort,
    codes: ::core::ffi::c_uint,
    table: *mut *mut crate::src::inftrees::code,
    bits: *mut ::core::ffi::c_uint,
    work: *mut ::core::ffi::c_ushort,
    table_capacity: usize,
    work_capacity: usize,
) -> ::core::ffi::c_int {
    if lens.is_null() || table.is_null() || bits.is_null() || work.is_null() || codes as usize > 320
    {
        return 1;
    }
    // These ABI cursors are only borrowed for this adapter call.  Keep the
    // raw access at the edge and let the builder below operate entirely on
    // owned/slice data.
    let table = &mut *table;
    let bits = &mut *bits;
    let mut lens_copy = [0; 320];
    for index in 0..codes as usize {
        lens_copy[index] = *lens.wrapping_add(index);
    }
    let root = *bits;
    let build = match inflate_table_build(type_0, &lens_copy[..codes as usize], root) {
        Ok(build) => build,
        Err(status) => return status,
    };
    let table_start = *table;
    if table_start.is_null()
        || build.entries.len() > table_capacity
        || build.work.len() > work_capacity
    {
        return 1;
    }
    for (index, symbol) in build.work.iter().enumerate() {
        *work.wrapping_add(index) = *symbol;
    }
    for (index, entry) in build.entries.iter().enumerate() {
        *table_start.wrapping_add(index) = *entry;
    }
    *table = table_start.wrapping_add(build.entries.len());
    *bits = build.root;
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
    // The C ABI does not carry output capacities.  Internal callers use the
    // capacity-aware adapter above; retain this legacy entry point's contract
    // for external callers that provide the documented ENOUGH-sized storage.
    inflate_table(
        type_0,
        lens,
        codes,
        table,
        bits,
        work,
        usize::MAX,
        usize::MAX,
    )
}
pub(crate) fn inflate_fixed_state(state: &mut crate::src::inflate::inflate_state) {
    state.lencode = &raw const lenfix as *const crate::src::inftrees::code;
    state.lenbits = 9 as ::core::ffi::c_uint;
    state.distcode = &raw const distfix as *const crate::src::inftrees::code;
    state.distbits = 5 as ::core::ffi::c_uint;
}

#[export_name = "inflate_fixed"]

pub unsafe extern "C" fn inflate_fixed_ffi(mut state: *mut crate::src::inflate::inflate_state) {
    inflate_fixed_state(&mut *state)
}
