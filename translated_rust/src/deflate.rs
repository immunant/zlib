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
#[derive(Clone)]
#[repr(C)]

pub struct ct_data_s {
    pub fc: crate::src::deflate::C2Rust_Unnamed_1,
    pub dl: crate::src::deflate::C2Rust_Unnamed_0,
}
#[derive(Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub freq: crate::zutil_h::ush,
}
#[derive(Clone)]
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
#[derive(Clone)]
#[repr(C)]

pub struct tree_desc_s {
    pub max_code: ::core::ffi::c_int,
    pub stat_desc: StaticTreeKind,
}

pub type Pos = crate::zutil_h::ush;

pub type Posf = crate::src::deflate::Pos;

pub type IPos = ::core::ffi::c_uint;

pub type deflate_state = crate::src::deflate::internal_state;

#[derive(Clone)]
#[repr(C)]

pub struct internal_state {
    // This is an opaque, nullable identity for the ABI stream. Rust
    // implementation code compares identities only; the legacy FFI tree
    // bridge is the sole place that recreates the raw stream handle.
    pub strm: usize,
    // Preserve initializer-time allocator origin without carrying callback
    // pointers into the eventual owned-storage facade.
    pub(crate) allocator_provenance: crate::src::zutil::AllocatorProvenance,
    // Streams that installed zlib's complete default allocator pair own their
    // workspace outright. Custom and mixed allocator pairs retain the ABI
    // callback allocations below, so their allocation/free observations stay
    // exactly as supplied by the caller.
    owned_storage: Option<DeflateOwnedStorage>,
    pub status: ::core::ffi::c_int,
    pub pending_buf: *mut crate::stdlib::Bytef,
    pub pending_buf_size: crate::zutil_h::ulg,
    // These are offsets into `pending_buf`, not independently owned pointers.
    // Keeping interior cursors as indexes is the first step toward owned
    // pending storage while preserving the opaque C stream-state ABI.
    pub pending_out: usize,
    pub pending: crate::zutil_h::ulg,
    pub wrap: ::core::ffi::c_int,
    // `deflateSetHeader()` snapshots the caller-owned header at the FFI
    // boundary.  The resumable stream engine therefore never needs to walk
    // foreign header pointers while emitting a gzip member.
    gzhead: Option<DeflateGzipHeader>,
    // `deflateBound()` only needs these stable header lengths.  Keep that
    // metadata in the stream state so its ABI wrapper does not need to walk
    // the retained caller header.
    gzhead_bound_set: bool,
    gzhead_bound_has_extra: bool,
    gzhead_bound_extra_len: crate::stdlib::uInt,
    gzhead_bound_name_len: usize,
    gzhead_bound_comment_len: usize,
    gzhead_bound_hcrc: ::core::ffi::c_int,
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

/// The four separately allocated deflate work areas have a fixed relationship
/// to the scalar state configuration.  Keep that relationship in safe data so
/// the allocator boundary only performs allocation; a later owned-storage
/// conversion can replace these calls without rediscovering their layout.
struct DeflateStorageLayout {
    window_items: crate::stdlib::uInt,
    hash_items: crate::stdlib::uInt,
    pending_items: crate::stdlib::uInt,
}

/// The eventual safe owner for deflate's four work buffers.  The legacy
/// callback allocator still owns the live buffers today, but keeping their
/// checked Rust representation here lets the allocator facade switch storage
/// ownership without changing strategy code or recreating its geometry.
#[derive(Clone)]
struct DeflateOwnedStorage {
    window: Vec<crate::stdlib::Bytef>,
    prev: Vec<crate::src::deflate::Posf>,
    head: Vec<crate::src::deflate::Posf>,
    pending_buf: Vec<crate::stdlib::Bytef>,
}

/// Temporarily separate default-allocator storage from its containing state.
///
/// The opaque stream state must remain mutable while a streaming operation
/// mutates the owned vectors.  Taking the owner for the duration makes that
/// relationship explicit and guarantees that every return path puts the
/// workspace back.  The full default-allocator streaming call can grow this
/// scope later without reintroducing a raw workspace bridge.
fn with_owned_deflate_storage<R>(
    state: &mut crate::src::deflate::deflate_state,
    action: impl FnOnce(&mut crate::src::deflate::deflate_state, &mut DeflateOwnedStorage) -> R,
) -> Option<R> {
    let mut storage = state.owned_storage.take()?;
    let result = action(state, &mut storage);
    state.owned_storage = Some(storage);
    Some(result)
}

/// Run one strategy update through the default allocator's owned workspace.
///
/// This keeps the take/reinstall discipline in one place while publishing
/// only safe slices to the update path.  The wider streaming call can extend
/// this scope later without having to recreate the same owner juggling around
/// every header, flush, and strategy step.
fn with_owned_deflate_workspace<R>(
    state: &mut crate::src::deflate::deflate_state,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    action: impl FnOnce(&mut crate::src::deflate::deflate_state, &mut DeflateWorkspace<'_>) -> Option<R>,
) -> Option<R> {
    with_owned_deflate_storage(state, |state, owned| {
        let mut workspace = owned.workspace(state, input, output)?;
        action(state, &mut workspace)
    })
    .flatten()
}

impl DeflateOwnedStorage {
    /// Check that an owned workspace still has exactly the geometry advertised
    /// by a deflate state.  The eventual allocator facade must make this
    /// check before handing owned buffers to the legacy strategy engine: the
    /// scalar state remains resumable across calls, while the buffers are no
    /// longer raw allocation handles.
    fn matches_state(&self, state: &crate::src::deflate::deflate_state) -> bool {
        let storage = DeflateStorageLayout::from_state(state);
        let Some(window_bytes) = storage.window_bytes() else {
            return false;
        };
        let Some(pending_bytes) = storage.pending_byte_len() else {
            return false;
        };
        self.window.len() == window_bytes
            && self.prev.len() == storage.window_items as usize
            && self.head.len() == storage.hash_items as usize
            && self.pending_buf.len() == pending_bytes
            && usize::try_from(state.window_size).ok() == Some(window_bytes)
            && usize::try_from(state.pending_buf_size).ok() == Some(pending_bytes)
            && state.sym_buf == state.lit_bufsize as usize
    }

    /// Attach this owned allocation to an already-initialized deflate state.
    ///
    /// This is deliberately the one place where the eventual ownership
    /// facade recreates the legacy buffer handles.  It first proves that the
    /// state geometry and all four `Vec`s agree, so callers cannot derive a
    /// buffer view from a capacity advertised by an unrelated state.  The
    /// vectors are retained by `self` and are never resized after this handoff,
    /// keeping the established handles stable for the resumable legacy
    /// strategy engine.
    fn bind_state_buffers(&mut self, state: &mut crate::src::deflate::deflate_state) -> bool {
        if !self.matches_state(state) {
            return false;
        }
        state.window = self.window.as_mut_ptr();
        state.prev = self.prev.as_mut_ptr();
        state.head = self.head.as_mut_ptr();
        state.pending_buf = self.pending_buf.as_mut_ptr();
        true
    }

    /// Construct the pointer-free strategy view only after its resumable
    /// state metadata agrees with the owned buffers.  This is the direct
    /// hand-off the callback-preserving allocator facade will use; current
    /// live streams still enter through the reviewed raw allocation bridge.
    fn workspace<'a>(
        &'a mut self,
        state: &crate::src::deflate::deflate_state,
        input: &'a [crate::stdlib::Bytef],
        output: &'a mut [crate::stdlib::Bytef],
    ) -> Option<DeflateWorkspace<'a>> {
        self.matches_state(state)
            .then(|| DeflateWorkspace::from_owned(self, input, output))
    }

    /// Deep-copy the portions of a resumable deflate workspace that
    /// `deflateCopy` preserves.  This deliberately follows the legacy copy
    /// layout instead of cloning whole capacities: a later owned-storage
    /// facade can use it without recreating allocator-derived raw slices.
    fn try_copy_for_state(&self, state: &crate::src::deflate::deflate_state) -> Option<Self> {
        if !self.matches_state(state) {
            return None;
        }
        let copy = DeflateCopyLayout::from_state(state)?;
        let mut duplicate = DeflateStorageLayout::from_state(state).try_owned()?;

        duplicate
            .window
            .get_mut(..copy.window_bytes)?
            .copy_from_slice(self.window.get(..copy.window_bytes)?);
        duplicate
            .prev
            .get_mut(..copy.prev_items)?
            .copy_from_slice(self.prev.get(..copy.prev_items)?);
        duplicate
            .head
            .get_mut(..copy.head_items)?
            .copy_from_slice(self.head.get(..copy.head_items)?);

        let pending_end = copy.pending_offset.checked_add(copy.pending_bytes)?;
        duplicate
            .pending_buf
            .get_mut(copy.pending_offset..pending_end)?
            .copy_from_slice(self.pending_buf.get(copy.pending_offset..pending_end)?);
        let sym_end = copy.sym_offset.checked_add(copy.sym_bytes)?;
        duplicate
            .pending_buf
            .get_mut(copy.sym_offset..sym_end)?
            .copy_from_slice(self.pending_buf.get(copy.sym_offset..sym_end)?);
        Some(duplicate)
    }
}

/// All of the byte and item spans that `deflateCopy` must duplicate.  This is
/// deliberately pointer-free: the legacy allocation and copy boundary uses
/// these validated lengths, while a later owned-storage conversion can use
/// the same layout to create ordinary slices.
struct DeflateCopyLayout {
    window_bytes: usize,
    prev_items: usize,
    head_items: usize,
    pending_offset: usize,
    pending_bytes: usize,
    sym_offset: usize,
    sym_bytes: usize,
}

impl DeflateCopyLayout {
    fn from_state(state: &crate::src::deflate::deflate_state) -> Option<Self> {
        let storage = DeflateStorageLayout::from_state(state);
        let window_capacity = usize::try_from(storage.window_items).ok()?.checked_mul(2)?;
        let pending_capacity = usize::try_from(storage.pending_bytes()).ok()?;
        if state.window.is_null()
            || state.prev.is_null()
            || state.head.is_null()
            || state.pending_buf.is_null()
            || usize::try_from(state.window_size).ok()? != window_capacity
            || usize::try_from(state.pending_buf_size).ok()? != pending_capacity
        {
            return None;
        }

        let window_bytes = usize::try_from(state.high_water).ok()?;
        if window_bytes > window_capacity {
            return None;
        }
        let prior = state.strstart.wrapping_sub(state.insert);
        let prev_items = if state.slid != 0 || prior > state.w_size {
            state.w_size
        } else {
            prior
        } as usize;
        if prev_items > storage.window_items as usize {
            return None;
        }

        let pending_offset =
            pending_buffer_offset(state.pending_out, state.pending_buf_size, state.pending)?;
        let pending_bytes = usize::try_from(state.pending).ok()?;
        let sym_offset = state.sym_buf;
        let sym_bytes = usize::try_from(state.sym_next).ok()?;
        if sym_offset != state.lit_bufsize as usize {
            return None;
        }
        pending_buffer_range(state.sym_buf, state.sym_next, state.pending_buf_size)?;

        Some(Self {
            window_bytes,
            prev_items,
            head_items: storage.hash_items as usize,
            pending_offset,
            pending_bytes,
            sym_offset,
            sym_bytes,
        })
    }
}

impl DeflateStorageLayout {
    /// Derive the complete allocation geometry from validated initializer
    /// parameters.  Keeping this independent of raw state storage means the
    /// allocator boundary can consume one checked, pointer-free plan today,
    /// and an owned workspace can use the same plan later.
    fn from_init(window_bits: ::core::ffi::c_int, mem_level: ::core::ffi::c_int) -> Option<Self> {
        let window_bits = u32::try_from(window_bits).ok()?;
        let hash_bits = u32::try_from(mem_level.checked_add(7)?).ok()?;
        let pending_bits = u32::try_from(mem_level.checked_add(6)?).ok()?;
        let window_items = (1 as crate::stdlib::uInt).checked_shl(window_bits)?;
        let hash_items = (1 as crate::stdlib::uInt).checked_shl(hash_bits)?;
        let pending_items = (1 as crate::stdlib::uInt).checked_shl(pending_bits)?;
        Some(Self {
            window_items,
            hash_items,
            pending_items,
        })
    }

    fn from_state(state: &crate::src::deflate::deflate_state) -> Self {
        Self {
            window_items: state.w_size,
            hash_items: state.hash_size,
            pending_items: state.lit_bufsize,
        }
    }

    fn pending_bytes(&self) -> crate::zutil_h::ulg {
        (self.pending_items as crate::zutil_h::ulg).wrapping_mul(4)
    }

    fn window_bytes(&self) -> Option<usize> {
        usize::try_from(self.window_items).ok()?.checked_mul(2)
    }

    fn pending_byte_len(&self) -> Option<usize> {
        usize::try_from(self.pending_items).ok()?.checked_mul(4)
    }

    fn try_owned(&self) -> Option<DeflateOwnedStorage> {
        fn zeroed<T: Clone>(len: usize, value: T) -> Option<Vec<T>> {
            let mut storage = Vec::new();
            storage.try_reserve_exact(len).ok()?;
            storage.resize(len, value);
            Some(storage)
        }

        Some(DeflateOwnedStorage {
            window: zeroed(self.window_bytes()?, 0)?,
            prev: zeroed(usize::try_from(self.window_items).ok()?, 0)?,
            head: zeroed(usize::try_from(self.hash_items).ok()?, 0)?,
            pending_buf: zeroed(self.pending_byte_len()?, 0)?,
        })
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

/// Clear the full-flush hash table through the default allocator's owned
/// storage.  The owner is temporarily removed so the state can still be
/// passed to the established safe reset helper without aliasing it.
fn clear_owned_full_flush_hash(state: &mut crate::src::deflate::deflate_state) -> bool {
    let Some(mut owned) = state.owned_storage.take() else {
        return false;
    };
    let result = owned.matches_state(state) && clear_full_flush_hash(state, &mut owned.head);
    state.owned_storage = Some(owned);
    result
}

/// Borrow a custom-allocator hash table only for one typed operation.
///
/// Callback-owned deflate buffers still use ABI pointer handles, but reset
/// policy should not need to construct a raw slice itself.  This narrow
/// adapter is deliberately the single conversion point for the callback head
/// table; an eventual allocator owner can replace it without changing reset
/// behavior or its callers.
fn with_callback_deflate_head<R>(
    state: &mut crate::src::deflate::deflate_state,
    action: impl FnOnce(
        &mut crate::src::deflate::deflate_state,
        &mut [crate::src::deflate::Posf],
    ) -> R,
) -> Option<R> {
    if state.head.is_null() {
        return None;
    }
    let head = unsafe { ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize) };
    Some(action(state, head))
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
                state
                    .w_size
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
            state.ins_h =
                (state.ins_h << state.hash_shift ^ second as crate::stdlib::uInt) & state.hash_mask;
            while state.insert != 0 {
                let Some(&next) = window.get(
                    str.wrapping_add(3 as crate::stdlib::uInt)
                        .wrapping_sub(1 as crate::stdlib::uInt) as usize,
                ) else {
                    return;
                };
                state.ins_h = (state.ins_h << state.hash_shift ^ next as crate::stdlib::uInt)
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

pub enum DeflateInitMode {
    Zlib,
    Gzip { strategy: ::core::ffi::c_int },
}

struct DeflateInitConfig {
    level: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
    window_bits: ::core::ffi::c_int,
}

fn deflate_version_matches(version: &::core::ffi::c_char, stream_size: ::core::ffi::c_int) -> bool {
    *version as ::core::ffi::c_int
        == crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        && stream_size as usize == ::core::mem::size_of::<crate::zlib_h::z_stream>()
}

fn normalize_deflate_init_config(
    mut level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    mut window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Option<DeflateInitConfig> {
    if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        level = 6;
    }
    let mut wrap = 1;
    if window_bits < 0 {
        wrap = 0;
        if window_bits < -15 {
            return None;
        }
        window_bits = -window_bits;
    } else if window_bits > 15 {
        wrap = 2;
        window_bits -= 16;
    }
    if mem_level < 1
        || mem_level > crate::stdlib::MAX_MEM_LEVEL
        || method != crate::zlib_h::Z_DEFLATED
        || !(8..=15).contains(&window_bits)
        || !(0..=9).contains(&level)
        || !(0..=crate::zlib_h::Z_FIXED).contains(&strategy)
        || window_bits == 8 && wrap != 1
    {
        return None;
    }
    if window_bits == 8 {
        window_bits = 9;
    }
    Some(DeflateInitConfig {
        level,
        wrap,
        window_bits,
    })
}

fn initialize_deflate_state_base(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    allocator_provenance: crate::src::zutil::AllocatorProvenance,
    wrap: ::core::ffi::c_int,
    window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
) {
    state.strm = stream_identity(strm);
    state.allocator_provenance = allocator_provenance;
    state.status = crate::src::deflate::INIT_STATE;
    state.wrap = wrap;
    state.gzhead = None;
    state.gzhead_bound_set = false;
    state.w_bits = window_bits as crate::stdlib::uInt;
    state.w_size = (1 as crate::stdlib::uInt) << state.w_bits;
    state.w_mask = state.w_size.wrapping_sub(1);
    state.hash_bits = (mem_level as crate::stdlib::uInt).wrapping_add(7);
    state.hash_size = (1 as crate::stdlib::uInt) << state.hash_bits;
    state.hash_mask = state.hash_size.wrapping_sub(1);
    state.hash_shift = state
        .hash_bits
        .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
        .wrapping_sub(1)
        .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
}

fn stream_identity(strm: &crate::zlib_h::z_stream) -> usize {
    ::core::ptr::from_ref(strm).addr()
}

/// The C initializer zeroed its allocator-provided storage before configuring
/// it.  Construct the equivalent valid Rust value instead, so the named
/// allocation boundary never treats untyped bytes as an initialized state.
fn empty_deflate_state() -> crate::src::deflate::deflate_state {
    let empty_tree = || crate::src::deflate::ct_data_s {
        fc: crate::src::deflate::C2Rust_Unnamed_1 { freq: 0 },
        dl: crate::src::deflate::C2Rust_Unnamed_0 { len: 0 },
    };
    crate::src::deflate::deflate_state {
        strm: 0,
        allocator_provenance: crate::src::zutil::UNKNOWN_ALLOCATOR_PROVENANCE,
        owned_storage: None,
        status: 0,
        pending_buf: ::core::ptr::null_mut(),
        pending_buf_size: 0,
        pending_out: 0,
        pending: 0,
        wrap: 0,
        gzhead: None,
        gzhead_bound_set: false,
        gzhead_bound_has_extra: false,
        gzhead_bound_extra_len: 0,
        gzhead_bound_name_len: 0,
        gzhead_bound_comment_len: 0,
        gzhead_bound_hcrc: 0,
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
        dyn_ltree: ::core::array::from_fn(|_| empty_tree()),
        dyn_dtree: ::core::array::from_fn(|_| empty_tree()),
        bl_tree: ::core::array::from_fn(|_| empty_tree()),
        l_desc: crate::src::deflate::tree_desc_s {
            max_code: 0,
            stat_desc: 0,
        },
        d_desc: crate::src::deflate::tree_desc_s {
            max_code: 0,
            stat_desc: 0,
        },
        bl_desc: crate::src::deflate::tree_desc_s {
            max_code: 0,
            stat_desc: 0,
        },
        bl_count: [0; 16],
        heap: [0; 573],
        heap_len: 0,
        heap_max: 0,
        depth: [0; 573],
        sym_buf: 0,
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

/// The safe portion of configuring a freshly initialized deflate state.
///
/// Custom and mixed allocator workspaces remain behind
/// `deflate_reset_state`, its single existing raw hash-table view.
/// Fully default-allocated workspaces can reset directly through their owned
/// hash vector. Keeping that distinction here avoids rebuilding the same raw
/// view during initialization.
enum DeflateInitializationOutcome {
    ReadyForLegacyReset,
    Complete(::core::ffi::c_int),
}

fn configure_allocated_deflate_state(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    storage: &DeflateStorageLayout,
    config: &DeflateInitConfig,
    method: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    allocator_provenance: crate::src::zutil::AllocatorProvenance,
) -> Result<DeflateInitializationOutcome, ::core::ffi::c_int> {
    initialize_deflate_state_base(
        state,
        strm,
        allocator_provenance,
        config.wrap,
        config.window_bits,
        mem_level,
    );
    state.high_water = 0 as crate::zutil_h::ulg;
    state.lit_bufsize = storage.pending_items;
    state.pending_buf_size = storage.pending_bytes();
    state.window_size =
        (2 as crate::zutil_h::ulg).wrapping_mul(state.w_size as crate::zutil_h::ulg);
    state.sym_buf = state.lit_bufsize as usize;
    state.sym_end = state
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    if crate::src::zutil::allocator_pair_is_fully_default(&state.allocator_provenance) {
        let Some(mut owned) = storage.try_owned() else {
            return Err(crate::zlib_h::Z_MEM_ERROR);
        };
        if !owned.bind_state_buffers(state) {
            return Err(crate::zlib_h::Z_MEM_ERROR);
        }
        state.owned_storage = Some(owned);
    } else {
        state.window = Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
            storage.window_items,
            (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
                as crate::stdlib::uInt,
        ) as *mut crate::stdlib::Bytef;
        state.prev = Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
            storage.window_items,
            ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::Posf;
        state.head = Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
            storage.hash_items,
            ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::Posf;
        state.pending_buf = Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
            storage.pending_items,
            4 as crate::stdlib::uInt,
        ) as *mut crate::zutil_h::uchf as *mut crate::stdlib::Bytef;
    }
    if state.window.is_null()
        || state.prev.is_null()
        || state.head.is_null()
        || state.pending_buf.is_null()
    {
        return Err(crate::zlib_h::Z_MEM_ERROR);
    }
    state.level = config.level;
    state.strategy = strategy;
    state.method = method as crate::stdlib::Byte;
    if state.owned_storage.is_none() {
        return Ok(DeflateInitializationOutcome::ReadyForLegacyReset);
    }
    let Some(result) = with_owned_deflate_storage(state, |state, owned| {
        if owned.matches_state(state) {
            deflate_reset(strm, state, &mut owned.head)
        } else {
            crate::zlib_h::Z_STREAM_ERROR
        }
    }) else {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    };
    Ok(DeflateInitializationOutcome::Complete(result))
}

/// Allocate, initialize, and install the opaque deflate state through one
/// named implementation boundary. The stream assumes ownership only after
/// its ABI state field has been installed.
fn initialize_allocated_deflate_state(
    strm: &mut crate::zlib_h::z_stream,
    config: DeflateInitConfig,
    method: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    allocator_provenance: crate::src::zutil::AllocatorProvenance,
) -> ::core::ffi::c_int {
    let Some(storage) = DeflateStorageLayout::from_init(config.window_bits, mem_level) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    crate::src::zutil::with_callback_state_slot(strm, empty_deflate_state(), |strm, state| {
        strm.state = ::core::ptr::from_mut(state).cast::<crate::src::deflate::internal_state>();
        let outcome = configure_allocated_deflate_state(
            state,
            strm,
            &storage,
            &config,
            method,
            mem_level,
            strategy,
            allocator_provenance,
        );
        match outcome {
            Ok(DeflateInitializationOutcome::ReadyForLegacyReset) => {
                deflate_reset_state(strm, Some(state))
            }
            Ok(DeflateInitializationOutcome::Complete(result)) => result,
            Err(error) => {
                state.status = crate::src::deflate::FINISH_STATE;
                strm.msg = crate::src::zutil::zError(-4 as ::core::ffi::c_int)
                    .load(::core::sync::atomic::Ordering::Relaxed);
                deflateEnd(strm);
                error
            }
        }
    })
    .unwrap_or(crate::zlib_h::Z_MEM_ERROR)
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
    deflateInit2_(
        strm,
        level,
        crate::zlib_h::Z_DEFLATED,
        window_bits,
        mem_level,
        strategy,
        version,
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
    deflateInit_(
        strm.as_mut(),
        level,
        version.as_ref(),
        stream_size,
        DeflateInitMode::Zlib,
    )
}
pub fn deflateInit2_(
    strm: &mut crate::zlib_h::z_stream,
    level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    windowBits: ::core::ffi::c_int,
    memLevel: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    version: &::core::ffi::c_char,
    stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deflate_version_matches(version, stream_size) {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let allocator_provenance = crate::src::zutil::install_default_allocators(strm);
    let Some(config) = normalize_deflate_init_config(level, method, windowBits, memLevel, strategy)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    initialize_allocated_deflate_state(
        strm,
        config,
        method,
        memLevel,
        strategy,
        allocator_provenance,
    )
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
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(version) = version.as_ref() else {
        return crate::zlib_h::Z_VERSION_ERROR;
    };
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
        && state.strm == stream_identity(strm)
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

/// Call an operation with the opaque state after its caller has completed
/// operation-specific ABI checks.
///
/// Validation intentionally stays with each operation: their callback and
/// null-state checks have distinct required order.  This type-specific
/// boundary only creates the two disjoint mutable views needed by the legacy
/// state allocation, avoiding repeated opaque-state conversions.
fn with_deflate_stream_state<R>(
    stream: &mut crate::zlib_h::z_stream,
    action: impl FnOnce(
        &mut crate::zlib_h::z_stream,
        &mut crate::src::deflate::deflate_state,
    ) -> R,
) -> Option<R> {
    let state = stream.state as *mut crate::src::deflate::deflate_state;
    if state.is_null() {
        return None;
    }
    // The state allocation is separate from the caller-owned `z_stream`.
    // The validated opaque handle is only exposed for this synchronous call.
    unsafe { Some(action(stream, &mut *state)) }
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
            let count = state
                .lookahead
                .wrapping_sub((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt);
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
        || state.strm != stream_identity(strm)
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
    let state =
        strm.and_then(|strm| (strm.state as *const crate::src::deflate::deflate_state).as_ref());
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
    state.strm == stream_identity(strm)
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
    state.pending_out = 0;
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
    state.prev_length =
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
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

/// Reset an already-borrowed deflate state.
///
/// The FFI wrapper converts the opaque ABI state handle only after validating
/// the callback pair.  This core keeps all reset policy and the one remaining
/// callback-workspace borrowing boundary, while initialized Rust callers can
/// dispatch here without recreating their state reference.
pub(crate) fn deflate_reset_state(
    stream: &mut crate::zlib_h::z_stream,
    state: Option<&mut crate::src::deflate::deflate_state>,
) -> ::core::ffi::c_int {
    if !deflate_reset_keep_stream_valid(Some(stream)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(stream), Some(state)) || state.head.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Fully default-allocated streams retain their hash table in an owned
    // vector.  Borrow that vector directly for reset instead of recreating a
    // raw slice from the compatibility handle.  Take/reinstall the owner so
    // the state and the vector can be borrowed independently; the handle
    // stays stable for the legacy engine between calls.
    if state.owned_storage.is_some() {
        return with_owned_deflate_storage(state, |state, owned| {
            if owned.matches_state(state) {
                deflate_reset(stream, state, &mut owned.head)
            } else {
                crate::zlib_h::Z_STREAM_ERROR
            }
        })
        .unwrap_or(crate::zlib_h::Z_STREAM_ERROR);
    }
    // Custom and mixed allocator streams retain callback-owned storage. Keep
    // their one raw borrowing boundary in the named adapter until the
    // allocator facade can represent that ownership without changing callback
    // observations.
    with_callback_deflate_head(state, |state, head| deflate_reset(stream, state, head))
        .unwrap_or(crate::zlib_h::Z_STREAM_ERROR)
}

/// Reset a gzip-owned ABI stream that does not yet retain a typed state.
///
/// Gzip's public state still stores the deflate state as an opaque ABI handle,
/// so this is its one conversion boundary.  All reset policy remains in the
/// typed `deflate_reset_state` core shared with initialization and the FFI
/// entry point.
pub(crate) fn deflate_reset_legacy_stream(
    stream: &mut crate::zlib_h::z_stream,
) -> ::core::ffi::c_int {
    if !deflate_reset_keep_stream_valid(Some(stream)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    with_deflate_stream_state(stream, |stream, state| {
        deflate_reset_state(stream, Some(state))
    })
    .unwrap_or(crate::zlib_h::Z_STREAM_ERROR)
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
    if !deflate_reset_keep_stream_valid(Some(stream)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = (stream.state as *mut crate::src::deflate::deflate_state).as_mut();
    deflate_reset_state(stream, state)
}
fn deflate_set_header(
    state: &mut crate::src::deflate::deflate_state,
    head: DeflateGzipHeader,
    bound: DeflateBoundHeaderMetadata,
) -> ::core::ffi::c_int {
    if state.wrap != 2 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.gzhead = Some(head);
    state.gzhead_bound_set = true;
    state.gzhead_bound_has_extra = bound.has_extra;
    state.gzhead_bound_extra_len = bound.extra_len;
    state.gzhead_bound_name_len = bound.name_len;
    state.gzhead_bound_comment_len = bound.comment_len;
    state.gzhead_bound_hcrc = bound.hcrc;
    crate::zlib_h::Z_OK
}

fn deflate_clear_header(state: &mut crate::src::deflate::deflate_state) -> ::core::ffi::c_int {
    if state.wrap != 2 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.gzhead = None;
    state.gzhead_bound_set = false;
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
        let header = &mut *head;
        let name = if header.name.is_null() {
            None
        } else {
            Some(::std::ffi::CStr::from_ptr(header.name.cast()))
        };
        let comment = if header.comment.is_null() {
            None
        } else {
            Some(::std::ffi::CStr::from_ptr(header.comment.cast()))
        };
        let extra = if header.extra.is_null() {
            None
        } else {
            Some(::core::slice::from_raw_parts(
                header.extra,
                (header.extra_len & 0xffff as crate::stdlib::uInt) as usize,
            ))
        };
        let bound = deflate_bound_header_metadata(
            !header.extra.is_null(),
            header.extra_len,
            name,
            comment,
            header.hcrc,
        );
        let Some(head) = DeflateGzipHeader::snapshot(
            header.text,
            header.time,
            header.os,
            extra,
            name,
            comment,
            header.hcrc,
        ) else {
            return crate::zlib_h::Z_MEM_ERROR;
        };
        deflate_set_header(state, head, bound)
    }
}

struct DeflateBoundHeaderMetadata {
    has_extra: bool,
    extra_len: crate::stdlib::uInt,
    name_len: usize,
    comment_len: usize,
    hcrc: ::core::ffi::c_int,
}

fn deflate_bound_header_metadata(
    has_extra: bool,
    extra_len: crate::stdlib::uInt,
    name: Option<&::std::ffi::CStr>,
    comment: Option<&::std::ffi::CStr>,
    hcrc: ::core::ffi::c_int,
) -> DeflateBoundHeaderMetadata {
    DeflateBoundHeaderMetadata {
        has_extra,
        extra_len,
        name_len: name.map_or(0, |name| name.to_bytes_with_nul().len()),
        comment_len: comment.map_or(0, |comment| comment.to_bytes_with_nul().len()),
        hcrc,
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
    if state.strm != stream_identity(strm)
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
        || state.strm != stream_identity(strm)
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
    let pending_out = state.pending_out;
    let sym_buf = state.sym_buf;
    if pending_out > pending_buf.len() || sym_buf > pending_buf.len() {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    let Some(min_sym_buf) = pending_out.checked_add(
        ((crate::src::deflate::Buf_size + 7 as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int)
            as usize,
    ) else {
        return crate::zlib_h::Z_BUF_ERROR;
    };
    if bits < 0 as ::core::ffi::c_int || bits > 16 as ::core::ffi::c_int || sym_buf < min_sym_buf {
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
    // A malformed or partially initialized stream can retain a state without
    // its pending allocation.  Do not turn that absent storage into a slice:
    // the safe core owns the matching Z_STREAM_ERROR dispatch.
    if state.pending_buf.is_null() {
        return deflate_prime(Some(strm), Some(state), None, bits, value);
    }
    let pending_buf =
        ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize);
    deflate_prime(Some(strm), Some(state), Some(pending_buf), bits, value)
}
pub(crate) fn deflateParams(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    input: Option<&[crate::stdlib::Bytef]>,
    output: Option<&mut [crate::stdlib::Bytef]>,
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
        let err = deflate(strm, crate::zlib_h::Z_BLOCK, input, output);
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
    // The input cursor is borrowed only when the parameter transition needs
    // to flush a block.  A missing slice deliberately represents the same
    // malformed non-empty/null cursor that `deflate` reports as Z_STREAM_ERROR.
    let input = if strm.avail_in == 0 || strm.next_in.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts(
            strm.next_in,
            strm.avail_in as usize,
        ))
    };
    let output = if strm.next_out.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(
            strm.next_out,
            strm.avail_out as usize,
        ))
    };
    let hash_tables = if state.head.is_null() || state.prev.is_null() {
        None
    } else {
        Some((
            ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize),
            ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize),
        ))
    };
    deflateParams(strm, state, input, output, hash_tables, level, strategy)
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
fn deflate_bound_z(
    state: Option<&crate::src::deflate::deflate_state>,
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
            if state.gzhead_bound_set {
                if state.gzhead_bound_has_extra {
                    wraplen = wraplen.wrapping_add(
                        (2 as crate::stdlib::uInt).wrapping_add(state.gzhead_bound_extra_len)
                            as crate::stdlib::z_size_t,
                    );
                }
                wraplen = wraplen.wrapping_add(state.gzhead_bound_name_len);
                wraplen = wraplen.wrapping_add(state.gzhead_bound_comment_len);
                if state.gzhead_bound_hcrc != 0 {
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
    deflate_bound_z(state, sourceLen)
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
    deflate_bound_result(deflate_bound_z(state, sourceLen as crate::stdlib::z_size_t))
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
    } else if state.strategy >= 2 as ::core::ffi::c_int || state.level < 2 as ::core::ffi::c_int {
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
    state.pending = state
        .pending
        .wrapping_add(header.len() as crate::zutil_h::ulg);
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

#[derive(Clone)]
struct DeflateGzipHeader {
    text: ::core::ffi::c_int,
    time: crate::stdlib::uLong,
    os: ::core::ffi::c_int,
    extra: Option<Vec<crate::stdlib::Bytef>>,
    name: Option<Vec<crate::stdlib::Bytef>>,
    comment: Option<Vec<crate::stdlib::Bytef>>,
    hcrc: ::core::ffi::c_int,
}

impl DeflateGzipHeader {
    fn snapshot(
        text: ::core::ffi::c_int,
        time: crate::stdlib::uLong,
        os: ::core::ffi::c_int,
        extra: Option<&[crate::stdlib::Bytef]>,
        name: Option<&::std::ffi::CStr>,
        comment: Option<&::std::ffi::CStr>,
        hcrc: ::core::ffi::c_int,
    ) -> Option<Self> {
        fn copy(bytes: &[crate::stdlib::Bytef]) -> Option<Vec<crate::stdlib::Bytef>> {
            let mut owned = Vec::new();
            owned.try_reserve_exact(bytes.len()).ok()?;
            owned.extend_from_slice(bytes);
            Some(owned)
        }

        Some(Self {
            text,
            time,
            os,
            extra: match extra {
                Some(extra) => Some(copy(extra)?),
                None => None,
            },
            name: match name {
                Some(name) => Some(copy(name.to_bytes_with_nul())?),
                None => None,
            },
            comment: match comment {
                Some(comment) => Some(copy(comment.to_bytes_with_nul())?),
                None => None,
            },
            hcrc,
        })
    }
}

fn write_gzip_header_fixed(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    header: &DeflateGzipHeader,
) -> bool {
    let Ok(start) = usize::try_from(state.pending) else {
        return false;
    };
    let flags = (if header.text != 0 { 1 } else { 0 })
        + (if header.hcrc != 0 { 2 } else { 0 })
        + (if header.extra.is_none() { 0 } else { 4 })
        + (if header.name.is_none() { 0 } else { 8 })
        + (if header.comment.is_none() { 0 } else { 16 });
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
    let length = if header.extra.is_none() { 7 } else { 9 };
    if let Some(extra) = header.extra.as_deref() {
        let extra_len = extra.len() as crate::stdlib::uInt;
        fixed[7] = extra_len as crate::stdlib::Bytef;
        fixed[8] = (extra_len >> 8) as crate::stdlib::Bytef;
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
    state.pending = state
        .pending
        .wrapping_add(trailer.len() as crate::zutil_h::ulg);
    true
}

fn write_zlib_header(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    pending_buf: &mut [crate::stdlib::Bytef],
) -> bool {
    let mut header: crate::stdlib::uInt =
        (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt).wrapping_add(
            state.w_bits.wrapping_sub(8 as crate::stdlib::uInt) << 4 as ::core::ffi::c_int,
        ) << 8 as ::core::ffi::c_int;
    let level_flags = if state.strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || state.level < 2 {
        0
    } else if state.level < 6 {
        1
    } else if state.level == 6 {
        2
    } else {
        3
    };
    header |= level_flags << 6 as ::core::ffi::c_int;
    let dictionary = state.strstart != 0;
    if dictionary {
        header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
    }
    header = header.wrapping_add(
        (31 as crate::stdlib::uInt).wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
    );
    if !put_short_msb(state, pending_buf, header) {
        return false;
    }
    if dictionary
        && (!put_short_msb(
            state,
            pending_buf,
            (strm.adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
        ) || !put_short_msb(
            state,
            pending_buf,
            (strm.adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
        ))
    {
        return false;
    }
    strm.adler = crate::src::adler32::adler32(0 as crate::stdlib::uLong, None);
    state.status = crate::src::deflate::BUSY_STATE;
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
    state.pending_out = source_end;
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out -= len as crate::stdlib::uInt;
    state.pending -= len as crate::zutil_h::ulg;
    if state.pending == 0 {
        state.pending_out = 0;
    }
    true
}

fn flush_pending(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) {
    if state.pending_out > pending_buf.len() || output.len() != strm.avail_out as usize {
        return;
    }
    let pending_start = state.pending_out;
    let _ = flush_pending_impl(strm, state, pending_buf, pending_start, output);
}

fn output_tail<'a>(
    strm: &crate::zlib_h::z_stream,
    output: &'a mut [crate::stdlib::Bytef],
) -> Option<&'a mut [crate::stdlib::Bytef]> {
    if strm.avail_out == 0 {
        return Some(&mut []);
    }
    if strm.next_out.is_null() {
        return None;
    }
    let start = strm.next_out.addr().checked_sub(output.as_ptr().addr())?;
    let end = start.checked_add(strm.avail_out as usize)?;
    output.get_mut(start..end)
}

// A single borrowed view of all storage a deflate strategy may touch.  The
// legacy engine constructs this only after validating its ABI allocations;
// strategy dispatch and the strategies themselves consequently need no raw
// storage handles.  Keeping this pointer-free view separate from
// `deflate_state` is the bridge to replacing the legacy allocations with
// owned buffers.
struct DeflateWorkspace<'a> {
    window: &'a mut [crate::stdlib::Bytef],
    head: Option<&'a mut [crate::src::deflate::Posf]>,
    prev: Option<&'a mut [crate::src::deflate::Posf]>,
    pending_buf: &'a mut [crate::stdlib::Bytef],
    input: &'a [crate::stdlib::Bytef],
    output: &'a mut [crate::stdlib::Bytef],
}

impl<'a> DeflateWorkspace<'a> {
    /// Build the same strategy view from safe-owned storage.  The legacy
    /// engine still constructs this view from validated allocator storage;
    /// the future allocator facade can use this constructor directly.
    fn from_owned(
        storage: &'a mut DeflateOwnedStorage,
        input: &'a [crate::stdlib::Bytef],
        output: &'a mut [crate::stdlib::Bytef],
    ) -> Self {
        let DeflateOwnedStorage {
            window,
            prev,
            head,
            pending_buf,
        } = storage;
        Self {
            window,
            head: Some(head),
            prev: Some(prev),
            pending_buf,
            input,
            output,
        }
    }
}

// The legacy stream engine owns the one raw-to-slice conversion for this
// update. Keep strategy selection here, over ordinary Rust borrows, so the
// compression modes cannot grow their own raw stream paths.
fn deflate_update(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    workspace: &mut DeflateWorkspace<'_>,
    flush: ::core::ffi::c_int,
) -> Option<block_state> {
    if state.level == 0 {
        return Some(deflate_stored(
            state,
            strm,
            &mut *workspace.window,
            workspace.input,
            &mut *workspace.pending_buf,
            &mut *workspace.output,
            flush,
        ));
    }
    if state.strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
        let (Some(head), Some(prev)) =
            (workspace.head.as_deref_mut(), workspace.prev.as_deref_mut())
        else {
            return None;
        };
        return Some(deflate_huff(
            state,
            strm,
            &mut *workspace.window,
            head,
            prev,
            workspace.input,
            &mut *workspace.pending_buf,
            &mut *workspace.output,
            flush,
        ));
    }
    if state.strategy == crate::zlib_h::Z_RLE {
        let (Some(head), Some(prev)) =
            (workspace.head.as_deref_mut(), workspace.prev.as_deref_mut())
        else {
            return None;
        };
        return Some(deflate_rle(
            state,
            strm,
            &mut *workspace.window,
            head,
            prev,
            workspace.input,
            &mut *workspace.pending_buf,
            &mut *workspace.output,
            flush,
        ));
    }
    match &configuration_table.get(state.level as usize)?.func {
        CompressorKind::Stored => Some(deflate_stored(
            state,
            strm,
            &mut *workspace.window,
            workspace.input,
            &mut *workspace.pending_buf,
            &mut *workspace.output,
            flush,
        )),
        CompressorKind::Fast => {
            let (Some(head), Some(prev)) =
                (workspace.head.as_deref_mut(), workspace.prev.as_deref_mut())
            else {
                return None;
            };
            Some(deflate_fast(
                state,
                strm,
                &mut *workspace.window,
                head,
                prev,
                workspace.input,
                &mut *workspace.pending_buf,
                &mut *workspace.output,
                flush,
            ))
        }
        CompressorKind::Slow => {
            let (Some(head), Some(prev)) =
                (workspace.head.as_deref_mut(), workspace.prev.as_deref_mut())
            else {
                return None;
            };
            Some(deflate_slow(
                state,
                strm,
                &mut *workspace.window,
                head,
                prev,
                workspace.input,
                &mut *workspace.pending_buf,
                &mut *workspace.output,
                flush,
            ))
        }
    }
}

/// Construct one typed view of a callback-owned deflate workspace.
///
/// Default-pair streams use `DeflateOwnedStorage` and never enter this
/// bridge.  The legacy streaming engine converts callback-owned allocation
/// handles to these typed views; this constructor and the strategy dispatcher
/// only operate on already-borrowed storage.  A future callback-storage owner
/// can replace that boundary without changing the dispatcher or strategies.
fn callback_deflate_workspace<'a>(
    window: &'a mut [crate::stdlib::Bytef],
    head: Option<&'a mut [crate::src::deflate::Posf]>,
    prev: Option<&'a mut [crate::src::deflate::Posf]>,
    input: &'a [crate::stdlib::Bytef],
    output: &'a mut [crate::stdlib::Bytef],
    pending_buf: &'a mut [crate::stdlib::Bytef],
) -> DeflateWorkspace<'a> {
    DeflateWorkspace {
        window,
        head,
        prev,
        pending_buf,
        input,
        output,
    }
}

/// Run one update against the typed buffers supplied by a custom allocator.
///
/// The legacy streaming engine performs the raw callback-storage conversion;
/// this dispatcher retains only the existing typed update behavior.
fn update_callback_deflate_workspace(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    workspace: &mut DeflateWorkspace<'_>,
    flush: ::core::ffi::c_int,
) -> Option<DeflateUpdateResult> {
    let bstate = deflate_update(state, strm, workspace, flush)?;
    let block_handled = bstate as ::core::ffi::c_uint
        == block_done as ::core::ffi::c_int as ::core::ffi::c_uint;
    if block_handled
        && !finish_callback_deflate_block(
            state,
            workspace.pending_buf,
            workspace.head.as_deref_mut(),
            flush,
        )
    {
        return None;
    }
    Some(DeflateUpdateResult {
        bstate,
        block_handled,
    })
}

/// Complete a callback-owned update while its checked workspace is still
/// borrowed.  In particular, a full flush clears the same hash-table view
/// that the strategy update used, so the legacy engine does not need to
/// recreate a separate raw head-table slice afterward.
fn finish_callback_deflate_block(
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    head: Option<&mut [crate::src::deflate::Posf]>,
    flush: ::core::ffi::c_int,
) -> bool {
    if flush == crate::zlib_h::Z_PARTIAL_FLUSH {
        crate::src::trees::_tr_align(state, pending_buf);
    } else if flush != crate::zlib_h::Z_BLOCK {
        crate::src::trees::tr_stored_block(
            state,
            pending_buf,
            None,
            0 as crate::zutil_h::ulg,
            0 as ::core::ffi::c_int,
        );
        if flush == crate::zlib_h::Z_FULL_FLUSH {
            let Some(head) = head else {
                return false;
            };
            if !clear_full_flush_hash(state, head) {
                return false;
            }
        }
    }
    true
}

struct DeflateUpdateResult {
    bstate: block_state,
    block_handled: bool,
}

/// Borrow a callback-owned workspace for exactly one deflate update.
///
/// Default-allocator streams keep their buffers in `DeflateOwnedStorage` and
/// never enter this boundary.  Custom and mixed allocator buffers retain ABI
/// handles, so form all of their typed views together here after `deflate`
/// has validated the stream and pending buffer.  This leaves the streaming
/// engine with a named, pointer-free update operation and gives a future
/// callback-storage owner one replacement point.
fn with_callback_deflate_workspace<R>(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    pending_buf: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
    action: impl FnOnce(
        &mut crate::src::deflate::deflate_state,
        &mut crate::zlib_h::z_stream,
        &mut DeflateWorkspace<'_>,
        ::core::ffi::c_int,
    ) -> Option<R>,
) -> Option<R> {
    if state.window.is_null() {
        return None;
    }
    // Callback-owned allocations remain foreign storage.  Keep their three
    // raw-to-slice conversions together at this type-specific boundary.
    let (window, head, prev) = unsafe {
        (
            ::core::slice::from_raw_parts_mut(state.window, state.window_size as usize),
            (!state.head.is_null())
                .then(|| ::core::slice::from_raw_parts_mut(state.head, state.hash_size as usize)),
            (!state.prev.is_null())
                .then(|| ::core::slice::from_raw_parts_mut(state.prev, state.w_size as usize)),
        )
    };
    let mut workspace = callback_deflate_workspace(window, head, prev, input, output, pending_buf);
    action(state, strm, &mut workspace, flush)
}

/// Emit and drain the initial zlib wrapper through already-borrowed buffers.
///
/// The legacy stream engine still supplies the pending/output views, but the
/// wrapper transition itself is ordinary slice-based state-machine work.  An
/// owned pending-buffer call path can reuse this without recreating a raw
/// view of `state.pending_buf`.
fn initialize_deflate_wrapper(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    pending_buf: &mut [crate::stdlib::Bytef],
    output_buffer: &mut [crate::stdlib::Bytef],
) -> Result<Option<::core::ffi::c_int>, ()> {
    if state.status == crate::src::deflate::INIT_STATE
        && state.wrap == 0 as ::core::ffi::c_int
    {
        state.status = crate::src::deflate::BUSY_STATE;
    }
    if state.status != crate::src::deflate::INIT_STATE {
        return Ok(None);
    }
    if !write_zlib_header(state, strm, pending_buf) {
        return Err(());
    }
    let output = output_tail(strm, output_buffer).ok_or(())?;
    flush_pending(strm, state, pending_buf, output);
    if state.pending != 0 as crate::zutil_h::ulg {
        state.last_flush = -1 as ::core::ffi::c_int;
        return Ok(Some(crate::zlib_h::Z_OK));
    }
    Ok(None)
}

pub fn deflate(
    strm: &mut crate::zlib_h::z_stream,
    mut flush: ::core::ffi::c_int,
    input: Option<&[crate::stdlib::Bytef]>,
    output: Option<&mut [crate::stdlib::Bytef]>,
) -> ::core::ffi::c_int {
    // The exported wrapper and internal callers provide a live stream
    // reference. Keep the translated raw-state implementation below local
    // until stream ownership is converted.
    let mut old_flush: ::core::ffi::c_int = 0;
    if strm.zalloc.is_none()
        || strm.zfree.is_none()
        || strm.state.is_null()
        || flush > crate::zlib_h::Z_BLOCK
        || flush < 0 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The stream/state handle has now passed the ABI validation above. Keep
    // the raw handle conversion inside its own legacy boundary.
    let Some(state) = (unsafe { (strm.state as *mut crate::src::deflate::deflate_state).as_mut() })
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(strm), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if strm.avail_in != 0 as crate::stdlib::uInt
            && (strm.next_in.is_null()
                || input.map_or(true, |input| input.len() != strm.avail_in as usize))
        || output.as_ref().map_or(true, |output| {
            output.len() != strm.avail_out as usize
                || output.as_ptr() != strm.next_out
        })
        || state.status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH
    {
        strm.msg = crate::src::zutil::zError(-2 as ::core::ffi::c_int)
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -2 as ::core::ffi::c_int;
    }
    if strm.avail_out == 0 as crate::stdlib::uInt {
        strm.msg = crate::src::zutil::zError(-5 as ::core::ffi::c_int)
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -5 as ::core::ffi::c_int;
    }
    // `pending_buf` is allocated with the deflate state and remains stable for
    // the whole call.  Form its checked Rust view once at this legacy storage
    // boundary instead of rebuilding raw views for every header, flush, and
    // update path below.
    if state.pending_buf.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut pending_buffer = unsafe {
        ::core::slice::from_raw_parts_mut(state.pending_buf, state.pending_buf_size as usize)
    };
    let mut output_buffer = output.expect("validated deflate output");
    old_flush = state.last_flush;
    state.last_flush = flush;
    if state.pending != 0 as crate::zutil_h::ulg {
        let Some(output) = output_tail(strm, &mut output_buffer) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        flush_pending(strm, state, &mut pending_buffer, output);
        if strm.avail_out == 0 as crate::stdlib::uInt {
            state.last_flush = -1 as ::core::ffi::c_int;
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
        strm.msg = crate::src::zutil::zError(-5 as ::core::ffi::c_int)
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -5 as ::core::ffi::c_int;
    }
    if state.status == crate::src::deflate::FINISH_STATE
        && strm.avail_in != 0 as crate::stdlib::uInt
    {
        strm.msg = crate::src::zutil::zError(-5 as ::core::ffi::c_int)
            .load(::core::sync::atomic::Ordering::Relaxed);
        return -5 as ::core::ffi::c_int;
    }
    match initialize_deflate_wrapper(state, strm, &mut pending_buffer, &mut output_buffer) {
        Ok(Some(result)) => return result,
        Ok(None) => {}
        Err(()) => return crate::zlib_h::Z_STREAM_ERROR,
    }
    // A custom header is copied at `deflateSetHeader`'s ABI boundary.  Header
    // emission below consequently uses only owned Rust data and never needs
    // to dereference a caller buffer while a stream is resumable.
    // Keep the snapshot independent from the mutable stream state during the
    // resumable header state machine below.  The original snapshot remains
    // retained in `state` for subsequent calls and `deflateCopy`; once header
    // emission is complete, avoid cloning it on ordinary compression calls.
    let gzip_header = if matches!(
        state.status,
        crate::src::deflate::GZIP_STATE
            | crate::src::deflate::EXTRA_STATE
            | crate::src::deflate::NAME_STATE
            | crate::src::deflate::COMMENT_STATE
            | crate::src::deflate::HCRC_STATE
    ) {
        state.gzhead.clone()
    } else {
        None
    };
    // Gzip header emission may resume in any of these phases.  Keep its
    // pending-buffer conversion at this one legacy boundary, then reborrow
    // the checked slice for each phase below.
    let mut gzip_pending_buf = if state.status == crate::src::deflate::GZIP_STATE
        || state.status == crate::src::deflate::EXTRA_STATE
        || state.status == crate::src::deflate::NAME_STATE
        || state.status == crate::src::deflate::COMMENT_STATE
        || state.status == crate::src::deflate::HCRC_STATE
    {
        Some(&mut pending_buffer[..])
    } else {
        None
    };
    if state.status == crate::src::deflate::GZIP_STATE {
        strm.adler = crate::src::crc32::crc32(0 as crate::stdlib::uLong, None);
        let Some(pending_buf) = gzip_pending_buf.as_deref_mut() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if !write_gzip_header_magic(state, pending_buf) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if gzip_header.is_none() {
            if !write_default_gzip_header_tail(state, pending_buf) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            let Some(output) = output_tail(strm, &mut output_buffer) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            flush_pending(strm, state, pending_buf, output);
            if state.pending != 0 as crate::zutil_h::ulg {
                state.last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        } else if let Some(header) = gzip_header.as_ref() {
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
                strm.adler = crate::src::crc32::crc32_z(strm.adler, Some(header_bytes));
            }
            state.gzindex = 0 as crate::zutil_h::ulg;
            state.status = crate::src::deflate::EXTRA_STATE;
        }
    }
    if state.status == crate::src::deflate::EXTRA_STATE {
        let Some(header) = gzip_header.as_ref() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if let Some(extra) = header.extra.as_deref() {
            let extra_len = extra.len();
            let Some(pending_buf) = gzip_pending_buf.as_deref_mut() else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
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
                    if !checksum_gzip_header_range(&mut strm.adler, pending_buf, beg, state.pending)
                    {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    }
                }
                let Some(output) = output_tail(strm, &mut output_buffer) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                flush_pending(strm, state, pending_buf, output);
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
                if !checksum_gzip_header_range(&mut strm.adler, pending_buf, beg, state.pending) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
            }
            state.gzindex = 0 as crate::zutil_h::ulg;
        }
        state.status = crate::src::deflate::NAME_STATE;
    }
    if state.status == crate::src::deflate::NAME_STATE {
        let Some(header) = gzip_header.as_ref() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if let Some(name) = header.name.as_deref() {
            let Some(pending_buf) = gzip_pending_buf.as_deref_mut() else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let hcrc = header.hcrc != 0;
            let mut beg_0: crate::zutil_h::ulg = state.pending;
            loop {
                if state.pending == state.pending_buf_size {
                    if hcrc && state.pending > beg_0 {
                        if !checksum_gzip_header_range(
                            &mut strm.adler,
                            pending_buf,
                            beg_0,
                            state.pending,
                        ) {
                            return crate::zlib_h::Z_STREAM_ERROR;
                        }
                    }
                    let Some(output) = output_tail(strm, &mut output_buffer) else {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    };
                    flush_pending(strm, state, pending_buf, output);
                    if state.pending != 0 as crate::zutil_h::ulg {
                        state.last_flush = -1 as ::core::ffi::c_int;
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
            if hcrc && state.pending > beg_0 {
                if !checksum_gzip_header_range(&mut strm.adler, pending_buf, beg_0, state.pending) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
            }
            state.gzindex = 0 as crate::zutil_h::ulg;
        }
        state.status = crate::src::deflate::COMMENT_STATE;
    }
    if state.status == crate::src::deflate::COMMENT_STATE {
        let Some(header) = gzip_header.as_ref() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if let Some(comment) = header.comment.as_deref() {
            let Some(pending_buf) = gzip_pending_buf.as_deref_mut() else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let hcrc = header.hcrc != 0;
            let mut beg_1: crate::zutil_h::ulg = state.pending;
            loop {
                if state.pending == state.pending_buf_size {
                    if hcrc && state.pending > beg_1 {
                        if !checksum_gzip_header_range(
                            &mut strm.adler,
                            pending_buf,
                            beg_1,
                            state.pending,
                        ) {
                            return crate::zlib_h::Z_STREAM_ERROR;
                        }
                    }
                    let Some(output) = output_tail(strm, &mut output_buffer) else {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    };
                    flush_pending(strm, state, pending_buf, output);
                    if state.pending != 0 as crate::zutil_h::ulg {
                        state.last_flush = -1 as ::core::ffi::c_int;
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
            if hcrc && state.pending > beg_1 {
                if !checksum_gzip_header_range(&mut strm.adler, pending_buf, beg_1, state.pending) {
                    return crate::zlib_h::Z_STREAM_ERROR;
                }
            }
        }
        state.status = crate::src::deflate::HCRC_STATE;
    }
    if state.status == crate::src::deflate::HCRC_STATE {
        let Some(header) = gzip_header.as_ref() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Some(pending_buf) = gzip_pending_buf.as_deref_mut() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if header.hcrc != 0 {
            if state.pending.wrapping_add(2 as crate::zutil_h::ulg) > state.pending_buf_size {
                let Some(output) = output_tail(strm, &mut output_buffer) else {
                    return crate::zlib_h::Z_STREAM_ERROR;
                };
                flush_pending(strm, state, pending_buf, output);
                if state.pending != 0 as crate::zutil_h::ulg {
                    state.last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
            if !write_gzip_header_crc(state, pending_buf, strm.adler) {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            strm.adler = crate::src::crc32::crc32(0 as crate::stdlib::uLong, None);
        }
        state.status = crate::src::deflate::BUSY_STATE;
        let Some(output) = output_tail(strm, &mut output_buffer) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        flush_pending(strm, state, pending_buf, output);
        if state.pending != 0 as crate::zutil_h::ulg {
            state.last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    // The resumable gzip phases above may borrow the pending buffer through
    // this option. Release that borrow before the update and finish paths use
    // the same checked view.
    drop(gzip_pending_buf);
    if strm.avail_in != 0 as crate::stdlib::uInt
        || state.lookahead != 0 as crate::stdlib::uInt
        || flush != crate::zlib_h::Z_NO_FLUSH && state.status != crate::src::deflate::FINISH_STATE
    {
        // The caller input remains an ABI borrow, but the default allocator's
        // window and hash chains are owned vectors. Form the input span once,
        // then let that branch dispatch through the vectors directly instead
        // of recreating raw slices for all three work areas.
        let input = input.unwrap_or(&[]);
        let using_owned_workspace = state.owned_storage.is_some();
        // This is the last raw stream/storage bridge for custom and mixed
        // allocator workspaces. The named safe update below owns level and
        // strategy selection for both allocation modes.
        let update = if using_owned_workspace {
            let Some(output) = output_tail(strm, &mut output_buffer) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let Some(bstate) =
                with_owned_deflate_workspace(state, input, output, |state, workspace| {
                    deflate_update(state, strm, workspace, flush)
                })
            else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            DeflateUpdateResult {
                bstate,
                block_handled: false,
            }
        } else {
            let Some(output) = output_tail(strm, &mut output_buffer) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let Some(bstate) = with_callback_deflate_workspace(
                state,
                strm,
                input,
                output,
                &mut pending_buffer,
                flush,
                update_callback_deflate_workspace,
            )
            else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            bstate
        };
        let bstate = update.bstate;
        if bstate as ::core::ffi::c_uint
            == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
            || bstate as ::core::ffi::c_uint
                == finish_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            state.status = crate::src::deflate::FINISH_STATE;
        }
        if bstate as ::core::ffi::c_uint == need_more as ::core::ffi::c_int as ::core::ffi::c_uint
            || bstate as ::core::ffi::c_uint
                == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if strm.avail_out == 0 as crate::stdlib::uInt {
                state.last_flush = -1 as ::core::ffi::c_int;
            }
            return crate::zlib_h::Z_OK;
        }
        if !update.block_handled
            && bstate as ::core::ffi::c_uint
                == block_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if flush == crate::zlib_h::Z_PARTIAL_FLUSH {
                crate::src::trees::_tr_align(state, &mut pending_buffer);
            } else if flush != crate::zlib_h::Z_BLOCK {
                crate::src::trees::tr_stored_block(
                    state,
                    &mut pending_buffer,
                    None,
                    0 as crate::zutil_h::ulg,
                    0 as ::core::ffi::c_int,
                );
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    let cleared = if using_owned_workspace {
                        clear_owned_full_flush_hash(state)
                    } else {
                        with_callback_deflate_head(state, |state, head| {
                            clear_full_flush_hash(state, head)
                        })
                        .unwrap_or(false)
                    };
                    if !cleared {
                        return crate::zlib_h::Z_STREAM_ERROR;
                    }
                }
            }
            let Some(output) = output_tail(strm, &mut output_buffer) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            flush_pending(strm, state, &mut pending_buffer, output);
            if strm.avail_out == 0 as crate::stdlib::uInt {
                state.last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        }
    }
    if flush != crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_OK;
    }
    // The legacy engine owns this final raw storage conversion.  The wrapper
    // calculation, trailer emission, flush, and wrap transition stay in the
    // safe named helper above.
    if state.wrap <= 0 {
        return crate::zlib_h::Z_STREAM_END;
    }
    if state.pending_out > pending_buffer.len() || strm.avail_out != 0 && strm.next_out.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(output) = output_tail(strm, &mut output_buffer) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    finish_deflate_stream(strm, state, &mut pending_buffer, output)
}
#[export_name = "deflate"]

pub unsafe extern "C" fn deflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let input = if strm.avail_in == 0 || strm.next_in.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts(
            strm.next_in,
            strm.avail_in as usize,
        ))
    };
    let output = if strm.next_out.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(
            strm.next_out,
            strm.avail_out as usize,
        ))
    };
    deflate(strm, flush, input, output)
}
/// Tear down an already-borrowed deflate state.
///
/// Keep allocator validation and the callback free order in this typed core.
/// `deflateEnd` is the compatibility bridge for legacy callers that still
/// retain the state only in the ABI stream handle; an eventual stream owner
/// can call this directly without recreating that handle as a Rust reference.
fn deflate_end(
    stream: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::deflate::deflate_state,
) -> ::core::ffi::c_int {
    if stream.zalloc.is_none() || stream.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !deflate_stream_state_valid(Some(stream), Some(state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let status = state.status;
    // Default-pair workspaces are ordinary owned vectors. Drop them before
    // releasing the still callback-allocated opaque state; custom and mixed
    // streams retain zlib's exact pending/head/prev/window/state free order.
    let allocations = if state.owned_storage.take().is_some() {
        [
            ::core::ptr::null_mut(),
            ::core::ptr::null_mut(),
            ::core::ptr::null_mut(),
            ::core::ptr::null_mut(),
            stream.state as crate::stdlib::voidpf,
        ]
    } else {
        [
            state.pending_buf as crate::stdlib::voidpf,
            state.head as crate::stdlib::voidpf,
            state.prev as crate::stdlib::voidpf,
            state.window as crate::stdlib::voidpf,
            stream.state as crate::stdlib::voidpf,
        ]
    };
    for allocation in allocations {
        if !allocation.is_null() {
            Some(stream.zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(stream.opaque, allocation);
        }
    }
    stream.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
}

/// Tear down a legacy ABI-backed deflate stream.
///
/// Keep the sole opaque-state conversion here while the public stream still
/// stores the callback-allocated state as an ABI pointer.  All teardown
/// policy is in `deflate_end`, which accepts only typed references.
pub fn deflateEnd(stream: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    if stream.zalloc.is_none() || stream.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    with_deflate_stream_state(stream, deflate_end).unwrap_or(crate::zlib_h::Z_STREAM_ERROR)
}
#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflateEnd(strm)
}

fn pending_buffer_offset(
    pending_out: usize,
    pending_buf_len: crate::zutil_h::ulg,
    pending_len: crate::zutil_h::ulg,
) -> Option<usize> {
    let pending_buf_len = usize::try_from(pending_buf_len).ok()?;
    let pending_len = usize::try_from(pending_len).ok()?;
    pending_out
        .checked_add(pending_len)
        .filter(|&end| end <= pending_buf_len)?;
    Some(pending_out)
}

fn pending_buffer_range(
    start: usize,
    len: crate::stdlib::uInt,
    pending_buf_len: crate::zutil_h::ulg,
) -> Option<usize> {
    let pending_buf_len = usize::try_from(pending_buf_len).ok()?;
    let len = usize::try_from(len).ok()?;
    start.checked_add(len).filter(|&end| end <= pending_buf_len)
}

/// Check the source stream fields that must be valid before the FFI boundary
/// may borrow its opaque state handle.
///
/// Keeping this as a named implementation helper leaves `deflateCopy_ffi`
/// responsible only for ABI conversion.  In particular, a malformed source
/// with no allocator callbacks must be rejected before its `state` handle is
/// converted to a Rust reference.
fn deflate_copy_source_stream(
    source: Option<&crate::zlib_h::z_stream>,
) -> Option<&crate::zlib_h::z_stream> {
    let source = source?;
    (source.zalloc.is_some() && source.zfree.is_some()).then_some(source)
}

pub fn deflateCopy(
    dest: Option<&mut crate::zlib_h::z_stream>,
    source: Option<&crate::zlib_h::z_stream>,
    source_state: Option<&crate::src::deflate::deflate_state>,
) -> ::core::ffi::c_int {
    let Some(source_stream) = deflate_copy_source_stream(source) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(dest_stream) = dest else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(source_state) = source_state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_stream_state_valid(Some(source_stream), Some(source_state)) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(copy_layout) = DeflateCopyLayout::from_state(source_state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Default-pair streams already retain their four work buffers as owned
    // vectors. Clone the same validated live ranges before allocating the
    // opaque destination state, so this path never recreates callback-backed
    // workspace just to immediately treat it as owned again. Custom and
    // mixed allocator streams intentionally stay on the legacy path below:
    // their allocation and free callbacks remain observable API behavior.
    let owned_copy = match source_state.owned_storage.as_ref() {
        Some(storage) => match storage.try_copy_for_state(source_state) {
            Some(storage) => Some(storage),
            None => return crate::zlib_h::Z_STREAM_ERROR,
        },
        None => None,
    };
    crate::zlib_h::copy_z_stream(dest_stream, source_stream);
    crate::src::zutil::with_callback_state_slot(
        dest_stream,
        source_state.clone(),
        |dest_stream, dest_state| {
            dest_stream.state =
                ::core::ptr::from_mut(dest_state).cast::<crate::src::deflate::internal_state>();
            // Do not retain the derived clone's workspace. The default-pair branch
            // installs the range-preserving copy prepared above; custom and mixed
            // streams replace the raw handles through their original callbacks.
            dest_state.owned_storage = None;
            dest_state.strm = stream_identity(dest_stream);
            if let Some(mut storage) = owned_copy {
                if !storage.bind_state_buffers(dest_state) {
                    // The destination is still a fresh callback allocation. Clear
                    // copied source handles before teardown so a corrupt layout
                    // cannot make its failure path free source-owned storage.
                    dest_state.window = ::core::ptr::null_mut();
                    dest_state.prev = ::core::ptr::null_mut();
                    dest_state.head = ::core::ptr::null_mut();
                    dest_state.pending_buf = ::core::ptr::null_mut();
                    deflateEnd(dest_stream);
                    return crate::zlib_h::Z_MEM_ERROR;
                }
                dest_state.owned_storage = Some(storage);
                return crate::zlib_h::Z_OK;
            }
            let storage = DeflateStorageLayout::from_state(dest_state);
            dest_state.window = Some(dest_stream.zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                dest_stream.opaque,
                storage.window_items,
                (2 as usize).wrapping_mul(::core::mem::size_of::<crate::stdlib::Byte>())
                    as crate::stdlib::uInt,
            ) as *mut crate::stdlib::Bytef;
            dest_state.prev = Some(dest_stream.zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                dest_stream.opaque,
                storage.window_items,
                ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
            ) as *mut crate::src::deflate::Posf;
            dest_state.head = Some(dest_stream.zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                dest_stream.opaque,
                storage.hash_items,
                ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
            ) as *mut crate::src::deflate::Posf;
            dest_state.pending_buf = Some(dest_stream.zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                dest_stream.opaque,
                storage.pending_items,
                4 as crate::stdlib::uInt,
            ) as *mut crate::zutil_h::uchf
                as *mut crate::stdlib::Bytef;
            if dest_state.window.is_null()
                || dest_state.prev.is_null()
                || dest_state.head.is_null()
                || dest_state.pending_buf.is_null()
            {
                deflateEnd(dest_stream);
                return crate::zlib_h::Z_MEM_ERROR;
            }
            unsafe {
                crate::stdlib::memcpy(
                    dest_state.window as *mut ::core::ffi::c_void,
                    source_state.window as *const ::core::ffi::c_void,
                    copy_layout.window_bytes as crate::__stddef_size_t_h::size_t,
                )
            };
            unsafe {
                crate::stdlib::memcpy(
                    dest_state.prev as *mut ::core::ffi::c_void,
                    source_state.prev as *const ::core::ffi::c_void,
                    (copy_layout.prev_items as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Pos>()),
                )
            };
            unsafe {
                crate::stdlib::memcpy(
                    dest_state.head as *mut ::core::ffi::c_void,
                    source_state.head as *const ::core::ffi::c_void,
                    (copy_layout.head_items as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<crate::src::deflate::Pos>()),
                )
            };
            dest_state.pending_out = copy_layout.pending_offset;
            unsafe {
                crate::stdlib::memcpy(
                    dest_state.pending_buf.wrapping_add(dest_state.pending_out)
                        as *mut ::core::ffi::c_void,
                    source_state
                        .pending_buf
                        .wrapping_add(source_state.pending_out)
                        as *const ::core::ffi::c_void,
                    copy_layout.pending_bytes as crate::__stddef_size_t_h::size_t,
                )
            };
            dest_state.sym_buf = dest_state.lit_bufsize as usize;
            unsafe {
                crate::stdlib::memcpy(
                    dest_state.pending_buf.wrapping_add(dest_state.sym_buf)
                        as *mut ::core::ffi::c_void,
                    source_state
                        .pending_buf
                        .wrapping_add(copy_layout.sym_offset)
                        as *const ::core::ffi::c_void,
                    copy_layout.sym_bytes as crate::__stddef_size_t_h::size_t,
                )
            };
            crate::zlib_h::Z_OK
        },
    )
    .unwrap_or(crate::zlib_h::Z_MEM_ERROR)
}
#[export_name = "deflateCopy"]

pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(source) = deflate_copy_source_stream(source.as_ref()) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // The named validator above checks callback availability before this ABI
    // handle conversion. Keep source validation ahead of destination access,
    // matching the C copy contract for malformed streams.
    let source_state = (source.state as *const crate::src::deflate::deflate_state).as_ref();
    let dest = dest.as_mut();
    deflateCopy(dest, Some(source), source_state)
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

        let Some(&next_match) =
            prev.get((cur_match as crate::stdlib::uInt & state.w_mask) as usize)
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
        state.strstart = state
            .strstart
            .wrapping_add(input_tail.len() as crate::stdlib::uInt);
        state.insert = state.insert.wrapping_add(
            if input_tail.len() as crate::stdlib::uInt > state.w_size.wrapping_sub(state.insert) {
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

fn stored_input_remaining<'a>(
    strm: &crate::zlib_h::z_stream,
    input: &'a [crate::stdlib::Bytef],
) -> Option<&'a [crate::stdlib::Bytef]> {
    let available = usize::try_from(strm.avail_in).ok()?;
    let start = input.len().checked_sub(available)?;
    input.get(start..)
}

fn stored_output_remaining<'a>(
    strm: &crate::zlib_h::z_stream,
    output: &'a mut [crate::stdlib::Bytef],
) -> Option<&'a mut [crate::stdlib::Bytef]> {
    let available = usize::try_from(strm.avail_out).ok()?;
    let start = output.len().checked_sub(available)?;
    output.get_mut(start..)
}

fn deflate_stored(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    window: &mut [crate::stdlib::Bytef],
    input: &[crate::stdlib::Bytef],
    pending_buf: &mut [crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
) -> block_state {
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
            > (left as crate::zutil_h::ulg).wrapping_add(strm.avail_in as crate::zutil_h::ulg)
        {
            len = (left as crate::stdlib::uInt).wrapping_add(strm.avail_in) as ::core::ffi::c_uint;
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
        if !flush_strategy_pending(state, strm, pending_buf, output) {
            return need_more;
        }
        if left != 0 {
            if left > len {
                left = len;
            }
            let Some(output) = stored_output_remaining(strm, output) else {
                return need_more;
            };
            let Some(output) = output.get_mut(..left as usize) else {
                return need_more;
            };
            if !copy_stored_history(state, strm, window, output, left) {
                return need_more;
            }
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            let Some(input) = stored_input_remaining(strm, input) else {
                return need_more;
            };
            let Some(output) = stored_output_remaining(strm, output) else {
                return need_more;
            };
            let Some(output) = output.get_mut(..len as usize) else {
                return need_more;
            };
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
        let Some(input_tail) = input.get(..used as usize) else {
            return need_more;
        };
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
        let Some(slid) = slide_stored_window(state, window) else {
            return need_more;
        };
        have = have.wrapping_add(slid);
    }
    if have > strm.avail_in {
        have = strm.avail_in as ::core::ffi::c_uint;
    }
    if have != 0 {
        let Some(input) = stored_input_remaining(strm, input) else {
            return need_more;
        };
        let Ok(start) = usize::try_from(state.strstart) else {
            return need_more;
        };
        let Some(end) = start.checked_add(have as usize) else {
            return need_more;
        };
        let Some(output) = window.get_mut(start..end) else {
            return need_more;
        };
        read_buf(strm, state.wrap, input, output);
        state.strstart = state.strstart.wrapping_add(have);
        state.insert =
            state
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
        let Ok(start) = usize::try_from(state.block_start) else {
            return need_more;
        };
        let Some(end) = start.checked_add(len as usize) else {
            return need_more;
        };
        let Some(source) = window.get(start..end) else {
            return need_more;
        };
        crate::src::trees::tr_stored_block(
            state,
            pending_buf,
            Some(source),
            len as crate::zutil_h::ulg,
            last,
        );
        state.block_start += len as ::core::ffi::c_long;
        if !flush_strategy_pending(state, strm, pending_buf, output) {
            return need_more;
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
    state.ins_h =
        ((state.ins_h << state.hash_shift) ^ *second as crate::stdlib::uInt) & state.hash_mask;
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

fn deflate_fast(
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
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            let available_before = strm.avail_in as usize;
            fill_window(state, strm, window, head, prev, input);
            let Some(consumed) = available_before.checked_sub(strm.avail_in as usize) else {
                return need_more;
            };
            let Some(remaining) = input.get(consumed..) else {
                return need_more;
            };
            input = remaining;
            if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if state.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        hash_head = NIL as crate::src::deflate::IPos;
        if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let Some(inserted) = insert_hash(state, window, prev, head) else {
                return need_more;
            };
            hash_head = inserted;
        }
        if hash_head != NIL as crate::src::deflate::IPos
            && (state.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                <= state
                    .w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
        {
            state.match_length = longest_match(state, window, prev, hash_head);
        }
        if state.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len: crate::zutil_h::uch =
                state.match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush =
                state.strstart.wrapping_sub(state.match_start) as crate::zutil_h::ush;
            bflush = crate::src::trees::tr_tally(
                state,
                pending_buf,
                dist as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
            );
            state.lookahead = state.lookahead.wrapping_sub(state.match_length);
            if state.match_length <= state.max_lazy_match
                && state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            {
                state.match_length = state.match_length.wrapping_sub(1);
                loop {
                    state.strstart = state.strstart.wrapping_add(1);
                    let Some(inserted) = insert_hash(state, window, prev, head) else {
                        return need_more;
                    };
                    hash_head = inserted;
                    state.match_length = state.match_length.wrapping_sub(1);
                    if state.match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                state.strstart = state.strstart.wrapping_add(1);
            } else {
                state.strstart = state.strstart.wrapping_add(state.match_length);
                state.match_length = 0 as crate::stdlib::uInt;
                if !seed_hash(state, window) {
                    return need_more;
                }
            }
        } else {
            let Some(flush) = tally_literal(state, window, pending_buf) else {
                return need_more;
            };
            bflush = flush;
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            if !flush_strategy_block(state, strm, window, pending_buf, 0) {
                return need_more;
            }
            if !flush_strategy_pending(state, strm, pending_buf, output) {
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
    state.insert = if state.strstart
        < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        state.strstart
    } else {
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    };
    if flush == crate::zlib_h::Z_FINISH {
        if !flush_strategy_block(state, strm, window, pending_buf, 1) {
            return need_more;
        }
        if !flush_strategy_pending(state, strm, pending_buf, output) {
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
        if !flush_strategy_block(state, strm, window, pending_buf, 0) {
            return need_more;
        }
        if !flush_strategy_pending(state, strm, pending_buf, output) {
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

fn deflate_slow(
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
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            let available_before = strm.avail_in as usize;
            fill_window(state, strm, window, head, prev, input);
            let Some(consumed) = available_before.checked_sub(strm.avail_in as usize) else {
                return need_more;
            };
            let Some(remaining) = input.get(consumed..) else {
                return need_more;
            };
            input = remaining;
            if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if state.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        hash_head = NIL as crate::src::deflate::IPos;
        if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let Some(inserted) = insert_hash(state, window, prev, head) else {
                return need_more;
            };
            hash_head = inserted;
        }
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
            state.match_length = longest_match(state, window, prev, hash_head);
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
        if state.prev_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
            && state.match_length <= state.prev_length
        {
            let max_insert: crate::stdlib::uInt = state
                .strstart
                .wrapping_add(state.lookahead)
                .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
            let len: crate::zutil_h::uch =
                state.prev_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush = (state.strstart as crate::src::deflate::IPos)
                .wrapping_sub(1 as crate::src::deflate::IPos)
                .wrapping_sub(state.prev_match)
                as crate::zutil_h::ush;
            bflush = crate::src::trees::tr_tally(
                state,
                pending_buf,
                dist as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
            );
            state.lookahead = state
                .lookahead
                .wrapping_sub(state.prev_length.wrapping_sub(1 as crate::stdlib::uInt));
            state.prev_length = state.prev_length.wrapping_sub(2 as crate::stdlib::uInt);
            loop {
                state.strstart = state.strstart.wrapping_add(1);
                if state.strstart <= max_insert {
                    let Some(inserted) = insert_hash(state, window, prev, head) else {
                        return need_more;
                    };
                    hash_head = inserted;
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
                if !flush_strategy_block(state, strm, window, pending_buf, 0) {
                    return need_more;
                }
                if !flush_strategy_pending(state, strm, pending_buf, output) {
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
        } else if state.match_available != 0 {
            let Some(flush) = tally_previous_literal(state, window, pending_buf) else {
                return need_more;
            };
            bflush = flush;
            if bflush != 0 {
                if !flush_strategy_block(state, strm, window, pending_buf, 0) {
                    return need_more;
                }
                if !flush_strategy_pending(state, strm, pending_buf, output) {
                    return need_more;
                }
            }
            state.strstart = state.strstart.wrapping_add(1);
            state.lookahead = state.lookahead.wrapping_sub(1);
            if strm.avail_out == 0 as crate::stdlib::uInt {
                return need_more;
            }
        } else {
            state.match_available = 1 as ::core::ffi::c_int;
            state.strstart = state.strstart.wrapping_add(1);
            state.lookahead = state.lookahead.wrapping_sub(1);
        }
    }
    if state.match_available != 0 {
        let Some(flush) = tally_previous_literal(state, window, pending_buf) else {
            return need_more;
        };
        bflush = flush;
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
        if !flush_strategy_block(state, strm, window, pending_buf, 1) {
            return need_more;
        }
        if !flush_strategy_pending(state, strm, pending_buf, output) {
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
        if !flush_strategy_block(state, strm, window, pending_buf, 0) {
            return need_more;
        }
        if !flush_strategy_pending(state, strm, pending_buf, output) {
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

fn deflate_rle(
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
        if state.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            let available_before = strm.avail_in as usize;
            fill_window(state, strm, window, head, prev, input);
            let Some(consumed) = available_before.checked_sub(strm.avail_in as usize) else {
                return need_more;
            };
            let Some(remaining) = input.get(consumed..) else {
                return need_more;
            };
            input = remaining;
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
            let len: crate::zutil_h::uch =
                state.match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            bflush = crate::src::trees::tr_tally(
                state,
                pending_buf,
                1 as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
            );
            state.lookahead = state.lookahead.wrapping_sub(state.match_length);
            state.strstart = state.strstart.wrapping_add(state.match_length);
            state.match_length = 0 as crate::stdlib::uInt;
        } else {
            let Some(flush) = tally_literal(state, window, pending_buf) else {
                return need_more;
            };
            bflush = flush;
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            if !flush_strategy_block(state, strm, window, pending_buf, 0) {
                return need_more;
            }
            if !flush_strategy_pending(state, strm, pending_buf, output) {
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
        if !flush_strategy_block(state, strm, window, pending_buf, 1) {
            return need_more;
        }
        if !flush_strategy_pending(state, strm, pending_buf, output) {
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
        if !flush_strategy_block(state, strm, window, pending_buf, 0) {
            return need_more;
        }
        if !flush_strategy_pending(state, strm, pending_buf, output) {
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

fn flush_strategy_pending(
    state: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream,
    pending_buf: &mut [crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) -> bool {
    let pending_start = state.pending_out;
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

fn finish_deflate_stream(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::deflate::deflate_state,
    pending_buf: &mut [crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if state.wrap <= 0 {
        return crate::zlib_h::Z_STREAM_END;
    }
    if state.wrap == 2 {
        if !write_gzip_trailer(state, pending_buf, strm.adler, strm.total_in) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
    } else if !put_short_msb(
        state,
        pending_buf,
        (strm.adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
    ) || !put_short_msb(
        state,
        pending_buf,
        (strm.adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
    ) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !flush_strategy_pending(state, strm, pending_buf, output) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.wrap > 0 {
        state.wrap = -state.wrap;
    }
    if state.pending != 0 {
        crate::zlib_h::Z_OK
    } else {
        crate::zlib_h::Z_STREAM_END
    }
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
            if !flush_strategy_pending(state, strm, pending_buf, output) {
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
        if !flush_strategy_pending(state, strm, pending_buf, output) {
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
        if !flush_strategy_pending(state, strm, pending_buf, output) {
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
