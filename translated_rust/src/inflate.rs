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

fn inflate_mode_on_entry(mode: inflate_mode) -> inflate_mode {
    if mode == TYPE {
        TYPEDO
    } else {
        mode
    }
}

fn inflate_head_skip_mode(wrap: ::core::ffi::c_int) -> Option<inflate_mode> {
    if wrap == 0 {
        Some(TYPEDO)
    } else {
        None
    }
}

fn inflate_state_metadata_is_valid(stream_matches: bool, mode: inflate_mode) -> bool {
    stream_matches && inflate_mode_is_valid(mode)
}

fn inflate_stream_has_allocator_callbacks(has_zalloc: bool, has_zfree: bool) -> bool {
    has_zalloc && has_zfree
}

fn inflate_state_is_usable(
    has_zalloc: bool,
    has_zfree: bool,
    stream_matches: bool,
    mode: inflate_mode,
) -> bool {
    inflate_stream_has_allocator_callbacks(has_zalloc, has_zfree)
        && inflate_state_metadata_is_valid(stream_matches, mode)
}

fn inflate_state_check_result(
    has_stream: bool,
    has_state: bool,
    state_is_usable: bool,
) -> ::core::ffi::c_int {
    if has_stream && has_state && state_is_usable {
        0
    } else {
        1
    }
}

fn inflate_stream_buffers_are_valid(
    has_output: bool,
    has_input: bool,
    available_input: crate::stdlib::uInt,
) -> bool {
    has_output && (has_input || available_input == 0)
}

fn inflate_reset_keep_adler(wrap: ::core::ffi::c_int) -> Option<crate::stdlib::uLong> {
    if wrap == 0 {
        None
    } else {
        Some((wrap & 1) as crate::stdlib::uLong)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InflateZlibHeaderError {
    IncorrectCheck,
    UnknownCompressionMethod,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InflateOutputChecksum {
    Adler32,
    Crc32,
}

fn inflate_output_checksum(
    wrap: ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
    output_len: ::core::ffi::c_uint,
) -> Option<InflateOutputChecksum> {
    if wrap & 4 == 0 || output_len == 0 {
        None
    } else if flags != 0 {
        Some(InflateOutputChecksum::Crc32)
    } else {
        Some(InflateOutputChecksum::Adler32)
    }
}

fn inflate_is_gzip_header(wrap: ::core::ffi::c_int, hold: crate::stdlib::uLong) -> bool {
    wrap & 2 != 0 && hold == 0x8b1f as crate::stdlib::uLong
}

fn inflate_gzip_window_bits(wbits: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    if wbits == 0 {
        15
    } else {
        wbits
    }
}

fn inflate_zlib_header_error(
    wrap: ::core::ffi::c_int,
    hold: crate::stdlib::uLong,
) -> Option<InflateZlibHeaderError> {
    if wrap & 1 == 0 {
        return Some(InflateZlibHeaderError::IncorrectCheck);
    }

    let header = (((hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << 8).wrapping_sub(1))
        << 8) as crate::stdlib::uLong)
        .wrapping_add(hold >> 8);
    if header.wrapping_rem(31) != 0 {
        return Some(InflateZlibHeaderError::IncorrectCheck);
    }

    if hold as ::core::ffi::c_uint & ((1 as ::core::ffi::c_uint) << 4).wrapping_sub(1)
        != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
    {
        return Some(InflateZlibHeaderError::UnknownCompressionMethod);
    }

    None
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct InflateZlibWindowParams {
    wbits: ::core::ffi::c_uint,
    dmax: ::core::ffi::c_uint,
    next_mode: inflate_mode,
}

fn inflate_zlib_window_params(
    hold: crate::stdlib::uLong,
    configured_wbits: ::core::ffi::c_uint,
) -> Option<InflateZlibWindowParams> {
    let header_wbits = (hold as ::core::ffi::c_uint & 0x0f).wrapping_add(8);
    let wbits = if configured_wbits == 0 {
        header_wbits
    } else {
        configured_wbits
    };

    if header_wbits > 15 || header_wbits > wbits {
        return None;
    }

    Some(InflateZlibWindowParams {
        wbits,
        dmax: (1 as ::core::ffi::c_uint) << header_wbits,
        next_mode: if hold & 0x200 as crate::stdlib::uLong != 0 {
            DICTID
        } else {
            TYPE
        },
    })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InflateZlibHeaderTransition {
    Error(InflateZlibHeaderError),
    InvalidWindow {
        hold: crate::stdlib::uLong,
        bits: ::core::ffi::c_uint,
    },
    Accepted(InflateZlibWindowParams),
}

fn inflate_zlib_header_transition(
    wrap: ::core::ffi::c_int,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
    configured_wbits: ::core::ffi::c_uint,
) -> InflateZlibHeaderTransition {
    if let Some(error) = inflate_zlib_header_error(wrap, hold) {
        return InflateZlibHeaderTransition::Error(error);
    }

    let hold = hold >> 4;
    let bits = bits.wrapping_sub(4);
    match inflate_zlib_window_params(hold, configured_wbits) {
        Some(params) => InflateZlibHeaderTransition::Accepted(params),
        None => InflateZlibHeaderTransition::InvalidWindow { hold, bits },
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InflateBlockKind {
    Stored,
    Fixed,
    Dynamic,
    Invalid,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct InflateBlockHeader {
    last: ::core::ffi::c_int,
    kind: InflateBlockKind,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
}

fn inflate_block_header(
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
) -> InflateBlockHeader {
    let kind = match (hold >> 1) as ::core::ffi::c_uint & 3 {
        0 => InflateBlockKind::Stored,
        1 => InflateBlockKind::Fixed,
        2 => InflateBlockKind::Dynamic,
        _ => InflateBlockKind::Invalid,
    };

    InflateBlockHeader {
        last: (hold & 1) as ::core::ffi::c_int,
        kind,
        hold: hold >> 3,
        bits: bits.wrapping_sub(3),
    }
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
    (have as u64) + (repeat as u64) <= (nlen as u64) + (ndist as u64)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct DynamicCodeLengthRepeat {
    base: ::core::ffi::c_uint,
    extra_bits: ::core::ffi::c_uint,
    repeats_previous: bool,
}

fn dynamic_code_length_repeat_spec(symbol: ::core::ffi::c_ushort) -> DynamicCodeLengthRepeat {
    match symbol {
        16 => DynamicCodeLengthRepeat {
            base: 3,
            extra_bits: 2,
            repeats_previous: true,
        },
        17 => DynamicCodeLengthRepeat {
            base: 3,
            extra_bits: 3,
            repeats_previous: false,
        },
        _ => DynamicCodeLengthRepeat {
            base: 11,
            extra_bits: 7,
            repeats_previous: false,
        },
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct InflateCopyProgress {
    pub copied: ::core::ffi::c_uint,
    pub remaining_input: ::core::ffi::c_uint,
    pub remaining_output: ::core::ffi::c_uint,
    pub remaining_length: ::core::ffi::c_uint,
}

pub(crate) fn inflate_copy_progress(
    length: ::core::ffi::c_uint,
    available_input: ::core::ffi::c_uint,
    available_output: ::core::ffi::c_uint,
) -> InflateCopyProgress {
    let copied = length.min(available_input).min(available_output);

    InflateCopyProgress {
        copied,
        remaining_input: available_input.wrapping_sub(copied),
        remaining_output: available_output.wrapping_sub(copied),
        remaining_length: length.wrapping_sub(copied),
    }
}

fn stored_block_length(hold: crate::stdlib::uLong) -> Option<::core::ffi::c_uint> {
    if hold & 0xffff as crate::stdlib::uLong
        == hold >> 16 as ::core::ffi::c_int ^ 0xffff as crate::stdlib::uLong
    {
        Some(hold as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint)
    } else {
        None
    }
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
    if current_bits > 32 || requested_bits > 16 {
        return InflatePrimeUpdate::StreamError;
    }
    let Some(new_bits) = current_bits.checked_add(requested_bits) else {
        return InflatePrimeUpdate::StreamError;
    };
    if new_bits > 32 {
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

fn reset_window_history(
    wsize: &mut ::core::ffi::c_uint,
    whave: &mut ::core::ffi::c_uint,
    wnext: &mut ::core::ffi::c_uint,
) {
    *wsize = 0;
    *whave = 0;
    *wnext = 0;
}

fn window_needs_allocation(has_window: bool) -> bool {
    !has_window
}

fn window_allocation_request(
    wbits: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::stdlib::uInt) {
    (1_u32 << wbits, 1)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum WindowAllocationPlan {
    Existing,
    Allocate {
        items: crate::stdlib::uInt,
        size: crate::stdlib::uInt,
    },
}

fn window_allocation_plan(has_window: bool, wbits: crate::stdlib::uInt) -> WindowAllocationPlan {
    if window_needs_allocation(has_window) {
        let (items, size) = window_allocation_request(wbits);
        WindowAllocationPlan::Allocate { items, size }
    } else {
        WindowAllocationPlan::Existing
    }
}

fn window_allocation_request_for_plan(
    plan: WindowAllocationPlan,
) -> Option<(crate::stdlib::uInt, crate::stdlib::uInt)> {
    match plan {
        WindowAllocationPlan::Allocate { items, size } => Some((items, size)),
        WindowAllocationPlan::Existing => None,
    }
}

fn window_allocation_failed(plan: WindowAllocationPlan, has_window: bool) -> bool {
    matches!(plan, WindowAllocationPlan::Allocate { .. }) && !has_window
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

fn copy_dictionary_from_window(
    window: &[crate::stdlib::Bytef],
    wnext: usize,
    dictionary: &mut [crate::stdlib::Bytef],
) {
    if dictionary.is_empty() {
        return;
    }

    let first = dictionary.len() - wnext;
    dictionary[..first].copy_from_slice(&window[wnext..wnext + first]);
    dictionary[first..].copy_from_slice(&window[..wnext]);
}

fn inflate_get_dictionary_result(
    whave: crate::stdlib::uInt,
    wnext: crate::stdlib::uInt,
    window: Option<&[crate::stdlib::Bytef]>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    if let (Some(window), Some(dictionary)) = (window, dictionary) {
        copy_dictionary_from_window(window, wnext as usize, dictionary);
    }
    if let Some(dict_length) = dict_length {
        *dict_length = whave;
    }
    crate::zlib_h::Z_OK
}

fn inflate_mark_value(
    back: ::core::ffi::c_int,
    mode: inflate_mode,
    length: crate::stdlib::uInt,
    was: crate::stdlib::uInt,
) -> ::core::ffi::c_long {
    let progress = inflate_mark_progress(mode, length, was);
    (((back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int)
        as ::core::ffi::c_long)
        + progress as ::core::ffi::c_long
}

fn inflate_mark_progress(
    mode: inflate_mode,
    length: crate::stdlib::uInt,
    was: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    if mode == crate::src::inflate::COPY_1 {
        length
    } else if mode == crate::src::inflate::MATCH {
        was.wrapping_sub(length)
    } else {
        0
    }
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

fn inflate_reset2_discards_window(
    has_window: bool,
    current_wbits: ::core::ffi::c_uint,
    requested_wbits: ::core::ffi::c_uint,
) -> bool {
    has_window && current_wbits != requested_wbits
}

fn inflate_state_check_impl(
    stream: Option<&crate::zlib_h::z_stream>,
    state: Option<&crate::src::inflate::inflate_state>,
    state_matches_stream: bool,
) -> ::core::ffi::c_int {
    let Some(stream) = stream else {
        return inflate_state_check_result(false, false, false);
    };
    let Some(state) = state else {
        return inflate_state_check_result(true, false, false);
    };
    let state_is_usable = inflate_state_is_usable(
        stream.zalloc.is_some(),
        stream.zfree.is_some(),
        state_matches_stream,
        state.mode,
    );
    inflate_state_check_result(true, true, state_is_usable)
}

fn inflate_align_to_byte_boundary(
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
) -> (crate::stdlib::uLong, ::core::ffi::c_uint) {
    let discarded_bits = bits & 7 as ::core::ffi::c_uint;
    (hold >> discarded_bits, bits.wrapping_sub(discarded_bits))
}

unsafe fn inflateStateCheck(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() {
        return inflate_state_check_impl(None, None, false);
    }
    let stream = &*strm;
    let state = stream.state as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return inflate_state_check_impl(Some(stream), None, false);
    }
    let state = &*state;
    inflate_state_check_impl(Some(stream), Some(state), state.strm == strm)
}

fn inflate_reset_keep_core(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) {
    state.total = 0 as ::core::ffi::c_ulong;
    strm.total_out = state.total as crate::stdlib::uLong;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = 0 as ::core::ffi::c_int;
    if let Some(adler) = inflate_reset_keep_adler(state.wrap) {
        strm.adler = adler;
    }
    state.mode = crate::src::inflate::HEAD;
    state.last = 0 as ::core::ffi::c_int;
    state.havedict = 0 as ::core::ffi::c_int;
    state.flags = -1 as ::core::ffi::c_int;
    state.dmax = 32768 as ::core::ffi::c_uint;
    state.head = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
    state.hold = 0 as ::core::ffi::c_ulong;
    state.bits = 0 as ::core::ffi::c_uint;
    state.sane = 1 as ::core::ffi::c_int;
    state.back = -1 as ::core::ffi::c_int;
}

pub unsafe extern "C" fn inflateResetKeep(strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    inflate_reset_keep_core(strm, state);
    state.next = &raw mut state.codes as *mut crate::src::inftrees::code;
    state.distcode = state.next;
    state.lencode = state.distcode;
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
    reset_window_history(
        &mut (*state).wsize,
        &mut (*state).whave,
        &mut (*state).wnext,
    );
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
    if inflate_reset2_discards_window(!(*state).window.is_null(), (*state).wbits, window_bits) {
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
        || *version as ::core::ffi::c_int
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
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit2_(strm, crate::zutil_h::DEF_WBITS, version, stream_size)
}
#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = (*strm).state as *mut crate::src::inflate::inflate_state;
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

pub(crate) fn inflate_can_use_fast_path(
    available_input: ::core::ffi::c_uint,
    available_output: ::core::ffi::c_uint,
) -> bool {
    available_input >= 6 && available_output >= 258
}

fn inflate_dictionary_id_from_hold(hold: crate::stdlib::uLong) -> crate::stdlib::uLong {
    (hold >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_ulong)
        .wrapping_add(hold >> 8 as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_ulong)
        .wrapping_add((hold & 0xff00 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int)
        .wrapping_add((hold & 0xff as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int)
}

fn inflate_trailer_checksum_from_hold(
    flags: ::core::ffi::c_int,
    hold: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    if flags != 0 {
        hold
    } else {
        inflate_dictionary_id_from_hold(hold)
    }
}

fn inflate_gzip_length_check_required(wrap: ::core::ffi::c_int, flags: ::core::ffi::c_int) -> bool {
    wrap != 0 && flags != 0
}

fn update_window_metadata(
    wbits: ::core::ffi::c_uint,
    wsize: &mut ::core::ffi::c_uint,
    wnext: &mut ::core::ffi::c_uint,
    whave: &mut ::core::ffi::c_uint,
) {
    if *wsize == 0 {
        let metadata = initial_window_metadata(wbits);
        *wsize = metadata.wsize;
        *wnext = metadata.wnext;
        *whave = metadata.whave;
    }
}

fn update_window_core(
    wbits: ::core::ffi::c_uint,
    wsize: &mut ::core::ffi::c_uint,
    wnext: &mut ::core::ffi::c_uint,
    whave: &mut ::core::ffi::c_uint,
    window: &mut [crate::stdlib::Bytef],
    produced: &[crate::stdlib::Bytef],
) {
    update_window_metadata(wbits, wsize, wnext, whave);

    let plan = apply_window_update(window, *wnext, *whave, produced);
    *wnext = plan.wnext;
    *whave = plan.whave;
}

fn update_window_buffer_len(wsize: ::core::ffi::c_uint) -> usize {
    wsize as usize
}

fn update_window_produced_len(copy: ::core::ffi::c_uint) -> Option<usize> {
    if copy == 0 {
        None
    } else {
        Some(copy as usize)
    }
}

fn update_window_produced_slice<'a>(
    produced: Option<&'a [crate::stdlib::Bytef]>,
) -> &'a [crate::stdlib::Bytef] {
    produced.unwrap_or(&[])
}

unsafe fn updatewindow(
    mut strm: crate::zlib_h::z_streamp,
    mut produced_start: *const crate::stdlib::Bytef,
    mut copy: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let state = (*strm).state as *mut crate::src::inflate::inflate_state;
    let allocation_plan = window_allocation_plan(!(*state).window.is_null(), (*state).wbits);
    if let Some((items, size)) = window_allocation_request_for_plan(allocation_plan) {
        (*state).window = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque, items, size
        ) as *mut ::core::ffi::c_uchar;
    }
    if window_allocation_failed(allocation_plan, !(*state).window.is_null()) {
        return 1;
    }
    let state = &mut *state;
    update_window_metadata(
        state.wbits,
        &mut state.wsize,
        &mut state.wnext,
        &mut state.whave,
    );
    let window =
        core::slice::from_raw_parts_mut(state.window, update_window_buffer_len(state.wsize));
    let produced = match update_window_produced_len(copy) {
        Some(produced_len) => Some(core::slice::from_raw_parts(produced_start, produced_len)),
        None => None,
    };
    update_window_core(
        state.wbits,
        &mut state.wsize,
        &mut state.wnext,
        &mut state.whave,
        window,
        update_window_produced_slice(produced),
    );
    0
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
    let mut output_start: *const crate::stdlib::Bytef = ::core::ptr::null::<crate::stdlib::Bytef>();
    let mut checksum_start: *const crate::stdlib::Bytef =
        ::core::ptr::null::<crate::stdlib::Bytef>();
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
        || !inflate_stream_buffers_are_valid(
            !(*strm).next_out.is_null(),
            !(*strm).next_in.is_null(),
            (*strm).avail_in,
        )
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    (*state).mode = inflate_mode_on_entry((*state).mode);
    put = (*strm).next_out as *mut ::core::ffi::c_uchar;
    output_start = put as *const crate::stdlib::Bytef;
    checksum_start = output_start;
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
                if let Some(next_mode) = inflate_head_skip_mode((*state).wrap) {
                    (*state).mode = next_mode;
                    continue;
                } else {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh0 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh0 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if inflate_is_gzip_header((*state).wrap, hold) {
                        (*state).wbits = inflate_gzip_window_bits((*state).wbits);
                        (*state).check = crate::src::crc32::CRC32_INITIAL as ::core::ffi::c_ulong;
                        hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                        hbuf[1 as ::core::ffi::c_int as usize] =
                            (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        (*state).check = crate::src::crc32::crc32_z(
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
                        match inflate_zlib_header_transition(
                            (*state).wrap,
                            hold,
                            bits,
                            (*state).wbits,
                        ) {
                            InflateZlibHeaderTransition::Error(
                                InflateZlibHeaderError::IncorrectCheck,
                            ) => {
                                (*strm).msg = b"incorrect header check\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            }
                            InflateZlibHeaderTransition::Error(
                                InflateZlibHeaderError::UnknownCompressionMethod,
                            ) => {
                                (*strm).msg = b"unknown compression method\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            }
                            InflateZlibHeaderTransition::InvalidWindow {
                                hold: transition_hold,
                                bits: transition_bits,
                            } => {
                                hold = transition_hold;
                                bits = transition_bits;
                                (*strm).msg = b"invalid window size\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            }
                            InflateZlibHeaderTransition::Accepted(window_params) => {
                                (*state).wbits = window_params.wbits;
                                (*state).dmax = window_params.dmax;
                                (*state).flags = 0 as ::core::ffi::c_int;
                                (*state).check =
                                    crate::src::adler32::ADLER32_INITIAL as ::core::ffi::c_ulong;
                                (*strm).adler = (*state).check as crate::stdlib::uLong;
                                (*state).mode = window_params.next_mode;
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
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh1 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                let (gzip_flags, gzip_flags_error) = inflate_gzip_flags_validation(hold);
                (*state).flags = gzip_flags.flags;
                if let Some(gzip_flags_error) = gzip_flags_error {
                    (*strm).msg = match gzip_flags_error {
                        InflateGzipFlagsError::UnknownCompressionMethod => {
                            b"unknown compression method\0".as_ptr()
                        }
                        InflateGzipFlagsError::UnknownHeaderFlags => {
                            b"unknown header flags set\0".as_ptr()
                        }
                    } as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                }
                if !(*state).head.is_null() {
                    (*(*state).head).text = gzip_flags.text;
                }
                if inflate_header_crc_enabled((*state).flags, (*state).wrap) {
                    hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                    hbuf[1 as ::core::ffi::c_int as usize] =
                        (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    (*state).check = crate::src::crc32::crc32_z(
                        (*state).check as crate::stdlib::uLong,
                        &hbuf[..2],
                    ) as ::core::ffi::c_ulong;
                }
                hold = 0 as ::core::ffi::c_ulong;
                bits = 0 as ::core::ffi::c_uint;
                (*state).mode = crate::src::inflate::TIME;
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
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh10 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                (*state).check = inflate_dictionary_id_from_hold(hold);
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
                (hold, bits) = inflate_align_to_byte_boundary(hold, bits);
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh12 = next;
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh12 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                let Some(length) = stored_block_length(hold) else {
                    (*strm).msg = b"invalid stored block lengths\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                };
                (*state).length = length;
                hold = 0 as ::core::ffi::c_ulong;
                bits = 0 as ::core::ffi::c_uint;
                (*state).mode = crate::src::inflate::COPY_;
                if flush == crate::zlib_h::Z_TREES {
                    break;
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
                    next = next.wrapping_add(1);
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
                put = put.wrapping_add(1);
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
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh33 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    out = inflate_cursor_progress(out, left);
                    (*strm).total_out = (*strm).total_out.wrapping_add(out as crate::stdlib::uLong);
                    (*state).total = (*state).total.wrapping_add(out as ::core::ffi::c_ulong);
                    if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
                        (*state).check = (if (*state).flags != 0 {
                            crate::src::crc32::crc32_ffi(
                                (*state).check as crate::stdlib::uLong,
                                checksum_start,
                                out as crate::stdlib::uInt,
                            )
                        } else {
                            crate::src::adler32::adler32_ffi(
                                (*state).check as crate::stdlib::uLong,
                                checksum_start,
                                out as crate::stdlib::uInt,
                            )
                        }) as ::core::ffi::c_ulong;
                        (*strm).adler = (*state).check as crate::stdlib::uLong;
                    }
                    checksum_start = put as *const crate::stdlib::Bytef;
                    out = left;
                    if (*state).wrap & 4 as ::core::ffi::c_int != 0
                        && inflate_trailer_checksum_from_hold((*state).flags, hold)
                            != (*state).check
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
                if inflate_gzip_length_check_required((*state).wrap, (*state).flags) {
                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh34 = next;
                        next = next.wrapping_add(1);
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
                        next = next.wrapping_add(1);
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
                (*state).check = crate::src::adler32::ADLER32_INITIAL as ::core::ffi::c_ulong;
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
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh2 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if !(*state).head.is_null() {
                    (*(*state).head).time = hold as crate::stdlib::uLong;
                }
                if inflate_header_crc_enabled((*state).flags, (*state).wrap) {
                    hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                    hbuf[1 as ::core::ffi::c_int as usize] =
                        (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    hbuf[2 as ::core::ffi::c_int as usize] =
                        (hold >> 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    hbuf[3 as ::core::ffi::c_int as usize] =
                        (hold >> 24 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    (*state).check = crate::src::crc32::crc32_z(
                        (*state).check as crate::stdlib::uLong,
                        &hbuf[..4],
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
                        here = *(*state).lencode.wrapping_add(
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                as usize,
                        );
                        if here.bits as ::core::ffi::c_uint <= bits {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh17 = next;
                        next = next.wrapping_add(1);
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
                        let repeat = dynamic_code_length_repeat_spec(here.val);
                        while bits
                            < (here.bits as ::core::ffi::c_uint).wrapping_add(repeat.extra_bits)
                        {
                            if have == 0 as ::core::ffi::c_uint {
                                break 's_88;
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh19 = next;
                            next = next.wrapping_add(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh19 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        hold >>= here.bits as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                        if repeat.repeats_previous && (*state).have == 0 as ::core::ffi::c_uint {
                            (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                            break;
                        } else {
                            len = if repeat.repeats_previous {
                                (*state).lens
                                    [(*state).have.wrapping_sub(1 as ::core::ffi::c_uint) as usize]
                                    as ::core::ffi::c_uint
                            } else {
                                0 as ::core::ffi::c_uint
                            };
                            copy = repeat.base.wrapping_add(
                                hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint) << repeat.extra_bits)
                                        .wrapping_sub(1 as ::core::ffi::c_uint),
                            );
                            hold >>= repeat.extra_bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(repeat.extra_bits);
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
                    let progress = inflate_copy_progress(copy, have, left);
                    copy = progress.copied;
                    if copy == 0 as ::core::ffi::c_uint {
                        break;
                    }
                    crate::stdlib::memcpy(
                        put as *mut ::core::ffi::c_void,
                        next as *const ::core::ffi::c_void,
                        copy as crate::__stddef_size_t_h::size_t,
                    );
                    have = progress.remaining_input;
                    next = next.wrapping_add(copy as usize);
                    left = progress.remaining_output;
                    put = put.wrapping_add(copy as usize);
                    (*state).length = progress.remaining_length;
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
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh3 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if !(*state).head.is_null() {
                    (*(*state).head).xflags =
                        (hold & 0xff as ::core::ffi::c_ulong) as ::core::ffi::c_int;
                    (*(*state).head).os = (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
                }
                if inflate_header_crc_enabled((*state).flags, (*state).wrap) {
                    hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                    hbuf[1 as ::core::ffi::c_int as usize] =
                        (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    (*state).check = crate::src::crc32::crc32_z(
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
                    (hold, bits) = inflate_align_to_byte_boundary(hold, bits);
                    (*state).mode = crate::src::inflate::CHECK;
                    continue;
                } else {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh11 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh11 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    let header = inflate_block_header(hold, bits);
                    (*state).last = header.last;
                    hold = header.hold;
                    bits = header.bits;
                    match header.kind {
                        InflateBlockKind::Stored => {
                            (*state).mode = crate::src::inflate::STORED;
                        }
                        InflateBlockKind::Fixed => {
                            crate::src::inftrees::inflate_fixed(&mut *state);
                            (*state).mode = crate::src::inflate::LEN_;
                            if flush == crate::zlib_h::Z_TREES {
                                break;
                            }
                        }
                        InflateBlockKind::Dynamic => {
                            (*state).mode = crate::src::inflate::TABLE;
                        }
                        InflateBlockKind::Invalid => {
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
                if inflate_gzip_header_has_extra((*state).flags) {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh4 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh4 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    (*state).length = hold as ::core::ffi::c_uint;
                    if !(*state).head.is_null() {
                        (*(*state).head).extra_len =
                            hold as ::core::ffi::c_uint as crate::stdlib::uInt;
                    }
                    if inflate_header_crc_enabled((*state).flags, (*state).wrap) {
                        hbuf[0 as ::core::ffi::c_int as usize] = hold as ::core::ffi::c_uchar;
                        hbuf[1 as ::core::ffi::c_int as usize] =
                            (hold >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        (*state).check = crate::src::crc32::crc32_z(
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
                if inflate_can_use_fast_path(have, left) {
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
                        here = *(*state).lencode.wrapping_add(
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                as usize,
                        );
                        if here.bits as ::core::ffi::c_uint <= bits {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh24 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh24 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if here.op as ::core::ffi::c_int != 0
                        && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        last = here;
                        loop {
                            here = *(*state).lencode.wrapping_add(
                                (last.val as ::core::ffi::c_uint).wrapping_add(
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint)
                                            << last.bits as ::core::ffi::c_int
                                                + last.op as ::core::ffi::c_int)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        >> last.bits as ::core::ffi::c_int,
                                ) as usize,
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
                            next = next.wrapping_add(1);
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
                if inflate_gzip_header_has_extra((*state).flags) {
                    let extra_progress = inflate_gzip_extra_progress((*state).length, have);
                    copy = extra_progress.copy;
                    if copy != 0 {
                        if !(*state).head.is_null() && !(*(*state).head).extra.is_null() {
                            if let Some((offset, copy_len)) = gzip_extra_copy_bounds(
                                (*(*state).head).extra_len as ::core::ffi::c_uint,
                                (*state).length,
                                (*(*state).head).extra_max,
                                copy,
                            ) {
                                crate::stdlib::memcpy(
                                    (*(*state).head).extra.wrapping_add(offset as usize)
                                        as *mut ::core::ffi::c_void,
                                    next as *const ::core::ffi::c_void,
                                    copy_len as crate::__stddef_size_t_h::size_t,
                                );
                            }
                        }
                        if inflate_header_crc_enabled((*state).flags, (*state).wrap) {
                            (*state).check = crate::src::crc32::crc32_ffi(
                                (*state).check as crate::stdlib::uLong,
                                next,
                                copy as crate::stdlib::uInt,
                            ) as ::core::ffi::c_ulong;
                        }
                        have = have.wrapping_sub(copy);
                        next = next.wrapping_add(copy as usize);
                        (*state).length = extra_progress.remaining;
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
                        next = next.wrapping_add(1);
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
                if inflate_gzip_header_has_name((*state).flags) {
                    if have == 0 as ::core::ffi::c_uint {
                        break;
                    }
                    copy = 0 as ::core::ffi::c_uint;
                    loop {
                        let c2rust_fresh5 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.wrapping_add(c2rust_fresh5 as usize) as ::core::ffi::c_uint;
                        if !(*state).head.is_null()
                            && !(*(*state).head).name.is_null()
                            && (*state).length < (*(*state).head).name_max
                        {
                            let c2rust_fresh6 = (*state).length;
                            (*state).length = (*state).length.wrapping_add(1);
                            *(*(*state).head).name.wrapping_add(c2rust_fresh6 as usize) =
                                len as crate::stdlib::Bytef;
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if inflate_header_crc_enabled((*state).flags, (*state).wrap) {
                        (*state).check = crate::src::crc32::crc32_ffi(
                            (*state).check as crate::stdlib::uLong,
                            next,
                            copy as crate::stdlib::uInt,
                        ) as ::core::ffi::c_ulong;
                    }
                    have = have.wrapping_sub(copy);
                    next = next.wrapping_add(copy as usize);
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
                    here = *(*state).distcode.wrapping_add(
                        (hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).distbits)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as usize,
                    );
                    if here.bits as ::core::ffi::c_uint <= bits {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh27 = next;
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh27 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    last = here;
                    loop {
                        here = *(*state).distcode.wrapping_add(
                            (last.val as ::core::ffi::c_uint).wrapping_add(
                                (hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint)
                                        << last.bits as ::core::ffi::c_int
                                            + last.op as ::core::ffi::c_int)
                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                    >> last.bits as ::core::ffi::c_int,
                            ) as usize,
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
                        next = next.wrapping_add(1);
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
                if inflate_gzip_header_has_comment((*state).flags) {
                    if have == 0 as ::core::ffi::c_uint {
                        break;
                    }
                    copy = 0 as ::core::ffi::c_uint;
                    loop {
                        let c2rust_fresh7 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.wrapping_add(c2rust_fresh7 as usize) as ::core::ffi::c_uint;
                        if !(*state).head.is_null()
                            && !(*(*state).head).comment.is_null()
                            && (*state).length < (*(*state).head).comm_max
                        {
                            let c2rust_fresh8 = (*state).length;
                            (*state).length = (*state).length.wrapping_add(1);
                            *(*(*state).head)
                                .comment
                                .wrapping_add(c2rust_fresh8 as usize) = len as crate::stdlib::Bytef;
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if inflate_header_crc_enabled((*state).flags, (*state).wrap) {
                        (*state).check = crate::src::crc32::crc32_ffi(
                            (*state).check as crate::stdlib::uLong,
                            next,
                            copy as crate::stdlib::uInt,
                        ) as ::core::ffi::c_ulong;
                    }
                    have = have.wrapping_sub(copy);
                    next = next.wrapping_add(copy as usize);
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
                        next = next.wrapping_add(1);
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
                if inflate_gzip_header_has_crc((*state).flags) {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh9 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh9 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if !inflate_gzip_header_crc_is_valid((*state).wrap, hold, (*state).check) {
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
                        inflate_gzip_header_has_crc((*state).flags) as ::core::ffi::c_int;
                    (*(*state).head).done = 1 as ::core::ffi::c_int;
                }
                (*state).check = crate::src::crc32::CRC32_INITIAL as ::core::ffi::c_ulong;
                (*strm).adler = (*state).check as crate::stdlib::uLong;
                (*state).mode = crate::src::inflate::TYPE;
                continue;
            }
            _ => {}
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        let match_plan = inflate_match_copy_plan(
            (*state).offset,
            inflate_cursor_progress(out, left),
            (*state).whave,
            (*state).wnext,
            (*state).wsize,
            (*state).length,
            left,
            (*state).sane != 0,
        );
        let InflateMatchPlan::Copy {
            source,
            count,
            remaining_output,
            remaining_length,
        } = match_plan
        else {
            (*strm).msg = b"invalid distance too far back\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            (*state).mode = crate::src::inflate::BAD;
            continue;
        };
        from = match source {
            InflateMatchSource::Window { index } => (*state).window.wrapping_add(index as usize),
            InflateMatchSource::Output { offset } => put.wrapping_sub(offset as usize),
        };
        copy = count;
        left = remaining_output;
        (*state).length = remaining_length;
        loop {
            let c2rust_fresh30 = from;
            from = from.wrapping_add(1);
            let c2rust_fresh31 = put;
            put = put.wrapping_add(1);
            *c2rust_fresh31 = *c2rust_fresh30;
            copy = copy.wrapping_sub(1);
            if !(copy != 0) {
                break;
            }
        }
        if inflate_match_is_complete((*state).length) {
            (*state).mode = crate::src::inflate::LEN;
        }
    }
    (*strm).next_out = put as *mut crate::stdlib::Bytef;
    (*strm).avail_out = left as crate::stdlib::uInt;
    (*strm).next_in = next as *mut crate::stdlib::Bytef;
    (*strm).avail_in = have as crate::stdlib::uInt;
    (*state).hold = hold;
    (*state).bits = bits;
    if inflate_should_update_window((*state).wsize, out, left, (*state).mode, flush) {
        if updatewindow(strm, output_start, inflate_cursor_progress(out, left)) != 0 {
            (*state).mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    in_0 = inflate_cursor_progress(in_0, (*strm).avail_in as ::core::ffi::c_uint);
    out = inflate_cursor_progress(out, (*strm).avail_out as ::core::ffi::c_uint);
    (*strm).total_in = (*strm).total_in.wrapping_add(in_0 as crate::stdlib::uLong);
    (*strm).total_out = (*strm).total_out.wrapping_add(out as crate::stdlib::uLong);
    (*state).total = (*state).total.wrapping_add(out as ::core::ffi::c_ulong);
    if let Some(checksum) = inflate_output_checksum((*state).wrap, (*state).flags, out) {
        (*state).check = (match checksum {
            InflateOutputChecksum::Crc32 => crate::src::crc32::crc32_ffi(
                (*state).check as crate::stdlib::uLong,
                checksum_start,
                out as crate::stdlib::uInt,
            ),
            InflateOutputChecksum::Adler32 => crate::src::adler32::adler32_ffi(
                (*state).check as crate::stdlib::uLong,
                checksum_start,
                out as crate::stdlib::uInt,
            ),
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
#[export_name = "inflateGetDictionary"]
pub unsafe extern "C" fn inflateGetDictionary_ffi(
    strm: crate::zlib_h::z_streamp,
    dictionary: *mut crate::stdlib::Bytef,
    dict_length: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    let whave = (*state).whave;
    let wnext = (*state).wnext;
    let (window, dictionary) = if whave != 0 && !dictionary.is_null() {
        let window = core::slice::from_raw_parts((*state).window, (*state).wsize as usize);
        let dictionary = core::slice::from_raw_parts_mut(dictionary, whave as usize);
        (Some(window), Some(dictionary))
    } else {
        (None, None)
    };

    inflate_get_dictionary_result(whave, wnext, window, dictionary, dict_length.as_mut())
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
    let mut ret: ::core::ffi::c_int = 0;
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if !inflate_dictionary_is_allowed((*state).wrap, (*state).mode) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).mode as ::core::ffi::c_uint
        == crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        dictid = crate::src::adler32::ADLER32_INITIAL as ::core::ffi::c_ulong;
        dictid = crate::src::adler32::adler32_ffi(
            dictid as crate::stdlib::uLong,
            dictionary,
            dictLength,
        ) as ::core::ffi::c_ulong;
        if dictid != (*state).check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }
    ret = updatewindow(strm, dictionary, dictLength as ::core::ffi::c_uint);
    if ret != 0 {
        (*state).mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*state).havedict = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}

fn inflate_dictionary_is_allowed(
    wrap: ::core::ffi::c_int,
    mode: crate::src::inflate::inflate_mode,
) -> bool {
    wrap == 0 || mode == crate::src::inflate::DICT
}

fn inflate_header_wrap_allows_capture(wrap: ::core::ffi::c_int) -> bool {
    wrap & 2 as ::core::ffi::c_int != 0
}

#[export_name = "inflateGetHeader"]
pub unsafe extern "C" fn inflateGetHeader_ffi(
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

fn inflate_mode_data_type_flags(mode: inflate_mode) -> ::core::ffi::c_int {
    match mode {
        TYPE => 128,
        LEN_ | COPY_ => 256,
        _ => 0,
    }
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
        + inflate_mode_data_type_flags(mode)
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

fn inflate_header_crc_enabled(flags: ::core::ffi::c_int, wrap: ::core::ffi::c_int) -> bool {
    inflate_gzip_header_has_crc(flags) && wrap & 4 != 0
}

fn inflate_gzip_header_crc_is_valid(
    wrap: ::core::ffi::c_int,
    received_crc: crate::stdlib::uLong,
    calculated_crc: crate::stdlib::uLong,
) -> bool {
    wrap & 4 == 0 || received_crc == calculated_crc & 0xffff
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InflateGzipFlagsError {
    UnknownCompressionMethod,
    UnknownHeaderFlags,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct InflateGzipFlags {
    flags: ::core::ffi::c_int,
    text: ::core::ffi::c_int,
}

fn inflate_gzip_flags(hold: crate::stdlib::uLong) -> InflateGzipFlags {
    let flags = hold as ::core::ffi::c_int;
    let text = ((hold >> 8) & 1) as ::core::ffi::c_int;

    InflateGzipFlags { flags, text }
}

fn inflate_gzip_flags_error(flags: ::core::ffi::c_int) -> Option<InflateGzipFlagsError> {
    if flags & 0xff != crate::zlib_h::Z_DEFLATED {
        Some(InflateGzipFlagsError::UnknownCompressionMethod)
    } else if flags & 0xe000 != 0 {
        Some(InflateGzipFlagsError::UnknownHeaderFlags)
    } else {
        None
    }
}

fn inflate_gzip_flags_validation(
    hold: crate::stdlib::uLong,
) -> (InflateGzipFlags, Option<InflateGzipFlagsError>) {
    let flags = inflate_gzip_flags(hold);
    let error = inflate_gzip_flags_error(flags.flags);
    (flags, error)
}

fn inflate_gzip_header_has_extra(flags: ::core::ffi::c_int) -> bool {
    flags & 0x400 != 0
}

fn inflate_gzip_header_has_crc(flags: ::core::ffi::c_int) -> bool {
    flags & 0x200 != 0
}

fn inflate_gzip_header_has_name(flags: ::core::ffi::c_int) -> bool {
    flags & 0x800 != 0
}

fn inflate_gzip_header_has_comment(flags: ::core::ffi::c_int) -> bool {
    flags & 0x1000 != 0
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct InflateGzipExtraProgress {
    copy: ::core::ffi::c_uint,
    remaining: ::core::ffi::c_uint,
}

fn inflate_gzip_extra_progress(
    remaining: ::core::ffi::c_uint,
    available_input: ::core::ffi::c_uint,
) -> InflateGzipExtraProgress {
    let copy = remaining.min(available_input);
    InflateGzipExtraProgress {
        copy,
        remaining: remaining.wrapping_sub(copy),
    }
}

fn gzip_extra_copy_bounds(
    extra_len: ::core::ffi::c_uint,
    remaining: ::core::ffi::c_uint,
    extra_max: ::core::ffi::c_uint,
    input_copy: ::core::ffi::c_uint,
) -> Option<(::core::ffi::c_uint, ::core::ffi::c_uint)> {
    let offset = extra_len.wrapping_sub(remaining);
    if offset >= extra_max {
        return None;
    }
    let copy = if offset.wrapping_add(input_copy) > extra_max {
        extra_max.wrapping_sub(offset)
    } else {
        input_copy
    };
    Some((offset, copy))
}

fn inflate_should_update_window(
    wsize: ::core::ffi::c_uint,
    initial_out: ::core::ffi::c_uint,
    remaining_out: ::core::ffi::c_uint,
    mode: inflate_mode,
    flush: ::core::ffi::c_int,
) -> bool {
    wsize != 0
        || initial_out != remaining_out
            && mode < crate::src::inflate::BAD
            && (mode < crate::src::inflate::CHECK || flush != crate::zlib_h::Z_FINISH)
}

fn inflate_cursor_progress(
    initial: ::core::ffi::c_uint,
    remaining: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    initial.wrapping_sub(remaining)
}

fn inflate_match_is_complete(remaining_length: ::core::ffi::c_uint) -> bool {
    remaining_length == 0
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InflateMatchSource {
    Window { index: ::core::ffi::c_uint },
    Output { offset: ::core::ffi::c_uint },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InflateMatchPlan {
    InvalidDistance,
    Copy {
        source: InflateMatchSource,
        count: ::core::ffi::c_uint,
        remaining_output: ::core::ffi::c_uint,
        remaining_length: ::core::ffi::c_uint,
    },
}

fn inflate_match_copy_plan(
    offset: ::core::ffi::c_uint,
    output_written: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    length: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    sane: bool,
) -> InflateMatchPlan {
    let source = if offset > output_written {
        let distance = offset.wrapping_sub(output_written);
        if distance > whave && sane {
            return InflateMatchPlan::InvalidDistance;
        }

        let index = if distance > wnext {
            wsize.wrapping_sub(distance.wrapping_sub(wnext))
        } else {
            wnext.wrapping_sub(distance)
        };
        InflateMatchSource::Window { index }
    } else {
        InflateMatchSource::Output { offset }
    };
    let count = length.min(left);

    InflateMatchPlan::Copy {
        source,
        count,
        remaining_output: left.wrapping_sub(count),
        remaining_length: length.wrapping_sub(count),
    }
}

fn inflate_copy_match_from_output(
    output: &mut [::core::ffi::c_uchar],
    output_written: usize,
    distance: usize,
    count: usize,
) -> bool {
    let Some(destination_end) = output_written.checked_add(count) else {
        return false;
    };
    if output_written > output.len()
        || distance == 0
        || distance > output_written
        || destination_end > output.len()
    {
        return false;
    }

    for destination_index in output_written..destination_end {
        let source_index = destination_index - distance;
        output[destination_index] = output[source_index];
    }
    true
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

fn inflate_sync_remaining_input(
    input: &[::core::ffi::c_uchar],
    consumed: usize,
) -> &[::core::ffi::c_uchar] {
    &input[consumed..]
}

fn inflate_sync_input_progress(
    avail_in: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    consumed: usize,
) -> (crate::stdlib::uInt, crate::stdlib::uLong) {
    (
        avail_in.wrapping_sub(consumed as crate::stdlib::uInt),
        total_in.wrapping_add(consumed as crate::stdlib::uLong),
    )
}

fn inflate_sync_normalized_wrap(
    flags: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if flags == -1 {
        0
    } else {
        wrap & !4
    }
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
    let avail_in = (*strm).avail_in;
    let input = if avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts((*strm).next_in, avail_in as usize)
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
    let (remaining_input, total_input) =
        inflate_sync_input_progress(avail_in, (*strm).total_in, consumed);
    (*strm).avail_in = remaining_input;
    if consumed != 0 {
        (*strm).next_in =
            inflate_sync_remaining_input(input, consumed).as_ptr() as *mut ::core::ffi::c_uchar;
    }
    (*strm).total_in = total_input;
    if let InflateSyncSearch::DataError { .. } = result {
        return crate::zlib_h::Z_DATA_ERROR;
    }

    let flags = (*state).flags;
    (*state).wrap = inflate_sync_normalized_wrap(flags, (*state).wrap);
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

#[export_name = "inflateSyncPoint"]
pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let state = (*strm).state as *mut crate::src::inflate::inflate_state;
    inflate_sync_point_value((*state).mode as ::core::ffi::c_uint, (*state).bits)
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
                .wrapping_add(crate::src::inftrees::ENOUGH.wrapping_sub(1) as usize)
                as *const crate::src::inftrees::code
    {
        (*copy).lencode = (&raw mut (*copy).codes as *mut crate::src::inftrees::code).wrapping_add(
            (*state)
                .lencode
                .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
                as usize,
        );
        (*copy).distcode = (&raw mut (*copy).codes as *mut crate::src::inftrees::code)
            .wrapping_add(
                (*state)
                    .distcode
                    .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
                    as usize,
            );
    }
    (*copy).next = (&raw mut (*copy).codes as *mut crate::src::inftrees::code).wrapping_add(
        (*state)
            .next
            .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
            as usize,
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
fn inflate_undermine_core(sane: &mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    *sane = 1 as ::core::ffi::c_int;
    crate::zlib_h::Z_DATA_ERROR
}

#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = (*strm).state as *mut crate::src::inflate::inflate_state;
    inflate_undermine_core(&mut (*state).sane)
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

fn inflate_validate_core(
    wrap: &mut ::core::ffi::c_int,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *wrap = inflate_validate_wrap(*wrap, check);
    crate::zlib_h::Z_OK
}

#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    inflate_validate_core(&mut state.wrap, check)
}
#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    if inflateStateCheck(strm) != 0 {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    }
    let state = (*strm).state as *mut crate::src::inflate::inflate_state;
    inflate_mark_value((*state).back, (*state).mode, (*state).length, (*state).was)
}
fn inflate_codes_used_offset_value(offset: ::core::ffi::c_long) -> ::core::ffi::c_ulong {
    offset as ::core::ffi::c_ulong
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    if inflateStateCheck(strm) != 0 {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    }
    let state = (*strm).state as *mut crate::src::inflate::inflate_state;
    inflate_codes_used_offset_value(
        (*state)
            .next
            .offset_from(&raw mut (*state).codes as *mut crate::src::inftrees::code)
            as ::core::ffi::c_long,
    )
}

#[cfg(test)]
mod tests {
    use super::{
        apply_window_update, copy_dictionary_from_window, dynamic_code_length_repeat_fits,
        dynamic_code_length_repeat_spec, dynamic_header_counts, gzip_extra_copy_bounds,
        inflateSyncPoint_ffi, inflate_align_to_byte_boundary, inflate_block_header,
        inflate_can_use_fast_path, inflate_codes_used_offset_value, inflate_copy_match_from_output,
        inflate_copy_progress, inflate_data_type_value, inflate_dictionary_id_from_hold,
        inflate_dictionary_is_allowed, inflate_get_dictionary_result, inflate_gzip_extra_progress,
        inflate_gzip_flags, inflate_gzip_flags_error, inflate_gzip_flags_validation,
        inflate_gzip_header_crc_is_valid, inflate_gzip_header_has_comment,
        inflate_gzip_header_has_crc, inflate_gzip_header_has_extra, inflate_gzip_header_has_name,
        inflate_gzip_length_check_required, inflate_gzip_window_bits, inflate_head_skip_mode,
        inflate_header_crc_enabled, inflate_header_wrap_allows_capture, inflate_is_gzip_header,
        inflate_mark_progress, inflate_mark_value, inflate_match_copy_plan,
        inflate_match_is_complete, inflate_mode_data_type_flags, inflate_mode_is_valid,
        inflate_mode_on_entry, inflate_needs_buffer_error, inflate_output_checksum,
        inflate_prime_update, inflate_reset2_discards_window, inflate_reset2_params,
        inflate_reset_keep_adler, inflate_should_update_window, inflate_state_check_impl,
        inflate_state_check_result, inflate_state_is_usable, inflate_state_metadata_is_valid,
        inflate_stream_buffers_are_valid, inflate_stream_has_allocator_callbacks,
        inflate_sync_input_progress, inflate_sync_normalized_wrap, inflate_sync_point_value,
        inflate_sync_remaining_input, inflate_sync_search_core, inflate_trailer_checksum_from_hold,
        inflate_undermine_core, inflate_validate_core, inflate_validate_wrap,
        inflate_zlib_header_error, inflate_zlib_header_transition, inflate_zlib_window_params,
        initial_window_metadata, reset_window_history, stored_block_length, syncsearch_safe,
        update_window_buffer_len, update_window_core, update_window_produced_len,
        window_allocation_failed, window_allocation_plan, window_allocation_request,
        window_allocation_request_for_plan, window_needs_allocation, window_update_plan,
        DynamicCodeLengthRepeat, InflateBlockKind, InflateCopyProgress, InflateGzipExtraProgress,
        InflateGzipFlags, InflateGzipFlagsError, InflateMatchPlan, InflateMatchSource,
        InflateOutputChecksum, InflatePrimeUpdate, InflateSyncSearch, InflateZlibHeaderError,
        InflateZlibHeaderTransition, InflateZlibWindowParams, WindowAllocationPlan, BAD, CHECK,
        CODE_LENGTH_ORDER, COPY_, COPY_1, DICT, DICTID, HEAD, LEN_, MATCH, STORED, SYNC, TYPE,
        TYPEDO,
    };

    #[test]
    fn inflate_gzip_extra_progress_preserves_partial_input() {
        assert_eq!(
            inflate_gzip_extra_progress(8, 3),
            InflateGzipExtraProgress {
                copy: 3,
                remaining: 5,
            }
        );
        assert_eq!(
            inflate_gzip_extra_progress(5, 0),
            InflateGzipExtraProgress {
                copy: 0,
                remaining: 5,
            }
        );
    }

    #[test]
    fn inflate_alignment_discards_only_partial_low_bytes() {
        assert_eq!(inflate_align_to_byte_boundary(0x1234, 0), (0x1234, 0));
        assert_eq!(inflate_align_to_byte_boundary(0x1234, 8), (0x1234, 8));
        assert_eq!(inflate_align_to_byte_boundary(0x1234, 11), (0x246, 8));
        assert_eq!(inflate_align_to_byte_boundary(0x1234, 7), (0x24, 0));
    }

    #[test]
    fn inflate_gzip_extra_progress_completes_with_available_input() {
        assert_eq!(
            inflate_gzip_extra_progress(5, 8),
            InflateGzipExtraProgress {
                copy: 5,
                remaining: 0,
            }
        );
        assert_eq!(
            inflate_gzip_extra_progress(0, 0),
            InflateGzipExtraProgress {
                copy: 0,
                remaining: 0,
            }
        );
    }

    #[test]
    fn gzip_extra_copy_bounds_preserves_destination_limits() {
        assert_eq!(gzip_extra_copy_bounds(10, 8, 5, 2), Some((2, 2)));
        assert_eq!(gzip_extra_copy_bounds(10, 7, 5, 8), Some((3, 2)));
        assert_eq!(gzip_extra_copy_bounds(10, 5, 5, 1), None);
        assert_eq!(gzip_extra_copy_bounds(0, 1, 5, 1), None);
    }

    #[test]
    fn inflate_fast_path_requires_input_and_output_boundaries() {
        assert!(!inflate_can_use_fast_path(5, 258));
        assert!(!inflate_can_use_fast_path(6, 257));
        assert!(inflate_can_use_fast_path(6, 258));
        assert!(inflate_can_use_fast_path(7, 259));
        assert!(inflate_can_use_fast_path(
            ::core::ffi::c_uint::MAX,
            ::core::ffi::c_uint::MAX
        ));
    }

    #[test]
    fn inflate_reset2_discards_only_existing_mismatched_windows() {
        assert!(!inflate_reset2_discards_window(false, 15, 14));
        assert!(!inflate_reset2_discards_window(true, 15, 15));
        assert!(inflate_reset2_discards_window(true, 15, 14));
    }

    #[test]
    fn inflate_stream_buffers_require_output_and_input_when_available() {
        assert!(inflate_stream_buffers_are_valid(true, true, 4));
        assert!(inflate_stream_buffers_are_valid(true, false, 0));
        assert!(!inflate_stream_buffers_are_valid(true, false, 1));
        assert!(!inflate_stream_buffers_are_valid(false, true, 0));
    }

    #[test]
    fn inflate_dictionary_id_reverses_the_four_dictid_bytes() {
        assert_eq!(inflate_dictionary_id_from_hold(0x7856_3412), 0x1234_5678);
        assert_eq!(inflate_dictionary_id_from_hold(0x0102_0408), 0x0804_0201);
        assert_eq!(inflate_dictionary_id_from_hold(0xffff_ffff), 0xffff_ffff);
    }

    #[test]
    fn gzip_header_requires_wrapper_support_and_magic_bytes() {
        assert!(inflate_is_gzip_header(2, 0x8b1f));
        assert!(inflate_is_gzip_header(3, 0x8b1f));
        assert!(!inflate_is_gzip_header(1, 0x8b1f));
        assert!(!inflate_is_gzip_header(2, 0x1f8b));
    }

    #[test]
    fn inflate_gzip_window_bits_defaults_only_zero_width() {
        assert_eq!(inflate_gzip_window_bits(0), 15);
        assert_eq!(inflate_gzip_window_bits(8), 8);
        assert_eq!(inflate_gzip_window_bits(15), 15);
    }

    #[test]
    fn inflate_reset_keep_adler_preserves_wrapper_specific_reset_behavior() {
        assert_eq!(inflate_reset_keep_adler(0), None);
        assert_eq!(inflate_reset_keep_adler(1), Some(1));
        assert_eq!(inflate_reset_keep_adler(2), Some(0));
        assert_eq!(inflate_reset_keep_adler(3), Some(1));
    }

    #[test]
    fn inflate_match_copy_plan_selects_output_and_window_sources() {
        assert_eq!(
            inflate_match_copy_plan(3, 5, 0, 0, 0, 9, 4, true),
            InflateMatchPlan::Copy {
                source: InflateMatchSource::Output { offset: 3 },
                count: 4,
                remaining_output: 0,
                remaining_length: 5,
            }
        );
        assert_eq!(
            inflate_match_copy_plan(9, 2, 8, 6, 8, 3, 5, true),
            InflateMatchPlan::Copy {
                source: InflateMatchSource::Window { index: 7 },
                count: 3,
                remaining_output: 2,
                remaining_length: 0,
            }
        );
        assert_eq!(
            inflate_match_copy_plan(5, 2, 3, 6, 8, 7, 4, true),
            InflateMatchPlan::Copy {
                source: InflateMatchSource::Window { index: 3 },
                count: 4,
                remaining_output: 0,
                remaining_length: 3,
            }
        );
    }

    #[test]
    fn inflate_match_copy_plan_rejects_only_sane_out_of_window_distances() {
        assert_eq!(
            inflate_match_copy_plan(10, 2, 7, 4, 8, 3, 3, true),
            InflateMatchPlan::InvalidDistance
        );
        assert_eq!(
            inflate_match_copy_plan(10, 2, 7, 4, 8, 3, 3, false),
            InflateMatchPlan::Copy {
                source: InflateMatchSource::Window { index: 4 },
                count: 3,
                remaining_output: 0,
                remaining_length: 0,
            }
        );
    }

    #[test]
    fn inflate_match_completion_requires_no_remaining_length() {
        assert!(inflate_match_is_complete(0));
        assert!(!inflate_match_is_complete(1));
        assert!(!inflate_match_is_complete(::core::ffi::c_uint::MAX));
    }

    #[test]
    fn inflate_output_match_copy_handles_nonoverlapping_matches() {
        let mut output = *b"abc___";

        assert!(inflate_copy_match_from_output(&mut output, 3, 3, 3));
        assert_eq!(output, *b"abcabc");
    }

    #[test]
    fn inflate_output_match_copy_preserves_forward_overlap_semantics() {
        let mut output = *b"abcd_____";

        assert!(inflate_copy_match_from_output(&mut output, 4, 3, 5));
        assert_eq!(output, *b"abcdbcdbc");
    }

    #[test]
    fn inflate_output_match_copy_repeats_distance_one() {
        let mut output = *b"a_____";

        assert!(inflate_copy_match_from_output(&mut output, 1, 1, 5));
        assert_eq!(output, *b"aaaaaa");
    }

    #[test]
    fn inflate_output_match_copy_rejects_zero_distance_without_mutation() {
        let mut output = *b"abc___";
        let original = output;

        assert!(!inflate_copy_match_from_output(&mut output, 3, 0, 3));
        assert_eq!(output, original);
    }

    #[test]
    fn inflate_output_match_copy_rejects_too_far_distance_without_mutation() {
        let mut output = *b"abc___";
        let original = output;

        assert!(!inflate_copy_match_from_output(&mut output, 3, 4, 1));
        assert_eq!(output, original);
    }

    #[test]
    fn inflate_output_match_copy_rejects_short_tail_without_mutation() {
        let mut output = *b"abc__";
        let original = output;

        assert!(!inflate_copy_match_from_output(&mut output, 3, 3, 3));
        assert_eq!(output, original);
    }

    #[test]
    fn inflate_output_match_copy_allows_zero_count_without_mutation() {
        let mut output = *b"abc";
        let original = output;

        assert!(inflate_copy_match_from_output(&mut output, 3, 1, 0));
        assert_eq!(output, original);
    }

    #[test]
    fn inflate_data_type_value_sets_expected_flags() {
        assert_eq!(inflate_data_type_value(5, 0, HEAD), 5);
        assert_eq!(inflate_data_type_value(5, 1, HEAD), 5 + 64);
        assert_eq!(inflate_data_type_value(5, 1, TYPE), 5 + 64 + 128);
        assert_eq!(inflate_data_type_value(5, 0, LEN_), 5 + 256);
        assert_eq!(inflate_data_type_value(5, 0, COPY_), 5 + 256);
    }

    #[test]
    fn inflate_mode_data_type_flags_match_mode() {
        assert_eq!(inflate_mode_data_type_flags(HEAD), 0);
        assert_eq!(inflate_mode_data_type_flags(SYNC), 0);
        assert_eq!(inflate_mode_data_type_flags(TYPE), 128);
        assert_eq!(inflate_mode_data_type_flags(LEN_), 256);
        assert_eq!(inflate_mode_data_type_flags(COPY_), 256);
    }

    #[test]
    fn inflate_dictionary_permission_matches_wrapper_and_mode() {
        assert!(inflate_dictionary_is_allowed(0, HEAD));
        assert!(inflate_dictionary_is_allowed(0, DICT));
        assert!(inflate_dictionary_is_allowed(1, DICT));
        assert!(!inflate_dictionary_is_allowed(1, HEAD));
        assert!(!inflate_dictionary_is_allowed(4, BAD));
    }

    #[test]
    fn inflate_header_crc_requires_gzip_header_and_checksum_wrapping() {
        assert!(inflate_header_crc_enabled(0x200, 4));
        assert!(inflate_header_crc_enabled(0x600, 5));
        assert!(!inflate_header_crc_enabled(0, 4));
        assert!(!inflate_header_crc_enabled(0x200, 0));
        assert!(!inflate_header_crc_enabled(0x400, 2));
    }

    #[test]
    fn inflate_gzip_flags_classification_decodes_values_and_preserves_validation_order() {
        assert_eq!(
            inflate_gzip_flags(0x108),
            InflateGzipFlags {
                flags: 0x108,
                text: 1,
            }
        );
        assert_eq!(inflate_gzip_flags_error(0x108), None);
        assert_eq!(
            inflate_gzip_flags_error(0xe009),
            Some(InflateGzipFlagsError::UnknownCompressionMethod)
        );
        assert_eq!(
            inflate_gzip_flags_error(0xe008),
            Some(InflateGzipFlagsError::UnknownHeaderFlags)
        );
    }

    #[test]
    fn inflate_gzip_flags_validation_combines_decoding_with_ordered_errors() {
        assert_eq!(
            inflate_gzip_flags_validation(0x108),
            (
                InflateGzipFlags {
                    flags: 0x108,
                    text: 1,
                },
                None,
            )
        );
        assert_eq!(
            inflate_gzip_flags_validation(0xe009),
            (
                InflateGzipFlags {
                    flags: 0xe009,
                    text: 0,
                },
                Some(InflateGzipFlagsError::UnknownCompressionMethod),
            )
        );
        assert_eq!(
            inflate_gzip_flags_validation(0xe008),
            (
                InflateGzipFlags {
                    flags: 0xe008,
                    text: 0,
                },
                Some(InflateGzipFlagsError::UnknownHeaderFlags),
            )
        );
    }

    #[test]
    fn inflate_gzip_header_extra_flag_requires_the_extra_bit() {
        assert!(inflate_gzip_header_has_extra(0x400));
        assert!(inflate_gzip_header_has_extra(0x600));
        assert!(!inflate_gzip_header_has_extra(0));
        assert!(!inflate_gzip_header_has_extra(0x200));
    }

    #[test]
    fn inflate_gzip_header_crc_flag_requires_the_crc_bit() {
        assert!(inflate_gzip_header_has_crc(0x200));
        assert!(inflate_gzip_header_has_crc(0x1e00));
        assert!(!inflate_gzip_header_has_crc(0));
        assert!(!inflate_gzip_header_has_crc(0x400));
        assert!(!inflate_gzip_header_has_crc(0x800));
        assert!(!inflate_gzip_header_has_crc(0x1000));
    }

    #[test]
    fn inflate_gzip_header_name_flag_requires_the_name_bit() {
        assert!(inflate_gzip_header_has_name(0x800));
        assert!(inflate_gzip_header_has_name(0x1800));
        assert!(!inflate_gzip_header_has_name(0));
        assert!(!inflate_gzip_header_has_name(0x400));
        assert!(!inflate_gzip_header_has_name(0x1000));
    }

    #[test]
    fn inflate_gzip_header_comment_flag_requires_the_comment_bit() {
        assert!(inflate_gzip_header_has_comment(0x1000));
        assert!(inflate_gzip_header_has_comment(0x1800));
        assert!(!inflate_gzip_header_has_comment(0));
        assert!(!inflate_gzip_header_has_comment(0x400));
        assert!(!inflate_gzip_header_has_comment(0x800));
    }

    #[test]
    fn inflate_gzip_header_crc_validation_honors_wrap_and_low_16_bits() {
        assert!(inflate_gzip_header_crc_is_valid(0, 0, 0xbeef));
        assert!(inflate_gzip_header_crc_is_valid(4, 0xbeef, 0x1234_beef));
        assert!(!inflate_gzip_header_crc_is_valid(4, 0xbeef, 0x1234_dead));
    }

    #[test]
    fn inflate_zlib_header_classification_preserves_validation_order() {
        assert_eq!(inflate_zlib_header_error(1, 0x9c78), None);
        assert_eq!(
            inflate_zlib_header_error(0, 0x9c78),
            Some(InflateZlibHeaderError::IncorrectCheck)
        );
        assert_eq!(
            inflate_zlib_header_error(1, 0x9d78),
            Some(InflateZlibHeaderError::IncorrectCheck)
        );
        assert_eq!(
            inflate_zlib_header_error(1, 0x0977),
            Some(InflateZlibHeaderError::UnknownCompressionMethod)
        );
    }

    #[test]
    fn inflate_zlib_window_params_negotiates_window_and_dictionary_mode() {
        assert_eq!(
            inflate_zlib_window_params(7, 0),
            Some(InflateZlibWindowParams {
                wbits: 15,
                dmax: 32_768,
                next_mode: TYPE,
            })
        );
        assert_eq!(
            inflate_zlib_window_params(0x207, 15),
            Some(InflateZlibWindowParams {
                wbits: 15,
                dmax: 32_768,
                next_mode: DICTID,
            })
        );
    }

    #[test]
    fn inflate_zlib_window_params_rejects_oversized_headers() {
        assert_eq!(inflate_zlib_window_params(8, 0), None);
        assert_eq!(inflate_zlib_window_params(3, 10), None);
    }

    #[test]
    fn inflate_zlib_header_transition_preserves_error_and_window_consumption_order() {
        assert_eq!(
            inflate_zlib_header_transition(0, 0x9c78, 16, 0),
            InflateZlibHeaderTransition::Error(InflateZlibHeaderError::IncorrectCheck)
        );
        assert_eq!(
            inflate_zlib_header_transition(1, 0x0977, 16, 0),
            InflateZlibHeaderTransition::Error(InflateZlibHeaderError::UnknownCompressionMethod)
        );
        assert_eq!(
            inflate_zlib_header_transition(1, 0x1c88, 16, 0),
            InflateZlibHeaderTransition::InvalidWindow {
                hold: 0x1c8,
                bits: 12,
            }
        );
        assert_eq!(
            inflate_zlib_header_transition(1, 0x9c78, 16, 0),
            InflateZlibHeaderTransition::Accepted(InflateZlibWindowParams {
                wbits: 15,
                dmax: 32_768,
                next_mode: TYPE,
            })
        );
    }

    #[test]
    fn inflate_block_header_classifies_and_consumes_three_bits() {
        let stored = inflate_block_header(0b000, 3);
        assert_eq!(stored.last, 0);
        assert_eq!(stored.kind, InflateBlockKind::Stored);
        assert_eq!(stored.hold, 0);
        assert_eq!(stored.bits, 0);

        let fixed = inflate_block_header(0b011, 3);
        assert_eq!(fixed.last, 1);
        assert_eq!(fixed.kind, InflateBlockKind::Fixed);

        assert_eq!(
            inflate_block_header(0b100, 3).kind,
            InflateBlockKind::Dynamic
        );
        assert_eq!(
            inflate_block_header(0b111, 3).kind,
            InflateBlockKind::Invalid
        );
    }

    #[test]
    fn inflate_block_header_preserves_remaining_bit_buffer() {
        let header = inflate_block_header(0b101_101, 9);

        assert_eq!(header.last, 1);
        assert_eq!(header.kind, InflateBlockKind::Dynamic);
        assert_eq!(header.hold, 0b101);
        assert_eq!(header.bits, 6);
    }

    #[test]
    fn dictionary_copy_preserves_contiguous_window_order() {
        let window = *b"abcdefgh";
        let mut dictionary = [0; 5];

        copy_dictionary_from_window(&window, 0, &mut dictionary);

        assert_eq!(dictionary, *b"abcde");
    }

    #[test]
    fn dictionary_copy_wraps_from_window_end_to_start() {
        let window = *b"YZcdefWX";
        let mut dictionary = [0; 8];

        copy_dictionary_from_window(&window, 2, &mut dictionary);

        assert_eq!(dictionary, *b"cdefWXYZ");
    }

    #[test]
    fn dictionary_copy_ignores_an_empty_dictionary() {
        let mut dictionary = [];

        copy_dictionary_from_window(b"abc", 3, &mut dictionary);

        assert!(dictionary.is_empty());
    }

    #[test]
    fn dictionary_result_copies_window_and_reports_length() {
        let window = *b"YZcdefWX";
        let mut dictionary = [0; 8];
        let mut length = 0;

        assert_eq!(
            inflate_get_dictionary_result(
                8,
                2,
                Some(&window),
                Some(&mut dictionary),
                Some(&mut length),
            ),
            crate::zlib_h::Z_OK
        );
        assert_eq!(dictionary, *b"cdefWXYZ");
        assert_eq!(length, 8);
    }

    #[test]
    fn dictionary_result_allows_null_output_equivalents() {
        let mut length = 0;

        assert_eq!(
            inflate_get_dictionary_result(5, 0, None, None, Some(&mut length)),
            crate::zlib_h::Z_OK
        );
        assert_eq!(length, 5);
        assert_eq!(
            inflate_get_dictionary_result(5, 0, None, None, None),
            crate::zlib_h::Z_OK
        );
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
    fn stored_block_length_requires_complementary_nlen() {
        assert_eq!(stored_block_length(0xedcb_1234), Some(0x1234));
        assert_eq!(stored_block_length(0xffff_0000), Some(0));
        assert_eq!(stored_block_length(0x1234_1234), None);
        assert_eq!(stored_block_length(0x0000_0001), None);
    }

    #[test]
    fn inflate_undermine_core_marks_stream_sane_and_returns_data_error() {
        let mut sane = 0;

        assert_eq!(
            inflate_undermine_core(&mut sane),
            crate::zlib_h::Z_DATA_ERROR
        );
        assert_eq!(sane, 1);
    }

    #[test]
    fn inflate_validate_wrap_updates_only_the_validation_bit() {
        assert_eq!(inflate_validate_wrap(1, 1), 5);
        assert_eq!(inflate_validate_wrap(4, -1), 4);
        assert_eq!(inflate_validate_wrap(9, 0), 9);
        assert_eq!(inflate_validate_wrap(0, 1), 0);
    }

    #[test]
    fn inflate_validate_core_updates_wrap_in_place() {
        for (mut wrap, check, expected) in [(1, 1, 5), (4, -1, 4), (9, 0, 9), (0, 1, 0)] {
            assert_eq!(inflate_validate_core(&mut wrap, check), crate::zlib_h::Z_OK);
            assert_eq!(wrap, expected);
        }
    }

    #[test]
    fn inflate_header_wrap_requires_gzip_capture_bit() {
        assert!(!inflate_header_wrap_allows_capture(0));
        assert!(inflate_header_wrap_allows_capture(2));
        assert!(inflate_header_wrap_allows_capture(3));
        assert!(!inflate_header_wrap_allows_capture(4));
    }

    #[test]
    fn inflate_window_update_gate_preserves_mode_and_flush_boundaries() {
        assert!(inflate_should_update_window(
            1,
            8,
            8,
            BAD,
            crate::zlib_h::Z_FINISH
        ));
        assert!(!inflate_should_update_window(
            0,
            8,
            8,
            TYPE,
            crate::zlib_h::Z_NO_FLUSH
        ));
        assert!(inflate_should_update_window(
            0,
            8,
            7,
            TYPE,
            crate::zlib_h::Z_FINISH
        ));
        assert!(!inflate_should_update_window(
            0,
            8,
            7,
            BAD,
            crate::zlib_h::Z_NO_FLUSH
        ));
        assert!(!inflate_should_update_window(
            0,
            8,
            7,
            CHECK,
            crate::zlib_h::Z_FINISH
        ));
        assert!(inflate_should_update_window(
            0,
            8,
            7,
            CHECK,
            crate::zlib_h::Z_NO_FLUSH
        ));
    }

    #[test]
    fn inflate_sync_point_value_requires_stored_mode_without_pending_bits() {
        assert_eq!(inflate_sync_point_value(STORED as u32, 0), 1);
        assert_eq!(inflate_sync_point_value(STORED as u32, 1), 0);
        assert_eq!(inflate_sync_point_value(HEAD as u32, 0), 0);
        assert_eq!(inflate_sync_point_value(u32::MAX, 0), 0);
    }

    #[test]
    fn inflate_sync_point_ffi_rejects_a_null_stream() {
        assert_eq!(
            unsafe { inflateSyncPoint_ffi(::core::ptr::null_mut()) },
            crate::zlib_h::Z_STREAM_ERROR
        );
    }

    #[test]
    fn inflate_codes_used_offset_preserves_negative_sentinel() {
        assert_eq!(
            inflate_codes_used_offset_value(-1),
            ::core::ffi::c_ulong::MAX
        );
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
    fn dynamic_code_length_repeat_fits_rejects_wrapping_progress() {
        assert!(!dynamic_code_length_repeat_fits(
            ::core::ffi::c_uint::MAX,
            1,
            0,
            0,
        ));
    }

    #[test]
    fn dynamic_code_length_repeat_spec_preserves_deflate_repeat_symbols() {
        assert_eq!(
            dynamic_code_length_repeat_spec(16),
            DynamicCodeLengthRepeat {
                base: 3,
                extra_bits: 2,
                repeats_previous: true,
            }
        );
        assert_eq!(
            dynamic_code_length_repeat_spec(17),
            DynamicCodeLengthRepeat {
                base: 3,
                extra_bits: 3,
                repeats_previous: false,
            }
        );
        assert_eq!(
            dynamic_code_length_repeat_spec(18),
            DynamicCodeLengthRepeat {
                base: 11,
                extra_bits: 7,
                repeats_previous: false,
            }
        );
    }

    #[test]
    fn dynamic_code_length_repeat_spec_preserves_invalid_symbol_fallback() {
        assert_eq!(
            dynamic_code_length_repeat_spec(::core::ffi::c_ushort::MAX),
            dynamic_code_length_repeat_spec(18)
        );
    }

    #[test]
    fn inflate_sync_remaining_input_uses_consumed_slice_prefix() {
        let input = *b"marker";

        assert_eq!(inflate_sync_remaining_input(&input, 0), b"marker");
        assert_eq!(inflate_sync_remaining_input(&input, 2), b"rker");
        assert_eq!(inflate_sync_remaining_input(&input, input.len()), b"");
    }

    #[test]
    fn inflate_sync_input_progress_preserves_wrapping_counters() {
        assert_eq!(inflate_sync_input_progress(8, 12, 3), (5, 15));
        assert_eq!(
            inflate_sync_input_progress(0, crate::stdlib::uLong::MAX, 0),
            (0, crate::stdlib::uLong::MAX)
        );
        assert_eq!(
            inflate_sync_input_progress(1, crate::stdlib::uLong::MAX, 2),
            (crate::stdlib::uInt::MAX, 1)
        );
    }

    #[test]
    fn inflate_sync_normalized_wrap_resets_unknown_headers_and_validation() {
        assert_eq!(inflate_sync_normalized_wrap(-1, 7), 0);
        assert_eq!(inflate_sync_normalized_wrap(0, 7), 3);
        assert_eq!(inflate_sync_normalized_wrap(42, 9), 9);
    }

    #[test]
    fn inflate_copy_progress_tracks_all_post_copy_counters() {
        assert_eq!(
            inflate_copy_progress(8, 7, 6),
            InflateCopyProgress {
                copied: 6,
                remaining_input: 1,
                remaining_output: 0,
                remaining_length: 2,
            }
        );
        assert_eq!(
            inflate_copy_progress(0, 8, 8),
            InflateCopyProgress {
                copied: 0,
                remaining_input: 8,
                remaining_output: 8,
                remaining_length: 0,
            }
        );
        assert_eq!(
            inflate_copy_progress(8, 0, 8),
            InflateCopyProgress {
                copied: 0,
                remaining_input: 0,
                remaining_output: 8,
                remaining_length: 8,
            }
        );
        assert_eq!(
            inflate_copy_progress(8, 8, 0),
            InflateCopyProgress {
                copied: 0,
                remaining_input: 8,
                remaining_output: 0,
                remaining_length: 8,
            }
        );
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
        assert_eq!(
            inflate_prime_update(0, 33, 1, 0),
            InflatePrimeUpdate::StreamError
        );
        assert_eq!(
            inflate_prime_update(0, ::core::ffi::c_uint::MAX, 1, 0),
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
    fn inflate_stream_requires_both_allocator_callbacks() {
        assert!(inflate_stream_has_allocator_callbacks(true, true));
        assert!(!inflate_stream_has_allocator_callbacks(false, true));
        assert!(!inflate_stream_has_allocator_callbacks(true, false));
        assert!(!inflate_stream_has_allocator_callbacks(false, false));
    }

    #[test]
    fn inflate_state_usability_requires_callbacks_and_valid_metadata() {
        assert!(inflate_state_is_usable(true, true, true, HEAD));
        assert!(!inflate_state_is_usable(false, true, true, HEAD));
        assert!(!inflate_state_is_usable(true, false, true, HEAD));
        assert!(!inflate_state_is_usable(true, true, false, HEAD));
        assert!(!inflate_state_is_usable(true, true, true, SYNC + 1));
    }

    #[test]
    fn inflate_state_check_result_requires_stream_state_and_usable_metadata() {
        let cases = [
            (false, false, false, 1, "null stream"),
            (
                false,
                false,
                true,
                1,
                "null stream with otherwise usable state",
            ),
            (false, true, false, 1, "null stream with state"),
            (false, true, true, 1, "null stream with usable state"),
            (true, false, false, 1, "null state"),
            (true, false, true, 1, "null state with usable metadata"),
            (true, true, false, 1, "invalid metadata"),
            (true, true, true, 0, "valid state"),
        ];

        for (has_stream, has_state, state_is_usable, expected, scenario) in cases {
            assert_eq!(
                inflate_state_check_result(has_stream, has_state, state_is_usable),
                expected,
                "{scenario}"
            );
        }
    }

    #[test]
    fn inflate_state_check_impl_rejects_missing_stream_or_state() {
        let stream = crate::zlib_h::z_stream {
            next_in: ::core::ptr::null_mut(),
            avail_in: 0,
            total_in: 0,
            next_out: ::core::ptr::null_mut(),
            avail_out: 0,
            total_out: 0,
            msg: ::core::ptr::null_mut(),
            state: ::core::ptr::null_mut(),
            zalloc: None,
            zfree: None,
            opaque: ::core::ptr::null_mut(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        };

        assert_eq!(inflate_state_check_impl(None, None, false), 1);
        assert_eq!(inflate_state_check_impl(Some(&stream), None, false), 1);
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
    fn inflate_mark_progress_isolated_from_backtracking_prefix() {
        assert_eq!(inflate_mark_progress(COPY_1, 7, 99), 7);
        assert_eq!(inflate_mark_progress(MATCH, 7, 10), 3);
        assert_eq!(inflate_mark_progress(HEAD, 7, 10), 0);
        assert_eq!(inflate_mark_progress(MATCH, 5, 2), 2_u32.wrapping_sub(5));
    }

    #[test]
    fn inflate_mark_rejects_a_null_stream() {
        assert_eq!(
            unsafe { super::inflateMark_ffi(::core::ptr::null_mut()) },
            -(1 << 16)
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
    fn inflate_mode_on_entry_only_advances_type() {
        assert_eq!(inflate_mode_on_entry(TYPE), TYPEDO);
        assert_eq!(inflate_mode_on_entry(HEAD), HEAD);
        assert_eq!(inflate_mode_on_entry(LEN_), LEN_);
    }

    #[test]
    fn inflate_head_skip_mode_only_skips_raw_deflate() {
        assert_eq!(inflate_head_skip_mode(0), Some(TYPEDO));
        assert_eq!(inflate_head_skip_mode(1), None);
        assert_eq!(inflate_head_skip_mode(-1), None);
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
    fn reset_window_history_clears_all_history_metadata() {
        let mut wsize = 32_768;
        let mut whave = 16_384;
        let mut wnext = 8_192;

        reset_window_history(&mut wsize, &mut whave, &mut wnext);

        assert_eq!(wsize, 0);
        assert_eq!(whave, 0);
        assert_eq!(wnext, 0);
    }

    #[test]
    fn window_allocation_is_needed_only_without_a_window() {
        assert!(window_needs_allocation(false));
        assert!(!window_needs_allocation(true));
    }

    #[test]
    fn window_allocation_failure_requires_a_missing_allocated_window() {
        let allocation = window_allocation_plan(false, 15);
        assert!(window_allocation_failed(allocation, false));
        assert!(!window_allocation_failed(allocation, true));
        assert!(!window_allocation_failed(
            window_allocation_plan(true, 15),
            false
        ));
    }

    #[test]
    fn window_allocation_request_matches_supported_window_widths() {
        assert_eq!(window_allocation_request(8), (256, 1));
        assert_eq!(window_allocation_request(15), (32_768, 1));
    }

    #[test]
    fn window_allocation_plan_distinguishes_existing_and_required_windows() {
        assert_eq!(
            window_allocation_plan(true, 15),
            WindowAllocationPlan::Existing
        );
        assert_eq!(
            window_allocation_plan(false, 8),
            WindowAllocationPlan::Allocate {
                items: 256,
                size: 1,
            }
        );
    }

    #[test]
    fn window_allocation_request_for_plan_preserves_allocation_branch() {
        assert_eq!(
            window_allocation_request_for_plan(window_allocation_plan(false, 8)),
            Some((256, 1))
        );
        assert_eq!(
            window_allocation_request_for_plan(window_allocation_plan(true, 15)),
            None
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

    #[test]
    fn window_update_core_initializes_metadata_before_copying() {
        let mut window = [0; 8];
        let mut wsize = 0;
        let mut wnext = 5;
        let mut whave = 4;

        update_window_core(3, &mut wsize, &mut wnext, &mut whave, &mut window, b"xyz");

        assert_eq!(wsize, 8);
        assert_eq!(wnext, 3);
        assert_eq!(whave, 3);
        assert_eq!(&window[..3], b"xyz");
    }

    #[test]
    fn window_update_core_preserves_initialized_history_for_empty_output() {
        let mut window = *b"abcdefgh";
        let mut wsize = 8;
        let mut wnext = 3;
        let mut whave = 5;

        update_window_core(3, &mut wsize, &mut wnext, &mut whave, &mut window, b"");

        assert_eq!((wsize, wnext, whave), (8, 3, 5));
        assert_eq!(window, *b"abcdefgh");
    }

    #[test]
    fn update_window_buffer_len_preserves_c_uint_widths() {
        assert_eq!(update_window_buffer_len(0), 0);
        assert_eq!(update_window_buffer_len(32_768), 32_768);
        assert_eq!(
            update_window_buffer_len(::core::ffi::c_uint::MAX),
            ::core::ffi::c_uint::MAX as usize
        );
    }

    #[test]
    fn update_window_produced_len_only_constructs_nonempty_slices() {
        assert_eq!(update_window_produced_len(0), None);
        assert_eq!(update_window_produced_len(1), Some(1));
        assert_eq!(
            update_window_produced_len(::core::ffi::c_uint::MAX),
            Some(::core::ffi::c_uint::MAX as usize)
        );
    }

    #[test]
    fn update_window_produced_slice_uses_empty_slice_when_output_is_absent() {
        assert_eq!(super::update_window_produced_slice(None), b"");
    }

    #[test]
    fn update_window_produced_slice_preserves_output_reference() {
        let produced = *b"xyz";

        assert_eq!(super::update_window_produced_slice(Some(&produced)), b"xyz");
    }

    #[test]
    fn cursor_progress_tracks_input_output_and_wrapping() {
        assert_eq!(super::inflate_cursor_progress(16, 7), 9);
        assert_eq!(super::inflate_cursor_progress(8, 3), 5);
        assert_eq!(
            super::inflate_cursor_progress(0, 1),
            ::core::ffi::c_uint::MAX
        );
    }

    #[test]
    fn output_checksum_selection_uses_wrap_length_and_flags() {
        assert_eq!(
            inflate_output_checksum(5, 0, 1),
            Some(InflateOutputChecksum::Adler32)
        );
        assert_eq!(
            inflate_output_checksum(4, 1, 1),
            Some(InflateOutputChecksum::Crc32)
        );
        assert_eq!(
            inflate_output_checksum(4, -1, 1),
            Some(InflateOutputChecksum::Crc32)
        );
        assert_eq!(inflate_output_checksum(3, 1, 1), None);
        assert_eq!(inflate_output_checksum(4, 1, 0), None);
    }

    #[test]
    fn trailer_checksum_uses_gzip_order_or_zlib_byte_order() {
        let hold = 0x7856_3412;

        assert_eq!(inflate_trailer_checksum_from_hold(1, hold), hold);
        assert_eq!(inflate_trailer_checksum_from_hold(-1, hold), hold);
        assert_eq!(inflate_trailer_checksum_from_hold(0, hold), 0x1234_5678);
    }

    #[test]
    fn gzip_length_check_requires_wrapped_gzip_streams() {
        assert!(inflate_gzip_length_check_required(1, 1));
        assert!(inflate_gzip_length_check_required(4, -1));
        assert!(!inflate_gzip_length_check_required(0, 1));
        assert!(!inflate_gzip_length_check_required(4, 0));
    }

    #[test]
    fn checksum_cursor_preserves_the_adler_output_range() {
        let output = *b"hello";
        let first_checksum =
            unsafe { crate::src::adler32::adler32_ffi(1, output[..2].as_ptr(), 2) };
        let checksum_cursor = output[2..].as_ptr();
        let cursor_checksum =
            unsafe { crate::src::adler32::adler32_ffi(first_checksum, checksum_cursor, 3) };
        let whole_output_checksum = unsafe {
            crate::src::adler32::adler32_ffi(
                1,
                output.as_ptr(),
                output.len() as crate::stdlib::uInt,
            )
        };

        assert_eq!(cursor_checksum, whole_output_checksum);
        assert_eq!(cursor_checksum, 0x062c_0215);
    }
}
