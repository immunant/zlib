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
#[repr(C)]

pub struct internal_state {
    pub strm: crate::zlib_h::z_streamp,
    pub status: ::core::ffi::c_int,
    pub pending_buf: *mut crate::stdlib::Bytef,
    pub pending_buf_size: crate::zutil_h::ulg,
    // Cursor within `pending_buf`.  Keep this as an offset so copying/resetting a
    // stream never retains an interior raw pointer into the allocation.
    pub pending_out: usize,
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
pub use crate::src::trees::_tr_align;
pub use crate::src::trees::_tr_flush_block;
pub use crate::src::trees::tr_init;
pub use crate::src::trees::_tr_stored_block;
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
                state.w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            // `window` is allocated with exactly `window_size` bytes in
            // `deflateInit2_()` and `deflateCopy()`.
            let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
            slide_window_bytes(window, wsize, more);
            state.match_start = state.match_start.wrapping_sub(wsize);
            state.strstart = state.strstart.wrapping_sub(wsize);
            state.block_start -= wsize as ::core::ffi::c_long;
            if state.insert > state.strstart {
                state.insert = state.strstart;
            }
            // `head` and `prev` are allocated at these exact element counts
            // in `deflateInit2_()` and `deflateCopy()`.
            let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
            let prev = ::core::slice::from_raw_parts_mut(state.prev, wsize as usize);
            slide_hash_table(head, wsize);
            slide_hash_table(prev, wsize);
            state.slid = 1 as ::core::ffi::c_int;
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if (&*state.strm).avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        let stream = &mut *state.strm;
        n = stream.avail_in.min(more);
        if n != 0 {
            stream.avail_in = stream.avail_in.wrapping_sub(n);
            let input = ::core::slice::from_raw_parts(stream.next_in, n as usize);
            let next_in = input.as_ptr_range().end.cast_mut();
            // `window` is allocated with exactly `window_size` bytes in
            // `deflateInit2_()` and `deflateCopy()`, and this write is bounded
            // by the `more` capacity calculated above.
            let window =
                ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
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
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let prev = ::core::slice::from_raw_parts_mut(state.prev, wsize as usize);
            let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
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
            && (&*state.strm).avail_in != 0 as crate::stdlib::uInt)
        {
            break;
        }
    }
    if state.high_water < state.window_size {
        let mut curr: crate::zutil_h::ulg = (state.strstart as crate::zutil_h::ulg)
            .wrapping_add(state.lookahead as crate::zutil_h::ulg);
        let mut init: crate::zutil_h::ulg = 0;
        if state.high_water < curr {
            init = state.window_size.wrapping_sub(curr);
            if init > crate::src::deflate::WIN_INIT as crate::zutil_h::ulg {
                init = crate::src::deflate::WIN_INIT as crate::zutil_h::ulg;
            }
            let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
            clear_window_bytes(window, curr, init);
            state.high_water = curr.wrapping_add(init);
        } else if state.high_water
            < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
        {
            init = curr
                .wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
                .wrapping_sub(state.high_water);
            if init > state.window_size.wrapping_sub(state.high_water) {
                init = state.window_size.wrapping_sub(state.high_water);
            }
            let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
            clear_window_bytes(window, state.high_water, init);
            state.high_water = state.high_water.wrapping_add(init);
        }
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
    let mut wrap: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
    s = Some((*strm).zalloc.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
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
    (*strm).state = s as *mut crate::src::deflate::internal_state;
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
    (*s).window = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
            as crate::stdlib::uInt,
    ) as *mut crate::stdlib::Bytef;
    (*s).prev = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).w_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    (*s).head = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).hash_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    (*s).high_water = 0 as crate::zutil_h::ulg;
    (*s).lit_bufsize =
        ((1 as ::core::ffi::c_int) << memLevel + 6 as ::core::ffi::c_int) as crate::stdlib::uInt;
    (*s).pending_buf = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
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
    (*s).level = level;
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
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as ::core::ffi::c_int;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    if s.is_null()
        || (*s).strm != strm
        || (*s).status != crate::src::deflate::INIT_STATE
            && (*s).status != crate::src::deflate::GZIP_STATE
            && (*s).status != crate::src::deflate::EXTRA_STATE
            && (*s).status != crate::src::deflate::NAME_STATE
            && (*s).status != crate::src::deflate::COMMENT_STATE
            && (*s).status != crate::src::deflate::HCRC_STATE
            && (*s).status != crate::src::deflate::BUSY_STATE
            && (*s).status != crate::src::deflate::FINISH_STATE
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn deflateSetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut str: crate::stdlib::uInt = 0;
    let mut n: crate::stdlib::uInt = 0;
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
        (*strm).adler = crate::src::adler32::adler32(
            (*strm).adler,
            ::core::slice::from_raw_parts(dictionary, dictLength as usize),
        );
    }
    (*s).wrap = 0 as ::core::ffi::c_int;
    if dictLength >= (*s).w_size {
        if wrap == 0 as ::core::ffi::c_int {
            // `head` has exactly `hash_size` elements from `deflateInit2_()`
            // or `deflateCopy()`.
            let head = ::core::slice::from_raw_parts_mut((*s).head, (*s).hash_size as usize);
            clear_hash_table(head);
            (*s).slid = 0 as ::core::ffi::c_int;
            (*s).strstart = 0 as crate::stdlib::uInt;
            (*s).block_start = 0 as ::core::ffi::c_long;
            (*s).insert = 0 as crate::stdlib::uInt;
        }
        dictionary = dictionary.offset(dictLength.wrapping_sub((*s).w_size) as isize);
        dictLength = (*s).w_size;
    }
    avail = (*strm).avail_in as ::core::ffi::c_uint;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    (*strm).avail_in = dictLength;
    (*strm).next_in = dictionary as *mut crate::stdlib::Bytef;
    fill_window(s);
    while (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        str = (*s).strstart;
        n = (*s).lookahead.wrapping_sub(
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
        );
        loop {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset(
                    str.wrapping_add(3 as crate::stdlib::uInt)
                        .wrapping_sub(1 as crate::stdlib::uInt) as isize,
                ) as crate::stdlib::uInt)
                & (*s).hash_mask;
            *(*s).prev.offset((str & (*s).w_mask) as isize) =
                *(*s).head.offset((*s).ins_h as isize);
            *(*s).head.offset((*s).ins_h as isize) =
                str as crate::src::deflate::Pos as crate::src::deflate::Posf;
            str = str.wrapping_add(1);
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
        (*s).strstart = str;
        (*s).lookahead =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
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
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut len: crate::stdlib::uInt = 0;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    len = (*s).strstart.wrapping_add((*s).lookahead);
    if len > (*s).w_size {
        len = (*s).w_size;
    }
    if !dictionary.is_null() && len != 0 {
        crate::stdlib::memcpy(
            dictionary as *mut ::core::ffi::c_void,
            (*s).window
                .offset((*s).strstart as isize)
                .offset((*s).lookahead as isize)
                .offset(-(len as isize)) as *const ::core::ffi::c_void,
            len as crate::__stddef_size_t_h::size_t,
        );
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
        let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
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
    (*(*strm).state).gzhead = head;
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
pub unsafe extern "C" fn deflatePrime(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut put: ::core::ffi::c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || (*s).pending_out.wrapping_add(
            (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                as usize,
        ) > (*s).lit_bufsize as usize
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    loop {
        put = crate::src::deflate::Buf_size - (*s).bi_valid;
        if put > bits {
            put = bits;
        }
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | ((value & ((1 as ::core::ffi::c_int) << put) - 1 as ::core::ffi::c_int)
                << (*s).bi_valid) as crate::zutil_h::ush as ::core::ffi::c_int)
            as crate::zutil_h::ush;
        (*s).bi_valid += put;
        crate::src::trees::bi_flush_or_windup(
            s as *mut crate::src::deflate::internal_state,
            false,
        );
        value >>= put;
        bits -= put;
        if bits == 0 {
            break;
        }
    }
    return crate::zlib_h::Z_OK;
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
                let head =
                    ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
                let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
                slide_hash_table(head, state.w_size);
                slide_hash_table(prev, state.w_size);
                state.slid = 1 as ::core::ffi::c_int;
            } else {
                // `head` has exactly `hash_size` elements from
                // `deflateInit2_()` or `deflateCopy()`.
                let head = ::core::slice::from_raw_parts_mut((*s).head, (*s).hash_size as usize);
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
        if !state.gzhead.is_null() {
            let header = &*state.gzhead;
            if !header.extra.is_null() {
                gzip_extra_len = Some(header.extra_len);
            }
            if !header.name.is_null() {
                gzip_name_len = Some(
                    ::std::ffi::CStr::from_ptr(header.name.cast())
                        .to_bytes_with_nul()
                        .len(),
                );
            }
            if !header.comment.is_null() {
                gzip_comment_len = Some(
                    ::std::ffi::CStr::from_ptr(header.comment.cast())
                        .to_bytes_with_nul()
                        .len(),
                );
            }
            gzip_hcrc = header.hcrc != 0;
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

unsafe extern "C" fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    let mut len: ::core::ffi::c_uint = 0;
    // The stream and its opaque state are distinct allocations.  Project each
    // once so the bounded pending/output helpers operate on Rust references
    // rather than repeatedly dereferencing the ABI pointers.
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::deflate::deflate_state);
    crate::src::trees::bi_flush_or_windup(
        state as *mut crate::src::deflate::internal_state,
        false,
    );
    len = if state.pending > strm.avail_out as crate::zutil_h::ulg {
        strm.avail_out as ::core::ffi::c_uint
    } else {
        state.pending as ::core::ffi::c_uint
    };
    if len == 0 as ::core::ffi::c_uint {
        return;
    }
    let output = ::core::slice::from_raw_parts_mut(strm.next_out, len as usize);
    let pending_buf =
        ::core::slice::from_raw_parts(state.pending_buf, state.pending_buf_size as usize);
    let len = flush_pending_bytes(
        output,
        pending_buf,
        &mut state.pending_out,
        &mut state.pending,
    );
    strm.next_out = strm.next_out.offset(len as isize);
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out = strm.avail_out.wrapping_sub(len);
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
        (*strm).msg = crate::src::zutil::z_errmsg[(if (-2 as ::core::ffi::c_int)
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
    if (*strm).avail_out == 0 as crate::stdlib::uInt {
        (*strm).msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
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
    old_flush = (*s).last_flush;
    (*s).last_flush = flush;
    if (*s).pending != 0 as crate::zutil_h::ulg {
        flush_pending(strm);
        if (*strm).avail_out == 0 as crate::stdlib::uInt {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    } else if (*strm).avail_in == 0 as crate::stdlib::uInt
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
        (*strm).msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
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
    if (*s).status == crate::src::deflate::FINISH_STATE
        && (*strm).avail_in != 0 as crate::stdlib::uInt
    {
        (*strm).msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
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
    if (*s).status == crate::src::deflate::INIT_STATE && (*s).wrap == 0 as ::core::ffi::c_int {
        (*s).status = crate::src::deflate::BUSY_STATE;
    }
    if (*s).status == crate::src::deflate::INIT_STATE {
        let mut header: crate::stdlib::uInt =
            (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt).wrapping_add(
                (*s).w_bits.wrapping_sub(8 as crate::stdlib::uInt) << 4 as ::core::ffi::c_int,
            ) << 8 as ::core::ffi::c_int;
        let mut level_flags: crate::stdlib::uInt = 0;
        if (*s).strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || (*s).level < 2 as ::core::ffi::c_int {
            level_flags = 0 as crate::stdlib::uInt;
        } else if (*s).level < 6 as ::core::ffi::c_int {
            level_flags = 1 as crate::stdlib::uInt;
        } else if (*s).level == 6 as ::core::ffi::c_int {
            level_flags = 2 as crate::stdlib::uInt;
        } else {
            level_flags = 3 as crate::stdlib::uInt;
        }
        header |= level_flags << 6 as ::core::ffi::c_int;
        if (*s).strstart != 0 as crate::stdlib::uInt {
            header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
        }
        header = header.wrapping_add(
            (31 as crate::stdlib::uInt)
                .wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
        );
        let has_dictionary = (*s).strstart != 0 as crate::stdlib::uInt;
        let dictionary_adler = (*strm).adler;
        {
            let state = &mut *s;
            // `pending_buf` has exactly `pending_buf_size` bytes (allocated in
            // `deflateInit2_()` and copied at that extent in `deflateCopy()`).
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
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
        (*strm).adler = crate::src::adler32::adler32_z(0 as crate::stdlib::uLong, None);
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if (*s).status == crate::src::deflate::GZIP_STATE {
        (*strm).adler = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None);
        let state = &mut *s;
        // This initial gzip header always fits in the pending allocation. Keep
        // a single exact-capacity view for the contiguous write sequence.
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        push_pending_byte(pending_buf, &mut state.pending, 31);
        push_pending_byte(pending_buf, &mut state.pending, 139);
        push_pending_byte(pending_buf, &mut state.pending, 8);
        let xfl = if state.level == 9 as ::core::ffi::c_int {
            2
        } else if state.strategy >= 2 as ::core::ffi::c_int
            || state.level < 2 as ::core::ffi::c_int
        {
            4
        } else {
            0
        };
        if state.gzhead.is_null() {
            for byte in [0, 0, 0, 0, 0, xfl, 3] {
                push_pending_byte(pending_buf, &mut state.pending, byte);
            }
            (*s).status = crate::src::deflate::BUSY_STATE;
            flush_pending(strm);
            if (*s).pending != 0 as crate::zutil_h::ulg {
                (*s).last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        } else {
            let gzhead = &*state.gzhead;
            push_pending_byte(
                pending_buf,
                &mut state.pending,
                ((if gzhead.text != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) + (if gzhead.hcrc != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) + (if gzhead.extra.is_null() {
                    0 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                }) + (if gzhead.name.is_null() {
                    0 as ::core::ffi::c_int
                } else {
                    8 as ::core::ffi::c_int
                }) + (if gzhead.comment.is_null() {
                    0 as ::core::ffi::c_int
                } else {
                    16 as ::core::ffi::c_int
                })) as crate::stdlib::Bytef,
            );
            for shift in [0, 8, 16, 24] {
                push_pending_byte(
                    pending_buf,
                    &mut state.pending,
                    (gzhead.time >> shift & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                );
            }
            push_pending_byte(pending_buf, &mut state.pending, xfl);
            push_pending_byte(
                pending_buf,
                &mut state.pending,
                (gzhead.os & 0xff as ::core::ffi::c_int) as crate::stdlib::Bytef,
            );
            if !gzhead.extra.is_null() {
                push_pending_byte(
                    pending_buf,
                    &mut state.pending,
                    (gzhead.extra_len & 0xff as crate::stdlib::uInt) as crate::stdlib::Bytef,
                );
                push_pending_byte(
                    pending_buf,
                    &mut state.pending,
                    (gzhead.extra_len >> 8 & 0xff as crate::stdlib::uInt) as crate::stdlib::Bytef,
                );
            }
            if gzhead.hcrc != 0 {
                (*strm).adler = crate::src::crc32::crc32_z(
                    (*strm).adler,
                    Some(&pending_buf[..state.pending as usize]),
                );
            }
            state.gzindex = 0 as crate::zutil_h::ulg;
            state.status = crate::src::deflate::EXTRA_STATE;
        }
    }
    if (*s).status == crate::src::deflate::EXTRA_STATE {
        if !(*(*s).gzhead).extra.is_null() {
            let gzhead = &*(*s).gzhead;
            let hcrc = gzhead.hcrc != 0;
            let mut left: crate::zutil_h::ulg =
                ((gzhead.extra_len & 0xffff as crate::stdlib::uInt) as crate::zutil_h::ulg)
                    .wrapping_sub((*s).gzindex);
            while (*s).pending.wrapping_add(left) > (*s).pending_buf_size {
                let mut copy: crate::zutil_h::ulg =
                    (*s).pending_buf_size.wrapping_sub((*s).pending);
                // The source view is limited to the still-unemitted bytes,
                // not the whole foreign header allocation.  The destination
                // allocation has exactly `pending_buf_size` bytes.
                let extra = ::core::slice::from_raw_parts(
                    gzhead.extra.add((*s).gzindex as usize),
                    left as usize,
                );
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    (*s).pending_buf,
                    (*s).pending_buf_size as usize,
                );
                append_pending_bytes(pending_buf, &mut (*s).pending, &extra[..copy as usize]);
                if hcrc {
                    (*strm).adler = crate::src::crc32::crc32_z(
                        (*strm).adler,
                        Some(&extra[..copy as usize]),
                    );
                }
                (*s).gzindex = (*s).gzindex.wrapping_add(copy);
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
                left = left.wrapping_sub(copy);
            }
            if left != 0 {
                let extra = ::core::slice::from_raw_parts(
                    gzhead.extra.add((*s).gzindex as usize),
                    left as usize,
                );
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    (*s).pending_buf,
                    (*s).pending_buf_size as usize,
                );
                append_pending_bytes(pending_buf, &mut (*s).pending, extra);
                if hcrc {
                    (*strm).adler = crate::src::crc32::crc32_z((*strm).adler, Some(extra));
                }
            }
            (*s).gzindex = 0 as crate::zutil_h::ulg;
        }
        (*s).status = crate::src::deflate::NAME_STATE;
    }
    if (*s).status == crate::src::deflate::NAME_STATE {
        if !(*(*s).gzhead).name.is_null() {
            let mut beg_0: crate::zutil_h::ulg = (*s).pending;
            let mut val: ::core::ffi::c_int = 0;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_0 {
                        (*strm).adler = crate::src::crc32::crc32_z(
                            (*strm).adler,
                            Some(::core::slice::from_raw_parts(
                                (*s).pending_buf.offset(beg_0 as isize),
                                ((*s).pending as usize).wrapping_sub(beg_0 as usize),
                            )),
                        );
                    }
                    flush_pending(strm);
                    if (*s).pending != 0 as crate::zutil_h::ulg {
                        (*s).last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                    beg_0 = 0 as crate::zutil_h::ulg;
                }
                let c2rust_fresh19 = (*s).gzindex;
                (*s).gzindex = (*s).gzindex.wrapping_add(1);
                val = *(*(*s).gzhead).name.offset(c2rust_fresh19 as isize) as ::core::ffi::c_int;
                let c2rust_fresh20 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(c2rust_fresh20 as isize) = val as crate::stdlib::Bytef;
                if val == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_0 {
                (*strm).adler = crate::src::crc32::crc32_z(
                    (*strm).adler,
                    Some(::core::slice::from_raw_parts(
                        (*s).pending_buf.offset(beg_0 as isize),
                        ((*s).pending as usize).wrapping_sub(beg_0 as usize),
                    )),
                );
            }
            (*s).gzindex = 0 as crate::zutil_h::ulg;
        }
        (*s).status = crate::src::deflate::COMMENT_STATE;
    }
    if (*s).status == crate::src::deflate::COMMENT_STATE {
        if !(*(*s).gzhead).comment.is_null() {
            let mut beg_1: crate::zutil_h::ulg = (*s).pending;
            let mut val_0: ::core::ffi::c_int = 0;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_1 {
                        (*strm).adler = crate::src::crc32::crc32_z(
                            (*strm).adler,
                            Some(::core::slice::from_raw_parts(
                                (*s).pending_buf.offset(beg_1 as isize),
                                ((*s).pending as usize).wrapping_sub(beg_1 as usize),
                            )),
                        );
                    }
                    flush_pending(strm);
                    if (*s).pending != 0 as crate::zutil_h::ulg {
                        (*s).last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                    beg_1 = 0 as crate::zutil_h::ulg;
                }
                let c2rust_fresh21 = (*s).gzindex;
                (*s).gzindex = (*s).gzindex.wrapping_add(1);
                val_0 =
                    *(*(*s).gzhead).comment.offset(c2rust_fresh21 as isize) as ::core::ffi::c_int;
                let c2rust_fresh22 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(c2rust_fresh22 as isize) = val_0 as crate::stdlib::Bytef;
                if val_0 == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_1 {
                (*strm).adler = crate::src::crc32::crc32_z(
                    (*strm).adler,
                    Some(::core::slice::from_raw_parts(
                        (*s).pending_buf.offset(beg_1 as isize),
                        ((*s).pending as usize).wrapping_sub(beg_1 as usize),
                    )),
                );
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
            let c2rust_fresh23 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh23 as isize) =
                ((*strm).adler & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte;
            let c2rust_fresh24 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh24 as isize) =
                ((*strm).adler >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte;
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
                crate::src::trees::_tr_align(s as *mut crate::src::deflate::internal_state);
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
                    let head =
                        ::core::slice::from_raw_parts_mut((*s).head, (*s).hash_size as usize);
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
        let pending_buf =
            ::core::slice::from_raw_parts_mut((*s).pending_buf, (*s).pending_buf_size as usize);
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
                state.pending_buf,
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
    (*ds).pending_out = (*ss).pending_out;
    crate::stdlib::memcpy(
        (*ds).pending_buf.offset((*ds).pending_out as isize) as *mut ::core::ffi::c_void,
        (*ss).pending_buf.offset((*ss).pending_out as isize) as *const ::core::ffi::c_void,
        (*ss).pending as crate::__stddef_size_t_h::size_t,
    );
    crate::stdlib::memcpy(
        (*ds).pending_buf.offset((*ds).sym_buf_start as isize) as *mut ::core::ffi::c_void,
        (*ss).pending_buf.offset((*ss).sym_buf_start as isize) as *const ::core::ffi::c_void,
        (*ss).sym_next as crate::__stddef_size_t_h::size_t,
    );
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
    let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
    let prev = ::core::slice::from_raw_parts(state.prev, state.w_size as usize);
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
    let mut min_block: ::core::ffi::c_uint =
        (if state.pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg)
            > state.w_size as crate::zutil_h::ulg
        {
            state.w_size as crate::zutil_h::ulg
        } else {
            state.pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg)
        }) as ::core::ffi::c_uint;
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used: ::core::ffi::c_uint = (*state.strm).avail_in as ::core::ffi::c_uint;
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        have = (state.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
            >> 3 as ::core::ffi::c_int;
        if (*state.strm).avail_out < have {
            break;
        }
        have = ((*state.strm).avail_out as ::core::ffi::c_uint).wrapping_sub(have);
        left = (state.strstart as ::core::ffi::c_long - state.block_start) as ::core::ffi::c_uint;
        if len as crate::zutil_h::ulg
            > (left as crate::zutil_h::ulg)
                .wrapping_add((*state.strm).avail_in as crate::zutil_h::ulg)
        {
            len = (left as crate::stdlib::uInt).wrapping_add((*state.strm).avail_in)
                as ::core::ffi::c_uint;
        }
        if len > have {
            len = have;
        }
        if len < min_block
            && (len == 0 as ::core::ffi::c_uint && flush != crate::zlib_h::Z_FINISH
                || flush == crate::zlib_h::Z_NO_FLUSH
                || len != (left as crate::stdlib::uInt).wrapping_add((*state.strm).avail_in))
        {
            break;
        }
        last = if flush == crate::zlib_h::Z_FINISH
            && len == (left as crate::stdlib::uInt).wrapping_add((*state.strm).avail_in)
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        crate::src::trees::_tr_stored_block(
            s as *mut crate::src::deflate::internal_state,
            ::core::ptr::null_mut::<crate::stdlib::charf>(),
            0 as crate::zutil_h::ulg,
            last,
        );
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        set_stored_block_length(pending_buf, state.pending, len);
        flush_pending(state.strm);
        if left != 0 {
            if left > len {
                left = len;
            }
            crate::stdlib::memcpy(
                (*state.strm).next_out as *mut ::core::ffi::c_void,
                state.window.offset(state.block_start as isize) as *const ::core::ffi::c_void,
                left as crate::__stddef_size_t_h::size_t,
            );
            (*state.strm).next_out = (*state.strm).next_out.offset(left as isize);
            (*state.strm).avail_out = (*state.strm).avail_out.wrapping_sub(left);
            (*state.strm).total_out = (*state.strm)
                .total_out
                .wrapping_add(left as crate::stdlib::uLong);
            state.block_start += left as ::core::ffi::c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            let wrap = state.wrap;
            let stream = &mut *state.strm;
            stream.avail_in = stream.avail_in.wrapping_sub(len);
            let input = ::core::slice::from_raw_parts(stream.next_in, len as usize);
            let next_in = input.as_ptr_range().end.cast_mut();
            let output = ::core::slice::from_raw_parts_mut(stream.next_out, len as usize);
            stream.adler = read_buf_bytes(input, output, stream.adler, wrap);
            stream.next_in = next_in;
            stream.total_in = stream.total_in.wrapping_add(len as crate::stdlib::uLong);
            stream.next_out = stream.next_out.offset(len as isize);
            stream.avail_out = stream.avail_out.wrapping_sub(len);
            stream.total_out = stream.total_out.wrapping_add(len as crate::stdlib::uLong);
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub((*state.strm).avail_in as ::core::ffi::c_uint);
    if used != 0 {
        if used >= state.w_size {
            state.matches = 2 as crate::stdlib::uInt;
            crate::stdlib::memcpy(
                state.window as *mut ::core::ffi::c_void,
                (*state.strm).next_in.offset(-(state.w_size as isize)) as *const ::core::ffi::c_void,
                state.w_size as crate::__stddef_size_t_h::size_t,
            );
            state.strstart = state.w_size;
            state.insert = state.strstart;
        } else {
            if state
                .window_size
                .wrapping_sub(state.strstart as crate::zutil_h::ulg)
                <= used as crate::zutil_h::ulg
            {
                state.strstart = state.strstart.wrapping_sub(state.w_size);
                crate::stdlib::memcpy(
                    state.window as *mut ::core::ffi::c_void,
                    state.window.offset(state.w_size as isize) as *const ::core::ffi::c_void,
                    state.strstart as crate::__stddef_size_t_h::size_t,
                );
                if state.matches < 2 as crate::stdlib::uInt {
                    state.matches = state.matches.wrapping_add(1);
                }
                if state.insert > state.strstart {
                    state.insert = state.strstart;
                }
            }
            crate::stdlib::memcpy(
                state.window.offset(state.strstart as isize) as *mut ::core::ffi::c_void,
                (*state.strm).next_in.offset(-(used as isize)) as *const ::core::ffi::c_void,
                used as crate::__stddef_size_t_h::size_t,
            );
            state.strstart = state.strstart.wrapping_add(used);
            state.insert =
                state.insert
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
        && (*state.strm).avail_in == 0 as crate::stdlib::uInt
        && state.strstart as ::core::ffi::c_long == state.block_start
    {
        return block_done;
    }
    have = state
        .window_size
        .wrapping_sub(state.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if (*state.strm).avail_in > have && state.block_start >= state.w_size as ::core::ffi::c_long {
        state.block_start -= state.w_size as ::core::ffi::c_long;
        state.strstart = state.strstart.wrapping_sub(state.w_size);
        crate::stdlib::memcpy(
            state.window as *mut ::core::ffi::c_void,
            state.window.offset(state.w_size as isize) as *const ::core::ffi::c_void,
            state.strstart as crate::__stddef_size_t_h::size_t,
        );
        if state.matches < 2 as crate::stdlib::uInt {
            state.matches = state.matches.wrapping_add(1);
        }
        have = have.wrapping_add(state.w_size as ::core::ffi::c_uint);
        if state.insert > state.strstart {
            state.insert = state.strstart;
        }
    }
    if have > (*state.strm).avail_in {
        have = (*state.strm).avail_in as ::core::ffi::c_uint;
    }
    if have != 0 {
        let wrap = state.wrap;
        let start = state.strstart as usize;
        let stream = &mut *state.strm;
        stream.avail_in = stream.avail_in.wrapping_sub(have);
        let input = ::core::slice::from_raw_parts(stream.next_in, have as usize);
        let next_in = input.as_ptr_range().end.cast_mut();
        // `window` is allocated with exactly `window_size` bytes in
        // `deflateInit2_()` and `deflateCopy()`, and `have` is capped by the
        // remaining capacity from `strstart` above.
        let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
        let output = &mut window[start..start + have as usize];
        stream.adler = read_buf_bytes(input, output, stream.adler, wrap);
        stream.next_in = next_in;
        stream.total_in = stream.total_in.wrapping_add(have as crate::stdlib::uLong);
        state.strstart = state.strstart.wrapping_add(have);
        state.insert = state
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
        state.pending_buf_size
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
            && (*state.strm).avail_in == 0 as crate::stdlib::uInt
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == crate::zlib_h::Z_FINISH
            && (*state.strm).avail_in == 0 as crate::stdlib::uInt
            && len == left
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        crate::src::trees::_tr_stored_block(
            s as *mut crate::src::deflate::internal_state,
            (state.window as *mut crate::stdlib::charf).offset(state.block_start as isize),
            len as crate::zutil_h::ulg,
            last,
        );
        state.block_start += len as ::core::ffi::c_long;
        flush_pending(state.strm);
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
    let sym_buf_len = ((*s).pending_buf_size as usize).wrapping_sub(sym_buf_start);
    let sym_buf = ::core::slice::from_raw_parts_mut(
        (*s).pending_buf.wrapping_add(sym_buf_start),
        sym_buf_len,
    );
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
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset((*s).strstart.wrapping_add(
                    (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
                ) as isize) as crate::stdlib::uInt)
                & (*s).hash_mask;
            *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize) =
                *(*s).head.offset((*s).ins_h as isize);
            hash_head = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize)
                as crate::src::deflate::IPos;
            *(*s).head.offset((*s).ins_h as isize) =
                (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
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
            let mut len: crate::zutil_h::uch =
                (*s).match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let mut dist: crate::zutil_h::ush =
                (*s).strstart.wrapping_sub((*s).match_start) as crate::zutil_h::ush;
            let c2rust_fresh44 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh44 as usize] = dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh45 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh45 as usize] =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh46 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh46 as usize] = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as usize;
            let distance_code = if dist < 256 {
                crate::src::trees::_dist_code[dist as usize] as usize
            } else {
                crate::src::trees::_dist_code[256 + (dist as usize >> 7)] as usize
            };
            let length_entry = length_code + crate::src::deflate::LITERALS as usize + 1;
            (*s).dyn_ltree[length_entry].fc = (*s).dyn_ltree[length_entry].fc.wrapping_add(1);
            (*s).dyn_dtree[distance_code].fc = (*s).dyn_dtree[distance_code].fc.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub((*s).match_length);
            if (*s).match_length <= (*s).max_lazy_match
                && (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            {
                (*s).match_length = (*s).match_length.wrapping_sub(1);
                loop {
                    (*s).strstart = (*s).strstart.wrapping_add(1);
                    (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                        ^ *(*s).window.offset((*s).strstart.wrapping_add(
                            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as crate::stdlib::uInt,
                        ) as isize) as crate::stdlib::uInt)
                        & (*s).hash_mask;
                    *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize) =
                        *(*s).head.offset((*s).ins_h as isize);
                    hash_head = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize)
                        as crate::src::deflate::IPos;
                    *(*s).head.offset((*s).ins_h as isize) =
                        (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
                    (*s).match_length = (*s).match_length.wrapping_sub(1);
                    if (*s).match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                (*s).strstart = (*s).strstart.wrapping_add(1);
            } else {
                (*s).strstart = (*s).strstart.wrapping_add((*s).match_length);
                (*s).match_length = 0 as crate::stdlib::uInt;
                (*s).ins_h = *(*s).window.offset((*s).strstart as isize) as crate::stdlib::uInt;
                (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                    ^ *(*s)
                        .window
                        .offset((*s).strstart.wrapping_add(1 as crate::stdlib::uInt) as isize)
                        as crate::stdlib::uInt)
                    & (*s).hash_mask;
            }
        } else {
            let mut cc: crate::zutil_h::uch =
                *(*s).window.offset((*s).strstart as isize) as crate::zutil_h::uch;
            let c2rust_fresh47 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh47 as usize] = 0 as crate::zutil_h::uchf;
            let c2rust_fresh48 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh48 as usize] = 0 as crate::zutil_h::uchf;
            let c2rust_fresh49 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh49 as usize] = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc = (*s).dyn_ltree[cc as usize].fc.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            (*s).strstart = (*s).strstart.wrapping_add(1);
        }
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
                s as *mut crate::src::deflate::internal_state,
                if (*s).block_start >= 0 as ::core::ffi::c_long {
                    (*s).window
                        .offset((*s).block_start as ::core::ffi::c_uint as isize)
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
    let sym_buf_start = (*s).sym_buf_start;
    let sym_buf_len = ((*s).pending_buf_size as usize).wrapping_sub(sym_buf_start);
    let sym_buf = ::core::slice::from_raw_parts_mut(
        (*s).pending_buf.wrapping_add(sym_buf_start),
        sym_buf_len,
    );
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
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset((*s).strstart.wrapping_add(
                    (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
                ) as isize) as crate::stdlib::uInt)
                & (*s).hash_mask;
            *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize) =
                *(*s).head.offset((*s).ins_h as isize);
            hash_head = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize)
                as crate::src::deflate::IPos;
            *(*s).head.offset((*s).ins_h as isize) =
                (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
        }
        (*s).prev_length = (*s).match_length;
        (*s).prev_match = (*s).match_start as crate::src::deflate::IPos;
        (*s).match_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        if hash_head != NIL as crate::src::deflate::IPos
            && (*s).prev_length < (*s).max_lazy_match
            && ((*s).strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                <= (*s)
                    .w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
        {
            (*s).match_length = longest_match(s, hash_head);
            if (*s).match_length <= 5 as crate::stdlib::uInt
                && ((*s).strategy == crate::zlib_h::Z_FILTERED
                    || (*s).match_length == crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                        && (*s).strstart.wrapping_sub((*s).match_start)
                            > TOO_FAR as crate::stdlib::uInt)
            {
                (*s).match_length =
                    (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            }
        }
        if (*s).prev_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            && (*s).match_length <= (*s).prev_length
        {
            let mut max_insert: crate::stdlib::uInt = (*s)
                .strstart
                .wrapping_add((*s).lookahead)
                .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
            let mut len: crate::zutil_h::uch =
                (*s).prev_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let mut dist: crate::zutil_h::ush = ((*s).strstart as crate::src::deflate::IPos)
                .wrapping_sub(1 as crate::src::deflate::IPos)
                .wrapping_sub((*s).prev_match)
                as crate::zutil_h::ush;
            let c2rust_fresh35 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh35 as usize] = dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh36 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh36 as usize] =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh37 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh37 as usize] = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as usize;
            let distance_code = if dist < 256 {
                crate::src::trees::_dist_code[dist as usize] as usize
            } else {
                crate::src::trees::_dist_code[256 + (dist as usize >> 7)] as usize
            };
            let length_entry = length_code + crate::src::deflate::LITERALS as usize + 1;
            (*s).dyn_ltree[length_entry].fc = (*s).dyn_ltree[length_entry].fc.wrapping_add(1);
            (*s).dyn_dtree[distance_code].fc = (*s).dyn_dtree[distance_code].fc.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s)
                .lookahead
                .wrapping_sub((*s).prev_length.wrapping_sub(1 as crate::stdlib::uInt));
            (*s).prev_length = (*s).prev_length.wrapping_sub(2 as crate::stdlib::uInt);
            loop {
                (*s).strstart = (*s).strstart.wrapping_add(1);
                if (*s).strstart <= max_insert {
                    (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                        ^ *(*s).window.offset((*s).strstart.wrapping_add(
                            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as crate::stdlib::uInt,
                        ) as isize) as crate::stdlib::uInt)
                        & (*s).hash_mask;
                    *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize) =
                        *(*s).head.offset((*s).ins_h as isize);
                    hash_head = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize)
                        as crate::src::deflate::IPos;
                    *(*s).head.offset((*s).ins_h as isize) =
                        (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
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
                            .offset((*s).block_start as ::core::ffi::c_uint as isize)
                            as *mut crate::stdlib::charf
                    } else {
                        ::core::ptr::null_mut::<crate::stdlib::charf>()
                    },
                    ((*s).strstart as ::core::ffi::c_long - (*s).block_start)
                        as crate::zutil_h::ulg,
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
            let mut cc: crate::zutil_h::uch = *(*s)
                .window
                .offset((*s).strstart.wrapping_sub(1 as crate::stdlib::uInt) as isize)
                as crate::zutil_h::uch;
            let c2rust_fresh38 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh38 as usize] = 0 as crate::zutil_h::uchf;
            let c2rust_fresh39 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh39 as usize] = 0 as crate::zutil_h::uchf;
            let c2rust_fresh40 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh40 as usize] = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc = (*s).dyn_ltree[cc as usize].fc.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            if bflush != 0 {
                crate::src::trees::_tr_flush_block(
                    s as *mut crate::src::deflate::internal_state,
                    if (*s).block_start >= 0 as ::core::ffi::c_long {
                        (*s).window
                            .offset((*s).block_start as ::core::ffi::c_uint as isize)
                            as *mut crate::stdlib::charf
                    } else {
                        ::core::ptr::null_mut::<crate::stdlib::charf>()
                    },
                    ((*s).strstart as ::core::ffi::c_long - (*s).block_start)
                        as crate::zutil_h::ulg,
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
        let mut cc_0: crate::zutil_h::uch = *(*s)
            .window
            .offset((*s).strstart.wrapping_sub(1 as crate::stdlib::uInt) as isize)
            as crate::zutil_h::uch;
        let c2rust_fresh41 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        sym_buf[c2rust_fresh41 as usize] = 0 as crate::zutil_h::uchf;
        let c2rust_fresh42 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        sym_buf[c2rust_fresh42 as usize] = 0 as crate::zutil_h::uchf;
        let c2rust_fresh43 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        sym_buf[c2rust_fresh43 as usize] = cc_0 as crate::zutil_h::uchf;
        (*s).dyn_ltree[cc_0 as usize].fc = (*s).dyn_ltree[cc_0 as usize].fc.wrapping_add(1);
        bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
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
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
    let sym_buf_start = (*s).sym_buf_start;
    let sym_buf_len = ((*s).pending_buf_size as usize).wrapping_sub(sym_buf_start);
    let sym_buf = ::core::slice::from_raw_parts_mut(
        (*s).pending_buf.wrapping_add(sym_buf_start),
        sym_buf_len,
    );
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
        (*s).match_length = 0 as crate::stdlib::uInt;
        // `window_size` is the full allocation capacity set by
        // `deflateInit2_()`/`deflateCopy()`, not merely the current input.
        // `rle_match_length()` uses checked slice accesses for the walk.
        let window = ::core::slice::from_raw_parts((*s).window, (*s).window_size as usize);
        (*s).match_length = rle_match_length(
            window,
            (*s).strstart as usize,
            (*s).lookahead,
        );
        if (*s).match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut len: crate::zutil_h::uch =
                (*s).match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let mut dist: crate::zutil_h::ush = 1 as ::core::ffi::c_int as crate::zutil_h::ush;
            let c2rust_fresh50 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh50 as usize] = dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh51 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh51 as usize] =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh52 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh52 as usize] = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as usize;
            let distance_code = if dist < 256 {
                crate::src::trees::_dist_code[dist as usize] as usize
            } else {
                crate::src::trees::_dist_code[256 + (dist as usize >> 7)] as usize
            };
            let length_entry = length_code + crate::src::deflate::LITERALS as usize + 1;
            (*s).dyn_ltree[length_entry].fc = (*s).dyn_ltree[length_entry].fc.wrapping_add(1);
            (*s).dyn_dtree[distance_code].fc = (*s).dyn_dtree[distance_code].fc.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub((*s).match_length);
            (*s).strstart = (*s).strstart.wrapping_add((*s).match_length);
            (*s).match_length = 0 as crate::stdlib::uInt;
        } else {
            let mut cc: crate::zutil_h::uch = window[(*s).strstart as usize] as crate::zutil_h::uch;
            let c2rust_fresh53 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh53 as usize] = 0 as crate::zutil_h::uchf;
            let c2rust_fresh54 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh54 as usize] = 0 as crate::zutil_h::uchf;
            let c2rust_fresh55 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            sym_buf[c2rust_fresh55 as usize] = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc = (*s).dyn_ltree[cc as usize].fc.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            (*s).strstart = (*s).strstart.wrapping_add(1);
        }
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
                s as *mut crate::src::deflate::internal_state,
                if (*s).block_start >= 0 as ::core::ffi::c_long {
                    (*s).window
                        .offset((*s).block_start as ::core::ffi::c_uint as isize)
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
    let sym_buf_start = (*s).sym_buf_start;
    let sym_buf_len = ((*s).pending_buf_size as usize).wrapping_sub(sym_buf_start);
    let sym_buf = ::core::slice::from_raw_parts_mut(
        (*s).pending_buf.wrapping_add(sym_buf_start),
        sym_buf_len,
    );
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
        (*s).match_length = 0 as crate::stdlib::uInt;
        let mut cc: crate::zutil_h::uch =
            *(*s).window.offset((*s).strstart as isize) as crate::zutil_h::uch;
        let c2rust_fresh56 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        sym_buf[c2rust_fresh56 as usize] = 0 as crate::zutil_h::uchf;
        let c2rust_fresh57 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        sym_buf[c2rust_fresh57 as usize] = 0 as crate::zutil_h::uchf;
        let c2rust_fresh58 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        sym_buf[c2rust_fresh58 as usize] = cc as crate::zutil_h::uchf;
        (*s).dyn_ltree[cc as usize].fc = (*s).dyn_ltree[cc as usize].fc.wrapping_add(1);
        bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
        (*s).lookahead = (*s).lookahead.wrapping_sub(1);
        (*s).strstart = (*s).strstart.wrapping_add(1);
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
                s as *mut crate::src::deflate::internal_state,
                if (*s).block_start >= 0 as ::core::ffi::c_long {
                    (*s).window
                        .offset((*s).block_start as ::core::ffi::c_uint as isize)
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
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
