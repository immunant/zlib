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
#[repr(C)]

pub struct ct_data_s {
    pub fc: crate::zutil_h::ush,
    pub dl: crate::zutil_h::ush,
}

pub type static_tree_desc = crate::src::deflate::static_tree_desc_s;

pub type tree_desc = crate::src::deflate::tree_desc_s;
pub enum TreeKind {
    LitLen,
    Dist,
    BitLen,
}

pub struct tree_desc_s {
    pub kind: TreeKind,
    pub max_code: ::core::ffi::c_int,
}

pub type Pos = crate::zutil_h::ush;

pub type Posf = crate::src::deflate::Pos;

pub type IPos = ::core::ffi::c_uint;

pub type deflate_state = crate::src::deflate::internal_state;

// `deflateSetHeader()` registers header data for later deflate calls.  Keep a
// byte-exact owned snapshot instead of retaining a foreign header pointer
// across those calls.  The C-facing conversion is confined to the setter;
// the compressor and bound calculation only inspect this pointer-free form.
struct GzipHeader {
    text: ::core::ffi::c_int,
    time: crate::stdlib::uLong,
    os: ::core::ffi::c_int,
    extra_len: crate::stdlib::uInt,
    extra: Option<Box<[u8]>>,
    name: Option<Box<[u8]>>,
    comment: Option<Box<[u8]>>,
    hcrc: bool,
}

fn copy_gzip_header(header: &GzipHeader) -> GzipHeader {
    GzipHeader {
        text: header.text,
        time: header.time,
        os: header.os,
        extra_len: header.extra_len,
        extra: header.extra.as_deref().map(Into::into),
        name: header.name.as_deref().map(Into::into),
        comment: header.comment.as_deref().map(Into::into),
        hcrc: header.hcrc,
    }
}

#[repr(C)]

pub struct internal_state {
    // A deflate state is only installed after its ABI stream has been
    // validated, and `deflateCopy()` rejects a null destination.  Preserve
    // that invariant in the opaque state instead of retaining a nullable raw
    // backlink.
    pub strm: ::core::ptr::NonNull<crate::zlib_h::z_stream_s>,
    pub status: ::core::ffi::c_int,
    // This allocation is released through the stream's zfree callback, so it
    // cannot yet become a Box. NonNull makes the initialized-owner invariant
    // explicit without retaining a raw pointer in the state layout.
    pub pending_buf: Option<::core::ptr::NonNull<crate::stdlib::Bytef>>,
    pub pending_buf_size: crate::zutil_h::ulg,
    // Cursor within `pending_buf`.  Keep this as an offset so copying/resetting a
    // stream never retains an interior raw pointer into the allocation.
    pub pending_out: usize,
    pub pending: crate::zutil_h::ulg,
    pub wrap: ::core::ffi::c_int,
    gzhead: Option<GzipHeader>,
    pub gzindex: usize,
    pub method: crate::stdlib::Byte,
    pub last_flush: ::core::ffi::c_int,
    pub w_size: crate::stdlib::uInt,
    pub w_bits: crate::stdlib::uInt,
    pub w_mask: crate::stdlib::uInt,
    // Like the pending allocation, the window is callback-owned.  Its null
    // initialization/failure state is explicit instead of living in a raw
    // pointer field.
    pub window: Option<::core::ptr::NonNull<crate::stdlib::Bytef>>,
    pub window_size: crate::zutil_h::ulg,
    // These hash tables share the callback-owned allocation model used by the
    // window and pending buffer.  Keep allocation failure explicit instead of
    // storing nullable raw pointers in the state.
    pub prev: Option<::core::ptr::NonNull<crate::src::deflate::Posf>>,
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
    // Offset of the symbol region within `pending_buf`.  The C translation
    // retained this as an interior pointer; keeping an offset prevents a
    // copied/reset state from carrying a pointer into another allocation.
    pub sym_buf_start: usize,
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

pub use crate::src::adler32::adler32;
pub use crate::src::crc32::crc32;
pub use crate::src::crc32::crc32_z;
pub use crate::src::trees::_dist_code;
pub use crate::src::trees::_length_code;
pub use crate::src::trees::_tr_flush_block;
pub use crate::src::trees::_tr_stored_block;
pub use crate::src::trees::tr_init;
pub use crate::src::zutil::z_errmsg;
pub use crate::src::zutil::zcalloc;
pub use crate::src::zutil::zcfree;
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

#[derive(PartialEq, Eq)]
pub enum DeflateAlgorithm {
    Stored,
    Fast,
    Slow,
}

impl Copy for DeflateAlgorithm {}

impl Clone for DeflateAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}

pub type config = config_s;
#[repr(C)]

pub struct config_s {
    pub good_length: crate::zutil_h::ush,
    pub max_lazy: crate::zutil_h::ush,
    pub nice_length: crate::zutil_h::ush,
    pub max_chain: crate::zutil_h::ush,
    pub algorithm: DeflateAlgorithm,
}
const fn c_char_bytes(bytes: [u8; 70]) -> [::core::ffi::c_char; 70] {
    let mut chars = [0; 70];
    let mut index = 0;
    while index < bytes.len() {
        chars[index] = bytes[index] as ::core::ffi::c_char;
        index += 1;
    }
    chars
}

#[no_mangle]
pub static deflate_copyright: [::core::ffi::c_char; 70] =
    c_char_bytes(*b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0");

pub const NIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TOO_FAR: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

static configuration_table: [config; 10] = [
    config_s {
        good_length: 0 as crate::zutil_h::ush,
        max_lazy: 0 as crate::zutil_h::ush,
        nice_length: 0 as crate::zutil_h::ush,
        max_chain: 0 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Stored,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
];

// The allocation geometry is pure policy: it depends only on the public
// initialization arguments and does not need access to the ABI stream or the
// raw state allocation.  Keep it separate so the eventual owner-backed state
// can retain the exact zlib validation and capacities without reproducing the
// boundary projection.
struct DeflateLayout {
    level: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
    w_bits: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    hash_size: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    lit_bufsize: crate::stdlib::uInt,
}

// Keep the allocator-facing shape separate from the ABI-shaped state.  These
// are the exact `(items, size)` requests made to a custom zalloc callback, so
// later owner-backed storage can preserve allocation pairing and timing rather
// than collapsing the four buffers into an unobservable byte count.
struct DeflateAllocation {
    items: crate::stdlib::uInt,
    size: crate::stdlib::uInt,
}

impl DeflateAllocation {
    // Keep the allocation request and its view capacity together. The
    // callback still receives the original `(items, size)` pair, while a
    // future owner-backed allocation broker can use this checked capacity to
    // construct its bounded byte view without re-deriving it from ABI state.
    fn byte_len(&self) -> Option<usize> {
        (self.items as usize).checked_mul(self.size as usize)
    }

    // Typed views must agree with the callback's exact element size. Keep
    // that check in the pointer-free allocation plan so later owned storage
    // does not have to recover element counts from raw copy byte lengths.
    fn element_len<T>(&self) -> Option<usize> {
        (self.size as usize == ::core::mem::size_of::<T>()).then_some(self.items as usize)
    }
}

struct DeflateStorageLayout {
    window: DeflateAllocation,
    prev: DeflateAllocation,
    head: DeflateAllocation,
    pending: DeflateAllocation,
}

impl DeflateLayout {
    fn storage(&self) -> DeflateStorageLayout {
        DeflateStorageLayout::new(self.w_size, self.hash_size, self.lit_bufsize)
    }
}

impl DeflateStorageLayout {
    fn new(
        w_size: crate::stdlib::uInt,
        hash_size: crate::stdlib::uInt,
        lit_bufsize: crate::stdlib::uInt,
    ) -> Self {
        Self {
            window: DeflateAllocation {
                items: w_size,
                size: (2 * ::core::mem::size_of::<crate::stdlib::Byte>()) as crate::stdlib::uInt,
            },
            prev: DeflateAllocation {
                items: w_size,
                size: ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
            },
            head: DeflateAllocation {
                items: hash_size,
                size: ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
            },
            pending: DeflateAllocation {
                items: lit_bufsize,
                size: 4,
            },
        }
    }
}

fn deflate_layout(
    level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    mut window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Option<DeflateLayout> {
    let level = if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        6
    } else {
        level
    };
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
        || window_bits == 8 && wrap != 1
    {
        return None;
    }
    if window_bits == 8 {
        window_bits = 9;
    }
    let w_bits = window_bits as crate::stdlib::uInt;
    let w_size = (1 << w_bits) as crate::stdlib::uInt;
    let hash_bits = (mem_level as crate::stdlib::uInt).wrapping_add(7);
    let hash_size = (1 << hash_bits) as crate::stdlib::uInt;
    let lit_bufsize = (1 << (mem_level + 6)) as crate::stdlib::uInt;
    Some(DeflateLayout {
        level,
        wrap,
        w_bits,
        w_size,
        w_mask: w_size.wrapping_sub(1),
        hash_bits,
        hash_size,
        hash_mask: hash_size.wrapping_sub(1),
        hash_shift: hash_bits
            .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
            .wrapping_sub(1)
            .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt),
        lit_bufsize,
    })
}

fn slide_hash_table(table: &mut [crate::src::deflate::Posf], wsize: crate::stdlib::uInt) {
    for entry in table.iter_mut().rev() {
        let position = *entry as ::core::ffi::c_uint;
        *entry = (if position >= wsize {
            position.wrapping_sub(wsize as ::core::ffi::c_uint)
        } else {
            NIL as ::core::ffi::c_uint
        }) as crate::src::deflate::Pos as crate::src::deflate::Posf;
    }
}

fn clear_hash_table(table: &mut [crate::src::deflate::Posf]) {
    table.fill(NIL as crate::src::deflate::Posf);
}

fn slide_window_bytes(
    window: &mut [crate::stdlib::Bytef],
    wsize: crate::stdlib::uInt,
    more: ::core::ffi::c_uint,
) {
    let start = wsize as usize;
    let count = wsize.wrapping_sub(more) as usize;
    window.copy_within(start..start + count, 0);
}

fn clear_window_bytes(
    window: &mut [crate::stdlib::Bytef],
    start: crate::zutil_h::ulg,
    len: crate::zutil_h::ulg,
) {
    let start = start as usize;
    window[start..start + len as usize].fill(0);
}

// Keep high-water initialization independent of the ABI state.  The caller
// supplies the one bounded window view and writes the returned cursor back to
// its opaque state.
fn initialize_window_high_water(
    window: &mut [crate::stdlib::Bytef],
    high_water: crate::zutil_h::ulg,
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::zutil_h::ulg {
    let window_size = window.len() as crate::zutil_h::ulg;
    let curr = (strstart as crate::zutil_h::ulg).wrapping_add(lookahead as crate::zutil_h::ulg);
    if high_water < curr {
        let mut init = window_size.wrapping_sub(curr);
        if init > crate::src::deflate::WIN_INIT as crate::zutil_h::ulg {
            init = crate::src::deflate::WIN_INIT as crate::zutil_h::ulg;
        }
        clear_window_bytes(window, curr, init);
        curr.wrapping_add(init)
    } else if high_water < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg) {
        let mut init = curr
            .wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
            .wrapping_sub(high_water);
        if init > window_size.wrapping_sub(high_water) {
            init = window_size.wrapping_sub(high_water);
        }
        clear_window_bytes(window, high_water, init);
        high_water.wrapping_add(init)
    } else {
        high_water
    }
}

fn read_buf_checksum(
    checksum: crate::stdlib::uLong,
    wrap: ::core::ffi::c_int,
    bytes: &[crate::stdlib::Bytef],
) -> crate::stdlib::uLong {
    match wrap {
        1 => crate::src::adler32::adler32(checksum, bytes),
        2 => crate::src::crc32::crc32_z(checksum, Some(bytes)),
        _ => checksum,
    }
}

fn read_buf_bytes(
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
    wrap: ::core::ffi::c_int,
) -> crate::stdlib::uLong {
    output.copy_from_slice(input);
    read_buf_checksum(checksum, wrap, output)
}

unsafe extern "C" fn fill_window(mut s: *mut crate::src::deflate::deflate_state) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    // Keep the ABI-state projection at this boundary.  The work below uses
    // the checked slice helpers and this single scoped state view instead of
    // repeatedly dereferencing the raw state cursor.
    let state = &mut *s;
    let mut wsize: crate::stdlib::uInt = state.w_size;
    loop {
        more = state
            .window_size
            .wrapping_sub(state.lookahead as crate::zutil_h::ulg)
            .wrapping_sub(state.strstart as crate::zutil_h::ulg)
            as ::core::ffi::c_uint;
        if ::core::mem::size_of::<::core::ffi::c_int>() <= 2 as usize {
            if more == 0 as ::core::ffi::c_uint
                && state.strstart == 0 as crate::stdlib::uInt
                && state.lookahead == 0 as crate::stdlib::uInt
            {
                more = wsize as ::core::ffi::c_uint;
            } else if more == -1 as ::core::ffi::c_int as ::core::ffi::c_uint {
                more = more.wrapping_sub(1);
            }
        }
        if state.strstart
            >= wsize.wrapping_add(
                state
                    .w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            // `window` is allocated with exactly `window_size` bytes in
            // `deflateInit2_()` and `deflateCopy()`.
            let window = ::core::slice::from_raw_parts_mut(
                state.window.expect("initialized window").as_ptr(),
                state.window_size as usize,
            );
            slide_window_bytes(window, wsize, more);
            state.match_start = state.match_start.wrapping_sub(wsize);
            state.strstart = state.strstart.wrapping_sub(wsize);
            state.block_start -= wsize as ::core::ffi::c_long;
            if state.insert > state.strstart {
                state.insert = state.strstart;
            }
            // `head` and `prev` are allocated at these exact element counts
            // in `deflateInit2_()` and `deflateCopy()`.
            let head = ::core::slice::from_raw_parts_mut(
                state.head.expect("initialized head table").as_ptr(),
                state.hash_size as usize,
            );
            let prev = ::core::slice::from_raw_parts_mut(
                state.prev.expect("initialized prev table").as_ptr(),
                wsize as usize,
            );
            slide_hash_table(head, wsize);
            slide_hash_table(prev, wsize);
            state.slid = 1 as ::core::ffi::c_int;
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if (&*state.strm.as_ptr()).avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        let stream = &mut *state.strm.as_ptr();
        n = stream.avail_in.min(more);
        if n != 0 {
            stream.avail_in = stream.avail_in.wrapping_sub(n);
            let input = ::core::slice::from_raw_parts(stream.next_in, n as usize);
            let next_in = input.as_ptr_range().end.cast_mut();
            // `window` is allocated with exactly `window_size` bytes in
            // `deflateInit2_()` and `deflateCopy()`, and this write is bounded
            // by the `more` capacity calculated above.
            let window = ::core::slice::from_raw_parts_mut(
                state.window.expect("initialized window").as_ptr(),
                state.window_size as usize,
            );
            let start = state.strstart.wrapping_add(state.lookahead) as usize;
            let output = &mut window[start..start + n as usize];
            stream.adler = read_buf_bytes(input, output, stream.adler, state.wrap);
            stream.next_in = next_in;
            stream.total_in = stream.total_in.wrapping_add(n as crate::stdlib::uLong);
        }
        state.lookahead = state.lookahead.wrapping_add(n);
        if state.lookahead.wrapping_add(state.insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        {
            let mut str: crate::stdlib::uInt = state.strstart.wrapping_sub(state.insert);
            // These are the exact capacities allocated by `deflateInit2_()`
            // and `deflateCopy()`. Keep the raw views local to this update,
            // rather than repeatedly indexing through the raw cursors.
            let window = ::core::slice::from_raw_parts(
                state.window.expect("initialized window").as_ptr(),
                state.window_size as usize,
            );
            let prev = ::core::slice::from_raw_parts_mut(
                state.prev.expect("initialized prev table").as_ptr(),
                wsize as usize,
            );
            let head = ::core::slice::from_raw_parts_mut(
                state.head.expect("initialized head table").as_ptr(),
                state.hash_size as usize,
            );
            state.ins_h = window[str as usize] as crate::stdlib::uInt;
            state.ins_h = (state.ins_h << state.hash_shift
                ^ window[str.wrapping_add(1 as crate::stdlib::uInt) as usize]
                    as crate::stdlib::uInt)
                & state.hash_mask;
            while state.insert != 0 {
                state.ins_h = (state.ins_h << state.hash_shift
                    ^ window[str
                        .wrapping_add(3 as crate::stdlib::uInt)
                        .wrapping_sub(1 as crate::stdlib::uInt)
                        as usize] as crate::stdlib::uInt)
                    & state.hash_mask;
                prev[(str & state.w_mask) as usize] = head[state.ins_h as usize];
                head[state.ins_h as usize] =
                    str as crate::src::deflate::Pos as crate::src::deflate::Posf;
                str = str.wrapping_add(1);
                state.insert = state.insert.wrapping_sub(1);
                if state.lookahead.wrapping_add(state.insert)
                    < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                {
                    break;
                }
            }
        }
        if !(state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
            && (&*state.strm.as_ptr()).avail_in != 0 as crate::stdlib::uInt)
        {
            break;
        }
    }
    if state.high_water < state.window_size {
        // `window` has exactly `window_size` bytes by construction.  Form
        // that bounded view once; the initialization policy itself is fully
        // pointer-free.
        let window = ::core::slice::from_raw_parts_mut(
            state.window.expect("initialized window").as_ptr(),
            state.window_size as usize,
        );
        state.high_water =
            initialize_window_high_water(window, state.high_water, state.strstart, state.lookahead);
    }
}
pub unsafe extern "C" fn deflateInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return deflateInit2_(
        strm,
        level,
        crate::zlib_h::Z_DEFLATED,
        crate::stdlib::MAX_WBITS,
        crate::zutil_h::DEF_MEM_LEVEL,
        crate::zlib_h::Z_DEFAULT_STRATEGY,
        version,
        stream_size,
    );
}
#[export_name = "deflateInit_"]

pub unsafe extern "C" fn deflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateInit_(strm, level, version, stream_size)
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
    static my_version: [::core::ffi::c_char; 15] = crate::zlib_h::ZLIB_VERSION;
    if version.is_null()
        || *version.offset(0 as isize) as ::core::ffi::c_int
            != my_version[0 as usize] as ::core::ffi::c_int
        || stream_size as usize != ::core::mem::size_of::<crate::zlib_h::z_stream>()
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*strm).zalloc.is_none() {
        (*strm).zalloc = Some(
            crate::src::zutil::zcalloc
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
            crate::src::zutil::zcfree
                as unsafe extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> (),
        ) as crate::zlib_h::free_func;
    }
    let Some(layout) = deflate_layout(level, method, windowBits, memLevel, strategy) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let storage = layout.storage();
    s = Some((*strm).zalloc.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::deflate_state;
    if s.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    ::core::ptr::write_bytes(s, 0, 1);
    (*strm).state = s as *mut crate::src::deflate::internal_state;
    (*s).strm = ::core::ptr::NonNull::new(strm).expect("validated stream");
    (*s).status = crate::src::deflate::INIT_STATE;
    (*s).wrap = layout.wrap;
    (*s).gzhead = None;
    (*s).w_bits = layout.w_bits;
    (*s).w_size = layout.w_size;
    (*s).w_mask = layout.w_mask;
    (*s).hash_bits = layout.hash_bits;
    (*s).hash_size = layout.hash_size;
    (*s).hash_mask = layout.hash_mask;
    (*s).hash_shift = layout.hash_shift;
    (*s).window =
        ::core::ptr::NonNull::new(Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            storage.window.items,
            storage.window.size,
        ) as *mut crate::stdlib::Bytef);
    (*s).prev = ::core::ptr::NonNull::new(Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque, storage.prev.items, storage.prev.size
    ) as *mut crate::src::deflate::Posf);
    (*s).head = ::core::ptr::NonNull::new(Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque, storage.head.items, storage.head.size
    ) as *mut crate::src::deflate::Posf);
    (*s).high_water = 0 as crate::zutil_h::ulg;
    (*s).lit_bufsize = layout.lit_bufsize;
    (*s).pending_buf =
        ::core::ptr::NonNull::new(Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            storage.pending.items,
            storage.pending.size,
        ) as *mut crate::zutil_h::uchf
            as *mut crate::stdlib::Bytef);
    (*s).pending_buf_size = storage
        .pending
        .byte_len()
        .expect("validated pending allocation geometry")
        as crate::zutil_h::ulg;
    if (*s).window.is_none()
        || (*s).prev.is_none()
        || (*s).head.is_none()
        || (*s).pending_buf.is_none()
    {
        (*s).status = crate::src::deflate::FINISH_STATE;
        (*strm).msg = crate::src::zutil::z_errmsg[(if (-4 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -4 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -4 as ::core::ffi::c_int
        }) as usize]
            .load(::core::sync::atomic::Ordering::Relaxed);
        deflateEnd(strm);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*s).sym_buf_start = (*s).lit_bufsize as usize;
    (*s).sym_end = (*s)
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    (*s).level = layout.level;
    (*s).strategy = strategy;
    (*s).method = method as crate::stdlib::Byte;
    return deflateReset(strm);
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
unsafe extern "C" fn deflateStateCheck(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let strm_ref = &*strm;
    if strm_ref.zalloc.is_none() || strm_ref.zfree.is_none() {
        return 1 as ::core::ffi::c_int;
    }
    let s = strm_ref.state as *mut crate::src::deflate::deflate_state;
    if s.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let state = &*s;
    if state.strm.as_ptr() != strm
        || state.status != crate::src::deflate::INIT_STATE
            && state.status != crate::src::deflate::GZIP_STATE
            && state.status != crate::src::deflate::EXTRA_STATE
            && state.status != crate::src::deflate::NAME_STATE
            && state.status != crate::src::deflate::COMMENT_STATE
            && state.status != crate::src::deflate::HCRC_STATE
            && state.status != crate::src::deflate::BUSY_STATE
            && state.status != crate::src::deflate::FINISH_STATE
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

// Insert the initial dictionary strings into the hash chains.  The caller
// supplies bounded views of the state allocations, so this kernel can keep
// the cursor arithmetic and table updates entirely pointer-free.
fn insert_dictionary_hashes(
    window: &[crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    mut ins_h: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    mut str: crate::stdlib::uInt,
    count: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::stdlib::uInt) {
    for _ in 0..count {
        let byte_index = str
            .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
            .wrapping_sub(1) as usize;
        ins_h = ((ins_h << hash_shift) ^ window[byte_index] as crate::stdlib::uInt) & hash_mask;
        let prev_index = (str & w_mask) as usize;
        let head_index = ins_h as usize;
        prev[prev_index] = head[head_index];
        head[head_index] = str as crate::src::deflate::Pos as crate::src::deflate::Posf;
        str = str.wrapping_add(1);
    }
    (str, ins_h)
}

// Insert one current string into the hash chains through bounded allocation
// views.  The deflate modes retain their state/cursor policy, while this
// kernel owns the byte and table indexing.
fn insert_hash(
    window: &[crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    ins_h: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::src::deflate::IPos) {
    let next_hash = ((ins_h << hash_shift)
        ^ window
            [strstart.wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt - 1) as usize]
            as crate::stdlib::uInt)
        & hash_mask;
    let prev_index = (strstart & w_mask) as usize;
    let head_index = next_hash as usize;
    prev[prev_index] = head[head_index];
    let hash_head = prev[prev_index] as crate::src::deflate::IPos;
    head[head_index] = strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
    (next_hash, hash_head)
}

// The slow matcher emits either a length/distance pair or one delayed literal
// into the deferred-symbol suffix of the pending allocation.  Keep the tree
// accounting itself independent of the ABI-backed deflate state so the block
// algorithm only has to project the bounded symbol view at its boundary.
fn tally_slow_symbol(
    sym_buf: &mut [crate::stdlib::Bytef],
    sym_next: &mut crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    dyn_ltree: &mut [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &mut [crate::src::deflate::ct_data_s; 61],
    matches: &mut crate::stdlib::uInt,
    dist: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    crate::src::trees::tally_symbol(
        sym_buf, sym_next, sym_end, dyn_ltree, dyn_dtree, matches, dist, len,
    )
}

// The lazy matcher keeps a one-byte delayed decision.  Its policy and symbol
// accounting are entirely scalar/slice based; only the caller performs the
// raw allocation projections needed to refill, search, or flush a block.
struct SlowMatchState {
    prev_length: crate::stdlib::uInt,
    match_length: crate::stdlib::uInt,
    prev_match: crate::src::deflate::IPos,
    match_available: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
}

enum SlowMatchAction {
    Match {
        insert_start: crate::stdlib::uInt,
        insert_count: crate::stdlib::uInt,
        bflush: ::core::ffi::c_int,
    },
    Literal(::core::ffi::c_int),
    Defer,
}

fn advance_slow_match(
    state: &mut SlowMatchState,
    sym_buf: &mut [crate::stdlib::Bytef],
    sym_next: &mut crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    dyn_ltree: &mut [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &mut [crate::src::deflate::ct_data_s; 61],
    matches: &mut crate::stdlib::uInt,
    delayed_literal: Option<crate::zutil_h::uch>,
) -> SlowMatchAction {
    if state.prev_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        && state.match_length <= state.prev_length
    {
        let max_insert = state
            .strstart
            .wrapping_add(state.lookahead)
            .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
        let len = state.prev_length.wrapping_sub(3) as crate::zutil_h::uch;
        let dist = (state.strstart as crate::src::deflate::IPos)
            .wrapping_sub(1)
            .wrapping_sub(state.prev_match) as crate::zutil_h::ush;
        let bflush = tally_slow_symbol(
            sym_buf,
            sym_next,
            sym_end,
            dyn_ltree,
            dyn_dtree,
            matches,
            dist as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
        );
        let insert_count = state
            .prev_length
            .wrapping_sub(2)
            .min(max_insert.wrapping_sub(state.strstart));
        let insert_start = state.strstart.wrapping_add(1);
        state.lookahead = state
            .lookahead
            .wrapping_sub(state.prev_length.wrapping_sub(1));
        state.strstart = state.strstart.wrapping_add(state.prev_length.wrapping_sub(1));
        state.prev_length = 0;
        state.match_available = 0;
        state.match_length = crate::zutil_h::MIN_MATCH as crate::stdlib::uInt - 1;
        SlowMatchAction::Match {
            insert_start,
            insert_count,
            bflush,
        }
    } else if state.match_available != 0 {
        let bflush = tally_slow_symbol(
            sym_buf,
            sym_next,
            sym_end,
            dyn_ltree,
            dyn_dtree,
            matches,
            0,
            delayed_literal.expect("delayed literal is available") as ::core::ffi::c_uint,
        );
        SlowMatchAction::Literal(bflush)
    } else {
        state.match_available = 1;
        state.strstart = state.strstart.wrapping_add(1);
        state.lookahead = state.lookahead.wrapping_sub(1);
        SlowMatchAction::Defer
    }
}

fn initial_hash(
    window: &[crate::stdlib::Bytef],
    strstart: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    (((window[strstart as usize] as crate::stdlib::uInt) << hash_shift)
        ^ window[strstart.wrapping_add(1) as usize] as crate::stdlib::uInt)
        & hash_mask
}

// The dictionary setter only needs a small scalar portion of the deflate
// state once its three callback-owned allocations have been projected.  Keep
// that policy separate so dictionary copying and hash construction never
// need to touch ABI stream cursors or raw allocations.
struct DictionaryState {
    wrap: ::core::ffi::c_int,
    status: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    slid: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    insert: crate::stdlib::uInt,
    ins_h: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    match_length: crate::stdlib::uInt,
    match_available: ::core::ffi::c_int,
    high_water: crate::zutil_h::ulg,
}

fn set_dictionary_core(
    state: &mut DictionaryState,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    dictionary: &[crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
) -> Result<Option<crate::stdlib::uLong>, ()> {
    let wrap = state.wrap;
    if wrap == 2
        || wrap == 1 && state.status != crate::src::deflate::INIT_STATE
        || state.lookahead != 0
    {
        return Err(());
    }
    let checksum = (wrap == 1).then(|| crate::src::adler32::adler32(checksum, dictionary));
    state.wrap = 0;
    let dictionary = if dictionary.len() >= state.w_size as usize {
        if wrap == 0 {
            clear_hash_table(head);
            state.slid = 0;
            state.strstart = 0;
            state.block_start = 0;
            state.insert = 0;
        }
        &dictionary[dictionary.len() - state.w_size as usize..]
    } else {
        dictionary
    };
    window[..dictionary.len()].copy_from_slice(dictionary);
    state.lookahead = dictionary.len() as crate::stdlib::uInt;
    if state.lookahead.wrapping_add(state.insert)
        >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
    {
        state.ins_h = initial_hash(window, state.strstart, state.hash_shift, state.hash_mask);
    }
    while state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        let str = state.strstart;
        let count = state
            .lookahead
            .wrapping_sub((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt);
        (state.strstart, state.ins_h) = insert_dictionary_hashes(
            window,
            head,
            prev,
            state.ins_h,
            state.hash_shift,
            state.hash_mask,
            state.w_mask,
            str,
            count,
        );
        state.lookahead = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
    }
    state.strstart = state.strstart.wrapping_add(state.lookahead);
    state.block_start = state.strstart as ::core::ffi::c_long;
    state.insert = state.lookahead;
    state.lookahead = 0;
    state.prev_length = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
    state.match_length = state.prev_length;
    state.match_available = 0;
    state.high_water = initialize_window_high_water(
        window,
        state.high_water,
        state.strstart,
        state.lookahead,
    );
    state.wrap = wrap;
    Ok(checksum)
}

pub unsafe extern "C" fn deflateSetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 || dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = ::core::slice::from_raw_parts(dictionary, dictLength as usize);
    let strm = &mut *strm;
    let s = &mut *(strm.state as *mut crate::src::deflate::deflate_state);
    let window = ::core::slice::from_raw_parts_mut(
        s.window.expect("initialized window").as_ptr(),
        s.window_size as usize,
    );
    let head = ::core::slice::from_raw_parts_mut(
        s.head.expect("initialized head table").as_ptr(),
        s.hash_size as usize,
    );
    let prev = ::core::slice::from_raw_parts_mut(
        s.prev.expect("initialized prev table").as_ptr(),
        s.w_size as usize,
    );
    let mut state = DictionaryState {
        wrap: s.wrap,
        status: s.status,
        lookahead: s.lookahead,
        w_size: s.w_size,
        slid: s.slid,
        strstart: s.strstart,
        block_start: s.block_start,
        insert: s.insert,
        ins_h: s.ins_h,
        hash_shift: s.hash_shift,
        hash_mask: s.hash_mask,
        w_mask: s.w_mask,
        prev_length: s.prev_length,
        match_length: s.match_length,
        match_available: s.match_available,
        high_water: s.high_water,
    };
    let Ok(checksum) = set_dictionary_core(&mut state, window, head, prev, dictionary, strm.adler)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    s.wrap = state.wrap;
    s.slid = state.slid;
    s.strstart = state.strstart;
    s.block_start = state.block_start;
    s.insert = state.insert;
    s.ins_h = state.ins_h;
    s.lookahead = state.lookahead;
    s.prev_length = state.prev_length;
    s.match_length = state.match_length;
    s.match_available = state.match_available;
    s.high_water = state.high_water;
    if let Some(checksum) = checksum {
        strm.adler = checksum;
    }
    crate::zlib_h::Z_OK
}
#[export_name = "deflateSetDictionary"]

pub unsafe extern "C" fn deflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    deflateSetDictionary(strm, dictionary, dictLength)
}
pub unsafe extern "C" fn deflateGetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut len: crate::stdlib::uInt = 0;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    len = state.strstart.wrapping_add(state.lookahead);
    if len > state.w_size {
        len = state.w_size;
    }
    if !dictionary.is_null() && len != 0 {
        // The window allocation has exactly `window_size` bytes.  The C API
        // supplies a destination large enough for the returned dictionary.
        let window = ::core::slice::from_raw_parts(
            state.window.expect("initialized window").as_ptr(),
            state.window_size as usize,
        );
        let end = state.strstart.wrapping_add(state.lookahead) as usize;
        let len = len as usize;
        let source = &window[end - len..end];
        let output = ::core::slice::from_raw_parts_mut(dictionary, len);
        output.copy_from_slice(source);
    }
    if !dictLength.is_null() {
        *dictLength = len;
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateGetDictionary"]

pub unsafe extern "C" fn deflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    deflateGetDictionary(strm, dictionary, dictLength)
}
pub unsafe extern "C" fn deflateResetKeep(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*strm).total_out = 0 as crate::stdlib::uLong;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*strm).data_type = crate::zlib_h::Z_UNKNOWN;
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    (*s).pending = 0 as crate::zutil_h::ulg;
    (*s).pending_out = 0;
    if (*s).wrap < 0 as ::core::ffi::c_int {
        (*s).wrap = -(*s).wrap;
    }
    (*s).status = if (*s).wrap == 2 as ::core::ffi::c_int {
        crate::src::deflate::GZIP_STATE
    } else {
        crate::src::deflate::INIT_STATE
    };
    (*strm).adler = if (*s).wrap == 2 as ::core::ffi::c_int {
        crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None)
    } else {
        crate::src::adler32::adler32_z(0 as crate::stdlib::uLong, None)
    };
    (*s).last_flush = -2 as ::core::ffi::c_int;
    let state = &mut *s;
    crate::src::trees::tr_init(
        &mut state.dyn_ltree,
        &mut state.dyn_dtree,
        &mut state.bl_tree,
        &mut state.l_desc,
        &mut state.d_desc,
        &mut state.bl_desc,
        &mut state.static_len,
        &mut state.opt_len,
        &mut state.matches,
        &mut state.sym_next,
        &mut state.bi_buf,
        &mut state.bi_valid,
        &mut state.bi_used,
    );
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateResetKeep(strm)
}
pub unsafe extern "C" fn deflateReset(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = deflateResetKeep(strm);
    if ret == crate::zlib_h::Z_OK {
        let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
        state.window_size = (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
            .wrapping_mul(state.w_size as crate::zutil_h::ulg);
        // `head` has exactly `hash_size` elements from `deflateInit2_()` or
        // `deflateCopy()`.
        let head = ::core::slice::from_raw_parts_mut(
            state.head.expect("initialized head table").as_ptr(),
            state.hash_size as usize,
        );
        clear_hash_table(head);
        state.slid = 0 as ::core::ffi::c_int;
        state.max_lazy_match =
            configuration_table[state.level as usize].max_lazy as crate::stdlib::uInt;
        state.good_match =
            configuration_table[state.level as usize].good_length as crate::stdlib::uInt;
        state.nice_match =
            configuration_table[state.level as usize].nice_length as ::core::ffi::c_int;
        state.max_chain_length =
            configuration_table[state.level as usize].max_chain as crate::stdlib::uInt;
        state.strstart = 0 as crate::stdlib::uInt;
        state.block_start = 0 as ::core::ffi::c_long;
        state.lookahead = 0 as crate::stdlib::uInt;
        state.insert = 0 as crate::stdlib::uInt;
        state.prev_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        state.match_length = state.prev_length;
        state.match_available = 0 as ::core::ffi::c_int;
        state.ins_h = 0 as crate::stdlib::uInt;
    }
    return ret;
}
#[export_name = "deflateReset"]

pub unsafe extern "C" fn deflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateReset(strm)
}
pub unsafe extern "C" fn deflateSetHeader(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 || (*(*strm).state).wrap != 2 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let header = if head.is_null() {
        None
    } else {
        let header = &*head;
        let extra_len = header.extra_len & 0xffff as crate::stdlib::uInt;
        Some(GzipHeader {
            text: header.text,
            time: header.time,
            os: header.os,
            extra_len: header.extra_len,
            extra: if header.extra.is_null() {
                None
            } else {
                Some(::core::slice::from_raw_parts(header.extra, extra_len as usize).into())
            },
            name: if header.name.is_null() {
                None
            } else {
                Some(
                    ::std::ffi::CStr::from_ptr(header.name.cast())
                        .to_bytes_with_nul()
                        .into(),
                )
            },
            comment: if header.comment.is_null() {
                None
            } else {
                Some(
                    ::std::ffi::CStr::from_ptr(header.comment.cast())
                        .to_bytes_with_nul()
                        .into(),
                )
            },
            hcrc: header.hcrc != 0,
        })
    };
    (*(*strm).state).gzhead = header;
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    deflateSetHeader(strm, head)
}

fn deflate_pending_impl(
    pending_count: crate::zutil_h::ulg,
    bi_valid: ::core::ffi::c_int,
) -> (::core::ffi::c_uint, ::core::ffi::c_int, ::core::ffi::c_int) {
    let pending = pending_count as ::core::ffi::c_uint;
    if pending as crate::zutil_h::ulg != pending_count {
        (
            -1 as ::core::ffi::c_int as ::core::ffi::c_uint,
            bi_valid,
            crate::zlib_h::Z_BUF_ERROR,
        )
    } else {
        (pending, bi_valid, crate::zlib_h::Z_OK)
    }
}

pub unsafe extern "C" fn deflatePending(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*((*strm).state as *const crate::src::deflate::deflate_state);
    let (pending_value, bits_value, status) = deflate_pending_impl(state.pending, state.bi_valid);
    if !bits.is_null() {
        *bits = bits_value;
    }
    if !pending.is_null() {
        *pending = pending_value;
        return status;
    }
    crate::zlib_h::Z_OK
}
#[export_name = "deflatePending"]

pub unsafe extern "C" fn deflatePending_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflatePending(strm, pending, bits)
}

fn deflate_used_impl(
    bi_used: ::core::ffi::c_int,
    bits: Option<&mut ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    if let Some(bits) = bits {
        *bits = bi_used;
    }
    crate::zlib_h::Z_OK
}

pub unsafe extern "C" fn deflateUsed(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*((*strm).state as *const crate::src::deflate::deflate_state);
    let bits = if bits.is_null() {
        None
    } else {
        Some(&mut *bits)
    };
    deflate_used_impl(state.bi_used, bits)
}
#[export_name = "deflateUsed"]

pub unsafe extern "C" fn deflateUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateUsed(strm, bits)
}

fn deflate_prime_bits(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    bi_buf: &mut crate::zutil_h::ush,
    bi_valid: &mut ::core::ffi::c_int,
    pending_out: usize,
    lit_bufsize: crate::stdlib::uInt,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if bits < 0
        || bits > 16
        || pending_out.wrapping_add(((crate::src::deflate::Buf_size + 7) >> 3) as usize)
            > lit_bufsize as usize
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    loop {
        let mut put = crate::src::deflate::Buf_size - *bi_valid;
        if put > bits {
            put = bits;
        }
        *bi_buf = (*bi_buf as ::core::ffi::c_int
            | ((value & ((1 as ::core::ffi::c_int) << put) - 1) << *bi_valid) as crate::zutil_h::ush
                as ::core::ffi::c_int) as crate::zutil_h::ush;
        *bi_valid += put;
        crate::src::trees::flush_pending_bits(pending_buf, pending, bi_buf, bi_valid);
        value >>= put;
        bits -= put;
        if bits == 0 {
            break;
        }
    }
    crate::zlib_h::Z_OK
}

pub unsafe extern "C" fn deflatePrime(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state
            .pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        state.pending_buf_size as usize,
    );
    deflate_prime_bits(
        pending_buf,
        &mut state.pending,
        &mut state.bi_buf,
        &mut state.bi_valid,
        state.pending_out,
        state.lit_bufsize,
        bits,
        value,
    )
}
#[export_name = "deflatePrime"]

pub unsafe extern "C" fn deflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflatePrime(strm, bits, value)
}
pub unsafe extern "C" fn deflateParams(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut algorithm: DeflateAlgorithm;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        level = 6 as ::core::ffi::c_int;
    }
    if level < 0 as ::core::ffi::c_int
        || level > 9 as ::core::ffi::c_int
        || strategy < 0 as ::core::ffi::c_int
        || strategy > crate::zlib_h::Z_FIXED
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    algorithm = configuration_table[(*s).level as usize].algorithm;
    if (strategy != (*s).strategy || algorithm != configuration_table[level as usize].algorithm)
        && (*s).last_flush != -2 as ::core::ffi::c_int
    {
        let mut err: ::core::ffi::c_int = deflate(strm, crate::zlib_h::Z_BLOCK);
        if err == crate::zlib_h::Z_STREAM_ERROR {
            return err;
        }
        if (*strm).avail_in != 0
            || (*s).strstart as ::core::ffi::c_long - (*s).block_start
                + (*s).lookahead as ::core::ffi::c_long
                != 0
        {
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    if (*s).level != level {
        if (*s).level == 0 as ::core::ffi::c_int && (*s).matches != 0 as crate::stdlib::uInt {
            if (*s).matches == 1 as crate::stdlib::uInt {
                let state = &mut *s;
                // `head` and `prev` are allocated at these exact element
                // counts in `deflateInit2_()` and `deflateCopy()`.
                let head = ::core::slice::from_raw_parts_mut(
                    state.head.expect("initialized head table").as_ptr(),
                    state.hash_size as usize,
                );
                let prev = ::core::slice::from_raw_parts_mut(
                    state.prev.expect("initialized prev table").as_ptr(),
                    state.w_size as usize,
                );
                slide_hash_table(head, state.w_size);
                slide_hash_table(prev, state.w_size);
                state.slid = 1 as ::core::ffi::c_int;
            } else {
                // `head` has exactly `hash_size` elements from
                // `deflateInit2_()` or `deflateCopy()`.
                let head = ::core::slice::from_raw_parts_mut(
                    (*s).head.expect("initialized head table").as_ptr(),
                    (*s).hash_size as usize,
                );
                clear_hash_table(head);
                (*s).slid = 0 as ::core::ffi::c_int;
            }
            (*s).matches = 0 as crate::stdlib::uInt;
        }
        (*s).level = level;
        (*s).max_lazy_match = configuration_table[level as usize].max_lazy as crate::stdlib::uInt;
        (*s).good_match = configuration_table[level as usize].good_length as crate::stdlib::uInt;
        (*s).nice_match = configuration_table[level as usize].nice_length as ::core::ffi::c_int;
        (*s).max_chain_length =
            configuration_table[level as usize].max_chain as crate::stdlib::uInt;
    }
    (*s).strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateParams"]

pub unsafe extern "C" fn deflateParams_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateParams(strm, level, strategy)
}
fn deflate_tune_values(
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) -> (
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    ::core::ffi::c_int,
    crate::stdlib::uInt,
) {
    (
        good_length as crate::stdlib::uInt,
        max_lazy as crate::stdlib::uInt,
        nice_length,
        max_chain as crate::stdlib::uInt,
    )
}

pub unsafe extern "C" fn deflateTune(
    mut strm: crate::zlib_h::z_streamp,
    mut good_length: ::core::ffi::c_int,
    mut max_lazy: ::core::ffi::c_int,
    mut nice_length: ::core::ffi::c_int,
    mut max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let s = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    let (good_match, max_lazy_match, nice_match, max_chain_length) =
        deflate_tune_values(good_length, max_lazy, nice_length, max_chain);
    s.good_match = good_match;
    s.max_lazy_match = max_lazy_match;
    s.nice_match = nice_match;
    s.max_chain_length = max_chain_length;
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateTune"]

pub unsafe extern "C" fn deflateTune_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut good_length: ::core::ffi::c_int,
    mut max_lazy: ::core::ffi::c_int,
    mut nice_length: ::core::ffi::c_int,
    mut max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateTune(strm, good_length, max_lazy, nice_length, max_chain)
}
struct DeflateBoundState {
    wrap: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    gzip_extra_len: Option<crate::stdlib::uInt>,
    gzip_name_len: Option<crate::stdlib::z_size_t>,
    gzip_comment_len: Option<crate::stdlib::z_size_t>,
    gzip_hcrc: bool,
    w_bits: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
}

fn deflate_bound_impl(
    source_len: crate::stdlib::z_size_t,
    state: Option<DeflateBoundState>,
) -> crate::stdlib::z_size_t {
    let mut fixedlen = source_len
        .wrapping_add(source_len >> 3 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 8 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 9 as ::core::ffi::c_int)
        .wrapping_add(4 as crate::stdlib::z_size_t);
    if fixedlen < source_len {
        fixedlen = -1 as ::core::ffi::c_int as crate::stdlib::z_size_t;
    }
    let mut storelen = source_len
        .wrapping_add(source_len >> 5 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 7 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 11 as ::core::ffi::c_int)
        .wrapping_add(7 as crate::stdlib::z_size_t);
    if storelen < source_len {
        storelen = -1 as ::core::ffi::c_int as crate::stdlib::z_size_t;
    }
    let Some(state) = state else {
        let bound = if fixedlen > storelen {
            fixedlen
        } else {
            storelen
        };
        return if bound.wrapping_add(18 as crate::stdlib::z_size_t) < bound {
            -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
        } else {
            bound.wrapping_add(18 as crate::stdlib::z_size_t)
        };
    };
    let mut wraplen = 0 as crate::stdlib::z_size_t;
    match if state.wrap < 0 as ::core::ffi::c_int {
        -state.wrap
    } else {
        state.wrap
    } {
        0 => {
            wraplen = 0 as crate::stdlib::z_size_t;
        }
        1 => {
            wraplen = (6 as ::core::ffi::c_int
                + (if state.strstart != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as crate::stdlib::z_size_t;
        }
        2 => {
            wraplen = 18 as crate::stdlib::z_size_t;
            if let Some(extra_len) = state.gzip_extra_len {
                wraplen = wraplen
                    .wrapping_add((2 as crate::stdlib::uInt).wrapping_add(extra_len)
                        as crate::stdlib::z_size_t);
            }
            if let Some(name_len) = state.gzip_name_len {
                wraplen = wraplen.wrapping_add(name_len);
            }
            if let Some(comment_len) = state.gzip_comment_len {
                wraplen = wraplen.wrapping_add(comment_len);
            }
            if state.gzip_hcrc {
                wraplen = wraplen.wrapping_add(2 as crate::stdlib::z_size_t);
            }
        }
        _ => {
            wraplen = 18 as crate::stdlib::z_size_t;
        }
    }
    if state.w_bits != 15 as crate::stdlib::uInt
        || state.hash_bits
            != (8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        let bound = if state.w_bits <= state.hash_bits && state.level != 0 {
            fixedlen
        } else {
            storelen
        };
        return if bound.wrapping_add(wraplen) < bound {
            -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
        } else {
            bound.wrapping_add(wraplen)
        };
    }
    let bound = source_len
        .wrapping_add(source_len >> 12 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 14 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as crate::stdlib::z_size_t)
        .wrapping_sub(6 as crate::stdlib::z_size_t)
        .wrapping_add(wraplen);
    if bound < source_len {
        -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
    } else {
        bound
    }
}

pub unsafe extern "C" fn deflateBound_z(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if deflateStateCheck(strm) != 0 {
        return deflate_bound_impl(sourceLen, None);
    }
    let stream = &*strm;
    let state = &*(stream.state as *const crate::src::deflate::deflate_state);
    let mut gzip_extra_len = None;
    let mut gzip_name_len = None;
    let mut gzip_comment_len = None;
    let mut gzip_hcrc = false;
    if state.wrap == 2 as ::core::ffi::c_int || state.wrap == -2 as ::core::ffi::c_int {
        if let Some(header) = state.gzhead.as_ref() {
            if header.extra.is_some() {
                gzip_extra_len = Some(header.extra_len);
            }
            gzip_name_len = header.name.as_ref().map(|name| name.len());
            gzip_comment_len = header.comment.as_ref().map(|comment| comment.len());
            gzip_hcrc = header.hcrc;
        }
    }
    deflate_bound_impl(
        sourceLen,
        Some(DeflateBoundState {
            wrap: state.wrap,
            strstart: state.strstart,
            gzip_extra_len,
            gzip_name_len,
            gzip_comment_len,
            gzip_hcrc,
            w_bits: state.w_bits,
            hash_bits: state.hash_bits,
            level: state.level,
        }),
    )
}
#[export_name = "deflateBound_z"]

pub unsafe extern "C" fn deflateBound_z_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    deflateBound_z(strm, sourceLen)
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    deflateBound_z(strm, sourceLen as crate::stdlib::z_size_t) as crate::stdlib::uLong
}

fn put_short_msb_bytes(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    value: crate::stdlib::uInt,
) {
    let start = *pending as usize;
    let Some(bytes) = pending_buf.get_mut(start..start.saturating_add(2)) else {
        return;
    };
    bytes[0] = (value >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte;
    bytes[1] = (value & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte;
    *pending = pending.wrapping_add(2);
}

fn append_pending_bytes(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    bytes: &[crate::stdlib::Bytef],
) {
    let start = *pending as usize;
    let Some(end) = start.checked_add(bytes.len()) else {
        return;
    };
    let Some(output) = pending_buf.get_mut(start..end) else {
        return;
    };
    output.copy_from_slice(bytes);
    *pending = pending.wrapping_add(bytes.len() as crate::zutil_h::ulg);
}

fn push_pending_byte(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    byte: crate::stdlib::Bytef,
) {
    let index = *pending as usize;
    pending_buf[index] = byte;
    *pending = pending.wrapping_add(1);
}

// Build the fixed and optional portions of the gzip member header using only
// owned header metadata and the already-bounded pending allocation. `None`
// preserves the compact default header path; `Some(hcrc)` records whether the
// caller must checksum the bytes written here.
fn append_gzip_header_prefix(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    gzhead: Option<&GzipHeader>,
) -> Option<bool> {
    push_pending_byte(pending_buf, pending, 31);
    push_pending_byte(pending_buf, pending, 139);
    push_pending_byte(pending_buf, pending, 8);
    let xfl = if level == 9 {
        2
    } else if strategy >= 2 || level < 2 {
        4
    } else {
        0
    };
    let Some(gzhead) = gzhead else {
        for byte in [0, 0, 0, 0, 0, xfl, 3] {
            push_pending_byte(pending_buf, pending, byte);
        }
        return None;
    };
    push_pending_byte(
        pending_buf,
        pending,
        ((if gzhead.text != 0 { 1 } else { 0 })
            + (if gzhead.hcrc { 2 } else { 0 })
            + (if gzhead.extra.is_none() { 0 } else { 4 })
            + (if gzhead.name.is_none() { 0 } else { 8 })
            + (if gzhead.comment.is_none() { 0 } else { 16 })) as crate::stdlib::Bytef,
    );
    for shift in [0, 8, 16, 24] {
        push_pending_byte(
            pending_buf,
            pending,
            (gzhead.time >> shift & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
        );
    }
    push_pending_byte(pending_buf, pending, xfl);
    push_pending_byte(
        pending_buf,
        pending,
        (gzhead.os & 0xff as ::core::ffi::c_int) as crate::stdlib::Bytef,
    );
    if gzhead.extra.is_some() {
        push_pending_byte(
            pending_buf,
            pending,
            (gzhead.extra_len & 0xff as crate::stdlib::uInt) as crate::stdlib::Bytef,
        );
        push_pending_byte(
            pending_buf,
            pending,
            (gzhead.extra_len >> 8 & 0xff as crate::stdlib::uInt) as crate::stdlib::Bytef,
        );
    }
    Some(gzhead.hcrc)
}

// Copy as much of a NUL-terminated gzip header field as fits in the current
// pending buffer. The caller keeps the cursor and flush policy in the ABI
// state layer; this kernel only operates on bounded byte views.
fn append_gzip_cstring_bytes(
    source: &[crate::stdlib::Bytef],
    source_index: &mut usize,
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
) -> bool {
    let Some(remaining) = source.get(*source_index..) else {
        return true;
    };
    let available = pending_buf.len().saturating_sub(*pending as usize);
    if available == 0 {
        return false;
    }
    let limit = remaining.len().min(available);
    let count = remaining[..limit]
        .iter()
        .position(|&byte| byte == 0)
        .map_or(limit, |index| index + 1);
    let start = *pending as usize;
    pending_buf[start..start + count].copy_from_slice(&remaining[..count]);
    *source_index += count;
    *pending = pending.wrapping_add(count as crate::zutil_h::ulg);
    count != 0 && remaining[count - 1] == 0
}

fn flush_pending_bytes(
    output: &mut [crate::stdlib::Bytef],
    pending_buf: &[crate::stdlib::Bytef],
    pending_out: &mut usize,
    pending_len: &mut crate::zutil_h::ulg,
) -> crate::stdlib::uInt {
    let len = if *pending_len > output.len() as crate::zutil_h::ulg {
        output.len()
    } else {
        *pending_len as usize
    };
    if len == 0 {
        return 0;
    }
    output[..len].copy_from_slice(&pending_buf[*pending_out..*pending_out + len]);
    *pending_out += len;
    *pending_len = pending_len.wrapping_sub(len as crate::zutil_h::ulg);
    if *pending_len == 0 {
        *pending_out = 0;
    }
    len as crate::stdlib::uInt
}

// The pending allocation has two logical regions: queued output and deferred
// symbols.  Keep those ranges as ordinary Rust indices so the eventual owned
// pending storage can use this same description without reconstructing raw
// cursors.
struct PendingRegions {
    queued: ::core::ops::Range<usize>,
    symbols: ::core::ops::Range<usize>,
}

// Copying a deflate stream has a fixed set of allocation extents derived from
// scalar state.  Keep that geometry pointer-free so the eventual owned-state
// copy can use the same plan, rather than reconstructing lengths from raw
// allocation bases.
struct DeflateCopyLayout {
    window_bytes: usize,
    prev_entries: usize,
    head_entries: usize,
    pending: Option<PendingRegions>,
}

fn deflate_copy_layout(
    high_water: crate::zutil_h::ulg,
    slid: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    storage: &DeflateStorageLayout,
    pending_out: usize,
    pending_len: usize,
    sym_buf_start: usize,
    sym_next: usize,
) -> DeflateCopyLayout {
    let prev_capacity = storage
        .prev
        .element_len::<crate::src::deflate::Posf>()
        .expect("validated previous-table allocation geometry");
    let prev_entries =
        if slid != 0 || strstart.wrapping_sub(insert) > prev_capacity as crate::stdlib::uInt {
            prev_capacity
        } else {
            strstart.wrapping_sub(insert) as usize
        };
    DeflateCopyLayout {
        window_bytes: high_water as usize,
        prev_entries,
        head_entries: storage
            .head
            .element_len::<crate::src::deflate::Posf>()
            .expect("validated hash-table allocation geometry"),
        pending: PendingRegions::new(pending_out, pending_len, sym_buf_start, sym_next),
    }
}

impl PendingRegions {
    fn new(
        pending_out: usize,
        pending_len: usize,
        sym_buf_start: usize,
        sym_next: usize,
    ) -> Option<Self> {
        Some(Self {
            queued: pending_out..pending_out.checked_add(pending_len)?,
            symbols: sym_buf_start..sym_buf_start.checked_add(sym_next)?,
        })
    }

    fn copy_from(self, source: &[crate::stdlib::Bytef], destination: &mut [crate::stdlib::Bytef]) {
        let Some(queued) = source.get(self.queued.clone()) else {
            return;
        };
        let Some(symbols) = source.get(self.symbols.clone()) else {
            return;
        };
        let Some(destination_queued) = destination.get_mut(self.queued) else {
            return;
        };
        destination_queued.copy_from_slice(queued);
        let Some(destination_symbols) = destination.get_mut(self.symbols) else {
            return;
        };
        destination_symbols.copy_from_slice(symbols);
    }
}

// `deflateCopy()` duplicates those two logical regions.  Allocation setup
// establishes the extents before the implementation constructs these views;
// the copy order matches the two original memcpy operations.
fn copy_pending_regions(
    source: &[crate::stdlib::Bytef],
    destination: &mut [crate::stdlib::Bytef],
    regions: Option<PendingRegions>,
) {
    let Some(regions) = regions else {
        return;
    };
    regions.copy_from(source, destination);
}

// `_tr_stored_block()` has already reserved these four bytes after winding up
// the bit buffer.  Patch that header through the declared pending allocation
// instead of reconstructing four raw cursors in the stored-block algorithm.
fn set_stored_block_length(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: crate::zutil_h::ulg,
    len: crate::stdlib::uInt,
) {
    let Some(start) = (pending as usize).checked_sub(4) else {
        return;
    };
    let Some(header) = pending_buf.get_mut(start..start.saturating_add(4)) else {
        return;
    };
    header[0] = len as crate::stdlib::Bytef;
    header[1] = (len >> 8) as crate::stdlib::Bytef;
    header[2] = !len as crate::stdlib::Bytef;
    header[3] = (!len >> 8) as crate::stdlib::Bytef;
}

unsafe extern "C" fn flush_pending(mut strm: crate::zlib_h::z_streamp) -> crate::stdlib::uInt {
    let mut len: ::core::ffi::c_uint = 0;
    // The stream and its opaque state are distinct allocations.  Project each
    // once so the bounded pending/output helpers operate on Rust references
    // rather than repeatedly dereferencing the ABI pointers.
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::deflate::deflate_state);
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state
            .pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        state.pending_buf_size as usize,
    );
    crate::src::trees::flush_pending_bits(
        pending_buf,
        &mut state.pending,
        &mut state.bi_buf,
        &mut state.bi_valid,
    );
    len = if state.pending > strm.avail_out as crate::zutil_h::ulg {
        strm.avail_out as ::core::ffi::c_uint
    } else {
        state.pending as ::core::ffi::c_uint
    };
    if len == 0 as ::core::ffi::c_uint {
        return strm.avail_out;
    }
    let output = ::core::slice::from_raw_parts_mut(strm.next_out, len as usize);
    let len = flush_pending_bytes(
        output,
        pending_buf,
        &mut state.pending_out,
        &mut state.pending,
    );
    strm.next_out = strm.next_out.wrapping_add(len as usize);
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out = strm.avail_out.wrapping_sub(len);
    strm.avail_out
}
pub unsafe extern "C" fn deflate(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut old_flush: ::core::ffi::c_int = 0;
    if deflateStateCheck(strm) != 0
        || flush > crate::zlib_h::Z_BLOCK
        || flush < 0 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Keep the ABI projections at this boundary.  The rest of this dispatcher
    // only uses the scoped stream/state borrows, while the lower-level block
    // routines continue to receive their existing opaque state pointer.
    let strm = &mut *strm;
    let s = &mut *(strm.state as *mut crate::src::deflate::deflate_state);
    if strm.next_out.is_null()
        || strm.avail_in != 0 as crate::stdlib::uInt && strm.next_in.is_null()
        || s.status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH
    {
        strm.msg = crate::src::zutil::z_errmsg[(if (-2 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -2 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -2 as ::core::ffi::c_int
        }) as usize]
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -2 as ::core::ffi::c_int;
    }
    if strm.avail_out == 0 as crate::stdlib::uInt {
        strm.msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
        }) as usize]
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -5 as ::core::ffi::c_int;
    }
    old_flush = s.last_flush;
    s.last_flush = flush;
    if s.pending != 0 as crate::zutil_h::ulg {
        flush_pending(strm);
        if strm.avail_out == 0 as crate::stdlib::uInt {
            s.last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    } else if strm.avail_in == 0 as crate::stdlib::uInt
        && flush * 2 as ::core::ffi::c_int
            - (if flush > 4 as ::core::ffi::c_int {
                9 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            <= old_flush * 2 as ::core::ffi::c_int
                - (if old_flush > 4 as ::core::ffi::c_int {
                    9 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })
        && flush != crate::zlib_h::Z_FINISH
    {
        strm.msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
        }) as usize]
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -5 as ::core::ffi::c_int;
    }
    if s.status == crate::src::deflate::FINISH_STATE
        && strm.avail_in != 0 as crate::stdlib::uInt
    {
        strm.msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
        }) as usize]
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -5 as ::core::ffi::c_int;
    }
    if s.status == crate::src::deflate::INIT_STATE && s.wrap == 0 as ::core::ffi::c_int {
        s.status = crate::src::deflate::BUSY_STATE;
    }
    if s.status == crate::src::deflate::INIT_STATE {
        let mut header: crate::stdlib::uInt =
            (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt).wrapping_add(
                s.w_bits.wrapping_sub(8 as crate::stdlib::uInt) << 4 as ::core::ffi::c_int,
            ) << 8 as ::core::ffi::c_int;
        let mut level_flags: crate::stdlib::uInt = 0;
        if s.strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || s.level < 2 as ::core::ffi::c_int {
            level_flags = 0 as crate::stdlib::uInt;
        } else if s.level < 6 as ::core::ffi::c_int {
            level_flags = 1 as crate::stdlib::uInt;
        } else if s.level == 6 as ::core::ffi::c_int {
            level_flags = 2 as crate::stdlib::uInt;
        } else {
            level_flags = 3 as crate::stdlib::uInt;
        }
        header |= level_flags << 6 as ::core::ffi::c_int;
        if s.strstart != 0 as crate::stdlib::uInt {
            header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
        }
        header = header.wrapping_add(
            (31 as crate::stdlib::uInt)
                .wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
        );
        let has_dictionary = s.strstart != 0 as crate::stdlib::uInt;
        let dictionary_adler = strm.adler;
        {
            let state = &mut *s;
            // `pending_buf` has exactly `pending_buf_size` bytes (allocated in
            // `deflateInit2_()` and copied at that extent in `deflateCopy()`).
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state
                    .pending_buf
                    .expect("initialized pending buffer")
                    .as_ptr(),
                state.pending_buf_size as usize,
            );
            put_short_msb_bytes(pending_buf, &mut state.pending, header);
            if has_dictionary {
                put_short_msb_bytes(
                    pending_buf,
                    &mut state.pending,
                    (dictionary_adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
                );
                put_short_msb_bytes(
                    pending_buf,
                    &mut state.pending,
                    (dictionary_adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
                );
            }
        }
        strm.adler = crate::src::adler32::adler32_z(0 as crate::stdlib::uLong, None);
        s.status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if s.pending != 0 as crate::zutil_h::ulg {
            s.last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if s.status == crate::src::deflate::GZIP_STATE {
        strm.adler = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None);
        let state = &mut *s;
        // This initial gzip header always fits in the pending allocation. Keep
        // a single exact-capacity view for the contiguous write sequence.
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state
                .pending_buf
                .expect("initialized pending buffer")
                .as_ptr(),
            state.pending_buf_size as usize,
        );
        match append_gzip_header_prefix(
            pending_buf,
            &mut state.pending,
            state.level,
            state.strategy,
            state.gzhead.as_ref(),
        ) {
            None => {
            state.status = crate::src::deflate::BUSY_STATE;
            flush_pending(strm);
            if s.pending != 0 as crate::zutil_h::ulg {
                s.last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
            }
            Some(hcrc) => {
            if hcrc {
                strm.adler = crate::src::crc32::crc32_z(
                    strm.adler,
                    Some(&pending_buf[..state.pending as usize]),
                );
            }
            state.gzindex = 0;
            state.status = crate::src::deflate::EXTRA_STATE;
            }
        }
    }
    if (*s).status == crate::src::deflate::EXTRA_STATE {
        if (*s)
            .gzhead
            .as_ref()
            .is_some_and(|header| header.extra.is_some())
        {
            let gzhead = (*s).gzhead.as_ref().expect("gzip header was checked");
            let extra = gzhead.extra.as_deref().expect("gzip extra was checked");
            let hcrc = gzhead.hcrc;
            let mut left = extra.len().wrapping_sub((*s).gzindex);
            while (*s).pending.wrapping_add(left as crate::zutil_h::ulg) > (*s).pending_buf_size {
                let mut copy: crate::zutil_h::ulg =
                    (*s).pending_buf_size.wrapping_sub((*s).pending);
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    (*s).pending_buf
                        .expect("initialized pending buffer")
                        .as_ptr(),
                    (*s).pending_buf_size as usize,
                );
                append_pending_bytes(pending_buf, &mut (*s).pending, &extra[..copy as usize]);
                if hcrc {
                    (*strm).adler =
                        crate::src::crc32::crc32_z((*strm).adler, Some(&extra[..copy as usize]));
                }
                (*s).gzindex = (*s).gzindex.wrapping_add(copy as usize);
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
                left = left.wrapping_sub(copy as usize);
            }
            if left != 0 {
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    (*s).pending_buf
                        .expect("initialized pending buffer")
                        .as_ptr(),
                    (*s).pending_buf_size as usize,
                );
                append_pending_bytes(pending_buf, &mut (*s).pending, &extra[..left]);
                if hcrc {
                    (*strm).adler = crate::src::crc32::crc32_z((*strm).adler, Some(&extra[..left]));
                }
            }
            (*s).gzindex = 0;
        }
        (*s).status = crate::src::deflate::NAME_STATE;
    }
    if (*s).status == crate::src::deflate::NAME_STATE {
        if (*s)
            .gzhead
            .as_ref()
            .is_some_and(|header| header.name.is_some())
        {
            // The header is owned by the state.  Copy this retained byte
            // string before advancing the pending cursor so the safe state
            // borrow does not overlap its mutable pending-buffer updates.
            let (name, hcrc) = {
                let gzhead = (*s).gzhead.as_ref().expect("gzip header was checked");
                (
                    gzhead.name.as_deref().expect("gzip name was checked").to_vec(),
                    gzhead.hcrc,
                )
            };
            let mut beg_0: crate::zutil_h::ulg = (*s).pending;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if hcrc && (*s).pending > beg_0 {
                        let state = &mut *s;
                        let pending_buf = ::core::slice::from_raw_parts(
                            state
                                .pending_buf
                                .expect("initialized pending buffer")
                                .as_ptr(),
                            state.pending_buf_size as usize,
                        );
                        (*strm).adler = crate::src::crc32::crc32_z(
                            (*strm).adler,
                            Some(&pending_buf[beg_0 as usize..state.pending as usize]),
                        );
                    }
                    flush_pending(strm);
                    if (*s).pending != 0 as crate::zutil_h::ulg {
                        (*s).last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                    beg_0 = 0 as crate::zutil_h::ulg;
                }
                let complete = {
                    let state = &mut *s;
                    let mut source_index = state.gzindex;
                    let pending_buf = ::core::slice::from_raw_parts_mut(
                        state
                            .pending_buf
                            .expect("initialized pending buffer")
                            .as_ptr(),
                        state.pending_buf_size as usize,
                    );
                    let complete = append_gzip_cstring_bytes(
                        &name,
                        &mut source_index,
                        pending_buf,
                        &mut state.pending,
                    );
                    state.gzindex = source_index;
                    complete
                };
                if complete {
                    break;
                }
            }
            if hcrc && (*s).pending > beg_0 {
                let state = &mut *s;
                let pending_buf = ::core::slice::from_raw_parts(
                    state
                        .pending_buf
                        .expect("initialized pending buffer")
                        .as_ptr(),
                    state.pending_buf_size as usize,
                );
                (*strm).adler = crate::src::crc32::crc32_z(
                    (*strm).adler,
                    Some(&pending_buf[beg_0 as usize..state.pending as usize]),
                );
            }
            (*s).gzindex = 0;
        }
        (*s).status = crate::src::deflate::COMMENT_STATE;
    }
    if (*s).status == crate::src::deflate::COMMENT_STATE {
        if (*s)
            .gzhead
            .as_ref()
            .is_some_and(|header| header.comment.is_some())
        {
            // See the NAME-state copy above: retain an independent byte view
            // while the pending cursor advances through this header field.
            let (comment, hcrc) = {
                let gzhead = (*s).gzhead.as_ref().expect("gzip header was checked");
                (
                    gzhead
                        .comment
                        .as_deref()
                        .expect("gzip comment was checked")
                        .to_vec(),
                    gzhead.hcrc,
                )
            };
            let mut beg_1: crate::zutil_h::ulg = (*s).pending;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if hcrc && (*s).pending > beg_1 {
                        let state = &mut *s;
                        let pending_buf = ::core::slice::from_raw_parts(
                            state
                                .pending_buf
                                .expect("initialized pending buffer")
                                .as_ptr(),
                            state.pending_buf_size as usize,
                        );
                        (*strm).adler = crate::src::crc32::crc32_z(
                            (*strm).adler,
                            Some(&pending_buf[beg_1 as usize..state.pending as usize]),
                        );
                    }
                    flush_pending(strm);
                    if (*s).pending != 0 as crate::zutil_h::ulg {
                        (*s).last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                    beg_1 = 0 as crate::zutil_h::ulg;
                }
                let complete = {
                    let state = &mut *s;
                    let mut source_index = state.gzindex;
                    let pending_buf = ::core::slice::from_raw_parts_mut(
                        state
                            .pending_buf
                            .expect("initialized pending buffer")
                            .as_ptr(),
                        state.pending_buf_size as usize,
                    );
                    let complete = append_gzip_cstring_bytes(
                        &comment,
                        &mut source_index,
                        pending_buf,
                        &mut state.pending,
                    );
                    state.gzindex = source_index;
                    complete
                };
                if complete {
                    break;
                }
            }
            if hcrc && (*s).pending > beg_1 {
                let state = &mut *s;
                let pending_buf = ::core::slice::from_raw_parts(
                    state
                        .pending_buf
                        .expect("initialized pending buffer")
                        .as_ptr(),
                    state.pending_buf_size as usize,
                );
                (*strm).adler = crate::src::crc32::crc32_z(
                    (*strm).adler,
                    Some(&pending_buf[beg_1 as usize..state.pending as usize]),
                );
            }
        }
        (*s).status = crate::src::deflate::HCRC_STATE;
    }
    if (*s).status == crate::src::deflate::HCRC_STATE {
        if (*s).gzhead.as_ref().is_some_and(|header| header.hcrc) {
            if (*s).pending.wrapping_add(2 as crate::zutil_h::ulg) > (*s).pending_buf_size {
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
            let hcrc = (*strm).adler;
            let state = &mut *s;
            // The preceding capacity check ensures that both HCRC bytes fit
            // in this exact pending allocation.
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state
                    .pending_buf
                    .expect("initialized pending buffer")
                    .as_ptr(),
                state.pending_buf_size as usize,
            );
            append_pending_bytes(
                pending_buf,
                &mut state.pending,
                &[
                    (hcrc & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                    (hcrc >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                        as crate::stdlib::Byte,
                ],
            );
            (*strm).adler = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None);
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
            let func = match configuration_table[(*s).level as usize].algorithm {
                DeflateAlgorithm::Stored => deflate_stored,
                DeflateAlgorithm::Fast => deflate_fast,
                DeflateAlgorithm::Slow => deflate_slow,
            };
            func(s, flush) as ::core::ffi::c_uint
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
                crate::src::trees::bi_flush_or_windup(
                    s as *mut crate::src::deflate::internal_state,
                    crate::src::trees::BitOutputAction::Align,
                );
            } else if flush != crate::zlib_h::Z_BLOCK {
                crate::src::trees::_tr_stored_block(
                    s as *mut crate::src::deflate::internal_state,
                    ::core::ptr::null_mut::<crate::stdlib::charf>(),
                    0 as crate::zutil_h::ulg,
                    0 as ::core::ffi::c_int,
                );
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    // `head` has exactly `hash_size` elements from
                    // `deflateInit2_()` or `deflateCopy()`.
                    let head = ::core::slice::from_raw_parts_mut(
                        (*s).head.expect("initialized head table").as_ptr(),
                        (*s).hash_size as usize,
                    );
                    clear_hash_table(head);
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
        // The final gzip trailer has a fixed eight-byte representation.  The
        // pending allocation has exactly `pending_buf_size` bytes, established
        // by `deflateInit2_()` or `deflateCopy()`.
        let pending_buf = ::core::slice::from_raw_parts_mut(
            (*s).pending_buf
                .expect("initialized pending buffer")
                .as_ptr(),
            (*s).pending_buf_size as usize,
        );
        append_pending_bytes(
            pending_buf,
            &mut (*s).pending,
            &[
                ((*strm).adler & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                ((*strm).adler >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                ((*strm).adler >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                ((*strm).adler >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                ((*strm).total_in & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                ((*strm).total_in >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                ((*strm).total_in >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                ((*strm).total_in >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
            ],
        );
    } else {
        let adler = (*strm).adler;
        {
            let state = &mut *s;
            // `pending_buf` has exactly `pending_buf_size` bytes (allocated in
            // `deflateInit2_()` and copied at that extent in `deflateCopy()`).
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state
                    .pending_buf
                    .expect("initialized pending buffer")
                    .as_ptr(),
                state.pending_buf_size as usize,
            );
            put_short_msb_bytes(
                pending_buf,
                &mut state.pending,
                (adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
            );
            put_short_msb_bytes(
                pending_buf,
                &mut state.pending,
                (adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
            );
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
pub unsafe extern "C" fn deflateEnd(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    status = (*(*strm).state).status;
    // The state itself is released through the caller's zfree callback, so
    // drop the owned gzip-header snapshot before releasing that allocation.
    ::core::ptr::drop_in_place(::core::ptr::addr_of_mut!((*(*strm).state).gzhead));
    if let Some(pending_buf) = (*(*strm).state).pending_buf {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            pending_buf.as_ptr() as crate::stdlib::voidpf,
        );
    }
    if let Some(head) = (*(*strm).state).head {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            head.as_ptr() as crate::stdlib::voidpf,
        );
    }
    if let Some(prev) = (*(*strm).state).prev {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            prev.as_ptr() as crate::stdlib::voidpf,
        );
    }
    if let Some(window) = (*(*strm).state).window {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            window.as_ptr() as crate::stdlib::voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as crate::stdlib::voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
}
#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    deflateEnd(strm)
}
pub unsafe extern "C" fn deflateCopy(
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
    // zlib's `deflateCopy()` requires distinct source and destination stream
    // objects.  Preserve the translated byte-for-byte copy, but do not route
    // it through the C `memcpy` import.
    ::core::ptr::copy_nonoverlapping(
        source.cast::<u8>(),
        dest.cast::<u8>(),
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
    (*dest).state = ds as *mut crate::src::deflate::internal_state;
    // The allocation is immediately overwritten with the source state before
    // any field is observed.  Do not clear it first: that C-style write is
    // dead and the copied state supplies every byte.
    ::core::ptr::copy_nonoverlapping(
        ss.cast::<u8>(),
        ds.cast::<u8>(),
        ::core::mem::size_of::<crate::src::deflate::deflate_state>(),
    );
    // The bytewise state copy above is needed for the C allocator-backed
    // storage.  Replace the copied owner before it can be observed or
    // released, making the header registration an independent deep copy.
    ::core::ptr::addr_of_mut!((*ds).gzhead).write((*ss).gzhead.as_ref().map(copy_gzip_header));
    (*ds).strm = ::core::ptr::NonNull::new(dest).expect("validated destination stream");
    let storage = DeflateStorageLayout::new((*ds).w_size, (*ds).hash_size, (*ds).lit_bufsize);
    (*ds).window =
        ::core::ptr::NonNull::new(Some((*dest).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*dest).opaque,
            storage.window.items,
            storage.window.size,
        ) as *mut crate::stdlib::Bytef);
    (*ds).prev =
        ::core::ptr::NonNull::new(Some((*dest).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*dest).opaque,
            storage.prev.items,
            storage.prev.size,
        ) as *mut crate::src::deflate::Posf);
    (*ds).head =
        ::core::ptr::NonNull::new(Some((*dest).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*dest).opaque,
            storage.head.items,
            storage.head.size,
        ) as *mut crate::src::deflate::Posf);
    (*ds).pending_buf =
        ::core::ptr::NonNull::new(Some((*dest).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*dest).opaque,
            storage.pending.items,
            storage.pending.size,
        ) as *mut crate::zutil_h::uchf
            as *mut crate::stdlib::Bytef);
    if (*ds).window.is_none()
        || (*ds).prev.is_none()
        || (*ds).head.is_none()
        || (*ds).pending_buf.is_none()
    {
        deflateEnd(dest);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    let copy_layout = deflate_copy_layout(
        (*ss).high_water,
        (*ss).slid,
        (*ss).strstart,
        (*ss).insert,
        &storage,
        (*ss).pending_out,
        (*ss).pending as usize,
        (*ss).sym_buf_start,
        (*ss).sym_next as usize,
    );
    ::core::ptr::copy_nonoverlapping(
        (*ss).window.expect("initialized window").as_ptr(),
        (*ds).window.expect("initialized window").as_ptr(),
        copy_layout.window_bytes,
    );
    ::core::ptr::copy_nonoverlapping(
        (*ss).prev.expect("initialized prev table").as_ptr(),
        (*ds).prev.expect("initialized prev table").as_ptr(),
        copy_layout.prev_entries,
    );
    ::core::ptr::copy_nonoverlapping(
        (*ss).head.expect("initialized head table").as_ptr(),
        (*ds).head.expect("initialized head table").as_ptr(),
        copy_layout.head_entries,
    );
    (*ds).pending_out = (*ss).pending_out;
    // Both allocations have the copied `pending_buf_size` capacity.  Form
    // each bounded view once and keep the two logical-region copies in the
    // pointer-free kernel.
    let source_pending = ::core::slice::from_raw_parts(
        (*ss)
            .pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        (*ss).pending_buf_size as usize,
    );
    let destination_pending = ::core::slice::from_raw_parts_mut(
        (*ds)
            .pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        (*ds).pending_buf_size as usize,
    );
    copy_pending_regions(source_pending, destination_pending, copy_layout.pending);
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateCopy"]

pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateCopy(dest, source)
}
struct LongestMatchInput {
    max_chain_length: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
}

struct LongestMatchResult {
    length: crate::stdlib::uInt,
    start: crate::stdlib::uInt,
}

fn longest_match_core(
    window: &[crate::stdlib::Bytef],
    prev: &[crate::src::deflate::Posf],
    input: &LongestMatchInput,
    mut cur_match: crate::src::deflate::IPos,
) -> LongestMatchResult {
    let mut chain_length = input.max_chain_length;
    let mut best_length = input.prev_length.min(input.lookahead);
    let mut best_start = input.match_start;
    let mut nice_match = input.nice_match.max(0) as crate::stdlib::uInt;
    let limit = if input.strstart
        > input
            .w_size
            .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
    {
        input.strstart.wrapping_sub(
            input
                .w_size
                .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        ) as crate::src::deflate::IPos
    } else {
        NIL as crate::src::deflate::IPos
    };
    let max_match = crate::zutil_h::MAX_MATCH as usize;
    let scan_start = input.strstart as usize;

    if input.prev_length >= input.good_match {
        chain_length >>= 2;
    }
    nice_match = nice_match.min(input.lookahead);

    while scan_start
        .checked_add(max_match)
        .is_some_and(|end| end <= window.len())
    {
        let candidate = cur_match as usize;
        let Some(candidate_end) = candidate.checked_add(max_match) else {
            break;
        };
        if candidate_end > window.len() {
            break;
        }

        let mut length = 0usize;
        while length < max_match && window[candidate + length] == window[scan_start + length] {
            length += 1;
        }
        if length > best_length as usize {
            best_length = length as crate::stdlib::uInt;
            best_start = cur_match as crate::stdlib::uInt;
            if best_length >= nice_match {
                break;
            }
        }

        let prev_index = (cur_match as crate::stdlib::uInt & input.w_mask) as usize;
        let Some(&next_match) = prev.get(prev_index) else {
            break;
        };
        cur_match = next_match as crate::src::deflate::IPos;
        if cur_match <= limit {
            break;
        }
        chain_length = chain_length.wrapping_sub(1);
        if chain_length == 0 {
            break;
        }
    }

    LongestMatchResult {
        length: best_length.min(input.lookahead),
        start: best_start,
    }
}

unsafe extern "C" fn longest_match(
    mut s: *mut crate::src::deflate::deflate_state,
    mut cur_match: crate::src::deflate::IPos,
) -> crate::stdlib::uInt {
    let state = &mut *s;
    // `window_size` and `w_size` are the exact capacities established by
    // deflateInit2_() and retained by deflateCopy().  Keep the raw views
    // bounded by those capacities before handing matching to the safe core.
    let window = ::core::slice::from_raw_parts(
        state.window.expect("initialized window").as_ptr(),
        state.window_size as usize,
    );
    let prev = ::core::slice::from_raw_parts(
        state.prev.expect("initialized prev table").as_ptr(),
        state.w_size as usize,
    );
    let result = longest_match_core(
        window,
        prev,
        &LongestMatchInput {
            max_chain_length: state.max_chain_length,
            strstart: state.strstart,
            w_size: state.w_size,
            w_mask: state.w_mask,
            prev_length: state.prev_length,
            good_match: state.good_match,
            nice_match: state.nice_match,
            lookahead: state.lookahead,
            match_start: state.match_start,
        },
        cur_match,
    );
    state.match_start = result.start;
    result.length
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

unsafe extern "C" fn deflate_stored(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    // Keep the opaque-state projection at this implementation boundary.  The
    // stored-mode policy below can then operate on fields through the scoped
    // Rust view instead of repeatedly dereferencing the ABI cursor.
    let state = &mut *s;
    // The state keeps a validated backlink to its caller stream for the
    // lifetime of this deflate invocation.  Project it once, so stored-mode
    // accounting below uses the scoped stream rather than repeatedly
    // dereferencing that external cursor.
    let stream = &mut *state.strm.as_ptr();
    // The backing window has the exact `window_size` established by
    // `deflateInit2_()` and retained by `deflateCopy()`. Retain this one
    // bounded view while stored blocks copy or slide its contents.
    let window = ::core::slice::from_raw_parts_mut(
        state.window.expect("initialized window").as_ptr(),
        state.window_size as usize,
    );
    // Both stored-block emissions below use the same declared pending
    // allocation. Keep it as a bounded view for the whole stored-mode call
    // instead of routing either emission back through the raw state API.
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state
            .pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        state.pending_buf_size as usize,
    );
    let mut min_block: ::core::ffi::c_uint = (if state
        .pending_buf_size
        .wrapping_sub(5 as crate::zutil_h::ulg)
        > state.w_size as crate::zutil_h::ulg
    {
        state.w_size as crate::zutil_h::ulg
    } else {
        state
            .pending_buf_size
            .wrapping_sub(5 as crate::zutil_h::ulg)
    }) as ::core::ffi::c_uint;
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used: ::core::ffi::c_uint = stream.avail_in as ::core::ffi::c_uint;
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        have = (state.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
            >> 3 as ::core::ffi::c_int;
        if stream.avail_out < have {
            break;
        }
        have = (stream.avail_out as ::core::ffi::c_uint).wrapping_sub(have);
        left = (state.strstart as ::core::ffi::c_long - state.block_start) as ::core::ffi::c_uint;
        if len as crate::zutil_h::ulg
            > (left as crate::zutil_h::ulg)
                .wrapping_add(stream.avail_in as crate::zutil_h::ulg)
        {
            len = (left as crate::stdlib::uInt).wrapping_add(stream.avail_in)
                as ::core::ffi::c_uint;
        }
        if len > have {
            len = have;
        }
        if len < min_block
            && (len == 0 as ::core::ffi::c_uint && flush != crate::zlib_h::Z_FINISH
                || flush == crate::zlib_h::Z_NO_FLUSH
                || len
                    != (left as crate::stdlib::uInt).wrapping_add(stream.avail_in))
        {
            break;
        }
        last = if flush == crate::zlib_h::Z_FINISH
            && len == (left as crate::stdlib::uInt).wrapping_add(stream.avail_in)
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        crate::src::trees::stored_block_bytes(
            pending_buf,
            &mut state.pending,
            &mut state.bi_buf,
            &mut state.bi_valid,
            &mut state.bi_used,
            &[],
            0 as crate::zutil_h::ulg,
            last,
        );
        set_stored_block_length(pending_buf, state.pending, len);
        flush_pending(state.strm.as_ptr());
        if left != 0 {
            if left > len {
                left = len;
            }
            let output = ::core::slice::from_raw_parts_mut(stream.next_out, left as usize);
            let start = state.block_start as usize;
            output.copy_from_slice(&window[start..start + left as usize]);
            stream.next_out = stream.next_out.wrapping_add(left as usize);
            stream.avail_out = stream.avail_out.wrapping_sub(left);
            stream.total_out = stream.total_out.wrapping_add(left as crate::stdlib::uLong);
            state.block_start += left as ::core::ffi::c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            let wrap = state.wrap;
            stream.avail_in = stream.avail_in.wrapping_sub(len);
            let input = ::core::slice::from_raw_parts(stream.next_in, len as usize);
            let next_in = input.as_ptr_range().end.cast_mut();
            let output = ::core::slice::from_raw_parts_mut(stream.next_out, len as usize);
            stream.adler = read_buf_bytes(input, output, stream.adler, wrap);
            stream.next_in = next_in;
            stream.total_in = stream.total_in.wrapping_add(len as crate::stdlib::uLong);
            stream.next_out = stream.next_out.wrapping_add(len as usize);
            stream.avail_out = stream.avail_out.wrapping_sub(len);
            stream.total_out = stream.total_out.wrapping_add(len as crate::stdlib::uLong);
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub(stream.avail_in as ::core::ffi::c_uint);
    if used != 0 {
        if used >= state.w_size {
            state.matches = 2 as crate::stdlib::uInt;
            let input = ::core::slice::from_raw_parts(
                stream.next_in.wrapping_sub(state.w_size as usize),
                state.w_size as usize,
            );
            window[..state.w_size as usize].copy_from_slice(input);
            state.strstart = state.w_size;
            state.insert = state.strstart;
        } else {
            if state
                .window_size
                .wrapping_sub(state.strstart as crate::zutil_h::ulg)
                <= used as crate::zutil_h::ulg
            {
                state.strstart = state.strstart.wrapping_sub(state.w_size);
                window.copy_within(
                    state.w_size as usize..state.w_size as usize + state.strstart as usize,
                    0,
                );
                if state.matches < 2 as crate::stdlib::uInt {
                    state.matches = state.matches.wrapping_add(1);
                }
                if state.insert > state.strstart {
                    state.insert = state.strstart;
                }
            }
            let input = ::core::slice::from_raw_parts(
                stream.next_in.wrapping_sub(used as usize),
                used as usize,
            );
            let start = state.strstart as usize;
            window[start..start + used as usize].copy_from_slice(input);
            state.strstart = state.strstart.wrapping_add(used);
            state.insert =
                state
                    .insert
                    .wrapping_add(if used > state.w_size.wrapping_sub(state.insert) {
                        (state.w_size as ::core::ffi::c_uint)
                            .wrapping_sub(state.insert as ::core::ffi::c_uint)
                    } else {
                        used
                    });
        }
        state.block_start = state.strstart as ::core::ffi::c_long;
    }
    if state.high_water < state.strstart as crate::zutil_h::ulg {
        state.high_water = state.strstart as crate::zutil_h::ulg;
    }
    if last != 0 {
        state.bi_used = 8 as ::core::ffi::c_int;
        return finish_done;
    }
    if flush != crate::zlib_h::Z_NO_FLUSH
        && flush != crate::zlib_h::Z_FINISH
        && stream.avail_in == 0 as crate::stdlib::uInt
        && state.strstart as ::core::ffi::c_long == state.block_start
    {
        return block_done;
    }
    have = state
        .window_size
        .wrapping_sub(state.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if stream.avail_in > have
        && state.block_start >= state.w_size as ::core::ffi::c_long
    {
        state.block_start -= state.w_size as ::core::ffi::c_long;
        state.strstart = state.strstart.wrapping_sub(state.w_size);
        window.copy_within(
            state.w_size as usize..state.w_size as usize + state.strstart as usize,
            0,
        );
        if state.matches < 2 as crate::stdlib::uInt {
            state.matches = state.matches.wrapping_add(1);
        }
        have = have.wrapping_add(state.w_size as ::core::ffi::c_uint);
        if state.insert > state.strstart {
            state.insert = state.strstart;
        }
    }
    if have > stream.avail_in {
        have = stream.avail_in as ::core::ffi::c_uint;
    }
    if have != 0 {
        let wrap = state.wrap;
        let start = state.strstart as usize;
        stream.avail_in = stream.avail_in.wrapping_sub(have);
        let input = ::core::slice::from_raw_parts(stream.next_in, have as usize);
        let next_in = input.as_ptr_range().end.cast_mut();
        let output = &mut window[start..start + have as usize];
        stream.adler = read_buf_bytes(input, output, stream.adler, wrap);
        stream.next_in = next_in;
        stream.total_in = stream.total_in.wrapping_add(have as crate::stdlib::uLong);
        state.strstart = state.strstart.wrapping_add(have);
        state.insert =
            state
                .insert
                .wrapping_add(if have > state.w_size.wrapping_sub(state.insert) {
                    (state.w_size as ::core::ffi::c_uint)
                        .wrapping_sub(state.insert as ::core::ffi::c_uint)
                } else {
                    have
                });
    }
    if state.high_water < state.strstart as crate::zutil_h::ulg {
        state.high_water = state.strstart as crate::zutil_h::ulg;
    }
    have = (state.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
        >> 3 as ::core::ffi::c_int;
    have = (if state
        .pending_buf_size
        .wrapping_sub(have as crate::zutil_h::ulg)
        > 65535 as crate::zutil_h::ulg
    {
        65535 as crate::zutil_h::ulg
    } else {
        state
            .pending_buf_size
            .wrapping_sub(have as crate::zutil_h::ulg)
    }) as ::core::ffi::c_uint;
    min_block = if have > state.w_size {
        state.w_size as ::core::ffi::c_uint
    } else {
        have
    };
    left = (state.strstart as ::core::ffi::c_long - state.block_start) as ::core::ffi::c_uint;
    if left >= min_block
        || (left != 0 || flush == crate::zlib_h::Z_FINISH)
            && flush != crate::zlib_h::Z_NO_FLUSH
            && stream.avail_in == 0 as crate::stdlib::uInt
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == crate::zlib_h::Z_FINISH
            && stream.avail_in == 0 as crate::stdlib::uInt
            && len == left
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let start = state.block_start as usize;
        let input = &window[start..start + len as usize];
        crate::src::trees::stored_block_bytes(
            pending_buf,
            &mut state.pending,
            &mut state.bi_buf,
            &mut state.bi_valid,
            &mut state.bi_used,
            input,
            len as crate::zutil_h::ulg,
            last,
        );
        state.block_start += len as ::core::ffi::c_long;
        flush_pending(state.strm.as_ptr());
    }
    if last != 0 {
        state.bi_used = 8 as ::core::ffi::c_int;
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
    let sym_buf_start = (*s).sym_buf_start;
    // `pending_buf` is the full allocation; symbols occupy its suffix after
    // the literal area. Keeping one full-capacity view avoids a raw cursor.
    let pending_buf = ::core::slice::from_raw_parts_mut(
        (*s).pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        (*s).pending_buf_size as usize,
    );
    let sym_buf = &mut pending_buf[sym_buf_start..];
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
            let window = ::core::slice::from_raw_parts(
                (*s).window.expect("initialized window").as_ptr(),
                (*s).window_size as usize,
            );
            let head = ::core::slice::from_raw_parts_mut(
                (*s).head.expect("initialized head table").as_ptr(),
                (*s).hash_size as usize,
            );
            let prev = ::core::slice::from_raw_parts_mut(
                (*s).prev.expect("initialized prev table").as_ptr(),
                (*s).w_size as usize,
            );
            ((*s).ins_h, hash_head) = insert_hash(
                window,
                head,
                prev,
                (*s).ins_h,
                (*s).hash_shift,
                (*s).hash_mask,
                (*s).w_mask,
                (*s).strstart,
            );
        }
        if hash_head != NIL as crate::src::deflate::IPos
            && ((*s).strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                <= (*s)
                    .w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
        {
            (*s).match_length = longest_match(s, hash_head);
        }
        if (*s).match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len: crate::zutil_h::uch =
                (*s).match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush =
                (*s).strstart.wrapping_sub((*s).match_start) as crate::zutil_h::ush;
            bflush = crate::src::trees::tally_symbol(
                sym_buf,
                &mut (*s).sym_next,
                (*s).sym_end,
                &mut (*s).dyn_ltree,
                &mut (*s).dyn_dtree,
                &mut (*s).matches,
                dist as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
            );
            (*s).lookahead = (*s).lookahead.wrapping_sub((*s).match_length);
            if (*s).match_length <= (*s).max_lazy_match
                && (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            {
                (*s).match_length = (*s).match_length.wrapping_sub(1);
                loop {
                    (*s).strstart = (*s).strstart.wrapping_add(1);
                    let window = ::core::slice::from_raw_parts(
                        (*s).window.expect("initialized window").as_ptr(),
                        (*s).window_size as usize,
                    );
                    let head = ::core::slice::from_raw_parts_mut(
                        (*s).head.expect("initialized head table").as_ptr(),
                        (*s).hash_size as usize,
                    );
                    let prev = ::core::slice::from_raw_parts_mut(
                        (*s).prev.expect("initialized prev table").as_ptr(),
                        (*s).w_size as usize,
                    );
                    ((*s).ins_h, hash_head) = insert_hash(
                        window,
                        head,
                        prev,
                        (*s).ins_h,
                        (*s).hash_shift,
                        (*s).hash_mask,
                        (*s).w_mask,
                        (*s).strstart,
                    );
                    (*s).match_length = (*s).match_length.wrapping_sub(1);
                    if (*s).match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                (*s).strstart = (*s).strstart.wrapping_add(1);
            } else {
                (*s).strstart = (*s).strstart.wrapping_add((*s).match_length);
                (*s).match_length = 0 as crate::stdlib::uInt;
                let window = ::core::slice::from_raw_parts(
                    (*s).window.expect("initialized window").as_ptr(),
                    (*s).window_size as usize,
                );
                (*s).ins_h = initial_hash(window, (*s).strstart, (*s).hash_shift, (*s).hash_mask);
            }
        } else {
            let mut cc: crate::zutil_h::uch = *(*s)
                .window
                .expect("initialized window")
                .as_ptr()
                .wrapping_add((*s).strstart as usize)
                as crate::zutil_h::uch;
            bflush = crate::src::trees::tally_symbol(
                sym_buf,
                &mut (*s).sym_next,
                (*s).sym_end,
                &mut (*s).dyn_ltree,
                &mut (*s).dyn_dtree,
                &mut (*s).matches,
                0,
                cc as ::core::ffi::c_uint,
            );
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            (*s).strstart = (*s).strstart.wrapping_add(1);
        }
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
                s as *mut crate::src::deflate::internal_state,
                if (*s).block_start >= 0 as ::core::ffi::c_long {
                    (*s).window
                        .expect("initialized window")
                        .as_ptr()
                        .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
                0 as ::core::ffi::c_int,
            );
            (*s).block_start = (*s).strstart as ::core::ffi::c_long;
            flush_pending((*s).strm.as_ptr());
            if (*(*s).strm.as_ptr()).avail_out == 0 as crate::stdlib::uInt {
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
                    .expect("initialized window")
                    .as_ptr()
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm.as_ptr());
        if (*(*s).strm.as_ptr()).avail_out == 0 as crate::stdlib::uInt {
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
                    .expect("initialized window")
                    .as_ptr()
                    .wrapping_add((*s).block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
            0 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm.as_ptr());
        if (*(*s).strm.as_ptr()).avail_out == 0 as crate::stdlib::uInt {
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
    // The legacy dispatcher still owns the ABI cursor.  Project it once so
    // the lazy-match state machine below only works through this scoped
    // state view and bounded allocation slices.
    let state = &mut *s;
    let sym_buf_start = state.sym_buf_start;
    // `pending_buf` is the full allocation; symbols occupy its suffix after
    // the literal area. Keeping one full-capacity view avoids a raw cursor.
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state.pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        state.pending_buf_size as usize,
    );
    let sym_buf = &mut pending_buf[sym_buf_start..];
    loop {
        if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window(s);
            if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if state.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        hash_head = NIL as crate::src::deflate::IPos;
        // `fill_window()` above has established the initialized extent for
        // this iteration. Reuse one bounded read view for both hashing and a
        // possible delayed literal instead of rebuilding raw views for each.
        let window = ::core::slice::from_raw_parts(
            state.window.expect("initialized window").as_ptr(),
            state.window_size as usize,
        );
        // The insertion step does not inspect the previous match fields, so
        // establish the lazy-match candidate before borrowing `prev`. This
        // lets the safe matcher reuse that same bounded hash-table view.
        state.prev_length = state.match_length;
        state.prev_match = state.match_start as crate::src::deflate::IPos;
        state.match_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let head = ::core::slice::from_raw_parts_mut(
                state.head.expect("initialized head table").as_ptr(),
                state.hash_size as usize,
            );
            let prev = ::core::slice::from_raw_parts_mut(
                state.prev.expect("initialized prev table").as_ptr(),
                state.w_size as usize,
            );
            (state.ins_h, hash_head) = insert_hash(
                window,
                head,
                prev,
                state.ins_h,
                state.hash_shift,
                state.hash_mask,
                state.w_mask,
                state.strstart,
            );
            if hash_head != NIL as crate::src::deflate::IPos
                && state.prev_length < state.max_lazy_match
                && (state.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                    <= state
                        .w_size
                        .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
            {
                let result = longest_match_core(
                    window,
                    prev,
                    &LongestMatchInput {
                        max_chain_length: state.max_chain_length,
                        strstart: state.strstart,
                        w_size: state.w_size,
                        w_mask: state.w_mask,
                        prev_length: state.prev_length,
                        good_match: state.good_match,
                        nice_match: state.nice_match,
                        lookahead: state.lookahead,
                        match_start: state.match_start,
                    },
                    hash_head,
                );
                state.match_start = result.start;
                state.match_length = result.length;
            }
        }
        if state.match_length <= 5 as crate::stdlib::uInt
            && (state.strategy == crate::zlib_h::Z_FILTERED
                || state.match_length == crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                    && state.strstart.wrapping_sub(state.match_start) > TOO_FAR as crate::stdlib::uInt)
        {
            state.match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        }
        let delayed_literal = if state.match_available != 0 {
            Some(window[state.strstart.wrapping_sub(1) as usize] as crate::zutil_h::uch)
        } else {
            None
        };
        let mut slow_state = SlowMatchState {
            prev_length: state.prev_length,
            match_length: state.match_length,
            prev_match: state.prev_match,
            match_available: state.match_available,
            strstart: state.strstart,
            lookahead: state.lookahead,
        };
        let action = advance_slow_match(
            &mut slow_state,
            sym_buf,
            &mut state.sym_next,
            state.sym_end,
            &mut state.dyn_ltree,
            &mut state.dyn_dtree,
            &mut state.matches,
            delayed_literal,
        );
        state.prev_length = slow_state.prev_length;
        state.match_length = slow_state.match_length;
        state.match_available = slow_state.match_available;
        state.strstart = slow_state.strstart;
        state.lookahead = slow_state.lookahead;
        match action {
            SlowMatchAction::Match {
                insert_start,
                insert_count,
                bflush: next_bflush,
            } => {
                bflush = next_bflush;
                for offset in 0..insert_count {
                    let window = ::core::slice::from_raw_parts(
                        state.window.expect("initialized window").as_ptr(),
                        state.window_size as usize,
                    );
                    let head = ::core::slice::from_raw_parts_mut(
                        state.head.expect("initialized head table").as_ptr(),
                        state.hash_size as usize,
                    );
                    let prev = ::core::slice::from_raw_parts_mut(
                        state.prev.expect("initialized prev table").as_ptr(),
                        state.w_size as usize,
                    );
                    (state.ins_h, hash_head) = insert_hash(
                        window,
                        head,
                        prev,
                        state.ins_h,
                        state.hash_shift,
                        state.hash_mask,
                        state.w_mask,
                        insert_start.wrapping_add(offset),
                    );
                }
                if bflush != 0 {
                    crate::src::trees::_tr_flush_block(
                        s as *mut crate::src::deflate::internal_state,
                        if state.block_start >= 0 as ::core::ffi::c_long {
                            state.window
                                .expect("initialized window")
                                .as_ptr()
                                .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                                as *mut crate::stdlib::charf
                        } else {
                            ::core::ptr::null_mut::<crate::stdlib::charf>()
                        },
                        (state.strstart as ::core::ffi::c_long - state.block_start)
                            as crate::zutil_h::ulg,
                        0,
                    );
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_pending(state.strm.as_ptr());
                    if (&*state.strm.as_ptr()).avail_out == 0 as crate::stdlib::uInt {
                        return need_more;
                    }
                }
            }
            SlowMatchAction::Literal(next_bflush) => {
                bflush = next_bflush;
                if bflush != 0 {
                    crate::src::trees::_tr_flush_block(
                        s as *mut crate::src::deflate::internal_state,
                        if state.block_start >= 0 as ::core::ffi::c_long {
                            state.window
                                .expect("initialized window")
                                .as_ptr()
                                .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                                as *mut crate::stdlib::charf
                        } else {
                            ::core::ptr::null_mut::<crate::stdlib::charf>()
                        },
                        (state.strstart as ::core::ffi::c_long - state.block_start)
                            as crate::zutil_h::ulg,
                        0,
                    );
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_pending(state.strm.as_ptr());
                }
                state.strstart = state.strstart.wrapping_add(1);
                state.lookahead = state.lookahead.wrapping_sub(1);
                if (&*state.strm.as_ptr()).avail_out == 0 as crate::stdlib::uInt {
                    return need_more;
                }
            }
            SlowMatchAction::Defer => {}
        }
    }
    if state.match_available != 0 {
        let window = ::core::slice::from_raw_parts(
            state.window.expect("initialized window").as_ptr(),
            state.window_size as usize,
        );
        let cc_0 = window[state.strstart.wrapping_sub(1) as usize] as crate::zutil_h::uch;
        bflush = tally_slow_symbol(
            sym_buf,
            &mut state.sym_next,
            state.sym_end,
            &mut state.dyn_ltree,
            &mut state.dyn_dtree,
            &mut state.matches,
            0,
            cc_0 as ::core::ffi::c_uint,
        );
        state.match_available = 0;
    }
    state.insert = if state.strstart
        < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        state.strstart
    } else {
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    };
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if state.block_start >= 0 as ::core::ffi::c_long {
                state.window
                    .expect("initialized window")
                    .as_ptr()
                    .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            1 as ::core::ffi::c_int,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        flush_pending(state.strm.as_ptr());
        if (&*state.strm.as_ptr()).avail_out == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if state.block_start >= 0 as ::core::ffi::c_long {
                state.window
                    .expect("initialized window")
                    .as_ptr()
                    .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            0 as ::core::ffi::c_int,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        flush_pending(state.strm.as_ptr());
        if (&*state.strm.as_ptr()).avail_out == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

// Find a repeated-byte match using offsets into the fully allocated sliding
// window.  `fill_window()` maintains the initialized window extent; the ABI
// caller only projects that allocation to a slice for this bounded kernel.
fn rle_match_length(
    window: &[crate::stdlib::Bytef],
    strstart: usize,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    if lookahead < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt || strstart == 0 {
        return 0;
    }

    let Some(&previous) = window.get(strstart - 1) else {
        return 0;
    };
    let mut scan = strstart - 1;
    for _ in 0..3 {
        scan += 1;
        if window.get(scan) != Some(&previous) {
            return 0;
        }
    }

    let strend = strstart + crate::zutil_h::MAX_MATCH as usize;
    loop {
        for _ in 0..8 {
            scan += 1;
            if window.get(scan) != Some(&previous) {
                let length = (crate::zutil_h::MAX_MATCH as usize)
                    .saturating_sub(strend.saturating_sub(scan));
                return (length as crate::stdlib::uInt).min(lookahead);
            }
        }
        if scan >= strend {
            return (crate::zutil_h::MAX_MATCH as crate::stdlib::uInt).min(lookahead);
        }
    }
}

unsafe extern "C" fn deflate_rle(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    let state = &mut *s;
    let sym_buf_start = state.sym_buf_start;
    // `pending_buf` is the full allocation; symbols occupy its suffix after
    // the literal area. Keeping one full-capacity view avoids a raw cursor.
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state
            .pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        state.pending_buf_size as usize,
    );
    let sym_buf = &mut pending_buf[sym_buf_start..];
    loop {
        if state.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_window(state as *mut crate::src::deflate::deflate_state);
            if state.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if state.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        state.match_length = 0 as crate::stdlib::uInt;
        // `window_size` is the full allocation capacity set by
        // `deflateInit2_()`/`deflateCopy()`, not merely the current input.
        // `rle_match_length()` uses checked slice accesses for the walk.
        let window = ::core::slice::from_raw_parts(
            state.window.expect("initialized window").as_ptr(),
            state.window_size as usize,
        );
        state.match_length = rle_match_length(window, state.strstart as usize, state.lookahead);
        if state.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len = state.match_length.wrapping_sub(3 as crate::stdlib::uInt);
            bflush = crate::src::trees::tally_symbol(
                sym_buf,
                &mut state.sym_next,
                state.sym_end,
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                &mut state.matches,
                1,
                len,
            );
            state.lookahead = state.lookahead.wrapping_sub(state.match_length);
            state.strstart = state.strstart.wrapping_add(state.match_length);
            state.match_length = 0 as crate::stdlib::uInt;
        } else {
            let cc = window[state.strstart as usize] as crate::zutil_h::uch;
            bflush = crate::src::trees::tally_symbol(
                sym_buf,
                &mut state.sym_next,
                state.sym_end,
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                &mut state.matches,
                0,
                cc as ::core::ffi::c_uint,
            );
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            let block = if state.block_start >= 0 as ::core::ffi::c_long {
                window[state.block_start as usize..state.strstart as usize]
                    .as_ptr()
                    .cast_mut()
                    .cast::<crate::stdlib::charf>()
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            };
            crate::src::trees::_tr_flush_block(
                state as *mut crate::src::deflate::internal_state,
                block,
                (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
                0 as ::core::ffi::c_int,
            );
            state.block_start = state.strstart as ::core::ffi::c_long;
            if flush_pending(state.strm.as_ptr()) == 0 as crate::stdlib::uInt {
                return (if false {
                    finish_started as ::core::ffi::c_int
                } else {
                    need_more as ::core::ffi::c_int
                }) as block_state;
            }
        }
    }
    state.insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block(
            state as *mut crate::src::deflate::internal_state,
            if state.block_start >= 0 as ::core::ffi::c_long {
                state.window
                    .expect("initialized window")
                    .as_ptr()
                    .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            1 as ::core::ffi::c_int,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        if flush_pending(state.strm.as_ptr()) == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        crate::src::trees::_tr_flush_block(
            state as *mut crate::src::deflate::internal_state,
            if state.block_start >= 0 as ::core::ffi::c_long {
                state.window
                    .expect("initialized window")
                    .as_ptr()
                    .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            0 as ::core::ffi::c_int,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        if flush_pending(state.strm.as_ptr()) == 0 as crate::stdlib::uInt {
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
    let state = &mut *s;
    let sym_buf_start = state.sym_buf_start;
    // `pending_buf` is the full allocation; symbols occupy its suffix after
    // the literal area. Keeping one full-capacity view avoids a raw cursor.
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state
            .pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        state.pending_buf_size as usize,
    );
    let sym_buf = &mut pending_buf[sym_buf_start..];
    loop {
        if state.lookahead == 0 as crate::stdlib::uInt {
            fill_window(state as *mut crate::src::deflate::deflate_state);
            if state.lookahead == 0 as crate::stdlib::uInt {
                if flush == crate::zlib_h::Z_NO_FLUSH {
                    return need_more;
                }
                break;
            }
        }
        state.match_length = 0 as crate::stdlib::uInt;
        let window = ::core::slice::from_raw_parts(
            state.window.expect("initialized window").as_ptr(),
            state.window_size as usize,
        );
        let cc = window[state.strstart as usize] as crate::zutil_h::uch;
        bflush = crate::src::trees::tally_symbol(
            sym_buf,
            &mut state.sym_next,
            state.sym_end,
            &mut state.dyn_ltree,
            &mut state.dyn_dtree,
            &mut state.matches,
            0,
            cc as ::core::ffi::c_uint,
        );
        state.lookahead = state.lookahead.wrapping_sub(1);
        state.strstart = state.strstart.wrapping_add(1);
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
                state as *mut crate::src::deflate::internal_state,
                if state.block_start >= 0 as ::core::ffi::c_long {
                    state
                        .window
                        .expect("initialized window")
                        .as_ptr()
                        .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
                0 as ::core::ffi::c_int,
            );
            state.block_start = state.strstart as ::core::ffi::c_long;
            if flush_pending(state.strm.as_ptr()) == 0 as crate::stdlib::uInt {
                return (if false {
                    finish_started as ::core::ffi::c_int
                } else {
                    need_more as ::core::ffi::c_int
                }) as block_state;
            }
        }
    }
    state.insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block(
            state as *mut crate::src::deflate::internal_state,
            if state.block_start >= 0 as ::core::ffi::c_long {
                state
                    .window
                    .expect("initialized window")
                    .as_ptr()
                    .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            1 as ::core::ffi::c_int,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        if flush_pending(state.strm.as_ptr()) == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        crate::src::trees::_tr_flush_block(
            state as *mut crate::src::deflate::internal_state,
            if state.block_start >= 0 as ::core::ffi::c_long {
                state
                    .window
                    .expect("initialized window")
                    .as_ptr()
                    .wrapping_add(state.block_start as ::core::ffi::c_uint as usize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            0 as ::core::ffi::c_int,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        if flush_pending(state.strm.as_ptr()) == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}
