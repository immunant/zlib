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

// State allocations come from zlib's configurable allocator and are therefore
// initially uninitialized.  Construct the exact field-wise zero value before
// exposing such an allocation as `deflate_state`, rather than relying on a C
// bytewise write after it has been allocated.
fn deflate_state_zero_value() -> internal_state {
    let zero_ct = ct_data_s {
        fc: C2Rust_Unnamed_1 { freq: 0 },
        dl: C2Rust_Unnamed_0 { dad: 0 },
    };
    let zero_tree = tree_desc_s {
        dyn_tree: ::core::ptr::null_mut(),
        max_code: 0,
        stat_desc: ::core::ptr::null(),
    };
    internal_state {
        strm: ::core::ptr::null_mut(),
        status: 0,
        pending_buf: ::core::ptr::null_mut(),
        pending_buf_size: 0,
        pending_out: ::core::ptr::null_mut(),
        pending: 0,
        wrap: 0,
        gzhead: ::core::ptr::null_mut(),
        gzindex: 0,
        method: 0,
        last_flush: 0,
        w_size: 0,
        w_bits: 0,
        w_mask: 0,
        window: ::core::ptr::null_mut(),
        window_size: 0,
        prev: ::core::ptr::null_mut(),
        head: ::core::ptr::null_mut(),
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
        dyn_ltree: [zero_ct; 573],
        dyn_dtree: [zero_ct; 61],
        bl_tree: [zero_ct; 39],
        l_desc: zero_tree,
        d_desc: zero_tree,
        bl_desc: zero_tree,
        bl_count: [0; 16],
        heap: [0; 573],
        heap_len: 0,
        heap_max: 0,
        depth: [0; 573],
        sym_buf: ::core::ptr::null_mut(),
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

pub use crate::src::crc32::crc32_z_ffi as crc32_z;
pub use crate::src::trees::_dist_code;
pub use crate::src::trees::_length_code;
pub use crate::src::trees::_tr_align_ffi as _tr_align;
pub use crate::src::trees::_tr_flush_block_ffi as _tr_flush_block;
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

// Compression strategies are private Rust implementations.  Their table
// dispatch receives the stream/state pair already validated by `deflate()`,
// so it need not recreate that relationship from a raw state pointer.
pub type compress_func = Option<
    fn(
        &mut crate::src::deflate::deflate_state,
        &mut crate::zlib_h::z_stream,
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
        func: Some(deflate_stored),
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        func: Some(deflate_fast),
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        func: Some(deflate_fast),
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: Some(deflate_fast),
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        func: Some(deflate_slow),
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
                state
                    .w_size
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
        n = read_buf_bytes(
            stream,
            &mut window[start..start + len],
            &input[..len],
            state.wrap,
        );
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
                    ^ window[str.wrapping_add(3).wrapping_sub(1) as usize] as crate::stdlib::uInt)
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
            window[state.high_water as usize..state.high_water.wrapping_add(init) as usize].fill(0);
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

// Default callbacks are an implementation detail of stream initialization.
// Keeping their selection reference-bound means the ABI entry point only has
// to validate and bind its caller-provided stream once.
fn deflate_prepare_stream(stream: &mut crate::zlib_h::z_stream) -> bool {
    stream.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    crate::src::zutil::prepare_stream_allocator(stream)
}

// This internal parameter-defaulting dispatcher only accepts references
// already bound by its callers. The exported `deflateInit2_` adapter binds
// the raw ABI arguments before reaching the implementation below.
//
// zlib checks the version byte and stream layout before it considers the
// stream pointer. Keep that scalar preflight separate so both initializers
// preserve the same externally visible error ordering without duplicating
// the condition at a pointer-handling boundary.
fn deflate_init_version_and_size_valid(
    version: Option<::core::ffi::c_char>,
    stream_size: ::core::ffi::c_int,
) -> bool {
    let Some(version) = version else {
        return false;
    };
    version as ::core::ffi::c_int == crate::zlib_h::ZLIB_VERSION[0] as ::core::ffi::c_int
        && stream_size == ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
}

// Keep all scalar initialization decisions together.  This runs only after
// the public initializer has preserved zlib's version, layout, and stream
// checks, so allocation setup can consume a single validated configuration.
struct DeflateInitOptions {
    level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
}

fn deflate_init_options(
    level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Result<DeflateInitOptions, ::core::ffi::c_int> {
    let level = if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        6
    } else {
        level
    };
    let (wrap, window_bits) = if window_bits < 0 {
        if window_bits < -15 {
            return Err(crate::zlib_h::Z_STREAM_ERROR);
        }
        (0, -window_bits)
    } else if window_bits > 15 {
        (2, window_bits - 16)
    } else {
        (1, window_bits)
    };
    if mem_level < 1
        || mem_level > crate::stdlib::MAX_MEM_LEVEL
        || method != crate::zlib_h::Z_DEFLATED
        || !(8..=15).contains(&window_bits)
        || !(0..=9).contains(&level)
        || !(0..=crate::zlib_h::Z_FIXED).contains(&strategy)
        || window_bits == 8 && wrap != 1
    {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok(DeflateInitOptions {
        level,
        method,
        window_bits: if window_bits == 8 { 9 } else { window_bits },
        mem_level,
        strategy,
        wrap,
    })
}

pub fn deflateInit_(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut level: ::core::ffi::c_int,
    version: Option<&::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // `deflateInit2_()` owns the shared version-before-null validation.  Do
    // not repeat it here: keeping one implementation path prevents this
    // convenience initializer from drifting from the exported initializer.
    deflateInit2_(
        strm,
        level,
        crate::zlib_h::Z_DEFLATED,
        crate::stdlib::MAX_WBITS,
        crate::zutil_h::DEF_MEM_LEVEL,
        crate::zlib_h::Z_DEFAULT_STRATEGY,
        version.copied(),
        stream_size,
    )
}
#[export_name = "deflateInit_"]

pub unsafe extern "C" fn deflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // SAFETY: this is the ABI boundary that binds optional foreign pointers.
    // The safe dispatcher preserves `deflateInit2_`'s validation order.
    let strm = unsafe { strm.as_mut() };
    let version = unsafe { version.as_ref() };
    deflateInit_(strm, level, version, stream_size)
}
// The ABI adapter binds the optional stream and version byte before reaching
// this implementation. Keeping the initializer reference- and value-based
// removes the raw-pointer contract from the core allocation and reset path.
pub fn deflateInit2_(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut level: ::core::ffi::c_int,
    mut method: ::core::ffi::c_int,
    mut windowBits: ::core::ffi::c_int,
    mut memLevel: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
    version: Option<::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if !deflate_init_version_and_size_valid(version, stream_size) {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    let Some(stream) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let uses_default_allocator = deflate_prepare_stream(stream);
    let options = match deflate_init_options(level, method, windowBits, memLevel, strategy) {
        Ok(options) => options,
        Err(error) => return error,
    };
    // A missing allocator was replaced above with zlib's known-safe default.
    // Only a caller-provided callback retains the foreign callback boundary.
    s = if uses_default_allocator {
        crate::src::zutil::zcalloc(
            stream.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::deflate_state
    } else {
        Some(stream.zalloc.expect("non-null function pointer")).expect("non-null function pointer")(
            stream.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::deflate_state
    };
    if s.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // SAFETY: the allocator returned a non-null allocation large enough for
    // one `deflate_state`. Bind it as uninitialized storage only long enough
    // to write the complete safe zero value, then retain the initialized
    // reference for the rest of this function.
    let state = unsafe {
        (&mut *s.cast::<::core::mem::MaybeUninit<crate::src::deflate::deflate_state>>())
            .write(deflate_state_zero_value())
    };
    stream.state = s as *mut crate::src::deflate::internal_state;
    // The allocator returned a non-null `deflate_state` above. It is owned by
    // this stream until `deflateEnd()` handles the failure path below.
    state.strm = stream;
    state.status = crate::src::deflate::INIT_STATE;
    state.wrap = options.wrap;
    state.gzhead = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
    state.w_bits = options.window_bits as crate::stdlib::uInt;
    state.w_size = ((1 as ::core::ffi::c_int) << state.w_bits) as crate::stdlib::uInt;
    state.w_mask = state.w_size.wrapping_sub(1 as crate::stdlib::uInt);
    state.hash_bits =
        (options.mem_level as crate::stdlib::uInt).wrapping_add(7 as crate::stdlib::uInt);
    state.hash_size = ((1 as ::core::ffi::c_int) << state.hash_bits) as crate::stdlib::uInt;
    state.hash_mask = state.hash_size.wrapping_sub(1 as crate::stdlib::uInt);
    state.hash_shift = state
        .hash_bits
        .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
    if uses_default_allocator {
        state.window = crate::src::zutil::zcalloc(
            stream.opaque,
            state.w_size,
            (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
                as crate::stdlib::uInt,
        ) as *mut crate::stdlib::Bytef;
        state.prev = crate::src::zutil::zcalloc(
            stream.opaque,
            state.w_size,
            ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::Posf;
        state.head = crate::src::zutil::zcalloc(
            stream.opaque,
            state.hash_size,
            ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::Posf;
    } else {
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
    }
    state.high_water = 0 as crate::zutil_h::ulg;
    state.lit_bufsize = ((1 as ::core::ffi::c_int) << options.mem_level + 6 as ::core::ffi::c_int)
        as crate::stdlib::uInt;
    state.pending_buf = if uses_default_allocator {
        crate::src::zutil::zcalloc(stream.opaque, state.lit_bufsize, 4 as crate::stdlib::uInt)
            as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef
    } else {
        Some(stream.zalloc.expect("non-null function pointer")).expect("non-null function pointer")(
            stream.opaque,
            state.lit_bufsize,
            4 as crate::stdlib::uInt,
        ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef
    };
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
        deflateEnd(stream);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // The allocation above reserves four bytes for every literal entry, so
    // this cursor remains within that allocation.  Wrapping arithmetic keeps
    // the pointer calculation explicit without requiring `offset`'s unsafe
    // in-bounds contract here.
    state.sym_buf =
        state.pending_buf.wrapping_add(state.lit_bufsize as usize) as *mut crate::zutil_h::uchf;
    state.sym_end = state
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    state.level = options.level;
    state.strategy = options.strategy;
    state.method = options.method as crate::stdlib::Byte;
    return deflate_reset_bound(stream, state, true);
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
    // SAFETY: this ABI adapter alone binds optional foreign pointers.
    // Validation and initialization remain in `deflateInit2_`.
    let strm = unsafe { strm.as_mut() };
    let version = unsafe { version.as_ref().copied() };
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
fn deflateStateCheck<'a>(
    mut strm: crate::zlib_h::z_streamp,
) -> Option<(
    &'a mut crate::zlib_h::z_stream,
    &'a mut crate::src::deflate::deflate_state,
)> {
    if strm.is_null() {
        return None;
    }
    // SAFETY: this private adapter first rejects null stream and state
    // pointers, then validates their reciprocal link and status before
    // exposing either allocation to its reference-only callers.
    unsafe {
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
// The ABI wrapper has already bound both caller-owned ranges. Dictionary
// installation itself only needs those bounded views and the validated stream
// state, so keep it as ordinary reference-and-slice work.
pub fn deflateSetDictionary(
    strm: &mut crate::zlib_h::z_stream,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if dictionary.len() > crate::stdlib::uInt::MAX as usize {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some((stream, state)) = deflateStateCheck(strm as *mut _) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // `deflate_set_dictionary()` binds its (possibly shortened) dictionary
    // tail immediately before filling the window, then restores the caller's
    // cursor. Do not first publish the full dictionary through the generic
    // input adapter: that adapter only needs the deflater-owned allocations
    // for this operation.
    fill_window(
        state,
        stream,
        false,
        |state, stream, window, head, prev, _input| {
            deflate_set_dictionary(state, stream, window, head, prev, dictionary)
        },
    )
}

// After the caller dictionary and deflater allocations are bound, zlib's
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
        let mut entries = state
            .lookahead
            .wrapping_sub((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt);
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
    let next = window
        [strstart.wrapping_add((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt) as usize];
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
    if strm.is_null() || deflateStateCheck(strm).is_none() || dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = if dictLength == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(dictionary, dictLength as usize)
    };
    deflateSetDictionary(&mut *strm, dictionary)
}
pub fn deflateGetDictionary(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    // The window is an allocation owned by the validated deflater, so bind it
    // here with the other implementation-owned buffers.  The ABI wrapper only
    // binds the optional caller outputs and dispatches to this helper.
    fill_window(
        state,
        stream,
        false,
        |state, _stream, window, _head, _prev, _input| {
            deflate_get_dictionary(state, Some(window), dictionary, dict_length)
        },
    )
}

// Once the implementation has bound the state window and optional caller
// ranges, dictionary reporting is entirely ordinary slice and scalar work.
fn deflate_dictionary_length(state: &crate::src::deflate::deflate_state) -> crate::stdlib::uInt {
    state
        .strstart
        .wrapping_add(state.lookahead)
        .min(state.w_size)
}

fn deflate_get_dictionary(
    state: &crate::src::deflate::deflate_state,
    window: Option<&[crate::stdlib::Bytef]>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    let len = deflate_dictionary_length(state);
    if let (Some(window), Some(dictionary)) = (window, dictionary) {
        let end = state.strstart.wrapping_add(state.lookahead) as usize;
        dictionary[..len as usize].copy_from_slice(&window[end - len as usize..end]);
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
    let Some((strm, state)) = deflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // zlib requires a dictionary output buffer large enough for the complete
    // window.  The named implementation determines how much of it is live.
    let dictionary = if dictionary.is_null() || state.w_size == 0 {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(
            dictionary,
            state.w_size as usize,
        ))
    };
    let dict_length = dictLength.as_mut();
    deflateGetDictionary(state, strm, dictionary, dict_length)
}
// Resetting a previously validated deflater only needs its bound stream.
// Keep raw stream dereferencing at the ABI wrappers so gzip's private reset
// path can reuse this without an unsafe call.
pub fn deflateResetKeep(
    strm: &mut crate::zlib_h::z_stream,
    initialize_matcher: bool,
) -> ::core::ffi::c_int {
    let Some((strm, state)) = deflateStateCheck(strm as *mut _) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_reset_bound(strm, state, initialize_matcher)
}

// Initialization and reset both already have the same validated stream/state
// pair. Keep their common reset transition reference-bound so initialization
// does not need to route through the public dispatcher and bind it again.
fn deflate_reset_bound(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    initialize_matcher: bool,
) -> ::core::ffi::c_int {
    let result = deflate_reset_keep(strm, state);
    crate::src::trees::_tr_init(state);
    if result == crate::zlib_h::Z_OK && initialize_matcher {
        fill_window(
            state,
            strm,
            false,
            |state, _stream, _window, head, _prev, _input| lm_init_state(state, head),
        );
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
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflateResetKeep(&mut *strm, false)
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
pub fn deflateReset(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    deflateResetKeep(strm, true)
}
#[export_name = "deflateReset"]

pub unsafe extern "C" fn deflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflateReset(&mut *strm)
}
// The named implementation stays entirely reference-bound. The public ABI
// adapter below is limited to binding its two optional foreign pointers.
fn deflateSetHeader(
    strm: Option<&mut crate::zlib_h::z_stream>,
    head: Option<&mut crate::zlib_h::gz_header>,
) -> ::core::ffi::c_int {
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((_strm, state)) = deflateStateCheck(strm as *mut _) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let result = deflate_header_is_supported(state);
    if result == crate::zlib_h::Z_OK {
        match head {
            Some(head) => state.gzhead = head,
            None => state.gzhead = ::core::ptr::null_mut(),
        }
    }
    result
}

fn deflate_header_is_supported(state: &crate::src::deflate::deflate_state) -> ::core::ffi::c_int {
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
    // Bind foreign arguments only. The named dispatcher preserves zlib's
    // stream validation and header-clearing semantics.
    deflateSetHeader(strm.as_mut(), head.as_mut())
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
// The checked operation is reference-based.  The exported ABI adapter below
// binds the state-owned pending allocation once, leaving the reservation and
// bit emission decisions here in ordinary implementation code.
fn deflate_prime_checked(
    state: &mut crate::src::deflate::deflate_state,
    pending: &mut [crate::stdlib::Bytef],
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deflate_prime_can_reserve(state, bits) {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    deflate_prime(state, pending, bits, value)
}

// The reservation decision only inspects already-bound deflater state. Keep
// it separate from the raw entry point, which is responsible for binding the
// pending allocation after this preflight succeeds.
fn deflate_prime_can_reserve(
    state: &crate::src::deflate::deflate_state,
    bits: ::core::ffi::c_int,
) -> bool {
    bits >= 0 as ::core::ffi::c_int
        && bits <= 16 as ::core::ffi::c_int
        && state.sym_buf
            >= state.pending_out.wrapping_add(
                (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                    as usize,
            )
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
// Prime only changes validated deflater state and its owned pending buffer.
// Reuse the common bounded buffer adapter so the exported ABI shim only has
// to bind its stream argument.
pub fn deflatePrime(
    strm: &mut crate::zlib_h::z_stream,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((stream, state)) = deflateStateCheck(strm as *mut _) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    flush_pending(state, stream, false, |state, _stream, pending, _output| {
        deflate_prime_checked(state, pending, bits, value)
    })
}
#[export_name = "deflatePrime"]

pub unsafe extern "C" fn deflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflatePrime(strm, bits, value)
}
// All raw stream/state binding is contained in `deflateStateCheck()` and the
// bounded helpers it calls, so this implementation itself has no unsafe
// contract. The exported ABI wrapper below remains the foreign-call boundary.
pub extern "C" fn deflateParams(
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
        if _strm.avail_in != 0
            || state.strstart as ::core::ffi::c_long - state.block_start
                + state.lookahead as ::core::ffi::c_long
                != 0
        {
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    // `fill_window()` already owns the one validated binding of these
    // deflater allocations. Reuse it here rather than creating a second raw
    // slice view solely for the parameter update.
    fill_window(
        state,
        _strm,
        false,
        |state, _stream, _window, head, prev, _input| {
            deflate_params(state, head, prev, level, strategy)
        },
    )
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
        state.max_chain_length =
            configuration_table[level as usize].max_chain as crate::stdlib::uInt;
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

// The raw stream and retained gzip-header cursors are inspected only while
// producing a value snapshot for the bound calculation. Keep that localized
// here so Rust callers do not inherit an unsafe-function contract; the C ABI
// wrapper below remains the thin exported dispatcher.
pub fn deflateBound_z(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    // SAFETY: `deflateStateCheck()` validates the stream/state association
    // before exposing it. A configured gzip header, name, and comment remain
    // caller-owned C strings for the duration of this synchronous bound
    // calculation, matching zlib's stream contract.
    unsafe {
        let (state, gzip_header) = match strm {
            Some(strm) => match deflateStateCheck(strm as *mut _) {
                Some((_strm, state)) => {
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
                }
                None => (None, None),
            },
            None => (None, None),
        };
        deflate_bound_from_state(sourceLen, state.as_deref(), gzip_header)
    }
}
#[export_name = "deflateBound_z"]

pub unsafe extern "C" fn deflateBound_z_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    // SAFETY: this ABI adapter only binds the optional foreign stream. The
    // implementation retains the live gzip-header snapshot and bound math.
    let strm = unsafe { strm.as_mut() };
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

#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    // SAFETY: this ABI adapter only binds the optional foreign stream. The
    // implementation retains the live gzip-header snapshot and bound math.
    let strm = unsafe { strm.as_mut() };
    deflate_bound_result(deflateBound_z(strm, sourceLen as crate::stdlib::z_size_t))
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

// The no-header gzip form has seven fixed bytes after the magic and method.
// Keep that serialization on the already-bound pending slice instead of
// reopening the pending allocation for individual raw cursor stores.
fn write_gzip_default_fields(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
) {
    let pending = state.pending as usize;
    pending_buf[pending..pending + 7].copy_from_slice(&[
        0,
        0,
        0,
        0,
        0,
        deflate_gzip_xfl(state.level, state.strategy),
        3,
    ]);
    state.pending = state.pending.wrapping_add(7);
}

fn gzip_header_flags(head: &crate::zlib_h::gz_header) -> crate::stdlib::Bytef {
    ((if head.text != 0 { 1 } else { 0 })
        + (if head.hcrc != 0 { 2 } else { 0 })
        + (if head.extra.is_null() { 0 } else { 4 })
        + (if head.name.is_null() { 0 } else { 8 })
        + (if head.comment.is_null() { 0 } else { 16 })) as crate::stdlib::Bytef
}

// A caller-supplied gzip header is bound at the deflate boundary. Once that
// reference and the state-owned pending range exist, its fixed fields are
// ordinary scalar-to-byte serialization. Variable-length extra/name/comment
// handling remains in the state machine below.
fn write_gzip_header_fields(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    head: &crate::zlib_h::gz_header,
) {
    let pending = state.pending as usize;
    let mut fields = [
        gzip_header_flags(head),
        (head.time & 0xff) as crate::stdlib::Bytef,
        ((head.time >> 8) & 0xff) as crate::stdlib::Bytef,
        ((head.time >> 16) & 0xff) as crate::stdlib::Bytef,
        ((head.time >> 24) & 0xff) as crate::stdlib::Bytef,
        deflate_gzip_xfl(state.level, state.strategy),
        (head.os & 0xff) as crate::stdlib::Bytef,
        0,
        0,
    ];
    let len = if head.extra.is_null() {
        7
    } else {
        fields[7] = (head.extra_len & 0xff) as crate::stdlib::Bytef;
        fields[8] = ((head.extra_len >> 8) & 0xff) as crate::stdlib::Bytef;
        9
    };
    pending_buf[pending..pending + len].copy_from_slice(&fields[..len]);
    state.pending = state.pending.wrapping_add(len as crate::zutil_h::ulg);
}

// The optional gzip header CRC is a fixed little-endian trailer. Serialize it
// through the bound pending slice, keeping the state-machine branch free of
// raw pending-buffer cursor writes.
fn write_gzip_header_crc(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
) {
    let pending = state.pending as usize;
    pending_buf[pending..pending + 2].copy_from_slice(&(checksum as u16).to_le_bytes());
    state.pending = state.pending.wrapping_add(2);
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

// Both pending flushes and direct stored-block copies publish output through
// the same stream counters. Keep that state transition reference-bound so
// their bounded copy paths cannot diverge on counter order or wrapping.
fn deflate_output_progress(stream: &mut crate::zlib_h::z_stream, len: ::core::ffi::c_uint) {
    stream.total_out = stream.total_out.wrapping_add(len as crate::stdlib::uLong);
    stream.avail_out = stream.avail_out.wrapping_sub(len);
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
    deflate_output_progress(stream, len);
    state.pending = state.pending.wrapping_sub(len as crate::zutil_h::ulg);
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

// Pending output is an allocation owned by the validated deflater. Bind it
// once at this narrow boundary so the several block strategies that need it
// can keep their transfer logic slice-based.  Output is bound only for an
// operation that will actually transfer bytes, preserving callers that only
// manipulate pending state from touching an unrelated output cursor.
fn flush_pending<T>(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    bind_output: bool,
    operation: impl FnOnce(
        &mut crate::src::deflate::deflate_state,
        &mut crate::zlib_h::z_stream,
        &mut [crate::zutil_h::uch],
        Option<&mut [crate::stdlib::Bytef]>,
    ) -> T,
) -> T {
    // SAFETY: the validated deflater owns `pending_buf` for
    // `pending_buf_size` bytes. A transferring call with a nonempty output
    // cursor has `avail_out` writable bytes supplied by its caller.
    unsafe {
        let pending_buf =
            ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize);
        let output = if !bind_output || stream.avail_out == 0 {
            None
        } else {
            Some(::core::slice::from_raw_parts_mut(
                stream.next_out,
                stream.avail_out as usize,
            ))
        };
        operation(state, stream, pending_buf, output)
    }
}

// The validated deflate dispatch is the only caller. Reuse the common pending
// binding above, then keep the transfer itself bounded and reference-based in
// `flush_pending_bound()`.
fn flush_pending_transfer(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    transfer: bool,
    operation: impl FnOnce(&mut crate::src::deflate::deflate_state, &mut [crate::zutil_h::uch]),
) {
    flush_pending(state, stream, transfer, |state, stream, pending_buf, output| {
        operation(state, pending_buf);
        if transfer {
            flush_pending_bound(state, stream, pending_buf, output);
        }
    });
}

fn deflate_flush_rank(flush: ::core::ffi::c_int) -> ::core::ffi::c_int {
    flush * 2 - if flush > 4 { 9 } else { 0 }
}

fn deflate_no_progress(
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    old_flush: ::core::ffi::c_int,
) -> bool {
    avail_in == 0
        && deflate_flush_rank(flush) <= deflate_flush_rank(old_flush)
        && flush != crate::zlib_h::Z_FINISH
}

// A full flush discards every hash-chain entry while retaining the final
// sentinel slot. Operate on the already-bound table so this reset does not
// need a raw store or a C memory call at the dispatch site.
fn deflate_clear_hash_table(
    state: &mut crate::src::deflate::deflate_state,
    head: &mut [crate::src::deflate::Posf],
) {
    let (entries, sentinel) = head.split_at_mut(head.len() - 1);
    entries.fill(0);
    sentinel[0] = NIL as crate::src::deflate::Posf;
    state.slid = 0;
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

// Initializing a zlib stream only updates the already-bound stream, deflater,
// and pending output.  Keep the header calculation and transition
// reference-based so the raw entry point only supplies those established
// views.
fn deflate_start_zlib_stream(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    dictionary_adler: crate::stdlib::uLong,
) {
    let mut header: crate::stdlib::uInt =
        (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt).wrapping_add(
            state.w_bits.wrapping_sub(8 as crate::stdlib::uInt) << 4 as ::core::ffi::c_int,
        ) << 8 as ::core::ffi::c_int;
    let level_flags = deflate_zlib_level_flags(state.level, state.strategy);
    header |= level_flags << 6 as ::core::ffi::c_int;
    if state.strstart != 0 as crate::stdlib::uInt {
        header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
    }
    header = header.wrapping_add(
        (31 as crate::stdlib::uInt).wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
    );
    let dictionary_adler = if state.strstart != 0 as crate::stdlib::uInt {
        Some(dictionary_adler)
    } else {
        None
    };
    write_zlib_header(state, pending_buf, header, dictionary_adler);
    state.status = crate::src::deflate::BUSY_STATE;
}

// The gzip-header state machine has completed before this point. Compression,
// flush handling, and trailer emission only use the validated stream and
// deflater, so keep the remainder of a deflate call fully reference-bound.
fn deflate_compress_and_finish(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if stream.avail_in != 0
        || state.lookahead != 0
        || flush != crate::zlib_h::Z_NO_FLUSH && state.status != crate::src::deflate::FINISH_STATE
    {
        let bstate = if state.level == 0 {
            deflate_stored(state, stream, flush)
        } else if state.strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
            deflate_huff(state, stream, flush)
        } else if state.strategy == crate::zlib_h::Z_RLE {
            deflate_rle(state, stream, flush)
        } else {
            configuration_table[state.level as usize]
                .func
                .expect("non-null function pointer")(state, stream, flush)
        };
        if bstate == finish_started || bstate == finish_done {
            state.status = crate::src::deflate::FINISH_STATE;
        }
        if bstate == need_more || bstate == finish_started {
            if stream.avail_out == 0 {
                state.last_flush = -1;
            }
            return crate::zlib_h::Z_OK;
        }
        if bstate == block_done {
            if flush != crate::zlib_h::Z_BLOCK {
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    fill_window(
                        state,
                        stream,
                        false,
                        |state, _stream, _window, head, _prev, _input| {
                            deflate_clear_hash_table(state, head);
                        },
                    );
                    if state.lookahead == 0 {
                        state.strstart = 0;
                        state.block_start = 0;
                        state.insert = 0;
                    }
                }
                flush_pending_transfer(state, stream, true, |state, pending| {
                    if flush == crate::zlib_h::Z_PARTIAL_FLUSH {
                        let end_code = crate::src::trees::static_ltree[256].fc.freq;
                        let end_len = crate::src::trees::static_ltree[256].dl.dad as ::core::ffi::c_int;
                        crate::src::trees::tr_align(state, pending, end_code, end_len);
                    } else {
                        crate::src::trees::tr_stored_block(state, pending, &[], 0);
                    }
                });
            } else {
                flush_pending_transfer(state, stream, true, |_state, _pending| {});
            }
            if stream.avail_out == 0 {
                state.last_flush = -1;
                return crate::zlib_h::Z_OK;
            }
        }
    }
    if flush != crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_OK;
    }
    if state.wrap <= 0 {
        return crate::zlib_h::Z_STREAM_END;
    }
    let checksum = stream.adler;
    let total_in = stream.total_in;
    flush_pending_transfer(state, stream, true, |state, pending| {
        if state.wrap == 2 {
            write_gzip_trailer(state, pending, checksum, total_in);
        } else {
            write_zlib_trailer(state, pending, checksum);
        }
    });
    if state.wrap > 0 {
        state.wrap = -state.wrap;
    }
    if state.pending != 0 {
        crate::zlib_h::Z_OK
    } else {
        crate::zlib_h::Z_STREAM_END
    }
}

enum DeflatePreparation {
    Return(::core::ffi::c_int),
    GzipHeader,
    Compress,
}

fn deflate_set_error(stream: &mut crate::zlib_h::z_stream, error: ::core::ffi::c_int) {
    stream.msg = crate::src::zutil::zError(error) as *mut ::core::ffi::c_char;
}

// The stream/state relationship has already been checked by the ABI entry
// point before this preparation runs.  Keep the common validation, pending
// publication, and fixed wrapper setup reference-bound; only the optional
// caller-owned gzip header remains at the raw boundary below.
fn deflate_prepare_call(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> DeflatePreparation {
    if stream.next_out.is_null()
        || stream.avail_in != 0 && stream.next_in.is_null()
        || state.status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH
    {
        deflate_set_error(stream, crate::zlib_h::Z_STREAM_ERROR);
        return DeflatePreparation::Return(crate::zlib_h::Z_STREAM_ERROR);
    }
    if stream.avail_out == 0 {
        deflate_set_error(stream, crate::zlib_h::Z_BUF_ERROR);
        return DeflatePreparation::Return(crate::zlib_h::Z_BUF_ERROR);
    }

    let old_flush = state.last_flush;
    state.last_flush = flush;
    if state.pending != 0 {
        flush_pending_transfer(state, stream, true, |_state, _pending| {});
        if stream.avail_out == 0 {
            state.last_flush = -1;
            return DeflatePreparation::Return(crate::zlib_h::Z_OK);
        }
    } else if deflate_no_progress(stream.avail_in, flush, old_flush) {
        deflate_set_error(stream, crate::zlib_h::Z_BUF_ERROR);
        return DeflatePreparation::Return(crate::zlib_h::Z_BUF_ERROR);
    }
    if state.status == crate::src::deflate::FINISH_STATE && stream.avail_in != 0 {
        deflate_set_error(stream, crate::zlib_h::Z_BUF_ERROR);
        return DeflatePreparation::Return(crate::zlib_h::Z_BUF_ERROR);
    }
    if state.status == crate::src::deflate::INIT_STATE && state.wrap == 0 {
        state.status = crate::src::deflate::BUSY_STATE;
    }
    if state.status == crate::src::deflate::INIT_STATE {
        let dictionary_adler = stream.adler;
        flush_pending_transfer(state, stream, true, |state, pending| {
            deflate_start_zlib_stream(state, pending, dictionary_adler);
        });
        stream.adler = crate::src::adler32::adler32_buffer(0, None);
        if state.pending != 0 {
            state.last_flush = -1;
            return DeflatePreparation::Return(crate::zlib_h::Z_OK);
        }
    }
    if state.status != crate::src::deflate::GZIP_STATE {
        return DeflatePreparation::Compress;
    }

    stream.adler = crate::src::crc32::crc32_buffer(0, None);
    if !state.gzhead.is_null() {
        return DeflatePreparation::GzipHeader;
    }
    flush_pending_transfer(state, stream, true, |state, pending| {
        write_gzip_prefix(state, pending);
        write_gzip_default_fields(state, pending);
    });
    state.status = crate::src::deflate::BUSY_STATE;
    if state.pending != 0 {
        state.last_flush = -1;
        DeflatePreparation::Return(crate::zlib_h::Z_OK)
    } else {
        DeflatePreparation::Compress
    }
}

// The caller-owned gzip strings are bound at the ABI boundary.  Once they
// are slices, emitting them through the pending buffer and updating the
// optional header CRC is entirely reference-bound state-machine work.
fn deflate_update_gzip_header_crc(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    begin: usize,
) {
    let checksum = stream.adler;
    let mut updated = None;
    flush_pending_transfer(state, stream, false, |state, pending| {
        let end = state.pending as usize;
        if end > begin {
            updated = Some(crate::src::crc32::crc32_bytes(checksum, &pending[begin..end]));
        }
    });
    if let Some(checksum) = updated {
        stream.adler = checksum;
    }
}

fn deflate_write_gzip_header_bytes(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    bytes: &[crate::stdlib::Bytef],
    hcrc: bool,
) -> bool {
    let mut begin = state.pending as usize;
    while (state.gzindex as usize) < bytes.len() {
        if state.pending == state.pending_buf_size {
            if hcrc {
                deflate_update_gzip_header_crc(stream, state, begin);
            }
            flush_pending_transfer(state, stream, true, |_state, _pending| {});
            if state.pending != 0 {
                state.last_flush = -1;
                return false;
            }
            begin = 0;
        }
        let copy = (state.pending_buf_size - state.pending)
            .min(bytes.len() as crate::zutil_h::ulg - state.gzindex) as usize;
        flush_pending_transfer(state, stream, false, |state, pending| {
            let start = state.pending as usize;
            let index = state.gzindex as usize;
            pending[start..start + copy].copy_from_slice(&bytes[index..index + copy]);
            state.pending = state.pending.wrapping_add(copy as crate::zutil_h::ulg);
            state.gzindex = state.gzindex.wrapping_add(copy as crate::zutil_h::ulg);
        });
    }
    if hcrc {
        deflate_update_gzip_header_crc(stream, state, begin);
    }
    state.gzindex = 0;
    true
}

fn deflate_finish_gzip_header(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    head: &crate::zlib_h::gz_header,
    extra: Option<&[crate::stdlib::Bytef]>,
    name: Option<&::core::ffi::CStr>,
    comment: Option<&::core::ffi::CStr>,
) -> ::core::ffi::c_int {
    if state.status == crate::src::deflate::GZIP_STATE {
        flush_pending_transfer(state, stream, false, |state, pending| {
            write_gzip_prefix(state, pending);
            write_gzip_header_fields(state, pending, head);
            state.gzindex = 0;
            state.status = crate::src::deflate::EXTRA_STATE;
        });
        if head.hcrc != 0 {
            deflate_update_gzip_header_crc(stream, state, 0);
        }
    }
    if state.status == crate::src::deflate::EXTRA_STATE {
        if let Some(extra) = extra {
            if !deflate_write_gzip_header_bytes(stream, state, extra, head.hcrc != 0) {
                return crate::zlib_h::Z_OK;
            }
        }
        state.status = crate::src::deflate::NAME_STATE;
    }
    if state.status == crate::src::deflate::NAME_STATE {
        if let Some(name) = name {
            if !deflate_write_gzip_header_bytes(
                stream,
                state,
                name.to_bytes_with_nul(),
                head.hcrc != 0,
            ) {
                return crate::zlib_h::Z_OK;
            }
        }
        state.status = crate::src::deflate::COMMENT_STATE;
    }
    if state.status == crate::src::deflate::COMMENT_STATE {
        if let Some(comment) = comment {
            if !deflate_write_gzip_header_bytes(
                stream,
                state,
                comment.to_bytes_with_nul(),
                head.hcrc != 0,
            ) {
                return crate::zlib_h::Z_OK;
            }
        }
        state.status = crate::src::deflate::HCRC_STATE;
    }
    if state.status == crate::src::deflate::HCRC_STATE {
        if head.hcrc != 0 {
            if state.pending.wrapping_add(2) > state.pending_buf_size {
                flush_pending_transfer(state, stream, true, |_state, _pending| {});
                if state.pending != 0 {
                    state.last_flush = -1;
                    return crate::zlib_h::Z_OK;
                }
            }
            let checksum = stream.adler;
            flush_pending_transfer(state, stream, false, |state, pending| {
                write_gzip_header_crc(state, pending, checksum);
            });
            stream.adler = crate::src::crc32::crc32_buffer(0, None);
        }
        state.status = crate::src::deflate::BUSY_STATE;
        flush_pending_transfer(state, stream, true, |_state, _pending| {});
        if state.pending != 0 {
            state.last_flush = -1;
            return crate::zlib_h::Z_OK;
        }
    }
    crate::zlib_h::Z_STREAM_END
}

// The ordinary outcomes of preparation do not need to re-bind the raw
// stream/state pair.  Keep them on the references already checked by the
// dispatcher, so only the caller-owned gzip header follows the separate
// foreign-data path below.
fn deflate_finish_prepared(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
    preparation: DeflatePreparation,
) -> Option<::core::ffi::c_int> {
    match preparation {
        DeflatePreparation::Return(result) => Some(result),
        DeflatePreparation::Compress => Some(deflate_compress_and_finish(stream, state, flush)),
        DeflatePreparation::GzipHeader => None,
    }
}

// Once the retained gzip header has been bound, choosing whether to continue
// compression is entirely reference-based.  This keeps the raw header
// conversion in the one small section of `deflate()` that needs it.
fn deflate_finish_bound_gzip_header(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
    head: &crate::zlib_h::gz_header,
    extra: Option<&[crate::stdlib::Bytef]>,
    name: Option<&::core::ffi::CStr>,
    comment: Option<&::core::ffi::CStr>,
) -> ::core::ffi::c_int {
    match deflate_finish_gzip_header(stream, state, head, extra, name, comment) {
        crate::zlib_h::Z_OK => crate::zlib_h::Z_OK,
        crate::zlib_h::Z_STREAM_END => deflate_compress_and_finish(stream, state, flush),
        result => result,
    }
}

// The raw stream handle is validated and bound by `deflateStateCheck()` before
// any state transition. Keep the core dispatcher safe; the exported adapter
// below retains the foreign-call boundary.
pub extern "C" fn deflate(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm).is_none()
        || flush > crate::zlib_h::Z_BLOCK
        || flush < 0 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let (stream, state) = deflateStateCheck(strm).expect("stream validated above");
    let preparation = deflate_prepare_call(stream, state, flush);
    if let Some(result) = deflate_finish_prepared(stream, state, flush, preparation) {
        return result;
    }
    // SAFETY: `deflateSetHeader()` retains caller-owned header storage. The
    // ABI requires those optional fields to remain valid through deflate().
    let head = unsafe { &*state.gzhead };
    let extra = if head.extra.is_null() || head.extra_len == 0 {
        None
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts(head.extra, (head.extra_len & 0xffff) as usize)
        })
    };
    let name = if head.name.is_null() {
        None
    } else {
        Some(unsafe { ::core::ffi::CStr::from_ptr(head.name as *const ::core::ffi::c_char) })
    };
    let comment = if head.comment.is_null() {
        None
    } else {
        Some(unsafe { ::core::ffi::CStr::from_ptr(head.comment as *const ::core::ffi::c_char) })
    };
    deflate_finish_bound_gzip_header(stream, state, flush, head, extra, name, comment)
}
#[export_name = "deflate"]

pub unsafe extern "C" fn deflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflate(strm, flush)
}
// The public ABI wrapper binds the foreign stream pointer.  Teardown itself
// only needs the already-owned stream and state, so keep the release plan
// reference-bound here.
pub fn deflateEnd(stream: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    let Some((stream, state)) = deflateStateCheck(stream as *mut _) else {
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
        zfree(opaque, pending_buf as crate::stdlib::voidpf);
    }
    if !head.is_null() {
        zfree(opaque, head as crate::stdlib::voidpf);
    }
    if !prev.is_null() {
        zfree(opaque, prev as crate::stdlib::voidpf);
    }
    if !window.is_null() {
        zfree(opaque, window as crate::stdlib::voidpf);
    }
    zfree(opaque, state_ptr as crate::stdlib::voidpf);
    // `deflateStateCheck()` returned this same bound stream.  Complete the
    // teardown through that reference rather than re-binding the raw ABI
    // pointer after the release callbacks have run.
    deflate_end_complete(stream, status)
}

// Gzip creates its private deflater with the default zlib callbacks.  For
// that internal close path, release the captured allocations directly after
// validation instead of routing through the public callback-based ABI.  The
// release plan is fully copied before the state allocation is freed.
pub(crate) fn deflate_end_default_bound(
    stream: &mut crate::zlib_h::z_stream,
) -> ::core::ffi::c_int {
    let (status, pending_buf, head, prev, window, state_ptr) = {
        let Some((bound_stream, state)) = deflateStateCheck(stream as *mut _) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        (
            state.status,
            state.pending_buf,
            state.head,
            state.prev,
            state.window,
            bound_stream.state,
        )
    };
    if !pending_buf.is_null() {
        crate::src::zutil::zcfree(
            ::core::ptr::null_mut(),
            pending_buf as crate::stdlib::voidpf,
        );
    }
    if !head.is_null() {
        crate::src::zutil::zcfree(::core::ptr::null_mut(), head as crate::stdlib::voidpf);
    }
    if !prev.is_null() {
        crate::src::zutil::zcfree(::core::ptr::null_mut(), prev as crate::stdlib::voidpf);
    }
    if !window.is_null() {
        crate::src::zutil::zcfree(::core::ptr::null_mut(), window as crate::stdlib::voidpf);
    }
    crate::src::zutil::zcfree(::core::ptr::null_mut(), state_ptr as crate::stdlib::voidpf);
    deflate_end_complete(stream, status)
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
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflateEnd(strm)
}
// The exported adapter below owns the foreign-call boundary. Keep this
// implementation callable through that dispatcher without exposing its raw
// stream and allocation work as an unsafe-function contract to Rust callers.
pub fn deflateCopy(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // SAFETY: this implementation preserves zlib's raw stream and allocator
    // protocol. Each raw allocation or stream binding is validated before it
    // is turned into a reference, and no such reference spans an allocator
    // callback that may inspect either stream.
    unsafe {
    // Do all source inspection before the first allocator callback.  Apart
    // from preserving zlib's observable publication order, this keeps the
    // copy plan reference-bound instead of repeatedly dereferencing the
    // source state through the allocation sequence below.
    let (pending_offset, plan) = {
        let Some((source_stream, source_state)) = deflateStateCheck(source) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if dest.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        // Publish the source stream fields before the allocator callbacks, as
        // zlib's original whole-struct copy does. A typed copy preserves every
        // observable stream field without an untyped foreign-memory operation.
        *dest = *source_stream;
        let pending_offset = source_state
            .pending_out
            .addr()
            .wrapping_sub(source_state.pending_buf.addr());
        let Some(plan) = deflate_copy_plan(source_state, pending_offset) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        (pending_offset, plan)
    };
    // Snapshot every source-derived allocation request before any callback.
    // A custom allocator is allowed to inspect the destination stream, so
    // later requests must not need to revisit the source state through raw
    // pointers.
    // Capture this allocation callback and its argument together before it
    // runs, matching the original evaluation order without retaining a
    // stream reference across the foreign callback.
    let (zalloc, opaque) = {
        let dest_stream = &mut *dest;
        (
            dest_stream.zalloc.expect("non-null function pointer"),
            dest_stream.opaque,
        )
    };
    let destination_state_ptr = Some(zalloc).expect("non-null function pointer")(
        opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::deflate_state;
    if destination_state_ptr.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (&mut *dest).state = destination_state_ptr as *mut crate::src::deflate::internal_state;
    // Copy the state before invoking the allocator again: custom allocation
    // callbacks can inspect `dest->state`, just as they can in zlib's C
    // implementation.  The assignment avoids an untyped whole-struct copy.
    {
        // The allocation callback above may inspect either stream. Rebind the
        // source after it returns before publishing the copied state.
        let Some((_source_stream, source_state)) = deflateStateCheck(source) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let destination_state = &mut *destination_state_ptr;
        deflate_copy_state(destination_state, source_state);
        destination_state.strm = dest;
    }
    // Re-read the destination callback and opaque argument before every
    // allocation. A user allocator can modify the published destination
    // stream, and zlib observes those changes on each subsequent request.
    let window = {
        let Some((dest_stream, _destination_state)) = deflateStateCheck(dest) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        Some(dest_stream.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            dest_stream.opaque,
            plan.window_items,
            (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
                as crate::stdlib::uInt,
        ) as *mut crate::stdlib::Bytef
    };
    let prev = {
        let Some((dest_stream, _destination_state)) = deflateStateCheck(dest) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        Some(dest_stream.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            dest_stream.opaque,
            plan.prev_items,
            ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::Posf
    };
    let head = {
        let Some((dest_stream, _destination_state)) = deflateStateCheck(dest) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        Some(dest_stream.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            dest_stream.opaque,
            plan.head_items,
            ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::Posf
    };
    let pending_buf = {
        let Some((dest_stream, _destination_state)) = deflateStateCheck(dest) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        Some(dest_stream.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            dest_stream.opaque,
            plan.pending_buf_items,
            4 as crate::stdlib::uInt,
        ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef
    };
    if window.is_null() || prev.is_null() || head.is_null() || pending_buf.is_null() {
        if let Some((destination_stream, _destination_state)) = deflateStateCheck(dest) {
            deflateEnd(destination_stream);
        }
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // All allocation callbacks are complete. Bind the two independently
    // owned state allocations once so the rebinding and copies below are
    // ordinary reference and slice operations instead of repeated raw-state
    // dereferences. The initial state copy intentionally remains before the
    // callbacks above, where zlib makes it observable through `dest->state`.
    let Some((source_stream, source_state)) = deflateStateCheck(source) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((destination_stream, destination_state)) = deflateStateCheck(dest) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // `pending_out` is an offset into the source pending allocation. Retain
    // that byte offset while rebinding it to the destination allocation
    // without requiring raw-pointer in-bounds arithmetic.
    destination_state.window = window;
    destination_state.prev = prev;
    destination_state.head = head;
    destination_state.pending_buf = pending_buf;
    destination_state.pending_out = pending_buf.wrapping_add(pending_offset);
    destination_state.sym_buf = pending_buf.wrapping_add(destination_state.lit_bufsize as usize)
        as *mut crate::zutil_h::uchf;
    // Reuse the common pending binder for both independently-owned pending
    // allocations. Their prevalidated copy length keeps those views bounded,
    // while `fill_window()` supplies the window and hash-table views below.
    flush_pending(
        source_state,
        source_stream,
        false,
        |source_state, source_stream, source_pending, _| {
            flush_pending(
                destination_state,
                destination_stream,
                false,
                |destination_state, destination_stream, destination_pending, _| {
                    fill_window(
                        source_state,
                        source_stream,
                        false,
                        |_, _, source_window, source_head, source_prev, _| {
                            fill_window(
                                destination_state,
                                destination_stream,
                                false,
                                |_, _, destination_window, destination_head, destination_prev, _| {
                                    deflate_copy_buffers(
                                        &plan,
                                        source_window,
                                        destination_window,
                                        source_head,
                                        destination_head,
                                        source_prev,
                                        destination_prev,
                                        source_pending,
                                        destination_pending,
                                    );
                                },
                            );
                        },
                    );
                },
            );
        },
    );
    destination_state.l_desc.dyn_tree = &raw mut destination_state.dyn_ltree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    destination_state.d_desc.dyn_tree = &raw mut destination_state.dyn_dtree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    destination_state.bl_desc.dyn_tree = &raw mut destination_state.bl_tree as *mut crate::src::deflate::ct_data_s
        as *mut crate::src::deflate::ct_data;
    return crate::zlib_h::Z_OK;
    }
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
    window_items: crate::stdlib::uInt,
    prev_items: crate::stdlib::uInt,
    head_items: crate::stdlib::uInt,
    pending_buf_items: crate::stdlib::uInt,
    window_len: usize,
    prev_len: usize,
    pending_buf_len: usize,
    pending: ::core::ops::Range<usize>,
    symbols: ::core::ops::Range<usize>,
}

fn deflate_copy_range(
    offset: usize,
    len: usize,
    allocation_len: usize,
) -> Option<::core::ops::Range<usize>> {
    let end = offset.checked_add(len)?;
    (end <= allocation_len).then_some(offset..end)
}

fn deflate_copy_plan(
    source: &crate::src::deflate::deflate_state,
    pending_offset: usize,
) -> Option<DeflateCopyPlan> {
    let prev_entries =
        if source.slid != 0 || source.strstart.wrapping_sub(source.insert) > source.w_size {
            source.w_size
        } else {
            source.strstart.wrapping_sub(source.insert)
        };
    let window_capacity = (source.w_size as usize).checked_mul(2)?;
    let prev_len = (prev_entries as usize)
        .checked_mul(::core::mem::size_of::<crate::src::deflate::Pos>())?;
    let prev_capacity = (source.w_size as usize)
        .checked_mul(::core::mem::size_of::<crate::src::deflate::Pos>())?;
    let pending_buf_len = (source.lit_bufsize as usize).checked_mul(4)?;
    let window_len = source.high_water as usize;
    if window_len > window_capacity || prev_len > prev_capacity {
        return None;
    }
    Some(DeflateCopyPlan {
        window_items: source.w_size,
        prev_items: source.w_size,
        head_items: source.hash_size,
        pending_buf_items: source.lit_bufsize,
        window_len,
        prev_len,
        pending_buf_len,
        pending: deflate_copy_range(pending_offset, source.pending as usize, pending_buf_len)?,
        symbols: deflate_copy_range(
            source.lit_bufsize as usize,
            source.sym_next as usize,
            pending_buf_len,
        )?,
    })
}

fn deflate_copy_buffers(
    plan: &DeflateCopyPlan,
    source_window: &[crate::stdlib::Bytef],
    destination_window: &mut [crate::stdlib::Bytef],
    source_head: &[crate::src::deflate::Posf],
    destination_head: &mut [crate::src::deflate::Posf],
    source_prev: &[crate::src::deflate::Posf],
    destination_prev: &mut [crate::src::deflate::Posf],
    source_pending: &[crate::stdlib::Bytef],
    destination_pending: &mut [crate::stdlib::Bytef],
) {
    destination_window[..plan.window_len].copy_from_slice(&source_window[..plan.window_len]);
    let prev_entries = plan.prev_len / ::core::mem::size_of::<crate::src::deflate::Pos>();
    destination_prev[..prev_entries].copy_from_slice(&source_prev[..prev_entries]);
    destination_head.copy_from_slice(source_head);
    destination_pending[plan.pending.clone()]
        .copy_from_slice(&source_pending[plan.pending.clone()]);
    destination_pending[plan.symbols.clone()]
        .copy_from_slice(&source_pending[plan.symbols.clone()]);
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
        if state
            .window_size
            .wrapping_sub(state.strstart as crate::zutil_h::ulg)
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
        state.insert =
            state
                .insert
                .wrapping_add(if used > state.w_size.wrapping_sub(state.insert) {
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

// The stored-block strategy receives the stream/state relationship already
// validated by the compression dispatch. Reuse the common window/input binder;
// this strategy only needs to bind its pending and output ranges directly.
fn deflate_stored(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    flush: ::core::ffi::c_int,
) -> block_state {
    // The compression dispatch invokes this only with the validated state
    // maintained by `deflate()`. The common adapter supplies its pending
    // allocation and, when present, the caller's writable output range.
    flush_pending(state, stream, true, |state, stream, pending, output| {
        deflate_stored_bound(state, stream, pending, output.unwrap_or(&mut []), flush)
    })
}

// The stored-block algorithm only needs the bounded buffers supplied by its
// binding adapter. Keeping its window/input dispatch reference-based leaves
// `deflate_stored()` responsible solely for binding its three raw ranges.
fn deflate_stored_bound(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    pending: &mut [crate::zutil_h::uch],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    fill_window(
        state,
        stream,
        true,
        |state, stream, window, _head, _prev, input| {
            deflate_stored_impl(state, stream, window, pending, input, output, flush)
        },
    )
}

// Stored and symbol-block compression share this reference-only drain path;
// `flush_pending_bound()` retains the bounded pending/output transfer.
fn flush_pending_output(
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
    deflate_output_progress(stream, copied);
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
        flush_pending_output(state, stream, pending, output, &mut output_used);
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
                deflate_output_progress(stream, copied);
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
        state.insert =
            state
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
        if let Some(block) = stored_window_bytes(window, state.block_start, len) {
            crate::src::trees::tr_stored_block(state, pending, block, last);
            state.block_start += len as ::core::ffi::c_long;
            flush_pending_output(state, stream, pending, output, &mut output_used);
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

// `deflate_fast` is the established configuration-table target for fast
// compression. Reuse the common window/hash/input binder used by the stored
// strategy and the pending/output binder used by stored blocks. The symbol
// overlay is the one additional allocation view this dispatcher needs.
fn deflate_fast(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    flush: ::core::ffi::c_int,
) -> block_state {
    // SAFETY: `sym_buf` is the configured symbol overlay within the validated
    // deflater's pending allocation. `flush_pending()` binds that allocation
    // and the caller output cursor for the bounded strategy dispatch below.
    let symbols = unsafe {
        let symbols = ::core::slice::from_raw_parts_mut(
            state.sym_buf,
            state.lit_bufsize.wrapping_mul(3) as usize,
        );
        symbols
    };
    flush_pending(state, stream, true, |state, stream, pending, output| {
        deflate_fast_bound(
            state,
            stream,
            pending,
            symbols,
            output.unwrap_or(&mut []),
            flush,
        )
    })
}

// Strategy selection and compression operate only on the views already bound
// by `deflate_fast()`. Separating that dispatch keeps the raw binding scope
// from growing with each strategy implementation.
fn deflate_fast_bound(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    pending: &mut [crate::zutil_h::uch],
    symbols: &mut [crate::zutil_h::uchf],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
    fill_window(
        state,
        stream,
        true,
        |state, stream, window, head, prev, input| {
            if state.strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
                deflate_huff_impl(
                    state, stream, window, head, prev, pending, symbols, input, output, flush,
                )
            } else if state.strategy == crate::zlib_h::Z_RLE {
                deflate_rle_impl(
                    state, stream, window, head, prev, pending, symbols, input, output, flush,
                )
            } else if state.level <= 3 {
                deflate_fast_impl(
                    state, stream, window, head, prev, pending, symbols, input, output, flush,
                )
            } else {
                deflate_slow_impl(
                    state, stream, window, head, prev, pending, symbols, input, output, flush,
                )
            }
        },
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
                state
                    .match_length
                    .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt),
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
                state,
                stream,
                window,
                pending,
                symbols,
                output,
                &mut output_used,
                0,
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
            state,
            stream,
            window,
            pending,
            symbols,
            output,
            &mut output_used,
            1,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return finish_started;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_symbol_block(
            state,
            stream,
            window,
            pending,
            symbols,
            output,
            &mut output_used,
            0,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return need_more;
        }
    }
    block_done
}

// The lazy strategy receives validated state and stream references, then
// dispatches through the shared raw-buffer binder.
fn deflate_slow(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    flush: ::core::ffi::c_int,
) -> block_state {
    deflate_fast(state, stream, flush)
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
                    state.match_length = (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int)
                        as crate::stdlib::uInt;
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
                state
                    .prev_length
                    .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt),
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
                    state,
                    stream,
                    window,
                    pending,
                    symbols,
                    output,
                    &mut output_used,
                    0,
                );
                if stream.avail_out == 0 as crate::stdlib::uInt {
                    return need_more;
                }
            }
        } else if state.match_available != 0 {
            let cc = literal_byte(
                window,
                state.strstart.wrapping_sub(1 as crate::stdlib::uInt),
            );
            let bflush =
                crate::src::trees::tally_bound(state, symbols, 0, cc as ::core::ffi::c_uint);
            if bflush != 0 {
                flush_symbol_block(
                    state,
                    stream,
                    window,
                    pending,
                    symbols,
                    output,
                    &mut output_used,
                    0,
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
            state,
            stream,
            window,
            pending,
            symbols,
            output,
            &mut output_used,
            1,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return finish_started;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_symbol_block(
            state,
            stream,
            window,
            pending,
            symbols,
            output,
            &mut output_used,
            0,
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
    let limit =
        ::core::cmp::min(lookahead, crate::zutil_h::MAX_MATCH as crate::stdlib::uInt) as usize;

    if window[start] != previous || window[start + 1] != previous || window[start + 2] != previous {
        return 0;
    }

    let mut length = crate::zutil_h::MIN_MATCH as usize;
    while length < limit && window[start + length] == previous {
        length += 1;
    }
    length as crate::stdlib::uInt
}

// The RLE strategy receives validated state and stream references, then
// dispatches through the shared raw-buffer binder.
fn deflate_rle(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    flush: ::core::ffi::c_int,
) -> block_state {
    deflate_fast(state, stream, flush)
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
                state
                    .match_length
                    .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt),
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
                state,
                stream,
                window,
                pending,
                symbols,
                output,
                &mut output_used,
                0,
            );
            if stream.avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
        }
    }
    state.insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        flush_symbol_block(
            state,
            stream,
            window,
            pending,
            symbols,
            output,
            &mut output_used,
            1,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return finish_started;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_symbol_block(
            state,
            stream,
            window,
            pending,
            symbols,
            output,
            &mut output_used,
            0,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return need_more;
        }
    }
    block_done
}

// The Huffman-only strategy receives validated state and stream references,
// then dispatches through the shared raw-buffer binder.
fn deflate_huff(
    state: &mut crate::src::deflate::deflate_state,
    stream: &mut crate::zlib_h::z_stream,
    flush: ::core::ffi::c_int,
) -> block_state {
    deflate_fast(state, stream, flush)
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
    let stored_len =
        (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg;
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
    flush_pending_output(state, stream, pending, output, output_used);
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
                state,
                stream,
                window,
                pending,
                symbols,
                output,
                &mut output_used,
                0,
            );
            if stream.avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
        }
    }
    state.insert = 0 as crate::stdlib::uInt;
    if flush == crate::zlib_h::Z_FINISH {
        flush_symbol_block(
            state,
            stream,
            window,
            pending,
            symbols,
            output,
            &mut output_used,
            1,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return finish_started;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_symbol_block(
            state,
            stream,
            window,
            pending,
            symbols,
            output,
            &mut output_used,
            0,
        );
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return need_more;
        }
    }
    block_done
}
