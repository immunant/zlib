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
#[repr(transparent)]
pub struct C2Rust_Unnamed_1 {
    // `freq` and `code` are C union aliases with the same type.  Keeping one
    // storage field preserves the representation without requiring unsafe
    // union-field reads.
    pub freq: crate::zutil_h::ush,
}
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0 {
    // `dad` and `len` likewise occupy the same `ush` storage in C.
    pub dad: crate::zutil_h::ush,
}

pub type static_tree_desc = crate::src::deflate::static_tree_desc_s;

pub type tree_desc = crate::src::deflate::tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tree_desc_s {
    pub max_code: ::core::ffi::c_int,
    pub stat_desc: &'static crate::src::deflate::static_tree_desc,
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

pub static mut deflate_copyright: [::core::ffi::c_char; 70] = unsafe {
    ::core::mem::transmute::<[u8; 70], [::core::ffi::c_char; 70]>(
        *b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0",
    )
};

pub const NIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TOO_FAR: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

static mut configuration_table: [config; 10] = [
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

unsafe extern "C" fn slide_hash(mut s: *mut crate::src::deflate::deflate_state) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut m: ::core::ffi::c_uint = 0;
    let mut p: *mut crate::src::deflate::Posf =
        ::core::ptr::null_mut::<crate::src::deflate::Posf>();
    let mut wsize: crate::stdlib::uInt = (*s).w_size;
    n = (*s).hash_size as ::core::ffi::c_uint;
    p = (*s).head.offset(n as isize);
    loop {
        p = p.offset(-1);
        m = *p as ::core::ffi::c_uint;
        *p = (if m >= wsize {
            m.wrapping_sub(wsize as ::core::ffi::c_uint)
        } else {
            NIL as ::core::ffi::c_uint
        }) as crate::src::deflate::Pos as crate::src::deflate::Posf;
        n = n.wrapping_sub(1);
        if n == 0 {
            break;
        }
    }
    n = wsize as ::core::ffi::c_uint;
    p = (*s).prev.offset(n as isize);
    loop {
        p = p.offset(-1);
        m = *p as ::core::ffi::c_uint;
        *p = (if m >= wsize {
            m.wrapping_sub(wsize as ::core::ffi::c_uint)
        } else {
            NIL as ::core::ffi::c_uint
        }) as crate::src::deflate::Pos as crate::src::deflate::Posf;
        n = n.wrapping_sub(1);
        if n == 0 {
            break;
        }
    }
    (*s).slid = 1 as ::core::ffi::c_int;
}

unsafe extern "C" fn read_buf(
    mut strm: crate::zlib_h::z_streamp,
    mut buf: *mut crate::stdlib::Bytef,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut len: ::core::ffi::c_uint = (*strm).avail_in as ::core::ffi::c_uint;
    if len > size {
        len = size;
    }
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
            crate::src::adler32::adler32((*strm).adler, buf, len as crate::stdlib::uInt);
    } else if (*(*strm).state).wrap == 2 as ::core::ffi::c_int {
        (*strm).adler = crate::src::crc32::crc32((*strm).adler, buf, len as crate::stdlib::uInt);
    }
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in = (*strm).total_in.wrapping_add(len as crate::stdlib::uLong);
    return len;
}

unsafe extern "C" fn fill_window(mut s: *mut crate::src::deflate::deflate_state) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    let mut wsize: crate::stdlib::uInt = (*s).w_size;
    loop {
        more = (*s)
            .window_size
            .wrapping_sub((*s).lookahead as crate::zutil_h::ulg)
            .wrapping_sub((*s).strstart as crate::zutil_h::ulg)
            as ::core::ffi::c_uint;
        if ::core::mem::size_of::<::core::ffi::c_int>() <= 2 as usize {
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
            crate::stdlib::memcpy(
                (*s).window as *mut ::core::ffi::c_void,
                (*s).window.offset(wsize as isize) as *const ::core::ffi::c_void,
                wsize.wrapping_sub(more) as crate::__stddef_size_t_h::size_t,
            );
            (*s).match_start = (*s).match_start.wrapping_sub(wsize);
            (*s).strstart = (*s).strstart.wrapping_sub(wsize);
            (*s).block_start -= wsize as ::core::ffi::c_long;
            if (*s).insert > (*s).strstart {
                (*s).insert = (*s).strstart;
            }
            slide_hash(s);
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if (*(*s).strm).avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        n = read_buf(
            (*s).strm,
            (*s).window
                .offset((*s).strstart as isize)
                .offset((*s).lookahead as isize),
            more,
        );
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
                (*s).window.offset(curr as isize) as *mut ::core::ffi::c_void,
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
                (*s).window.offset((*s).high_water as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                init as ::core::ffi::c_uint as crate::__stddef_size_t_h::size_t,
            );
            (*s).high_water = (*s).high_water.wrapping_add(init);
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
    static mut my_version: [::core::ffi::c_char; 15] = crate::zlib_h::ZLIB_VERSION;
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
        (*strm).adler = crate::src::adler32::adler32((*strm).adler, dictionary, dictLength);
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
                    .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Posf>()),
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
    let s = &mut *s;
    s.pending = 0 as crate::zutil_h::ulg;
    s.pending_out = s.pending_buf;
    if s.wrap < 0 as ::core::ffi::c_int {
        s.wrap = -s.wrap;
    }
    s.status = if s.wrap == 2 as ::core::ffi::c_int {
        crate::src::deflate::GZIP_STATE
    } else {
        crate::src::deflate::INIT_STATE
    };
    (*strm).adler = if s.wrap == 2 as ::core::ffi::c_int {
        crate::src::crc32::crc32(
            0 as crate::stdlib::uLong,
            ::core::ptr::null::<crate::stdlib::Bytef>(),
            0 as crate::stdlib::uInt,
        )
    } else {
        crate::src::adler32::adler32(
            0 as crate::stdlib::uLong,
            ::core::ptr::null::<crate::stdlib::Bytef>(),
            0 as crate::stdlib::uInt,
        )
    };
    s.last_flush = -2 as ::core::ffi::c_int;
    crate::src::trees::_tr_init(s);
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
            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Posf>()),
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
pub unsafe extern "C" fn deflatePending(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !bits.is_null() {
        *bits = (*(*strm).state).bi_valid;
    }
    if !pending.is_null() {
        *pending = (*(*strm).state).pending as ::core::ffi::c_uint;
        if *pending as crate::zutil_h::ulg != (*(*strm).state).pending {
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
    deflatePending(strm, pending, bits)
}
pub unsafe extern "C" fn deflateUsed(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !bits.is_null() {
        *bits = (*(*strm).state).bi_used;
    }
    return crate::zlib_h::Z_OK;
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
        || (*s).sym_buf
            < (*s).pending_out.offset(
                (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                    as isize,
            )
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    let s = &mut *s;
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
        let pending_buf =
            ::core::slice::from_raw_parts_mut((*s).pending_buf, (*s).pending_buf_size as usize);
        crate::src::trees::bi_flush(s, pending_buf);
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
    let mut func: compress_func = None;
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
                        .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Posf>()),
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
pub unsafe extern "C" fn deflateBound_z(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut fixedlen: crate::stdlib::z_size_t = 0;
    let mut storelen: crate::stdlib::z_size_t = 0;
    let mut wraplen: crate::stdlib::z_size_t = 0;
    let mut bound: crate::stdlib::z_size_t = 0;
    fixedlen = sourceLen
        .wrapping_add(sourceLen >> 3 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 8 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 9 as ::core::ffi::c_int)
        .wrapping_add(4 as crate::stdlib::z_size_t);
    if fixedlen < sourceLen {
        fixedlen = -1 as ::core::ffi::c_int as crate::stdlib::z_size_t;
    }
    storelen = sourceLen
        .wrapping_add(sourceLen >> 5 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 7 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 11 as ::core::ffi::c_int)
        .wrapping_add(7 as crate::stdlib::z_size_t);
    if storelen < sourceLen {
        storelen = -1 as ::core::ffi::c_int as crate::stdlib::z_size_t;
    }
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
                        let c2rust_fresh59 = str;
                        str = str.offset(1);
                        if *c2rust_fresh59 == 0 {
                            break;
                        }
                    }
                }
                str = (*(*s).gzhead).comment;
                if !str.is_null() {
                    loop {
                        wraplen = wraplen.wrapping_add(1);
                        let c2rust_fresh60 = str;
                        str = str.offset(1);
                        if *c2rust_fresh60 == 0 {
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
pub unsafe extern "C" fn deflateBound(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let mut bound: crate::stdlib::z_size_t =
        deflateBound_z(strm, sourceLen as crate::stdlib::z_size_t);
    return if bound != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        bound as crate::stdlib::uLong
    };
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    deflateBound(strm, sourceLen)
}
unsafe extern "C" fn putShortMSB(
    mut s: *mut crate::src::deflate::deflate_state,
    mut b: crate::stdlib::uInt,
) {
    let c2rust_fresh33 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(c2rust_fresh33 as isize) =
        (b >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte;
    let c2rust_fresh34 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(c2rust_fresh34 as isize) =
        (b & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte;
}

unsafe extern "C" fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    let mut len: ::core::ffi::c_uint = 0;
    let mut s: *mut crate::src::deflate::deflate_state =
        (*strm).state as *mut crate::src::deflate::deflate_state;
    let s = &mut *s;
    let pending_buf =
        ::core::slice::from_raw_parts_mut((*s).pending_buf, (*s).pending_buf_size as usize);
    crate::src::trees::bi_flush(s, pending_buf);
    len = if (*s).pending > (*strm).avail_out as crate::zutil_h::ulg {
        (*strm).avail_out as ::core::ffi::c_uint
    } else {
        (*s).pending as ::core::ffi::c_uint
    };
    if len == 0 as ::core::ffi::c_uint {
        return;
    }
    crate::stdlib::memcpy(
        (*strm).next_out as *mut ::core::ffi::c_void,
        (*s).pending_out as *const ::core::ffi::c_void,
        len as crate::__stddef_size_t_h::size_t,
    );
    (*strm).next_out = (*strm).next_out.offset(len as isize);
    (*s).pending_out = (*s).pending_out.offset(len as isize);
    (*strm).total_out = (*strm).total_out.wrapping_add(len as crate::stdlib::uLong);
    (*strm).avail_out = (*strm).avail_out.wrapping_sub(len);
    (*s).pending = (*s).pending.wrapping_sub(len as crate::zutil_h::ulg);
    if (*s).pending == 0 as crate::zutil_h::ulg {
        (*s).pending_out = (*s).pending_buf;
    }
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
        (*strm).adler = crate::src::adler32::adler32(
            0 as crate::stdlib::uLong,
            ::core::ptr::null::<crate::stdlib::Bytef>(),
            0 as crate::stdlib::uInt,
        );
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if (*s).status == crate::src::deflate::GZIP_STATE {
        (*strm).adler = crate::src::crc32::crc32(
            0 as crate::stdlib::uLong,
            ::core::ptr::null::<crate::stdlib::Bytef>(),
            0 as crate::stdlib::uInt,
        );
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
                (if (*s).level == 9 as ::core::ffi::c_int {
                    2 as ::core::ffi::c_int
                } else if (*s).strategy >= 2 as ::core::ffi::c_int
                    || (*s).level < 2 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as crate::stdlib::Bytef;
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
            *(*s).pending_buf.offset(c2rust_fresh10 as isize) =
                ((if (*(*s).gzhead).text != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) + (if (*(*s).gzhead).hcrc != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) + (if (*(*s).gzhead).extra.is_null() {
                    0 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                }) + (if (*(*s).gzhead).name.is_null() {
                    0 as ::core::ffi::c_int
                } else {
                    8 as ::core::ffi::c_int
                }) + (if (*(*s).gzhead).comment.is_null() {
                    0 as ::core::ffi::c_int
                } else {
                    16 as ::core::ffi::c_int
                })) as crate::stdlib::Bytef;
            let c2rust_fresh11 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh11 as isize) =
                ((*(*s).gzhead).time & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte;
            let c2rust_fresh12 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh12 as isize) =
                ((*(*s).gzhead).time >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte;
            let c2rust_fresh13 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh13 as isize) =
                ((*(*s).gzhead).time >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte;
            let c2rust_fresh14 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(c2rust_fresh14 as isize) =
                ((*(*s).gzhead).time >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
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
                ((*(*s).gzhead).os & 0xff as ::core::ffi::c_int) as crate::stdlib::Bytef;
            if !(*(*s).gzhead).extra.is_null() {
                let c2rust_fresh17 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(c2rust_fresh17 as isize) = ((*(*s).gzhead).extra_len
                    & 0xff as crate::stdlib::uInt)
                    as crate::stdlib::Bytef;
                let c2rust_fresh18 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(c2rust_fresh18 as isize) =
                    ((*(*s).gzhead).extra_len >> 8 as ::core::ffi::c_int
                        & 0xff as crate::stdlib::uInt) as crate::stdlib::Bytef;
            }
            if (*(*s).gzhead).hcrc != 0 {
                (*strm).adler = crate::src::crc32::crc32_z(
                    (*strm).adler,
                    (*s).pending_buf,
                    (*s).pending as crate::stdlib::z_size_t,
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
                if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg {
                    (*strm).adler = crate::src::crc32::crc32_z(
                        (*strm).adler,
                        (*s).pending_buf.offset(beg as isize),
                        ((*s).pending as crate::stdlib::z_size_t)
                            .wrapping_sub(beg as crate::stdlib::z_size_t),
                    );
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
                (*strm).adler = crate::src::crc32::crc32_z(
                    (*strm).adler,
                    (*s).pending_buf.offset(beg as isize),
                    ((*s).pending as crate::stdlib::z_size_t)
                        .wrapping_sub(beg as crate::stdlib::z_size_t),
                );
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
                            (*s).pending_buf.offset(beg_0 as isize),
                            ((*s).pending as crate::stdlib::z_size_t)
                                .wrapping_sub(beg_0 as crate::stdlib::z_size_t),
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
                    (*s).pending_buf.offset(beg_0 as isize),
                    ((*s).pending as crate::stdlib::z_size_t)
                        .wrapping_sub(beg_0 as crate::stdlib::z_size_t),
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
                            (*s).pending_buf.offset(beg_1 as isize),
                            ((*s).pending as crate::stdlib::z_size_t)
                                .wrapping_sub(beg_1 as crate::stdlib::z_size_t),
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
                    (*s).pending_buf.offset(beg_1 as isize),
                    ((*s).pending as crate::stdlib::z_size_t)
                        .wrapping_sub(beg_1 as crate::stdlib::z_size_t),
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
            (*strm).adler = crate::src::crc32::crc32(
                0 as crate::stdlib::uLong,
                ::core::ptr::null::<crate::stdlib::Bytef>(),
                0 as crate::stdlib::uInt,
            );
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
                crate::src::trees::_tr_stored_block(state, pending_buf, &[], 0);
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    *state
                        .head
                        .offset(state.hash_size.wrapping_sub(1 as crate::stdlib::uInt) as isize) =
                        NIL as crate::src::deflate::Posf;
                    crate::stdlib::memset(
                        state.head as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (state.hash_size.wrapping_sub(1 as crate::stdlib::uInt)
                            as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Posf>()),
                    );
                    state.slid = 0 as ::core::ffi::c_int;
                    if state.lookahead == 0 as crate::stdlib::uInt {
                        state.strstart = 0 as crate::stdlib::uInt;
                        state.block_start = 0 as ::core::ffi::c_long;
                        state.insert = 0 as crate::stdlib::uInt;
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
                *match_0 as ::core::ffi::c_int != *scan.offset(1 as isize) as ::core::ffi::c_int
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
            len = crate::zutil_h::MAX_MATCH - strend.offset_from(scan) as ::core::ffi::c_int;
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

unsafe extern "C" fn deflate_stored(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut min_block: ::core::ffi::c_uint =
        (if (*s).pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg)
            > (*s).w_size as crate::zutil_h::ulg
        {
            (*s).w_size as crate::zutil_h::ulg
        } else {
            (*s).pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg)
        }) as ::core::ffi::c_uint;
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used: ::core::ffi::c_uint = (*(*s).strm).avail_in as ::core::ffi::c_uint;
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        have = ((*s).bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
            >> 3 as ::core::ffi::c_int;
        if (*(*s).strm).avail_out < have {
            break;
        }
        have = ((*(*s).strm).avail_out as ::core::ffi::c_uint).wrapping_sub(have);
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
        if len < min_block
            && (len == 0 as ::core::ffi::c_uint && flush != crate::zlib_h::Z_FINISH
                || flush == crate::zlib_h::Z_NO_FLUSH
                || len != (left as crate::stdlib::uInt).wrapping_add((*(*s).strm).avail_in))
        {
            break;
        }
        last = if flush == crate::zlib_h::Z_FINISH
            && len == (left as crate::stdlib::uInt).wrapping_add((*(*s).strm).avail_in)
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let state = &mut *s;
        let prefix_start = state.pending as usize;
        let (prefix, prefix_len) = crate::src::trees::stored_block_prefix(state, last);
        for (index, byte) in prefix[..prefix_len].iter().enumerate() {
            *state.pending_buf.offset((prefix_start + index) as isize) = *byte;
        }
        let stored_len = len as crate::zutil_h::ush;
        let header_start = state.pending as usize;
        for (index, byte) in [
            stored_len as crate::stdlib::Bytef,
            (stored_len >> 8) as crate::stdlib::Bytef,
            (!stored_len) as crate::stdlib::Bytef,
            ((!stored_len) >> 8) as crate::stdlib::Bytef,
        ]
        .iter()
        .enumerate()
        {
            *state.pending_buf.offset((header_start + index) as isize) = *byte;
        }
        state.pending = state.pending.wrapping_add(4);
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
        if last != 0 as ::core::ffi::c_int {
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
            (*s).insert =
                (*s).insert
                    .wrapping_add(if used > (*s).w_size.wrapping_sub((*s).insert) {
                        ((*s).w_size as ::core::ffi::c_uint)
                            .wrapping_sub((*s).insert as ::core::ffi::c_uint)
                    } else {
                        used
                    });
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
        read_buf((*s).strm, (*s).window.offset((*s).strstart as isize), have);
        (*s).strstart = (*s).strstart.wrapping_add(have);
        (*s).insert = (*s)
            .insert
            .wrapping_add(if have > (*s).w_size.wrapping_sub((*s).insert) {
                ((*s).w_size as ::core::ffi::c_uint)
                    .wrapping_sub((*s).insert as ::core::ffi::c_uint)
            } else {
                have
            });
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
        let state = &mut *s;
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        let stored = ::core::slice::from_raw_parts(
            state.window.offset(state.block_start as isize),
            len as usize,
        );
        crate::src::trees::_tr_stored_block(state, pending_buf, stored, last);
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
            *(*s).sym_buf.offset(c2rust_fresh44 as isize) =
                dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh45 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh45 as isize) =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh46 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh46 as isize) = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as ::core::ffi::c_int;
            let dist_code = crate::src::trees::_dist_code[if (dist as ::core::ffi::c_int)
                < 256 as ::core::ffi::c_int
            {
                dist as usize
            } else {
                (256 as ::core::ffi::c_int
                    + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                    as usize
            }] as ::core::ffi::c_int;
            (*s).dyn_ltree[(length_code
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(length_code
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[dist_code as usize].fc.freq =
                (*s).dyn_dtree[dist_code as usize].fc.freq.wrapping_add(1);
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
            *(*s).sym_buf.offset(c2rust_fresh47 as isize) = 0 as crate::zutil_h::uchf;
            let c2rust_fresh48 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh48 as isize) = 0 as crate::zutil_h::uchf;
            let c2rust_fresh49 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh49 as isize) = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
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
            *(*s).sym_buf.offset(c2rust_fresh35 as isize) =
                dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh36 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh36 as isize) =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh37 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh37 as isize) = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as ::core::ffi::c_int;
            let dist_code = crate::src::trees::_dist_code[if (dist as ::core::ffi::c_int)
                < 256 as ::core::ffi::c_int
            {
                dist as usize
            } else {
                (256 as ::core::ffi::c_int
                    + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                    as usize
            }] as ::core::ffi::c_int;
            (*s).dyn_ltree[(length_code
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(length_code
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[dist_code as usize].fc.freq =
                (*s).dyn_dtree[dist_code as usize].fc.freq.wrapping_add(1);
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
            *(*s).sym_buf.offset(c2rust_fresh38 as isize) = 0 as crate::zutil_h::uchf;
            let c2rust_fresh39 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh39 as isize) = 0 as crate::zutil_h::uchf;
            let c2rust_fresh40 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh40 as isize) = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
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
        *(*s).sym_buf.offset(c2rust_fresh41 as isize) = 0 as crate::zutil_h::uchf;
        let c2rust_fresh42 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        *(*s).sym_buf.offset(c2rust_fresh42 as isize) = 0 as crate::zutil_h::uchf;
        let c2rust_fresh43 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        *(*s).sym_buf.offset(c2rust_fresh43 as isize) = cc_0 as crate::zutil_h::uchf;
        (*s).dyn_ltree[cc_0 as usize].fc.freq =
            (*s).dyn_ltree[cc_0 as usize].fc.freq.wrapping_add(1);
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
                .offset((*s).strstart as isize)
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
                strend = (*s)
                    .window
                    .offset((*s).strstart as isize)
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
                (*s).match_length = (crate::zutil_h::MAX_MATCH as crate::stdlib::uInt)
                    .wrapping_sub(strend.offset_from(scan) as crate::stdlib::uInt);
                if (*s).match_length > (*s).lookahead {
                    (*s).match_length = (*s).lookahead;
                }
            }
        }
        if (*s).match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut len: crate::zutil_h::uch =
                (*s).match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let mut dist: crate::zutil_h::ush = 1 as ::core::ffi::c_int as crate::zutil_h::ush;
            let c2rust_fresh50 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh50 as isize) =
                dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh51 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh51 as isize) =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh52 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh52 as isize) = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as ::core::ffi::c_int;
            let dist_code = crate::src::trees::_dist_code[if (dist as ::core::ffi::c_int)
                < 256 as ::core::ffi::c_int
            {
                dist as usize
            } else {
                (256 as ::core::ffi::c_int
                    + (dist as ::core::ffi::c_int >> 7 as ::core::ffi::c_int))
                    as usize
            }] as ::core::ffi::c_int;
            (*s).dyn_ltree[(length_code
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(length_code
                + crate::src::deflate::LITERALS
                + 1 as ::core::ffi::c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[dist_code as usize].fc.freq =
                (*s).dyn_dtree[dist_code as usize].fc.freq.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub((*s).match_length);
            (*s).strstart = (*s).strstart.wrapping_add((*s).match_length);
            (*s).match_length = 0 as crate::stdlib::uInt;
        } else {
            let mut cc: crate::zutil_h::uch =
                *(*s).window.offset((*s).strstart as isize) as crate::zutil_h::uch;
            let c2rust_fresh53 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh53 as isize) = 0 as crate::zutil_h::uchf;
            let c2rust_fresh54 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh54 as isize) = 0 as crate::zutil_h::uchf;
            let c2rust_fresh55 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *(*s).sym_buf.offset(c2rust_fresh55 as isize) = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
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
        *(*s).sym_buf.offset(c2rust_fresh56 as isize) = 0 as crate::zutil_h::uchf;
        let c2rust_fresh57 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        *(*s).sym_buf.offset(c2rust_fresh57 as isize) = 0 as crate::zutil_h::uchf;
        let c2rust_fresh58 = (*s).sym_next;
        (*s).sym_next = (*s).sym_next.wrapping_add(1);
        *(*s).sym_buf.offset(c2rust_fresh58 as isize) = cc as crate::zutil_h::uchf;
        (*s).dyn_ltree[cc as usize].fc.freq = (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
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
