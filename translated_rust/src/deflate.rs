// =============== BEGIN deflate_h ================

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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    // The Huffman builder uses this one `ush` slot first for parent links and
    // later for code lengths.  The phases do not overlap, so one safe field
    // preserves the C union's storage semantics.
    pub len: crate::zutil_h::ush,
}

pub type StaticTreeKind = ::core::ffi::c_int;
pub const STATIC_LITERAL_LENGTH: StaticTreeKind = 1;
pub const STATIC_DISTANCE: StaticTreeKind = 2;
pub const STATIC_BIT_LENGTH: StaticTreeKind = 3;

pub type tree_desc = crate::src::deflate::tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tree_desc_s {
    pub max_code: ::core::ffi::c_int,
    pub stat_desc: StaticTreeKind,
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

pub use crate::src::adler32::adler32;
pub use crate::src::crc32::crc32;
pub use crate::src::crc32::crc32_z;
pub use crate::src::trees::_dist_code;
pub use crate::src::trees::_length_code;
pub use crate::src::trees::_tr_align;
pub use crate::src::trees::_tr_flush_bits;
pub use crate::src::trees::_tr_init;
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

#[derive(Eq, PartialEq)]
pub enum CompressorKind {
    Stored,
    Fast,
    Slow,
}

pub type config = config_s;
#[repr(C)]

pub struct config_s {
    pub good_length: crate::zutil_h::ush,
    pub max_lazy: crate::zutil_h::ush,
    pub nice_length: crate::zutil_h::ush,
    pub max_chain: crate::zutil_h::ush,
    pub func: CompressorKind,
}
const fn copyright_chars(bytes: [u8; 70]) -> [::core::ffi::c_char; 70] {
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
    copyright_chars(*b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0");

pub const NIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TOO_FAR: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

static configuration_table: [config; 10] = [
    config_s {
        good_length: 0 as crate::zutil_h::ush,
        max_lazy: 0 as crate::zutil_h::ush,
        nice_length: 0 as crate::zutil_h::ush,
        max_chain: 0 as crate::zutil_h::ush,
        func: CompressorKind::Stored,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        func: CompressorKind::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        func: CompressorKind::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: CompressorKind::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        func: CompressorKind::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: CompressorKind::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        func: CompressorKind::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        func: CompressorKind::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        func: CompressorKind::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        func: CompressorKind::Slow,
    },
];

fn slide_hash(
    state: &mut crate::src::deflate::deflate_state,
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
) -> bool {
    let wsize = state.w_size as usize;
    let hash_size = state.hash_size as usize;
    if head.len() < hash_size || prev.len() < wsize {
        return false;
    }
    for entry in head[..hash_size].iter_mut() {
        let value = *entry as usize;
        *entry = if value >= wsize {
            value.wrapping_sub(wsize) as crate::src::deflate::Posf
        } else {
            NIL as crate::src::deflate::Posf
        };
    }
    for entry in prev[..wsize].iter_mut() {
        let value = *entry as usize;
        *entry = if value >= wsize {
            value.wrapping_sub(wsize) as crate::src::deflate::Posf
        } else {
            NIL as crate::src::deflate::Posf
        };
    }
    state.slid = 1 as ::core::ffi::c_int;
    true
}

fn clear_full_flush_hash(
    state: &mut crate::src::deflate::deflate_state,
    head: &mut [crate::src::deflate::Posf],
) -> bool {
    let hash_size = state.hash_size as usize;
    let Some(head) = head.get_mut(..hash_size) else {
        return false;
    };
    head.fill(NIL as crate::src::deflate::Posf);
    state.slid = 0 as ::core::ffi::c_int;
    if state.lookahead == 0 as crate::stdlib::uInt {
        state.strstart = 0 as crate::stdlib::uInt;
        state.block_start = 0 as ::core::ffi::c_long;
        state.insert = 0 as crate::stdlib::uInt;
    }
    true
}

fn read_buf(
    strm: &mut crate::zlib_h::z_stream,
    wrap: ::core::ffi::c_int,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) -> ::core::ffi::c_uint {
    let len = (strm.avail_in as usize).min(input.len()).min(output.len());
    if len == 0 {
        return 0;
    }
    output[..len].copy_from_slice(&input[..len]);
    if wrap == 1 as ::core::ffi::c_int {
        strm.adler = crate::src::adler32::adler32(strm.adler, Some(&output[..len]));
    } else if wrap == 2 as ::core::ffi::c_int {
        strm.adler = crate::src::crc32::crc32(strm.adler, Some(&output[..len]));
    }
    strm.avail_in = strm.avail_in.wrapping_sub(len as crate::stdlib::uInt);
    strm.next_in = strm.next_in.wrapping_add(len);
    strm.total_in = strm.total_in.wrapping_add(len as crate::stdlib::uLong);
    len as ::core::ffi::c_uint
}

fn fill_window(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    input: &[crate::stdlib::Bytef],
) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    let wsize = state.w_size;
    let mut input_offset = 0usize;
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
            let copy = wsize.wrapping_sub(more) as usize;
            let Some(source_end) = (wsize as usize).checked_add(copy) else {
                return;
            };
            if source_end > window.len() {
                return;
            }
            window.copy_within(wsize as usize..source_end, 0);
            state.match_start = state.match_start.wrapping_sub(wsize);
            state.strstart = state.strstart.wrapping_sub(wsize);
            state.block_start -= wsize as ::core::ffi::c_long;
            if state.insert > state.strstart {
                state.insert = state.strstart;
            }
            if !slide_hash(state, head, prev) {
                return;
            }
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if strm.avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        let input_len = strm.avail_in as usize;
        let Some(input_end) = input_offset.checked_add(input_len) else {
            return;
        };
        let Some(input) = input.get(input_offset..input_end) else {
            return;
        };
        let Some(output_start) = (state.strstart as usize).checked_add(state.lookahead as usize)
        else {
            return;
        };
        let Some(output_end) = output_start.checked_add(more as usize) else {
            return;
        };
        let Some(output) = window.get_mut(output_start..output_end) else {
            return;
        };
        n = read_buf(strm, state.wrap, input, output);
        let Some(next_input_offset) = input_offset.checked_add(n as usize) else {
            return;
        };
        input_offset = next_input_offset;
        state.lookahead = state.lookahead.wrapping_add(n);
        if state.lookahead.wrapping_add(state.insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        {
            let mut str = state.strstart.wrapping_sub(state.insert);
            let Some(&first) = window.get(str as usize) else {
                return;
            };
            let Some(&second) = window.get(str.wrapping_add(1) as usize) else {
                return;
            };
            state.ins_h = first as crate::stdlib::uInt;
            state.ins_h = (state.ins_h << state.hash_shift
                ^ second as crate::stdlib::uInt)
                & state.hash_mask;
            while state.insert != 0 {
                let Some(&next) = window.get(
                    str.wrapping_add(3 as crate::stdlib::uInt)
                        .wrapping_sub(1 as crate::stdlib::uInt) as usize,
                ) else {
                    return;
                };
                state.ins_h = (state.ins_h << state.hash_shift
                    ^ next as crate::stdlib::uInt)
                    & state.hash_mask;
                let Some(&hash) = head.get(state.ins_h as usize) else {
                    return;
                };
                let Some(previous) = prev.get_mut((str & state.w_mask) as usize) else {
                    return;
                };
                *previous = hash;
                let Some(hash) = head.get_mut(state.ins_h as usize) else {
                    return;
                };
                *hash = str as crate::src::deflate::Pos as crate::src::deflate::Posf;
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
            && strm.avail_in != 0 as crate::stdlib::uInt)
        {
            break;
        }
    }
    if state.high_water < state.window_size {
        let curr: crate::zutil_h::ulg = (state.strstart as crate::zutil_h::ulg)
            .wrapping_add(state.lookahead as crate::zutil_h::ulg);
        let mut init: crate::zutil_h::ulg = 0;
        if state.high_water < curr {
            init = state.window_size.wrapping_sub(curr);
            if init > crate::src::deflate::WIN_INIT as crate::zutil_h::ulg {
                init = crate::src::deflate::WIN_INIT as crate::zutil_h::ulg;
            }
            let Some(end) = (curr as usize).checked_add(init as usize) else {
                return;
            };
            let Some(range) = window.get_mut(curr as usize..end) else {
                return;
            };
            range.fill(0);
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
            let Some(end) = (state.high_water as usize).checked_add(init as usize) else {
                return;
            };
            let Some(range) = window.get_mut(state.high_water as usize..end) else {
                return;
            };
            range.fill(0);
            state.high_water = state.high_water.wrapping_add(init);
        }
    }
}

macro_rules! fill_window_from_raw {
    ($state:expr) => {{
        let state = &mut *$state;
        let strm = &mut *state.strm;
        let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
        let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
        let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
        let input = if strm.avail_in == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
        };
        fill_window(state, strm, window, head, prev, input);
    }};
}
pub enum DeflateInitMode {
    Zlib,
    Gzip { strategy: ::core::ffi::c_int },
}

pub fn deflateInit_(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut level: ::core::ffi::c_int,
    version: Option<&::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
    mode: DeflateInitMode,
) -> ::core::ffi::c_int {
    let Some(version) = version else {
        return crate::zlib_h::Z_VERSION_ERROR;
    };
    if *version != crate::zlib_h::ZLIB_VERSION[0]
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let (window_bits, mem_level, strategy) = match mode {
        DeflateInitMode::Zlib => (
            crate::stdlib::MAX_WBITS,
            crate::zutil_h::DEF_MEM_LEVEL,
            crate::zlib_h::Z_DEFAULT_STRATEGY,
        ),
        DeflateInitMode::Gzip { strategy } => (15 + 16, 8, strategy),
    };
    // The safe inputs above establish the version, stream, and wrapper-mode
    // invariants that the translated initializer still expects as raw arguments.
    unsafe {
        deflateInit2_(
            strm,
            level,
            crate::zlib_h::Z_DEFLATED,
            window_bits,
            mem_level,
            strategy,
            ::core::ptr::from_ref(version),
            stream_size,
        )
    }
}
#[export_name = "deflateInit_"]

pub unsafe extern "C" fn deflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateInit_(
        strm.as_mut(),
        level,
        version.as_ref(),
        stream_size,
        DeflateInitMode::Zlib,
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
    let my_version = crate::zlib_h::ZLIB_VERSION;
    if version.is_null()
        || *version.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != my_version[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
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
        (*strm).msg = crate::src::zutil::zError(-4 as ::core::ffi::c_int)
            .load(::core::sync::atomic::Ordering::Relaxed);
        deflateEnd(&mut *strm);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*s).sym_buf = (*s).pending_buf.offset((*s).lit_bufsize as isize) as *mut crate::zutil_h::uchf;
    (*s).sym_end = (*s)
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    (*s).level = level;
    (*s).strategy = strategy;
    (*s).method = method as crate::stdlib::Byte;
    let stream = &mut *strm;
    let state = &mut *(stream.state as *mut crate::src::deflate::deflate_state);
    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    return deflate_reset(stream, state, head);
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
fn deflate_state_valid(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::deflate::deflate_state,
) -> bool {
    strm.zalloc.is_some()
        && strm.zfree.is_some()
        && state.strm == ::core::ptr::from_ref(strm).cast_mut()
        && (state.status == crate::src::deflate::INIT_STATE
            || state.status == crate::src::deflate::GZIP_STATE
            || state.status == crate::src::deflate::EXTRA_STATE
            || state.status == crate::src::deflate::NAME_STATE
            || state.status == crate::src::deflate::COMMENT_STATE
            || state.status == crate::src::deflate::HCRC_STATE
            || state.status == crate::src::deflate::BUSY_STATE
            || state.status == crate::src::deflate::FINISH_STATE)
}

fn deflate_stream_state_valid(
    strm: Option<&crate::zlib_h::z_stream>,
    state: Option<&crate::src::deflate::deflate_state>,
) -> bool {
    let (Some(strm), Some(state)) = (strm, state) else {
        return false;
    };
    deflate_state_valid(strm, state)
}
fn deflate_set_dictionary(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    dictionary: &[crate::stdlib::Bytef],
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
) -> ::core::ffi::c_int {
    if !deflate_state_valid(strm, state)
        || window.len() < state.window_size as usize
        || head.len() < state.hash_size as usize
        || prev.len() < state.w_size as usize
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let wrap = state.wrap;
    if wrap == 2
        || wrap == 1 && state.status != crate::src::deflate::INIT_STATE
        || state.lookahead != 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if wrap == 1 {
        strm.adler = crate::src::adler32::adler32(strm.adler, Some(dictionary));
    }
    state.wrap = 0;
    let dictionary = if dictionary.len() >= state.w_size as usize {
        if wrap == 0 {
            head[..state.hash_size as usize].fill(NIL as crate::src::deflate::Posf);
            state.slid = 0;
            state.strstart = 0;
            state.block_start = 0;
            state.insert = 0;
        }
        &dictionary[dictionary.len() - state.w_size as usize..]
    } else {
        dictionary
    };
    let avail = strm.avail_in;
    let next = strm.next_in;
    strm.avail_in = dictionary.len() as crate::stdlib::uInt;
    strm.next_in = dictionary.as_ptr().cast_mut();
    let result = (|| {
        let Some(input) = dictionary.get(
            dictionary
                .len()
                .checked_sub(strm.avail_in as usize)
                .ok_or(crate::zlib_h::Z_STREAM_ERROR)?..,
        ) else {
            return Err(crate::zlib_h::Z_STREAM_ERROR);
        };
        fill_window(state, strm, window, head, prev, input);
        while state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut string = state.strstart;
            let count = state.lookahead.wrapping_sub(
                (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt,
            );
            for _ in 0..count {
                let Some(&byte) = window.get(
                    string
                        .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
                        .wrapping_sub(1) as usize,
                ) else {
                    return Err(crate::zlib_h::Z_STREAM_ERROR);
                };
                state.ins_h = (state.ins_h << state.hash_shift ^ byte as crate::stdlib::uInt)
                    & state.hash_mask;
                let Some(&previous) = head.get(state.ins_h as usize) else {
                    return Err(crate::zlib_h::Z_STREAM_ERROR);
                };
                let Some(previous_slot) = prev.get_mut((string & state.w_mask) as usize) else {
                    return Err(crate::zlib_h::Z_STREAM_ERROR);
                };
                *previous_slot = previous;
                let Some(head_slot) = head.get_mut(state.ins_h as usize) else {
                    return Err(crate::zlib_h::Z_STREAM_ERROR);
                };
                *head_slot = string as crate::src::deflate::Posf;
                string = string.wrapping_add(1);
            }
            state.strstart = string;
            state.lookahead = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
            let Some(input) = dictionary.get(
                dictionary
                    .len()
                    .checked_sub(strm.avail_in as usize)
                    .ok_or(crate::zlib_h::Z_STREAM_ERROR)?..,
            ) else {
                return Err(crate::zlib_h::Z_STREAM_ERROR);
            };
            fill_window(state, strm, window, head, prev, input);
        }
        state.strstart = state.strstart.wrapping_add(state.lookahead);
        state.block_start = state.strstart as ::core::ffi::c_long;
        state.insert = state.lookahead;
        state.lookahead = 0;
        state.prev_length = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
        state.match_length = state.prev_length;
        state.match_available = 0;
        Ok(())
    })();
    strm.next_in = next;
    strm.avail_in = avail;
    state.wrap = wrap;
    match result {
        Ok(()) => crate::zlib_h::Z_OK,
        Err(error) => error,
    }
}
#[export_name = "deflateSetDictionary"]

pub unsafe extern "C" fn deflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Check callbacks before borrowing state. A stale non-null state must not
    // be dereferenced when the stream itself has no valid allocator pair.
    if strm.zalloc.is_none() || strm.zfree.is_none() || dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::deflate::deflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(strm), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.window.is_null() || state.head.is_null() || state.prev.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = ::core::slice::from_raw_parts(dictionary, dictLength as usize);
    let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
    deflate_set_dictionary(strm, state, dictionary, window, head, prev)
}
fn deflate_dictionary_len(
    strm: Option<&crate::zlib_h::z_stream_s>,
    state: Option<&crate::src::deflate::deflate_state>,
) -> Result<usize, ::core::ffi::c_int> {
    let (Some(strm), Some(state)) = (strm, state) else {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    };
    if strm.zalloc.is_none()
        || strm.zfree.is_none()
        || state.strm != strm as *const crate::zlib_h::z_stream_s as crate::zlib_h::z_streamp
        || state.status != crate::src::deflate::INIT_STATE
            && state.status != crate::src::deflate::GZIP_STATE
            && state.status != crate::src::deflate::EXTRA_STATE
            && state.status != crate::src::deflate::NAME_STATE
            && state.status != crate::src::deflate::COMMENT_STATE
            && state.status != crate::src::deflate::HCRC_STATE
            && state.status != crate::src::deflate::BUSY_STATE
            && state.status != crate::src::deflate::FINISH_STATE
    {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    let mut len = state.strstart.wrapping_add(state.lookahead);
    if len > state.w_size {
        len = state.w_size;
    }
    Ok(len as usize)
}

fn deflate_get_dictionary(
    strm: Option<&crate::zlib_h::z_stream_s>,
    state: Option<&crate::src::deflate::deflate_state>,
    window: Option<&[crate::stdlib::Bytef]>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    let len = match deflate_dictionary_len(strm, state) {
        Ok(len) => len,
        Err(error) => return error,
    };
    if len != 0 {
        let (Some(state), Some(window)) = (state, window) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let end = state.strstart.wrapping_add(state.lookahead) as usize;
        let Some(start) = end.checked_sub(len) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Some(source) = window.get(start..end) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if let Some(dictionary) = dictionary {
            let Some(destination) = dictionary.get_mut(..len) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            destination.copy_from_slice(source);
        }
    }
    if let Some(dict_length) = dict_length {
        *dict_length = len as crate::stdlib::uInt;
    }
    crate::zlib_h::Z_OK
}
#[export_name = "deflateGetDictionary"]

pub unsafe extern "C" fn deflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let strm = strm.as_ref();
    let state = strm.and_then(|strm| {
        (strm.state as *const crate::src::deflate::deflate_state).as_ref()
    });
    let len = match deflate_dictionary_len(strm, state) {
        Ok(len) => len,
        Err(error) => return error,
    };
    let window = match state {
        Some(state) if state.window.is_null() && state.window_size != 0 => None,
        Some(state) if state.window_size == 0 => Some(&[][..]),
        Some(state) => Some(::core::slice::from_raw_parts(
            state.window,
            state.window_size as usize,
        )),
        None => None,
    };
    let dictionary = if dictionary.is_null() || len == 0 {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(dictionary, len))
    };
    deflate_get_dictionary(strm, state, window, dictionary, dictLength.as_mut())
}
fn deflate_reset_keep_stream_valid(strm: Option<&crate::zlib_h::z_stream>) -> bool {
    let Some(strm) = strm else {
        return false;
    };
    strm.zalloc.is_some() && strm.zfree.is_some() && !strm.state.is_null()
}

fn deflate_reset_keep_state_valid(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::deflate::deflate_state,
) -> bool {
    state.strm == ::core::ptr::from_ref(strm).cast_mut()
        && (state.status == crate::src::deflate::INIT_STATE
            || state.status == crate::src::deflate::GZIP_STATE
            || state.status == crate::src::deflate::EXTRA_STATE
            || state.status == crate::src::deflate::NAME_STATE
            || state.status == crate::src::deflate::COMMENT_STATE
            || state.status == crate::src::deflate::HCRC_STATE
            || state.status == crate::src::deflate::BUSY_STATE
            || state.status == crate::src::deflate::FINISH_STATE)
}

fn deflate_reset_keep(
    strm: &mut crate::zlib_h::z_stream,
    state: Option<&mut crate::src::deflate::deflate_state>,
) -> ::core::ffi::c_int {
    if !deflate_reset_keep_stream_valid(Some(strm)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_reset_keep_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*strm).total_out = 0 as crate::stdlib::uLong;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*strm).data_type = crate::zlib_h::Z_UNKNOWN;
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
    (*strm).adler = if state.wrap == 2 as ::core::ffi::c_int {
        crate::src::crc32::crc32(0 as crate::stdlib::uLong, None)
    } else {
        crate::src::adler32::adler32(0 as crate::stdlib::uLong, None)
    };
    state.last_flush = -2 as ::core::ffi::c_int;
    crate::src::trees::_tr_init(state);
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateResetKeep"]
pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // This check tests callbacks before it reads `state`.  Keep it ahead of
    // either raw-pointer-to-reference conversion so malformed streams with a
    // stale state and no allocators are still rejected without dereferencing it.
    let Some(stream) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if stream.zalloc.is_none() || stream.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (stream.state as *mut crate::src::deflate::deflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(stream), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflate_reset_keep(stream, Some(state))
}
fn lm_init(
    state: &mut crate::src::deflate::deflate_state,
    head: &mut [crate::src::deflate::Pos],
) -> bool {
    let hash_size = state.hash_size as usize;
    let Some(last) = hash_size.checked_sub(1) else {
        return false;
    };
    let Some(head) = head.get_mut(..hash_size) else {
        return false;
    };
    let Some(configuration) = configuration_table.get(state.level as usize) else {
        return false;
    };
    state.window_size = (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
        .wrapping_mul(state.w_size as crate::zutil_h::ulg);
    head[..last].fill(0);
    head[last] = NIL as crate::src::deflate::Posf;
    state.slid = 0 as ::core::ffi::c_int;
    state.max_lazy_match = configuration.max_lazy as crate::stdlib::uInt;
    state.good_match = configuration.good_length as crate::stdlib::uInt;
    state.nice_match = configuration.nice_length as ::core::ffi::c_int;
    state.max_chain_length = configuration.max_chain as crate::stdlib::uInt;
    state.strstart = 0 as crate::stdlib::uInt;
    state.block_start = 0 as ::core::ffi::c_long;
    state.lookahead = 0 as crate::stdlib::uInt;
    state.insert = 0 as crate::stdlib::uInt;
    state.prev_length = (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
    state.match_length = state.prev_length;
    state.match_available = 0 as ::core::ffi::c_int;
    state.ins_h = 0 as crate::stdlib::uInt;
    true
}

pub(crate) fn deflate_reset(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    head: &mut [crate::src::deflate::Pos],
) -> ::core::ffi::c_int {
    let ret = deflate_reset_keep(stream, Some(state));
    if ret != crate::zlib_h::Z_OK || !lm_init(state, head) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    ret
}
#[export_name = "deflateReset"]

pub unsafe extern "C" fn deflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // This check tests callbacks before it reads `state`.  Keep it ahead of
    // either raw-pointer-to-reference conversion so malformed streams with a
    // stale state and no allocators are still rejected without dereferencing it.
    let Some(stream) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if stream.zalloc.is_none() || stream.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (stream.state as *mut crate::src::deflate::deflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(stream), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.head.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    deflate_reset(stream, state, head)
}
fn deflate_set_header(
    state: &mut crate::src::deflate::deflate_state,
    head: &mut crate::zlib_h::gz_header_s,
) -> ::core::ffi::c_int {
    if state.wrap != 2 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.gzhead = head;
    crate::zlib_h::Z_OK
}

fn deflate_clear_header(
    state: &mut crate::src::deflate::deflate_state,
) -> ::core::ffi::c_int {
    if state.wrap != 2 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.gzhead = ::core::ptr::null_mut();
    crate::zlib_h::Z_OK
}
#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some(stream) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if stream.zalloc.is_none() || stream.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (stream.state as *mut crate::src::deflate::deflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(stream), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if head.is_null() {
        deflate_clear_header(state)
    } else {
        deflate_set_header(state, &mut *head)
    }
}
fn deflate_pending(
    state: &crate::src::deflate::deflate_state,
    pending: Option<&mut ::core::ffi::c_uint>,
    bits: Option<&mut ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    if let Some(bits) = bits {
        *bits = state.bi_valid;
    }
    if let Some(pending) = pending {
        *pending = state.pending as ::core::ffi::c_uint;
        if *pending as crate::zutil_h::ulg != state.pending {
            *pending = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflatePending"]

pub unsafe extern "C" fn deflatePending_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_ref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *const crate::src::deflate::deflate_state).as_ref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(strm), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflate_pending(state, pending.as_mut(), bits.as_mut())
}
fn deflate_used(
    state: &crate::src::deflate::deflate_state,
    bits: Option<&mut ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    if let Some(bits) = bits {
        *bits = state.bi_used;
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateUsed"]

pub unsafe extern "C" fn deflateUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_ref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *const crate::src::deflate::deflate_state).as_ref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if state.strm != strm as *const crate::zlib_h::z_stream_s as crate::zlib_h::z_streamp
        || state.status != crate::src::deflate::INIT_STATE
            && state.status != crate::src::deflate::GZIP_STATE
            && state.status != crate::src::deflate::EXTRA_STATE
            && state.status != crate::src::deflate::NAME_STATE
            && state.status != crate::src::deflate::COMMENT_STATE
            && state.status != crate::src::deflate::HCRC_STATE
            && state.status != crate::src::deflate::BUSY_STATE
            && state.status != crate::src::deflate::FINISH_STATE
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflate_used(state, bits.as_mut())
}
fn deflate_prime(
    strm: Option<&crate::zlib_h::z_stream_s>,
    state: Option<&mut crate::src::deflate::deflate_state>,
    pending_buf: Option<&mut [crate::stdlib::Bytef]>,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut put: ::core::ffi::c_int = 0;
    let (Some(strm), Some(state), Some(pending_buf)) = (strm, state, pending_buf) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if strm.zalloc.is_none()
        || strm.zfree.is_none()
        || state.strm != strm as *const crate::zlib_h::z_stream_s as crate::zlib_h::z_streamp
        || state.status != crate::src::deflate::INIT_STATE
            && state.status != crate::src::deflate::GZIP_STATE
            && state.status != crate::src::deflate::EXTRA_STATE
            && state.status != crate::src::deflate::NAME_STATE
            && state.status != crate::src::deflate::COMMENT_STATE
            && state.status != crate::src::deflate::HCRC_STATE
            && state.status != crate::src::deflate::BUSY_STATE
            && state.status != crate::src::deflate::FINISH_STATE
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(pending_out) = state
        .pending_out
        .addr()
        .checked_sub(pending_buf.as_ptr().addr())
    else {
        return crate::zlib_h::Z_BUF_ERROR;
    };
    let Some(sym_buf) = state
        .sym_buf
        .addr()
        .checked_sub(pending_buf.as_ptr().addr())
    else {
        return crate::zlib_h::Z_BUF_ERROR;
    };
    if pending_out > pending_buf.len() || sym_buf > pending_buf.len() {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    let Some(min_sym_buf) = pending_out.checked_add(
        ((crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int)
            >> 3 as ::core::ffi::c_int) as usize,
    ) else {
        return crate::zlib_h::Z_BUF_ERROR;
    };
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || sym_buf < min_sym_buf
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    loop {
        put = crate::src::deflate::Buf_size - state.bi_valid;
        if put > bits {
            put = bits;
        }
        state.bi_buf = (state.bi_buf as ::core::ffi::c_int
            | ((value & ((1 as ::core::ffi::c_int) << put) - 1 as ::core::ffi::c_int)
                << state.bi_valid) as crate::zutil_h::ush as ::core::ffi::c_int)
            as crate::zutil_h::ush;
        state.bi_valid += put;
        crate::src::trees::_tr_flush_bits(state, pending_buf);
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
    let Some(strm) = strm.as_ref() else {
        return deflate_prime(None, None, None, bits, value);
    };
    let Some(state) = (strm.state as *mut crate::src::deflate::deflate_state).as_mut() else {
        return deflate_prime(Some(strm), None, None, bits, value);
    };
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state.pending_buf,
        state.pending_buf_size as usize,
    );
    deflate_prime(Some(strm), Some(state), Some(pending_buf), bits, value)
}
pub(crate) fn deflateParams(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    hash_tables: Option<(
        &mut [crate::src::deflate::Posf],
        &mut [crate::src::deflate::Posf],
    )>,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
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
    let Some(current_kind) = configuration_table
        .get(state.level as usize)
        .map(|configuration| &configuration.func)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(next_kind) = configuration_table
        .get(level as usize)
        .map(|configuration| &configuration.func)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if (strategy != state.strategy || current_kind != next_kind)
        && state.last_flush != -2 as ::core::ffi::c_int
    {
        let err = unsafe {
            deflate(
                strm,
                crate::zlib_h::Z_BLOCK,
            )
        };
        if err == crate::zlib_h::Z_STREAM_ERROR {
            return err;
        }
        if strm.avail_in != 0
            || state.strstart as ::core::ffi::c_long - state.block_start
                + state.lookahead as ::core::ffi::c_long
                != 0
        {
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    if state.level != level {
        if state.level == 0 as ::core::ffi::c_int && state.matches != 0 as crate::stdlib::uInt {
            let Some((head, prev)) = hash_tables else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            if state.matches == 1 as crate::stdlib::uInt {
                if !slide_hash(state, head, prev) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
            } else {
                let Some((last, rest)) = head.split_last_mut() else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                rest.fill(0);
                *last = NIL as crate::src::deflate::Posf;
                state.slid = 0 as ::core::ffi::c_int;
            }
            state.matches = 0 as crate::stdlib::uInt;
        }
        state.level = level;
        state.max_lazy_match = configuration_table[level as usize].max_lazy as crate::stdlib::uInt;
        state.good_match = configuration_table[level as usize].good_length as crate::stdlib::uInt;
        state.nice_match = configuration_table[level as usize].nice_length as ::core::ffi::c_int;
        state.max_chain_length =
            configuration_table[level as usize].max_chain as crate::stdlib::uInt;
    }
    state.strategy = strategy;
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateParams"]

pub unsafe extern "C" fn deflateParams_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // A missing allocator pair makes `state` untrustworthy; reject it before
    // borrowing the state handle.
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::deflate::deflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let hash_tables = if state.head.is_null() || state.prev.is_null() {
        None
    } else {
        Some((
            ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize),
            ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize),
        ))
    };
    deflateParams(strm, state, hash_tables, level, strategy)
}
fn deflate_tune(
    state: &mut crate::src::deflate::deflate_state,
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    state.good_match = good_length as crate::stdlib::uInt;
    state.max_lazy_match = max_lazy as crate::stdlib::uInt;
    state.nice_match = nice_length;
    state.max_chain_length = max_chain as crate::stdlib::uInt;
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
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::deflate::deflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(strm), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflate_tune(state, good_length, max_lazy, nice_length, max_chain)
}
struct DeflateBoundGzipHeader<'a> {
    has_extra: bool,
    extra_len: crate::stdlib::uInt,
    name: Option<&'a ::std::ffi::CStr>,
    comment: Option<&'a ::std::ffi::CStr>,
    hcrc: ::core::ffi::c_int,
}

fn deflate_bound_z(
    state: Option<&crate::src::deflate::deflate_state>,
    gzip_header: Option<DeflateBoundGzipHeader<'_>>,
    source_len: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut fixedlen: crate::stdlib::z_size_t = 0;
    let mut storelen: crate::stdlib::z_size_t = 0;
    let mut wraplen: crate::stdlib::z_size_t = 0;
    let mut bound: crate::stdlib::z_size_t = 0;
    fixedlen = source_len
        .wrapping_add(source_len >> 3 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 8 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 9 as ::core::ffi::c_int)
        .wrapping_add(4 as crate::stdlib::z_size_t);
    if fixedlen < source_len {
        fixedlen = -1 as ::core::ffi::c_int as crate::stdlib::z_size_t;
    }
    storelen = source_len
        .wrapping_add(source_len >> 5 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 7 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 11 as ::core::ffi::c_int)
        .wrapping_add(7 as crate::stdlib::z_size_t);
    if storelen < source_len {
        storelen = -1 as ::core::ffi::c_int as crate::stdlib::z_size_t;
    }
    let Some(state) = state else {
        bound = if fixedlen > storelen {
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
            if let Some(header) = gzip_header {
                if header.has_extra {
                    wraplen = wraplen.wrapping_add(
                        (2 as crate::stdlib::uInt).wrapping_add(header.extra_len)
                            as crate::stdlib::z_size_t,
                    );
                }
                if let Some(name) = header.name {
                    wraplen = wraplen.wrapping_add(name.to_bytes_with_nul().len());
                }
                if let Some(comment) = header.comment {
                    wraplen = wraplen.wrapping_add(comment.to_bytes_with_nul().len());
                }
                if header.hcrc != 0 {
                    wraplen = wraplen.wrapping_add(2 as crate::stdlib::z_size_t);
                }
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
        bound = if state.w_bits <= state.hash_bits && state.level != 0 {
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
    bound = source_len
        .wrapping_add(source_len >> 12 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 14 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as crate::stdlib::z_size_t)
        .wrapping_sub(6 as crate::stdlib::z_size_t)
        .wrapping_add(wraplen);
    return if bound < source_len {
        -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
    } else {
        bound
    };
}
#[export_name = "deflateBound_z"]

pub unsafe extern "C" fn deflateBound_z_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let state = strm.as_ref().and_then(|strm| {
        if strm.zalloc.is_none() || strm.zfree.is_none() {
            return None;
        }
        let state = (strm.state as *const crate::src::deflate::deflate_state).as_ref()?;
        deflate_stream_state_valid(Some(strm), Some(state)).then_some(state)
    });
    let gzip_header = state.and_then(|state| {
        if state.gzhead.is_null() || (state.wrap != 2 && state.wrap != -2) {
            None
        } else {
            let header = &*state.gzhead;
            Some(DeflateBoundGzipHeader {
                has_extra: !header.extra.is_null(),
                extra_len: header.extra_len,
                name: if header.name.is_null() {
                    None
                } else {
                    Some(::std::ffi::CStr::from_ptr(header.name.cast()))
                },
                comment: if header.comment.is_null() {
                    None
                } else {
                    Some(::std::ffi::CStr::from_ptr(header.comment.cast()))
                },
                hcrc: header.hcrc,
            })
        }
    });
    deflate_bound_z(state, gzip_header, sourceLen)
}
fn deflate_bound_result(bound: crate::stdlib::z_size_t) -> crate::stdlib::uLong {
    if bound != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        bound as crate::stdlib::uLong
    }
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let state = strm.as_ref().and_then(|strm| {
        if strm.zalloc.is_none() || strm.zfree.is_none() {
            return None;
        }
        let state = (strm.state as *const crate::src::deflate::deflate_state).as_ref()?;
        deflate_stream_state_valid(Some(strm), Some(state)).then_some(state)
    });
    let gzip_header = state.and_then(|state| {
        if state.gzhead.is_null() || (state.wrap != 2 && state.wrap != -2) {
            None
        } else {
            let header = &*state.gzhead;
            Some(DeflateBoundGzipHeader {
                has_extra: !header.extra.is_null(),
                extra_len: header.extra_len,
                name: if header.name.is_null() {
                    None
                } else {
                    Some(::std::ffi::CStr::from_ptr(header.name.cast()))
                },
                comment: if header.comment.is_null() {
                    None
                } else {
                    Some(::std::ffi::CStr::from_ptr(header.comment.cast()))
                },
                hcrc: header.hcrc,
            })
        }
    });
    deflate_bound_result(deflate_bound_z(
        state,
        gzip_header,
        sourceLen as crate::stdlib::z_size_t,
    ))
}
fn put_short_msb(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    value: crate::stdlib::uInt,
) -> bool {
    let Ok(start) = usize::try_from(state.pending) else {
        return false;
    };
    let Some(end) = start.checked_add(2) else {
        return false;
    };
    let Some(bytes) = pending_buf.get_mut(start..end) else {
        return false;
    };
    bytes[0] = (value >> 8) as crate::stdlib::Byte;
    bytes[1] = (value & 0xff) as crate::stdlib::Byte;
    state.pending = state.pending.wrapping_add(2);
    true
}

fn write_default_gzip_header_tail(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
) -> bool {
    let Some(start) = usize::try_from(state.pending).ok() else {
        return false;
    };
    let xfl = if state.level == 9 as ::core::ffi::c_int {
        2
    } else if state.strategy >= 2 as ::core::ffi::c_int
        || state.level < 2 as ::core::ffi::c_int
    {
        4
    } else {
        0
    };
    let header = [0, 0, 0, 0, 0, xfl, 3];
    let Some(end) = start.checked_add(header.len()) else {
        return false;
    };
    let Some(destination) = pending_buf.get_mut(start..end) else {
        return false;
    };
    destination.copy_from_slice(&header);
    state.pending = state.pending.wrapping_add(header.len() as crate::zutil_h::ulg);
    state.status = crate::src::deflate::BUSY_STATE;
    true
}

fn write_gzip_header_magic(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
) -> bool {
    let Ok(start) = usize::try_from(state.pending) else {
        return false;
    };
    let Some(end) = start.checked_add(3) else {
        return false;
    };
    let Some(destination) = pending_buf.get_mut(start..end) else {
        return false;
    };
    destination.copy_from_slice(&[31, 139, 8]);
    state.pending = state.pending.wrapping_add(3);
    true
}

fn write_gzip_header_fixed(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    header: &crate::zlib_h::gz_header_s,
) -> bool {
    let Ok(start) = usize::try_from(state.pending) else {
        return false;
    };
    let flags = (if header.text != 0 { 1 } else { 0 })
        + (if header.hcrc != 0 { 2 } else { 0 })
        + (if header.extra.is_null() { 0 } else { 4 })
        + (if header.name.is_null() { 0 } else { 8 })
        + (if header.comment.is_null() { 0 } else { 16 });
    let xfl = if state.level == 9 {
        2
    } else if state.strategy >= 2 || state.level < 2 {
        4
    } else {
        0
    };
    let mut fixed = [
        flags as crate::stdlib::Bytef,
        header.time as crate::stdlib::Bytef,
        (header.time >> 8) as crate::stdlib::Bytef,
        (header.time >> 16) as crate::stdlib::Bytef,
        (header.time >> 24) as crate::stdlib::Bytef,
        xfl,
        header.os as crate::stdlib::Bytef,
        0,
        0,
    ];
    let length = if header.extra.is_null() { 7 } else { 9 };
    if !header.extra.is_null() {
        fixed[7] = header.extra_len as crate::stdlib::Bytef;
        fixed[8] = (header.extra_len >> 8) as crate::stdlib::Bytef;
    }
    let Some(end) = start.checked_add(length) else {
        return false;
    };
    let Some(destination) = pending_buf.get_mut(start..end) else {
        return false;
    };
    destination.copy_from_slice(&fixed[..length]);
    state.pending = state.pending.wrapping_add(length as crate::zutil_h::ulg);
    true
}

fn write_gzip_header_crc(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
) -> bool {
    let Ok(start) = usize::try_from(state.pending) else {
        return false;
    };
    let Some(end) = start.checked_add(2) else {
        return false;
    };
    let Some(destination) = pending_buf.get_mut(start..end) else {
        return false;
    };
    destination.copy_from_slice(&[
        checksum as crate::stdlib::Bytef,
        (checksum >> 8) as crate::stdlib::Bytef,
    ]);
    state.pending = state.pending.wrapping_add(2);
    true
}

fn append_gzip_header_extra(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    extra: &[crate::stdlib::Bytef],
    count: crate::zutil_h::ulg,
) -> bool {
    let (Ok(destination_start), Ok(source_start), Ok(count)) = (
        usize::try_from(state.pending),
        usize::try_from(state.gzindex),
        usize::try_from(count),
    ) else {
        return false;
    };
    let (Some(destination_end), Some(source_end)) = (
        destination_start.checked_add(count),
        source_start.checked_add(count),
    ) else {
        return false;
    };
    let Some(source) = extra.get(source_start..source_end) else {
        return false;
    };
    let Some(destination) = pending_buf.get_mut(destination_start..destination_end) else {
        return false;
    };
    destination.copy_from_slice(source);
    state.pending = state.pending.wrapping_add(count as crate::zutil_h::ulg);
    state.gzindex = state.gzindex.wrapping_add(count as crate::zutil_h::ulg);
    true
}

fn append_gzip_header_string_byte(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    text: &[crate::stdlib::Bytef],
) -> Option<bool> {
    let (Ok(source), Ok(destination)) = (
        usize::try_from(state.gzindex),
        usize::try_from(state.pending),
    ) else {
        return None;
    };
    let byte = *text.get(source)?;
    *pending_buf.get_mut(destination)? = byte;
    state.gzindex = state.gzindex.checked_add(1)?;
    state.pending = state.pending.checked_add(1)?;
    Some(byte == 0)
}

fn checksum_gzip_header_range(
    checksum: &mut crate::stdlib::uLong,
    pending_buf: &[crate::stdlib::Bytef],
    start: crate::zutil_h::ulg,
    end: crate::zutil_h::ulg,
) -> bool {
    let (Ok(start), Ok(end)) = (usize::try_from(start), usize::try_from(end)) else {
        return false;
    };
    let Some(bytes) = pending_buf.get(start..end) else {
        return false;
    };
    *checksum = crate::src::crc32::crc32_z(*checksum, Some(bytes));
    true
}

fn write_gzip_trailer(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
    total_in: crate::stdlib::uLong,
) -> bool {
    let Ok(start) = usize::try_from(state.pending) else {
        return false;
    };
    let trailer = [
        checksum as crate::stdlib::Bytef,
        (checksum >> 8) as crate::stdlib::Bytef,
        (checksum >> 16) as crate::stdlib::Bytef,
        (checksum >> 24) as crate::stdlib::Bytef,
        total_in as crate::stdlib::Bytef,
        (total_in >> 8) as crate::stdlib::Bytef,
        (total_in >> 16) as crate::stdlib::Bytef,
        (total_in >> 24) as crate::stdlib::Bytef,
    ];
    let Some(end) = start.checked_add(trailer.len()) else {
        return false;
    };
    let Some(destination) = pending_buf.get_mut(start..end) else {
        return false;
    };
    destination.copy_from_slice(&trailer);
    state.pending = state.pending.wrapping_add(trailer.len() as crate::zutil_h::ulg);
    true
}

fn flush_pending_impl(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    pending_start: usize,
    output: &mut [crate::stdlib::Bytef],
) -> bool {
    let pending_before = match usize::try_from(state.pending) {
        Ok(pending) => pending,
        Err(_) => return false,
    };
    let flush_bytes = if state.bi_valid == 16 {
        2
    } else if state.bi_valid >= 8 {
        1
    } else {
        0
    };
    let Some(flush_end) = pending_before.checked_add(flush_bytes) else {
        return false;
    };
    if flush_end > pending_buf.len() || pending_start > pending_buf.len() {
        return false;
    }
    crate::src::trees::_tr_flush_bits(state, pending_buf);

    let pending = match usize::try_from(state.pending) {
        Ok(pending) => pending,
        Err(_) => return false,
    };
    let available = match usize::try_from(strm.avail_out) {
        Ok(available) => available,
        Err(_) => return false,
    };
    if output.len() != available || pending > pending_buf.len().saturating_sub(pending_start) {
        return false;
    }
    let len = pending.min(available);
    if len == 0 {
        return true;
    }
    let Some(source_end) = pending_start.checked_add(len) else {
        return false;
    };
    output[..len].copy_from_slice(&pending_buf[pending_start..source_end]);
    strm.next_out = output.as_mut_ptr().wrapping_add(len);
    state.pending_out = pending_buf.as_mut_ptr().wrapping_add(source_end);
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out -= len as crate::stdlib::uInt;
    state.pending -= len as crate::zutil_h::ulg;
    if state.pending == 0 {
        state.pending_out = pending_buf.as_mut_ptr();
    }
    true
}

unsafe extern "C" fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::deflate::deflate_state);
    if state.pending_buf.is_null()
        || state.pending_out.is_null()
        || strm.avail_out != 0 && strm.next_out.is_null()
    {
        return;
    }
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state.pending_buf,
        state.pending_buf_size as usize,
    );
    let Some(pending_start) = state.pending_out.addr().checked_sub(pending_buf.as_ptr().addr())
    else {
        return;
    };
    let output = if strm.avail_out == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(strm.next_out, strm.avail_out as usize)
    };
    let _ = flush_pending_impl(strm, state, pending_buf, pending_start, output);
}
pub unsafe fn deflate(
    strm: &mut crate::zlib_h::z_stream,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // The exported wrapper and internal callers provide a live stream
    // reference. Keep the translated raw-state implementation below local
    // until stream ownership is converted.
    let mut old_flush: ::core::ffi::c_int = 0;
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if strm.zalloc.is_none()
        || strm.zfree.is_none()
        || strm.state.is_null()
        || flush > crate::zlib_h::Z_BLOCK
        || flush < 0 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = strm.state as *mut crate::src::deflate::deflate_state;
    if !deflate_stream_state_valid(Some(strm), Some(&*s)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = strm as *mut crate::zlib_h::z_stream;
    if (*strm).next_out.is_null()
        || (*strm).avail_in != 0 as crate::stdlib::uInt && (*strm).next_in.is_null()
        || (*s).status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH
    {
        (*strm).msg = crate::src::zutil::zError(-2 as ::core::ffi::c_int)
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -2 as ::core::ffi::c_int;
    }
    if (*strm).avail_out == 0 as crate::stdlib::uInt {
        (*strm).msg = crate::src::zutil::zError(-5 as ::core::ffi::c_int)
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
        (*strm).msg = crate::src::zutil::zError(-5 as ::core::ffi::c_int)
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -5 as ::core::ffi::c_int;
    }
    if (*s).status == crate::src::deflate::FINISH_STATE
        && (*strm).avail_in != 0 as crate::stdlib::uInt
    {
        (*strm).msg = crate::src::zutil::zError(-5 as ::core::ffi::c_int)
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
        let dictionary = (*s).strstart != 0 as crate::stdlib::uInt;
        let state = &mut *s;
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !put_short_msb(state, pending_buf, header) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if dictionary {
            if !put_short_msb(
                state,
                pending_buf,
                ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
            ) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            if !put_short_msb(
                state,
                pending_buf,
                ((*strm).adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
            ) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
        }
        (*strm).adler = crate::src::adler32::adler32(0 as crate::stdlib::uLong, None);
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    // A custom header is retained by the stream for the duration of a deflate
    // call. Borrow it once at this legacy boundary so the header phases below
    // do not repeatedly dereference the raw state/header pointers.
    let gzip_header = if (*s).gzhead.is_null() {
        None
    } else {
        Some(&*(*s).gzhead)
    };
    if (*s).status == crate::src::deflate::GZIP_STATE {
        (*strm).adler = crate::src::crc32::crc32(0 as crate::stdlib::uLong, None);
        let state = &mut *s;
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !write_gzip_header_magic(state, pending_buf) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if gzip_header.is_none() {
            if !write_default_gzip_header_tail(state, pending_buf) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            flush_pending(strm);
            if (*s).pending != 0 as crate::zutil_h::ulg {
                (*s).last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        } else if let Some(header) = gzip_header {
            if !write_gzip_header_fixed(state, pending_buf, header) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            if header.hcrc != 0 {
                let Ok(pending) = usize::try_from(state.pending) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                let Some(header_bytes) = pending_buf.get(..pending) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                (*strm).adler = crate::src::crc32::crc32_z(
                    (*strm).adler,
                    Some(header_bytes),
                );
            }
            state.gzindex = 0 as crate::zutil_h::ulg;
            state.status = crate::src::deflate::EXTRA_STATE;
        }
    }
    if (*s).status == crate::src::deflate::EXTRA_STATE {
        let Some(header) = gzip_header else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if !header.extra.is_null() {
            let extra_len = (header.extra_len & 0xffff as crate::stdlib::uInt) as usize;
            let extra = ::core::slice::from_raw_parts(header.extra, extra_len);
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            let hcrc = header.hcrc != 0;
            let mut beg: crate::zutil_h::ulg = state.pending;
            let mut left: crate::zutil_h::ulg =
                (extra_len as crate::zutil_h::ulg).wrapping_sub(state.gzindex);
            while state.pending.wrapping_add(left) > state.pending_buf_size {
                let mut copy: crate::zutil_h::ulg =
                    state.pending_buf_size.wrapping_sub(state.pending);
                if !append_gzip_header_extra(state, pending_buf, extra, copy) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                if hcrc && state.pending > beg {
                    if !checksum_gzip_header_range(
                        &mut (*strm).adler,
                        pending_buf,
                        beg,
                        state.pending,
                    ) {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    }
                }
                flush_pending(strm);
                if state.pending != 0 as crate::zutil_h::ulg {
                    state.last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
                beg = 0 as crate::zutil_h::ulg;
                left = left.wrapping_sub(copy);
            }
            if !append_gzip_header_extra(state, pending_buf, extra, left) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            if hcrc && state.pending > beg {
                if !checksum_gzip_header_range(
                    &mut (*strm).adler,
                    pending_buf,
                    beg,
                    state.pending,
                ) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
            }
            state.gzindex = 0 as crate::zutil_h::ulg;
        }
        (*s).status = crate::src::deflate::NAME_STATE;
    }
    if (*s).status == crate::src::deflate::NAME_STATE {
        let Some(header) = gzip_header else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if !header.name.is_null() {
            let name = ::std::ffi::CStr::from_ptr(
                header.name as *const ::core::ffi::c_char,
            )
            .to_bytes_with_nul();
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            let hcrc = header.hcrc != 0;
            let mut beg_0: crate::zutil_h::ulg = (*s).pending;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if hcrc && (*s).pending > beg_0 {
                        if !checksum_gzip_header_range(
                            &mut (*strm).adler,
                            pending_buf,
                            beg_0,
                            (*s).pending,
                        ) {
                            return crate::zlib_h::Z_STREAM_ERROR;
                        }
                    }
                    flush_pending(strm);
                    if (*s).pending != 0 as crate::zutil_h::ulg {
                        (*s).last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                    beg_0 = 0 as crate::zutil_h::ulg;
                }
                let Some(done) = append_gzip_header_string_byte(state, pending_buf, name) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if done {
                    break;
                }
            }
            if hcrc && (*s).pending > beg_0 {
                if !checksum_gzip_header_range(
                    &mut (*strm).adler,
                    pending_buf,
                    beg_0,
                    (*s).pending,
                ) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
            }
            (*s).gzindex = 0 as crate::zutil_h::ulg;
        }
        (*s).status = crate::src::deflate::COMMENT_STATE;
    }
    if (*s).status == crate::src::deflate::COMMENT_STATE {
        let Some(header) = gzip_header else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if !header.comment.is_null() {
            let comment = ::std::ffi::CStr::from_ptr(
                header.comment as *const ::core::ffi::c_char,
            )
            .to_bytes_with_nul();
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            let hcrc = header.hcrc != 0;
            let mut beg_1: crate::zutil_h::ulg = (*s).pending;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if hcrc && (*s).pending > beg_1 {
                        if !checksum_gzip_header_range(
                            &mut (*strm).adler,
                            pending_buf,
                            beg_1,
                            (*s).pending,
                        ) {
                            return crate::zlib_h::Z_STREAM_ERROR;
                        }
                    }
                    flush_pending(strm);
                    if (*s).pending != 0 as crate::zutil_h::ulg {
                        (*s).last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                    beg_1 = 0 as crate::zutil_h::ulg;
                }
                let Some(done) = append_gzip_header_string_byte(state, pending_buf, comment) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if done {
                    break;
                }
            }
            if hcrc && (*s).pending > beg_1 {
                if !checksum_gzip_header_range(
                    &mut (*strm).adler,
                    pending_buf,
                    beg_1,
                    (*s).pending,
                ) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
            }
        }
        (*s).status = crate::src::deflate::HCRC_STATE;
    }
    if (*s).status == crate::src::deflate::HCRC_STATE {
        let Some(header) = gzip_header else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if header.hcrc != 0 {
            if (*s).pending.wrapping_add(2 as crate::zutil_h::ulg) > (*s).pending_buf_size {
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            if !write_gzip_header_crc(state, pending_buf, (*strm).adler) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            (*strm).adler = crate::src::crc32::crc32(0 as crate::stdlib::uLong, None);
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
            let state = &mut *s;
            let strm = &mut *state.strm;
            let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
            let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
            let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
            let input = if strm.avail_in == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
            };
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            let output = ::core::slice::from_raw_parts_mut(strm.next_out, strm.avail_out as usize);
            deflate_huff(
                state,
                strm,
                window,
                head,
                prev,
                input,
                pending_buf,
                output,
                flush,
            ) as ::core::ffi::c_uint
        } else if (*s).strategy == crate::zlib_h::Z_RLE {
            deflate_rle(s, flush) as ::core::ffi::c_uint
        } else {
            let Some(kind) = configuration_table
                .get((*s).level as usize)
                .map(|configuration| &configuration.func)
            else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            (match kind {
                CompressorKind::Stored => deflate_stored(s, flush),
                CompressorKind::Fast => deflate_fast(s, flush),
                CompressorKind::Slow => deflate_slow(s, flush),
            }) as ::core::ffi::c_uint
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
                let state = &mut *s;
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    state.pending_buf,
                    state.pending_buf_size as usize,
                );
                crate::src::trees::_tr_align(state, pending_buf);
            } else if flush != crate::zlib_h::Z_BLOCK {
                let state = &mut *s;
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    state.pending_buf,
                    state.pending_buf_size as usize,
                );
                crate::src::trees::tr_stored_block(
                    state,
                    pending_buf,
                    None,
                    0 as crate::zutil_h::ulg,
                    0 as ::core::ffi::c_int,
                );
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    if state.head.is_null() {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    }
                    let head = ::core::slice::from_raw_parts_mut(
                        state.head,
                        state.hash_size as usize,
                    );
                    if !clear_full_flush_hash(state, head) {
                        return crate::zlib_h::Z_STREAM_ERROR;
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
        let state = &mut *s;
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !write_gzip_trailer(state, pending_buf, (*strm).adler, (*strm).total_in) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
    } else {
        let state = &mut *s;
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !put_short_msb(
            state,
            pending_buf,
            ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
        ) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if !put_short_msb(
            state,
            pending_buf,
            ((*strm).adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
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
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate(strm, flush)
}
pub unsafe fn deflateEnd(stream: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    if stream.zalloc.is_none() || stream.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (stream.state as *mut crate::src::deflate::deflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(stream), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let status = state.status;
    if !state.pending_buf.is_null() {
        Some(stream.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            stream.opaque,
            state.pending_buf as crate::stdlib::voidpf,
        );
    }
    if !state.head.is_null() {
        Some(stream.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            stream.opaque,
            state.head as crate::stdlib::voidpf,
        );
    }
    if !state.prev.is_null() {
        Some(stream.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            stream.opaque,
            state.prev as crate::stdlib::voidpf,
        );
    }
    if !state.window.is_null() {
        Some(stream.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            stream.opaque,
            state.window as crate::stdlib::voidpf,
        );
    }
    Some(stream.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        stream.opaque,
        stream.state as crate::stdlib::voidpf,
    );
    stream.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
}
#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
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
    let Some(source_stream) = source.as_ref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if source_stream.zalloc.is_none() || source_stream.zfree.is_none() || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(source_state) =
        (source_stream.state as *const crate::src::deflate::deflate_state).as_ref()
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(source_stream), Some(source_state)) {
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
        deflateEnd(&mut *dest);
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
    (*ds).pending_out = (*ds)
        .pending_buf
        .offset((*ss).pending_out.offset_from((*ss).pending_buf) as isize);
    crate::stdlib::memcpy(
        (*ds).pending_out as *mut ::core::ffi::c_void,
        (*ss).pending_out as *const ::core::ffi::c_void,
        (*ss).pending as crate::__stddef_size_t_h::size_t,
    );
    (*ds).sym_buf =
        (*ds).pending_buf.offset((*ds).lit_bufsize as isize) as *mut crate::zutil_h::uchf;
    crate::stdlib::memcpy(
        (*ds).sym_buf as *mut ::core::ffi::c_void,
        (*ss).sym_buf as *const ::core::ffi::c_void,
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
fn longest_match(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
    prev: &[crate::src::deflate::Posf],
    mut cur_match: crate::src::deflate::IPos,
) -> crate::stdlib::uInt {
    let mut chain_length = state.max_chain_length;
    let mut best_len = state.prev_length as usize;
    let mut nice_match = state.nice_match;
    let limit = if state.strstart
        > state
            .w_size
            .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
    {
        state.strstart.wrapping_sub(
            state
                .w_size
                .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        ) as crate::src::deflate::IPos
    } else {
        NIL as crate::src::deflate::IPos
    };
    let strstart = state.strstart as usize;
    let Some(scan) = window.get(strstart..) else {
        return 0;
    };
    if state.prev_length >= state.good_match {
        chain_length >>= 2;
    }
    if nice_match as crate::stdlib::uInt > state.lookahead {
        nice_match = state.lookahead as ::core::ffi::c_int;
    }

    loop {
        let Some(candidate) = window.get(cur_match as usize..) else {
            break;
        };
        let max_len = scan
            .len()
            .min(candidate.len())
            .min(crate::zutil_h::MAX_MATCH as usize)
            .min(state.lookahead as usize);
        let mut len = 0usize;
        while len < max_len && scan[len] == candidate[len] {
            len += 1;
        }
        if len > best_len {
            state.match_start = cur_match as crate::stdlib::uInt;
            best_len = len;
            if len as ::core::ffi::c_int >= nice_match {
                break;
            }
        }

        let Some(&next_match) = prev.get((cur_match as crate::stdlib::uInt & state.w_mask) as usize)
        else {
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
    best_len.min(state.lookahead as usize) as crate::stdlib::uInt
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

fn flush_strategy_block(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    window: &[crate::stdlib::Bytef],
    pending_buf: &mut [crate::stdlib::Bytef],
    last: ::core::ffi::c_int,
) -> bool {
    let (source, stored_len) = if state.block_start < 0 {
        (None, 0)
    } else {
        let Ok(start) = usize::try_from(state.block_start) else {
            return false;
        };
        let Ok(end) = usize::try_from(state.strstart) else {
            return false;
        };
        let Some(source) = window.get(start..end) else {
            return false;
        };
        (Some(source), source.len() as crate::zutil_h::ulg)
    };
    crate::src::trees::tr_flush_block(state, strm, pending_buf, source, stored_len, last);
    state.block_start = state.strstart as ::core::ffi::c_long;
    true
}

fn write_stored_block_length(
    state: &crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    len: ::core::ffi::c_uint,
) -> bool {
    let Some(pending) = usize::try_from(state.pending).ok() else {
        return false;
    };
    let Some(start) = pending.checked_sub(4) else {
        return false;
    };
    let Some(header) = pending_buf.get_mut(start..pending) else {
        return false;
    };
    header[0] = len as crate::stdlib::Bytef;
    header[1] = (len >> 8 as ::core::ffi::c_int) as crate::stdlib::Bytef;
    header[2] = !len as crate::stdlib::Bytef;
    header[3] = (!len >> 8 as ::core::ffi::c_int) as crate::stdlib::Bytef;
    true
}

fn copy_stored_history(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    window: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    len: ::core::ffi::c_uint,
) -> bool {
    let Ok(copy) = usize::try_from(len) else {
        return false;
    };
    let Ok(start) = usize::try_from(state.block_start) else {
        return false;
    };
    let Some(end) = start.checked_add(copy) else {
        return false;
    };
    let Some(source) = window.get(start..end) else {
        return false;
    };
    let Some(destination) = output.get_mut(..copy) else {
        return false;
    };
    destination.copy_from_slice(source);
    strm.next_out = output.as_mut_ptr().wrapping_add(copy);
    strm.avail_out = strm.avail_out.wrapping_sub(len);
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    state.block_start += len as ::core::ffi::c_long;
    true
}

fn copy_stored_input(
    strm: &mut crate::zlib_h::z_stream,
    wrap: ::core::ffi::c_int,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) -> bool {
    let copied = read_buf(strm, wrap, input, output) as usize;
    if copied != output.len() {
        return false;
    }
    strm.next_out = output.as_mut_ptr().wrapping_add(copied);
    strm.avail_out = strm.avail_out.wrapping_sub(copied as crate::stdlib::uInt);
    strm.total_out = strm.total_out.wrapping_add(copied as crate::stdlib::uLong);
    true
}

fn update_stored_history(
    state: &mut crate::src::deflate::deflate_state,
    window: &mut [crate::stdlib::Bytef],
    input_tail: &[crate::stdlib::Bytef],
) -> bool {
    let used = input_tail.len();
    let Ok(wsize) = usize::try_from(state.w_size) else {
        return false;
    };
    let Ok(window_size) = usize::try_from(state.window_size) else {
        return false;
    };
    let Ok(mut strstart) = usize::try_from(state.strstart) else {
        return false;
    };
    if used == 0 || wsize == 0 || window.len() < window_size || strstart > window_size {
        return used == 0;
    }
    if used >= wsize {
        let Some(source) = input_tail.get(used - wsize..) else {
            return false;
        };
        let Some(destination) = window.get_mut(..wsize) else {
            return false;
        };
        destination.copy_from_slice(source);
        state.matches = 2;
        state.strstart = state.w_size;
        state.insert = state.strstart;
    } else {
        if window_size - strstart <= used {
            let Some(retained) = strstart.checked_sub(wsize) else {
                return false;
            };
            let Some(source_end) = wsize.checked_add(retained) else {
                return false;
            };
            if source_end > window.len() {
                return false;
            }
            window.copy_within(wsize..source_end, 0);
            strstart = retained;
            state.strstart = strstart as crate::stdlib::uInt;
            if state.matches < 2 {
                state.matches = state.matches.wrapping_add(1);
            }
            if state.insert > state.strstart {
                state.insert = state.strstart;
            }
        }
        let Some(end) = strstart.checked_add(used) else {
            return false;
        };
        let Some(destination) = window.get_mut(strstart..end) else {
            return false;
        };
        destination.copy_from_slice(input_tail);
        state.strstart = state.strstart.wrapping_add(input_tail.len() as crate::stdlib::uInt);
        state.insert = state.insert.wrapping_add(
            if input_tail.len() as crate::stdlib::uInt
                > state.w_size.wrapping_sub(state.insert)
            {
                state.w_size.wrapping_sub(state.insert)
            } else {
                input_tail.len() as crate::stdlib::uInt
            },
        );
    }
    state.block_start = state.strstart as ::core::ffi::c_long;
    true
}

fn slide_stored_window(
    state: &mut crate::src::deflate::deflate_state,
    window: &mut [crate::stdlib::Bytef],
) -> Option<crate::stdlib::uInt> {
    let w_size = usize::try_from(state.w_size).ok()?;
    let strstart = usize::try_from(state.strstart).ok()?;
    let block_start = usize::try_from(state.block_start).ok()?;
    let retained = strstart.checked_sub(w_size)?;
    let source_end = w_size.checked_add(retained)?;
    if block_start < w_size || source_end > window.len() {
        return None;
    }
    window.copy_within(w_size..source_end, 0);
    state.block_start -= state.w_size as ::core::ffi::c_long;
    state.strstart -= state.w_size;
    if state.matches < 2 as crate::stdlib::uInt {
        state.matches += 1;
    }
    if state.insert > state.strstart {
        state.insert = state.strstart;
    }
    Some(state.w_size)
}

unsafe extern "C" fn deflate_stored(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    // The strategy is still called through the translated raw dispatcher.
    // Convert that state and its stream once at this boundary; the block
    // bookkeeping below can then use the checked slice helpers directly.
    let state = &mut *s;
    let strm = &mut *state.strm;
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
    let mut used: ::core::ffi::c_uint = strm.avail_in as ::core::ffi::c_uint;
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        have = (state.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
            >> 3 as ::core::ffi::c_int;
        if strm.avail_out < have {
            break;
        }
        have = (strm.avail_out as ::core::ffi::c_uint).wrapping_sub(have);
        left = (state.strstart as ::core::ffi::c_long - state.block_start) as ::core::ffi::c_uint;
        if len as crate::zutil_h::ulg
            > (left as crate::zutil_h::ulg)
                .wrapping_add(strm.avail_in as crate::zutil_h::ulg)
        {
            len = (left as crate::stdlib::uInt).wrapping_add(strm.avail_in)
                as ::core::ffi::c_uint;
        }
        if len > have {
            len = have;
        }
        if len < min_block
            && (len == 0 as ::core::ffi::c_uint && flush != crate::zlib_h::Z_FINISH
                || flush == crate::zlib_h::Z_NO_FLUSH
                || len != (left as crate::stdlib::uInt).wrapping_add(strm.avail_in))
        {
            break;
        }
        last = if flush == crate::zlib_h::Z_FINISH
            && len == (left as crate::stdlib::uInt).wrapping_add(strm.avail_in)
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        crate::src::trees::tr_stored_block(
            state,
            pending_buf,
            None,
            0 as crate::zutil_h::ulg,
            last,
        );
        if !write_stored_block_length(state, pending_buf, len) {
            return need_more;
        }
        flush_pending(state.strm);
        if left != 0 {
            if left > len {
                left = len;
            }
            let window = ::core::slice::from_raw_parts(
                state.window,
                state.window_size as usize,
            );
            let output = ::core::slice::from_raw_parts_mut(strm.next_out, left as usize);
            if !copy_stored_history(state, strm, window, output, left) {
                return need_more;
            }
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            let input = ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize);
            let output = ::core::slice::from_raw_parts_mut(strm.next_out, len as usize);
            if !copy_stored_input(strm, state.wrap, input, output) {
                return need_more;
            }
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub(strm.avail_in as ::core::ffi::c_uint);
    if used != 0 {
        let input_tail = ::core::slice::from_raw_parts(
            strm.next_in.offset(-(used as isize)),
            used as usize,
        );
        let window = ::core::slice::from_raw_parts_mut(
            state.window,
            state.window_size as usize,
        );
        if !update_stored_history(state, window, input_tail) {
            return need_more;
        }
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
        && strm.avail_in == 0 as crate::stdlib::uInt
        && state.strstart as ::core::ffi::c_long == state.block_start
    {
        return block_done;
    }
    have = state
        .window_size
        .wrapping_sub(state.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if strm.avail_in > have && state.block_start >= state.w_size as ::core::ffi::c_long {
        let window = ::core::slice::from_raw_parts_mut(
            state.window,
            state.window_size as usize,
        );
        let Some(slid) = slide_stored_window(state, window) else {
            return need_more;
        };
        have = have.wrapping_add(slid);
    }
    if have > strm.avail_in {
        have = strm.avail_in as ::core::ffi::c_uint;
    }
    if have != 0 {
        let input = ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize);
        let output = ::core::slice::from_raw_parts_mut(
            state.window.offset(state.strstart as isize),
            have as usize,
        );
        read_buf(strm, state.wrap, input, output);
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
            && strm.avail_in == 0 as crate::stdlib::uInt
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == crate::zlib_h::Z_FINISH
            && strm.avail_in == 0 as crate::stdlib::uInt
            && len == left
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let source = ::core::slice::from_raw_parts(
            state.window.offset(state.block_start as isize),
            len as usize,
        );
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        crate::src::trees::tr_stored_block(
            state,
            pending_buf,
            Some(source),
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

fn insert_hash(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
    prev: &mut [crate::src::deflate::Posf],
    head: &mut [crate::src::deflate::Posf],
) -> Option<crate::src::deflate::IPos> {
    let hash_byte_index = state
        .strstart
        .checked_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt - 1)?
        as usize;
    let hash_byte = *window.get(hash_byte_index)? as crate::stdlib::uInt;
    state.ins_h = ((state.ins_h << state.hash_shift) ^ hash_byte) & state.hash_mask;
    let hash_index = state.ins_h as usize;
    let prev_index = (state.strstart & state.w_mask) as usize;
    let previous = *head.get(hash_index)?;
    *prev.get_mut(prev_index)? = previous;
    *head.get_mut(hash_index)? = state.strstart as crate::src::deflate::Posf;
    Some(previous as crate::src::deflate::IPos)
}

fn seed_hash(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
) -> bool {
    let Some(first) = window.get(state.strstart as usize) else {
        return false;
    };
    let Some(second) = window.get(state.strstart.wrapping_add(1) as usize) else {
        return false;
    };
    state.ins_h = *first as crate::stdlib::uInt;
    state.ins_h = ((state.ins_h << state.hash_shift) ^ *second as crate::stdlib::uInt)
        & state.hash_mask;
    true
}

fn tally_literal(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
    pending_buf: &mut [crate::stdlib::Bytef],
) -> Option<::core::ffi::c_int> {
    let literal = *window.get(state.strstart as usize)? as ::core::ffi::c_uint;
    Some(crate::src::trees::tr_tally(
        state,
        pending_buf,
        0 as ::core::ffi::c_uint,
        literal,
    ))
}

fn tally_previous_literal(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
    pending_buf: &mut [crate::stdlib::Bytef],
) -> Option<::core::ffi::c_int> {
    let previous = state.strstart.checked_sub(1)? as usize;
    let literal = *window.get(previous)? as ::core::ffi::c_uint;
    Some(crate::src::trees::tr_tally(
        state,
        pending_buf,
        0 as ::core::ffi::c_uint,
        literal,
    ))
}

fn rle_match_length(
    window: &[crate::stdlib::Bytef],
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    let min_match = crate::zutil_h::MIN_MATCH as usize;
    if lookahead < min_match as crate::stdlib::uInt || strstart == 0 {
        return 0;
    }
    let Ok(start) = usize::try_from(strstart) else {
        return 0;
    };
    let Some(&previous) = start.checked_sub(1).and_then(|index| window.get(index)) else {
        return 0;
    };
    let Some(run) = window.get(start..) else {
        return 0;
    };
    if run.get(..min_match).is_none() || run[..min_match].iter().any(|&byte| byte != previous) {
        return 0;
    }
    let count = run
        .iter()
        .take(crate::zutil_h::MAX_MATCH as usize)
        .take_while(|&&byte| byte == previous)
        .count();
    count.min(lookahead as usize) as crate::stdlib::uInt
}

unsafe extern "C" fn deflate_fast(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window_from_raw!(s);
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
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
            let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
            let Some(inserted) = insert_hash(state, window, prev, head) else {
                return need_more;
            };
            hash_head = inserted;
        }
        if hash_head != NIL as crate::src::deflate::IPos
            && ((*s).strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                <= (*s)
                    .w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
        {
            let state = &mut *s;
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let prev = ::core::slice::from_raw_parts(state.prev, state.w_size as usize);
            state.match_length = longest_match(state, window, prev, hash_head);
        }
        if (*s).match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len: crate::zutil_h::uch =
                (*s).match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush =
                (*s).strstart.wrapping_sub((*s).match_start) as crate::zutil_h::ush;
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            bflush = crate::src::trees::tr_tally(
                state,
                pending_buf,
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
                    let state = &mut *s;
                    let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
                    let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
                    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
                    let Some(inserted) = insert_hash(state, window, prev, head) else {
                        return need_more;
                    };
                    hash_head = inserted;
                    (*s).match_length = (*s).match_length.wrapping_sub(1);
                    if (*s).match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                (*s).strstart = (*s).strstart.wrapping_add(1);
            } else {
                (*s).strstart = (*s).strstart.wrapping_add((*s).match_length);
                (*s).match_length = 0 as crate::stdlib::uInt;
                let state = &mut *s;
                let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
                if !seed_hash(state, window) {
                    return need_more;
                }
            }
        } else {
            let state = &mut *s;
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            let Some(flush) = tally_literal(state, window, pending_buf) else {
                return need_more;
            };
            bflush = flush;
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            let state = &mut *s;
            let strm = &mut *state.strm;
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            if !flush_strategy_block(state, strm, window, pending_buf, 0) {
                return need_more;
            }
            flush_pending(strm);
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
        let state = &mut *s;
        let strm = &mut *state.strm;
        let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !flush_strategy_block(state, strm, window, pending_buf, 1) {
            return need_more;
        }
        flush_pending(strm);
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
        let state = &mut *s;
        let strm = &mut *state.strm;
        let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !flush_strategy_block(state, strm, window, pending_buf, 0) {
            return need_more;
        }
        flush_pending(strm);
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
            fill_window_from_raw!(s);
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
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
            let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
            let Some(inserted) = insert_hash(state, window, prev, head) else {
                return need_more;
            };
            hash_head = inserted;
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
            let state = &mut *s;
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let prev = ::core::slice::from_raw_parts(state.prev, state.w_size as usize);
            state.match_length = longest_match(state, window, prev, hash_head);
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
            let len: crate::zutil_h::uch =
                (*s).prev_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush = ((*s).strstart as crate::src::deflate::IPos)
                .wrapping_sub(1 as crate::src::deflate::IPos)
                .wrapping_sub((*s).prev_match)
                as crate::zutil_h::ush;
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            bflush = crate::src::trees::tr_tally(
                state,
                pending_buf,
                dist as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
            );
            (*s).lookahead = (*s)
                .lookahead
                .wrapping_sub((*s).prev_length.wrapping_sub(1 as crate::stdlib::uInt));
            (*s).prev_length = (*s).prev_length.wrapping_sub(2 as crate::stdlib::uInt);
            loop {
                (*s).strstart = (*s).strstart.wrapping_add(1);
                if (*s).strstart <= max_insert {
                    let state = &mut *s;
                    let window =
                        ::core::slice::from_raw_parts(state.window, state.window_size as usize);
                    let prev =
                        ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
                    let head = ::core::slice::from_raw_parts_mut(
                        state.head,
                        state.hash_size as usize,
                    );
                    let Some(inserted) = insert_hash(state, window, prev, head) else {
                        return need_more;
                    };
                    hash_head = inserted;
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
                let state = &mut *s;
                let strm = &mut *state.strm;
                let window =
                    ::core::slice::from_raw_parts(state.window, state.window_size as usize);
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    state.pending_buf,
                    state.pending_buf_size as usize,
                );
                if !flush_strategy_block(state, strm, window, pending_buf, 0) {
                    return need_more;
                }
                flush_pending(strm);
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
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            let Some(flush) = tally_previous_literal(state, window, pending_buf) else {
                return need_more;
            };
            bflush = flush;
            if bflush != 0 {
                let state = &mut *s;
                let strm = &mut *state.strm;
                let window =
                    ::core::slice::from_raw_parts(state.window, state.window_size as usize);
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    state.pending_buf,
                    state.pending_buf_size as usize,
                );
                if !flush_strategy_block(state, strm, window, pending_buf, 0) {
                    return need_more;
                }
                flush_pending(strm);
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
        let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        let Some(flush) = tally_previous_literal(state, window, pending_buf) else {
            return need_more;
        };
        bflush = flush;
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
        let state = &mut *s;
        let strm = &mut *state.strm;
        let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !flush_strategy_block(state, strm, window, pending_buf, 1) {
            return need_more;
        }
        flush_pending(strm);
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
        let state = &mut *s;
        let strm = &mut *state.strm;
        let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !flush_strategy_block(state, strm, window, pending_buf, 0) {
            return need_more;
        }
        flush_pending(strm);
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

unsafe extern "C" fn deflate_rle(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_window_from_raw!(s);
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
        if (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            && (*s).strstart > 0 as crate::stdlib::uInt
        {
            let state = &mut *s;
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            state.match_length = rle_match_length(window, state.strstart, state.lookahead);
        }
        if (*s).match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len: crate::zutil_h::uch =
                (*s).match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            bflush = crate::src::trees::tr_tally(
                state,
                pending_buf,
                1 as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
            );
            (*s).lookahead = (*s).lookahead.wrapping_sub((*s).match_length);
            (*s).strstart = (*s).strstart.wrapping_add((*s).match_length);
            (*s).match_length = 0 as crate::stdlib::uInt;
        } else {
            let state = &mut *s;
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            let Some(flush) = tally_literal(state, window, pending_buf) else {
                return need_more;
            };
            bflush = flush;
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            let state = &mut *s;
            let strm = &mut *state.strm;
            let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            if !flush_strategy_block(state, strm, window, pending_buf, 0) {
                return need_more;
            }
            flush_pending(strm);
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
        let state = &mut *s;
        let strm = &mut *state.strm;
        let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !flush_strategy_block(state, strm, window, pending_buf, 1) {
            return need_more;
        }
        flush_pending(strm);
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
        let state = &mut *s;
        let strm = &mut *state.strm;
        let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        if !flush_strategy_block(state, strm, window, pending_buf, 0) {
            return need_more;
        }
        flush_pending(strm);
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

fn flush_huff_block(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    window: &[crate::stdlib::Bytef],
    pending_buf: &mut [crate::stdlib::Bytef],
    last: ::core::ffi::c_int,
) -> bool {
    let (source, stored_len) = if state.block_start < 0 {
        (None, 0)
    } else {
        let Ok(start) = usize::try_from(state.block_start) else {
            return false;
        };
        let Ok(end) = usize::try_from(state.strstart) else {
            return false;
        };
        let Some(source) = window.get(start..end) else {
            return false;
        };
        (Some(source), source.len() as crate::zutil_h::ulg)
    };
    crate::src::trees::tr_flush_block(state, strm, pending_buf, source, stored_len, last);
    state.block_start = state.strstart as ::core::ffi::c_long;
    true
}

fn flush_huff_pending(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    pending_buf: &mut [crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) -> bool {
    let Some(pending_start) = state
        .pending_out
        .addr()
        .checked_sub(pending_buf.as_ptr().addr())
    else {
        return false;
    };
    if pending_start > pending_buf.len() {
        return false;
    }
    let Ok(available) = usize::try_from(strm.avail_out) else {
        return false;
    };
    let Some(output_start) = output.len().checked_sub(available) else {
        return false;
    };
    let Some(output) = output.get_mut(output_start..) else {
        return false;
    };
    flush_pending_impl(strm, state, pending_buf, pending_start, output)
}

fn deflate_huff(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    input: &[crate::stdlib::Bytef],
    pending_buf: &mut [crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    deflate_huff_impl(
        state,
        strm,
        window,
        head,
        prev,
        input,
        pending_buf,
        output,
        flush,
    )
}

fn deflate_huff_impl(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    mut input: &[crate::stdlib::Bytef],
    pending_buf: &mut [crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if state.lookahead == 0 as crate::stdlib::uInt {
            let available_before = strm.avail_in as usize;
            fill_window(state, strm, window, head, prev, input);
            let Some(consumed) = available_before.checked_sub(strm.avail_in as usize) else {
                return need_more;
            };
            let Some(remaining) = input.get(consumed..) else {
                return need_more;
            };
            input = remaining;
            if state.lookahead == 0 as crate::stdlib::uInt {
                if flush == crate::zlib_h::Z_NO_FLUSH {
                    return need_more;
                }
                break;
            }
        }
        state.match_length = 0 as crate::stdlib::uInt;
        let Some(&cc) = window.get(state.strstart as usize) else {
            return need_more;
        };
        bflush = crate::src::trees::tr_tally(
            state,
            pending_buf,
            0 as ::core::ffi::c_uint,
            cc as ::core::ffi::c_uint,
        );
        state.lookahead = state.lookahead.wrapping_sub(1);
        state.strstart = state.strstart.wrapping_add(1);
        if bflush != 0 {
            if !flush_huff_block(state, strm, window, pending_buf, 0 as ::core::ffi::c_int) {
                return need_more;
            }
            if !flush_huff_pending(state, strm, pending_buf, output) {
                return need_more;
            }
            if strm.avail_out == 0 as crate::stdlib::uInt {
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
        if !flush_huff_block(state, strm, window, pending_buf, 1 as ::core::ffi::c_int) {
            return need_more;
        }
        if !flush_huff_pending(state, strm, pending_buf, output) {
            return need_more;
        }
        if strm.avail_out == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        if !flush_huff_block(state, strm, window, pending_buf, 0 as ::core::ffi::c_int) {
            return need_more;
        }
        if !flush_huff_pending(state, strm, pending_buf, output) {
            return need_more;
        }
        if strm.avail_out == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}
