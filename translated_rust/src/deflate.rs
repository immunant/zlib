// =============== BEGIN deflate_h ================
pub use crate::src::trees::static_tree_desc_s;

pub const LENGTH_CODES: ::core::ffi::c_int = 29 as ::core::ffi::c_int;

pub const LITERALS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;

pub const L_CODES: ::core::ffi::c_int =
    crate::src::deflate::LITERALS + 1 as ::core::ffi::c_int + crate::src::deflate::LENGTH_CODES;

pub const D_CODES: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

pub const BL_CODES: ::core::ffi::c_int = 19 as ::core::ffi::c_int;

pub const HEAP_SIZE: ::core::ffi::c_int =
    2 as ::core::ffi::c_int * crate::src::deflate::L_CODES + 1 as ::core::ffi::c_int;

pub const MAX_BITS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const Buf_size: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const INIT_STATE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;

pub const GZIP_STATE: ::core::ffi::c_int = 57 as ::core::ffi::c_int;

pub const EXTRA_STATE: ::core::ffi::c_int = 69 as ::core::ffi::c_int;

pub const NAME_STATE: ::core::ffi::c_int = 73 as ::core::ffi::c_int;

pub const COMMENT_STATE: ::core::ffi::c_int = 91 as ::core::ffi::c_int;

pub const HCRC_STATE: ::core::ffi::c_int = 103 as ::core::ffi::c_int;

pub const BUSY_STATE: ::core::ffi::c_int = 113 as ::core::ffi::c_int;

pub const FINISH_STATE: ::core::ffi::c_int = 666 as ::core::ffi::c_int;

pub type ct_data = crate::src::deflate::ct_data_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ct_data_s {
    pub fc: crate::src::deflate::C2Rust_Unnamed_1,
    pub dl: crate::src::deflate::C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_1 {
    pub freq: crate::zutil_h::ush,
    pub code: crate::zutil_h::ush,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_0 {
    pub dad: crate::zutil_h::ush,
    pub len: crate::zutil_h::ush,
}

pub type static_tree_desc = crate::src::deflate::static_tree_desc_s;

pub type tree_desc = crate::src::deflate::tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tree_desc_s {
    pub max_code: ::core::ffi::c_int,
    pub stat_desc: Option<&'static crate::src::deflate::static_tree_desc>,
}

pub type Pos = crate::zutil_h::ush;

pub type Posf = crate::src::deflate::Pos;

pub type IPos = ::core::ffi::c_uint;

pub type deflate_state = crate::src::deflate::internal_state;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct internal_state {
    pub strm: crate::zlib_h::z_streamp,
    pub status: ::core::ffi::c_int,
    pub pending_buf: *mut crate::stdlib::Bytef,
    pub pending_buf_size: crate::zutil_h::ulg,
    pub pending_out: *mut crate::stdlib::Bytef,
    pub pending: crate::zutil_h::ulg,
    pub wrap: ::core::ffi::c_int,
    pub gzhead: crate::zlib_h::gz_headerp,
    pub gzindex: crate::zutil_h::ulg,
    pub method: crate::stdlib::Byte,
    pub last_flush: ::core::ffi::c_int,
    pub w_size: crate::stdlib::uInt,
    pub w_bits: crate::stdlib::uInt,
    pub w_mask: crate::stdlib::uInt,
    pub window: *mut crate::stdlib::Bytef,
    pub window_size: crate::zutil_h::ulg,
    pub prev: *mut crate::src::deflate::Posf,
    pub head: *mut crate::src::deflate::Posf,
    pub ins_h: crate::stdlib::uInt,
    pub hash_size: crate::stdlib::uInt,
    pub hash_bits: crate::stdlib::uInt,
    pub hash_mask: crate::stdlib::uInt,
    pub hash_shift: crate::stdlib::uInt,
    pub block_start: ::core::ffi::c_long,
    pub match_length: crate::stdlib::uInt,
    pub prev_match: crate::src::deflate::IPos,
    pub match_available: ::core::ffi::c_int,
    pub strstart: crate::stdlib::uInt,
    pub match_start: crate::stdlib::uInt,
    pub lookahead: crate::stdlib::uInt,
    pub prev_length: crate::stdlib::uInt,
    pub max_chain_length: crate::stdlib::uInt,
    pub max_lazy_match: crate::stdlib::uInt,
    pub level: ::core::ffi::c_int,
    pub strategy: ::core::ffi::c_int,
    pub good_match: crate::stdlib::uInt,
    pub nice_match: ::core::ffi::c_int,
    pub dyn_ltree: [crate::src::deflate::ct_data_s; 573],
    pub dyn_dtree: [crate::src::deflate::ct_data_s; 61],
    pub bl_tree: [crate::src::deflate::ct_data_s; 39],
    pub l_desc: crate::src::deflate::tree_desc_s,
    pub d_desc: crate::src::deflate::tree_desc_s,
    pub bl_desc: crate::src::deflate::tree_desc_s,
    pub bl_count: [crate::zutil_h::ush; 16],
    pub heap: [::core::ffi::c_int; 573],
    pub heap_len: ::core::ffi::c_int,
    pub heap_max: ::core::ffi::c_int,
    pub depth: [crate::zutil_h::uch; 573],
    pub sym_buf: *mut crate::zutil_h::uchf,
    pub lit_bufsize: crate::stdlib::uInt,
    pub sym_next: crate::stdlib::uInt,
    pub sym_end: crate::stdlib::uInt,
    pub opt_len: crate::zutil_h::ulg,
    pub static_len: crate::zutil_h::ulg,
    pub matches: crate::stdlib::uInt,
    pub insert: crate::stdlib::uInt,
    pub bi_buf: crate::zutil_h::ush,
    pub bi_valid: ::core::ffi::c_int,
    pub bi_used: ::core::ffi::c_int,
    pub high_water: crate::zutil_h::ulg,
    pub slid: ::core::ffi::c_int,
}

pub const MIN_LOOKAHEAD: ::core::ffi::c_int =
    crate::zutil_h::MAX_MATCH + crate::zutil_h::MIN_MATCH + 1 as ::core::ffi::c_int;

pub const WIN_INIT: ::core::ffi::c_int = crate::zutil_h::MAX_MATCH;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::trees::_dist_code;
pub use crate::src::trees::_length_code;
pub use crate::src::trees::_tr_align;
pub use crate::src::trees::_tr_flush_block;
pub use crate::src::trees::_tr_stored_block;
pub use crate::src::zutil::z_errmsg;
pub use crate::stdlib::charf;

pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::stdlib::MAX_MEM_LEVEL;
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
pub use crate::zlib_h::Z_DEFAULT_COMPRESSION;
pub use crate::zlib_h::Z_DEFAULT_STRATEGY;
pub use crate::zlib_h::Z_DEFLATED;
pub use crate::zlib_h::Z_FILTERED;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_FIXED;
pub use crate::zlib_h::Z_FULL_FLUSH;
pub use crate::zlib_h::Z_HUFFMAN_ONLY;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_PARTIAL_FLUSH;
pub use crate::zlib_h::Z_RLE;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub use crate::zlib_h::Z_UNKNOWN;
pub use crate::zlib_h::Z_VERSION_ERROR;
pub use crate::zutil_h::uch;
pub use crate::zutil_h::uchf;
pub use crate::zutil_h::ulg;
pub use crate::zutil_h::ush;
pub use crate::zutil_h::DEF_MEM_LEVEL;
pub use crate::zutil_h::MAX_MATCH;
pub use crate::zutil_h::MIN_MATCH;
pub use crate::zutil_h::PRESET_DICT;

pub const block_done: block_state = 1;

pub type block_state = ::core::ffi::c_uint;

pub const finish_done: block_state = 3;

pub const finish_started: block_state = 2;

pub const need_more: block_state = 0;

type compress_func = Option<
    unsafe extern "C" fn(
        *mut crate::src::deflate::deflate_state,
        ::core::ffi::c_int,
    ) -> block_state,
>;

#[derive(Copy, Clone, PartialEq, Eq)]
enum CompressorKind {
    Stored,
    Fast,
    Slow,
}

pub type config = config_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct config_s {
    pub good_length: crate::zutil_h::ush,
    pub max_lazy: crate::zutil_h::ush,
    pub nice_length: crate::zutil_h::ush,
    pub max_chain: crate::zutil_h::ush,
    kind: CompressorKind,
}
#[no_mangle]

pub static deflate_copyright: [::core::ffi::c_char; 70] = [
    32, 100, 101, 102, 108, 97, 116, 101, 32, 49, 46, 51, 46, 50, 46, 49, 32, 67, 111, 112, 121,
    114, 105, 103, 104, 116, 32, 49, 57, 57, 53, 45, 50, 48, 50, 54, 32, 74, 101, 97, 110, 45, 108,
    111, 117, 112, 32, 71, 97, 105, 108, 108, 121, 32, 97, 110, 100, 32, 77, 97, 114, 107, 32, 65,
    100, 108, 101, 114, 32, 0,
];

pub const NIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TOO_FAR: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

static configuration_table: [config; 10] = [
    config_s {
        good_length: 0 as crate::zutil_h::ush,
        max_lazy: 0 as crate::zutil_h::ush,
        nice_length: 0 as crate::zutil_h::ush,
        max_chain: 0 as crate::zutil_h::ush,
        kind: CompressorKind::Stored,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        kind: CompressorKind::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        kind: CompressorKind::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        kind: CompressorKind::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        kind: CompressorKind::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        kind: CompressorKind::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        kind: CompressorKind::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        kind: CompressorKind::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        kind: CompressorKind::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        kind: CompressorKind::Slow,
    },
];

pub(crate) fn slide_hash_state(
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    wsize: crate::stdlib::uInt,
) {
    let wsize = wsize as ::core::ffi::c_uint;
    for entry in head.iter_mut().rev().chain(prev.iter_mut().rev()) {
        let value = *entry as ::core::ffi::c_uint;
        *entry = if value >= wsize {
            value.wrapping_sub(wsize) as crate::src::deflate::Posf
        } else {
            NIL as crate::src::deflate::Posf
        };
    }
}

pub(crate) fn clear_hash_state(
    head: &mut [crate::src::deflate::Posf],
    slid: &mut ::core::ffi::c_int,
) {
    head.fill(NIL as crate::src::deflate::Posf);
    *slid = 0;
}

/// Translate the legacy state-owned hash allocations into temporary slices for
/// the pre-existing private window-fill adapter.  Export boundaries call the
/// slice core directly instead.
pub(crate) unsafe extern "C" fn slide_hash(mut s: *mut crate::src::deflate::deflate_state) {
    if s.is_null() {
        return;
    }
    // This legacy adapter still has to lend the callback-allocated hash
    // buffers, but adopt the validated state record once so the remainder of
    // the state transition uses ordinary field access.
    let s = &mut *s;
    let Ok(head_len) = usize::try_from(s.hash_size) else {
        return;
    };
    let Ok(prev_len) = usize::try_from(s.w_size) else {
        return;
    };
    if (head_len != 0 && s.head.is_null()) || (prev_len != 0 && s.prev.is_null()) {
        return;
    }
    let head = if head_len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(s.head, head_len)
    };
    let prev = if prev_len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(s.prev, prev_len)
    };
    slide_hash_state(head, prev, s.w_size);
    s.slid = 1;
}

fn read_buf_state(
    input: &[crate::stdlib::Byte],
    output: &mut [crate::stdlib::Byte],
    wrap: ::core::ffi::c_int,
    adler: crate::stdlib::uLong,
) -> (::core::ffi::c_uint, crate::stdlib::uLong) {
    let len = input.len().min(output.len());
    output[..len].copy_from_slice(&input[..len]);
    let adler = match wrap {
        1 => crate::src::adler32::adler32_slice(adler, &output[..len]),
        2 => crate::src::crc32::crc32_slice(adler, &output[..len]),
        _ => adler,
    };
    (len as ::core::ffi::c_uint, adler)
}

/// Copy available stream input into the deflate window and calculate the
/// corresponding stream counters.  The raw adapter owns only the temporary
/// caller/window lends and pointer cursor updates.
fn read_buf_progress_state(
    input: &[crate::stdlib::Byte],
    output: &mut [crate::stdlib::Byte],
    wrap: ::core::ffi::c_int,
    adler: crate::stdlib::uLong,
    avail_in: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
) -> (
    ::core::ffi::c_uint,
    crate::stdlib::uLong,
    crate::stdlib::uInt,
    crate::stdlib::uLong,
) {
    let (len, adler) = read_buf_state(input, output, wrap, adler);
    (
        len,
        adler,
        avail_in.wrapping_sub(len),
        total_in.wrapping_add(len as crate::stdlib::uLong),
    )
}

fn insert_pending_strings_state(
    window: &[crate::stdlib::Byte],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    insert: &mut crate::stdlib::uInt,
    ins_h: &mut crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
) -> bool {
    if lookahead.wrapping_add(*insert) < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        return true;
    }

    let Some(start) = strstart.checked_sub(*insert) else {
        return false;
    };
    let Ok(start) = usize::try_from(start) else {
        return false;
    };
    let Ok(insert_len) = usize::try_from(*insert) else {
        return false;
    };
    let Some(last) = start
        .checked_add(insert_len)
        .and_then(|end| end.checked_add(1))
    else {
        return false;
    };
    let Ok(max_head_index) = usize::try_from(hash_mask) else {
        return false;
    };
    let Ok(max_prev_index) = usize::try_from(w_mask) else {
        return false;
    };
    if last >= window.len() || max_head_index >= head.len() || max_prev_index >= prev.len() {
        return false;
    }

    let mut current = start;
    *ins_h = window[current] as crate::stdlib::uInt;
    *ins_h = ((*ins_h << hash_shift) ^ window[current + 1] as crate::stdlib::uInt) & hash_mask;
    while *insert != 0 {
        *ins_h = ((*ins_h << hash_shift) ^ window[current + 2] as crate::stdlib::uInt) & hash_mask;
        let Ok(head_index) = usize::try_from(*ins_h) else {
            return false;
        };
        let Ok(prev_index) = usize::try_from((current as crate::stdlib::uInt) & w_mask) else {
            return false;
        };
        let Some(head_entry) = head.get_mut(head_index) else {
            return false;
        };
        let Some(prev_entry) = prev.get_mut(prev_index) else {
            return false;
        };
        *prev_entry = *head_entry;
        *head_entry = current as crate::src::deflate::Posf;
        current += 1;
        *insert = insert.wrapping_sub(1);
        if lookahead.wrapping_add(*insert) < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            break;
        }
    }
    true
}

/// Insert the current three-byte string into the hash chain and return the
/// previous head of that chain.  The deflate modes use this one-string form
/// while `fill_window` uses the batched form above.
fn insert_string_state(
    window: &[crate::stdlib::Byte],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    strstart: crate::stdlib::uInt,
    ins_h: &mut crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
) -> Option<crate::src::deflate::IPos> {
    let start = usize::try_from(strstart).ok()?;
    let byte_index = start.checked_add((crate::zutil_h::MIN_MATCH - 1) as usize)?;
    let byte = *window.get(byte_index)?;
    *ins_h = ((*ins_h << hash_shift) ^ crate::stdlib::uInt::from(byte)) & hash_mask;

    let head_index = usize::try_from(*ins_h).ok()?;
    let prev_index = usize::try_from(strstart & w_mask).ok()?;
    let previous = *head.get(head_index)?;
    *prev.get_mut(prev_index)? = previous;
    *head.get_mut(head_index)? = strstart as crate::src::deflate::Posf;
    Some(previous as crate::src::deflate::IPos)
}

fn initialize_hash_state(
    window: &[crate::stdlib::Byte],
    strstart: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
) -> Option<crate::stdlib::uInt> {
    let start = usize::try_from(strstart).ok()?;
    let first = crate::stdlib::uInt::from(*window.get(start)?);
    let second = crate::stdlib::uInt::from(*window.get(start.checked_add(1)?)?);
    Some(((first << hash_shift) ^ second) & hash_mask)
}

fn insert_dictionary_strings_state(
    s: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Byte],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
) -> bool {
    if s.lookahead < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        return false;
    }
    let Ok(strstart) = usize::try_from(s.strstart) else {
        return false;
    };
    let Ok(count) = usize::try_from(
        s.lookahead
            .wrapping_sub((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt),
    ) else {
        return false;
    };
    let Some(last) = strstart
        .checked_add(count)
        .and_then(|end| end.checked_add(1))
    else {
        return false;
    };
    let Ok(max_head_index) = usize::try_from(s.hash_mask) else {
        return false;
    };
    let Ok(max_prev_index) = usize::try_from(s.w_mask) else {
        return false;
    };
    if last >= window.len() || max_head_index >= head.len() || max_prev_index >= prev.len() {
        return false;
    }

    let mut str = s.strstart;
    for _ in 0..count {
        s.ins_h = ((s.ins_h << s.hash_shift)
            ^ window[str as usize + crate::zutil_h::MIN_MATCH as usize - 1] as crate::stdlib::uInt)
            & s.hash_mask;
        let head_index = s.ins_h as usize;
        let prev_index = (str & s.w_mask) as usize;
        prev[prev_index] = head[head_index];
        head[head_index] = str as crate::src::deflate::Posf;
        str = str.wrapping_add(1);
    }
    s.strstart = str;
    s.lookahead = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
    true
}

fn clear_window_tail_state(
    window: &mut [crate::stdlib::Byte],
    window_size: crate::zutil_h::ulg,
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    high_water: &mut crate::zutil_h::ulg,
) -> bool {
    if *high_water >= window_size {
        return true;
    }
    let Ok(window_size) = usize::try_from(window_size) else {
        return false;
    };
    if window_size > window.len() {
        return false;
    }

    let curr = (strstart as crate::zutil_h::ulg).wrapping_add(lookahead as crate::zutil_h::ulg);
    let Ok(curr_index) = usize::try_from(curr) else {
        return false;
    };
    if curr_index > window_size {
        return false;
    }
    let high_water_value = *high_water;
    let (start, init) = if *high_water < curr {
        let init = window_size
            .wrapping_sub(curr_index)
            .min(crate::src::deflate::WIN_INIT as usize);
        (curr_index, init)
    } else if *high_water < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
    {
        let Ok(high_water) = usize::try_from(high_water_value) else {
            return false;
        };
        let init = (curr
            .wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
            .wrapping_sub(high_water_value) as usize)
            .min(window_size.wrapping_sub(high_water));
        (high_water, init)
    } else {
        return true;
    };
    let Some(end) = start.checked_add(init) else {
        return false;
    };
    if end > window_size {
        return false;
    }
    window[start..end].fill(0);
    *high_water = end as crate::zutil_h::ulg;
    true
}

fn slide_window_state(
    window: &mut [crate::stdlib::Byte],
    wsize: crate::stdlib::uInt,
    more: ::core::ffi::c_uint,
    match_start: &mut crate::stdlib::uInt,
    strstart: &mut crate::stdlib::uInt,
    block_start: &mut ::core::ffi::c_long,
    insert: &mut crate::stdlib::uInt,
) -> Option<::core::ffi::c_uint> {
    let copy_len = wsize.wrapping_sub(more);
    let source_start = usize::try_from(wsize).ok()?;
    let copy_len = usize::try_from(copy_len).ok()?;
    let source_end = source_start.checked_add(copy_len)?;
    if source_end > window.len() {
        return None;
    }

    window.copy_within(source_start..source_end, 0);
    *match_start = match_start.wrapping_sub(wsize);
    *strstart = strstart.wrapping_sub(wsize);
    *block_start = block_start.wrapping_sub(wsize as ::core::ffi::c_long);
    if *insert > *strstart {
        *insert = *strstart;
    }
    Some(more.wrapping_add(wsize))
}

unsafe extern "C" fn read_buf(
    mut strm: crate::zlib_h::z_streamp,
    mut buf: *mut crate::stdlib::Bytef,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if strm.is_null() {
        return 0 as ::core::ffi::c_uint;
    }
    let strm = &mut *strm;
    let len = strm.avail_in.min(size);
    if len == 0 {
        return 0;
    }
    if strm.next_in.is_null() || buf.is_null() || strm.state.is_null() {
        return 0;
    }
    let input = ::core::slice::from_raw_parts(strm.next_in, len as usize);
    let output = ::core::slice::from_raw_parts_mut(buf, len as usize);
    let (len, adler, avail_in, total_in) = read_buf_progress_state(
        input,
        output,
        (*(strm.state as *const crate::src::deflate::deflate_state)).wrap,
        strm.adler,
        strm.avail_in,
        strm.total_in,
    );
    strm.avail_in = avail_in;
    strm.adler = adler;
    // `len` is bounded by the validated input slice above.  Preserve the
    // translated cursor arithmetic without performing an unsafe raw-pointer
    // offset in this private adapter.
    strm.next_in = strm.next_in.wrapping_add(len as usize);
    strm.total_in = total_in;
    len
}

fn fill_window_space_state(
    window_size: crate::zutil_h::ulg,
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    wsize: crate::stdlib::uInt,
) -> (::core::ffi::c_uint, bool) {
    let mut more = window_size
        .wrapping_sub(lookahead as crate::zutil_h::ulg)
        .wrapping_sub(strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if ::core::mem::size_of::<::core::ffi::c_int>() <= 2 {
        if more == 0 && strstart == 0 && lookahead == 0 {
            more = wsize;
        } else if more == -1i32 as ::core::ffi::c_uint {
            more = more.wrapping_sub(1);
        }
    }
    let slide = strstart
        >= wsize.wrapping_add(
            wsize.wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        );
    (more, slide)
}

unsafe extern "C" fn fill_window(mut s: *mut crate::src::deflate::deflate_state) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    let wsize: crate::stdlib::uInt = (*s).w_size;
    loop {
        let (space, should_slide) =
            fill_window_space_state((*s).window_size, (*s).lookahead, (*s).strstart, wsize);
        more = space;
        if should_slide {
            let Ok(window_len) = usize::try_from((*s).window_size) else {
                return;
            };
            if window_len != 0 && (*s).window.is_null() {
                return;
            }
            let window = if window_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut((*s).window, window_len)
            };
            let Some(next_more) = slide_window_state(
                window,
                wsize,
                more,
                &mut (*s).match_start,
                &mut (*s).strstart,
                &mut (*s).block_start,
                &mut (*s).insert,
            ) else {
                return;
            };
            slide_hash(s);
            more = next_more;
        }
        if (*(*s).strm).avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        n = read_buf(
            (*s).strm,
            (*s).window
                .wrapping_add((*s).strstart as usize)
                .wrapping_add((*s).lookahead as usize),
            more,
        );
        (*s).lookahead = (*s).lookahead.wrapping_add(n);
        if (*s).lookahead.wrapping_add((*s).insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        {
            let Ok(window_len) = usize::try_from((*s).window_size) else {
                return;
            };
            let Ok(head_len) = usize::try_from((*s).hash_size) else {
                return;
            };
            let Ok(prev_len) = usize::try_from((*s).w_size) else {
                return;
            };
            if (window_len != 0 && (*s).window.is_null())
                || (head_len != 0 && (*s).head.is_null())
                || (prev_len != 0 && (*s).prev.is_null())
            {
                return;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts((*s).window, window_len)
            };
            let head = if head_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut((*s).head, head_len)
            };
            let prev = if prev_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut((*s).prev, prev_len)
            };
            if !insert_pending_strings_state(
                window,
                head,
                prev,
                (*s).strstart,
                (*s).lookahead,
                &mut (*s).insert,
                &mut (*s).ins_h,
                (*s).hash_shift,
                (*s).hash_mask,
                (*s).w_mask,
            ) {
                return;
            }
        }
        if !((*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
            && (*(*s).strm).avail_in != 0 as crate::stdlib::uInt)
        {
            break;
        }
    }
    if (*s).high_water < (*s).window_size {
        let Ok(window_len) = usize::try_from((*s).window_size) else {
            return;
        };
        if window_len != 0 && (*s).window.is_null() {
            return;
        }
        let window = if window_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut((*s).window, window_len)
        };
        if !clear_window_tail_state(
            window,
            (*s).window_size,
            (*s).strstart,
            (*s).lookahead,
            &mut (*s).high_water,
        ) {
            return;
        }
    }
}

#[export_name = "deflateInit_"]

pub unsafe extern "C" fn deflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateInit2_(
        strm,
        level,
        crate::zlib_h::Z_DEFLATED,
        crate::stdlib::MAX_WBITS,
        crate::zutil_h::DEF_MEM_LEVEL,
        crate::zlib_h::Z_DEFAULT_STRATEGY,
        version,
        stream_size,
    )
}
pub unsafe extern "C" fn deflateInit2_(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut method: ::core::ffi::c_int,
    mut windowBits: ::core::ffi::c_int,
    mut memLevel: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut wrap: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    const MY_VERSION: [::core::ffi::c_char; 15] = crate::zlib_h::ZLIB_VERSION;
    if version.is_null()
        || *version as ::core::ffi::c_int
            != MY_VERSION[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        || stream_size as usize != ::core::mem::size_of::<crate::zlib_h::z_stream>()
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm_ref = &mut *strm;
    strm_ref.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if strm_ref.zalloc.is_none() {
        strm_ref.zalloc = Some(
            crate::src::zutil::zcalloc_ffi
                as unsafe extern "C" fn(
                    crate::stdlib::voidpf,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> crate::stdlib::voidpf,
        ) as crate::zlib_h::alloc_func;
        strm_ref.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if strm_ref.zfree.is_none() {
        strm_ref.zfree = Some(
            crate::src::zutil::zcfree_ffi
                as unsafe extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> (),
        ) as crate::zlib_h::free_func;
    }
    if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        level = 6 as ::core::ffi::c_int;
    }
    if windowBits < 0 as ::core::ffi::c_int {
        wrap = 0 as ::core::ffi::c_int;
        if windowBits < -15 as ::core::ffi::c_int {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        windowBits = -windowBits;
    } else if windowBits > 15 as ::core::ffi::c_int {
        wrap = 2 as ::core::ffi::c_int;
        windowBits -= 16 as ::core::ffi::c_int;
    }
    if memLevel < 1 as ::core::ffi::c_int
        || memLevel > crate::stdlib::MAX_MEM_LEVEL
        || method != crate::zlib_h::Z_DEFLATED
        || windowBits < 8 as ::core::ffi::c_int
        || windowBits > 15 as ::core::ffi::c_int
        || level < 0 as ::core::ffi::c_int
        || level > 9 as ::core::ffi::c_int
        || strategy < 0 as ::core::ffi::c_int
        || strategy > crate::zlib_h::Z_FIXED
        || windowBits == 8 as ::core::ffi::c_int && wrap != 1 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if windowBits == 8 as ::core::ffi::c_int {
        windowBits = 9 as ::core::ffi::c_int;
    }
    s = Some(strm_ref.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        strm_ref.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::deflate_state;
    if s.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    crate::stdlib::memset(
        s as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>(),
    );
    strm_ref.state = s as *mut crate::src::deflate::internal_state;
    (*s).strm = strm;
    (*s).status = crate::src::deflate::INIT_STATE;
    (*s).wrap = wrap;
    (*s).gzhead = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
    (*s).w_bits = windowBits as crate::stdlib::uInt;
    (*s).w_size = ((1 as ::core::ffi::c_int) << (*s).w_bits) as crate::stdlib::uInt;
    (*s).w_mask = (*s).w_size.wrapping_sub(1 as crate::stdlib::uInt);
    (*s).hash_bits = (memLevel as crate::stdlib::uInt).wrapping_add(7 as crate::stdlib::uInt);
    (*s).hash_size = ((1 as ::core::ffi::c_int) << (*s).hash_bits) as crate::stdlib::uInt;
    (*s).hash_mask = (*s).hash_size.wrapping_sub(1 as crate::stdlib::uInt);
    (*s).hash_shift = (*s)
        .hash_bits
        .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
    (*s).window = Some(strm_ref.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        strm_ref.opaque,
        (*s).w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
            as crate::stdlib::uInt,
    ) as *mut crate::stdlib::Bytef;
    (*s).prev = Some(strm_ref.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        strm_ref.opaque,
        (*s).w_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    (*s).head = Some(strm_ref.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        strm_ref.opaque,
        (*s).hash_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    (*s).high_water = 0 as crate::zutil_h::ulg;
    (*s).lit_bufsize =
        ((1 as ::core::ffi::c_int) << memLevel + 6 as ::core::ffi::c_int) as crate::stdlib::uInt;
    (*s).pending_buf = Some(strm_ref.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        strm_ref.opaque,
        (*s).lit_bufsize,
        4 as crate::stdlib::uInt,
    ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef;
    (*s).pending_buf_size =
        ((*s).lit_bufsize as crate::zutil_h::ulg).wrapping_mul(4 as crate::zutil_h::ulg);
    if (*s).window.is_null()
        || (*s).prev.is_null()
        || (*s).head.is_null()
        || (*s).pending_buf.is_null()
    {
        (*s).status = crate::src::deflate::FINISH_STATE;
        strm_ref.msg =
            crate::src::zutil::z_errmsg[(if (-4 as ::core::ffi::c_int) < -6 as ::core::ffi::c_int
                || -4 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -4 as ::core::ffi::c_int
            }) as usize] as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        deflateEnd(strm);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*s).sym_buf =
        (*s).pending_buf.wrapping_add((*s).lit_bufsize as usize) as *mut crate::zutil_h::uchf;
    (*s).sym_end = (*s)
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    (*s).level = level;
    (*s).strategy = strategy;
    (*s).method = method as crate::stdlib::Byte;
    let state = &mut *s;
    let ret = deflate_reset_keep_state(strm_ref, state);
    if ret == crate::zlib_h::Z_OK {
        let Ok(head_len) = usize::try_from(state.hash_size) else {
            return ret;
        };
        if head_len == 0 {
            let _ = lm_init_state(state, &mut []);
        } else if !state.head.is_null() {
            let head = ::core::slice::from_raw_parts_mut(state.head, head_len);
            let _ = lm_init_state(state, head);
        }
    }
    ret
}
#[export_name = "deflateInit2_"]

pub unsafe extern "C" fn deflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut method: ::core::ffi::c_int,
    mut windowBits: ::core::ffi::c_int,
    mut memLevel: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateInit2_(
        strm,
        level,
        method,
        windowBits,
        memLevel,
        strategy,
        version,
        stream_size,
    )
}
fn deflate_state_values_are_valid(
    has_zalloc: bool,
    has_zfree: bool,
    state_matches_stream: bool,
    status: ::core::ffi::c_int,
) -> bool {
    has_zalloc
        && has_zfree
        && state_matches_stream
        && matches!(
            status,
            crate::src::deflate::INIT_STATE
                | crate::src::deflate::GZIP_STATE
                | crate::src::deflate::EXTRA_STATE
                | crate::src::deflate::NAME_STATE
                | crate::src::deflate::COMMENT_STATE
                | crate::src::deflate::HCRC_STATE
                | crate::src::deflate::BUSY_STATE
                | crate::src::deflate::FINISH_STATE
        )
}

pub(crate) unsafe extern "C" fn deflateStateCheck(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return 1;
    }
    let strm_ref = &*strm;
    let state = strm_ref.state as *mut crate::src::deflate::deflate_state;
    if state.is_null() {
        return 1;
    }
    let state = &*state;
    if deflate_state_values_are_valid(
        strm_ref.zalloc.is_some(),
        strm_ref.zfree.is_some(),
        state.strm == strm,
        state.status,
    ) {
        0
    } else {
        1
    }
}
#[export_name = "deflateSetDictionary"]
pub unsafe extern "C" fn deflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut wrap: ::core::ffi::c_int = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if deflateStateCheck(strm) != 0 || dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    wrap = (*s).wrap;
    if wrap == 2 as ::core::ffi::c_int
        || wrap == 1 as ::core::ffi::c_int && (*s).status != crate::src::deflate::INIT_STATE
        || (*s).lookahead != 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if wrap == 1 as ::core::ffi::c_int {
        (*strm).adler = crate::src::adler32::adler32_z(
            (*strm).adler,
            core::slice::from_raw_parts(dictionary, dictLength as usize),
        );
    }
    (*s).wrap = 0 as ::core::ffi::c_int;
    if dictLength >= (*s).w_size {
        if wrap == 0 as ::core::ffi::c_int {
            let Ok(head_len) = usize::try_from((*s).hash_size) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            if head_len != 0 && (*s).head.is_null() {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            let head = if head_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut((*s).head, head_len)
            };
            clear_hash_state(head, &mut (*s).slid);
            (*s).strstart = 0 as crate::stdlib::uInt;
            (*s).block_start = 0 as ::core::ffi::c_long;
            (*s).insert = 0 as crate::stdlib::uInt;
        }
        dictionary = dictionary.wrapping_add(dictLength.wrapping_sub((*s).w_size) as usize);
        dictLength = (*s).w_size;
    }
    avail = (*strm).avail_in as ::core::ffi::c_uint;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    (*strm).avail_in = dictLength;
    (*strm).next_in = dictionary as *mut crate::stdlib::Bytef;
    fill_window(s);
    while (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        let Ok(window_len) = usize::try_from((*s).window_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Ok(head_len) = usize::try_from((*s).hash_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Ok(prev_len) = usize::try_from((*s).w_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if (window_len != 0 && (*s).window.is_null())
            || (head_len != 0 && (*s).head.is_null())
            || (prev_len != 0 && (*s).prev.is_null())
        {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let window = if window_len == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts((*s).window, window_len)
        };
        let head = if head_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut((*s).head, head_len)
        };
        let prev = if prev_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut((*s).prev, prev_len)
        };
        if !insert_dictionary_strings_state(&mut *s, window, head, prev) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        fill_window(s);
    }
    (*s).strstart = (*s).strstart.wrapping_add((*s).lookahead);
    (*s).block_start = (*s).strstart as ::core::ffi::c_long;
    (*s).insert = (*s).lookahead;
    (*s).lookahead = 0 as crate::stdlib::uInt;
    (*s).prev_length = (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
    (*s).match_length = (*s).prev_length;
    (*s).match_available = 0 as ::core::ffi::c_int;
    (*strm).next_in = next as *mut crate::stdlib::Bytef;
    (*strm).avail_in = avail as crate::stdlib::uInt;
    (*s).wrap = wrap;
    return crate::zlib_h::Z_OK;
}
fn deflate_dictionary_range_state(
    state: &crate::src::deflate::deflate_state,
) -> Option<::core::ops::Range<usize>> {
    let len = state
        .strstart
        .wrapping_add(state.lookahead)
        .min(state.w_size);
    let end = usize::try_from(state.strstart)
        .ok()?
        .checked_add(usize::try_from(state.lookahead).ok()?)?;
    let len = usize::try_from(len).ok()?;
    let start = end.checked_sub(len)?;
    Some(start..end)
}

fn deflate_get_dictionary_state(
    state: &crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Byte],
    dictionary: &mut [crate::stdlib::Byte],
) -> Option<::core::ffi::c_int> {
    let range = deflate_dictionary_range_state(state)?;
    if dictionary.len() != range.len() {
        return None;
    }
    dictionary.copy_from_slice(window.get(range)?);
    Some(crate::zlib_h::Z_OK)
}

#[export_name = "deflateGetDictionary"]

pub unsafe extern "C" fn deflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*((*strm).state as *mut crate::src::deflate::deflate_state);
    let Some(range) = deflate_dictionary_range_state(state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let len = range.len();
    if !dictionary.is_null() && len != 0 {
        let Ok(window_len) = usize::try_from(state.window_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if state.window.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let window = ::core::slice::from_raw_parts(state.window, window_len);
        let output = ::core::slice::from_raw_parts_mut(dictionary, len);
        if deflate_get_dictionary_state(state, window, output).is_none() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
    }
    if !dictLength.is_null() {
        *dictLength = len as crate::stdlib::uInt;
    }
    crate::zlib_h::Z_OK
}
pub(crate) fn deflate_reset_keep_state(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::deflate::deflate_state,
) -> ::core::ffi::c_int {
    strm.total_out = 0 as crate::stdlib::uLong;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = crate::zlib_h::Z_UNKNOWN;
    state.pending = 0 as crate::zutil_h::ulg;
    state.pending_out = state.pending_buf;
    if state.wrap < 0 as ::core::ffi::c_int {
        state.wrap = -state.wrap;
    }
    state.status = if state.wrap == 2 as ::core::ffi::c_int {
        crate::src::deflate::GZIP_STATE
    } else {
        crate::src::deflate::INIT_STATE
    };
    strm.adler = if state.wrap == 2 as ::core::ffi::c_int {
        crate::src::crc32::crc32_slice(0, &[])
    } else {
        crate::src::adler32::ADLER32_INITIAL
    };
    state.last_flush = -2 as ::core::ffi::c_int;
    crate::src::trees::tr_init_state(state);
    crate::zlib_h::Z_OK
}

// This expands only at ABI boundaries.  The state allocation is still owned
// by the callback-backed deflate stream, so converting its raw handle remains
// at that boundary while the reset itself is safe state mutation.
macro_rules! deflate_reset_keep_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::deflate::deflateStateCheck(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = (*strm).state as *mut crate::src::deflate::deflate_state;
            crate::src::deflate::deflate_reset_keep_state(&mut *strm, &mut *state)
        }
    }};
}
#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflate_reset_keep_at_boundary!(strm)
}
pub(crate) fn lm_init_state(
    s: &mut crate::src::deflate::deflate_state,
    head: &mut [crate::src::deflate::Posf],
) -> bool {
    let Ok(hash_size) = usize::try_from(s.hash_size) else {
        return false;
    };
    let Ok(level) = usize::try_from(s.level) else {
        return false;
    };
    let Some(config) = configuration_table.get(level) else {
        return false;
    };
    if head.len() != hash_size {
        return false;
    }

    s.window_size = (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
        .wrapping_mul(s.w_size as crate::zutil_h::ulg);
    head.fill(NIL as crate::src::deflate::Posf);
    s.slid = 0 as ::core::ffi::c_int;
    s.max_lazy_match = config.max_lazy as crate::stdlib::uInt;
    s.good_match = config.good_length as crate::stdlib::uInt;
    s.nice_match = config.nice_length as ::core::ffi::c_int;
    s.max_chain_length = config.max_chain as crate::stdlib::uInt;
    s.strstart = 0 as crate::stdlib::uInt;
    s.block_start = 0 as ::core::ffi::c_long;
    s.lookahead = 0 as crate::stdlib::uInt;
    s.insert = 0 as crate::stdlib::uInt;
    s.prev_length = (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
    s.match_length = s.prev_length;
    s.match_available = 0 as ::core::ffi::c_int;
    s.ins_h = 0 as crate::stdlib::uInt;
    true
}

// The raw head allocation belongs to the callback-backed deflate state. Keep
// its temporary slice at the ABI boundary; the reset arithmetic itself lives
// in the safe helpers above.
macro_rules! deflate_reset_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::deflate::deflateStateCheck(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = (*strm).state as *mut crate::src::deflate::deflate_state;
            let ret = crate::src::deflate::deflate_reset_keep_state(&mut *strm, &mut *state);
            if ret == crate::zlib_h::Z_OK {
                if let Ok(head_len) = usize::try_from((*state).hash_size) {
                    if head_len == 0 {
                        let _ = crate::src::deflate::lm_init_state(&mut *state, &mut []);
                    } else if !(*state).head.is_null() {
                        let head = ::core::slice::from_raw_parts_mut((*state).head, head_len);
                        let _ = crate::src::deflate::lm_init_state(&mut *state, head);
                    }
                }
            }
            ret
        }
    }};
}
pub(crate) use deflate_reset_at_boundary;
#[export_name = "deflateReset"]

pub unsafe extern "C" fn deflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflate_reset_at_boundary!(strm)
}
#[export_name = "deflateSetHeader"]
pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 || (*(*strm).state).wrap != 2 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*(*strm).state).gzhead = head;
    return crate::zlib_h::Z_OK;
}
fn deflate_pending_state(
    state: &crate::src::deflate::deflate_state,
) -> Result<(::core::ffi::c_uint, ::core::ffi::c_int), ()> {
    let pending = ::core::ffi::c_uint::try_from(state.pending).map_err(|_| ())?;
    Ok((pending, state.bi_valid))
}
#[export_name = "deflatePending"]

pub unsafe extern "C" fn deflatePending_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*((*strm).state as *mut crate::src::deflate::deflate_state);
    match deflate_pending_state(state) {
        Ok((pending_count, bit_count)) => {
            if !bits.is_null() {
                *bits = bit_count;
            }
            if !pending.is_null() {
                *pending = pending_count;
            }
            crate::zlib_h::Z_OK
        }
        Err(()) => {
            if !pending.is_null() {
                *pending = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
            }
            crate::zlib_h::Z_BUF_ERROR
        }
    }
}
fn deflate_used_state(state: &crate::src::deflate::deflate_state) -> ::core::ffi::c_int {
    state.bi_used
}
#[export_name = "deflateUsed"]

pub unsafe extern "C" fn deflateUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !bits.is_null() {
        let state = &*((*strm).state as *mut crate::src::deflate::deflate_state);
        *bits = deflate_used_state(state);
    }
    crate::zlib_h::Z_OK
}

struct PrimeBitsStep {
    bi_buf: crate::zutil_h::ush,
    bi_valid: ::core::ffi::c_int,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
}

fn prime_bits_step(
    bi_buf: crate::zutil_h::ush,
    bi_valid: ::core::ffi::c_int,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> Option<PrimeBitsStep> {
    if !(0..=crate::src::deflate::Buf_size).contains(&bi_valid) || !(0..=16).contains(&bits) {
        return None;
    }
    let put = (crate::src::deflate::Buf_size - bi_valid).min(bits);
    let mask = (1 as ::core::ffi::c_int).checked_shl(put as u32)? - 1;
    let fragment = (value & mask).checked_shl(bi_valid as u32)?;
    Some(PrimeBitsStep {
        bi_buf: (bi_buf as ::core::ffi::c_int
            | (fragment as crate::zutil_h::ush as ::core::ffi::c_int))
            as crate::zutil_h::ush,
        bi_valid: bi_valid + put,
        bits: bits - put,
        value: value >> put,
    })
}

#[export_name = "deflatePrime"]
pub unsafe extern "C" fn deflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || (*s).sym_buf
            < (*s).pending_out.wrapping_add(
                (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                    as usize,
            )
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    loop {
        let Some(step) = prime_bits_step((*s).bi_buf, (*s).bi_valid, bits, value) else {
            return crate::zlib_h::Z_BUF_ERROR;
        };
        (*s).bi_buf = step.bi_buf;
        (*s).bi_valid = step.bi_valid;
        let Ok(pending_len) = usize::try_from((*s).pending_buf_size) else {
            return crate::zlib_h::Z_BUF_ERROR;
        };
        if pending_len != 0 && (*s).pending_buf.is_null() {
            return crate::zlib_h::Z_BUF_ERROR;
        }
        let pending_buf = if pending_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut((*s).pending_buf, pending_len)
        };
        crate::src::trees::flush_bits_state(&mut *s, pending_buf);
        value = step.value;
        bits = step.bits;
        if bits == 0 {
            break;
        }
    }
    return crate::zlib_h::Z_OK;
}
#[derive(Copy, Clone)]
pub(crate) struct DeflateParamsPlan {
    pub(crate) level: ::core::ffi::c_int,
    pub(crate) strategy: ::core::ffi::c_int,
    pub(crate) config: config,
    pub(crate) needs_block_flush: bool,
}

/// Validate and normalize a parameter change without touching stream state.
///
/// Keeping the configuration-table lookup here makes malformed internal levels
/// an ordinary stream error rather than an unchecked table index in the raw
/// stream adapter.
pub(crate) fn deflate_params_plan(
    requested_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    current_level: ::core::ffi::c_int,
    current_strategy: ::core::ffi::c_int,
    last_flush: ::core::ffi::c_int,
) -> Option<DeflateParamsPlan> {
    let level = if requested_level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        6
    } else {
        requested_level
    };
    if !(0..=9).contains(&level) || !(0..=crate::zlib_h::Z_FIXED).contains(&strategy) {
        return None;
    }
    let current_config = configuration_table.get(usize::try_from(current_level).ok()?)?;
    let config = *configuration_table.get(usize::try_from(level).ok()?)?;
    Some(DeflateParamsPlan {
        level,
        strategy,
        config,
        needs_block_flush: (strategy != current_strategy || current_config.kind != config.kind)
            && last_flush != -2,
    })
}

// This still drives the legacy raw deflate state, so it is deliberately an
// export-boundary macro.  `deflateParams` has no Rust implementation callers:
// the gzip setter expands it directly at its own ABI boundary.
macro_rules! deflate_params_at_boundary {
    ($strm:expr, $level:expr, $strategy:expr) => {{
        let strm = $strm;
        let level = $level;
        let strategy = $strategy;
        'deflate_params_result: {
            if crate::src::deflate::deflateStateCheck(strm) != 0 {
                break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
            }
            let s = (*strm).state as *mut crate::src::deflate::deflate_state;
            let Some(plan) = crate::src::deflate::deflate_params_plan(
                level,
                strategy,
                (*s).level,
                (*s).strategy,
                (*s).last_flush,
            ) else {
                break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
            };
            if plan.needs_block_flush {
                let err = crate::src::deflate::deflate(strm, crate::zlib_h::Z_BLOCK);
                if err == crate::zlib_h::Z_STREAM_ERROR {
                    break 'deflate_params_result err;
                }
                if (*strm).avail_in != 0
                    || (*s).strstart as ::core::ffi::c_long - (*s).block_start
                        + (*s).lookahead as ::core::ffi::c_long
                        != 0
                {
                    break 'deflate_params_result crate::zlib_h::Z_BUF_ERROR;
                }
            }
            if (*s).level != plan.level {
                if (*s).level == 0 as ::core::ffi::c_int && (*s).matches != 0 as crate::stdlib::uInt
                {
                    if (*s).matches == 1 as crate::stdlib::uInt {
                        let Ok(head_len) = usize::try_from((*s).hash_size) else {
                            break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                        };
                        let Ok(prev_len) = usize::try_from((*s).w_size) else {
                            break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                        };
                        if (head_len != 0 && (*s).head.is_null())
                            || (prev_len != 0 && (*s).prev.is_null())
                        {
                            break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                        }
                        let head = if head_len == 0 {
                            &mut []
                        } else {
                            ::core::slice::from_raw_parts_mut((*s).head, head_len)
                        };
                        let prev = if prev_len == 0 {
                            &mut []
                        } else {
                            ::core::slice::from_raw_parts_mut((*s).prev, prev_len)
                        };
                        crate::src::deflate::slide_hash_state(head, prev, (*s).w_size);
                        (*s).slid = 1;
                    } else {
                        let Ok(head_len) = usize::try_from((*s).hash_size) else {
                            break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                        };
                        if head_len != 0 && (*s).head.is_null() {
                            break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                        }
                        let head = if head_len == 0 {
                            &mut []
                        } else {
                            ::core::slice::from_raw_parts_mut((*s).head, head_len)
                        };
                        crate::src::deflate::clear_hash_state(head, &mut (*s).slid);
                    }
                    (*s).matches = 0 as crate::stdlib::uInt;
                }
                (*s).level = plan.level;
                (*s).max_lazy_match = plan.config.max_lazy as crate::stdlib::uInt;
                (*s).good_match = plan.config.good_length as crate::stdlib::uInt;
                (*s).nice_match = plan.config.nice_length as ::core::ffi::c_int;
                (*s).max_chain_length = plan.config.max_chain as crate::stdlib::uInt;
            }
            (*s).strategy = plan.strategy;
            crate::zlib_h::Z_OK
        }
    }};
}
pub(crate) use deflate_params_at_boundary;

#[export_name = "deflateParams"]

pub unsafe extern "C" fn deflateParams_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflate_params_at_boundary!(strm, level, strategy)
}
fn deflate_tune_state(
    state: &mut crate::src::deflate::deflate_state,
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) {
    state.good_match = good_length as crate::stdlib::uInt;
    state.max_lazy_match = max_lazy as crate::stdlib::uInt;
    state.nice_match = nice_length;
    state.max_chain_length = max_chain as crate::stdlib::uInt;
}
#[export_name = "deflateTune"]

pub unsafe extern "C" fn deflateTune_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut good_length: ::core::ffi::c_int,
    mut max_lazy: ::core::ffi::c_int,
    mut nice_length: ::core::ffi::c_int,
    mut max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    deflate_tune_state(state, good_length, max_lazy, nice_length, max_chain);
    crate::zlib_h::Z_OK
}
#[derive(Copy, Clone)]
struct DeflateBoundHeader {
    extra_len: Option<usize>,
    name_len: Option<usize>,
    comment_len: Option<usize>,
    hcrc: bool,
}
#[derive(Copy, Clone)]
struct DeflateBoundState {
    wrap: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    w_bits: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
    gzip_header: Option<DeflateBoundHeader>,
}

/// Reduce the optional C gzip header to the byte lengths that affect the
/// compression bound.  The ABI boundary is responsible for constructing the
/// short-lived `CStr` views; this core never retains them.
fn deflate_bound_header(
    extra_len: Option<usize>,
    name: Option<&::std::ffi::CStr>,
    comment: Option<&::std::ffi::CStr>,
    hcrc: bool,
) -> DeflateBoundHeader {
    DeflateBoundHeader {
        extra_len,
        name_len: name.map(|value| value.to_bytes_with_nul().len()),
        comment_len: comment.map(|value| value.to_bytes_with_nul().len()),
        hcrc,
    }
}

// The stream and gzip-header pointers remain ABI-owned, including the
// transient C-string views.  Expand this only at exported boundaries.
macro_rules! deflate_bound_state_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::deflate::deflateStateCheck(strm) != 0 {
            None
        } else {
            let state = &*((*strm).state as *mut crate::src::deflate::deflate_state);
            let gzip_header = if state.gzhead.is_null() {
                None
            } else {
                let header = &*state.gzhead;
                let name = if header.name.is_null() {
                    None
                } else {
                    Some(::std::ffi::CStr::from_ptr(header.name.cast()))
                };
                let comment = if header.comment.is_null() {
                    None
                } else {
                    Some(::std::ffi::CStr::from_ptr(header.comment.cast()))
                };
                Some(crate::src::deflate::deflate_bound_header(
                    (!header.extra.is_null()).then_some(header.extra_len as usize),
                    name,
                    comment,
                    header.hcrc != 0,
                ))
            };
            Some(crate::src::deflate::DeflateBoundState {
                wrap: state.wrap,
                strstart: state.strstart,
                w_bits: state.w_bits,
                hash_bits: state.hash_bits,
                level: state.level,
                gzip_header,
            })
        }
    }};
}

fn deflate_bound(
    source_len: crate::stdlib::z_size_t,
    state: Option<DeflateBoundState>,
) -> crate::stdlib::z_size_t {
    let mut fixed_len = source_len
        .wrapping_add(source_len >> 3)
        .wrapping_add(source_len >> 8)
        .wrapping_add(source_len >> 9)
        .wrapping_add(4);
    if fixed_len < source_len {
        fixed_len = usize::MAX;
    }
    let mut stored_len = source_len
        .wrapping_add(source_len >> 5)
        .wrapping_add(source_len >> 7)
        .wrapping_add(source_len >> 11)
        .wrapping_add(7);
    if stored_len < source_len {
        stored_len = usize::MAX;
    }
    let Some(state) = state else {
        let bound = fixed_len.max(stored_len);
        return bound.checked_add(18).unwrap_or(usize::MAX);
    };

    let wrap_len = match state.wrap.wrapping_abs() {
        0 => 0,
        1 => 6 + usize::from(state.strstart != 0) * 4,
        2 => {
            let mut length = 18usize;
            if let Some(header) = state.gzip_header {
                if let Some(extra_len) = header.extra_len {
                    length = length.wrapping_add(2usize.wrapping_add(extra_len));
                }
                if let Some(name_len) = header.name_len {
                    length = length.wrapping_add(name_len);
                }
                if let Some(comment_len) = header.comment_len {
                    length = length.wrapping_add(comment_len);
                }
                if header.hcrc {
                    length = length.wrapping_add(2);
                }
            }
            length
        }
        _ => 18,
    };
    if state.w_bits != 15 || state.hash_bits != 15 {
        let bound = if state.w_bits <= state.hash_bits && state.level != 0 {
            fixed_len
        } else {
            stored_len
        };
        return bound.checked_add(wrap_len).unwrap_or(usize::MAX);
    }
    let bound = source_len
        .wrapping_add(source_len >> 12)
        .wrapping_add(source_len >> 14)
        .wrapping_add(source_len >> 25)
        .wrapping_add(13)
        .wrapping_sub(6)
        .wrapping_add(wrap_len);
    if bound < source_len {
        usize::MAX
    } else {
        bound
    }
}
#[export_name = "deflateBound_z"]

pub unsafe extern "C" fn deflateBound_z_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let state = deflate_bound_state_at_boundary!(strm);
    deflate_bound(sourceLen, state)
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let state = deflate_bound_state_at_boundary!(strm);
    deflate_bound(sourceLen as crate::stdlib::z_size_t, state) as crate::stdlib::uLong
}
/// Append big-endian zlib wrapper words before advancing the pending cursor.
/// Callers provide a complete header or trailer, so a capacity failure cannot
/// leave a partially emitted wrapper field behind.
fn append_zlib_words_state(
    pending_buf: &mut [crate::stdlib::Byte],
    pending: &mut crate::zutil_h::ulg,
    words: &[crate::stdlib::uInt],
) -> bool {
    let Ok(start) = usize::try_from(*pending) else {
        return false;
    };
    let Some(byte_len) = words.len().checked_mul(2) else {
        return false;
    };
    let Some(end) = start.checked_add(byte_len) else {
        return false;
    };
    let Some(bytes) = pending_buf.get_mut(start..end) else {
        return false;
    };

    for (bytes, word) in bytes.chunks_exact_mut(2).zip(words) {
        bytes[0] = (word >> 8) as crate::stdlib::Byte;
        bytes[1] = *word as crate::stdlib::Byte;
    }
    *pending = end as crate::zutil_h::ulg;
    true
}

/// Append the eight-byte gzip trailer in its on-the-wire little-endian order.
///
/// The pending buffer is callback-owned in the legacy state, so callers lend it
/// only for this operation.  Keep the range check before either the bytes or
/// the cursor are changed: a malformed internal state must not leave a partial
/// trailer behind.
fn append_gzip_trailer_state(
    pending_buf: &mut [crate::stdlib::Byte],
    pending: &mut crate::zutil_h::ulg,
    adler: crate::stdlib::uLong,
    total_in: crate::stdlib::uLong,
) -> bool {
    let Ok(start) = usize::try_from(*pending) else {
        return false;
    };
    let Some(end) = start.checked_add(8) else {
        return false;
    };
    let Some(bytes) = pending_buf.get_mut(start..end) else {
        return false;
    };

    bytes[0] = adler as crate::stdlib::Byte;
    bytes[1] = (adler >> 8) as crate::stdlib::Byte;
    bytes[2] = (adler >> 16) as crate::stdlib::Byte;
    bytes[3] = (adler >> 24) as crate::stdlib::Byte;
    bytes[4] = total_in as crate::stdlib::Byte;
    bytes[5] = (total_in >> 8) as crate::stdlib::Byte;
    bytes[6] = (total_in >> 16) as crate::stdlib::Byte;
    bytes[7] = (total_in >> 24) as crate::stdlib::Byte;
    *pending = pending.wrapping_add(8);
    true
}

/// Append the two-byte gzip header CRC after all optional header fields.
///
/// The wire format stores the low sixteen bits in little-endian order.  Check
/// both bytes before changing the pending cursor so a malformed pending buffer
/// cannot leave a partial header CRC behind.
fn append_gzip_hcrc_state(
    pending_buf: &mut [crate::stdlib::Byte],
    pending: &mut crate::zutil_h::ulg,
    hcrc: crate::stdlib::uLong,
) -> bool {
    let Ok(start) = usize::try_from(*pending) else {
        return false;
    };
    let Some(end) = start.checked_add(2) else {
        return false;
    };
    let Some(bytes) = pending_buf.get_mut(start..end) else {
        return false;
    };

    bytes[0] = hcrc as crate::stdlib::Byte;
    bytes[1] = (hcrc >> 8) as crate::stdlib::Byte;
    *pending = end as crate::zutil_h::ulg;
    true
}

/// The pointer-free portion of a gzip header that is emitted before optional
/// extra/name/comment fields.  The ABI header is observed at the raw state
/// boundary and reduced to these values before touching callback-owned output.
#[derive(Copy, Clone)]
struct GzipFixedHeader {
    text: bool,
    hcrc: bool,
    has_extra: bool,
    has_name: bool,
    has_comment: bool,
    time: crate::stdlib::uLong,
    os: ::core::ffi::c_int,
    extra_len: crate::stdlib::uInt,
}

fn gzip_xflags(level: ::core::ffi::c_int, strategy: ::core::ffi::c_int) -> crate::stdlib::Byte {
    if level == 9 {
        2
    } else if strategy >= 2 || level < 2 {
        4
    } else {
        0
    }
}

/// Append the fixed gzip header, including the optional extra-field length.
///
/// A header CRC, when requested, covers all pending bytes just as the legacy
/// code did.  The complete destination range is checked before either the
/// pending cursor or CRC state is updated.
fn append_gzip_fixed_header_state(
    pending_buf: &mut [crate::stdlib::Byte],
    pending: &mut crate::zutil_h::ulg,
    adler: &mut crate::stdlib::uLong,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    header: Option<GzipFixedHeader>,
) -> bool {
    let Ok(start) = usize::try_from(*pending) else {
        return false;
    };
    let extra_length_bytes = header
        .as_ref()
        .map_or(0, |header| usize::from(header.has_extra) * 2);
    let Some(end) = start.checked_add(10 + extra_length_bytes) else {
        return false;
    };
    if end > pending_buf.len() {
        return false;
    }

    let bytes = &mut pending_buf[start..end];
    bytes[0] = 31;
    bytes[1] = 139;
    bytes[2] = 8;
    match header {
        None => {
            bytes[3..10].copy_from_slice(&[0, 0, 0, 0, 0, gzip_xflags(level, strategy), 3]);
        }
        Some(header) => {
            bytes[3] = (u8::from(header.text)
                | (u8::from(header.hcrc) << 1)
                | (u8::from(header.has_extra) << 2)
                | (u8::from(header.has_name) << 3)
                | (u8::from(header.has_comment) << 4))
                as crate::stdlib::Byte;
            bytes[4] = header.time as crate::stdlib::Byte;
            bytes[5] = (header.time >> 8) as crate::stdlib::Byte;
            bytes[6] = (header.time >> 16) as crate::stdlib::Byte;
            bytes[7] = (header.time >> 24) as crate::stdlib::Byte;
            bytes[8] = gzip_xflags(level, strategy);
            bytes[9] = header.os as crate::stdlib::Byte;
            if header.has_extra {
                bytes[10] = header.extra_len as crate::stdlib::Byte;
                bytes[11] = (header.extra_len >> 8) as crate::stdlib::Byte;
            }
        }
    }
    *pending = end as crate::zutil_h::ulg;
    if header.is_some_and(|header| header.hcrc) {
        *adler = crate::src::crc32::crc32_slice(*adler, &pending_buf[..end]);
    }
    true
}

/// Copy as much of a gzip extra field as fits in the pending buffer.
///
/// The caller owns the boundary borrow of `extra` and retains responsibility
/// for flushing when this fills the pending buffer.  Keeping one bounded copy
/// here makes the state updates and optional header-CRC coverage atomic.
fn append_gzip_extra_chunk_state(
    pending_buf: &mut [crate::stdlib::Byte],
    pending: &mut crate::zutil_h::ulg,
    gzindex: &mut crate::zutil_h::ulg,
    extra: &[crate::stdlib::Byte],
    hcrc: bool,
    adler: &mut crate::stdlib::uLong,
) -> Option<bool> {
    let start = usize::try_from(*pending).ok()?;
    let source_start = usize::try_from(*gzindex).ok()?;
    if start > pending_buf.len() || source_start > extra.len() {
        return None;
    }

    let copy = (pending_buf.len() - start).min(extra.len() - source_start);
    if copy == 0 {
        return Some(source_start == extra.len());
    }
    let end = start.checked_add(copy)?;
    pending_buf[start..end].copy_from_slice(&extra[source_start..source_start + copy]);
    if hcrc {
        *adler = crate::src::crc32::crc32_slice(*adler, &pending_buf[start..end]);
    }
    *pending = end as crate::zutil_h::ulg;
    *gzindex = source_start.checked_add(copy)? as crate::zutil_h::ulg;
    Some(*gzindex as usize == extra.len())
}

/// Copy as much of a NUL-terminated gzip name or comment as fits in pending
/// storage.  `source` includes its terminating NUL, so a completed copy is
/// exactly the legacy loop's terminating-byte case.
fn append_gzip_cstring_chunk_state(
    pending_buf: &mut [crate::stdlib::Byte],
    pending: &mut crate::zutil_h::ulg,
    gzindex: &mut crate::zutil_h::ulg,
    source: &[u8],
    hcrc: bool,
    adler: &mut crate::stdlib::uLong,
) -> Option<bool> {
    let start = usize::try_from(*pending).ok()?;
    let source_start = usize::try_from(*gzindex).ok()?;
    if start > pending_buf.len() || source_start >= source.len() {
        return None;
    }

    let copy = (pending_buf.len() - start).min(source.len() - source_start);
    if copy == 0 {
        return None;
    }
    let end = start.checked_add(copy)?;
    pending_buf[start..end].copy_from_slice(&source[source_start..source_start + copy]);
    if hcrc {
        *adler = crate::src::crc32::crc32_slice(*adler, &pending_buf[start..end]);
    }
    *pending = end as crate::zutil_h::ulg;
    *gzindex = source_start.checked_add(copy)? as crate::zutil_h::ulg;
    Some(*gzindex as usize == source.len())
}

fn flush_pending_state(
    pending_buf_size: crate::zutil_h::ulg,
    pending: &mut crate::zutil_h::ulg,
    output_len: crate::stdlib::uInt,
) -> Option<(::core::ffi::c_uint, bool)> {
    if *pending > pending_buf_size {
        return None;
    }
    let len = if *pending > output_len as crate::zutil_h::ulg {
        output_len
    } else {
        *pending as ::core::ffi::c_uint
    };
    if len == 0 {
        return Some((0, false));
    }
    *pending = pending.checked_sub(len as crate::zutil_h::ulg)?;
    Some((len, *pending == 0))
}

/// Copy a bounded pending-output span into the caller's output span.  The
/// legacy adapter validates the two non-overlapping ABI lends; this core only
/// deals with ordinary slices so it cannot retain or dereference either
/// cursor.
fn flush_pending_copy_state(
    output: &mut [crate::stdlib::Byte],
    pending: &[crate::stdlib::Byte],
) -> bool {
    if output.len() != pending.len() {
        return false;
    }
    output.copy_from_slice(pending);
    true
}

fn set_stored_block_length_state(
    pending_buf: &mut [crate::stdlib::Byte],
    pending: crate::zutil_h::ulg,
    len: ::core::ffi::c_uint,
) -> bool {
    let Some(start) = pending
        .checked_sub(4)
        .and_then(|start| usize::try_from(start).ok())
    else {
        return false;
    };
    let Some(end) = start.checked_add(4) else {
        return false;
    };
    let Some(bytes) = pending_buf.get_mut(start..end) else {
        return false;
    };
    bytes[0] = len as crate::stdlib::Bytef;
    bytes[1] = (len >> 8) as crate::stdlib::Bytef;
    bytes[2] = !len as crate::stdlib::Bytef;
    bytes[3] = (!len >> 8) as crate::stdlib::Bytef;
    true
}

unsafe extern "C" fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    if strm.is_null() {
        return;
    }
    let strm = &mut *strm;
    let state = strm.state as *mut crate::src::deflate::deflate_state;
    if state.is_null() {
        return;
    }
    let s = &mut *state;
    let Ok(pending_len) = usize::try_from(s.pending_buf_size) else {
        return;
    };
    if pending_len != 0 && s.pending_buf.is_null() {
        return;
    }
    let pending_buf = if pending_len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(s.pending_buf, pending_len)
    };
    crate::src::trees::flush_bits_state(s, pending_buf);
    let Some((len, reset_pending_out)) =
        flush_pending_state(s.pending_buf_size, &mut s.pending, strm.avail_out)
    else {
        return;
    };
    if len == 0 {
        return;
    }
    let Ok(len) = usize::try_from(len) else {
        return;
    };
    if (len != 0 && (strm.next_out.is_null() || s.pending_out.is_null()))
        || len > strm.avail_out as usize
    {
        return;
    }
    // `memcpy` required raw cursors even after their bounds had been
    // validated.  Keep the ABI lends here, reject aliasing just as C
    // `memcpy` requires, then perform the actual copy in the slice core.
    let Some(output_end) = (strm.next_out as usize).checked_add(len) else {
        return;
    };
    let Some(pending_end) = (s.pending_out as usize).checked_add(len) else {
        return;
    };
    if (strm.next_out as usize) < pending_end && (s.pending_out as usize) < output_end {
        return;
    }
    let Some(pending_start) = (s.pending_out as usize).checked_sub(s.pending_buf as usize) else {
        return;
    };
    let Some(pending_end) = pending_start.checked_add(len) else {
        return;
    };
    let Some(pending) = pending_buf.get(pending_start..pending_end) else {
        return;
    };
    let output = ::core::slice::from_raw_parts_mut(strm.next_out, len);
    if !flush_pending_copy_state(output, pending) {
        return;
    }
    // The copied length is bounded by the validated output and pending
    // spans above. Preserve zlib's cursor advance without an unsafe pointer
    // offset operation in this transitional ABI adapter.
    strm.next_out = strm.next_out.wrapping_add(len);
    s.pending_out = s.pending_out.wrapping_add(len);
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out = strm.avail_out.wrapping_sub(len as crate::stdlib::uInt);
    if reset_pending_out {
        s.pending_out = s.pending_buf;
    }
}

/// Build the big-endian words emitted at the start of a zlib-wrapped stream.
///
/// The wrapper-specific bit and preset-dictionary decisions do not require
/// access to either the ABI stream or callback-owned output storage.
fn zlib_header_words(
    w_bits: crate::stdlib::uInt,
    strategy: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    adler: crate::stdlib::uLong,
) -> (
    crate::stdlib::uInt,
    Option<(crate::stdlib::uInt, crate::stdlib::uInt)>,
) {
    let mut header = (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt)
        .wrapping_add(w_bits.wrapping_sub(8) << 4)
        << 8;
    let level_flags = if strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || level < 2 {
        0
    } else if level < 6 {
        1
    } else if level == 6 {
        2
    } else {
        3
    };
    header |= (level_flags as crate::stdlib::uInt) << 6;
    let dictionary_adler = if strstart != 0 {
        header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
        Some((
            (adler >> 16) as crate::stdlib::uInt,
            (adler & 0xffff) as crate::stdlib::uInt,
        ))
    } else {
        None
    };
    header = header.wrapping_add(31u32.wrapping_sub(header.wrapping_rem(31)));
    (header, dictionary_adler)
}

/// Return whether a flush request cannot make progress without new input.
///
/// zlib ranks the finish-style flush values specially before comparing them
/// with the previous request.  Keep that C-style wrapping arithmetic in this
/// value-only helper so the stream adapter need not reproduce it inline.
fn flush_rank(flush: ::core::ffi::c_int) -> ::core::ffi::c_int {
    flush
        .wrapping_mul(2)
        .wrapping_sub(if flush > 4 { 9 } else { 0 })
}

fn repeated_flush_is_buffer_error(
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    previous_flush: ::core::ffi::c_int,
) -> bool {
    avail_in == 0
        && flush_rank(flush) <= flush_rank(previous_flush)
        && flush != crate::zlib_h::Z_FINISH
}

pub unsafe extern "C" fn deflate(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut old_flush: ::core::ffi::c_int = 0;
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflateStateCheck(strm) != 0
        || flush > crate::zlib_h::Z_BLOCK
        || flush < 0 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    if (*strm).next_out.is_null()
        || (*strm).avail_in != 0 as crate::stdlib::uInt && (*strm).next_in.is_null()
        || (*s).status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH
    {
        (*strm).msg =
            crate::src::zutil::z_errmsg[(if (-2 as ::core::ffi::c_int) < -6 as ::core::ffi::c_int
                || -2 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -2 as ::core::ffi::c_int
            }) as usize] as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        return -2 as ::core::ffi::c_int;
    }
    if (*strm).avail_out == 0 as crate::stdlib::uInt {
        (*strm).msg =
            crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int) < -6 as ::core::ffi::c_int
                || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
            }) as usize] as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    old_flush = (*s).last_flush;
    (*s).last_flush = flush;
    if (*s).pending != 0 as crate::zutil_h::ulg {
        flush_pending(strm);
        if (*strm).avail_out == 0 as crate::stdlib::uInt {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    } else if repeated_flush_is_buffer_error((*strm).avail_in, flush, old_flush) {
        (*strm).msg =
            crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int) < -6 as ::core::ffi::c_int
                || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
            }) as usize] as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    if (*s).status == crate::src::deflate::FINISH_STATE
        && (*strm).avail_in != 0 as crate::stdlib::uInt
    {
        (*strm).msg =
            crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int) < -6 as ::core::ffi::c_int
                || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
            }) as usize] as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    if (*s).status == crate::src::deflate::INIT_STATE && (*s).wrap == 0 as ::core::ffi::c_int {
        (*s).status = crate::src::deflate::BUSY_STATE;
    }
    if (*s).status == crate::src::deflate::INIT_STATE {
        let state = &mut *s;
        let (header, dictionary_adler) = zlib_header_words(
            state.w_bits,
            state.strategy,
            state.level,
            state.strstart,
            (*strm).adler,
        );
        let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if pending_len != 0 && state.pending_buf.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let pending_buf = if pending_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut(state.pending_buf, pending_len)
        };
        let header_written = match dictionary_adler {
            Some((adler_high, adler_low)) => append_zlib_words_state(
                pending_buf,
                &mut state.pending,
                &[header, adler_high, adler_low],
            ),
            None => append_zlib_words_state(pending_buf, &mut state.pending, &[header]),
        };
        if !header_written {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        (*strm).adler = crate::src::adler32::ADLER32_INITIAL;
        state.status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if (*s).status == crate::src::deflate::GZIP_STATE {
        (*strm).adler = crate::src::crc32::crc32_slice(0, &[]);
        let header = if (*s).gzhead.is_null() {
            None
        } else {
            let header = &*(*s).gzhead;
            Some(GzipFixedHeader {
                text: header.text != 0,
                hcrc: header.hcrc != 0,
                has_extra: !header.extra.is_null(),
                has_name: !header.name.is_null(),
                has_comment: !header.comment.is_null(),
                time: header.time,
                os: header.os,
                extra_len: header.extra_len,
            })
        };
        let Ok(pending_len) = usize::try_from((*s).pending_buf_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if pending_len != 0 && (*s).pending_buf.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let pending_buf = if pending_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut((*s).pending_buf, pending_len)
        };
        if !append_gzip_fixed_header_state(
            pending_buf,
            &mut (*s).pending,
            &mut (*strm).adler,
            (*s).level,
            (*s).strategy,
            header,
        ) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if header.is_none() {
            (*s).status = crate::src::deflate::BUSY_STATE;
            flush_pending(strm);
            if (*s).pending != 0 as crate::zutil_h::ulg {
                (*s).last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        } else {
            (*s).gzindex = 0 as crate::zutil_h::ulg;
            (*s).status = crate::src::deflate::EXTRA_STATE;
        }
    }
    if (*s).status == crate::src::deflate::EXTRA_STATE {
        if !(*(*s).gzhead).extra.is_null() {
            let header = &*(*s).gzhead;
            let extra_len = (header.extra_len & 0xffff as crate::stdlib::uInt) as usize;
            let extra = ::core::slice::from_raw_parts(header.extra, extra_len);
            loop {
                let state = &mut *s;
                let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_len != 0 && state.pending_buf.is_null() {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                let pending_buf = if pending_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(state.pending_buf, pending_len)
                };
                let Some(done) = append_gzip_extra_chunk_state(
                    pending_buf,
                    &mut state.pending,
                    &mut state.gzindex,
                    extra,
                    header.hcrc != 0,
                    &mut (*strm).adler,
                ) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if done {
                    state.gzindex = 0 as crate::zutil_h::ulg;
                    break;
                }
                if state.pending != state.pending_buf_size {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
        }
        (*s).status = crate::src::deflate::NAME_STATE;
    }
    if (*s).status == crate::src::deflate::NAME_STATE {
        if !(*(*s).gzhead).name.is_null() {
            let header = &*(*s).gzhead;
            let name = ::std::ffi::CStr::from_ptr(header.name.cast()).to_bytes_with_nul();
            loop {
                let state = &mut *s;
                let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_len != 0 && state.pending_buf.is_null() {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                let pending_buf = if pending_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(state.pending_buf, pending_len)
                };
                let Some(done) = append_gzip_cstring_chunk_state(
                    pending_buf,
                    &mut state.pending,
                    &mut state.gzindex,
                    name,
                    header.hcrc != 0,
                    &mut (*strm).adler,
                ) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if done {
                    state.gzindex = 0 as crate::zutil_h::ulg;
                    break;
                }
                if state.pending != state.pending_buf_size {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
        }
        (*s).status = crate::src::deflate::COMMENT_STATE;
    }
    if (*s).status == crate::src::deflate::COMMENT_STATE {
        if !(*(*s).gzhead).comment.is_null() {
            let header = &*(*s).gzhead;
            let comment = ::std::ffi::CStr::from_ptr(header.comment.cast()).to_bytes_with_nul();
            loop {
                let state = &mut *s;
                let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_len != 0 && state.pending_buf.is_null() {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                let pending_buf = if pending_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(state.pending_buf, pending_len)
                };
                let Some(done) = append_gzip_cstring_chunk_state(
                    pending_buf,
                    &mut state.pending,
                    &mut state.gzindex,
                    comment,
                    header.hcrc != 0,
                    &mut (*strm).adler,
                ) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if done {
                    state.gzindex = 0 as crate::zutil_h::ulg;
                    break;
                }
                if state.pending != state.pending_buf_size {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
        }
        (*s).status = crate::src::deflate::HCRC_STATE;
    }
    if (*s).status == crate::src::deflate::HCRC_STATE {
        if (*(*s).gzhead).hcrc != 0 {
            if (*s).pending.wrapping_add(2 as crate::zutil_h::ulg) > (*s).pending_buf_size {
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
            let hcrc = (*strm).adler;
            let state = &mut *s;
            let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            if pending_len != 0 && state.pending_buf.is_null() {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            let pending_buf = if pending_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.pending_buf, pending_len)
            };
            if !append_gzip_hcrc_state(pending_buf, &mut state.pending, hcrc) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            (*strm).adler = crate::src::crc32::crc32_slice(0, &[]);
        }
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if (*strm).avail_in != 0 as crate::stdlib::uInt
        || (*s).lookahead != 0 as crate::stdlib::uInt
        || flush != crate::zlib_h::Z_NO_FLUSH && (*s).status != crate::src::deflate::FINISH_STATE
    {
        let mut bstate: block_state = need_more;
        bstate = (if (*s).level == 0 as ::core::ffi::c_int {
            deflate_stored(s, flush) as ::core::ffi::c_uint
        } else if (*s).strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
            deflate_huff(s, flush) as ::core::ffi::c_uint
        } else if (*s).strategy == crate::zlib_h::Z_RLE {
            deflate_rle(s, flush) as ::core::ffi::c_uint
        } else {
            let compressor: compress_func = match configuration_table[(*s).level as usize].kind {
                CompressorKind::Stored => Some(deflate_stored),
                CompressorKind::Fast => Some(deflate_fast),
                CompressorKind::Slow => Some(deflate_slow),
            };
            compressor.expect("configuration compressor")(s, flush) as ::core::ffi::c_uint
        }) as block_state;
        if bstate as ::core::ffi::c_uint
            == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
            || bstate as ::core::ffi::c_uint
                == finish_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*s).status = crate::src::deflate::FINISH_STATE;
        }
        if bstate as ::core::ffi::c_uint == need_more as ::core::ffi::c_int as ::core::ffi::c_uint
            || bstate as ::core::ffi::c_uint
                == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                (*s).last_flush = -1 as ::core::ffi::c_int;
            }
            return crate::zlib_h::Z_OK;
        }
        if bstate as ::core::ffi::c_uint == block_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if flush == crate::zlib_h::Z_PARTIAL_FLUSH {
                crate::src::trees::_tr_align(s as *mut crate::src::deflate::internal_state);
            } else if flush != crate::zlib_h::Z_BLOCK {
                crate::src::trees::_tr_stored_block(
                    s as *mut crate::src::deflate::internal_state,
                    ::core::ptr::null_mut::<crate::stdlib::charf>(),
                    0 as crate::zutil_h::ulg,
                    0 as ::core::ffi::c_int,
                );
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    *(*s).head.wrapping_add(
                        (*s).hash_size.wrapping_sub(1 as crate::stdlib::uInt) as usize,
                    ) = NIL as crate::src::deflate::Posf;
                    crate::stdlib::memset(
                        (*s).head as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ((*s).hash_size.wrapping_sub(1 as crate::stdlib::uInt)
                            as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Posf>()),
                    );
                    (*s).slid = 0 as ::core::ffi::c_int;
                    if (*s).lookahead == 0 as crate::stdlib::uInt {
                        (*s).strstart = 0 as crate::stdlib::uInt;
                        (*s).block_start = 0 as ::core::ffi::c_long;
                        (*s).insert = 0 as crate::stdlib::uInt;
                    }
                }
            }
            flush_pending(strm);
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                (*s).last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        }
    }
    if flush != crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_OK;
    }
    if (*s).wrap <= 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_END;
    }
    if (*s).wrap == 2 as ::core::ffi::c_int {
        let Ok(pending_len) = usize::try_from((*s).pending_buf_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if pending_len != 0 && (*s).pending_buf.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let pending_buf = if pending_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut((*s).pending_buf, pending_len)
        };
        if !append_gzip_trailer_state(
            pending_buf,
            &mut (*s).pending,
            (*strm).adler,
            (*strm).total_in,
        ) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
    } else {
        let state = &mut *s;
        let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if pending_len != 0 && state.pending_buf.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let pending_buf = if pending_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut(state.pending_buf, pending_len)
        };
        if !append_zlib_words_state(
            pending_buf,
            &mut state.pending,
            &[
                ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
                ((*strm).adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
            ],
        ) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
    }
    flush_pending(strm);
    if (*s).wrap > 0 as ::core::ffi::c_int {
        (*s).wrap = -(*s).wrap;
    }
    return if (*s).pending != 0 as crate::zutil_h::ulg {
        crate::zlib_h::Z_OK
    } else {
        crate::zlib_h::Z_STREAM_END
    };
}
#[export_name = "deflate"]

pub unsafe extern "C" fn deflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflate(strm, flush)
}
fn deflate_end_status(status: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    }
}

pub unsafe extern "C" fn deflateEnd(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    status = (*(*strm).state).status;
    if !(*(*strm).state).pending_buf.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).pending_buf as crate::stdlib::voidpf,
        );
    }
    if !(*(*strm).state).head.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).head as crate::stdlib::voidpf,
        );
    }
    if !(*(*strm).state).prev.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).prev as crate::stdlib::voidpf,
        );
    }
    if !(*(*strm).state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).window as crate::stdlib::voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as crate::stdlib::voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    deflate_end_status(status)
}
#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    deflateEnd(strm)
}
#[export_name = "deflateCopy"]
pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut ds: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut ss: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflateStateCheck(source) != 0 || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    ss = (*source).state as *mut crate::src::deflate::deflate_state;
    crate::stdlib::memcpy(
        dest as *mut ::core::ffi::c_void,
        source as *const ::core::ffi::c_void,
        ::core::mem::size_of::<crate::zlib_h::z_stream>(),
    );
    ds = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::deflate_state;
    if ds.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    crate::stdlib::memset(
        ds as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>(),
    );
    (*dest).state = ds as *mut crate::src::deflate::internal_state;
    crate::stdlib::memcpy(
        ds as *mut ::core::ffi::c_void,
        ss as *const ::core::ffi::c_void,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>(),
    );
    (*ds).strm = dest;
    (*ds).window = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
            as crate::stdlib::uInt,
    ) as *mut crate::stdlib::Bytef;
    (*ds).prev = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).w_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    (*ds).head = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).hash_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    (*ds).pending_buf = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).lit_bufsize,
        4 as crate::stdlib::uInt,
    ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef;
    if (*ds).window.is_null()
        || (*ds).prev.is_null()
        || (*ds).head.is_null()
        || (*ds).pending_buf.is_null()
    {
        deflateEnd(dest);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    crate::stdlib::memcpy(
        (*ds).window as *mut ::core::ffi::c_void,
        (*ss).window as *const ::core::ffi::c_void,
        (*ss).high_water as crate::__stddef_size_t_h::size_t,
    );
    crate::stdlib::memcpy(
        (*ds).prev as *mut ::core::ffi::c_void,
        (*ss).prev as *const ::core::ffi::c_void,
        ((if (*ss).slid != 0 || (*ss).strstart.wrapping_sub((*ss).insert) > (*ds).w_size {
            (*ds).w_size
        } else {
            (*ss).strstart.wrapping_sub((*ss).insert)
        }) as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Pos>()),
    );
    crate::stdlib::memcpy(
        (*ds).head as *mut ::core::ffi::c_void,
        (*ss).head as *const ::core::ffi::c_void,
        ((*ds).hash_size as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Pos>()),
    );
    let pending_out_offset = (*ss).pending_out.offset_from((*ss).pending_buf);
    (*ds).pending_out = (*ds).pending_buf.wrapping_offset(pending_out_offset);
    crate::stdlib::memcpy(
        (*ds).pending_out as *mut ::core::ffi::c_void,
        (*ss).pending_out as *const ::core::ffi::c_void,
        (*ss).pending as crate::__stddef_size_t_h::size_t,
    );
    (*ds).sym_buf =
        (*ds).pending_buf.wrapping_add((*ds).lit_bufsize as usize) as *mut crate::zutil_h::uchf;
    crate::stdlib::memcpy(
        (*ds).sym_buf as *mut ::core::ffi::c_void,
        (*ss).sym_buf as *const ::core::ffi::c_void,
        (*ss).sym_next as crate::__stddef_size_t_h::size_t,
    );
    return crate::zlib_h::Z_OK;
}
fn longest_match_state(
    s: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Byte],
    prev: &[crate::src::deflate::Posf],
    mut cur_match: crate::src::deflate::IPos,
) -> Option<crate::stdlib::uInt> {
    let max_match = usize::try_from(crate::zutil_h::MAX_MATCH).ok()?;
    let strstart = usize::try_from(s.strstart).ok()?;
    let strend = strstart.checked_add(max_match)?;
    if strend > window.len() {
        return None;
    }

    let mut chain_length = s.max_chain_length as ::core::ffi::c_uint;
    let mut best_len = usize::try_from(s.prev_length).ok()?;
    if best_len < 2 || best_len >= max_match {
        return None;
    }
    let mut nice_match = s.nice_match;
    let limit = if s.strstart
        > s.w_size
            .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
    {
        (s.strstart as crate::src::deflate::IPos).wrapping_sub(
            s.w_size
                .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        )
    } else {
        NIL as crate::src::deflate::IPos
    };
    if s.prev_length >= s.good_match {
        chain_length >>= 2;
    }
    if nice_match as crate::stdlib::uInt > s.lookahead {
        nice_match = s.lookahead as ::core::ffi::c_int;
    }

    loop {
        let candidate = usize::try_from(cur_match).ok()?;
        let candidate_end = candidate.checked_add(max_match)?;
        if candidate_end > window.len() {
            return None;
        }
        if window[candidate + best_len] == window[strstart + best_len]
            && window[candidate + best_len - 1] == window[strstart + best_len - 1]
            && window[candidate] == window[strstart]
            && window[candidate + 1] == window[strstart + 1]
        {
            let mut len = 2usize;
            while len < max_match && window[strstart + len] == window[candidate + len] {
                len += 1;
            }
            if len > best_len {
                s.match_start = cur_match as crate::stdlib::uInt;
                best_len = len;
                if len as ::core::ffi::c_int >= nice_match {
                    break;
                }
            }
        }
        let prev_index = usize::try_from((cur_match as crate::stdlib::uInt) & s.w_mask).ok()?;
        cur_match = *prev.get(prev_index)? as crate::src::deflate::IPos;
        chain_length = chain_length.wrapping_sub(1);
        if cur_match <= limit || chain_length == 0 {
            break;
        }
    }
    Some((best_len as crate::stdlib::uInt).min(s.lookahead))
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

fn update_stored_history_state(
    s: &mut crate::src::deflate::deflate_state,
    window: &mut [crate::stdlib::Byte],
    consumed: &[crate::stdlib::Byte],
) -> bool {
    let Ok(used) = crate::stdlib::uInt::try_from(consumed.len()) else {
        return false;
    };
    if used == 0 {
        return true;
    }
    let Ok(wsize) = usize::try_from(s.w_size) else {
        return false;
    };
    let Ok(window_size) = usize::try_from(s.window_size) else {
        return false;
    };
    let Ok(strstart) = usize::try_from(s.strstart) else {
        return false;
    };
    let Ok(insert) = usize::try_from(s.insert) else {
        return false;
    };
    if wsize == 0
        || window_size > window.len()
        || wsize > window_size
        || strstart > window_size
        || insert > wsize
    {
        return false;
    }

    if used >= s.w_size {
        if wsize > consumed.len() || wsize > window_size {
            return false;
        }
        let tail_start = consumed.len() - wsize;
        window[..wsize].copy_from_slice(&consumed[tail_start..]);
        s.matches = 2;
        s.strstart = s.w_size;
        s.insert = s.strstart;
    } else {
        let used = used as usize;
        let slide = s
            .window_size
            .wrapping_sub(s.strstart as crate::zutil_h::ulg)
            <= used as crate::zutil_h::ulg;
        let new_strstart = if slide {
            let Some(new_strstart) = strstart.checked_sub(wsize) else {
                return false;
            };
            let Some(source_end) = wsize.checked_add(new_strstart) else {
                return false;
            };
            if source_end > window_size {
                return false;
            }
            new_strstart
        } else {
            strstart
        };
        let Some(destination_end) = new_strstart.checked_add(used) else {
            return false;
        };
        if destination_end > window_size {
            return false;
        }
        let new_insert = if slide {
            insert.min(new_strstart)
        } else {
            insert
        };
        let insert_add = used.min(wsize - new_insert);

        if slide {
            let source_end = wsize + new_strstart;
            window.copy_within(wsize..source_end, 0);
            if s.matches < 2 {
                s.matches = s.matches.wrapping_add(1);
            }
        }
        window[new_strstart..destination_end].copy_from_slice(consumed);
        s.strstart = (new_strstart + used) as crate::stdlib::uInt;
        s.insert = (new_insert + insert_add) as crate::stdlib::uInt;
    }
    s.block_start = s.strstart as ::core::ffi::c_long;
    if s.high_water < s.strstart as crate::zutil_h::ulg {
        s.high_water = s.strstart as crate::zutil_h::ulg;
    }
    true
}

/// Make room at the front of the stored-mode history window when the next
/// caller input will not fit after the current block.  This is the exact
/// overlap-aware move performed by `deflate_stored`; keeping it here means
/// the callback-owned window is only lent once at the legacy adapter.
fn rebalance_stored_window_state(
    s: &mut crate::src::deflate::deflate_state,
    window: &mut [crate::stdlib::Byte],
    avail_in: crate::stdlib::uInt,
) -> Option<::core::ffi::c_uint> {
    let mut have = s
        .window_size
        .wrapping_sub(s.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if avail_in <= have || s.block_start < s.w_size as ::core::ffi::c_long {
        return Some(have);
    }

    let window_size = usize::try_from(s.window_size).ok()?;
    let wsize = usize::try_from(s.w_size).ok()?;
    let strstart = usize::try_from(s.strstart).ok()?;
    if window_size > window.len() || wsize > window_size || strstart < wsize {
        return None;
    }
    let source_end = wsize.checked_add(strstart)?;
    if source_end > window_size {
        return None;
    }

    window.copy_within(wsize..source_end, 0);
    s.block_start = s.block_start.wrapping_sub(s.w_size as ::core::ffi::c_long);
    s.strstart = s.strstart.wrapping_sub(s.w_size);
    if s.matches < 2 as crate::stdlib::uInt {
        s.matches = s.matches.wrapping_add(1);
    }
    have = have.wrapping_add(s.w_size as ::core::ffi::c_uint);
    if s.insert > s.strstart {
        s.insert = s.strstart;
    }
    Some(have)
}

struct StoredBlockPlan {
    len: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    last: ::core::ffi::c_int,
}

struct StoredOutputCopy {
    copied: crate::stdlib::uInt,
    remaining: ::core::ffi::c_uint,
    block_start: ::core::ffi::c_long,
}

/// Calculate the source length passed to the tree block flusher.  The cast
/// deliberately retains zlib's wrapping conversion when an ABI state has a
/// negative block cursor; the raw mode adapters still choose the null source
/// pointer for that case.
fn block_flush_len_state(
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
) -> crate::zutil_h::ulg {
    (strstart as ::core::ffi::c_long - block_start) as crate::zutil_h::ulg
}

/// Copy the already-buffered portion of a stored block to the caller's output.
/// The mode adapter lends the legacy window and output storage; this core only
/// performs the checked, non-overlapping byte copy and its scalar transitions.
fn copy_stored_window_to_output_state(
    window: &[crate::stdlib::Byte],
    output: &mut [crate::stdlib::Byte],
    block_start: ::core::ffi::c_long,
    left: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
) -> Option<StoredOutputCopy> {
    if block_start < 0 {
        return None;
    }
    let copied = left.min(len);
    let copied = usize::try_from(copied).ok()?;
    let start = usize::try_from(block_start).ok()?;
    let end = start.checked_add(copied)?;
    let source = window.get(start..end)?;
    let destination = output.get_mut(..copied)?;
    destination.copy_from_slice(source);
    Some(StoredOutputCopy {
        copied: crate::stdlib::uInt::try_from(copied).ok()?,
        remaining: len.wrapping_sub(copied as ::core::ffi::c_uint),
        block_start: block_start.checked_add(copied as ::core::ffi::c_long)?,
    })
}

fn stored_block_plan(
    min_block: ::core::ffi::c_uint,
    bi_valid: ::core::ffi::c_int,
    avail_out: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> Option<StoredBlockPlan> {
    let mut len = MAX_STORED as ::core::ffi::c_uint;
    let bit_bytes = (bi_valid as ::core::ffi::c_uint).wrapping_add(42) >> 3;
    if avail_out < bit_bytes {
        return None;
    }
    let have = avail_out.wrapping_sub(bit_bytes);
    let left = (strstart as ::core::ffi::c_long - block_start) as ::core::ffi::c_uint;
    if len as crate::zutil_h::ulg
        > (left as crate::zutil_h::ulg).wrapping_add(avail_in as crate::zutil_h::ulg)
    {
        len = left.wrapping_add(avail_in);
    }
    if len > have {
        len = have;
    }
    if len < min_block
        && (len == 0 && flush != crate::zlib_h::Z_FINISH
            || flush == crate::zlib_h::Z_NO_FLUSH
            || len != left.wrapping_add(avail_in))
    {
        return None;
    }
    Some(StoredBlockPlan {
        len,
        left,
        last: (flush == crate::zlib_h::Z_FINISH && len == left.wrapping_add(avail_in))
            as ::core::ffi::c_int,
    })
}

/// Select the first stored block for the current mode iteration without
/// touching callback-owned storage.  Keeping the capacity calculation with
/// the block admission test prevents the raw mode adapter from reimplementing
/// any of the wrapping arithmetic.
fn stored_initial_block_plan(
    pending_buf_size: crate::zutil_h::ulg,
    wsize: crate::stdlib::uInt,
    bi_valid: ::core::ffi::c_int,
    avail_out: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> Option<StoredBlockPlan> {
    let min_block = (if pending_buf_size.wrapping_sub(5) > wsize as crate::zutil_h::ulg {
        wsize as crate::zutil_h::ulg
    } else {
        pending_buf_size.wrapping_sub(5)
    }) as ::core::ffi::c_uint;
    stored_block_plan(
        min_block,
        bi_valid,
        avail_out,
        strstart,
        block_start,
        avail_in,
        flush,
    )
}

/// Plan the stored block that may be emitted after refilling the history
/// window.  This is deliberately value-only: the mode adapter retains the
/// callback-owned pending and window buffers used to carry out the plan.
fn stored_tail_block_plan(
    pending_buf_size: crate::zutil_h::ulg,
    bi_valid: ::core::ffi::c_int,
    wsize: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> Option<StoredBlockPlan> {
    let bit_bytes = (bi_valid as ::core::ffi::c_uint).wrapping_add(42) >> 3;
    let have = (if pending_buf_size.wrapping_sub(bit_bytes as crate::zutil_h::ulg)
        > 65535 as crate::zutil_h::ulg
    {
        65535 as crate::zutil_h::ulg
    } else {
        pending_buf_size.wrapping_sub(bit_bytes as crate::zutil_h::ulg)
    }) as ::core::ffi::c_uint;
    let min_block = if have > wsize {
        wsize as ::core::ffi::c_uint
    } else {
        have
    };
    let left = (strstart as ::core::ffi::c_long - block_start) as ::core::ffi::c_uint;
    if left < min_block
        && !((left != 0 || flush == crate::zlib_h::Z_FINISH)
            && flush != crate::zlib_h::Z_NO_FLUSH
            && avail_in == 0
            && left <= have)
    {
        return None;
    }
    let len = left.min(have);
    Some(StoredBlockPlan {
        len,
        left,
        last: (flush == crate::zlib_h::Z_FINISH && avail_in == 0 && len == left)
            as ::core::ffi::c_int,
    })
}

/// Record the history-window cursors after stored mode has copied `have`
/// caller bytes into the window.  The legacy adapter retains the raw input and
/// window lends; this transition is only scalar state and deliberately keeps
/// zlib's wrapping arithmetic.
fn record_stored_input_state(
    s: &mut crate::src::deflate::deflate_state,
    have: crate::stdlib::uInt,
) {
    s.strstart = s.strstart.wrapping_add(have);
    s.insert = s
        .insert
        .wrapping_add(if have > s.w_size.wrapping_sub(s.insert) {
            (s.w_size as ::core::ffi::c_uint).wrapping_sub(s.insert as ::core::ffi::c_uint)
        } else {
            have
        });
    if s.high_water < s.strstart as crate::zutil_h::ulg {
        s.high_water = s.strstart as crate::zutil_h::ulg;
    }
}

unsafe extern "C" fn deflate_stored(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used: ::core::ffi::c_uint = (*(*s).strm).avail_in as ::core::ffi::c_uint;
    loop {
        let Some(plan) = stored_initial_block_plan(
            (*s).pending_buf_size,
            (*s).w_size,
            (*s).bi_valid,
            (*(*s).strm).avail_out,
            (*s).strstart,
            (*s).block_start,
            (*(*s).strm).avail_in,
            flush,
        ) else {
            break;
        };
        len = plan.len;
        left = plan.left;
        last = plan.last;
        // The zero-length stored block needs only the already-owned pending
        // buffer.  Lend that validated buffer directly to the safe core
        // instead of round-tripping through the raw `_tr_stored_block`
        // adapter.
        let state = &mut *s;
        let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
            return need_more;
        };
        if pending_len != 0 && state.pending_buf.is_null() {
            return need_more;
        }
        let pending_buf = if pending_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut(state.pending_buf, pending_len)
        };
        let _ = crate::src::trees::tr_stored_block_state(
            pending_buf,
            &mut state.pending,
            &mut state.bi_buf,
            &mut state.bi_valid,
            &mut state.bi_used,
            &[],
            last,
        );
        if !set_stored_block_length_state(pending_buf, state.pending, len) {
            return need_more;
        }
        flush_pending(state.strm);
        if left != 0 {
            let state = &mut *s;
            let strm = &mut *state.strm;
            let (Ok(window_len), Ok(output_len)) = (
                usize::try_from(state.window_size),
                usize::try_from(strm.avail_out),
            ) else {
                return need_more;
            };
            if (window_len != 0 && state.window.is_null())
                || (output_len != 0 && strm.next_out.is_null())
            {
                return need_more;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(state.window, window_len)
            };
            let output = if output_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(strm.next_out, output_len)
            };
            let Some(copy) =
                copy_stored_window_to_output_state(window, output, state.block_start, left, len)
            else {
                return need_more;
            };
            // `copy.copied` is bounded by the output view above.  Preserve
            // the ABI cursor advance without unsafe pointer arithmetic.
            strm.next_out = strm.next_out.wrapping_add(copy.copied as usize);
            strm.avail_out = strm.avail_out.wrapping_sub(copy.copied);
            strm.total_out = strm
                .total_out
                .wrapping_add(copy.copied as crate::stdlib::uLong);
            state.block_start = copy.block_start;
            len = copy.remaining;
        }
        if len != 0 {
            read_buf((*s).strm, (*(*s).strm).next_out, len);
            // `read_buf()` consumed at most the requested `len` bytes, so
            // this is a cursor update only; no pointer dereference is needed.
            (*(*s).strm).next_out = (*(*s).strm).next_out.wrapping_add(len as usize);
            (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(len);
            (*(*s).strm).total_out = (*(*s).strm)
                .total_out
                .wrapping_add(len as crate::stdlib::uLong);
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub((*(*s).strm).avail_in as ::core::ffi::c_uint);
    if used != 0 {
        let Ok(window_len) = usize::try_from((*s).window_size) else {
            return need_more;
        };
        let Ok(used_len) = usize::try_from(used) else {
            return need_more;
        };
        if (*s).window.is_null() || (*(*s).strm).next_in.is_null() {
            return need_more;
        }
        let window = ::core::slice::from_raw_parts_mut((*s).window, window_len);
        let consumed = ::core::slice::from_raw_parts(
            (*(*s).strm).next_in.wrapping_sub(used as usize),
            used_len,
        );
        if !update_stored_history_state(&mut *s, window, consumed) {
            return need_more;
        }
    }
    if last != 0 {
        (*s).bi_used = 8 as ::core::ffi::c_int;
        return finish_done;
    }
    if flush != crate::zlib_h::Z_NO_FLUSH
        && flush != crate::zlib_h::Z_FINISH
        && (*(*s).strm).avail_in == 0 as crate::stdlib::uInt
        && (*s).strstart as ::core::ffi::c_long == (*s).block_start
    {
        return block_done;
    }
    let Ok(window_len) = usize::try_from((*s).window_size) else {
        return need_more;
    };
    if window_len != 0 && (*s).window.is_null() {
        return need_more;
    }
    let window = if window_len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut((*s).window, window_len)
    };
    let Some(next_have) = rebalance_stored_window_state(&mut *s, window, (*(*s).strm).avail_in)
    else {
        return need_more;
    };
    have = next_have;
    if have > (*(*s).strm).avail_in {
        have = (*(*s).strm).avail_in as ::core::ffi::c_uint;
    }
    if have != 0 {
        read_buf(
            (*s).strm,
            (*s).window.wrapping_add((*s).strstart as usize),
            have,
        );
        record_stored_input_state(&mut *s, have);
    }
    if let Some(plan) = stored_tail_block_plan(
        (*s).pending_buf_size,
        (*s).bi_valid,
        (*s).w_size,
        (*s).strstart,
        (*s).block_start,
        (*(*s).strm).avail_in,
        flush,
    ) {
        len = plan.len;
        last = plan.last;
        crate::src::trees::_tr_stored_block(
            s as *mut crate::src::deflate::internal_state,
            ((*s).window as *mut crate::stdlib::charf).wrapping_offset((*s).block_start as isize),
            len as crate::zutil_h::ulg,
            last,
        );
        (*s).block_start += len as ::core::ffi::c_long;
        flush_pending((*s).strm);
    }
    if last != 0 {
        (*s).bi_used = 8 as ::core::ffi::c_int;
    }
    return (if last != 0 {
        finish_started as ::core::ffi::c_int
    } else {
        need_more as ::core::ffi::c_int
    }) as block_state;
}

unsafe extern "C" fn deflate_fast(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window(s);
            if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if (*s).lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        hash_head = NIL as crate::src::deflate::IPos;
        if (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let state = &mut *s;
            let (Ok(window_len), Ok(head_len), Ok(prev_len)) = (
                usize::try_from(state.window_size),
                usize::try_from(state.hash_size),
                usize::try_from(state.w_size),
            ) else {
                return need_more;
            };
            if (window_len != 0 && state.window.is_null())
                || (head_len != 0 && state.head.is_null())
                || (prev_len != 0 && state.prev.is_null())
            {
                return need_more;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(state.window, window_len)
            };
            let head = if head_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.head, head_len)
            };
            let prev = if prev_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.prev, prev_len)
            };
            let Some(previous) = insert_string_state(
                window,
                head,
                prev,
                state.strstart,
                &mut state.ins_h,
                state.hash_shift,
                state.hash_mask,
                state.w_mask,
            ) else {
                return need_more;
            };
            hash_head = previous;
            if hash_match_is_usable(
                state.strstart as crate::src::deflate::IPos,
                hash_head,
                state.w_size,
            ) {
                state.match_length =
                    longest_match_state(state, window, prev, hash_head).unwrap_or(0);
            }
        }
        if (*s).match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let state = &mut *s;
            let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                return need_more;
            };
            if symbol_len != 0 && state.sym_buf.is_null() {
                return need_more;
            }
            let symbols = if symbol_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.sym_buf, symbol_len)
            };
            let Some((flush_now, length)) = fast_tally_match_state(state, symbols) else {
                return need_more;
            };
            bflush = flush_now as ::core::ffi::c_int;
            if length <= state.max_lazy_match
                && state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            {
                state.match_length = length.wrapping_sub(1);
                loop {
                    state.strstart = state.strstart.wrapping_add(1);
                    let (Ok(window_len), Ok(head_len), Ok(prev_len)) = (
                        usize::try_from(state.window_size),
                        usize::try_from(state.hash_size),
                        usize::try_from(state.w_size),
                    ) else {
                        return need_more;
                    };
                    if (window_len != 0 && state.window.is_null())
                        || (head_len != 0 && state.head.is_null())
                        || (prev_len != 0 && state.prev.is_null())
                    {
                        return need_more;
                    }
                    let window = if window_len == 0 {
                        &[]
                    } else {
                        ::core::slice::from_raw_parts(state.window, window_len)
                    };
                    let head = if head_len == 0 {
                        &mut []
                    } else {
                        ::core::slice::from_raw_parts_mut(state.head, head_len)
                    };
                    let prev = if prev_len == 0 {
                        &mut []
                    } else {
                        ::core::slice::from_raw_parts_mut(state.prev, prev_len)
                    };
                    let Some(previous) = insert_string_state(
                        window,
                        head,
                        prev,
                        state.strstart,
                        &mut state.ins_h,
                        state.hash_shift,
                        state.hash_mask,
                        state.w_mask,
                    ) else {
                        return need_more;
                    };
                    hash_head = previous;
                    state.match_length = state.match_length.wrapping_sub(1);
                    if state.match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                state.strstart = state.strstart.wrapping_add(1);
            } else {
                state.strstart = state.strstart.wrapping_add(state.match_length);
                state.match_length = 0 as crate::stdlib::uInt;
                let Ok(window_len) = usize::try_from(state.window_size) else {
                    return need_more;
                };
                if window_len != 0 && state.window.is_null() {
                    return need_more;
                }
                let window = if window_len == 0 {
                    &[]
                } else {
                    ::core::slice::from_raw_parts(state.window, window_len)
                };
                let Some(ins_h) = initialize_hash_state(
                    window,
                    state.strstart,
                    state.hash_shift,
                    state.hash_mask,
                ) else {
                    return need_more;
                };
                state.ins_h = ins_h;
            }
        } else {
            let state = &mut *s;
            let Ok(window_len) = usize::try_from(state.window_size) else {
                return need_more;
            };
            let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                return need_more;
            };
            if (window_len != 0 && state.window.is_null())
                || (symbol_len != 0 && state.sym_buf.is_null())
            {
                return need_more;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(state.window, window_len)
            };
            let symbols = if symbol_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.sym_buf, symbol_len)
            };
            let Some(flush_now) = tally_current_literal_state(state, window, symbols) else {
                return need_more;
            };
            bflush = flush_now as ::core::ffi::c_int;
        }
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
                s as *mut crate::src::deflate::internal_state,
                if (*s).block_start >= 0 as ::core::ffi::c_long {
                    (*s).window
                        .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                block_flush_len_state((*s).strstart, (*s).block_start),
                0 as ::core::ffi::c_int,
            );
            (*s).block_start = (*s).strstart as ::core::ffi::c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
                return (if false {
                    finish_started as ::core::ffi::c_int
                } else {
                    need_more as ::core::ffi::c_int
                }) as block_state;
            }
        }
    }
    (*s).insert = if (*s).strstart
        < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        (*s).strstart
    } else {
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    };
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            block_flush_len_state((*s).strstart, (*s).block_start),
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).sym_next != 0 {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            block_flush_len_state((*s).strstart, (*s).block_start),
            0 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

unsafe extern "C" fn deflate_slow(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window(s);
            if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if (*s).lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        hash_head = NIL as crate::src::deflate::IPos;
        (*s).prev_length = (*s).match_length;
        (*s).prev_match = (*s).match_start as crate::src::deflate::IPos;
        (*s).match_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        if (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let state = &mut *s;
            let (Ok(window_len), Ok(head_len), Ok(prev_len)) = (
                usize::try_from(state.window_size),
                usize::try_from(state.hash_size),
                usize::try_from(state.w_size),
            ) else {
                return need_more;
            };
            if (window_len != 0 && state.window.is_null())
                || (head_len != 0 && state.head.is_null())
                || (prev_len != 0 && state.prev.is_null())
            {
                return need_more;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(state.window, window_len)
            };
            let head = if head_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.head, head_len)
            };
            let prev = if prev_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.prev, prev_len)
            };
            let Some(previous) = insert_string_state(
                window,
                head,
                prev,
                state.strstart,
                &mut state.ins_h,
                state.hash_shift,
                state.hash_mask,
                state.w_mask,
            ) else {
                return need_more;
            };
            hash_head = previous;
            if lazy_hash_match_is_usable(
                state.strstart as crate::src::deflate::IPos,
                hash_head,
                state.prev_length,
                state.max_lazy_match,
                state.w_size,
            ) {
                state.match_length =
                    longest_match_state(state, window, prev, hash_head).unwrap_or(0);
                state.match_length = filtered_match_length(
                    state.match_length,
                    state.strategy,
                    state.strstart,
                    state.match_start,
                );
            }
        }
        if (*s).prev_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            && (*s).match_length <= (*s).prev_length
        {
            let max_insert: crate::stdlib::uInt = (*s)
                .strstart
                .wrapping_add((*s).lookahead)
                .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
            let len: crate::zutil_h::uch =
                (*s).prev_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush = ((*s).strstart as crate::src::deflate::IPos)
                .wrapping_sub(1 as crate::src::deflate::IPos)
                .wrapping_sub((*s).prev_match)
                as crate::zutil_h::ush;
            let state = &mut *s;
            let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                return need_more;
            };
            if symbol_len != 0 && state.sym_buf.is_null() {
                return need_more;
            }
            let symbols = if symbol_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.sym_buf, symbol_len)
            };
            let Some(flush_now) = tally_symbol_state(state, symbols, dist.into(), len.into())
            else {
                return need_more;
            };
            bflush = flush_now as ::core::ffi::c_int;
            (*s).lookahead = (*s)
                .lookahead
                .wrapping_sub((*s).prev_length.wrapping_sub(1 as crate::stdlib::uInt));
            (*s).prev_length = (*s).prev_length.wrapping_sub(2 as crate::stdlib::uInt);
            loop {
                (*s).strstart = (*s).strstart.wrapping_add(1);
                if (*s).strstart <= max_insert {
                    let state = &mut *s;
                    let (Ok(window_len), Ok(head_len), Ok(prev_len)) = (
                        usize::try_from(state.window_size),
                        usize::try_from(state.hash_size),
                        usize::try_from(state.w_size),
                    ) else {
                        return need_more;
                    };
                    if (window_len != 0 && state.window.is_null())
                        || (head_len != 0 && state.head.is_null())
                        || (prev_len != 0 && state.prev.is_null())
                    {
                        return need_more;
                    }
                    let window = if window_len == 0 {
                        &[]
                    } else {
                        ::core::slice::from_raw_parts(state.window, window_len)
                    };
                    let head = if head_len == 0 {
                        &mut []
                    } else {
                        ::core::slice::from_raw_parts_mut(state.head, head_len)
                    };
                    let prev = if prev_len == 0 {
                        &mut []
                    } else {
                        ::core::slice::from_raw_parts_mut(state.prev, prev_len)
                    };
                    let Some(previous) = insert_string_state(
                        window,
                        head,
                        prev,
                        state.strstart,
                        &mut state.ins_h,
                        state.hash_shift,
                        state.hash_mask,
                        state.w_mask,
                    ) else {
                        return need_more;
                    };
                    hash_head = previous;
                }
                (*s).prev_length = (*s).prev_length.wrapping_sub(1);
                if (*s).prev_length == 0 as crate::stdlib::uInt {
                    break;
                }
            }
            (*s).match_available = 0 as ::core::ffi::c_int;
            (*s).match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            (*s).strstart = (*s).strstart.wrapping_add(1);
            if bflush != 0 {
                crate::src::trees::_tr_flush_block(
                    s as *mut crate::src::deflate::internal_state,
                    if (*s).block_start >= 0 as ::core::ffi::c_long {
                        (*s).window
                            .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                            as *mut crate::stdlib::charf
                    } else {
                        ::core::ptr::null_mut::<crate::stdlib::charf>()
                    },
                    block_flush_len_state((*s).strstart, (*s).block_start),
                    0 as ::core::ffi::c_int,
                );
                (*s).block_start = (*s).strstart as ::core::ffi::c_long;
                flush_pending((*s).strm);
                if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
                    return (if false {
                        finish_started as ::core::ffi::c_int
                    } else {
                        need_more as ::core::ffi::c_int
                    }) as block_state;
                }
            }
        } else if (*s).match_available != 0 {
            let state = &mut *s;
            let Ok(window_len) = usize::try_from(state.window_size) else {
                return need_more;
            };
            let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                return need_more;
            };
            if (window_len != 0 && state.window.is_null())
                || (symbol_len != 0 && state.sym_buf.is_null())
            {
                return need_more;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(state.window, window_len)
            };
            let symbols = if symbol_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.sym_buf, symbol_len)
            };
            let Some(flush_now) = tally_previous_literal_state(state, window, symbols) else {
                return need_more;
            };
            bflush = flush_now as ::core::ffi::c_int;
            if bflush != 0 {
                crate::src::trees::_tr_flush_block(
                    s as *mut crate::src::deflate::internal_state,
                    if (*s).block_start >= 0 as ::core::ffi::c_long {
                        (*s).window
                            .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                            as *mut crate::stdlib::charf
                    } else {
                        ::core::ptr::null_mut::<crate::stdlib::charf>()
                    },
                    block_flush_len_state((*s).strstart, (*s).block_start),
                    0 as ::core::ffi::c_int,
                );
                (*s).block_start = (*s).strstart as ::core::ffi::c_long;
                flush_pending((*s).strm);
            }
            (*s).strstart = (*s).strstart.wrapping_add(1);
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
        } else {
            (*s).match_available = 1 as ::core::ffi::c_int;
            (*s).strstart = (*s).strstart.wrapping_add(1);
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
        }
    }
    if (*s).match_available != 0 {
        let state = &mut *s;
        let Ok(window_len) = usize::try_from(state.window_size) else {
            return need_more;
        };
        let Ok(symbol_len) = usize::try_from(state.sym_end) else {
            return need_more;
        };
        if (window_len != 0 && state.window.is_null())
            || (symbol_len != 0 && state.sym_buf.is_null())
        {
            return need_more;
        }
        let window = if window_len == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts(state.window, window_len)
        };
        let symbols = if symbol_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut(state.sym_buf, symbol_len)
        };
        let Some(flush_now) = tally_previous_literal_state(state, window, symbols) else {
            return need_more;
        };
        bflush = flush_now as ::core::ffi::c_int;
        (*s).match_available = 0 as ::core::ffi::c_int;
    }
    (*s).insert = if (*s).strstart
        < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        (*s).strstart
    } else {
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    };
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            block_flush_len_state((*s).strstart, (*s).block_start),
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).sym_next != 0 {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            block_flush_len_state((*s).strstart, (*s).block_start),
            0 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

fn rle_match_length_state(
    window: &[crate::stdlib::Byte],
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    let Ok(start) = usize::try_from(strstart) else {
        return 0;
    };
    let Ok(lookahead) = usize::try_from(lookahead) else {
        return 0;
    };
    let max_match = crate::zutil_h::MAX_MATCH as usize;
    let min_match = crate::zutil_h::MIN_MATCH as usize;
    if start == 0 || lookahead < min_match {
        return 0;
    }
    let run_limit = lookahead.min(max_match);
    let Some(end) = start.checked_add(run_limit) else {
        return 0;
    };
    if end > window.len() {
        return 0;
    }
    let previous = window[start - 1];
    let run = window[start..end]
        .iter()
        .take_while(|&&byte| byte == previous)
        .count();
    if run < min_match {
        0
    } else {
        crate::stdlib::uInt::try_from(run).unwrap_or(0)
    }
}

fn tally_symbol_state(
    s: &mut crate::src::deflate::deflate_state,
    symbols: &mut [crate::zutil_h::uch],
    dist: ::core::ffi::c_uint,
    lc: ::core::ffi::c_uint,
) -> Option<bool> {
    let start = usize::try_from(s.sym_next).ok()?;
    let end = start.checked_add(3)?;
    if end > symbols.len() || end > usize::try_from(s.sym_end).ok()? {
        return None;
    }
    if dist == 0 {
        if usize::try_from(lc).ok()? >= s.dyn_ltree.len() {
            return None;
        }
    } else {
        let length_code = *crate::src::trees::_length_code.get(usize::try_from(lc).ok()?)?;
        let literal_code =
            usize::from(length_code).checked_add(crate::src::deflate::LITERALS as usize + 1)?;
        if literal_code >= s.dyn_ltree.len() {
            return None;
        }
        let distance = dist.checked_sub(1)?;
        let distance_index = if distance < 256 {
            usize::try_from(distance).ok()?
        } else {
            256usize.checked_add(usize::try_from(distance >> 7).ok()?)?
        };
        let distance_code = *crate::src::trees::_dist_code.get(distance_index)?;
        if usize::from(distance_code) >= s.dyn_dtree.len() {
            return None;
        }
    }

    let _ = crate::src::trees::_tr_tally(
        symbols,
        &mut s.sym_next,
        s.sym_end,
        &mut s.matches,
        &mut s.dyn_ltree,
        &mut s.dyn_dtree,
        dist,
        lc,
    );
    Some(s.sym_next == s.sym_end)
}

fn tally_current_literal_state(
    s: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Byte],
    symbols: &mut [crate::zutil_h::uch],
) -> Option<bool> {
    let literal = *window.get(usize::try_from(s.strstart).ok()?)?;
    let flush_now = tally_symbol_state(s, symbols, 0, literal.into())?;
    s.lookahead = s.lookahead.wrapping_sub(1);
    s.strstart = s.strstart.wrapping_add(1);
    Some(flush_now)
}

/// Tally the literal deferred by the lazy parser.  The caller retains the
/// callback-owned buffer lends; this helper keeps the `strstart - 1` lookup
/// and symbol update checked and in their original order.
fn tally_previous_literal_state(
    s: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Byte],
    symbols: &mut [crate::zutil_h::uch],
) -> Option<bool> {
    let index = usize::try_from(s.strstart.checked_sub(1)?).ok()?;
    let literal = (*window.get(index)?).into();
    tally_symbol_state(s, symbols, 0, literal)
}

/// Record the current fast-mode match and apply the corresponding input
/// consumption.  Hash insertion remains in the legacy adapter, since its
/// tables are still callback-owned.
fn fast_tally_match_state(
    s: &mut crate::src::deflate::deflate_state,
    symbols: &mut [crate::zutil_h::uch],
) -> Option<(bool, crate::stdlib::uInt)> {
    let length = s.match_length;
    if length < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        return None;
    }
    let len = length.wrapping_sub(3) as crate::zutil_h::uch;
    let dist = s.strstart.wrapping_sub(s.match_start);
    let flush_now = tally_symbol_state(s, symbols, dist, len.into())?;
    s.lookahead = s.lookahead.wrapping_sub(length);
    Some((flush_now, length))
}

/// Select the current literal for Huffman-only mode without borrowing the
/// callback-owned symbol buffer.  The legacy adapter lends that buffer only
/// after this window read has completed.
fn huff_literal_plan_state(
    s: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Byte],
) -> Option<crate::stdlib::uInt> {
    Some((*window.get(usize::try_from(s.strstart).ok()?)?).into())
}

/// Commit a previously selected Huffman-only literal.  Match state changes
/// remain after the bounded tally, matching the original cursor ordering.
fn huff_tally_literal_state(
    s: &mut crate::src::deflate::deflate_state,
    symbols: &mut [crate::zutil_h::uch],
    literal: crate::stdlib::uInt,
) -> Option<bool> {
    let flush_now = tally_symbol_state(s, symbols, 0, literal)?;
    s.match_length = 0;
    Some(flush_now)
}

/// Emit the distance-one match selected by the RLE parser and commit its
/// cursor transitions only after the symbol tally succeeds.
fn rle_tally_match_state(
    s: &mut crate::src::deflate::deflate_state,
    symbols: &mut [crate::zutil_h::uch],
) -> Option<bool> {
    let length = s.match_length;
    if length < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        return None;
    }
    let len = length.wrapping_sub(3) as crate::zutil_h::uch;
    let flush_now = tally_symbol_state(s, symbols, 1, len.into())?;
    s.lookahead = s.lookahead.wrapping_sub(length);
    s.strstart = s.strstart.wrapping_add(length);
    s.match_length = 0;
    Some(flush_now)
}

/// The RLE mode's parser decision is independent of callback-owned symbol
/// storage.  Keep the window inspection and the subsequent symbol write as
/// two separate lends in the legacy adapter.
enum RleSymbol {
    Match,
    Literal(crate::stdlib::uInt),
}

fn rle_symbol_plan_state(
    s: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Byte],
) -> Option<RleSymbol> {
    s.match_length = 0;
    if s.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt && s.strstart != 0 {
        s.match_length = rle_match_length_state(window, s.strstart, s.lookahead);
    }
    if s.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        Some(RleSymbol::Match)
    } else {
        Some(RleSymbol::Literal(
            (*window.get(usize::try_from(s.strstart).ok()?)?).into(),
        ))
    }
}

fn rle_tally_symbol_state(
    s: &mut crate::src::deflate::deflate_state,
    symbols: &mut [crate::zutil_h::uch],
    symbol: RleSymbol,
) -> Option<bool> {
    match symbol {
        RleSymbol::Match => rle_tally_match_state(s, symbols),
        RleSymbol::Literal(literal) => tally_symbol_state(s, symbols, 0, literal),
    }
}

fn hash_match_is_usable(
    strstart: crate::src::deflate::IPos,
    hash_head: crate::src::deflate::IPos,
    w_size: crate::stdlib::uInt,
) -> bool {
    hash_head != NIL as crate::src::deflate::IPos
        && strstart.wrapping_sub(hash_head)
            <= w_size.wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
}

fn lazy_hash_match_is_usable(
    strstart: crate::src::deflate::IPos,
    hash_head: crate::src::deflate::IPos,
    prev_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
) -> bool {
    prev_length < max_lazy_match && hash_match_is_usable(strstart, hash_head, w_size)
}

fn filtered_match_length(
    match_length: crate::stdlib::uInt,
    strategy: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    if match_length <= 5
        && (strategy == crate::zlib_h::Z_FILTERED
            || match_length == crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                && strstart.wrapping_sub(match_start) > TOO_FAR as crate::stdlib::uInt)
    {
        (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt
    } else {
        match_length
    }
}

unsafe extern "C" fn deflate_rle(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_window(s);
            if (*s).lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if (*s).lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        let symbol = {
            let state = &mut *s;
            let Ok(window_len) = usize::try_from(state.window_size) else {
                return need_more;
            };
            if window_len != 0 && state.window.is_null() {
                return need_more;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(state.window, window_len)
            };
            let Some(symbol) = rle_symbol_plan_state(state, window) else {
                return need_more;
            };
            symbol
        };
        {
            let state = &mut *s;
            let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                return need_more;
            };
            if symbol_len != 0 && state.sym_buf.is_null() {
                return need_more;
            }
            let symbols = if symbol_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.sym_buf, symbol_len)
            };
            let Some(flush_now) = rle_tally_symbol_state(state, symbols, symbol) else {
                return need_more;
            };
            bflush = flush_now as ::core::ffi::c_int;
        }
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
                s as *mut crate::src::deflate::internal_state,
                if (*s).block_start >= 0 as ::core::ffi::c_long {
                    (*s).window
                        .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                block_flush_len_state((*s).strstart, (*s).block_start),
                0 as ::core::ffi::c_int,
            );
            (*s).block_start = (*s).strstart as ::core::ffi::c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
                return (if false {
                    finish_started as ::core::ffi::c_int
                } else {
                    need_more as ::core::ffi::c_int
                }) as block_state;
            }
        }
    }
    (*s).insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            block_flush_len_state((*s).strstart, (*s).block_start),
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).sym_next != 0 {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            block_flush_len_state((*s).strstart, (*s).block_start),
            0 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

unsafe extern "C" fn deflate_huff(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead == 0 as crate::stdlib::uInt {
            fill_window(s);
            if (*s).lookahead == 0 as crate::stdlib::uInt {
                if flush == crate::zlib_h::Z_NO_FLUSH {
                    return need_more;
                }
                break;
            }
        }
        let state = &mut *s;
        let literal = {
            let Ok(window_len) = usize::try_from(state.window_size) else {
                return need_more;
            };
            if window_len != 0 && state.window.is_null() {
                return need_more;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(state.window, window_len)
            };
            let Some(literal) = huff_literal_plan_state(state, window) else {
                return need_more;
            };
            literal
        };
        {
            let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                return need_more;
            };
            if symbol_len != 0 && state.sym_buf.is_null() {
                return need_more;
            }
            let symbols = if symbol_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(state.sym_buf, symbol_len)
            };
            let Some(flush_now) = huff_tally_literal_state(state, symbols, literal) else {
                return need_more;
            };
            bflush = flush_now as ::core::ffi::c_int;
        }
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
                s as *mut crate::src::deflate::internal_state,
                if (*s).block_start >= 0 as ::core::ffi::c_long {
                    (*s).window
                        .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                block_flush_len_state((*s).strstart, (*s).block_start),
                0 as ::core::ffi::c_int,
            );
            (*s).block_start = (*s).strstart as ::core::ffi::c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
                return (if false {
                    finish_started as ::core::ffi::c_int
                } else {
                    need_more as ::core::ffi::c_int
                }) as block_state;
            }
        }
    }
    (*s).insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            block_flush_len_state((*s).strstart, (*s).block_start),
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).sym_next != 0 {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            block_flush_len_state((*s).strstart, (*s).block_start),
            0 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}
