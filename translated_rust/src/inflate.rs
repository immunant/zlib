// =============== BEGIN inflate_h ================
pub type inflate_mode = ::core::ffi::c_uint;

pub const HEAD: crate::src::inflate::inflate_mode = 16180;

pub const FLAGS: crate::src::inflate::inflate_mode = 16181;

pub const TIME: crate::src::inflate::inflate_mode = 16182;

pub const OS: crate::src::inflate::inflate_mode = 16183;

pub const EXLEN: crate::src::inflate::inflate_mode = 16184;

pub const EXTRA: crate::src::inflate::inflate_mode = 16185;

pub const NAME: crate::src::inflate::inflate_mode = 16186;

pub const COMMENT: crate::src::inflate::inflate_mode = 16187;

pub const HCRC: crate::src::inflate::inflate_mode = 16188;

pub const DICTID: crate::src::inflate::inflate_mode = 16189;

pub const DICT: crate::src::inflate::inflate_mode = 16190;

pub const TYPE: crate::src::inflate::inflate_mode = 16191;

pub const TYPEDO: crate::src::inflate::inflate_mode = 16192;

pub const STORED: crate::src::inflate::inflate_mode = 16193;

pub const COPY_: crate::src::inflate::inflate_mode = 16194;

pub const COPY_1: crate::src::inflate::inflate_mode = 16195;

pub const TABLE: crate::src::inflate::inflate_mode = 16196;

pub const LENLENS: crate::src::inflate::inflate_mode = 16197;

pub const CODELENS: crate::src::inflate::inflate_mode = 16198;

pub const LEN_: crate::src::inflate::inflate_mode = 16199;

pub const LEN: crate::src::inflate::inflate_mode = 16200;

pub const LENEXT: crate::src::inflate::inflate_mode = 16201;

pub const DIST: crate::src::inflate::inflate_mode = 16202;

pub const DISTEXT: crate::src::inflate::inflate_mode = 16203;

pub const MATCH: crate::src::inflate::inflate_mode = 16204;

pub const LIT: crate::src::inflate::inflate_mode = 16205;

pub const CHECK: crate::src::inflate::inflate_mode = 16206;

pub const LENGTH: crate::src::inflate::inflate_mode = 16207;

pub const DONE: crate::src::inflate::inflate_mode = 16208;

pub const BAD: crate::src::inflate::inflate_mode = 16209;

pub const MEM: crate::src::inflate::inflate_mode = 16210;

pub const SYNC: crate::src::inflate::inflate_mode = 16211;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct inflate_state {
    pub strm: crate::zlib_h::z_streamp,
    pub mode: crate::src::inflate::inflate_mode,
    pub last: ::core::ffi::c_int,
    pub wrap: ::core::ffi::c_int,
    pub havedict: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub dmax: ::core::ffi::c_uint,
    pub check: ::core::ffi::c_ulong,
    pub total: ::core::ffi::c_ulong,
    pub head: crate::zlib_h::gz_headerp,
    pub wbits: ::core::ffi::c_uint,
    pub wsize: ::core::ffi::c_uint,
    pub whave: ::core::ffi::c_uint,
    pub wnext: ::core::ffi::c_uint,
    pub window: *mut ::core::ffi::c_uchar,
    pub hold: ::core::ffi::c_ulong,
    pub bits: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub extra: ::core::ffi::c_uint,
    pub lencode: *const crate::src::inftrees::code,
    pub distcode: *const crate::src::inftrees::code,
    pub lenbits: ::core::ffi::c_uint,
    pub distbits: ::core::ffi::c_uint,
    pub ncode: ::core::ffi::c_uint,
    pub nlen: ::core::ffi::c_uint,
    pub ndist: ::core::ffi::c_uint,
    pub have: ::core::ffi::c_uint,
    pub next: *mut crate::src::inftrees::code,
    pub lens: [::core::ffi::c_ushort; 320],
    pub work: [::core::ffi::c_ushort; 288],
    pub codes: [crate::src::inftrees::code; 1444],
    pub sane: ::core::ffi::c_int,
    pub back: ::core::ffi::c_int,
    pub was: ::core::ffi::c_uint,
}
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::adler32::adler32_ffi;
pub use crate::src::crc32::crc32_ffi;
pub use crate::src::deflate::internal_state;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::inflate_fixed_ffi;
pub use crate::src::inftrees::inflate_table_ffi;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::ENOUGH;
pub use crate::src::inftrees::ENOUGH_DISTS;
pub use crate::src::inftrees::ENOUGH_LENS;
pub use crate::src::inftrees::LENS;

pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::stdlib::MAX_WBITS;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gz_header;
pub use crate::zlib_h::gz_header_s;
pub use crate::zlib_h::gz_headerp;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BLOCK;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_DEFLATED;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub use crate::zlib_h::Z_TREES;
pub use crate::zlib_h::Z_VERSION_ERROR;
pub use crate::zutil_h::DEF_WBITS;

fn inflate_mode_is_valid(mode: crate::src::inflate::inflate_mode) -> bool {
    mode >= crate::src::inflate::HEAD && mode <= crate::src::inflate::SYNC
}

pub(crate) const INFLATE_CODE_LENGTH_ORDER: [::core::ffi::c_ushort; 19] = [
    16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
];

pub(crate) fn inflate_zero_code_length_order_tail(
    lens: &mut [::core::ffi::c_ushort],
    have: &mut ::core::ffi::c_uint,
) {
    while *have < INFLATE_CODE_LENGTH_ORDER.len() as ::core::ffi::c_uint {
        let index = INFLATE_CODE_LENGTH_ORDER[*have as usize] as usize;
        lens[index] = 0 as ::core::ffi::c_ushort;
        *have = (*have).wrapping_add(1);
    }
}

pub(crate) struct InflateDynamicCounts {
    pub(crate) nlen: ::core::ffi::c_uint,
    pub(crate) ndist: ::core::ffi::c_uint,
    pub(crate) ncode: ::core::ffi::c_uint,
    pub(crate) hold: ::core::ffi::c_ulong,
    pub(crate) bits: ::core::ffi::c_uint,
}

pub(crate) fn inflate_dynamic_counts(
    mut hold: ::core::ffi::c_ulong,
    mut bits: ::core::ffi::c_uint,
) -> InflateDynamicCounts {
    let nlen = (hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_uint))
    .wrapping_add(257 as ::core::ffi::c_uint);
    hold >>= 5 as ::core::ffi::c_int;
    bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
    let ndist = (hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_uint))
    .wrapping_add(1 as ::core::ffi::c_uint);
    hold >>= 5 as ::core::ffi::c_int;
    bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
    let ncode = (hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_uint))
    .wrapping_add(4 as ::core::ffi::c_uint);
    hold >>= 4 as ::core::ffi::c_int;
    bits = bits.wrapping_sub(4 as ::core::ffi::c_int as ::core::ffi::c_uint);

    InflateDynamicCounts {
        nlen,
        ndist,
        ncode,
        hold,
        bits,
    }
}

pub(crate) fn inflate_dynamic_counts_are_valid(
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
) -> bool {
    nlen <= 286 as ::core::ffi::c_uint && ndist <= 30 as ::core::ffi::c_uint
}

pub(crate) fn inflate_stored_block_length(
    hold: ::core::ffi::c_ulong,
) -> Option<::core::ffi::c_uint> {
    if hold & 0xffff as ::core::ffi::c_ulong
        != hold >> 16 as ::core::ffi::c_int ^ 0xffff as ::core::ffi::c_ulong
    {
        None
    } else {
        Some(hold as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint)
    }
}

pub(crate) struct InflateCodeLengthRepeat {
    pub(crate) len: ::core::ffi::c_uint,
    pub(crate) copy: ::core::ffi::c_uint,
    pub(crate) hold: ::core::ffi::c_ulong,
    pub(crate) bits: ::core::ffi::c_uint,
}

pub(crate) fn inflate_code_length_repeat_extra_bits(
    repeat_code: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if repeat_code == 16 as ::core::ffi::c_uint {
        2 as ::core::ffi::c_uint
    } else if repeat_code == 17 as ::core::ffi::c_uint {
        3 as ::core::ffi::c_uint
    } else {
        7 as ::core::ffi::c_uint
    }
}

pub(crate) fn inflate_code_length_repeat(
    repeat_code: ::core::ffi::c_uint,
    previous_len: ::core::ffi::c_uint,
    mut hold: ::core::ffi::c_ulong,
    mut bits: ::core::ffi::c_uint,
) -> InflateCodeLengthRepeat {
    let extra = inflate_code_length_repeat_extra_bits(repeat_code);
    let (len, base) = if repeat_code == 16 as ::core::ffi::c_uint {
        (previous_len, 3 as ::core::ffi::c_uint)
    } else if repeat_code == 17 as ::core::ffi::c_uint {
        (0 as ::core::ffi::c_uint, 3 as ::core::ffi::c_uint)
    } else {
        (0 as ::core::ffi::c_uint, 11 as ::core::ffi::c_uint)
    };
    let copy = base.wrapping_add(
        hold as ::core::ffi::c_uint
            & ((1 as ::core::ffi::c_uint) << extra).wrapping_sub(1 as ::core::ffi::c_uint),
    );
    hold >>= extra;
    bits = bits.wrapping_sub(extra);

    InflateCodeLengthRepeat {
        len,
        copy,
        hold,
        bits,
    }
}

pub(crate) fn inflate_code_length_repeat_fits(
    have: ::core::ffi::c_uint,
    copy: ::core::ffi::c_uint,
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
) -> bool {
    have.wrapping_add(copy) <= nlen.wrapping_add(ndist)
}

pub(crate) struct InflateExtraBitsResult {
    pub(crate) value: ::core::ffi::c_uint,
    pub(crate) hold: ::core::ffi::c_ulong,
    pub(crate) bits: ::core::ffi::c_uint,
}

pub(crate) fn inflate_apply_extra_bits(
    value: ::core::ffi::c_uint,
    extra: ::core::ffi::c_uint,
    mut hold: ::core::ffi::c_ulong,
    mut bits: ::core::ffi::c_uint,
) -> InflateExtraBitsResult {
    let value = value.wrapping_add(
        hold as ::core::ffi::c_uint
            & ((1 as ::core::ffi::c_uint) << extra).wrapping_sub(1 as ::core::ffi::c_uint),
    );
    hold >>= extra;
    bits = bits.wrapping_sub(extra);

    InflateExtraBitsResult { value, hold, bits }
}

pub(crate) fn inflate_has_end_of_block_code(lens: &[::core::ffi::c_ushort]) -> bool {
    lens.get(256 as usize)
        .copied()
        .unwrap_or(0 as ::core::ffi::c_ushort)
        != 0 as ::core::ffi::c_ushort
}

enum InflateDynamicTables {
    Built {
        lens_used: usize,
        total_used: usize,
        lenbits: ::core::ffi::c_uint,
        distbits: ::core::ffi::c_uint,
    },
    InvalidLiteralLengths {
        lenbits: ::core::ffi::c_uint,
    },
    InvalidDistances {
        lens_used: usize,
        lenbits: ::core::ffi::c_uint,
        distbits: ::core::ffi::c_uint,
    },
}

fn inflate_build_dynamic_tables(
    lens: &[::core::ffi::c_ushort],
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
    codes: &mut [crate::src::inftrees::code],
    work: &mut [::core::ffi::c_ushort],
) -> InflateDynamicTables {
    let mut lenbits = 9 as ::core::ffi::c_uint;
    let mut lens_used = 0usize;
    let ret = crate::src::inftrees::inflate_table_impl(
        crate::src::inftrees::LENS,
        lens,
        nlen,
        codes,
        &mut lens_used,
        &mut lenbits,
        work,
    );
    if ret != 0 {
        return InflateDynamicTables::InvalidLiteralLengths { lenbits };
    }

    let mut distbits = 6 as ::core::ffi::c_uint;
    let mut dist_used = 0usize;
    let dist_lens = lens.get(nlen as usize..).unwrap_or(&[]);
    let dist_table = if lens_used <= codes.len() {
        &mut codes[lens_used..]
    } else {
        &mut []
    };
    let ret = crate::src::inftrees::inflate_table_impl(
        crate::src::inftrees::DISTS,
        dist_lens,
        ndist,
        dist_table,
        &mut dist_used,
        &mut distbits,
        work,
    );
    if ret != 0 {
        return InflateDynamicTables::InvalidDistances {
            lens_used,
            lenbits,
            distbits,
        };
    }

    InflateDynamicTables::Built {
        lens_used,
        total_used: lens_used + dist_used,
        lenbits,
        distbits,
    }
}

struct InflateGzipHeaderFieldScan {
    consumed: usize,
    terminated: bool,
}

fn inflate_gzip_header_field_scan(input: &[crate::stdlib::Bytef]) -> InflateGzipHeaderFieldScan {
    match input.iter().position(|&byte| byte == 0) {
        Some(index) => InflateGzipHeaderFieldScan {
            consumed: index + 1,
            terminated: true,
        },
        None => InflateGzipHeaderFieldScan {
            consumed: input.len(),
            terminated: false,
        },
    }
}

enum InflateMatchCopySource {
    Output { distance: crate::stdlib::uInt },
    Window { index: crate::stdlib::uInt },
}

struct InflateMatchCopyPlan {
    source: InflateMatchCopySource,
    copy: crate::stdlib::uInt,
}

fn inflate_match_copy_plan(
    offset: crate::stdlib::uInt,
    produced: crate::stdlib::uInt,
    whave: crate::stdlib::uInt,
    wnext: crate::stdlib::uInt,
    wsize: crate::stdlib::uInt,
    length: crate::stdlib::uInt,
    left: crate::stdlib::uInt,
    sane: bool,
) -> Option<InflateMatchCopyPlan> {
    let mut copy;
    let source;
    if offset > produced {
        copy = offset.wrapping_sub(produced);
        if copy > whave && sane {
            return None;
        }
        let index = if copy > wnext {
            copy = copy.wrapping_sub(wnext);
            wsize.wrapping_sub(copy)
        } else {
            wnext.wrapping_sub(copy)
        };
        if copy > length {
            copy = length;
        }
        source = InflateMatchCopySource::Window { index };
    } else {
        source = InflateMatchCopySource::Output { distance: offset };
        copy = length;
    }
    if copy > left {
        copy = left;
    }

    Some(InflateMatchCopyPlan { source, copy })
}

fn inflate_direct_copy_len(
    length: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    let mut copy = length;
    if copy > have {
        copy = have;
    }
    if copy > left {
        copy = left;
    }
    if copy == 0 as ::core::ffi::c_uint {
        None
    } else {
        Some(copy)
    }
}

pub(crate) struct InflateBlockHeader {
    pub(crate) last: ::core::ffi::c_int,
    pub(crate) block_type: ::core::ffi::c_uint,
    pub(crate) hold: ::core::ffi::c_ulong,
    pub(crate) bits: ::core::ffi::c_uint,
}

pub(crate) fn inflate_block_header(
    mut hold: ::core::ffi::c_ulong,
    mut bits: ::core::ffi::c_uint,
) -> InflateBlockHeader {
    let last = (hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_uint)) as ::core::ffi::c_int;
    hold >>= 1 as ::core::ffi::c_int;
    bits = bits.wrapping_sub(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
    let block_type = hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_uint);
    hold >>= 2 as ::core::ffi::c_int;
    bits = bits.wrapping_sub(2 as ::core::ffi::c_int as ::core::ffi::c_uint);

    InflateBlockHeader {
        last,
        block_type,
        hold,
        bits,
    }
}

fn inflate_zlib_check_word(hold: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    (hold >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_ulong)
        .wrapping_add(hold >> 8 as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_ulong)
        .wrapping_add((hold & 0xff00 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int)
        .wrapping_add((hold & 0xff as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int)
}

fn inflate_expected_check_word(
    hold: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    if flags != 0 {
        hold
    } else {
        inflate_zlib_check_word(hold)
    }
}

fn inflate_gzip_length_matches(hold: ::core::ffi::c_ulong, total: ::core::ffi::c_ulong) -> bool {
    hold == total & 0xffffffff as ::core::ffi::c_ulong
}

fn inflate_gzip_header_crc_matches(
    hold: ::core::ffi::c_ulong,
    check: ::core::ffi::c_ulong,
) -> bool {
    hold == check & 0xffff as ::core::ffi::c_ulong
}

fn inflate_gzip_header_crc_update_enabled(
    flags: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
) -> bool {
    flags & 0x200 as ::core::ffi::c_int != 0 && wrap & 4 as ::core::ffi::c_int != 0
}

fn inflate_update_output_check(
    check: crate::stdlib::uLong,
    flags: ::core::ffi::c_int,
    output: &[crate::stdlib::Bytef],
) -> crate::stdlib::uLong {
    if flags != 0 {
        crate::src::crc32::crc32_update(check, output)
    } else {
        crate::src::adler32::adler32_update(check, output)
    }
}

enum InflateZlibHeaderError {
    IncorrectHeaderCheck,
    UnknownCompressionMethod,
    InvalidWindowSize { wbits: ::core::ffi::c_uint },
}

struct InflateZlibHeader {
    wbits: ::core::ffi::c_uint,
    dmax: ::core::ffi::c_uint,
    needs_dictionary: bool,
}

fn inflate_zlib_header(
    hold: ::core::ffi::c_ulong,
    wrap: ::core::ffi::c_int,
    current_wbits: ::core::ffi::c_uint,
) -> Result<InflateZlibHeader, InflateZlibHeaderError> {
    if wrap & 1 as ::core::ffi::c_int == 0
        || (((hold as ::core::ffi::c_uint
            & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_uint))
            << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
            .wrapping_add(hold >> 8 as ::core::ffi::c_int)
            .wrapping_rem(31 as ::core::ffi::c_ulong)
            != 0
    {
        return Err(InflateZlibHeaderError::IncorrectHeaderCheck);
    }

    if hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_uint)
        != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
    {
        return Err(InflateZlibHeaderError::UnknownCompressionMethod);
    }

    let shifted = hold >> 4 as ::core::ffi::c_int;
    let len = (shifted as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
            .wrapping_sub(1 as ::core::ffi::c_uint))
    .wrapping_add(8 as ::core::ffi::c_uint);
    let wbits = if current_wbits == 0 as ::core::ffi::c_uint {
        len
    } else {
        current_wbits
    };
    if len > 15 as ::core::ffi::c_uint || len > wbits {
        return Err(InflateZlibHeaderError::InvalidWindowSize { wbits });
    }

    Ok(InflateZlibHeader {
        wbits,
        dmax: (1 as ::core::ffi::c_uint) << len,
        needs_dictionary: shifted & 0x200 as ::core::ffi::c_ulong != 0,
    })
}

fn inflate_data_type(
    bits: ::core::ffi::c_uint,
    last: ::core::ffi::c_int,
    mode: crate::src::inflate::inflate_mode,
) -> ::core::ffi::c_int {
    bits as ::core::ffi::c_int
        + (if last != 0 {
            64 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
        + (if mode as ::core::ffi::c_uint
            == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            128 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
        + (if mode as ::core::ffi::c_uint
            == crate::src::inflate::LEN_ as ::core::ffi::c_int as ::core::ffi::c_uint
            || mode as ::core::ffi::c_uint
                == crate::src::inflate::COPY_ as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            256 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
}

fn inflate_should_update_window(
    wsize: ::core::ffi::c_uint,
    produced: ::core::ffi::c_uint,
    mode: crate::src::inflate::inflate_mode,
    flush: ::core::ffi::c_int,
) -> bool {
    wsize != 0
        || produced != 0
            && (mode as ::core::ffi::c_uint) < crate::src::inflate::BAD as ::core::ffi::c_uint
            && ((mode as ::core::ffi::c_uint) < crate::src::inflate::CHECK as ::core::ffi::c_uint
                || flush != crate::zlib_h::Z_FINISH)
}

#[derive(Copy, Clone)]
struct InflateWindowCopy {
    dst: ::core::ffi::c_uint,
    src_back: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
}

#[derive(Copy, Clone)]
struct InflateWindowUpdatePlan {
    first: InflateWindowCopy,
    second: Option<InflateWindowCopy>,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
}

fn inflate_window_update_plan(
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    copy: ::core::ffi::c_uint,
) -> InflateWindowUpdatePlan {
    if copy >= wsize {
        return InflateWindowUpdatePlan {
            first: InflateWindowCopy {
                dst: 0 as ::core::ffi::c_uint,
                src_back: wsize,
                len: wsize,
            },
            second: None,
            wnext: 0 as ::core::ffi::c_uint,
            whave: wsize,
        };
    }

    let mut first_len = wsize.wrapping_sub(wnext);
    if first_len > copy {
        first_len = copy;
    }
    let remaining = copy.wrapping_sub(first_len);
    if remaining != 0 {
        InflateWindowUpdatePlan {
            first: InflateWindowCopy {
                dst: wnext,
                src_back: copy,
                len: first_len,
            },
            second: Some(InflateWindowCopy {
                dst: 0 as ::core::ffi::c_uint,
                src_back: remaining,
                len: remaining,
            }),
            wnext: remaining,
            whave: wsize,
        }
    } else {
        let mut next = wnext.wrapping_add(first_len);
        if next == wsize {
            next = 0 as ::core::ffi::c_uint;
        }
        InflateWindowUpdatePlan {
            first: InflateWindowCopy {
                dst: wnext,
                src_back: copy,
                len: first_len,
            },
            second: None,
            wnext: next,
            whave: if whave < wsize {
                whave.wrapping_add(first_len)
            } else {
                whave
            },
        }
    }
}

fn updatewindow_impl(
    wbits: ::core::ffi::c_uint,
    wsize: &mut ::core::ffi::c_uint,
    wnext: &mut ::core::ffi::c_uint,
    whave: &mut ::core::ffi::c_uint,
    window: &mut [crate::stdlib::Bytef],
    end: &[crate::stdlib::Bytef],
) {
    let copy = end.len() as ::core::ffi::c_uint;
    if *wsize == 0 as ::core::ffi::c_uint {
        *wsize = (1 as ::core::ffi::c_uint) << wbits;
        *wnext = 0 as ::core::ffi::c_uint;
        *whave = 0 as ::core::ffi::c_uint;
    }
    let plan = inflate_window_update_plan(*wsize, *wnext, *whave, copy);
    let mut apply_copy = |copy: InflateWindowCopy| {
        let len = copy.len as usize;
        if len == 0 {
            return;
        }
        let dst = copy.dst as usize;
        let src = end.len() - copy.src_back as usize;
        window[dst..dst + len].copy_from_slice(&end[src..src + len]);
    };
    apply_copy(plan.first);
    if let Some(second) = plan.second {
        apply_copy(second);
    }
    *wnext = plan.wnext;
    *whave = plan.whave;
}

fn inflate_finish_return(
    ret: ::core::ffi::c_int,
    consumed: ::core::ffi::c_uint,
    produced: ::core::ffi::c_uint,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (consumed == 0 as ::core::ffi::c_uint && produced == 0 as ::core::ffi::c_uint
        || flush == crate::zlib_h::Z_FINISH)
        && ret == crate::zlib_h::Z_OK
    {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        ret
    }
}

fn inflate_state_fields_are_valid(state: &crate::src::inflate::inflate_state) -> bool {
    inflate_mode_is_valid(state.mode)
}

macro_rules! inflate_state_check_raw {
    ($strm:expr) => {{
        let strm = $strm;
        if strm.is_null() {
            1 as ::core::ffi::c_int
        } else {
            let strm_ref = &*strm;
            if strm_ref.zalloc.is_none() || strm_ref.zfree.is_none() {
                1 as ::core::ffi::c_int
            } else {
                let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
                if state.is_null() {
                    1 as ::core::ffi::c_int
                } else {
                    let state_ref = &*state;
                    (state_ref.strm != strm || !inflate_state_fields_are_valid(state_ref))
                        as ::core::ffi::c_int
                }
            }
        }
    }};
}

fn inflate_reset_keep_state(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    state.total = 0 as ::core::ffi::c_ulong;
    strm.total_out = state.total as crate::stdlib::uLong;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = 0 as ::core::ffi::c_int;
    if state.wrap != 0 {
        strm.adler = (state.wrap & 1 as ::core::ffi::c_int) as crate::stdlib::uLong;
    }
    state.mode = crate::src::inflate::HEAD;
    state.last = 0 as ::core::ffi::c_int;
    state.havedict = 0 as ::core::ffi::c_int;
    state.flags = -1 as ::core::ffi::c_int;
    state.dmax = 32768 as ::core::ffi::c_uint;
    state.head = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
    state.hold = 0 as ::core::ffi::c_ulong;
    state.bits = 0 as ::core::ffi::c_uint;
    state.next = state.codes.as_mut_ptr();
    state.distcode = state.next;
    state.lencode = state.distcode;
    state.sane = 1 as ::core::ffi::c_int;
    state.back = -1 as ::core::ffi::c_int;
    crate::zlib_h::Z_OK
}

#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    return inflate_reset_keep_state(&mut *strm, &mut *state);
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    let state_ref = &mut *state;
    inflate_reset_window_state(state_ref);
    return inflate_reset_keep_state(&mut *strm, state_ref);
}

fn inflate_reset_window_state(state: &mut crate::src::inflate::inflate_state) {
    state.wsize = 0 as ::core::ffi::c_uint;
    state.whave = 0 as ::core::ffi::c_uint;
    state.wnext = 0 as ::core::ffi::c_uint;
}

#[derive(Copy, Clone)]
struct InflateReset2Config {
    wrap: ::core::ffi::c_int,
    window_bits: ::core::ffi::c_int,
}

fn inflate_reset2_config(mut window_bits: ::core::ffi::c_int) -> Option<InflateReset2Config> {
    let wrap: ::core::ffi::c_int;
    if window_bits < 0 as ::core::ffi::c_int {
        if window_bits < -15 as ::core::ffi::c_int {
            return None;
        }
        wrap = 0 as ::core::ffi::c_int;
        window_bits = -window_bits;
    } else {
        wrap = (window_bits >> 4 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int;
        if window_bits < 48 as ::core::ffi::c_int {
            window_bits &= 15 as ::core::ffi::c_int;
        }
    }
    if window_bits != 0
        && (window_bits < 8 as ::core::ffi::c_int || window_bits > 15 as ::core::ffi::c_int)
    {
        return None;
    }
    Some(InflateReset2Config { wrap, window_bits })
}

fn inflate_reset2_should_free_window(
    window_is_null: bool,
    current_wbits: ::core::ffi::c_uint,
    new_window_bits: ::core::ffi::c_int,
) -> bool {
    !window_is_null && current_wbits != new_window_bits as ::core::ffi::c_uint
}

#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm_ref = &mut *strm;
    let state_ref = &mut *(strm_ref.state as *mut crate::src::inflate::inflate_state);
    let Some(config) = inflate_reset2_config(windowBits) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if inflate_reset2_should_free_window(
        state_ref.window.is_null(),
        state_ref.wbits,
        config.window_bits,
    ) {
        let zfree = Some(strm_ref.zfree.expect("non-null function pointer"))
            .expect("non-null function pointer");
        zfree(strm_ref.opaque, state_ref.window as crate::stdlib::voidpf);
        state_ref.window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    state_ref.wrap = config.wrap;
    state_ref.wbits = config.window_bits as ::core::ffi::c_uint;
    inflate_reset_window_state(state_ref);
    return inflate_reset_keep_state(strm_ref, state_ref);
}
macro_rules! inflate_init2_body {
    ($strm:expr, $windowBits:expr, $version:expr, $stream_size:expr $(,)?) => {{
        let strm = $strm;
        let windowBits = $windowBits;
        let version = $version;
        let stream_size = $stream_size;
        let mut ret: ::core::ffi::c_int = 0;
        let mut state: *mut crate::src::inflate::inflate_state =
            ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
        if version.is_null()
            || *version as ::core::ffi::c_int
                != crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
            || stream_size
                != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
        {
            return crate::zlib_h::Z_VERSION_ERROR;
        }
        if strm.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if (*strm).zalloc.is_none() {
            (*strm).zalloc = Some(
                crate::src::zutil::zcalloc_ffi
                    as unsafe extern "C" fn(
                        crate::stdlib::voidpf,
                        ::core::ffi::c_uint,
                        ::core::ffi::c_uint,
                    ) -> crate::stdlib::voidpf,
            ) as crate::zlib_h::alloc_func;
            (*strm).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        if (*strm).zfree.is_none() {
            (*strm).zfree = Some(
                crate::src::zutil::zcfree_ffi
                    as unsafe extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> (),
            ) as crate::zlib_h::free_func;
        }
        state = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::inflate::inflate_state;
        if state.is_null() {
            return crate::zlib_h::Z_MEM_ERROR;
        }
        ::core::ptr::write_bytes(state, 0, 1);
        (*strm).state = state as *mut crate::src::deflate::internal_state;
        (*state).strm = strm;
        (*state).window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        (*state).mode = crate::src::inflate::HEAD;
        if let Some(config) = inflate_reset2_config(windowBits) {
            let state_ref = &mut *state;
            state_ref.wrap = config.wrap;
            state_ref.wbits = config.window_bits as ::core::ffi::c_uint;
            inflate_reset_window_state(state_ref);
            ret = inflate_reset_keep_state(&mut *strm, state_ref);
        } else {
            ret = crate::zlib_h::Z_STREAM_ERROR;
        }
        if ret != crate::zlib_h::Z_OK {
            Some((*strm).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*strm).opaque,
                state as crate::stdlib::voidpf,
            );
            (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
        }
        ret
    }};
}

#[export_name = "inflateInit2_"]
pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_init2_body!(strm, windowBits, version, stream_size)
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_init2_body!(strm, crate::zutil_h::DEF_WBITS, version, stream_size)
}
pub fn inflatePrime(
    state: &mut crate::src::inflate::inflate_state,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if bits == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_OK;
    }
    if bits < 0 as ::core::ffi::c_int {
        state.hold = 0 as ::core::ffi::c_ulong;
        state.bits = 0 as ::core::ffi::c_uint;
        return crate::zlib_h::Z_OK;
    }
    if bits > 16 as ::core::ffi::c_int
        || (state.bits as crate::stdlib::uInt).wrapping_add(bits as crate::stdlib::uInt)
            > 32 as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    value = (value as ::core::ffi::c_long
        & ((1 as ::core::ffi::c_long) << bits) - 1 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
    state.hold = state
        .hold
        .wrapping_add((value as ::core::ffi::c_ulong) << state.bits);
    state.bits = state
        .bits
        .wrapping_add(bits as crate::stdlib::uInt as ::core::ffi::c_uint);
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    inflatePrime(state, bits, value)
}
#[export_name = "inflate"]
pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut put: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut have: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut in_0: ::core::ffi::c_uint = 0;
    let mut out: ::core::ffi::c_uint = 0;
    let mut copy: ::core::ffi::c_uint = 0;
    let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut here: crate::src::inftrees::code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut last: crate::src::inftrees::code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut len: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut hbuf: [::core::ffi::c_uchar; 4] = [0; 4];
    if inflate_state_check_raw!(strm) != 0
        || (*strm).next_out.is_null()
        || (*strm).next_in.is_null() && (*strm).avail_in != 0 as crate::stdlib::uInt
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).mode as ::core::ffi::c_uint
        == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*state).mode = crate::src::inflate::TYPEDO;
    }
    put = (*strm).next_out as *mut ::core::ffi::c_uchar;
    left = (*strm).avail_out as ::core::ffi::c_uint;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    have = (*strm).avail_in as ::core::ffi::c_uint;
    hold = (*state).hold;
    bits = (*state).bits;
    in_0 = have;
    out = left;
    ret = crate::zlib_h::Z_OK;
    's_88: loop {
        match (*state).mode as ::core::ffi::c_uint {
            16180 => {
                if (*state).wrap == 0 as ::core::ffi::c_int {
                    (*state).mode = crate::src::inflate::TYPEDO;
                    continue;
                } else {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh0 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if (*state).wrap & 2 as ::core::ffi::c_int != 0
                        && hold == 0x8b1f as ::core::ffi::c_ulong
                    {
                        if (*state).wbits == 0 as ::core::ffi::c_uint {
                            (*state).wbits = 15 as ::core::ffi::c_uint;
                        }
                        (*state).check = crate::src::crc32::crc32_initial() as ::core::ffi::c_ulong;
                        hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                        hbuf[1 as ::core::ffi::c_int as usize] =
                            (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        (*state).check = crate::src::crc32::crc32_update(
                            (*state).check as crate::stdlib::uLong,
                            &hbuf[..2],
                        ) as ::core::ffi::c_ulong;
                        hold = 0 as ::core::ffi::c_ulong;
                        bits = 0 as ::core::ffi::c_uint;
                        (*state).mode = crate::src::inflate::FLAGS;
                        continue;
                    } else {
                        if !(*state).head.is_null() {
                            (*(*state).head).done = -1 as ::core::ffi::c_int;
                        }
                        match inflate_zlib_header(hold, (*state).wrap, (*state).wbits) {
                            Ok(header) => {
                                (*state).wbits = header.wbits;
                                (*state).dmax = header.dmax;
                                (*state).flags = 0 as ::core::ffi::c_int;
                                (*state).check =
                                    crate::src::adler32::adler32_initial() as ::core::ffi::c_ulong;
                                (*strm).adler = (*state).check as crate::stdlib::uLong;
                                (*state).mode = (if header.needs_dictionary {
                                    crate::src::inflate::DICTID as ::core::ffi::c_int
                                } else {
                                    crate::src::inflate::TYPE as ::core::ffi::c_int
                                })
                                    as crate::src::inflate::inflate_mode;
                                hold = 0 as ::core::ffi::c_ulong;
                                bits = 0 as ::core::ffi::c_uint;
                                continue;
                            }
                            Err(InflateZlibHeaderError::IncorrectHeaderCheck) => {
                                (*strm).msg = b"incorrect header check\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            }
                            Err(InflateZlibHeaderError::UnknownCompressionMethod) => {
                                (*strm).msg = b"unknown compression method\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            }
                            Err(InflateZlibHeaderError::InvalidWindowSize { wbits }) => {
                                (*state).wbits = wbits;
                                hold >>= 4 as ::core::ffi::c_int;
                                bits = bits
                                    .wrapping_sub(4 as ::core::ffi::c_int as ::core::ffi::c_uint);
                                (*strm).msg = b"invalid window size\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            }
                        }
                    }
                }
            }
            16181 => {
                while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh1 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                (*state).flags = hold as ::core::ffi::c_int;
                if (*state).flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED {
                    (*strm).msg = b"unknown compression method\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else if (*state).flags & 0xe000 as ::core::ffi::c_int != 0 {
                    (*strm).msg = b"unknown header flags set\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    if !(*state).head.is_null() {
                        (*(*state).head).text = (hold >> 8 as ::core::ffi::c_int
                            & 1 as ::core::ffi::c_ulong)
                            as ::core::ffi::c_int;
                    }
                    if inflate_gzip_header_crc_update_enabled((*state).flags, (*state).wrap) {
                        hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                        hbuf[1 as ::core::ffi::c_int as usize] =
                            (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        (*state).check = crate::src::crc32::crc32_update(
                            (*state).check as crate::stdlib::uLong,
                            &hbuf[..2],
                        ) as ::core::ffi::c_ulong;
                    }
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    (*state).mode = crate::src::inflate::TIME;
                }
                c2rust_current_block = 15855550149339537395;
            }
            16182 => {
                c2rust_current_block = 15855550149339537395;
            }
            16183 => {
                c2rust_current_block = 562309032768341766;
            }
            16184 => {
                c2rust_current_block = 14452068164804587099;
            }
            16185 => {
                c2rust_current_block = 7763740415849674987;
            }
            16186 => {
                c2rust_current_block = 18304778756172692371;
            }
            16187 => {
                c2rust_current_block = 9191988293914270845;
            }
            16188 => {
                c2rust_current_block = 13612704868423442610;
            }
            16189 => {
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh10 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh10 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                (*state).check = inflate_zlib_check_word(hold);
                (*strm).adler = (*state).check as crate::stdlib::uLong;
                hold = 0 as ::core::ffi::c_ulong;
                bits = 0 as ::core::ffi::c_uint;
                (*state).mode = crate::src::inflate::DICT;
                c2rust_current_block = 10495276606450942267;
            }
            16190 => {
                c2rust_current_block = 10495276606450942267;
            }
            16191 => {
                c2rust_current_block = 11604185039344352166;
            }
            16192 => {
                c2rust_current_block = 9224094624523183306;
            }
            16193 => {
                hold >>= bits & 7 as ::core::ffi::c_uint;
                bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh12 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh12 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if let Some(length) = inflate_stored_block_length(hold) {
                    (*state).length = length;
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    (*state).mode = crate::src::inflate::COPY_;
                    if flush == crate::zlib_h::Z_TREES {
                        break;
                    }
                } else {
                    (*strm).msg = b"invalid stored block lengths\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                }
                c2rust_current_block = 17610290921369817802;
            }
            16194 => {
                c2rust_current_block = 17610290921369817802;
            }
            16195 => {
                c2rust_current_block = 16745500758254703311;
            }
            16196 => {
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh13 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh13 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                let counts = inflate_dynamic_counts(hold, bits);
                (*state).nlen = counts.nlen;
                (*state).ndist = counts.ndist;
                (*state).ncode = counts.ncode;
                hold = counts.hold;
                bits = counts.bits;
                if !inflate_dynamic_counts_are_valid((*state).nlen, (*state).ndist) {
                    (*strm).msg = b"too many length or distance symbols\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    (*state).have = 0 as ::core::ffi::c_uint;
                    (*state).mode = crate::src::inflate::LENLENS;
                }
                c2rust_current_block = 18280650742570575093;
            }
            16197 => {
                c2rust_current_block = 18280650742570575093;
            }
            16198 => {
                c2rust_current_block = 12883017672845564788;
            }
            16199 => {
                c2rust_current_block = 8747825537946998525;
            }
            16200 => {
                c2rust_current_block = 12354422184948796071;
            }
            16201 => {
                c2rust_current_block = 10473654687254177392;
            }
            16202 => {
                c2rust_current_block = 14619999244790055076;
            }
            16203 => {
                c2rust_current_block = 4315581362918593597;
            }
            16204 => {
                c2rust_current_block = 14970513664919854643;
            }
            16205 => {
                if left == 0 as ::core::ffi::c_uint {
                    break;
                }
                let c2rust_fresh32 = put;
                put = put.offset(1);
                *c2rust_fresh32 = (*state).length as ::core::ffi::c_uchar;
                left = left.wrapping_sub(1);
                (*state).mode = crate::src::inflate::LEN;
                continue;
            }
            16206 => {
                if (*state).wrap != 0 {
                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh33 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh33 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    out = out.wrapping_sub(left);
                    (*strm).total_out = (*strm).total_out.wrapping_add(out as crate::stdlib::uLong);
                    (*state).total = (*state).total.wrapping_add(out as ::core::ffi::c_ulong);
                    if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
                        let output = ::core::slice::from_raw_parts(
                            put.offset(-(out as isize)),
                            out as usize,
                        );
                        (*state).check = inflate_update_output_check(
                            (*state).check as crate::stdlib::uLong,
                            (*state).flags,
                            output,
                        ) as ::core::ffi::c_ulong;
                        (*strm).adler = (*state).check as crate::stdlib::uLong;
                    }
                    out = left;
                    if (*state).wrap & 4 as ::core::ffi::c_int != 0
                        && inflate_expected_check_word(hold, (*state).flags) != (*state).check
                    {
                        (*strm).msg = b"incorrect data check\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        hold = 0 as ::core::ffi::c_ulong;
                        bits = 0 as ::core::ffi::c_uint;
                    }
                }
                (*state).mode = crate::src::inflate::LENGTH;
                c2rust_current_block = 10372812520561896112;
            }
            16207 => {
                c2rust_current_block = 10372812520561896112;
            }
            16208 => {
                c2rust_current_block = 12591847850361142309;
            }
            16209 => {
                ret = crate::zlib_h::Z_DATA_ERROR;
                break;
            }
            16210 => return crate::zlib_h::Z_MEM_ERROR,
            16211 | _ => return crate::zlib_h::Z_STREAM_ERROR,
        }
        match c2rust_current_block {
            10372812520561896112 => {
                if (*state).wrap != 0 && (*state).flags != 0 {
                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh34 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh34 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if (*state).wrap & 4 as ::core::ffi::c_int != 0
                        && !inflate_gzip_length_matches(hold, (*state).total)
                    {
                        (*strm).msg = b"incorrect length check\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        hold = 0 as ::core::ffi::c_ulong;
                        bits = 0 as ::core::ffi::c_uint;
                    }
                }
                (*state).mode = crate::src::inflate::DONE;
                c2rust_current_block = 12591847850361142309;
            }
            18280650742570575093 => {
                while (*state).have < (*state).ncode {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh14 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh14 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    let c2rust_fresh15 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[crate::src::inflate::INFLATE_CODE_LENGTH_ORDER
                        [c2rust_fresh15 as usize] as usize] = (hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_ushort;
                    hold >>= 3 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(3 as ::core::ffi::c_int as ::core::ffi::c_uint);
                }
                crate::src::inflate::inflate_zero_code_length_order_tail(
                    &mut (*state).lens,
                    &mut (*state).have,
                );
                (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                (*state).distcode = (*state).next as *const crate::src::inftrees::code;
                (*state).lencode = (*state).distcode;
                (*state).lenbits = 7 as ::core::ffi::c_uint;
                let mut table_used = 0usize;
                ret = crate::src::inftrees::inflate_table_impl(
                    crate::src::inftrees::CODES,
                    &(*state).lens,
                    19 as ::core::ffi::c_uint,
                    &mut (*state).codes,
                    &mut table_used,
                    &mut (*state).lenbits,
                    &mut (*state).work,
                );
                if ret == 0 {
                    (*state).next = (*state).codes.as_mut_ptr().wrapping_add(table_used);
                }
                if ret != 0 {
                    (*strm).msg = b"invalid code lengths set\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    (*state).have = 0 as ::core::ffi::c_uint;
                    (*state).mode = crate::src::inflate::CODELENS;
                }
                c2rust_current_block = 12883017672845564788;
            }
            10495276606450942267 => {
                if (*state).havedict == 0 as ::core::ffi::c_int {
                    (*strm).next_out = put as *mut crate::stdlib::Bytef;
                    (*strm).avail_out = left as crate::stdlib::uInt;
                    (*strm).next_in = next as *mut crate::stdlib::Bytef;
                    (*strm).avail_in = have as crate::stdlib::uInt;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    return crate::zlib_h::Z_NEED_DICT;
                }
                (*state).check = crate::src::adler32::adler32_initial() as ::core::ffi::c_ulong;
                (*strm).adler = (*state).check as crate::stdlib::uLong;
                (*state).mode = crate::src::inflate::TYPE;
                c2rust_current_block = 11604185039344352166;
            }
            15855550149339537395 => {
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh2 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh2 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if !(*state).head.is_null() {
                    (*(*state).head).time = hold as crate::stdlib::uLong;
                }
                if inflate_gzip_header_crc_update_enabled((*state).flags, (*state).wrap) {
                    hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                    hbuf[1 as ::core::ffi::c_int as usize] =
                        (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    hbuf[2 as ::core::ffi::c_int as usize] =
                        (hold >> 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    hbuf[3 as ::core::ffi::c_int as usize] =
                        (hold >> 24 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    (*state).check = crate::src::crc32::crc32_update(
                        (*state).check as crate::stdlib::uLong,
                        &hbuf,
                    ) as ::core::ffi::c_ulong;
                }
                hold = 0 as ::core::ffi::c_ulong;
                bits = 0 as ::core::ffi::c_uint;
                (*state).mode = crate::src::inflate::OS;
                c2rust_current_block = 562309032768341766;
            }
            17610290921369817802 => {
                (*state).mode = crate::src::inflate::COPY_1;
                c2rust_current_block = 16745500758254703311;
            }
            _ => {}
        }
        match c2rust_current_block {
            12883017672845564788 => {
                while (*state).have < (*state).nlen.wrapping_add((*state).ndist) {
                    loop {
                        here = *(*state).lencode.offset(
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                as isize,
                        );
                        if here.bits as ::core::ffi::c_uint <= bits {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh17 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh17 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if (here.val as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                        hold >>= here.bits as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                        let c2rust_fresh18 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[c2rust_fresh18 as usize] = here.val;
                    } else {
                        let repeat_code = here.val as ::core::ffi::c_uint;
                        let repeat_extra =
                            crate::src::inflate::inflate_code_length_repeat_extra_bits(repeat_code);
                        while bits < (here.bits as ::core::ffi::c_uint).wrapping_add(repeat_extra) {
                            if have == 0 as ::core::ffi::c_uint {
                                break 's_88;
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh19 = next;
                            next = next.offset(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh19 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        hold >>= here.bits as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                        let previous_len = if repeat_code == 16 as ::core::ffi::c_uint {
                            if (*state).have == 0 as ::core::ffi::c_uint {
                                (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                break;
                            }
                            (*state).lens
                                [(*state).have.wrapping_sub(1 as ::core::ffi::c_uint) as usize]
                                as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        };
                        let repeat = crate::src::inflate::inflate_code_length_repeat(
                            repeat_code,
                            previous_len,
                            hold,
                            bits,
                        );
                        len = repeat.len;
                        copy = repeat.copy;
                        hold = repeat.hold;
                        bits = repeat.bits;
                        if !crate::src::inflate::inflate_code_length_repeat_fits(
                            (*state).have,
                            copy,
                            (*state).nlen,
                            (*state).ndist,
                        ) {
                            (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                            break;
                        } else {
                            loop {
                                let c2rust_fresh22 = copy;
                                copy = copy.wrapping_sub(1);
                                if !(c2rust_fresh22 != 0) {
                                    break;
                                }
                                let c2rust_fresh23 = (*state).have;
                                (*state).have = (*state).have.wrapping_add(1);
                                (*state).lens[c2rust_fresh23 as usize] =
                                    len as ::core::ffi::c_ushort;
                            }
                        }
                    }
                }
                if (*state).mode as ::core::ffi::c_uint
                    == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    continue;
                }
                if !inflate_has_end_of_block_code(&(*state).lens) {
                    (*strm).msg = b"invalid code -- missing end-of-block\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    let state_ref = &mut *state;
                    let codes_base = state_ref.codes.as_mut_ptr();
                    state_ref.next = codes_base;
                    state_ref.lencode = codes_base as *const crate::src::inftrees::code;
                    match crate::src::inflate::inflate_build_dynamic_tables(
                        &state_ref.lens,
                        state_ref.nlen,
                        state_ref.ndist,
                        &mut state_ref.codes,
                        &mut state_ref.work,
                    ) {
                        crate::src::inflate::InflateDynamicTables::Built {
                            lens_used,
                            total_used,
                            lenbits,
                            distbits,
                        } => {
                            state_ref.lenbits = lenbits;
                            state_ref.next = codes_base.wrapping_add(lens_used);
                            state_ref.distcode =
                                state_ref.next as *const crate::src::inftrees::code;
                            state_ref.distbits = distbits;
                            state_ref.next = codes_base.wrapping_add(total_used);
                            state_ref.mode = crate::src::inflate::LEN_;
                            if flush == crate::zlib_h::Z_TREES {
                                break;
                            }
                        }
                        crate::src::inflate::InflateDynamicTables::InvalidLiteralLengths {
                            lenbits,
                        } => {
                            state_ref.lenbits = lenbits;
                            (*strm).msg = b"invalid literal/lengths set\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            state_ref.mode = crate::src::inflate::BAD;
                            continue;
                        }
                        crate::src::inflate::InflateDynamicTables::InvalidDistances {
                            lens_used,
                            lenbits,
                            distbits,
                        } => {
                            state_ref.lenbits = lenbits;
                            state_ref.next = codes_base.wrapping_add(lens_used);
                            state_ref.distcode =
                                state_ref.next as *const crate::src::inftrees::code;
                            state_ref.distbits = distbits;
                            (*strm).msg = b"invalid distances set\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            state_ref.mode = crate::src::inflate::BAD;
                            continue;
                        }
                    }
                }
                c2rust_current_block = 8747825537946998525;
            }
            16745500758254703311 => {
                copy = (*state).length;
                if copy != 0 {
                    let Some(copy_len) = inflate_direct_copy_len(copy, have, left) else {
                        break;
                    };
                    copy = copy_len;
                    let input = ::core::slice::from_raw_parts(next, copy as usize);
                    let output = ::core::slice::from_raw_parts_mut(put, copy as usize);
                    output.copy_from_slice(input);
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    left = left.wrapping_sub(copy);
                    put = put.offset(copy as isize);
                    (*state).length = (*state).length.wrapping_sub(copy);
                    continue;
                } else {
                    (*state).mode = crate::src::inflate::TYPE;
                    continue;
                }
            }
            562309032768341766 => {
                while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh3 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh3 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if !(*state).head.is_null() {
                    (*(*state).head).xflags =
                        (hold & 0xff as ::core::ffi::c_ulong) as ::core::ffi::c_int;
                    (*(*state).head).os = (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
                }
                if inflate_gzip_header_crc_update_enabled((*state).flags, (*state).wrap) {
                    hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                    hbuf[1 as ::core::ffi::c_int as usize] =
                        (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    (*state).check = crate::src::crc32::crc32_update(
                        (*state).check as crate::stdlib::uLong,
                        &hbuf[..2],
                    ) as ::core::ffi::c_ulong;
                }
                hold = 0 as ::core::ffi::c_ulong;
                bits = 0 as ::core::ffi::c_uint;
                (*state).mode = crate::src::inflate::EXLEN;
                c2rust_current_block = 14452068164804587099;
            }
            11604185039344352166 => {
                if flush == crate::zlib_h::Z_BLOCK || flush == crate::zlib_h::Z_TREES {
                    break;
                }
                c2rust_current_block = 9224094624523183306;
            }
            12591847850361142309 => {
                ret = crate::zlib_h::Z_STREAM_END;
                break;
            }
            _ => {}
        }
        match c2rust_current_block {
            9224094624523183306 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as ::core::ffi::c_uint;
                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                    (*state).mode = crate::src::inflate::CHECK;
                    continue;
                } else {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh11 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh11 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    let block_header = inflate_block_header(hold, bits);
                    (*state).last = block_header.last;
                    hold = block_header.hold;
                    bits = block_header.bits;
                    match block_header.block_type {
                        0 => {
                            (*state).mode = crate::src::inflate::STORED;
                        }
                        1 => {
                            let fixed = crate::src::inftrees::inflate_fixed_tables();
                            (*state).lencode = fixed.lencode.as_ptr();
                            (*state).lenbits = fixed.lenbits;
                            (*state).distcode = fixed.distcode.as_ptr();
                            (*state).distbits = fixed.distbits;
                            (*state).mode = crate::src::inflate::LEN_;
                            if flush == crate::zlib_h::Z_TREES {
                                break;
                            }
                        }
                        2 => {
                            (*state).mode = crate::src::inflate::TABLE;
                        }
                        _ => {
                            (*strm).msg = b"invalid block type\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                        }
                    }
                    continue;
                }
            }
            14452068164804587099 => {
                if (*state).flags & 0x400 as ::core::ffi::c_int != 0 {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh4 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh4 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    (*state).length = hold as ::core::ffi::c_uint;
                    if !(*state).head.is_null() {
                        (*(*state).head).extra_len =
                            hold as ::core::ffi::c_uint as crate::stdlib::uInt;
                    }
                    if inflate_gzip_header_crc_update_enabled((*state).flags, (*state).wrap) {
                        hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                        hbuf[1 as ::core::ffi::c_int as usize] =
                            (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        (*state).check = crate::src::crc32::crc32_update(
                            (*state).check as crate::stdlib::uLong,
                            &hbuf[..2],
                        ) as ::core::ffi::c_ulong;
                    }
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                } else if !(*state).head.is_null() {
                    (*(*state).head).extra = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                }
                (*state).mode = crate::src::inflate::EXTRA;
                c2rust_current_block = 7763740415849674987;
            }
            8747825537946998525 => {
                (*state).mode = crate::src::inflate::LEN;
                c2rust_current_block = 12354422184948796071;
            }
            _ => {}
        }
        match c2rust_current_block {
            12354422184948796071 => {
                if have >= 6 as ::core::ffi::c_uint && left >= 258 as ::core::ffi::c_uint {
                    (*strm).next_out = put as *mut crate::stdlib::Bytef;
                    (*strm).avail_out = left as crate::stdlib::uInt;
                    (*strm).next_in = next as *mut crate::stdlib::Bytef;
                    (*strm).avail_in = have as crate::stdlib::uInt;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    crate::src::inffast::inflate_fast_ffi(
                        strm as *mut crate::zlib_h::z_stream_s,
                        out,
                    );
                    put = (*strm).next_out as *mut ::core::ffi::c_uchar;
                    left = (*strm).avail_out as ::core::ffi::c_uint;
                    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
                    have = (*strm).avail_in as ::core::ffi::c_uint;
                    hold = (*state).hold;
                    bits = (*state).bits;
                    if (*state).mode as ::core::ffi::c_uint
                        == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*state).back = -1 as ::core::ffi::c_int;
                    }
                    continue;
                } else {
                    (*state).back = 0 as ::core::ffi::c_int;
                    loop {
                        here = *(*state).lencode.offset(
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                as isize,
                        );
                        if here.bits as ::core::ffi::c_uint <= bits {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh24 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh24 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if here.op as ::core::ffi::c_int != 0
                        && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        last = here;
                        loop {
                            here = *(*state).lencode.offset(
                                (last.val as ::core::ffi::c_uint).wrapping_add(
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint)
                                            << last.bits as ::core::ffi::c_int
                                                + last.op as ::core::ffi::c_int)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        >> last.bits as ::core::ffi::c_int,
                                ) as isize,
                            );
                            if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                <= bits
                            {
                                break;
                            }
                            if have == 0 as ::core::ffi::c_uint {
                                break 's_88;
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh25 = next;
                            next = next.offset(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh25 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        hold >>= last.bits as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                        (*state).back += last.bits as ::core::ffi::c_int;
                    }
                    hold >>= here.bits as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                    (*state).back += here.bits as ::core::ffi::c_int;
                    (*state).length = here.val as ::core::ffi::c_uint;
                    if here.op as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        (*state).mode = crate::src::inflate::LIT;
                        continue;
                    } else if here.op as ::core::ffi::c_int & 32 as ::core::ffi::c_int != 0 {
                        (*state).back = -1 as ::core::ffi::c_int;
                        (*state).mode = crate::src::inflate::TYPE;
                        continue;
                    } else if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                        (*strm).msg = b"invalid literal/length code\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        (*state).extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                        (*state).mode = crate::src::inflate::LENEXT;
                    }
                }
                c2rust_current_block = 10473654687254177392;
            }
            7763740415849674987 => {
                if (*state).flags & 0x400 as ::core::ffi::c_int != 0 {
                    copy = (*state).length;
                    if copy > have {
                        copy = have;
                    }
                    if copy != 0 {
                        if !(*state).head.is_null() && !(*(*state).head).extra.is_null() && {
                            len = ((*(*state).head).extra_len as ::core::ffi::c_uint)
                                .wrapping_sub((*state).length);
                            len < (*(*state).head).extra_max
                        } {
                            let write_len = if len.wrapping_add(copy) > (*(*state).head).extra_max {
                                ((*(*state).head).extra_max as ::core::ffi::c_uint)
                                    .wrapping_sub(len)
                            } else {
                                copy
                            };
                            let input = ::core::slice::from_raw_parts(next, write_len as usize);
                            let output = ::core::slice::from_raw_parts_mut(
                                (*(*state).head).extra.offset(len as isize),
                                write_len as usize,
                            );
                            output.copy_from_slice(input);
                        }
                        if inflate_gzip_header_crc_update_enabled((*state).flags, (*state).wrap) {
                            let input = ::core::slice::from_raw_parts(next, copy as usize);
                            (*state).check = crate::src::crc32::crc32_update(
                                (*state).check as crate::stdlib::uLong,
                                input,
                            ) as ::core::ffi::c_ulong;
                        }
                        have = have.wrapping_sub(copy);
                        next = next.offset(copy as isize);
                        (*state).length = (*state).length.wrapping_sub(copy);
                    }
                    if (*state).length != 0 {
                        break;
                    }
                }
                (*state).length = 0 as ::core::ffi::c_uint;
                (*state).mode = crate::src::inflate::NAME;
                c2rust_current_block = 18304778756172692371;
            }
            _ => {}
        }
        match c2rust_current_block {
            10473654687254177392 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh26 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh26 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    let extra_bits =
                        inflate_apply_extra_bits((*state).length, (*state).extra, hold, bits);
                    (*state).length = extra_bits.value;
                    hold = extra_bits.hold;
                    bits = extra_bits.bits;
                    (*state).back = ((*state).back as ::core::ffi::c_uint)
                        .wrapping_add((*state).extra)
                        as ::core::ffi::c_int;
                }
                (*state).was = (*state).length;
                (*state).mode = crate::src::inflate::DIST;
                c2rust_current_block = 14619999244790055076;
            }
            18304778756172692371 => {
                if (*state).flags & 0x800 as ::core::ffi::c_int != 0 {
                    if have == 0 as ::core::ffi::c_uint {
                        break;
                    }
                    let input = ::core::slice::from_raw_parts(
                        next,
                        have as crate::__stddef_size_t_h::size_t,
                    );
                    let scan = inflate_gzip_header_field_scan(input);
                    copy = scan.consumed as ::core::ffi::c_uint;
                    for &byte in &input[..scan.consumed] {
                        len = byte as ::core::ffi::c_uint;
                        if !(*state).head.is_null()
                            && !(*(*state).head).name.is_null()
                            && (*state).length < (*(*state).head).name_max
                        {
                            let c2rust_fresh6 = (*state).length;
                            (*state).length = (*state).length.wrapping_add(1);
                            *(*(*state).head).name.offset(c2rust_fresh6 as isize) =
                                len as crate::stdlib::Bytef;
                        }
                    }
                    if inflate_gzip_header_crc_update_enabled((*state).flags, (*state).wrap) {
                        (*state).check = crate::src::crc32::crc32_update(
                            (*state).check as crate::stdlib::uLong,
                            &input[..scan.consumed],
                        ) as ::core::ffi::c_ulong;
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if !scan.terminated {
                        break;
                    }
                } else if !(*state).head.is_null() {
                    (*(*state).head).name = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                }
                (*state).length = 0 as ::core::ffi::c_uint;
                (*state).mode = crate::src::inflate::COMMENT;
                c2rust_current_block = 9191988293914270845;
            }
            _ => {}
        }
        match c2rust_current_block {
            14619999244790055076 => {
                loop {
                    here = *(*state).distcode.offset(
                        (hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).distbits)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as isize,
                    );
                    if here.bits as ::core::ffi::c_uint <= bits {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh27 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh27 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    last = here;
                    loop {
                        here = *(*state).distcode.offset(
                            (last.val as ::core::ffi::c_uint).wrapping_add(
                                (hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint)
                                        << last.bits as ::core::ffi::c_int
                                            + last.op as ::core::ffi::c_int)
                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                    >> last.bits as ::core::ffi::c_int,
                            ) as isize,
                        );
                        if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            <= bits
                        {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh28 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh28 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    hold >>= last.bits as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                    (*state).back += last.bits as ::core::ffi::c_int;
                }
                hold >>= here.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                (*state).back += here.bits as ::core::ffi::c_int;
                if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                    (*strm).msg = b"invalid distance code\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    (*state).offset = here.val as ::core::ffi::c_uint;
                    (*state).extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                    (*state).mode = crate::src::inflate::DISTEXT;
                }
                c2rust_current_block = 4315581362918593597;
            }
            9191988293914270845 => {
                if (*state).flags & 0x1000 as ::core::ffi::c_int != 0 {
                    if have == 0 as ::core::ffi::c_uint {
                        break;
                    }
                    let input = ::core::slice::from_raw_parts(
                        next,
                        have as crate::__stddef_size_t_h::size_t,
                    );
                    let scan = inflate_gzip_header_field_scan(input);
                    copy = scan.consumed as ::core::ffi::c_uint;
                    for &byte in &input[..scan.consumed] {
                        len = byte as ::core::ffi::c_uint;
                        if !(*state).head.is_null()
                            && !(*(*state).head).comment.is_null()
                            && (*state).length < (*(*state).head).comm_max
                        {
                            let c2rust_fresh8 = (*state).length;
                            (*state).length = (*state).length.wrapping_add(1);
                            *(*(*state).head).comment.offset(c2rust_fresh8 as isize) =
                                len as crate::stdlib::Bytef;
                        }
                    }
                    if inflate_gzip_header_crc_update_enabled((*state).flags, (*state).wrap) {
                        (*state).check = crate::src::crc32::crc32_update(
                            (*state).check as crate::stdlib::uLong,
                            &input[..scan.consumed],
                        ) as ::core::ffi::c_ulong;
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if !scan.terminated {
                        break;
                    }
                } else if !(*state).head.is_null() {
                    (*(*state).head).comment = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                }
                (*state).mode = crate::src::inflate::HCRC;
                c2rust_current_block = 13612704868423442610;
            }
            _ => {}
        }
        match c2rust_current_block {
            4315581362918593597 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh29 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh29 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    let extra_bits =
                        inflate_apply_extra_bits((*state).offset, (*state).extra, hold, bits);
                    (*state).offset = extra_bits.value;
                    hold = extra_bits.hold;
                    bits = extra_bits.bits;
                    (*state).back = ((*state).back as ::core::ffi::c_uint)
                        .wrapping_add((*state).extra)
                        as ::core::ffi::c_int;
                }
                (*state).mode = crate::src::inflate::MATCH;
            }
            13612704868423442610 => {
                if (*state).flags & 0x200 as ::core::ffi::c_int != 0 {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh9 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh9 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if (*state).wrap & 4 as ::core::ffi::c_int != 0
                        && !inflate_gzip_header_crc_matches(hold, (*state).check)
                    {
                        (*strm).msg = b"header crc mismatch\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        hold = 0 as ::core::ffi::c_ulong;
                        bits = 0 as ::core::ffi::c_uint;
                    }
                }
                if !(*state).head.is_null() {
                    (*(*state).head).hcrc =
                        (*state).flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                    (*(*state).head).done = 1 as ::core::ffi::c_int;
                }
                (*state).check = crate::src::crc32::crc32_initial() as ::core::ffi::c_ulong;
                (*strm).adler = (*state).check as crate::stdlib::uLong;
                (*state).mode = crate::src::inflate::TYPE;
                continue;
            }
            _ => {}
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        let Some(copy_plan) = inflate_match_copy_plan(
            (*state).offset,
            out.wrapping_sub(left),
            (*state).whave,
            (*state).wnext,
            (*state).wsize,
            (*state).length,
            left,
            (*state).sane != 0,
        ) else {
            (*strm).msg = b"invalid distance too far back\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            (*state).mode = crate::src::inflate::BAD;
            continue;
        };
        from = match copy_plan.source {
            InflateMatchCopySource::Output { distance } => put.offset(-(distance as isize)),
            InflateMatchCopySource::Window { index } => (*state).window.offset(index as isize),
        };
        copy = copy_plan.copy;
        left = left.wrapping_sub(copy);
        (*state).length = (*state).length.wrapping_sub(copy);
        loop {
            let c2rust_fresh30 = from;
            from = from.offset(1);
            let c2rust_fresh31 = put;
            put = put.offset(1);
            *c2rust_fresh31 = *c2rust_fresh30;
            copy = copy.wrapping_sub(1);
            if !(copy != 0) {
                break;
            }
        }
        if (*state).length == 0 as ::core::ffi::c_uint {
            (*state).mode = crate::src::inflate::LEN;
        }
    }
    (*strm).next_out = put as *mut crate::stdlib::Bytef;
    (*strm).avail_out = left as crate::stdlib::uInt;
    (*strm).next_in = next as *mut crate::stdlib::Bytef;
    (*strm).avail_in = have as crate::stdlib::uInt;
    (*state).hold = hold;
    (*state).bits = bits;
    let produced = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
    if inflate_should_update_window((*state).wsize, produced, (*state).mode, flush) {
        let state_ref = &mut *state;
        if state_ref.window.is_null() {
            state_ref.window = Some((*strm).zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*strm).opaque,
                (1 as crate::stdlib::uInt) << state_ref.wbits,
                ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
            ) as *mut ::core::ffi::c_uchar;
            if state_ref.window.is_null() {
                state_ref.mode = crate::src::inflate::MEM;
                return crate::zlib_h::Z_MEM_ERROR;
            }
        }
        let window_len = ((1 as crate::stdlib::uInt) << state_ref.wbits) as usize;
        let window = ::core::slice::from_raw_parts_mut(state_ref.window, window_len);
        let output = if produced == 0 as ::core::ffi::c_uint {
            &[][..]
        } else {
            ::core::slice::from_raw_parts(
                (*strm).next_out.offset(-(produced as isize)),
                produced as usize,
            )
        };
        updatewindow_impl(
            state_ref.wbits,
            &mut state_ref.wsize,
            &mut state_ref.wnext,
            &mut state_ref.whave,
            window,
            output,
        );
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in as ::core::ffi::c_uint);
    out = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
    (*strm).total_in = (*strm).total_in.wrapping_add(in_0 as crate::stdlib::uLong);
    (*strm).total_out = (*strm).total_out.wrapping_add(out as crate::stdlib::uLong);
    (*state).total = (*state).total.wrapping_add(out as ::core::ffi::c_ulong);
    if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
        let output =
            ::core::slice::from_raw_parts((*strm).next_out.offset(-(out as isize)), out as usize);
        (*state).check = inflate_update_output_check(
            (*state).check as crate::stdlib::uLong,
            (*state).flags,
            output,
        ) as ::core::ffi::c_ulong;
        (*strm).adler = (*state).check as crate::stdlib::uLong;
    }
    (*strm).data_type = inflate_data_type((*state).bits, (*state).last, (*state).mode);
    return inflate_finish_return(ret, in_0, out, flush);
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm_ref = &mut *strm;
    let state_ptr = strm_ref.state as *mut crate::src::inflate::inflate_state;
    let state = &mut *state_ptr;
    let zfree = strm_ref.zfree.expect("non-null function pointer");
    let opaque = strm_ref.opaque;
    if !state.window.is_null() {
        zfree(opaque, state.window as crate::stdlib::voidpf);
    }
    zfree(opaque, state_ptr as crate::stdlib::voidpf);
    strm_ref.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return crate::zlib_h::Z_OK;
}
pub fn inflateGetDictionary(
    state: &crate::src::inflate::inflate_state,
) -> (::core::ffi::c_uint, ::core::ffi::c_uint) {
    return (state.wnext, state.whave);
}

fn copy_inflate_dictionary_window(
    dictionary: &mut [crate::stdlib::Bytef],
    window: &[crate::stdlib::Bytef],
    wnext: usize,
    whave: usize,
) {
    let first = whave - wnext;
    dictionary[..first].copy_from_slice(&window[wnext..whave]);
    dictionary[first..whave].copy_from_slice(&window[..wnext]);
}

#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*((*strm).state as *mut crate::src::inflate::inflate_state);
    let (wnext, whave) = inflateGetDictionary(state);
    if whave != 0 && !dictionary.is_null() {
        let dictionary_slice = ::core::slice::from_raw_parts_mut(dictionary, whave as usize);
        let window_slice = ::core::slice::from_raw_parts(state.window, whave as usize);
        copy_inflate_dictionary_window(
            dictionary_slice,
            window_slice,
            wnext as usize,
            whave as usize,
        );
    }
    if !dictLength.is_null() {
        *dictLength = whave as crate::stdlib::uInt;
    }
    return crate::zlib_h::Z_OK;
}

enum InflateDictionaryState {
    Reject,
    CheckId,
    Accept,
}

fn inflate_dictionary_state(
    wrap: ::core::ffi::c_int,
    mode: ::core::ffi::c_uint,
) -> InflateDictionaryState {
    let dict_mode = crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint;
    if wrap != 0 as ::core::ffi::c_int && mode != dict_mode {
        InflateDictionaryState::Reject
    } else if mode == dict_mode {
        InflateDictionaryState::CheckId
    } else {
        InflateDictionaryState::Accept
    }
}

fn inflate_dictionary_id_matches(
    dictid: ::core::ffi::c_ulong,
    expected: ::core::ffi::c_ulong,
) -> bool {
    dictid == expected
}

#[export_name = "inflateSetDictionary"]

pub unsafe extern "C" fn inflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut dictid: ::core::ffi::c_ulong = 0;
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    match inflate_dictionary_state((*state).wrap, (*state).mode as ::core::ffi::c_uint) {
        InflateDictionaryState::Reject => {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        InflateDictionaryState::CheckId => {
            dictid = crate::src::adler32::adler32_initial() as ::core::ffi::c_ulong;
            if dictLength != 0 as crate::stdlib::uInt || !dictionary.is_null() {
                let dictionary_slice =
                    ::core::slice::from_raw_parts(dictionary, dictLength as usize);
                dictid = crate::src::adler32::adler32_update(
                    dictid as crate::stdlib::uLong,
                    dictionary_slice,
                ) as ::core::ffi::c_ulong;
            }
            if !inflate_dictionary_id_matches(dictid, (*state).check) {
                return crate::zlib_h::Z_DATA_ERROR;
            }
        }
        InflateDictionaryState::Accept => {}
    }
    let state_ref = &mut *state;
    if state_ref.window.is_null() {
        state_ref.window = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            (1 as crate::stdlib::uInt) << state_ref.wbits,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if state_ref.window.is_null() {
            state_ref.mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    let window_len = ((1 as crate::stdlib::uInt) << state_ref.wbits) as usize;
    let window = ::core::slice::from_raw_parts_mut(state_ref.window, window_len);
    let dictionary_slice = if dictLength == 0 as crate::stdlib::uInt {
        &[][..]
    } else {
        ::core::slice::from_raw_parts(dictionary, dictLength as usize)
    };
    updatewindow_impl(
        state_ref.wbits,
        &mut state_ref.wsize,
        &mut state_ref.wnext,
        &mut state_ref.whave,
        window,
        dictionary_slice,
    );
    state_ref.havedict = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
fn inflate_get_header_allowed(state: &crate::src::inflate::inflate_state) -> bool {
    state.wrap & 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if !inflate_get_header_allowed(&*state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*state).head = head;
    (*head).done = 0 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
fn syncsearch(
    mut got: ::core::ffi::c_uint,
    buf: &[::core::ffi::c_uchar],
) -> (::core::ffi::c_uint, ::core::ffi::c_uint) {
    let mut next = 0 as ::core::ffi::c_uint;
    for &byte in buf {
        if got >= 4 as ::core::ffi::c_uint {
            break;
        }
        if byte as ::core::ffi::c_int
            == if got < 2 as ::core::ffi::c_uint {
                0 as ::core::ffi::c_int
            } else {
                0xff as ::core::ffi::c_int
            }
        {
            got = got.wrapping_add(1);
        } else if byte != 0 {
            got = 0 as ::core::ffi::c_uint;
        } else {
            got = (4 as ::core::ffi::c_uint).wrapping_sub(got);
        }
        next = next.wrapping_add(1);
    }
    (got, next)
}

struct InflateSyncDrain {
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    bytes: [::core::ffi::c_uchar; 4],
    len: usize,
}

fn inflate_sync_drain_pending_bytes(
    mut hold: ::core::ffi::c_ulong,
    mut bits: ::core::ffi::c_uint,
) -> InflateSyncDrain {
    let discard = bits & 7 as ::core::ffi::c_uint;
    hold >>= discard;
    bits = bits.wrapping_sub(discard);

    let mut bytes = [0 as ::core::ffi::c_uchar; 4];
    let mut len = 0usize;
    while bits >= 8 as ::core::ffi::c_uint && len < bytes.len() {
        bytes[len] = hold as ::core::ffi::c_uchar;
        len += 1;
        hold >>= 8 as ::core::ffi::c_int;
        bits = bits.wrapping_sub(8 as ::core::ffi::c_uint);
    }

    InflateSyncDrain {
        hold,
        bits,
        bytes,
        len,
    }
}

#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_uint = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut in_0: ::core::ffi::c_ulong = 0;
    let mut out: ::core::ffi::c_ulong = 0;
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    {
        let strm_ref = &mut *strm;
        let state_ref = &mut *state;
        if strm_ref.avail_in == 0 as crate::stdlib::uInt
            && state_ref.bits < 8 as ::core::ffi::c_uint
        {
            return crate::zlib_h::Z_BUF_ERROR;
        }
        if state_ref.mode as ::core::ffi::c_uint
            != crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            state_ref.mode = crate::src::inflate::SYNC;
            let drain = inflate_sync_drain_pending_bytes(state_ref.hold, state_ref.bits);
            state_ref.hold = drain.hold;
            state_ref.bits = drain.bits;
            state_ref.have = 0 as ::core::ffi::c_uint;
            let (have, _) = syncsearch(state_ref.have, &drain.bytes[..drain.len]);
            state_ref.have = have;
        }
        let input = if strm_ref.avail_in == 0 as crate::stdlib::uInt {
            &[][..]
        } else {
            ::core::slice::from_raw_parts(strm_ref.next_in, strm_ref.avail_in as usize)
        };
        let (have, consumed) = syncsearch(state_ref.have, input);
        state_ref.have = have;
        len = consumed;
        strm_ref.avail_in = strm_ref.avail_in.wrapping_sub(len);
        strm_ref.next_in = strm_ref.next_in.offset(len as isize);
        strm_ref.total_in = strm_ref.total_in.wrapping_add(len as crate::stdlib::uLong);
        if state_ref.have != 4 as ::core::ffi::c_uint {
            return crate::zlib_h::Z_DATA_ERROR;
        }
        if state_ref.flags == -1 as ::core::ffi::c_int {
            state_ref.wrap = 0 as ::core::ffi::c_int;
        } else {
            state_ref.wrap &= !(4 as ::core::ffi::c_int);
        }
        flags = state_ref.flags;
        in_0 = strm_ref.total_in as ::core::ffi::c_ulong;
        out = strm_ref.total_out as ::core::ffi::c_ulong;
    }
    {
        let strm_ref = &mut *strm;
        let state_ref = &mut *state;
        inflate_reset_window_state(state_ref);
        inflate_reset_keep_state(strm_ref, state_ref);
        strm_ref.total_in = in_0 as crate::stdlib::uLong;
        strm_ref.total_out = out as crate::stdlib::uLong;
        state_ref.flags = flags;
        state_ref.mode = crate::src::inflate::TYPE;
    }
    return crate::zlib_h::Z_OK;
}
pub fn inflateSyncPoint(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_int {
    return (state.mode as ::core::ffi::c_uint
        == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint
        && state.bits == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[export_name = "inflateSyncPoint"]

pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*((*strm).state as *mut crate::src::inflate::inflate_state);
    inflateSyncPoint(state)
}
#[export_name = "inflateCopy"]
pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if inflate_state_check_raw!(source) != 0 || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*source).state as *mut crate::src::inflate::inflate_state;
    copy = Some((*source).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*source).opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if copy.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !(*state).window.is_null() {
        window = Some((*source).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*source).opaque,
            (1 as crate::stdlib::uInt) << (*state).wbits,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if window.is_null() {
            Some((*source).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*source).opaque,
                copy as crate::stdlib::voidpf,
            );
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    ::core::ptr::write(dest, *source);
    ::core::ptr::write(copy, *state);
    (*copy).strm = dest;
    if (*state).lencode
        >= &raw mut (*state).codes as *mut crate::src::inftrees::code
            as *const crate::src::inftrees::code
        && (*state).lencode
            <= (&raw mut (*state).codes as *mut crate::src::inftrees::code)
                .offset(crate::src::inftrees::ENOUGH as isize)
                .offset(-(1 as ::core::ffi::c_int as isize))
                as *const crate::src::inftrees::code
    {
        (*copy).lencode = (&raw mut (*copy).codes as *mut crate::src::inftrees::code).offset(
            (*state)
                .lencode
                .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
                as ::core::ffi::c_long as isize,
        );
        (*copy).distcode = (&raw mut (*copy).codes as *mut crate::src::inftrees::code).offset(
            (*state)
                .distcode
                .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
                as ::core::ffi::c_long as isize,
        );
    }
    (*copy).next = (&raw mut (*copy).codes as *mut crate::src::inftrees::code).offset(
        (*state)
            .next
            .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
            as ::core::ffi::c_long as isize,
    );
    if !window.is_null() && (*state).whave != 0 {
        let len = (*state).whave as usize;
        let source_window = ::core::slice::from_raw_parts((*state).window, len);
        let dest_window = ::core::slice::from_raw_parts_mut(window, len);
        dest_window.copy_from_slice(source_window);
    }
    (*copy).window = window;
    (*dest).state = copy as *mut crate::src::deflate::internal_state;
    return crate::zlib_h::Z_OK;
}
pub fn inflateUndermine(
    state: &mut crate::src::inflate::inflate_state,
    _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    state.sane = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_DATA_ERROR;
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    inflateUndermine(state, subvert)
}
pub fn inflateValidate(
    state: &mut crate::src::inflate::inflate_state,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if check != 0 && state.wrap != 0 {
        state.wrap |= 4 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    inflateValidate(state, check)
}
fn inflate_mark_progress(
    mode: crate::src::inflate::inflate_mode,
    length: ::core::ffi::c_uint,
    was: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if mode == crate::src::inflate::COPY_1 {
        length
    } else if mode == crate::src::inflate::MATCH {
        was.wrapping_sub(length)
    } else {
        0 as ::core::ffi::c_uint
    }
}

pub fn inflateMark(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_long {
    return ((state.back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int)
        as ::core::ffi::c_long
        + inflate_mark_progress(state.mode, state.length, state.was) as ::core::ffi::c_long;
}
#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    if inflate_state_check_raw!(strm) != 0 {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    }
    let state = &*((*strm).state as *mut crate::src::inflate::inflate_state);
    inflateMark(state)
}
#[export_name = "inflateCodesUsed"]
pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflate_state_check_raw!(strm) != 0 {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    return (*state)
        .next
        .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
        as ::core::ffi::c_long as ::core::ffi::c_ulong;
}
