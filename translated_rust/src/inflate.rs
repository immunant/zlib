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

const CODE_LENGTH_ORDER: [::core::ffi::c_ushort; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];
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

pub use crate::src::adler32::adler32;
pub use crate::src::crc32::crc32;
pub use crate::src::deflate::internal_state;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::inflate_table;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::ENOUGH;
pub use crate::src::inftrees::ENOUGH_DISTS;
pub use crate::src::inftrees::ENOUGH_LENS;
pub use crate::src::inftrees::LENS;
pub use crate::src::zutil::zcalloc_ffi;
pub use crate::src::zutil::zcfree_ffi;

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

fn inflate_mode_is_valid(mode: inflate_mode) -> bool {
    mode >= HEAD && mode <= SYNC
}

fn inflate_state_metadata_is_valid(stream_matches: bool, mode: inflate_mode) -> bool {
    stream_matches && inflate_mode_is_valid(mode)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct DynamicHeaderCounts {
    pub nlen: ::core::ffi::c_uint,
    pub ndist: ::core::ffi::c_uint,
    pub ncode: ::core::ffi::c_uint,
}

impl DynamicHeaderCounts {
    pub(crate) fn is_valid(self) -> bool {
        self.nlen <= 286 && self.ndist <= 30
    }
}

pub(crate) fn dynamic_header_counts(low_14_bits: ::core::ffi::c_uint) -> DynamicHeaderCounts {
    DynamicHeaderCounts {
        nlen: (low_14_bits & 0x1f).wrapping_add(257),
        ndist: ((low_14_bits >> 5) & 0x1f).wrapping_add(1),
        ncode: ((low_14_bits >> 10) & 0x0f).wrapping_add(4),
    }
}

fn dynamic_code_length_repeat_fits(
    have: ::core::ffi::c_uint,
    repeat: ::core::ffi::c_uint,
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
) -> bool {
    have.wrapping_add(repeat) <= nlen.wrapping_add(ndist)
}

fn stored_block_lengths_are_valid(hold: crate::stdlib::uLong) -> bool {
    hold & 0xffff as crate::stdlib::uLong
        == hold >> 16 as ::core::ffi::c_int ^ 0xffff as crate::stdlib::uLong
}

#[derive(Debug, PartialEq, Eq)]
enum InflatePrimeUpdate {
    Keep,
    Clear,
    Set {
        hold: crate::stdlib::uLong,
        bits: ::core::ffi::c_uint,
    },
    StreamError,
}

fn inflate_prime_update(
    hold: crate::stdlib::uLong,
    current_bits: ::core::ffi::c_uint,
    requested_bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> InflatePrimeUpdate {
    if requested_bits == 0 {
        return InflatePrimeUpdate::Keep;
    }
    if requested_bits < 0 {
        return InflatePrimeUpdate::Clear;
    }

    let requested_bits = requested_bits as ::core::ffi::c_uint;
    let new_bits = current_bits.wrapping_add(requested_bits);
    if requested_bits > 16 || new_bits > 32 {
        return InflatePrimeUpdate::StreamError;
    }

    let value_mask = (1_i64 << requested_bits) - 1;
    let masked_value = (value as i64 & value_mask) as crate::stdlib::uLong;
    InflatePrimeUpdate::Set {
        hold: hold.wrapping_add(masked_value << current_bits),
        bits: new_bits,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct WindowUpdate {
    replace: bool,
    first: ::core::ffi::c_uint,
    second: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct WindowMetadata {
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
}

fn initial_window_metadata(wbits: ::core::ffi::c_uint) -> WindowMetadata {
    WindowMetadata {
        wsize: (1 as ::core::ffi::c_uint) << wbits,
        wnext: 0,
        whave: 0,
    }
}

fn window_update_plan(
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    copy: ::core::ffi::c_uint,
) -> WindowUpdate {
    if copy >= wsize {
        return WindowUpdate {
            replace: true,
            first: wsize,
            second: 0,
            wnext: 0,
            whave: wsize,
        };
    }

    let first = wsize.wrapping_sub(wnext).min(copy);
    let second = copy.wrapping_sub(first);
    if second != 0 {
        WindowUpdate {
            replace: false,
            first,
            second,
            wnext: second,
            whave: wsize,
        }
    } else {
        let next = wnext.wrapping_add(first);
        WindowUpdate {
            replace: false,
            first,
            second,
            wnext: if next == wsize { 0 } else { next },
            whave: if whave < wsize {
                whave.wrapping_add(first)
            } else {
                whave
            },
        }
    }
}

fn apply_window_update(
    window: &mut [crate::stdlib::Bytef],
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    produced: &[crate::stdlib::Bytef],
) -> WindowUpdate {
    let plan = window_update_plan(
        window.len() as ::core::ffi::c_uint,
        wnext,
        whave,
        produced.len() as ::core::ffi::c_uint,
    );

    if plan.replace {
        let start = produced.len() - window.len();
        window.copy_from_slice(&produced[start..]);
    } else {
        let first = plan.first as usize;
        let second = plan.second as usize;
        let copy = produced.len();
        let next = wnext as usize;

        window[next..next + first].copy_from_slice(&produced[..first]);
        if second != 0 {
            window[..second].copy_from_slice(&produced[copy - second..]);
        }
    }

    plan
}

fn inflate_mark_value(
    back: ::core::ffi::c_int,
    mode: inflate_mode,
    length: crate::stdlib::uInt,
    was: crate::stdlib::uInt,
) -> ::core::ffi::c_long {
    (((back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int)
        as ::core::ffi::c_long)
        + (if mode as ::core::ffi::c_uint
            == crate::src::inflate::COPY_1 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            length
        } else if mode as ::core::ffi::c_uint
            == crate::src::inflate::MATCH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            was.wrapping_sub(length)
        } else {
            0 as ::core::ffi::c_uint
        }) as ::core::ffi::c_long
}

fn inflate_reset2_params(
    window_bits: ::core::ffi::c_int,
) -> Option<(::core::ffi::c_int, ::core::ffi::c_uint)> {
    let (wrap, window_bits) = if window_bits < 0 {
        if window_bits < -15 {
            return None;
        }
        (0, -window_bits)
    } else {
        let wrap = (window_bits >> 4) + 5;
        let window_bits = if window_bits < 48 {
            window_bits & 15
        } else {
            window_bits
        };
        (wrap, window_bits)
    };

    if window_bits != 0 && !(8..=15).contains(&window_bits) {
        return None;
    }

    Some((wrap, window_bits as ::core::ffi::c_uint))
}

unsafe extern "C" fn inflateStateCheck(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as ::core::ffi::c_int;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if state.is_null() || !inflate_state_metadata_is_valid((*state).strm == strm, (*state).mode) {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn inflateResetKeep(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    (*state).total = 0 as ::core::ffi::c_ulong;
    (*strm).total_out = (*state).total as crate::stdlib::uLong;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*strm).data_type = 0 as ::core::ffi::c_int;
    if (*state).wrap != 0 {
        (*strm).adler = ((*state).wrap & 1 as ::core::ffi::c_int) as crate::stdlib::uLong;
    }
    (*state).mode = crate::src::inflate::HEAD;
    (*state).last = 0 as ::core::ffi::c_int;
    (*state).havedict = 0 as ::core::ffi::c_int;
    (*state).flags = -1 as ::core::ffi::c_int;
    (*state).dmax = 32768 as ::core::ffi::c_uint;
    (*state).head = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
    (*state).hold = 0 as ::core::ffi::c_ulong;
    (*state).bits = 0 as ::core::ffi::c_uint;
    (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
    (*state).distcode = (*state).next;
    (*state).lencode = (*state).distcode;
    (*state).sane = 1 as ::core::ffi::c_int;
    (*state).back = -1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateResetKeep(strm)
}
pub unsafe extern "C" fn inflateReset(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    (*state).wsize = 0 as ::core::ffi::c_uint;
    (*state).whave = 0 as ::core::ffi::c_uint;
    (*state).wnext = 0 as ::core::ffi::c_uint;
    return inflateResetKeep(strm);
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateReset(strm)
}
pub unsafe extern "C" fn inflateReset2(
    mut strm: crate::zlib_h::z_streamp,
    windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    let Some((wrap, window_bits)) = inflate_reset2_params(windowBits) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !(*state).window.is_null() && (*state).wbits != window_bits {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as crate::stdlib::voidpf,
        );
        (*state).window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    (*state).wrap = wrap;
    (*state).wbits = window_bits;
    return inflateReset(strm);
}
#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateReset2(strm, windowBits)
}
pub unsafe extern "C" fn inflateInit2_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if version.is_null()
        || *version.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
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
    crate::stdlib::memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>()
            as crate::__stddef_size_t_h::size_t,
    );
    (*strm).state = state as *mut crate::src::deflate::internal_state;
    (*state).strm = strm;
    (*state).window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*state).mode = crate::src::inflate::HEAD;
    ret = inflateReset2(strm, windowBits);
    if ret != crate::zlib_h::Z_OK {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            state as crate::stdlib::voidpf,
        );
        (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    }
    return ret;
}
#[export_name = "inflateInit2_"]

pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit2_(strm, windowBits, version, stream_size)
}
pub unsafe extern "C" fn inflateInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return inflateInit2_(strm, crate::zutil_h::DEF_WBITS, version, stream_size);
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit_(strm, version, stream_size)
}
pub unsafe extern "C" fn inflatePrime(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    match inflate_prime_update((*state).hold, (*state).bits, bits, value) {
        InflatePrimeUpdate::Keep => crate::zlib_h::Z_OK,
        InflatePrimeUpdate::Clear => {
            (*state).hold = 0;
            (*state).bits = 0;
            crate::zlib_h::Z_OK
        }
        InflatePrimeUpdate::Set { hold, bits } => {
            (*state).hold = hold;
            (*state).bits = bits;
            crate::zlib_h::Z_OK
        }
        InflatePrimeUpdate::StreamError => crate::zlib_h::Z_STREAM_ERROR,
    }
}
#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflatePrime(strm, bits, value)
}
unsafe extern "C" fn updatewindow(
    mut strm: crate::zlib_h::z_streamp,
    mut end: *const crate::stdlib::Bytef,
    mut copy: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let plan: WindowUpdate;
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).window.is_null() {
        (*state).window = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            (1 as crate::stdlib::uInt) << (*state).wbits,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if (*state).window.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    if (*state).wsize == 0 as ::core::ffi::c_uint {
        let metadata = initial_window_metadata((*state).wbits);
        (*state).wsize = metadata.wsize;
        (*state).wnext = metadata.wnext;
        (*state).whave = metadata.whave;
    }
    let window = core::slice::from_raw_parts_mut((*state).window, (*state).wsize as usize);
    let produced = if copy == 0 {
        &[]
    } else {
        core::slice::from_raw_parts(end.sub(copy as usize), copy as usize)
    };
    plan = apply_window_update(window, (*state).wnext, (*state).whave, produced);
    (*state).wnext = plan.wnext;
    (*state).whave = plan.whave;
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn inflate(
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
    if inflateStateCheck(strm) != 0
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
                        (*state).check = crate::src::crc32::crc32_ffi(
                            0 as crate::stdlib::uLong,
                            ::core::ptr::null::<crate::stdlib::Bytef>(),
                            0 as crate::stdlib::uInt,
                        ) as ::core::ffi::c_ulong;
                        hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                        hbuf[1 as ::core::ffi::c_int as usize] =
                            (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        (*state).check = crate::src::crc32::crc32_ffi(
                            (*state).check as crate::stdlib::uLong,
                            &raw mut hbuf as *mut ::core::ffi::c_uchar,
                            2 as crate::stdlib::uInt,
                        ) as ::core::ffi::c_ulong;
                        hold = 0 as ::core::ffi::c_ulong;
                        bits = 0 as ::core::ffi::c_uint;
                        (*state).mode = crate::src::inflate::FLAGS;
                        continue;
                    } else {
                        if !(*state).head.is_null() {
                            (*(*state).head).done = -1 as ::core::ffi::c_int;
                        }
                        if (*state).wrap & 1 as ::core::ffi::c_int == 0
                            || (((hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                << 8 as ::core::ffi::c_int)
                                as ::core::ffi::c_ulong)
                                .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                .wrapping_rem(31 as ::core::ffi::c_ulong)
                                != 0
                        {
                            (*strm).msg = b"incorrect header check\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                            continue;
                        } else if hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                            != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                        {
                            (*strm).msg = b"unknown compression method\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                            continue;
                        } else {
                            hold >>= 4 as ::core::ffi::c_int;
                            bits =
                                bits.wrapping_sub(4 as ::core::ffi::c_int as ::core::ffi::c_uint);
                            len = (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                            .wrapping_add(8 as ::core::ffi::c_uint);
                            if (*state).wbits == 0 as ::core::ffi::c_uint {
                                (*state).wbits = len;
                            }
                            if len > 15 as ::core::ffi::c_uint || len > (*state).wbits {
                                (*strm).msg = b"invalid window size\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            } else {
                                (*state).dmax = (1 as ::core::ffi::c_uint) << len;
                                (*state).flags = 0 as ::core::ffi::c_int;
                                (*state).check = crate::src::adler32::adler32_ffi(
                                    0 as crate::stdlib::uLong,
                                    ::core::ptr::null::<crate::stdlib::Bytef>(),
                                    0 as crate::stdlib::uInt,
                                )
                                    as ::core::ffi::c_ulong;
                                (*strm).adler = (*state).check as crate::stdlib::uLong;
                                (*state).mode = (if hold & 0x200 as ::core::ffi::c_ulong != 0 {
                                    crate::src::inflate::DICTID as ::core::ffi::c_int
                                } else {
                                    crate::src::inflate::TYPE as ::core::ffi::c_int
                                })
                                    as crate::src::inflate::inflate_mode;
                                hold = 0 as ::core::ffi::c_ulong;
                                bits = 0 as ::core::ffi::c_uint;
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
                    if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                        && (*state).wrap & 4 as ::core::ffi::c_int != 0
                    {
                        hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                        hbuf[1 as ::core::ffi::c_int as usize] =
                            (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        (*state).check = crate::src::crc32::crc32_ffi(
                            (*state).check as crate::stdlib::uLong,
                            &raw mut hbuf as *mut ::core::ffi::c_uchar,
                            2 as crate::stdlib::uInt,
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
                (*state).check = (hold >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_ulong)
                    .wrapping_add(hold >> 8 as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_ulong)
                    .wrapping_add(
                        (hold & 0xff00 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(
                        (hold & 0xff as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int,
                    );
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
                if !stored_block_lengths_are_valid(hold) {
                    (*strm).msg = b"invalid stored block lengths\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    (*state).length = hold as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint;
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    (*state).mode = crate::src::inflate::COPY_;
                    if flush == crate::zlib_h::Z_TREES {
                        break;
                    }
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
                let counts = dynamic_header_counts(hold as ::core::ffi::c_uint);
                (*state).nlen = counts.nlen;
                (*state).ndist = counts.ndist;
                (*state).ncode = counts.ncode;
                hold >>= 14 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(14 as ::core::ffi::c_uint);
                if !counts.is_valid() {
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
                        (*state).check = (if (*state).flags != 0 {
                            crate::src::crc32::crc32_ffi(
                                (*state).check as crate::stdlib::uLong,
                                put.offset(-(out as isize)),
                                out as crate::stdlib::uInt,
                            )
                        } else {
                            crate::src::adler32::adler32_ffi(
                                (*state).check as crate::stdlib::uLong,
                                put.offset(-(out as isize)),
                                out as crate::stdlib::uInt,
                            )
                        }) as ::core::ffi::c_ulong;
                        (*strm).adler = (*state).check as crate::stdlib::uLong;
                    }
                    out = left;
                    if (*state).wrap & 4 as ::core::ffi::c_int != 0
                        && (if (*state).flags != 0 {
                            hold
                        } else {
                            (hold >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_ulong)
                                .wrapping_add(
                                    hold >> 8 as ::core::ffi::c_int
                                        & 0xff00 as ::core::ffi::c_ulong,
                                )
                                .wrapping_add(
                                    (hold & 0xff00 as ::core::ffi::c_ulong)
                                        << 8 as ::core::ffi::c_int,
                                )
                                .wrapping_add(
                                    (hold & 0xff as ::core::ffi::c_ulong)
                                        << 24 as ::core::ffi::c_int,
                                )
                        }) != (*state).check
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
                        && hold != (*state).total & 0xffffffff as ::core::ffi::c_ulong
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
                    (*state).lens[CODE_LENGTH_ORDER[c2rust_fresh15 as usize] as usize] = (hold
                        as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_ushort;
                    hold >>= 3 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(3 as ::core::ffi::c_int as ::core::ffi::c_uint);
                }
                while (*state).have < 19 as ::core::ffi::c_uint {
                    let c2rust_fresh16 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[CODE_LENGTH_ORDER[c2rust_fresh16 as usize] as usize] =
                        0 as ::core::ffi::c_ushort;
                }
                let state = &mut *state;
                let mut table_cursor = 0usize;
                state.next = state.codes.as_mut_ptr();
                state.distcode = state.next as *const crate::src::inftrees::code;
                state.lencode = state.distcode;
                state.lenbits = 7 as ::core::ffi::c_uint;
                ret = crate::src::inftrees::inflate_table_safe(
                    crate::src::inftrees::CODES,
                    &state.lens[..19],
                    &mut state.codes,
                    &mut table_cursor,
                    &mut state.lenbits,
                    &mut state.work[..19],
                );
                if ret == 0 {
                    state.next = state.codes.as_mut_ptr().wrapping_add(table_cursor);
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
                (*state).check = crate::src::adler32::adler32_ffi(
                    0 as crate::stdlib::uLong,
                    ::core::ptr::null::<crate::stdlib::Bytef>(),
                    0 as crate::stdlib::uInt,
                ) as ::core::ffi::c_ulong;
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
                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                {
                    hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                    hbuf[1 as ::core::ffi::c_int as usize] =
                        (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    hbuf[2 as ::core::ffi::c_int as usize] =
                        (hold >> 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    hbuf[3 as ::core::ffi::c_int as usize] =
                        (hold >> 24 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    (*state).check = crate::src::crc32::crc32_ffi(
                        (*state).check as crate::stdlib::uLong,
                        &raw mut hbuf as *mut ::core::ffi::c_uchar,
                        4 as crate::stdlib::uInt,
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
                        if here.val as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
                            while bits
                                < (here.bits as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint
                            {
                                if have == 0 as ::core::ffi::c_uint {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh19 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add(
                                    (*c2rust_fresh19 as ::core::ffi::c_ulong) << bits,
                                );
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            hold >>= here.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                            if (*state).have == 0 as ::core::ffi::c_uint {
                                (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                break;
                            } else {
                                len = (*state).lens
                                    [(*state).have.wrapping_sub(1 as ::core::ffi::c_uint) as usize]
                                    as ::core::ffi::c_uint;
                                copy = (3 as ::core::ffi::c_uint).wrapping_add(
                                    hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                                            .wrapping_sub(1 as ::core::ffi::c_uint),
                                );
                                hold >>= 2 as ::core::ffi::c_int;
                                bits = bits
                                    .wrapping_sub(2 as ::core::ffi::c_int as ::core::ffi::c_uint);
                            }
                        } else if here.val as ::core::ffi::c_int == 17 as ::core::ffi::c_int {
                            while bits
                                < (here.bits as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint
                            {
                                if have == 0 as ::core::ffi::c_uint {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh20 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add(
                                    (*c2rust_fresh20 as ::core::ffi::c_ulong) << bits,
                                );
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            hold >>= here.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                            len = 0 as ::core::ffi::c_uint;
                            copy = (3 as ::core::ffi::c_uint).wrapping_add(
                                hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                        .wrapping_sub(1 as ::core::ffi::c_uint),
                            );
                            hold >>= 3 as ::core::ffi::c_int;
                            bits =
                                bits.wrapping_sub(3 as ::core::ffi::c_int as ::core::ffi::c_uint);
                        } else {
                            while bits
                                < (here.bits as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint
                            {
                                if have == 0 as ::core::ffi::c_uint {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh21 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add(
                                    (*c2rust_fresh21 as ::core::ffi::c_ulong) << bits,
                                );
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            hold >>= here.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                            len = 0 as ::core::ffi::c_uint;
                            copy = (11 as ::core::ffi::c_uint).wrapping_add(
                                hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int)
                                        .wrapping_sub(1 as ::core::ffi::c_uint),
                            );
                            hold >>= 7 as ::core::ffi::c_int;
                            bits =
                                bits.wrapping_sub(7 as ::core::ffi::c_int as ::core::ffi::c_uint);
                        }
                        if !dynamic_code_length_repeat_fits(
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
                if (*state).lens[256 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    (*strm).msg = b"invalid code -- missing end-of-block\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    let state = &mut *state;
                    let mut table_cursor = 0usize;
                    state.next = state.codes.as_mut_ptr();
                    state.lencode = state.next as *const crate::src::inftrees::code;
                    state.lenbits = 9 as ::core::ffi::c_uint;
                    ret = crate::src::inftrees::inflate_table_safe(
                        crate::src::inftrees::LENS,
                        &state.lens[..state.nlen as usize],
                        &mut state.codes,
                        &mut table_cursor,
                        &mut state.lenbits,
                        &mut state.work[..state.nlen as usize],
                    );
                    if ret == 0 {
                        state.next = state.codes.as_mut_ptr().wrapping_add(table_cursor);
                    }
                    if ret != 0 {
                        (*strm).msg = b"invalid literal/lengths set\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        state.distcode = state.next as *const crate::src::inftrees::code;
                        state.distbits = 6 as ::core::ffi::c_uint;
                        ret = crate::src::inftrees::inflate_table_safe(
                            crate::src::inftrees::DISTS,
                            &state.lens[state.nlen as usize
                                ..state.nlen.wrapping_add(state.ndist) as usize],
                            &mut state.codes,
                            &mut table_cursor,
                            &mut state.distbits,
                            &mut state.work[..state.ndist as usize],
                        );
                        if ret == 0 {
                            state.next = state.codes.as_mut_ptr().wrapping_add(table_cursor);
                        }
                        if ret != 0 {
                            (*strm).msg = b"invalid distances set\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                            continue;
                        } else {
                            (*state).mode = crate::src::inflate::LEN_;
                            if flush == crate::zlib_h::Z_TREES {
                                break;
                            }
                        }
                    }
                }
                c2rust_current_block = 8747825537946998525;
            }
            16745500758254703311 => {
                copy = (*state).length;
                if copy != 0 {
                    if copy > have {
                        copy = have;
                    }
                    if copy > left {
                        copy = left;
                    }
                    if copy == 0 as ::core::ffi::c_uint {
                        break;
                    }
                    crate::stdlib::memcpy(
                        put as *mut ::core::ffi::c_void,
                        next as *const ::core::ffi::c_void,
                        copy as crate::__stddef_size_t_h::size_t,
                    );
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
                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                {
                    hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                    hbuf[1 as ::core::ffi::c_int as usize] =
                        (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    (*state).check = crate::src::crc32::crc32_ffi(
                        (*state).check as crate::stdlib::uLong,
                        &raw mut hbuf as *mut ::core::ffi::c_uchar,
                        2 as crate::stdlib::uInt,
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
                    (*state).last = (hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_int;
                    hold >>= 1 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    match hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint)
                    {
                        0 => {
                            (*state).mode = crate::src::inflate::STORED;
                        }
                        1 => {
                            crate::src::inftrees::inflate_fixed(&mut *state);
                            (*state).mode = crate::src::inflate::LEN_;
                            if flush == crate::zlib_h::Z_TREES {
                                hold >>= 2 as ::core::ffi::c_int;
                                bits = bits
                                    .wrapping_sub(2 as ::core::ffi::c_int as ::core::ffi::c_uint);
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
                    hold >>= 2 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(2 as ::core::ffi::c_int as ::core::ffi::c_uint);
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
                    if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                        && (*state).wrap & 4 as ::core::ffi::c_int != 0
                    {
                        hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                        hbuf[1 as ::core::ffi::c_int as usize] =
                            (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        (*state).check = crate::src::crc32::crc32_ffi(
                            (*state).check as crate::stdlib::uLong,
                            &raw mut hbuf as *mut ::core::ffi::c_uchar,
                            2 as crate::stdlib::uInt,
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
                    crate::src::inffast::inflate_fast(strm as *mut crate::zlib_h::z_stream_s, out);
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
                            crate::stdlib::memcpy(
                                (*(*state).head).extra.offset(len as isize)
                                    as *mut ::core::ffi::c_void,
                                next as *const ::core::ffi::c_void,
                                (if len.wrapping_add(copy) > (*(*state).head).extra_max {
                                    ((*(*state).head).extra_max as ::core::ffi::c_uint)
                                        .wrapping_sub(len)
                                } else {
                                    copy
                                })
                                    as crate::__stddef_size_t_h::size_t,
                            );
                        }
                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                        {
                            (*state).check = crate::src::crc32::crc32_ffi(
                                (*state).check as crate::stdlib::uLong,
                                next,
                                copy as crate::stdlib::uInt,
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
                    (*state).length = (*state).length.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
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
                    copy = 0 as ::core::ffi::c_uint;
                    loop {
                        let c2rust_fresh5 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.offset(c2rust_fresh5 as isize) as ::core::ffi::c_uint;
                        if !(*state).head.is_null()
                            && !(*(*state).head).name.is_null()
                            && (*state).length < (*(*state).head).name_max
                        {
                            let c2rust_fresh6 = (*state).length;
                            (*state).length = (*state).length.wrapping_add(1);
                            *(*(*state).head).name.offset(c2rust_fresh6 as isize) =
                                len as crate::stdlib::Bytef;
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                        && (*state).wrap & 4 as ::core::ffi::c_int != 0
                    {
                        (*state).check = crate::src::crc32::crc32_ffi(
                            (*state).check as crate::stdlib::uLong,
                            next,
                            copy as crate::stdlib::uInt,
                        ) as ::core::ffi::c_ulong;
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if len != 0 {
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
                    copy = 0 as ::core::ffi::c_uint;
                    loop {
                        let c2rust_fresh7 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.offset(c2rust_fresh7 as isize) as ::core::ffi::c_uint;
                        if !(*state).head.is_null()
                            && !(*(*state).head).comment.is_null()
                            && (*state).length < (*(*state).head).comm_max
                        {
                            let c2rust_fresh8 = (*state).length;
                            (*state).length = (*state).length.wrapping_add(1);
                            *(*(*state).head).comment.offset(c2rust_fresh8 as isize) =
                                len as crate::stdlib::Bytef;
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                        && (*state).wrap & 4 as ::core::ffi::c_int != 0
                    {
                        (*state).check = crate::src::crc32::crc32_ffi(
                            (*state).check as crate::stdlib::uLong,
                            next,
                            copy as crate::stdlib::uInt,
                        ) as ::core::ffi::c_ulong;
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if len != 0 {
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
                    (*state).offset = (*state).offset.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
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
                        && hold != (*state).check & 0xffff as ::core::ffi::c_ulong
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
                (*state).check = crate::src::crc32::crc32_ffi(
                    0 as crate::stdlib::uLong,
                    ::core::ptr::null::<crate::stdlib::Bytef>(),
                    0 as crate::stdlib::uInt,
                ) as ::core::ffi::c_ulong;
                (*strm).adler = (*state).check as crate::stdlib::uLong;
                (*state).mode = crate::src::inflate::TYPE;
                continue;
            }
            _ => {}
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        copy = out.wrapping_sub(left);
        if (*state).offset > copy {
            copy = (*state).offset.wrapping_sub(copy);
            if copy > (*state).whave {
                if (*state).sane != 0 {
                    (*strm).msg = b"invalid distance too far back\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                }
            }
            if copy > (*state).wnext {
                copy = copy.wrapping_sub((*state).wnext);
                from = (*state)
                    .window
                    .offset((*state).wsize.wrapping_sub(copy) as isize);
            } else {
                from = (*state)
                    .window
                    .offset((*state).wnext.wrapping_sub(copy) as isize);
            }
            if copy > (*state).length {
                copy = (*state).length;
            }
        } else {
            from = put.offset(-((*state).offset as isize));
            copy = (*state).length;
        }
        if copy > left {
            copy = left;
        }
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
    if (*state).wsize != 0
        || out != (*strm).avail_out
            && ((*state).mode as ::core::ffi::c_uint)
                < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
            && (((*state).mode as ::core::ffi::c_uint)
                < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                || flush != crate::zlib_h::Z_FINISH)
    {
        if updatewindow(
            strm,
            (*strm).next_out,
            out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint),
        ) != 0
        {
            (*state).mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in as ::core::ffi::c_uint);
    out = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
    (*strm).total_in = (*strm).total_in.wrapping_add(in_0 as crate::stdlib::uLong);
    (*strm).total_out = (*strm).total_out.wrapping_add(out as crate::stdlib::uLong);
    (*state).total = (*state).total.wrapping_add(out as ::core::ffi::c_ulong);
    if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
        (*state).check = (if (*state).flags != 0 {
            crate::src::crc32::crc32_ffi(
                (*state).check as crate::stdlib::uLong,
                (*strm).next_out.offset(-(out as isize)),
                out as crate::stdlib::uInt,
            )
        } else {
            crate::src::adler32::adler32_ffi(
                (*state).check as crate::stdlib::uLong,
                (*strm).next_out.offset(-(out as isize)),
                out as crate::stdlib::uInt,
            )
        }) as ::core::ffi::c_ulong;
        (*strm).adler = (*state).check as crate::stdlib::uLong;
    }
    (*strm).data_type = inflate_data_type_value((*state).bits, (*state).last, (*state).mode);
    if inflate_needs_buffer_error(in_0, out, flush, ret) {
        ret = crate::zlib_h::Z_BUF_ERROR;
    }
    return ret;
}
#[export_name = "inflate"]

pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate(strm, flush)
}
pub unsafe extern "C" fn inflateEnd(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if !(*state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as crate::stdlib::voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as crate::stdlib::voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    inflateEnd(strm)
}
pub unsafe extern "C" fn inflateGetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).whave != 0 && !dictionary.is_null() {
        crate::stdlib::memcpy(
            dictionary as *mut ::core::ffi::c_void,
            (*state).window.offset((*state).wnext as isize) as *const ::core::ffi::c_void,
            (*state).whave.wrapping_sub((*state).wnext) as crate::__stddef_size_t_h::size_t,
        );
        crate::stdlib::memcpy(
            dictionary
                .offset((*state).whave as isize)
                .offset(-((*state).wnext as isize)) as *mut ::core::ffi::c_void,
            (*state).window as *const ::core::ffi::c_void,
            (*state).wnext as crate::__stddef_size_t_h::size_t,
        );
    }
    if !dictLength.is_null() {
        *dictLength = (*state).whave as crate::stdlib::uInt;
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    inflateGetDictionary(strm, dictionary, dictLength)
}
pub unsafe extern "C" fn inflateSetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut dictid: ::core::ffi::c_ulong = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).wrap != 0 as ::core::ffi::c_int
        && (*state).mode as ::core::ffi::c_uint
            != crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).mode as ::core::ffi::c_uint
        == crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        dictid = crate::src::adler32::adler32_ffi(
            0 as crate::stdlib::uLong,
            ::core::ptr::null::<crate::stdlib::Bytef>(),
            0 as crate::stdlib::uInt,
        ) as ::core::ffi::c_ulong;
        dictid = crate::src::adler32::adler32_ffi(
            dictid as crate::stdlib::uLong,
            dictionary,
            dictLength,
        ) as ::core::ffi::c_ulong;
        if dictid != (*state).check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }
    ret = updatewindow(
        strm,
        dictionary.offset(dictLength as isize),
        dictLength as ::core::ffi::c_uint,
    );
    if ret != 0 {
        (*state).mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*state).havedict = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSetDictionary"]

pub unsafe extern "C" fn inflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    inflateSetDictionary(strm, dictionary, dictLength)
}
fn inflate_header_wrap_allows_capture(wrap: ::core::ffi::c_int) -> bool {
    wrap & 2 as ::core::ffi::c_int != 0
}

pub unsafe extern "C" fn inflateGetHeader(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if !inflate_header_wrap_allows_capture((*state).wrap) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*state).head = head;
    (*head).done = 0 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    inflateGetHeader(strm, head)
}
fn inflate_data_type_value(
    bits: ::core::ffi::c_uint,
    last: ::core::ffi::c_int,
    mode: inflate_mode,
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

fn inflate_needs_buffer_error(
    consumed: ::core::ffi::c_uint,
    produced: ::core::ffi::c_uint,
    flush: ::core::ffi::c_int,
    result: ::core::ffi::c_int,
) -> bool {
    (consumed == 0 && produced == 0 || flush == crate::zlib_h::Z_FINISH)
        && result == crate::zlib_h::Z_OK
}

fn syncsearch_safe(have: &mut ::core::ffi::c_uint, buf: &[::core::ffi::c_uchar]) -> usize {
    let mut got = *have;
    let mut next = 0_usize;
    while next < buf.len() && got < 4 {
        if buf[next] as ::core::ffi::c_int == (if got < 2 { 0 } else { 0xff }) {
            got = got.wrapping_add(1);
        } else if buf[next] != 0 {
            got = 0;
        } else {
            got = 4_u32.wrapping_sub(got);
        }
        next += 1;
    }
    *have = got;
    next
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InflateSyncSearch {
    BufferError,
    DataError { consumed: usize },
    MarkerFound { consumed: usize },
}

fn inflate_sync_search_core(
    mode: &mut crate::src::inflate::inflate_mode,
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
    have: &mut ::core::ffi::c_uint,
    input: &[::core::ffi::c_uchar],
) -> InflateSyncSearch {
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    if input.is_empty() && *bits < 8 {
        return InflateSyncSearch::BufferError;
    }
    if *mode as ::core::ffi::c_uint
        != crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *mode = crate::src::inflate::SYNC;
        *hold >>= *bits & 7;
        *bits = (*bits).wrapping_sub(*bits & 7);
        let mut length = 0;
        while *bits >= 8 {
            buf[length] = *hold as ::core::ffi::c_uchar;
            length += 1;
            *hold >>= 8;
            *bits = (*bits).wrapping_sub(8);
        }
        *have = 0;
        syncsearch_safe(have, &buf[..length]);
    }

    let consumed = syncsearch_safe(have, input);
    if *have != 4 {
        InflateSyncSearch::DataError { consumed }
    } else {
        InflateSyncSearch::MarkerFound { consumed }
    }
}

#[export_name = "inflateSync"]
pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let state = (*strm).state as *mut crate::src::inflate::inflate_state;
    let input = if (*strm).avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts((*strm).next_in, (*strm).avail_in as usize)
    };
    let result = inflate_sync_search_core(
        &mut (*state).mode,
        &mut (*state).hold,
        &mut (*state).bits,
        &mut (*state).have,
        input,
    );
    let consumed = match result {
        InflateSyncSearch::BufferError => return crate::zlib_h::Z_BUF_ERROR,
        InflateSyncSearch::DataError { consumed } | InflateSyncSearch::MarkerFound { consumed } => {
            consumed
        }
    };
    (*strm).avail_in = (*strm)
        .avail_in
        .wrapping_sub(consumed as crate::stdlib::uInt);
    (*strm).next_in = (*strm).next_in.offset(consumed as isize);
    (*strm).total_in = (*strm)
        .total_in
        .wrapping_add(consumed as crate::stdlib::uLong);
    if let InflateSyncSearch::DataError { .. } = result {
        return crate::zlib_h::Z_DATA_ERROR;
    }

    if (*state).flags == -1 {
        (*state).wrap = 0;
    } else {
        (*state).wrap &= !4;
    }
    let flags = (*state).flags;
    let input_total = (*strm).total_in;
    let output_total = (*strm).total_out;
    inflateReset(strm);
    (*strm).total_in = input_total;
    (*strm).total_out = output_total;
    (*state).flags = flags;
    (*state).mode = crate::src::inflate::TYPE;
    crate::zlib_h::Z_OK
}
fn inflate_sync_point_value(
    mode: ::core::ffi::c_uint,
    bits: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    (mode == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint && bits == 0)
        as ::core::ffi::c_int
}

pub unsafe extern "C" fn inflateSyncPoint(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    inflate_sync_point_value((*state).mode as ::core::ffi::c_uint, (*state).bits)
}

#[export_name = "inflateSyncPoint"]
pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateSyncPoint(strm)
}
pub unsafe extern "C" fn inflateCopy(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if inflateStateCheck(source) != 0 || dest.is_null() {
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
    crate::stdlib::memset(
        copy as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>()
            as crate::__stddef_size_t_h::size_t,
    );
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
    crate::stdlib::memcpy(
        dest as *mut ::core::ffi::c_void,
        source as *const ::core::ffi::c_void,
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as crate::__stddef_size_t_h::size_t,
    );
    crate::stdlib::memcpy(
        copy as *mut ::core::ffi::c_void,
        state as *const ::core::ffi::c_void,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>()
            as crate::__stddef_size_t_h::size_t,
    );
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
    if !window.is_null() {
        crate::stdlib::memcpy(
            window as *mut ::core::ffi::c_void,
            (*state).window as *const ::core::ffi::c_void,
            (*state).whave as crate::__stddef_size_t_h::size_t,
        );
    }
    (*copy).window = window;
    (*dest).state = copy as *mut crate::src::deflate::internal_state;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateCopy"]

pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateCopy(dest, source)
}
pub unsafe extern "C" fn inflateUndermine(
    mut strm: crate::zlib_h::z_streamp,
    mut _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    (*state).sane = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_DATA_ERROR;
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateUndermine(strm, subvert)
}
fn inflate_validate_wrap(
    wrap: ::core::ffi::c_int,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if check != 0 && wrap != 0 {
        wrap | 4 as ::core::ffi::c_int
    } else {
        wrap & !(4 as ::core::ffi::c_int)
    }
}

pub unsafe extern "C" fn inflateValidate(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    (*state).wrap = inflate_validate_wrap((*state).wrap, check);
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateValidate(strm, check)
}
pub unsafe extern "C" fn inflateMark(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_long {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    return inflate_mark_value((*state).back, (*state).mode, (*state).length, (*state).was);
}
#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    inflateMark(strm)
}
pub unsafe extern "C" fn inflateCodesUsed(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    return (*state)
        .next
        .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
        as ::core::ffi::c_long as ::core::ffi::c_ulong;
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    inflateCodesUsed(strm)
}

#[cfg(test)]
mod tests {
    use super::{
        apply_window_update, dynamic_code_length_repeat_fits, dynamic_header_counts,
        inflate_data_type_value, inflate_header_wrap_allows_capture, inflate_mark_value,
        inflate_mode_is_valid, inflate_needs_buffer_error, inflate_prime_update,
        inflate_reset2_params, inflate_state_metadata_is_valid, inflate_sync_point_value,
        inflate_sync_search_core, inflate_validate_wrap, initial_window_metadata,
        stored_block_lengths_are_valid, syncsearch_safe, window_update_plan, InflatePrimeUpdate,
        InflateSyncSearch, BAD, CODE_LENGTH_ORDER, COPY_, COPY_1, HEAD, LEN_, MATCH, STORED, SYNC,
        TYPE,
    };

    #[test]
    fn inflate_data_type_value_sets_expected_flags() {
        assert_eq!(inflate_data_type_value(5, 0, HEAD), 5);
        assert_eq!(inflate_data_type_value(5, 1, HEAD), 5 + 64);
        assert_eq!(inflate_data_type_value(5, 0, TYPE), 5 + 128);
        assert_eq!(inflate_data_type_value(5, 0, LEN_), 5 + 256);
        assert_eq!(inflate_data_type_value(5, 0, COPY_), 5 + 256);
    }

    #[test]
    fn inflate_buffer_error_predicate_matches_progress_and_finish_rules() {
        assert!(inflate_needs_buffer_error(0, 0, 0, crate::zlib_h::Z_OK));
        assert!(!inflate_needs_buffer_error(1, 0, 0, crate::zlib_h::Z_OK));
        assert!(!inflate_needs_buffer_error(0, 1, 0, crate::zlib_h::Z_OK));
        assert!(inflate_needs_buffer_error(
            1,
            1,
            crate::zlib_h::Z_FINISH,
            crate::zlib_h::Z_OK
        ));
        assert!(!inflate_needs_buffer_error(
            0,
            0,
            crate::zlib_h::Z_FINISH,
            crate::zlib_h::Z_STREAM_END
        ));
    }

    #[test]
    fn stored_block_lengths_require_complementary_nlen() {
        assert!(stored_block_lengths_are_valid(0xedcb_1234));
        assert!(stored_block_lengths_are_valid(0xffff_0000));
        assert!(!stored_block_lengths_are_valid(0x1234_1234));
        assert!(!stored_block_lengths_are_valid(0x0000_0001));
    }

    #[test]
    fn inflate_validate_wrap_updates_only_the_validation_bit() {
        assert_eq!(inflate_validate_wrap(1, 1), 5);
        assert_eq!(inflate_validate_wrap(4, -1), 4);
        assert_eq!(inflate_validate_wrap(9, 0), 9);
        assert_eq!(inflate_validate_wrap(0, 1), 0);
    }

    #[test]
    fn inflate_header_wrap_requires_gzip_capture_bit() {
        assert!(!inflate_header_wrap_allows_capture(0));
        assert!(inflate_header_wrap_allows_capture(2));
        assert!(inflate_header_wrap_allows_capture(3));
        assert!(!inflate_header_wrap_allows_capture(4));
    }

    #[test]
    fn inflate_sync_point_value_requires_stored_mode_without_pending_bits() {
        assert_eq!(inflate_sync_point_value(STORED as u32, 0), 1);
        assert_eq!(inflate_sync_point_value(STORED as u32, 1), 0);
        assert_eq!(inflate_sync_point_value(HEAD as u32, 0), 0);
        assert_eq!(inflate_sync_point_value(u32::MAX, 0), 0);
    }

    #[test]
    fn code_length_order_matches_deflate_spec() {
        assert_eq!(
            CODE_LENGTH_ORDER,
            [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15]
        );
    }

    #[test]
    fn dynamic_header_counts_decode_the_valid_range() {
        assert_eq!(
            dynamic_header_counts(0),
            super::DynamicHeaderCounts {
                nlen: 257,
                ndist: 1,
                ncode: 4,
            }
        );
        assert_eq!(
            dynamic_header_counts(29 | (29 << 5) | (15 << 10)),
            super::DynamicHeaderCounts {
                nlen: 286,
                ndist: 30,
                ncode: 19,
            }
        );
    }

    #[test]
    fn dynamic_header_counts_reject_invalid_length_or_distance_counts() {
        assert!(!dynamic_header_counts(30).is_valid());
        assert!(!dynamic_header_counts(30 << 5).is_valid());
        assert!(dynamic_header_counts(15 << 10).is_valid());
    }

    #[test]
    fn dynamic_code_length_repeat_fits_accepts_only_available_entries() {
        assert!(dynamic_code_length_repeat_fits(3, 2, 4, 1));
        assert!(!dynamic_code_length_repeat_fits(3, 3, 4, 1));
        assert!(dynamic_code_length_repeat_fits(
            ::core::ffi::c_uint::MAX,
            1,
            ::core::ffi::c_uint::MAX,
            1,
        ));
    }

    #[test]
    fn inflate_prime_update_preserves_reset_and_zero_bit_requests() {
        assert_eq!(
            inflate_prime_update(0x1234, 12, 0, 99),
            InflatePrimeUpdate::Keep
        );
        assert_eq!(
            inflate_prime_update(0x1234, 12, -1, 99),
            InflatePrimeUpdate::Clear
        );
    }

    #[test]
    fn inflate_prime_update_masks_value_and_appends_bits() {
        assert_eq!(
            inflate_prime_update(0b101, 3, 4, 0b1_1110),
            InflatePrimeUpdate::Set {
                hold: 0b111_0101,
                bits: 7,
            }
        );
    }

    #[test]
    fn inflate_prime_update_rejects_oversized_requests() {
        assert_eq!(
            inflate_prime_update(0, 16, 17, 0),
            InflatePrimeUpdate::StreamError
        );
        assert_eq!(
            inflate_prime_update(0, 20, 16, 0),
            InflatePrimeUpdate::StreamError
        );
    }

    #[test]
    fn inflate_reset2_params_decodes_raw_and_wrapped_windows() {
        assert_eq!(inflate_reset2_params(-15), Some((0, 15)));
        assert_eq!(inflate_reset2_params(0), Some((5, 0)));
        assert_eq!(inflate_reset2_params(15), Some((5, 15)));
        assert_eq!(inflate_reset2_params(31), Some((6, 15)));
        assert_eq!(inflate_reset2_params(32), Some((7, 0)));
    }

    #[test]
    fn inflate_reset2_params_rejects_invalid_windows() {
        assert_eq!(inflate_reset2_params(-16), None);
        assert_eq!(inflate_reset2_params(7), None);
        assert_eq!(inflate_reset2_params(48), None);
    }

    #[test]
    fn syncsearch_preserves_partial_marker_across_chunks() {
        let mut have = 0;
        assert_eq!(syncsearch_safe(&mut have, &[0, 0]), 2);
        assert_eq!(have, 2);
        assert_eq!(syncsearch_safe(&mut have, &[0xff, 0xff]), 2);
        assert_eq!(have, 4);
    }

    #[test]
    fn syncsearch_accepts_every_marker_split_and_empty_chunks() {
        let marker = [0, 0, 0xff, 0xff];
        for split in 0..=marker.len() {
            let mut have = 0;
            assert_eq!(syncsearch_safe(&mut have, &marker[..split]), split);
            assert_eq!(syncsearch_safe(&mut have, &[]), 0);
            assert_eq!(
                syncsearch_safe(&mut have, &marker[split..]),
                marker.len() - split
            );
            assert_eq!(have, 4, "split={split}");
        }
    }

    #[test]
    fn syncsearch_restarts_after_non_marker_bytes() {
        let mut have = 3;
        assert_eq!(syncsearch_safe(&mut have, &[1, 0, 0, 0xff, 0xff]), 5);
        assert_eq!(have, 4);
    }

    #[test]
    fn inflate_sync_core_carries_marker_from_pending_bits_into_input() {
        let mut mode = HEAD;
        let mut hold = 0;
        let mut bits = 16;
        let mut have = 0;

        assert_eq!(
            inflate_sync_search_core(&mut mode, &mut hold, &mut bits, &mut have, &[0xff, 0xff]),
            InflateSyncSearch::MarkerFound { consumed: 2 }
        );
        assert_eq!(mode, SYNC);
        assert_eq!(bits, 0);
        assert_eq!(have, 4);
    }

    #[test]
    fn inflate_sync_core_reports_buffer_error_without_input_or_pending_bytes() {
        let mut mode = HEAD;
        let mut hold = 0;
        let mut bits = 0;
        let mut have = 0;

        assert_eq!(
            inflate_sync_search_core(&mut mode, &mut hold, &mut bits, &mut have, &[]),
            InflateSyncSearch::BufferError
        );
        assert_eq!(mode, HEAD);
        assert_eq!(have, 0);
    }

    #[test]
    fn inflate_mode_validation_accepts_only_known_range() {
        assert!(inflate_mode_is_valid(HEAD));
        assert!(inflate_mode_is_valid(BAD));
        assert!(inflate_mode_is_valid(SYNC));
        assert!(!inflate_mode_is_valid(HEAD - 1));
        assert!(!inflate_mode_is_valid(SYNC + 1));
    }

    #[test]
    fn inflate_state_metadata_requires_matching_stream_and_valid_mode() {
        assert!(inflate_state_metadata_is_valid(true, HEAD));
        assert!(!inflate_state_metadata_is_valid(false, HEAD));
        assert!(!inflate_state_metadata_is_valid(true, SYNC + 1));
    }

    #[test]
    fn inflate_mark_value_preserves_mode_specific_progress() {
        assert_eq!(inflate_mark_value(2, COPY_1, 7, 99), (2 << 16) + 7);
        assert_eq!(inflate_mark_value(2, MATCH, 7, 10), (2 << 16) + 3);
        assert_eq!(inflate_mark_value(-1, HEAD, 7, 10), -(1 << 16));
        assert_eq!(
            inflate_mark_value(0, MATCH, 5, 2),
            (2_u32.wrapping_sub(5)) as ::core::ffi::c_long
        );
    }

    #[test]
    fn window_update_plan_handles_replace_append_and_wrap() {
        assert_eq!(
            window_update_plan(8, 3, 5, 8),
            super::WindowUpdate {
                replace: true,
                first: 8,
                second: 0,
                wnext: 0,
                whave: 8,
            }
        );
        assert_eq!(
            window_update_plan(8, 3, 5, 2),
            super::WindowUpdate {
                replace: false,
                first: 2,
                second: 0,
                wnext: 5,
                whave: 7,
            }
        );
        assert_eq!(
            window_update_plan(8, 6, 8, 4),
            super::WindowUpdate {
                replace: false,
                first: 2,
                second: 2,
                wnext: 2,
                whave: 8,
            }
        );
    }

    #[test]
    fn initial_window_metadata_resets_history_positions() {
        assert_eq!(
            initial_window_metadata(8),
            super::WindowMetadata {
                wsize: 256,
                wnext: 0,
                whave: 0,
            }
        );
        assert_eq!(
            initial_window_metadata(15),
            super::WindowMetadata {
                wsize: 32_768,
                wnext: 0,
                whave: 0,
            }
        );
    }

    #[test]
    fn window_update_copies_replace_append_and_wrap_data() {
        let mut replace_window = [0; 8];
        let replace = apply_window_update(&mut replace_window, 3, 5, b"0123456789");
        assert_eq!(replace.wnext, 0);
        assert_eq!(replace.whave, 8);
        assert_eq!(replace_window, *b"23456789");

        let mut append_window = *b"abcdefgh";
        let append = apply_window_update(&mut append_window, 3, 5, b"XY");
        assert_eq!(append.wnext, 5);
        assert_eq!(append.whave, 7);
        assert_eq!(append_window, *b"abcXYfgh");

        let mut wrap_window = *b"abcdefgh";
        let wrap = apply_window_update(&mut wrap_window, 6, 8, b"WXYZ");
        assert_eq!(wrap.wnext, 2);
        assert_eq!(wrap.whave, 8);
        assert_eq!(wrap_window, *b"YZcdefWX");
    }

    #[test]
    fn window_update_ignores_empty_output() {
        let mut window = *b"abcdefgh";
        let update = apply_window_update(&mut window, 3, 5, b"");
        assert_eq!(update.wnext, 3);
        assert_eq!(update.whave, 5);
        assert_eq!(window, *b"abcdefgh");
    }
}
