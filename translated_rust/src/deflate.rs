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
    // `freq` and `code` were C union aliases with identical representation.
    pub freq: crate::zutil_h::ush,
}
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0 {
    // `dad` and `len` were C union aliases with identical representation.
    pub dad: crate::zutil_h::ush,
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

pub use crate::src::crc32::crc32_z_ffi as crc32_z;
pub use crate::src::trees::_dist_code;
pub use crate::src::trees::_length_code;
pub use crate::src::trees::_tr_align;
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
    unsafe fn(
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

pub static deflate_copyright: [::core::ffi::c_char; 70] = crate::c_char_bytes(
    *b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0",
);

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
                as unsafe fn(
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
                as unsafe fn(
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
                as unsafe fn(
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
                as unsafe fn(
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
                as unsafe fn(
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
                as unsafe fn(
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
                as unsafe fn(
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
                as unsafe fn(
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
                as unsafe fn(
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
                as unsafe fn(
                    *mut crate::src::deflate::deflate_state,
                    ::core::ffi::c_int,
                ) -> block_state,
        ),
    },
];

fn slide_hash_tables(
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    wsize: crate::stdlib::uInt,
) {
    for entry in head.iter_mut().chain(prev.iter_mut()) {
        let value = *entry as ::core::ffi::c_uint;
        *entry = (if value >= wsize {
            value.wrapping_sub(wsize as ::core::ffi::c_uint)
        } else {
            NIL as ::core::ffi::c_uint
        }) as crate::src::deflate::Pos as crate::src::deflate::Posf;
    }
}

fn read_buf_bytes(
    strm: &mut crate::zlib_h::z_stream,
    output: &mut [crate::stdlib::Bytef],
    input: &[crate::stdlib::Bytef],
    wrap: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    output.copy_from_slice(input);
    let len = input.len() as ::core::ffi::c_uint;
    strm.avail_in = strm.avail_in.wrapping_sub(len);
    if wrap == 1 as ::core::ffi::c_int {
        strm.adler = crate::src::adler32::adler32_bytes(strm.adler, input);
    } else if wrap == 2 as ::core::ffi::c_int {
        strm.adler = crate::src::crc32::crc32_bytes(strm.adler, input);
    }
    strm.total_in = strm.total_in.wrapping_add(len as crate::stdlib::uLong);
    len
}

// This private adapter binds the allocations owned by a validated deflater
// before passing them to a bounded operation.  Callers that only need the
// owned allocations pass `false` for `bind_input`, avoiding an unnecessary
// bind of the unrelated caller input cursor.
fn fill_window<T>(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    bind_input: bool,
    operation: impl FnOnce(
        &mut crate::src::deflate::deflate_state,
        &mut crate::zlib_h::z_stream,
        &mut [crate::stdlib::Bytef],
        &mut [crate::src::deflate::Posf],
        &mut [crate::src::deflate::Posf],
        &[crate::stdlib::Bytef],
    ) -> T,
) -> T {
    let window = if state.window_size == 0 {
        &mut []
    } else {
        // SAFETY: `window_size` describes the allocation made for `window`.
        unsafe { ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize) }
    };
    // SAFETY: these table sizes are the allocation lengths established during
    // deflater initialization.
    let head = unsafe { ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize) };
    let prev = unsafe { ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize) };
    let input = if !bind_input || stream.avail_in == 0 {
        &[]
    } else {
        // SAFETY: a nonempty input cursor has `avail_in` readable bytes.
        unsafe { ::core::slice::from_raw_parts(stream.next_in, stream.avail_in as usize) }
    };
    operation(state, stream, window, head, prev, input)
}

fn fill_window_bytes(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    mut input: &[crate::stdlib::Bytef],
) -> usize {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    let wsize = state.w_size;
    let initial_input_len = input.len();
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
            let copied = wsize.wrapping_sub(more) as usize;
            window.copy_within(wsize as usize..wsize as usize + copied, 0);
            state.match_start = state.match_start.wrapping_sub(wsize);
            state.strstart = state.strstart.wrapping_sub(wsize);
            state.block_start -= wsize as ::core::ffi::c_long;
            if state.insert > state.strstart {
                state.insert = state.strstart;
            }
            slide_hash_tables(head, prev, wsize);
            state.slid = 1 as ::core::ffi::c_int;
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if stream.avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        let len = ::core::cmp::min(stream.avail_in, more) as usize;
        let start = state.strstart.wrapping_add(state.lookahead) as usize;
        n = read_buf_bytes(stream, &mut window[start..start + len], &input[..len], state.wrap);
        input = &input[len..];
        state.lookahead = state.lookahead.wrapping_add(n);
        if state.lookahead.wrapping_add(state.insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        {
            let mut str = state.strstart.wrapping_sub(state.insert);
            state.ins_h = window[str as usize] as crate::stdlib::uInt;
            state.ins_h = (state.ins_h << state.hash_shift
                ^ window[str.wrapping_add(1) as usize] as crate::stdlib::uInt)
                & state.hash_mask;
            while state.insert != 0 {
                state.ins_h = (state.ins_h << state.hash_shift
                    ^ window[str.wrapping_add(3).wrapping_sub(1) as usize]
                        as crate::stdlib::uInt)
                    & state.hash_mask;
                prev[(str & state.w_mask) as usize] = head[state.ins_h as usize];
                head[state.ins_h as usize] = str as crate::src::deflate::Pos as crate::src::deflate::Posf;
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
            && stream.avail_in != 0 as crate::stdlib::uInt)
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
            window[curr as usize..curr.wrapping_add(init) as usize].fill(0);
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
            window[state.high_water as usize..state.high_water.wrapping_add(init) as usize]
                .fill(0);
            state.high_water = state.high_water.wrapping_add(init);
        }
    }
    initial_input_len - input.len()
}

// The compression loops retain a fixed view of the caller input for one
// deflate call.  Refill through that view, advancing both its index and the
// public cursor only by the bounded amount accepted by `fill_window_bytes()`.
fn fill_window_bound(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    input: &[crate::stdlib::Bytef],
    input_used: &mut usize,
) {
    let consumed = fill_window_bytes(state, stream, window, head, prev, &input[*input_used..]);
    *input_used += consumed;
    stream.next_in = stream.next_in.wrapping_add(consumed);
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
        || *version as ::core::ffi::c_int
            != my_version[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        || stream_size as usize != ::core::mem::size_of::<crate::zlib_h::z_stream>()
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // `strm` has passed the ABI null check. Bind it once for the remaining
    // initialization so the allocation and state setup below do not keep
    // recovering the same reference from the raw stream pointer.
    let stream = &mut *strm;
    stream.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if stream.zalloc.is_none() {
        stream.zalloc = Some(
            crate::src::zutil::zcalloc
                as unsafe extern "C" fn(
                    crate::stdlib::voidpf,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> crate::stdlib::voidpf,
        ) as crate::zlib_h::alloc_func;
        stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if stream.zfree.is_none() {
        stream.zfree = Some(
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
    s = Some(stream.zalloc.expect("non-null function pointer")).expect("non-null function pointer")(
        stream.opaque,
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
    stream.state = s as *mut crate::src::deflate::internal_state;
    // The allocator returned a non-null `deflate_state` above. It is owned by
    // this stream until `deflateEnd()` handles the failure path below.
    let state = &mut *s;
    state.strm = strm;
    state.status = crate::src::deflate::INIT_STATE;
    state.wrap = wrap;
    state.gzhead = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
    state.w_bits = windowBits as crate::stdlib::uInt;
    state.w_size = ((1 as ::core::ffi::c_int) << state.w_bits) as crate::stdlib::uInt;
    state.w_mask = state.w_size.wrapping_sub(1 as crate::stdlib::uInt);
    state.hash_bits = (memLevel as crate::stdlib::uInt).wrapping_add(7 as crate::stdlib::uInt);
    state.hash_size = ((1 as ::core::ffi::c_int) << state.hash_bits) as crate::stdlib::uInt;
    state.hash_mask = state.hash_size.wrapping_sub(1 as crate::stdlib::uInt);
    state.hash_shift = state
        .hash_bits
        .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
    state.window = Some(stream.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        stream.opaque,
        state.w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
            as crate::stdlib::uInt,
    ) as *mut crate::stdlib::Bytef;
    state.prev = Some(stream.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        stream.opaque,
        state.w_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    state.head = Some(stream.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        stream.opaque,
        state.hash_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    state.high_water = 0 as crate::zutil_h::ulg;
    state.lit_bufsize =
        ((1 as ::core::ffi::c_int) << memLevel + 6 as ::core::ffi::c_int) as crate::stdlib::uInt;
    state.pending_buf = Some(stream.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        stream.opaque,
        state.lit_bufsize,
        4 as crate::stdlib::uInt,
    ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef;
    state.pending_buf_size =
        (state.lit_bufsize as crate::zutil_h::ulg).wrapping_mul(4 as crate::zutil_h::ulg);
    if state.window.is_null()
        || state.prev.is_null()
        || state.head.is_null()
        || state.pending_buf.is_null()
    {
        state.status = crate::src::deflate::FINISH_STATE;
        stream.msg = crate::src::zutil::z_errmsg[(if (-4 as ::core::ffi::c_int)
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
    // The allocation above reserves four bytes for every literal entry, so
    // this cursor remains within that allocation.  Wrapping arithmetic keeps
    // the pointer calculation explicit without requiring `offset`'s unsafe
    // in-bounds contract here.
    state.sym_buf = state
        .pending_buf
        .wrapping_add(state.lit_bufsize as usize) as *mut crate::zutil_h::uchf;
    state.sym_end = state
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    state.level = level;
    state.strategy = strategy;
    state.method = method as crate::stdlib::Byte;
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
// State checking is likewise internal; its raw stream binding is required by
// the translated callers, but it has no C ABI contract of its own.  Bind the
// validated pair here so simple deflater operations do not need to dereference
// the same raw stream and state pointers a second time.
unsafe fn deflateStateCheck<'a>(
    mut strm: crate::zlib_h::z_streamp,
) -> Option<(
    &'a mut crate::zlib_h::z_stream,
    &'a mut crate::src::deflate::deflate_state,
)> {
    if strm.is_null() {
        return None;
    }
    let strm_ref = &mut *strm;
    let state_ptr = strm_ref.state as *mut crate::src::deflate::deflate_state;
    if state_ptr.is_null() {
        return None;
    }
    let state = &mut *state_ptr;
    if !deflate_state_is_valid(strm_ref, state, state.strm == strm) {
        return None;
    }
    Some((strm_ref, state))
}

fn deflate_state_is_valid(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::deflate::deflate_state,
    points_back_to_stream: bool,
) -> bool {
    strm.zalloc.is_some()
        && strm.zfree.is_some()
        && points_back_to_stream
        && deflate_status_is_valid(state.status)
}

fn deflate_status_is_valid(status: ::core::ffi::c_int) -> bool {
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
pub unsafe extern "C" fn deflateSetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some((stream, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let avail_in = stream.avail_in;
    let next_in = stream.next_in;
    stream.avail_in = dictLength;
    stream.next_in = dictionary as *mut crate::stdlib::Bytef;
    let result = fill_window(state, stream, true, |state, stream, window, head, prev, dictionary| {
        deflate_set_dictionary(state, stream, window, head, prev, dictionary)
    });
    stream.next_in = next_in;
    stream.avail_in = avail_in;
    result
}

// After the raw caller dictionary and deflater allocations are bound, zlib's
// dictionary prefill is entirely cursor and slice work. In particular, keep
// the temporary stream input binding local so the original caller input is
// restored exactly as in the C implementation.
fn deflate_set_dictionary(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let wrap = state.wrap;
    if wrap == 2
        || wrap == 1 && state.status != crate::src::deflate::INIT_STATE
        || state.lookahead != 0
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if wrap == 1 {
        stream.adler = crate::src::adler32::adler32_bytes(stream.adler, dictionary);
    }
    state.wrap = 0;
    let dictionary = if dictionary.len() >= state.w_size as usize {
        if wrap == 0 {
            head.fill(NIL as crate::src::deflate::Posf);
            state.slid = 0;
            state.strstart = 0;
            state.block_start = 0;
            state.insert = 0;
        }
        &dictionary[dictionary.len() - state.w_size as usize..]
    } else {
        dictionary
    };
    let avail_in = stream.avail_in;
    let next_in = stream.next_in;
    stream.avail_in = dictionary.len() as crate::stdlib::uInt;
    stream.next_in = dictionary.as_ptr() as *mut crate::stdlib::Bytef;
    let mut dictionary_used = 0usize;
    deflate_fill_dictionary_window(
        state,
        stream,
        window,
        head,
        prev,
        dictionary,
        &mut dictionary_used,
    );
    while state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        let mut strstart = state.strstart;
        let mut entries = state.lookahead.wrapping_sub(
            (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt,
        );
        while entries != 0 {
            insert_dictionary_hash_entry(state, window, head, prev, strstart);
            strstart = strstart.wrapping_add(1);
            entries = entries.wrapping_sub(1);
        }
        state.strstart = strstart;
        state.lookahead = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
        deflate_fill_dictionary_window(
            state,
            stream,
            window,
            head,
            prev,
            dictionary,
            &mut dictionary_used,
        );
    }
    state.strstart = state.strstart.wrapping_add(state.lookahead);
    state.block_start = state.strstart as ::core::ffi::c_long;
    state.insert = state.lookahead;
    state.lookahead = 0;
    state.prev_length = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
    state.match_length = state.prev_length;
    state.match_available = 0;
    stream.next_in = next_in;
    stream.avail_in = avail_in;
    state.wrap = wrap;
    crate::zlib_h::Z_OK
}

fn deflate_fill_dictionary_window(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    dictionary: &[crate::stdlib::Bytef],
    dictionary_used: &mut usize,
) {
    let consumed = fill_window_bytes(
        state,
        stream,
        window,
        head,
        prev,
        &dictionary[*dictionary_used..],
    );
    *dictionary_used += consumed;
    stream.next_in = stream.next_in.wrapping_add(consumed);
}

fn insert_dictionary_hash_entry(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    strstart: crate::stdlib::uInt,
) {
    let next = window[strstart
        .wrapping_add((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt)
        as usize];
    state.ins_h = update_hash(state.ins_h, state.hash_shift, state.hash_mask, next);
    prev[(strstart & state.w_mask) as usize] = head[state.ins_h as usize];
    head[state.ins_h as usize] = strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
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
    let Some((_strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let mut len = state.strstart.wrapping_add(state.lookahead);
    if len > state.w_size {
        len = state.w_size;
    }
    let dictionary = if dictionary.is_null() || len == 0 {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(dictionary, len as usize))
    };
    let window = if len == 0 {
        None
    } else {
        let end = state.strstart.wrapping_add(state.lookahead) as usize;
        let window = ::core::slice::from_raw_parts(state.window, state.window_size as usize);
        Some(&window[end - len as usize..end])
    };
    let dict_length = if dictLength.is_null() {
        None
    } else {
        Some(&mut *dictLength)
    };
    deflate_get_dictionary(window, dictionary, dict_length, len)
}

fn deflate_get_dictionary(
    window: Option<&[crate::stdlib::Bytef]>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
    len: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if let (Some(window), Some(dictionary)) = (window, dictionary) {
        dictionary.copy_from_slice(window);
    }
    if let Some(dict_length) = dict_length {
        *dict_length = len;
    }
    crate::zlib_h::Z_OK
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
    initialize_matcher: bool,
) -> ::core::ffi::c_int {
    let Some((strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let result = deflate_reset_keep(strm, state);
    crate::src::trees::_tr_init(state);
    if result == crate::zlib_h::Z_OK && initialize_matcher {
        fill_window(state, strm, false, |state, _stream, _window, head, _prev, _input| {
            lm_init_state(state, head)
        });
    }
    result
}

fn deflate_reset_keep(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
) -> ::core::ffi::c_int {
    strm.total_out = 0;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = crate::zlib_h::Z_UNKNOWN;
    state.pending = 0;
    state.pending_out = state.pending_buf;
    if state.wrap < 0 {
        state.wrap = -state.wrap;
    }
    state.status = if state.wrap == 2 {
        crate::src::deflate::GZIP_STATE
    } else {
        crate::src::deflate::INIT_STATE
    };
    strm.adler = if state.wrap == 2 {
        crate::src::crc32::crc32_bytes(0, &[])
    } else {
        1
    };
    state.last_flush = -2;
    crate::zlib_h::Z_OK
}
#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateResetKeep(strm, false)
}
fn lm_init_state(
    state: &mut crate::src::deflate::deflate_state,
    head: &mut [crate::src::deflate::Posf],
) {
    state.window_size = 2u64.wrapping_mul(state.w_size as crate::zutil_h::ulg);
    let (head, sentinel) = head.split_at_mut(head.len() - 1);
    head.fill(0);
    sentinel[0] = NIL as crate::src::deflate::Posf;
    state.slid = 0;
    let configuration = configuration_table[state.level as usize];
    state.max_lazy_match = configuration.max_lazy as crate::stdlib::uInt;
    state.good_match = configuration.good_length as crate::stdlib::uInt;
    state.nice_match = configuration.nice_length as ::core::ffi::c_int;
    state.max_chain_length = configuration.max_chain as crate::stdlib::uInt;
    state.strstart = 0;
    state.block_start = 0;
    state.lookahead = 0;
    state.insert = 0;
    state.prev_length = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
    state.match_length = state.prev_length;
    state.match_available = 0;
    state.ins_h = 0;
}
pub unsafe extern "C" fn deflateReset(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    deflateResetKeep(strm, true)
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
    let Some((_strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let result = deflate_set_header(state);
    if result == crate::zlib_h::Z_OK {
        state.gzhead = head;
    }
    result
}

fn deflate_set_header(state: &crate::src::deflate::deflate_state) -> ::core::ffi::c_int {
    if state.wrap != 2 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::zlib_h::Z_OK
}
#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    deflateSetHeader(strm, head)
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
    crate::zlib_h::Z_OK
}
#[export_name = "deflatePending"]

pub unsafe extern "C" fn deflatePending_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let pending = if pending.is_null() {
        None
    } else {
        Some(&mut *pending)
    };
    let bits = if bits.is_null() {
        None
    } else {
        Some(&mut *bits)
    };
    deflate_pending(state, pending, bits)
}

fn deflate_used(
    state: &crate::src::deflate::deflate_state,
    bits: Option<&mut ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    if let Some(bits) = bits {
        *bits = state.bi_used;
    }
    crate::zlib_h::Z_OK
}
#[export_name = "deflateUsed"]

pub unsafe extern "C" fn deflateUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let bits = if bits.is_null() {
        None
    } else {
        Some(&mut *bits)
    };
    deflate_used(state, bits)
}
pub unsafe extern "C" fn deflatePrime(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || state.sym_buf
            < state.pending_out.wrapping_add(
                (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                    as usize,
            )
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    let pending = ::core::slice::from_raw_parts_mut(
        state.pending_buf,
        state.pending_buf_size as usize,
    );
    deflate_prime(state, pending, bits, value)
}

fn deflate_prime(
    state: &mut crate::src::deflate::deflate_state,
    pending: &mut [crate::stdlib::Bytef],
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    loop {
        let mut put = crate::src::deflate::Buf_size - state.bi_valid;
        if put > bits {
            put = bits;
        }
        state.bi_buf = (state.bi_buf as ::core::ffi::c_int
            | ((value & ((1 as ::core::ffi::c_int) << put) - 1 as ::core::ffi::c_int)
                << state.bi_valid) as crate::zutil_h::ush as ::core::ffi::c_int)
            as crate::zutil_h::ush;
        state.bi_valid += put;
        deflate_flush_bits(state, pending);
        value >>= put;
        bits -= put;
        if bits == 0 {
            break;
        }
    }
    crate::zlib_h::Z_OK
}

fn deflate_flush_bits(
    state: &mut crate::src::deflate::deflate_state,
    pending: &mut [crate::stdlib::Bytef],
) {
    let pending_index = state.pending as usize;
    if state.bi_valid == crate::src::deflate::Buf_size {
        pending[pending_index] = (state.bi_buf as ::core::ffi::c_int & 0xff) as crate::zutil_h::uch;
        pending[pending_index + 1] =
            (state.bi_buf as ::core::ffi::c_int >> 8) as crate::zutil_h::uch;
        state.pending = state.pending.wrapping_add(2);
        state.bi_buf = 0;
        state.bi_valid = 0;
    } else if state.bi_valid >= 8 {
        pending[pending_index] = state.bi_buf as crate::stdlib::Byte;
        state.pending = state.pending.wrapping_add(1);
        state.bi_buf = (state.bi_buf as ::core::ffi::c_int >> 8) as crate::zutil_h::ush;
        state.bi_valid -= 8;
    }
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
    let Some((_strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
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
    let func = configuration_table[state.level as usize].func;
    if (strategy != state.strategy || func != configuration_table[level as usize].func)
        && state.last_flush != -2 as ::core::ffi::c_int
    {
        let mut err: ::core::ffi::c_int = deflate(strm, crate::zlib_h::Z_BLOCK);
        if err == crate::zlib_h::Z_STREAM_ERROR {
            return err;
        }
        if (*strm).avail_in != 0
            || state.strstart as ::core::ffi::c_long - state.block_start
                + state.lookahead as ::core::ffi::c_long
                != 0
        {
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
    deflate_params(state, head, prev, level, strategy)
}

fn deflate_params(
    state: &mut crate::src::deflate::deflate_state,
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if state.level != level {
        if state.level == 0 && state.matches != 0 {
            if state.matches == 1 {
                slide_hash_tables(head, prev, state.w_size);
                state.slid = 1;
            } else if let Some((last, entries)) = head.split_last_mut() {
                *last = NIL as crate::src::deflate::Posf;
                entries.fill(0);
                state.slid = 0;
            }
            state.matches = 0;
        }
        state.level = level;
        state.max_lazy_match = configuration_table[level as usize].max_lazy as crate::stdlib::uInt;
        state.good_match = configuration_table[level as usize].good_length as crate::stdlib::uInt;
        state.nice_match = configuration_table[level as usize].nice_length as ::core::ffi::c_int;
        state.max_chain_length = configuration_table[level as usize].max_chain as crate::stdlib::uInt;
    }
    state.strategy = strategy;
    crate::zlib_h::Z_OK
}
#[export_name = "deflateParams"]

pub unsafe extern "C" fn deflateParams_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateParams(strm, level, strategy)
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
    crate::zlib_h::Z_OK
}
#[export_name = "deflateTune"]

pub unsafe extern "C" fn deflateTune_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut good_length: ::core::ffi::c_int,
    mut max_lazy: ::core::ffi::c_int,
    mut nice_length: ::core::ffi::c_int,
    mut max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // This is the C ABI boundary: bind the validated stream/state pair once,
    // then leave the tuning operation itself reference-bound.
    let Some((_strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_tune(state, good_length, max_lazy, nice_length, max_chain)
}
struct DeflateBoundState {
    wrap: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    w_bits: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
}

struct DeflateBoundGzipHeader {
    has_extra: bool,
    extra_len: crate::stdlib::uInt,
    name_len: crate::stdlib::z_size_t,
    comment_len: crate::stdlib::z_size_t,
    hcrc: ::core::ffi::c_int,
}

// Once a stream has been bound, the bound calculation needs only this
// value snapshot. Keep the state-only projection out of the raw stream and
// optional-header adapter below.
fn deflate_bound_state(state: &crate::src::deflate::deflate_state) -> DeflateBoundState {
    DeflateBoundState {
        wrap: state.wrap,
        strstart: state.strstart,
        w_bits: state.w_bits,
        hash_bits: state.hash_bits,
        level: state.level,
    }
}

fn deflate_bound_z(
    source_len: crate::stdlib::z_size_t,
    state: Option<DeflateBoundState>,
    gzip_header: Option<DeflateBoundGzipHeader>,
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
    let mut wraplen = 0;
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
            if let Some(gzip_header) = gzip_header {
                if gzip_header.has_extra {
                    wraplen = wraplen.wrapping_add(
                        (2 as crate::stdlib::uInt).wrapping_add(gzip_header.extra_len)
                            as crate::stdlib::z_size_t,
                    );
                }
                wraplen = wraplen.wrapping_add(gzip_header.name_len);
                wraplen = wraplen.wrapping_add(gzip_header.comment_len);
                if gzip_header.hcrc != 0 {
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

// Header length accounting is ordinary, reference-bound state inspection.
// The exported adapter below is solely responsible for binding optional C
// strings before this helper sees them.
fn deflate_bound_gzip_header(
    header: &crate::zlib_h::gz_header,
    name: Option<&::core::ffi::CStr>,
    comment: Option<&::core::ffi::CStr>,
) -> DeflateBoundGzipHeader {
    DeflateBoundGzipHeader {
        has_extra: !header.extra.is_null(),
        extra_len: header.extra_len,
        // zlib's bound includes each terminating nul when a name or comment
        // is present. `to_bytes_with_nul()` preserves that exact count.
        name_len: name
            .map(|name| name.to_bytes_with_nul().len() as crate::stdlib::z_size_t)
            .unwrap_or(0),
        comment_len: comment
            .map(|comment| comment.to_bytes_with_nul().len() as crate::stdlib::z_size_t)
            .unwrap_or(0),
        hcrc: header.hcrc,
    }
}

// The numeric bound is defined solely by a bound deflater state and its
// already-decoded gzip-header lengths. The raw adapter retains the C-pointer
// conversions needed to obtain those inputs.
fn deflate_bound_from_state(
    source_len: crate::stdlib::z_size_t,
    state: Option<&crate::src::deflate::deflate_state>,
    gzip_header: Option<DeflateBoundGzipHeader>,
) -> crate::stdlib::z_size_t {
    deflate_bound_z(source_len, state.map(deflate_bound_state), gzip_header)
}

pub unsafe extern "C" fn deflateBound_z(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let (state, gzip_header) = if let Some((_strm, state)) = deflateStateCheck(strm) {
        let gzip_header = if state.gzhead.is_null() {
            None
        } else {
            let header = &*state.gzhead;
            let name = if header.name.is_null() {
                None
            } else {
                Some(::core::ffi::CStr::from_ptr(
                    header.name as *const ::core::ffi::c_char,
                ))
            };
            let comment = if header.comment.is_null() {
                None
            } else {
                Some(::core::ffi::CStr::from_ptr(
                    header.comment as *const ::core::ffi::c_char,
                ))
            };
            Some(deflate_bound_gzip_header(header, name, comment))
        };
        (Some(state), gzip_header)
    } else {
        (None, None)
    };
    deflate_bound_from_state(sourceLen, state.as_deref(), gzip_header)
}
#[export_name = "deflateBound_z"]

pub unsafe extern "C" fn deflateBound_z_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    deflateBound_z(strm, sourceLen)
}

// The legacy ABI narrows `z_size_t` to `uLong`. Keep the overflow result
// policy independent of the raw stream boundary so both entry points only
// need to obtain the size bound.
fn deflate_bound_result(bound: crate::stdlib::z_size_t) -> crate::stdlib::uLong {
    let result = bound as crate::stdlib::uLong;
    if result as crate::stdlib::z_size_t != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        result
    }
}

pub unsafe extern "C" fn deflateBound(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    deflate_bound_result(deflateBound_z(
        strm,
        sourceLen as crate::stdlib::z_size_t,
    ))
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    deflateBound(strm, sourceLen)
}
fn put_short_msb_bytes(output: &mut [crate::stdlib::Bytef], b: crate::stdlib::uInt) {
    output[0] = (b >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte;
    output[1] = (b & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte;
}

// The two-byte header write only changes already-bound deflater state and a
// bounded pending-output slice. Keep that bookkeeping safe; the raw state and
// allocation-backed buffer binding remain in the narrow adapter below.
fn put_short_msb(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    b: crate::stdlib::uInt,
) {
    let pending = state.pending as usize;
    put_short_msb_bytes(&mut pending_buf[pending..pending + 2], b);
    state.pending = state.pending.wrapping_add(2);
}

fn write_zlib_header(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    header: crate::stdlib::uInt,
    dictionary_adler: Option<crate::stdlib::uLong>,
) {
    put_short_msb(state, pending_buf, header);
    if let Some(dictionary_adler) = dictionary_adler {
        put_short_msb(
            state,
            pending_buf,
            (dictionary_adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
        );
        put_short_msb(
            state,
            pending_buf,
            (dictionary_adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
        );
    }
}

// The gzip magic and method always fit in the initialized pending allocation.
// Serialize this fixed prefix through the bound slice so it does not need raw
// pending-buffer cursor writes.
fn write_gzip_prefix(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
) {
    let pending = state.pending as usize;
    pending_buf[pending..pending + 3].copy_from_slice(&[31, 139, 8]);
    state.pending = state.pending.wrapping_add(3);
}

fn write_gzip_trailer(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
    total_in: crate::stdlib::uLong,
) {
    let pending = state.pending as usize;
    pending_buf[pending..pending + 4].copy_from_slice(&(checksum as u32).to_le_bytes());
    pending_buf[pending + 4..pending + 8].copy_from_slice(&(total_in as u32).to_le_bytes());
    state.pending = state.pending.wrapping_add(8);
}

fn write_zlib_trailer(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
) {
    put_short_msb(
        state,
        pending_buf,
        (checksum >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
    );
    put_short_msb(
        state,
        pending_buf,
        (checksum & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
    );
}

fn pending_copy_len(
    pending: crate::zutil_h::ulg,
    avail_out: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    if pending > avail_out as crate::zutil_h::ulg {
        avail_out as ::core::ffi::c_uint
    } else {
        pending as ::core::ffi::c_uint
    }
}

fn flush_pending_progress(
    avail_out: &mut crate::stdlib::uInt,
    total_out: &mut crate::stdlib::uLong,
    pending: &mut crate::zutil_h::ulg,
    len: ::core::ffi::c_uint,
) {
    *total_out = total_out.wrapping_add(len as crate::stdlib::uLong);
    *avail_out = avail_out.wrapping_sub(len);
    *pending = pending.wrapping_sub(len as crate::zutil_h::ulg);
}

// Once the pending allocation has been bound by the raw adapter, bit draining
// and its transfer size are ordinary reference-and-slice work.
fn flush_pending_bytes(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    pending_buf: &mut [crate::zutil_h::uch],
) -> ::core::ffi::c_uint {
    crate::src::trees::tr_flush_bits(state, pending_buf);
    pending_copy_len(state.pending, stream.avail_out)
}

fn flush_pending_account(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    len: ::core::ffi::c_uint,
) {
    flush_pending_progress(
        &mut stream.avail_out,
        &mut stream.total_out,
        &mut state.pending,
        len,
    );
}

// Once the stream, deflater, and pending allocation are bound, copying one
// pending chunk and publishing its progress is ordinary reference-and-slice
// work.  Keeping it here makes the raw adapter below only a binding boundary.
fn flush_pending_bound(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    pending_buf: &mut [crate::zutil_h::uch],
    output: Option<&mut [crate::stdlib::Bytef]>,
) {
    let len = flush_pending_bytes(state, stream, pending_buf);
    if len == 0 as ::core::ffi::c_uint {
        return;
    }
    // `pending_out` always identifies a subrange of the bound pending
    // allocation. Keep that source as a subslice, bind only the caller's
    // already-validated output range, and make the transfer ordinary slice
    // work instead of a C memory call.
    let pending_start = (state.pending_out as usize).wrapping_sub(state.pending_buf as usize);
    let pending = &pending_buf[pending_start..pending_start + len as usize];
    output.expect("a pending transfer requires output space")[..len as usize]
        .copy_from_slice(pending);
    flush_pending_account(state, stream, len);
    // Both ranges were validated by the deflater before this flush. Advance
    // their addresses without requiring an in-bounds raw-pointer operation.
    stream.next_out = stream.next_out.wrapping_add(len as usize);
    state.pending_out = state.pending_out.wrapping_add(len as usize);
    if state.pending == 0 as crate::zutil_h::ulg {
        state.pending_out = state.pending_buf;
    }
}

// Private raw adapters are Rust-ABI functions. The public `_ffi` wrappers
// remain the only C ABI boundary for callers outside this crate.
unsafe fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    let stream = &mut *strm;
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state.pending_buf,
        state.pending_buf_size as usize,
    );
    let output = if stream.avail_out == 0 {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(
            stream.next_out,
            stream.avail_out as usize,
        ))
    };
    flush_pending_bound(state, stream, pending_buf, output);
}

fn deflate_flush_rank(flush: ::core::ffi::c_int) -> ::core::ffi::c_int {
    flush * 2 - if flush > 4 { 9 } else { 0 }
}

fn deflate_no_progress(
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    old_flush: ::core::ffi::c_int,
) -> bool {
    avail_in == 0 && deflate_flush_rank(flush) <= deflate_flush_rank(old_flush)
        && flush != crate::zlib_h::Z_FINISH
}

fn deflate_zlib_level_flags(
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> crate::stdlib::uInt {
    if strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || level < 2 {
        0
    } else if level < 6 {
        1
    } else if level == 6 {
        2
    } else {
        3
    }
}

fn deflate_gzip_xfl(
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> crate::stdlib::Bytef {
    if level == 9 {
        2
    } else if strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || level < 2 {
        4
    } else {
        0
    }
}

pub unsafe extern "C" fn deflate(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut old_flush: ::core::ffi::c_int = 0;
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if deflateStateCheck(strm).is_none()
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
    } else if deflate_no_progress((*strm).avail_in, flush, old_flush) {
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
        let level_flags = deflate_zlib_level_flags((*s).level, (*s).strategy);
        header |= level_flags << 6 as ::core::ffi::c_int;
        if (*s).strstart != 0 as crate::stdlib::uInt {
            header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
        }
        header = header.wrapping_add(
            (31 as crate::stdlib::uInt)
                .wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
        );
        let dictionary_adler = if (*s).strstart != 0 as crate::stdlib::uInt {
            Some((*strm).adler)
        } else {
            None
        };
        {
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            write_zlib_header(state, pending_buf, header, dictionary_adler);
        }
        (*strm).adler = crate::src::adler32::adler32_buffer(
            0 as crate::stdlib::uLong,
            None,
        );
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if (*s).status == crate::src::deflate::GZIP_STATE {
        (*strm).adler = crate::src::crc32::crc32_buffer(0 as crate::stdlib::uLong, None);
        {
            let state = &mut *s;
            let pending_buf = ::core::slice::from_raw_parts_mut(
                state.pending_buf,
                state.pending_buf_size as usize,
            );
            write_gzip_prefix(state, pending_buf);
        }
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
                deflate_gzip_xfl((*s).level, (*s).strategy);
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
                deflate_gzip_xfl((*s).level, (*s).strategy);
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
                (*strm).adler = crate::src::crc32::crc32_z_ffi(
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
                    (*strm).adler = crate::src::crc32::crc32_z_ffi(
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
                (*strm).adler = crate::src::crc32::crc32_z_ffi(
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
                        (*strm).adler = crate::src::crc32::crc32_z_ffi(
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
                (*strm).adler = crate::src::crc32::crc32_z_ffi(
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
                        (*strm).adler = crate::src::crc32::crc32_z_ffi(
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
                (*strm).adler = crate::src::crc32::crc32_z_ffi(
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
            (*strm).adler = crate::src::crc32::crc32_buffer(0 as crate::stdlib::uLong, None);
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
            configuration_table[(*s).level as usize]
                .func
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
                let state = &mut *s;
                let pending = ::core::slice::from_raw_parts_mut(
                    state.pending_buf,
                    state.pending_buf_size as usize,
                );
                crate::src::trees::tr_stored_block(state, pending, &[], 0);
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    // The initialized head table has `hash_size` entries, so
                    // this is its final entry.  Keep cursor formation safe;
                    // the existing allocation invariant still justifies the
                    // final raw store.
                    *state
                        .head
                        .wrapping_add(
                            state.hash_size.wrapping_sub(1 as crate::stdlib::uInt) as usize,
                        ) =
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
        let checksum = (*strm).adler;
        let total_in = (*strm).total_in;
        let state = &mut *s;
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        write_gzip_trailer(state, pending_buf, checksum, total_in);
    } else {
        let checksum = (*strm).adler;
        let state = &mut *s;
        let pending_buf = ::core::slice::from_raw_parts_mut(
            state.pending_buf,
            state.pending_buf_size as usize,
        );
        write_zlib_trailer(state, pending_buf, checksum);
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
    let Some((stream, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // `deflateStateCheck()` has established both links. Snapshot the release
    // plan before invoking user-supplied deallocators, so no Rust reference
    // spans one of those callbacks.
    let (zfree, opaque, status, pending_buf, head, prev, window, state_ptr) = {
        (
            stream.zfree.expect("non-null function pointer"),
            stream.opaque,
            state.status,
            state.pending_buf,
            state.head,
            state.prev,
            state.window,
            stream.state,
        )
    };
    if !pending_buf.is_null() {
        Some(zfree).expect("non-null function pointer")(opaque, pending_buf as crate::stdlib::voidpf);
    }
    if !head.is_null() {
        Some(zfree).expect("non-null function pointer")(opaque, head as crate::stdlib::voidpf);
    }
    if !prev.is_null() {
        Some(zfree).expect("non-null function pointer")(opaque, prev as crate::stdlib::voidpf);
    }
    if !window.is_null() {
        Some(zfree).expect("non-null function pointer")(opaque, window as crate::stdlib::voidpf);
    }
    Some(zfree).expect("non-null function pointer")(opaque, state_ptr as crate::stdlib::voidpf);
    deflate_end_complete(&mut *strm, status)
}

fn deflate_end_complete(
    stream: &mut crate::zlib_h::z_stream,
    status: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    stream.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    }
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
    if deflateStateCheck(source).is_none() || dest.is_null() {
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
    (*dest).state = ds as *mut crate::src::deflate::internal_state;
    // Copy the state before invoking the allocator again: custom allocation
    // callbacks can inspect `dest->state`, just as they can in zlib's C
    // implementation.  The assignment avoids an untyped whole-struct copy.
    deflate_copy_state(&mut *ds, &*ss);
    (*ds).strm = dest;
    let window = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ss).w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
            as crate::stdlib::uInt,
    ) as *mut crate::stdlib::Bytef;
    let prev = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ss).w_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    let head = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ss).hash_size,
        ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::Posf;
    let pending_buf = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ss).lit_bufsize,
        4 as crate::stdlib::uInt,
    ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef;
    if window.is_null() || prev.is_null() || head.is_null() || pending_buf.is_null() {
        deflateEnd(dest);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // `pending_out` is an offset into the source pending allocation. Retain
    // that byte offset while rebinding it to the destination allocation
    // without requiring raw-pointer in-bounds arithmetic.
    let pending_offset = (*ss)
        .pending_out
        .addr()
        .wrapping_sub((*ss).pending_buf.addr());
    let plan = deflate_copy_plan(&*ss, pending_offset);
    (*ds).window = window;
    (*ds).prev = prev;
    (*ds).head = head;
    (*ds).pending_buf = pending_buf;
    (*ds).pending_out = pending_buf.wrapping_add(pending_offset);
    (*ds).sym_buf = pending_buf.wrapping_add((*ds).lit_bufsize as usize) as *mut crate::zutil_h::uchf;
    // These allocations are all owned by the validated source deflater and
    // have the lengths captured in `plan`. Bind the window, chain table, and
    // pending allocation once, then make four C memory copies ordinary
    // bounded slice copies. The head-table copy stays on the existing raw
    // path, avoiding an additional unsafe allocation bind here.
    let source_window = ::core::slice::from_raw_parts((*ss).window, plan.window_len);
    let destination_window = ::core::slice::from_raw_parts_mut((*ds).window, plan.window_len);
    let source_prev = ::core::slice::from_raw_parts((*ss).prev as *const crate::stdlib::Bytef, plan.prev_len);
    let destination_prev = ::core::slice::from_raw_parts_mut((*ds).prev as *mut crate::stdlib::Bytef, plan.prev_len);
    let source_pending = ::core::slice::from_raw_parts((*ss).pending_buf, plan.pending_buf_len);
    let destination_pending = ::core::slice::from_raw_parts_mut((*ds).pending_buf, plan.pending_buf_len);
    deflate_copy_buffers(
        &plan,
        source_window,
        destination_window,
        source_prev,
        destination_prev,
        source_pending,
        destination_pending,
    );
    crate::stdlib::memcpy(
        (*ds).head as *mut ::core::ffi::c_void,
        (*ss).head as *const ::core::ffi::c_void,
        plan.head_len,
    );
    (*ds).l_desc.dyn_tree = &raw mut (*ds).dyn_ltree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    (*ds).d_desc.dyn_tree = &raw mut (*ds).dyn_dtree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    (*ds).bl_desc.dyn_tree = &raw mut (*ds).bl_tree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    return crate::zlib_h::Z_OK;
}

fn deflate_copy_state(
    destination_state: &mut crate::src::deflate::deflate_state,
    source_state: &crate::src::deflate::deflate_state,
) {
    *destination_state = *source_state;
}

// The source state has passed `deflateStateCheck()` before this is called.
// Keep all byte-count and range policy here, separate from the raw binding in
// `deflateCopy()`, so the actual copies below are checked slice operations.
struct DeflateCopyPlan {
    window_len: usize,
    prev_len: usize,
    head_len: usize,
    pending_buf_len: usize,
    pending_offset: usize,
    pending_len: usize,
    sym_offset: usize,
    sym_len: usize,
}

fn deflate_copy_plan(
    source: &crate::src::deflate::deflate_state,
    pending_offset: usize,
) -> DeflateCopyPlan {
    let prev_entries = if source.slid != 0
        || source.strstart.wrapping_sub(source.insert) > source.w_size
    {
        source.w_size
    } else {
        source.strstart.wrapping_sub(source.insert)
    };
    DeflateCopyPlan {
        window_len: source.high_water as usize,
        prev_len: (prev_entries as usize).wrapping_mul(::core::mem::size_of::<crate::src::deflate::Pos>()),
        head_len: (source.hash_size as usize).wrapping_mul(::core::mem::size_of::<crate::src::deflate::Pos>()),
        pending_buf_len: (source.lit_bufsize as usize).wrapping_mul(4),
        pending_offset,
        pending_len: source.pending as usize,
        sym_offset: source.lit_bufsize as usize,
        sym_len: source.sym_next as usize,
    }
}

fn deflate_copy_buffers(
    plan: &DeflateCopyPlan,
    source_window: &[crate::stdlib::Bytef],
    destination_window: &mut [crate::stdlib::Bytef],
    source_prev: &[crate::stdlib::Bytef],
    destination_prev: &mut [crate::stdlib::Bytef],
    source_pending: &[crate::stdlib::Bytef],
    destination_pending: &mut [crate::stdlib::Bytef],
) {
    destination_window.copy_from_slice(source_window);
    destination_prev.copy_from_slice(source_prev);
    let pending = plan.pending_offset..plan.pending_offset + plan.pending_len;
    destination_pending[pending.clone()].copy_from_slice(&source_pending[pending]);
    let symbols = plan.sym_offset..plan.sym_offset + plan.sym_len;
    destination_pending[symbols.clone()].copy_from_slice(&source_pending[symbols]);
}
#[export_name = "deflateCopy"]

pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateCopy(dest, source)
}
fn longest_match_bytes(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
    prev: &[crate::src::deflate::Posf],
    mut cur_match: crate::src::deflate::IPos,
) -> crate::stdlib::uInt {
    let mut chain_length = state.max_chain_length as ::core::ffi::c_uint;
    let mut best_len = state.prev_length as usize;
    let mut nice_match = state.nice_match as usize;
    let limit = if state.strstart
        > state
            .w_size
            .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
    {
        (state.strstart as crate::src::deflate::IPos).wrapping_sub(
            state
                .w_size
                .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        )
    } else {
        NIL as crate::src::deflate::IPos
    };
    let scan = state.strstart as usize;

    if state.prev_length >= state.good_match {
        chain_length >>= 2;
    }
    if nice_match > state.lookahead as usize {
        nice_match = state.lookahead as usize;
    }
    loop {
        let candidate = cur_match as usize;
        if window[candidate + best_len] == window[scan + best_len]
            && window[candidate + best_len - 1] == window[scan + best_len - 1]
            && window[candidate] == window[scan]
            && window[candidate + 1] == window[scan + 1]
        {
            // Hash-chain entries already share the first MIN_MATCH bytes.  The
            // original unrolled loop resumes its full comparison at byte 3,
            // regardless of the current best length.
            let mut len = crate::zutil_h::MIN_MATCH as usize;
            while len < crate::zutil_h::MAX_MATCH as usize
                && window[candidate + len] == window[scan + len]
            {
                len += 1;
            }
            if len > best_len {
                state.match_start = cur_match as crate::stdlib::uInt;
                best_len = len;
                if len >= nice_match {
                    break;
                }
            }
        }
        cur_match = prev[(cur_match as crate::stdlib::uInt & state.w_mask) as usize]
            as crate::src::deflate::IPos;
        if cur_match <= limit {
            break;
        }
        chain_length = chain_length.wrapping_sub(1);
        if chain_length == 0 {
            break;
        }
    }
    ::core::cmp::min(best_len as crate::stdlib::uInt, state.lookahead)
}

fn update_hash(
    ins_h: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    next: crate::stdlib::Bytef,
) -> crate::stdlib::uInt {
    (ins_h << hash_shift ^ next as crate::stdlib::uInt) & hash_mask
}

fn insert_hash_entry(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
) -> crate::src::deflate::IPos {
    let strstart = state.strstart;
    let next = window[strstart
        .wrapping_add((crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt)
        as usize];
    state.ins_h = update_hash(state.ins_h, state.hash_shift, state.hash_mask, next);
    let previous = head[state.ins_h as usize];
    prev[(strstart & state.w_mask) as usize] = previous;
    head[state.ins_h as usize] = strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
    previous as crate::src::deflate::IPos
}

fn reset_insert_hash(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
) {
    let strstart = state.strstart as usize;
    state.ins_h = window[strstart] as crate::stdlib::uInt;
    state.ins_h = update_hash(
        state.ins_h,
        state.hash_shift,
        state.hash_mask,
        window[strstart + 1],
    );
}

fn literal_byte(
    window: &[crate::stdlib::Bytef],
    strstart: crate::stdlib::uInt,
) -> crate::zutil_h::uch {
    window[strstart as usize] as crate::zutil_h::uch
}

fn block_start_bytes(
    window: &[crate::stdlib::Bytef],
    block_start: ::core::ffi::c_long,
) -> Option<&[crate::stdlib::Bytef]> {
    usize::try_from(block_start)
        .ok()
        .and_then(|start| window.get(start..))
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

fn stored_block_length_bytes(output: &mut [crate::stdlib::Bytef], len: ::core::ffi::c_uint) {
    output[0] = len as crate::stdlib::Bytef;
    output[1] = (len >> 8 as ::core::ffi::c_int) as crate::stdlib::Bytef;
    output[2] = !len as crate::stdlib::Bytef;
    output[3] = (!len >> 8 as ::core::ffi::c_int) as crate::stdlib::Bytef;
}

fn stored_output_progress(stream: &mut crate::zlib_h::z_stream, len: ::core::ffi::c_uint) {
    stream.avail_out = stream.avail_out.wrapping_sub(len);
    stream.total_out = stream.total_out.wrapping_add(len as crate::stdlib::uLong);
}

fn stored_window_bytes(
    window: &[crate::stdlib::Bytef],
    block_start: ::core::ffi::c_long,
    len: ::core::ffi::c_uint,
) -> Option<&[crate::stdlib::Bytef]> {
    block_start_bytes(window, block_start).and_then(|bytes| bytes.get(..len as usize))
}

fn copy_stored_window(
    output: &mut [crate::stdlib::Bytef],
    window: &[crate::stdlib::Bytef],
    block_start: ::core::ffi::c_long,
    len: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    let input = stored_window_bytes(window, block_start, len)?;
    let output = output.get_mut(..input.len())?;
    output.copy_from_slice(input);
    Some(input.len() as ::core::ffi::c_uint)
}

fn update_stored_window(
    state: &mut crate::src::deflate::deflate_state,
    window: &mut [crate::stdlib::Bytef],
    input: &[crate::stdlib::Bytef],
) {
    let used = input.len() as crate::stdlib::uInt;
    if used >= state.w_size {
        state.matches = 2;
        let start = input.len() - state.w_size as usize;
        window[..state.w_size as usize].copy_from_slice(&input[start..]);
        state.strstart = state.w_size;
        state.insert = state.strstart;
    } else {
        if state.window_size.wrapping_sub(state.strstart as crate::zutil_h::ulg)
            <= used as crate::zutil_h::ulg
        {
            state.strstart = state.strstart.wrapping_sub(state.w_size);
            window.copy_within(
                state.w_size as usize..state.w_size.wrapping_add(state.strstart) as usize,
                0,
            );
            if state.matches < 2 {
                state.matches = state.matches.wrapping_add(1);
            }
            if state.insert > state.strstart {
                state.insert = state.strstart;
            }
        }
        let start = state.strstart as usize;
        window[start..start + input.len()].copy_from_slice(input);
        state.strstart = state.strstart.wrapping_add(used);
        state.insert = state.insert.wrapping_add(if used > state.w_size.wrapping_sub(state.insert) {
            state.w_size.wrapping_sub(state.insert)
        } else {
            used
        });
    }
}

fn stored_block_size(
    min_block: ::core::ffi::c_uint,
    bi_valid: ::core::ffi::c_int,
    avail_out: crate::stdlib::uInt,
    left: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> Option<(::core::ffi::c_uint, ::core::ffi::c_int)> {
    let header = (bi_valid as ::core::ffi::c_uint).wrapping_add(42) >> 3;
    if avail_out < header {
        return None;
    }
    let available = avail_out.wrapping_sub(header);
    let total = left.wrapping_add(avail_in);
    let len = ::core::cmp::min(MAX_STORED as ::core::ffi::c_uint, total);
    let len = ::core::cmp::min(len, available);
    if len < min_block
        && (len == 0 && flush != crate::zlib_h::Z_FINISH
            || flush == crate::zlib_h::Z_NO_FLUSH
            || len != total)
    {
        return None;
    }
    Some((
        len,
        if flush == crate::zlib_h::Z_FINISH && len == total {
            1
        } else {
            0
        },
    ))
}

// This private adapter binds the deflater allocations and caller cursors once.
// The stored-block algorithm below uses only those bounded views.
unsafe fn deflate_stored(
    s: *mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> block_state {
    let state = &mut *s;
    let stream = &mut *state.strm;
    let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
    let pending =
        ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize);
    let input = if stream.avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(stream.next_in, stream.avail_in as usize)
    };
    let output = if stream.avail_out == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(stream.next_out, stream.avail_out as usize)
    };
    deflate_stored_impl(state, stream, window, pending, input, output, flush)
}

fn flush_pending_stored(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    pending: &mut [crate::zutil_h::uch],
    output: &mut [crate::stdlib::Bytef],
    output_used: &mut usize,
) {
    let available = stream.avail_out as usize;
    if available == 0 {
        flush_pending_bound(state, stream, pending, None);
        return;
    }
    let start = *output_used;
    flush_pending_bound(
        state,
        stream,
        pending,
        Some(&mut output[start..start + available]),
    );
    *output_used += available - stream.avail_out as usize;
}

fn read_stored_input(
    stream: &mut crate::zlib_h::z_stream,
    output: &mut [crate::stdlib::Bytef],
    output_used: &mut usize,
    input: &[crate::stdlib::Bytef],
    input_used: &mut usize,
    len: usize,
    wrap: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let copied = read_buf_bytes(
        stream,
        &mut output[*output_used..*output_used + len],
        &input[*input_used..*input_used + len],
        wrap,
    );
    *input_used += copied as usize;
    *output_used += copied as usize;
    stream.next_in = stream.next_in.wrapping_add(copied as usize);
    stream.next_out = stream.next_out.wrapping_add(copied as usize);
    stored_output_progress(stream, copied);
    copied
}

fn deflate_stored_impl(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    pending: &mut [crate::zutil_h::uch],
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut input_used = 0usize;
    let mut output_used = 0usize;
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
    let mut used: ::core::ffi::c_uint = stream.avail_in as ::core::ffi::c_uint;
    loop {
        left = (state.strstart as ::core::ffi::c_long - state.block_start) as ::core::ffi::c_uint;
        let Some((block_len, is_last)) = stored_block_size(
            min_block,
            state.bi_valid,
            stream.avail_out,
            left,
            stream.avail_in,
            flush,
        ) else {
            break;
        };
        len = block_len;
        last = is_last;
        crate::src::trees::tr_stored_block(state, pending, &[], last);
        let header_start = state.pending.wrapping_sub(4 as crate::zutil_h::ulg) as usize;
        stored_block_length_bytes(&mut pending[header_start..header_start + 4], len);
        flush_pending_stored(state, stream, pending, output, &mut output_used);
        if left != 0 {
            if left > len {
                left = len;
            }
            if let Some(copied) = copy_stored_window(
                &mut output[output_used..output_used + left as usize],
                window,
                state.block_start,
                left,
            ) {
                // `copied` is bounded by the output slice above. Wrapping
                // arithmetic keeps the valid zero-length null cursor case
                // without requiring `offset`'s in-bounds unsafe operation.
                stream.next_out = stream.next_out.wrapping_add(copied as usize);
                stored_output_progress(stream, copied);
                output_used += copied as usize;
                state.block_start += copied as ::core::ffi::c_long;
                len = len.wrapping_sub(copied);
            }
        }
        if len != 0 {
            read_stored_input(
                stream,
                output,
                &mut output_used,
                input,
                &mut input_used,
                len as usize,
                state.wrap,
            );
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub(stream.avail_in as ::core::ffi::c_uint);
    if used != 0 {
        update_stored_window(state, window, &input[..used as usize]);
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
    if stream.avail_in > have && state.block_start >= state.w_size as ::core::ffi::c_long {
        state.block_start -= state.w_size as ::core::ffi::c_long;
        state.strstart = state.strstart.wrapping_sub(state.w_size);
        window.copy_within(
            state.w_size as usize..state.w_size.wrapping_add(state.strstart) as usize,
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
        let output_start = state.strstart as usize;
        let copied = read_buf_bytes(
            stream,
            &mut window[output_start..output_start + have as usize],
            &input[input_used..input_used + have as usize],
            state.wrap,
        );
        input_used += copied as usize;
        stream.next_in = stream.next_in.wrapping_add(copied as usize);
        state.strstart = state.strstart.wrapping_add(copied);
        state.insert = state
            .insert
            .wrapping_add(if copied > state.w_size.wrapping_sub(state.insert) {
                (state.w_size as ::core::ffi::c_uint)
                    .wrapping_sub(state.insert as ::core::ffi::c_uint)
            } else {
                copied
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
        if let Some(block) = stored_window_bytes(window, state.block_start, len) {
            crate::src::trees::tr_stored_block(state, pending, block, last);
            state.block_start += len as ::core::ffi::c_long;
            flush_pending_stored(state, stream, pending, output, &mut output_used);
        }
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

unsafe fn deflate_fast(
    s: *mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> block_state {
    let state = &mut *s;
    let stream = &mut *state.strm;
    let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
    let pending =
        ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize);
    let symbols = ::core::slice::from_raw_parts_mut(
        state.sym_buf,
        state.lit_bufsize.wrapping_mul(3) as usize,
    );
    let input = if stream.avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(stream.next_in, stream.avail_in as usize)
    };
    let output = if stream.avail_out == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(stream.next_out, stream.avail_out as usize)
    };
    deflate_fast_impl(
        state, stream, window, head, prev, pending, symbols, input, output, flush,
    )
}

// The fast deflater has a fixed view of all state allocations and caller
// cursors for one call. Refill, matching, tallying, block emission, and
// draining can therefore remain ordinary bounded slice operations.
fn deflate_fast_impl(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    pending: &mut [crate::zutil_h::uch],
    symbols: &mut [crate::zutil_h::uchf],
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut input_used = 0usize;
    let mut output_used = 0usize;
    loop {
        if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window_bound(state, stream, window, head, prev, input, &mut input_used);
            if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if state.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        let mut hash_head = NIL as crate::src::deflate::IPos;
        if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            hash_head = insert_hash_entry(state, window, head, prev);
            if hash_head != NIL as crate::src::deflate::IPos
                && (state.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                    <= state
                        .w_size
                        .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
            {
                state.match_length = longest_match_bytes(state, window, prev, hash_head);
            }
        }
        let bflush;
        if state.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            bflush = crate::src::trees::tally_bound(
                state,
                symbols,
                state.strstart.wrapping_sub(state.match_start),
                state.match_length.wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt),
            );
            state.lookahead = state.lookahead.wrapping_sub(state.match_length);
            if state.match_length <= state.max_lazy_match
                && state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            {
                state.match_length = state.match_length.wrapping_sub(1);
                loop {
                    state.strstart = state.strstart.wrapping_add(1);
                    hash_head = insert_hash_entry(state, window, head, prev);
                    state.match_length = state.match_length.wrapping_sub(1);
                    if state.match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                state.strstart = state.strstart.wrapping_add(1);
            } else {
                state.strstart = state.strstart.wrapping_add(state.match_length);
                state.match_length = 0 as crate::stdlib::uInt;
                reset_insert_hash(state, window);
            }
        } else {
            let cc = literal_byte(window, state.strstart);
            bflush = crate::src::trees::tally_bound(state, symbols, 0, cc as ::core::ffi::c_uint);
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            flush_symbol_block(
                state, stream, window, pending, symbols, output, &mut output_used, 0,
            );
            if stream.avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
        }
    }
    state.insert = if state.strstart
        < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        state.strstart
    } else {
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    };
    if flush == crate::zlib_h::Z_FINISH {
        flush_symbol_block(
            state, stream, window, pending, symbols, output, &mut output_used, 1,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return finish_started;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_symbol_block(
            state, stream, window, pending, symbols, output, &mut output_used, 0,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return need_more;
        }
    }
    block_done
}

unsafe fn deflate_slow(
    s: *mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> block_state {
    let state = &mut *s;
    let stream = &mut *state.strm;
    let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
    let pending =
        ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize);
    let symbols = ::core::slice::from_raw_parts_mut(
        state.sym_buf,
        state.lit_bufsize.wrapping_mul(3) as usize,
    );
    let input = if stream.avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(stream.next_in, stream.avail_in as usize)
    };
    let output = if stream.avail_out == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(stream.next_out, stream.avail_out as usize)
    };
    deflate_slow_impl(
        state, stream, window, head, prev, pending, symbols, input, output, flush,
    )
}

// The lazy deflater retains bounded views of every allocation and caller
// cursor for one call. Its match decisions can therefore stay entirely in
// safe indexed operations, including refill and block emission.
fn deflate_slow_impl(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    pending: &mut [crate::zutil_h::uch],
    symbols: &mut [crate::zutil_h::uchf],
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut input_used = 0usize;
    let mut output_used = 0usize;
    loop {
        if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window_bound(state, stream, window, head, prev, input, &mut input_used);
            if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if state.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        let mut hash_head = NIL as crate::src::deflate::IPos;
        if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            hash_head = insert_hash_entry(state, window, head, prev);
            state.prev_length = state.match_length;
            state.prev_match = state.match_start as crate::src::deflate::IPos;
            state.match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            if hash_head != NIL as crate::src::deflate::IPos
                && state.prev_length < state.max_lazy_match
                && (state.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                    <= state
                        .w_size
                        .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
            {
                state.match_length = longest_match_bytes(state, window, prev, hash_head);
                if state.match_length <= 5 as crate::stdlib::uInt
                    && (state.strategy == crate::zlib_h::Z_FILTERED
                        || state.match_length == crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                            && state.strstart.wrapping_sub(state.match_start)
                                > TOO_FAR as crate::stdlib::uInt)
                {
                    state.match_length =
                        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
                }
            }
        } else {
            state.prev_length = state.match_length;
            state.prev_match = state.match_start as crate::src::deflate::IPos;
            state.match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        }
        if state.prev_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            && state.match_length <= state.prev_length
        {
            let mut max_insert: crate::stdlib::uInt = state
                .strstart
                .wrapping_add(state.lookahead)
                .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
            let bflush = crate::src::trees::tally_bound(
                state,
                symbols,
                (state.strstart as crate::src::deflate::IPos)
                    .wrapping_sub(1 as crate::src::deflate::IPos)
                    .wrapping_sub(state.prev_match),
                state.prev_length.wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt),
            );
            state.lookahead = state
                .lookahead
                .wrapping_sub(state.prev_length.wrapping_sub(1 as crate::stdlib::uInt));
            state.prev_length = state.prev_length.wrapping_sub(2 as crate::stdlib::uInt);
            loop {
                state.strstart = state.strstart.wrapping_add(1);
                if state.strstart <= max_insert {
                    hash_head = insert_hash_entry(state, window, head, prev);
                }
                state.prev_length = state.prev_length.wrapping_sub(1);
                if state.prev_length == 0 as crate::stdlib::uInt {
                    break;
                }
            }
            state.match_available = 0 as ::core::ffi::c_int;
            state.match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            state.strstart = state.strstart.wrapping_add(1);
            if bflush != 0 {
                flush_symbol_block(
                    state, stream, window, pending, symbols, output, &mut output_used, 0,
                );
                if stream.avail_out == 0 as crate::stdlib::uInt {
                    return need_more;
                }
            }
        } else if state.match_available != 0 {
            let cc = literal_byte(window, state.strstart.wrapping_sub(1 as crate::stdlib::uInt));
            let bflush = crate::src::trees::tally_bound(state, symbols, 0, cc as ::core::ffi::c_uint);
            if bflush != 0 {
                flush_symbol_block(
                    state, stream, window, pending, symbols, output, &mut output_used, 0,
                );
            }
            state.strstart = state.strstart.wrapping_add(1);
            state.lookahead = state.lookahead.wrapping_sub(1);
            if stream.avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
        } else {
            state.match_available = 1 as ::core::ffi::c_int;
            state.strstart = state.strstart.wrapping_add(1);
            state.lookahead = state.lookahead.wrapping_sub(1);
        }
    }
    if state.match_available != 0 {
        let cc_0 = literal_byte(
            window,
            state.strstart.wrapping_sub(1 as crate::stdlib::uInt),
        );
        crate::src::trees::tally_bound(state, symbols, 0, cc_0 as ::core::ffi::c_uint);
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
        flush_symbol_block(
            state, stream, window, pending, symbols, output, &mut output_used, 1,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return finish_started;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_symbol_block(
            state, stream, window, pending, symbols, output, &mut output_used, 0,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return need_more;
        }
    }
    block_done
}

fn rle_match_length(
    window: &[crate::stdlib::Bytef],
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    let start = strstart as usize;
    let previous = window[start - 1];
    let limit = ::core::cmp::min(lookahead, crate::zutil_h::MAX_MATCH as crate::stdlib::uInt)
        as usize;

    if window[start] != previous
        || window[start + 1] != previous
        || window[start + 2] != previous
    {
        return 0;
    }

    let mut length = crate::zutil_h::MIN_MATCH as usize;
    while length < limit && window[start + length] == previous {
        length += 1;
    }
    length as crate::stdlib::uInt
}

// This private adapter binds the RLE deflater's state, allocations, and
// caller cursors once. The compression loop below only uses bounded views.
unsafe fn deflate_rle(
    s: *mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> block_state {
    let state = &mut *s;
    let stream = &mut *state.strm;
    let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
    let pending =
        ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize);
    let symbols = ::core::slice::from_raw_parts_mut(
        state.sym_buf,
        state.lit_bufsize.wrapping_mul(3) as usize,
    );
    let input = if stream.avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(stream.next_in, stream.avail_in as usize)
    };
    let output = if stream.avail_out == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(stream.next_out, stream.avail_out as usize)
    };
    deflate_rle_impl(
        state, stream, window, head, prev, pending, symbols, input, output, flush,
    )
}

fn deflate_rle_impl(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    pending: &mut [crate::zutil_h::uch],
    symbols: &mut [crate::zutil_h::uchf],
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    let mut input_used = 0usize;
    let mut output_used = 0usize;
    loop {
        if state.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_window_bound(state, stream, window, head, prev, input, &mut input_used);
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
        if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            && state.strstart > 0 as crate::stdlib::uInt
        {
            state.match_length = rle_match_length(window, state.strstart, state.lookahead);
        }
        if state.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            bflush = crate::src::trees::tally_bound(
                state,
                symbols,
                1,
                state.match_length.wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt),
            );
            state.lookahead = state.lookahead.wrapping_sub(state.match_length);
            state.strstart = state.strstart.wrapping_add(state.match_length);
            state.match_length = 0 as crate::stdlib::uInt;
        } else {
            let cc = literal_byte(window, state.strstart);
            bflush = crate::src::trees::tally_bound(state, symbols, 0, cc as ::core::ffi::c_uint);
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            flush_symbol_block(
                state, stream, window, pending, symbols, output, &mut output_used, 0,
            );
            if stream.avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
        }
    }
    state.insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        flush_symbol_block(
            state, stream, window, pending, symbols, output, &mut output_used, 1,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return finish_started;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_symbol_block(
            state, stream, window, pending, symbols, output, &mut output_used, 0,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return need_more;
        }
    }
    block_done
}

// The raw compression dispatch binds the deflater, stream, allocations, and
// caller cursors once.  This Huffman-only loop then works entirely with those
// bounded references, including refill, tally, block emission, and draining.
unsafe fn deflate_huff(
    s: *mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> block_state {
    let state = &mut *s;
    let stream = &mut *state.strm;
    let window = ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize);
    let head = ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    let prev = ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize);
    let pending =
        ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize);
    let symbols = ::core::slice::from_raw_parts_mut(
        state.sym_buf,
        state.lit_bufsize.wrapping_mul(3) as usize,
    );
    let input = if stream.avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(stream.next_in, stream.avail_in as usize)
    };
    let output = if stream.avail_out == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(stream.next_out, stream.avail_out as usize)
    };
    deflate_huff_impl(
        state, stream, window, head, prev, pending, symbols, input, output, flush,
    )
}

fn flush_pending_symbols(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    pending: &mut [crate::zutil_h::uch],
    output: &mut [crate::stdlib::Bytef],
    output_used: &mut usize,
) {
    let available = stream.avail_out as usize;
    if available == 0 {
        flush_pending_bound(state, stream, pending, None);
        return;
    }
    let start = *output_used;
    flush_pending_bound(
        state,
        stream,
        pending,
        Some(&mut output[start..start + available]),
    );
    *output_used += available - stream.avail_out as usize;
}

fn flush_symbol_block(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &[crate::stdlib::Bytef],
    pending: &mut [crate::zutil_h::uch],
    symbols: &[crate::zutil_h::uchf],
    output: &mut [crate::stdlib::Bytef],
    output_used: &mut usize,
    last: ::core::ffi::c_int,
) {
    let stored_len = (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg;
    // `_tr_flush_block` receives exactly this block's bytes, not the rest of
    // the sliding window.  Preserve that raw-pointer length contract in the
    // bounded source view used by the safe emitter.
    let source = block_start_bytes(window, state.block_start)
        .and_then(|bytes| bytes.get(..stored_len as usize));
    crate::src::trees::tr_flush_block_bound(
        state,
        stream,
        pending,
        source,
        &symbols[..state.sym_next as usize],
        stored_len,
        last,
    );
    state.block_start = state.strstart as ::core::ffi::c_long;
    flush_pending_symbols(state, stream, pending, output, output_used);
}

fn deflate_huff_impl(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    pending: &mut [crate::zutil_h::uch],
    symbols: &mut [crate::zutil_h::uchf],
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    let mut input_used = 0usize;
    let mut output_used = 0usize;
    loop {
        if state.lookahead == 0 as crate::stdlib::uInt {
            fill_window_bound(state, stream, window, head, prev, input, &mut input_used);
            if state.lookahead == 0 as crate::stdlib::uInt {
                if flush == crate::zlib_h::Z_NO_FLUSH {
                    return need_more;
                }
                break;
            }
        }
        state.match_length = 0 as crate::stdlib::uInt;
        let cc = literal_byte(window, state.strstart);
        bflush = crate::src::trees::tally_bound(state, symbols, 0, cc as ::core::ffi::c_uint);
        state.lookahead = state.lookahead.wrapping_sub(1);
        state.strstart = state.strstart.wrapping_add(1);
        if bflush != 0 {
            flush_symbol_block(
                state, stream, window, pending, symbols, output, &mut output_used, 0,
            );
            if stream.avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
        }
    }
    state.insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        flush_symbol_block(
            state, stream, window, pending, symbols, output, &mut output_used, 1,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return finish_started;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_symbol_block(
            state, stream, window, pending, symbols, output, &mut output_used, 0,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return need_more;
        }
    }
    block_done
}
