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

pub struct C2Rust_Unnamed_0 {
    pub value: crate::zutil_h::ush,
}

pub type static_tree_desc = crate::src::deflate::static_tree_desc_s;

pub type tree_desc = crate::src::deflate::tree_desc_s;
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DynamicTree {
    Literal = 0,
    Distance = 1,
    BitLength = 2,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tree_desc_s {
    pub dynamic_tree: crate::src::deflate::DynamicTree,
    pub max_code: ::core::ffi::c_int,
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
    /// Callback-owned pending/symbol allocation. `None` represents the
    /// pre-allocation and allocation-failure states. A callback result is
    /// recorded as non-null before implementation code can use it.
    ///
    /// This removes nullable raw storage from the algorithm state while the
    /// remaining callback-ownership migration is completed. Existing private
    /// algorithms still establish short-lived views; moving those crossings
    /// to exported boundaries is the next migration step.
    pub pending_buf: Option<::core::ptr::NonNull<crate::stdlib::Bytef>>,
    pub pending_buf_size: crate::zutil_h::ulg,
    pub pending_out_offset: usize,
    pub pending: crate::zutil_h::ulg,
    /// A one-call override for the stored-block length written by the tree
    /// boundary. `deflate_stored()` emits that header into pending storage
    /// while sending its payload directly to caller output.
    pub pending_header_len_override: Option<crate::zutil_h::ulg>,
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
    /// Byte offset of the symbol triplets within `pending_buf`.
    ///
    /// The triplets temporally overlay the pending-byte allocation, so this
    /// must remain an index rather than an interior raw pointer.
    pub sym_buf_offset: usize,
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
    /// Initializes the callback-allocated deflate state without relying on a
    /// foreign whole-struct memset.  The zero values intentionally match the
    /// translated C allocation state before `deflateInit2_` fills its fields.
    fn newly_allocated() -> Self {
        let zero_tree = ct_data_s {
            fc: C2Rust_Unnamed_1 { value: 0 },
            dl: C2Rust_Unnamed_0 { value: 0 },
        };
        let zero_desc = tree_desc_s {
            dynamic_tree: DynamicTree::Literal,
            max_code: 0,
        };

        Self {
            strm: ::core::ptr::null_mut(),
            status: 0,
            pending_buf: None,
            pending_buf_size: 0,
            pending_out_offset: 0,
            pending: 0,
            pending_header_len_override: None,
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
            dyn_ltree: [zero_tree; 573],
            dyn_dtree: [zero_tree; 61],
            bl_tree: [zero_tree; 39],
            l_desc: zero_desc,
            d_desc: zero_desc,
            bl_desc: zero_desc,
            bl_count: [0; 16],
            heap: [0; 573],
            heap_len: 0,
            heap_max: 0,
            depth: [0; 573],
            sym_buf_offset: 0,
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PendingStorageLayout {
    pub total_len: usize,
    pub symbol_offset: usize,
    pub symbol_len: usize,
    pub symbol_flush_threshold: crate::stdlib::uInt,
}

/// The single callback allocation that backs both pending output and symbols.
///
/// The byte count must be computed in zlib's `uInt` domain: this is the
/// `zalloc(opaque, lit_bufsize, 4)` request, not an unrestricted `usize`
/// allocation.  Keeping the calculation here lets future ownership code
/// reject impossible requests before it crosses the callback boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PendingStorageAllocationPlan {
    pub items: crate::stdlib::uInt,
    pub item_size: crate::stdlib::uInt,
    pub total_len: usize,
}

impl PendingStorageAllocationPlan {
    /// Describe the two temporal views of this one callback allocation.
    ///
    /// Keeping this derivation on the checked allocation plan prevents a
    /// lifecycle boundary from validating one geometry and allocating another
    /// one with a hand-written item size.
    pub(crate) fn layout(self) -> PendingStorageLayout {
        let symbol_offset = self.items as usize;
        PendingStorageLayout {
            total_len: self.total_len,
            symbol_offset,
            symbol_len: self.total_len.wrapping_sub(symbol_offset),
            symbol_flush_threshold: self.items.wrapping_sub(1).wrapping_mul(3),
        }
    }
}

pub(crate) fn pending_storage_allocation_plan(
    lit_bufsize: crate::stdlib::uInt,
) -> Option<PendingStorageAllocationPlan> {
    const ITEM_SIZE: crate::stdlib::uInt = 4;

    let total_in_c_domain = lit_bufsize.checked_mul(ITEM_SIZE)?;
    Some(PendingStorageAllocationPlan {
        items: lit_bufsize,
        item_size: ITEM_SIZE,
        total_len: usize::try_from(total_in_c_domain).ok()?,
    })
}

pub(crate) fn pending_storage_layout(lit_bufsize: crate::stdlib::uInt) -> PendingStorageLayout {
    let symbol_offset = lit_bufsize as usize;
    // Valid deflate states use the checked callback allocation plan.  Retain
    // the translated wrapping layout for malformed/internal state so the
    // callers that only inspect layout keep their existing behavior.
    pending_storage_allocation_plan(lit_bufsize).map_or_else(
        || PendingStorageLayout {
            total_len: symbol_offset.wrapping_mul(4),
            symbol_offset,
            symbol_len: symbol_offset.wrapping_mul(3),
            symbol_flush_threshold: lit_bufsize.wrapping_sub(1).wrapping_mul(3),
        },
        PendingStorageAllocationPlan::layout,
    )
}

/// Validate the scalar metadata that describes a callback-backed pending
/// allocation before an FFI boundary turns its raw pointer into a slice.
///
/// The pending bytes and the symbol triplets are two temporal views of one
/// allocation.  Keeping their geometry in sync here prevents a later safe
/// view from silently trusting stale or hand-written state fields.
pub(crate) fn pending_storage_layout_from_metadata(
    lit_bufsize: crate::stdlib::uInt,
    pending_buf_size: crate::zutil_h::ulg,
    sym_buf_offset: usize,
    sym_end: crate::stdlib::uInt,
) -> Option<PendingStorageLayout> {
    let layout = pending_storage_allocation_plan(lit_bufsize)?.layout();
    if usize::try_from(pending_buf_size).ok()? != layout.total_len
        || sym_buf_offset != layout.symbol_offset
        || sym_end != layout.symbol_flush_threshold
    {
        return None;
    }
    Some(layout)
}

/// Read the pending/symbol allocation geometry from an established deflate
/// state.  Exported boundaries use this before forming a raw slice, so the
/// slice length always comes from the checked callback-allocation plan rather
/// than a separately mutable size field.
pub(crate) fn pending_storage_layout_for_state(
    state: &internal_state,
) -> Option<PendingStorageLayout> {
    pending_storage_layout_from_metadata(
        state.lit_bufsize,
        state.pending_buf_size,
        state.sym_buf_offset,
        state.sym_end,
    )
}

pub(crate) struct PendingStorageView<'a> {
    bytes: &'a mut [crate::stdlib::Bytef],
    layout: PendingStorageLayout,
}

/// A read-only view of the single allocation shared by pending output and
/// symbol data.  Keeping the source side immutable lets copy operations use
/// ordinary slice copies without reintroducing an interior `sym_buf` alias.
pub(crate) struct PendingStorageReadView<'a> {
    bytes: &'a [crate::stdlib::Bytef],
    layout: PendingStorageLayout,
}

/// The initialized portions of a pending/symbol allocation that `deflateCopy`
/// must reproduce.  Pending output begins at its drain cursor, whereas symbol
/// data always starts at the shared allocation's symbol offset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PendingStorageCopyPlan {
    pending_out_offset: usize,
    pending_len: usize,
    symbol_len: usize,
}

pub(crate) fn pending_storage_copy_plan(
    layout: PendingStorageLayout,
    pending_out_offset: usize,
    pending: crate::zutil_h::ulg,
    sym_next: crate::stdlib::uInt,
) -> Option<PendingStorageCopyPlan> {
    let pending_len = usize::try_from(pending).ok()?;
    let symbol_len = usize::try_from(sym_next).ok()?;
    let pending_end = pending_out_offset.checked_add(pending_len)?;
    if pending_end > layout.total_len || symbol_len > layout.symbol_len {
        return None;
    }

    Some(PendingStorageCopyPlan {
        pending_out_offset,
        pending_len,
        symbol_len,
    })
}

impl<'a> PendingStorageView<'a> {
    pub(crate) fn new(
        bytes: &'a mut [crate::stdlib::Bytef],
        layout: PendingStorageLayout,
    ) -> Option<Self> {
        // A caller can pass a larger established buffer, but this view never
        // exposes bytes after the callback-allocation layout.  This keeps the
        // temporal pending/symbol owner bounded without requiring callers to
        // split a larger borrowed buffer first.
        if bytes.len() < layout.total_len {
            return None;
        }
        let bytes = bytes.get_mut(..layout.total_len)?;
        Some(Self { bytes, layout })
    }

    pub(crate) fn pending_bytes(&mut self) -> &mut [crate::stdlib::Bytef] {
        &mut self.bytes[..self.layout.total_len]
    }

    pub(crate) fn symbol_bytes(&mut self) -> &mut [crate::stdlib::Bytef] {
        &mut self.bytes[self.layout.symbol_offset..self.layout.total_len]
    }

    pub(crate) fn pending_range(
        &mut self,
        offset: crate::zutil_h::ulg,
        len: crate::zutil_h::ulg,
    ) -> Option<&mut [crate::stdlib::Bytef]> {
        let start = usize::try_from(offset).ok()?;
        let len = usize::try_from(len).ok()?;
        self.pending_range_usize(start, len)
    }

    pub(crate) fn pending_triplet(&self, first: usize) -> Option<[crate::stdlib::Bytef; 3]> {
        let second = first.checked_add(1)?;
        let third = first.checked_add(2)?;
        Some([
            *self.bytes.get(first)?,
            *self.bytes.get(second)?,
            *self.bytes.get(third)?,
        ])
    }

    pub(crate) fn append_pending(
        &mut self,
        pending: &mut crate::zutil_h::ulg,
        bytes: &[crate::stdlib::Bytef],
    ) -> bool {
        let Ok(len) = crate::zutil_h::ulg::try_from(bytes.len()) else {
            return false;
        };
        let Some(output) = self.pending_range(*pending, len) else {
            return false;
        };
        output.copy_from_slice(bytes);
        *pending = pending.wrapping_add(bytes.len() as crate::zutil_h::ulg);
        true
    }

    pub(crate) fn write_symbol_triplet(
        &mut self,
        cursors: [crate::stdlib::uInt; 3],
        bytes: [crate::zutil_h::uchf; 3],
    ) -> bool {
        let Ok(first) = usize::try_from(cursors[0]) else {
            return false;
        };
        let Ok(second) = usize::try_from(cursors[1]) else {
            return false;
        };
        let Ok(third) = usize::try_from(cursors[2]) else {
            return false;
        };
        let symbols = self.symbol_bytes();
        if first >= symbols.len() || second >= symbols.len() || third >= symbols.len() {
            return false;
        }
        symbols[first] = bytes[0];
        symbols[second] = bytes[1];
        symbols[third] = bytes[2];
        true
    }

    pub(crate) fn copy_initialized_from(
        &mut self,
        source: &PendingStorageReadView<'_>,
        plan: PendingStorageCopyPlan,
    ) -> bool {
        let Some(pending_end) = plan.pending_out_offset.checked_add(plan.pending_len) else {
            return false;
        };
        if self.layout != source.layout
            || pending_end > self.layout.total_len
            || plan.symbol_len > self.layout.symbol_len
        {
            return false;
        }

        let Some(source_pending) = source.pending_range(plan.pending_out_offset, plan.pending_len)
        else {
            return false;
        };
        let Some(source_symbols) = source.symbol_prefix(plan.symbol_len) else {
            return false;
        };

        // Preflight both destination ranges before copying either one.  The
        // ranges may overlap temporally, so preserve the C copy order: pending
        // bytes first, then the current symbol prefix.
        if self
            .pending_range_usize(plan.pending_out_offset, plan.pending_len)
            .is_none()
            || self.symbol_prefix(plan.symbol_len).is_none()
        {
            return false;
        }
        let Some(destination_pending) =
            self.pending_range_usize(plan.pending_out_offset, plan.pending_len)
        else {
            return false;
        };
        destination_pending.copy_from_slice(source_pending);
        let Some(destination_symbols) = self.symbol_prefix(plan.symbol_len) else {
            return false;
        };
        destination_symbols.copy_from_slice(source_symbols);
        true
    }

    fn pending_range_usize(
        &mut self,
        offset: usize,
        len: usize,
    ) -> Option<&mut [crate::stdlib::Bytef]> {
        let end = offset.checked_add(len)?;
        self.pending_bytes().get_mut(offset..end)
    }
}

impl<'a> PendingStorageReadView<'a> {
    pub(crate) fn new(
        bytes: &'a [crate::stdlib::Bytef],
        layout: PendingStorageLayout,
    ) -> Option<Self> {
        // Match the mutable view's bounded-prefix behavior so deflateCopy's
        // source and destination describe the same callback allocation even
        // when their established backing views are larger.
        if bytes.len() < layout.total_len {
            return None;
        }
        let bytes = bytes.get(..layout.total_len)?;
        Some(Self { bytes, layout })
    }

    fn pending_range(&self, offset: usize, len: usize) -> Option<&[crate::stdlib::Bytef]> {
        let end = offset.checked_add(len)?;
        self.bytes.get(offset..end)
    }

    fn symbol_prefix(&self, len: usize) -> Option<&[crate::stdlib::Bytef]> {
        let end = self.layout.symbol_offset.checked_add(len)?;
        self.bytes.get(self.layout.symbol_offset..end)
    }
}

impl<'a> PendingStorageView<'a> {
    fn symbol_prefix(&mut self, len: usize) -> Option<&mut [crate::stdlib::Bytef]> {
        let end = self.layout.symbol_offset.checked_add(len)?;
        self.bytes.get_mut(self.layout.symbol_offset..end)
    }
}

pub(crate) fn with_pending_storage<Result>(
    bytes: &mut [crate::stdlib::Bytef],
    layout: PendingStorageLayout,
    callback: impl FnOnce(&mut PendingStorageView<'_>) -> Result,
) -> Option<Result> {
    let mut storage = PendingStorageView::new(bytes, layout)?;
    Some(callback(&mut storage))
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
pub use crate::src::trees::_tr_align_ffi as _tr_align;
pub use crate::src::trees::_tr_flush_block_ffi as _tr_flush_block;
pub use crate::src::trees::_tr_init;
pub use crate::src::trees::_tr_stored_block_ffi as _tr_stored_block;
use crate::src::zutil::z_error_message;
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

// zlib's configuration table selects one of three compressor routines. Keep
// that policy as data instead of storing raw function pointers: function
// pointer equality is not a stable Rust abstraction, and the table is an
// internal implementation detail rather than an ABI record.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DeflateCompressionFunction {
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
    function: DeflateCompressionFunction,
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
        function: DeflateCompressionFunction::Stored,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        function: DeflateCompressionFunction::Slow,
    },
];

fn slide_hash_entry(position: ::core::ffi::c_uint, window_size: crate::stdlib::uInt) -> Posf {
    (if position >= window_size {
        position.wrapping_sub(window_size)
    } else {
        NIL as ::core::ffi::c_uint
    }) as crate::src::deflate::Pos as Posf
}

fn slide_hash_entries(entries: &mut [Posf], window_size: crate::stdlib::uInt) {
    for entry in entries {
        *entry = slide_hash_entry(*entry as ::core::ffi::c_uint, window_size);
    }
}

fn slide_hash_core(head: &mut [Posf], prev: &mut [Posf], window_size: crate::stdlib::uInt) {
    slide_hash_entries(head, window_size);
    slide_hash_entries(prev, window_size);
}

fn clamped_copy_len(
    available: crate::zutil_h::ulg,
    requested: crate::zutil_h::ulg,
) -> ::core::ffi::c_uint {
    available.min(requested) as ::core::ffi::c_uint
}

fn deflate_rle_clamp_match_length(
    match_length: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    match_length.min(lookahead)
}

fn deflate_rle_match_length(
    scan_distance: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    deflate_rle_clamp_match_length(
        (crate::zutil_h::MAX_MATCH as crate::stdlib::uInt).wrapping_sub(scan_distance),
        lookahead,
    )
}

fn deflate_rle_can_scan_match(
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
) -> bool {
    lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt && strstart > 0
}

fn deflate_rle_scan_indices(
    strstart: crate::stdlib::uInt,
) -> (
    crate::stdlib::uInt,
    [crate::stdlib::uInt; 3],
    crate::stdlib::uInt,
    crate::stdlib::uInt,
) {
    let initial = [strstart, strstart.wrapping_add(1), strstart.wrapping_add(2)];
    (
        strstart.wrapping_sub(1),
        initial,
        initial[2],
        strstart.wrapping_add(crate::zutil_h::MAX_MATCH as crate::stdlib::uInt),
    )
}

fn deflate_rle_next_scan_indices(
    scan: crate::stdlib::uInt,
) -> ([crate::stdlib::uInt; 8], crate::stdlib::uInt) {
    let first = scan.wrapping_add(1);
    let comparisons = [
        first,
        first.wrapping_add(1),
        first.wrapping_add(2),
        first.wrapping_add(3),
        first.wrapping_add(4),
        first.wrapping_add(5),
        first.wrapping_add(6),
        first.wrapping_add(7),
    ];
    (comparisons, comparisons[7])
}

/// Find the RLE run at `strstart` using only checked window accesses.
///
/// The translated loop deliberately reads an unrolled group past the logical
/// `MAX_MATCH` endpoint before clamping its result.  Keep that order here so
/// a fully repeated run has the same length, while malformed callback-backed
/// storage simply declines to produce a match instead of indexing past it.
fn deflate_rle_scan_match(
    window: &[crate::stdlib::Bytef],
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
) -> Option<crate::stdlib::uInt> {
    if !deflate_rle_can_scan_match(lookahead, strstart) {
        return Some(0);
    }

    let (previous_index, initial_indices, mut scan, strend) = deflate_rle_scan_indices(strstart);
    let previous = *window.get(previous_index as usize)?;
    for index in initial_indices {
        if *window.get(index as usize)? != previous {
            return Some(0);
        }
    }

    loop {
        let (comparisons, next_scan) = deflate_rle_next_scan_indices(scan);
        scan = next_scan;
        let mut matches = true;
        for index in comparisons {
            if *window.get(index as usize)? != previous {
                matches = false;
                break;
            }
        }
        if !matches || scan >= strend {
            break;
        }
    }

    Some(deflate_rle_match_length(
        strend.wrapping_sub(scan),
        lookahead,
    ))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeflateRleTallyPlan {
    MatchWithoutCount,
    Literal,
}

fn deflate_rle_tally_plan(match_length: crate::stdlib::uInt) -> DeflateRleTallyPlan {
    if match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        DeflateRleTallyPlan::MatchWithoutCount
    } else {
        DeflateRleTallyPlan::Literal
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DeflateRleMatchTally {
    cursors: [crate::stdlib::uInt; 3],
    next_sym: crate::stdlib::uInt,
    symbol_bytes: [crate::zutil_h::uchf; 3],
    length_tree_index: usize,
    distance_tree_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DeflateLiteralTally {
    cursors: [crate::stdlib::uInt; 3],
    next_sym: crate::stdlib::uInt,
    symbol_bytes: [crate::zutil_h::uchf; 3],
    literal_tree_index: usize,
}

fn deflate_rle_match_tally_plan(
    match_length: crate::stdlib::uInt,
    sym_next: crate::stdlib::uInt,
) -> DeflateRleMatchTally {
    let length = match_length.wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
    let distance = 1;
    let (cursors, next_sym) = symbol_triplet_cursors(sym_next);
    let (length_tree_index, distance_tree_index) =
        crate::src::trees::tally_match_tree_indices(distance, length);

    DeflateRleMatchTally {
        cursors,
        next_sym,
        symbol_bytes: crate::src::trees::tally_symbol_bytes(distance, length),
        length_tree_index,
        distance_tree_index,
    }
}

fn deflate_literal_tally_plan(
    literal: crate::zutil_h::uch,
    sym_next: crate::stdlib::uInt,
) -> DeflateLiteralTally {
    let (cursors, next_sym) = symbol_triplet_cursors(sym_next);

    DeflateLiteralTally {
        cursors,
        next_sym,
        symbol_bytes: [0, 0, literal as crate::zutil_h::uchf],
        literal_tree_index: literal as usize,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeflateRleRefillAction {
    Continue,
    NeedMore,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeflateMatchRefillAction {
    Continue,
    NeedMore,
    EndBlock,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeflateHuffRefillAction {
    Continue,
    NeedMore,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeflateFinalFlushAction {
    Finish,
    FlushPendingSymbols,
    Done,
}

fn deflate_match_refill_action(
    lookahead: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> DeflateMatchRefillAction {
    if lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
        && flush == crate::zlib_h::Z_NO_FLUSH
    {
        DeflateMatchRefillAction::NeedMore
    } else if lookahead == 0 {
        DeflateMatchRefillAction::EndBlock
    } else {
        DeflateMatchRefillAction::Continue
    }
}

fn deflate_rle_refill_action(
    lookahead: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> DeflateRleRefillAction {
    if lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
        if flush == crate::zlib_h::Z_NO_FLUSH {
            DeflateRleRefillAction::NeedMore
        } else if lookahead == 0 {
            DeflateRleRefillAction::Done
        } else {
            DeflateRleRefillAction::Continue
        }
    } else {
        DeflateRleRefillAction::Continue
    }
}

fn deflate_huff_refill_action(
    lookahead: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
) -> DeflateHuffRefillAction {
    if lookahead != 0 {
        DeflateHuffRefillAction::Continue
    } else if flush == crate::zlib_h::Z_NO_FLUSH {
        DeflateHuffRefillAction::NeedMore
    } else {
        DeflateHuffRefillAction::Done
    }
}

fn deflate_final_flush_action(
    flush: ::core::ffi::c_int,
    sym_next: crate::stdlib::uInt,
) -> DeflateFinalFlushAction {
    if flush == crate::zlib_h::Z_FINISH {
        DeflateFinalFlushAction::Finish
    } else if sym_next != 0 {
        DeflateFinalFlushAction::FlushPendingSymbols
    } else {
        DeflateFinalFlushAction::Done
    }
}

fn deflate_flush_block_state_after_output(
    avail_out: crate::stdlib::uInt,
    final_block: bool,
) -> Option<block_state> {
    if avail_out == 0 {
        Some(if final_block {
            finish_started
        } else {
            need_more
        })
    } else {
        None
    }
}

fn deflate_rle_match_state_after_emit(
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    match_length: crate::stdlib::uInt,
) -> (
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    crate::stdlib::uInt,
) {
    (
        lookahead.wrapping_sub(match_length),
        strstart.wrapping_add(match_length),
        0,
    )
}

fn deflate_literal_state_after_emit(
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::stdlib::uInt) {
    (lookahead.wrapping_sub(1), strstart.wrapping_add(1))
}

fn deflate_fast_should_insert_match(
    match_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> bool {
    match_length <= max_lazy_match && lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DeflateFastMatchProgress {
    lookahead: crate::stdlib::uInt,
    remaining_match_length: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    insert: bool,
}

fn deflate_fast_match_progress(
    match_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
) -> DeflateFastMatchProgress {
    let lookahead = lookahead.wrapping_sub(match_length);
    let insert = deflate_fast_should_insert_match(match_length, max_lazy_match, lookahead);
    DeflateFastMatchProgress {
        lookahead,
        remaining_match_length: if insert {
            match_length.wrapping_sub(1)
        } else {
            0
        },
        strstart: strstart.wrapping_add(match_length),
        insert,
    }
}

/// Translate a match length offset to its dynamic literal/length tree slot.
///
/// Match offsets are stored as a byte in the deflate symbol stream, so the
/// immutable zlib lookup table can be indexed directly.  Keeping this lookup
/// safe removes the old raw address-plus-offset reads from both compressor
/// strategies.
fn deflate_length_tree_index(length_offset: crate::zutil_h::uch) -> usize {
    crate::src::trees::_length_code[length_offset as usize] as usize
        + crate::src::deflate::LITERALS as usize
        + 1
}

fn deflate_insert_after_block(strstart: crate::stdlib::uInt) -> crate::stdlib::uInt {
    strstart.min((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt)
}

pub(crate) fn symbol_triplet_cursors(
    start: crate::stdlib::uInt,
) -> ([crate::stdlib::uInt; 3], crate::stdlib::uInt) {
    let second = start.wrapping_add(1);
    let third = second.wrapping_add(1);
    ([start, second, third], third.wrapping_add(1))
}

fn symbol_buffer_is_full(sym_next: crate::stdlib::uInt, sym_end: crate::stdlib::uInt) -> bool {
    sym_next == sym_end
}

fn deflate_huff_literal_progress(
    sym_next_after_literal: crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
) -> (
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    bool,
) {
    let (lookahead, strstart) = deflate_literal_state_after_emit(lookahead, strstart);
    (
        sym_next_after_literal,
        lookahead,
        strstart,
        symbol_buffer_is_full(sym_next_after_literal, sym_end),
    )
}

fn can_search_hash_match(
    hash_head: crate::src::deflate::IPos,
    strstart: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
) -> bool {
    hash_head != NIL as crate::src::deflate::IPos
        && (strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
            <= w_size.wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
}

fn deflate_slow_can_search_match(
    hash_head: crate::src::deflate::IPos,
    previous_match_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
) -> bool {
    hash_head != NIL as crate::src::deflate::IPos
        && previous_match_length < max_lazy_match
        && can_search_hash_match(hash_head, strstart, w_size)
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

fn read_buf_input_progress_after_copy(
    avail_in: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    copied: ::core::ffi::c_uint,
) -> (crate::stdlib::uInt, crate::stdlib::uLong) {
    (
        avail_in.wrapping_sub(copied),
        read_buf_total_in_after_copy(total_in, copied),
    )
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ReadBufChecksum {
    Adler32,
    Crc32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ReadBufResult {
    copied: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
}

fn read_buf_checksum(wrap: ::core::ffi::c_int) -> Option<ReadBufChecksum> {
    match wrap {
        1 => Some(ReadBufChecksum::Adler32),
        2 => Some(ReadBufChecksum::Crc32),
        _ => None,
    }
}

fn read_buf_core(
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    avail_in: crate::stdlib::uInt,
    requested: ::core::ffi::c_uint,
    total_in: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    wrap: ::core::ffi::c_int,
) -> ReadBufResult {
    let copied = read_buf_len(avail_in, requested);
    let copied_len = copied as usize;
    output[..copied_len].copy_from_slice(&input[..copied_len]);
    let (avail_in, total_in) = read_buf_input_progress_after_copy(avail_in, total_in, copied);
    let adler = match read_buf_checksum(wrap) {
        Some(ReadBufChecksum::Adler32) => {
            crate::src::adler32::adler32_z(adler, &output[..copied_len])
        }
        Some(ReadBufChecksum::Crc32) => crate::src::crc32::crc32_z(adler, &output[..copied_len]),
        None => adler,
    };
    ReadBufResult {
        copied,
        avail_in,
        total_in,
        adler,
    }
}

// The raw stream and callback-backed output storage are still established by
// the deflate boundary. Keep each crossing explicit while the copy and
// checksum work stays in the safe slice core above.
fn read_buf(
    strm: crate::zlib_h::z_streamp,
    buf: *mut crate::stdlib::Bytef,
    size: ::core::ffi::c_uint,
    wrap: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let stream = unsafe { &mut *strm };
    let len = read_buf_len(stream.avail_in, size);
    if len == 0 {
        return 0;
    }
    let input = unsafe { core::slice::from_raw_parts(stream.next_in, len as usize) };
    let output = unsafe { core::slice::from_raw_parts_mut(buf, len as usize) };
    let result = read_buf_core(
        input,
        output,
        stream.avail_in,
        size,
        stream.total_in,
        stream.adler,
        wrap,
    );
    stream.avail_in = result.avail_in;
    stream.adler = result.adler;
    stream.next_in = stream.next_in.wrapping_add(result.copied as usize);
    stream.total_in = result.total_in;
    result.copied
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
    insert.min(strstart)
}

fn fill_window_state_after_slide(
    match_start: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    insert: crate::stdlib::uInt,
    wsize: crate::stdlib::uInt,
) -> (
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    ::core::ffi::c_long,
    crate::stdlib::uInt,
) {
    let strstart = strstart.wrapping_sub(wsize);
    (
        match_start.wrapping_sub(wsize),
        strstart,
        block_start.wrapping_sub(wsize as ::core::ffi::c_long),
        fill_window_insert_after_slide(insert, strstart),
    )
}

fn fill_window_cursor(
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::zutil_h::ulg {
    (strstart as crate::zutil_h::ulg).wrapping_add(lookahead as crate::zutil_h::ulg)
}

fn fill_window_hash_update(
    hash: crate::stdlib::uInt,
    next_byte: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    (hash << hash_shift ^ next_byte) & hash_mask
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

fn fill_window_high_water_after_zero(
    start: crate::zutil_h::ulg,
    len: crate::zutil_h::ulg,
) -> crate::zutil_h::ulg {
    start.wrapping_add(len)
}

fn fill_window_should_refill(
    lookahead: crate::stdlib::uInt,
    avail_in: crate::stdlib::uInt,
) -> bool {
    lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt && avail_in != 0
}

fn fill_window_has_insertable_match(
    lookahead: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
) -> bool {
    lookahead.wrapping_add(insert) >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
}

fn fill_window_should_slide(strstart: crate::stdlib::uInt, wsize: crate::stdlib::uInt) -> bool {
    strstart
        >= wsize.wrapping_add(
            wsize.wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        )
}

fn fill_window_lookahead_after_read(
    lookahead: crate::stdlib::uInt,
    read: ::core::ffi::c_uint,
) -> crate::stdlib::uInt {
    lookahead.wrapping_add(read)
}

/// Slide the retained window suffix to the front.
///
/// `fill_window()` establishes the callback-owned window view at its state
/// boundary.  Keeping the overlapping move here avoids recreating raw
/// pointers for an operation that is naturally expressed by a slice.
fn fill_window_slide(
    window: &mut [crate::stdlib::Bytef],
    wsize: crate::stdlib::uInt,
    more: ::core::ffi::c_uint,
) -> bool {
    let source_start = wsize as usize;
    let copy_len = wsize.wrapping_sub(more) as usize;
    let Some(source_end) = source_start.checked_add(copy_len) else {
        return false;
    };
    if source_end > window.len() {
        return false;
    }
    window.copy_within(source_start..source_end, 0);
    true
}

/// Clear the uninitialized tail needed by the longest-match lookahead.
fn fill_window_zero(
    window: &mut [crate::stdlib::Bytef],
    start: crate::zutil_h::ulg,
    len: crate::zutil_h::ulg,
) -> bool {
    let Ok(start) = usize::try_from(start) else {
        return false;
    };
    let Ok(len) = usize::try_from(len) else {
        return false;
    };
    let Some(end) = start.checked_add(len) else {
        return false;
    };
    let Some(range) = window.get_mut(start..end) else {
        return false;
    };
    range.fill(0);
    true
}

/// Rebuild the hash entries for bytes retained after a window refill.
///
/// The state values use zlib's wrapping integer arithmetic, but every buffer
/// access is checked.  A malformed state therefore stops the refill instead
/// of deriving an out-of-bounds pointer from its scalar fields.
fn fill_window_reinsert(
    state: &mut crate::src::deflate::deflate_state,
    window: &[crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
) -> bool {
    if !fill_window_has_insertable_match(state.lookahead, state.insert) {
        return true;
    }

    let mut str = state.strstart.wrapping_sub(state.insert);
    let Some(&first) = window.get(str as usize) else {
        return false;
    };
    let Some(&second) = window.get(str.wrapping_add(1) as usize) else {
        return false;
    };
    state.ins_h = first as crate::stdlib::uInt;
    state.ins_h = fill_window_hash_update(
        state.ins_h,
        second as crate::stdlib::uInt,
        state.hash_shift,
        state.hash_mask,
    );

    while state.insert != 0 {
        let Some(&next) = window.get(
            str.wrapping_add(3 as crate::stdlib::uInt)
                .wrapping_sub(1 as crate::stdlib::uInt) as usize,
        ) else {
            return false;
        };
        state.ins_h = fill_window_hash_update(
            state.ins_h,
            next as crate::stdlib::uInt,
            state.hash_shift,
            state.hash_mask,
        );

        let hash_index = state.ins_h as usize;
        let previous = match head.get(hash_index) {
            Some(entry) => *entry,
            None => return false,
        };
        let prev_index = (str & state.w_mask) as usize;
        let Some(prev_entry) = prev.get_mut(prev_index) else {
            return false;
        };
        *prev_entry = previous;
        let Some(head_entry) = head.get_mut(hash_index) else {
            return false;
        };
        *head_entry = str as crate::src::deflate::Pos as crate::src::deflate::Posf;
        str = str.wrapping_add(1);
        state.insert = state.insert.wrapping_sub(1);
        if !fill_window_has_insertable_match(state.lookahead, state.insert) {
            break;
        }
    }
    true
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FillWindowInputProgress {
    avail_in: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    consumed: usize,
}

/// Refill the deflate window through established input and callback-storage
/// views.  The FFI-facing caller is responsible only for constructing those
/// short-lived views and committing the input pointer after this safe core
/// has updated the scalar stream progress.
fn fill_window_core(
    state: &mut crate::src::deflate::deflate_state,
    input: &[crate::stdlib::Bytef],
    avail_in: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
) -> FillWindowInputProgress {
    let mut progress = FillWindowInputProgress {
        avail_in,
        total_in,
        adler,
        consumed: 0,
    };
    let Ok(expected_input_len) = usize::try_from(avail_in) else {
        return progress;
    };
    if input.len() < expected_input_len {
        return progress;
    }
    let mut more: ::core::ffi::c_uint;
    let wsize = state.w_size;
    loop {
        more = fill_window_available_space(
            state.window_size,
            state.lookahead,
            state.strstart,
            wsize,
            ::core::mem::size_of::<::core::ffi::c_int>() <= 2,
        );
        if fill_window_should_slide(state.strstart, wsize) {
            if !fill_window_slide(window, wsize, more) {
                return progress;
            }
            (
                state.match_start,
                state.strstart,
                state.block_start,
                state.insert,
            ) = fill_window_state_after_slide(
                state.match_start,
                state.strstart,
                state.block_start,
                state.insert,
                wsize,
            );
            slide_hash_core(head, prev, wsize);
            state.slid = 1;
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if progress.avail_in == 0 {
            break;
        }
        let Ok(cursor) = usize::try_from(fill_window_cursor(state.strstart, state.lookahead))
        else {
            return progress;
        };
        let Some(output) = window.get_mut(cursor..) else {
            return progress;
        };
        if output.len() < more as usize {
            return progress;
        }
        let Some(remaining_input) = input.get(progress.consumed..) else {
            return progress;
        };
        let result = read_buf_core(
            remaining_input,
            output,
            progress.avail_in,
            more,
            progress.total_in,
            progress.adler,
            state.wrap,
        );
        progress.avail_in = result.avail_in;
        progress.total_in = result.total_in;
        progress.adler = result.adler;
        let Some(consumed) = progress.consumed.checked_add(result.copied as usize) else {
            return progress;
        };
        progress.consumed = consumed;
        state.lookahead = fill_window_lookahead_after_read(state.lookahead, result.copied);
        if !fill_window_reinsert(state, window, head, prev) {
            return progress;
        }
        if !fill_window_should_refill(state.lookahead, progress.avail_in) {
            break;
        }
    }
    if let Some((start, len)) = fill_window_zero_range(
        state.high_water,
        state.window_size,
        state.strstart,
        state.lookahead,
    ) {
        if !fill_window_zero(window, start, len) {
            return progress;
        }
        state.high_water = fill_window_high_water_after_zero(start, len);
    }
    progress
}

unsafe fn fill_window(mut s: *mut crate::src::deflate::deflate_state) {
    let state = &mut *s;
    let stream = &mut *state.strm;
    if stream.avail_in != 0 && stream.next_in.is_null() {
        return;
    }
    // The callback allocation has exactly these lengths.  This boundary
    // creates short-lived views; the refill algorithm itself is slice-based.
    let window =
        &mut *::core::ptr::slice_from_raw_parts_mut(state.window, state.window_size as usize);
    let head = &mut *::core::ptr::slice_from_raw_parts_mut(state.head, state.hash_size as usize);
    let prev = &mut *::core::ptr::slice_from_raw_parts_mut(state.prev, state.w_size as usize);
    let input = if stream.avail_in == 0 {
        &[]
    } else {
        core::slice::from_raw_parts(stream.next_in, stream.avail_in as usize)
    };
    let progress = fill_window_core(
        state,
        input,
        stream.avail_in,
        stream.total_in,
        stream.adler,
        window,
        head,
        prev,
    );
    stream.avail_in = progress.avail_in;
    stream.total_in = progress.total_in;
    stream.adler = progress.adler;
    stream.next_in = stream.next_in.wrapping_add(progress.consumed);
}
#[export_name = "deflateInit_"]

pub unsafe extern "C" fn deflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateInit2_(
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
    if version.is_null() || !deflate_version_matches(*version, stream_size) {
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
    core::ptr::write(s, internal_state::newly_allocated());
    (*strm).state = s as *mut ::core::ffi::c_void;
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
    let lit_bufsize =
        ((1 as ::core::ffi::c_int) << memLevel + 6 as ::core::ffi::c_int) as crate::stdlib::uInt;
    (*s).lit_bufsize = lit_bufsize;
    // The pending bytes and symbol triplets share exactly one callback
    // allocation.  Derive both its request and its safe view geometry from
    // one checked plan before crossing the allocator boundary.
    // `memLevel` was constrained to 1..=MAX_MEM_LEVEL above, so this fixed
    // C-width request is representable.  Keep the checked-plan derivation as
    // the single allocation/layout source of truth.
    let pending_plan =
        pending_storage_allocation_plan(lit_bufsize).expect("validated deflate pending allocation");
    let pending_layout = pending_plan.layout();
    let pending_buf = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        pending_plan.items,
        pending_plan.item_size,
    ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef;
    (*s).pending_buf = ::core::ptr::NonNull::new(pending_buf);
    (*s).pending_buf_size = pending_layout.total_len as crate::zutil_h::ulg;
    if (*s).window.is_null()
        || (*s).prev.is_null()
        || (*s).head.is_null()
        || (*s).pending_buf.is_none()
    {
        (*s).status = crate::src::deflate::FINISH_STATE;
        (*strm).msg =
            z_error_message(-4 as ::core::ffi::c_int).as_ptr() as *mut ::core::ffi::c_char;
        deflateEnd_ffi(strm);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*s).sym_buf_offset = pending_layout.symbol_offset;
    (*s).sym_end = pending_layout.symbol_flush_threshold;
    (*s).level = level;
    (*s).strategy = strategy;
    (*s).method = method as crate::stdlib::Byte;
    return deflateReset_ffi(strm);
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

fn deflate_state_is_usable(
    has_zalloc: bool,
    has_zfree: bool,
    state_points_to_stream: bool,
    status: ::core::ffi::c_int,
) -> bool {
    has_zalloc && has_zfree && state_points_to_stream && deflate_state_status_valid(status)
}

fn deflate_state_metadata_is_usable(
    stream: &crate::zlib_h::z_stream,
    state: &crate::src::deflate::deflate_state,
    state_points_to_stream: bool,
) -> bool {
    deflate_state_is_usable(
        stream.zalloc.is_some(),
        stream.zfree.is_some(),
        state_points_to_stream,
        state.status,
    )
}

fn deflate_state_check_references(
    stream: &crate::zlib_h::z_stream,
    state: &crate::src::deflate::deflate_state,
    state_points_to_stream: bool,
) -> ::core::ffi::c_int {
    if deflate_state_metadata_is_usable(stream, state, state_points_to_stream) {
        0
    } else {
        1
    }
}

// Keep raw stream/state conversion at exported entry points.  This expands in
// those wrappers, rather than leaving a private raw-pointer implementation
// helper in the deflate core.
macro_rules! deflate_state_is_valid_at_ffi_boundary {
    ($strm:expr) => {{
        let strm = $strm;
        if strm.is_null()
            || strm.align_offset(::core::mem::align_of::<crate::zlib_h::z_stream>()) != 0
        {
            false
        } else {
            let stream = unsafe { &*strm };
            let state = stream.state as *mut crate::src::deflate::deflate_state;
            if state.is_null()
                || state.align_offset(::core::mem::align_of::<crate::src::deflate::deflate_state>())
                    != 0
            {
                false
            } else {
                let state = unsafe { &*state };
                deflate_state_check_references(stream, state, state.strm == strm) == 0
            }
        }
    }};
}

fn deflate_reset_status_and_adler(
    wrap: ::core::ffi::c_int,
) -> (::core::ffi::c_int, crate::stdlib::uLong) {
    if wrap == 2 as ::core::ffi::c_int {
        (crate::src::deflate::GZIP_STATE, 0 as crate::stdlib::uLong)
    } else {
        (crate::src::deflate::INIT_STATE, 1 as crate::stdlib::uLong)
    }
}

fn deflate_set_dictionary_allowed(
    wrap: ::core::ffi::c_int,
    status: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
) -> bool {
    wrap != 2 as ::core::ffi::c_int
        && (wrap != 1 as ::core::ffi::c_int || status == crate::src::deflate::INIT_STATE)
        && lookahead == 0
}

fn dictionary_tail_offset(
    dict_length: crate::stdlib::uInt,
    window_size: crate::stdlib::uInt,
) -> usize {
    dict_length.wrapping_sub(window_size) as usize
}

fn deflate_dictionary_state_after_load(
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> (
    crate::stdlib::uInt,
    ::core::ffi::c_long,
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    ::core::ffi::c_int,
) {
    let strstart = strstart.wrapping_add(lookahead);
    let previous_match_length =
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;

    (
        strstart,
        strstart as ::core::ffi::c_long,
        lookahead,
        0,
        previous_match_length,
        previous_match_length,
        0,
    )
}

/// Insert the current dictionary lookahead into the deflate hash chains.
///
/// This is the slice-based counterpart of the translated `INSERT_STRING`
/// loop.  It deliberately retains its wrapping cursor arithmetic: dictionary
/// preparation relies on the same hash order as normal match insertion.
fn deflate_dictionary_insert_hashes(
    window: &[crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    mut strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    ins_h: &mut crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
) -> Option<crate::stdlib::uInt> {
    let min_match_minus_one =
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
    let mut remaining = lookahead.checked_sub(min_match_minus_one)?;

    while remaining != 0 {
        let byte_index = usize::try_from(strstart.wrapping_add(2)).ok()?;
        let next_byte = *window.get(byte_index)? as crate::stdlib::uInt;
        *ins_h = (*ins_h << hash_shift ^ next_byte) & hash_mask;

        let hash_index = usize::try_from(*ins_h).ok()?;
        let previous = *head.get(hash_index)?;
        let prev_index = usize::try_from(strstart & w_mask).ok()?;
        *prev.get_mut(prev_index)? = previous;
        *head.get_mut(hash_index)? = strstart as crate::src::deflate::Posf;

        strstart = strstart.wrapping_add(1);
        remaining = remaining.wrapping_sub(1);
    }

    Some(strstart)
}

#[export_name = "deflateSetDictionary"]
pub unsafe extern "C" fn deflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut s: *mut crate::src::deflate::deflate_state =
        ::core::ptr::null_mut::<crate::src::deflate::deflate_state>();
    let mut wrap: ::core::ffi::c_int = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !deflate_state_is_valid_at_ffi_boundary!(strm) || dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    wrap = (*s).wrap;
    if !deflate_set_dictionary_allowed(wrap, (*s).status, (*s).lookahead) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Ok(window_len) = usize::try_from((*s).window_size) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Ok(head_len) = usize::try_from((*s).hash_size) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Ok(prev_len) = usize::try_from((*s).w_size) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if (*s).window.is_null()
        || (*s).head.is_null()
        || (*s).prev.is_null()
        || (*s)
            .window
            .align_offset(::core::mem::align_of::<crate::stdlib::Bytef>())
            != 0
        || (*s)
            .head
            .align_offset(::core::mem::align_of::<crate::src::deflate::Posf>())
            != 0
        || (*s)
            .prev
            .align_offset(::core::mem::align_of::<crate::src::deflate::Posf>())
            != 0
        || window_len > isize::MAX as usize / ::core::mem::size_of::<crate::stdlib::Bytef>()
        || head_len > isize::MAX as usize / ::core::mem::size_of::<crate::src::deflate::Posf>()
        || prev_len > isize::MAX as usize / ::core::mem::size_of::<crate::src::deflate::Posf>()
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if wrap == 1 as ::core::ffi::c_int {
        (*strm).adler = crate::src::adler32::adler32_ffi((*strm).adler, dictionary, dictLength);
    }
    (*s).wrap = 0 as ::core::ffi::c_int;
    if dictLength >= (*s).w_size {
        if wrap == 0 as ::core::ffi::c_int {
            let head = core::slice::from_raw_parts_mut((*s).head, (*s).hash_size as usize);
            clear_hash(head);
            (*s).slid = 0 as ::core::ffi::c_int;
            (*s).strstart = 0 as crate::stdlib::uInt;
            (*s).block_start = 0 as ::core::ffi::c_long;
            (*s).insert = 0 as crate::stdlib::uInt;
        }
        dictionary = dictionary.wrapping_add(dictionary_tail_offset(dictLength, (*s).w_size));
        dictLength = (*s).w_size;
    }
    avail = (*strm).avail_in as ::core::ffi::c_uint;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    (*strm).avail_in = dictLength;
    (*strm).next_in = dictionary as *mut crate::stdlib::Bytef;
    fill_window(s);
    while (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        // `fill_window()` may update the state before each iteration, so make
        // fresh short-lived views and release them before the next refill.
        let window = core::slice::from_raw_parts((*s).window, window_len);
        let head = core::slice::from_raw_parts_mut((*s).head, head_len);
        let prev = core::slice::from_raw_parts_mut((*s).prev, prev_len);
        let Some(strstart) = deflate_dictionary_insert_hashes(
            window,
            head,
            prev,
            (*s).strstart,
            (*s).lookahead,
            &mut (*s).ins_h,
            (*s).hash_shift,
            (*s).hash_mask,
            (*s).w_mask,
        ) else {
            (*strm).next_in = next as *mut crate::stdlib::Bytef;
            (*strm).avail_in = avail as crate::stdlib::uInt;
            (*s).wrap = wrap;
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        (*s).strstart = strstart;
        (*s).lookahead =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        fill_window(s);
    }
    (
        (*s).strstart,
        (*s).block_start,
        (*s).insert,
        (*s).lookahead,
        (*s).prev_length,
        (*s).match_length,
        (*s).match_available,
    ) = deflate_dictionary_state_after_load((*s).strstart, (*s).lookahead);
    (*strm).next_in = next as *mut crate::stdlib::Bytef;
    (*strm).avail_in = avail as crate::stdlib::uInt;
    (*s).wrap = wrap;
    return crate::zlib_h::Z_OK;
}
fn deflate_dictionary_len(
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    strstart.wrapping_add(lookahead).min(w_size)
}

fn deflate_get_dictionary_core(
    dictionary_source: &[crate::stdlib::Bytef],
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
) {
    if let Some(dictionary) = dictionary {
        dictionary.copy_from_slice(dictionary_source);
    }
}
#[export_name = "deflateGetDictionary"]

pub unsafe extern "C" fn deflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let state = &*((*strm).state as *mut crate::src::deflate::deflate_state);
    let len = deflate_dictionary_len(state.strstart, state.lookahead, state.w_size);
    if !dictionary.is_null() && len != 0 {
        let start = state
            .strstart
            .wrapping_add(state.lookahead)
            .wrapping_sub(len) as usize;
        let dictionary_source = core::slice::from_raw_parts(state.window.add(start), len as usize);
        let dictionary = core::slice::from_raw_parts_mut(dictionary, len as usize);
        deflate_get_dictionary_core(dictionary_source, Some(dictionary));
    }
    if !dictLength.is_null() {
        *dictLength = len;
    }
    crate::zlib_h::Z_OK
}

fn deflate_reset_keep_state(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
) {
    strm.total_out = 0;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = crate::zlib_h::Z_UNKNOWN;
    state.pending = 0;
    state.pending_out_offset = 0;
    if state.wrap < 0 {
        state.wrap = -state.wrap;
    }
    (state.status, strm.adler) = deflate_reset_status_and_adler(state.wrap);
    state.last_flush = -2;
    crate::src::trees::tr_init(state);
}

#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let stream = &mut *strm;
    let state = &mut *(stream.state as *mut crate::src::deflate::deflate_state);
    deflate_reset_keep_state(stream, state);
    crate::zlib_h::Z_OK
}

fn lm_match_parameters(
    level: ::core::ffi::c_int,
) -> (
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    ::core::ffi::c_int,
    crate::stdlib::uInt,
) {
    let config = configuration_table[level as usize];
    (
        config.max_lazy as crate::stdlib::uInt,
        config.good_length as crate::stdlib::uInt,
        config.nice_length as ::core::ffi::c_int,
        config.max_chain as crate::stdlib::uInt,
    )
}

fn lm_initial_state(w_size: crate::stdlib::uInt) -> (crate::zutil_h::ulg, crate::stdlib::uInt) {
    (
        (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
            .wrapping_mul(w_size as crate::zutil_h::ulg),
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
    )
}

fn lm_head_clear_len(hash_size: crate::stdlib::uInt) -> crate::__stddef_size_t_h::size_t {
    (hash_size as crate::__stddef_size_t_h::size_t)
        .wrapping_mul(
            ::core::mem::size_of::<crate::src::deflate::Posf>() as crate::__stddef_size_t_h::size_t
        )
}

#[derive(Debug, PartialEq, Eq)]
struct LmHeadResetPlan {
    clear_len: crate::__stddef_size_t_h::size_t,
}

fn lm_head_reset_plan(hash_size: crate::stdlib::uInt) -> LmHeadResetPlan {
    LmHeadResetPlan {
        clear_len: lm_head_clear_len(hash_size),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LmResetPlan {
    window_size: crate::zutil_h::ulg,
    prev_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    max_chain_length: crate::stdlib::uInt,
}

fn lm_reset_plan(w_size: crate::stdlib::uInt, level: ::core::ffi::c_int) -> LmResetPlan {
    let (window_size, prev_length) = lm_initial_state(w_size);
    let (max_lazy_match, good_match, nice_match, max_chain_length) = lm_match_parameters(level);

    LmResetPlan {
        window_size,
        prev_length,
        max_lazy_match,
        good_match,
        nice_match,
        max_chain_length,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LmInitPlan {
    head_reset: LmHeadResetPlan,
    reset: LmResetPlan,
}

fn lm_init_plan(
    hash_size: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
) -> LmInitPlan {
    LmInitPlan {
        head_reset: lm_head_reset_plan(hash_size),
        reset: lm_reset_plan(w_size, level),
    }
}

fn lm_apply_reset(state: &mut crate::src::deflate::deflate_state, plan: LmResetPlan) {
    state.window_size = plan.window_size;
    state.slid = 0 as ::core::ffi::c_int;
    state.max_lazy_match = plan.max_lazy_match;
    state.good_match = plan.good_match;
    state.nice_match = plan.nice_match;
    state.max_chain_length = plan.max_chain_length;
    state.strstart = 0 as crate::stdlib::uInt;
    state.block_start = 0 as ::core::ffi::c_long;
    state.lookahead = 0 as crate::stdlib::uInt;
    state.insert = 0 as crate::stdlib::uInt;
    state.prev_length = plan.prev_length;
    state.match_length = state.prev_length;
    state.match_available = 0 as ::core::ffi::c_int;
    state.ins_h = 0 as crate::stdlib::uInt;
}

fn lm_init(state: &mut crate::src::deflate::deflate_state, head: &mut [crate::src::deflate::Posf]) {
    let plan = lm_init_plan(state.hash_size, state.w_size, state.level);
    head[..state.hash_size as usize].fill(0);
    lm_apply_reset(state, plan.reset);
}

/// Forget every hash-chain entry while retaining the translated C macro's
/// explicit final-slot initialization.  Callers establish the callback-owned
/// hash storage at an exported boundary; the reset itself is ordinary safe
/// slice work.
fn clear_hash(head: &mut [crate::src::deflate::Posf]) {
    let Some((last, entries)) = head.split_last_mut() else {
        return;
    };
    *last = NIL as crate::src::deflate::Posf;
    entries.fill(0);
}
#[export_name = "deflateReset"]

pub unsafe extern "C" fn deflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let stream = &mut *strm;
    let state = &mut *(stream.state as *mut crate::src::deflate::deflate_state);
    if state.head.is_null()
        || state
            .head
            .align_offset(::core::mem::align_of::<crate::src::deflate::Posf>())
            != 0
        || state.hash_size as usize
            > isize::MAX as usize / ::core::mem::size_of::<crate::src::deflate::Posf>()
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let head = core::slice::from_raw_parts_mut(state.head, state.hash_size as usize);
    deflate_reset_keep_state(stream, state);
    lm_init(state, head);
    crate::zlib_h::Z_OK
}
fn deflate_set_header_allowed(wrap: ::core::ffi::c_int) -> bool {
    wrap == 2 as ::core::ffi::c_int
}

fn deflate_set_header_core(
    state: &crate::src::deflate::deflate_state,
) -> Result<(), ::core::ffi::c_int> {
    if deflate_set_header_allowed(state.wrap) {
        Ok(())
    } else {
        Err(crate::zlib_h::Z_STREAM_ERROR)
    }
}

#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    if let Err(status) = deflate_set_header_core(state) {
        return status;
    }

    state.gzhead = head;
    crate::zlib_h::Z_OK
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
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
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
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
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

fn deflate_prime_has_pending_space(symbol_offset: usize, pending_out_offset: usize) -> bool {
    pending_out_offset
        .checked_add(((crate::src::deflate::Buf_size + 7) >> 3) as usize)
        .is_some_and(|required| symbol_offset >= required)
}

fn deflate_prime_flush_bits(
    storage: &mut PendingStorageView<'_>,
    pending: &mut crate::zutil_h::ulg,
    bi_buf: &mut crate::zutil_h::ush,
    bi_valid: &mut ::core::ffi::c_int,
) -> bool {
    let bytes = [
        (*bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as crate::zutil_h::uch,
        (*bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch,
    ];
    let (count, next_bi_buf, next_bi_valid) = if *bi_valid == crate::src::deflate::Buf_size {
        (2, 0, 0)
    } else if *bi_valid >= 8 {
        (
            1,
            (*bi_buf as ::core::ffi::c_int >> 8) as crate::zutil_h::ush,
            *bi_valid - 8,
        )
    } else {
        (0, *bi_buf, *bi_valid)
    };

    if !storage.append_pending(pending, &bytes[..count]) {
        return false;
    }

    *bi_buf = next_bi_buf;
    *bi_valid = next_bi_valid;
    true
}

fn deflate_prime_insert_bits(
    storage: &mut PendingStorageView<'_>,
    pending: &mut crate::zutil_h::ulg,
    bi_buf: &mut crate::zutil_h::ush,
    bi_valid: &mut ::core::ffi::c_int,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> bool {
    loop {
        let mut put = crate::src::deflate::Buf_size - *bi_valid;
        if put > bits {
            put = bits;
        }
        let inserted = ((value & ((1 as ::core::ffi::c_int) << put) - 1 as ::core::ffi::c_int)
            << *bi_valid) as crate::zutil_h::ush as ::core::ffi::c_int;
        *bi_buf = (*bi_buf as ::core::ffi::c_int | inserted) as crate::zutil_h::ush;
        *bi_valid += put;
        if !deflate_prime_flush_bits(storage, pending, bi_buf, bi_valid) {
            return false;
        }
        value >>= put;
        bits -= put;
        if !(bits != 0) {
            break;
        }
    }
    true
}

fn deflatePrime(
    state: &mut crate::src::deflate::deflate_state,
    storage: &mut PendingStorageView<'_>,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let layout = pending_storage_layout(state.lit_bufsize);
    if !deflate_prime_bits_valid(bits)
        || !deflate_prime_has_pending_space(layout.symbol_offset, state.pending_out_offset)
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }

    if deflate_prime_insert_bits(
        storage,
        &mut state.pending,
        &mut state.bi_buf,
        &mut state.bi_valid,
        bits,
        value,
    ) {
        crate::zlib_h::Z_OK
    } else {
        crate::zlib_h::Z_BUF_ERROR
    }
}
#[export_name = "deflatePrime"]

pub unsafe extern "C" fn deflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    let Some(layout) = pending_storage_layout_from_metadata(
        state.lit_bufsize,
        state.pending_buf_size,
        state.sym_buf_offset,
        state.sym_end,
    ) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(pending_buf) = state.pending_buf else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let pending_buffer =
        core::slice::from_raw_parts_mut(pending_buf.as_ptr(), state.pending_buf_size as usize);
    with_pending_storage(pending_buffer, layout, |storage| {
        deflatePrime(state, storage, bits, value)
    })
    .unwrap_or(crate::zlib_h::Z_BUF_ERROR)
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DeflateParamsHashAction {
    None,
    Rebase,
    Clear,
}

fn deflate_compression_function_for_level(
    level: ::core::ffi::c_int,
) -> Option<DeflateCompressionFunction> {
    match level {
        0 => Some(DeflateCompressionFunction::Stored),
        1..=3 => Some(DeflateCompressionFunction::Fast),
        4..=9 => Some(DeflateCompressionFunction::Slow),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct DeflateParamsPlan {
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    flush_before_apply: bool,
    hash_action: DeflateParamsHashAction,
}

fn deflate_params_plan(
    state: &crate::src::deflate::deflate_state,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Option<DeflateParamsPlan> {
    let (level, strategy) = normalize_deflate_params(level, strategy)?;
    let current_function = deflate_compression_function_for_level(state.level)?;
    let requested_function = deflate_compression_function_for_level(level)?;
    let flush_before_apply = (strategy != state.strategy || current_function != requested_function)
        && state.last_flush != -2;
    let hash_action = if state.level != level && state.level == 0 && state.matches != 0 {
        if state.matches == 1 {
            DeflateParamsHashAction::Rebase
        } else {
            DeflateParamsHashAction::Clear
        }
    } else {
        DeflateParamsHashAction::None
    };

    Some(DeflateParamsPlan {
        level,
        strategy,
        flush_before_apply,
        hash_action,
    })
}

fn deflate_params_flush_is_complete(
    stream: &crate::zlib_h::z_stream,
    state: &crate::src::deflate::deflate_state,
) -> bool {
    stream.avail_in == 0
        && state.strstart as ::core::ffi::c_long - state.block_start
            + state.lookahead as ::core::ffi::c_long
            == 0
}

fn deflate_params_apply_hash_action(
    action: DeflateParamsHashAction,
    head: &mut [crate::src::deflate::Posf],
    prev: Option<&mut [crate::src::deflate::Posf]>,
    wsize: crate::stdlib::uInt,
) {
    match action {
        DeflateParamsHashAction::None => {}
        DeflateParamsHashAction::Rebase => {
            if let Some(prev) = prev {
                slide_hash_core(head, prev, wsize);
            }
        }
        DeflateParamsHashAction::Clear => head.fill(NIL as crate::src::deflate::Posf),
    }
}

fn deflate_params_apply(state: &mut crate::src::deflate::deflate_state, plan: DeflateParamsPlan) {
    if state.level != plan.level {
        if state.level == 0 && state.matches != 0 {
            state.slid = match plan.hash_action {
                DeflateParamsHashAction::Rebase => 1,
                DeflateParamsHashAction::Clear | DeflateParamsHashAction::None => 0,
            };
            state.matches = 0;
        }
        state.level = plan.level;
        state.max_lazy_match =
            configuration_table[plan.level as usize].max_lazy as crate::stdlib::uInt;
        state.good_match =
            configuration_table[plan.level as usize].good_length as crate::stdlib::uInt;
        state.nice_match =
            configuration_table[plan.level as usize].nice_length as ::core::ffi::c_int;
        state.max_chain_length =
            configuration_table[plan.level as usize].max_chain as crate::stdlib::uInt;
    }
    state.strategy = plan.strategy;
}

#[export_name = "deflateParams"]

pub unsafe extern "C" fn deflateParams_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let plan = {
        let stream = &mut *strm;
        let state = &mut *(stream.state as *mut crate::src::deflate::deflate_state);
        match deflate_params_plan(state, level, strategy) {
            Some(plan) => plan,
            None => return crate::zlib_h::Z_STREAM_ERROR,
        }
    };

    if plan.flush_before_apply {
        let err = deflate_ffi(strm, crate::zlib_h::Z_BLOCK);
        if err == crate::zlib_h::Z_STREAM_ERROR {
            return err;
        }
        let stream = &mut *strm;
        let state = &mut *(stream.state as *mut crate::src::deflate::deflate_state);
        if !deflate_params_flush_is_complete(stream, state) {
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }

    let stream = &mut *strm;
    let state = &mut *(stream.state as *mut crate::src::deflate::deflate_state);
    match plan.hash_action {
        DeflateParamsHashAction::None => {}
        DeflateParamsHashAction::Rebase => {
            let head =
                &mut *::core::ptr::slice_from_raw_parts_mut(state.head, state.hash_size as usize);
            let prev =
                &mut *::core::ptr::slice_from_raw_parts_mut(state.prev, state.w_size as usize);
            deflate_params_apply_hash_action(plan.hash_action, head, Some(prev), state.w_size);
        }
        DeflateParamsHashAction::Clear => {
            let head =
                &mut *::core::ptr::slice_from_raw_parts_mut(state.head, state.hash_size as usize);
            deflate_params_apply_hash_action(plan.hash_action, head, None, state.w_size);
        }
    }
    deflate_params_apply(state, plan);
    crate::zlib_h::Z_OK
}
fn deflate_tune_core(
    good_match: &mut crate::stdlib::uInt,
    max_lazy_match: &mut crate::stdlib::uInt,
    nice_match: &mut ::core::ffi::c_int,
    max_chain_length: &mut crate::stdlib::uInt,
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *good_match = good_length as crate::stdlib::uInt;
    *max_lazy_match = max_lazy as crate::stdlib::uInt;
    *nice_match = nice_length;
    *max_chain_length = max_chain as crate::stdlib::uInt;
    crate::zlib_h::Z_OK
}
#[export_name = "deflateTune"]

pub unsafe extern "C" fn deflateTune_ffi(
    strm: crate::zlib_h::z_streamp,
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::deflate::deflate_state);
    deflate_tune_core(
        &mut state.good_match,
        &mut state.max_lazy_match,
        &mut state.nice_match,
        &mut state.max_chain_length,
        good_length,
        max_lazy,
        nice_length,
        max_chain,
    )
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct DeflateBoundGzipHeader {
    has_extra: bool,
    extra_len: crate::stdlib::uInt,
    name_len: Option<crate::stdlib::z_size_t>,
    comment_len: Option<crate::stdlib::z_size_t>,
    has_header_crc: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct DeflateBoundState {
    wrap: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    w_bits: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
    gzip_header: Option<DeflateBoundGzipHeader>,
}

fn deflate_bound_wrapped_add(
    bound: crate::stdlib::z_size_t,
    extra: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let bound_with_extra = bound.wrapping_add(extra);
    if bound_with_extra < bound {
        -1_i32 as crate::stdlib::z_size_t
    } else {
        bound_with_extra
    }
}

fn deflate_bound_gzip_wrapper_len(
    header: Option<DeflateBoundGzipHeader>,
) -> crate::stdlib::z_size_t {
    let Some(header) = header else {
        return 18;
    };

    let mut length = 18_usize;
    if header.has_extra {
        length = length
            .wrapping_add(2)
            .wrapping_add(header.extra_len as usize);
    }
    if let Some(name_len) = header.name_len {
        length = length.wrapping_add(name_len).wrapping_add(1);
    }
    if let Some(comment_len) = header.comment_len {
        length = length.wrapping_add(comment_len).wrapping_add(1);
    }
    if header.has_header_crc {
        length = length.wrapping_add(2);
    }
    length
}

fn deflate_bound_z_core(
    source_len: crate::stdlib::z_size_t,
    state: Option<DeflateBoundState>,
) -> crate::stdlib::z_size_t {
    let (fixed_len, stored_len) = deflate_bound_lengths(source_len);
    let Some(state) = state else {
        return deflate_bound_wrapped_add(fixed_len.max(stored_len), 18);
    };

    let wrap_len = match state.wrap.wrapping_abs() {
        0 => 0,
        1 => 6 + if state.strstart != 0 { 4 } else { 0 },
        2 => deflate_bound_gzip_wrapper_len(state.gzip_header),
        _ => 18,
    };
    if state.w_bits != 15 || state.hash_bits != 15 {
        let bound = if state.w_bits <= state.hash_bits && state.level != 0 {
            fixed_len
        } else {
            stored_len
        };
        return deflate_bound_wrapped_add(bound, wrap_len);
    }

    let bound = source_len
        .wrapping_add(source_len >> 12)
        .wrapping_add(source_len >> 14)
        .wrapping_add(source_len >> 25)
        .wrapping_add(13)
        .wrapping_sub(6)
        .wrapping_add(wrap_len);
    if bound < source_len {
        -1_i32 as crate::stdlib::z_size_t
    } else {
        bound
    }
}

macro_rules! deflate_bound_state_at_ffi_boundary {
    ($strm:expr) => {{
        let strm = $strm;
        if !deflate_state_is_valid_at_ffi_boundary!(strm) {
            None
        } else {
            let state = &*((*strm).state as *mut crate::src::deflate::deflate_state);
            let gzip_header = if state.gzhead.is_null() {
                None
            } else {
                let header = &*state.gzhead;
                Some(DeflateBoundGzipHeader {
                    has_extra: !header.extra.is_null(),
                    extra_len: header.extra_len,
                    name_len: if header.name.is_null() {
                        None
                    } else {
                        Some(
                            crate::stdlib::strlen(header.name as *const ::core::ffi::c_char)
                                as crate::stdlib::z_size_t,
                        )
                    },
                    comment_len: if header.comment.is_null() {
                        None
                    } else {
                        Some(
                            crate::stdlib::strlen(header.comment as *const ::core::ffi::c_char)
                                as crate::stdlib::z_size_t,
                        )
                    },
                    has_header_crc: header.hcrc != 0,
                })
            };
            Some(DeflateBoundState {
                wrap: state.wrap,
                strstart: state.strstart,
                w_bits: state.w_bits,
                hash_bits: state.hash_bits,
                level: state.level,
                gzip_header,
            })
        }
    }};
}

#[export_name = "deflateBound_z"]

pub unsafe extern "C" fn deflateBound_z_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    deflate_bound_z_core(sourceLen, deflate_bound_state_at_ffi_boundary!(strm))
}
#[export_name = "deflateBound"]
pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let mut bound: crate::stdlib::z_size_t =
        deflateBound_z_ffi(strm, sourceLen as crate::stdlib::z_size_t);
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

fn put_short_msb_core(
    storage: &mut PendingStorageView<'_>,
    pending: &mut crate::zutil_h::ulg,
    value: crate::stdlib::uInt,
) -> bool {
    storage.append_pending(pending, &short_msb_bytes(value))
}

fn pending_output_len(
    pending: crate::zutil_h::ulg,
    avail_out: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    clamped_copy_len(pending, avail_out as crate::zutil_h::ulg)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct PendingDrainState {
    pending: crate::zutil_h::ulg,
    pending_out_offset: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FlushPendingResult {
    copied: ::core::ffi::c_uint,
    next: PendingDrainState,
    avail_out: crate::stdlib::uInt,
    total_out: crate::stdlib::uLong,
    reset_pending_out: bool,
}

fn flush_pending_core(
    drain: PendingDrainState,
    avail_out: crate::stdlib::uInt,
    total_out: crate::stdlib::uLong,
) -> Option<FlushPendingResult> {
    let copied = pending_output_len(drain.pending, avail_out);
    if copied == 0 {
        return None;
    }
    let remaining = drain.pending.wrapping_sub(copied as crate::zutil_h::ulg);
    let reset_pending_out = remaining == 0;
    Some(FlushPendingResult {
        copied,
        next: PendingDrainState {
            pending: remaining,
            pending_out_offset: if reset_pending_out {
                0
            } else {
                drain.pending_out_offset.wrapping_add(copied as usize)
            },
        },
        avail_out: avail_out.wrapping_sub(copied),
        total_out: total_out.wrapping_add(copied as crate::stdlib::uLong),
        reset_pending_out,
    })
}

/// Drain one pending-output segment through checked slice ranges.
///
/// The callback-backed allocation is still established at the FFI boundary,
/// but the temporal pending-output view itself has no reason to use pointer
/// arithmetic or `memcpy`.  Keeping the copy here also makes an invalid
/// pending cursor a no-op instead of deriving an out-of-bounds raw pointer.
fn drain_pending(
    pending_storage: &[crate::stdlib::Bytef],
    drain: PendingDrainState,
    output: &mut [crate::stdlib::Bytef],
    avail_out: crate::stdlib::uInt,
    total_out: crate::stdlib::uLong,
) -> Option<FlushPendingResult> {
    let result = flush_pending_core(drain, avail_out, total_out)?;
    let copied = result.copied as usize;
    let end = drain.pending_out_offset.checked_add(copied)?;
    let source = pending_storage.get(drain.pending_out_offset..end)?;
    let destination = output.get_mut(..copied)?;
    destination.copy_from_slice(source);
    Some(result)
}

unsafe fn flush_pending(mut strm: crate::zlib_h::z_streamp) {
    let stream = &mut *strm;
    let state = &mut *(stream.state as *mut crate::src::deflate::deflate_state);
    let Some(layout) = pending_storage_layout_for_state(state) else {
        return;
    };
    let Some(pending_buf) = state.pending_buf else {
        return;
    };
    let pending_storage = core::slice::from_raw_parts_mut(pending_buf.as_ptr(), layout.total_len);
    assert!(with_pending_storage(pending_storage, layout, |storage| {
        crate::src::trees::tr_flush_bits_core(
            storage,
            &mut state.pending,
            &mut state.bi_buf,
            &mut state.bi_valid,
        )
    })
    .expect("pending storage layout matches its allocation"));
    let drain = PendingDrainState {
        pending: state.pending,
        pending_out_offset: state.pending_out_offset,
    };
    let Some(preview) = flush_pending_core(drain, stream.avail_out, stream.total_out) else {
        return;
    };
    // These raw allocations are established by the exported deflate
    // initializer.  Once their bounded views exist, the drain itself uses
    // checked ranges and a safe slice copy in `drain_pending()`.
    let output = core::slice::from_raw_parts_mut(stream.next_out, preview.copied as usize);
    let Some(result) = drain_pending(
        pending_storage,
        drain,
        output,
        stream.avail_out,
        stream.total_out,
    ) else {
        return;
    };
    stream.next_out = stream.next_out.wrapping_add(result.copied as usize);
    stream.total_out = result.total_out;
    stream.avail_out = result.avail_out;
    state.pending = result.next.pending;
    state.pending_out_offset = result.next.pending_out_offset;
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

/// Append one bounded portion of a gzip extra field to pending storage.
///
/// The extra field has an explicit length in the public gzip header, unlike
/// the NUL-terminated name and comment fields.  Once the exported deflate
/// wrapper has established those two short-lived slices, this copy needs no
/// raw pointer arithmetic or foreign `memcpy`.  Validate both ranges before
/// mutating pending storage so malformed cursor metadata cannot leave a
/// partially copied header behind.
fn gzip_extra_copy_chunk(
    pending_buffer: &mut [crate::stdlib::Bytef],
    pending: crate::zutil_h::ulg,
    extra: &[crate::stdlib::Bytef],
    extra_index: crate::zutil_h::ulg,
    count: crate::zutil_h::ulg,
) -> Option<crate::zutil_h::ulg> {
    let pending_start = usize::try_from(pending).ok()?;
    let extra_start = usize::try_from(extra_index).ok()?;
    let count = usize::try_from(count).ok()?;
    let pending_end = pending_start.checked_add(count)?;
    let extra_end = extra_start.checked_add(count)?;
    let destination = pending_buffer.get_mut(pending_start..pending_end)?;
    let source = extra.get(extra_start..extra_end)?;
    destination.copy_from_slice(source);
    crate::zutil_h::ulg::try_from(pending_end).ok()
}

fn pending_buffer_needs_flush(
    pending: crate::zutil_h::ulg,
    required: crate::zutil_h::ulg,
    pending_buf_size: crate::zutil_h::ulg,
) -> bool {
    pending.wrapping_add(required) > pending_buf_size
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

fn gzip_default_header_bytes(
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> [crate::stdlib::Bytef; 10] {
    [
        31,
        139,
        8,
        0,
        0,
        0,
        0,
        0,
        gzip_default_xfl(level, strategy),
        3,
    ]
}

fn gzip_custom_header_bytes(
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
) -> ([crate::stdlib::Bytef; 9], usize) {
    let flags = (if text != 0 { 1 } else { 0 })
        + (if hcrc != 0 { 2 } else { 0 })
        + (if has_extra { 4 } else { 0 })
        + (if has_name { 8 } else { 0 })
        + (if has_comment { 16 } else { 0 });
    let mut header = [
        flags as crate::stdlib::Bytef,
        time as crate::stdlib::Byte,
        (time >> 8) as crate::stdlib::Byte,
        (time >> 16) as crate::stdlib::Byte,
        (time >> 24) as crate::stdlib::Byte,
        gzip_default_xfl(level, strategy),
        os as crate::stdlib::Bytef,
        0,
        0,
    ];
    let len = if has_extra {
        header[7] = extra_len as crate::stdlib::Bytef;
        header[8] = (extra_len >> 8) as crate::stdlib::Bytef;
        9
    } else {
        7
    };

    (header, len)
}

fn gzip_header_crc_bytes(crc: crate::stdlib::uLong) -> [crate::stdlib::Bytef; 2] {
    [
        crc as crate::stdlib::Byte,
        (crc >> 8) as crate::stdlib::Byte,
    ]
}

fn gzip_trailer_bytes(
    adler: crate::stdlib::uLong,
    total_in: crate::stdlib::uLong,
) -> [crate::stdlib::Bytef; 8] {
    [
        adler as crate::stdlib::Byte,
        (adler >> 8) as crate::stdlib::Byte,
        (adler >> 16) as crate::stdlib::Byte,
        (adler >> 24) as crate::stdlib::Byte,
        total_in as crate::stdlib::Byte,
        (total_in >> 8) as crate::stdlib::Byte,
        (total_in >> 16) as crate::stdlib::Byte,
        (total_in >> 24) as crate::stdlib::Byte,
    ]
}

fn deflate_flush_rank(flush: ::core::ffi::c_int) -> ::core::ffi::c_int {
    flush * 2 - if flush > 4 { 9 } else { 0 }
}

fn deflate_should_return_buf_error(
    avail_in: crate::stdlib::uInt,
    flush: ::core::ffi::c_int,
    old_flush: ::core::ffi::c_int,
) -> bool {
    avail_in == 0
        && deflate_flush_rank(flush) <= deflate_flush_rank(old_flush)
        && flush != crate::zlib_h::Z_FINISH
}

fn deflate_block_state_actions(bstate: block_state) -> (bool, bool) {
    (
        bstate == finish_started || bstate == finish_done,
        bstate == need_more || bstate == finish_started,
    )
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DeflatePreflight {
    StreamError,
    BufError,
    Continue,
}

fn deflate_preflight(
    next_out_is_null: bool,
    avail_in: crate::stdlib::uInt,
    next_in_is_null: bool,
    avail_out: crate::stdlib::uInt,
    status: ::core::ffi::c_int,
    flush: ::core::ffi::c_int,
) -> DeflatePreflight {
    if deflate_request_is_invalid(next_out_is_null, avail_in, next_in_is_null, status, flush) {
        DeflatePreflight::StreamError
    } else if avail_out == 0 {
        DeflatePreflight::BufError
    } else {
        DeflatePreflight::Continue
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
    if !deflate_state_is_valid_at_ffi_boundary!(strm)
        || flush > crate::zlib_h::Z_BLOCK
        || flush < 0 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut crate::src::deflate::deflate_state;
    let Some(pending_layout) = pending_storage_layout_from_metadata(
        (*s).lit_bufsize,
        (*s).pending_buf_size,
        (*s).sym_buf_offset,
        (*s).sym_end,
    ) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if (*s).pending_buf.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let pending_buffer = {
        core::slice::from_raw_parts(
            (*s).pending_buf
                .expect("validated pending storage")
                .as_ptr(),
            pending_layout.total_len,
        )
    };
    match deflate_preflight(
        (*strm).next_out.is_null(),
        (*strm).avail_in,
        (*strm).next_in.is_null(),
        (*strm).avail_out,
        (*s).status,
        flush,
    ) {
        DeflatePreflight::StreamError => {
            (*strm).msg =
                z_error_message(-2 as ::core::ffi::c_int).as_ptr() as *mut ::core::ffi::c_char;
            return -2 as ::core::ffi::c_int;
        }
        DeflatePreflight::BufError => {
            (*strm).msg =
                z_error_message(-5 as ::core::ffi::c_int).as_ptr() as *mut ::core::ffi::c_char;
            return -5 as ::core::ffi::c_int;
        }
        DeflatePreflight::Continue => {}
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
        (*strm).msg =
            z_error_message(-5 as ::core::ffi::c_int).as_ptr() as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    if (*s).status == crate::src::deflate::FINISH_STATE
        && (*strm).avail_in != 0 as crate::stdlib::uInt
    {
        (*strm).msg =
            z_error_message(-5 as ::core::ffi::c_int).as_ptr() as *mut ::core::ffi::c_char;
        return -5 as ::core::ffi::c_int;
    }
    if (*s).status == crate::src::deflate::INIT_STATE && (*s).wrap == 0 as ::core::ffi::c_int {
        (*s).status = crate::src::deflate::BUSY_STATE;
    }
    let initialized = (*s).status == crate::src::deflate::INIT_STATE;
    if initialized {
        let state = &mut *s;
        let header = zlib_header(
            state.w_bits,
            state.strategy,
            state.level,
            state.strstart != 0,
        );
        let pending_buffer = core::slice::from_raw_parts_mut(
            state
                .pending_buf
                .expect("validated pending storage")
                .as_ptr(),
            state.pending_buf_size as usize,
        );
        let has_dictionary = state.strstart != 0 as crate::stdlib::uInt;
        let adler = (*strm).adler;
        let pending = &mut state.pending;
        assert!(
            with_pending_storage(pending_buffer, pending_layout, |storage| {
                let mut wrote = put_short_msb_core(storage, pending, header);
                if has_dictionary {
                    wrote &= put_short_msb_core(
                        storage,
                        pending,
                        (adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
                    );
                    wrote &= put_short_msb_core(
                        storage,
                        pending,
                        (adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
                    );
                }
                wrote
            })
            .expect("pending storage layout matches its allocation")
        );
        (*strm).adler = 1 as crate::stdlib::uLong;
        state.status = crate::src::deflate::BUSY_STATE;
    }
    if initialized {
        flush_pending(strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if (*s).status == crate::src::deflate::GZIP_STATE {
        (*strm).adler = 0 as crate::stdlib::uLong;
        let state = &mut *s;
        let pending_buffer = core::slice::from_raw_parts_mut(
            state
                .pending_buf
                .expect("validated pending storage")
                .as_ptr(),
            state.pending_buf_size as usize,
        );
        let layout = pending_storage_layout(state.lit_bufsize);
        assert!(with_pending_storage(pending_buffer, layout, |storage| {
            storage.append_pending(&mut state.pending, &[31, 139, 8])
        })
        .expect("pending storage layout matches its allocation"));
        if (*s).gzhead.is_null() {
            let state = &mut *s;
            let pending_buffer = core::slice::from_raw_parts_mut(
                state
                    .pending_buf
                    .expect("validated pending storage")
                    .as_ptr(),
                state.pending_buf_size as usize,
            );
            let layout = pending_storage_layout(state.lit_bufsize);
            let header = gzip_default_header_bytes(state.level, state.strategy);
            assert!(with_pending_storage(pending_buffer, layout, |storage| {
                storage.append_pending(&mut state.pending, &header[3..])
            })
            .expect("pending storage layout matches its allocation"));
            (*s).status = crate::src::deflate::BUSY_STATE;
            flush_pending(strm);
            if (*s).pending != 0 as crate::zutil_h::ulg {
                (*s).last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        } else {
            let gzhead = &*(*s).gzhead;
            let (header, header_len) = gzip_custom_header_bytes(
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
            let pending_buffer = core::slice::from_raw_parts_mut(
                (*s).pending_buf
                    .expect("validated pending storage")
                    .as_ptr(),
                (*s).pending_buf_size as usize,
            );
            let layout = pending_storage_layout((*s).lit_bufsize);
            let mut pending = (*s).pending;
            assert!(with_pending_storage(pending_buffer, layout, |storage| {
                storage.append_pending(&mut pending, &header[..header_len])
            })
            .expect("pending storage layout matches its allocation"));
            (*s).pending = pending;
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
            let extra_len = ((*(*s).gzhead).extra_len & 0xffff as crate::stdlib::uInt) as usize;
            // `gzhead` is a caller-provided ABI record.  Its non-null extra
            // pointer and explicit 16-bit length establish this short-lived
            // FFI-boundary view; the pending copy itself stays in the safe
            // helper below.
            let extra = core::slice::from_raw_parts((*(*s).gzhead).extra, extra_len);
            let mut beg: crate::zutil_h::ulg = (*s).pending;
            let mut left: crate::zutil_h::ulg =
                (((*(*s).gzhead).extra_len & 0xffff as crate::stdlib::uInt) as crate::zutil_h::ulg)
                    .wrapping_sub((*s).gzindex);
            while pending_buffer_needs_flush((*s).pending, left, (*s).pending_buf_size) {
                let copy: crate::zutil_h::ulg = (*s).pending_buf_size.wrapping_sub((*s).pending);
                let pending = core::slice::from_raw_parts_mut(
                    (*s).pending_buf
                        .expect("validated pending storage")
                        .as_ptr(),
                    (*s).pending_buf_size as usize,
                );
                let Some(next_pending) =
                    gzip_extra_copy_chunk(pending, (*s).pending, extra, (*s).gzindex, copy)
                else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                (*s).pending = next_pending;
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
            let pending = core::slice::from_raw_parts_mut(
                (*s).pending_buf
                    .expect("validated pending storage")
                    .as_ptr(),
                (*s).pending_buf_size as usize,
            );
            let Some(next_pending) =
                gzip_extra_copy_chunk(pending, (*s).pending, extra, (*s).gzindex, left)
            else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            (*s).pending = next_pending;
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
                *(*s)
                    .pending_buf
                    .expect("validated pending storage")
                    .as_ptr()
                    .wrapping_add(c2rust_fresh20 as usize) = val as crate::stdlib::Bytef;
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
                *(*s)
                    .pending_buf
                    .expect("validated pending storage")
                    .as_ptr()
                    .wrapping_add(c2rust_fresh22 as usize) = val_0 as crate::stdlib::Bytef;
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
            if pending_buffer_needs_flush(
                (*s).pending,
                2 as crate::zutil_h::ulg,
                (*s).pending_buf_size,
            ) {
                flush_pending(strm);
                if (*s).pending != 0 as crate::zutil_h::ulg {
                    (*s).last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
            let pending_buffer = core::slice::from_raw_parts_mut(
                (*s).pending_buf
                    .expect("validated pending storage")
                    .as_ptr(),
                (*s).pending_buf_size as usize,
            );
            let layout = pending_storage_layout((*s).lit_bufsize);
            let header_crc = gzip_header_crc_bytes((*strm).adler);
            let mut pending = (*s).pending;
            assert!(with_pending_storage(pending_buffer, layout, |storage| {
                storage.append_pending(&mut pending, &header_crc)
            })
            .expect("pending storage layout matches its allocation"));
            (*s).pending = pending;
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
            (match configuration_table[(*s).level as usize].function {
                DeflateCompressionFunction::Stored => deflate_stored(s, flush),
                DeflateCompressionFunction::Fast => deflate_fast(s, flush),
                DeflateCompressionFunction::Slow => deflate_slow(s, flush),
            }) as ::core::ffi::c_uint
        }) as block_state;
        let (set_finish_state, return_ok) = deflate_block_state_actions(bstate);
        if set_finish_state {
            (*s).status = crate::src::deflate::FINISH_STATE;
        }
        if return_ok {
            if (*strm).avail_out == 0 as crate::stdlib::uInt {
                (*s).last_flush = -1 as ::core::ffi::c_int;
            }
            return crate::zlib_h::Z_OK;
        }
        if bstate as ::core::ffi::c_uint == block_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if flush == crate::zlib_h::Z_PARTIAL_FLUSH {
                crate::src::trees::_tr_align_ffi(s as *mut crate::src::deflate::internal_state);
            } else if flush != crate::zlib_h::Z_BLOCK {
                crate::src::trees::_tr_stored_block(
                    s as *mut crate::src::deflate::internal_state,
                    ::core::ptr::null_mut::<crate::stdlib::charf>(),
                    0 as crate::zutil_h::ulg,
                    0 as ::core::ffi::c_int,
                );
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    let head = core::slice::from_raw_parts_mut((*s).head, (*s).hash_size as usize);
                    clear_hash(head);
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
        let state = &mut *s;
        let pending_buffer = core::slice::from_raw_parts_mut(
            state
                .pending_buf
                .expect("validated pending storage")
                .as_ptr(),
            state.pending_buf_size as usize,
        );
        let layout = pending_storage_layout(state.lit_bufsize);
        let trailer = gzip_trailer_bytes((*strm).adler, (*strm).total_in);
        assert!(with_pending_storage(pending_buffer, layout, |storage| {
            storage.append_pending(&mut state.pending, &trailer)
        })
        .expect("pending storage layout matches its allocation"));
    } else {
        let state = &mut *s;
        let pending_buffer = core::slice::from_raw_parts_mut(
            state
                .pending_buf
                .expect("validated pending storage")
                .as_ptr(),
            state.pending_buf_size as usize,
        );
        let adler = (*strm).adler;
        let pending = &mut state.pending;
        assert!(
            with_pending_storage(pending_buffer, pending_layout, |storage| {
                let high = put_short_msb_core(
                    storage,
                    pending,
                    (adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
                );
                let low = put_short_msb_core(
                    storage,
                    pending,
                    (adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
                );
                high && low
            })
            .expect("pending storage layout matches its allocation")
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
#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    if !deflate_state_is_valid_at_ffi_boundary!(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = (*strm).state as *mut crate::src::deflate::deflate_state;
    status = (*state).status;
    if let Some(pending_buf) = (*state).pending_buf.take() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            pending_buf.as_ptr() as crate::stdlib::voidpf,
        );
    }
    if !(*state).head.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).head as crate::stdlib::voidpf,
        );
    }
    if !(*state).prev.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).prev as crate::stdlib::voidpf,
        );
    }
    if !(*state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as crate::stdlib::voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as crate::stdlib::voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<::core::ffi::c_void>();
    return if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
}
fn deflate_copy_prev_len(
    slid: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    let used = strstart.wrapping_sub(insert);
    if slid != 0 || used > w_size {
        w_size
    } else {
        used
    }
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
    if !deflate_state_is_valid_at_ffi_boundary!(source) || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    ss = (*source).state as *mut crate::src::deflate::deflate_state;
    // Reject an impossible callback request before allocating or copying any
    // destination state.  Normal states always originate from deflateInit2_,
    // which established this same plan.
    let Some(pending_plan) = pending_storage_allocation_plan((*ss).lit_bufsize) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(source_pending_layout) = pending_storage_layout_from_metadata(
        (*ss).lit_bufsize,
        (*ss).pending_buf_size,
        (*ss).sym_buf_offset,
        (*ss).sym_end,
    ) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if (*ss).pending_buf.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // `z_stream` is an ABI mirror made entirely of `Copy` fields.  Copy the
    // initialized record directly instead of crossing the foreign `memcpy`
    // boundary; the opaque state pointer is rebound below after its separate
    // callback allocation succeeds.
    *dest = *source;
    ds = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::deflate_state;
    if ds.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*dest).state = ds as *mut ::core::ffi::c_void;
    // The internal state is likewise a fully initialized `Copy` record at
    // this boundary.  Its separately allocated buffers are replaced below,
    // so this preserves the existing shallow-copy-then-rebind sequence
    // without a foreign whole-record copy.
    *ds = *ss;
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
    let pending_buf = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        pending_plan.items,
        pending_plan.item_size,
    ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef;
    (*ds).pending_buf = ::core::ptr::NonNull::new(pending_buf);
    if (*ds).window.is_null()
        || (*ds).prev.is_null()
        || (*ds).head.is_null()
        || (*ds).pending_buf.is_none()
    {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_MEM_ERROR;
    }
    let Ok(window_len) = usize::try_from((*ss).high_water) else {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Ok(prev_len) = usize::try_from(deflate_copy_prev_len(
        (*ss).slid,
        (*ss).strstart,
        (*ss).insert,
        (*ds).w_size,
    )) else {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Ok(head_len) = usize::try_from((*ds).hash_size) else {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Ok(window_capacity) = usize::try_from((*ss).window_size) else {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if window_len > window_capacity || prev_len > (*ds).w_size as usize {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // These separately callback-allocated buffers have no temporal overlap.
    // Establish bounded views only at this exported ownership boundary, then
    // use ordinary slice copies rather than foreign whole-buffer memcpy calls.
    if window_len != 0 {
        if (*ss).window.is_null() {
            deflateEnd_ffi(dest);
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        core::slice::from_raw_parts_mut((*ds).window, window_len)
            .copy_from_slice(core::slice::from_raw_parts((*ss).window, window_len));
    }
    if prev_len != 0 {
        if (*ss).prev.is_null() {
            deflateEnd_ffi(dest);
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        core::slice::from_raw_parts_mut((*ds).prev, prev_len)
            .copy_from_slice(core::slice::from_raw_parts((*ss).prev, prev_len));
    }
    if head_len != 0 {
        if (*ss).head.is_null() {
            deflateEnd_ffi(dest);
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        core::slice::from_raw_parts_mut((*ds).head, head_len)
            .copy_from_slice(core::slice::from_raw_parts((*ss).head, head_len));
    }
    let pending_layout = pending_plan.layout();
    debug_assert_eq!(source_pending_layout, pending_layout);
    let Some(pending_copy) = pending_storage_copy_plan(
        pending_layout,
        (*ss).pending_out_offset,
        (*ss).pending,
        (*ss).sym_next,
    ) else {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if (*ss).pending_buf.is_none() {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Both raw views are established at this exported ownership boundary.
    // The safe storage core below preserves the historical copy order for
    // the pending/symbol temporal overlay.
    let source_pending = core::slice::from_raw_parts(
        (*ss)
            .pending_buf
            .expect("validated source pending storage")
            .as_ptr(),
        pending_layout.total_len,
    );
    let destination_pending = core::slice::from_raw_parts_mut(
        (*ds)
            .pending_buf
            .expect("validated destination pending storage")
            .as_ptr(),
        pending_layout.total_len,
    );
    let Some(source_storage) = PendingStorageReadView::new(source_pending, pending_layout) else {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(mut destination_storage) =
        PendingStorageView::new(destination_pending, pending_layout)
    else {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !destination_storage.copy_initialized_from(&source_storage, pending_copy) {
        deflateEnd_ffi(dest);
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*ds).sym_buf_offset = pending_layout.symbol_offset;
    return crate::zlib_h::Z_OK;
}
fn longest_match_limit(
    strstart: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
) -> crate::src::deflate::IPos {
    let window_start =
        w_size.wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt);
    if strstart > window_start {
        (strstart as crate::src::deflate::IPos).wrapping_sub(window_start)
    } else {
        NIL as crate::src::deflate::IPos
    }
}

fn longest_match_search_parameters(
    max_chain_length: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
) -> (::core::ffi::c_uint, ::core::ffi::c_int) {
    let mut chain_length: ::core::ffi::c_uint = max_chain_length as ::core::ffi::c_uint;
    let mut nice_match = nice_match;

    if prev_length >= good_match {
        chain_length >>= 2 as ::core::ffi::c_int;
    }
    if nice_match as crate::stdlib::uInt > lookahead {
        nice_match = lookahead as ::core::ffi::c_int;
    }

    (chain_length, nice_match)
}

fn longest_match_next_chain_length(
    cur_match: crate::src::deflate::IPos,
    limit: crate::src::deflate::IPos,
    chain_length: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    if cur_match <= limit {
        return None;
    }

    let next_chain_length = chain_length.wrapping_sub(1);
    (next_chain_length != 0).then_some(next_chain_length)
}

fn longest_match_candidate_update(
    candidate_length: ::core::ffi::c_int,
    best_length: ::core::ffi::c_int,
    candidate_start: crate::src::deflate::IPos,
    nice_match: ::core::ffi::c_int,
) -> Option<(crate::src::deflate::IPos, ::core::ffi::c_int, bool)> {
    (candidate_length > best_length).then_some((
        candidate_start,
        candidate_length,
        candidate_length >= nice_match,
    ))
}

fn longest_match_clamp_length(
    best_len: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    let best_len = best_len as crate::stdlib::uInt;
    if best_len <= lookahead {
        best_len
    } else {
        lookahead
    }
}

fn deflate_fast_match_codes(
    match_length: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
) -> (crate::zutil_h::uch, crate::zutil_h::ush) {
    (
        match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch,
        strstart.wrapping_sub(match_start) as crate::zutil_h::ush,
    )
}

fn deflate_distance_tree_code(distance_minus_one: crate::zutil_h::ush) -> crate::zutil_h::uch {
    debug_assert!(distance_minus_one <= 32767 as crate::zutil_h::ush);
    let table_index = if distance_minus_one < 256 as crate::zutil_h::ush {
        distance_minus_one as usize
    } else {
        256usize + (distance_minus_one as usize >> 7)
    };
    crate::src::trees::_dist_code[table_index]
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct LongestMatchResult {
    match_start: crate::src::deflate::IPos,
    length: crate::stdlib::uInt,
}

/// Search the hash chain for the longest prefix shared with `strstart`.
///
/// The caller establishes the callback-backed window and hash-chain views.
/// Once those views exist, this is ordinary checked slice indexing: a corrupt
/// cursor simply aborts the search instead of deriving an out-of-bounds raw
/// pointer.
fn longest_match_core(
    window: &[crate::stdlib::Bytef],
    prev: &[crate::src::deflate::Posf],
    mut cur_match: crate::src::deflate::IPos,
    max_chain_length: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
) -> Option<LongestMatchResult> {
    let (mut chain_length, nice_match) = longest_match_search_parameters(
        max_chain_length,
        prev_length,
        good_match,
        nice_match,
        lookahead,
    );
    let max_match = usize::try_from(crate::zutil_h::MAX_MATCH).ok()?;
    let scan_start = usize::try_from(strstart).ok()?;
    let scan_end = scan_start.checked_add(max_match)?;
    let scan = window.get(scan_start..scan_end)?;
    let mut best_len = usize::try_from(prev_length).ok()?;
    if best_len > max_match {
        return None;
    }
    let limit = longest_match_limit(strstart, w_size);
    let mut best_match_start = 0;

    loop {
        let match_start = usize::try_from(cur_match).ok()?;
        let match_end = match_start.checked_add(max_match)?;
        let candidate = window.get(match_start..match_end)?;
        let candidate_len = scan
            .iter()
            .zip(candidate)
            .take(max_match)
            .take_while(|(left, right)| left == right)
            .count();

        if candidate_len > best_len {
            best_match_start = cur_match;
            best_len = candidate_len;
            if candidate_len >= nice_match.max(0) as usize {
                break;
            }
        }

        cur_match = *prev.get((cur_match & w_mask) as usize)? as crate::src::deflate::IPos;
        let Some(next_chain_length) =
            longest_match_next_chain_length(cur_match, limit, chain_length)
        else {
            break;
        };
        chain_length = next_chain_length;
    }

    Some(LongestMatchResult {
        match_start: best_match_start,
        length: longest_match_clamp_length(best_len as ::core::ffi::c_int, lookahead),
    })
}

unsafe fn longest_match(
    mut s: *mut crate::src::deflate::deflate_state,
    mut cur_match: crate::src::deflate::IPos,
) -> crate::stdlib::uInt {
    let state = &mut *s;
    if state.window.is_null() || state.prev.is_null() {
        return longest_match_clamp_length(
            state.prev_length as ::core::ffi::c_int,
            state.lookahead,
        );
    }
    let window = core::slice::from_raw_parts(state.window, state.window_size as usize);
    let prev = core::slice::from_raw_parts(state.prev, state.w_size as usize);
    let Some(result) = longest_match_core(
        window,
        prev,
        cur_match,
        state.max_chain_length,
        state.prev_length,
        state.good_match,
        state.nice_match,
        state.lookahead,
        state.strstart,
        state.w_size,
        state.w_mask,
    ) else {
        return longest_match_clamp_length(
            state.prev_length as ::core::ffi::c_int,
            state.lookahead,
        );
    };
    state.match_start = result.match_start as crate::stdlib::uInt;
    result.length
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

fn stored_block_header_bytes(bi_valid: ::core::ffi::c_int) -> ::core::ffi::c_uint {
    (bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
        >> 3 as ::core::ffi::c_int
}

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

fn stored_block_buffered_len(
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
) -> ::core::ffi::c_uint {
    (strstart as ::core::ffi::c_long - block_start) as ::core::ffi::c_uint
}

fn deflate_block_len(
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
) -> crate::zutil_h::ulg {
    (strstart as ::core::ffi::c_long - block_start) as crate::zutil_h::ulg
}

fn stored_block_available_output(
    bi_valid: ::core::ffi::c_int,
    avail_out: crate::stdlib::uInt,
) -> Option<::core::ffi::c_uint> {
    let header_bytes = stored_block_header_bytes(bi_valid);
    if avail_out < header_bytes {
        None
    } else {
        Some((avail_out as ::core::ffi::c_uint).wrapping_sub(header_bytes))
    }
}

fn stored_block_payload_len(
    left: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
    available_output: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let max_stored = MAX_STORED as ::core::ffi::c_uint;
    let input_available =
        (left as crate::zutil_h::ulg).wrapping_add(avail_in as crate::zutil_h::ulg);
    let payload_len = if (max_stored as crate::zutil_h::ulg) > input_available {
        (left as crate::stdlib::uInt).wrapping_add(avail_in) as ::core::ffi::c_uint
    } else {
        max_stored
    };

    payload_len.min(available_output)
}

fn stored_block_copy_lengths(
    buffered_len: ::core::ffi::c_uint,
    payload_len: ::core::ffi::c_uint,
) -> (::core::ffi::c_uint, ::core::ffi::c_uint) {
    let window_len = buffered_len.min(payload_len);
    (window_len, payload_len.wrapping_sub(window_len))
}

/// Copy the already-buffered portion of a stored block to its direct output.
///
/// The strategy adapter establishes the callback-owned window and caller
/// output views.  Keeping the range validation and copy here means that the
/// actual block transfer cannot derive an interior pointer or call C memcpy.
fn stored_block_copy_buffered_output(
    window: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    block_start: ::core::ffi::c_long,
    len: ::core::ffi::c_uint,
) -> bool {
    let Ok(start) = usize::try_from(block_start) else {
        return false;
    };
    let Ok(len) = usize::try_from(len) else {
        return false;
    };
    let Some(end) = start.checked_add(len) else {
        return false;
    };
    let Some(source) = window.get(start..end) else {
        return false;
    };
    let Some(destination) = output.get_mut(..len) else {
        return false;
    };
    destination.copy_from_slice(source);
    true
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct StoredHistoryState {
    matches: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
}

/// Commit direct-output input to the stored-block history window.
///
/// `deflate_stored()` historically did this with three raw `memcpy` calls.
/// The source slice is the exact consumed suffix of the caller input, while
/// the destination is the callback-owned window.  This core validates all
/// ranges before changing the window, including the overlapping slide.
fn stored_block_update_history(
    window: &mut [crate::stdlib::Bytef],
    input: &[crate::stdlib::Bytef],
    used: ::core::ffi::c_uint,
    w_size: crate::stdlib::uInt,
    window_size: crate::zutil_h::ulg,
    state: StoredHistoryState,
) -> Option<StoredHistoryState> {
    let used = usize::try_from(used).ok()?;
    let w_size = usize::try_from(w_size).ok()?;
    let window_size = usize::try_from(window_size).ok()?;
    let strstart = usize::try_from(state.strstart).ok()?;
    let insert = usize::try_from(state.insert).ok()?;
    if input.len() != used || window.len() < window_size {
        return None;
    }

    if used >= w_size {
        let source = input.get(used.checked_sub(w_size)?..used)?;
        let destination = window.get_mut(..w_size)?;
        destination.copy_from_slice(source);
        return Some(StoredHistoryState {
            matches: 2,
            strstart: w_size as crate::stdlib::uInt,
            insert: w_size as crate::stdlib::uInt,
            block_start: w_size as ::core::ffi::c_long,
        });
    }

    let slide = window_size.checked_sub(strstart)? <= used;
    let (next_strstart, next_matches, next_insert) = if slide {
        let next_strstart = strstart.checked_sub(w_size)?;
        let source_end = w_size.checked_add(strstart)?;
        window.get(w_size..source_end)?;
        window.get(..strstart)?;
        (
            next_strstart,
            if state.matches < 2 {
                state.matches.wrapping_add(1)
            } else {
                state.matches
            },
            insert.min(next_strstart),
        )
    } else {
        (strstart, state.matches, insert)
    };
    let input_end = next_strstart.checked_add(used)?;
    window.get(next_strstart..input_end)?;

    if slide {
        let source_end = w_size + strstart;
        window.copy_within(w_size..source_end, 0);
    }
    window[next_strstart..input_end].copy_from_slice(input);
    let final_strstart = next_strstart.checked_add(used)?;
    Some(StoredHistoryState {
        matches: next_matches,
        strstart: final_strstart as crate::stdlib::uInt,
        insert: stored_insert_after_input(
            next_insert as crate::stdlib::uInt,
            w_size as crate::stdlib::uInt,
            used as ::core::ffi::c_uint,
        ),
        block_start: final_strstart as ::core::ffi::c_long,
    })
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

fn stored_block_is_last(
    flush: ::core::ffi::c_int,
    len: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    avail_in: crate::stdlib::uInt,
) -> bool {
    flush == crate::zlib_h::Z_FINISH && len == (left as crate::stdlib::uInt).wrapping_add(avail_in)
}

fn stored_block_can_emit(
    left: ::core::ffi::c_uint,
    min_block: ::core::ffi::c_uint,
    flush: ::core::ffi::c_int,
    avail_in: crate::stdlib::uInt,
    have: ::core::ffi::c_uint,
) -> bool {
    left >= min_block
        || (left != 0 || flush == crate::zlib_h::Z_FINISH)
            && flush != crate::zlib_h::Z_NO_FLUSH
            && avail_in == 0
            && left <= have
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

fn stored_block_length_bytes(len: ::core::ffi::c_uint) -> [crate::stdlib::Bytef; 4] {
    [
        len as crate::stdlib::Bytef,
        (len >> 8) as crate::stdlib::Bytef,
        !len as crate::stdlib::Bytef,
        (!len >> 8) as crate::stdlib::Bytef,
    ]
}

/// Consume the direct-output stored-block header override, if one was set by
/// `deflate_stored()`. All other tree callers use their payload length.
pub(crate) fn take_pending_header_len_override(
    state: &mut crate::src::deflate::deflate_state,
    stored_len: crate::zutil_h::ulg,
) -> crate::zutil_h::ulg {
    state
        .pending_header_len_override
        .take()
        .unwrap_or(stored_len)
}

unsafe extern "C" fn deflate_stored(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let (pending_buf_size, w_size, wrap) = {
        let state = &*s;
        (state.pending_buf_size, state.w_size, state.wrap)
    };
    let mut min_block: ::core::ffi::c_uint = stored_block_min_size(pending_buf_size, w_size);
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
        left = stored_block_buffered_len((*s).strstart, (*s).block_start);
        len = stored_block_payload_len(left, (*(*s).strm).avail_in, have);
        if stored_block_should_wait(len, min_block, left, (*(*s).strm).avail_in, flush) {
            break;
        }
        last = stored_block_is_last(flush, len, left, (*(*s).strm).avail_in) as ::core::ffi::c_int;
        // The tree boundary owns the callback-backed pending-storage view.
        // Tell it to encode this direct-output block's length there, rather
        // than rewriting four pending bytes through raw interior pointers.
        (*s).pending_header_len_override = Some(len as crate::zutil_h::ulg);
        crate::src::trees::_tr_stored_block(
            s as *mut crate::src::deflate::internal_state,
            ::core::ptr::null_mut::<crate::stdlib::charf>(),
            0 as crate::zutil_h::ulg,
            last,
        );
        flush_pending((*s).strm);
        let (window_len, input_len) = stored_block_copy_lengths(left, len);
        if window_len != 0 {
            let window = core::slice::from_raw_parts((*s).window, (*s).window_size as usize);
            let output =
                core::slice::from_raw_parts_mut((*(*s).strm).next_out, window_len as usize);
            if !stored_block_copy_buffered_output(window, output, (*s).block_start, window_len) {
                return need_more;
            }
            (*(*s).strm).next_out = (*(*s).strm).next_out.offset(window_len as isize);
            (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(window_len);
            (*(*s).strm).total_out = (*(*s).strm)
                .total_out
                .wrapping_add(window_len as crate::stdlib::uLong);
            (*s).block_start += window_len as ::core::ffi::c_long;
        }
        if input_len != 0 {
            read_buf((*s).strm, (*(*s).strm).next_out, input_len, wrap);
            (*(*s).strm).next_out = (*(*s).strm).next_out.offset(input_len as isize);
            (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(input_len);
            (*(*s).strm).total_out = (*(*s).strm)
                .total_out
                .wrapping_add(input_len as crate::stdlib::uLong);
        }
        if !(last == 0 as ::core::ffi::c_int) {
            break;
        }
    }
    used = used.wrapping_sub((*(*s).strm).avail_in as ::core::ffi::c_uint);
    if used != 0 {
        let window = core::slice::from_raw_parts_mut((*s).window, (*s).window_size as usize);
        let input = core::slice::from_raw_parts(
            (*(*s).strm).next_in.offset(-(used as isize)),
            used as usize,
        );
        let Some(history) = stored_block_update_history(
            window,
            input,
            used,
            (*s).w_size,
            (*s).window_size,
            StoredHistoryState {
                matches: (*s).matches,
                strstart: (*s).strstart,
                insert: (*s).insert,
                block_start: (*s).block_start,
            },
        ) else {
            return need_more;
        };
        (*s).matches = history.matches;
        (*s).strstart = history.strstart;
        (*s).insert = history.insert;
        (*s).block_start = history.block_start;
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
            wrap,
        );
        (*s).strstart = (*s).strstart.wrapping_add(have);
        (*s).insert = stored_insert_after_input((*s).insert, (*s).w_size, have);
    }
    if (*s).high_water < (*s).strstart as crate::zutil_h::ulg {
        (*s).high_water = (*s).strstart as crate::zutil_h::ulg;
    }
    have = stored_block_header_bytes((*s).bi_valid);
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
    left = stored_block_buffered_len((*s).strstart, (*s).block_start);
    if stored_block_can_emit(left, min_block, flush, (*(*s).strm).avail_in, have) {
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
    let symbol_base = (*s)
        .pending_buf
        .expect("validated pending storage")
        .as_ptr()
        .wrapping_add((*s).sym_buf_offset);
    loop {
        if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window(s);
            match deflate_match_refill_action((*s).lookahead, flush) {
                DeflateMatchRefillAction::Continue => {}
                DeflateMatchRefillAction::NeedMore => return need_more,
                DeflateMatchRefillAction::EndBlock => break,
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
            *c2rust_fresh46 = *(*s).head.wrapping_add((*s).ins_h as usize);
            hash_head = *c2rust_fresh46 as crate::src::deflate::IPos;
            *(*s).head.wrapping_add((*s).ins_h as usize) =
                (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
        }
        if can_search_hash_match(hash_head, (*s).strstart, (*s).w_size) {
            (*s).match_length = longest_match(s, hash_head);
        }
        if (*s).match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let (len, mut dist) =
                deflate_fast_match_codes((*s).match_length, (*s).strstart, (*s).match_start);
            let c2rust_fresh47 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *symbol_base.wrapping_add(c2rust_fresh47 as usize) =
                dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh48 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *symbol_base.wrapping_add(c2rust_fresh48 as usize) =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh49 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *symbol_base.wrapping_add(c2rust_fresh49 as usize) = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            let length_tree_index = deflate_length_tree_index(len);
            (*s).dyn_ltree[length_tree_index].fc.value =
                (*s).dyn_ltree[length_tree_index].fc.value.wrapping_add(1);
            let distance_code = deflate_distance_tree_code(dist) as usize;
            (*s).dyn_dtree[distance_code].fc.value =
                (*s).dyn_dtree[distance_code].fc.value.wrapping_add(1);
            bflush = symbol_buffer_is_full((*s).sym_next, (*s).sym_end) as ::core::ffi::c_int;
            let progress = deflate_fast_match_progress(
                (*s).match_length,
                (*s).max_lazy_match,
                (*s).lookahead,
                (*s).strstart,
            );
            (*s).lookahead = progress.lookahead;
            if progress.insert {
                (*s).match_length = progress.remaining_match_length;
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
                    *c2rust_fresh50 = *(*s).head.wrapping_add((*s).ins_h as usize);
                    hash_head = *c2rust_fresh50 as crate::src::deflate::IPos;
                    *(*s).head.wrapping_add((*s).ins_h as usize) =
                        (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
                    (*s).match_length = (*s).match_length.wrapping_sub(1);
                    if !((*s).match_length != 0 as crate::stdlib::uInt) {
                        break;
                    }
                }
                (*s).strstart = progress.strstart;
            } else {
                (*s).strstart = progress.strstart;
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
            *symbol_base.wrapping_add(cursors[0] as usize) = 0 as crate::zutil_h::uchf;
            *symbol_base.wrapping_add(cursors[1] as usize) = 0 as crate::zutil_h::uchf;
            *symbol_base.wrapping_add(cursors[2] as usize) = cc as crate::zutil_h::uchf;
            (*s).dyn_ltree[cc as usize].fc.value =
                (*s).dyn_ltree[cc as usize].fc.value.wrapping_add(1);
            bflush = symbol_buffer_is_full((*s).sym_next, (*s).sym_end) as ::core::ffi::c_int;
            ((*s).lookahead, (*s).strstart) =
                deflate_literal_state_after_emit((*s).lookahead, (*s).strstart);
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
                deflate_block_len((*s).strstart, (*s).block_start),
                0 as ::core::ffi::c_int,
            );
            (*s).block_start = (*s).strstart as ::core::ffi::c_long;
            flush_pending((*s).strm);
            if let Some(state) =
                deflate_flush_block_state_after_output((*(*s).strm).avail_out, false)
            {
                return state;
            }
        }
    }
    (*s).insert = deflate_insert_after_block((*s).strstart);
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
            deflate_block_len((*s).strstart, (*s).block_start),
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if let Some(state) = deflate_flush_block_state_after_output((*(*s).strm).avail_out, true) {
            return state;
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
            deflate_block_len((*s).strstart, (*s).block_start),
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

fn deflate_slow_should_discard_match(
    match_length: crate::stdlib::uInt,
    strategy: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
) -> bool {
    match_length <= 5 as crate::stdlib::uInt
        && (strategy == crate::zlib_h::Z_FILTERED
            || match_length == crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                && strstart.wrapping_sub(match_start) > TOO_FAR as crate::stdlib::uInt)
}

fn deflate_slow_should_emit_previous_match(
    previous_match_length: crate::stdlib::uInt,
    match_length: crate::stdlib::uInt,
) -> bool {
    previous_match_length >= 3 && match_length <= previous_match_length
}

fn deflate_slow_max_insert(
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    strstart
        .wrapping_add(lookahead)
        .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
}

unsafe extern "C" fn deflate_slow(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    let symbol_base = (*s)
        .pending_buf
        .expect("validated pending storage")
        .as_ptr()
        .wrapping_add((*s).sym_buf_offset);
    loop {
        if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window(s);
            match deflate_match_refill_action((*s).lookahead, flush) {
                DeflateMatchRefillAction::Continue => {}
                DeflateMatchRefillAction::NeedMore => return need_more,
                DeflateMatchRefillAction::EndBlock => break,
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
            *c2rust_fresh35 = *(*s).head.wrapping_add((*s).ins_h as usize);
            hash_head = *c2rust_fresh35 as crate::src::deflate::IPos;
            *(*s).head.wrapping_add((*s).ins_h as usize) =
                (*s).strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
        }
        (*s).prev_length = (*s).match_length;
        (*s).prev_match = (*s).match_start as crate::src::deflate::IPos;
        (*s).match_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        if deflate_slow_can_search_match(
            hash_head,
            (*s).prev_length,
            (*s).max_lazy_match,
            (*s).strstart,
            (*s).w_size,
        ) {
            (*s).match_length = longest_match(s, hash_head);
            if deflate_slow_should_discard_match(
                (*s).match_length,
                (*s).strategy,
                (*s).strstart,
                (*s).match_start,
            ) {
                (*s).match_length =
                    (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
            }
        }
        if deflate_slow_should_emit_previous_match((*s).prev_length, (*s).match_length) {
            let mut max_insert: crate::stdlib::uInt =
                deflate_slow_max_insert((*s).strstart, (*s).lookahead);
            let mut len: crate::zutil_h::uch =
                (*s).prev_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let mut dist: crate::zutil_h::ush = ((*s).strstart as crate::src::deflate::IPos)
                .wrapping_sub(1 as crate::src::deflate::IPos)
                .wrapping_sub((*s).prev_match)
                as crate::zutil_h::ush;
            let c2rust_fresh36 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *symbol_base.wrapping_add(c2rust_fresh36 as usize) =
                dist as crate::zutil_h::uch as crate::zutil_h::uchf;
            let c2rust_fresh37 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *symbol_base.wrapping_add(c2rust_fresh37 as usize) =
                (dist as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as crate::zutil_h::uch
                    as crate::zutil_h::uchf;
            let c2rust_fresh38 = (*s).sym_next;
            (*s).sym_next = (*s).sym_next.wrapping_add(1);
            *symbol_base.wrapping_add(c2rust_fresh38 as usize) = len as crate::zutil_h::uchf;
            dist = dist.wrapping_sub(1);
            let length_tree_index = deflate_length_tree_index(len);
            (*s).dyn_ltree[length_tree_index].fc.value =
                (*s).dyn_ltree[length_tree_index].fc.value.wrapping_add(1);
            let distance_code = deflate_distance_tree_code(dist) as usize;
            (*s).dyn_dtree[distance_code].fc.value =
                (*s).dyn_dtree[distance_code].fc.value.wrapping_add(1);
            bflush = symbol_buffer_is_full((*s).sym_next, (*s).sym_end) as ::core::ffi::c_int;
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
                    *c2rust_fresh39 = *(*s).head.wrapping_add((*s).ins_h as usize);
                    hash_head = *c2rust_fresh39 as crate::src::deflate::IPos;
                    *(*s).head.wrapping_add((*s).ins_h as usize) =
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
                    deflate_block_len((*s).strstart, (*s).block_start),
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
            let tally = deflate_literal_tally_plan(cc, (*s).sym_next);
            (*s).sym_next = tally.next_sym;
            *symbol_base.wrapping_add(tally.cursors[0] as usize) = tally.symbol_bytes[0];
            *symbol_base.wrapping_add(tally.cursors[1] as usize) = tally.symbol_bytes[1];
            *symbol_base.wrapping_add(tally.cursors[2] as usize) = tally.symbol_bytes[2];
            (*s).dyn_ltree[tally.literal_tree_index].fc.value = (*s).dyn_ltree
                [tally.literal_tree_index]
                .fc
                .value
                .wrapping_add(1);
            bflush = symbol_buffer_is_full((*s).sym_next, (*s).sym_end) as ::core::ffi::c_int;
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
                    deflate_block_len((*s).strstart, (*s).block_start),
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
        let tally = deflate_literal_tally_plan(cc_0, (*s).sym_next);
        (*s).sym_next = tally.next_sym;
        *symbol_base.wrapping_add(tally.cursors[0] as usize) = tally.symbol_bytes[0];
        *symbol_base.wrapping_add(tally.cursors[1] as usize) = tally.symbol_bytes[1];
        *symbol_base.wrapping_add(tally.cursors[2] as usize) = tally.symbol_bytes[2];
        (*s).dyn_ltree[tally.literal_tree_index].fc.value = (*s).dyn_ltree
            [tally.literal_tree_index]
            .fc
            .value
            .wrapping_add(1);
        bflush = symbol_buffer_is_full((*s).sym_next, (*s).sym_end) as ::core::ffi::c_int;
        (*s).match_available = 0 as ::core::ffi::c_int;
    }
    (*s).insert = deflate_insert_after_block((*s).strstart);
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
            deflate_block_len((*s).strstart, (*s).block_start),
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if let Some(state) = deflate_flush_block_state_after_output((*(*s).strm).avail_out, true) {
            return state;
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
            deflate_block_len((*s).strstart, (*s).block_start),
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
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_window(s);
            match deflate_rle_refill_action((*s).lookahead, flush) {
                DeflateRleRefillAction::Continue => {}
                DeflateRleRefillAction::NeedMore => return need_more,
                DeflateRleRefillAction::Done => break,
            }
        }
        let state = &mut *s;
        let window = if state.window.is_null() {
            None
        } else {
            Some(&*core::ptr::slice_from_raw_parts(
                state.window,
                state.window_size as usize,
            ))
        };
        state.match_length = 0 as crate::stdlib::uInt;
        if let Some(window) = window {
            if let Some(match_length) =
                deflate_rle_scan_match(window, state.lookahead, state.strstart)
            {
                state.match_length = match_length;
            }
        }
        let layout =
            pending_storage_layout_for_state(state).expect("validated pending storage layout");
        let pending = &mut *core::ptr::slice_from_raw_parts_mut(
            state
                .pending_buf
                .expect("validated pending storage")
                .as_ptr(),
            layout.total_len,
        );
        let mut storage = PendingStorageView::new(pending, layout)
            .expect("pending storage layout matches its allocation");
        match deflate_rle_tally_plan(state.match_length) {
            DeflateRleTallyPlan::MatchWithoutCount => {
                let tally = deflate_rle_match_tally_plan(state.match_length, state.sym_next);
                state.sym_next = tally.next_sym;
                assert!(storage.write_symbol_triplet(tally.cursors, tally.symbol_bytes));
                state.dyn_ltree[tally.length_tree_index].fc.value = state.dyn_ltree
                    [tally.length_tree_index]
                    .fc
                    .value
                    .wrapping_add(1);
                state.dyn_dtree[tally.distance_tree_index].fc.value = state.dyn_dtree
                    [tally.distance_tree_index]
                    .fc
                    .value
                    .wrapping_add(1);
                bflush = symbol_buffer_is_full(state.sym_next, state.sym_end) as ::core::ffi::c_int;
                (state.lookahead, state.strstart, state.match_length) =
                    deflate_rle_match_state_after_emit(
                        state.lookahead,
                        state.strstart,
                        state.match_length,
                    );
            }
            DeflateRleTallyPlan::Literal => {
                let Some(&literal) = window.and_then(|bytes| bytes.get(state.strstart as usize))
                else {
                    return need_more;
                };
                let tally =
                    deflate_literal_tally_plan(literal as crate::zutil_h::uch, state.sym_next);
                state.sym_next = tally.next_sym;
                assert!(storage.write_symbol_triplet(tally.cursors, tally.symbol_bytes));
                state.dyn_ltree[tally.literal_tree_index].fc.value = state.dyn_ltree
                    [tally.literal_tree_index]
                    .fc
                    .value
                    .wrapping_add(1);
                bflush = symbol_buffer_is_full(state.sym_next, state.sym_end) as ::core::ffi::c_int;
                (state.lookahead, state.strstart) =
                    deflate_literal_state_after_emit(state.lookahead, state.strstart);
            }
        }
        if bflush != 0 {
            let stored_len = deflate_block_len(state.strstart, state.block_start);
            let stored_data = if state.block_start >= 0 as ::core::ffi::c_long {
                let start = state.block_start as ::core::ffi::c_uint as usize;
                let Some(end) = start.checked_add(stored_len as usize) else {
                    return need_more;
                };
                let Some(data) = window.and_then(|bytes| bytes.get(start..end)) else {
                    return need_more;
                };
                Some(data)
            } else {
                None
            };
            let stream = &mut *state.strm;
            crate::src::trees::tr_flush_block_core(
                &mut storage,
                state,
                Some(stream),
                stored_data,
                stored_len,
                0 as ::core::ffi::c_int,
            );
            drop(storage);
            state.block_start = state.strstart as ::core::ffi::c_long;
            flush_pending(state.strm);
            if let Some(state) = deflate_flush_block_state_after_output((*stream).avail_out, false)
            {
                return state;
            }
        }
    }
    (*s).insert = 0 as crate::stdlib::uInt;
    let final_flush_action = deflate_final_flush_action(flush, (*s).sym_next);
    if final_flush_action == DeflateFinalFlushAction::Finish {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::Bytef as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            deflate_block_len((*s).strstart, (*s).block_start),
            1 as ::core::ffi::c_int,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        flush_pending((*s).strm);
        if let Some(state) = deflate_flush_block_state_after_output((*(*s).strm).avail_out, true) {
            return state;
        }
        return finish_done;
    }
    if final_flush_action == DeflateFinalFlushAction::FlushPendingSymbols {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if (*s).block_start >= 0 as ::core::ffi::c_long {
                (*s).window
                    .offset((*s).block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::Bytef as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            deflate_block_len((*s).strstart, (*s).block_start),
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

unsafe fn deflate_huff(
    mut s: *mut crate::src::deflate::deflate_state,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead == 0 as crate::stdlib::uInt {
            fill_window(s);
            match deflate_huff_refill_action((*s).lookahead, flush) {
                DeflateHuffRefillAction::Continue => {}
                DeflateHuffRefillAction::NeedMore => return need_more,
                DeflateHuffRefillAction::Done => break,
            }
        }
        let state = &mut *s;
        state.match_length = 0 as crate::stdlib::uInt;
        let literal: crate::zutil_h::uch =
            *state.window.offset(state.strstart as isize) as crate::zutil_h::uch;
        let layout =
            pending_storage_layout_for_state(state).expect("validated pending storage layout");
        let pending = &mut *core::ptr::slice_from_raw_parts_mut(
            state
                .pending_buf
                .expect("validated pending storage")
                .as_ptr(),
            layout.total_len,
        );
        let mut storage = PendingStorageView::new(pending, layout)
            .expect("pending storage layout matches its allocation");
        let tally = deflate_literal_tally_plan(literal, state.sym_next);
        state.sym_next = tally.next_sym;
        assert!(storage.write_symbol_triplet(tally.cursors, tally.symbol_bytes));
        state.dyn_ltree[tally.literal_tree_index].fc.value = state.dyn_ltree
            [tally.literal_tree_index]
            .fc
            .value
            .wrapping_add(1);
        bflush = symbol_buffer_is_full(state.sym_next, state.sym_end) as ::core::ffi::c_int;
        (state.lookahead, state.strstart) =
            deflate_literal_state_after_emit(state.lookahead, state.strstart);
        if bflush != 0 {
            let stored_len = deflate_block_len(state.strstart, state.block_start);
            let stored_data = if state.block_start >= 0 as ::core::ffi::c_long {
                Some(core::slice::from_raw_parts(
                    state
                        .window
                        .add(state.block_start as ::core::ffi::c_uint as usize),
                    stored_len as usize,
                ))
            } else {
                None
            };
            let stream = &mut *state.strm;
            crate::src::trees::tr_flush_block_core(
                &mut storage,
                state,
                Some(stream),
                stored_data,
                stored_len,
                0 as ::core::ffi::c_int,
            );
            drop(storage);
            state.block_start = state.strstart as ::core::ffi::c_long;
            flush_pending(state.strm);
            if let Some(state) = deflate_flush_block_state_after_output((*stream).avail_out, false)
            {
                return state;
            }
        }
    }
    let state = &mut *s;
    state.insert = 0 as crate::stdlib::uInt;
    let final_flush_action = deflate_final_flush_action(flush, state.sym_next);
    if final_flush_action == DeflateFinalFlushAction::Finish {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if state.block_start >= 0 as ::core::ffi::c_long {
                state
                    .window
                    .offset(state.block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::Bytef as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            deflate_block_len(state.strstart, state.block_start),
            1 as ::core::ffi::c_int,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        flush_pending(state.strm);
        let stream = &mut *state.strm;
        if let Some(state) = deflate_flush_block_state_after_output((*stream).avail_out, true) {
            return state;
        }
        return finish_done;
    }
    if final_flush_action == DeflateFinalFlushAction::FlushPendingSymbols {
        crate::src::trees::_tr_flush_block(
            s as *mut crate::src::deflate::internal_state,
            if state.block_start >= 0 as ::core::ffi::c_long {
                state
                    .window
                    .offset(state.block_start as ::core::ffi::c_uint as isize)
                    as *mut crate::stdlib::Bytef as *mut crate::stdlib::charf
            } else {
                ::core::ptr::null_mut::<crate::stdlib::charf>()
            },
            deflate_block_len(state.strstart, state.block_start),
            0 as ::core::ffi::c_int,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        flush_pending(state.strm);
        let stream = &mut *state.strm;
        if (*stream).avail_out == 0 as crate::stdlib::uInt {
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
        can_search_hash_match, clamped_copy_len, clear_hash, deflate_block_len,
        deflate_block_state_actions, deflate_bound_lengths, deflate_bound_z_core,
        deflate_copy_prev_len, deflate_copyright, deflate_dictionary_insert_hashes,
        deflate_dictionary_len, deflate_dictionary_state_after_load, deflate_distance_tree_code,
        deflate_fast_match_codes, deflate_fast_match_progress, deflate_fast_should_insert_match,
        deflate_final_flush_action, deflate_flush_block_state_after_output, deflate_flush_rank,
        deflate_huff_literal_progress, deflate_insert_after_block, deflate_length_tree_index,
        deflate_literal_state_after_emit, deflate_literal_tally_plan, deflate_match_refill_action,
        deflate_pending_value, deflate_preflight, deflate_prime_bits_valid,
        deflate_prime_has_pending_space, deflate_prime_insert_bits, deflate_request_is_invalid,
        deflate_reset_status_and_adler, deflate_rle_can_scan_match, deflate_rle_clamp_match_length,
        deflate_rle_match_length, deflate_rle_match_state_after_emit, deflate_rle_match_tally_plan,
        deflate_rle_next_scan_indices, deflate_rle_refill_action, deflate_rle_scan_indices,
        deflate_rle_scan_match, deflate_rle_tally_plan, deflate_set_dictionary_allowed,
        deflate_should_return_buf_error, deflate_slow_can_search_match, deflate_state_is_usable,
        deflate_state_status_valid, deflate_version_matches, dictionary_tail_offset, drain_pending,
        fill_window_available_space, fill_window_cursor, fill_window_has_insertable_match,
        fill_window_hash_update, fill_window_high_water_after_zero, fill_window_insert_after_slide,
        fill_window_lookahead_after_read, fill_window_reinsert, fill_window_should_refill,
        fill_window_should_slide, fill_window_slide, fill_window_state_after_slide,
        fill_window_zero, fill_window_zero_range, flush_pending_core, gzip_custom_header_bytes,
        gzip_default_header_bytes, gzip_default_xfl, gzip_extra_copy_chunk, gzip_header_crc,
        gzip_header_crc_bytes, gzip_header_crc_pending, gzip_header_crc_pending_range,
        gzip_trailer_bytes, lm_head_reset_plan, lm_init_plan, lm_initial_state,
        lm_match_parameters, lm_reset_plan, longest_match_candidate_update,
        longest_match_clamp_length, longest_match_core, longest_match_limit,
        longest_match_next_chain_length, longest_match_search_parameters, normalize_deflate_params,
        pending_buffer_needs_flush, pending_output_len, pending_storage_copy_plan,
        pending_storage_layout, pending_storage_layout_for_state,
        pending_storage_layout_from_metadata, put_short_msb_core, read_buf_checksum, read_buf_core,
        read_buf_input_progress_after_copy, read_buf_len, read_buf_total_in_after_copy,
        short_msb_bytes, slide_hash_core, slide_hash_entry, stored_block_available_output,
        stored_block_buffered_len, stored_block_can_emit, stored_block_copy_buffered_output,
        stored_block_copy_lengths, stored_block_header_bytes, stored_block_is_last,
        stored_block_length_bytes, stored_block_min_size, stored_block_payload_len,
        stored_block_should_wait, stored_block_update_history, stored_insert_after_input,
        symbol_buffer_is_full, symbol_triplet_cursors, take_pending_header_len_override,
        with_pending_storage, zlib_header, DeflateBoundGzipHeader, DeflateBoundState,
        DeflateFastMatchProgress, DeflateFinalFlushAction, DeflateMatchRefillAction,
        DeflatePreflight, DeflateRleRefillAction, DeflateRleTallyPlan, FlushPendingResult,
        LongestMatchResult, PendingDrainState, PendingStorageReadView, PendingStorageView,
        ReadBufChecksum, ReadBufResult, StoredHistoryState,
    };

    #[test]
    fn newly_allocated_deflate_state_matches_c_zero_initialization() {
        let state = super::internal_state::newly_allocated();

        assert!(state.strm.is_null());
        assert!(state.pending_buf.is_none());
        assert!(state.gzhead.is_null());
        assert!(state.window.is_null());
        assert!(state.prev.is_null());
        assert!(state.head.is_null());
        assert_eq!(state.sym_buf_offset, 0);
        assert_eq!(state.status, 0);
        assert_eq!(state.pending_out_offset, 0);
        assert_eq!(state.pending_header_len_override, None);
        assert_eq!(state.window_size, 0);
        assert_eq!(state.block_start, 0);
        assert_eq!(state.high_water, 0);
        assert!(state
            .dyn_ltree
            .iter()
            .chain(state.dyn_dtree.iter())
            .chain(state.bl_tree.iter())
            .all(|entry| entry.fc.value == 0 && entry.dl.value == 0));
        assert!(state.bl_count.iter().all(|count| *count == 0));
        assert!(state.heap.iter().all(|entry| *entry == 0));
        assert!(state.depth.iter().all(|entry| *entry == 0));
        assert_eq!(state.l_desc.dynamic_tree as u8, 0);
        assert_eq!(state.d_desc.dynamic_tree as u8, 0);
        assert_eq!(state.bl_desc.dynamic_tree as u8, 0);
        assert_eq!(state.l_desc.max_code, 0);
        assert_eq!(state.d_desc.max_code, 0);
        assert_eq!(state.bl_desc.max_code, 0);
    }

    #[test]
    fn deflate_params_plan_and_apply_keep_hash_work_at_the_boundary() {
        let mut state = super::internal_state::newly_allocated();
        state.level = 0;
        state.strategy = crate::zlib_h::Z_DEFAULT_STRATEGY;
        state.last_flush = -2;
        state.matches = 1;

        let plan = super::deflate_params_plan(&state, 1, crate::zlib_h::Z_DEFAULT_STRATEGY)
            .expect("valid parameters must produce a plan");
        assert!(!plan.flush_before_apply);
        assert_eq!(plan.hash_action, super::DeflateParamsHashAction::Rebase);

        super::deflate_params_apply(&mut state, plan);
        assert_eq!(state.level, 1);
        assert_eq!(state.strategy, crate::zlib_h::Z_DEFAULT_STRATEGY);
        assert_eq!(state.matches, 0);
        assert_eq!(state.slid, 1);
        assert_eq!(state.max_lazy_match, 4);
        assert_eq!(state.good_match, 4);
        assert_eq!(state.nice_match, 8);
        assert_eq!(state.max_chain_length, 4);
    }

    #[test]
    fn deflate_params_plan_uses_configuration_function_groups() {
        use super::DeflateCompressionFunction::{Fast, Slow, Stored};

        assert_eq!(
            super::deflate_compression_function_for_level(0),
            Some(Stored)
        );
        for level in 1..=3 {
            assert_eq!(
                super::deflate_compression_function_for_level(level),
                Some(Fast)
            );
        }
        for level in 4..=9 {
            assert_eq!(
                super::deflate_compression_function_for_level(level),
                Some(Slow)
            );
        }
        assert_eq!(super::deflate_compression_function_for_level(-1), None);
        assert_eq!(super::deflate_compression_function_for_level(10), None);

        let mut state = super::internal_state::newly_allocated();
        state.level = 1;
        state.strategy = crate::zlib_h::Z_DEFAULT_STRATEGY;
        state.last_flush = crate::zlib_h::Z_NO_FLUSH;

        assert!(
            !super::deflate_params_plan(&state, 3, state.strategy)
                .expect("valid parameters must produce a plan")
                .flush_before_apply
        );
        assert!(
            super::deflate_params_plan(&state, 4, state.strategy)
                .expect("valid parameters must produce a plan")
                .flush_before_apply
        );

        state.level = 0;
        assert!(
            super::deflate_params_plan(&state, 1, state.strategy)
                .expect("valid parameters must produce a plan")
                .flush_before_apply
        );
    }

    #[test]
    fn configuration_table_encodes_compression_policy_without_function_pointers() {
        use super::DeflateCompressionFunction::{Fast, Slow, Stored};

        let expected = [Stored, Fast, Fast, Fast, Slow, Slow, Slow, Slow, Slow, Slow];
        for (config, function) in super::configuration_table.iter().zip(expected) {
            assert_eq!(config.function, function);
        }
    }

    #[test]
    fn deflate_params_hash_actions_use_only_safe_table_views() {
        let mut head = [7, 3, 0, 12];
        super::deflate_params_apply_hash_action(
            super::DeflateParamsHashAction::Clear,
            &mut head,
            None,
            4,
        );
        assert_eq!(head, [0; 4]);

        let mut head = [0, 4, 6, 8];
        let mut prev = [1, 2, 5, 9];
        super::deflate_params_apply_hash_action(
            super::DeflateParamsHashAction::Rebase,
            &mut head,
            Some(&mut prev),
            4,
        );
        assert_eq!(head, [0, 0, 2, 4]);
        assert_eq!(prev, [0, 0, 1, 5]);
    }

    #[test]
    fn deflate_tune_core_updates_all_tuning_fields() {
        let mut good_match = 0;
        let mut max_lazy_match = 0;
        let mut nice_match = 0;
        let mut max_chain_length = 0;

        assert_eq!(
            super::deflate_tune_core(
                &mut good_match,
                &mut max_lazy_match,
                &mut nice_match,
                &mut max_chain_length,
                4,
                5,
                6,
                7,
            ),
            crate::zlib_h::Z_OK
        );
        assert_eq!(good_match, 4);
        assert_eq!(max_lazy_match, 5);
        assert_eq!(nice_match, 6);
        assert_eq!(max_chain_length, 7);
    }

    #[test]
    fn deflate_tune_core_preserves_c_cast_and_signed_value_behavior() {
        let mut good_match = 0;
        let mut max_lazy_match = 0;
        let mut nice_match = 0;
        let mut max_chain_length = 0;

        super::deflate_tune_core(
            &mut good_match,
            &mut max_lazy_match,
            &mut nice_match,
            &mut max_chain_length,
            ::core::ffi::c_int::MIN,
            -1,
            ::core::ffi::c_int::MIN,
            ::core::ffi::c_int::MAX,
        );

        assert_eq!(good_match, ::core::ffi::c_int::MIN as crate::stdlib::uInt);
        assert_eq!(
            max_lazy_match,
            (-1 as ::core::ffi::c_int) as crate::stdlib::uInt
        );
        assert_eq!(nice_match, ::core::ffi::c_int::MIN);
        assert_eq!(
            max_chain_length,
            ::core::ffi::c_int::MAX as crate::stdlib::uInt
        );
    }

    #[test]
    fn deflate_block_state_actions_preserve_finish_and_return_semantics() {
        assert_eq!(
            deflate_block_state_actions(crate::src::deflate::need_more),
            (false, true)
        );
        assert_eq!(
            deflate_block_state_actions(crate::src::deflate::block_done),
            (false, false)
        );
        assert_eq!(
            deflate_block_state_actions(crate::src::deflate::finish_started),
            (true, true)
        );
        assert_eq!(
            deflate_block_state_actions(crate::src::deflate::finish_done),
            (true, false)
        );
        assert_eq!(deflate_block_state_actions(4), (false, false));
    }

    #[test]
    fn deflate_rle_clamp_match_length_preserves_match_bounds() {
        assert_eq!(deflate_rle_clamp_match_length(3, 258), 3);
        assert_eq!(deflate_rle_clamp_match_length(258, 258), 258);
        assert_eq!(deflate_rle_clamp_match_length(258, 17), 17);
        assert_eq!(deflate_rle_clamp_match_length(258, 0), 0);
        assert_eq!(
            deflate_rle_clamp_match_length(crate::stdlib::uInt::MAX, crate::stdlib::uInt::MAX),
            crate::stdlib::uInt::MAX
        );
    }

    #[test]
    fn stored_block_buffered_len_preserves_signed_difference_conversion() {
        assert_eq!(stored_block_buffered_len(17, 5), 12);
        assert_eq!(stored_block_buffered_len(5, 5), 0);
        assert_eq!(stored_block_buffered_len(0, 1), ::core::ffi::c_uint::MAX);
    }

    #[test]
    fn deflate_block_len_preserves_signed_difference_conversion() {
        assert_eq!(deflate_block_len(17, 5), 12);
        assert_eq!(deflate_block_len(0, -1), 1);
        assert_eq!(deflate_block_len(0, 1), crate::zutil_h::ulg::MAX);
    }

    #[test]
    fn deflate_rle_match_length_preserves_scan_distance_and_lookahead_bounds() {
        let max_match = crate::zutil_h::MAX_MATCH as crate::stdlib::uInt;

        assert_eq!(deflate_rle_match_length(0, max_match), max_match);
        assert_eq!(deflate_rle_match_length(5, max_match), max_match - 5);
        assert_eq!(deflate_rle_match_length(0, 17), 17);
        assert_eq!(
            deflate_rle_match_length(max_match.wrapping_add(1), crate::stdlib::uInt::MAX),
            crate::stdlib::uInt::MAX,
        );
    }

    #[test]
    fn deflate_rle_can_scan_match_requires_minimum_lookahead_and_prior_byte() {
        let min_match = crate::zutil_h::MIN_MATCH as crate::stdlib::uInt;

        assert!(!deflate_rle_can_scan_match(min_match.wrapping_sub(1), 1));
        assert!(!deflate_rle_can_scan_match(min_match, 0));
        assert!(deflate_rle_can_scan_match(min_match, 1));
    }

    #[test]
    fn deflate_rle_match_tally_plan_encodes_minimum_match_without_counting_it() {
        let plan =
            deflate_rle_match_tally_plan(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt, 7);

        assert_eq!(plan.cursors, [7, 8, 9]);
        assert_eq!(plan.next_sym, 10);
        assert_eq!(plan.symbol_bytes, [1, 0, 0]);
        assert_eq!(plan.length_tree_index, 257);
        assert_eq!(plan.distance_tree_index, 0);
    }

    #[test]
    fn deflate_rle_match_tally_plan_preserves_maximum_length_and_cursor_wrapping() {
        let plan = deflate_rle_match_tally_plan(
            crate::zutil_h::MAX_MATCH as crate::stdlib::uInt,
            crate::stdlib::uInt::MAX,
        );

        assert_eq!(plan.cursors, [crate::stdlib::uInt::MAX, 0, 1]);
        assert_eq!(plan.next_sym, 2);
        assert_eq!(plan.symbol_bytes, [1, 0, 255]);
        assert_eq!(plan.length_tree_index, 285);
        assert_eq!(plan.distance_tree_index, 0);
    }

    #[test]
    fn deflate_literal_tally_plan_preserves_literal_bytes_and_cursor_wrapping() {
        let normal = deflate_literal_tally_plan(0, 7);
        assert_eq!(normal.cursors, [7, 8, 9]);
        assert_eq!(normal.next_sym, 10);
        assert_eq!(normal.symbol_bytes, [0, 0, 0]);
        assert_eq!(normal.literal_tree_index, 0);

        let wrapped =
            deflate_literal_tally_plan(crate::zutil_h::uch::MAX, crate::stdlib::uInt::MAX);
        assert_eq!(wrapped.cursors, [crate::stdlib::uInt::MAX, 0, 1]);
        assert_eq!(wrapped.next_sym, 2);
        assert_eq!(wrapped.symbol_bytes, [0, 0, crate::zutil_h::uch::MAX]);
        assert_eq!(
            wrapped.literal_tree_index,
            crate::zutil_h::uch::MAX as usize
        );
    }

    #[test]
    fn huffman_literal_tally_uses_the_bounded_symbol_view() {
        let layout = pending_storage_layout(4);
        let mut bytes = [0xaa; 16];
        let mut storage = PendingStorageView::new(&mut bytes, layout).unwrap();
        let tally = deflate_literal_tally_plan(b'Q', 6);

        assert!(storage.write_symbol_triplet(tally.cursors, tally.symbol_bytes));
        assert_eq!(tally.next_sym, 9);
        assert_eq!(&storage.symbol_bytes()[6..9], &[0, 0, b'Q']);
        assert_eq!(storage.symbol_bytes()[5], 0xaa);
        assert_eq!(storage.symbol_bytes()[9], 0xaa);
        assert!(symbol_buffer_is_full(9, 9));
    }

    #[test]
    fn deflate_rle_tally_plan_uses_direct_match_tally_at_minimum_length() {
        let min_match = crate::zutil_h::MIN_MATCH as crate::stdlib::uInt;

        assert_eq!(
            deflate_rle_tally_plan(min_match),
            DeflateRleTallyPlan::MatchWithoutCount
        );
        assert_eq!(
            deflate_rle_tally_plan(crate::zutil_h::MAX_MATCH as crate::stdlib::uInt),
            DeflateRleTallyPlan::MatchWithoutCount
        );
    }

    #[test]
    fn deflate_rle_tally_plan_uses_literal_below_minimum_length() {
        let min_match = crate::zutil_h::MIN_MATCH as crate::stdlib::uInt;

        assert_eq!(deflate_rle_tally_plan(0), DeflateRleTallyPlan::Literal);
        assert_eq!(
            deflate_rle_tally_plan(min_match.wrapping_sub(1)),
            DeflateRleTallyPlan::Literal
        );
    }

    #[test]
    fn deflate_match_refill_action_preserves_flush_and_lookahead_boundaries() {
        let min_lookahead = crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt;

        assert_eq!(
            deflate_match_refill_action(0, crate::zlib_h::Z_NO_FLUSH),
            DeflateMatchRefillAction::NeedMore
        );
        assert_eq!(
            deflate_match_refill_action(0, crate::zlib_h::Z_FINISH),
            DeflateMatchRefillAction::EndBlock
        );
        assert_eq!(
            deflate_match_refill_action(min_lookahead.wrapping_sub(1), crate::zlib_h::Z_FINISH),
            DeflateMatchRefillAction::Continue
        );
        assert_eq!(
            deflate_match_refill_action(min_lookahead, crate::zlib_h::Z_NO_FLUSH),
            DeflateMatchRefillAction::Continue
        );
    }

    #[test]
    fn deflate_rle_refill_action_preserves_flush_and_lookahead_boundaries() {
        let max_match = crate::zutil_h::MAX_MATCH as crate::stdlib::uInt;

        assert_eq!(
            deflate_rle_refill_action(max_match, crate::zlib_h::Z_NO_FLUSH),
            DeflateRleRefillAction::NeedMore
        );
        assert_eq!(
            deflate_rle_refill_action(0, crate::zlib_h::Z_FINISH),
            DeflateRleRefillAction::Done
        );
        assert_eq!(
            deflate_rle_refill_action(max_match, crate::zlib_h::Z_FINISH),
            DeflateRleRefillAction::Continue
        );
        assert_eq!(
            deflate_rle_refill_action(max_match.wrapping_add(1), crate::zlib_h::Z_NO_FLUSH),
            DeflateRleRefillAction::Continue
        );
    }

    #[test]
    fn deflate_rle_match_state_after_emit_preserves_wrapping_and_clears_length() {
        assert_eq!(deflate_rle_match_state_after_emit(10, 5, 3), (7, 8, 0));
        assert_eq!(
            deflate_rle_match_state_after_emit(0, crate::stdlib::uInt::MAX, 1),
            (crate::stdlib::uInt::MAX, 0, 0),
        );
    }

    #[test]
    fn deflate_literal_state_after_emit_preserves_wrapping_progress() {
        assert_eq!(deflate_literal_state_after_emit(10, 5), (9, 6));
        assert_eq!(
            deflate_literal_state_after_emit(0, crate::stdlib::uInt::MAX),
            (crate::stdlib::uInt::MAX, 0),
        );
    }

    #[test]
    fn deflate_fast_should_insert_match_honors_match_and_lookahead_boundaries() {
        let min_match = crate::zutil_h::MIN_MATCH as crate::stdlib::uInt;

        assert!(deflate_fast_should_insert_match(4, 4, min_match));
        assert!(!deflate_fast_should_insert_match(5, 4, min_match));
        assert!(!deflate_fast_should_insert_match(4, 4, min_match - 1));
        assert!(deflate_fast_should_insert_match(
            crate::stdlib::uInt::MAX,
            crate::stdlib::uInt::MAX,
            crate::stdlib::uInt::MAX,
        ));
    }

    #[test]
    fn deflate_fast_match_progress_preserves_insert_skip_and_wrapping_transitions() {
        let min_match = crate::zutil_h::MIN_MATCH as crate::stdlib::uInt;

        assert_eq!(
            deflate_fast_match_progress(4, 4, min_match + 6, 10),
            DeflateFastMatchProgress {
                lookahead: min_match + 2,
                remaining_match_length: 3,
                strstart: 14,
                insert: true,
            },
        );
        assert_eq!(
            deflate_fast_match_progress(5, 4, min_match + 6, 10),
            DeflateFastMatchProgress {
                lookahead: min_match + 1,
                remaining_match_length: 0,
                strstart: 15,
                insert: false,
            },
        );
        assert_eq!(
            deflate_fast_match_progress(3, 3, min_match, crate::stdlib::uInt::MAX),
            DeflateFastMatchProgress {
                lookahead: 0,
                remaining_match_length: 0,
                strstart: 2,
                insert: false,
            },
        );
    }

    #[test]
    fn deflate_insert_after_block_caps_to_hashable_prefix() {
        let prefix = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;

        assert_eq!(deflate_insert_after_block(0), 0);
        assert_eq!(
            deflate_insert_after_block(prefix.wrapping_sub(1)),
            prefix - 1
        );
        assert_eq!(deflate_insert_after_block(prefix), prefix);
        assert_eq!(deflate_insert_after_block(prefix.wrapping_add(1)), prefix);
        assert_eq!(deflate_insert_after_block(crate::stdlib::uInt::MAX), prefix);
    }

    #[test]
    fn deflate_huff_literal_progress_preserves_cursor_and_flush_boundaries() {
        assert_eq!(
            deflate_huff_literal_progress(3, 6, 5, 10),
            (3, 4, 11, false)
        );
        assert_eq!(deflate_huff_literal_progress(6, 6, 5, 10), (6, 4, 11, true));
        assert_eq!(
            deflate_huff_literal_progress(7, 6, 0, crate::stdlib::uInt::MAX),
            (7, crate::stdlib::uInt::MAX, 0, false)
        );
    }

    #[test]
    fn deflate_final_flush_action_prioritizes_finish_and_pending_symbols() {
        assert_eq!(
            deflate_final_flush_action(crate::zlib_h::Z_FINISH, 0),
            DeflateFinalFlushAction::Finish
        );
        assert_eq!(
            deflate_final_flush_action(crate::zlib_h::Z_FINISH, 7),
            DeflateFinalFlushAction::Finish
        );
        assert_eq!(
            deflate_final_flush_action(crate::zlib_h::Z_NO_FLUSH, 7),
            DeflateFinalFlushAction::FlushPendingSymbols
        );
        assert_eq!(
            deflate_final_flush_action(crate::zlib_h::Z_NO_FLUSH, 0),
            DeflateFinalFlushAction::Done
        );
    }

    #[test]
    fn deflate_flush_block_state_after_output_preserves_final_block_boundary() {
        assert_eq!(deflate_flush_block_state_after_output(1, false), None);
        assert_eq!(
            deflate_flush_block_state_after_output(0, false),
            Some(crate::src::deflate::need_more)
        );
        assert_eq!(
            deflate_flush_block_state_after_output(0, true),
            Some(crate::src::deflate::finish_started)
        );
    }

    #[test]
    fn fill_window_has_insertable_match_preserves_minimum_and_wrapping_thresholds() {
        let min_match = crate::zutil_h::MIN_MATCH as crate::stdlib::uInt;

        assert!(!fill_window_has_insertable_match(
            min_match.wrapping_sub(1),
            0
        ));
        assert!(fill_window_has_insertable_match(min_match, 0));
        assert!(fill_window_has_insertable_match(
            1,
            min_match.wrapping_sub(1)
        ));
        assert!(fill_window_has_insertable_match(
            crate::stdlib::uInt::MAX,
            4
        ));
        assert!(!fill_window_has_insertable_match(
            crate::stdlib::uInt::MAX,
            3
        ));
    }

    #[test]
    fn fill_window_high_water_after_zero_preserves_wrapping_advance() {
        assert_eq!(fill_window_high_water_after_zero(10, 5), 15);
        assert_eq!(
            fill_window_high_water_after_zero(crate::zutil_h::ulg::MAX, 1),
            0
        );
    }

    #[test]
    fn deflate_rle_scan_indices_preserve_initial_and_unrolled_positions() {
        assert_eq!(deflate_rle_scan_indices(10), (9, [10, 11, 12], 12, 268));
        assert_eq!(
            deflate_rle_next_scan_indices(12),
            ([13, 14, 15, 16, 17, 18, 19, 20], 20),
        );
        assert_eq!(
            deflate_rle_scan_indices(crate::stdlib::uInt::MAX),
            (
                crate::stdlib::uInt::MAX - 1,
                [crate::stdlib::uInt::MAX, 0, 1],
                1,
                257,
            ),
        );
    }

    #[test]
    fn deflate_rle_scan_match_uses_checked_window_reads_and_preserves_run_length() {
        let mut window = [b'x'; 300];
        assert_eq!(deflate_rle_scan_match(&window, 258, 10), Some(258));

        window[11] = b'y';
        assert_eq!(deflate_rle_scan_match(&window, 258, 10), Some(0));
        window[11] = b'x';
        assert_eq!(deflate_rle_scan_match(&window[..12], 258, 10), None);
        assert_eq!(deflate_rle_scan_match(&window, 2, 10), Some(0));
    }

    #[test]
    fn lm_match_parameters_preserve_compression_level_tuning() {
        assert_eq!(lm_match_parameters(0), (0, 0, 0, 0));
        assert_eq!(lm_match_parameters(1), (4, 4, 8, 4));
        assert_eq!(lm_match_parameters(6), (16, 8, 128, 128));
        assert_eq!(lm_match_parameters(9), (258, 32, 258, 4096));
    }

    #[test]
    fn lm_initial_state_preserves_window_size_and_match_baseline() {
        let baseline = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;

        assert_eq!(lm_initial_state(0), (0, baseline));
        assert_eq!(lm_initial_state(32), (64, baseline));
        assert_eq!(
            lm_initial_state(crate::stdlib::uInt::MAX),
            (
                (crate::stdlib::uInt::MAX as crate::zutil_h::ulg).wrapping_mul(2),
                baseline,
            ),
        );
    }

    #[test]
    fn lm_reset_plan_preserves_window_and_level_initialization() {
        assert_eq!(
            lm_reset_plan(32, 6),
            super::LmResetPlan {
                window_size: 64,
                prev_length: (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt,
                max_lazy_match: 16,
                good_match: 8,
                nice_match: 128,
                max_chain_length: 128,
            },
        );
    }

    #[test]
    fn lm_init_plan_combines_head_clear_and_scalar_reset() {
        let entry_size =
            ::core::mem::size_of::<crate::src::deflate::Posf>() as crate::__stddef_size_t_h::size_t;

        assert_eq!(
            lm_init_plan(17, 32, 6),
            super::LmInitPlan {
                head_reset: super::LmHeadResetPlan {
                    clear_len: 17 * entry_size,
                },
                reset: super::LmResetPlan {
                    window_size: 64,
                    prev_length: (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt,
                    max_lazy_match: 16,
                    good_match: 8,
                    nice_match: 128,
                    max_chain_length: 128,
                },
            },
        );
        assert_eq!(lm_init_plan(0, 0, 0).head_reset.clear_len, 0);
    }

    #[test]
    fn lm_head_reset_plan_covers_the_entire_hash_table() {
        let entry_size =
            ::core::mem::size_of::<crate::src::deflate::Posf>() as crate::__stddef_size_t_h::size_t;

        assert_eq!(
            lm_head_reset_plan(1),
            super::LmHeadResetPlan {
                clear_len: entry_size,
            },
        );
        assert_eq!(
            lm_head_reset_plan(17),
            super::LmHeadResetPlan {
                clear_len: 17 * entry_size,
            },
        );
        assert_eq!(
            lm_head_reset_plan(0),
            super::LmHeadResetPlan { clear_len: 0 },
        );
    }

    #[test]
    fn clear_hash_matches_the_c_macro_layout() {
        let mut head: [crate::src::deflate::Posf; 4] = [9, 8, 7, 6];

        clear_hash(&mut head);

        assert_eq!(
            head,
            [
                0,
                0,
                0,
                crate::src::deflate::NIL as crate::src::deflate::Posf
            ]
        );

        let mut empty: [crate::src::deflate::Posf; 0] = [];
        clear_hash(&mut empty);
        assert_eq!(empty, []);
    }

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
    fn symbol_buffer_is_full_requires_exact_cursor_equality() {
        assert!(symbol_buffer_is_full(12, 12));
        assert!(!symbol_buffer_is_full(11, 12));
        assert!(!symbol_buffer_is_full(
            crate::stdlib::uInt::MAX,
            crate::stdlib::uInt::MAX - 1,
        ));
    }

    #[test]
    fn hash_match_search_gate_preserves_nil_and_distance_boundaries() {
        let w_size = 32_768;
        let maximum_distance = w_size - crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt;

        assert!(!can_search_hash_match(
            crate::src::deflate::NIL as crate::src::deflate::IPos,
            1,
            w_size
        ));
        assert!(can_search_hash_match(
            1,
            maximum_distance.wrapping_add(1),
            w_size
        ));
        assert!(!can_search_hash_match(
            1,
            maximum_distance.wrapping_add(2),
            w_size
        ));
        assert!(can_search_hash_match(
            crate::stdlib::uInt::MAX as crate::src::deflate::IPos,
            0,
            w_size
        ));
    }

    #[test]
    fn deflate_slow_can_search_match_preserves_lazy_and_distance_boundaries() {
        let w_size = 32_768;
        let max_lazy_match = 16;
        let maximum_distance = w_size - crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt;
        let valid_strstart = maximum_distance.wrapping_add(1);

        assert!(!deflate_slow_can_search_match(
            crate::src::deflate::NIL as crate::src::deflate::IPos,
            max_lazy_match - 1,
            max_lazy_match,
            valid_strstart,
            w_size,
        ));
        assert!(!deflate_slow_can_search_match(
            1,
            max_lazy_match,
            max_lazy_match,
            valid_strstart,
            w_size,
        ));
        assert!(deflate_slow_can_search_match(
            1,
            max_lazy_match - 1,
            max_lazy_match,
            valid_strstart,
            w_size,
        ));
        assert!(deflate_slow_can_search_match(
            1,
            0,
            max_lazy_match,
            valid_strstart,
            w_size,
        ));
        assert!(!deflate_slow_can_search_match(
            1,
            0,
            max_lazy_match,
            valid_strstart.wrapping_add(1),
            w_size,
        ));
        assert!(deflate_slow_can_search_match(
            crate::stdlib::uInt::MAX as crate::src::deflate::IPos,
            0,
            max_lazy_match,
            0,
            w_size,
        ));
    }

    #[test]
    fn deflate_slow_should_discard_match_preserves_filtered_and_distance_rules() {
        assert!(super::deflate_slow_should_discard_match(
            5,
            crate::zlib_h::Z_FILTERED,
            0,
            0,
        ));
        assert!(!super::deflate_slow_should_discard_match(
            6,
            crate::zlib_h::Z_FILTERED,
            0,
            0,
        ));
        assert!(!super::deflate_slow_should_discard_match(
            crate::zutil_h::MIN_MATCH as crate::stdlib::uInt,
            crate::zlib_h::Z_DEFAULT_STRATEGY,
            super::TOO_FAR as crate::stdlib::uInt,
            0,
        ));
        assert!(super::deflate_slow_should_discard_match(
            crate::zutil_h::MIN_MATCH as crate::stdlib::uInt,
            crate::zlib_h::Z_DEFAULT_STRATEGY,
            (super::TOO_FAR as crate::stdlib::uInt).wrapping_add(1),
            0,
        ));
        assert!(super::deflate_slow_should_discard_match(
            crate::zutil_h::MIN_MATCH as crate::stdlib::uInt,
            crate::zlib_h::Z_DEFAULT_STRATEGY,
            0,
            crate::stdlib::uInt::MAX.wrapping_sub(super::TOO_FAR as crate::stdlib::uInt),
        ));
    }

    #[test]
    fn deflate_slow_should_emit_previous_match_requires_a_minimum_non_worse_match() {
        assert!(!super::deflate_slow_should_emit_previous_match(2, 0));
        assert!(super::deflate_slow_should_emit_previous_match(3, 3));
        assert!(super::deflate_slow_should_emit_previous_match(4, 3));
        assert!(!super::deflate_slow_should_emit_previous_match(3, 4));
    }

    #[test]
    fn deflate_slow_max_insert_preserves_wrapping_bounds() {
        assert_eq!(super::deflate_slow_max_insert(10, 7), 14);
        assert_eq!(
            super::deflate_slow_max_insert(crate::stdlib::uInt::MAX, 1),
            0u32.wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
        );
        assert_eq!(
            super::deflate_slow_max_insert(0, 0),
            crate::stdlib::uInt::MAX - 2
        );
    }

    #[test]
    fn deflate_huff_refill_action_preserves_empty_input_behavior() {
        assert_eq!(
            super::deflate_huff_refill_action(1, crate::zlib_h::Z_NO_FLUSH),
            super::DeflateHuffRefillAction::Continue,
        );
        assert_eq!(
            super::deflate_huff_refill_action(0, crate::zlib_h::Z_NO_FLUSH),
            super::DeflateHuffRefillAction::NeedMore,
        );
        assert_eq!(
            super::deflate_huff_refill_action(0, crate::zlib_h::Z_FINISH),
            super::DeflateHuffRefillAction::Done,
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
    fn deflate_dictionary_state_after_load_preserves_post_load_state() {
        let previous_match_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;

        assert_eq!(
            deflate_dictionary_state_after_load(10, 5),
            (
                15,
                15,
                5,
                0,
                previous_match_length,
                previous_match_length,
                0
            ),
        );
    }

    #[test]
    fn deflate_dictionary_state_after_load_preserves_wrapping_cursor() {
        let previous_match_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;

        assert_eq!(
            deflate_dictionary_state_after_load(crate::stdlib::uInt::MAX, 1),
            (0, 0, 1, 0, previous_match_length, previous_match_length, 0),
        );
    }

    #[test]
    fn dictionary_hash_insert_preserves_hash_chain_order() {
        let window = [10, 11, 2, 3, 4];
        let mut head: [crate::src::deflate::Posf; 8] = [0, 0, 7, 8, 9, 0, 0, 0];
        let mut prev: [crate::src::deflate::Posf; 4] = [0; 4];
        let mut ins_h = 0;

        assert_eq!(
            deflate_dictionary_insert_hashes(
                &window, &mut head, &mut prev, 0, 5, &mut ins_h, 5, 7, 3,
            ),
            Some(3)
        );
        assert_eq!(ins_h, 4);
        assert_eq!(prev, [7, 8, 9, 0]);
        assert_eq!(head, [0, 0, 0, 1, 2, 0, 0, 0]);
    }

    #[test]
    fn dictionary_hash_insert_rejects_short_window_without_mutation() {
        let window = [0, 1];
        let mut head: [crate::src::deflate::Posf; 8] = [4; 8];
        let mut prev: [crate::src::deflate::Posf; 4] = [5; 4];
        let mut ins_h = 6;

        assert_eq!(
            deflate_dictionary_insert_hashes(
                &window, &mut head, &mut prev, 0, 3, &mut ins_h, 5, 7, 3,
            ),
            None
        );
        assert_eq!(ins_h, 6);
        assert_eq!(head, [4; 8]);
        assert_eq!(prev, [5; 4]);
    }

    #[test]
    fn deflate_set_header_requires_gzip_wrapping() {
        assert!(!super::deflate_set_header_allowed(0));
        assert!(!super::deflate_set_header_allowed(1));
        assert!(super::deflate_set_header_allowed(2));
        assert!(!super::deflate_set_header_allowed(-2));
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
    fn put_short_msb_core_writes_network_order_and_advances_pending() {
        let mut pending_buffer = [0; 4];
        let mut pending = 1;
        let layout = pending_storage_layout(1);

        assert!(
            with_pending_storage(&mut pending_buffer, layout, |storage| {
                put_short_msb_core(storage, &mut pending, 0x1234)
            })
            .unwrap()
        );
        assert_eq!(pending_buffer, [0, 0x12, 0x34, 0]);
        assert_eq!(pending, 3);
    }

    #[test]
    fn put_short_msb_core_rejects_short_or_overflowed_ranges_without_progress() {
        let mut pending_buffer = [0xaa; 4];
        let mut pending = 3;
        let layout = pending_storage_layout(1);

        assert!(
            !with_pending_storage(&mut pending_buffer, layout, |storage| {
                put_short_msb_core(storage, &mut pending, 0x1234)
            })
            .unwrap()
        );
        assert_eq!(pending_buffer, [0xaa; 4]);
        assert_eq!(pending, 3);

        let mut pending_buffer = [0xaa; 4];
        let mut pending = crate::zutil_h::ulg::MAX;

        assert!(
            !with_pending_storage(&mut pending_buffer, layout, |storage| {
                put_short_msb_core(storage, &mut pending, 0x1234)
            })
            .unwrap()
        );
        assert_eq!(pending_buffer, [0xaa; 4]);
        assert_eq!(pending, crate::zutil_h::ulg::MAX);
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
    fn pending_buffer_needs_flush_preserves_wrapping_capacity_checks() {
        assert!(!pending_buffer_needs_flush(3, 5, 8));
        assert!(pending_buffer_needs_flush(3, 6, 8));
        assert!(!pending_buffer_needs_flush(0, 0, 0));
        assert!(!pending_buffer_needs_flush(
            crate::zutil_h::ulg::MAX,
            1,
            crate::zutil_h::ulg::MAX,
        ));
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
    fn gzip_default_header_bytes_match_the_fixed_gzip_header() {
        assert_eq!(
            gzip_default_header_bytes(6, crate::zlib_h::Z_DEFAULT_STRATEGY),
            [31, 139, 8, 0, 0, 0, 0, 0, 0, 3]
        );
        assert_eq!(
            gzip_default_header_bytes(9, crate::zlib_h::Z_DEFAULT_STRATEGY),
            [31, 139, 8, 0, 0, 0, 0, 0, 2, 3]
        );
    }

    #[test]
    fn gzip_custom_header_bytes_include_flags_timestamp_xfl_and_optional_extra_length() {
        assert_eq!(
            gzip_custom_header_bytes(1, 1, true, true, true, 0x1234_5678, 9, 0, 3, 0xabcd),
            ([0x1f, 0x78, 0x56, 0x34, 0x12, 2, 3, 0xcd, 0xab], 9)
        );
        assert_eq!(
            gzip_custom_header_bytes(0, 0, false, false, false, 0, 1, 0, 255, 0),
            ([0, 0, 0, 0, 0, 4, 255, 0, 0], 7)
        );
    }

    #[test]
    fn gzip_header_crc_bytes_are_little_endian() {
        assert_eq!(gzip_header_crc_bytes(0x1234_5678), [0x78, 0x56]);
    }

    #[test]
    fn gzip_trailer_bytes_are_little_endian_checksum_then_size() {
        assert_eq!(
            gzip_trailer_bytes(0x1234_5678, 0x9abc_def0),
            [0x78, 0x56, 0x34, 0x12, 0xf0, 0xde, 0xbc, 0x9a]
        );
    }

    #[test]
    fn pending_output_len_uses_available_output_capacity() {
        assert_eq!(pending_output_len(0, 0), 0);
        assert_eq!(pending_output_len(3, 5), 3);
        assert_eq!(pending_output_len(5, 5), 5);
        assert_eq!(pending_output_len(8, 5), 5);
    }

    #[test]
    fn flush_pending_core_copies_partial_pending_output() {
        assert_eq!(
            flush_pending_core(
                PendingDrainState {
                    pending: 5,
                    pending_out_offset: 7,
                },
                3,
                12,
            ),
            Some(FlushPendingResult {
                copied: 3,
                next: PendingDrainState {
                    pending: 2,
                    pending_out_offset: 10,
                },
                avail_out: 0,
                total_out: 15,
                reset_pending_out: false,
            })
        );
    }

    #[test]
    fn flush_pending_core_copies_exact_pending_output() {
        assert_eq!(
            flush_pending_core(
                PendingDrainState {
                    pending: 3,
                    pending_out_offset: 7,
                },
                8,
                crate::stdlib::uLong::MAX,
            ),
            Some(FlushPendingResult {
                copied: 3,
                next: PendingDrainState {
                    pending: 0,
                    pending_out_offset: 0,
                },
                avail_out: 5,
                total_out: 2,
                reset_pending_out: true,
            })
        );
    }

    #[test]
    fn flush_pending_core_is_a_zero_pending_no_op() {
        assert_eq!(
            flush_pending_core(
                PendingDrainState {
                    pending: 0,
                    pending_out_offset: 7,
                },
                3,
                12,
            ),
            None
        );
    }

    #[test]
    fn drain_pending_copies_the_selected_pending_segment() {
        let pending = *b"0123456789";
        let mut output = [0xaa; 5];

        assert_eq!(
            drain_pending(
                &pending,
                PendingDrainState {
                    pending: 4,
                    pending_out_offset: 3,
                },
                &mut output,
                5,
                11,
            ),
            Some(FlushPendingResult {
                copied: 4,
                next: PendingDrainState {
                    pending: 0,
                    pending_out_offset: 0,
                },
                avail_out: 1,
                total_out: 15,
                reset_pending_out: true,
            })
        );
        assert_eq!(output, [b'3', b'4', b'5', b'6', 0xaa]);
    }

    #[test]
    fn drain_pending_rejects_invalid_storage_or_output_without_writing() {
        let pending = *b"012345";
        let mut short_output = [0xaa; 2];
        assert_eq!(
            drain_pending(
                &pending,
                PendingDrainState {
                    pending: 3,
                    pending_out_offset: 2,
                },
                &mut short_output,
                3,
                0,
            ),
            None
        );
        assert_eq!(short_output, [0xaa; 2]);

        let mut output = [0xaa; 3];
        assert_eq!(
            drain_pending(
                &pending,
                PendingDrainState {
                    pending: 3,
                    pending_out_offset: 5,
                },
                &mut output,
                3,
                0,
            ),
            None
        );
        assert_eq!(output, [0xaa; 3]);
    }

    #[test]
    fn drain_pending_leaves_output_untouched_when_no_bytes_can_be_drained() {
        let pending = *b"012345";
        let mut output = [0xaa; 2];
        assert_eq!(
            drain_pending(
                &pending,
                PendingDrainState {
                    pending: 2,
                    pending_out_offset: 1,
                },
                &mut output,
                0,
                9,
            ),
            None
        );
        assert_eq!(output, [0xaa; 2]);
    }

    #[test]
    fn pending_storage_layout_preserves_shared_allocation_geometry() {
        assert_eq!(
            pending_storage_layout(16),
            super::PendingStorageLayout {
                total_len: 64,
                symbol_offset: 16,
                symbol_len: 48,
                symbol_flush_threshold: 45,
            }
        );
        assert_eq!(
            pending_storage_layout(1),
            super::PendingStorageLayout {
                total_len: 4,
                symbol_offset: 1,
                symbol_len: 3,
                symbol_flush_threshold: 0,
            }
        );
    }

    #[test]
    fn pending_storage_allocation_plan_preserves_the_single_c_domain_request() {
        assert_eq!(
            super::pending_storage_allocation_plan(16),
            Some(super::PendingStorageAllocationPlan {
                items: 16,
                item_size: 4,
                total_len: 64,
            })
        );
    }

    #[test]
    fn pending_storage_allocation_plan_rejects_c_uint_multiplication_overflow() {
        assert_eq!(
            super::pending_storage_allocation_plan(crate::stdlib::uInt::MAX),
            None
        );
        assert_eq!(
            super::pending_storage_allocation_plan(crate::stdlib::uInt::MAX / 4),
            Some(super::PendingStorageAllocationPlan {
                items: crate::stdlib::uInt::MAX / 4,
                item_size: 4,
                total_len: (crate::stdlib::uInt::MAX / 4 * 4) as usize,
            })
        );
    }

    #[test]
    fn pending_storage_plan_and_layout_share_exact_geometry() {
        let plan = super::pending_storage_allocation_plan(16).unwrap();
        assert_eq!(plan.layout(), pending_storage_layout(16));
        assert_eq!(
            plan.layout(),
            super::PendingStorageLayout {
                total_len: plan.total_len,
                symbol_offset: plan.items as usize,
                symbol_len: plan.total_len - plan.items as usize,
                symbol_flush_threshold: plan.items.wrapping_sub(1).wrapping_mul(3),
            }
        );
    }

    #[test]
    fn pending_storage_metadata_accepts_only_the_allocation_plan_geometry() {
        let layout = pending_storage_layout(16);

        assert_eq!(
            pending_storage_layout_from_metadata(
                16,
                layout.total_len as crate::zutil_h::ulg,
                layout.symbol_offset,
                layout.symbol_flush_threshold,
            ),
            Some(layout)
        );
        assert_eq!(
            pending_storage_layout_from_metadata(
                16,
                layout.total_len as crate::zutil_h::ulg - 1,
                layout.symbol_offset,
                layout.symbol_flush_threshold,
            ),
            None
        );
        assert_eq!(
            pending_storage_layout_from_metadata(
                16,
                layout.total_len as crate::zutil_h::ulg,
                layout.symbol_offset + 1,
                layout.symbol_flush_threshold,
            ),
            None
        );
        assert_eq!(
            pending_storage_layout_from_metadata(
                16,
                layout.total_len as crate::zutil_h::ulg,
                layout.symbol_offset,
                layout.symbol_flush_threshold.wrapping_add(1),
            ),
            None
        );
    }

    #[test]
    fn pending_storage_metadata_rejects_unrepresentable_allocation_plans() {
        assert_eq!(
            pending_storage_layout_from_metadata(crate::stdlib::uInt::MAX, 0, 0, 0),
            None
        );
    }

    #[test]
    fn pending_storage_state_layout_uses_only_checked_metadata() {
        let mut state = super::internal_state::newly_allocated();
        let layout = pending_storage_layout(16);
        state.lit_bufsize = 16;
        state.pending_buf_size = layout.total_len as crate::zutil_h::ulg;
        state.sym_buf_offset = layout.symbol_offset;
        state.sym_end = layout.symbol_flush_threshold;

        assert_eq!(pending_storage_layout_for_state(&state), Some(layout));

        state.pending_buf_size = state.pending_buf_size.wrapping_sub(1);
        assert_eq!(pending_storage_layout_for_state(&state), None);
    }

    #[test]
    fn pending_storage_view_preserves_temporal_pending_and_symbol_access() {
        let layout = pending_storage_layout(4);
        let mut bytes = [0; 16];
        let mut storage = super::PendingStorageView::new(&mut bytes, layout).unwrap();
        let mut pending = 0;

        assert!(storage.append_pending(&mut pending, &[1, 2, 3]));
        assert_eq!(pending, 3);
        assert_eq!(storage.pending_bytes()[..3], [1, 2, 3]);
        assert_eq!(storage.symbol_bytes().len(), 12);
        assert!(!storage.append_pending(&mut pending, &[0; 14]));
        assert_eq!(pending, 3);
    }

    #[test]
    fn pending_storage_read_view_limits_a_larger_backing_slice_to_its_layout() {
        let layout = pending_storage_layout(4);
        let bytes = [0x5a; 20];
        let storage = super::PendingStorageReadView::new(&bytes, layout).unwrap();

        assert_eq!(
            storage.pending_range(0, layout.total_len),
            Some(&bytes[..16])
        );
        assert_eq!(storage.pending_range(layout.total_len, 1), None);
    }

    #[test]
    fn pending_storage_view_writes_symbol_triplets_atomically() {
        let layout = pending_storage_layout(4);
        let mut bytes = [0; 16];
        let mut storage = super::PendingStorageView::new(&mut bytes, layout).unwrap();

        assert!(storage.write_symbol_triplet([0, 1, 2], [7, 8, 9]));
        assert_eq!(&storage.symbol_bytes()[..3], &[7, 8, 9]);

        assert!(!storage.write_symbol_triplet([2, 12, 3], [1, 2, 3]));
        assert_eq!(&storage.symbol_bytes()[..3], &[7, 8, 9]);
    }

    #[test]
    fn pending_storage_copy_plan_preserves_pending_cursor_and_symbol_prefix() {
        let layout = pending_storage_layout(4);
        let source = [
            0x00, 0x01, 0x02, 0x03, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
            0x1a, 0x1b,
        ];
        let mut destination = [0xaa; 16];
        let plan = pending_storage_copy_plan(layout, 3, 5, 4).unwrap();
        let source_storage = PendingStorageReadView::new(&source, layout).unwrap();
        let mut destination_storage = PendingStorageView::new(&mut destination, layout).unwrap();

        assert!(destination_storage.copy_initialized_from(&source_storage, plan));
        drop(destination_storage);

        assert_eq!(&destination[..3], &[0xaa; 3]);
        assert_eq!(&destination[3..8], &source[3..8]);
        assert_eq!(&destination[8..], &[0xaa; 8]);
    }

    #[test]
    fn pending_storage_copy_plan_rejects_out_of_range_initialized_bytes() {
        let layout = pending_storage_layout(2);
        assert_eq!(pending_storage_copy_plan(layout, 7, 2, 0), None);
        assert_eq!(pending_storage_copy_plan(layout, 0, 0, 7), None);
    }

    #[test]
    fn gzip_extra_copy_chunk_moves_only_the_requested_checked_ranges() {
        let mut pending = [0xaa; 8];
        let extra = *b"abcdef";

        assert_eq!(
            gzip_extra_copy_chunk(&mut pending, 2, &extra, 1, 3),
            Some(5)
        );
        assert_eq!(pending, [0xaa, 0xaa, b'b', b'c', b'd', 0xaa, 0xaa, 0xaa]);
    }

    #[test]
    fn gzip_extra_copy_chunk_rejects_invalid_ranges_without_mutation() {
        let mut pending = [0xaa; 4];
        let original = pending;
        let extra = *b"abc";

        assert_eq!(gzip_extra_copy_chunk(&mut pending, 3, &extra, 0, 2), None);
        assert_eq!(pending, original);
        assert_eq!(gzip_extra_copy_chunk(&mut pending, 0, &extra, 2, 2), None);
        assert_eq!(pending, original);
    }

    #[test]
    fn deflate_prime_bits_valid_accepts_the_supported_inclusive_range() {
        assert!(!deflate_prime_bits_valid(-1));
        assert!(deflate_prime_bits_valid(0));
        assert!(deflate_prime_bits_valid(16));
        assert!(!deflate_prime_bits_valid(17));
    }

    #[test]
    fn deflate_prime_requires_two_pending_bytes_before_symbol_storage() {
        assert!(deflate_prime_has_pending_space(8, 6));
        assert!(!deflate_prime_has_pending_space(8, 7));
        assert!(!deflate_prime_has_pending_space(8, usize::MAX));
    }

    #[test]
    fn deflate_prime_inserts_and_flushes_sixteen_bits() {
        let layout = pending_storage_layout(2);
        let mut bytes = [0; 8];
        let mut storage = PendingStorageView::new(&mut bytes, layout).unwrap();
        let mut pending = 0;
        let mut bi_buf = 0;
        let mut bi_valid = 0;

        assert!(deflate_prime_insert_bits(
            &mut storage,
            &mut pending,
            &mut bi_buf,
            &mut bi_valid,
            16,
            0xbeef,
        ));
        drop(storage);

        assert_eq!(pending, 2);
        assert_eq!(bi_buf, 0);
        assert_eq!(bi_valid, 0);
        assert_eq!(&bytes[..2], &[0xef, 0xbe]);
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
    fn slide_hash_entry_handles_all_boundary_positions() {
        let window_size = 32 as crate::stdlib::uInt;
        let entries: [crate::src::deflate::Posf; 4] = [0, 31, 32, 47];
        let rebased =
            entries.map(|entry| slide_hash_entry(entry as ::core::ffi::c_uint, window_size));

        assert_eq!(rebased, [0, 0, 0, 15]);
    }

    #[test]
    fn slide_hash_core_rebases_both_hash_tables() {
        let window_size = 32 as crate::stdlib::uInt;
        let mut head: [crate::src::deflate::Posf; 4] = [0, 31, 32, 47];
        let mut prev: [crate::src::deflate::Posf; 3] = [33, 63, 64];

        slide_hash_core(&mut head, &mut prev, window_size);

        assert_eq!(head, [0, 0, 0, 15]);
        assert_eq!(prev, [1, 31, 32]);
    }

    #[test]
    fn slide_hash_core_accepts_empty_tables() {
        let mut head: [crate::src::deflate::Posf; 0] = [];
        let mut prev: [crate::src::deflate::Posf; 0] = [];

        slide_hash_core(&mut head, &mut prev, 32);
    }

    #[test]
    fn longest_match_limit_preserves_window_threshold_and_wrapping() {
        let w_size = crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt + 10;

        assert_eq!(longest_match_limit(10, w_size), 0);
        assert_eq!(longest_match_limit(11, w_size), 1);
        assert_eq!(
            longest_match_limit(crate::stdlib::uInt::MAX, w_size),
            crate::stdlib::uInt::MAX.wrapping_sub(10),
        );
        assert_eq!(
            longest_match_limit(crate::stdlib::uInt::MAX, 0),
            crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt - 1,
        );
    }

    #[test]
    fn longest_match_search_parameters_preserve_initialization_and_adjustments() {
        assert_eq!(
            longest_match_search_parameters(1023, 3, 4, 100, 50),
            (1023, 50),
        );
        assert_eq!(
            longest_match_search_parameters(1023, 4, 4, 100, 50),
            (255, 50),
        );
        assert_eq!(longest_match_search_parameters(7, 0, 1, -1, 9), (7, 9),);
    }

    #[test]
    fn longest_match_next_chain_length_preserves_limit_and_wrapping_behavior() {
        assert_eq!(longest_match_next_chain_length(5, 5, 2), None);
        assert_eq!(longest_match_next_chain_length(4, 5, 2), None);
        assert_eq!(longest_match_next_chain_length(6, 5, 2), Some(1));
        assert_eq!(longest_match_next_chain_length(6, 5, 1), None);
        assert_eq!(
            longest_match_next_chain_length(6, 5, 0),
            Some(::core::ffi::c_uint::MAX),
        );
    }

    #[test]
    fn longest_match_candidate_update_accepts_only_longer_matches() {
        assert_eq!(
            longest_match_candidate_update(8, 7, 123, 8),
            Some((123, 8, true))
        );
        assert_eq!(longest_match_candidate_update(8, 8, 123, 8), None);
        assert_eq!(longest_match_candidate_update(7, 8, 123, 8), None);
    }

    #[test]
    fn longest_match_candidate_update_preserves_nice_match_boundary() {
        assert_eq!(
            longest_match_candidate_update(9, 8, 123, 10),
            Some((123, 9, false))
        );
        assert_eq!(
            longest_match_candidate_update(::core::ffi::c_int::MAX, -1, 0, -1),
            Some((0, ::core::ffi::c_int::MAX, true)),
        );
    }

    #[test]
    fn longest_match_clamp_length_preserves_unsigned_cast_and_lookahead_limit() {
        assert_eq!(longest_match_clamp_length(3, 8), 3);
        assert_eq!(longest_match_clamp_length(8, 8), 8);
        assert_eq!(longest_match_clamp_length(9, 8), 8);
        assert_eq!(longest_match_clamp_length(-1, 8), 8);
    }

    #[test]
    fn longest_match_core_follows_the_hash_chain_with_checked_slices() {
        let mut window = vec![0_u8; 700];
        let scan_start = 400usize;
        for offset in 0..crate::zutil_h::MAX_MATCH as usize {
            window[scan_start + offset] = (offset % 251) as u8;
        }
        let (candidates, scan_and_tail) = window.split_at_mut(scan_start);
        candidates[200..206].copy_from_slice(&scan_and_tail[..6]);
        candidates[206] = 255;
        candidates[180..192].copy_from_slice(&scan_and_tail[..12]);
        window[192] = 254;
        let mut prev = vec![0 as crate::src::deflate::Posf; 256];
        prev[200] = 180;

        assert_eq!(
            longest_match_core(
                &window,
                &prev,
                200,
                8,
                3,
                4,
                258,
                258,
                scan_start as crate::stdlib::uInt,
                256,
                255,
            ),
            Some(LongestMatchResult {
                match_start: 180,
                length: 12,
            })
        );
    }

    #[test]
    fn longest_match_core_rejects_out_of_range_storage() {
        assert_eq!(
            longest_match_core(&[0; 16], &[0; 1], 0, 1, 3, 4, 8, 8, 0, 1, 0),
            None
        );
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
    fn read_buf_input_progress_after_copy_preserves_input_and_total_wrapping() {
        assert_eq!(read_buf_input_progress_after_copy(12, 100, 5), (7, 105));
        assert_eq!(
            read_buf_input_progress_after_copy(0, crate::stdlib::uLong::MAX, 1),
            (crate::stdlib::uInt::MAX, 0)
        );
    }

    #[test]
    fn read_buf_checksum_selects_only_zlib_and_gzip_checksums() {
        assert_eq!(read_buf_checksum(1), Some(ReadBufChecksum::Adler32));
        assert_eq!(read_buf_checksum(2), Some(ReadBufChecksum::Crc32));
        assert_eq!(read_buf_checksum(0), None);
        assert_eq!(read_buf_checksum(-1), None);
        assert_eq!(read_buf_checksum(3), None);
    }

    #[test]
    fn read_buf_core_copies_clamped_input_and_updates_adler() {
        let input = [1, 2, 3, 4];
        let mut output = [0; 4];
        let adler = crate::src::adler32::adler32_z(1, &input[..3]);

        let result = read_buf_core(&input, &mut output, 3, 4, 10, 1, 1);

        assert_eq!(
            result,
            ReadBufResult {
                copied: 3,
                avail_in: 0,
                total_in: 13,
                adler,
            }
        );
        assert_eq!(output, [1, 2, 3, 0]);
    }

    #[test]
    fn read_buf_core_copies_clamped_input_and_updates_gzip_crc() {
        let input = [1, 2, 3, 4];
        let mut output = [0; 4];
        let crc = crate::src::crc32::crc32_z(0, &input[..3]);

        let result = read_buf_core(&input, &mut output, 4, 3, 10, 0, 2);

        assert_eq!(
            result,
            ReadBufResult {
                copied: 3,
                avail_in: 1,
                total_in: 13,
                adler: crc,
            }
        );
        assert_eq!(output, [1, 2, 3, 0]);
    }

    #[test]
    fn read_buf_core_preserves_adler_when_checksum_is_disabled() {
        let input = [9, 8];
        let mut output = [0; 2];

        let result = read_buf_core(&input, &mut output, 2, 2, 7, 99, 0);

        assert_eq!(
            result,
            ReadBufResult {
                copied: 2,
                avail_in: 0,
                total_in: 9,
                adler: 99,
            }
        );
        assert_eq!(output, input);
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
    fn fill_window_lookahead_after_read_preserves_wrapping_accounting() {
        assert_eq!(fill_window_lookahead_after_read(0, 0), 0);
        assert_eq!(fill_window_lookahead_after_read(12, 20), 32);
        assert_eq!(
            fill_window_lookahead_after_read(crate::stdlib::uInt::MAX, 1),
            0,
        );
    }

    #[test]
    fn fill_window_should_refill_preserves_loop_break_condition() {
        let min_lookahead = crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt;

        assert!(fill_window_should_refill(min_lookahead - 1, 1));
        assert!(!fill_window_should_refill(min_lookahead, 1));
        assert!(!fill_window_should_refill(0, 0));
    }

    #[test]
    fn fill_window_should_slide_preserves_threshold_and_wrapping() {
        let min_lookahead = crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt;
        let wsize: crate::stdlib::uInt = 32;
        let threshold = wsize.wrapping_add(wsize.wrapping_sub(min_lookahead));

        assert!(!fill_window_should_slide(threshold - 1, wsize));
        assert!(fill_window_should_slide(threshold, wsize));
        assert!(fill_window_should_slide(crate::stdlib::uInt::MAX, wsize));

        let wrapped_threshold = crate::stdlib::uInt::MAX
            .wrapping_add(crate::stdlib::uInt::MAX.wrapping_sub(min_lookahead));
        assert!(!fill_window_should_slide(
            wrapped_threshold.wrapping_sub(1),
            crate::stdlib::uInt::MAX,
        ));
        assert!(fill_window_should_slide(
            wrapped_threshold,
            crate::stdlib::uInt::MAX,
        ));
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
        assert_eq!(
            fill_window_insert_after_slide(crate::stdlib::uInt::MAX, crate::stdlib::uInt::MAX),
            crate::stdlib::uInt::MAX,
        );
    }

    #[test]
    fn fill_window_state_after_slide_rebases_and_clamps_scalar_state() {
        assert_eq!(
            fill_window_state_after_slide(80, 64, 20, 40, 32),
            (48, 32, -12, 32),
        );
        assert_eq!(
            fill_window_state_after_slide(32, 32, 0, 0, 32),
            (0, 0, -32, 0),
        );
    }

    #[test]
    fn fill_window_state_after_slide_preserves_wrapping_and_insert_clamping() {
        assert_eq!(
            fill_window_state_after_slide(
                0,
                0,
                ::core::ffi::c_long::MIN,
                crate::stdlib::uInt::MAX,
                1,
            ),
            (
                crate::stdlib::uInt::MAX,
                crate::stdlib::uInt::MAX,
                ::core::ffi::c_long::MAX,
                crate::stdlib::uInt::MAX,
            ),
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
    fn fill_window_hash_update_preserves_shift_xor_and_mask_order() {
        assert_eq!(fill_window_hash_update(0, 0, 5, 0xff), 0);
        assert_eq!(fill_window_hash_update(0x12, 0xab, 5, 0xff), 0xeb);
        assert_eq!(fill_window_hash_update(0xff, 0x34, 8, 0x7fff), 0x7f34);
        assert_eq!(fill_window_hash_update(0x1234, 0xffff, 4, 0), 0);
    }

    #[test]
    fn fill_window_reinsert_uses_checked_window_and_hash_indices() {
        let mut state = super::internal_state::newly_allocated();
        state.strstart = 2;
        state.lookahead = 3;
        state.insert = 2;
        state.w_mask = 3;
        state.hash_shift = 1;
        state.hash_mask = 7;
        let window = [1, 2, 3, 4, 5];
        let mut head = [0; 8];
        let mut prev = [0; 4];

        assert!(fill_window_reinsert(
            &mut state, &window, &mut head, &mut prev
        ));
        assert_eq!(state.insert, 0);
        assert_eq!(head[3], 0);
        assert_eq!(head[2], 1);
        assert_eq!(prev[0], 0);
        assert_eq!(prev[1], 0);

        state.insert = 1;
        state.strstart = 0;
        assert!(!fill_window_reinsert(
            &mut state, &window, &mut head, &mut prev
        ));
    }

    #[test]
    fn deflate_distance_tree_code_uses_the_checked_static_table_index() {
        for distance_minus_one in [0, 255, 256, 32767] {
            let table_index = if distance_minus_one < 256 {
                distance_minus_one as usize
            } else {
                256usize + (distance_minus_one as usize >> 7)
            };
            assert_eq!(
                deflate_distance_tree_code(distance_minus_one),
                crate::src::trees::_dist_code[table_index]
            );
        }
    }

    #[test]
    fn deflate_length_tree_index_uses_the_immutable_lookup_table() {
        for length_offset in [0, 1, 127, 255] {
            assert_eq!(
                deflate_length_tree_index(length_offset),
                crate::src::trees::_length_code[length_offset as usize] as usize
                    + crate::src::deflate::LITERALS as usize
                    + 1,
            );
        }
    }

    #[test]
    fn deflate_fast_match_codes_preserve_wrapping_and_narrowing() {
        assert_eq!(deflate_fast_match_codes(258, 1_000, 1), (255, 999));
        assert_eq!(
            deflate_fast_match_codes(2, 0, 1),
            (crate::zutil_h::uch::MAX, crate::zutil_h::ush::MAX),
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
    fn fill_window_slice_helpers_preserve_overlap_and_reject_bad_ranges() {
        let mut window = *b"0123456789abcdef";
        assert!(fill_window_slide(&mut window, 8, 2));
        assert_eq!(&window[..6], b"89abcd");
        assert_eq!(&window[6..], b"6789abcdef");
        assert!(!fill_window_slide(&mut window, 12, 0));
        assert_eq!(&window[..6], b"89abcd");

        assert!(fill_window_zero(&mut window, 4, 3));
        assert_eq!(&window[..8], b"89ab\0\0\07");
        let before = window;
        assert!(!fill_window_zero(&mut window, 15, 2));
        assert_eq!(window, before);
    }

    #[test]
    fn fill_window_core_updates_only_established_slice_views() {
        let mut state = super::internal_state::newly_allocated();
        state.w_size = 8;
        state.w_mask = 7;
        state.window_size = 16;
        state.hash_size = 1;
        state.wrap = 1;
        let input = *b"abc";
        let mut window = [0xff; 16];
        let mut head = [0; 1];
        let mut prev = [0; 8];

        let progress = super::fill_window_core(
            &mut state,
            &input,
            input.len() as crate::stdlib::uInt,
            5,
            1,
            &mut window,
            &mut head,
            &mut prev,
        );

        assert_eq!(
            progress,
            super::FillWindowInputProgress {
                avail_in: 0,
                total_in: 8,
                adler: crate::src::adler32::adler32_z(1, &input),
                consumed: input.len(),
            }
        );
        assert_eq!(&window[..3], &input);
        assert!(window[3..].iter().all(|byte| *byte == 0));
        assert_eq!(state.lookahead, input.len() as crate::stdlib::uInt);
        assert_eq!(state.high_water, 16);
    }

    #[test]
    fn fill_window_core_rejects_a_short_input_view_before_mutation() {
        let mut state = super::internal_state::newly_allocated();
        state.w_size = 8;
        state.window_size = 16;
        let original = state;
        let input = *b"abc";
        let mut window = [0xff; 16];
        let mut head = [0; 1];
        let mut prev = [0; 8];

        let progress = super::fill_window_core(
            &mut state,
            &input,
            4,
            5,
            1,
            &mut window,
            &mut head,
            &mut prev,
        );

        assert_eq!(
            progress,
            super::FillWindowInputProgress {
                avail_in: 4,
                total_in: 5,
                adler: 1,
                consumed: 0,
            }
        );
        assert_eq!(state.lookahead, original.lookahead);
        assert_eq!(state.high_water, original.high_water);
        assert_eq!(window, [0xff; 16]);
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
    fn deflate_bound_core_preserves_wrapper_and_configuration_choices() {
        let default_state = DeflateBoundState {
            wrap: 0,
            strstart: 0,
            w_bits: 15,
            hash_bits: 15,
            level: 6,
            gzip_header: None,
        };
        assert_eq!(deflate_bound_z_core(0, None), 25);
        assert_eq!(deflate_bound_z_core(100, Some(default_state)), 107);
        assert_eq!(
            deflate_bound_z_core(
                100,
                Some(DeflateBoundState {
                    wrap: 1,
                    strstart: 1,
                    ..default_state
                }),
            ),
            117,
        );
        assert_eq!(
            deflate_bound_z_core(
                1024,
                Some(DeflateBoundState {
                    w_bits: 14,
                    ..default_state
                }),
            ),
            1162,
        );
        assert_eq!(
            deflate_bound_z_core(
                1024,
                Some(DeflateBoundState {
                    w_bits: 14,
                    level: 0,
                    ..default_state
                }),
            ),
            1071,
        );
    }

    #[test]
    fn deflate_bound_core_counts_only_present_gzip_header_fields() {
        let header = DeflateBoundGzipHeader {
            has_extra: true,
            extra_len: 5,
            name_len: Some(3),
            comment_len: Some(2),
            has_header_crc: true,
        };
        let state = DeflateBoundState {
            wrap: 2,
            strstart: 0,
            w_bits: 15,
            hash_bits: 15,
            level: 6,
            gzip_header: Some(header),
        };
        assert_eq!(deflate_bound_z_core(0, Some(state)), 41);
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
    fn stored_block_header_bytes_preserves_rounding_and_wrapping() {
        assert_eq!(stored_block_header_bytes(0), 5);
        assert_eq!(stored_block_header_bytes(5), 5);
        assert_eq!(stored_block_header_bytes(6), 6);
        assert_eq!(stored_block_header_bytes(13), 6);
        assert_eq!(stored_block_header_bytes(14), 7);
        assert_eq!(stored_block_header_bytes(-1), 5);
    }

    #[test]
    fn stored_block_length_bytes_preserves_len_and_nlen_encoding() {
        assert_eq!(stored_block_length_bytes(0x0000), [0x00, 0x00, 0xff, 0xff]);
        assert_eq!(stored_block_length_bytes(0x0001), [0x01, 0x00, 0xfe, 0xff]);
        assert_eq!(stored_block_length_bytes(0x00ff), [0xff, 0x00, 0x00, 0xff]);
        assert_eq!(stored_block_length_bytes(0xff00), [0x00, 0xff, 0xff, 0x00]);
        assert_eq!(
            stored_block_length_bytes(crate::src::deflate::MAX_STORED as ::core::ffi::c_uint),
            [0xff, 0xff, 0x00, 0x00]
        );
    }

    #[test]
    fn stored_header_length_override_is_one_call_and_defaults_to_payload_length() {
        let mut state = super::internal_state::newly_allocated();

        assert_eq!(take_pending_header_len_override(&mut state, 7), 7);
        state.pending_header_len_override = Some(0x1234);
        assert_eq!(take_pending_header_len_override(&mut state, 7), 0x1234);
        assert_eq!(state.pending_header_len_override, None);
        assert_eq!(take_pending_header_len_override(&mut state, 7), 7);
    }

    #[test]
    fn stored_block_payload_len_limits_input_maximum_and_output_capacity() {
        assert_eq!(stored_block_payload_len(20, 30, 100), 50);
        assert_eq!(
            stored_block_payload_len(60_000, 10_000, 100_000),
            crate::src::deflate::MAX_STORED as u32
        );
        assert_eq!(stored_block_payload_len(60_000, 10_000, 4_096), 4_096);
        assert_eq!(
            stored_block_payload_len(
                ::core::ffi::c_uint::MAX,
                1,
                crate::src::deflate::MAX_STORED as u32,
            ),
            crate::src::deflate::MAX_STORED as u32,
        );
    }

    #[test]
    fn stored_block_copy_lengths_split_buffered_and_input_bytes() {
        assert_eq!(stored_block_copy_lengths(0, 8), (0, 8));
        assert_eq!(stored_block_copy_lengths(3, 8), (3, 5));
        assert_eq!(stored_block_copy_lengths(8, 8), (8, 0));
        assert_eq!(stored_block_copy_lengths(12, 8), (8, 0));
    }

    #[test]
    fn stored_block_copy_buffered_output_uses_checked_window_range() {
        let window = [10, 11, 12, 13, 14, 15];
        let mut output = [0; 3];
        assert!(stored_block_copy_buffered_output(
            &window,
            &mut output,
            2,
            3
        ));
        assert_eq!(output, [12, 13, 14]);

        let before = output;
        assert!(!stored_block_copy_buffered_output(
            &window,
            &mut output,
            -1,
            3
        ));
        assert!(!stored_block_copy_buffered_output(
            &window,
            &mut output,
            4,
            3
        ));
        assert_eq!(output, before);
    }

    #[test]
    fn stored_block_history_copies_consumed_input_and_slides_safely() {
        let state = StoredHistoryState {
            matches: 0,
            strstart: 2,
            insert: 2,
            block_start: 0,
        };
        let mut window = [0, 1, 2, 3, 4, 5, 6, 7];
        let result = stored_block_update_history(&mut window, &[9, 10], 2, 4, 8, state)
            .expect("valid no-slide history update");
        assert_eq!(&window[..4], &[0, 1, 9, 10]);
        assert_eq!(result.strstart, 4);
        assert_eq!(result.insert, 4);
        assert_eq!(result.matches, 0);
        assert_eq!(result.block_start, 4);

        let state = StoredHistoryState {
            matches: 1,
            strstart: 4,
            insert: 4,
            block_start: 2,
        };
        let result = stored_block_update_history(&mut window, &[20, 21, 22, 23], 4, 4, 8, state)
            .expect("valid sliding history update");
        assert_eq!(&window[..4], &[20, 21, 22, 23]);
        assert_eq!(result.strstart, 4);
        assert_eq!(result.insert, 4);
        assert_eq!(result.matches, 2);
        assert_eq!(result.block_start, 4);

        let result =
            stored_block_update_history(&mut window, &[30, 31, 32, 33, 34], 5, 4, 8, state)
                .expect("valid large-input history update");
        assert_eq!(&window[..4], &[31, 32, 33, 34]);
        assert_eq!(result.strstart, 4);
        assert_eq!(result.insert, 4);
        assert_eq!(result.matches, 2);
    }

    #[test]
    fn stored_block_history_rejects_short_window_without_mutation() {
        let state = StoredHistoryState {
            matches: 0,
            strstart: 4,
            insert: 4,
            block_start: 0,
        };
        let mut window = [0, 1, 2, 3];
        let before = window;
        assert_eq!(
            stored_block_update_history(&mut window, &[9, 10, 11, 12], 4, 4, 8, state),
            None
        );
        assert_eq!(window, before);
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
    fn stored_block_is_last_requires_finish_and_preserves_wrapping_input_total() {
        assert!(stored_block_is_last(crate::zlib_h::Z_FINISH, 7, 3, 4));
        assert!(!stored_block_is_last(crate::zlib_h::Z_FULL_FLUSH, 7, 3, 4,));
        assert!(!stored_block_is_last(crate::zlib_h::Z_FINISH, 6, 3, 4));
        assert!(stored_block_is_last(
            crate::zlib_h::Z_FINISH,
            0,
            ::core::ffi::c_uint::MAX,
            1,
        ));
    }

    #[test]
    fn stored_block_can_emit_preserves_threshold_and_flush_rules() {
        assert!(stored_block_can_emit(8, 8, crate::zlib_h::Z_NO_FLUSH, 3, 4,));
        assert!(stored_block_can_emit(4, 8, crate::zlib_h::Z_FINISH, 0, 4,));
        assert!(!stored_block_can_emit(
            4,
            8,
            crate::zlib_h::Z_NO_FLUSH,
            0,
            4,
        ));
        assert!(!stored_block_can_emit(4, 8, crate::zlib_h::Z_FINISH, 1, 4,));
        assert!(!stored_block_can_emit(
            5,
            8,
            crate::zlib_h::Z_FULL_FLUSH,
            0,
            4,
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
    fn deflate_flush_rank_preserves_flush_ordering() {
        assert_eq!(deflate_flush_rank(0), 0);
        assert_eq!(deflate_flush_rank(1), 2);
        assert_eq!(deflate_flush_rank(2), 4);
        assert_eq!(deflate_flush_rank(3), 6);
        assert_eq!(deflate_flush_rank(4), 8);
        assert_eq!(deflate_flush_rank(5), 1);
        assert_eq!(deflate_flush_rank(6), 3);
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
    fn deflate_preflight_preserves_error_precedence() {
        assert_eq!(
            deflate_preflight(true, 0, false, 0, 0, crate::zlib_h::Z_NO_FLUSH),
            DeflatePreflight::StreamError,
        );
        assert_eq!(
            deflate_preflight(false, 0, true, 0, 0, crate::zlib_h::Z_NO_FLUSH),
            DeflatePreflight::BufError,
        );
        assert_eq!(
            deflate_preflight(false, 0, true, 1, 0, crate::zlib_h::Z_NO_FLUSH),
            DeflatePreflight::Continue,
        );
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
    fn dictionary_tail_offset_preserves_window_tail_bounds() {
        assert_eq!(dictionary_tail_offset(32, 32), 0);
        assert_eq!(dictionary_tail_offset(33, 32), 1);
        assert_eq!(
            dictionary_tail_offset(crate::stdlib::uInt::MAX, 32),
            crate::stdlib::uInt::MAX.wrapping_sub(32) as usize,
        );
    }

    #[test]
    fn deflate_copy_prev_len_preserves_slide_clamp_and_wrapping() {
        assert_eq!(deflate_copy_prev_len(0, 10, 3, 16), 7);
        assert_eq!(deflate_copy_prev_len(0, 20, 3, 16), 16);
        assert_eq!(deflate_copy_prev_len(1, 10, 3, 16), 16);
        assert_eq!(deflate_copy_prev_len(0, 0, 1, 16), 16);
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
    #[test]
    fn deflate_state_is_usable_requires_callbacks_stream_identity_and_status() {
        assert!(deflate_state_is_usable(
            true,
            true,
            true,
            crate::src::deflate::BUSY_STATE,
        ));
        assert!(!deflate_state_is_usable(
            false,
            true,
            true,
            crate::src::deflate::BUSY_STATE,
        ));
        assert!(!deflate_state_is_usable(
            true,
            false,
            true,
            crate::src::deflate::BUSY_STATE,
        ));
        assert!(!deflate_state_is_usable(
            true,
            true,
            false,
            crate::src::deflate::BUSY_STATE,
        ));
        assert!(!deflate_state_is_usable(true, true, true, 0));
    }

    #[test]
    fn deflate_reset_status_and_adler_selects_wrapper_initial_state() {
        assert_eq!(
            deflate_reset_status_and_adler(2),
            (crate::src::deflate::GZIP_STATE, 0),
        );
        assert_eq!(
            deflate_reset_status_and_adler(1),
            (crate::src::deflate::INIT_STATE, 1),
        );
        assert_eq!(
            deflate_reset_status_and_adler(0),
            (crate::src::deflate::INIT_STATE, 1),
        );
    }

    #[test]
    fn with_pending_storage_limits_the_callback_to_the_pending_layout() {
        let layout = pending_storage_layout(2);
        let mut bytes = [0; 10];

        let result = crate::src::deflate::with_pending_storage(&mut bytes, layout, |storage| {
            storage.pending_bytes()[1] = 0x12;
            storage.symbol_bytes()[0] = 0x34;
            (storage.pending_bytes().len(), storage.symbol_bytes().len())
        });

        assert_eq!(result, Some((8, 6)));
        assert_eq!(bytes, [0, 0x12, 0x34, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn with_pending_storage_rejects_short_backing_storage() {
        let layout = pending_storage_layout(2);
        let mut bytes = [0; 7];

        assert_eq!(
            crate::src::deflate::with_pending_storage(&mut bytes, layout, |_| ()),
            None
        );
    }

    #[test]
    fn pending_storage_range_checks_offset_and_length_before_borrowing() {
        let layout = pending_storage_layout(2);
        let mut bytes = [0; 8];

        let result = crate::src::deflate::with_pending_storage(&mut bytes, layout, |storage| {
            storage
                .pending_range(1, 2)
                .unwrap()
                .copy_from_slice(&[0x12, 0x34]);
            (
                storage.pending_range(7, 2).is_none(),
                storage.pending_range(crate::zutil_h::ulg::MAX, 1).is_none(),
            )
        });

        assert_eq!(result, Some((true, true)));
        assert_eq!(bytes, [0, 0x12, 0x34, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn pending_storage_append_does_not_advance_cursor_when_range_is_invalid() {
        let layout = pending_storage_layout(2);
        let mut bytes = [0x55; 8];
        let mut pending = 7;

        let result = crate::src::deflate::with_pending_storage(&mut bytes, layout, |storage| {
            storage.append_pending(&mut pending, &[0x12, 0x34])
        });

        assert_eq!(result, Some(false));
        assert_eq!(pending, 7);
        assert_eq!(bytes, [0x55; 8]);
    }

    #[test]
    fn deflate_set_dictionary_allowed_requires_matching_wrapper_state() {
        assert!(deflate_set_dictionary_allowed(
            0,
            crate::src::deflate::BUSY_STATE,
            0
        ));
        assert!(deflate_set_dictionary_allowed(
            1,
            crate::src::deflate::INIT_STATE,
            0,
        ));
        assert!(!deflate_set_dictionary_allowed(
            2,
            crate::src::deflate::INIT_STATE,
            0,
        ));
        assert!(!deflate_set_dictionary_allowed(
            1,
            crate::src::deflate::BUSY_STATE,
            0,
        ));
        assert!(!deflate_set_dictionary_allowed(
            0,
            crate::src::deflate::INIT_STATE,
            1,
        ));
    }
}
