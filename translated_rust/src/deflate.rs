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
#[derive(Clone)]
#[repr(C)]

pub struct internal_state {
    pub status: ::core::ffi::c_int,
    pub pending_buf_size: crate::zutil_h::ulg,
    // Offset of the first pending byte in `deflate_buffers::pending`.
    pub pending_out: usize,
    pub pending: crate::zutil_h::ulg,
    pub wrap: ::core::ffi::c_int,
    pub gzindex: crate::zutil_h::ulg,
    pub method: crate::stdlib::Byte,
    pub last_flush: ::core::ffi::c_int,
    // Tree construction updates this without borrowing the FFI stream.
    pub data_type: ::core::ffi::c_int,
    pub w_size: crate::stdlib::uInt,
    pub w_bits: crate::stdlib::uInt,
    pub w_mask: crate::stdlib::uInt,
    pub window_size: crate::zutil_h::ulg,
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
    // `z_stream::state` is opaque at the ABI boundary.  Keep the translated
    // cursors above while code is migrated, but make their backing allocation
    // Rust-owned so implementations can borrow slices instead of rebuilding
    // them from allocator-owned raw pointers.
    buffers: Option<Box<deflate_buffers>>,
}

#[derive(Clone)]
struct deflate_buffers {
    pending: Vec<crate::stdlib::Bytef>,
    sym_buf: Vec<crate::zutil_h::uchf>,
    window: Vec<crate::stdlib::Bytef>,
    prev: Vec<crate::src::deflate::Posf>,
    head: Vec<crate::src::deflate::Posf>,
    gzip_header: Option<gzip_header>,
}

/// The fast compressor only needs a cursor over the caller's input and a
/// bounded destination for pending bytes.  Keeping those cursors owned while
/// it runs lets the compressor itself work entirely with slices.  Conversion
/// to and from the ABI's raw `z_stream` pointers is deliberately kept at the
/// `deflate` boundary.
struct deflate_io {
    input: Vec<crate::stdlib::Bytef>,
    input_pos: usize,
    output: Vec<crate::stdlib::Bytef>,
    output_capacity: usize,
    adler: crate::stdlib::uLong,
    data_type: ::core::ffi::c_int,
}

impl deflate_io {
    fn new(
        input: Vec<crate::stdlib::Bytef>,
        output_capacity: usize,
        adler: crate::stdlib::uLong,
    ) -> Self {
        Self {
            input,
            input_pos: 0,
            output: Vec::with_capacity(output_capacity),
            output_capacity,
            adler,
            data_type: crate::zlib_h::Z_UNKNOWN,
        }
    }

    fn avail_in(&self) -> crate::stdlib::uInt {
        self.input.len().saturating_sub(self.input_pos) as crate::stdlib::uInt
    }

    fn avail_out(&self) -> crate::stdlib::uInt {
        self.output_capacity.saturating_sub(self.output.len()) as crate::stdlib::uInt
    }

    fn read_into(
        &mut self,
        wrap: ::core::ffi::c_int,
        destination: &mut [crate::stdlib::Bytef],
    ) -> crate::stdlib::uInt {
        let len = destination
            .len()
            .min(self.input.len().saturating_sub(self.input_pos));
        if len == 0 {
            return 0;
        }
        let input_end = self.input_pos + len;
        let input = &self.input[self.input_pos..input_end];
        destination[..len].copy_from_slice(input);
        if wrap == 1 {
            self.adler = crate::src::adler32::adler32(self.adler, input);
        } else if wrap == 2 {
            self.adler = crc32_slice(self.adler, input);
        }
        self.input_pos = input_end;
        len as crate::stdlib::uInt
    }

    fn write(&mut self, source: &[crate::stdlib::Bytef]) {
        debug_assert!(source.len() <= self.output_capacity.saturating_sub(self.output.len()));
        self.output.extend_from_slice(source);
    }

    fn copy_input_to_output(&mut self, wrap: ::core::ffi::c_int, len: usize) {
        let end = self.input_pos + len;
        let input = &self.input[self.input_pos..end];
        if wrap == 1 {
            self.adler = crate::src::adler32::adler32(self.adler, input);
        } else if wrap == 2 {
            self.adler = crc32_slice(self.adler, input);
        }
        self.output.extend_from_slice(input);
        self.input_pos = end;
    }

    fn consumed_tail(&self, len: usize) -> &[crate::stdlib::Bytef] {
        &self.input[self.input_pos - len..self.input_pos]
    }
}

/// zlib's CRC update is the reflected IEEE CRC-32 recurrence.  This small
/// slice helper avoids sending the fast path back through the raw-pointer CRC
/// compatibility entry point.
fn crc32_slice(
    adler: crate::stdlib::uLong,
    bytes: &[crate::stdlib::Bytef],
) -> crate::stdlib::uLong {
    let mut crc = !(adler as u32);
    for &byte in bytes {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    (!crc) as crate::stdlib::uLong
}

impl deflate_buffers {
    fn gzip_header(&self) -> Option<&gzip_header> {
        self.gzip_header.as_ref()
    }

    fn zeroed<T: Default + Clone>(len: usize) -> Result<Vec<T>, ()> {
        let mut values = Vec::new();
        values.try_reserve_exact(len).map_err(|_| ())?;
        values.resize(len, T::default());
        Ok(values)
    }

    fn new(
        w_size: crate::stdlib::uInt,
        hash_size: crate::stdlib::uInt,
        lit_bufsize: crate::stdlib::uInt,
    ) -> Result<Self, ()> {
        let sym_len = lit_bufsize.wrapping_sub(1).wrapping_mul(3) as usize;
        Ok(Self {
            pending: Self::zeroed(lit_bufsize as usize * 4)?,
            sym_buf: Self::zeroed(sym_len)?,
            window: Self::zeroed(w_size as usize * 2)?,
            prev: Self::zeroed(w_size as usize)?,
            head: Self::zeroed(hash_size as usize)?,
            gzip_header: None,
        })
    }
}

static EMPTY_STATIC_TREE_DESC: crate::src::deflate::static_tree_desc_s =
    crate::src::deflate::static_tree_desc_s {
        static_tree: None,
        extra_bits: &[],
        extra_base: 0,
        elems: 0,
        max_length: 0,
    };

const EMPTY_CT_DATA: ct_data_s = ct_data_s {
    fc: C2Rust_Unnamed_1 { freq: 0 },
    dl: C2Rust_Unnamed_0 { dad: 0 },
};

impl Default for internal_state {
    fn default() -> Self {
        Self {
            status: 0,
            pending_buf_size: 0,
            pending_out: 0,
            pending: 0,
            wrap: 0,
            gzindex: 0,
            method: 0,
            last_flush: 0,
            data_type: crate::zlib_h::Z_UNKNOWN,
            w_size: 0,
            w_bits: 0,
            w_mask: 0,
            window_size: 0,
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
            dyn_ltree: [EMPTY_CT_DATA; 573],
            dyn_dtree: [EMPTY_CT_DATA; 61],
            bl_tree: [EMPTY_CT_DATA; 39],
            l_desc: tree_desc_s {
                max_code: 0,
                stat_desc: &EMPTY_STATIC_TREE_DESC,
            },
            d_desc: tree_desc_s {
                max_code: 0,
                stat_desc: &EMPTY_STATIC_TREE_DESC,
            },
            bl_desc: tree_desc_s {
                max_code: 0,
                stat_desc: &EMPTY_STATIC_TREE_DESC,
            },
            bl_count: [0; 16],
            heap: [0; 573],
            heap_len: 0,
            heap_max: 0,
            depth: [0; 573],
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
            buffers: None,
        }
    }
}

impl internal_state {
    fn buffers(&self) -> &deflate_buffers {
        self.buffers
            .as_deref()
            .expect("deflate buffers initialized")
    }

    fn buffers_mut(&mut self) -> &mut deflate_buffers {
        self.buffers
            .as_deref_mut()
            .expect("deflate buffers initialized")
    }

    /// Run a pending-buffer operation while borrowing the rest of the state.
    /// Moving the Vec out briefly lets the callback update state and the
    /// buffer without fabricating two mutable aliases from a raw pointer.
    pub(crate) fn with_pending<R>(
        &mut self,
        operation: impl FnOnce(&mut Self, &mut [crate::stdlib::Bytef]) -> R,
    ) -> R {
        let mut pending = std::mem::take(&mut self.buffers_mut().pending);
        let result = operation(self, pending.as_mut_slice());
        self.buffers_mut().pending = pending;
        result
    }

    fn push_pending(&mut self, byte: crate::stdlib::Bytef) {
        let index = self.pending as usize;
        self.buffers_mut().pending[index] = byte;
        self.pending = self.pending.wrapping_add(1);
    }

    fn clear_head(&mut self) {
        self.buffers_mut()
            .head
            .fill(NIL as crate::src::deflate::Posf);
    }

    fn insert_hash_at(&mut self, str: crate::stdlib::uInt) -> crate::src::deflate::IPos {
        let window_index =
            str.wrapping_add((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt) as usize;
        self.ins_h = ((self.ins_h << self.hash_shift)
            ^ self.buffers().window[window_index] as crate::stdlib::uInt)
            & self.hash_mask;
        let prev_index = (str & self.w_mask) as usize;
        let ins_h = self.ins_h as usize;
        let buffers = self.buffers_mut();
        let hash_head = buffers.head[ins_h];
        buffers.prev[prev_index] = hash_head;
        buffers.head[ins_h] = str as crate::src::deflate::Posf;
        hash_head as crate::src::deflate::IPos
    }

    fn slide_hash(&mut self) {
        let wsize = self.w_size as ::core::ffi::c_uint;
        let buffers = self.buffers_mut();
        for value in buffers.head.iter_mut().chain(buffers.prev.iter_mut()) {
            let value_as_uint = *value as ::core::ffi::c_uint;
            *value = if value_as_uint >= wsize {
                value_as_uint.wrapping_sub(wsize) as crate::src::deflate::Posf
            } else {
                NIL as crate::src::deflate::Posf
            };
        }
        self.slid = 1;
    }

    pub(crate) fn symbol_slice(&self) -> &[crate::zutil_h::uchf] {
        self.buffers
            .as_deref()
            .expect("deflate buffers initialized")
            .sym_buf
            .as_slice()
    }

    pub(crate) fn tally_symbol(
        &mut self,
        dist: ::core::ffi::c_uint,
        lc: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int {
        let Self {
            buffers,
            sym_next,
            dyn_ltree,
            dyn_dtree,
            matches,
            ..
        } = self;
        let sym_buf = buffers
            .as_deref_mut()
            .expect("deflate buffers initialized")
            .sym_buf
            .as_mut_slice();
        crate::src::trees::_tr_tally(sym_buf, sym_next, dyn_ltree, dyn_dtree, matches, dist, lc)
    }
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CompressionFunction {
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
    pub func: CompressionFunction,
}
#[no_mangle]

pub static mut deflate_copyright: [::core::ffi::c_char; 70] = unsafe {
    ::core::mem::transmute::<[u8; 70], [::core::ffi::c_char; 70]>(
        *b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0",
    )
};

pub const NIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TOO_FAR: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

static configuration_table: [config; 10] = [
    config_s {
        good_length: 0 as crate::zutil_h::ush,
        max_lazy: 0 as crate::zutil_h::ush,
        nice_length: 0 as crate::zutil_h::ush,
        max_chain: 0 as crate::zutil_h::ush,
        func: CompressionFunction::Stored,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        func: CompressionFunction::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        func: CompressionFunction::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: CompressionFunction::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        func: CompressionFunction::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: CompressionFunction::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        func: CompressionFunction::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        func: CompressionFunction::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        func: CompressionFunction::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        func: CompressionFunction::Slow,
    },
];

unsafe extern "C" fn read_buf(
    strm: &mut crate::zlib_h::z_stream,
    mut buf: *mut crate::stdlib::Bytef,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut len: ::core::ffi::c_uint = strm.avail_in as ::core::ffi::c_uint;
    if len > size {
        len = size;
    }
    if len == 0 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_uint;
    }
    strm.avail_in = strm.avail_in.wrapping_sub(len);
    crate::stdlib::memcpy(
        buf as *mut ::core::ffi::c_void,
        strm.next_in as *const ::core::ffi::c_void,
        len as crate::__stddef_size_t_h::size_t,
    );
    if (*strm.state).wrap == 1 as ::core::ffi::c_int {
        strm.adler = crate::src::adler32::adler32(
            strm.adler,
            ::core::slice::from_raw_parts(buf, len as usize),
        );
    } else if (*strm.state).wrap == 2 as ::core::ffi::c_int {
        strm.adler = crate::src::crc32::crc32(strm.adler, buf, len as crate::stdlib::uInt);
    }
    strm.next_in = strm.next_in.offset(len as isize);
    strm.total_in = strm.total_in.wrapping_add(len as crate::stdlib::uLong);
    return len;
}

unsafe extern "C" fn fill_window(
    mut s: *mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
) {
    let s = &mut *s;
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
                s.buffers_mut().window.as_mut_ptr() as *mut ::core::ffi::c_void,
                s.buffers().window.as_ptr().wrapping_add(wsize as usize)
                    as *const ::core::ffi::c_void,
                wsize.wrapping_sub(more) as crate::__stddef_size_t_h::size_t,
            );
            (*s).match_start = (*s).match_start.wrapping_sub(wsize);
            (*s).strstart = (*s).strstart.wrapping_sub(wsize);
            (*s).block_start -= wsize as ::core::ffi::c_long;
            if (*s).insert > (*s).strstart {
                (*s).insert = (*s).strstart;
            }
            (&mut *s).slide_hash();
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if strm.avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        n = read_buf(
            strm,
            s.buffers_mut()
                .window
                .as_mut_ptr()
                .wrapping_add(s.strstart.wrapping_add(s.lookahead) as usize),
            more,
        );
        (*s).lookahead = (*s).lookahead.wrapping_add(n);
        if (*s).lookahead.wrapping_add((*s).insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        {
            let mut str: crate::stdlib::uInt = (*s).strstart.wrapping_sub((*s).insert);
            s.ins_h = s.buffers().window[str as usize] as crate::stdlib::uInt;
            s.ins_h = (s.ins_h << s.hash_shift
                ^ s.buffers().window[str.wrapping_add(1) as usize] as crate::stdlib::uInt)
                & s.hash_mask;
            while (*s).insert != 0 {
                s.ins_h = (s.ins_h << s.hash_shift
                    ^ s.buffers().window[str.wrapping_add(2) as usize] as crate::stdlib::uInt)
                    & s.hash_mask;
                s.insert_hash_at(str);
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
            && strm.avail_in != 0 as crate::stdlib::uInt)
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
                s.buffers_mut()
                    .window
                    .as_mut_ptr()
                    .wrapping_add(curr as usize) as *mut ::core::ffi::c_void,
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
                s.buffers_mut()
                    .window
                    .as_mut_ptr()
                    .wrapping_add(s.high_water as usize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                init as ::core::ffi::c_uint as crate::__stddef_size_t_h::size_t,
            );
            (*s).high_water = (*s).high_water.wrapping_add(init);
        }
    }
}

fn fill_window_fast(s: &mut crate::src::deflate::deflate_state, io: &mut deflate_io) {
    let wsize = s.w_size;
    loop {
        let mut more =
            s.window_size
                .wrapping_sub(s.lookahead as crate::zutil_h::ulg)
                .wrapping_sub(s.strstart as crate::zutil_h::ulg) as crate::stdlib::uInt;
        if ::core::mem::size_of::<::core::ffi::c_int>() <= 2 {
            if more == 0 && s.strstart == 0 && s.lookahead == 0 {
                more = wsize;
            } else if more == -1i32 as crate::stdlib::uInt {
                more = more.wrapping_sub(1);
            }
        }
        if s.strstart
            >= wsize.wrapping_add(
                s.w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            let copy_len = wsize.wrapping_sub(more) as usize;
            s.buffers_mut()
                .window
                .copy_within(wsize as usize..wsize as usize + copy_len, 0);
            s.match_start = s.match_start.wrapping_sub(wsize);
            s.strstart = s.strstart.wrapping_sub(wsize);
            s.block_start -= wsize as ::core::ffi::c_long;
            if s.insert > s.strstart {
                s.insert = s.strstart;
            }
            s.slide_hash();
            more = more.wrapping_add(wsize);
        }
        if io.avail_in() == 0 {
            break;
        }
        let start = s.strstart.wrapping_add(s.lookahead) as usize;
        let end = start + more as usize;
        let wrap = s.wrap;
        let n = {
            let window = &mut s.buffers_mut().window[start..end];
            io.read_into(wrap, window)
        };
        s.lookahead = s.lookahead.wrapping_add(n);
        if s.lookahead.wrapping_add(s.insert) >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut str = s.strstart.wrapping_sub(s.insert);
            s.ins_h = s.buffers().window[str as usize] as crate::stdlib::uInt;
            s.ins_h = (s.ins_h << s.hash_shift
                ^ s.buffers().window[str.wrapping_add(1) as usize] as crate::stdlib::uInt)
                & s.hash_mask;
            while s.insert != 0 {
                s.ins_h = (s.ins_h << s.hash_shift
                    ^ s.buffers().window[str.wrapping_add(2) as usize] as crate::stdlib::uInt)
                    & s.hash_mask;
                s.insert_hash_at(str);
                str = str.wrapping_add(1);
                s.insert = s.insert.wrapping_sub(1);
                if s.lookahead.wrapping_add(s.insert)
                    < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                {
                    break;
                }
            }
        }
        if !(s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
            && io.avail_in() != 0)
        {
            break;
        }
    }
    if s.high_water < s.window_size {
        let curr =
            (s.strstart as crate::zutil_h::ulg).wrapping_add(s.lookahead as crate::zutil_h::ulg);
        let init = if s.high_water < curr {
            s.window_size
                .wrapping_sub(curr)
                .min(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
        } else if s.high_water
            < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
        {
            curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
                .wrapping_sub(s.high_water)
                .min(s.window_size.wrapping_sub(s.high_water))
        } else {
            0
        };
        let start = if s.high_water < curr {
            curr
        } else {
            s.high_water
        } as usize;
        if init != 0 {
            s.buffers_mut().window[start..start + init as usize].fill(0);
            s.high_water = if s.high_water < curr {
                curr.wrapping_add(init)
            } else {
                s.high_water.wrapping_add(init)
            };
        }
    }
}

/// Fill the match window from a preset dictionary.  Unlike the normal input
/// path this deliberately does not update a stream checksum or any stream
/// cursors: a dictionary is history, not compressed input.
fn fill_window_from_dictionary(
    s: &mut crate::src::deflate::deflate_state,
    dictionary: &[crate::stdlib::Bytef],
    input_pos: &mut usize,
) {
    let wsize = s.w_size;
    loop {
        let mut more =
            s.window_size
                .wrapping_sub(s.lookahead as crate::zutil_h::ulg)
                .wrapping_sub(s.strstart as crate::zutil_h::ulg) as crate::stdlib::uInt;
        if ::core::mem::size_of::<::core::ffi::c_int>() <= 2 {
            if more == 0 && s.strstart == 0 && s.lookahead == 0 {
                more = wsize;
            } else if more == -1i32 as crate::stdlib::uInt {
                more = more.wrapping_sub(1);
            }
        }
        if s.strstart
            >= wsize.wrapping_add(
                s.w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            let copy_len = wsize.wrapping_sub(more) as usize;
            s.buffers_mut()
                .window
                .copy_within(wsize as usize..wsize as usize + copy_len, 0);
            s.match_start = s.match_start.wrapping_sub(wsize);
            s.strstart = s.strstart.wrapping_sub(wsize);
            s.block_start -= wsize as ::core::ffi::c_long;
            if s.insert > s.strstart {
                s.insert = s.strstart;
            }
            s.slide_hash();
            more = more.wrapping_add(wsize);
        }
        if *input_pos == dictionary.len() {
            break;
        }
        let start = s.strstart.wrapping_add(s.lookahead) as usize;
        let len = (more as usize).min(dictionary.len() - *input_pos);
        s.buffers_mut().window[start..start + len]
            .copy_from_slice(&dictionary[*input_pos..*input_pos + len]);
        *input_pos += len;
        s.lookahead = s.lookahead.wrapping_add(len as crate::stdlib::uInt);
        if s.lookahead.wrapping_add(s.insert) >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut str = s.strstart.wrapping_sub(s.insert);
            s.ins_h = s.buffers().window[str as usize] as crate::stdlib::uInt;
            s.ins_h = (s.ins_h << s.hash_shift
                ^ s.buffers().window[str.wrapping_add(1) as usize] as crate::stdlib::uInt)
                & s.hash_mask;
            while s.insert != 0 {
                s.ins_h = (s.ins_h << s.hash_shift
                    ^ s.buffers().window[str.wrapping_add(2) as usize] as crate::stdlib::uInt)
                    & s.hash_mask;
                s.insert_hash_at(str);
                str = str.wrapping_add(1);
                s.insert = s.insert.wrapping_sub(1);
                if s.lookahead.wrapping_add(s.insert)
                    < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                {
                    break;
                }
            }
        }
        if !(s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
            && *input_pos != dictionary.len())
        {
            break;
        }
    }
    if s.high_water < s.window_size {
        let curr =
            (s.strstart as crate::zutil_h::ulg).wrapping_add(s.lookahead as crate::zutil_h::ulg);
        let init = if s.high_water < curr {
            s.window_size
                .wrapping_sub(curr)
                .min(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
        } else if s.high_water
            < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
        {
            curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
                .wrapping_sub(s.high_water)
                .min(s.window_size.wrapping_sub(s.high_water))
        } else {
            0
        };
        let start = if s.high_water < curr {
            curr
        } else {
            s.high_water
        } as usize;
        if init != 0 {
            s.buffers_mut().window[start..start + init as usize].fill(0);
            s.high_water = if s.high_water < curr {
                curr.wrapping_add(init)
            } else {
                s.high_water.wrapping_add(init)
            };
        }
    }
}
pub fn deflateInit_(
    strm: &mut crate::zlib_h::z_stream,
    level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return deflateInit2_(
        strm,
        level,
        crate::zlib_h::Z_DEFLATED,
        crate::stdlib::MAX_WBITS,
        crate::zutil_h::DEF_MEM_LEVEL,
        crate::zlib_h::Z_DEFAULT_STRATEGY,
    );
}
#[export_name = "deflateInit_"]

pub unsafe extern "C" fn deflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if version.is_null()
        || *version != crate::zlib_h::ZLIB_VERSION[0]
        || stream_size as usize != ::core::mem::size_of::<crate::zlib_h::z_stream>()
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflateInit_(&mut *strm, level)
}
pub fn deflateInit2_(
    strm: &mut crate::zlib_h::z_stream,
    mut level: ::core::ffi::c_int,
    mut method: ::core::ffi::c_int,
    mut windowBits: ::core::ffi::c_int,
    mut memLevel: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wrap: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if strm.zalloc.is_none() {
        strm.zalloc = Some(
            crate::src::zutil::zcalloc
                as unsafe extern "C" fn(
                    crate::stdlib::voidpf,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> crate::stdlib::voidpf,
        ) as crate::zlib_h::alloc_func;
        strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if strm.zfree.is_none() {
        strm.zfree = Some(
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
    let mut owned = Box::new(crate::src::deflate::internal_state::default());
    owned.status = crate::src::deflate::INIT_STATE;
    owned.wrap = wrap;
    // The owned gzip-header snapshot starts unset.
    owned.w_bits = windowBits as crate::stdlib::uInt;
    owned.w_size = ((1 as ::core::ffi::c_int) << owned.w_bits) as crate::stdlib::uInt;
    owned.w_mask = owned.w_size.wrapping_sub(1 as crate::stdlib::uInt);
    owned.hash_bits = (memLevel as crate::stdlib::uInt).wrapping_add(7 as crate::stdlib::uInt);
    owned.hash_size = ((1 as ::core::ffi::c_int) << owned.hash_bits) as crate::stdlib::uInt;
    owned.hash_mask = owned.hash_size.wrapping_sub(1 as crate::stdlib::uInt);
    owned.hash_shift = owned
        .hash_bits
        .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
    owned.high_water = 0 as crate::zutil_h::ulg;
    owned.lit_bufsize =
        ((1 as ::core::ffi::c_int) << memLevel + 6 as ::core::ffi::c_int) as crate::stdlib::uInt;
    owned.buffers = match crate::src::deflate::deflate_buffers::new(
        owned.w_size,
        owned.hash_size,
        owned.lit_bufsize,
    ) {
        Ok(buffers) => Some(Box::new(buffers)),
        Err(()) => {
            // `z_errmsg` is still a legacy mutable C export. Its fixed
            // memory-error entry is read only at this FFI-facing boundary.
            strm.msg = unsafe { crate::src::zutil::z_errmsg[6] };
            return crate::zlib_h::Z_MEM_ERROR;
        }
    };
    owned.sym_end = owned
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    owned.level = level;
    owned.strategy = strategy;
    owned.method = method as crate::stdlib::Byte;
    let result = deflateResetKeep(strm, &mut owned, true);
    strm.state = Box::into_raw(owned);
    result
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
    if version.is_null()
        || *version != crate::zlib_h::ZLIB_VERSION[0]
        || stream_size as usize != ::core::mem::size_of::<crate::zlib_h::z_stream>()
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflateInit2_(&mut *strm, level, method, windowBits, memLevel, strategy)
}
unsafe extern "C" fn deflateStateCheck(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as ::core::ffi::c_int;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    if s.is_null()
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

fn deflate_state_is_valid(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::deflate::deflate_state,
) -> bool {
    strm.zalloc.is_some()
        && strm.zfree.is_some()
        && matches!(
            state.status,
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
pub fn deflateSetDictionary(
    strm: &mut crate::zlib_h::z_stream,
    s: &mut crate::src::deflate::deflate_state,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid(strm, s) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let wrap = s.wrap;
    if wrap == 2 || (wrap == 1 && s.status != crate::src::deflate::INIT_STATE) || s.lookahead != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if wrap == 1 {
        strm.adler = crate::src::adler32::adler32(strm.adler, dictionary);
    }
    s.wrap = 0;
    let dictionary = if dictionary.len() >= s.w_size as usize {
        if wrap == 0 {
            s.clear_head();
            s.slid = 0;
            s.strstart = 0;
            s.block_start = 0;
            s.insert = 0;
        }
        &dictionary[dictionary.len() - s.w_size as usize..]
    } else {
        dictionary
    };
    let mut input_pos = 0;
    fill_window_from_dictionary(s, dictionary, &mut input_pos);
    while s.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        let mut string = s.strstart;
        let mut remaining = s
            .lookahead
            .wrapping_sub((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt);
        loop {
            s.ins_h = (s.ins_h << s.hash_shift
                ^ s.buffers().window[string
                    .wrapping_add((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt)
                    as usize] as crate::stdlib::uInt)
                & s.hash_mask;
            s.insert_hash_at(string);
            string = string.wrapping_add(1);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
        s.strstart = string;
        s.lookahead = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
        fill_window_from_dictionary(s, dictionary, &mut input_pos);
    }
    s.strstart = s.strstart.wrapping_add(s.lookahead);
    s.block_start = s.strstart as ::core::ffi::c_long;
    s.insert = s.lookahead;
    s.lookahead = 0;
    s.prev_length = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
    s.match_length = s.prev_length;
    s.match_available = 0;
    s.wrap = wrap;
    crate::zlib_h::Z_OK
}
#[export_name = "deflateSetDictionary"]

pub unsafe extern "C" fn deflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if strm.is_null() || dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let Some(state) = strm.state.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let dictionary = ::core::slice::from_raw_parts(dictionary, dictLength as usize);
    deflateSetDictionary(strm, state, dictionary)
}
pub fn deflateGetDictionary<'a>(
    strm: &crate::zlib_h::z_stream,
    s: &'a crate::src::deflate::deflate_state,
) -> Result<&'a [crate::stdlib::Bytef], ::core::ffi::c_int> {
    if !deflate_state_is_valid(strm, s) {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    let len = s.strstart.wrapping_add(s.lookahead).min(s.w_size) as usize;
    let end = s.strstart.wrapping_add(s.lookahead) as usize;
    Ok(&s.buffers().window[end - len..end])
}
#[export_name = "deflateGetDictionary"]

pub unsafe extern "C" fn deflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let Some(state) = strm.state.as_ref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let dictionary_bytes = match deflateGetDictionary(strm, state) {
        Ok(dictionary_bytes) => dictionary_bytes,
        Err(code) => return code,
    };
    if !dictionary.is_null() && !dictionary_bytes.is_empty() {
        ::core::slice::from_raw_parts_mut(dictionary, dictionary_bytes.len())
            .copy_from_slice(dictionary_bytes);
    }
    if !dictLength.is_null() {
        *dictLength = dictionary_bytes.len() as crate::stdlib::uInt;
    }
    crate::zlib_h::Z_OK
}
pub fn deflateResetKeep(
    strm: &mut crate::zlib_h::z_stream,
    s: &mut crate::src::deflate::deflate_state,
    reset_match_finder: bool,
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid(strm, s) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    strm.total_out = 0 as crate::stdlib::uLong;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = crate::zlib_h::Z_UNKNOWN;
    s.data_type = crate::zlib_h::Z_UNKNOWN;
    s.pending = 0 as crate::zutil_h::ulg;
    s.pending_out = 0;
    if s.wrap < 0 as ::core::ffi::c_int {
        s.wrap = -s.wrap;
    }
    s.status = if s.wrap == 2 as ::core::ffi::c_int {
        crate::src::deflate::GZIP_STATE
    } else {
        crate::src::deflate::INIT_STATE
    };
    strm.adler = if s.wrap == 2 as ::core::ffi::c_int {
        crc32_slice(0, &[])
    } else {
        crate::src::adler32::ADLER32_INITIAL
    };
    s.last_flush = -2 as ::core::ffi::c_int;
    crate::src::trees::_tr_init(s);
    if reset_match_finder {
        lm_init(s);
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *strm.state;
    deflateResetKeep(strm, state, false)
}
fn lm_init(s: &mut crate::src::deflate::deflate_state) {
    s.pending_buf_size = s.buffers().pending.len() as crate::zutil_h::ulg;
    s.pending_out = 0;
    s.window_size = (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
        .wrapping_mul(s.w_size as crate::zutil_h::ulg);
    s.buffers
        .as_mut()
        .expect("deflate buffers initialized")
        .head
        .fill(NIL as crate::src::deflate::Posf);
    s.slid = 0 as ::core::ffi::c_int;
    s.max_lazy_match = configuration_table[s.level as usize].max_lazy as crate::stdlib::uInt;
    s.good_match = configuration_table[s.level as usize].good_length as crate::stdlib::uInt;
    s.nice_match = configuration_table[s.level as usize].nice_length as ::core::ffi::c_int;
    s.max_chain_length = configuration_table[s.level as usize].max_chain as crate::stdlib::uInt;
    s.strstart = 0 as crate::stdlib::uInt;
    s.block_start = 0 as ::core::ffi::c_long;
    s.lookahead = 0 as crate::stdlib::uInt;
    s.insert = 0 as crate::stdlib::uInt;
    s.prev_length = (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
    s.match_length = s.prev_length;
    s.match_available = 0 as ::core::ffi::c_int;
    s.ins_h = 0 as crate::stdlib::uInt;
}
pub fn deflateReset(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    unsafe {
        let Some(state) = strm.state.as_mut() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        deflateResetKeep(strm, state, true)
    }
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

#[derive(Clone)]
struct gzip_header {
    text: ::core::ffi::c_int,
    time: crate::stdlib::uLong,
    os: ::core::ffi::c_int,
    extra: Option<Vec<crate::stdlib::Bytef>>,
    name: Option<Vec<crate::stdlib::Bytef>>,
    comment: Option<Vec<crate::stdlib::Bytef>>,
    hcrc: ::core::ffi::c_int,
}

impl gzip_header {
    fn copy_bytes(
        bytes: Option<&[crate::stdlib::Bytef]>,
    ) -> Result<Option<Vec<crate::stdlib::Bytef>>, ()> {
        let Some(bytes) = bytes else {
            return Ok(None);
        };
        let mut copied = Vec::new();
        copied.try_reserve_exact(bytes.len()).map_err(|_| ())?;
        copied.extend_from_slice(bytes);
        Ok(Some(copied))
    }

    fn from_parts(
        text: ::core::ffi::c_int,
        time: crate::stdlib::uLong,
        os: ::core::ffi::c_int,
        extra: Option<&[crate::stdlib::Bytef]>,
        name: Option<&[crate::stdlib::Bytef]>,
        comment: Option<&[crate::stdlib::Bytef]>,
        hcrc: ::core::ffi::c_int,
    ) -> Result<Self, ()> {
        Ok(Self {
            text,
            time,
            os,
            extra: Self::copy_bytes(extra)?,
            name: Self::copy_bytes(name)?,
            comment: Self::copy_bytes(comment)?,
            hcrc,
        })
    }
}

#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 || (*(*strm).state).wrap != 2 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let head = if head.is_null() {
        None
    } else {
        let head = &*head;
        let extra = if head.extra.is_null() {
            None
        } else {
            Some(::core::slice::from_raw_parts(
                head.extra,
                head.extra_len as usize,
            ))
        };
        let name = if head.name.is_null() {
            None
        } else {
            Some(std::ffi::CStr::from_ptr(head.name.cast()).to_bytes_with_nul())
        };
        let comment = if head.comment.is_null() {
            None
        } else {
            Some(std::ffi::CStr::from_ptr(head.comment.cast()).to_bytes_with_nul())
        };
        match gzip_header::from_parts(
            head.text, head.time, head.os, extra, name, comment, head.hcrc,
        ) {
            Ok(head) => Some(head),
            Err(()) => return crate::zlib_h::Z_MEM_ERROR,
        }
    };
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    state
        .buffers
        .as_mut()
        .expect("deflate buffers initialized")
        .gzip_header = head;
    crate::zlib_h::Z_OK
}
pub fn deflatePending(
    strm: &crate::zlib_h::z_stream,
    s: &crate::src::deflate::deflate_state,
) -> Result<(crate::zutil_h::ulg, ::core::ffi::c_int), ::core::ffi::c_int> {
    if !deflate_state_is_valid(strm, s) {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok((s.pending, s.bi_valid))
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
    let strm = &mut *strm;
    let state = &*strm.state;
    let Ok((pending_count, bit_count)) = deflatePending(strm, state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !bits.is_null() {
        *bits = bit_count;
    }
    if !pending.is_null() {
        *pending = pending_count as ::core::ffi::c_uint;
        if *pending as crate::zutil_h::ulg != pending_count {
            *pending = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    crate::zlib_h::Z_OK
}
pub fn deflateUsed(
    strm: &crate::zlib_h::z_stream,
    s: &crate::src::deflate::deflate_state,
) -> Result<::core::ffi::c_int, ::core::ffi::c_int> {
    if !deflate_state_is_valid(strm, s) {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok(s.bi_used)
}
#[export_name = "deflateUsed"]

pub unsafe extern "C" fn deflateUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &*strm.state;
    let Ok(bit_count) = deflateUsed(strm, state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !bits.is_null() {
        *bits = bit_count;
    }
    crate::zlib_h::Z_OK
}
pub fn deflatePrime(
    strm: &mut crate::zlib_h::z_stream,
    s: &mut crate::src::deflate::deflate_state,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut put: ::core::ffi::c_int = 0;
    if !deflate_state_is_valid(strm, s) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let required = ((crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int)
        >> 3 as ::core::ffi::c_int) as usize;
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || s.pending_out
            .checked_add(required)
            .map_or(true, |end| end > s.lit_bufsize as usize)
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    loop {
        put = crate::src::deflate::Buf_size - s.bi_valid;
        if put > bits {
            put = bits;
        }
        s.bi_buf = (s.bi_buf as ::core::ffi::c_int
            | ((value & ((1 as ::core::ffi::c_int) << put) - 1 as ::core::ffi::c_int) << s.bi_valid)
                as crate::zutil_h::ush as ::core::ffi::c_int)
            as crate::zutil_h::ush;
        s.bi_valid += put;
        s.with_pending(|state, pending_buf| crate::src::trees::bi_flush(state, pending_buf));
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
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *strm.state;
    deflatePrime(strm, state, bits, value)
}
pub fn deflateParams(
    strm: &mut crate::zlib_h::z_stream,
    s: &mut crate::src::deflate::deflate_state,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    let func = configuration_table[s.level as usize].func;
    if (strategy != s.strategy || func != configuration_table[level as usize].func)
        && s.last_flush != -2 as ::core::ffi::c_int
    {
        let err = deflate(strm, crate::zlib_h::Z_BLOCK);
        if err == crate::zlib_h::Z_STREAM_ERROR {
            return err;
        }
        if strm.avail_in != 0
            || s.strstart as ::core::ffi::c_long - s.block_start
                + s.lookahead as ::core::ffi::c_long
                != 0
        {
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    if s.level != level {
        if s.level == 0 as ::core::ffi::c_int && s.matches != 0 as crate::stdlib::uInt {
            if s.matches == 1 as crate::stdlib::uInt {
                s.slide_hash();
            } else {
                s.clear_head();
                s.slid = 0 as ::core::ffi::c_int;
            }
            s.matches = 0 as crate::stdlib::uInt;
        }
        s.level = level;
        s.max_lazy_match = configuration_table[level as usize].max_lazy as crate::stdlib::uInt;
        s.good_match = configuration_table[level as usize].good_length as crate::stdlib::uInt;
        s.nice_match = configuration_table[level as usize].nice_length as ::core::ffi::c_int;
        s.max_chain_length = configuration_table[level as usize].max_chain as crate::stdlib::uInt;
    }
    s.strategy = strategy;
    crate::zlib_h::Z_OK
}
#[export_name = "deflateParams"]

pub unsafe extern "C" fn deflateParams_ffi(
    strm: crate::zlib_h::z_streamp,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *strm.state;
    deflateParams(strm, state, level, strategy)
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
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflateTune(
        &mut *(*strm).state,
        good_length,
        max_lazy,
        nice_length,
        max_chain,
    )
}
pub fn deflateBound_z(
    strm: Option<(
        &crate::zlib_h::z_stream,
        &crate::src::deflate::deflate_state,
    )>,
    sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let mut fixedlen = sourceLen
        .wrapping_add(sourceLen >> 3 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 8 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 9 as ::core::ffi::c_int)
        .wrapping_add(4 as crate::stdlib::z_size_t);
    if fixedlen < sourceLen {
        fixedlen = -1 as ::core::ffi::c_int as crate::stdlib::z_size_t;
    }
    let mut storelen = sourceLen
        .wrapping_add(sourceLen >> 5 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 7 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 11 as ::core::ffi::c_int)
        .wrapping_add(7 as crate::stdlib::z_size_t);
    if storelen < sourceLen {
        storelen = -1 as ::core::ffi::c_int as crate::stdlib::z_size_t;
    }
    let Some((_stream, s)) = strm.filter(|(stream, state)| deflate_state_is_valid(stream, state))
    else {
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
    let wraplen = match if s.wrap < 0 as ::core::ffi::c_int {
        -s.wrap
    } else {
        s.wrap
    } {
        0 => 0 as crate::stdlib::z_size_t,
        1 => {
            (6 as ::core::ffi::c_int
                + (if s.strstart != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as crate::stdlib::z_size_t
        }
        2 => {
            let mut wraplen = 18 as crate::stdlib::z_size_t;
            if let Some(gzhead) = s.buffers.as_deref().and_then(deflate_buffers::gzip_header) {
                if let Some(extra) = gzhead.extra.as_ref() {
                    wraplen = wraplen.wrapping_add(
                        (2 as crate::stdlib::uInt).wrapping_add(extra.len() as crate::stdlib::uInt)
                            as crate::stdlib::z_size_t,
                    );
                }
                if let Some(name) = gzhead.name.as_ref() {
                    wraplen = wraplen.wrapping_add(name.len() as crate::stdlib::z_size_t);
                }
                if let Some(comment) = gzhead.comment.as_ref() {
                    wraplen = wraplen.wrapping_add(comment.len() as crate::stdlib::z_size_t);
                }
                if gzhead.hcrc != 0 {
                    wraplen = wraplen.wrapping_add(2 as crate::stdlib::z_size_t);
                }
            }
            wraplen
        }
        _ => 18 as crate::stdlib::z_size_t,
    };
    if s.w_bits != 15 as crate::stdlib::uInt
        || s.hash_bits != (8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        let bound = if s.w_bits <= s.hash_bits && s.level != 0 {
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
    let bound = sourceLen
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
    strm: crate::zlib_h::z_streamp,
    sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let stream = if strm.is_null() {
        None
    } else {
        let stream = &*strm;
        if stream.zalloc.is_none() || stream.zfree.is_none() || stream.state.is_null() {
            None
        } else {
            Some((stream, &*stream.state))
        }
    };
    deflateBound_z(stream, sourceLen)
}
pub fn deflateBound(
    strm: Option<(
        &crate::zlib_h::z_stream,
        &crate::src::deflate::deflate_state,
    )>,
    sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let bound = deflateBound_z(strm, sourceLen as crate::stdlib::z_size_t);
    crate::stdlib::uLong::try_from(bound).unwrap_or(crate::stdlib::uLong::MAX)
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    strm: crate::zlib_h::z_streamp,
    sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let stream = if strm.is_null() {
        None
    } else {
        let stream = &*strm;
        if stream.zalloc.is_none() || stream.zfree.is_none() || stream.state.is_null() {
            None
        } else {
            Some((stream, &*stream.state))
        }
    };
    deflateBound(stream, sourceLen)
}
fn putShortMSB(s: &mut crate::src::deflate::deflate_state, b: crate::stdlib::uInt) {
    s.push_pending((b >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte);
    s.push_pending((b & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte);
}

unsafe extern "C" fn flush_pending(strm: &mut crate::zlib_h::z_stream) {
    let mut len: ::core::ffi::c_uint = 0;
    let s = &mut *strm.state;
    s.with_pending(|state, pending_buf| crate::src::trees::bi_flush(state, pending_buf));
    strm.data_type = s.data_type;
    len = if s.pending > strm.avail_out as crate::zutil_h::ulg {
        strm.avail_out as ::core::ffi::c_uint
    } else {
        s.pending as ::core::ffi::c_uint
    };
    if len == 0 as ::core::ffi::c_uint {
        return;
    }
    let output_end = s
        .pending_out
        .checked_add(len as usize)
        .expect("pending output overflow");
    {
        let pending_output = &s
            .buffers
            .as_ref()
            .expect("deflate buffers initialized")
            .pending[s.pending_out..output_end];
        crate::stdlib::memcpy(
            strm.next_out as *mut ::core::ffi::c_void,
            pending_output.as_ptr() as *const ::core::ffi::c_void,
            len as crate::__stddef_size_t_h::size_t,
        );
    }
    strm.next_out = strm.next_out.offset(len as isize);
    s.pending_out = output_end;
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out = strm.avail_out.wrapping_sub(len);
    s.pending = s.pending.wrapping_sub(len as crate::zutil_h::ulg);
    if s.pending == 0 as crate::zutil_h::ulg {
        s.pending_out = 0;
    }
}

fn flush_pending_io(s: &mut crate::src::deflate::deflate_state, io: &mut deflate_io) {
    s.with_pending(|state, pending_buf| crate::src::trees::bi_flush(state, pending_buf));
    io.data_type = s.data_type;
    let len = (s.pending as usize).min(io.avail_out() as usize);
    if len == 0 {
        return;
    }
    let output_end = s
        .pending_out
        .checked_add(len)
        .expect("pending output overflow");
    io.write(&s.buffers().pending[s.pending_out..output_end]);
    s.pending_out = output_end;
    s.pending = s.pending.wrapping_sub(len as crate::zutil_h::ulg);
    if s.pending == 0 {
        s.pending_out = 0;
    }
}

pub fn deflate(
    strm: &mut crate::zlib_h::z_stream,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut old_flush: ::core::ffi::c_int = 0;
        let mut s: *mut crate::src::deflate::deflate_state =
            ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
        if deflateStateCheck(strm) != 0
            || flush > crate::zlib_h::Z_BLOCK
            || flush < 0 as ::core::ffi::c_int
        {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        s = strm.state as *mut crate::src::deflate::deflate_state;
        if strm.next_out.is_null()
            || strm.avail_in != 0 as crate::stdlib::uInt && strm.next_in.is_null()
            || (*s).status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH
        {
            strm.msg = crate::src::zutil::z_errmsg[(if (-2 as ::core::ffi::c_int)
                < -6 as ::core::ffi::c_int
                || -2 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -2 as ::core::ffi::c_int
            }) as usize];
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
            let state = &mut *s;
            let mut header: crate::stdlib::uInt =
                (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt).wrapping_add(
                    state.w_bits.wrapping_sub(8 as crate::stdlib::uInt) << 4 as ::core::ffi::c_int,
                ) << 8 as ::core::ffi::c_int;
            let mut level_flags: crate::stdlib::uInt = 0;
            if state.strategy >= crate::zlib_h::Z_HUFFMAN_ONLY
                || state.level < 2 as ::core::ffi::c_int
            {
                level_flags = 0 as crate::stdlib::uInt;
            } else if state.level < 6 as ::core::ffi::c_int {
                level_flags = 1 as crate::stdlib::uInt;
            } else if state.level == 6 as ::core::ffi::c_int {
                level_flags = 2 as crate::stdlib::uInt;
            } else {
                level_flags = 3 as crate::stdlib::uInt;
            }
            header |= level_flags << 6 as ::core::ffi::c_int;
            if state.strstart != 0 as crate::stdlib::uInt {
                header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
            }
            header = header.wrapping_add(
                (31 as crate::stdlib::uInt)
                    .wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
            );
            putShortMSB(state, header);
            if state.strstart != 0 as crate::stdlib::uInt {
                putShortMSB(
                    state,
                    ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
                );
                putShortMSB(
                    state,
                    ((*strm).adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
                );
            }
            (*strm).adler = crate::src::adler32::ADLER32_INITIAL;
            state.status = crate::src::deflate::BUSY_STATE;
            flush_pending(strm);
            if state.pending != 0 as crate::zutil_h::ulg {
                state.last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        }
        if (*s).status == crate::src::deflate::GZIP_STATE {
            // A CRC over no bytes is zero.  Keep this header state machine on
            // the slice-based checksum path, rather than going back through
            // the raw-pointer compatibility entry point.
            (*strm).adler = crc32_slice(0, &[]);
            (&mut *s).push_pending(31);
            (&mut *s).push_pending(139);
            (&mut *s).push_pending(8);
            if (*s)
                .buffers
                .as_deref()
                .and_then(deflate_buffers::gzip_header)
                .is_none()
            {
                (&mut *s).push_pending(0);
                (&mut *s).push_pending(0);
                (&mut *s).push_pending(0);
                (&mut *s).push_pending(0);
                (&mut *s).push_pending(0);
                (&mut *s).push_pending(
                    (if (*s).level == 9 as ::core::ffi::c_int {
                        2 as ::core::ffi::c_int
                    } else if (*s).strategy >= 2 as ::core::ffi::c_int
                        || (*s).level < 2 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as crate::stdlib::Bytef,
                );
                (&mut *s).push_pending(3);
                (*s).status = crate::src::deflate::BUSY_STATE;
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            } else {
                let gzhead = (*s)
                    .buffers
                    .as_deref()
                    .and_then(deflate_buffers::gzip_header)
                    .expect("gzip header is present")
                    .clone();
                (&mut *s).push_pending(
                    ((if gzhead.text != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) + (if gzhead.hcrc != 0 {
                        2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) + (if gzhead.extra.is_none() {
                        0 as ::core::ffi::c_int
                    } else {
                        4 as ::core::ffi::c_int
                    }) + (if gzhead.name.is_none() {
                        0 as ::core::ffi::c_int
                    } else {
                        8 as ::core::ffi::c_int
                    }) + (if gzhead.comment.is_none() {
                        0 as ::core::ffi::c_int
                    } else {
                        16 as ::core::ffi::c_int
                    })) as crate::stdlib::Bytef,
                );
                (&mut *s).push_pending(
                    (gzhead.time & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                );
                (&mut *s).push_pending(
                    (gzhead.time >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                        as crate::stdlib::Byte,
                );
                (&mut *s).push_pending(
                    (gzhead.time >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                        as crate::stdlib::Byte,
                );
                (&mut *s).push_pending(
                    (gzhead.time >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                        as crate::stdlib::Byte,
                );
                (&mut *s).push_pending(
                    (if (*s).level == 9 as ::core::ffi::c_int {
                        2 as ::core::ffi::c_int
                    } else if (*s).strategy >= 2 as ::core::ffi::c_int
                        || (*s).level < 2 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as crate::stdlib::Bytef,
                );
                (&mut *s)
                    .push_pending((gzhead.os & 0xff as ::core::ffi::c_int) as crate::stdlib::Bytef);
                if let Some(extra) = gzhead.extra.as_ref() {
                    (&mut *s).push_pending(
                        (extra.len() as crate::stdlib::uInt & 0xff as crate::stdlib::uInt)
                            as crate::stdlib::Bytef,
                    );
                    (&mut *s).push_pending(
                        ((extra.len() as crate::stdlib::uInt) >> 8 as ::core::ffi::c_int
                            & 0xff as crate::stdlib::uInt)
                            as crate::stdlib::Bytef,
                    );
                }
                if gzhead.hcrc != 0 {
                    (*strm).adler = crc32_slice(
                        (*strm).adler,
                        &(*s).buffers().pending[..(*s).pending as usize],
                    );
                }
                (*s).gzindex = 0 as crate::zutil_h::ulg;
                (*s).status = crate::src::deflate::EXTRA_STATE;
            }
        }
        if (*s).status == crate::src::deflate::EXTRA_STATE {
            if let Some(extra) = (*s)
                .buffers
                .as_deref()
                .and_then(deflate_buffers::gzip_header)
                .and_then(|header| header.extra.as_ref())
            {
                let mut beg: crate::zutil_h::ulg = (*s).pending;
                let mut left: crate::zutil_h::ulg = (((extra.len() as crate::stdlib::uInt)
                    & 0xffff as crate::stdlib::uInt)
                    as crate::zutil_h::ulg)
                    .wrapping_sub((*s).gzindex);
                while (*s).pending.wrapping_add(left) > (*s).pending_buf_size {
                    let mut copy: crate::zutil_h::ulg =
                        (*s).pending_buf_size.wrapping_sub((*s).pending);
                    let pending_start = (*s).pending as usize;
                    let pending_end = pending_start + copy as usize;
                    let extra_start = (*s).gzindex as usize;
                    let extra_end = extra_start + copy as usize;
                    (*s).buffers
                        .as_mut()
                        .expect("deflate buffers initialized")
                        .pending[pending_start..pending_end]
                        .copy_from_slice(&extra[extra_start..extra_end]);
                    (*s).pending = (*s).pending_buf_size;
                    if (*s)
                        .buffers
                        .as_deref()
                        .and_then(deflate_buffers::gzip_header)
                        .expect("gzip header is present")
                        .hcrc
                        != 0
                        && (*s).pending > beg
                    {
                        (*strm).adler = crc32_slice(
                            (*strm).adler,
                            &(*s).buffers().pending[beg as usize..(*s).pending as usize],
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
                let pending_start = (*s).pending as usize;
                let pending_end = pending_start + left as usize;
                let extra_start = (*s).gzindex as usize;
                let extra_end = extra_start + left as usize;
                (*s).buffers
                    .as_mut()
                    .expect("deflate buffers initialized")
                    .pending[pending_start..pending_end]
                    .copy_from_slice(&extra[extra_start..extra_end]);
                (*s).pending = (*s).pending.wrapping_add(left);
                if (*s)
                    .buffers
                    .as_deref()
                    .and_then(deflate_buffers::gzip_header)
                    .expect("gzip header is present")
                    .hcrc
                    != 0
                    && (*s).pending > beg
                {
                    (*strm).adler = crc32_slice(
                        (*strm).adler,
                        &(*s).buffers().pending[beg as usize..(*s).pending as usize],
                    );
                }
                (*s).gzindex = 0 as crate::zutil_h::ulg;
            }
            (*s).status = crate::src::deflate::NAME_STATE;
        }
        if (*s).status == crate::src::deflate::NAME_STATE {
            if let Some(name) = (*s)
                .buffers
                .as_deref()
                .and_then(deflate_buffers::gzip_header)
                .and_then(|header| header.name.as_ref())
            {
                let mut beg_0: crate::zutil_h::ulg = (*s).pending;
                let mut val: ::core::ffi::c_int = 0;
                loop {
                    if (*s).pending == (*s).pending_buf_size {
                        if (*s)
                            .buffers
                            .as_deref()
                            .and_then(deflate_buffers::gzip_header)
                            .expect("gzip header is present")
                            .hcrc
                            != 0
                            && (*s).pending > beg_0
                        {
                            (*strm).adler = crc32_slice(
                                (*strm).adler,
                                &(*s).buffers().pending[beg_0 as usize..(*s).pending as usize],
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
                    val = name[c2rust_fresh19 as usize] as ::core::ffi::c_int;
                    (&mut *s).push_pending(val as crate::stdlib::Bytef);
                    if val == 0 as ::core::ffi::c_int {
                        break;
                    }
                }
                if (*s)
                    .buffers
                    .as_deref()
                    .and_then(deflate_buffers::gzip_header)
                    .expect("gzip header is present")
                    .hcrc
                    != 0
                    && (*s).pending > beg_0
                {
                    (*strm).adler = crc32_slice(
                        (*strm).adler,
                        &(*s).buffers().pending[beg_0 as usize..(*s).pending as usize],
                    );
                }
                (*s).gzindex = 0 as crate::zutil_h::ulg;
            }
            (*s).status = crate::src::deflate::COMMENT_STATE;
        }
        if (*s).status == crate::src::deflate::COMMENT_STATE {
            if let Some(comment) = (*s)
                .buffers
                .as_deref()
                .and_then(deflate_buffers::gzip_header)
                .and_then(|header| header.comment.as_ref())
            {
                let mut beg_1: crate::zutil_h::ulg = (*s).pending;
                let mut val_0: ::core::ffi::c_int = 0;
                loop {
                    if (*s).pending == (*s).pending_buf_size {
                        if (*s)
                            .buffers
                            .as_deref()
                            .and_then(deflate_buffers::gzip_header)
                            .expect("gzip header is present")
                            .hcrc
                            != 0
                            && (*s).pending > beg_1
                        {
                            (*strm).adler = crc32_slice(
                                (*strm).adler,
                                &(*s).buffers().pending[beg_1 as usize..(*s).pending as usize],
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
                    val_0 = comment[c2rust_fresh21 as usize] as ::core::ffi::c_int;
                    (&mut *s).push_pending(val_0 as crate::stdlib::Bytef);
                    if val_0 == 0 as ::core::ffi::c_int {
                        break;
                    }
                }
                if (*s)
                    .buffers
                    .as_deref()
                    .and_then(deflate_buffers::gzip_header)
                    .expect("gzip header is present")
                    .hcrc
                    != 0
                    && (*s).pending > beg_1
                {
                    (*strm).adler = crc32_slice(
                        (*strm).adler,
                        &(*s).buffers().pending[beg_1 as usize..(*s).pending as usize],
                    );
                }
            }
            (*s).status = crate::src::deflate::HCRC_STATE;
        }
        if (*s).status == crate::src::deflate::HCRC_STATE {
            if (*s)
                .buffers
                .as_deref()
                .and_then(deflate_buffers::gzip_header)
                .expect("gzip header is present")
                .hcrc
                != 0
            {
                if (*s).pending.wrapping_add(2 as crate::zutil_h::ulg) > (*s).pending_buf_size {
                    flush_pending(strm);
                    if (*s).pending != 0 as crate::zutil_h::ulg {
                        (*s).last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                }
                (&mut *s).push_pending(
                    ((*strm).adler & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                );
                (&mut *s).push_pending(
                    ((*strm).adler >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                        as crate::stdlib::Byte,
                );
                (*strm).adler = crc32_slice(0, &[]);
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
            || flush != crate::zlib_h::Z_NO_FLUSH
                && (*s).status != crate::src::deflate::FINISH_STATE
        {
            let mut bstate: block_state = need_more;
            let level = (*s).level;
            let compression = configuration_table[level as usize].func;
            let input = if (*strm).avail_in == 0 {
                Vec::new()
            } else {
                ::core::slice::from_raw_parts((*strm).next_in, (*strm).avail_in as usize).to_vec()
            };
            let mut io = deflate_io::new(input, (*strm).avail_out as usize, (*strm).adler);
            let state = &mut *s;
            bstate = if level != 0 && state.strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
                deflate_huff(state, &mut io, flush)
            } else if level != 0 && state.strategy == crate::zlib_h::Z_RLE {
                deflate_rle(state, &mut io, flush)
            } else {
                match compression {
                    CompressionFunction::Stored => deflate_stored(state, &mut io, flush),
                    CompressionFunction::Fast => deflate_fast(state, &mut io, flush),
                    CompressionFunction::Slow => deflate_slow(state, &mut io, flush),
                }
            };
            ::core::slice::from_raw_parts_mut((*strm).next_out, io.output_capacity)
                [..io.output.len()]
                .copy_from_slice(&io.output);
            (*strm).next_in = (*strm).next_in.offset(io.input_pos as isize);
            (*strm).avail_in = io.avail_in();
            (*strm).total_in = (*strm)
                .total_in
                .wrapping_add(io.input_pos as crate::stdlib::uLong);
            (*strm).next_out = (*strm).next_out.offset(io.output.len() as isize);
            (*strm).avail_out = io.avail_out();
            (*strm).total_out = (*strm)
                .total_out
                .wrapping_add(io.output.len() as crate::stdlib::uLong);
            (*strm).adler = io.adler;
            (*strm).data_type = io.data_type;
            if bstate as ::core::ffi::c_uint
                == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
                || bstate as ::core::ffi::c_uint
                    == finish_done as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*s).status = crate::src::deflate::FINISH_STATE;
            }
            if bstate as ::core::ffi::c_uint
                == need_more as ::core::ffi::c_int as ::core::ffi::c_uint
                || bstate as ::core::ffi::c_uint
                    == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*strm).avail_out == 0 as crate::stdlib::uInt {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                }
                return crate::zlib_h::Z_OK;
            }
            if bstate as ::core::ffi::c_uint
                == block_done as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if flush == crate::zlib_h::Z_PARTIAL_FLUSH {
                    let state = &mut *s;
                    state.with_pending(|state, pending_buf| {
                        crate::src::trees::_tr_align(state, pending_buf)
                    });
                } else if flush != crate::zlib_h::Z_BLOCK {
                    let state = &mut *s;
                    state.with_pending(|state, pending_buf| {
                        crate::src::trees::_tr_stored_block(state, pending_buf, &[], 0)
                    });
                    if flush == crate::zlib_h::Z_FULL_FLUSH {
                        state.clear_head();
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
            for shift in [0, 8, 16, 24] {
                (&mut *s).push_pending(((*strm).adler >> shift & 0xff) as crate::stdlib::Byte);
            }
            for shift in [0, 8, 16, 24] {
                (&mut *s).push_pending(((*strm).total_in >> shift & 0xff) as crate::stdlib::Byte);
            }
        } else {
            putShortMSB(
                &mut *s,
                ((*strm).adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
            );
            putShortMSB(
                &mut *s,
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
}
#[export_name = "deflate"]

pub unsafe extern "C" fn deflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflate(&mut *strm, flush)
}
pub unsafe extern "C" fn deflateEnd(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = Box::from_raw((*strm).state as *mut crate::src::deflate::deflate_state);
    status = state.status;
    drop(state);
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
pub fn deflateCopy(
    source: &crate::src::deflate::deflate_state,
) -> Box<crate::src::deflate::deflate_state> {
    let pending_offset = source.pending_out;
    let mut copied = source.clone();
    copied.pending_buf_size = copied.buffers().pending.len() as crate::zutil_h::ulg;
    copied.pending_out = pending_offset;
    Box::new(copied)
}
#[export_name = "deflateCopy"]

pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if dest.is_null() || source.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let source = &*source;
    if source.state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source_state = &*source.state;
    if !deflate_state_is_valid(source, source_state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let copied = deflateCopy(source_state);
    let dest = &mut *dest;
    *dest = *source;
    dest.state = Box::into_raw(copied);
    crate::zlib_h::Z_OK
}
fn longest_match_in_buffers(
    window: &[crate::stdlib::Bytef],
    prev: &[crate::src::deflate::Posf],
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    max_chain_length: crate::stdlib::uInt,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    w_size: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    mut cur_match: crate::src::deflate::IPos,
) -> (crate::stdlib::uInt, crate::stdlib::uInt) {
    let mut chain_length = max_chain_length as ::core::ffi::c_uint;
    let mut best_len = prev_length as usize;
    let mut nice_match = nice_match as usize;
    let limit = if strstart
        > w_size.wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
    {
        strstart.wrapping_sub(
            w_size.wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        ) as crate::src::deflate::IPos
    } else {
        NIL as crate::src::deflate::IPos
    };
    let scan = strstart as usize;
    let max_len = crate::zutil_h::MAX_MATCH as usize;
    let mut match_start = 0;

    if prev_length >= good_match {
        chain_length >>= 2;
    }
    nice_match = nice_match.min(lookahead as usize);

    loop {
        let candidate = cur_match as usize;
        if window[candidate + best_len] == window[scan + best_len]
            && window[candidate + best_len - 1] == window[scan + best_len - 1]
            && window[candidate] == window[scan]
            && window[candidate + 1] == window[scan + 1]
        {
            let mut len = 2;
            while len < max_len && window[candidate + len] == window[scan + len] {
                len += 1;
            }
            if len > best_len {
                match_start = cur_match as crate::stdlib::uInt;
                best_len = len;
                if len >= nice_match {
                    break;
                }
            }
        }
        cur_match =
            prev[(cur_match as crate::stdlib::uInt & w_mask) as usize] as crate::src::deflate::IPos;
        chain_length = chain_length.wrapping_sub(1);
        if cur_match <= limit || chain_length == 0 {
            break;
        }
    }

    (
        best_len.min(lookahead as usize) as crate::stdlib::uInt,
        match_start,
    )
}

fn longest_match(
    s: &mut crate::src::deflate::deflate_state,
    cur_match: crate::src::deflate::IPos,
) -> crate::stdlib::uInt {
    let buffers = s.buffers.as_ref().expect("deflate buffers initialized");
    let (length, match_start) = longest_match_in_buffers(
        &buffers.window,
        &buffers.prev,
        s.strstart,
        s.lookahead,
        s.prev_length,
        s.max_chain_length,
        s.good_match,
        s.nice_match,
        s.w_size,
        s.w_mask,
        cur_match,
    );
    if match_start != 0 {
        s.match_start = match_start;
    }
    length
}

/// Encode the current block from a borrowed window slice.  Keeping the block
/// bytes independent of the mutable compressor state avoids a raw alias into
/// the owned window while tree construction updates that state.
fn flush_block(
    state: &mut crate::src::deflate::deflate_state,
    stored: Option<&[crate::stdlib::Bytef]>,
    last: ::core::ffi::c_int,
) {
    let sym_buf = state.symbol_slice()[..state.sym_next as usize].to_vec();
    state.with_pending(|state, pending_buf| {
        crate::src::trees::tr_flush_block_safe(state, pending_buf, &sym_buf, stored, last)
    });
}

/// Encode a block after taking a snapshot of the relevant owned window range.
fn encode_block_from_window(
    state: &mut crate::src::deflate::deflate_state,
    last: ::core::ffi::c_int,
) {
    let stored = if state.block_start >= 0 {
        let start = state.block_start as usize;
        let end = start + (state.strstart as ::core::ffi::c_long - state.block_start) as usize;
        Some(state.buffers().window[start..end].to_vec())
    } else {
        None
    };
    flush_block(state, stored.as_deref(), last);
}

fn rle_match_length(
    window: &[crate::stdlib::Bytef],
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    if lookahead < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt || strstart == 0 {
        return 0;
    }
    let start = strstart as usize;
    let byte = window[start - 1];
    if window[start] != byte || window[start + 1] != byte || window[start + 2] != byte {
        return 0;
    }
    let mut length = crate::zutil_h::MIN_MATCH as usize;
    while length < crate::zutil_h::MAX_MATCH as usize && window[start + length] == byte {
        length += 1;
    }
    length.min(lookahead as usize) as crate::stdlib::uInt
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

fn deflate_stored(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut deflate_io,
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut min_block = s
        .pending_buf_size
        .saturating_sub(5)
        .min(s.w_size as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used = io.avail_in();
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        have = (s.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
            >> 3 as ::core::ffi::c_int;
        if io.avail_out() < have {
            break;
        }
        have = io.avail_out().wrapping_sub(have);
        left = (s.strstart as ::core::ffi::c_long - s.block_start) as ::core::ffi::c_uint;
        if len as crate::zutil_h::ulg
            > (left as crate::zutil_h::ulg).wrapping_add(io.avail_in() as crate::zutil_h::ulg)
        {
            len = (left as crate::stdlib::uInt).wrapping_add(io.avail_in()) as ::core::ffi::c_uint;
        }
        if len > have {
            len = have;
        }
        if len < min_block
            && (len == 0 as ::core::ffi::c_uint && flush != crate::zlib_h::Z_FINISH
                || flush == crate::zlib_h::Z_NO_FLUSH
                || len != (left as crate::stdlib::uInt).wrapping_add(io.avail_in()))
        {
            break;
        }
        last = if flush == crate::zlib_h::Z_FINISH
            && len == (left as crate::stdlib::uInt).wrapping_add(io.avail_in())
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let prefix_start = s.pending as usize;
        let (prefix, prefix_len) = crate::src::trees::stored_block_prefix(s, last);
        s.buffers_mut().pending[prefix_start..prefix_start + prefix_len]
            .copy_from_slice(&prefix[..prefix_len]);
        let stored_len = len as crate::zutil_h::ush;
        let header_start = s.pending as usize;
        let header = [
            stored_len as crate::stdlib::Bytef,
            (stored_len >> 8) as crate::stdlib::Bytef,
            (!stored_len) as crate::stdlib::Bytef,
            ((!stored_len) >> 8) as crate::stdlib::Bytef,
        ];
        s.buffers_mut().pending[header_start..header_start + header.len()].copy_from_slice(&header);
        s.pending = s.pending.wrapping_add(4);
        flush_pending_io(s, io);
        if left != 0 {
            if left > len {
                left = len;
            }
            let start = s.block_start as usize;
            let end = start + left as usize;
            let stored = s.buffers().window[start..end].to_vec();
            io.write(&stored);
            s.block_start += left as ::core::ffi::c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            io.copy_input_to_output(s.wrap, len as usize);
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub(io.avail_in());
    if used != 0 {
        let consumed = io.consumed_tail(used as usize).to_vec();
        if used >= s.w_size {
            s.matches = 2;
            let w_size = s.w_size as usize;
            let start = consumed.len() - w_size;
            s.buffers_mut().window[..w_size].copy_from_slice(&consumed[start..]);
            s.strstart = s.w_size;
            s.insert = s.strstart;
        } else {
            if s.window_size
                .wrapping_sub(s.strstart as crate::zutil_h::ulg)
                <= used as crate::zutil_h::ulg
            {
                s.strstart = s.strstart.wrapping_sub(s.w_size);
                let w_size = s.w_size as usize;
                let strstart = s.strstart as usize;
                s.buffers_mut()
                    .window
                    .copy_within(w_size..w_size + strstart, 0);
                if s.matches < 2 {
                    s.matches = s.matches.wrapping_add(1);
                }
                if s.insert > s.strstart {
                    s.insert = s.strstart;
                }
            }
            let start = s.strstart as usize;
            let end = start + used as usize;
            s.buffers_mut().window[start..end].copy_from_slice(&consumed);
            s.strstart = s.strstart.wrapping_add(used);
            s.insert = s
                .insert
                .wrapping_add(if used > s.w_size.wrapping_sub(s.insert) {
                    (s.w_size as ::core::ffi::c_uint).wrapping_sub(s.insert as ::core::ffi::c_uint)
                } else {
                    used
                });
        }
        s.block_start = s.strstart as ::core::ffi::c_long;
    }
    if s.high_water < s.strstart as crate::zutil_h::ulg {
        s.high_water = s.strstart as crate::zutil_h::ulg;
    }
    if last != 0 {
        s.bi_used = 8;
        return finish_done;
    }
    if flush != crate::zlib_h::Z_NO_FLUSH
        && flush != crate::zlib_h::Z_FINISH
        && io.avail_in() == 0
        && s.strstart as ::core::ffi::c_long == s.block_start
    {
        return block_done;
    }
    have = s
        .window_size
        .wrapping_sub(s.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if io.avail_in() > have && s.block_start >= s.w_size as ::core::ffi::c_long {
        s.block_start -= s.w_size as ::core::ffi::c_long;
        s.strstart = s.strstart.wrapping_sub(s.w_size);
        let w_size = s.w_size as usize;
        let strstart = s.strstart as usize;
        s.buffers_mut()
            .window
            .copy_within(w_size..w_size + strstart, 0);
        if s.matches < 2 {
            s.matches = s.matches.wrapping_add(1);
        }
        have = have.wrapping_add(s.w_size);
        if s.insert > s.strstart {
            s.insert = s.strstart;
        }
    }
    if have > io.avail_in() {
        have = io.avail_in();
    }
    if have != 0 {
        let strstart = s.strstart as usize;
        let end = strstart + have as usize;
        let wrap = s.wrap;
        io.read_into(wrap, &mut s.buffers_mut().window[strstart..end]);
        s.strstart = s.strstart.wrapping_add(have);
        s.insert = s
            .insert
            .wrapping_add(if have > s.w_size.wrapping_sub(s.insert) {
                (s.w_size as ::core::ffi::c_uint).wrapping_sub(s.insert as ::core::ffi::c_uint)
            } else {
                have
            });
    }
    if s.high_water < s.strstart as crate::zutil_h::ulg {
        s.high_water = s.strstart as crate::zutil_h::ulg;
    }
    have = (s.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
        >> 3 as ::core::ffi::c_int;
    have = (if s.pending_buf_size.wrapping_sub(have as crate::zutil_h::ulg)
        > 65535 as crate::zutil_h::ulg
    {
        65535 as crate::zutil_h::ulg
    } else {
        s.pending_buf_size.wrapping_sub(have as crate::zutil_h::ulg)
    }) as ::core::ffi::c_uint;
    min_block = if have > s.w_size { s.w_size } else { have };
    left = (s.strstart as ::core::ffi::c_long - s.block_start) as ::core::ffi::c_uint;
    if left >= min_block
        || (left != 0 || flush == crate::zlib_h::Z_FINISH)
            && flush != crate::zlib_h::Z_NO_FLUSH
            && io.avail_in() == 0
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == crate::zlib_h::Z_FINISH && io.avail_in() == 0 && len == left {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let stored = s.buffers().window
            [s.block_start as usize..s.block_start as usize + len as usize]
            .to_vec();
        s.with_pending(|state, pending_buf| {
            crate::src::trees::_tr_stored_block(state, pending_buf, &stored, last)
        });
        s.block_start += len as ::core::ffi::c_long;
        flush_pending_io(s, io);
    }
    if last != 0 {
        s.bi_used = 8;
    }
    (if last != 0 {
        finish_started as ::core::ffi::c_int
    } else {
        need_more as ::core::ffi::c_int
    }) as block_state
}

fn deflate_fast(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut deflate_io,
    flush: ::core::ffi::c_int,
) -> block_state {
    loop {
        if s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window_fast(s, io);
            if s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if s.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        let mut hash_head = NIL as crate::src::deflate::IPos;
        if s.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            hash_head = s.insert_hash_at(s.strstart);
        }
        if hash_head != NIL as crate::src::deflate::IPos
            && (s.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                <= s.w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
        {
            s.match_length = longest_match(s, hash_head);
        }
        let bflush = if s.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len = s.match_length.wrapping_sub(3) as crate::zutil_h::uch;
            let dist = s.strstart.wrapping_sub(s.match_start) as crate::zutil_h::ush;
            let flush_block =
                s.tally_symbol(dist as ::core::ffi::c_uint, len as ::core::ffi::c_uint) != 0;
            s.lookahead = s.lookahead.wrapping_sub(s.match_length);
            if s.match_length <= s.max_lazy_match
                && s.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            {
                s.match_length = s.match_length.wrapping_sub(1);
                loop {
                    s.strstart = s.strstart.wrapping_add(1);
                    hash_head = s.insert_hash_at(s.strstart);
                    s.match_length = s.match_length.wrapping_sub(1);
                    if s.match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                s.strstart = s.strstart.wrapping_add(1);
            } else {
                s.strstart = s.strstart.wrapping_add(s.match_length);
                s.match_length = 0;
                s.ins_h = s.buffers().window[s.strstart as usize] as crate::stdlib::uInt;
                s.ins_h = (s.ins_h << s.hash_shift
                    ^ s.buffers().window[s.strstart.wrapping_add(1) as usize]
                        as crate::stdlib::uInt)
                    & s.hash_mask;
            }
            flush_block
        } else {
            let cc = s.buffers().window[s.strstart as usize] as crate::zutil_h::uch;
            let flush_block = s.tally_symbol(0, cc as ::core::ffi::c_uint) != 0;
            s.lookahead = s.lookahead.wrapping_sub(1);
            s.strstart = s.strstart.wrapping_add(1);
            flush_block
        };
        if bflush {
            encode_block_from_window(s, 0);
            s.block_start = s.strstart as ::core::ffi::c_long;
            flush_pending_io(s, io);
            if io.avail_out() == 0 {
                return need_more;
            }
        }
    }
    s.insert = if s.strstart
        < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        s.strstart
    } else {
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    };
    if flush == crate::zlib_h::Z_FINISH {
        encode_block_from_window(s, 1);
        s.block_start = s.strstart as ::core::ffi::c_long;
        flush_pending_io(s, io);
        if io.avail_out() == 0 {
            return finish_started;
        }
        return finish_done;
    }
    if s.sym_next != 0 {
        encode_block_from_window(s, 0);
        s.block_start = s.strstart as ::core::ffi::c_long;
        flush_pending_io(s, io);
        if io.avail_out() == 0 {
            return need_more;
        }
    }
    block_done
}

fn flush_compression_block(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut deflate_io,
    last: ::core::ffi::c_int,
) -> bool {
    encode_block_from_window(s, last);
    flush_pending_io(s, io);
    io.avail_out() == 0
}

fn deflate_slow(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut deflate_io,
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush = false;
    loop {
        if s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window_fast(s, io);
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
        if s.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            hash_head = s.insert_hash_at(s.strstart);
        }
        s.prev_length = s.match_length;
        s.prev_match = s.match_start as crate::src::deflate::IPos;
        s.match_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        if hash_head != NIL as crate::src::deflate::IPos
            && s.prev_length < s.max_lazy_match
            && (s.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                <= s.w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
        {
            let (match_length, match_start) = {
                let buffers = s.buffers.as_ref().expect("deflate buffers initialized");
                longest_match_in_buffers(
                    &buffers.window,
                    &buffers.prev,
                    s.strstart,
                    s.lookahead,
                    s.prev_length,
                    s.max_chain_length,
                    s.good_match,
                    s.nice_match,
                    s.w_size,
                    s.w_mask,
                    hash_head,
                )
            };
            s.match_length = match_length;
            if match_start != 0 {
                s.match_start = match_start;
            }
            if s.match_length <= 5 as crate::stdlib::uInt
                && (s.strategy == crate::zlib_h::Z_FILTERED
                    || s.match_length == crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                        && s.strstart.wrapping_sub(s.match_start) > TOO_FAR as crate::stdlib::uInt)
            {
                s.match_length =
                    (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            }
        }
        if s.prev_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            && s.match_length <= s.prev_length
        {
            let max_insert: crate::stdlib::uInt = s
                .strstart
                .wrapping_add(s.lookahead)
                .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
            let mut len: crate::zutil_h::uch =
                s.prev_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush = (s.strstart as crate::src::deflate::IPos)
                .wrapping_sub(1 as crate::src::deflate::IPos)
                .wrapping_sub(s.prev_match)
                as crate::zutil_h::ush;
            bflush = s.tally_symbol(dist as ::core::ffi::c_uint, len as ::core::ffi::c_uint) != 0;
            s.lookahead = s
                .lookahead
                .wrapping_sub(s.prev_length.wrapping_sub(1 as crate::stdlib::uInt));
            s.prev_length = s.prev_length.wrapping_sub(2 as crate::stdlib::uInt);
            loop {
                s.strstart = s.strstart.wrapping_add(1);
                if s.strstart <= max_insert {
                    hash_head = s.insert_hash_at(s.strstart);
                }
                s.prev_length = s.prev_length.wrapping_sub(1);
                if s.prev_length == 0 as crate::stdlib::uInt {
                    break;
                }
            }
            s.match_available = 0 as ::core::ffi::c_int;
            s.match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            s.strstart = s.strstart.wrapping_add(1);
            if bflush {
                let out_full = flush_compression_block(s, io, 0);
                s.block_start = s.strstart as ::core::ffi::c_long;
                if out_full {
                    return need_more;
                }
            }
        } else if s.match_available != 0 {
            let cc = {
                let buffers = s.buffers.as_ref().expect("deflate buffers initialized");
                buffers.window[s.strstart.wrapping_sub(1) as usize] as crate::zutil_h::uch
            };
            bflush = s.tally_symbol(0, cc as ::core::ffi::c_uint) != 0;
            let out_full = if bflush {
                let out_full = flush_compression_block(s, io, 0);
                s.block_start = s.strstart as ::core::ffi::c_long;
                out_full
            } else {
                false
            };
            s.strstart = s.strstart.wrapping_add(1);
            s.lookahead = s.lookahead.wrapping_sub(1);
            if out_full {
                return need_more;
            }
        } else {
            s.match_available = 1 as ::core::ffi::c_int;
            s.strstart = s.strstart.wrapping_add(1);
            s.lookahead = s.lookahead.wrapping_sub(1);
        }
    }
    if s.match_available != 0 {
        let cc = {
            let buffers = s.buffers.as_ref().expect("deflate buffers initialized");
            buffers.window[s.strstart.wrapping_sub(1) as usize] as crate::zutil_h::uch
        };
        bflush = s.tally_symbol(0, cc as ::core::ffi::c_uint) != 0;
        s.match_available = 0 as ::core::ffi::c_int;
    }
    s.insert = if s.strstart
        < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        s.strstart
    } else {
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    };
    if flush == crate::zlib_h::Z_FINISH {
        let out_full = flush_compression_block(s, io, 1);
        s.block_start = s.strstart as ::core::ffi::c_long;
        if out_full {
            return finish_started;
        }
        return finish_done;
    }
    if s.sym_next != 0 {
        let out_full = flush_compression_block(s, io, 0);
        s.block_start = s.strstart as ::core::ffi::c_long;
        if out_full {
            return need_more;
        }
    }
    block_done
}

fn deflate_rle(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut deflate_io,
    flush: ::core::ffi::c_int,
) -> block_state {
    loop {
        if s.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_window_fast(s, io);
            if s.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if s.lookahead == 0 {
                break;
            }
        }
        s.match_length = rle_match_length(&s.buffers().window, s.strstart, s.lookahead);
        let bflush = if s.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len = s.match_length.wrapping_sub(3) as crate::zutil_h::uch;
            let flush_block = s.tally_symbol(1, len as ::core::ffi::c_uint) != 0;
            s.lookahead = s.lookahead.wrapping_sub(s.match_length);
            s.strstart = s.strstart.wrapping_add(s.match_length);
            s.match_length = 0;
            flush_block
        } else {
            let cc = s.buffers().window[s.strstart as usize] as crate::zutil_h::uch;
            let flush_block = s.tally_symbol(0, cc as ::core::ffi::c_uint) != 0;
            s.lookahead = s.lookahead.wrapping_sub(1);
            s.strstart = s.strstart.wrapping_add(1);
            flush_block
        };
        if bflush {
            let out_full = flush_compression_block(s, io, 0);
            s.block_start = s.strstart as ::core::ffi::c_long;
            if out_full {
                return need_more;
            }
        }
    }
    s.insert = 0;
    if flush == crate::zlib_h::Z_FINISH {
        let out_full = flush_compression_block(s, io, 1);
        s.block_start = s.strstart as ::core::ffi::c_long;
        if out_full {
            return finish_started;
        }
        return finish_done;
    }
    if s.sym_next != 0 {
        let out_full = flush_compression_block(s, io, 0);
        s.block_start = s.strstart as ::core::ffi::c_long;
        if out_full {
            return need_more;
        }
    }
    block_done
}

fn deflate_huff(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut deflate_io,
    flush: ::core::ffi::c_int,
) -> block_state {
    loop {
        if s.lookahead == 0 {
            fill_window_fast(s, io);
            if s.lookahead == 0 {
                if flush == crate::zlib_h::Z_NO_FLUSH {
                    return need_more;
                }
                break;
            }
        }
        s.match_length = 0;
        let cc = s.buffers().window[s.strstart as usize] as crate::zutil_h::uch;
        let bflush = s.tally_symbol(0, cc as ::core::ffi::c_uint) != 0;
        s.lookahead = s.lookahead.wrapping_sub(1);
        s.strstart = s.strstart.wrapping_add(1);
        if bflush {
            let out_full = flush_compression_block(s, io, 0);
            s.block_start = s.strstart as ::core::ffi::c_long;
            if out_full {
                return need_more;
            }
        }
    }
    s.insert = 0;
    if flush == crate::zlib_h::Z_FINISH {
        let out_full = flush_compression_block(s, io, 1);
        s.block_start = s.strstart as ::core::ffi::c_long;
        if out_full {
            return finish_started;
        }
        return finish_done;
    }
    if s.sym_next != 0 {
        let out_full = flush_compression_block(s, io, 0);
        s.block_start = s.strstart as ::core::ffi::c_long;
        if out_full {
            return need_more;
        }
    }
    block_done
}
