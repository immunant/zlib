// =============== BEGIN deflate_h ================
use core::ffi::CStr;
use core::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

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
    // The C representation used two same-sized union pairs: frequency/code
    // and depth/length.  Each pair is an overlay, not two independent values,
    // so a scalar retains the exact layout while avoiding union reads.
    pub fc: crate::zutil_h::ush,
    pub dl: crate::zutil_h::ush,
}

impl ct_data_s {
    #[inline]
    pub fn code(&self) -> crate::zutil_h::ush {
        self.fc
    }

    #[inline]
    pub fn len(&self) -> crate::zutil_h::ush {
        self.dl
    }

    #[inline]
    pub fn freq(&self) -> crate::zutil_h::ush {
        self.fc
    }

    #[inline]
    pub fn dad(&self) -> crate::zutil_h::ush {
        self.dl
    }

    #[inline]
    pub fn set_len(&mut self, len: crate::zutil_h::ush) {
        self.dl = len;
    }
}

pub type static_tree_desc = crate::src::deflate::static_tree_desc_s;

pub type tree_desc = crate::src::deflate::tree_desc_s;
pub const STATIC_TREE_LITERAL: u8 = 0;
pub const STATIC_TREE_DISTANCE: u8 = 1;
pub const STATIC_TREE_BIT_LENGTH: u8 = 2;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tree_desc_s {
    pub max_code: ::core::ffi::c_int,
    pub static_kind: u8,
}

pub type Pos = crate::zutil_h::ush;

pub type Posf = crate::src::deflate::Pos;

pub type IPos = ::core::ffi::c_uint;

pub type deflate_state = crate::src::deflate::internal_state;
#[repr(C)]

pub struct internal_state {
    pub strm: crate::zlib_h::z_streamp,
    pub status: ::core::ffi::c_int,
    // The pending output and symbol overlay share one fixed-size allocation.
    // Offsets below select their regions without retaining interior pointers.
    pub pending_buf: Option<Vec<crate::stdlib::Bytef>>,
    pub pending_buf_size: crate::zutil_h::ulg,
    // Offset of the first pending byte within `pending_buf`.
    //
    // Keeping this as an offset avoids retaining an interior raw pointer
    // across buffer allocation, reset, and state copying.
    pub pending_out: usize,
    pub pending: crate::zutil_h::ulg,
    pub wrap: ::core::ffi::c_int,
    pub gzhead: u64,
    pub gzindex: crate::zutil_h::ulg,
    pub method: crate::stdlib::Byte,
    pub last_flush: ::core::ffi::c_int,
    pub w_size: crate::stdlib::uInt,
    pub w_bits: crate::stdlib::uInt,
    pub w_mask: crate::stdlib::uInt,
    pub window: *mut crate::stdlib::Bytef,
    pub window_size: crate::zutil_h::ulg,
    // The LZ predecessor chain has one entry per window position.  Owning it
    // as a vector keeps the chain's allocation paired with the state instead
    // of retaining a second allocator-owned raw pointer.
    pub prev: Option<Vec<crate::src::deflate::Posf>>,
    // The hash heads are an owned fixed-size table.  `None` is the valid
    // all-zero initial state used while the surrounding opaque state is
    // being constructed by the ABI allocator.
    pub head: Option<Vec<crate::src::deflate::Posf>>,
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
    // Offset of the symbol overlay within `pending_buf`.
    //
    // The symbol bytes deliberately overlap the pending allocation, but an
    // offset remains valid when that allocation is cloned or replaced.
    pub sym_buf: usize,
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

impl internal_state {
    /// Append one byte to the pending allocation.
    #[inline]
    pub fn put_pending_byte(&mut self, byte: crate::stdlib::Bytef) {
        let Ok(len) = usize::try_from(self.pending_buf_size) else {
            return;
        };
        let Ok(index) = usize::try_from(self.pending) else {
            return;
        };
        let Some(pending) = self.pending_buf.as_mut() else {
            return;
        };
        if index >= len || index >= pending.len() {
            return;
        }
        pending[index] = byte;
        self.pending = self.pending.wrapping_add(1);
    }

    /// Replace the length and complement emitted at the end of an empty
    /// stored block.  Keeping this as a checked slice update avoids
    /// reconstructing four independent interior pointers at the call site.
    #[inline]
    pub fn set_stored_block_length(&mut self, len: crate::stdlib::uInt) -> bool {
        let Ok(allocation_len) = usize::try_from(self.pending_buf_size) else {
            return false;
        };
        let Ok(pending) = usize::try_from(self.pending) else {
            return false;
        };
        let Some(start) = pending.checked_sub(4) else {
            return false;
        };
        let Some(pending_bytes) = self.pending_buf.as_mut() else {
            return false;
        };
        if pending > allocation_len || pending > pending_bytes.len() {
            return false;
        }

        let length = len as crate::stdlib::Bytef;
        let length_hi = (len >> 8) as crate::stdlib::Bytef;
        pending_bytes[start..pending].copy_from_slice(&[length, length_hi, !length, !length_hi]);
        true
    }

    /// View a checked range of the pending allocation.  Pending data is
    /// always addressed by offsets, so callers do not need to reconstruct
    /// interior raw pointers for checksum updates.
    #[inline]
    fn pending_bytes(
        &self,
        start: crate::zutil_h::ulg,
        end: crate::zutil_h::ulg,
    ) -> Option<&[crate::stdlib::Bytef]> {
        let start = usize::try_from(start).ok()?;
        let end = usize::try_from(end).ok()?;
        let len = usize::try_from(self.pending_buf_size).ok()?;
        let pending = self.pending_buf.as_ref()?;
        if start > end || end > len || end > pending.len() {
            return None;
        }
        Some(&pending[start..end])
    }

    /// Record one literal or match descriptor in the three-byte symbol
    /// overlay.  The overlay deliberately shares `pending_buf`, so validate
    /// both the logical symbol limit and the backing allocation before
    /// temporarily viewing that allocation as a slice.
    #[inline]
    pub fn put_symbol(&mut self, dist: crate::stdlib::uInt, lc: crate::stdlib::uInt) -> bool {
        let Ok(len) = usize::try_from(self.pending_buf_size) else {
            return false;
        };
        let Ok(next) = usize::try_from(self.sym_next) else {
            return false;
        };
        let Ok(sym_end) = usize::try_from(self.sym_end) else {
            return false;
        };
        let Some(next_end) = next.checked_add(3) else {
            return false;
        };
        let Some(start) = self.sym_buf.checked_add(next) else {
            return false;
        };
        let Some(end) = start.checked_add(3) else {
            return false;
        };
        let Some(pending) = self.pending_buf.as_mut() else {
            return false;
        };
        if next_end > sym_end || end > len || end > pending.len() {
            return false;
        }
        pending[start..end].copy_from_slice(&[
            dist as crate::zutil_h::uch as crate::zutil_h::uchf,
            (dist >> 8) as crate::zutil_h::uch as crate::zutil_h::uchf,
            lc as crate::zutil_h::uch as crate::zutil_h::uchf,
        ]);
        self.sym_next = next_end as crate::stdlib::uInt;
        true
    }

    /// Copy-free view of the symbol overlay after validating both its offset
    /// and the number of bytes recorded in it.  Tree emission takes a short
    /// owned snapshot before it starts appending pending output, so the two
    /// regions never need to be addressed through interior pointers.
    #[inline]
    pub fn symbol_bytes(&self) -> Option<&[crate::stdlib::Bytef]> {
        let next = usize::try_from(self.sym_next).ok()?;
        let end = self.sym_buf.checked_add(next)?;
        let start = crate::zutil_h::ulg::try_from(self.sym_buf).ok()?;
        let end = crate::zutil_h::ulg::try_from(end).ok()?;
        self.pending_bytes(start, end)
    }

    /// Return one byte from the LZ window after checking it against the
    /// allocation retained by this state.  The window is allocated with a
    /// fixed size for the lifetime of a deflate stream, so this short-lived
    /// slice does not retain an interior pointer.
    #[inline]
    fn window_byte(&self, index: crate::stdlib::uInt) -> Option<crate::stdlib::Bytef> {
        self.window_bytes(index, 1)
            .and_then(|bytes| bytes.first().copied())
    }

    /// View a checked window range.  Compression engines use this for the
    /// short-lived block source passed to the tree writer instead of
    /// reconstructing window interior pointers at each flush point.
    #[inline]
    fn window_bytes(
        &self,
        start: crate::stdlib::uInt,
        len: usize,
    ) -> Option<&[crate::stdlib::Bytef]> {
        let start = usize::try_from(start).ok()?;
        let window_len = usize::try_from(self.window_size).ok()?;
        let end = start.checked_add(len)?;
        if self.window.is_null() || end > window_len {
            return None;
        }

        Some(unsafe { core::slice::from_raw_parts(self.window.add(start), len) })
    }

    /// The tree writer consumes a source slice while it mutates the state.
    /// Copying this short-lived view is the same ownership boundary used by
    /// the exported tree helper and avoids an overlapping borrow of `self`.
    #[inline]
    fn block_data(
        &self,
        start: crate::stdlib::uInt,
        len: crate::zutil_h::ulg,
    ) -> Option<Vec<crate::stdlib::Bytef>> {
        let len = usize::try_from(len).ok()?;
        self.window_bytes(start, len).map(ToOwned::to_owned)
    }

    #[inline]
    fn avail_out(&self) -> Option<crate::stdlib::uInt> {
        // The stream backlink is established together with this state and is
        // checked by the public deflate entry point before an engine runs.
        unsafe { self.strm.as_ref().map(|strm| strm.avail_out) }
    }

    /// Insert the string at `strstart` into the LZ hash chain and return its
    /// previous head.  Keeping the three backing allocations behind this
    /// checked operation avoids rebuilding raw interior pointers in every
    /// compression engine.
    #[inline]
    fn insert_string(&mut self) -> Option<crate::src::deflate::IPos> {
        let next = self.strstart.checked_add(
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
        )?;
        let byte = self.window_byte(next)? as crate::stdlib::uInt;
        self.ins_h = (self.ins_h << self.hash_shift ^ byte) & self.hash_mask;

        let prev_index = usize::try_from(self.strstart & self.w_mask).ok()?;
        let head_index = usize::try_from(self.ins_h).ok()?;
        let prev_len = usize::try_from(self.w_size).ok()?;
        let head_len = usize::try_from(self.hash_size).ok()?;
        if self.prev.as_ref().is_none_or(|prev| prev.len() != prev_len)
            || self.head.as_ref().is_none_or(|head| head.len() != head_len)
            || prev_index >= prev_len
            || head_index >= head_len
        {
            return None;
        }

        let previous = self.prev.as_mut()?;
        let heads = self.head.as_mut()?;
        previous[prev_index] = heads[head_index];
        let hash_head = previous[prev_index] as crate::src::deflate::IPos;
        heads[head_index] = self.strstart as crate::src::deflate::Pos;
        Some(hash_head)
    }
}

#[derive(Clone)]
pub struct GzipHeader {
    pub text: ::core::ffi::c_int,
    pub time: crate::stdlib::uLong,
    pub xflags: ::core::ffi::c_int,
    pub os: ::core::ffi::c_int,
    // `Some(empty)` is distinct from no field: it retains the gzip header
    // flag and its two-byte zero length, matching a non-null C pointer.
    pub extra: Option<Vec<crate::stdlib::Bytef>>,
    // These include their terminating NUL, as required by the gzip format.
    pub name: Option<Vec<crate::stdlib::Bytef>>,
    pub comment: Option<Vec<crate::stdlib::Bytef>>,
    pub hcrc: ::core::ffi::c_int,
}

static GZIP_HEADERS: OnceLock<Mutex<HashMap<u64, GzipHeader>>> = OnceLock::new();
static NEXT_GZIP_HEADER_ID: AtomicU64 = AtomicU64::new(1);

fn gzip_headers() -> &'static Mutex<HashMap<u64, GzipHeader>> {
    GZIP_HEADERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn gzip_header_store(header: Option<GzipHeader>) -> u64 {
    let Some(header) = header else {
        return 0;
    };
    let id = loop {
        let id = NEXT_GZIP_HEADER_ID.fetch_add(1, Ordering::Relaxed);
        if id != 0 {
            break id;
        }
    };
    gzip_headers()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(id, header);
    id
}

fn gzip_header_clone(id: u64) -> Option<GzipHeader> {
    if id == 0 {
        return None;
    }
    gzip_headers()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&id)
        .cloned()
}

fn gzip_header_remove(id: u64) {
    if id != 0 {
        gzip_headers()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&id);
    }
}

fn gzip_header_snapshot(
    text: ::core::ffi::c_int,
    time: crate::stdlib::uLong,
    xflags: ::core::ffi::c_int,
    os: ::core::ffi::c_int,
    extra: Option<&[crate::stdlib::Bytef]>,
    name: Option<&[crate::stdlib::Bytef]>,
    comment: Option<&[crate::stdlib::Bytef]>,
    hcrc: ::core::ffi::c_int,
) -> GzipHeader {
    GzipHeader {
        text,
        time,
        xflags,
        os,
        extra: extra.map(ToOwned::to_owned),
        name: name.map(ToOwned::to_owned),
        comment: comment.map(ToOwned::to_owned),
        hcrc,
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
pub enum CompressionEngine {
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
    pub func: CompressionEngine,
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
        func: CompressionEngine::Stored,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        func: CompressionEngine::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        func: CompressionEngine::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: CompressionEngine::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        func: CompressionEngine::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        func: CompressionEngine::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        func: CompressionEngine::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        func: CompressionEngine::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        func: CompressionEngine::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        func: CompressionEngine::Slow,
    },
];

fn slide_positions(positions: &mut [crate::src::deflate::Posf], wsize: crate::stdlib::uInt) {
    for position in positions.iter_mut().rev() {
        let value = *position as ::core::ffi::c_uint;
        *position = (if value >= wsize {
            value.wrapping_sub(wsize as ::core::ffi::c_uint)
        } else {
            NIL as ::core::ffi::c_uint
        }) as crate::src::deflate::Pos as crate::src::deflate::Posf;
    }
}

fn allocate_head_table(len: usize) -> Option<Vec<crate::src::deflate::Posf>> {
    let mut heads = Vec::new();
    heads.try_reserve_exact(len).ok()?;
    heads.resize(len, NIL as crate::src::deflate::Posf);
    Some(heads)
}

fn allocate_prev_table(len: usize) -> Option<Vec<crate::src::deflate::Posf>> {
    let mut prev = Vec::new();
    prev.try_reserve_exact(len).ok()?;
    prev.resize(len, NIL as crate::src::deflate::Posf);
    Some(prev)
}

fn allocate_pending_buffer(len: usize) -> Option<Vec<crate::stdlib::Bytef>> {
    let mut pending = Vec::new();
    pending.try_reserve_exact(len).ok()?;
    pending.resize(len, 0);
    Some(pending)
}

fn clone_head_table(
    head: &[crate::src::deflate::Posf],
) -> Option<Vec<crate::src::deflate::Posf>> {
    let mut copy = allocate_head_table(head.len())?;
    copy.copy_from_slice(head);
    Some(copy)
}

fn clone_prev_table(
    prev: &[crate::src::deflate::Posf],
) -> Option<Vec<crate::src::deflate::Posf>> {
    let mut copy = allocate_prev_table(prev.len())?;
    copy.copy_from_slice(prev);
    Some(copy)
}

unsafe fn slide_hash(s: *mut crate::src::deflate::deflate_state) {
    let s = &mut *s;
    let wsize = s.w_size;
    let Some(head) = s.head.as_mut() else {
        return;
    };
    if head.len() != s.hash_size as usize {
        return;
    }
    slide_positions(head, wsize);

    let Some(prev) = s.prev.as_mut() else {
        return;
    };
    if prev.len() != wsize as usize {
        return;
    }
    slide_positions(prev, wsize);
    s.slid = 1 as ::core::ffi::c_int;
}

fn read_buf_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    wrap: ::core::ffi::c_int,
) {
    let len = input.len() as crate::stdlib::uInt;
    strm.avail_in = strm.avail_in.wrapping_sub(len);
    output.copy_from_slice(input);
    if wrap == 1 as ::core::ffi::c_int {
        strm.adler = crate::src::adler32::adler32_z(strm.adler, output);
    } else if wrap == 2 as ::core::ffi::c_int {
        strm.adler = crate::src::crc32::crc32(strm.adler, output);
    }
    strm.total_in = strm.total_in.wrapping_add(len as crate::stdlib::uLong);
}

unsafe extern "C" fn read_buf(
    strm: crate::zlib_h::z_streamp,
    buf: *mut crate::stdlib::Bytef,
    size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let strm = &mut *strm;
    let len = strm.avail_in.min(size);
    if len == 0 {
        return 0;
    }

    // `len` is bounded by both the caller-provided destination capacity and
    // the stream's available input above, so each view covers exactly the
    // bytes transferred by this call.
    let input = ::core::slice::from_raw_parts(strm.next_in, len as usize);
    let output = ::core::slice::from_raw_parts_mut(buf, len as usize);
    let wrap = (&*strm.state).wrap;
    read_buf_impl(strm, input, output, wrap);
    strm.next_in = strm.next_in.add(len as usize);
    len
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
                let head_index = (*s).ins_h as usize;
                let Some(head) = (*s).head.as_mut() else {
                    return;
                };
                let Some(hash_head) = head.get(head_index).copied() else {
                    return;
                };
                let prev_index = (str & (*s).w_mask) as usize;
                let Some(previous) = (*s).prev.as_mut() else {
                    return;
                };
                let Some(slot) = previous.get_mut(prev_index) else {
                    return;
                };
                *slot = hash_head;
                head[head_index] =
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
    (*s).gzhead = 0;
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
    (*s).prev = allocate_prev_table((*s).w_size as usize);
    (*s).head = Some(allocate_head_table((*s).hash_size as usize).unwrap_or_default());
    (*s).high_water = 0 as crate::zutil_h::ulg;
    (*s).lit_bufsize =
        ((1 as ::core::ffi::c_int) << memLevel + 6 as ::core::ffi::c_int) as crate::stdlib::uInt;
    (*s).pending_buf_size =
        ((*s).lit_bufsize as crate::zutil_h::ulg).wrapping_mul(4 as crate::zutil_h::ulg);
    (*s).pending_buf = allocate_pending_buffer((*s).pending_buf_size as usize);
    if (*s).window.is_null()
        || (*s).prev.is_none()
        || (*s).head.as_ref().is_none_or(|head| head.len() != (*s).hash_size as usize)
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
        deflateEnd_from_stream_pointer(strm);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*s).sym_buf = (*s).lit_bufsize as usize;
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
    let stream_pointer = strm;
    if strm.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let strm = &*strm;
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return 1 as ::core::ffi::c_int;
    }
    if strm.state.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let s = &*strm.state;
    if s.strm != stream_pointer
        || (s.status != crate::src::deflate::INIT_STATE
            && s.status != crate::src::deflate::GZIP_STATE
            && s.status != crate::src::deflate::EXTRA_STATE
            && s.status != crate::src::deflate::NAME_STATE
            && s.status != crate::src::deflate::COMMENT_STATE
            && s.status != crate::src::deflate::HCRC_STATE
            && s.status != crate::src::deflate::BUSY_STATE
            && s.status != crate::src::deflate::FINISH_STATE)
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}

/// Check the portion of a deflate stream that can be inspected without
/// following its state link.  Parameter updates use this before borrowing the
/// state, leaving the raw link crossing confined to the caller.
fn deflate_params_stream_is_valid(strm: &crate::zlib_h::z_stream_s) -> bool {
    strm.zalloc.is_some() && strm.zfree.is_some() && !strm.state.is_null()
}

/// Validate a deflate state after its already-checked link has been borrowed.
/// Keeping the state-machine rules here makes the parameter-update path
/// independent of the raw-pointer-based general stream checker.
fn deflate_params_state_is_valid(
    strm: &crate::zlib_h::z_stream_s,
    s: &crate::src::deflate::deflate_state,
) -> bool {
    s.strm == core::ptr::from_ref(strm).cast_mut()
        && (s.status == crate::src::deflate::INIT_STATE
            || s.status == crate::src::deflate::GZIP_STATE
            || s.status == crate::src::deflate::EXTRA_STATE
            || s.status == crate::src::deflate::NAME_STATE
            || s.status == crate::src::deflate::COMMENT_STATE
            || s.status == crate::src::deflate::HCRC_STATE
            || s.status == crate::src::deflate::BUSY_STATE
            || s.status == crate::src::deflate::FINISH_STATE)
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
            let Some(head) = (*s).head.as_mut() else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            if head.len() != (*s).hash_size as usize {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            head.fill(NIL as crate::src::deflate::Posf);
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
            let head_index = (*s).ins_h as usize;
            let Some(head) = (*s).head.as_mut() else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let Some(hash_head) = head.get(head_index).copied() else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let prev_index = (str & (*s).w_mask) as usize;
            let Some(previous) = (*s).prev.as_mut() else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let Some(slot) = previous.get_mut(prev_index) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            *slot = hash_head;
            head[head_index] =
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
        crate::src::crc32::crc32(0 as crate::stdlib::uLong, &[])
    } else {
        crate::src::adler32::adler32(
            0 as crate::stdlib::uLong,
            ::core::ptr::null::<crate::stdlib::Bytef>(),
            0 as crate::stdlib::uInt,
        )
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
unsafe fn lm_init(s: *mut crate::src::deflate::deflate_state) {
    let s = unsafe { &mut *s };
    s.window_size = (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
        .wrapping_mul(s.w_size as crate::zutil_h::ulg);

    let head_len = s.hash_size as usize;
    if head_len == 0 {
        return;
    }
    let Some(head) = s.head.as_mut() else {
        return;
    };
    if head.len() != head_len {
        return;
    }
    head[head_len - 1] = NIL as crate::src::deflate::Posf;
    head[..head_len - 1].fill(0 as crate::src::deflate::Posf);

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
fn deflate_set_header_impl(
    state: &mut crate::src::deflate::deflate_state,
    header: Option<GzipHeader>,
) -> ::core::ffi::c_int {
    if state.wrap != 2 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let old_header = state.gzhead;
    state.gzhead = gzip_header_store(header);
    gzip_header_remove(old_header);
    return crate::zlib_h::Z_OK;
}

pub unsafe fn deflateSetHeader<F>(
    strm: &mut crate::zlib_h::z_stream_s,
    header: F,
) -> ::core::ffi::c_int
where
    F: FnOnce() -> Option<GzipHeader>,
{
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *strm.state;
    deflate_set_header_impl(state, header())
}
#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let header = head.as_ref().map(|head| {
        let extra = (!head.extra.is_null())
            .then(|| ::core::slice::from_raw_parts(head.extra, head.extra_len as usize));
        let name = (!head.name.is_null()).then(|| {
            CStr::from_ptr(head.name.cast::<::core::ffi::c_char>()).to_bytes_with_nul()
        });
        let comment = (!head.comment.is_null()).then(|| {
            CStr::from_ptr(head.comment.cast::<::core::ffi::c_char>()).to_bytes_with_nul()
        });
        gzip_header_snapshot(
            head.text,
            head.time,
            head.xflags,
            head.os,
            extra,
            name,
            comment,
            head.hcrc,
        )
    });
    deflateSetHeader(strm, || header)
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
    let mut put: ::core::ffi::c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let s = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    let pending_state = &*s;
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || pending_state.sym_buf
            < pending_state.pending_out.wrapping_add(
                (crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
                    as usize,
            )
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
        crate::src::trees::_tr_flush_bits(s);
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
pub unsafe fn deflateParams(
    strm: &mut crate::zlib_h::z_stream_s,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // Validate the stream before crossing its raw state link.  The link is
    // maintained by the allocation lifecycle and is the only remaining raw
    // boundary this parameter update needs to cross.
    if !deflate_params_stream_is_valid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let s = &mut *(strm.state as *mut crate::src::deflate::deflate_state);
    if !deflate_params_state_is_valid(strm, s) {
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
    let func = configuration_table[s.level as usize].func;
    if (strategy != s.strategy || func != configuration_table[level as usize].func)
        && s.last_flush != -2 as ::core::ffi::c_int
    {
        let err: ::core::ffi::c_int = deflate(strm, crate::zlib_h::Z_BLOCK);
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
                slide_hash(s as *mut crate::src::deflate::deflate_state);
            } else {
                let head_len = s.hash_size as usize;
                let Some(head) = s.head.as_mut() else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                if head.len() != head_len {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
                head[..head_len - 1].fill(0 as crate::src::deflate::Posf);
                head[head_len - 1] = NIL as crate::src::deflate::Posf;
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
    deflateParams(strm, level, strategy)
}
pub unsafe fn deflateTune(
    strm: crate::zlib_h::z_streamp,
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
    // The state pointer is installed by initialization.  Validate its
    // backlink and state machine before allowing the parameter update to
    // borrow it.
    if strm.state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let s = unsafe { &mut *strm.state };
    let stream_pointer = core::ptr::from_mut(strm);
    if s.strm != stream_pointer
        || (s.status != crate::src::deflate::INIT_STATE
            && s.status != crate::src::deflate::GZIP_STATE
            && s.status != crate::src::deflate::EXTRA_STATE
            && s.status != crate::src::deflate::NAME_STATE
            && s.status != crate::src::deflate::COMMENT_STATE
            && s.status != crate::src::deflate::HCRC_STATE
            && s.status != crate::src::deflate::BUSY_STATE
            && s.status != crate::src::deflate::FINISH_STATE)
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
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
    deflateTune(strm, good_length, max_lazy, nice_length, max_chain)
}
pub unsafe fn deflateBound_z(
    strm: Option<&crate::zlib_h::z_stream_s>,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
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
    let state = match strm {
        Some(strm) if strm.zalloc.is_some() && strm.zfree.is_some() && !strm.state.is_null() => {
            let s = &*strm.state;
            let stream_pointer =
                strm as *const crate::zlib_h::z_stream_s as crate::zlib_h::z_streamp;
            if s.strm == stream_pointer
                && (s.status == crate::src::deflate::INIT_STATE
                    || s.status == crate::src::deflate::GZIP_STATE
                    || s.status == crate::src::deflate::EXTRA_STATE
                    || s.status == crate::src::deflate::NAME_STATE
                    || s.status == crate::src::deflate::COMMENT_STATE
                    || s.status == crate::src::deflate::HCRC_STATE
                    || s.status == crate::src::deflate::BUSY_STATE
                    || s.status == crate::src::deflate::FINISH_STATE)
            {
                Some(s)
            } else {
                None
            }
        }
        _ => None,
    };
    let Some(s) = state else {
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
    match if s.wrap < 0 as ::core::ffi::c_int {
        -s.wrap
    } else {
        s.wrap
    } {
        0 => {
            wraplen = 0 as crate::stdlib::z_size_t;
        }
        1 => {
            wraplen = (6 as ::core::ffi::c_int
                + (if s.strstart != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as crate::stdlib::z_size_t;
        }
        2 => {
            wraplen = 18 as crate::stdlib::z_size_t;
            if let Some(gzhead) = gzip_header_clone(s.gzhead) {
                if let Some(extra) = gzhead.extra.as_ref() {
                    wraplen = wraplen.wrapping_add(2usize.wrapping_add(extra.len()));
                }
                if let Some(name) = gzhead.name.as_ref() {
                    wraplen = wraplen.wrapping_add(name.len());
                }
                if let Some(comment) = gzhead.comment.as_ref() {
                    wraplen = wraplen.wrapping_add(comment.len());
                }
                if gzhead.hcrc != 0 {
                    wraplen = wraplen.wrapping_add(2 as crate::stdlib::z_size_t);
                }
            }
        }
        _ => {
            wraplen = 18 as crate::stdlib::z_size_t;
        }
    }
    if s.w_bits != 15 as crate::stdlib::uInt
        || s.hash_bits != (8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        bound = if s.w_bits <= s.hash_bits && s.level != 0 {
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
    deflateBound_z(strm.as_ref(), sourceLen)
}
pub unsafe fn deflateBound(
    strm: Option<&crate::zlib_h::z_stream_s>,
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
    deflateBound(strm.as_ref(), sourceLen)
}
unsafe extern "C" fn putShortMSB(
    mut s: *mut crate::src::deflate::deflate_state,
    mut b: crate::stdlib::uInt,
) {
    let s = unsafe { &mut *s };
    s.put_pending_byte((b >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte);
    s.put_pending_byte((b & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte);
}

unsafe extern "C" fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    let mut len: ::core::ffi::c_uint = 0;
    let strm = &mut *strm;
    let s = &mut *(strm.state as *mut crate::src::deflate::deflate_state);
    crate::src::trees::_tr_flush_bits(s);
    len = if s.pending > strm.avail_out as crate::zutil_h::ulg {
        strm.avail_out as ::core::ffi::c_uint
    } else {
        s.pending as ::core::ffi::c_uint
    };
    if len == 0 as ::core::ffi::c_uint {
        return;
    }
    let Some(pending) = s.pending_buf.as_ref() else {
        return;
    };
    let start = s.pending_out;
    let Some(end) = start.checked_add(len as usize) else {
        return;
    };
    let Some(source) = pending.get(start..end) else {
        return;
    };
    core::ptr::copy_nonoverlapping(source.as_ptr(), strm.next_out, len as usize);
    strm.next_out = strm.next_out.offset(len as isize);
    s.pending_out = s.pending_out.wrapping_add(len as usize);
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out = strm.avail_out.wrapping_sub(len);
    s.pending = s.pending.wrapping_sub(len as crate::zutil_h::ulg);
    if s.pending == 0 as crate::zutil_h::ulg {
        s.pending_out = 0;
    }
}
pub unsafe fn deflate(
    strm: &mut crate::zlib_h::z_stream_s,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut old_flush: ::core::ffi::c_int = 0;
    if deflateStateCheck(strm as *mut crate::zlib_h::z_stream_s) != 0
        || flush > crate::zlib_h::Z_BLOCK
        || flush < 0 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Validate the state link before borrowing it.  The resulting reference
    // carries the state through this call, avoiding repeated raw-state
    // dereferences in the compression state machine.
    let s = &mut *strm.state;
    let gzhead = gzip_header_clone((*s).gzhead);
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
    old_flush = (*s).last_flush;
    (*s).last_flush = flush;
    if (*s).pending != 0 as crate::zutil_h::ulg {
        flush_pending(strm);
        if strm.avail_out == 0 as crate::stdlib::uInt {
            (*s).last_flush = -1 as ::core::ffi::c_int;
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
    if (*s).status == crate::src::deflate::FINISH_STATE && strm.avail_in != 0 as crate::stdlib::uInt
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
                (strm.adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
            );
            putShortMSB(
                s,
                (strm.adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
            );
        }
        strm.adler = crate::src::adler32::adler32(
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
        strm.adler = crate::src::crc32::crc32(0 as crate::stdlib::uLong, &[]);
        (*s).put_pending_byte(31);
        (*s).put_pending_byte(139);
        (*s).put_pending_byte(8);
        if gzhead.is_none() {
            (*s).put_pending_byte(0);
            (*s).put_pending_byte(0);
            (*s).put_pending_byte(0);
            (*s).put_pending_byte(0);
            (*s).put_pending_byte(0);
            (*s).put_pending_byte(if (*s).level == 9 {
                2
            } else if (*s).strategy >= 2 || (*s).level < 2 {
                4
            } else {
                0
            });
            (*s).put_pending_byte(3);
            (*s).status = crate::src::deflate::BUSY_STATE;
            flush_pending(strm);
            if (*s).pending != 0 as crate::zutil_h::ulg {
                (*s).last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        } else {
            let gzhead = gzhead.as_ref().expect("gzip header disappeared");
            (*s).put_pending_byte((if gzhead.text != 0 { 1 } else { 0 })
                + (if gzhead.hcrc != 0 { 2 } else { 0 })
                + (if gzhead.extra.is_none() { 0 } else { 4 })
                + (if gzhead.name.is_none() { 0 } else { 8 })
                + (if gzhead.comment.is_none() { 0 } else { 16 }));
            (*s).put_pending_byte((gzhead.time & 0xff) as crate::stdlib::Byte);
            (*s).put_pending_byte((gzhead.time >> 8 & 0xff) as crate::stdlib::Byte);
            (*s).put_pending_byte((gzhead.time >> 16 & 0xff) as crate::stdlib::Byte);
            (*s).put_pending_byte((gzhead.time >> 24 & 0xff) as crate::stdlib::Byte);
            (*s).put_pending_byte(if (*s).level == 9 {
                2
            } else if (*s).strategy >= 2 || (*s).level < 2 {
                4
            } else {
                0
            });
            (*s).put_pending_byte((gzhead.os & 0xff) as crate::stdlib::Bytef);
            if let Some(extra) = gzhead.extra.as_ref() {
                (*s).put_pending_byte((extra.len() as crate::stdlib::uInt & 0xff) as crate::stdlib::Bytef);
                (*s).put_pending_byte((extra.len() as crate::stdlib::uInt >> 8 & 0xff) as crate::stdlib::Bytef);
            }
            if gzhead.hcrc != 0 {
                if let Some(bytes) = (*s).pending_bytes(0, (*s).pending) {
                    strm.adler = crate::src::crc32::crc32_z(strm.adler, bytes);
                }
            }
            (*s).gzindex = 0 as crate::zutil_h::ulg;
            (*s).status = crate::src::deflate::EXTRA_STATE;
        }
    }
    if (*s).status == crate::src::deflate::EXTRA_STATE {
        let gzhead = gzhead
            .as_ref()
            .expect("missing gzip header in header state");
        if let Some(extra) = gzhead.extra.as_ref() {
            let mut beg: crate::zutil_h::ulg = (*s).pending;
            let mut left: crate::zutil_h::ulg = ((extra.len() as crate::stdlib::uInt
                & 0xffff as crate::stdlib::uInt)
                as crate::zutil_h::ulg)
                .wrapping_sub((*s).gzindex);
            while (*s).pending.wrapping_add(left) > (*s).pending_buf_size {
                let mut copy: crate::zutil_h::ulg =
                    (*s).pending_buf_size.wrapping_sub((*s).pending);
                let start = (*s).pending as usize;
                let end = start.wrapping_add(copy as usize);
                let extra_start = (*s).gzindex as usize;
                let extra_end = extra_start.wrapping_add(copy as usize);
                let Some(pending) = (*s).pending_buf.as_mut() else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                let (Some(destination), Some(source)) =
                    (pending.get_mut(start..end), extra.get(extra_start..extra_end))
                else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                destination.copy_from_slice(source);
                (*s).pending = (*s).pending_buf_size;
                if gzhead.hcrc != 0 && (*s).pending > beg {
                    if let Some(bytes) = (*s).pending_bytes(beg, (*s).pending) {
                        strm.adler = crate::src::crc32::crc32_z(strm.adler, bytes);
                    }
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
            let start = (*s).pending as usize;
            let end = start.wrapping_add(left as usize);
            let extra_start = (*s).gzindex as usize;
            let extra_end = extra_start.wrapping_add(left as usize);
            let Some(pending) = (*s).pending_buf.as_mut() else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let (Some(destination), Some(source)) =
                (pending.get_mut(start..end), extra.get(extra_start..extra_end))
            else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            destination.copy_from_slice(source);
            (*s).pending = (*s).pending.wrapping_add(left);
            if gzhead.hcrc != 0 && (*s).pending > beg {
                if let Some(bytes) = (*s).pending_bytes(beg, (*s).pending) {
                    strm.adler = crate::src::crc32::crc32_z(strm.adler, bytes);
                }
            }
            (*s).gzindex = 0 as crate::zutil_h::ulg;
        }
        (*s).status = crate::src::deflate::NAME_STATE;
    }
    if (*s).status == crate::src::deflate::NAME_STATE {
        let gzhead = gzhead
            .as_ref()
            .expect("missing gzip header in header state");
        if let Some(name) = gzhead.name.as_ref() {
            let mut beg_0: crate::zutil_h::ulg = (*s).pending;
            let mut val: ::core::ffi::c_int = 0;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if gzhead.hcrc != 0 && (*s).pending > beg_0 {
                        if let Some(bytes) = (*s).pending_bytes(beg_0, (*s).pending) {
                            strm.adler = crate::src::crc32::crc32_z(strm.adler, bytes);
                        }
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
                (*s).put_pending_byte(val as crate::stdlib::Bytef);
                if val == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            if gzhead.hcrc != 0 && (*s).pending > beg_0 {
                if let Some(bytes) = (*s).pending_bytes(beg_0, (*s).pending) {
                    strm.adler = crate::src::crc32::crc32_z(strm.adler, bytes);
                }
            }
            (*s).gzindex = 0 as crate::zutil_h::ulg;
        }
        (*s).status = crate::src::deflate::COMMENT_STATE;
    }
    if (*s).status == crate::src::deflate::COMMENT_STATE {
        let gzhead = gzhead
            .as_ref()
            .expect("missing gzip header in header state");
        if let Some(comment) = gzhead.comment.as_ref() {
            let mut beg_1: crate::zutil_h::ulg = (*s).pending;
            let mut val_0: ::core::ffi::c_int = 0;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if gzhead.hcrc != 0 && (*s).pending > beg_1 {
                        if let Some(bytes) = (*s).pending_bytes(beg_1, (*s).pending) {
                            strm.adler = crate::src::crc32::crc32_z(strm.adler, bytes);
                        }
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
                (*s).put_pending_byte(val_0 as crate::stdlib::Bytef);
                if val_0 == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            if gzhead.hcrc != 0 && (*s).pending > beg_1 {
                if let Some(bytes) = (*s).pending_bytes(beg_1, (*s).pending) {
                    strm.adler = crate::src::crc32::crc32_z(strm.adler, bytes);
                }
            }
        }
        (*s).status = crate::src::deflate::HCRC_STATE;
    }
    if (*s).status == crate::src::deflate::HCRC_STATE {
        if gzhead
            .as_ref()
            .expect("missing gzip header in header state")
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
            (*s).put_pending_byte((strm.adler & 0xff) as crate::stdlib::Byte);
            (*s).put_pending_byte((strm.adler >> 8 & 0xff) as crate::stdlib::Byte);
            strm.adler = crate::src::crc32::crc32(0 as crate::stdlib::uLong, &[]);
        }
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if strm.avail_in != 0 as crate::stdlib::uInt
        || (*s).lookahead != 0 as crate::stdlib::uInt
        || flush != crate::zlib_h::Z_NO_FLUSH && (*s).status != crate::src::deflate::FINISH_STATE
    {
        let mut bstate: block_state = need_more;
        bstate = (if (*s).level == 0 as ::core::ffi::c_int {
            deflate_stored(&mut *s, flush) as ::core::ffi::c_uint
        } else if (*s).strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
            deflate_huff(s, flush) as ::core::ffi::c_uint
        } else if (*s).strategy == crate::zlib_h::Z_RLE {
            deflate_rle(s, flush) as ::core::ffi::c_uint
        } else {
            (match configuration_table[(*s).level as usize].func {
                CompressionEngine::Stored => deflate_stored(&mut *s, flush),
                CompressionEngine::Fast => deflate_fast(s, flush),
                CompressionEngine::Slow => deflate_slow(&mut *s, flush),
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
            if strm.avail_out == 0 as crate::stdlib::uInt {
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
                    let Some(head) = (*s).head.as_mut() else {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    };
                    if head.len() != (*s).hash_size as usize {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    }
                    head.fill(NIL as crate::src::deflate::Posf);
                    (*s).slid = 0 as ::core::ffi::c_int;
                    if (*s).lookahead == 0 as crate::stdlib::uInt {
                        (*s).strstart = 0 as crate::stdlib::uInt;
                        (*s).block_start = 0 as ::core::ffi::c_long;
                        (*s).insert = 0 as crate::stdlib::uInt;
                    }
                }
            }
            flush_pending(strm);
            if strm.avail_out == 0 as crate::stdlib::uInt {
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
        (*s).put_pending_byte((strm.adler & 0xff) as crate::stdlib::Byte);
        (*s).put_pending_byte((strm.adler >> 8 & 0xff) as crate::stdlib::Byte);
        (*s).put_pending_byte((strm.adler >> 16 & 0xff) as crate::stdlib::Byte);
        (*s).put_pending_byte((strm.adler >> 24 & 0xff) as crate::stdlib::Byte);
        (*s).put_pending_byte((strm.total_in & 0xff) as crate::stdlib::Byte);
        (*s).put_pending_byte((strm.total_in >> 8 & 0xff) as crate::stdlib::Byte);
        (*s).put_pending_byte((strm.total_in >> 16 & 0xff) as crate::stdlib::Byte);
        (*s).put_pending_byte((strm.total_in >> 24 & 0xff) as crate::stdlib::Byte);
    } else {
        putShortMSB(
            s,
            (strm.adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
        );
        putShortMSB(
            s,
            (strm.adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
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
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate(strm, flush)
}
pub unsafe fn deflateEnd(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    let stream = core::ptr::from_mut(strm);
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let (status, gzhead, head, prev, pending, allocations, state_allocation) = {
        // The state was installed by the initialization boundary and is only
        // retained while this stream owns it.  All later cleanup uses the
        // reference and the allocator that belong to this stream.
        let Some(state) = (unsafe { strm.state.as_mut() }) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if !core::ptr::eq(state.strm, stream)
            || (state.status != crate::src::deflate::INIT_STATE
                && state.status != crate::src::deflate::GZIP_STATE
                && state.status != crate::src::deflate::EXTRA_STATE
                && state.status != crate::src::deflate::NAME_STATE
                && state.status != crate::src::deflate::COMMENT_STATE
                && state.status != crate::src::deflate::HCRC_STATE
                && state.status != crate::src::deflate::BUSY_STATE
                && state.status != crate::src::deflate::FINISH_STATE)
        {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        (
            state.status,
            state.gzhead,
            state.head.take(),
            state.prev.take(),
            state.pending_buf.take(),
            [state.window.cast::<::core::ffi::c_void>()],
            strm.state.cast::<::core::ffi::c_void>(),
        )
    };
    gzip_header_remove(gzhead);
    drop(head);
    drop(prev);
    drop(pending);
    let zfree = strm.zfree.expect("non-null function pointer");
    for allocation in allocations {
        if !allocation.is_null() {
            unsafe { zfree(strm.opaque, allocation) };
        }
    }
    unsafe { zfree(strm.opaque, state_allocation) };
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
}

/// Adapt legacy internal callers that still carry the ABI stream pointer.
/// The exported boundary converts the pointer directly; this adapter keeps
/// those callers from spreading another raw-to-reference conversion around
/// their cleanup paths.
unsafe fn deflateEnd_from_stream_pointer(strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflateEnd(strm)
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
    if deflateStateCheck(source) != 0 || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    ss = (*source).state as *mut crate::src::deflate::deflate_state;
    let gzhead = gzip_header_clone((*ss).gzhead);
    let Some(head) = (*ss).head.as_deref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if head.len() != (*ss).hash_size as usize {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(head_copy) = clone_head_table(head) else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let Some(source_prev) = (*ss).prev.as_deref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(prev_copy) = clone_prev_table(source_prev) else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let Some(source_pending) = (*ss).pending_buf.as_deref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(mut pending_copy) = allocate_pending_buffer(source_pending.len()) else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    pending_copy.copy_from_slice(source_pending);
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
    // The bytewise state copy above is needed for the ABI allocator-backed
    // fields. Give the copied stream its own owned header snapshot.
    (*ds).gzhead = gzip_header_store(gzhead);
    (*ds).strm = dest;
    // `memcpy` copied the source vector's representation.  Replace that
    // representation without dropping it, then install the independent
    // cloned table prepared above.
    ::core::ptr::write(::core::ptr::addr_of_mut!((*ds).head), Some(head_copy));
    ::core::ptr::write(
        ::core::ptr::addr_of_mut!((*ds).pending_buf),
        Some(pending_copy),
    );
    ::core::ptr::write(::core::ptr::addr_of_mut!((*ds).prev), Some(prev_copy));
    (*ds).window = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
            as crate::stdlib::uInt,
    ) as *mut crate::stdlib::Bytef;
    if (*ds).window.is_null()
        || (*ds).prev.as_ref().is_none_or(|prev| prev.len() != (*ds).w_size as usize)
        || (*ds).head.as_ref().is_none_or(|head| head.len() != (*ds).hash_size as usize)
        || (*ds).pending_buf.is_none()
    {
        deflateEnd_from_stream_pointer(dest);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    crate::stdlib::memcpy(
        (*ds).window as *mut ::core::ffi::c_void,
        (*ss).window as *const ::core::ffi::c_void,
        (*ss).high_water as crate::__stddef_size_t_h::size_t,
    );
    (*ds).sym_buf = (*ds).lit_bufsize as usize;
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
    let lookahead = (*s).lookahead;
    let Some(prev) = (*s).prev.as_deref() else {
        return lookahead;
    };
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
        let Some(&previous) = prev.get((cur_match as crate::stdlib::uInt & wmask) as usize)
        else {
            break;
        };
        cur_match = previous as crate::src::deflate::IPos;
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
    return lookahead;
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

unsafe extern "C" fn deflate_stored(
    s: &mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut min_block: ::core::ffi::c_uint =
        (if s.pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg)
            > s.w_size as crate::zutil_h::ulg
        {
            s.w_size as crate::zutil_h::ulg
        } else {
            s.pending_buf_size.wrapping_sub(5 as crate::zutil_h::ulg)
        }) as ::core::ffi::c_uint;
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used: ::core::ffi::c_uint = (*s.strm).avail_in as ::core::ffi::c_uint;
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        have = (s.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
            >> 3 as ::core::ffi::c_int;
        if (*s.strm).avail_out < have {
            break;
        }
        have = ((*s.strm).avail_out as ::core::ffi::c_uint).wrapping_sub(have);
        left = (s.strstart as ::core::ffi::c_long - s.block_start) as ::core::ffi::c_uint;
        if len as crate::zutil_h::ulg
            > (left as crate::zutil_h::ulg).wrapping_add((*s.strm).avail_in as crate::zutil_h::ulg)
        {
            len = (left as crate::stdlib::uInt).wrapping_add((*s.strm).avail_in)
                as ::core::ffi::c_uint;
        }
        if len > have {
            len = have;
        }
        if len < min_block
            && (len == 0 as ::core::ffi::c_uint && flush != crate::zlib_h::Z_FINISH
                || flush == crate::zlib_h::Z_NO_FLUSH
                || len != (left as crate::stdlib::uInt).wrapping_add((*s.strm).avail_in))
        {
            break;
        }
        last = if flush == crate::zlib_h::Z_FINISH
            && len == (left as crate::stdlib::uInt).wrapping_add((*s.strm).avail_in)
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        crate::src::trees::_tr_stored_block(
            core::ptr::from_mut(s),
            ::core::ptr::null_mut::<crate::stdlib::charf>(),
            0 as crate::zutil_h::ulg,
            last,
        );
        // `_tr_stored_block()` just appended the four-byte length trailer.
        // Replace its zero length with this block's actual length.
        let _ = s.set_stored_block_length(len as crate::stdlib::uInt);
        flush_pending(s.strm);
        if left != 0 {
            if left > len {
                left = len;
            }
            crate::stdlib::memcpy(
                (*s.strm).next_out as *mut ::core::ffi::c_void,
                s.window.offset(s.block_start as isize) as *const ::core::ffi::c_void,
                left as crate::__stddef_size_t_h::size_t,
            );
            (*s.strm).next_out = (*s.strm).next_out.offset(left as isize);
            (*s.strm).avail_out = (*s.strm).avail_out.wrapping_sub(left);
            (*s.strm).total_out = (*s.strm)
                .total_out
                .wrapping_add(left as crate::stdlib::uLong);
            s.block_start += left as ::core::ffi::c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            read_buf(s.strm, (*s.strm).next_out, len);
            (*s.strm).next_out = (*s.strm).next_out.offset(len as isize);
            (*s.strm).avail_out = (*s.strm).avail_out.wrapping_sub(len);
            (*s.strm).total_out = (*s.strm)
                .total_out
                .wrapping_add(len as crate::stdlib::uLong);
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub((*s.strm).avail_in as ::core::ffi::c_uint);
    if used != 0 {
        if used >= s.w_size {
            s.matches = 2 as crate::stdlib::uInt;
            crate::stdlib::memcpy(
                s.window as *mut ::core::ffi::c_void,
                (*s.strm).next_in.offset(-(s.w_size as isize)) as *const ::core::ffi::c_void,
                s.w_size as crate::__stddef_size_t_h::size_t,
            );
            s.strstart = s.w_size;
            s.insert = s.strstart;
        } else {
            if s.window_size
                .wrapping_sub(s.strstart as crate::zutil_h::ulg)
                <= used as crate::zutil_h::ulg
            {
                s.strstart = s.strstart.wrapping_sub(s.w_size);
                crate::stdlib::memcpy(
                    s.window as *mut ::core::ffi::c_void,
                    s.window.offset(s.w_size as isize) as *const ::core::ffi::c_void,
                    s.strstart as crate::__stddef_size_t_h::size_t,
                );
                if s.matches < 2 as crate::stdlib::uInt {
                    s.matches = s.matches.wrapping_add(1);
                }
                if s.insert > s.strstart {
                    s.insert = s.strstart;
                }
            }
            crate::stdlib::memcpy(
                s.window.offset(s.strstart as isize) as *mut ::core::ffi::c_void,
                (*s.strm).next_in.offset(-(used as isize)) as *const ::core::ffi::c_void,
                used as crate::__stddef_size_t_h::size_t,
            );
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
        s.bi_used = 8 as ::core::ffi::c_int;
        return finish_done;
    }
    if flush != crate::zlib_h::Z_NO_FLUSH
        && flush != crate::zlib_h::Z_FINISH
        && (*s.strm).avail_in == 0 as crate::stdlib::uInt
        && s.strstart as ::core::ffi::c_long == s.block_start
    {
        return block_done;
    }
    have = s
        .window_size
        .wrapping_sub(s.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if (*s.strm).avail_in > have && s.block_start >= s.w_size as ::core::ffi::c_long {
        s.block_start -= s.w_size as ::core::ffi::c_long;
        s.strstart = s.strstart.wrapping_sub(s.w_size);
        crate::stdlib::memcpy(
            s.window as *mut ::core::ffi::c_void,
            s.window.offset(s.w_size as isize) as *const ::core::ffi::c_void,
            s.strstart as crate::__stddef_size_t_h::size_t,
        );
        if s.matches < 2 as crate::stdlib::uInt {
            s.matches = s.matches.wrapping_add(1);
        }
        have = have.wrapping_add(s.w_size as ::core::ffi::c_uint);
        if s.insert > s.strstart {
            s.insert = s.strstart;
        }
    }
    if have > (*s.strm).avail_in {
        have = (*s.strm).avail_in as ::core::ffi::c_uint;
    }
    if have != 0 {
        read_buf(s.strm, s.window.offset(s.strstart as isize), have);
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
    min_block = if have > s.w_size {
        s.w_size as ::core::ffi::c_uint
    } else {
        have
    };
    left = (s.strstart as ::core::ffi::c_long - s.block_start) as ::core::ffi::c_uint;
    if left >= min_block
        || (left != 0 || flush == crate::zlib_h::Z_FINISH)
            && flush != crate::zlib_h::Z_NO_FLUSH
            && (*s.strm).avail_in == 0 as crate::stdlib::uInt
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == crate::zlib_h::Z_FINISH
            && (*s.strm).avail_in == 0 as crate::stdlib::uInt
            && len == left
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        crate::src::trees::_tr_stored_block(
            core::ptr::from_mut(s),
            (s.window as *mut crate::stdlib::charf).offset(s.block_start as isize),
            len as crate::zutil_h::ulg,
            last,
        );
        s.block_start += len as ::core::ffi::c_long;
        flush_pending(s.strm);
    }
    if last != 0 {
        s.bi_used = 8 as ::core::ffi::c_int;
    }
    return (if last != 0 {
        finish_started as ::core::ffi::c_int
    } else {
        need_more as ::core::ffi::c_int
    }) as block_state;
}

unsafe fn deflate_fast(
    s: &mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window(core::ptr::from_mut(s));
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
            hash_head = s
                .insert_string()
                .unwrap_or(NIL as crate::src::deflate::IPos);
        }
        if hash_head != NIL as crate::src::deflate::IPos
            && (s.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                <= s.w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
        {
            s.match_length = longest_match(core::ptr::from_mut(s), hash_head);
        }
        if s.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut len: crate::zutil_h::uch =
                s.match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let mut dist: crate::zutil_h::ush =
                s.strstart.wrapping_sub(s.match_start) as crate::zutil_h::ush;
            let _ = s.put_symbol(dist as crate::stdlib::uInt, len as crate::stdlib::uInt);
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as usize
                + crate::src::deflate::LITERALS as usize
                + 1;
            s.dyn_ltree[length_code].fc = s.dyn_ltree[length_code].fc.wrapping_add(1);
            let distance_code = (if (dist as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                crate::src::trees::_dist_code[dist as usize]
            } else {
                crate::src::trees::_dist_code[256 + (dist as usize >> 7)]
            }) as usize;
            s.dyn_dtree[distance_code].fc = s.dyn_dtree[distance_code].fc.wrapping_add(1);
            bflush = (s.sym_next == s.sym_end) as ::core::ffi::c_int;
            s.lookahead = s.lookahead.wrapping_sub(s.match_length);
            if s.match_length <= s.max_lazy_match
                && s.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            {
                s.match_length = s.match_length.wrapping_sub(1);
                loop {
                    s.strstart = s.strstart.wrapping_add(1);
                    let _ = s.insert_string();
                    s.match_length = s.match_length.wrapping_sub(1);
                    if s.match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                s.strstart = s.strstart.wrapping_add(1);
            } else {
                s.strstart = s.strstart.wrapping_add(s.match_length);
                s.match_length = 0 as crate::stdlib::uInt;
                let Some(first) = s.window_byte(s.strstart) else {
                    return need_more;
                };
                let Some(second) = s.window_byte(s.strstart.wrapping_add(1)) else {
                    return need_more;
                };
                s.ins_h = first as crate::stdlib::uInt;
                s.ins_h = (s.ins_h << s.hash_shift ^ second as crate::stdlib::uInt) & s.hash_mask;
            }
        } else {
            let Some(cc) = s.window_byte(s.strstart) else {
                return need_more;
            };
            let cc = cc as crate::zutil_h::uch;
            let _ = s.put_symbol(0, cc as crate::stdlib::uInt);
            s.dyn_ltree[cc as usize].fc = s.dyn_ltree[cc as usize].fc.wrapping_add(1);
            bflush = (s.sym_next == s.sym_end) as ::core::ffi::c_int;
            s.lookahead = s.lookahead.wrapping_sub(1);
            s.strstart = s.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            let block_len =
                (s.strstart as ::core::ffi::c_long - s.block_start) as crate::zutil_h::ulg;
            let block = if s.block_start >= 0 {
                s.block_data(s.block_start as crate::stdlib::uInt, block_len)
            } else {
                None
            };
            crate::src::trees::tr_flush_block_impl(s, block.as_deref(), block_len, 0);
            s.block_start = s.strstart as ::core::ffi::c_long;
            flush_pending(s.strm);
            if s.avail_out() == Some(0) {
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
        let block_len = (s.strstart as ::core::ffi::c_long - s.block_start) as crate::zutil_h::ulg;
        let block = if s.block_start >= 0 {
            s.block_data(s.block_start as crate::stdlib::uInt, block_len)
        } else {
            None
        };
        crate::src::trees::tr_flush_block_impl(s, block.as_deref(), block_len, 1);
        s.block_start = s.strstart as ::core::ffi::c_long;
        flush_pending(s.strm);
        if s.avail_out() == Some(0) {
            return finish_started;
        }
        return finish_done;
    }
    if s.sym_next != 0 {
        let block_len = (s.strstart as ::core::ffi::c_long - s.block_start) as crate::zutil_h::ulg;
        let block = if s.block_start >= 0 {
            s.block_data(s.block_start as crate::stdlib::uInt, block_len)
        } else {
            None
        };
        crate::src::trees::tr_flush_block_impl(s, block.as_deref(), block_len, 0);
        s.block_start = s.strstart as ::core::ffi::c_long;
        flush_pending(s.strm);
        if s.avail_out() == Some(0) {
            return need_more;
        }
    }
    return block_done;
}

unsafe fn deflate_slow(
    s: &mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window(s as *mut crate::src::deflate::deflate_state);
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
            hash_head = s
                .insert_string()
                .unwrap_or(NIL as crate::src::deflate::IPos);
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
            (*s).match_length =
                longest_match(s as *mut crate::src::deflate::deflate_state, hash_head);
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
            let _ = (*s).put_symbol(dist as crate::stdlib::uInt, len as crate::stdlib::uInt);
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as usize
                + crate::src::deflate::LITERALS as usize
                + 1;
            (*s).dyn_ltree[length_code].fc = (*s).dyn_ltree[length_code].fc.wrapping_add(1);
            let distance_code = if dist < 256 {
                crate::src::trees::_dist_code[dist as usize]
            } else {
                crate::src::trees::_dist_code[256 + (dist as usize >> 7)]
            } as usize;
            (*s).dyn_dtree[distance_code].fc = (*s).dyn_dtree[distance_code].fc.wrapping_add(1);
            bflush = ((*s).sym_next == (*s).sym_end) as ::core::ffi::c_int;
            (*s).lookahead = (*s)
                .lookahead
                .wrapping_sub((*s).prev_length.wrapping_sub(1 as crate::stdlib::uInt));
            (*s).prev_length = (*s).prev_length.wrapping_sub(2 as crate::stdlib::uInt);
            loop {
                (*s).strstart = (*s).strstart.wrapping_add(1);
                if (*s).strstart <= max_insert {
                    hash_head = s
                        .insert_string()
                        .unwrap_or(NIL as crate::src::deflate::IPos);
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
            let mut cc: crate::zutil_h::uch = s
                .window_byte((*s).strstart.wrapping_sub(1 as crate::stdlib::uInt))
                .unwrap_or(0) as crate::zutil_h::uch;
            let _ = (*s).put_symbol(0, cc as crate::stdlib::uInt);
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
        let mut cc_0: crate::zutil_h::uch = s
            .window_byte((*s).strstart.wrapping_sub(1 as crate::stdlib::uInt))
            .unwrap_or(0) as crate::zutil_h::uch;
        let _ = (*s).put_symbol(0, cc_0 as crate::stdlib::uInt);
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

unsafe fn deflate_rle(
    s: &mut crate::src::deflate::deflate_state,
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if s.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_window(core::ptr::from_mut(s));
            if s.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if s.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        s.match_length = 0;
        if s.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt && s.strstart > 0 {
            let run_start = s.strstart;
            let max_match = crate::zutil_h::MAX_MATCH as usize;
            if let (Some(previous), Some(window)) = (
                s.window_byte(run_start - 1),
                s.window_bytes(run_start, max_match),
            ) {
                // The old unrolled pointer loop measured the prefix of the
                // current window position that repeats the preceding byte.
                // A bounded slice has the same MAX_MATCH limit without
                // speculatively walking raw pointers past the prefix.
                let run = window
                    .iter()
                    .position(|&byte| byte != previous)
                    .unwrap_or(window.len());
                s.match_length = (run as crate::stdlib::uInt).min(s.lookahead);
            }
        }
        if s.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut len: crate::zutil_h::uch =
                s.match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let mut dist: crate::zutil_h::ush = 1 as ::core::ffi::c_int as crate::zutil_h::ush;
            let _ = s.put_symbol(dist as crate::stdlib::uInt, len as crate::stdlib::uInt);
            dist = dist.wrapping_sub(1);
            let length_code = crate::src::trees::_length_code[len as usize] as usize
                + crate::src::deflate::LITERALS as usize
                + 1;
            s.dyn_ltree[length_code].fc = s.dyn_ltree[length_code].fc.wrapping_add(1);
            let distance_code = (if (dist as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
                crate::src::trees::_dist_code[dist as usize]
            } else {
                crate::src::trees::_dist_code[256 + (dist as usize >> 7)]
            }) as usize;
            s.dyn_dtree[distance_code].fc = s.dyn_dtree[distance_code].fc.wrapping_add(1);
            bflush = (s.sym_next == s.sym_end) as ::core::ffi::c_int;
            s.lookahead = s.lookahead.wrapping_sub(s.match_length);
            s.strstart = s.strstart.wrapping_add(s.match_length);
            s.match_length = 0;
        } else {
            let Some(cc) = s.window_byte(s.strstart) else {
                return need_more;
            };
            let cc = cc as crate::zutil_h::uch;
            let _ = s.put_symbol(0, cc as crate::stdlib::uInt);
            s.dyn_ltree[cc as usize].fc = s.dyn_ltree[cc as usize].fc.wrapping_add(1);
            bflush = (s.sym_next == s.sym_end) as ::core::ffi::c_int;
            s.lookahead = s.lookahead.wrapping_sub(1);
            s.strstart = s.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            let block_len =
                (s.strstart as ::core::ffi::c_long - s.block_start) as crate::zutil_h::ulg;
            let block = if s.block_start >= 0 {
                s.block_data(s.block_start as crate::stdlib::uInt, block_len)
            } else {
                None
            };
            crate::src::trees::tr_flush_block_impl(s, block.as_deref(), block_len, 0);
            s.block_start = s.strstart as ::core::ffi::c_long;
            flush_pending(s.strm);
            if s.avail_out() == Some(0) {
                return need_more;
            }
        }
    }
    s.insert = 0;
    if flush == crate::zlib_h::Z_FINISH {
        let block_len = (s.strstart as ::core::ffi::c_long - s.block_start) as crate::zutil_h::ulg;
        let block = if s.block_start >= 0 {
            s.block_data(s.block_start as crate::stdlib::uInt, block_len)
        } else {
            None
        };
        crate::src::trees::tr_flush_block_impl(s, block.as_deref(), block_len, 1);
        s.block_start = s.strstart as ::core::ffi::c_long;
        flush_pending(s.strm);
        if s.avail_out() == Some(0) {
            return finish_started;
        }
        return finish_done;
    }
    if s.sym_next != 0 {
        let block_len = (s.strstart as ::core::ffi::c_long - s.block_start) as crate::zutil_h::ulg;
        let block = if s.block_start >= 0 {
            s.block_data(s.block_start as crate::stdlib::uInt, block_len)
        } else {
            None
        };
        crate::src::trees::tr_flush_block_impl(s, block.as_deref(), block_len, 0);
        s.block_start = s.strstart as ::core::ffi::c_long;
        flush_pending(s.strm);
        if s.avail_out() == Some(0) {
            return need_more;
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
        let _ = (*s).put_symbol(0, cc as crate::stdlib::uInt);
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
