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
    pub freq: crate::zutil_h::ush,
    pub dad: crate::zutil_h::ush,
}

pub type static_tree_desc = crate::src::deflate::static_tree_desc_s;

pub type tree_desc = crate::src::deflate::tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tree_desc_s {
    pub max_code: ::core::ffi::c_int,
    pub stat_desc_kind: ::core::ffi::c_int,
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
    pub pending_out_offset: crate::zutil_h::ulg,
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

pub use crate::src::adler32::adler32_ffi;
pub use crate::src::crc32::crc32_ffi;
pub use crate::src::crc32::crc32_z_ffi;
pub use crate::src::trees::_dist_code;
pub use crate::src::trees::_length_code;
pub use crate::src::trees::_tr_align_ffi;
pub use crate::src::trees::_tr_flush_bits_ffi;
pub use crate::src::trees::_tr_flush_block_ffi;
pub use crate::src::trees::_tr_init_ffi;
pub use crate::src::trees::_tr_stored_block_ffi;
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
pub enum DeflateFunc {
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
    pub func: DeflateFunc,
}
#[no_mangle]

pub static deflate_copyright: [::core::ffi::c_char; 70] =
    crate::c_char_array(b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0");

pub const NIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TOO_FAR: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

static configuration_table: [config; 10] = [
    config_s {
        good_length: 0 as crate::zutil_h::ush,
        max_lazy: 0 as crate::zutil_h::ush,
        nice_length: 0 as crate::zutil_h::ush,
        max_chain: 0 as crate::zutil_h::ush,
        func: DeflateFunc::Stored,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        func: DeflateFunc::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        func: DeflateFunc::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: DeflateFunc::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        func: DeflateFunc::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: DeflateFunc::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        func: DeflateFunc::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        func: DeflateFunc::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        func: DeflateFunc::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        func: DeflateFunc::Slow,
    },
];

fn slide_hash_entry(
    m: ::core::ffi::c_uint,
    wsize: crate::stdlib::uInt,
) -> crate::src::deflate::Posf {
    (if m >= wsize {
        m.wrapping_sub(wsize as ::core::ffi::c_uint)
    } else {
        NIL as ::core::ffi::c_uint
    }) as crate::src::deflate::Pos as crate::src::deflate::Posf
}

fn slide_hash_entries(entries: &mut [crate::src::deflate::Posf], wsize: crate::stdlib::uInt) {
    for entry in entries.iter_mut().rev() {
        *entry = slide_hash_entry(*entry as ::core::ffi::c_uint, wsize);
    }
}

fn slide_hash(s: &mut crate::src::deflate::deflate_state) {
    let wsize = s.w_size;
    let head = unsafe { &mut *::core::ptr::slice_from_raw_parts_mut(s.head, s.hash_size as usize) };
    slide_hash_entries(head, wsize);
    let prev = unsafe { &mut *::core::ptr::slice_from_raw_parts_mut(s.prev, wsize as usize) };
    slide_hash_entries(prev, wsize);
    s.slid = 1 as ::core::ffi::c_int;
}

fn deflate_hash_update(
    ins_h: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    byte: crate::stdlib::Byte,
) -> crate::stdlib::uInt {
    (ins_h << hash_shift ^ byte as crate::stdlib::uInt) & hash_mask
}

fn read_buf_updated_adler(
    wrap: ::core::ffi::c_int,
    adler: crate::stdlib::uLong,
    bytes: &[crate::stdlib::Bytef],
) -> crate::stdlib::uLong {
    if wrap == 1 as ::core::ffi::c_int {
        crate::src::adler32::adler32_update(adler, bytes)
    } else if wrap == 2 as ::core::ffi::c_int {
        crate::src::crc32::crc32_update(adler, bytes)
    } else {
        adler
    }
}

fn read_buf(
    strm: &mut crate::zlib_h::z_stream_s,
    wrap: ::core::ffi::c_int,
    out: &mut [crate::stdlib::Bytef],
) -> ::core::ffi::c_uint {
    let mut len: ::core::ffi::c_uint = strm.avail_in as ::core::ffi::c_uint;
    if len as usize > out.len() {
        len = out.len() as ::core::ffi::c_uint;
    }
    if len == 0 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_uint;
    }
    strm.avail_in = strm.avail_in.wrapping_sub(len);
    let copied = &mut out[..len as usize];
    let input = unsafe { ::core::slice::from_raw_parts(strm.next_in, len as usize) };
    copied.copy_from_slice(input);
    strm.adler = read_buf_updated_adler(wrap, strm.adler, copied);
    strm.next_in = strm.next_in.wrapping_add(len as usize);
    strm.total_in = strm.total_in.wrapping_add(len as crate::stdlib::uLong);
    return len;
}

unsafe fn fill_window(mut s: *mut crate::src::deflate::deflate_state) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    let mut wsize: crate::stdlib::uInt = (*s).w_size;
    loop {
        more = (*s)
            .window_size
            .wrapping_sub((*s).lookahead as crate::zutil_h::ulg)
            .wrapping_sub((*s).strstart as crate::zutil_h::ulg)
            as ::core::ffi::c_uint;
        if ::core::mem::size_of::<::core::ffi::c_int>() as usize <= 2 as usize {
            if more == 0 as ::core::ffi::c_uint
                && (*s).strstart == 0 as crate::stdlib::uInt
                && (*s).lookahead == 0 as crate::stdlib::uInt
            {
                more = wsize as ::core::ffi::c_uint;
            } else if more == -1 as ::core::ffi::c_int as ::core::ffi::c_uint {
                more = more.wrapping_sub(1);
            }
        }
        if (*s).strstart
            >= wsize.wrapping_add(
                (*s).w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            let state = &mut *s;
            crate::stdlib::memcpy(
                state.window as *mut ::core::ffi::c_void,
                state.window.wrapping_add(wsize as usize) as *const ::core::ffi::c_void,
                wsize.wrapping_sub(more) as crate::__stddef_size_t_h::size_t,
            );
            state.match_start = state.match_start.wrapping_sub(wsize);
            state.strstart = state.strstart.wrapping_sub(wsize);
            state.block_start -= wsize as ::core::ffi::c_long;
            if state.insert > state.strstart {
                state.insert = state.strstart;
            }
            slide_hash(state);
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if (*(*s).strm).avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        let state = &mut *s;
        let strm = &mut *state.strm;
        let out = ::core::slice::from_raw_parts_mut(
            state
                .window
                .wrapping_add(state.strstart as usize)
                .wrapping_add(state.lookahead as usize),
            more as usize,
        );
        n = read_buf(strm, state.wrap, out);
        (*s).lookahead = (*s).lookahead.wrapping_add(n);
        if (*s).lookahead.wrapping_add((*s).insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        {
            let mut str: crate::stdlib::uInt = (*s).strstart.wrapping_sub((*s).insert);
            (*s).ins_h = *(*s).window.wrapping_add(str as usize) as crate::stdlib::uInt;
            (*s).ins_h = deflate_hash_update(
                (*s).ins_h,
                (*s).hash_shift,
                (*s).hash_mask,
                *(*s)
                    .window
                    .wrapping_add(str.wrapping_add(1 as crate::stdlib::uInt) as usize),
            );
            while (*s).insert != 0 {
                (*s).ins_h = deflate_hash_update(
                    (*s).ins_h,
                    (*s).hash_shift,
                    (*s).hash_mask,
                    *(*s).window.wrapping_add(
                        str.wrapping_add(3 as crate::stdlib::uInt)
                            .wrapping_sub(1 as crate::stdlib::uInt)
                            as usize,
                    ),
                );
                *(*s).prev.wrapping_add((str & (*s).w_mask) as usize) =
                    *(*s).head.wrapping_add((*s).ins_h as usize);
                *(*s).head.wrapping_add((*s).ins_h as usize) =
                    str as crate::src::deflate::Pos as crate::src::deflate::Posf;
                str = str.wrapping_add(1);
                (*s).insert = (*s).insert.wrapping_sub(1);
                if (*s).lookahead.wrapping_add((*s).insert)
                    < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                {
                    break;
                }
            }
        }
        if !((*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
            && (*(*s).strm).avail_in != 0 as crate::stdlib::uInt)
        {
            break;
        }
    }
    if (*s).high_water < (*s).window_size {
        let mut curr: crate::zutil_h::ulg = ((*s).strstart as crate::zutil_h::ulg)
            .wrapping_add((*s).lookahead as crate::zutil_h::ulg);
        let mut init: crate::zutil_h::ulg = 0;
        if (*s).high_water < curr {
            init = (*s).window_size.wrapping_sub(curr);
            if init > crate::src::deflate::WIN_INIT as crate::zutil_h::ulg {
                init = crate::src::deflate::WIN_INIT as crate::zutil_h::ulg;
            }
            crate::stdlib::memset(
                (*s).window.wrapping_add(curr as usize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                init as ::core::ffi::c_uint as crate::__stddef_size_t_h::size_t,
            );
            (*s).high_water = curr.wrapping_add(init);
        } else if (*s).high_water
            < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
        {
            init = curr
                .wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
                .wrapping_sub((*s).high_water);
            if init > (*s).window_size.wrapping_sub((*s).high_water) {
                init = (*s).window_size.wrapping_sub((*s).high_water);
            }
            crate::stdlib::memset(
                (*s).window.wrapping_add((*s).high_water as usize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                init as ::core::ffi::c_uint as crate::__stddef_size_t_h::size_t,
            );
            (*s).high_water = (*s).high_water.wrapping_add(init);
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
    deflateInit2__ffi(
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

#[derive(Copy, Clone)]
struct DeflateInit2Config {
    level: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
    window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
}

fn deflate_init2_config(
    mut level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    mut window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Option<DeflateInit2Config> {
    let mut wrap = 1 as ::core::ffi::c_int;
    level = deflate_params_level(level);
    if window_bits < 0 as ::core::ffi::c_int {
        wrap = 0 as ::core::ffi::c_int;
        if window_bits < -15 as ::core::ffi::c_int {
            return None;
        }
        window_bits = -window_bits;
    } else if window_bits > 15 as ::core::ffi::c_int {
        wrap = 2 as ::core::ffi::c_int;
        window_bits -= 16 as ::core::ffi::c_int;
    }
    if mem_level < 1 as ::core::ffi::c_int
        || mem_level > crate::stdlib::MAX_MEM_LEVEL
        || method != crate::zlib_h::Z_DEFLATED
        || window_bits < 8 as ::core::ffi::c_int
        || window_bits > 15 as ::core::ffi::c_int
        || level < 0 as ::core::ffi::c_int
        || level > 9 as ::core::ffi::c_int
        || strategy < 0 as ::core::ffi::c_int
        || strategy > crate::zlib_h::Z_FIXED
        || window_bits == 8 as ::core::ffi::c_int && wrap != 1 as ::core::ffi::c_int
    {
        return None;
    }
    if window_bits == 8 as ::core::ffi::c_int {
        window_bits = 9 as ::core::ffi::c_int;
    }
    Some(DeflateInit2Config {
        level,
        wrap,
        window_bits,
        mem_level,
        strategy,
        method,
    })
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
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    static my_version: [::core::ffi::c_char; 15] = crate::zlib_h::ZLIB_VERSION;
    if version.is_null()
        || *version as ::core::ffi::c_int
            != my_version[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        || stream_size as usize != ::core::mem::size_of::<crate::zlib_h::z_stream>() as usize
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
    let Some(config) = deflate_init2_config(level, method, windowBits, memLevel, strategy) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
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
        ::core::mem::size_of::<crate::src::deflate::deflate_state>()
            as crate::__stddef_size_t_h::size_t,
    );
    (*strm).state = s as *mut crate::src::deflate::internal_state;
    (*s).strm = strm;
    (*s).status = crate::src::deflate::INIT_STATE;
    (*s).wrap = config.wrap;
    (*s).gzhead = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
    (*s).w_bits = config.window_bits as crate::stdlib::uInt;
    (*s).w_size = ((1 as ::core::ffi::c_int) << (*s).w_bits) as crate::stdlib::uInt;
    (*s).w_mask = (*s).w_size.wrapping_sub(1 as crate::stdlib::uInt);
    (*s).hash_bits =
        (config.mem_level as crate::stdlib::uInt).wrapping_add(7 as crate::stdlib::uInt);
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
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>() as usize)
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
    (*s).lit_bufsize = ((1 as ::core::ffi::c_int) << config.mem_level + 6 as ::core::ffi::c_int)
        as crate::stdlib::uInt;
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
        (*strm).msg = crate::src::zutil::zError(crate::zlib_h::Z_MEM_ERROR).as_ptr()
            as *mut ::core::ffi::c_char;
        deflateEnd_ffi(strm);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*s).sym_buf = (*s).pending_buf.offset((*s).lit_bufsize as isize) as *mut crate::zutil_h::uchf;
    (*s).sym_end = (*s)
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    (*s).level = config.level;
    (*s).strategy = config.strategy;
    (*s).method = config.method as crate::stdlib::Byte;
    let state = &mut *s;
    deflate_reset_keep_state(&mut *strm, state);
    let head_ptr = state.head;
    let hash_size = state.hash_size as usize;
    let head = ::core::slice::from_raw_parts_mut(head_ptr, hash_size);
    lm_init(state, head);
    return crate::zlib_h::Z_OK;
}
fn deflate_status_is_valid(status: ::core::ffi::c_int) -> bool {
    status == crate::src::deflate::INIT_STATE
        || status == crate::src::deflate::GZIP_STATE
        || status == crate::src::deflate::EXTRA_STATE
        || status == crate::src::deflate::NAME_STATE
        || status == crate::src::deflate::COMMENT_STATE
        || status == crate::src::deflate::HCRC_STATE
        || status == crate::src::deflate::BUSY_STATE
        || status == crate::src::deflate::FINISH_STATE
}

fn deflate_state_fields_are_valid(state: &crate::src::deflate::deflate_state) -> bool {
    deflate_status_is_valid(state.status)
}

macro_rules! deflate_state_check_raw {
    ($strm:expr) => {{
        let strm = $strm;
        if strm.is_null() {
            1 as ::core::ffi::c_int
        } else {
            let strm_ref = &*strm;
            if strm_ref.zalloc.is_none() || strm_ref.zfree.is_none() {
                1 as ::core::ffi::c_int
            } else {
                let s = strm_ref.state as *mut crate::src::deflate::deflate_state;
                if s.is_null() {
                    1 as ::core::ffi::c_int
                } else {
                    let state = &*s;
                    (state.strm != strm || !deflate_state_fields_are_valid(state))
                        as ::core::ffi::c_int
                }
            }
        }
    }};
}

fn deflate_dictionary_state_accepts(
    wrap: ::core::ffi::c_int,
    status: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
) -> bool {
    wrap != 2 as ::core::ffi::c_int
        && !(wrap == 1 as ::core::ffi::c_int && status != crate::src::deflate::INIT_STATE)
        && lookahead == 0
}

#[export_name = "deflateSetDictionary"]

pub unsafe extern "C" fn deflateSetDictionary_ffi(
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
    if deflate_state_check_raw!(strm) != 0 || dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    wrap = (*s).wrap;
    if !deflate_dictionary_state_accepts(wrap, (*s).status, (*s).lookahead) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if wrap == 1 as ::core::ffi::c_int {
        let dictionary_slice = ::core::slice::from_raw_parts(dictionary, dictLength as usize);
        (*strm).adler = crate::src::adler32::adler32_update((*strm).adler, dictionary_slice);
    }
    (*s).wrap = 0 as ::core::ffi::c_int;
    if dictLength >= (*s).w_size {
        if wrap == 0 as ::core::ffi::c_int {
            let head = ::core::slice::from_raw_parts_mut((*s).head, (*s).hash_size as usize);
            deflate_clear_hash_head(head);
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
            (*s).ins_h = deflate_hash_update(
                (*s).ins_h,
                (*s).hash_shift,
                (*s).hash_mask,
                *(*s).window.offset(
                    str.wrapping_add(3 as crate::stdlib::uInt)
                        .wrapping_sub(1 as crate::stdlib::uInt) as isize,
                ),
            );
            *(*s).prev.offset((str & (*s).w_mask) as isize) =
                *(*s).head.offset((*s).ins_h as isize);
            *(*s).head.offset((*s).ins_h as isize) =
                str as crate::src::deflate::Pos as crate::src::deflate::Posf;
            str = str.wrapping_add(1);
            n = n.wrapping_sub(1);
            if !(n != 0) {
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
pub fn deflateGetDictionary(
    state: &crate::src::deflate::deflate_state,
) -> (crate::stdlib::uInt, crate::stdlib::uInt) {
    let mut len: crate::stdlib::uInt = 0;
    len = state.strstart.wrapping_add(state.lookahead);
    if len > state.w_size {
        len = state.w_size;
    }
    return (
        state
            .strstart
            .wrapping_add(state.lookahead)
            .wrapping_sub(len),
        len,
    );
}
#[export_name = "deflateGetDictionary"]

pub unsafe extern "C" fn deflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*((*strm).state as *mut crate::src::deflate::deflate_state);
    let (offset, len) = deflateGetDictionary(state);
    if !dictionary.is_null() && len != 0 {
        let dictionary = ::core::slice::from_raw_parts_mut(dictionary, len as usize);
        let window =
            ::core::slice::from_raw_parts(state.window.wrapping_add(offset as usize), len as usize);
        copy_deflate_dictionary(dictionary, window);
    }
    if !dictLength.is_null() {
        *dictLength = len;
    }
    return crate::zlib_h::Z_OK;
}

struct DeflateResetKeepConfig {
    wrap: ::core::ffi::c_int,
    status: ::core::ffi::c_int,
    adler: crate::stdlib::uLong,
}

fn deflate_reset_keep_config(mut wrap: ::core::ffi::c_int) -> DeflateResetKeepConfig {
    if wrap < 0 as ::core::ffi::c_int {
        wrap = -wrap;
    }
    let status = if wrap == 2 as ::core::ffi::c_int {
        crate::src::deflate::GZIP_STATE
    } else {
        crate::src::deflate::INIT_STATE
    };
    let adler = if wrap == 2 as ::core::ffi::c_int {
        crate::src::crc32::crc32_initial()
    } else {
        crate::src::adler32::adler32_initial()
    };

    DeflateResetKeepConfig {
        wrap,
        status,
        adler,
    }
}

fn deflate_reset_keep_state(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::deflate::deflate_state,
) {
    strm.total_out = 0 as crate::stdlib::uLong;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = crate::zlib_h::Z_UNKNOWN;
    state.pending = 0 as crate::zutil_h::ulg;
    state.pending_out = state.pending_buf;
    state.pending_out_offset = 0 as crate::zutil_h::ulg;
    let reset = deflate_reset_keep_config(state.wrap);
    state.wrap = reset.wrap;
    state.status = reset.status;
    strm.adler = reset.adler;
    state.last_flush = -2 as ::core::ffi::c_int;
    crate::src::trees::tr_init(state);
}

#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    deflate_reset_keep_state(&mut *strm, &mut *s);
    return crate::zlib_h::Z_OK;
}
fn lm_init(state: &mut crate::src::deflate::deflate_state, head: &mut [crate::src::deflate::Posf]) {
    state.window_size = (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
        .wrapping_mul(state.w_size as crate::zutil_h::ulg);
    deflate_clear_hash_head(head);
    deflate_lm_init_reset_fields(state);
}

fn deflate_clear_hash_head(head: &mut [crate::src::deflate::Posf]) {
    if let Some((last, prefix)) = head.split_last_mut() {
        *last = NIL as crate::src::deflate::Posf;
        prefix.fill(0 as crate::src::deflate::Posf);
    }
}

fn copy_deflate_dictionary(
    dictionary: &mut [crate::stdlib::Bytef],
    window: &[crate::stdlib::Bytef],
) {
    dictionary.copy_from_slice(window);
}

fn deflate_lm_init_reset_fields(state: &mut crate::src::deflate::deflate_state) {
    state.slid = 0 as ::core::ffi::c_int;
    let level = state.level;
    deflate_params_apply_config(state, level);
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
#[export_name = "deflateReset"]

pub unsafe extern "C" fn deflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    deflate_reset_keep_state(&mut *strm, state);
    let head_ptr = state.head;
    let hash_size = state.hash_size as usize;
    let head = ::core::slice::from_raw_parts_mut(head_ptr, hash_size);
    lm_init(state, head);
    return crate::zlib_h::Z_OK;
}
fn deflate_set_header_allowed(state: &crate::src::deflate::deflate_state) -> bool {
    state.wrap == 2 as ::core::ffi::c_int
}
#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    if !deflate_set_header_allowed(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.gzhead = head;
    return crate::zlib_h::Z_OK;
}
pub fn deflatePending(
    state: &crate::src::deflate::deflate_state,
) -> (crate::zutil_h::ulg, ::core::ffi::c_int) {
    return (state.pending, state.bi_valid);
}

fn deflate_pending_c_uint(
    pending: crate::zutil_h::ulg,
) -> Result<::core::ffi::c_uint, ::core::ffi::c_int> {
    let narrowed = pending as ::core::ffi::c_uint;
    if narrowed as crate::zutil_h::ulg != pending {
        Err(crate::zlib_h::Z_BUF_ERROR)
    } else {
        Ok(narrowed)
    }
}
#[export_name = "deflatePending"]

pub unsafe extern "C" fn deflatePending_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*(*strm).state;
    let (pending_value, bits_value) = deflatePending(state);
    if !bits.is_null() {
        *bits = bits_value;
    }
    if !pending.is_null() {
        match deflate_pending_c_uint(pending_value) {
            Ok(pending_value) => *pending = pending_value,
            Err(err) => {
                *pending = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
                return err;
            }
        }
    }
    return crate::zlib_h::Z_OK;
}
pub fn deflateUsed(state: &crate::src::deflate::deflate_state) -> ::core::ffi::c_int {
    return state.bi_used;
}
#[export_name = "deflateUsed"]

pub unsafe extern "C" fn deflateUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*(*strm).state;
    if !bits.is_null() {
        *bits = deflateUsed(state);
    }
    return crate::zlib_h::Z_OK;
}

struct DeflatePrimeStep {
    bytes: [crate::stdlib::Byte; 2],
    len: usize,
    bi_buf: crate::zutil_h::ush,
    bi_valid: ::core::ffi::c_int,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
}

fn deflate_prime_step(
    mut bi_buf: crate::zutil_h::ush,
    mut bi_valid: ::core::ffi::c_int,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> DeflatePrimeStep {
    let mut put = crate::src::deflate::Buf_size - bi_valid;
    if put > bits {
        put = bits;
    }
    bi_buf = (bi_buf as ::core::ffi::c_int
        | ((value & ((1 as ::core::ffi::c_int) << put) - 1 as ::core::ffi::c_int) << bi_valid)
            as crate::zutil_h::ush as ::core::ffi::c_int) as crate::zutil_h::ush;
    bi_valid += put;

    let mut bytes = [0; 2];
    let mut len = 0;
    if bi_valid == 16 as ::core::ffi::c_int {
        bytes[0] =
            (bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as crate::stdlib::Byte;
        bytes[1] = (bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte;
        len = 2;
        bi_buf = 0 as crate::zutil_h::ush;
        bi_valid = 0 as ::core::ffi::c_int;
    } else if bi_valid >= 8 as ::core::ffi::c_int {
        bytes[0] = bi_buf as crate::stdlib::Byte;
        len = 1;
        bi_buf = (bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::ush;
        bi_valid -= 8 as ::core::ffi::c_int;
    }

    DeflatePrimeStep {
        bytes,
        len,
        bi_buf,
        bi_valid,
        bits: bits - put,
        value: value >> put,
    }
}

fn append_deflate_pending_bytes(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    bytes: &[crate::stdlib::Byte],
) {
    let start = *pending as usize;
    let end = start + bytes.len();
    pending_buf[start..end].copy_from_slice(bytes);
    *pending = pending.wrapping_add(bytes.len() as crate::zutil_h::ulg);
}

#[export_name = "deflatePrime"]

pub unsafe extern "C" fn deflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || ((*s).lit_bufsize as crate::zutil_h::ulg)
            < (*s).pending_out_offset.wrapping_add(
                (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                    as crate::zutil_h::ulg,
            )
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    let state = &mut *s;
    let pending_buf =
        ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize);
    loop {
        let step = deflate_prime_step(state.bi_buf, state.bi_valid, bits, value);
        append_deflate_pending_bytes(pending_buf, &mut state.pending, &step.bytes[..step.len]);
        state.bi_buf = step.bi_buf;
        state.bi_valid = step.bi_valid;
        value = step.value;
        bits = step.bits;
        if !(bits != 0) {
            break;
        }
    }
    return crate::zlib_h::Z_OK;
}
fn deflate_params_level(level: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        6 as ::core::ffi::c_int
    } else {
        level
    }
}

fn deflate_params_valid(level: ::core::ffi::c_int, strategy: ::core::ffi::c_int) -> bool {
    (0 as ::core::ffi::c_int..=9 as ::core::ffi::c_int).contains(&level)
        && (0 as ::core::ffi::c_int..=crate::zlib_h::Z_FIXED).contains(&strategy)
}

fn deflate_params_needs_flush(
    state: &crate::src::deflate::deflate_state,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> bool {
    let func = configuration_table[state.level as usize].func;
    (strategy != state.strategy || func != configuration_table[level as usize].func)
        && state.last_flush != -2 as ::core::ffi::c_int
}

fn deflate_params_drained(
    state: &crate::src::deflate::deflate_state,
    avail_in: crate::stdlib::uInt,
) -> bool {
    avail_in == 0
        && state.strstart as ::core::ffi::c_long - state.block_start
            + state.lookahead as ::core::ffi::c_long
            == 0
}

fn deflate_params_apply_config(
    state: &mut crate::src::deflate::deflate_state,
    level: ::core::ffi::c_int,
) {
    state.level = level;
    state.max_lazy_match = configuration_table[level as usize].max_lazy as crate::stdlib::uInt;
    state.good_match = configuration_table[level as usize].good_length as crate::stdlib::uInt;
    state.nice_match = configuration_table[level as usize].nice_length as ::core::ffi::c_int;
    state.max_chain_length = configuration_table[level as usize].max_chain as crate::stdlib::uInt;
}

#[export_name = "deflateParams"]

pub unsafe extern "C" fn deflateParams_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    level = deflate_params_level(level);
    if !deflate_params_valid(level, strategy) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if deflate_params_needs_flush(&*s, level, strategy) {
        let mut err: ::core::ffi::c_int = deflate_ffi(strm, crate::zlib_h::Z_BLOCK);
        if err == crate::zlib_h::Z_STREAM_ERROR {
            return err;
        }
        if !deflate_params_drained(&*s, (*strm).avail_in) {
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    if (*s).level != level {
        if (*s).level == 0 as ::core::ffi::c_int && (*s).matches != 0 as crate::stdlib::uInt {
            if (*s).matches == 1 as crate::stdlib::uInt {
                slide_hash(&mut *s);
            } else {
                let head = ::core::slice::from_raw_parts_mut((*s).head, (*s).hash_size as usize);
                deflate_clear_hash_head(head);
                (*s).slid = 0 as ::core::ffi::c_int;
            }
            (*s).matches = 0 as crate::stdlib::uInt;
        }
        deflate_params_apply_config(&mut *s, level);
    }
    (*s).strategy = strategy;
    return crate::zlib_h::Z_OK;
}
pub fn deflateTune(
    s: &mut crate::src::deflate::deflate_state,
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    s.good_match = good_length as crate::stdlib::uInt;
    s.max_lazy_match = max_lazy as crate::stdlib::uInt;
    s.nice_match = nice_length;
    s.max_chain_length = max_chain as crate::stdlib::uInt;
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
    if deflate_state_check_raw!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    deflateTune(state, good_length, max_lazy, nice_length, max_chain)
}

fn deflate_bound_overflow() -> crate::stdlib::z_size_t {
    -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
}

fn deflate_bound_add(
    bound: crate::stdlib::z_size_t,
    extra: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if bound.wrapping_add(extra) < bound {
        deflate_bound_overflow()
    } else {
        bound.wrapping_add(extra)
    }
}

fn deflate_bound_fixed_len(source_len: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    let fixed_len = source_len
        .wrapping_add(source_len >> 3 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 8 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 9 as ::core::ffi::c_int)
        .wrapping_add(4 as crate::stdlib::z_size_t);
    if fixed_len < source_len {
        deflate_bound_overflow()
    } else {
        fixed_len
    }
}

fn deflate_bound_store_len(source_len: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    let store_len = source_len
        .wrapping_add(source_len >> 5 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 7 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 11 as ::core::ffi::c_int)
        .wrapping_add(7 as crate::stdlib::z_size_t);
    if store_len < source_len {
        deflate_bound_overflow()
    } else {
        store_len
    }
}

fn deflate_bound_for_state(
    source_len: crate::stdlib::z_size_t,
    fixed_len: crate::stdlib::z_size_t,
    store_len: crate::stdlib::z_size_t,
    wrap_len: crate::stdlib::z_size_t,
    w_bits: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
) -> crate::stdlib::z_size_t {
    if w_bits != 15 as crate::stdlib::uInt
        || hash_bits != (8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        let bound = if w_bits <= hash_bits && level != 0 {
            fixed_len
        } else {
            store_len
        };
        return deflate_bound_add(bound, wrap_len);
    }
    let bound = source_len
        .wrapping_add(source_len >> 12 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 14 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as crate::stdlib::z_size_t)
        .wrapping_sub(6 as crate::stdlib::z_size_t)
        .wrapping_add(wrap_len);
    if bound < source_len {
        deflate_bound_overflow()
    } else {
        bound
    }
}

fn deflate_bound_cstring_len(value: &::core::ffi::CStr) -> crate::stdlib::z_size_t {
    value.to_bytes_with_nul().len() as crate::stdlib::z_size_t
}

#[derive(Copy, Clone)]
struct DeflateBoundState {
    wrap: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    w_bits: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
}

#[derive(Copy, Clone)]
struct DeflateBoundGzipHeader {
    extra_len: Option<crate::stdlib::uInt>,
    name_len: crate::stdlib::z_size_t,
    comment_len: crate::stdlib::z_size_t,
    has_hcrc: bool,
}

fn deflate_bound_gzip_header_len(
    extra_len: Option<crate::stdlib::uInt>,
    name_len: crate::stdlib::z_size_t,
    comment_len: crate::stdlib::z_size_t,
    has_hcrc: bool,
) -> crate::stdlib::z_size_t {
    let mut wraplen = 18 as crate::stdlib::z_size_t;
    if let Some(extra_len) = extra_len {
        wraplen = wraplen.wrapping_add(
            (2 as crate::stdlib::uInt).wrapping_add(extra_len) as crate::stdlib::z_size_t
        );
    }
    wraplen = wraplen.wrapping_add(name_len);
    wraplen = wraplen.wrapping_add(comment_len);
    if has_hcrc {
        wraplen = wraplen.wrapping_add(2 as crate::stdlib::z_size_t);
    }
    wraplen
}

fn deflate_bound_wrap_len(
    state: DeflateBoundState,
    gzip_header: Option<DeflateBoundGzipHeader>,
) -> crate::stdlib::z_size_t {
    match if state.wrap < 0 as ::core::ffi::c_int {
        -state.wrap
    } else {
        state.wrap
    } {
        0 => 0 as crate::stdlib::z_size_t,
        1 => {
            (6 as ::core::ffi::c_int
                + (if state.strstart != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as crate::stdlib::z_size_t
        }
        2 => {
            if let Some(header) = gzip_header {
                deflate_bound_gzip_header_len(
                    header.extra_len,
                    header.name_len,
                    header.comment_len,
                    header.has_hcrc,
                )
            } else {
                18 as crate::stdlib::z_size_t
            }
        }
        _ => 18 as crate::stdlib::z_size_t,
    }
}

fn deflate_bound_uses_gzip_header(wrap: ::core::ffi::c_int) -> bool {
    let normalized_wrap = if wrap < 0 as ::core::ffi::c_int {
        -wrap
    } else {
        wrap
    };
    normalized_wrap == 2 as ::core::ffi::c_int
}

fn deflate_bound_z_impl(
    source_len: crate::stdlib::z_size_t,
    state: Option<DeflateBoundState>,
    gzip_header: Option<DeflateBoundGzipHeader>,
) -> crate::stdlib::z_size_t {
    let fixed_len = deflate_bound_fixed_len(source_len);
    let store_len = deflate_bound_store_len(source_len);
    let Some(state) = state else {
        let bound = if fixed_len > store_len {
            fixed_len
        } else {
            store_len
        };
        return deflate_bound_add(bound, 18 as crate::stdlib::z_size_t);
    };
    let wrap_len = deflate_bound_wrap_len(state, gzip_header);
    deflate_bound_for_state(
        source_len,
        fixed_len,
        store_len,
        wrap_len,
        state.w_bits,
        state.hash_bits,
        state.level,
    )
}

fn deflate_zlib_header(
    w_bits: crate::stdlib::uInt,
    strategy: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    has_dictionary: bool,
) -> crate::stdlib::uInt {
    let mut header: crate::stdlib::uInt = (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt)
        .wrapping_add(w_bits.wrapping_sub(8 as crate::stdlib::uInt) << 4 as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int;
    let level_flags: crate::stdlib::uInt =
        if strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || level < 2 as ::core::ffi::c_int {
            0 as crate::stdlib::uInt
        } else if level < 6 as ::core::ffi::c_int {
            1 as crate::stdlib::uInt
        } else if level == 6 as ::core::ffi::c_int {
            2 as crate::stdlib::uInt
        } else {
            3 as crate::stdlib::uInt
        };
    header |= level_flags << 6 as ::core::ffi::c_int;
    if has_dictionary {
        header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
    }
    header.wrapping_add(
        (31 as crate::stdlib::uInt).wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
    )
}

fn deflate_gzip_xflags(
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> crate::stdlib::Bytef {
    if level == 9 as ::core::ffi::c_int {
        2 as crate::stdlib::Bytef
    } else if strategy >= 2 as ::core::ffi::c_int || level < 2 as ::core::ffi::c_int {
        4 as crate::stdlib::Bytef
    } else {
        0 as crate::stdlib::Bytef
    }
}

fn deflate_gzip_flags(
    text: ::core::ffi::c_int,
    hcrc: ::core::ffi::c_int,
    has_extra: bool,
    has_name: bool,
    has_comment: bool,
) -> crate::stdlib::Bytef {
    ((if text != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) + (if hcrc != 0 {
        2 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) + (if has_extra {
        4 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) + (if has_name {
        8 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) + (if has_comment {
        16 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    })) as crate::stdlib::Bytef
}

fn deflate_gzip_time_bytes(time: crate::stdlib::uLong) -> [crate::stdlib::Bytef; 4] {
    [
        (time & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (time >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (time >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (time >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
    ]
}

fn deflate_gzip_u16_le_bytes(value: crate::stdlib::uLong) -> [crate::stdlib::Bytef; 2] {
    [
        (value & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (value >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
    ]
}

fn deflate_flush_rank(flush: ::core::ffi::c_int) -> ::core::ffi::c_int {
    flush * 2 as ::core::ffi::c_int
        - if flush > crate::zlib_h::Z_FINISH {
            9 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }
}

fn deflate_repeated_flush_would_block(
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    old_flush: ::core::ffi::c_int,
) -> bool {
    avail_in == 0 as crate::stdlib::uInt
        && deflate_flush_rank(flush) <= deflate_flush_rank(old_flush)
        && flush != crate::zlib_h::Z_FINISH
}

#[export_name = "deflateBound_z"]
pub unsafe extern "C" fn deflateBound_z_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    if deflate_state_check_raw!(strm) != 0 {
        return deflate_bound_z_impl(sourceLen, None, None);
    }
    let s = &*((*strm).state as *mut crate::src::deflate::deflate_state);
    let state = DeflateBoundState {
        wrap: s.wrap,
        strstart: s.strstart,
        w_bits: s.w_bits,
        hash_bits: s.hash_bits,
        level: s.level,
    };
    let gzip_header = if deflate_bound_uses_gzip_header(state.wrap) && !s.gzhead.is_null() {
        let head = &*s.gzhead;
        let extra_len = if head.extra.is_null() {
            None
        } else {
            Some(head.extra_len)
        };
        let name_len = if head.name.is_null() {
            0 as crate::stdlib::z_size_t
        } else {
            let name = ::core::ffi::CStr::from_ptr(head.name as *const ::core::ffi::c_char);
            deflate_bound_cstring_len(name)
        };
        let comment_len = if head.comment.is_null() {
            0 as crate::stdlib::z_size_t
        } else {
            let comment = ::core::ffi::CStr::from_ptr(head.comment as *const ::core::ffi::c_char);
            deflate_bound_cstring_len(comment)
        };
        Some(DeflateBoundGzipHeader {
            extra_len,
            name_len,
            comment_len,
            has_hcrc: head.hcrc != 0,
        })
    } else {
        None
    };
    return deflate_bound_z_impl(sourceLen, Some(state), gzip_header);
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let mut bound: crate::stdlib::z_size_t = if deflate_state_check_raw!(strm) != 0 {
        deflate_bound_z_impl(sourceLen as crate::stdlib::z_size_t, None, None)
    } else {
        let s = &*((*strm).state as *mut crate::src::deflate::deflate_state);
        let state = DeflateBoundState {
            wrap: s.wrap,
            strstart: s.strstart,
            w_bits: s.w_bits,
            hash_bits: s.hash_bits,
            level: s.level,
        };
        let gzip_header = if deflate_bound_uses_gzip_header(state.wrap) && !s.gzhead.is_null() {
            let head = &*s.gzhead;
            let extra_len = if head.extra.is_null() {
                None
            } else {
                Some(head.extra_len)
            };
            let name_len = if head.name.is_null() {
                0 as crate::stdlib::z_size_t
            } else {
                let name = ::core::ffi::CStr::from_ptr(head.name as *const ::core::ffi::c_char);
                deflate_bound_cstring_len(name)
            };
            let comment_len = if head.comment.is_null() {
                0 as crate::stdlib::z_size_t
            } else {
                let comment =
                    ::core::ffi::CStr::from_ptr(head.comment as *const ::core::ffi::c_char);
                deflate_bound_cstring_len(comment)
            };
            Some(DeflateBoundGzipHeader {
                extra_len,
                name_len,
                comment_len,
                has_hcrc: head.hcrc != 0,
            })
        } else {
            None
        };
        deflate_bound_z_impl(
            sourceLen as crate::stdlib::z_size_t,
            Some(state),
            gzip_header,
        )
    };
    return if bound != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        bound as crate::stdlib::uLong
    };
}
fn put_short_msb(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    b: crate::stdlib::uInt,
) {
    let offset = *pending as usize;
    pending_buf[offset] = (b >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte;
    pending_buf[offset + 1] = (b & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte;
    *pending = (*pending).wrapping_add(2 as crate::zutil_h::ulg);
}

fn gzip_trailer_bytes(
    crc: crate::stdlib::uLong,
    total_in: crate::stdlib::uLong,
) -> [crate::stdlib::Bytef; 8] {
    [
        (crc & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (crc >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (crc >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (crc >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (total_in & 0xff as crate::stdlib::uLong) as crate::stdlib::Bytef,
        (total_in >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
            as crate::stdlib::Bytef,
        (total_in >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
            as crate::stdlib::Bytef,
        (total_in >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
            as crate::stdlib::Bytef,
    ]
}

fn deflate_pending_copy_len(
    pending: crate::zutil_h::ulg,
    avail_out: crate::stdlib::uInt,
) -> Option<::core::ffi::c_uint> {
    let len = if pending > avail_out as crate::zutil_h::ulg {
        avail_out as ::core::ffi::c_uint
    } else {
        pending as ::core::ffi::c_uint
    };
    if len == 0 as ::core::ffi::c_uint {
        None
    } else {
        Some(len)
    }
}

unsafe fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    let mut len: ::core::ffi::c_uint = 0;
    let strm_ref = &mut *strm;
    let state = &mut *(strm_ref.state as *mut crate::src::deflate::deflate_state);
    crate::src::trees::_tr_flush_bits_ffi(state as *mut crate::src::deflate::internal_state);
    let Some(copy_len) = deflate_pending_copy_len(state.pending, strm_ref.avail_out) else {
        return;
    };
    len = copy_len;
    crate::stdlib::memcpy(
        strm_ref.next_out as *mut ::core::ffi::c_void,
        state.pending_out as *const ::core::ffi::c_void,
        len as crate::__stddef_size_t_h::size_t,
    );
    strm_ref.next_out = strm_ref.next_out.wrapping_add(len as usize);
    state.pending_out = state.pending_out.wrapping_add(len as usize);
    state.pending_out_offset = state
        .pending_out_offset
        .wrapping_add(len as crate::zutil_h::ulg);
    strm_ref.total_out = strm_ref.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm_ref.avail_out = strm_ref.avail_out.wrapping_sub(len);
    state.pending = state.pending.wrapping_sub(len as crate::zutil_h::ulg);
    if state.pending == 0 as crate::zutil_h::ulg {
        state.pending_out = state.pending_buf;
        state.pending_out_offset = 0 as crate::zutil_h::ulg;
    }
}
#[export_name = "deflate"]
pub unsafe extern "C" fn deflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut old_flush: ::core::ffi::c_int = 0;
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflate_state_check_raw!(strm) != 0
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
        (*strm).msg = crate::src::zutil::zError(crate::zlib_h::Z_STREAM_ERROR).as_ptr()
            as *mut ::core::ffi::c_char;
        return -2 as ::core::ffi::c_int;
    }
    if (*strm).avail_out == 0 as crate::stdlib::uInt {
        (*strm).msg = crate::src::zutil::zError(crate::zlib_h::Z_BUF_ERROR).as_ptr()
            as *mut ::core::ffi::c_char;
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
    } else if deflate_repeated_flush_would_block((*strm).avail_in, flush, old_flush) {
        (*strm).msg = crate::src::zutil::zError(crate::zlib_h::Z_BUF_ERROR).as_ptr()
            as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    if (*s).status == crate::src::deflate::FINISH_STATE
        && (*strm).avail_in != 0 as crate::stdlib::uInt
    {
        (*strm).msg = crate::src::zutil::zError(crate::zlib_h::Z_BUF_ERROR).as_ptr()
            as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    if (*s).status == crate::src::deflate::INIT_STATE && (*s).wrap == 0 as ::core::ffi::c_int {
        (*s).status = crate::src::deflate::BUSY_STATE;
    }
    if (*s).status == crate::src::deflate::INIT_STATE {
        let header = deflate_zlib_header(
            (*s).w_bits,
            (*s).strategy,
            (*s).level,
            (*s).strstart != 0 as crate::stdlib::uInt,
        );
        {
            let pending_buf =
                ::core::slice::from_raw_parts_mut((*s).pending_buf, (*s).pending_buf_size as usize);
            put_short_msb(pending_buf, &mut (*s).pending, header);
        }
        if (*s).strstart != 0 as crate::stdlib::uInt {
            {
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    (*s).pending_buf,
                    (*s).pending_buf_size as usize,
                );
                put_short_msb(
                    pending_buf,
                    &mut (*s).pending,
                    ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
                );
            }
            {
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    (*s).pending_buf,
                    (*s).pending_buf_size as usize,
                );
                put_short_msb(
                    pending_buf,
                    &mut (*s).pending,
                    ((*strm).adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
                );
            }
        }
        (*strm).adler = crate::src::adler32::adler32_initial();
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if (*s).status == crate::src::deflate::GZIP_STATE {
        (*strm).adler = crate::src::crc32::crc32_initial();
        let c2rust_fresh0 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh0 as isize) =
            31 as ::core::ffi::c_int as crate::stdlib::Bytef;
        let c2rust_fresh1 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh1 as isize) =
            139 as ::core::ffi::c_int as crate::stdlib::Bytef;
        let c2rust_fresh2 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh2 as isize) =
            8 as ::core::ffi::c_int as crate::stdlib::Bytef;
        if (*s).gzhead.is_null() {
            let c2rust_fresh3 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh3 as isize) =
                0 as ::core::ffi::c_int as crate::stdlib::Bytef;
            let c2rust_fresh4 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh4 as isize) =
                0 as ::core::ffi::c_int as crate::stdlib::Bytef;
            let c2rust_fresh5 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh5 as isize) =
                0 as ::core::ffi::c_int as crate::stdlib::Bytef;
            let c2rust_fresh6 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh6 as isize) =
                0 as ::core::ffi::c_int as crate::stdlib::Bytef;
            let c2rust_fresh7 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh7 as isize) =
                0 as ::core::ffi::c_int as crate::stdlib::Bytef;
            let c2rust_fresh8 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh8 as isize) =
                deflate_gzip_xflags((*s).level, (*s).strategy);
            let c2rust_fresh9 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh9 as isize) =
                3 as ::core::ffi::c_int as crate::stdlib::Bytef;
            (*s).status = crate::src::deflate::BUSY_STATE;
            flush_pending(strm);
            if (*s).pending != 0 as crate::zutil_h::ulg {
                (*s).last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        } else {
            let c2rust_fresh10 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh10 as isize) = deflate_gzip_flags(
                (*(*s).gzhead).text,
                (*(*s).gzhead).hcrc,
                !(*(*s).gzhead).extra.is_null(),
                !(*(*s).gzhead).name.is_null(),
                !(*(*s).gzhead).comment.is_null(),
            );
            for byte in deflate_gzip_time_bytes((*(*s).gzhead).time) {
                let pending = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(pending as isize) = byte;
            }
            let c2rust_fresh15 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh15 as isize) =
                deflate_gzip_xflags((*s).level, (*s).strategy);
            let c2rust_fresh16 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh16 as isize) =
                ((*(*s).gzhead).os & 0xff as ::core::ffi::c_int) as crate::stdlib::Bytef;
            if !(*(*s).gzhead).extra.is_null() {
                for byte in
                    deflate_gzip_u16_le_bytes((*(*s).gzhead).extra_len as crate::stdlib::uLong)
                {
                    let pending = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(pending as isize) = byte;
                }
            }
            if (*(*s).gzhead).hcrc != 0 {
                let hcrc_bytes =
                    ::core::slice::from_raw_parts((*s).pending_buf, (*s).pending as usize);
                (*strm).adler = crate::src::crc32::crc32_update((*strm).adler, hcrc_bytes);
            }
            (*s).gzindex = 0 as crate::zutil_h::ulg;
            (*s).status = crate::src::deflate::EXTRA_STATE;
        }
    }
    if (*s).status == crate::src::deflate::EXTRA_STATE {
        if !(*(*s).gzhead).extra.is_null() {
            let mut beg: crate::zutil_h::ulg = (*s).pending;
            let mut left: crate::zutil_h::ulg =
                (((*(*s).gzhead).extra_len & 0xffff as crate::stdlib::uInt) as crate::zutil_h::ulg)
                    .wrapping_sub((*s).gzindex);
            while (*s).pending.wrapping_add(left) > (*s).pending_buf_size {
                let mut copy: crate::zutil_h::ulg =
                    (*s).pending_buf_size.wrapping_sub((*s).pending);
                crate::stdlib::memcpy(
                    (*s).pending_buf.offset((*s).pending as isize) as *mut ::core::ffi::c_void,
                    (*(*s).gzhead).extra.offset((*s).gzindex as isize)
                        as *const ::core::ffi::c_void,
                    copy as crate::__stddef_size_t_h::size_t,
                );
                (*s).pending = (*s).pending_buf_size;
                if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg {
                    let hcrc_bytes = ::core::slice::from_raw_parts(
                        (*s).pending_buf.offset(beg as isize),
                        ((*s).pending as crate::stdlib::z_size_t)
                            .wrapping_sub(beg as crate::stdlib::z_size_t)
                            as usize,
                    );
                    (*strm).adler = crate::src::crc32::crc32_update((*strm).adler, hcrc_bytes);
                }
                (*s).gzindex = (*s).gzindex.wrapping_add(copy);
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
                beg = 0 as crate::zutil_h::ulg;
                left = left.wrapping_sub(copy);
            }
            crate::stdlib::memcpy(
                (*s).pending_buf.offset((*s).pending as isize) as *mut ::core::ffi::c_void,
                (*(*s).gzhead).extra.offset((*s).gzindex as isize) as *const ::core::ffi::c_void,
                left as crate::__stddef_size_t_h::size_t,
            );
            (*s).pending = (*s).pending.wrapping_add(left);
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg {
                let hcrc_bytes = ::core::slice::from_raw_parts(
                    (*s).pending_buf.offset(beg as isize),
                    ((*s).pending as crate::stdlib::z_size_t)
                        .wrapping_sub(beg as crate::stdlib::z_size_t) as usize,
                );
                (*strm).adler = crate::src::crc32::crc32_update((*strm).adler, hcrc_bytes);
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
                        let hcrc_bytes = ::core::slice::from_raw_parts(
                            (*s).pending_buf.offset(beg_0 as isize),
                            ((*s).pending as crate::stdlib::z_size_t)
                                .wrapping_sub(beg_0 as crate::stdlib::z_size_t)
                                as usize,
                        );
                        (*strm).adler = crate::src::crc32::crc32_update((*strm).adler, hcrc_bytes);
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
                if !(val != 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_0 {
                let hcrc_bytes = ::core::slice::from_raw_parts(
                    (*s).pending_buf.offset(beg_0 as isize),
                    ((*s).pending as crate::stdlib::z_size_t)
                        .wrapping_sub(beg_0 as crate::stdlib::z_size_t)
                        as usize,
                );
                (*strm).adler = crate::src::crc32::crc32_update((*strm).adler, hcrc_bytes);
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
                        let hcrc_bytes = ::core::slice::from_raw_parts(
                            (*s).pending_buf.offset(beg_1 as isize),
                            ((*s).pending as crate::stdlib::z_size_t)
                                .wrapping_sub(beg_1 as crate::stdlib::z_size_t)
                                as usize,
                        );
                        (*strm).adler = crate::src::crc32::crc32_update((*strm).adler, hcrc_bytes);
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
                if !(val_0 != 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_1 {
                let hcrc_bytes = ::core::slice::from_raw_parts(
                    (*s).pending_buf.offset(beg_1 as isize),
                    ((*s).pending as crate::stdlib::z_size_t)
                        .wrapping_sub(beg_1 as crate::stdlib::z_size_t)
                        as usize,
                );
                (*strm).adler = crate::src::crc32::crc32_update((*strm).adler, hcrc_bytes);
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
            for byte in deflate_gzip_u16_le_bytes((*strm).adler) {
                let pending = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(pending as isize) = byte;
            }
            (*strm).adler = crate::src::crc32::crc32_initial();
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
            (match configuration_table[(*s).level as usize].func {
                DeflateFunc::Stored => deflate_stored(s, flush),
                DeflateFunc::Fast => deflate_fast(s, flush),
                DeflateFunc::Slow => deflate_slow(s, flush),
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
                crate::src::trees::tr_align_impl(state, pending_buf);
            } else if flush != crate::zlib_h::Z_BLOCK {
                let state = &mut *s;
                let pending_buf = ::core::slice::from_raw_parts_mut(
                    state.pending_buf,
                    state.pending_buf_size as usize,
                );
                crate::src::trees::tr_stored_block_impl(
                    state,
                    pending_buf,
                    &[],
                    0 as crate::zutil_h::ulg,
                    0 as ::core::ffi::c_int,
                );
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    *(*s)
                        .head
                        .offset((*s).hash_size.wrapping_sub(1 as crate::stdlib::uInt) as isize) =
                        NIL as crate::src::deflate::Posf;
                    crate::stdlib::memset(
                        (*s).head as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ((*s).hash_size.wrapping_sub(1 as crate::stdlib::uInt)
                            as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Posf>()
                                as crate::__stddef_size_t_h::size_t),
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
        for byte in gzip_trailer_bytes((*strm).adler, (*strm).total_in) {
            let pending = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(pending as isize) = byte;
        }
    } else {
        {
            let pending_buf =
                ::core::slice::from_raw_parts_mut((*s).pending_buf, (*s).pending_buf_size as usize);
            put_short_msb(
                pending_buf,
                &mut (*s).pending,
                ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
            );
        }
        {
            let pending_buf =
                ::core::slice::from_raw_parts_mut((*s).pending_buf, (*s).pending_buf_size as usize);
            put_short_msb(
                pending_buf,
                &mut (*s).pending,
                ((*strm).adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
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
#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    if deflate_state_check_raw!(strm) != 0 {
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
#[export_name = "deflateCopy"]

pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut ds: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut ss: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflate_state_check_raw!(source) != 0 || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    ss = (*source).state as *mut crate::src::deflate::deflate_state;
    crate::stdlib::memcpy(
        dest as *mut ::core::ffi::c_void,
        source as *const ::core::ffi::c_void,
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as crate::__stddef_size_t_h::size_t,
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
        ::core::mem::size_of::<crate::src::deflate::deflate_state>()
            as crate::__stddef_size_t_h::size_t,
    );
    (*dest).state = ds as *mut crate::src::deflate::internal_state;
    crate::stdlib::memcpy(
        ds as *mut ::core::ffi::c_void,
        ss as *const ::core::ffi::c_void,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>()
            as crate::__stddef_size_t_h::size_t,
    );
    (*ds).strm = dest;
    (*ds).window = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>() as usize)
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
        deflateEnd_ffi(dest);
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
            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Pos>()
                as crate::__stddef_size_t_h::size_t),
    );
    crate::stdlib::memcpy(
        (*ds).head as *mut ::core::ffi::c_void,
        (*ss).head as *const ::core::ffi::c_void,
        ((*ds).hash_size as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Pos>()
                as crate::__stddef_size_t_h::size_t),
    );
    (*ds).pending_out_offset = (*ss).pending_out_offset;
    (*ds).pending_out = (*ds).pending_buf.offset((*ds).pending_out_offset as isize);
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
unsafe fn longest_match(
    s: *mut crate::src::deflate::deflate_state,
    mut cur_match: crate::src::deflate::IPos,
) -> crate::stdlib::uInt {
    let s = &mut *s;
    let mut chain_length: ::core::ffi::c_uint = s.max_chain_length as ::core::ffi::c_uint;
    let mut scan: *mut crate::stdlib::Bytef = s.window.offset(s.strstart as isize);
    let mut match_0: *mut crate::stdlib::Bytef = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    let mut len: ::core::ffi::c_int = 0;
    let mut best_len: ::core::ffi::c_int = s.prev_length as ::core::ffi::c_int;
    let mut nice_match: ::core::ffi::c_int = s.nice_match;
    let mut limit: crate::src::deflate::IPos = if s.strstart
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
    let mut prev: *mut crate::src::deflate::Posf = s.prev;
    let mut wmask: crate::stdlib::uInt = s.w_mask;
    let mut strend: *mut crate::stdlib::Bytef = s
        .window
        .offset(s.strstart as isize)
        .offset(crate::zutil_h::MAX_MATCH as isize);
    let mut scan_end1: crate::stdlib::Byte =
        *scan.offset((best_len - 1 as ::core::ffi::c_int) as isize) as crate::stdlib::Byte;
    let mut scan_end: crate::stdlib::Byte = *scan.offset(best_len as isize) as crate::stdlib::Byte;
    if s.prev_length >= s.good_match {
        chain_length >>= 2 as ::core::ffi::c_int;
    }
    if nice_match as crate::stdlib::uInt > s.lookahead {
        nice_match = s.lookahead as ::core::ffi::c_int;
    }
    loop {
        match_0 = s.window.offset(cur_match as isize);
        if !(*match_0.offset(best_len as isize) as ::core::ffi::c_int
            != scan_end as ::core::ffi::c_int
            || *match_0.offset((best_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != scan_end1 as ::core::ffi::c_int
            || *match_0 as ::core::ffi::c_int != *scan as ::core::ffi::c_int
            || {
                match_0 = match_0.offset(1);
                *match_0 as ::core::ffi::c_int
                    != *scan.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            })
        {
            scan = scan.offset(2 as ::core::ffi::c_int as isize);
            match_0 = match_0.offset(1);
            loop {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                if !(*scan as ::core::ffi::c_int == *match_0 as ::core::ffi::c_int
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as ::core::ffi::c_int == *match_0 as ::core::ffi::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as ::core::ffi::c_int == *match_0 as ::core::ffi::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as ::core::ffi::c_int == *match_0 as ::core::ffi::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as ::core::ffi::c_int == *match_0 as ::core::ffi::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as ::core::ffi::c_int == *match_0 as ::core::ffi::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as ::core::ffi::c_int == *match_0 as ::core::ffi::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as ::core::ffi::c_int == *match_0 as ::core::ffi::c_int
                    }
                    && scan < strend)
                {
                    break;
                }
            }
            len = crate::zutil_h::MAX_MATCH
                - strend.offset_from(scan) as ::core::ffi::c_long as ::core::ffi::c_int;
            scan = strend.offset(-(crate::zutil_h::MAX_MATCH as isize));
            if len > best_len {
                s.match_start = cur_match as crate::stdlib::uInt;
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = *scan.offset((best_len - 1 as ::core::ffi::c_int) as isize)
                    as crate::stdlib::Byte;
                scan_end = *scan.offset(best_len as isize) as crate::stdlib::Byte;
            }
        }
        cur_match = *prev.offset((cur_match as crate::stdlib::uInt & wmask) as isize)
            as crate::src::deflate::IPos;
        if !(cur_match > limit && {
            chain_length = chain_length.wrapping_sub(1);
            chain_length != 0 as ::core::ffi::c_uint
        }) {
            break;
        }
    }
    if best_len as crate::stdlib::uInt <= s.lookahead {
        return best_len as crate::stdlib::uInt;
    }
    return s.lookahead;
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

struct DeflateStoredEmitPlan {
    len: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    last: ::core::ffi::c_int,
}

fn deflate_stored_min_block(
    pending_buf_size: crate::zutil_h::ulg,
    w_size: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    let room = pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg);
    if room > w_size as crate::zutil_h::ulg {
        w_size as ::core::ffi::c_uint
    } else {
        room as ::core::ffi::c_uint
    }
}

fn deflate_stored_loop_emit_plan(
    bi_valid: ::core::ffi::c_int,
    avail_out: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    min_block: ::core::ffi::c_uint,
) -> Option<DeflateStoredEmitPlan> {
    let header = (bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
        >> 3 as ::core::ffi::c_int;
    if avail_out < header {
        return None;
    }
    let have = avail_out.wrapping_sub(header);
    let left = (strstart as ::core::ffi::c_long - block_start) as ::core::ffi::c_uint;
    let total = (left as crate::stdlib::uInt).wrapping_add(avail_in);
    let mut len = MAX_STORED as ::core::ffi::c_uint;
    if len as crate::zutil_h::ulg
        > (left as crate::zutil_h::ulg).wrapping_add(avail_in as crate::zutil_h::ulg)
    {
        len = total as ::core::ffi::c_uint;
    }
    if len > have {
        len = have;
    }
    if len < min_block
        && (len == 0 as ::core::ffi::c_uint && flush != crate::zlib_h::Z_FINISH
            || flush == crate::zlib_h::Z_NO_FLUSH
            || len != total)
    {
        return None;
    }
    let last = if flush == crate::zlib_h::Z_FINISH && len == total {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    Some(DeflateStoredEmitPlan { len, left, last })
}

fn deflate_stored_final_emit_plan(
    bi_valid: ::core::ffi::c_int,
    pending_buf_size: crate::zutil_h::ulg,
    w_size: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> Option<DeflateStoredEmitPlan> {
    let header = (bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
        >> 3 as ::core::ffi::c_int;
    let have = if pending_buf_size.wrapping_sub(header as crate::zutil_h::ulg)
        > MAX_STORED as crate::zutil_h::ulg
    {
        MAX_STORED as crate::zutil_h::ulg
    } else {
        pending_buf_size.wrapping_sub(header as crate::zutil_h::ulg)
    } as ::core::ffi::c_uint;
    let min_block = if have > w_size {
        w_size as ::core::ffi::c_uint
    } else {
        have
    };
    let left = (strstart as ::core::ffi::c_long - block_start) as ::core::ffi::c_uint;
    if left >= min_block
        || (left != 0 as ::core::ffi::c_uint || flush == crate::zlib_h::Z_FINISH)
            && flush != crate::zlib_h::Z_NO_FLUSH
            && avail_in == 0 as crate::stdlib::uInt
            && left <= have
    {
        let len = if left > have { have } else { left };
        let last = if flush == crate::zlib_h::Z_FINISH
            && avail_in == 0 as crate::stdlib::uInt
            && len == left
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        Some(DeflateStoredEmitPlan { len, left, last })
    } else {
        None
    }
}

fn deflate_stored_advance_insert(
    insert: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    copied: ::core::ffi::c_uint,
) -> crate::stdlib::uInt {
    insert.wrapping_add(if copied > w_size.wrapping_sub(insert) {
        (w_size as ::core::ffi::c_uint).wrapping_sub(insert as ::core::ffi::c_uint)
    } else {
        copied
    })
}

fn deflate_insert_limit(strstart: crate::stdlib::uInt) -> crate::stdlib::uInt {
    let limit = (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
    if strstart < limit {
        strstart
    } else {
        limit
    }
}

fn deflate_flush_blocked_state(final_flush: bool) -> block_state {
    if final_flush {
        finish_started
    } else {
        need_more
    }
}

unsafe fn deflate_stored(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let min_block = deflate_stored_min_block((*s).pending_buf_size, (*s).w_size);
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used: ::core::ffi::c_uint = (*(*s).strm).avail_in as ::core::ffi::c_uint;
    loop {
        let Some(plan) = deflate_stored_loop_emit_plan(
            (*s).bi_valid,
            (*(*s).strm).avail_out,
            (*s).strstart,
            (*s).block_start,
            (*(*s).strm).avail_in,
            flush,
            min_block,
        ) else {
            break;
        };
        len = plan.len;
        left = plan.left;
        last = plan.last;
        crate::src::trees::_tr_stored_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            ::core::ptr::null_mut::<crate::stdlib::charf>(),
            0 as crate::zutil_h::ulg,
            last,
        );
        let len_bytes = crate::src::trees::stored_block_len_bytes(len as crate::zutil_h::ulg);
        *(*s)
            .pending_buf
            .wrapping_add((*s).pending.wrapping_sub(4 as crate::zutil_h::ulg) as usize) =
            len_bytes[0];
        *(*s)
            .pending_buf
            .wrapping_add((*s).pending.wrapping_sub(3 as crate::zutil_h::ulg) as usize) =
            len_bytes[1];
        *(*s)
            .pending_buf
            .wrapping_add((*s).pending.wrapping_sub(2 as crate::zutil_h::ulg) as usize) =
            len_bytes[2];
        *(*s)
            .pending_buf
            .wrapping_add((*s).pending.wrapping_sub(1 as crate::zutil_h::ulg) as usize) =
            len_bytes[3];
        flush_pending((*s).strm);
        if left != 0 {
            if left > len {
                left = len;
            }
            crate::stdlib::memcpy(
                (*(*s).strm).next_out as *mut ::core::ffi::c_void,
                (*s).window.wrapping_offset((*s).block_start as isize)
                    as *const ::core::ffi::c_void,
                left as crate::__stddef_size_t_h::size_t,
            );
            (*(*s).strm).next_out = (*(*s).strm).next_out.wrapping_add(left as usize);
            (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(left);
            (*(*s).strm).total_out = (*(*s).strm)
                .total_out
                .wrapping_add(left as crate::stdlib::uLong);
            (*s).block_start += left as ::core::ffi::c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            let state = &mut *s;
            let strm = &mut *state.strm;
            let out = ::core::slice::from_raw_parts_mut(strm.next_out, len as usize);
            read_buf(strm, state.wrap, out);
            (*(*s).strm).next_out = (*(*s).strm).next_out.wrapping_add(len as usize);
            (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(len);
            (*(*s).strm).total_out = (*(*s).strm)
                .total_out
                .wrapping_add(len as crate::stdlib::uLong);
        }
        if !(last == 0 as ::core::ffi::c_int) {
            break;
        }
    }
    used = used.wrapping_sub((*(*s).strm).avail_in as ::core::ffi::c_uint);
    if used != 0 {
        if used >= (*s).w_size {
            (*s).matches = 2 as crate::stdlib::uInt;
            crate::stdlib::memcpy(
                (*s).window as *mut ::core::ffi::c_void,
                (*(*s).strm).next_in.offset(-((*s).w_size as isize)) as *const ::core::ffi::c_void,
                (*s).w_size as crate::__stddef_size_t_h::size_t,
            );
            (*s).strstart = (*s).w_size;
            (*s).insert = (*s).strstart;
        } else {
            if (*s)
                .window_size
                .wrapping_sub((*s).strstart as crate::zutil_h::ulg)
                <= used as crate::zutil_h::ulg
            {
                (*s).strstart = (*s).strstart.wrapping_sub((*s).w_size);
                crate::stdlib::memcpy(
                    (*s).window as *mut ::core::ffi::c_void,
                    (*s).window.offset((*s).w_size as isize) as *const ::core::ffi::c_void,
                    (*s).strstart as crate::__stddef_size_t_h::size_t,
                );
                if (*s).matches < 2 as crate::stdlib::uInt {
                    (*s).matches = (*s).matches.wrapping_add(1);
                }
                if (*s).insert > (*s).strstart {
                    (*s).insert = (*s).strstart;
                }
            }
            crate::stdlib::memcpy(
                (*s).window.offset((*s).strstart as isize) as *mut ::core::ffi::c_void,
                (*(*s).strm).next_in.offset(-(used as isize)) as *const ::core::ffi::c_void,
                used as crate::__stddef_size_t_h::size_t,
            );
            (*s).strstart = (*s).strstart.wrapping_add(used);
            (*s).insert = deflate_stored_advance_insert((*s).insert, (*s).w_size, used);
        }
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
    }
    if (*s).high_water < (*s).strstart as crate::zutil_h::ulg {
        (*s).high_water = (*s).strstart as crate::zutil_h::ulg;
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
    have = (*s)
        .window_size
        .wrapping_sub((*s).strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if (*(*s).strm).avail_in > have && (*s).block_start >= (*s).w_size as ::core::ffi::c_long {
        (*s).block_start -= (*s).w_size as ::core::ffi::c_long;
        (*s).strstart = (*s).strstart.wrapping_sub((*s).w_size);
        crate::stdlib::memcpy(
            (*s).window as *mut ::core::ffi::c_void,
            (*s).window.offset((*s).w_size as isize) as *const ::core::ffi::c_void,
            (*s).strstart as crate::__stddef_size_t_h::size_t,
        );
        if (*s).matches < 2 as crate::stdlib::uInt {
            (*s).matches = (*s).matches.wrapping_add(1);
        }
        have = have.wrapping_add((*s).w_size as ::core::ffi::c_uint);
        if (*s).insert > (*s).strstart {
            (*s).insert = (*s).strstart;
        }
    }
    if have > (*(*s).strm).avail_in {
        have = (*(*s).strm).avail_in as ::core::ffi::c_uint;
    }
    if have != 0 {
        let state = &mut *s;
        let strm = &mut *state.strm;
        let out = ::core::slice::from_raw_parts_mut(
            state.window.offset(state.strstart as isize),
            have as usize,
        );
        read_buf(strm, state.wrap, out);
        (*s).strstart = (*s).strstart.wrapping_add(have);
        (*s).insert = deflate_stored_advance_insert((*s).insert, (*s).w_size, have);
    }
    if (*s).high_water < (*s).strstart as crate::zutil_h::ulg {
        (*s).high_water = (*s).strstart as crate::zutil_h::ulg;
    }
    if let Some(plan) = deflate_stored_final_emit_plan(
        (*s).bi_valid,
        (*s).pending_buf_size,
        (*s).w_size,
        (*s).strstart,
        (*s).block_start,
        (*(*s).strm).avail_in,
        flush,
    ) {
        len = plan.len;
        left = plan.left;
        last = plan.last;
        crate::src::trees::_tr_stored_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            ((*s).window as *mut crate::stdlib::charf).offset((*s).block_start as isize),
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

unsafe fn deflate_fast(
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
            (*s).ins_h = deflate_hash_update(
                (*s).ins_h,
                (*s).hash_shift,
                (*s).hash_mask,
                *(*s).window.offset((*s).strstart.wrapping_add(
                    (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
                ) as isize),
            );
            let ref mut c2rust_fresh46 = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
            *c2rust_fresh46 = *(*s).head.offset((*s).ins_h as isize);
            hash_head = *c2rust_fresh46 as crate::src::deflate::IPos;
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
            let original_dist = dist as ::core::ffi::c_uint;
            for byte in
                crate::src::trees::tr_tally_symbol_bytes(original_dist, len as ::core::ffi::c_uint)
            {
                let sym_next = (*s).sym_next;
                (*s).sym_next = (*s).sym_next.wrapping_add(1);
                *(*s).sym_buf.offset(sym_next as isize) = byte;
            }
            let codes =
                crate::src::trees::tr_tally_match_codes(original_dist, len as ::core::ffi::c_uint);
            crate::src::trees::tr_tally_update_match_counts(
                &mut (*s).dyn_ltree,
                &mut (*s).dyn_dtree,
                codes.length_code,
                codes.dist_code,
            );
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub((*s).match_length);
            if (*s).match_length <= (*s).max_lazy_match
                && (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            {
                (*s).match_length = (*s).match_length.wrapping_sub(1);
                loop {
                    (*s).strstart = (*s).strstart.wrapping_add(1);
                    (*s).ins_h = deflate_hash_update(
                        (*s).ins_h,
                        (*s).hash_shift,
                        (*s).hash_mask,
                        *(*s).window.offset((*s).strstart.wrapping_add(
                            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as crate::stdlib::uInt,
                        ) as isize),
                    );
                    let ref mut c2rust_fresh50 =
                        *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
                    *c2rust_fresh50 = *(*s).head.offset((*s).ins_h as isize);
                    hash_head = *c2rust_fresh50 as crate::src::deflate::IPos;
                    *(*s).head.offset((*s).ins_h as isize) =
                        (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
                    (*s).match_length = (*s).match_length.wrapping_sub(1);
                    if !((*s).match_length != 0 as crate::stdlib::uInt) {
                        break;
                    }
                }
                (*s).strstart = (*s).strstart.wrapping_add(1);
            } else {
                (*s).strstart = (*s).strstart.wrapping_add((*s).match_length);
                (*s).match_length = 0 as crate::stdlib::uInt;
                (*s).ins_h = *(*s).window.offset((*s).strstart as isize) as crate::stdlib::uInt;
                (*s).ins_h = deflate_hash_update(
                    (*s).ins_h,
                    (*s).hash_shift,
                    (*s).hash_mask,
                    *(*s)
                        .window
                        .offset((*s).strstart.wrapping_add(1 as crate::stdlib::uInt) as isize),
                );
            }
        } else {
            let mut cc: crate::zutil_h::uch =
                *(*s).window.offset((*s).strstart as isize) as crate::zutil_h::uch;
            for byte in crate::src::trees::tr_tally_literal_update(&mut (*s).dyn_ltree, cc) {
                let sym_next = (*s).sym_next;
                (*s).sym_next = (*s).sym_next.wrapping_add(1);
                *(*s).sym_buf.offset(sym_next as isize) = byte;
            }
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            (*s).strstart = (*s).strstart.wrapping_add(1);
        }
        if bflush != 0 {
            crate::src::trees::_tr_flush_block_ffi(
                s as *mut crate::src::deflate::internal_state,
                if (*s).block_start >= 0 as ::core::ffi::c_long {
                    (*s).window
                        .offset((*s).block_start as ::core::ffi::c_uint as isize)
                        as *mut crate::stdlib::Bytef
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
                return deflate_flush_blocked_state(false);
            }
        }
    }
    (*s).insert = deflate_insert_limit((*s).strstart);
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::Bytef as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return deflate_flush_blocked_state(true);
        }
        return finish_done;
    }
    if (*s).sym_next != 0 {
        crate::src::trees::_tr_flush_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::Bytef as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
            0 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return deflate_flush_blocked_state(false);
        }
    }
    return block_done;
}

unsafe fn deflate_slow(
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
            (*s).ins_h = deflate_hash_update(
                (*s).ins_h,
                (*s).hash_shift,
                (*s).hash_mask,
                *(*s).window.offset((*s).strstart.wrapping_add(
                    (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
                ) as isize),
            );
            let ref mut c2rust_fresh35 = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
            *c2rust_fresh35 = *(*s).head.offset((*s).ins_h as isize);
            hash_head = *c2rust_fresh35 as crate::src::deflate::IPos;
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
            let original_dist = dist as ::core::ffi::c_uint;
            for byte in
                crate::src::trees::tr_tally_symbol_bytes(original_dist, len as ::core::ffi::c_uint)
            {
                let sym_next = (*s).sym_next;
                (*s).sym_next = (*s).sym_next.wrapping_add(1);
                *(*s).sym_buf.offset(sym_next as isize) = byte;
            }
            let codes =
                crate::src::trees::tr_tally_match_codes(original_dist, len as ::core::ffi::c_uint);
            crate::src::trees::tr_tally_update_match_counts(
                &mut (*s).dyn_ltree,
                &mut (*s).dyn_dtree,
                codes.length_code,
                codes.dist_code,
            );
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s)
                .lookahead
                .wrapping_sub((*s).prev_length.wrapping_sub(1 as crate::stdlib::uInt));
            (*s).prev_length = (*s).prev_length.wrapping_sub(2 as crate::stdlib::uInt);
            loop {
                (*s).strstart = (*s).strstart.wrapping_add(1);
                if (*s).strstart <= max_insert {
                    (*s).ins_h = deflate_hash_update(
                        (*s).ins_h,
                        (*s).hash_shift,
                        (*s).hash_mask,
                        *(*s).window.offset((*s).strstart.wrapping_add(
                            (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as crate::stdlib::uInt,
                        ) as isize),
                    );
                    let ref mut c2rust_fresh39 =
                        *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
                    *c2rust_fresh39 = *(*s).head.offset((*s).ins_h as isize);
                    hash_head = *c2rust_fresh39 as crate::src::deflate::IPos;
                    *(*s).head.offset((*s).ins_h as isize) =
                        (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
                }
                (*s).prev_length = (*s).prev_length.wrapping_sub(1);
                if !((*s).prev_length != 0 as crate::stdlib::uInt) {
                    break;
                }
            }
            (*s).match_available = 0 as ::core::ffi::c_int;
            (*s).match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            (*s).strstart = (*s).strstart.wrapping_add(1);
            if bflush != 0 {
                crate::src::trees::_tr_flush_block_ffi(
                    s as *mut crate::src::deflate::internal_state,
                    if (*s).block_start >= 0 as ::core::ffi::c_long {
                        (*s).window
                            .offset((*s).block_start as ::core::ffi::c_uint as isize)
                            as *mut crate::stdlib::Bytef
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
                    return deflate_flush_blocked_state(false);
                }
            }
        } else if (*s).match_available != 0 {
            let mut cc: crate::zutil_h::uch = *(*s)
                .window
                .offset((*s).strstart.wrapping_sub(1 as crate::stdlib::uInt) as isize)
                as crate::zutil_h::uch;
            for byte in crate::src::trees::tr_tally_literal_update(&mut (*s).dyn_ltree, cc) {
                let sym_next = (*s).sym_next;
                (*s).sym_next = (*s).sym_next.wrapping_add(1);
                *(*s).sym_buf.offset(sym_next as isize) = byte;
            }
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            if bflush != 0 {
                crate::src::trees::_tr_flush_block_ffi(
                    s as *mut crate::src::deflate::internal_state,
                    if (*s).block_start >= 0 as ::core::ffi::c_long {
                        (*s).window
                            .offset((*s).block_start as ::core::ffi::c_uint as isize)
                            as *mut crate::stdlib::Bytef
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
        for byte in crate::src::trees::tr_tally_literal_update(&mut (*s).dyn_ltree, cc_0) {
            let sym_next = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(sym_next as isize) = byte;
        }
        bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
        (*s).match_available = 0 as ::core::ffi::c_int;
    }
    (*s).insert = deflate_insert_limit((*s).strstart);
    if flush == crate::zlib_h::Z_FINISH {
        crate::src::trees::_tr_flush_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::Bytef as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return deflate_flush_blocked_state(true);
        }
        return finish_done;
    }
    if (*s).sym_next != 0 {
        crate::src::trees::_tr_flush_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::Bytef as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg,
            0 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as crate::stdlib::uInt {
            return deflate_flush_blocked_state(false);
        }
    }
    return block_done;
}

unsafe fn deflate_rle(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    let mut prev: crate::stdlib::uInt = 0;
    let mut scan: *mut crate::stdlib::Bytef = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    let mut strend: *mut crate::stdlib::Bytef = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    loop {
        if {
            let state = &mut *s;
            state.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt
        } {
            fill_window(s);
            {
                let state = &mut *s;
                if state.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt
                    && flush == crate::zlib_h::Z_NO_FLUSH
                {
                    return need_more;
                }
                if state.lookahead == 0 as crate::stdlib::uInt {
                    break;
                }
            }
        }
        {
            let state = &mut *s;
            state.match_length = 0 as crate::stdlib::uInt;
            if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                && state.strstart > 0 as crate::stdlib::uInt
            {
                scan = state
                    .window
                    .offset(state.strstart as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize));
                prev = *scan as crate::stdlib::uInt;
                scan = scan.offset(1);
                if prev == *scan as crate::stdlib::uInt
                    && {
                        scan = scan.offset(1);
                        prev == *scan as crate::stdlib::uInt
                    }
                    && {
                        scan = scan.offset(1);
                        prev == *scan as crate::stdlib::uInt
                    }
                {
                    strend = state
                        .window
                        .offset(state.strstart as isize)
                        .offset(crate::zutil_h::MAX_MATCH as isize);
                    loop {
                        scan = scan.offset(1);
                        if !(prev == *scan as crate::stdlib::uInt
                            && {
                                scan = scan.offset(1);
                                prev == *scan as crate::stdlib::uInt
                            }
                            && {
                                scan = scan.offset(1);
                                prev == *scan as crate::stdlib::uInt
                            }
                            && {
                                scan = scan.offset(1);
                                prev == *scan as crate::stdlib::uInt
                            }
                            && {
                                scan = scan.offset(1);
                                prev == *scan as crate::stdlib::uInt
                            }
                            && {
                                scan = scan.offset(1);
                                prev == *scan as crate::stdlib::uInt
                            }
                            && {
                                scan = scan.offset(1);
                                prev == *scan as crate::stdlib::uInt
                            }
                            && {
                                scan = scan.offset(1);
                                prev == *scan as crate::stdlib::uInt
                            }
                            && scan < strend)
                        {
                            break;
                        }
                    }
                    state.match_length = (crate::zutil_h::MAX_MATCH as crate::stdlib::uInt)
                        .wrapping_sub(
                            strend.offset_from(scan) as ::core::ffi::c_long as crate::stdlib::uInt
                        );
                    if state.match_length > state.lookahead {
                        state.match_length = state.lookahead;
                    }
                }
            }
        }
        if {
            let state = &mut *s;
            state.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        } {
            let state = &mut *s;
            let len: crate::zutil_h::uch =
                state.match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush = 1 as ::core::ffi::c_int as crate::zutil_h::ush;
            let original_dist = dist as ::core::ffi::c_uint;
            for byte in
                crate::src::trees::tr_tally_symbol_bytes(original_dist, len as ::core::ffi::c_uint)
            {
                let sym_next = state.sym_next;
                state.sym_next = state.sym_next.wrapping_add(1);
                *state.sym_buf.offset(sym_next as isize) = byte;
            }
            let codes =
                crate::src::trees::tr_tally_match_codes(original_dist, len as ::core::ffi::c_uint);
            crate::src::trees::tr_tally_update_match_counts(
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                codes.length_code,
                codes.dist_code,
            );
            bflush = (state.sym_next == state.sym_end) as ::core::ffi::c_int;
            state.lookahead = state.lookahead.wrapping_sub(state.match_length);
            state.strstart = state.strstart.wrapping_add(state.match_length);
            state.match_length = 0 as crate::stdlib::uInt;
        } else {
            let state = &mut *s;
            let cc: crate::zutil_h::uch =
                *state.window.offset(state.strstart as isize) as crate::zutil_h::uch;
            for byte in crate::src::trees::tr_tally_literal_update(&mut state.dyn_ltree, cc) {
                let sym_next = state.sym_next;
                state.sym_next = state.sym_next.wrapping_add(1);
                *state.sym_buf.offset(sym_next as isize) = byte;
            }
            bflush = (state.sym_next == state.sym_end) as ::core::ffi::c_int;
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            let (buf, stored_len) = {
                let state = &mut *s;
                (
                    if state.block_start >= 0 as ::core::ffi::c_long {
                        state
                            .window
                            .offset(state.block_start as ::core::ffi::c_uint as isize)
                            as *mut crate::stdlib::Bytef
                            as *mut crate::stdlib::charf
                    } else {
                        ::core::ptr::null_mut::<crate::stdlib::charf>()
                    },
                    (state.strstart as ::core::ffi::c_long - state.block_start)
                        as crate::zutil_h::ulg,
                )
            };
            crate::src::trees::_tr_flush_block_ffi(
                s as *mut crate::src::deflate::internal_state,
                buf,
                stored_len,
                0 as ::core::ffi::c_int,
            );
            let strm = {
                let state = &mut *s;
                state.block_start = state.strstart as ::core::ffi::c_long;
                state.strm
            };
            flush_pending(strm);
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                return deflate_flush_blocked_state(false);
            }
        }
    }
    (*s).insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        let (buf, stored_len) = {
            let state = &mut *s;
            (
                if state.block_start >= 0 as ::core::ffi::c_long {
                    state
                        .window
                        .offset(state.block_start as ::core::ffi::c_uint as isize)
                        as *mut crate::stdlib::Bytef
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            )
        };
        crate::src::trees::_tr_flush_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            buf,
            stored_len,
            1 as ::core::ffi::c_int,
        );
        let strm = {
            let state = &mut *s;
            state.block_start = state.strstart as ::core::ffi::c_long;
            state.strm
        };
        flush_pending(strm);
        if (*strm).avail_out == 0 as crate::stdlib::uInt {
            return deflate_flush_blocked_state(true);
        }
        return finish_done;
    }
    if {
        let state = &mut *s;
        state.sym_next != 0
    } {
        let (buf, stored_len) = {
            let state = &mut *s;
            (
                if state.block_start >= 0 as ::core::ffi::c_long {
                    state
                        .window
                        .offset(state.block_start as ::core::ffi::c_uint as isize)
                        as *mut crate::stdlib::Bytef
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            )
        };
        crate::src::trees::_tr_flush_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            buf,
            stored_len,
            0 as ::core::ffi::c_int,
        );
        let strm = {
            let state = &mut *s;
            state.block_start = state.strstart as ::core::ffi::c_long;
            state.strm
        };
        flush_pending(strm);
        if (*strm).avail_out == 0 as crate::stdlib::uInt {
            return deflate_flush_blocked_state(false);
        }
    }
    return block_done;
}

unsafe fn deflate_huff(
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
        {
            let state = &mut *s;
            state.match_length = 0 as crate::stdlib::uInt;
            let mut cc: crate::zutil_h::uch =
                *state.window.offset(state.strstart as isize) as crate::zutil_h::uch;
            for byte in crate::src::trees::tr_tally_literal_update(&mut state.dyn_ltree, cc) {
                let sym_next = state.sym_next;
                state.sym_next = state.sym_next.wrapping_add(1);
                *state.sym_buf.offset(sym_next as isize) = byte;
            }
            bflush = (state.sym_next == state.sym_end) as ::core::ffi::c_int;
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            let (buf, stored_len) = {
                let state = &mut *s;
                (
                    if state.block_start >= 0 as ::core::ffi::c_long {
                        state
                            .window
                            .offset(state.block_start as ::core::ffi::c_uint as isize)
                            as *mut crate::stdlib::Bytef
                            as *mut crate::stdlib::charf
                    } else {
                        ::core::ptr::null_mut::<crate::stdlib::charf>()
                    },
                    (state.strstart as ::core::ffi::c_long - state.block_start)
                        as crate::zutil_h::ulg,
                )
            };
            crate::src::trees::_tr_flush_block_ffi(
                s as *mut crate::src::deflate::internal_state,
                buf,
                stored_len,
                0 as ::core::ffi::c_int,
            );
            let strm = {
                let state = &mut *s;
                state.block_start = state.strstart as ::core::ffi::c_long;
                state.strm
            };
            flush_pending(strm);
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                return deflate_flush_blocked_state(false);
            }
        }
    }
    (*s).insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        let (buf, stored_len) = {
            let state = &mut *s;
            (
                if state.block_start >= 0 as ::core::ffi::c_long {
                    state
                        .window
                        .offset(state.block_start as ::core::ffi::c_uint as isize)
                        as *mut crate::stdlib::Bytef
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            )
        };
        crate::src::trees::_tr_flush_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            buf,
            stored_len,
            1 as ::core::ffi::c_int,
        );
        let strm = {
            let state = &mut *s;
            state.block_start = state.strstart as ::core::ffi::c_long;
            state.strm
        };
        flush_pending(strm);
        if (*strm).avail_out == 0 as crate::stdlib::uInt {
            return deflate_flush_blocked_state(true);
        }
        return finish_done;
    }
    if (*s).sym_next != 0 {
        let (buf, stored_len) = {
            let state = &mut *s;
            (
                if state.block_start >= 0 as ::core::ffi::c_long {
                    state
                        .window
                        .offset(state.block_start as ::core::ffi::c_uint as isize)
                        as *mut crate::stdlib::Bytef
                        as *mut crate::stdlib::charf
                } else {
                    ::core::ptr::null_mut::<crate::stdlib::charf>()
                },
                (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg,
            )
        };
        crate::src::trees::_tr_flush_block_ffi(
            s as *mut crate::src::deflate::internal_state,
            buf,
            stored_len,
            0 as ::core::ffi::c_int,
        );
        let strm = {
            let state = &mut *s;
            state.block_start = state.strstart as ::core::ffi::c_long;
            state.strm
        };
        flush_pending(strm);
        if (*strm).avail_out == 0 as crate::stdlib::uInt {
            return deflate_flush_blocked_state(false);
        }
    }
    return block_done;
}
