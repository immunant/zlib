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
    /// Address token for the ABI stream that owns this opaque state.  The
    /// state is opaque across the ABI, so it needs only identity for its
    /// consistency checks; ordinary codec code never dereferences this.
    pub strm: usize,
    pub status: ::core::ffi::c_int,
    // `data_type` is observable through the ABI stream, but it is inferred by
    // the tree core.  Keep the working value with the opaque codec state so
    // that core tree work does not need to dereference `strm`.
    pub data_type: ::core::ffi::c_int,
    /// The callback-owned pending allocation is absent until init succeeds.
    /// This opaque state can retain the nullable C handle as an optional
    /// non-null pointer rather than as a raw pointer in codec state.
    pub pending_buf: Option<::core::ptr::NonNull<crate::stdlib::Bytef>>,
    pub pending_buf_size: crate::zutil_h::ulg,
    /// Offset of the next pending byte within `pending_buf`.  The deflate
    /// state is opaque across the ABI, so an offset preserves its observable
    /// behavior without retaining an interior raw pointer.
    pub pending_out: usize,
    pub pending: crate::zutil_h::ulg,
    pub wrap: ::core::ffi::c_int,
    /// The caller-retained gzip header is optional.  The state is opaque at
    /// the ABI, so retaining a non-null handle expresses that invariant
    /// without keeping a nullable raw pointer in ordinary codec state.
    pub gzhead: Option<::core::ptr::NonNull<crate::zlib_h::gz_header>>,
    pub gzindex: crate::zutil_h::ulg,
    pub method: crate::stdlib::Byte,
    pub last_flush: ::core::ffi::c_int,
    pub w_size: crate::stdlib::uInt,
    pub w_bits: crate::stdlib::uInt,
    pub w_mask: crate::stdlib::uInt,
    /// The callback-owned history allocation is absent until init succeeds.
    /// This opaque state never exposes its layout through the ABI, so model
    /// that nullable C handle as an optional non-null pointer internally.
    pub window: Option<::core::ptr::NonNull<crate::stdlib::Bytef>>,
    pub window_size: crate::zutil_h::ulg,
    /// The callback-owned previous-position table is absent until init
    /// succeeds.  As with the history window, this opaque state can retain a
    /// non-null handle instead of a nullable raw pointer.
    pub prev: Option<::core::ptr::NonNull<crate::src::deflate::Posf>>,
    /// The callback-owned hash-head table is absent until init succeeds.
    /// Since this state is opaque through the ABI, retain it as an optional
    /// non-null handle rather than as a nullable raw pointer.
    pub head: Option<::core::ptr::NonNull<crate::src::deflate::Posf>>,
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
    /// Offset of the literal/match symbols within `pending_buf`.  This was
    /// previously retained as an interior raw pointer derived from the
    /// pending allocation; an offset keeps that relationship checked and
    /// avoids storing another raw pointer in opaque codec state.
    pub sym_start: usize,
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

/// Construct the exact zeroed state that `deflateInit2_()` historically
/// obtained from `memset`.  The callback-allocated storage is still adopted
/// at the codec boundary; spelling the initial value out here keeps Rust
/// state initialization independent of libc byte operations.
fn deflate_initial_state() -> deflate_state {
    let empty_ct_data = ct_data_s {
        fc: C2Rust_Unnamed_1 { freq: 0, code: 0 },
        dl: C2Rust_Unnamed_0 { dad: 0, len: 0 },
    };
    let empty_tree_desc = tree_desc_s {
        max_code: 0,
        stat_desc: None,
    };
    internal_state {
        strm: 0,
        status: 0,
        data_type: crate::zlib_h::Z_UNKNOWN,
        pending_buf: None,
        pending_buf_size: 0,
        pending_out: 0,
        pending: 0,
        wrap: 0,
        gzhead: None,
        gzindex: 0,
        method: 0,
        last_flush: 0,
        w_size: 0,
        w_bits: 0,
        w_mask: 0,
        window: None,
        window_size: 0,
        prev: None,
        head: None,
        ins_h: 0,
        hash_size: 0,
        hash_bits: 0,
        hash_mask: 0,
        hash_shift: 0,
        block_start: 0,
        match_length: 0,
        prev_match: 0,
        match_available: 0,
        strstart: 0,
        match_start: 0,
        lookahead: 0,
        prev_length: 0,
        max_chain_length: 0,
        max_lazy_match: 0,
        level: 0,
        strategy: 0,
        good_match: 0,
        nice_match: 0,
        dyn_ltree: [empty_ct_data; 573],
        dyn_dtree: [empty_ct_data; 61],
        bl_tree: [empty_ct_data; 39],
        l_desc: empty_tree_desc,
        d_desc: empty_tree_desc,
        bl_desc: empty_tree_desc,
        bl_count: [0; 16],
        heap: [0; 573],
        heap_len: 0,
        heap_max: 0,
        depth: [0; 573],
        sym_start: 0,
        lit_bufsize: 0,
        sym_next: 0,
        sym_end: 0,
        opt_len: 0,
        static_len: 0,
        matches: 0,
        insert: 0,
        bi_buf: 0,
        bi_valid: 0,
        bi_used: 0,
        high_water: 0,
        slid: 0,
    }
}

pub const MIN_LOOKAHEAD: ::core::ffi::c_int =
    crate::zutil_h::MAX_MATCH + crate::zutil_h::MIN_MATCH + 1 as ::core::ffi::c_int;

pub const WIN_INIT: ::core::ffi::c_int = crate::zutil_h::MAX_MATCH;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::trees::_dist_code;
pub use crate::src::trees::_length_code;
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

/// The checked, pointer-free admission for one `read_buf()` refill.  The
/// compatibility adapter still has to lend the caller's input cursor, but it
/// should do so only after the amount to copy has been reconciled with the
/// destination span it was given by the window code.
#[derive(Copy, Clone)]
struct ReadBufPlan {
    len: usize,
}

fn read_buf_plan(
    avail_in: crate::stdlib::uInt,
    requested: ::core::ffi::c_uint,
    output_len: usize,
) -> Option<ReadBufPlan> {
    let len = avail_in.min(requested);
    let len = usize::try_from(len).ok()?;
    (len <= output_len).then_some(ReadBufPlan { len })
}

/// Scalar progress returned by the raw stream adapter. Keeping the updated
/// input availability with the copied length lets callers make their next
/// refill decision without re-dereferencing the compatibility stream.
#[derive(Copy, Clone, Default)]
struct ReadBufProgress {
    copied: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
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

fn read_buf(
    strm: &mut crate::zlib_h::z_stream,
    input: &mut &[crate::stdlib::Byte],
    buf: &mut [crate::stdlib::Bytef],
    mut size: ::core::ffi::c_uint,
    wrap: ::core::ffi::c_int,
) -> ReadBufProgress {
    // The caller owns both the bounded input view and exact writable
    // destination span. Keep those lends in the types instead of rebuilding a
    // raw input slice for every refill.
    let Some(plan) = read_buf_plan(strm.avail_in, size, buf.len()) else {
        return ReadBufProgress {
            copied: 0,
            avail_in: strm.avail_in,
        };
    };
    if plan.len == 0 {
        return ReadBufProgress {
            copied: 0,
            avail_in: strm.avail_in,
        };
    }
    if input.len() < plan.len {
        return ReadBufProgress {
            copied: 0,
            avail_in: strm.avail_in,
        };
    }
    let Some(output) = buf.get_mut(..plan.len) else {
        return ReadBufProgress {
            copied: 0,
            avail_in: strm.avail_in,
        };
    };
    let (len, adler, avail_in, total_in) = read_buf_progress_state(
        &input[..plan.len],
        output,
        wrap,
        strm.adler,
        strm.avail_in,
        strm.total_in,
    );
    let Some(input_tail) = input.get(len as usize..) else {
        return ReadBufProgress {
            copied: 0,
            avail_in: strm.avail_in,
        };
    };
    strm.avail_in = avail_in;
    strm.adler = adler;
    *input = input_tail;
    strm.next_in = input.as_ptr() as *mut crate::stdlib::Bytef;
    strm.total_in = total_in;
    ReadBufProgress {
        copied: len,
        avail_in,
    }
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

/// Select the exact writable window tail used for one input refill.  Keeping
/// this arithmetic slice-based ensures the transitional `read_buf` adapter
/// never receives a cursor formed past the callback-owned window allocation.
fn fill_window_write_span(
    window_len: usize,
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    requested: ::core::ffi::c_uint,
) -> Option<core::ops::Range<usize>> {
    let start = usize::try_from(strstart)
        .ok()?
        .checked_add(usize::try_from(lookahead).ok()?)?;
    let end = start.checked_add(usize::try_from(requested).ok()?)?;
    (end <= window_len).then_some(start..end)
}

/// The callback allocator owns these three deflate work buffers.  Keep their
/// lends together at the compatibility boundary so the refill state machine
/// below only manipulates checked Rust slices.
pub struct DeflateWindowHashBuffers<'a> {
    window: &'a mut [crate::stdlib::Byte],
    head: &'a mut [crate::src::deflate::Posf],
    prev: &'a mut [crate::src::deflate::Posf],
}

impl<'a> DeflateWindowHashBuffers<'a> {
    pub fn new(
        window: &'a mut [crate::stdlib::Byte],
        head: &'a mut [crate::src::deflate::Posf],
        prev: &'a mut [crate::src::deflate::Posf],
    ) -> Self {
        Self { window, head, prev }
    }
}

// The window and hash tables are callback-owned allocations.  Construct these
// exact lends only at ABI boundaries; the ordinary compressor merely threads
// the bundle through refill transitions.
macro_rules! deflate_window_hash_buffers_at_boundary {
    ($state:expr $(,)?) => {{
        let state = $state;
        match (
            usize::try_from(state.window_size),
            usize::try_from(state.hash_size),
            usize::try_from(state.w_size),
        ) {
            (Ok(window_len), Ok(head_len), Ok(prev_len))
                if window_len == 0 || state.window.is_some() =>
            {
                // Missing hash tables remain empty lends so
                // `fill_window_state()` rejects them at the legacy
                // slide/insertion transition.
                let window = if window_len == 0 {
                    &mut []
                } else {
                    let window = state.window.expect("validated deflate window");
                    ::core::slice::from_raw_parts_mut(window.as_ptr(), window_len)
                };
                let head = if state.head.is_none() {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(
                        state.head.expect("validated deflate hash table").as_ptr(),
                        head_len,
                    )
                };
                let prev = if state.prev.is_none() {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(
                        state
                            .prev
                            .expect("validated deflate previous table")
                            .as_ptr(),
                        prev_len,
                    )
                };
                Some(crate::src::deflate::DeflateWindowHashBuffers::new(
                    window, head, prev,
                ))
            }
            _ => None,
        }
    }};
}
pub(crate) use deflate_window_hash_buffers_at_boundary;

fn fill_window_hash_lends_match_state(
    state: &crate::src::deflate::deflate_state,
    buffers: &DeflateWindowHashBuffers<'_>,
) -> bool {
    let Ok(head_len) = usize::try_from(state.hash_size) else {
        return false;
    };
    let Ok(prev_len) = usize::try_from(state.w_size) else {
        return false;
    };
    buffers.head.len() == head_len && buffers.prev.len() == prev_len
}

/// The pointer-free refill state machine.  The caller has already lent the
/// callback-owned allocations; malformed or absent hash lends are rejected
/// only at the same slide/insertion transitions where legacy code used them.
fn fill_window_state(
    s: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    input: &mut &[crate::stdlib::Byte],
    buffers: &mut DeflateWindowHashBuffers<'_>,
) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    let state = s;
    let wsize: crate::stdlib::uInt = state.w_size;
    loop {
        let (space, should_slide) =
            fill_window_space_state(state.window_size, state.lookahead, state.strstart, wsize);
        more = space;
        let Ok(window_len) = usize::try_from(state.window_size) else {
            return;
        };
        if buffers.window.len() != window_len {
            return;
        }
        let hash_lends_match = fill_window_hash_lends_match_state(state, buffers);
        let (window, head, prev) = (&mut *buffers.window, &mut *buffers.head, &mut *buffers.prev);
        if should_slide {
            let Some(next_more) = slide_window_state(
                window,
                wsize,
                more,
                &mut state.match_start,
                &mut state.strstart,
                &mut state.block_start,
                &mut state.insert,
            ) else {
                return;
            };
            // Preserve the legacy early malformed-state exit before the
            // refill. The actual hash update can wait until after the refill:
            // no code between these points consults the hash chains.
            if !hash_lends_match {
                return;
            }
            more = next_more;
        }
        // `read_buf()` consumes at most this exact available-input snapshot.
        let Some(write_span) =
            fill_window_write_span(window.len(), state.strstart, state.lookahead, more)
        else {
            return;
        };
        let Some(output) = window.get_mut(write_span) else {
            return;
        };
        let progress = read_buf(strm, input, output, more, state.wrap);
        n = progress.copied;
        state.lookahead = state.lookahead.wrapping_add(n);
        let insert_pending = state.lookahead.wrapping_add(state.insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt;
        if should_slide || insert_pending {
            if !hash_lends_match {
                return;
            }
            if should_slide {
                slide_hash_state(head, prev, wsize);
                state.slid = 1;
            }
            if insert_pending
                && !insert_pending_strings_state(
                    window,
                    head,
                    prev,
                    state.strstart,
                    state.lookahead,
                    &mut state.insert,
                    &mut state.ins_h,
                    state.hash_shift,
                    state.hash_mask,
                    state.w_mask,
                )
            {
                return;
            }
        }
        if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
            && progress.avail_in != 0 as crate::stdlib::uInt
        {
            continue;
        }
        if state.high_water < state.window_size
            && !clear_window_tail_state(
                window,
                state.window_size,
                state.strstart,
                state.lookahead,
                &mut state.high_water,
            )
        {
            return;
        }
        break;
    }
}

#[export_name = "deflateInit_"]

pub unsafe extern "C" fn deflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if version.is_null() || !deflate_init_version_matches(*version, stream_size) {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflateInit2_(
        &mut *strm,
        level,
        crate::zlib_h::Z_DEFLATED,
        crate::stdlib::MAX_WBITS,
        crate::zutil_h::DEF_MEM_LEVEL,
        crate::zlib_h::Z_DEFAULT_STRATEGY,
    )
}

/// Validate the scalar portion of zlib's init ABI at the export boundary.
/// Internal callers use the implementation directly and therefore never need
/// to manufacture a C version pointer or stream-size argument.
fn deflate_init_version_matches(
    version_first: ::core::ffi::c_char,
    stream_size: ::core::ffi::c_int,
) -> bool {
    version_first == crate::zlib_h::ZLIB_VERSION[0]
        && usize::try_from(stream_size)
            .ok()
            .is_some_and(|size| size == ::core::mem::size_of::<crate::zlib_h::z_stream>())
}

/// The checked scalar choices needed to construct a deflate state.  Keeping
/// these separate from callback allocation lets the initialization state
/// machine remain ordinary Rust once the ABI boundary has obtained storage.
struct DeflateInitSettings {
    level: ::core::ffi::c_int,
    method: crate::stdlib::Byte,
    wrap: ::core::ffi::c_int,
    window_bits: crate::stdlib::uInt,
    mem_level: crate::stdlib::uInt,
    strategy: ::core::ffi::c_int,
}

fn deflate_init_settings(
    mut level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    mut window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Option<DeflateInitSettings> {
    if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        level = 6;
    }
    let wrap = if window_bits < 0 {
        if window_bits < -15 {
            return None;
        }
        window_bits = -window_bits;
        0
    } else if window_bits > 15 {
        window_bits -= 16;
        2
    } else {
        1
    };
    if mem_level < 1
        || mem_level > crate::stdlib::MAX_MEM_LEVEL
        || method != crate::zlib_h::Z_DEFLATED
        || !(8..=15).contains(&window_bits)
        || !(0..=9).contains(&level)
        || !(0..=crate::zlib_h::Z_FIXED).contains(&strategy)
        || (window_bits == 8 && wrap != 1)
    {
        return None;
    }
    if window_bits == 8 {
        window_bits = 9;
    }
    Some(DeflateInitSettings {
        level,
        method: method as crate::stdlib::Byte,
        wrap,
        window_bits: window_bits as crate::stdlib::uInt,
        mem_level: mem_level as crate::stdlib::uInt,
        strategy,
    })
}

/// Sizes for the four callback-owned allocations.  They are derived while a
/// state is safely borrowed, then used after that borrow ends for each foreign
/// allocator callback.
struct DeflateAllocationSizes {
    window: crate::stdlib::uInt,
    hash: crate::stdlib::uInt,
    pending: crate::stdlib::uInt,
}

fn deflate_prepare_initial_state(
    state: &mut deflate_state,
    stream_identity: usize,
    settings: &DeflateInitSettings,
) -> DeflateAllocationSizes {
    *state = deflate_initial_state();
    state.strm = stream_identity;
    state.status = crate::src::deflate::INIT_STATE;
    state.wrap = settings.wrap;
    state.w_bits = settings.window_bits;
    state.w_size = (1 as crate::stdlib::uInt) << state.w_bits;
    state.w_mask = state.w_size.wrapping_sub(1);
    state.hash_bits = settings.mem_level.wrapping_add(7);
    state.hash_size = (1 as crate::stdlib::uInt) << state.hash_bits;
    state.hash_mask = state.hash_size.wrapping_sub(1);
    state.hash_shift = state
        .hash_bits
        .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
        .wrapping_sub(1)
        .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
    state.high_water = 0;
    state.lit_bufsize = (1 as crate::stdlib::uInt) << (settings.mem_level + 6);
    state.pending_buf_size = (state.lit_bufsize as crate::zutil_h::ulg).wrapping_mul(4);
    DeflateAllocationSizes {
        window: state.w_size,
        hash: state.hash_size,
        pending: state.lit_bufsize,
    }
}

fn deflate_complete_initial_state(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut deflate_state,
    settings: &DeflateInitSettings,
    head: &mut [crate::src::deflate::Posf],
) -> ::core::ffi::c_int {
    state.sym_start = state.lit_bufsize as usize;
    state.sym_end = state.lit_bufsize.wrapping_sub(1).wrapping_mul(3);
    state.level = settings.level;
    state.strategy = settings.strategy;
    state.method = settings.method;
    let ret = deflate_reset_keep_state(strm, state);
    if ret == crate::zlib_h::Z_OK {
        let _ = lm_init_state(state, head);
    }
    ret
}

fn deflate_mark_initialization_memory_error(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut deflate_state,
) {
    state.status = crate::src::deflate::FINISH_STATE;
    strm.msg =
        crate::src::zutil::z_errmsg[6] as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
}

pub fn deflateInit2_(
    strm_ref: &mut crate::zlib_h::z_stream,
    level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    windowBits: ::core::ffi::c_int,
    memLevel: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // Allocation callbacks and the C-compatible stream/state records remain
    // an ABI boundary.  Keep their adoption confined here so callers use the
    // normal safe implementation interface.
    unsafe {
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
        let Some(settings) = deflate_init_settings(level, method, windowBits, memLevel, strategy)
        else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Some(zalloc) = strm_ref.zalloc else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let s = zalloc(
            strm_ref.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::deflate_state;
        if s.is_null() {
            return crate::zlib_h::Z_MEM_ERROR;
        }
        let allocation_sizes = {
            let state = &mut *s;
            strm_ref.state = s as *mut crate::src::deflate::internal_state;
            deflate_prepare_initial_state(
                state,
                strm_ref as *mut crate::zlib_h::z_stream as usize,
                &settings,
            )
        };
        // Do not retain the state borrow across a custom allocator callback.
        // Publish each allocation immediately, matching zlib's observable
        // partial-initialization state for allocator hooks.
        let Some(zalloc) = strm_ref.zalloc else {
            let _ = deflateEnd(strm_ref);
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let window = zalloc(
            strm_ref.opaque,
            allocation_sizes.window,
            (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
                as crate::stdlib::uInt,
        ) as *mut crate::stdlib::Bytef;
        (&mut *s).window = ::core::ptr::NonNull::new(window);
        let Some(zalloc) = strm_ref.zalloc else {
            let _ = deflateEnd(strm_ref);
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let prev = zalloc(
            strm_ref.opaque,
            allocation_sizes.window,
            ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::Posf;
        (&mut *s).prev = ::core::ptr::NonNull::new(prev);
        let Some(zalloc) = strm_ref.zalloc else {
            let _ = deflateEnd(strm_ref);
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let head = zalloc(
            strm_ref.opaque,
            allocation_sizes.hash,
            ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::Posf;
        (&mut *s).head = ::core::ptr::NonNull::new(head);
        let Some(zalloc) = strm_ref.zalloc else {
            let _ = deflateEnd(strm_ref);
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let pending_buf = zalloc(
            strm_ref.opaque,
            allocation_sizes.pending,
            4 as crate::stdlib::uInt,
        ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef;
        (&mut *s).pending_buf = ::core::ptr::NonNull::new(pending_buf);
        if window.is_null() || prev.is_null() || head.is_null() || pending_buf.is_null() {
            deflate_mark_initialization_memory_error(strm_ref, &mut *s);
            deflateEnd(strm_ref);
            return crate::zlib_h::Z_MEM_ERROR;
        }
        let state = &mut *s;
        let Ok(head_len) = usize::try_from(state.hash_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let head = ::core::slice::from_raw_parts_mut(
            state.head.expect("allocated deflate hash table").as_ptr(),
            head_len,
        );
        deflate_complete_initial_state(strm_ref, state, &settings, head)
    }
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
    if version.is_null() || !deflate_init_version_matches(*version, stream_size) {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflateInit2_(&mut *strm, level, method, windowBits, memLevel, strategy)
}
pub(crate) fn deflate_state_values_are_valid(
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

/// Represent a compatibility cursor relative to its allocation base without
/// dereferencing either address.  The ABI boundary converts raw cursors to
/// address tokens; the copy logic only needs the direction and byte count.
fn deflate_cursor_direction(cursor: usize, base: usize) -> (bool, usize) {
    if cursor >= base {
        (true, cursor - base)
    } else {
        (false, base - cursor)
    }
}

/// Validate a compatibility cursor and byte count against one owned buffer.
/// The ABI boundary converts pointers to address tokens; all later copying is
/// ordinary slice work.
fn deflate_copy_range(
    buffer_len: usize,
    buffer_start: usize,
    cursor: usize,
    count: usize,
) -> Option<::core::ops::Range<usize>> {
    let start = cursor.checked_sub(buffer_start)?;
    let end = start.checked_add(count)?;
    (end <= buffer_len).then_some(start..end)
}

/// Copy the independently allocated portions of a cloned deflate state.
/// The caller has already validated the allocation-backed slice lends and
/// cursor ranges, so this core has no ABI pointers or allocation concerns.
fn deflate_copy_owned_buffers(
    dst_window: &mut [crate::stdlib::Byte],
    src_window: &[crate::stdlib::Byte],
    window_len: usize,
    dst_prev: &mut [crate::src::deflate::Pos],
    src_prev: &[crate::src::deflate::Pos],
    prev_len: usize,
    dst_head: &mut [crate::src::deflate::Pos],
    src_head: &[crate::src::deflate::Pos],
    head_len: usize,
    dst_pending: &mut [crate::stdlib::Byte],
    src_pending: &[crate::stdlib::Byte],
    pending: ::core::ops::Range<usize>,
    dst_sym: ::core::ops::Range<usize>,
    src_sym: ::core::ops::Range<usize>,
) -> bool {
    let Some((dst_window, src_window)) = dst_window
        .get_mut(..window_len)
        .zip(src_window.get(..window_len))
    else {
        return false;
    };
    let Some((dst_prev, src_prev)) = dst_prev.get_mut(..prev_len).zip(src_prev.get(..prev_len))
    else {
        return false;
    };
    let Some((dst_head, src_head)) = dst_head.get_mut(..head_len).zip(src_head.get(..head_len))
    else {
        return false;
    };
    dst_window.copy_from_slice(src_window);
    dst_prev.copy_from_slice(src_prev);
    dst_head.copy_from_slice(src_head);
    {
        let Some((dst_pending_range, src_pending_range)) = dst_pending
            .get_mut(pending.clone())
            .zip(src_pending.get(pending))
        else {
            return false;
        };
        dst_pending_range.copy_from_slice(src_pending_range);
    }
    {
        let Some((dst_sym, src_sym)) = dst_pending.get_mut(dst_sym).zip(src_pending.get(src_sym))
        else {
            return false;
        };
        dst_sym.copy_from_slice(src_sym);
    }
    true
}

// This expands only at ABI boundaries. The stream and state are callback-owned
// compatibility records, so their raw validation belongs at the boundary; the
// validity decision itself remains a pointer-free core.
macro_rules! deflate_state_check_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if strm.is_null() {
            1
        } else {
            let strm_ref = &*strm;
            let state = strm_ref.state as *mut crate::src::deflate::deflate_state;
            if state.is_null() {
                1
            } else {
                let state = &*state;
                (!crate::src::deflate::deflate_state_values_are_valid(
                    strm_ref.zalloc.is_some(),
                    strm_ref.zfree.is_some(),
                    state.strm == strm as usize,
                    state.status,
                )) as ::core::ffi::c_int
            }
        }
    }};
}
pub(crate) use deflate_state_check_at_boundary;
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
    if deflate_state_check_at_boundary!(strm) != 0 || dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // This is the callback-owned ABI boundary. Adopt the validated stream
    // and state once for the setup phase; the refill bridge below still owns
    // its raw cursor conversion and therefore runs after this borrow ends.
    {
        let strm_ref = &mut *strm;
        s = strm_ref.state as *mut crate::src::deflate::deflate_state;
        let state_ref = &mut *s;
        wrap = state_ref.wrap;
        if wrap == 2 as ::core::ffi::c_int
            || wrap == 1 as ::core::ffi::c_int
                && state_ref.status != crate::src::deflate::INIT_STATE
            || state_ref.lookahead != 0
        {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if wrap == 1 as ::core::ffi::c_int {
            strm_ref.adler = crate::src::adler32::adler32_z(
                strm_ref.adler,
                core::slice::from_raw_parts(dictionary, dictLength as usize),
            );
        }
        state_ref.wrap = 0 as ::core::ffi::c_int;
        if dictLength >= state_ref.w_size {
            if wrap == 0 as ::core::ffi::c_int {
                let Ok(head_len) = usize::try_from(state_ref.hash_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if head_len != 0 && state_ref.head.is_none() {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                let head = if head_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(
                        state_ref
                            .head
                            .expect("validated deflate hash table")
                            .as_ptr(),
                        head_len,
                    )
                };
                clear_hash_state(head, &mut state_ref.slid);
                state_ref.strstart = 0 as crate::stdlib::uInt;
                state_ref.block_start = 0 as ::core::ffi::c_long;
                state_ref.insert = 0 as crate::stdlib::uInt;
            }
            dictionary =
                dictionary.wrapping_add(dictLength.wrapping_sub(state_ref.w_size) as usize);
            dictLength = state_ref.w_size;
        }
        avail = strm_ref.avail_in;
        next = strm_ref.next_in;
        strm_ref.avail_in = dictLength;
        strm_ref.next_in = dictionary as *mut crate::stdlib::Bytef;
    }
    let mut dictionary_input = core::slice::from_raw_parts(dictionary, dictLength as usize);
    let Some(mut window_hash) = deflate_window_hash_buffers_at_boundary!(&mut *s) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    fill_window_state(&mut *s, &mut *strm, &mut dictionary_input, &mut window_hash);
    while {
        let state_ref = &mut *s;
        state_ref.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
    } {
        {
            let state_ref = &mut *s;
            let Ok(window_len) = usize::try_from(state_ref.window_size) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let Ok(head_len) = usize::try_from(state_ref.hash_size) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let Ok(prev_len) = usize::try_from(state_ref.w_size) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            if (window_len != 0 && state_ref.window.is_none())
                || (head_len != 0 && state_ref.head.is_none())
                || (prev_len != 0 && state_ref.prev.is_none())
            {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            let window = if window_len == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(
                    state_ref.window.expect("validated deflate window").as_ptr(),
                    window_len,
                )
            };
            let head = if head_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(
                    state_ref
                        .head
                        .expect("validated deflate hash table")
                        .as_ptr(),
                    head_len,
                )
            };
            let prev = if prev_len == 0 {
                &mut []
            } else {
                ::core::slice::from_raw_parts_mut(
                    state_ref
                        .prev
                        .expect("validated deflate previous table")
                        .as_ptr(),
                    prev_len,
                )
            };
            if !insert_dictionary_strings_state(state_ref, window, head, prev) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
        }
        let Some(mut window_hash) = deflate_window_hash_buffers_at_boundary!(&mut *s) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        fill_window_state(&mut *s, &mut *strm, &mut dictionary_input, &mut window_hash);
    }
    {
        let strm_ref = &mut *strm;
        let state_ref = &mut *s;
        state_ref.strstart = state_ref.strstart.wrapping_add(state_ref.lookahead);
        state_ref.block_start = state_ref.strstart as ::core::ffi::c_long;
        state_ref.insert = state_ref.lookahead;
        state_ref.lookahead = 0 as crate::stdlib::uInt;
        state_ref.prev_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        state_ref.match_length = state_ref.prev_length;
        state_ref.match_available = 0 as ::core::ffi::c_int;
        strm_ref.next_in = next;
        strm_ref.avail_in = avail;
        state_ref.wrap = wrap;
    }
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

/// Check the non-overlap precondition before lending the callback-owned
/// history window and caller dictionary as Rust slices. zlib's original copy
/// has `memcpy` semantics, so an overlapping destination was never supported;
/// reject it rather than creating aliased shared and mutable views here.
fn deflate_spans_are_disjoint(
    left_address: usize,
    left_len: usize,
    right_address: usize,
    right_len: usize,
) -> bool {
    if left_len == 0 || right_len == 0 {
        return true;
    }
    let Some(left_end) = left_address.checked_add(left_len) else {
        return false;
    };
    let Some(right_end) = right_address.checked_add(right_len) else {
        return false;
    };
    left_end <= right_address || right_end <= left_address
}

#[export_name = "deflateGetDictionary"]

pub unsafe extern "C" fn deflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if deflate_state_check_at_boundary!(strm) != 0 {
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
        if state.window.is_none() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let window_address = state.window.expect("validated deflate window").as_ptr() as usize;
        if !deflate_spans_are_disjoint(window_address, window_len, dictionary as usize, len) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let window = ::core::slice::from_raw_parts(
            state.window.expect("validated deflate window").as_ptr(),
            window_len,
        );
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
    state.data_type = crate::zlib_h::Z_UNKNOWN;
    state.pending = 0 as crate::zutil_h::ulg;
    state.pending_out = 0;
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
        if crate::src::deflate::deflate_state_check_at_boundary!(strm) != 0 {
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
        if crate::src::deflate::deflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = (*strm).state as *mut crate::src::deflate::deflate_state;
            let ret = crate::src::deflate::deflate_reset_keep_state(&mut *strm, &mut *state);
            if ret == crate::zlib_h::Z_OK {
                if let Ok(head_len) = usize::try_from((*state).hash_size) {
                    if head_len == 0 {
                        let _ = crate::src::deflate::lm_init_state(&mut *state, &mut []);
                    } else if let Some(head_handle) = (*state).head {
                        let head =
                            ::core::slice::from_raw_parts_mut(head_handle.as_ptr(), head_len);
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
    if deflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The boundary validation above established that both records are live.
    // Keep the retained C header pointer assignment here; the pointer-free
    // policy decision belongs to the safe state helper.
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::deflate::deflate_state);
    if !deflate_allows_gzip_header(state.wrap) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.gzhead = ::core::ptr::NonNull::new(head);
    crate::zlib_h::Z_OK
}

/// A gzip header can only be installed for a gzip-format deflater.  Keeping
/// this decision independent of the ABI header pointer lets internal callers
/// share the format rule without handling an FFI pointer.
fn deflate_allows_gzip_header(wrap: ::core::ffi::c_int) -> bool {
    wrap == 2
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
    if deflate_state_check_at_boundary!(strm) != 0 {
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
    if deflate_state_check_at_boundary!(strm) != 0 {
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
    if deflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The boundary macro above validated both compatibility records. Adopt
    // them once so the bit-buffer state machine below has no repeated raw
    // stream/state dereferences.
    let s = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || s.pending_out
            .checked_add(
                (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                    as usize,
            )
            .map_or(true, |pending_end| pending_end > s.lit_bufsize as usize)
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    loop {
        let Some(step) = prime_bits_step(s.bi_buf, s.bi_valid, bits, value) else {
            return crate::zlib_h::Z_BUF_ERROR;
        };
        s.bi_buf = step.bi_buf;
        s.bi_valid = step.bi_valid;
        let Ok(pending_len) = usize::try_from(s.pending_buf_size) else {
            return crate::zlib_h::Z_BUF_ERROR;
        };
        if pending_len != 0 && s.pending_buf.is_none() {
            return crate::zlib_h::Z_BUF_ERROR;
        }
        let pending_buf = if pending_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut(
                s.pending_buf
                    .expect("validated deflate pending buffer")
                    .as_ptr(),
                pending_len,
            )
        };
        crate::src::trees::flush_bits_state(s, pending_buf);
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

/// Apply the already-admitted parameter change using only owned state and
/// checked hash-table views.  The ABI boundary decides whether those views are
/// needed and lends them before calling this core, so no cursor or allocation
/// pointer survives the optional block flush.
pub(crate) fn deflate_params_commit_state(
    state: &mut crate::src::deflate::deflate_state,
    plan: DeflateParamsPlan,
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
) -> bool {
    if state.level != plan.level {
        if state.level == 0 && state.matches != 0 {
            let Ok(head_len) = usize::try_from(state.hash_size) else {
                return false;
            };
            if head_len > head.len() {
                return false;
            }
            if state.matches == 1 {
                let Ok(prev_len) = usize::try_from(state.w_size) else {
                    return false;
                };
                if prev_len > prev.len() {
                    return false;
                }
                slide_hash_state(&mut head[..head_len], &mut prev[..prev_len], state.w_size);
                state.slid = 1;
            } else {
                clear_hash_state(&mut head[..head_len], &mut state.slid);
            }
            state.matches = 0;
        }
        state.level = plan.level;
        state.max_lazy_match = plan.config.max_lazy as crate::stdlib::uInt;
        state.good_match = plan.config.good_length as crate::stdlib::uInt;
        state.nice_match = plan.config.nice_length as ::core::ffi::c_int;
        state.max_chain_length = plan.config.max_chain as crate::stdlib::uInt;
    }
    state.strategy = plan.strategy;
    true
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
            if crate::src::deflate::deflate_state_check_at_boundary!(strm) != 0 {
                break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
            }
            let s = (*strm).state as *mut crate::src::deflate::deflate_state;
            let Some(plan) = ({
                let state = &*s;
                crate::src::deflate::deflate_params_plan(
                    level,
                    strategy,
                    state.level,
                    state.strategy,
                    state.last_flush,
                )
            }) else {
                break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
            };
            if plan.needs_block_flush {
                let input_len = (*strm).avail_in as usize;
                if input_len != 0 && (*strm).next_in.is_null() {
                    break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                }
                let mut input = if input_len == 0 {
                    &[]
                } else {
                    ::core::slice::from_raw_parts((*strm).next_in, input_len)
                };
                let output_len = (*strm).avail_out as usize;
                if output_len != 0 && (*strm).next_out.is_null() {
                    break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                }
                let output = if output_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut((*strm).next_out, output_len)
                };
                let mut output = crate::src::deflate::DeflateOutput::new(output);
                let Ok(pending_len) = usize::try_from((&*s).pending_buf_size) else {
                    break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_len != 0 && (&*s).pending_buf.is_none() {
                    break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                }
                let pending_buf = if pending_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(
                        (&*s)
                            .pending_buf
                            .expect("validated deflate pending buffer")
                            .as_ptr(),
                        pending_len,
                    )
                };
                let Some(mut window_hash) =
                    crate::src::deflate::deflate_window_hash_buffers_at_boundary!(&mut *s)
                else {
                    break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                };
                let Some(gzip_payloads) =
                    crate::src::deflate::deflate_gzip_payloads_at_boundary!(strm)
                else {
                    break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                };
                let err = crate::src::deflate::deflate(
                    &mut *strm,
                    &mut *s,
                    &mut input,
                    pending_buf,
                    &mut window_hash,
                    &mut output,
                    gzip_payloads,
                    crate::zlib_h::Z_BLOCK,
                );
                if err == crate::zlib_h::Z_STREAM_ERROR {
                    break 'deflate_params_result err;
                }
                let has_unprocessed_data = {
                    let stream = &*strm;
                    let state = &*s;
                    stream.avail_in != 0
                        || state.strstart as ::core::ffi::c_long - state.block_start
                            + state.lookahead as ::core::ffi::c_long
                            != 0
                };
                if has_unprocessed_data {
                    break 'deflate_params_result crate::zlib_h::Z_BUF_ERROR;
                }
            }
            // No callback-capable work remains after the optional block
            // flush above. Lend just the hash buffers this transition needs;
            // the checked core commits every scalar field only after it has
            // accepted the complete hash update.
            let state = &mut *s;
            let needs_hash_update =
                state.level != plan.level && state.level == 0 && state.matches != 0;
            let (head, prev) = if needs_hash_update {
                let Ok(head_len) = usize::try_from(state.hash_size) else {
                    break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                };
                let prev_len = if state.matches == 1 {
                    let Ok(prev_len) = usize::try_from(state.w_size) else {
                        break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                    };
                    prev_len
                } else {
                    0
                };
                if (head_len != 0 && state.head.is_none())
                    || (prev_len != 0 && state.prev.is_none())
                {
                    break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
                }
                let head = if head_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(
                        state.head.expect("validated deflate hash table").as_ptr(),
                        head_len,
                    )
                };
                let prev = if prev_len == 0 {
                    &mut []
                } else {
                    ::core::slice::from_raw_parts_mut(
                        state
                            .prev
                            .expect("validated deflate previous table")
                            .as_ptr(),
                        prev_len,
                    )
                };
                (head, prev)
            } else {
                (
                    &mut [] as &mut [crate::src::deflate::Posf],
                    &mut [] as &mut [crate::src::deflate::Posf],
                )
            };
            if !crate::src::deflate::deflate_params_commit_state(state, plan, head, prev) {
                break 'deflate_params_result crate::zlib_h::Z_STREAM_ERROR;
            }
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
    if deflate_state_check_at_boundary!(strm) != 0 {
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
        if crate::src::deflate::deflate_state_check_at_boundary!(strm) != 0 {
            None
        } else {
            let state = &*((*strm).state as *mut crate::src::deflate::deflate_state);
            let gzip_header = if let Some(header) = state.gzhead {
                let header = &*header.as_ptr();
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
            } else {
                None
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

/// Scalar gzip-header metadata retained for one `deflate()` dispatch. The
/// ABI boundary creates it with the caller-owned payload views, so the
/// compressor does not retain or dereference a header cursor.
#[derive(Copy, Clone)]
pub(crate) struct GzipHeaderFields {
    text: bool,
    hcrc: bool,
    has_extra: bool,
    has_name: bool,
    has_comment: bool,
    time: crate::stdlib::uLong,
    os: ::core::ffi::c_int,
    extra_len: crate::stdlib::uInt,
}

/// The caller-owned gzip header payloads borrowed for one ordinary
/// `deflate()` call.  The ABI boundary creates these views alongside the
/// input and output views, leaving the compressor to copy only bounded safe
/// slices.  They must not be retained after the call: an application remains
/// free to replace its header buffers between calls.
pub struct DeflateGzipPayloads<'a> {
    header: Option<GzipHeaderFields>,
    extra: Option<&'a [crate::stdlib::Byte]>,
    name: Option<&'a [u8]>,
    comment: Option<&'a [u8]>,
}

impl<'a> DeflateGzipPayloads<'a> {
    pub const fn empty() -> Self {
        Self {
            header: None,
            extra: None,
            name: None,
            comment: None,
        }
    }

    pub(crate) fn new(
        header: Option<GzipHeaderFields>,
        extra: Option<&'a [crate::stdlib::Byte]>,
        name: Option<&'a [u8]>,
        comment: Option<&'a [u8]>,
    ) -> Self {
        Self {
            header,
            extra,
            name,
            comment,
        }
    }
}

// This expands only at existing ABI boundaries. It snapshots the optional gzip
// header and lends its payloads once per ordinary deflate invocation; the
// compressor itself receives only safe values and views. The state pointer is
// checked before it is adopted so malformed streams remain stream errors.
macro_rules! deflate_gzip_payloads_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        let state = (*strm).state as *mut crate::src::deflate::deflate_state;
        if state.is_null() {
            None
        } else {
            let payloads = match (&*state).gzhead {
                None => crate::src::deflate::DeflateGzipPayloads::empty(),
                Some(header) => {
                    let header = &*header.as_ptr();
                    let fields = crate::src::deflate::gzip_header_fields(header);
                    let extra = if header.extra.is_null() {
                        None
                    } else {
                        let len = (header.extra_len & 0xffff as crate::stdlib::uInt) as usize;
                        Some(::core::slice::from_raw_parts(header.extra, len))
                    };
                    let name = if header.name.is_null() {
                        None
                    } else {
                        Some(::std::ffi::CStr::from_ptr(header.name.cast()).to_bytes_with_nul())
                    };
                    let comment = if header.comment.is_null() {
                        None
                    } else {
                        Some(::std::ffi::CStr::from_ptr(header.comment.cast()).to_bytes_with_nul())
                    };
                    crate::src::deflate::DeflateGzipPayloads::new(
                        Some(fields),
                        extra,
                        name,
                        comment,
                    )
                }
            };
            Some(payloads)
        }
    }};
}
pub(crate) use deflate_gzip_payloads_at_boundary;

/// Reduce the copied caller-owned ABI header to fields used by the fixed
/// header and HCRC transitions.  Payload pointer handling stays at its
/// individual copy boundary.
pub(crate) fn gzip_header_fields(header: &crate::zlib_h::gz_header) -> GzipHeaderFields {
    GzipHeaderFields {
        text: header.text != 0,
        hcrc: header.hcrc != 0,
        has_extra: !header.extra.is_null(),
        has_name: !header.name.is_null(),
        has_comment: !header.comment.is_null(),
        time: header.time,
        os: header.os,
        extra_len: header.extra_len,
    }
}

/// Decide the state transition immediately after emitting gzip's fixed header.
///
/// A caller-supplied header defers draining until the optional fields have
/// been emitted, while an absent header advances straight to normal block
/// output and must drain the fixed header now.  This is scalar policy only;
/// the ABI header observation and pending-buffer lend remain at the codec
/// boundary.
#[derive(Copy, Clone)]
struct GzipFixedHeaderPlan {
    next_status: ::core::ffi::c_int,
    reset_index: bool,
    flush_now: bool,
}

fn gzip_fixed_header_plan(has_header: bool) -> GzipFixedHeaderPlan {
    if has_header {
        GzipFixedHeaderPlan {
            next_status: crate::src::deflate::EXTRA_STATE,
            reset_index: true,
            flush_now: false,
        }
    } else {
        GzipFixedHeaderPlan {
            next_status: crate::src::deflate::BUSY_STATE,
            reset_index: false,
            flush_now: true,
        }
    }
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

/// Start a stored block whose bytes will be copied directly to the caller's
/// output.  The block header is emitted through the same slice-only tree core
/// as buffered blocks, then its advertised length is installed before the
/// legacy adapter lends the caller output span.
fn emit_stored_direct_header_state(
    s: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Byte],
    len: ::core::ffi::c_uint,
    last: ::core::ffi::c_int,
) -> bool {
    if !crate::src::trees::tr_stored_block_state(
        pending_buf,
        &mut s.pending,
        &mut s.bi_buf,
        &mut s.bi_valid,
        &mut s.bi_used,
        &[],
        last,
    ) || !set_stored_block_length_state(pending_buf, s.pending, len)
    {
        return false;
    }
    if last != 0 {
        s.bi_used = 8 as ::core::ffi::c_int;
    }
    true
}

/// Emit a stored block that is already buffered in the history window.
/// Keeping the bit state, pending range, and block cursor transition together
/// makes the tail path entirely slice/scalar based.
fn emit_stored_window_block_state(
    s: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Byte],
    stored: &[crate::stdlib::Byte],
    last: ::core::ffi::c_int,
) -> bool {
    if !crate::src::trees::tr_stored_block_state(
        pending_buf,
        &mut s.pending,
        &mut s.bi_buf,
        &mut s.bi_valid,
        &mut s.bi_used,
        stored,
        last,
    ) {
        return false;
    }
    s.block_start = s
        .block_start
        .wrapping_add(stored.len() as ::core::ffi::c_long);
    if last != 0 {
        s.bi_used = 8 as ::core::ffi::c_int;
    }
    true
}

#[derive(Clone, Copy)]
enum FlushPendingMark {
    Never,
    OutputFull,
    PendingRemains,
}

/// One bounded caller-output lend for an ordinary `deflate()` transition.
/// The ABI boundary creates this cursor once; internal flushes only advance
/// its checked slice offset and synchronize the legacy stream cursor.
pub struct DeflateOutput<'a> {
    bytes: &'a mut [crate::stdlib::Byte],
    used: usize,
}

impl<'a> DeflateOutput<'a> {
    pub fn new(bytes: &'a mut [crate::stdlib::Byte]) -> Self {
        Self { bytes, used: 0 }
    }

    fn remaining_len(&self) -> usize {
        self.bytes.len().saturating_sub(self.used)
    }

    fn remaining_mut(&mut self) -> Option<&mut [crate::stdlib::Byte]> {
        self.bytes.get_mut(self.used..)
    }

    fn advance(&mut self, len: usize) -> bool {
        let Some(next) = self.used.checked_add(len) else {
            return false;
        };
        if next > self.bytes.len() {
            return false;
        }
        self.used = next;
        true
    }
}

/// Flush pending output and return its post-flush progress.
///
/// Callers retain ownership of the stream/state records, so this boundary only
/// needs to lend the pending and output buffers for one bounded flush.
fn flush_pending(
    strm: &mut crate::zlib_h::z_stream,
    s: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Byte],
    output: &mut DeflateOutput<'_>,
    mark_last_flush: FlushPendingMark,
) -> (crate::stdlib::uInt, Option<crate::zutil_h::ulg>) {
    // The ABI boundary lends the callback-owned pending allocation once for
    // this codec transition. This implementation only validates its retained
    // capacity and performs checked slice operations.
    let Ok(pending_len) = usize::try_from(s.pending_buf_size) else {
        return (strm.avail_out, Some(s.pending));
    };
    if pending_buf.len() != pending_len {
        return (strm.avail_out, Some(s.pending));
    }
    // Tree construction keeps this derived value in opaque state.  Publish
    // it while this existing stream/state boundary is active.
    strm.data_type = s.data_type;
    crate::src::trees::flush_bits_state(s, pending_buf);
    let Some((len, reset_pending_out)) =
        flush_pending_state(s.pending_buf_size, &mut s.pending, strm.avail_out)
    else {
        return (strm.avail_out, Some(s.pending));
    };
    if len == 0 {
        return (strm.avail_out, Some(s.pending));
    }
    let Ok(len) = usize::try_from(len) else {
        return (strm.avail_out, Some(s.pending));
    };
    if len > output.remaining_len() || output.remaining_len() != strm.avail_out as usize {
        return (strm.avail_out, Some(s.pending));
    }
    // `memcpy` required raw cursors even after their bounds had been
    // validated.  Keep the ABI lends here, reject aliasing just as C
    // `memcpy` requires, then perform the actual copy in the slice core.
    let Some(pending_end) = s.pending_out.checked_add(len) else {
        return (strm.avail_out, Some(s.pending));
    };
    if pending_end > pending_buf.len() {
        return (strm.avail_out, Some(s.pending));
    }
    let Some(pending_address) = (pending_buf.as_ptr() as usize).checked_add(s.pending_out) else {
        return (strm.avail_out, Some(s.pending));
    };
    let Some(output_buf) = output.remaining_mut() else {
        return (strm.avail_out, Some(s.pending));
    };
    if !deflate_spans_are_disjoint(output_buf.as_mut_ptr() as usize, len, pending_address, len) {
        return (strm.avail_out, Some(s.pending));
    }
    let Some(pending) = pending_buf.get(s.pending_out..pending_end) else {
        return (strm.avail_out, Some(s.pending));
    };
    let Some(output_slice) = output_buf.get_mut(..len) else {
        return (strm.avail_out, Some(s.pending));
    };
    if !flush_pending_copy_state(output_slice, pending) {
        return (strm.avail_out, Some(s.pending));
    }
    if !output.advance(len) {
        return (strm.avail_out, Some(s.pending));
    }
    // The copied length is bounded by the validated output and pending
    // spans above. Preserve zlib's cursor advance without an unsafe pointer
    // offset operation in this transitional ABI adapter.
    strm.next_out = strm.next_out.wrapping_add(len);
    s.pending_out = pending_end;
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out = strm.avail_out.wrapping_sub(len as crate::stdlib::uInt);
    if reset_pending_out {
        s.pending_out = 0;
    }
    // The initial ordinary-deflate pending flush historically reset this
    // marker immediately after a full output span.  Keep that scalar
    // state commit in the existing stream/state boundary so its caller
    // does not need to re-adopt the raw state record after flushing.
    if matches!(mark_last_flush, FlushPendingMark::OutputFull) && strm.avail_out == 0
        || matches!(mark_last_flush, FlushPendingMark::PendingRemains) && s.pending != 0
    {
        s.last_flush = -1 as ::core::ffi::c_int;
    }
    (strm.avail_out, Some(s.pending))
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

/// Decide whether a gzip header CRC must be emitted and whether its two-byte
/// trailer first needs to drain the pending buffer.  This is scalar-only so
/// the legacy gzip-header boundary need not re-adopt the opaque state just to
/// repeat its capacity arithmetic.
#[derive(Copy, Clone)]
struct GzipHcrcPlan {
    emit: bool,
    flush_before_emit: bool,
}

fn gzip_hcrc_plan(
    hcrc_enabled: bool,
    pending: crate::zutil_h::ulg,
    pending_buf_size: crate::zutil_h::ulg,
) -> GzipHcrcPlan {
    GzipHcrcPlan {
        emit: hcrc_enabled,
        flush_before_emit: hcrc_enabled
            && pending.wrapping_add(2 as crate::zutil_h::ulg) > pending_buf_size,
    }
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

/// Choose the ordinary deflate block encoder from validated scalar state.
///
/// The compatibility state is opaque to C callers, but a malformed internal
/// level previously reached `configuration_table` through an unchecked index
/// in the raw dispatcher.  Keep table selection in this safe core so such a
/// state becomes the established stream error rather than a panic.
#[derive(Copy, Clone)]
enum DeflateCompressor {
    Stored,
    Huffman,
    Rle,
    Fast,
    Slow,
}

fn deflate_compressor_plan(
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Option<DeflateCompressor> {
    if !(0..=9).contains(&level) || !(0..=crate::zlib_h::Z_FIXED).contains(&strategy) {
        return None;
    }
    if level == 0 {
        return Some(DeflateCompressor::Stored);
    }
    if strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
        return Some(DeflateCompressor::Huffman);
    }
    if strategy == crate::zlib_h::Z_RLE {
        return Some(DeflateCompressor::Rle);
    }
    match configuration_table.get(level as usize)?.kind {
        CompressorKind::Stored => Some(DeflateCompressor::Stored),
        CompressorKind::Fast => Some(DeflateCompressor::Fast),
        CompressorKind::Slow => Some(DeflateCompressor::Slow),
    }
}

/// Make a short ordinary Rust reborrow explicit without reconstructing a
/// reference through `&mut *...` inside the transitional dispatcher.
fn deflate_reborrow_mut<T: ?Sized>(value: &mut T) -> &mut T {
    value
}

fn deflate_reborrow<T: ?Sized>(value: &T) -> &T {
    value
}

pub fn deflate(
    strm_ref: &mut crate::zlib_h::z_stream,
    s: &mut crate::src::deflate::deflate_state,
    mut input: &mut &[crate::stdlib::Byte],
    pending_buf: &mut [crate::stdlib::Byte],
    window_hash: &mut DeflateWindowHashBuffers<'_>,
    output: &mut DeflateOutput<'_>,
    gzip_payloads: DeflateGzipPayloads<'_>,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // The ABI boundary has already adopted the callback-owned state and lent
    // every cursor-backed buffer for this transition. Keep the dispatcher on
    // those checked references so it never has to reconstruct the state.
    let mut old_flush: ::core::ffi::c_int = 0;
    if flush > crate::zlib_h::Z_BLOCK || flush < 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Validate the separately lent stream and state before changing either
    // record. The ABI boundary establishes that `s` is the stream's state;
    // the retained identity token catches an inconsistent direct caller.
    let (pending, avail_in) = {
        let stream = deflate_reborrow_mut(strm_ref);
        if !deflate_state_values_are_valid(
            stream.zalloc.is_some(),
            stream.zfree.is_some(),
            s.strm == stream as *mut crate::zlib_h::z_stream as usize,
            s.status,
        ) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if stream.next_out.is_null()
            || stream.avail_in != 0 as crate::stdlib::uInt && stream.next_in.is_null()
            || s.status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH
        {
            stream.msg = crate::src::zutil::z_errmsg[(if (-2 as ::core::ffi::c_int)
                < -6 as ::core::ffi::c_int
                || -2 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -2 as ::core::ffi::c_int
            }) as usize] as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            return -2 as ::core::ffi::c_int;
        }
        if stream.avail_out == 0 as crate::stdlib::uInt {
            stream.msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
                < -6 as ::core::ffi::c_int
                || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
            }) as usize] as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            return -5 as ::core::ffi::c_int;
        }
        old_flush = s.last_flush;
        s.last_flush = flush;
        let pending = s.pending != 0;
        (pending, stream.avail_in)
    };
    // The FFI boundary reduced the caller-retained header to this scalar
    // snapshot along with its bounded payload views. Do not observe that
    // caller-owned pointer again in the ordinary compressor.
    let gzip_header_fields = gzip_payloads.header;
    let Ok(input_len) = usize::try_from(avail_in) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if input.len() != input_len {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if pending {
        if flush_pending(
            strm_ref,
            s,
            pending_buf,
            output,
            FlushPendingMark::OutputFull,
        )
        .0 == 0 as crate::stdlib::uInt
        {
            return crate::zlib_h::Z_OK;
        }
    } else if repeated_flush_is_buffer_error(avail_in, flush, old_flush) {
        let stream = deflate_reborrow_mut(strm_ref);
        stream.msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
        }) as usize] as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    let finish_with_input = {
        let stream = deflate_reborrow(strm_ref);
        let state = deflate_reborrow(s);
        state.status == crate::src::deflate::FINISH_STATE
            && stream.avail_in != 0 as crate::stdlib::uInt
    };
    if finish_with_input {
        let stream = deflate_reborrow_mut(strm_ref);
        stream.msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
        }) as usize] as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    {
        let state = deflate_reborrow_mut(s);
        if state.status == crate::src::deflate::INIT_STATE && state.wrap == 0 as ::core::ffi::c_int
        {
            state.status = crate::src::deflate::BUSY_STATE;
        }
    }
    let initialized_zlib_header = {
        let state = deflate_reborrow(s);
        state.status == crate::src::deflate::INIT_STATE
    };
    if initialized_zlib_header {
        let stream = deflate_reborrow_mut(strm_ref);
        let state = deflate_reborrow_mut(s);
        let (header, dictionary_adler) = zlib_header_words(
            state.w_bits,
            state.strategy,
            state.level,
            state.strstart,
            stream.adler,
        );
        let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if pending_buf.len() != pending_len {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
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
        stream.adler = crate::src::adler32::ADLER32_INITIAL;
        state.status = crate::src::deflate::BUSY_STATE;
    }
    let zlib_header_is_pending = initialized_zlib_header && {
        let state = deflate_reborrow(s);
        state.pending != 0
    };
    if zlib_header_is_pending {
        // A partial pending flush can only leave pending bytes when the
        // output cursor is full. Have the existing flush boundary commit
        // `last_flush` for that condition, then use its returned pending
        // balance instead of re-adopting the raw state record to inspect it.
        if flush_pending(
            strm_ref,
            s,
            pending_buf,
            output,
            FlushPendingMark::PendingRemains,
        )
        .1 != Some(0)
        {
            return crate::zlib_h::Z_OK;
        }
    }
    let gzip_header_pending = {
        let state = deflate_reborrow(s);
        state.status == crate::src::deflate::GZIP_STATE
    };
    if gzip_header_pending {
        // Keep fixed-header construction within one ordinary stream/state
        // borrow.  The pending buffer is still lent only here, and the
        // borrow ends before `flush_pending()` can revisit compatibility
        // state or callback-owned output.
        let fixed_header_plan = {
            let stream = deflate_reborrow_mut(strm_ref);
            let state = deflate_reborrow_mut(s);
            stream.adler = crate::src::crc32::crc32_slice(0, &[]);
            let header = gzip_header_fields.map(|header| GzipFixedHeader {
                text: header.text,
                hcrc: header.hcrc,
                has_extra: header.has_extra,
                has_name: header.has_name,
                has_comment: header.has_comment,
                time: header.time,
                os: header.os,
                extra_len: header.extra_len,
            });
            let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            if pending_buf.len() != pending_len {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            if !append_gzip_fixed_header_state(
                pending_buf,
                &mut state.pending,
                &mut stream.adler,
                state.level,
                state.strategy,
                header,
            ) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            let plan = gzip_fixed_header_plan(header.is_some());
            if plan.reset_index {
                state.gzindex = 0 as crate::zutil_h::ulg;
            }
            state.status = plan.next_status;
            plan
        };
        if fixed_header_plan.flush_now {
            if flush_pending(
                strm_ref,
                s,
                pending_buf,
                output,
                FlushPendingMark::PendingRemains,
            )
            .1 != Some(0)
            {
                return crate::zlib_h::Z_OK;
            }
        }
    }
    // The scalar snapshot carries the retained-header metadata through
    // the remaining gzip transitions. Payload views were lent by the ABI
    // boundary for this invocation and stay as ordinary safe slices here.
    let gzip_extra_pending = {
        let state = deflate_reborrow(s);
        state.status == crate::src::deflate::EXTRA_STATE
    };
    if gzip_extra_pending {
        // The header value was snapshotted at the ABI boundary; ordinary
        // state and stream borrows handle every chunk commit.
        if let Some(extra) = gzip_payloads.extra {
            loop {
                let state = deflate_reborrow_mut(s);
                let stream = deflate_reborrow_mut(strm_ref);
                let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_buf.len() != pending_len {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                let Some(done) = append_gzip_extra_chunk_state(
                    pending_buf,
                    &mut state.pending,
                    &mut state.gzindex,
                    extra,
                    gzip_header_fields.is_some_and(|header| header.hcrc),
                    &mut stream.adler,
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
                if flush_pending(
                    strm_ref,
                    s,
                    pending_buf,
                    output,
                    FlushPendingMark::PendingRemains,
                )
                .1 != Some(0)
                {
                    return crate::zlib_h::Z_OK;
                }
            }
        }
        let state = deflate_reborrow_mut(s);
        state.status = crate::src::deflate::NAME_STATE;
    }
    // NAME follows the same entry snapshot as EXTRA; the temporary C
    // string view remains at the actual compatibility boundary below.
    let gzip_name_pending = {
        let state = deflate_reborrow(s);
        state.status == crate::src::deflate::NAME_STATE
    };
    if gzip_name_pending {
        if let Some(name) = gzip_payloads.name {
            loop {
                let state = deflate_reborrow_mut(s);
                let stream = deflate_reborrow_mut(strm_ref);
                let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_buf.len() != pending_len {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                let Some(done) = append_gzip_cstring_chunk_state(
                    pending_buf,
                    &mut state.pending,
                    &mut state.gzindex,
                    name,
                    gzip_header_fields.is_some_and(|header| header.hcrc),
                    &mut stream.adler,
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
                if flush_pending(
                    strm_ref,
                    s,
                    pending_buf,
                    output,
                    FlushPendingMark::PendingRemains,
                )
                .1 != Some(0)
                {
                    return crate::zlib_h::Z_OK;
                }
            }
        }
        let state = deflate_reborrow_mut(s);
        state.status = crate::src::deflate::COMMENT_STATE;
    }
    let gzip_comment_pending = {
        let state = deflate_reborrow(s);
        state.status == crate::src::deflate::COMMENT_STATE
    };
    if gzip_comment_pending {
        if let Some(comment) = gzip_payloads.comment {
            loop {
                let state = deflate_reborrow_mut(s);
                let stream = deflate_reborrow_mut(strm_ref);
                let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_buf.len() != pending_len {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                let Some(done) = append_gzip_cstring_chunk_state(
                    pending_buf,
                    &mut state.pending,
                    &mut state.gzindex,
                    comment,
                    gzip_header_fields.is_some_and(|header| header.hcrc),
                    &mut stream.adler,
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
                if flush_pending(
                    strm_ref,
                    s,
                    pending_buf,
                    output,
                    FlushPendingMark::PendingRemains,
                )
                .1 != Some(0)
                {
                    return crate::zlib_h::Z_OK;
                }
            }
        }
        let state = deflate_reborrow_mut(s);
        state.status = crate::src::deflate::HCRC_STATE;
    }
    let (gzip_hcrc_pending, hcrc_pending, hcrc_pending_buf_size) = {
        let state = deflate_reborrow(s);
        (
            state.status == crate::src::deflate::HCRC_STATE,
            state.pending,
            state.pending_buf_size,
        )
    };
    if gzip_hcrc_pending {
        // Snapshot the only header-derived flag and the pending capacity
        // together.  The scalar planner then owns the two-byte admission
        // check, avoiding a second raw state adoption before a possible
        // flush.
        let hcrc_plan = match gzip_header_fields {
            None => GzipHcrcPlan {
                emit: false,
                flush_before_emit: false,
            },
            Some(header) => gzip_hcrc_plan(header.hcrc, hcrc_pending, hcrc_pending_buf_size),
        };
        if hcrc_plan.emit {
            if hcrc_plan.flush_before_emit {
                if flush_pending(
                    strm_ref,
                    s,
                    pending_buf,
                    output,
                    FlushPendingMark::PendingRemains,
                )
                .1 != Some(0)
                {
                    return crate::zlib_h::Z_OK;
                }
            }
            let hcrc = {
                let stream = deflate_reborrow(strm_ref);
                stream.adler
            };
            let state = deflate_reborrow_mut(s);
            let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            if pending_buf.len() != pending_len {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            if !append_gzip_hcrc_state(pending_buf, &mut state.pending, hcrc) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            let stream = deflate_reborrow_mut(strm_ref);
            stream.adler = crate::src::crc32::crc32_slice(0, &[]);
        }
        let state = deflate_reborrow_mut(s);
        state.status = crate::src::deflate::BUSY_STATE;
        if flush_pending(
            strm_ref,
            s,
            pending_buf,
            output,
            FlushPendingMark::PendingRemains,
        )
        .1 != Some(0)
        {
            return crate::zlib_h::Z_OK;
        }
    }
    let should_compress = {
        let stream = deflate_reborrow(strm_ref);
        let state = deflate_reborrow(s);
        stream.avail_in != 0 as crate::stdlib::uInt
            || state.lookahead != 0 as crate::stdlib::uInt
            || flush != crate::zlib_h::Z_NO_FLUSH
                && state.status != crate::src::deflate::FINISH_STATE
    };
    if should_compress {
        let mut bstate: block_state = {
            // Reborrow the state once for both compressor selection and
            // the selected private block encoder.  The encoders own their
            // separate transitional buffer lends, so this does not extend
            // an ABI borrow across a callback-capable operation.
            let state = deflate_reborrow_mut(s);
            let Some(compressor) = deflate_compressor_plan(state.level, state.strategy) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            match compressor {
                DeflateCompressor::Stored => deflate_stored(
                    state,
                    strm_ref,
                    &mut input,
                    pending_buf,
                    window_hash,
                    output,
                    flush,
                ),
                DeflateCompressor::Huffman => deflate_huff(
                    state,
                    strm_ref,
                    &mut input,
                    pending_buf,
                    window_hash,
                    output,
                    flush,
                ),
                DeflateCompressor::Rle => deflate_rle(
                    state,
                    strm_ref,
                    &mut input,
                    pending_buf,
                    window_hash,
                    output,
                    flush,
                ),
                DeflateCompressor::Fast => deflate_fast(
                    state,
                    strm_ref,
                    &mut input,
                    pending_buf,
                    window_hash,
                    output,
                    flush,
                ),
                DeflateCompressor::Slow => deflate_slow(
                    state,
                    strm_ref,
                    &mut input,
                    pending_buf,
                    window_hash,
                    output,
                    flush,
                ),
            }
        };
        if bstate as ::core::ffi::c_uint
            == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
            || bstate as ::core::ffi::c_uint
                == finish_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let state = deflate_reborrow_mut(s);
            state.status = crate::src::deflate::FINISH_STATE;
        }
        if bstate as ::core::ffi::c_uint == need_more as ::core::ffi::c_int as ::core::ffi::c_uint
            || bstate as ::core::ffi::c_uint
                == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let output_is_full = {
                let stream = deflate_reborrow(strm_ref);
                stream.avail_out == 0 as crate::stdlib::uInt
            };
            if output_is_full {
                let state = deflate_reborrow_mut(s);
                state.last_flush = -1 as ::core::ffi::c_int;
            }
            return crate::zlib_h::Z_OK;
        }
        if bstate as ::core::ffi::c_uint == block_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if flush == crate::zlib_h::Z_PARTIAL_FLUSH {
                // This exported stream boundary owns the callback-allocated
                // pending buffer lend.  The bit alignment itself is a safe
                // slice/state transition.
                let state = deflate_reborrow_mut(s);
                let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_buf.len() != pending_len {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                crate::src::trees::tr_align_state(
                    pending_buf,
                    &mut state.pending,
                    &mut state.bi_buf,
                    &mut state.bi_valid,
                );
            } else if flush != crate::zlib_h::Z_BLOCK {
                // This stream boundary already owns the callback-allocated
                // pending buffer lend.  An empty stored block has no caller
                // buffer, so emit it through the slice-only tree core.
                let state = deflate_reborrow_mut(s);
                let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if pending_buf.len() != pending_len {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                if !crate::src::trees::tr_stored_block_state(
                    pending_buf,
                    &mut state.pending,
                    &mut state.bi_buf,
                    &mut state.bi_valid,
                    &mut state.bi_used,
                    &[],
                    0,
                ) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    let Ok(head_len) = usize::try_from(state.hash_size) else {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    };
                    if head_len == 0 || state.head.is_none() || window_hash.head.len() != head_len {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    }
                    // The per-call ABI boundary already lent the
                    // callback-allocated hash table. Full flush clears
                    // that exact checked lend (NIL is zero) through the
                    // slice-only state core instead of rebuilding a raw
                    // slice or using libc `memset`.
                    let head = &mut *window_hash.head;
                    clear_hash_state(head, &mut state.slid);
                    if state.lookahead == 0 as crate::stdlib::uInt {
                        state.strstart = 0 as crate::stdlib::uInt;
                        state.block_start = 0 as ::core::ffi::c_long;
                        state.insert = 0 as crate::stdlib::uInt;
                    }
                }
            }
            if flush_pending(
                strm_ref,
                s,
                pending_buf,
                output,
                FlushPendingMark::OutputFull,
            )
            .0 == 0 as crate::stdlib::uInt
            {
                return crate::zlib_h::Z_OK;
            }
        }
    }
    if flush != crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_OK;
    }
    {
        let state = deflate_reborrow_mut(s);
        let trailer = {
            let stream = deflate_reborrow(strm_ref);
            deflate_trailer_plan(state.wrap, stream.adler, stream.total_in)
        };
        if matches!(trailer, DeflateTrailerPlan::None) {
            return crate::zlib_h::Z_STREAM_END;
        }
        let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if pending_buf.len() != pending_len {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let trailer_written = match trailer {
            DeflateTrailerPlan::Gzip { check, total_in } => {
                append_gzip_trailer_state(pending_buf, &mut state.pending, check, total_in)
            }
            DeflateTrailerPlan::Zlib {
                check_high,
                check_low,
            } => append_zlib_words_state(pending_buf, &mut state.pending, &[check_high, check_low]),
            DeflateTrailerPlan::None => return crate::zlib_h::Z_STREAM_END,
        };
        if !trailer_written {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
    };
    flush_pending(strm_ref, s, pending_buf, output, FlushPendingMark::Never);
    let pending_after_flush = {
        let state = deflate_reborrow_mut(s);
        if state.wrap > 0 as ::core::ffi::c_int {
            state.wrap = -state.wrap;
        }
        state.pending
    };
    if pending_after_flush != 0 as crate::zutil_h::ulg {
        crate::zlib_h::Z_OK
    } else {
        crate::zlib_h::Z_STREAM_END
    }
}
#[export_name = "deflate"]

pub unsafe extern "C" fn deflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if flush > crate::zlib_h::Z_BLOCK || flush < 0 || strm.is_null() {
        crate::zlib_h::Z_STREAM_ERROR
    } else {
        let input_len = (*strm).avail_in as usize;
        if input_len != 0 && (*strm).next_in.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let mut input = if input_len == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts((*strm).next_in, input_len)
        };
        let output_len = (*strm).avail_out as usize;
        if output_len != 0 && (*strm).next_out.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let output = if output_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut((*strm).next_out, output_len)
        };
        let mut output = DeflateOutput::new(output);
        let Some(gzip_payloads) = deflate_gzip_payloads_at_boundary!(strm) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let state = (*strm).state as *mut crate::src::deflate::deflate_state;
        if state.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let Ok(pending_len) = usize::try_from((*state).pending_buf_size) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if pending_len != 0 && (*state).pending_buf.is_none() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let pending_buf = if pending_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut(
                (*state)
                    .pending_buf
                    .expect("validated deflate pending buffer")
                    .as_ptr(),
                pending_len,
            )
        };
        let Some(mut window_hash) = deflate_window_hash_buffers_at_boundary!(&mut *state) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        deflate(
            &mut *strm,
            &mut *state,
            &mut input,
            pending_buf,
            &mut window_hash,
            &mut output,
            gzip_payloads,
            flush,
        )
    }
}
fn deflate_end_status(status: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    }
}

/// Validate the safe scalar/state portion of teardown before the ABI boundary
/// snapshots pointer handles and invokes the foreign free callback.
fn deflate_state_can_end(
    strm: &crate::zlib_h::z_stream,
    state: &deflate_state,
    stream_identity: usize,
) -> bool {
    deflate_state_values_are_valid(
        strm.zalloc.is_some(),
        strm.zfree.is_some(),
        state.strm == stream_identity,
        state.status,
    )
}

pub fn deflateEnd(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    // This compatibility teardown still has to adopt the ABI stream/state
    // records and invoke its C allocator. Keep that work at this codec
    // boundary so Rust callers do not inherit an unsafe-function contract.
    unsafe {
        // Validate and snapshot every callback argument before releasing any
        // allocation. A custom `zfree` is foreign code, so do not retain a Rust
        // borrow of the stream or state while it runs.
        let (state_ptr, zfree, opaque, status, pending_buf, head, prev, window) = {
            let stream_identity = strm as *mut crate::zlib_h::z_stream as usize;
            let state_ptr = strm.state as *mut crate::src::deflate::deflate_state;
            if state_ptr.is_null() {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            let state = &*state_ptr;
            if !deflate_state_can_end(strm, state, stream_identity) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            let Some(zfree) = strm.zfree else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            (
                state_ptr,
                zfree,
                strm.opaque,
                state.status,
                state.pending_buf,
                state.head,
                state.prev,
                state.window,
            )
        };
        if let Some(pending_buf) = pending_buf {
            zfree(opaque, pending_buf.as_ptr() as crate::stdlib::voidpf);
        }
        if let Some(head) = head {
            zfree(opaque, head.as_ptr() as crate::stdlib::voidpf);
        }
        if let Some(prev) = prev {
            zfree(opaque, prev.as_ptr() as crate::stdlib::voidpf);
        }
        if let Some(window) = window {
            zfree(opaque, window.as_ptr() as crate::stdlib::voidpf);
        }
        zfree(opaque, state_ptr as crate::stdlib::voidpf);
        strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
        deflate_end_status(status)
    }
}
#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflateEnd(&mut *strm)
}
#[export_name = "deflateCopy"]
pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if deflate_state_check_at_boundary!(source) != 0 || dest.is_null() || dest == source {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source_stream = &*source;
    let source_state = &*(source_stream.state as *mut crate::src::deflate::deflate_state);
    let dest_stream = &mut *dest;
    *dest_stream = *source_stream;
    let Some(zalloc) = dest_stream.zalloc else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let opaque = dest_stream.opaque;
    let ds = zalloc(
        opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::deflate_state;
    if ds.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    dest_stream.state = ds as *mut crate::src::deflate::internal_state;
    let dest_state = &mut *ds;
    *dest_state = *source_state;
    dest_state.strm = dest as usize;
    dest_state.window = ::core::ptr::NonNull::new(zalloc(
        opaque,
        dest_state.w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
            as crate::stdlib::uInt,
    ) as *mut crate::stdlib::Bytef);
    dest_state.prev = ::core::ptr::NonNull::new(zalloc(
        opaque,
        dest_state.w_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf);
    dest_state.head = ::core::ptr::NonNull::new(zalloc(
        opaque,
        dest_state.hash_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf);
    dest_state.pending_buf =
        ::core::ptr::NonNull::new(
            zalloc(opaque, dest_state.lit_bufsize, 4 as crate::stdlib::uInt)
                as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef,
        );
    if dest_state.window.is_none()
        || dest_state.prev.is_none()
        || dest_state.head.is_none()
        || dest_state.pending_buf.is_none()
    {
        deflateEnd(&mut *dest);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    let Some(window_len) = usize::try_from(dest_state.window_size).ok() else {
        deflateEnd(&mut *dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(prev_capacity) = usize::try_from(dest_state.w_size).ok() else {
        deflateEnd(&mut *dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(head_len) = usize::try_from(dest_state.hash_size).ok() else {
        deflateEnd(&mut *dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(pending_capacity) = usize::try_from(dest_state.pending_buf_size).ok() else {
        deflateEnd(&mut *dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if (window_len != 0 && source_state.window.is_none())
        || (prev_capacity != 0 && source_state.prev.is_none())
        || (head_len != 0 && source_state.head.is_none())
        || (pending_capacity != 0 && source_state.pending_buf.is_none())
    {
        deflateEnd(&mut *dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let window_len_to_copy = usize::try_from(source_state.high_water)
        .ok()
        .filter(|len| *len <= window_len);
    let prev_len = if source_state.slid != 0
        || source_state.strstart.wrapping_sub(source_state.insert) > dest_state.w_size
    {
        Some(prev_capacity)
    } else {
        usize::try_from(source_state.strstart.wrapping_sub(source_state.insert))
            .ok()
            .filter(|len| *len <= prev_capacity)
    };
    let pending_len = usize::try_from(source_state.pending).ok();
    let sym_len = usize::try_from(source_state.sym_next).ok();
    let pending_range = pending_len.and_then(|len| {
        source_state
            .pending_out
            .checked_add(len)
            .filter(|end| *end <= pending_capacity)
            .map(|end| source_state.pending_out..end)
    });
    let src_sym = sym_len.and_then(|len| {
        source_state
            .sym_start
            .checked_add(len)
            .filter(|end| *end <= pending_capacity)
            .map(|end| source_state.sym_start..end)
    });
    let dst_sym = sym_len.and_then(|len| {
        usize::try_from(dest_state.lit_bufsize)
            .ok()
            .and_then(|start| start.checked_add(len))
            .filter(|end| *end <= pending_capacity)
            .and_then(|end| end.checked_sub(len).map(|start| start..end))
    });
    let Some((window_len_to_copy, prev_len, pending_range, src_sym, dst_sym)) = window_len_to_copy
        .zip(prev_len)
        .zip(pending_range)
        .zip(src_sym)
        .zip(dst_sym)
        .map(
            |((((window_len_to_copy, prev_len), pending_range), src_sym), dst_sym)| {
                (
                    window_len_to_copy,
                    prev_len,
                    pending_range,
                    src_sym,
                    dst_sym,
                )
            },
        )
    else {
        deflateEnd(&mut *dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let dest_sym_start = dst_sym.start;
    let src_window = if window_len == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(
            source_state
                .window
                .expect("validated source deflate window")
                .as_ptr(),
            window_len,
        )
    };
    let dst_window = if window_len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(
            dest_state
                .window
                .expect("validated destination deflate window")
                .as_ptr(),
            window_len,
        )
    };
    let src_prev = if prev_capacity == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(
            source_state
                .prev
                .expect("validated source deflate previous table")
                .as_ptr(),
            prev_capacity,
        )
    };
    let dst_prev = if prev_capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(
            dest_state
                .prev
                .expect("validated destination deflate previous table")
                .as_ptr(),
            prev_capacity,
        )
    };
    let src_head = if head_len == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(
            source_state
                .head
                .expect("validated source deflate hash table")
                .as_ptr(),
            head_len,
        )
    };
    let dst_head = if head_len == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(
            dest_state
                .head
                .expect("validated destination deflate hash table")
                .as_ptr(),
            head_len,
        )
    };
    let src_pending = if pending_capacity == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(
            source_state
                .pending_buf
                .expect("validated source deflate pending buffer")
                .as_ptr(),
            pending_capacity,
        )
    };
    let dst_pending = if pending_capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(
            dest_state
                .pending_buf
                .expect("validated destination deflate pending buffer")
                .as_ptr(),
            pending_capacity,
        )
    };
    if !deflate_copy_owned_buffers(
        dst_window,
        src_window,
        window_len_to_copy,
        dst_prev,
        src_prev,
        prev_len,
        dst_head,
        src_head,
        head_len,
        dst_pending,
        src_pending,
        pending_range.clone(),
        dst_sym,
        src_sym,
    ) {
        deflateEnd(&mut *dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    dest_state.pending_out = pending_range.start;
    dest_state.sym_start = dest_sym_start;
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

/// Select a tree flusher's stored-block view from the validated history
/// window.  Unlike the legacy cursor adapter, this keeps both the source
/// range and the negative `block_start` sentinel explicit.
fn tree_block_window_slice(
    window: &[crate::stdlib::Byte],
    block_start: ::core::ffi::c_long,
    stored_len: crate::zutil_h::ulg,
) -> Option<&[crate::stdlib::Byte]> {
    let start = usize::try_from(block_start).ok()?;
    let len = usize::try_from(stored_len).ok()?;
    let end = start.checked_add(len)?;
    window.get(start..end)
}

/// Flush one codec block through the slice-based tree entry point.
/// A negative block cursor retains zlib's null-source behavior, while a
/// nonnegative cursor must name a complete span within the owned window.
fn flush_tree_window_block_state(
    state: &mut crate::src::deflate::deflate_state,
    pending_and_symbols: &mut [crate::stdlib::Byte],
    window: &[crate::stdlib::Byte],
    last: ::core::ffi::c_int,
) {
    let stored_len = block_flush_len_state(state.strstart, state.block_start);
    let stored = if state.block_start >= 0 {
        tree_block_window_slice(window, state.block_start, stored_len)
    } else {
        None
    };
    crate::src::trees::tr_flush_block(state, pending_and_symbols, stored, stored_len, last);
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
    // A valid deflate state always has room for the stored-block header.  Do
    // not let a malformed transitional state turn a too-small pending buffer
    // into a huge capacity through wrapping subtraction.
    let pending_capacity = pending_buf_size.checked_sub(5)?;
    let min_block = (if pending_capacity > wsize as crate::zutil_h::ulg {
        wsize as crate::zutil_h::ulg
    } else {
        pending_capacity
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
    let pending_capacity = pending_buf_size.checked_sub(bit_bytes as crate::zutil_h::ulg)?;
    let have = (if pending_capacity > 65535 as crate::zutil_h::ulg {
        65535 as crate::zutil_h::ulg
    } else {
        pending_capacity
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

/// Translate stored-mode input availability into the number of caller bytes
/// consumed by this dispatch. The mode boundary owns the ABI stream borrow;
/// this helper preserves zlib's wrapping counter arithmetic without looking
/// through that record.
fn stored_input_consumed(
    initial_avail_in: crate::stdlib::uInt,
    remaining_avail_in: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    initial_avail_in.wrapping_sub(remaining_avail_in)
}

/// Apply one stored-mode output write to the stream counters.  The codec
/// boundary remains responsible for advancing its ABI cursor, while this
/// value-only transition makes the capacity check and counter accounting
/// shared by buffered-window copies and direct input copies.
fn record_stored_output_state(
    avail_out: &mut crate::stdlib::uInt,
    total_out: &mut crate::stdlib::uLong,
    written: crate::stdlib::uInt,
) -> bool {
    let Some(remaining) = avail_out.checked_sub(written) else {
        return false;
    };
    *avail_out = remaining;
    *total_out = total_out.wrapping_add(written as crate::stdlib::uLong);
    true
}

/// The checked scalar commit after stored mode has copied caller bytes into
/// the history window.  Keeping the prospective values separate lets the
/// cursor boundary reject an incoherent state before it consumes input.
struct StoredInputProgress {
    strstart: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    high_water: crate::zutil_h::ulg,
}

fn stored_input_progress_state(
    s: &crate::src::deflate::deflate_state,
    have: crate::stdlib::uInt,
) -> Option<StoredInputProgress> {
    if s.insert > s.w_size {
        return None;
    }
    let strstart = s.strstart.checked_add(have)?;
    let insert_room = s.w_size.checked_sub(s.insert)?;
    let insert = s.insert.checked_add(have.min(insert_room))?;
    Some(StoredInputProgress {
        strstart,
        insert,
        high_water: s.high_water.max(strstart as crate::zutil_h::ulg),
    })
}

/// Commit one checked stored-mode history-input transition.  The legacy
/// adapter retains the raw input and window lends; this function only
/// publishes scalar state after the bytes have been copied.
fn record_stored_input_state(
    s: &mut crate::src::deflate::deflate_state,
    have: crate::stdlib::uInt,
) -> bool {
    let Some(progress) = stored_input_progress_state(s, have) else {
        return false;
    };
    s.strstart = progress.strstart;
    s.insert = progress.insert;
    s.high_water = progress.high_water;
    true
}

/// Select the bytes for a stored block from the owned deflate window.  The
/// caller lends that callback-allocated window at its existing transitional
/// boundary; range arithmetic itself stays ordinary slice logic.
fn stored_block_window_slice(
    window: &[crate::stdlib::Byte],
    block_start: ::core::ffi::c_long,
    stored_len: crate::stdlib::uInt,
) -> Option<&[crate::stdlib::Byte]> {
    let start = usize::try_from(block_start).ok()?;
    let len = usize::try_from(stored_len).ok()?;
    let end = start.checked_add(len)?;
    window.get(start..end)
}

fn deflate_stored(
    s: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    input: &mut &[crate::stdlib::Byte],
    pending_buf: &mut [crate::stdlib::Byte],
    window_hash: &mut DeflateWindowHashBuffers<'_>,
    output: &mut DeflateOutput<'_>,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    // The ABI boundary has already lent the callback-owned window. Stored
    // mode only reborrows that bounded span for its direct copies, history
    // update, and tail refill; it does not need to recreate raw window lends.
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    // Snapshot the initial input availability together with the first block
    // admission.  The planner is scalar-only, so this avoids a separate raw
    // stream adoption before the loop without changing when input is read.
    let mut used: ::core::ffi::c_uint = 0;
    let mut remaining_avail_in: crate::stdlib::uInt = 0;
    let mut input_start: &[crate::stdlib::Byte] = &[];
    let mut first_block = true;
    loop {
        let (initial_avail_in, initial_input, plan) = {
            (
                strm.avail_in,
                *input,
                stored_initial_block_plan(
                    s.pending_buf_size,
                    s.w_size,
                    s.bi_valid,
                    strm.avail_out,
                    s.strstart,
                    s.block_start,
                    strm.avail_in,
                    flush,
                ),
            )
        };
        remaining_avail_in = initial_avail_in;
        if first_block {
            used = initial_avail_in;
            input_start = initial_input;
            first_block = false;
        }
        let Some(plan) = plan else {
            break;
        };
        len = plan.len;
        left = plan.left;
        last = plan.last;
        // The zero-length stored block needs only the already-owned pending
        // buffer.  Lend that validated buffer directly to the safe core
        // instead of round-tripping through the raw `_tr_stored_block`
        // adapter.
        let Ok(pending_len) = usize::try_from(s.pending_buf_size) else {
            return need_more;
        };
        if pending_buf.len() != pending_len {
            return need_more;
        }
        if !emit_stored_direct_header_state(s, pending_buf, len, last) {
            return need_more;
        }
        flush_pending(strm, s, pending_buf, output, FlushPendingMark::Never);
        if left != 0 || len != 0 {
            let Ok(output_len) = usize::try_from(strm.avail_out) else {
                return need_more;
            };
            if output_len != output.remaining_len() {
                return need_more;
            }
            let Some(output_slice) = output.remaining_mut() else {
                return need_more;
            };
            let mut output_used = 0usize;
            if left != 0 {
                let Ok(window_len) = usize::try_from(s.window_size) else {
                    return need_more;
                };
                if window_hash.window.len() != window_len || window_len != 0 && s.window.is_none() {
                    return need_more;
                }
                let window = &*window_hash.window;
                let Some(copy) = copy_stored_window_to_output_state(
                    window,
                    output_slice,
                    s.block_start,
                    left,
                    len,
                ) else {
                    return need_more;
                };
                let Ok(copied) = usize::try_from(copy.copied) else {
                    return need_more;
                };
                output_used = copied;
                if !record_stored_output_state(
                    &mut strm.avail_out,
                    &mut strm.total_out,
                    copy.copied,
                ) {
                    return need_more;
                }
                s.block_start = copy.block_start;
                len = copy.remaining;
            }
            if len != 0 {
                let Some(destination) = output_slice.get_mut(output_used..) else {
                    return need_more;
                };
                let progress = read_buf(strm, input, destination, len, s.wrap);
                remaining_avail_in = progress.avail_in;
                // Advance by the bytes actually copied. On valid zlib state
                // this equals `len`; retaining the returned value keeps a
                // malformed cursor/input pairing from publishing progress it
                // did not make.
                output_used = output_used.wrapping_add(progress.copied as usize);
                if !record_stored_output_state(
                    &mut strm.avail_out,
                    &mut strm.total_out,
                    progress.copied,
                ) {
                    return need_more;
                }
            }
            if !output.advance(output_used) {
                return need_more;
            }
            strm.next_out = strm.next_out.wrapping_add(output_used);
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = stored_input_consumed(used, remaining_avail_in);
    if used != 0 {
        let Ok(window_len) = usize::try_from(s.window_size) else {
            return need_more;
        };
        let Ok(used_len) = usize::try_from(used) else {
            return need_more;
        };
        if s.window.is_none() || window_hash.window.len() != window_len {
            return need_more;
        }
        let window = &mut *window_hash.window;
        let Some(consumed) = input_start.get(..used_len) else {
            return need_more;
        };
        if !update_stored_history_state(s, window, consumed) {
            return need_more;
        }
    }
    if last != 0 {
        return finish_done;
    }
    {
        if flush != crate::zlib_h::Z_NO_FLUSH
            && flush != crate::zlib_h::Z_FINISH
            && strm.avail_in == 0 as crate::stdlib::uInt
            && s.strstart as ::core::ffi::c_long == s.block_start
        {
            return block_done;
        }
        let Ok(window_len) = usize::try_from(s.window_size) else {
            return need_more;
        };
        if window_hash.window.len() != window_len || window_len != 0 && s.window.is_none() {
            return need_more;
        }
        let window = &mut *window_hash.window;
        let Some(next_have) = rebalance_stored_window_state(s, window, strm.avail_in) else {
            return need_more;
        };
        have = next_have;
        if have > strm.avail_in {
            have = strm.avail_in as ::core::ffi::c_uint;
        }
        if have != 0 {
            // Select the refill tail through the same checked slice plan as
            // `fill_window`, rather than advancing a raw window cursor.
            // The raw adapter remains responsible only for its existing
            // ABI input/output lends.
            // Validate the complete possible state commit before reading:
            // `read_buf()` may advance the ABI input cursor, so a malformed
            // history state must be rejected before that observable change.
            if stored_input_progress_state(s, have).is_none() {
                return need_more;
            }
            let Some(write_span) = fill_window_write_span(window.len(), s.strstart, 0, have) else {
                return need_more;
            };
            let Some(output) = window.get_mut(write_span) else {
                return need_more;
            };
            let progress = read_buf(strm, input, output, have, s.wrap);
            if progress.copied > have || !record_stored_input_state(s, progress.copied) {
                return need_more;
            }
        }
        let tail_plan = stored_tail_block_plan(
            s.pending_buf_size,
            s.bi_valid,
            s.w_size,
            s.strstart,
            s.block_start,
            strm.avail_in,
            flush,
        );
        if let Some(plan) = tail_plan {
            len = plan.len;
            last = plan.last;
            let Ok(pending_len) = usize::try_from(s.pending_buf_size) else {
                return need_more;
            };
            if pending_buf.len() != pending_len {
                return need_more;
            }
            let stored = if len == 0 {
                &[]
            } else {
                let Some(stored) = stored_block_window_slice(window, s.block_start, len) else {
                    return need_more;
                };
                stored
            };
            if !emit_stored_window_block_state(s, pending_buf, stored, last) {
                return need_more;
            }
            flush_pending(strm, s, pending_buf, output, FlushPendingMark::Never);
        }
    }
    return (if last != 0 {
        finish_started as ::core::ffi::c_int
    } else {
        need_more as ::core::ffi::c_int
    }) as block_state;
}

fn deflate_fast(
    s: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    input: &mut &[crate::stdlib::Byte],
    pending_buf: &mut [crate::stdlib::Byte],
    window_hash: &mut DeflateWindowHashBuffers<'_>,
    output: &mut DeflateOutput<'_>,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    // The ABI boundary already lent the callback-owned window, hash tables,
    // and pending buffer. Keep each no-refill parser transition within those
    // checked spans.
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        let needs_input = s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt;
        if needs_input {
            fill_window_state(s, strm, input, window_hash);
            if s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if s.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        hash_head = NIL as crate::src::deflate::IPos;
        // Both parser transitions need the same callback-owned window
        // and pending allocation. Lend each once per no-refill
        // transition; only the match path additionally needs hash views.
        let state: &mut crate::src::deflate::deflate_state = s;
        let has_min_match = state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt;
        let (Ok(window_len), Ok(pending_len)) = (
            usize::try_from(state.window_size),
            usize::try_from(state.pending_buf_size),
        ) else {
            return need_more;
        };
        if (window_len != 0 && state.window.is_none())
            || window_hash.window.len() != window_len
            || pending_buf.len() != pending_len
        {
            return need_more;
        }
        let window = &*window_hash.window;
        let pending_and_symbols = &mut *pending_buf;
        if has_min_match {
            let (Ok(head_len), Ok(prev_len)) = (
                usize::try_from(state.hash_size),
                usize::try_from(state.w_size),
            ) else {
                return need_more;
            };
            if (head_len != 0 && state.head.is_none())
                || (prev_len != 0 && state.prev.is_none())
                || window_hash.head.len() != head_len
                || window_hash.prev.len() != prev_len
            {
                return need_more;
            }
            let head = &mut *window_hash.head;
            let prev = &mut *window_hash.prev;
            // `sym_start` is the literal-buffer offset within the pending
            // allocation. Keep this existing no-refill window/hash lend
            // alive through a checked tree flush, rather than rebuilding
            // either the symbol cursor or stored-block cursor below.
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
            if state.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
                let Ok(symbol_start) = usize::try_from(state.lit_bufsize) else {
                    return need_more;
                };
                let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                    return need_more;
                };
                let Some(symbol_end) = symbol_start.checked_add(symbol_len) else {
                    return need_more;
                };
                let Some(symbols) = pending_and_symbols.get_mut(symbol_start..symbol_end) else {
                    return need_more;
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
                let Ok(symbol_start) = usize::try_from(state.lit_bufsize) else {
                    return need_more;
                };
                let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                    return need_more;
                };
                let Some(symbol_end) = symbol_start.checked_add(symbol_len) else {
                    return need_more;
                };
                let Some(symbols) = pending_and_symbols.get_mut(symbol_start..symbol_end) else {
                    return need_more;
                };
                let Some(flush_now) = tally_current_literal_state(state, window, symbols) else {
                    return need_more;
                };
                bflush = flush_now as ::core::ffi::c_int;
            }
            if bflush != 0 {
                flush_tree_window_block_state(state, pending_and_symbols, window, 0);
                let avail_out = {
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
                };
                if let Some(result) = deflate_post_flush_result(avail_out, false) {
                    return result;
                }
                bflush = 0;
            }
        } else {
            let Ok(symbol_start) = usize::try_from(state.lit_bufsize) else {
                return need_more;
            };
            let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                return need_more;
            };
            let Some(symbol_end) = symbol_start.checked_add(symbol_len) else {
                return need_more;
            };
            let Some(symbols) = pending_and_symbols.get_mut(symbol_start..symbol_end) else {
                return need_more;
            };
            let Some(flush_now) = tally_current_literal_state(state, window, symbols) else {
                return need_more;
            };
            bflush = flush_now as ::core::ffi::c_int;
            if bflush != 0 {
                flush_tree_window_block_state(state, pending_and_symbols, window, 0);
                let avail_out = {
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
                };
                if let Some(result) = deflate_post_flush_result(avail_out, false) {
                    return result;
                }
                bflush = 0;
            }
        }
    }
    // The fast parser's final and symbol-only tails use the same checked
    // pending/window layout as its regular block flushes.  Finish all
    // tree work while those lends are live, then release them before
    // `flush_pending()` reaches the ABI-owned output cursor.
    let tail_last = {
        let state: &mut crate::src::deflate::deflate_state = s;
        state.insert = if state.strstart
            < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
        {
            state.strstart
        } else {
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
        };
        let needs_flush = flush == crate::zlib_h::Z_FINISH || state.sym_next != 0;
        if !needs_flush {
            None
        } else {
            let (Ok(window_len), Ok(pending_len)) = (
                usize::try_from(state.window_size),
                usize::try_from(state.pending_buf_size),
            ) else {
                return need_more;
            };
            if (window_len != 0 && state.window.is_none())
                || window_hash.window.len() != window_len
                || pending_buf.len() != pending_len
            {
                return need_more;
            }
            let window = &*window_hash.window;
            let pending_and_symbols = &mut *pending_buf;
            let last = flush == crate::zlib_h::Z_FINISH;
            flush_tree_window_block_state(
                state,
                pending_and_symbols,
                window,
                last as ::core::ffi::c_int,
            );
            Some(last)
        }
    };
    if let Some(last) = tail_last {
        let avail_out = {
            let state: &mut crate::src::deflate::deflate_state = s;
            state.block_start = state.strstart as ::core::ffi::c_long;
            flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
        };
        if let Some(result) = deflate_post_flush_result(avail_out, last) {
            return result;
        }
        if last {
            return finish_done;
        }
    }
    return block_done;
}

fn deflate_slow(
    s: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    input: &mut &[crate::stdlib::Byte],
    pending_buf: &mut [crate::stdlib::Byte],
    window_hash: &mut DeflateWindowHashBuffers<'_>,
    output: &mut DeflateOutput<'_>,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    // The ABI boundary already lent the callback-owned window, hash tables,
    // and pending buffer. Keep each no-refill parser transition within those
    // checked spans.
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        let needs_input =
            { s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt };
        if needs_input {
            fill_window_state(s, strm, input, window_hash);
            if s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if s.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        hash_head = NIL as crate::src::deflate::IPos;
        let has_min_match = {
            s.prev_length = s.match_length;
            s.prev_match = s.match_start as crate::src::deflate::IPos;
            s.match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            s.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        };
        let mut handled_previous_match = false;
        // Both slow-parser outcomes inspect the same window and may tally
        // into the same pending allocation.  Lend them once for this
        // no-refill transition, then end the lends before `flush_pending()`
        // can revisit the ABI output cursor.
        {
            let state = deflate_reborrow_mut(s);
            let (Ok(window_len), Ok(pending_len)) = (
                usize::try_from(state.window_size),
                usize::try_from(state.pending_buf_size),
            ) else {
                return need_more;
            };
            if (window_len != 0 && state.window.is_none())
                || window_hash.window.len() != window_len
                || pending_buf.len() != pending_len
            {
                return need_more;
            }
            let window = &*window_hash.window;
            // `sym_start` is the literal-buffer offset within this pending
            // allocation. Keep the existing no-refill window/hash lends
            // through a possible tree flush instead of rebuilding either
            // raw cursor at the block boundary.
            let pending_and_symbols = &mut *pending_buf;
            if has_min_match {
                let (Ok(head_len), Ok(prev_len)) = (
                    usize::try_from(state.hash_size),
                    usize::try_from(state.w_size),
                ) else {
                    return need_more;
                };
                if (head_len != 0 && state.head.is_none())
                    || (prev_len != 0 && state.prev.is_none())
                    || window_hash.head.len() != head_len
                    || window_hash.prev.len() != prev_len
                {
                    return need_more;
                }
                let head = &mut *window_hash.head;
                let prev = &mut *window_hash.prev;
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
                let use_previous_match = state.prev_length
                    >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                    && state.match_length <= state.prev_length;
                if use_previous_match {
                    let max_insert = state
                        .strstart
                        .wrapping_add(state.lookahead)
                        .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
                    let len = state.prev_length.wrapping_sub(3 as crate::stdlib::uInt)
                        as crate::zutil_h::uch;
                    let dist = (state.strstart as crate::src::deflate::IPos)
                        .wrapping_sub(1 as crate::src::deflate::IPos)
                        .wrapping_sub(state.prev_match)
                        as crate::zutil_h::ush;
                    let Ok(symbol_start) = usize::try_from(state.lit_bufsize) else {
                        return need_more;
                    };
                    let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                        return need_more;
                    };
                    let Some(symbol_end) = symbol_start.checked_add(symbol_len) else {
                        return need_more;
                    };
                    let Some(symbols) = pending_and_symbols.get_mut(symbol_start..symbol_end)
                    else {
                        return need_more;
                    };
                    let Some(flush_now) =
                        tally_symbol_state(state, symbols, dist.into(), len.into())
                    else {
                        return need_more;
                    };
                    bflush = flush_now as ::core::ffi::c_int;
                    state.lookahead = state
                        .lookahead
                        .wrapping_sub(state.prev_length.wrapping_sub(1 as crate::stdlib::uInt));
                    state.prev_length = state.prev_length.wrapping_sub(2 as crate::stdlib::uInt);
                    loop {
                        state.strstart = state.strstart.wrapping_add(1);
                        if state.strstart <= max_insert {
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
                        state.prev_length = state.prev_length.wrapping_sub(1);
                        if state.prev_length == 0 as crate::stdlib::uInt {
                            break;
                        }
                    }
                    state.match_available = 0 as ::core::ffi::c_int;
                    state.match_length = (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int)
                        as crate::stdlib::uInt;
                    state.strstart = state.strstart.wrapping_add(1);
                    handled_previous_match = true;
                    if bflush != 0 {
                        flush_tree_window_block_state(state, pending_and_symbols, window, 0);
                    }
                }
            }
            if !handled_previous_match && state.match_available != 0 {
                let Ok(symbol_start) = usize::try_from(state.lit_bufsize) else {
                    return need_more;
                };
                let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                    return need_more;
                };
                let Some(symbol_end) = symbol_start.checked_add(symbol_len) else {
                    return need_more;
                };
                let Some(symbols) = pending_and_symbols.get_mut(symbol_start..symbol_end) else {
                    return need_more;
                };
                let Some(flush_now) = tally_previous_literal_state(state, window, symbols) else {
                    return need_more;
                };
                if flush_now {
                    flush_tree_window_block_state(state, pending_and_symbols, window, 0);
                }
                bflush = flush_now as ::core::ffi::c_int;
                if bflush != 0 {
                    state.block_start = state.strstart as ::core::ffi::c_long;
                }
                state.strstart = state.strstart.wrapping_add(1);
                state.lookahead = state.lookahead.wrapping_sub(1);
            } else if !handled_previous_match {
                state.match_available = 1 as ::core::ffi::c_int;
                state.strstart = state.strstart.wrapping_add(1);
                state.lookahead = state.lookahead.wrapping_sub(1);
            } else if bflush != 0 {
                state.block_start = state.strstart as ::core::ffi::c_long;
            }
        }
        if bflush != 0 {
            let avail_out =
                { flush_pending(strm, s, pending_buf, output, FlushPendingMark::Never).0 };
            if let Some(result) = deflate_post_flush_result(avail_out, false) {
                return result;
            }
            if avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
            // The legacy branches consumed this flag at their individual
            // flush sites. The shared post-transition flush needs the
            // equivalent reset before the next parser iteration.
            bflush = 0;
        }
    }
    // At end of input the deferred literal, final block, and ordinary
    // symbol tail all share the same callback-owned window and pending
    // allocation. Lend each once, derive symbols from their documented
    // pending offset, and finish the tree work before `flush_pending()`
    // can revisit the ABI output cursor.
    let tail_last = {
        let state = deflate_reborrow_mut(s);
        let match_available = state.match_available != 0;
        let needs_flush = flush == crate::zlib_h::Z_FINISH || state.sym_next != 0;
        if match_available || needs_flush {
            let (Ok(window_len), Ok(pending_len)) = (
                usize::try_from(state.window_size),
                usize::try_from(state.pending_buf_size),
            ) else {
                return need_more;
            };
            if (window_len != 0 && state.window.is_none())
                || window_hash.window.len() != window_len
                || pending_buf.len() != pending_len
            {
                return need_more;
            }
            let window = &*window_hash.window;
            let pending_and_symbols = &mut *pending_buf;
            if match_available {
                let Ok(symbol_start) = usize::try_from(state.lit_bufsize) else {
                    return need_more;
                };
                let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                    return need_more;
                };
                let Some(symbol_end) = symbol_start.checked_add(symbol_len) else {
                    return need_more;
                };
                let Some(symbols) = pending_and_symbols.get_mut(symbol_start..symbol_end) else {
                    return need_more;
                };
                let Some(flush_now) = tally_previous_literal_state(state, window, symbols) else {
                    return need_more;
                };
                bflush = flush_now as ::core::ffi::c_int;
                state.match_available = 0 as ::core::ffi::c_int;
            }
            state.insert = if state.strstart
                < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
            {
                state.strstart
            } else {
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
            };
            if flush == crate::zlib_h::Z_FINISH {
                flush_tree_window_block_state(state, pending_and_symbols, window, 1);
                Some(true)
            } else if state.sym_next != 0 {
                flush_tree_window_block_state(state, pending_and_symbols, window, 0);
                Some(false)
            } else {
                None
            }
        } else {
            state.insert = if state.strstart
                < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
            {
                state.strstart
            } else {
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
            };
            None
        }
    };
    if let Some(last) = tail_last {
        let avail_out = {
            s.block_start = s.strstart as ::core::ffi::c_long;
            flush_pending(strm, s, pending_buf, output, FlushPendingMark::Never).0
        };
        if let Some(result) = deflate_post_flush_result(avail_out, last) {
            return result;
        }
        if last {
            return finish_done;
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
    s: &crate::src::deflate::deflate_state,
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

/// Decide the common RLE/Huffman tail operation while committing the one
/// parser-state update that must happen before either kind of final block.
/// The callers keep the tree call and pending-output boundary separate: both
/// still require the legacy callback-owned buffers.
enum DeflateTailAction {
    Finish,
    FlushSymbols,
    Done,
}

fn deflate_tail_action_state(
    s: &mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> DeflateTailAction {
    s.insert = 0;
    if flush == crate::zlib_h::Z_FINISH {
        DeflateTailAction::Finish
    } else if s.sym_next != 0 {
        DeflateTailAction::FlushSymbols
    } else {
        DeflateTailAction::Done
    }
}

/// Map the output capacity left by a pending-buffer flush to the block-mode
/// result.  This is deliberately scalar-only: the legacy modes still perform
/// the pending-buffer work at their existing compatibility boundary, while
/// the shared status decision cannot accidentally re-adopt a raw stream.
fn deflate_post_flush_result(
    avail_out: crate::stdlib::uInt,
    finishing: bool,
) -> Option<block_state> {
    if avail_out == 0 {
        Some(if finishing { finish_started } else { need_more })
    } else {
        None
    }
}

/// Describe the wrapper trailer that a completed deflate stream must append.
///
/// The dispatcher still owns the callback-allocated pending buffer, but the
/// wrapper choice and zlib's big-endian checksum split are plain scalar
/// policy.  Keeping them here prevents the transitional boundary from mixing
/// raw stream access with trailer arithmetic.
#[derive(Copy, Clone)]
enum DeflateTrailerPlan {
    None,
    Gzip {
        check: crate::stdlib::uLong,
        total_in: crate::stdlib::uLong,
    },
    Zlib {
        check_high: crate::stdlib::uInt,
        check_low: crate::stdlib::uInt,
    },
}

fn deflate_trailer_plan(
    wrap: ::core::ffi::c_int,
    check: crate::stdlib::uLong,
    total_in: crate::stdlib::uLong,
) -> DeflateTrailerPlan {
    if wrap <= 0 {
        DeflateTrailerPlan::None
    } else if wrap == 2 {
        DeflateTrailerPlan::Gzip { check, total_in }
    } else {
        DeflateTrailerPlan::Zlib {
            check_high: (check >> 16) as crate::stdlib::uInt,
            check_low: (check & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
        }
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

fn deflate_rle(
    s: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    input: &mut &[crate::stdlib::Byte],
    pending_buf: &mut [crate::stdlib::Byte],
    window_hash: &mut DeflateWindowHashBuffers<'_>,
    output: &mut DeflateOutput<'_>,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    // The ABI boundary already lent the callback-owned window and pending
    // buffer; RLE parsing stays within those checked spans.
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        let needs_input = s.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt;
        if needs_input {
            fill_window_state(s, strm, input, window_hash);
            if s.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
        }
        let state: &mut crate::src::deflate::deflate_state = s;
        if state.lookahead == 0 as crate::stdlib::uInt
            && !matches!(
                deflate_tail_action_state(state, flush),
                DeflateTailAction::Finish | DeflateTailAction::FlushSymbols
            )
        {
            return block_done;
        }
        let Ok(window_len) = usize::try_from(state.window_size) else {
            return need_more;
        };
        if (window_len != 0 && state.window.is_none()) || window_hash.window.len() != window_len {
            return need_more;
        }
        let window = &*window_hash.window;
        let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
            return need_more;
        };
        if pending_buf.len() != pending_len {
            return need_more;
        }
        // `sym_start` is the literal-buffer offset inside `pending_buf`.
        // Keep this existing callback-owned lend alive for a possible
        // checked tree flush instead of re-entering its raw adapter.
        let pending_and_symbols = &mut *pending_buf;
        if state.lookahead == 0 as crate::stdlib::uInt {
            let tail_action = deflate_tail_action_state(state, flush);
            if matches!(tail_action, DeflateTailAction::Finish) {
                flush_tree_window_block_state(state, pending_and_symbols, window, 1);
                let avail_out = {
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
                };
                if let Some(result) = deflate_post_flush_result(avail_out, true) {
                    return result;
                }
                return finish_done;
            }
            if matches!(tail_action, DeflateTailAction::FlushSymbols) {
                flush_tree_window_block_state(state, pending_and_symbols, window, 0);
                let avail_out = {
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
                };
                if let Some(result) = deflate_post_flush_result(avail_out, false) {
                    return result;
                }
            }
            return block_done;
        }
        let Some(symbol) = rle_symbol_plan_state(state, window) else {
            return need_more;
        };
        let Ok(symbol_start) = usize::try_from(state.lit_bufsize) else {
            return need_more;
        };
        let Ok(symbol_len) = usize::try_from(state.sym_end) else {
            return need_more;
        };
        let Some(symbol_end) = symbol_start.checked_add(symbol_len) else {
            return need_more;
        };
        let Some(symbols) = pending_and_symbols.get_mut(symbol_start..symbol_end) else {
            return need_more;
        };
        let Some(flush_now) = rle_tally_symbol_state(state, symbols, symbol) else {
            return need_more;
        };
        bflush = flush_now as ::core::ffi::c_int;
        if bflush != 0 {
            flush_tree_window_block_state(state, pending_and_symbols, window, 0);
            let avail_out = {
                state.block_start = state.strstart as ::core::ffi::c_long;
                flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
            };
            if let Some(result) = deflate_post_flush_result(avail_out, false) {
                return result;
            }
        }
    }
}

fn deflate_huff(
    s: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    input: &mut &[crate::stdlib::Byte],
    pending_buf: &mut [crate::stdlib::Byte],
    window_hash: &mut DeflateWindowHashBuffers<'_>,
    output: &mut DeflateOutput<'_>,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    // The caller already lends the callback-owned window and pending buffer.
    // Keep this parser entirely in those bounded views, so ordinary dispatch
    // does not inherit a raw-pointer compatibility boundary.
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        let needs_input = s.lookahead == 0 as crate::stdlib::uInt;
        if needs_input {
            fill_window_state(s, strm, input, window_hash);
            if s.lookahead == 0 as crate::stdlib::uInt && flush == crate::zlib_h::Z_NO_FLUSH {
                return need_more;
            }
        }
        let state: &mut crate::src::deflate::deflate_state = s;
        let Ok(window_len) = usize::try_from(state.window_size) else {
            return need_more;
        };
        if window_len != 0 && state.window.is_none() {
            return need_more;
        }
        if window_hash.window.len() != window_len {
            return need_more;
        }
        let window = &*window_hash.window;
        let Ok(pending_len) = usize::try_from(state.pending_buf_size) else {
            return need_more;
        };
        if pending_buf.len() != pending_len {
            return need_more;
        }
        // `sym_start` is initialized as the literal-buffer offset within
        // `pending_buf`. Lend that one callback-owned allocation once,
        // then split its symbol storage with checked slice arithmetic.
        let pending_and_symbols = &mut *pending_buf;
        if state.lookahead == 0 as crate::stdlib::uInt {
            let tail_action = deflate_tail_action_state(state, flush);
            if matches!(tail_action, DeflateTailAction::Finish) {
                flush_tree_window_block_state(state, pending_and_symbols, window, 1);
                let avail_out = {
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
                };
                if let Some(result) = deflate_post_flush_result(avail_out, true) {
                    return result;
                }
                return finish_done;
            }
            if matches!(tail_action, DeflateTailAction::FlushSymbols) {
                flush_tree_window_block_state(state, pending_and_symbols, window, 0);
                let avail_out = {
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
                };
                if let Some(result) = deflate_post_flush_result(avail_out, false) {
                    return result;
                }
            }
            return block_done;
        }
        let Some(literal) = huff_literal_plan_state(state, window) else {
            return need_more;
        };
        {
            let Ok(symbol_start) = usize::try_from(state.lit_bufsize) else {
                return need_more;
            };
            let Ok(symbol_len) = usize::try_from(state.sym_end) else {
                return need_more;
            };
            let Some(symbol_end) = symbol_start.checked_add(symbol_len) else {
                return need_more;
            };
            let Some(symbols) = pending_and_symbols.get_mut(symbol_start..symbol_end) else {
                return need_more;
            };
            let Some(flush_now) = huff_tally_literal_state(state, symbols, literal) else {
                return need_more;
            };
            bflush = flush_now as ::core::ffi::c_int;
        }
        if bflush != 0 {
            flush_tree_window_block_state(state, pending_and_symbols, window, 0);
            let avail_out = {
                state.block_start = state.strstart as ::core::ffi::c_long;
                flush_pending(strm, state, pending_buf, output, FlushPendingMark::Never).0
            };
            if let Some(result) = deflate_post_flush_result(avail_out, false) {
                return result;
            }
        }
    }
}
