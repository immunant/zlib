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
    pub value: crate::zutil_h::ush,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub union C2Rust_Unnamed_0 {
    pub dad: crate::zutil_h::ush,
    pub len: crate::zutil_h::ush,
}

pub type static_tree_desc = crate::src::deflate::static_tree_desc_s;

pub type tree_desc = crate::src::deflate::tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tree_desc_s {
    pub dyn_tree: *mut crate::src::deflate::ct_data,
    pub max_code: ::core::ffi::c_int,
    pub stat_desc: *const crate::src::deflate::static_tree_desc,
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
pub use crate::src::trees::_tr_flush_block;
pub use crate::src::trees::_tr_init;
pub use crate::src::trees::_tr_stored_block;
pub use crate::src::zutil::z_errmsg;
pub use crate::src::zutil::zcalloc_ffi;
pub use crate::src::zutil::zcfree_ffi;
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

pub type compress_func = Option<
    unsafe extern "C" fn(
        *mut crate::src::deflate::deflate_state,
        ::core::ffi::c_int,
    ) -> block_state,
>;

pub type config = config_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct config_s {
    pub good_length: crate::zutil_h::ush,
    pub max_lazy: crate::zutil_h::ush,
    pub nice_length: crate::zutil_h::ush,
    pub max_chain: crate::zutil_h::ush,
    pub func: compress_func,
}
#[no_mangle]
pub static deflate_copyright: [u8; 70] =
    *b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0";

pub const NIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TOO_FAR: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

static configuration_table: [config; 10] = [
    config_s {
        good_length: 0 as crate::zutil_h::ush,
        max_lazy: 0 as crate::zutil_h::ush,
        nice_length: 0 as crate::zutil_h::ush,
        max_chain: 0 as crate::zutil_h::ush,
        func: Some(
            deflate_stored
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        func: Some(
            deflate_fast
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        func: Some(
            deflate_fast
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: Some(
            deflate_fast
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        func: Some(
            deflate_slow
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: Some(
            deflate_slow
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        func: Some(
            deflate_slow
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        func: Some(
            deflate_slow
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        func: Some(
            deflate_slow
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        func: Some(
            deflate_slow
                as unsafe extern "C" fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
];

fn slide_hash_entry(position: ::core::ffi::c_uint, window_size: crate::stdlib::uInt) -> Posf {
    (if position >= window_size {
        position.wrapping_sub(window_size)
    } else {
        NIL as ::core::ffi::c_uint
    }) as crate::src::deflate::Pos as Posf
}

fn clamped_copy_len(
    available: crate::zutil_h::ulg,
    requested: crate::zutil_h::ulg,
) -> ::core::ffi::c_uint {
    available.min(requested) as ::core::ffi::c_uint
}

pub(crate) fn symbol_triplet_cursors(
    start: crate::stdlib::uInt,
) -> ([crate::stdlib::uInt; 3], crate::stdlib::uInt) {
    let second = start.wrapping_add(1);
    let third = second.wrapping_add(1);
    ([start, second, third], third.wrapping_add(1))
}

unsafe extern "C" fn slide_hash(mut s: *mut crate::src::deflate::deflate_state) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut m: ::core::ffi::c_uint = 0;
    let mut p: *mut crate::src::deflate::Posf =
        ::core::ptr::null_mut::<crate::src::deflate::Posf>();
    let mut wsize: crate::stdlib::uInt = (*s).w_size;
    n = (*s).hash_size as ::core::ffi::c_uint;
    p = (*s).head.wrapping_add(n as usize);
    loop {
        p = p.wrapping_sub(1);
        m = *p as ::core::ffi::c_uint;
        *p = slide_hash_entry(m, wsize);
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
    n = wsize as ::core::ffi::c_uint;
    p = (*s).prev.wrapping_add(n as usize);
    loop {
        p = p.wrapping_sub(1);
        m = *p as ::core::ffi::c_uint;
        *p = slide_hash_entry(m, wsize);
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
    (*s).slid = 1 as ::core::ffi::c_int;
}

fn read_buf_len(
    available: ::core::ffi::c_uint,
    requested: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    clamped_copy_len(
        available as crate::zutil_h::ulg,
        requested as crate::zutil_h::ulg,
    )
}

fn read_buf_total_in_after_copy(
    total_in: crate::stdlib::uLong,
    copied: ::core::ffi::c_uint,
) -> crate::stdlib::uLong {
    total_in.wrapping_add(copied as crate::stdlib::uLong)
}

unsafe extern "C" fn read_buf(
    mut strm: crate::zlib_h::z_streamp,
    mut buf: *mut crate::stdlib::Bytef,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let len = read_buf_len((*strm).avail_in as ::core::ffi::c_uint, size);
    if len == 0 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_uint;
    }
    (*strm).avail_in = (*strm).avail_in.wrapping_sub(len);
    crate::stdlib::memcpy(
        buf as *mut ::core::ffi::c_void,
        (*strm).next_in as *const ::core::ffi::c_void,
        len as crate::__stddef_size_t_h::size_t,
    );
    if (*(*strm).state).wrap == 1 as ::core::ffi::c_int {
        (*strm).adler =
            crate::src::adler32::adler32_ffi((*strm).adler, buf, len as crate::stdlib::uInt);
    } else if (*(*strm).state).wrap == 2 as ::core::ffi::c_int {
        (*strm).adler =
            crate::src::crc32::crc32_ffi((*strm).adler, buf, len as crate::stdlib::uInt);
    }
    (*strm).next_in = (*strm).next_in.wrapping_add(len as usize);
    (*strm).total_in = read_buf_total_in_after_copy((*strm).total_in, len);
    return len;
}

fn fill_window_available_space(
    window_size: crate::zutil_h::ulg,
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    wsize: crate::stdlib::uInt,
    narrow_int: bool,
) -> ::core::ffi::c_uint {
    let mut available = window_size
        .wrapping_sub(lookahead as crate::zutil_h::ulg)
        .wrapping_sub(strstart as crate::zutil_h::ulg)
        as ::core::ffi::c_uint;

    if narrow_int {
        if available == 0 && strstart == 0 && lookahead == 0 {
            available = wsize as ::core::ffi::c_uint;
        } else if available == -1 as ::core::ffi::c_int as ::core::ffi::c_uint {
            available = available.wrapping_sub(1);
        }
    }

    available
}

fn fill_window_insert_after_slide(
    insert: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    if insert > strstart {
        strstart
    } else {
        insert
    }
}

fn fill_window_cursor(
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::zutil_h::ulg {
    (strstart as crate::zutil_h::ulg).wrapping_add(lookahead as crate::zutil_h::ulg)
}

fn fill_window_zero_range(
    high_water: crate::zutil_h::ulg,
    window_size: crate::zutil_h::ulg,
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> Option<(crate::zutil_h::ulg, crate::zutil_h::ulg)> {
    if high_water >= window_size {
        return None;
    }

    let cursor = fill_window_cursor(strstart, lookahead);
    let win_init = crate::src::deflate::WIN_INIT as crate::zutil_h::ulg;
    if high_water < cursor {
        Some((cursor, window_size.wrapping_sub(cursor).min(win_init)))
    } else if high_water < cursor.wrapping_add(win_init) {
        Some((
            high_water,
            cursor
                .wrapping_add(win_init)
                .wrapping_sub(high_water)
                .min(window_size.wrapping_sub(high_water)),
        ))
    } else {
        None
    }
}

unsafe extern "C" fn fill_window(mut s: *mut crate::src::deflate::deflate_state) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    let mut wsize: crate::stdlib::uInt = (*s).w_size;
    loop {
        more = fill_window_available_space(
            (*s).window_size,
            (*s).lookahead,
            (*s).strstart,
            wsize,
            ::core::mem::size_of::<::core::ffi::c_int>() <= 2,
        );
        if (*s).strstart
            >= wsize.wrapping_add(
                (*s).w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            crate::stdlib::memcpy(
                (*s).window as *mut ::core::ffi::c_void,
                (*s).window.wrapping_add(wsize as usize) as *const ::core::ffi::c_void,
                wsize.wrapping_sub(more) as crate::__stddef_size_t_h::size_t,
            );
            (*s).match_start = (*s).match_start.wrapping_sub(wsize);
            (*s).strstart = (*s).strstart.wrapping_sub(wsize);
            (*s).block_start -= wsize as ::core::ffi::c_long;
            (*s).insert = fill_window_insert_after_slide((*s).insert, (*s).strstart);
            slide_hash(s);
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if (*(*s).strm).avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        let cursor = fill_window_cursor((*s).strstart, (*s).lookahead);
        n = read_buf((*s).strm, (*s).window.wrapping_add(cursor as usize), more);
        (*s).lookahead = (*s).lookahead.wrapping_add(n);
        if (*s).lookahead.wrapping_add((*s).insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        {
            let mut str: crate::stdlib::uInt = (*s).strstart.wrapping_sub((*s).insert);
            (*s).ins_h = *(*s).window.offset(str as isize) as crate::stdlib::uInt;
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s)
                    .window
                    .offset(str.wrapping_add(1 as crate::stdlib::uInt) as isize)
                    as crate::stdlib::uInt)
                & (*s).hash_mask;
            while (*s).insert != 0 {
                (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                    ^ *(*s).window.offset(
                        str.wrapping_add(3 as crate::stdlib::uInt)
                            .wrapping_sub(1 as crate::stdlib::uInt)
                            as isize,
                    ) as crate::stdlib::uInt)
                    & (*s).hash_mask;
                *(*s).prev.offset((str & (*s).w_mask) as isize) =
                    *(*s).head.offset((*s).ins_h as isize);
                *(*s).head.offset((*s).ins_h as isize) =
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
    if let Some((start, len)) = fill_window_zero_range(
        (*s).high_water,
        (*s).window_size,
        (*s).strstart,
        (*s).lookahead,
    ) {
        crate::stdlib::memset(
            (*s).window.offset(start as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            len as ::core::ffi::c_uint as crate::__stddef_size_t_h::size_t,
        );
        (*s).high_water = start.wrapping_add(len);
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
fn deflate_version_matches(
    version_first: ::core::ffi::c_char,
    stream_size: ::core::ffi::c_int,
) -> bool {
    version_first as ::core::ffi::c_int == crate::zlib_h::ZLIB_VERSION[0] as ::core::ffi::c_int
        && stream_size as usize == ::core::mem::size_of::<crate::zlib_h::z_stream>()
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
    if version.is_null()
        || !deflate_version_matches(
            *version.offset(0 as ::core::ffi::c_int as isize),
            stream_size,
        )
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
        ::core::mem::size_of::<crate::src::deflate::deflate_state>()
            as crate::__stddef_size_t_h::size_t,
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
        }) as usize];
        deflateEnd(strm);
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
fn deflate_state_status_valid(status: ::core::ffi::c_int) -> bool {
    matches!(
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

unsafe extern "C" fn deflateStateCheck(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as ::core::ffi::c_int;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    if s.is_null() || (*s).strm != strm || !deflate_state_status_valid((*s).status) {
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
        (*strm).adler = crate::src::adler32::adler32_ffi((*strm).adler, dictionary, dictLength);
    }
    (*s).wrap = 0 as ::core::ffi::c_int;
    if dictLength >= (*s).w_size {
        if wrap == 0 as ::core::ffi::c_int {
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
#[export_name = "deflateSetDictionary"]

pub unsafe extern "C" fn deflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    deflateSetDictionary(strm, dictionary, dictLength)
}
fn deflate_dictionary_len(
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    strstart.wrapping_add(lookahead).min(w_size)
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
    len = deflate_dictionary_len((*s).strstart, (*s).lookahead, (*s).w_size);
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
    (*s).pending_out = (*s).pending_buf;
    if (*s).wrap < 0 as ::core::ffi::c_int {
        (*s).wrap = -(*s).wrap;
    }
    (*s).status = if (*s).wrap == 2 as ::core::ffi::c_int {
        crate::src::deflate::GZIP_STATE
    } else {
        crate::src::deflate::INIT_STATE
    };
    (*strm).adler = if (*s).wrap == 2 as ::core::ffi::c_int {
        0 as crate::stdlib::uLong
    } else {
        1 as crate::stdlib::uLong
    };
    (*s).last_flush = -2 as ::core::ffi::c_int;
    crate::src::trees::_tr_init(s as *mut crate::src::deflate::internal_state);
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateResetKeep(strm)
}
unsafe extern "C" fn lm_init(mut s: *mut crate::src::deflate::deflate_state) {
    (*s).window_size = (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
        .wrapping_mul((*s).w_size as crate::zutil_h::ulg);
    *(*s)
        .head
        .offset((*s).hash_size.wrapping_sub(1 as crate::stdlib::uInt) as isize) =
        NIL as crate::src::deflate::Posf;
    crate::stdlib::memset(
        (*s).head as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((*s).hash_size.wrapping_sub(1 as crate::stdlib::uInt) as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Posf>()
                as crate::__stddef_size_t_h::size_t),
    );
    (*s).slid = 0 as ::core::ffi::c_int;
    (*s).max_lazy_match = configuration_table[(*s).level as usize].max_lazy as crate::stdlib::uInt;
    (*s).good_match = configuration_table[(*s).level as usize].good_length as crate::stdlib::uInt;
    (*s).nice_match = configuration_table[(*s).level as usize].nice_length as ::core::ffi::c_int;
    (*s).max_chain_length =
        configuration_table[(*s).level as usize].max_chain as crate::stdlib::uInt;
    (*s).strstart = 0 as crate::stdlib::uInt;
    (*s).block_start = 0 as ::core::ffi::c_long;
    (*s).lookahead = 0 as crate::stdlib::uInt;
    (*s).insert = 0 as crate::stdlib::uInt;
    (*s).prev_length = (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
    (*s).match_length = (*s).prev_length;
    (*s).match_available = 0 as ::core::ffi::c_int;
    (*s).ins_h = 0 as crate::stdlib::uInt;
}
pub unsafe extern "C" fn deflateReset(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = deflateResetKeep(strm);
    if ret == crate::zlib_h::Z_OK {
        lm_init((*strm).state as *mut crate::src::deflate::deflate_state);
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
fn deflate_pending_value(pending: crate::zutil_h::ulg) -> Result<::core::ffi::c_uint, ()> {
    let value = pending as ::core::ffi::c_uint;
    if value as crate::zutil_h::ulg == pending {
        Ok(value)
    } else {
        Err(())
    }
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
    let state = (*strm).state as *mut crate::src::deflate::deflate_state;
    if !bits.is_null() {
        *bits = (*state).bi_valid;
    }
    if !pending.is_null() {
        match deflate_pending_value((*state).pending) {
            Ok(value) => *pending = value,
            Err(()) => {
                *pending = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
                return crate::zlib_h::Z_BUF_ERROR;
            }
        }
    }
    crate::zlib_h::Z_OK
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
        let state = (*strm).state as *mut crate::src::deflate::deflate_state;
        *bits = (*state).bi_used;
    }
    crate::zlib_h::Z_OK
}
fn deflate_prime_bits_valid(bits: ::core::ffi::c_int) -> bool {
    bits >= 0 as ::core::ffi::c_int && bits <= 16 as ::core::ffi::c_int
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
    if !deflate_prime_bits_valid(bits)
        || (*s).sym_buf
            < (*s).pending_out.offset(
                (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                    as isize,
            )
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
        crate::src::trees::_tr_flush_bits(s as *mut crate::src::deflate::internal_state);
        value >>= put;
        bits -= put;
        if !(bits != 0) {
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

fn normalize_deflate_params(
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Option<(::core::ffi::c_int, ::core::ffi::c_int)> {
    let level = if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        6 as ::core::ffi::c_int
    } else {
        level
    };

    if level < 0 as ::core::ffi::c_int
        || level > 9 as ::core::ffi::c_int
        || strategy < 0 as ::core::ffi::c_int
        || strategy > crate::zlib_h::Z_FIXED
    {
        None
    } else {
        Some((level, strategy))
    }
}

pub unsafe extern "C" fn deflateParams(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut func: compress_func = None;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    let (level, strategy) = match normalize_deflate_params(level, strategy) {
        Some(params) => params,
        None => return crate::zlib_h::Z_STREAM_ERROR,
    };
    func = configuration_table[(*s).level as usize].func;
    if (strategy != (*s).strategy || func != configuration_table[level as usize].func)
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
                slide_hash(s);
            } else {
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
pub unsafe extern "C" fn deflateTune(
    mut strm: crate::zlib_h::z_streamp,
    mut good_length: ::core::ffi::c_int,
    mut max_lazy: ::core::ffi::c_int,
    mut nice_length: ::core::ffi::c_int,
    mut max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    (*s).good_match = good_length as crate::stdlib::uInt;
    (*s).max_lazy_match = max_lazy as crate::stdlib::uInt;
    (*s).nice_match = nice_length;
    (*s).max_chain_length = max_chain as crate::stdlib::uInt;
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
fn deflate_bound_lengths(
    source_len: crate::stdlib::z_size_t,
) -> (crate::stdlib::z_size_t, crate::stdlib::z_size_t) {
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
    (fixedlen, storelen)
}

pub unsafe extern "C" fn deflateBound_z(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let fixedlen: crate::stdlib::z_size_t;
    let storelen: crate::stdlib::z_size_t;
    let mut wraplen: crate::stdlib::z_size_t = 0;
    let mut bound: crate::stdlib::z_size_t = 0;
    (fixedlen, storelen) = deflate_bound_lengths(sourceLen);
    if deflateStateCheck(strm) != 0 {
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
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    match if (*s).wrap < 0 as ::core::ffi::c_int {
        -(*s).wrap
    } else {
        (*s).wrap
    } {
        0 => {
            wraplen = 0 as crate::stdlib::z_size_t;
        }
        1 => {
            wraplen = (6 as ::core::ffi::c_int
                + (if (*s).strstart != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as crate::stdlib::z_size_t;
        }
        2 => {
            wraplen = 18 as crate::stdlib::z_size_t;
            if !(*s).gzhead.is_null() {
                let mut str: *mut crate::stdlib::Bytef =
                    ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                if !(*(*s).gzhead).extra.is_null() {
                    wraplen = wraplen.wrapping_add(
                        (2 as crate::stdlib::uInt).wrapping_add((*(*s).gzhead).extra_len)
                            as crate::stdlib::z_size_t,
                    );
                }
                str = (*(*s).gzhead).name;
                if !str.is_null() {
                    loop {
                        wraplen = wraplen.wrapping_add(1);
                        let c2rust_fresh63 = str;
                        str = str.wrapping_add(1);
                        if !(*c2rust_fresh63 != 0) {
                            break;
                        }
                    }
                }
                str = (*(*s).gzhead).comment;
                if !str.is_null() {
                    loop {
                        wraplen = wraplen.wrapping_add(1);
                        let c2rust_fresh64 = str;
                        str = str.wrapping_add(1);
                        if !(*c2rust_fresh64 != 0) {
                            break;
                        }
                    }
                }
                if (*(*s).gzhead).hcrc != 0 {
                    wraplen = wraplen.wrapping_add(2 as crate::stdlib::z_size_t);
                }
            }
        }
        _ => {
            wraplen = 18 as crate::stdlib::z_size_t;
        }
    }
    if (*s).w_bits != 15 as crate::stdlib::uInt
        || (*s).hash_bits
            != (8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        bound = if (*s).w_bits <= (*s).hash_bits && (*s).level != 0 {
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
    bound = sourceLen
        .wrapping_add(sourceLen >> 12 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 14 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as crate::stdlib::z_size_t)
        .wrapping_sub(6 as crate::stdlib::z_size_t)
        .wrapping_add(wraplen);
    return if bound < sourceLen {
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
    deflateBound_z(strm, sourceLen)
}
#[export_name = "deflateBound"]
pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let mut bound: crate::stdlib::z_size_t =
        deflateBound_z(strm, sourceLen as crate::stdlib::z_size_t);
    if bound != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        bound as crate::stdlib::uLong
    }
}

fn short_msb_bytes(b: crate::stdlib::uInt) -> [crate::stdlib::Byte; 2] {
    [
        (b >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte,
        (b & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte,
    ]
}

fn pending_short_cursors(
    pending: crate::zutil_h::ulg,
) -> ([crate::zutil_h::ulg; 2], crate::zutil_h::ulg) {
    let second = pending.wrapping_add(1);
    ([pending, second], second.wrapping_add(1))
}

unsafe extern "C" fn putShortMSB(
    mut s: *mut crate::src::deflate::deflate_state,
    mut b: crate::stdlib::uInt,
) {
    let bytes = short_msb_bytes(b);
    let (cursors, next_pending) = pending_short_cursors((*s).pending);
    (*s).pending = next_pending;
    *(*s).pending_buf.wrapping_add(cursors[0] as usize) = bytes[0];
    *(*s).pending_buf.wrapping_add(cursors[1] as usize) = bytes[1];
}

fn pending_output_len(
    pending: crate::zutil_h::ulg,
    avail_out: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    clamped_copy_len(pending, avail_out as crate::zutil_h::ulg)
}

fn flush_pending_accounting(
    pending: crate::zutil_h::ulg,
    avail_out: crate::stdlib::uInt,
    total_out: crate::stdlib::uLong,
) -> Option<(
    ::core::ffi::c_uint,
    crate::zutil_h::ulg,
    crate::stdlib::uInt,
    crate::stdlib::uLong,
    bool,
)> {
    let len = pending_output_len(pending, avail_out);
    if len == 0 {
        return None;
    }

    let remaining = pending.wrapping_sub(len as crate::zutil_h::ulg);
    Some((
        len,
        remaining,
        avail_out.wrapping_sub(len),
        total_out.wrapping_add(len as crate::stdlib::uLong),
        remaining == 0,
    ))
}

unsafe extern "C" fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    let mut s: *mut crate::src::deflate::deflate_state =
        (*strm).state as *mut crate::src::deflate::deflate_state;
    crate::src::trees::_tr_flush_bits(s as *mut crate::src::deflate::internal_state);
    let Some((len, remaining, avail_out, total_out, reset_pending_out)) =
        flush_pending_accounting((*s).pending, (*strm).avail_out, (*strm).total_out)
    else {
        return;
    };
    crate::stdlib::memcpy(
        (*strm).next_out as *mut ::core::ffi::c_void,
        (*s).pending_out as *const ::core::ffi::c_void,
        len as crate::__stddef_size_t_h::size_t,
    );
    (*strm).next_out = (*strm).next_out.wrapping_add(len as usize);
    (*s).pending_out = (*s).pending_out.wrapping_add(len as usize);
    (*strm).total_out = total_out;
    (*strm).avail_out = avail_out;
    (*s).pending = remaining;
    if reset_pending_out {
        (*s).pending_out = (*s).pending_buf;
    }
}

fn gzip_header_crc(
    crc: crate::stdlib::uLong,
    text: ::core::ffi::c_int,
    hcrc: ::core::ffi::c_int,
    has_extra: bool,
    has_name: bool,
    has_comment: bool,
    time: crate::stdlib::uLong,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    os: ::core::ffi::c_int,
    extra_len: crate::stdlib::uInt,
) -> crate::stdlib::uLong {
    let flags = (if text != 0 { 1 } else { 0 })
        + (if hcrc != 0 { 2 } else { 0 })
        + (if has_extra { 4 } else { 0 })
        + (if has_name { 8 } else { 0 })
        + (if has_comment { 16 } else { 0 });
    let xfl = if level == 9 {
        2
    } else if strategy >= 2 || level < 2 {
        4
    } else {
        0
    };
    let mut header = [
        31,
        139,
        8,
        flags,
        (time & 0xff) as u8,
        ((time >> 8) & 0xff) as u8,
        ((time >> 16) & 0xff) as u8,
        ((time >> 24) & 0xff) as u8,
        xfl,
        (os & 0xff) as u8,
        0,
        0,
    ];
    let header_len = if has_extra {
        header[10] = (extra_len & 0xff) as u8;
        header[11] = ((extra_len >> 8) & 0xff) as u8;
        12
    } else {
        10
    };

    crate::src::crc32::crc32_z(crc, &header[..header_len])
}

fn gzip_header_crc_pending(
    crc: crate::stdlib::uLong,
    hcrc: ::core::ffi::c_int,
    pending_buffer: &[crate::stdlib::Bytef],
    begin: crate::zutil_h::ulg,
    end: crate::zutil_h::ulg,
) -> crate::stdlib::uLong {
    let Some(range) = gzip_header_crc_pending_range(hcrc, begin, end) else {
        return crc;
    };
    let Some(bytes) = pending_buffer.get(range) else {
        return crc;
    };
    crate::src::crc32::crc32_z(crc, bytes)
}

fn gzip_header_crc_pending_range(
    hcrc: ::core::ffi::c_int,
    begin: crate::zutil_h::ulg,
    end: crate::zutil_h::ulg,
) -> Option<::core::ops::Range<usize>> {
    if hcrc == 0 || end <= begin {
        return None;
    }

    Some(usize::try_from(begin).ok()?..usize::try_from(end).ok()?)
}

fn zlib_header(
    w_bits: crate::stdlib::uInt,
    strategy: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    has_dictionary: bool,
) -> crate::stdlib::uInt {
    let level_flags: crate::stdlib::uInt = if strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || level < 2
    {
        0
    } else if level < 6 {
        1
    } else if level == 6 {
        2
    } else {
        3
    };
    let mut header = (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt)
        .wrapping_add(w_bits.wrapping_sub(8) << 4)
        << 8;

    header |= level_flags << 6;
    if has_dictionary {
        header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
    }
    header.wrapping_add(
        (31 as crate::stdlib::uInt).wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
    )
}

fn gzip_default_xfl(
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

fn deflate_should_return_buf_error(
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    old_flush: ::core::ffi::c_int,
) -> bool {
    avail_in == 0
        && flush * 2 - if flush > 4 { 9 } else { 0 }
            <= old_flush * 2 - if old_flush > 4 { 9 } else { 0 }
        && flush != crate::zlib_h::Z_FINISH
}

fn deflate_request_is_invalid(
    next_out_is_null: bool,
    avail_in: crate::stdlib::uInt,
    next_in_is_null: bool,
    status: ::core::ffi::c_int,
    flush: ::core::ffi::c_int,
) -> bool {
    next_out_is_null
        || avail_in != 0 as crate::stdlib::uInt && next_in_is_null
        || status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH
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
    let pending_buffer = if (*s).pending_buf.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts((*s).pending_buf, (*s).pending_buf_size as usize)
    };
    if deflate_request_is_invalid(
        (*strm).next_out.is_null(),
        (*strm).avail_in,
        (*strm).next_in.is_null(),
        (*s).status,
        flush,
    ) {
        (*strm).msg = crate::src::zutil::z_errmsg[(if (-2 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -2 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -2 as ::core::ffi::c_int
        }) as usize];
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
        }) as usize];
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
    } else if deflate_should_return_buf_error((*strm).avail_in, flush, old_flush) {
        (*strm).msg = crate::src::zutil::z_errmsg[(if (-5 as ::core::ffi::c_int)
            < -6 as ::core::ffi::c_int
            || -5 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        {
            9 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int - -5 as ::core::ffi::c_int
        }) as usize];
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
        }) as usize];
        return -5 as ::core::ffi::c_int;
    }
    if (*s).status == crate::src::deflate::INIT_STATE && (*s).wrap == 0 as ::core::ffi::c_int {
        (*s).status = crate::src::deflate::BUSY_STATE;
    }
    if (*s).status == crate::src::deflate::INIT_STATE {
        let header = zlib_header((*s).w_bits, (*s).strategy, (*s).level, (*s).strstart != 0);
        putShortMSB(s, header);
        if (*s).strstart != 0 as crate::stdlib::uInt {
            putShortMSB(
                s,
                ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
            );
            putShortMSB(
                s,
                ((*strm).adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
            );
        }
        (*strm).adler = 1 as crate::stdlib::uLong;
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if (*s).status == crate::src::deflate::GZIP_STATE {
        (*strm).adler = 0 as crate::stdlib::uLong;
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
                gzip_default_xfl((*s).level, (*s).strategy);
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
            let gzhead = &*(*s).gzhead;
            let c2rust_fresh10 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh10 as isize) = ((if gzhead.text != 0 {
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
            }))
                as crate::stdlib::Bytef;
            let c2rust_fresh11 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh11 as isize) =
                (gzhead.time & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte;
            let c2rust_fresh12 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh12 as isize) =
                (gzhead.time >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte;
            let c2rust_fresh13 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh13 as isize) =
                (gzhead.time >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte;
            let c2rust_fresh14 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh14 as isize) =
                (gzhead.time >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte;
            let c2rust_fresh15 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh15 as isize) =
                (if (*s).level == 9 as ::core::ffi::c_int {
                    2 as ::core::ffi::c_int
                } else if (*s).strategy >= 2 as ::core::ffi::c_int
                    || (*s).level < 2 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as crate::stdlib::Bytef;
            let c2rust_fresh16 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh16 as isize) =
                (gzhead.os & 0xff as ::core::ffi::c_int) as crate::stdlib::Bytef;
            if !gzhead.extra.is_null() {
                let c2rust_fresh17 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(c2rust_fresh17 as isize) =
                    (gzhead.extra_len & 0xff as crate::stdlib::uInt) as crate::stdlib::Bytef;
                let c2rust_fresh18 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(c2rust_fresh18 as isize) =
                    (gzhead.extra_len >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uInt)
                        as crate::stdlib::Bytef;
            }
            if gzhead.hcrc != 0 {
                (*strm).adler = gzip_header_crc(
                    (*strm).adler,
                    gzhead.text,
                    gzhead.hcrc,
                    !gzhead.extra.is_null(),
                    !gzhead.name.is_null(),
                    !gzhead.comment.is_null(),
                    gzhead.time,
                    (*s).level,
                    (*s).strategy,
                    gzhead.os,
                    gzhead.extra_len,
                );
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
                (*strm).adler = gzip_header_crc_pending(
                    (*strm).adler,
                    (*(*s).gzhead).hcrc,
                    pending_buffer,
                    beg,
                    (*s).pending,
                );
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
            (*strm).adler = gzip_header_crc_pending(
                (*strm).adler,
                (*(*s).gzhead).hcrc,
                pending_buffer,
                beg,
                (*s).pending,
            );
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
                    (*strm).adler = gzip_header_crc_pending(
                        (*strm).adler,
                        (*(*s).gzhead).hcrc,
                        pending_buffer,
                        beg_0,
                        (*s).pending,
                    );
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
            (*strm).adler = gzip_header_crc_pending(
                (*strm).adler,
                (*(*s).gzhead).hcrc,
                pending_buffer,
                beg_0,
                (*s).pending,
            );
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
                    (*strm).adler = gzip_header_crc_pending(
                        (*strm).adler,
                        (*(*s).gzhead).hcrc,
                        pending_buffer,
                        beg_1,
                        (*s).pending,
                    );
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
            (*strm).adler = gzip_header_crc_pending(
                (*strm).adler,
                (*(*s).gzhead).hcrc,
                pending_buffer,
                beg_1,
                (*s).pending,
            );
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
            (*strm).adler = 0 as crate::stdlib::uLong;
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
            Some(
                (*(&raw const configuration_table as *const config).offset((*s).level as isize))
                    .func
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(s, flush) as ::core::ffi::c_uint
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
        let c2rust_fresh25 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh25 as isize) =
            ((*strm).adler & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte;
        let c2rust_fresh26 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh26 as isize) =
            ((*strm).adler >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                as crate::stdlib::Byte;
        let c2rust_fresh27 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh27 as isize) =
            ((*strm).adler >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                as crate::stdlib::Byte;
        let c2rust_fresh28 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh28 as isize) =
            ((*strm).adler >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                as crate::stdlib::Byte;
        let c2rust_fresh29 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh29 as isize) =
            ((*strm).total_in & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte;
        let c2rust_fresh30 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh30 as isize) =
            ((*strm).total_in >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                as crate::stdlib::Byte;
        let c2rust_fresh31 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh31 as isize) =
            ((*strm).total_in >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                as crate::stdlib::Byte;
        let c2rust_fresh32 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(c2rust_fresh32 as isize) =
            ((*strm).total_in >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                as crate::stdlib::Byte;
    } else {
        putShortMSB(
            s,
            ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
        );
        putShortMSB(
            s,
            ((*strm).adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
        );
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
    (*ds).pending_out = (*ds)
        .pending_buf
        .offset((*ss).pending_out.offset_from((*ss).pending_buf) as ::core::ffi::c_long as isize);
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
    (*ds).l_desc.dyn_tree = &raw mut (*ds).dyn_ltree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    (*ds).d_desc.dyn_tree = &raw mut (*ds).dyn_dtree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    (*ds).bl_desc.dyn_tree = &raw mut (*ds).bl_tree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateCopy"]

pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateCopy(dest, source)
}
unsafe extern "C" fn longest_match(
    mut s: *mut crate::src::deflate::deflate_state,
    mut cur_match: crate::src::deflate::IPos,
) -> crate::stdlib::uInt {
    let mut chain_length: ::core::ffi::c_uint = (*s).max_chain_length as ::core::ffi::c_uint;
    let mut scan: *mut crate::stdlib::Bytef = (*s).window.offset((*s).strstart as isize);
    let mut match_0: *mut crate::stdlib::Bytef = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    let mut len: ::core::ffi::c_int = 0;
    let mut best_len: ::core::ffi::c_int = (*s).prev_length as ::core::ffi::c_int;
    let mut nice_match: ::core::ffi::c_int = (*s).nice_match;
    let mut limit: crate::src::deflate::IPos = if (*s).strstart
        > (*s)
            .w_size
            .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
    {
        ((*s).strstart as crate::src::deflate::IPos).wrapping_sub(
            (*s).w_size
                .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        )
    } else {
        NIL as crate::src::deflate::IPos
    };
    let mut prev: *mut crate::src::deflate::Posf = (*s).prev;
    let mut wmask: crate::stdlib::uInt = (*s).w_mask;
    let mut strend: *mut crate::stdlib::Bytef = (*s)
        .window
        .offset((*s).strstart as isize)
        .offset(crate::zutil_h::MAX_MATCH as isize);
    let mut scan_end1: crate::stdlib::Byte =
        *scan.offset((best_len - 1 as ::core::ffi::c_int) as isize) as crate::stdlib::Byte;
    let mut scan_end: crate::stdlib::Byte = *scan.offset(best_len as isize) as crate::stdlib::Byte;
    if (*s).prev_length >= (*s).good_match {
        chain_length >>= 2 as ::core::ffi::c_int;
    }
    if nice_match as crate::stdlib::uInt > (*s).lookahead {
        nice_match = (*s).lookahead as ::core::ffi::c_int;
    }
    loop {
        match_0 = (*s).window.offset(cur_match as isize);
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
                (*s).match_start = cur_match as crate::stdlib::uInt;
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
    if best_len as crate::stdlib::uInt <= (*s).lookahead {
        return best_len as crate::stdlib::uInt;
    }
    return (*s).lookahead;
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

fn stored_block_min_size(
    pending_buf_size: crate::zutil_h::ulg,
    window_size: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    (if pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg) > window_size as crate::zutil_h::ulg
    {
        window_size as crate::zutil_h::ulg
    } else {
        pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg)
    }) as ::core::ffi::c_uint
}

fn stored_block_available_output(
    bi_valid: ::core::ffi::c_int,
    avail_out: crate::stdlib::uInt,
) -> Option<::core::ffi::c_uint> {
    let header_bytes = (bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
        >> 3 as ::core::ffi::c_int;
    if avail_out < header_bytes {
        None
    } else {
        Some((avail_out as ::core::ffi::c_uint).wrapping_sub(header_bytes))
    }
}

fn stored_block_should_wait(
    len: ::core::ffi::c_uint,
    min_block: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> bool {
    len < min_block
        && (len == 0 && flush != crate::zlib_h::Z_FINISH
            || flush == crate::zlib_h::Z_NO_FLUSH
            || len != (left as crate::stdlib::uInt).wrapping_add(avail_in))
}

fn stored_insert_after_input(
    insert: crate::stdlib::uInt,
    window_size: crate::stdlib::uInt,
    input_len: ::core::ffi::c_uint,
) -> crate::stdlib::uInt {
    insert.wrapping_add(if input_len > window_size.wrapping_sub(insert) {
        (window_size as ::core::ffi::c_uint).wrapping_sub(insert as ::core::ffi::c_uint)
    } else {
        input_len
    })
}

unsafe extern "C" fn deflate_stored(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut min_block: ::core::ffi::c_uint =
        stored_block_min_size((*s).pending_buf_size, (*s).w_size);
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used: ::core::ffi::c_uint = (*(*s).strm).avail_in as ::core::ffi::c_uint;
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        let Some(available_output) =
            stored_block_available_output((*s).bi_valid, (*(*s).strm).avail_out)
        else {
            break;
        };
        have = available_output;
        left = ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as ::core::ffi::c_uint;
        if len as crate::zutil_h::ulg
            > (left as crate::zutil_h::ulg)
                .wrapping_add((*(*s).strm).avail_in as crate::zutil_h::ulg)
        {
            len = (left as crate::stdlib::uInt).wrapping_add((*(*s).strm).avail_in)
                as ::core::ffi::c_uint;
        }
        if len > have {
            len = have;
        }
        if stored_block_should_wait(len, min_block, left, (*(*s).strm).avail_in, flush) {
            break;
        }
        last = if flush == crate::zlib_h::Z_FINISH
            && len == (left as crate::stdlib::uInt).wrapping_add((*(*s).strm).avail_in)
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
        *(*s)
            .pending_buf
            .offset((*s).pending.wrapping_sub(4 as crate::zutil_h::ulg) as isize) =
            len as crate::stdlib::Bytef;
        *(*s)
            .pending_buf
            .offset((*s).pending.wrapping_sub(3 as crate::zutil_h::ulg) as isize) =
            (len >> 8 as ::core::ffi::c_int) as crate::stdlib::Bytef;
        *(*s)
            .pending_buf
            .offset((*s).pending.wrapping_sub(2 as crate::zutil_h::ulg) as isize) =
            !len as crate::stdlib::Bytef;
        *(*s)
            .pending_buf
            .offset((*s).pending.wrapping_sub(1 as crate::zutil_h::ulg) as isize) =
            (!len >> 8 as ::core::ffi::c_int) as crate::stdlib::Bytef;
        flush_pending((*s).strm);
        if left != 0 {
            if left > len {
                left = len;
            }
            crate::stdlib::memcpy(
                (*(*s).strm).next_out as *mut ::core::ffi::c_void,
                (*s).window.offset((*s).block_start as isize) as *const ::core::ffi::c_void,
                left as crate::__stddef_size_t_h::size_t,
            );
            (*(*s).strm).next_out = (*(*s).strm).next_out.offset(left as isize);
            (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(left);
            (*(*s).strm).total_out = (*(*s).strm)
                .total_out
                .wrapping_add(left as crate::stdlib::uLong);
            (*s).block_start += left as ::core::ffi::c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            read_buf((*s).strm, (*(*s).strm).next_out, len);
            (*(*s).strm).next_out = (*(*s).strm).next_out.offset(len as isize);
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
            (*s).insert = stored_insert_after_input((*s).insert, (*s).w_size, used);
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
        read_buf(
            (*s).strm,
            (*s).window.wrapping_add((*s).strstart as usize),
            have,
        );
        (*s).strstart = (*s).strstart.wrapping_add(have);
        (*s).insert = stored_insert_after_input((*s).insert, (*s).w_size, have);
    }
    if (*s).high_water < (*s).strstart as crate::zutil_h::ulg {
        (*s).high_water = (*s).strstart as crate::zutil_h::ulg;
    }
    have = ((*s).bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
        >> 3 as ::core::ffi::c_int;
    have = (if (*s)
        .pending_buf_size
        .wrapping_sub(have as crate::zutil_h::ulg)
        > 65535 as crate::zutil_h::ulg
    {
        65535 as crate::zutil_h::ulg
    } else {
        (*s).pending_buf_size
            .wrapping_sub(have as crate::zutil_h::ulg)
    }) as ::core::ffi::c_uint;
    min_block = if have > (*s).w_size {
        (*s).w_size as ::core::ffi::c_uint
    } else {
        have
    };
    left = ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as ::core::ffi::c_uint;
    if left >= min_block
        || (left != 0 || flush == crate::zlib_h::Z_FINISH)
            && flush != crate::zlib_h::Z_NO_FLUSH
            && (*(*s).strm).avail_in == 0 as crate::stdlib::uInt
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == crate::zlib_h::Z_FINISH
            && (*(*s).strm).avail_in == 0 as crate::stdlib::uInt
            && len == left
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        crate::src::trees::_tr_stored_block(
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
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset((*s).strstart.wrapping_add(
                    (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
                ) as isize) as crate::stdlib::uInt)
                & (*s).hash_mask;
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
            let c2rust_fresh47 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh47 as isize) =
                dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh48 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh48 as isize) =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh49 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh49 as isize) = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*(&raw const crate::src::trees::_length_code
                as *const crate::zutil_h::uch)
                .offset(len as isize) as ::core::ffi::c_int
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .value = (*s).dyn_ltree[(*(&raw const crate::src::trees::_length_code
                as *const crate::zutil_h::uch)
                .offset(len as isize)
                as ::core::ffi::c_int
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .value
                .wrapping_add(1);
            (*s).dyn_dtree[(if (dist as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                    .offset(dist as isize) as ::core::ffi::c_int
            } else {
                *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch).offset(
                    (256 as ::core::ffi::c_int
                        + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                        as isize,
                ) as ::core::ffi::c_int
            }) as usize]
                .fc
                .value =
                (*s).dyn_dtree[(if (dist as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                    *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                        .offset(dist as isize) as ::core::ffi::c_int
                } else {
                    *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                        .offset(
                            (256 as ::core::ffi::c_int
                                + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                                as isize,
                        ) as ::core::ffi::c_int
                }) as usize]
                    .fc
                    .value
                    .wrapping_add(1);
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
            let (cursors, next) = symbol_triplet_cursors((*s).sym_next);
            (*s).sym_next = next;
            *(*s).sym_buf.offset(cursors[0] as isize) = 0 as crate::zutil_h::uchf;
            *(*s).sym_buf.offset(cursors[1] as isize) = 0 as crate::zutil_h::uchf;
            *(*s).sym_buf.offset(cursors[2] as isize) = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc.value =
                (*s).dyn_ltree[cc as usize].fc.value.wrapping_add(1);
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
        if (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset((*s).strstart.wrapping_add(
                    (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
                ) as isize) as crate::stdlib::uInt)
                & (*s).hash_mask;
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
            let c2rust_fresh36 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh36 as isize) =
                dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh37 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh37 as isize) =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh38 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh38 as isize) = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*(&raw const crate::src::trees::_length_code
                as *const crate::zutil_h::uch)
                .offset(len as isize) as ::core::ffi::c_int
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .value = (*s).dyn_ltree[(*(&raw const crate::src::trees::_length_code
                as *const crate::zutil_h::uch)
                .offset(len as isize)
                as ::core::ffi::c_int
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .value
                .wrapping_add(1);
            (*s).dyn_dtree[(if (dist as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                    .offset(dist as isize) as ::core::ffi::c_int
            } else {
                *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch).offset(
                    (256 as ::core::ffi::c_int
                        + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                        as isize,
                ) as ::core::ffi::c_int
            }) as usize]
                .fc
                .value =
                (*s).dyn_dtree[(if (dist as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                    *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                        .offset(dist as isize) as ::core::ffi::c_int
                } else {
                    *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                        .offset(
                            (256 as ::core::ffi::c_int
                                + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                                as isize,
                        ) as ::core::ffi::c_int
                }) as usize]
                    .fc
                    .value
                    .wrapping_add(1);
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
                crate::src::trees::_tr_flush_block(
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
            let c2rust_fresh40 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh40 as isize) = 0 as crate::zutil_h::uchf;
            let c2rust_fresh41 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh41 as isize) = 0 as crate::zutil_h::uchf;
            let c2rust_fresh42 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh42 as isize) = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc.value =
                (*s).dyn_ltree[cc as usize].fc.value.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            if bflush != 0 {
                crate::src::trees::_tr_flush_block(
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
        let c2rust_fresh43 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        *(*s).sym_buf.offset(c2rust_fresh43 as isize) = 0 as crate::zutil_h::uchf;
        let c2rust_fresh44 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        *(*s).sym_buf.offset(c2rust_fresh44 as isize) = 0 as crate::zutil_h::uchf;
        let c2rust_fresh45 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        *(*s).sym_buf.offset(c2rust_fresh45 as isize) = cc_0 as crate::zutil_h::uchf;
        (*s).dyn_ltree[cc_0 as usize].fc.value =
            (*s).dyn_ltree[cc_0 as usize].fc.value.wrapping_add(1);
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
    let mut prev: crate::stdlib::uInt = 0;
    let mut scan: *mut crate::stdlib::Bytef = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
    let mut strend: *mut crate::stdlib::Bytef = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
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
        if (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            && (*s).strstart > 0 as crate::stdlib::uInt
        {
            scan = (*s)
                .window
                .wrapping_offset((*s).strstart as isize)
                .wrapping_offset(-(1 as ::core::ffi::c_int as isize));
            prev = *scan as crate::stdlib::uInt;
            scan = scan.wrapping_add(1);
            if prev == *scan as crate::stdlib::uInt
                && {
                    scan = scan.wrapping_add(1);
                    prev == *scan as crate::stdlib::uInt
                }
                && {
                    scan = scan.wrapping_add(1);
                    prev == *scan as crate::stdlib::uInt
                }
            {
                strend = (*s)
                    .window
                    .wrapping_offset((*s).strstart as isize)
                    .wrapping_offset(crate::zutil_h::MAX_MATCH as isize);
                loop {
                    scan = scan.wrapping_add(1);
                    if !(prev == *scan as crate::stdlib::uInt
                        && {
                            scan = scan.wrapping_add(1);
                            prev == *scan as crate::stdlib::uInt
                        }
                        && {
                            scan = scan.wrapping_add(1);
                            prev == *scan as crate::stdlib::uInt
                        }
                        && {
                            scan = scan.wrapping_add(1);
                            prev == *scan as crate::stdlib::uInt
                        }
                        && {
                            scan = scan.wrapping_add(1);
                            prev == *scan as crate::stdlib::uInt
                        }
                        && {
                            scan = scan.wrapping_add(1);
                            prev == *scan as crate::stdlib::uInt
                        }
                        && {
                            scan = scan.wrapping_add(1);
                            prev == *scan as crate::stdlib::uInt
                        }
                        && {
                            scan = scan.wrapping_add(1);
                            prev == *scan as crate::stdlib::uInt
                        }
                        && scan < strend)
                    {
                        break;
                    }
                }
                (*s).match_length = (crate::zutil_h::MAX_MATCH as crate::stdlib::uInt)
                    .wrapping_sub(
                        strend.offset_from(scan) as ::core::ffi::c_long as crate::stdlib::uInt
                    );
                if (*s).match_length > (*s).lookahead {
                    (*s).match_length = (*s).lookahead;
                }
            }
        }
        if (*s).match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut len: crate::zutil_h::uch =
                (*s).match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let mut dist: crate::zutil_h::ush = 1 as ::core::ffi::c_int as crate::zutil_h::ush;
            let (cursors, next) = symbol_triplet_cursors((*s).sym_next);
            (*s).sym_next = next;
            *(*s).sym_buf.offset(cursors[0] as isize) =
                dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            *(*s).sym_buf.offset(cursors[1] as isize) =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            *(*s).sym_buf.offset(cursors[2] as isize) = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*(&raw const crate::src::trees::_length_code
                as *const crate::zutil_h::uch)
                .offset(len as isize) as ::core::ffi::c_int
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .value = (*s).dyn_ltree[(*(&raw const crate::src::trees::_length_code
                as *const crate::zutil_h::uch)
                .offset(len as isize)
                as ::core::ffi::c_int
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .value
                .wrapping_add(1);
            (*s).dyn_dtree[(if (dist as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                    .offset(dist as isize) as ::core::ffi::c_int
            } else {
                *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch).offset(
                    (256 as ::core::ffi::c_int
                        + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                        as isize,
                ) as ::core::ffi::c_int
            }) as usize]
                .fc
                .value =
                (*s).dyn_dtree[(if (dist as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                    *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                        .offset(dist as isize) as ::core::ffi::c_int
                } else {
                    *(&raw const crate::src::trees::_dist_code as *const crate::zutil_h::uch)
                        .offset(
                            (256 as ::core::ffi::c_int
                                + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                                as isize,
                        ) as ::core::ffi::c_int
                }) as usize]
                    .fc
                    .value
                    .wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub((*s).match_length);
            (*s).strstart = (*s).strstart.wrapping_add((*s).match_length);
            (*s).match_length = 0 as crate::stdlib::uInt;
        } else {
            let mut cc: crate::zutil_h::uch =
                *(*s).window.offset((*s).strstart as isize) as crate::zutil_h::uch;
            let (cursors, next) = symbol_triplet_cursors((*s).sym_next);
            (*s).sym_next = next;
            *(*s).sym_buf.offset(cursors[0] as isize) = 0 as crate::zutil_h::uchf;
            *(*s).sym_buf.offset(cursors[1] as isize) = 0 as crate::zutil_h::uchf;
            *(*s).sym_buf.offset(cursors[2] as isize) = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc.value =
                (*s).dyn_ltree[cc as usize].fc.value.wrapping_add(1);
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
        (*s).match_length = 0 as crate::stdlib::uInt;
        let mut cc: crate::zutil_h::uch =
            *(*s).window.offset((*s).strstart as isize) as crate::zutil_h::uch;
        let (cursors, next) = symbol_triplet_cursors((*s).sym_next);
        (*s).sym_next = next;
        *(*s).sym_buf.offset(cursors[0] as isize) = 0 as crate::zutil_h::uchf;
        *(*s).sym_buf.offset(cursors[1] as isize) = 0 as crate::zutil_h::uchf;
        *(*s).sym_buf.offset(cursors[2] as isize) = cc as crate::zutil_h::uchf;
        (*s).dyn_ltree[cc as usize].fc.value = (*s).dyn_ltree[cc as usize].fc.value.wrapping_add(1);
        bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
        (*s).lookahead = (*s).lookahead.wrapping_sub(1);
        (*s).strstart = (*s).strstart.wrapping_add(1);
        if bflush != 0 {
            crate::src::trees::_tr_flush_block(
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
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

#[cfg(test)]
mod tests {
    use super::{
        clamped_copy_len, deflate_bound_lengths, deflate_copyright, deflate_dictionary_len,
        deflate_pending_value, deflate_prime_bits_valid, deflate_request_is_invalid,
        deflate_should_return_buf_error, deflate_state_status_valid, deflate_version_matches,
        fill_window_available_space, fill_window_cursor, fill_window_insert_after_slide,
        fill_window_zero_range, flush_pending_accounting, gzip_default_xfl, gzip_header_crc,
        gzip_header_crc_pending, gzip_header_crc_pending_range, normalize_deflate_params,
        pending_output_len, pending_short_cursors, read_buf_len, read_buf_total_in_after_copy,
        short_msb_bytes, slide_hash_entry, stored_block_available_output, stored_block_min_size,
        stored_block_should_wait, stored_insert_after_input, symbol_triplet_cursors, zlib_header,
    };

    #[test]
    fn symbol_triplet_cursors_preserve_record_order_and_wrapping() {
        assert_eq!(symbol_triplet_cursors(7), ([7, 8, 9], 10));
        assert_eq!(
            symbol_triplet_cursors(crate::stdlib::uInt::MAX - 1),
            (
                [crate::stdlib::uInt::MAX - 1, crate::stdlib::uInt::MAX, 0],
                1
            )
        );
    }

    #[test]
    fn normalize_deflate_params_maps_default_level_to_six() {
        assert_eq!(
            normalize_deflate_params(
                crate::zlib_h::Z_DEFAULT_COMPRESSION,
                crate::zlib_h::Z_DEFAULT_STRATEGY,
            ),
            Some((6, crate::zlib_h::Z_DEFAULT_STRATEGY)),
        );
    }

    #[test]
    fn deflate_dictionary_len_preserves_wrapping_clamp_behavior() {
        assert_eq!(deflate_dictionary_len(0, 0, 32), 0);
        assert_eq!(deflate_dictionary_len(10, 5, 32), 15);
        assert_eq!(deflate_dictionary_len(27, 5, 32), 32);
        assert_eq!(deflate_dictionary_len(30, 5, 32), 32);
        assert_eq!(deflate_dictionary_len(crate::stdlib::uInt::MAX, 1, 32), 0);
    }

    #[test]
    fn deflate_pending_value_accepts_uint_and_rejects_truncation() {
        assert_eq!(deflate_pending_value(0), Ok(0));
        assert_eq!(
            deflate_pending_value(::core::ffi::c_uint::MAX as crate::zutil_h::ulg),
            Ok(::core::ffi::c_uint::MAX),
        );
        assert_eq!(
            deflate_pending_value((::core::ffi::c_uint::MAX as crate::zutil_h::ulg) + 1),
            Err(()),
        );
    }

    #[test]
    fn short_msb_bytes_keeps_the_low_sixteen_bits_in_network_order() {
        assert_eq!(short_msb_bytes(0), [0, 0]);
        assert_eq!(short_msb_bytes(0x1234), [0x12, 0x34]);
        assert_eq!(short_msb_bytes(0xabcd_1234), [0x12, 0x34]);
    }

    #[test]
    fn pending_short_cursors_preserve_order_and_wrapping_accounting() {
        assert_eq!(pending_short_cursors(7), ([7, 8], 9));
        assert_eq!(
            pending_short_cursors(crate::zutil_h::ulg::MAX),
            ([crate::zutil_h::ulg::MAX, 0], 1)
        );
    }

    #[test]
    fn normalize_deflate_params_accepts_level_and_strategy_endpoints() {
        assert_eq!(normalize_deflate_params(0, 0), Some((0, 0)));
        assert_eq!(
            normalize_deflate_params(9, crate::zlib_h::Z_FIXED),
            Some((9, crate::zlib_h::Z_FIXED)),
        );
    }

    #[test]
    fn normalize_deflate_params_rejects_out_of_range_levels() {
        assert_eq!(normalize_deflate_params(-2, 0), None);
        assert_eq!(normalize_deflate_params(10, 0), None);
    }

    #[test]
    fn normalize_deflate_params_rejects_strategy_past_z_fixed() {
        assert_eq!(
            normalize_deflate_params(6, crate::zlib_h::Z_FIXED + 1),
            None,
        );
    }

    #[test]
    fn copyright_export_has_stable_bytes() {
        assert_eq!(
            deflate_copyright,
            *b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0"
        );
    }

    #[test]
    fn gzip_header_crc_matches_fixed_header() {
        assert_eq!(
            gzip_header_crc(0, 0, 1, false, false, false, 0, 6, 0, 3, 0),
            0x0c5c_77a7,
        );
    }

    #[test]
    fn gzip_header_crc_includes_xlen_and_flags() {
        assert_eq!(
            gzip_header_crc(0, 1, 1, true, true, true, 0x1234_5678, 9, 0, 255, 0x1234),
            0xd74e_6245,
        );
    }

    #[test]
    fn gzip_header_crc_pending_preserves_partial_header_ranges() {
        let pending = *b"extra-name-comment";
        let crc = gzip_header_crc_pending(0, 1, &pending, 0, 5);
        let crc = gzip_header_crc_pending(crc, 1, &pending, 5, 10);
        let crc = gzip_header_crc_pending(crc, 1, &pending, 10, pending.len() as _);

        assert_eq!(crc, crate::src::crc32::crc32_z(0, &pending));
        assert_eq!(gzip_header_crc_pending(crc, 0, &pending, 0, 5), crc);
        assert_eq!(gzip_header_crc_pending(crc, 1, &pending, 5, 5), crc);
    }

    #[test]
    fn gzip_header_crc_pending_range_requires_enabled_ordered_indices() {
        assert_eq!(gzip_header_crc_pending_range(1, 2, 5), Some(2..5));
        assert_eq!(gzip_header_crc_pending_range(0, 2, 5), None);
        assert_eq!(gzip_header_crc_pending_range(1, 5, 5), None);
        assert_eq!(gzip_header_crc_pending_range(1, 5, 2), None);
    }

    #[test]
    fn zlib_header_uses_level_strategy_and_dictionary_flags() {
        let w_bits = 15 as crate::stdlib::uInt;

        assert_eq!(zlib_header(w_bits, 0, 1, false), 0x7801);
        assert_eq!(zlib_header(w_bits, 0, 2, false), 0x785e);
        assert_eq!(zlib_header(w_bits, 0, 6, false), 0x789c);
        assert_eq!(zlib_header(w_bits, 0, 7, false), 0x78da);
        assert_eq!(zlib_header(w_bits, 0, 6, true), 0x78bb);
        assert_eq!(
            zlib_header(w_bits, crate::zlib_h::Z_HUFFMAN_ONLY, 9, false),
            0x7801
        );
    }

    #[test]
    fn gzip_default_xfl_prioritizes_level_then_strategy_and_speed() {
        assert_eq!(gzip_default_xfl(9, crate::zlib_h::Z_DEFAULT_STRATEGY), 2);
        assert_eq!(gzip_default_xfl(9, 2), 2);
        assert_eq!(gzip_default_xfl(0, crate::zlib_h::Z_DEFAULT_STRATEGY), 4);
        assert_eq!(gzip_default_xfl(1, crate::zlib_h::Z_DEFAULT_STRATEGY), 4);
        assert_eq!(gzip_default_xfl(2, crate::zlib_h::Z_DEFAULT_STRATEGY), 0);
        assert_eq!(gzip_default_xfl(8, crate::zlib_h::Z_DEFAULT_STRATEGY), 0);
        assert_eq!(gzip_default_xfl(6, 2), 4);
    }

    #[test]
    fn pending_output_len_uses_available_output_capacity() {
        assert_eq!(pending_output_len(0, 0), 0);
        assert_eq!(pending_output_len(3, 5), 3);
        assert_eq!(pending_output_len(5, 5), 5);
        assert_eq!(pending_output_len(8, 5), 5);
    }

    #[test]
    fn flush_pending_accounting_preserves_copy_and_reset_transitions() {
        assert_eq!(flush_pending_accounting(0, 5, 12), None);
        assert_eq!(
            flush_pending_accounting(8, 3, 12),
            Some((3, 5, 0, 15, false))
        );
        assert_eq!(
            flush_pending_accounting(3, 8, crate::stdlib::uLong::MAX),
            Some((3, 0, 5, 2, true))
        );
    }

    #[test]
    fn deflate_prime_bits_valid_accepts_the_supported_inclusive_range() {
        assert!(!deflate_prime_bits_valid(-1));
        assert!(deflate_prime_bits_valid(0));
        assert!(deflate_prime_bits_valid(16));
        assert!(!deflate_prime_bits_valid(17));
    }

    #[test]
    fn slide_hash_entry_rebases_or_clears_positions() {
        let window_size = 32 as crate::stdlib::uInt;

        assert_eq!(slide_hash_entry(0, window_size), 0);
        assert_eq!(slide_hash_entry(31, window_size), 0);
        assert_eq!(slide_hash_entry(32, window_size), 0);
        assert_eq!(slide_hash_entry(47, window_size), 15);
    }

    #[test]
    fn read_buf_len_clamps_to_requested_input() {
        assert_eq!(read_buf_len(0, 0), 0);
        assert_eq!(read_buf_len(0, 8), 0);
        assert_eq!(read_buf_len(7, 8), 7);
        assert_eq!(read_buf_len(8, 8), 8);
        assert_eq!(read_buf_len(9, 8), 8);
        assert_eq!(
            read_buf_len(::core::ffi::c_uint::MAX, ::core::ffi::c_uint::MAX - 1),
            ::core::ffi::c_uint::MAX - 1,
        );
    }

    #[test]
    fn read_buf_total_in_after_copy_preserves_wrapping_accounting() {
        assert_eq!(read_buf_total_in_after_copy(0, 0), 0);
        assert_eq!(read_buf_total_in_after_copy(12, 20), 32);
        assert_eq!(
            read_buf_total_in_after_copy(crate::stdlib::uLong::MAX, 1),
            0,
        );
    }

    #[test]
    fn clamped_copy_len_returns_the_smaller_scalar_input() {
        assert_eq!(clamped_copy_len(0, 0), 0);
        assert_eq!(clamped_copy_len(3, 8), 3);
        assert_eq!(clamped_copy_len(8, 3), 3);
        assert_eq!(
            clamped_copy_len(
                ::core::ffi::c_uint::MAX as crate::zutil_h::ulg,
                (::core::ffi::c_uint::MAX - 1) as crate::zutil_h::ulg,
            ),
            ::core::ffi::c_uint::MAX - 1,
        );
    }

    #[test]
    fn symbol_triplet_cursors_preserve_order_and_wrapping() {
        assert_eq!(symbol_triplet_cursors(4), ([4, 5, 6], 7));
        assert_eq!(
            symbol_triplet_cursors(::core::ffi::c_uint::MAX - 1),
            (
                [::core::ffi::c_uint::MAX - 1, ::core::ffi::c_uint::MAX, 0],
                1
            )
        );
    }

    #[test]
    fn fill_window_available_space_preserves_normal_and_narrow_int_cases() {
        assert_eq!(fill_window_available_space(64, 10, 20, 32, false), 34);
        assert_eq!(fill_window_available_space(0, 0, 0, 32, false), 0);
        assert_eq!(fill_window_available_space(0, 0, 0, 32, true), 32);
        assert_eq!(
            fill_window_available_space(0, 1, 0, 32, true),
            ::core::ffi::c_uint::MAX - 1,
        );
    }

    #[test]
    fn fill_window_insert_after_slide_clamps_only_past_the_cursor() {
        assert_eq!(fill_window_insert_after_slide(0, 0), 0);
        assert_eq!(fill_window_insert_after_slide(12, 12), 12);
        assert_eq!(fill_window_insert_after_slide(11, 12), 11);
        assert_eq!(fill_window_insert_after_slide(13, 12), 12);
        assert_eq!(
            fill_window_insert_after_slide(crate::stdlib::uInt::MAX, 42),
            42,
        );
    }

    #[test]
    fn fill_window_cursor_uses_wide_wrapping_arithmetic() {
        assert_eq!(fill_window_cursor(0, 0), 0);
        assert_eq!(fill_window_cursor(12, 20), 32);
        assert_eq!(
            fill_window_cursor(crate::stdlib::uInt::MAX, 1),
            (crate::stdlib::uInt::MAX as crate::zutil_h::ulg) + 1,
        );
    }

    #[test]
    fn fill_window_zero_range_preserves_high_water_initialization_bounds() {
        assert_eq!(fill_window_zero_range(64, 64, 0, 0), None);
        assert_eq!(fill_window_zero_range(20, 1024, 100, 0), Some((100, 258)));
        assert_eq!(
            fill_window_zero_range(1000, 1024, 1000, 0),
            Some((1000, 24))
        );
        assert_eq!(fill_window_zero_range(300, 1024, 0, 0), None);
    }

    #[test]
    fn deflate_bound_lengths_match_the_translated_formulas() {
        assert_eq!(deflate_bound_lengths(0), (4, 7));
        assert_eq!(deflate_bound_lengths(1), (5, 8));
        assert_eq!(deflate_bound_lengths(1024), (1162, 1071));
    }

    #[test]
    fn deflate_bound_lengths_saturate_on_wrapping_overflow() {
        let max = crate::stdlib::z_size_t::MAX;

        assert_eq!(deflate_bound_lengths(max), (max, max));
    }

    #[test]
    fn stored_block_min_size_caps_pending_capacity_at_window_size() {
        assert_eq!(stored_block_min_size(21, 32), 16);
        assert_eq!(stored_block_min_size(37, 32), 32);
        assert_eq!(stored_block_min_size(1024, 32), 32);
    }

    #[test]
    fn stored_block_min_size_preserves_wrapping_underflow_behavior() {
        assert_eq!(stored_block_min_size(4, 32), 32);
        assert_eq!(stored_block_min_size(0, 32), 32);
    }

    #[test]
    fn stored_block_available_output_requires_space_for_header_bits() {
        assert_eq!(stored_block_available_output(0, 4), None);
        assert_eq!(stored_block_available_output(0, 5), Some(0));
        assert_eq!(stored_block_available_output(7, 5), None);
        assert_eq!(stored_block_available_output(7, 9), Some(3));
        assert_eq!(stored_block_available_output(-1, 5), Some(0));
    }

    #[test]
    fn stored_block_should_wait_preserves_small_block_flush_rules() {
        assert!(!stored_block_should_wait(
            8,
            8,
            8,
            0,
            crate::zlib_h::Z_NO_FLUSH,
        ));
        assert!(stored_block_should_wait(
            4,
            8,
            2,
            2,
            crate::zlib_h::Z_NO_FLUSH,
        ));
        assert!(stored_block_should_wait(
            4,
            8,
            2,
            3,
            crate::zlib_h::Z_FINISH,
        ));
        assert!(!stored_block_should_wait(
            4,
            8,
            2,
            2,
            crate::zlib_h::Z_FINISH,
        ));
        assert!(stored_block_should_wait(
            0,
            8,
            0,
            0,
            crate::zlib_h::Z_FULL_FLUSH,
        ));
        assert!(!stored_block_should_wait(
            0,
            8,
            0,
            0,
            crate::zlib_h::Z_FINISH,
        ));
    }

    #[test]
    fn deflate_should_return_buf_error_preserves_flush_rank_semantics() {
        assert!(!deflate_should_return_buf_error(
            1,
            crate::zlib_h::Z_NO_FLUSH,
            crate::zlib_h::Z_BLOCK,
        ));
        assert!(deflate_should_return_buf_error(
            0,
            crate::zlib_h::Z_NO_FLUSH,
            crate::zlib_h::Z_NO_FLUSH,
        ));
        assert!(deflate_should_return_buf_error(
            0,
            crate::zlib_h::Z_BLOCK,
            crate::zlib_h::Z_FULL_FLUSH,
        ));
        assert!(!deflate_should_return_buf_error(
            0,
            crate::zlib_h::Z_FULL_FLUSH,
            crate::zlib_h::Z_BLOCK,
        ));
        assert!(!deflate_should_return_buf_error(
            0,
            crate::zlib_h::Z_FINISH,
            crate::zlib_h::Z_FINISH,
        ));
    }

    #[test]
    fn deflate_request_is_invalid_preserves_pointer_and_finish_rules() {
        assert!(deflate_request_is_invalid(
            true,
            0,
            false,
            0,
            crate::zlib_h::Z_NO_FLUSH
        ));
        assert!(deflate_request_is_invalid(
            false,
            1,
            true,
            0,
            crate::zlib_h::Z_NO_FLUSH
        ));
        assert!(!deflate_request_is_invalid(
            false,
            0,
            true,
            0,
            crate::zlib_h::Z_NO_FLUSH
        ));
        assert!(deflate_request_is_invalid(
            false,
            0,
            false,
            crate::src::deflate::FINISH_STATE,
            crate::zlib_h::Z_NO_FLUSH,
        ));
        assert!(!deflate_request_is_invalid(
            false,
            0,
            false,
            crate::src::deflate::FINISH_STATE,
            crate::zlib_h::Z_FINISH,
        ));
    }

    #[test]
    fn stored_insert_after_input_preserves_clamping_and_wrapping() {
        assert_eq!(stored_insert_after_input(3, 10, 4), 7);
        assert_eq!(stored_insert_after_input(3, 10, 7), 10);
        assert_eq!(stored_insert_after_input(3, 10, 8), 10);
        assert_eq!(stored_insert_after_input(12, 10, 1), 13);
        assert_eq!(
            stored_insert_after_input(12, 10, ::core::ffi::c_uint::MAX),
            10,
        );
        assert_eq!(stored_insert_after_input(crate::stdlib::uInt::MAX, 8, 1), 0,);
    }

    #[test]
    fn deflate_version_matches_only_the_first_version_byte_and_stream_size() {
        let expected_size = ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int;

        assert!(deflate_version_matches(
            b'1' as ::core::ffi::c_char,
            expected_size
        ));
        assert!(!deflate_version_matches(
            b'2' as ::core::ffi::c_char,
            expected_size
        ));
        assert!(!deflate_version_matches(
            b'1' as ::core::ffi::c_char,
            expected_size.wrapping_add(1),
        ));
    }

    #[test]
    fn deflate_state_status_valid_accepts_only_deflate_states() {
        for status in [
            crate::src::deflate::INIT_STATE,
            crate::src::deflate::GZIP_STATE,
            crate::src::deflate::EXTRA_STATE,
            crate::src::deflate::NAME_STATE,
            crate::src::deflate::COMMENT_STATE,
            crate::src::deflate::HCRC_STATE,
            crate::src::deflate::BUSY_STATE,
            crate::src::deflate::FINISH_STATE,
        ] {
            assert!(deflate_state_status_valid(status));
        }

        assert!(!deflate_state_status_valid(0));
        assert!(!deflate_state_status_valid(-1));
        assert!(!deflate_state_status_valid(
            crate::src::deflate::FINISH_STATE - 1
        ));
    }
}
