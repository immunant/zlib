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
#[repr(C)]

pub struct ct_data_s {
    pub fc: crate::zutil_h::ush,
    pub dl: crate::zutil_h::ush,
}

pub type static_tree_desc = crate::src::deflate::static_tree_desc_s;

pub type tree_desc = crate::src::deflate::tree_desc_s;
pub enum TreeKind {
    LitLen,
    Dist,
    BitLen,
}

pub struct tree_desc_s {
    pub kind: TreeKind,
    pub max_code: ::core::ffi::c_int,
}

pub type Pos = crate::zutil_h::ush;

pub type Posf = crate::src::deflate::Pos;

pub type IPos = ::core::ffi::c_uint;

pub type deflate_state = crate::src::deflate::internal_state;

// `deflateSetHeader()` registers header data for later deflate calls.  Keep a
// byte-exact owned snapshot instead of retaining a foreign header pointer
// across those calls.  The C-facing conversion is confined to the setter;
// the compressor and bound calculation only inspect this pointer-free form.
struct GzipHeader {
    text: ::core::ffi::c_int,
    time: crate::stdlib::uLong,
    os: ::core::ffi::c_int,
    extra_len: crate::stdlib::uInt,
    extra: Option<Box<[u8]>>,
    name: Option<Box<[u8]>>,
    comment: Option<Box<[u8]>>,
    hcrc: bool,
}

fn copy_gzip_header(header: &GzipHeader) -> GzipHeader {
    GzipHeader {
        text: header.text,
        time: header.time,
        os: header.os,
        extra_len: header.extra_len,
        extra: header.extra.as_deref().map(Into::into),
        name: header.name.as_deref().map(Into::into),
        comment: header.comment.as_deref().map(Into::into),
        hcrc: header.hcrc,
    }
}

#[repr(C)]

pub struct internal_state {
    // Keep the legacy tree adapter's detected data type in opaque state.
    // Normal deflate dispatches publish their stream value directly, while
    // `_tr_flush_block()` has only this state handle available.
    pub data_type: ::core::ffi::c_int,
    pub status: ::core::ffi::c_int,
    // This allocation is released through the stream's zfree callback, so it
    // cannot yet become a Box. NonNull makes the initialized-owner invariant
    // explicit without retaining a raw pointer in the state layout.
    pub pending_buf: Option<::core::ptr::NonNull<crate::stdlib::Bytef>>,
    pub pending_buf_size: crate::zutil_h::ulg,
    // Cursor within `pending_buf`.  Keep this as an offset so copying/resetting a
    // stream never retains an interior raw pointer into the allocation.
    pub pending_out: usize,
    pub pending: crate::zutil_h::ulg,
    // Keep callback pairing as scalar ownership rather than deriving it from
    // raw allocation views during teardown. This distinguishes a partially
    // constructed copy's inherited source views from its own allocations.
    callback_storage: DeflateCallbackStorageOwner,
    pub wrap: ::core::ffi::c_int,
    gzhead: Option<GzipHeader>,
    pub gzindex: usize,
    pub method: crate::stdlib::Byte,
    pub last_flush: ::core::ffi::c_int,
    pub w_size: crate::stdlib::uInt,
    pub w_bits: crate::stdlib::uInt,
    pub w_mask: crate::stdlib::uInt,
    // Like the pending allocation, the window is callback-owned.  Its null
    // initialization/failure state is explicit instead of living in a raw
    // pointer field.
    pub window: Option<::core::ptr::NonNull<crate::stdlib::Bytef>>,
    pub window_size: crate::zutil_h::ulg,
    // These hash tables share the callback-owned allocation model used by the
    // window and pending buffer.  Keep allocation failure explicit instead of
    // storing nullable raw pointers in the state.
    pub prev: Option<::core::ptr::NonNull<crate::src::deflate::Posf>>,
    pub head: Option<::core::ptr::NonNull<crate::src::deflate::Posf>>,
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
    // Offset of the symbol region within `pending_buf`.  The C translation
    // retained this as an interior pointer; keeping an offset prevents a
    // copied/reset state from carrying a pointer into another allocation.
    pub sym_buf_start: usize,
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
pub use crate::src::trees::_tr_flush_block;
pub use crate::src::trees::tr_init;
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

#[derive(PartialEq, Eq)]
pub enum DeflateAlgorithm {
    Stored,
    Fast,
    Slow,
}

impl Copy for DeflateAlgorithm {}

impl Clone for DeflateAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}

pub type config = config_s;
#[repr(C)]

pub struct config_s {
    pub good_length: crate::zutil_h::ush,
    pub max_lazy: crate::zutil_h::ush,
    pub nice_length: crate::zutil_h::ush,
    pub max_chain: crate::zutil_h::ush,
    pub algorithm: DeflateAlgorithm,
}
const fn c_char_bytes(bytes: [u8; 70]) -> [::core::ffi::c_char; 70] {
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
    c_char_bytes(*b" deflate 1.3.2.1 Copyright 1995-2026 Jean-loup Gailly and Mark Adler \0");

pub const NIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const TOO_FAR: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

static configuration_table: [config; 10] = [
    config_s {
        good_length: 0 as crate::zutil_h::ush,
        max_lazy: 0 as crate::zutil_h::ush,
        nice_length: 0 as crate::zutil_h::ush,
        max_chain: 0 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Stored,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 8 as crate::zutil_h::ush,
        max_chain: 4 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 5 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 8 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 6 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Fast,
    },
    config_s {
        good_length: 4 as crate::zutil_h::ush,
        max_lazy: 4 as crate::zutil_h::ush,
        nice_length: 16 as crate::zutil_h::ush,
        max_chain: 16 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 32 as crate::zutil_h::ush,
        max_chain: 32 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 16 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 128 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 8 as crate::zutil_h::ush,
        max_lazy: 32 as crate::zutil_h::ush,
        nice_length: 128 as crate::zutil_h::ush,
        max_chain: 256 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 128 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 1024 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
    config_s {
        good_length: 32 as crate::zutil_h::ush,
        max_lazy: 258 as crate::zutil_h::ush,
        nice_length: 258 as crate::zutil_h::ush,
        max_chain: 4096 as crate::zutil_h::ush,
        algorithm: DeflateAlgorithm::Slow,
    },
];

// The allocation geometry is pure policy: it depends only on the public
// initialization arguments and does not need access to the ABI stream or the
// raw state allocation.  Keep it separate so the eventual owner-backed state
// can retain the exact zlib validation and capacities without reproducing the
// boundary projection.
struct DeflateLayout {
    level: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
    w_bits: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    hash_size: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    lit_bufsize: crate::stdlib::uInt,
}

// Keep the allocator-facing shape separate from the ABI-shaped state.  These
// are the exact `(items, size)` requests made to a custom zalloc callback, so
// later owner-backed storage can preserve allocation pairing and timing rather
// than collapsing the four buffers into an unobservable byte count.
#[derive(Clone, Copy)]
struct DeflateAllocation {
    items: crate::stdlib::uInt,
    size: crate::stdlib::uInt,
}

impl DeflateAllocation {
    // Keep the allocation request and its view capacity together. The
    // callback still receives the original `(items, size)` pair, while a
    // future owner-backed allocation broker can use this checked capacity to
    // construct its bounded byte view without re-deriving it from ABI state.
    fn byte_len(&self) -> Option<usize> {
        (self.items as usize).checked_mul(self.size as usize)
    }

    // Typed views must agree with the callback's exact element size. Keep
    // that check in the pointer-free allocation plan so later owned storage
    // does not have to recover element counts from raw copy byte lengths.
    fn element_len<T>(&self) -> Option<usize> {
        (self.size as usize == ::core::mem::size_of::<T>()).then_some(self.items as usize)
    }
}

#[derive(Clone, Copy)]
struct DeflateStorageLayout {
    window: DeflateAllocation,
    prev: DeflateAllocation,
    head: DeflateAllocation,
    pending: DeflateAllocation,
}

// The four backing allocations have an observable callback order: custom
// zalloc implementations can fail or inspect the stream after each request.
// Name that order independently of the raw allocation handles so the future
// owner can retain the same transaction without reconstructing it from state
// fields.
#[derive(Clone, Copy)]
enum DeflateStorageSlot {
    Window,
    Prev,
    Head,
    Pending,
}

// Callback-owned storage has two representations at the ABI boundary: the
// provenance-carrying handles in `internal_state`, and this pointer-free
// ownership ledger. The ledger decides which handles a stream may release,
// including during partial initialization and failed deep-copy setup.
#[derive(Clone, Copy)]
struct DeflateCallbackStorageOwner {
    // Keep the exact request geometry with the release ledger.  Callback
    // storage is still represented by provenance-carrying handles at the ABI
    // boundary, but every temporary typed view must use these immutable
    // allocation descriptors rather than mutable codec cursor/size fields.
    storage: DeflateStorageLayout,
    state: bool,
    window: bool,
    prev: bool,
    head: bool,
    pending: bool,
}

impl DeflateCallbackStorageOwner {
    fn new_state(storage: DeflateStorageLayout) -> Self {
        Self {
            storage,
            state: true,
            window: false,
            prev: false,
            head: false,
            pending: false,
        }
    }

    fn storage(&self) -> DeflateStorageLayout {
        self.storage
    }

    fn record_storage(&mut self, slot: DeflateStorageSlot, allocated: bool) {
        if !allocated {
            return;
        }
        match slot {
            DeflateStorageSlot::Window => self.window = true,
            DeflateStorageSlot::Prev => self.prev = true,
            DeflateStorageSlot::Head => self.head = true,
            DeflateStorageSlot::Pending => self.pending = true,
        }
    }

    fn take_release_plan(&mut self, status: ::core::ffi::c_int) -> DeflateReleasePlan {
        let plan = DeflateReleasePlan {
            status,
            pending: self.pending,
            head: self.head,
            prev: self.prev,
            window: self.window,
            state: self.state,
        };
        self.pending = false;
        self.head = false;
        self.prev = false;
        self.window = false;
        self.state = false;
        plan
    }

    // The owner persists with the callback lifecycle, while this operation
    // view borrows only the three regions needed by dictionary installation.
    // Verify the view against the immutable callback requests here so no
    // dictionary policy needs to recover a capacity from mutable codec state.
    fn dictionary_storage<'storage>(
        &self,
        storage: DeflateCallbackStorage<'storage>,
    ) -> Option<DeflateDictionaryStorage<'storage>> {
        if !(self.window && self.prev && self.head) {
            return None;
        }
        let layout = self.storage;
        let window = storage.window?;
        let prev = storage.prev?;
        let head = storage.head?;
        if window.len() != layout.window.byte_len()?
            || prev.len() != layout.prev.element_len::<crate::src::deflate::Posf>()?
            || head.len() != layout.head.element_len::<crate::src::deflate::Posf>()?
        {
            return None;
        }
        Some(DeflateDictionaryStorage { window, prev, head })
    }

    // A deflate copy crosses two independently callback-paired allocation
    // lifecycles.  Verify their immutable request geometry before the ABI
    // boundary performs its bounded raw copies.  The pointer-free copy facade
    // is reserved for owner-backed storage: constructing eight temporary raw
    // slice views here would duplicate the callback boundary's unsafe work.
    fn copy_geometry(
        &self,
        destination_owner: &DeflateCallbackStorageOwner,
        layout: &DeflateCopyLayout,
    ) -> bool {
        if !(self.window
            && self.prev
            && self.head
            && self.pending
            && destination_owner.window
            && destination_owner.prev
            && destination_owner.head
            && destination_owner.pending
            && self.storage.same_geometry(&destination_owner.storage)
            && layout.fits_storage(&self.storage))
        {
            return false;
        }
        true
    }
}

// This complete decision is pointer-free. The one callback boundary pairs
// each marked slot with its original handle in pending/head/prev/window/state
// order, preserving zlib's observable callback lifecycle.
#[derive(Clone, Copy)]
struct DeflateReleasePlan {
    status: ::core::ffi::c_int,
    pending: bool,
    head: bool,
    prev: bool,
    window: bool,
    state: bool,
}

impl DeflateReleasePlan {
    fn result(&self) -> ::core::ffi::c_int {
        if self.status == crate::src::deflate::BUSY_STATE {
            crate::zlib_h::Z_DATA_ERROR
        } else {
            crate::zlib_h::Z_OK
        }
    }
}

// Allocation success is independent of the callback-owned handles that are
// published into `internal_state`.  Keep that result as a pointer-free value
// so an eventual owner-backed allocator can retain the exact callback order
// without making its completion decision by inspecting raw storage.
struct DeflateStorageResults {
    window: bool,
    prev: bool,
    head: bool,
    pending: bool,
}

impl DeflateStorageResults {
    fn record(&mut self, slot: DeflateStorageSlot, allocated: bool) {
        match slot {
            DeflateStorageSlot::Window => self.window = allocated,
            DeflateStorageSlot::Prev => self.prev = allocated,
            DeflateStorageSlot::Head => self.head = allocated,
            DeflateStorageSlot::Pending => self.pending = allocated,
        }
    }

    fn is_complete(&self) -> bool {
        self.window && self.prev && self.head && self.pending
    }
}

// Keep the state record in the same pointer-free allocation plan as its four
// backing regions.  The current ABI adapter still invokes zalloc directly,
// but a future allocation broker can consume this complete plan and preserve
// zlib's allocation/failure/release sequence without inspecting raw state.
struct DeflateAllocationPlan {
    state: DeflateAllocation,
    storage: DeflateStorageLayout,
}

// This is the portion of a freshly allocated state that does not depend on
// either the ABI stream backlink or any callback-returned allocation.  Keeping
// it as a value makes the initialization contract available to the eventual
// owner-backed constructor without making that constructor reconstruct the
// C-era all-zero state by hand.
struct DeflateInitialState {
    status: ::core::ffi::c_int,
    pending_buf_size: crate::zutil_h::ulg,
    pending_out: usize,
    pending: crate::zutil_h::ulg,
    wrap: ::core::ffi::c_int,
    gzindex: usize,
    method: crate::stdlib::Byte,
    last_flush: ::core::ffi::c_int,
    w_size: crate::stdlib::uInt,
    w_bits: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    window_size: crate::zutil_h::ulg,
    ins_h: crate::stdlib::uInt,
    hash_size: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
}

// The one-shot APIs never expose their temporary stream.  Keep their bounded
// input and output borrows, along with the uInt-sized request accounting, in
// a pointer-free owner.  The small ABI adapter below is then the only place
// that needs to publish those borrows to z_stream for the existing codec.
pub(crate) struct DeflateOneShotOwner<'input, 'output> {
    input: &'input [crate::stdlib::Bytef],
    output: &'output mut [crate::stdlib::Bytef],
    input_remaining: crate::stdlib::z_size_t,
    output_remaining: crate::stdlib::z_size_t,
    output_capacity: crate::stdlib::z_size_t,
}

pub(crate) struct DeflateOneShotProgress {
    pub(crate) status: ::core::ffi::c_int,
    pub(crate) produced: crate::stdlib::z_size_t,
}

// Each temporary-stream dispatch borrows only the still-available portions
// of the caller buffers.  The ABI adapter turns these bounded views into
// cursors for one codec call and commits the returned availability before
// requesting the next chunk.
struct DeflateOneShotRequest<'input, 'output> {
    input: &'input [crate::stdlib::Bytef],
    output: &'output mut [crate::stdlib::Bytef],
    flush: ::core::ffi::c_int,
}

impl<'input, 'output> DeflateOneShotOwner<'input, 'output> {
    pub(crate) fn new(
        input: &'input [crate::stdlib::Bytef],
        output: &'output mut [crate::stdlib::Bytef],
    ) -> Self {
        let output_len = output.len();
        Self {
            input,
            output_capacity: output_len,
            output,
            input_remaining: input.len(),
            output_remaining: output_len,
        }
    }

    fn next_input_chunk(&mut self, max: crate::stdlib::uInt) -> crate::stdlib::uInt {
        let chunk = if self.input_remaining > max as crate::stdlib::z_size_t {
            max
        } else {
            self.input_remaining as crate::stdlib::uInt
        };
        self.input_remaining = self
            .input_remaining
            .wrapping_sub(chunk as crate::stdlib::z_size_t);
        chunk
    }

    fn next_output_chunk(&mut self, max: crate::stdlib::uInt) -> crate::stdlib::uInt {
        let chunk = if self.output_remaining > max as crate::stdlib::z_size_t {
            max
        } else {
            self.output_remaining as crate::stdlib::uInt
        };
        self.output_remaining = self
            .output_remaining
            .wrapping_sub(chunk as crate::stdlib::z_size_t);
        chunk
    }

    fn request(
        &mut self,
        max: crate::stdlib::uInt,
        input_offset: usize,
        input_available: &mut crate::stdlib::uInt,
        output_offset: usize,
        output_available: &mut crate::stdlib::uInt,
    ) -> DeflateOneShotRequest<'_, '_> {
        if *output_available == 0 {
            *output_available = self.next_output_chunk(max);
        }
        if *input_available == 0 {
            *input_available = self.next_input_chunk(max);
        }
        let input_end = input_offset + *input_available as usize;
        let output_end = output_offset + *output_available as usize;
        let flush = self.flush();
        DeflateOneShotRequest {
            input: &self.input[input_offset..input_end],
            output: &mut self.output[output_offset..output_end],
            flush,
        }
    }

    fn commit(
        &self,
        input_offset: &mut usize,
        input_available: &mut crate::stdlib::uInt,
        output_offset: &mut usize,
        output_available: &mut crate::stdlib::uInt,
        remaining_input: crate::stdlib::uInt,
        remaining_output: crate::stdlib::uInt,
    ) {
        *input_offset += (*input_available - remaining_input) as usize;
        *output_offset += (*output_available - remaining_output) as usize;
        *input_available = remaining_input;
        *output_available = remaining_output;
    }

    fn flush(&self) -> ::core::ffi::c_int {
        if self.input_remaining == 0 {
            crate::zlib_h::Z_FINISH
        } else {
            crate::zlib_h::Z_NO_FLUSH
        }
    }

    fn produced(&self, available_output: crate::stdlib::uInt) -> crate::stdlib::z_size_t {
        self.output_capacity.wrapping_sub(
            self.output_remaining
                .wrapping_add(available_output as crate::stdlib::z_size_t),
        )
    }
}

// This adapter owns the complete temporary ABI lifecycle for compress2_z().
// Its interface is pointer-free, so all chunking and result accounting remain
// in `DeflateOneShotOwner`.
pub(crate) fn deflate_one_shot(
    owner: &mut DeflateOneShotOwner<'_, '_>,
    level: ::core::ffi::c_int,
) -> DeflateOneShotProgress {
    deflate_one_shot_abi(owner, level)
}

// Keep the temporary ABI stream's complete lifecycle at one codec boundary.
// The caller owns only pointer-free input/output cursors and receives scalar
// completion, so none of the temporary stream's raw fields escape this
// adapter.
fn deflate_one_shot_abi(
    owner: &mut DeflateOneShotOwner<'_, '_>,
    level: ::core::ffi::c_int,
) -> DeflateOneShotProgress {
    let mut stream: crate::zlib_h::z_stream = crate::zlib_h::z_stream {
        next_in: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        state: None,
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut input_offset = 0usize;
    let mut output_offset = 0usize;
    let mut input_available = 0 as crate::stdlib::uInt;
    let mut output_available = 0 as crate::stdlib::uInt;
    let mut status = unsafe {
        deflateInit2_(
            Some(&mut stream),
            level,
            crate::zlib_h::Z_DEFLATED,
            crate::stdlib::MAX_WBITS,
            crate::zutil_h::DEF_MEM_LEVEL,
            crate::zlib_h::Z_DEFAULT_STRATEGY,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        )
    };
    if status != crate::zlib_h::Z_OK {
        return DeflateOneShotProgress {
            status,
            produced: 0,
        };
    }
    loop {
        let (remaining_input, remaining_output) = {
            let request = owner.request(
                max,
                input_offset,
                &mut input_available,
                output_offset,
                &mut output_available,
            );
            stream.next_in = request.input.as_ptr().cast_mut();
            stream.avail_in = request.input.len() as crate::stdlib::uInt;
            stream.next_out = request.output.as_mut_ptr();
            stream.avail_out = request.output.len() as crate::stdlib::uInt;
            status = unsafe { deflate_dispatch_from_abi_stream(&mut stream, request.flush) };
            (stream.avail_in, stream.avail_out)
        };
        owner.commit(
            &mut input_offset,
            &mut input_available,
            &mut output_offset,
            &mut output_available,
            remaining_input,
            remaining_output,
        );
        if status != crate::zlib_h::Z_OK {
            break;
        }
    }
    let produced = owner.produced(output_available);
    unsafe {
        deflateEnd(::core::ptr::NonNull::from(&mut stream));
    }
    DeflateOneShotProgress {
        status: if status == crate::zlib_h::Z_STREAM_END {
            crate::zlib_h::Z_OK
        } else {
            status
        },
        produced,
    }
}

impl DeflateLayout {
    fn storage(&self) -> DeflateStorageLayout {
        DeflateStorageLayout::new(self.w_size, self.hash_size, self.lit_bufsize)
    }

    fn allocation_plan(&self) -> DeflateAllocationPlan {
        DeflateAllocationPlan {
            state: DeflateAllocation {
                items: 1,
                size: ::core::mem::size_of::<crate::src::deflate::deflate_state>()
                    as crate::stdlib::uInt,
            },
            storage: self.storage(),
        }
    }

    fn initial_state(&self) -> DeflateInitialState {
        DeflateInitialState {
            status: crate::src::deflate::INIT_STATE,
            pending_buf_size: 0,
            pending_out: 0,
            pending: 0,
            wrap: self.wrap,
            gzindex: 0,
            method: 0,
            last_flush: 0,
            w_size: self.w_size,
            w_bits: self.w_bits,
            w_mask: self.w_mask,
            window_size: 0,
            ins_h: 0,
            hash_size: self.hash_size,
            hash_bits: self.hash_bits,
            hash_mask: self.hash_mask,
            hash_shift: self.hash_shift,
        }
    }
}

impl DeflateStorageLayout {
    fn same_geometry(&self, other: &Self) -> bool {
        self.window.items == other.window.items
            && self.window.size == other.window.size
            && self.prev.items == other.prev.items
            && self.prev.size == other.prev.size
            && self.head.items == other.head.items
            && self.head.size == other.head.size
            && self.pending.items == other.pending.items
            && self.pending.size == other.pending.size
    }

    fn new(
        w_size: crate::stdlib::uInt,
        hash_size: crate::stdlib::uInt,
        lit_bufsize: crate::stdlib::uInt,
    ) -> Self {
        Self {
            window: DeflateAllocation {
                items: w_size,
                size: (2 * ::core::mem::size_of::<crate::stdlib::Byte>()) as crate::stdlib::uInt,
            },
            prev: DeflateAllocation {
                items: w_size,
                size: ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
            },
            head: DeflateAllocation {
                items: hash_size,
                size: ::core::mem::size_of::<crate::src::deflate::Pos>() as crate::stdlib::uInt,
            },
            pending: DeflateAllocation {
                items: lit_bufsize,
                size: 4,
            },
        }
    }

    fn callback_requests(&self) -> [(DeflateStorageSlot, &DeflateAllocation); 4] {
        [
            (DeflateStorageSlot::Window, &self.window),
            (DeflateStorageSlot::Prev, &self.prev),
            (DeflateStorageSlot::Head, &self.head),
            (DeflateStorageSlot::Pending, &self.pending),
        ]
    }
}

// Keep the callback ordering and failure observation separate from the raw
// handles used to publish each allocation.  In particular, this must visit
// every request even if an earlier callback returned null: the C allocation
// path invokes all four callbacks before its common cleanup path.  The
// callback itself remains at the ABI boundary, while a future owner-backed
// transaction can reuse this pointer-free schedule unchanged.
fn request_deflate_storage(
    storage: &DeflateStorageLayout,
    mut request: impl FnMut(&DeflateStorageSlot, &DeflateAllocation) -> bool,
) -> DeflateStorageResults {
    let mut results = DeflateStorageResults {
        window: false,
        prev: false,
        head: false,
        pending: false,
    };
    for (slot, allocation) in storage.callback_requests() {
        let allocated = request(&slot, allocation);
        results.record(slot, allocated);
    }
    results
}

fn deflate_layout(
    level: ::core::ffi::c_int,
    method: ::core::ffi::c_int,
    mut window_bits: ::core::ffi::c_int,
    mem_level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> Option<DeflateLayout> {
    let level = if level == crate::zlib_h::Z_DEFAULT_COMPRESSION {
        6
    } else {
        level
    };
    let wrap = if window_bits < 0 {
        if window_bits < -15 {
            return None;
        }
        window_bits = -window_bits;
        0
    } else if window_bits > 15 {
        window_bits -= 16;
        2
    } else {
        1
    };
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
    let w_bits = window_bits as crate::stdlib::uInt;
    let w_size = (1 << w_bits) as crate::stdlib::uInt;
    let hash_bits = (mem_level as crate::stdlib::uInt).wrapping_add(7);
    let hash_size = (1 << hash_bits) as crate::stdlib::uInt;
    let lit_bufsize = (1 << (mem_level + 6)) as crate::stdlib::uInt;
    Some(DeflateLayout {
        level,
        wrap,
        w_bits,
        w_size,
        w_mask: w_size.wrapping_sub(1),
        hash_bits,
        hash_size,
        hash_mask: hash_size.wrapping_sub(1),
        hash_shift: hash_bits
            .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
            .wrapping_sub(1)
            .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt),
        lit_bufsize,
    })
}

fn slide_hash_table(table: &mut [crate::src::deflate::Posf], wsize: crate::stdlib::uInt) {
    for entry in table.iter_mut().rev() {
        let position = *entry as ::core::ffi::c_uint;
        *entry = (if position >= wsize {
            position.wrapping_sub(wsize as ::core::ffi::c_uint)
        } else {
            NIL as ::core::ffi::c_uint
        }) as crate::src::deflate::Pos as crate::src::deflate::Posf;
    }
}

fn clear_hash_table(table: &mut [crate::src::deflate::Posf]) {
    table.fill(NIL as crate::src::deflate::Posf);
}

fn slide_window_bytes(
    window: &mut [crate::stdlib::Bytef],
    wsize: crate::stdlib::uInt,
    more: ::core::ffi::c_uint,
) {
    let start = wsize as usize;
    let count = wsize.wrapping_sub(more) as usize;
    window.copy_within(start..start + count, 0);
}

fn clear_window_bytes(
    window: &mut [crate::stdlib::Bytef],
    start: crate::zutil_h::ulg,
    len: crate::zutil_h::ulg,
) {
    let start = start as usize;
    window[start..start + len as usize].fill(0);
}

// Keep high-water initialization independent of the ABI state.  The caller
// supplies the one bounded window view and writes the returned cursor back to
// its opaque state.
fn initialize_window_high_water(
    window: &mut [crate::stdlib::Bytef],
    high_water: crate::zutil_h::ulg,
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
) -> crate::zutil_h::ulg {
    let window_size = window.len() as crate::zutil_h::ulg;
    let curr = (strstart as crate::zutil_h::ulg).wrapping_add(lookahead as crate::zutil_h::ulg);
    if high_water < curr {
        let mut init = window_size.wrapping_sub(curr);
        if init > crate::src::deflate::WIN_INIT as crate::zutil_h::ulg {
            init = crate::src::deflate::WIN_INIT as crate::zutil_h::ulg;
        }
        clear_window_bytes(window, curr, init);
        curr.wrapping_add(init)
    } else if high_water < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg) {
        let mut init = curr
            .wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
            .wrapping_sub(high_water);
        if init > window_size.wrapping_sub(high_water) {
            init = window_size.wrapping_sub(high_water);
        }
        clear_window_bytes(window, high_water, init);
        high_water.wrapping_add(init)
    } else {
        high_water
    }
}

fn read_buf_checksum(
    checksum: crate::stdlib::uLong,
    wrap: ::core::ffi::c_int,
    bytes: &[crate::stdlib::Bytef],
) -> crate::stdlib::uLong {
    match wrap {
        1 => crate::src::adler32::adler32(checksum, bytes),
        2 => crate::src::crc32::crc32_z(checksum, Some(bytes)),
        _ => checksum,
    }
}

fn read_buf_bytes(
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
    wrap: ::core::ffi::c_int,
) -> crate::stdlib::uLong {
    output.copy_from_slice(input);
    read_buf_checksum(checksum, wrap, output)
}

// The fill operation has no need to retain the ABI stream or state.  The
// active strategy adapters project their bounded allocations and cursors,
// then use this core directly.
struct DeflateInputCursor<'a> {
    input: &'a [crate::stdlib::Bytef],
    consumed: usize,
    checksum: crate::stdlib::uLong,
    total_in: crate::stdlib::uLong,
}

struct FillWindowState<'a> {
    window: &'a mut [crate::stdlib::Bytef],
    prev: &'a mut [crate::src::deflate::Posf],
    head: &'a mut [crate::src::deflate::Posf],
    w_size: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    wrap: ::core::ffi::c_int,
    lookahead: &'a mut crate::stdlib::uInt,
    strstart: &'a mut crate::stdlib::uInt,
    match_start: &'a mut crate::stdlib::uInt,
    block_start: &'a mut ::core::ffi::c_long,
    insert: &'a mut crate::stdlib::uInt,
    slid: &'a mut ::core::ffi::c_int,
    ins_h: &'a mut crate::stdlib::uInt,
    high_water: &'a mut crate::zutil_h::ulg,
}

fn fill_window_from_views(state: &mut FillWindowState<'_>, input: &mut DeflateInputCursor<'_>) {
    let mut n: ::core::ffi::c_uint = 0;
    let mut more: ::core::ffi::c_uint = 0;
    let wsize = state.w_size;
    loop {
        more = (state.window.len() as crate::zutil_h::ulg)
            .wrapping_sub(*state.lookahead as crate::zutil_h::ulg)
            .wrapping_sub(*state.strstart as crate::zutil_h::ulg)
            as ::core::ffi::c_uint;
        if ::core::mem::size_of::<::core::ffi::c_int>() <= 2 as usize {
            if more == 0 as ::core::ffi::c_uint
                && *state.strstart == 0 as crate::stdlib::uInt
                && *state.lookahead == 0 as crate::stdlib::uInt
            {
                more = wsize as ::core::ffi::c_uint;
            } else if more == -1 as ::core::ffi::c_int as ::core::ffi::c_uint {
                more = more.wrapping_sub(1);
            }
        }
        if *state.strstart
            >= wsize.wrapping_add(
                wsize.wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            slide_window_bytes(state.window, wsize, more);
            *state.match_start = state.match_start.wrapping_sub(wsize);
            *state.strstart = state.strstart.wrapping_sub(wsize);
            *state.block_start -= wsize as ::core::ffi::c_long;
            if *state.insert > *state.strstart {
                *state.insert = *state.strstart;
            }
            slide_hash_table(state.head, wsize);
            slide_hash_table(state.prev, wsize);
            *state.slid = 1 as ::core::ffi::c_int;
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        let available = input.input.len().saturating_sub(input.consumed);
        if available == 0 {
            break;
        }
        n = (available as crate::stdlib::uInt).min(more);
        if n != 0 {
            let source = &input.input[input.consumed..input.consumed + n as usize];
            let start = state.strstart.wrapping_add(*state.lookahead) as usize;
            let output = &mut state.window[start..start + n as usize];
            input.checksum = read_buf_bytes(source, output, input.checksum, state.wrap);
            input.consumed += n as usize;
            input.total_in = input.total_in.wrapping_add(n as crate::stdlib::uLong);
        }
        *state.lookahead = state.lookahead.wrapping_add(n);
        if state.lookahead.wrapping_add(*state.insert)
            >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        {
            let mut str: crate::stdlib::uInt = state.strstart.wrapping_sub(*state.insert);
            *state.ins_h = state.window[str as usize] as crate::stdlib::uInt;
            *state.ins_h = (*state.ins_h << state.hash_shift
                ^ state.window[str.wrapping_add(1 as crate::stdlib::uInt) as usize]
                    as crate::stdlib::uInt)
                & state.hash_mask;
            while *state.insert != 0 {
                *state.ins_h = (*state.ins_h << state.hash_shift
                    ^ state.window[str
                        .wrapping_add(3 as crate::stdlib::uInt)
                        .wrapping_sub(1 as crate::stdlib::uInt)
                        as usize] as crate::stdlib::uInt)
                    & state.hash_mask;
                state.prev[(str & state.w_mask) as usize] = state.head[*state.ins_h as usize];
                state.head[*state.ins_h as usize] =
                    str as crate::src::deflate::Pos as crate::src::deflate::Posf;
                str = str.wrapping_add(1);
                *state.insert = state.insert.wrapping_sub(1);
                if state.lookahead.wrapping_add(*state.insert)
                    < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                {
                    break;
                }
            }
        }
        if !(*state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
            && input.consumed < input.input.len())
        {
            break;
        }
    }
    if *state.high_water < state.window.len() as crate::zutil_h::ulg {
        *state.high_water = initialize_window_high_water(
            state.window,
            *state.high_water,
            *state.strstart,
            *state.lookahead,
        );
    }
}

#[export_name = "deflateInit_"]

pub unsafe extern "C" fn deflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut level: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    deflateInit2_(
        strm.as_mut(),
        level,
        crate::zlib_h::Z_DEFLATED,
        crate::stdlib::MAX_WBITS,
        crate::zutil_h::DEF_MEM_LEVEL,
        crate::zlib_h::Z_DEFAULT_STRATEGY,
        version,
        stream_size,
    )
}
// The ABI wrappers convert the nullable stream handle before reaching this
// lifecycle implementation. Keep version validation here so it retains
// zlib's version-error precedence over a null stream.
pub unsafe fn deflateInit2_(
    strm: Option<&mut crate::zlib_h::z_stream_s>,
    mut level: ::core::ffi::c_int,
    mut method: ::core::ffi::c_int,
    mut windowBits: ::core::ffi::c_int,
    mut memLevel: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
    version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static my_version: [::core::ffi::c_char; 15] = crate::zlib_h::ZLIB_VERSION;
    if version.is_null()
        || *version as ::core::ffi::c_int != my_version[0 as usize] as ::core::ffi::c_int
        || stream_size as usize != ::core::mem::size_of::<crate::zlib_h::z_stream>()
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    let Some(stream) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
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
    let Some(layout) = deflate_layout(level, method, windowBits, memLevel, strategy) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let allocation_plan = layout.allocation_plan();
    let initial_state = layout.initial_state();
    let storage = allocation_plan.storage;
    let s = Some(stream.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        stream.opaque,
        allocation_plan.state.items,
        allocation_plan.state.size,
    ) as *mut crate::src::deflate::deflate_state;
    if s.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // Publish a valid Rust state before invoking the remaining callbacks.
    // This keeps the original allocation order and makes callback re-entry
    // observe the same installed stream state without first writing invalid
    // all-zero bytes into Rust enum fields.
    s.write(crate::src::deflate::internal_state {
        data_type: stream.data_type,
        status: initial_state.status,
        pending_buf: None,
        pending_buf_size: initial_state.pending_buf_size,
        pending_out: initial_state.pending_out,
        pending: initial_state.pending,
        callback_storage: DeflateCallbackStorageOwner::new_state(storage),
        wrap: initial_state.wrap,
        gzhead: None,
        gzindex: initial_state.gzindex,
        method: initial_state.method,
        last_flush: initial_state.last_flush,
        w_size: initial_state.w_size,
        w_bits: initial_state.w_bits,
        w_mask: initial_state.w_mask,
        window: None,
        window_size: initial_state.window_size,
        prev: None,
        head: None,
        ins_h: initial_state.ins_h,
        hash_size: initial_state.hash_size,
        hash_bits: initial_state.hash_bits,
        hash_mask: initial_state.hash_mask,
        hash_shift: initial_state.hash_shift,
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
        dyn_ltree: [const { crate::src::deflate::ct_data_s { fc: 0, dl: 0 } }; 573],
        dyn_dtree: [const { crate::src::deflate::ct_data_s { fc: 0, dl: 0 } }; 61],
        bl_tree: [const { crate::src::deflate::ct_data_s { fc: 0, dl: 0 } }; 39],
        l_desc: crate::src::deflate::tree_desc_s {
            kind: crate::src::deflate::TreeKind::LitLen,
            max_code: 0,
        },
        d_desc: crate::src::deflate::tree_desc_s {
            kind: crate::src::deflate::TreeKind::Dist,
            max_code: 0,
        },
        bl_desc: crate::src::deflate::tree_desc_s {
            kind: crate::src::deflate::TreeKind::BitLen,
            max_code: 0,
        },
        bl_count: [0; 16],
        heap: [0; 573],
        heap_len: 0,
        heap_max: 0,
        depth: [0; 573],
        sym_buf_start: 0,
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
    });
    stream.state = Some(
        ::core::ptr::NonNull::new(s)
            .expect("checked state allocation")
            .cast(),
    );
    // Do not keep a Rust borrow of the installed state across an allocator
    // callback: a caller allocator may observe the stream re-entrantly.
    // Each callback result is instead published through a short projection,
    // and all work after the final callback uses an ordinary Rust borrow.
    let storage_results = request_deflate_storage(&storage, |slot, request| {
        let allocation = Some(stream.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            stream.opaque, request.items, request.size
        );
        let allocated = !allocation.is_null();
        // Publish every callback result before requesting the next region:
        // custom allocators are permitted to inspect the stream re-entrantly.
        // This projection stays at the allocation boundary; the scheduling and
        // completion accounting above remain pointer-free.
        let state = &mut *s;
        match slot {
            DeflateStorageSlot::Window => {
                state.window = ::core::ptr::NonNull::new(allocation.cast());
            }
            DeflateStorageSlot::Prev => {
                state.prev = ::core::ptr::NonNull::new(allocation.cast());
            }
            DeflateStorageSlot::Head => {
                state.head = ::core::ptr::NonNull::new(allocation.cast());
            }
            DeflateStorageSlot::Pending => {
                state.pending_buf = ::core::ptr::NonNull::new(allocation.cast());
            }
        }
        state.callback_storage.record_storage(*slot, allocated);
        allocated
    });
    let state = &mut *s;
    state.data_type = crate::zlib_h::Z_UNKNOWN;
    state.high_water = 0 as crate::zutil_h::ulg;
    state.lit_bufsize = layout.lit_bufsize;
    state.pending_buf_size = storage
        .pending
        .byte_len()
        .expect("validated pending allocation geometry")
        as crate::zutil_h::ulg;
    if !storage_results.is_complete()
        || state.window.is_none()
        || state.prev.is_none()
        || state.head.is_none()
        || state.pending_buf.is_none()
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
        deflateEnd(::core::ptr::NonNull::from(stream));
        return crate::zlib_h::Z_MEM_ERROR;
    }
    state.sym_buf_start = state.lit_bufsize as usize;
    state.sym_end = state
        .lit_bufsize
        .wrapping_sub(1 as crate::stdlib::uInt)
        .wrapping_mul(3 as crate::stdlib::uInt);
    state.level = layout.level;
    state.strategy = strategy;
    state.method = method as crate::stdlib::Byte;
    // The state is already installed and all four callback allocations have
    // succeeded.  Apply the ordinary reset policy directly through the
    // pointer-free reset core instead of re-entering the raw stream API.
    // The one remaining projection is bounded by the exact head allocation
    // request in `storage`.
    stream.total_out = 0;
    stream.total_in = 0;
    stream.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    stream.data_type = crate::zlib_h::Z_UNKNOWN;
    let head = ::core::slice::from_raw_parts_mut(
        state.head.expect("initialized head table").as_ptr(),
        state.hash_size as usize,
    );
    stream.adler = reset_keep_core(
        &mut state.pending,
        &mut state.pending_out,
        &mut state.wrap,
        &mut state.status,
        &mut state.last_flush,
        &mut state.dyn_ltree,
        &mut state.dyn_dtree,
        &mut state.bl_tree,
        &mut state.l_desc,
        &mut state.d_desc,
        &mut state.bl_desc,
        &mut state.static_len,
        &mut state.opt_len,
        &mut state.matches,
        &mut state.sym_next,
        &mut state.bi_buf,
        &mut state.bi_valid,
        &mut state.bi_used,
    );
    let w_size = state.w_size;
    let config = &configuration_table[state.level as usize];
    DeflateResetCore {
        window_size: &mut state.window_size,
        slid: &mut state.slid,
        max_lazy_match: &mut state.max_lazy_match,
        good_match: &mut state.good_match,
        nice_match: &mut state.nice_match,
        max_chain_length: &mut state.max_chain_length,
        strstart: &mut state.strstart,
        block_start: &mut state.block_start,
        lookahead: &mut state.lookahead,
        insert: &mut state.insert,
        prev_length: &mut state.prev_length,
        match_length: &mut state.match_length,
        match_available: &mut state.match_available,
        ins_h: &mut state.ins_h,
    }
    .reset_after_keep(head, w_size, config);
    crate::zlib_h::Z_OK
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
        strm.as_mut(),
        level,
        method,
        windowBits,
        memLevel,
        strategy,
        version,
        stream_size,
    )
}
fn deflate_state_status_is_valid(status: ::core::ffi::c_int) -> bool {
    status == crate::src::deflate::INIT_STATE
        || status == crate::src::deflate::GZIP_STATE
        || status == crate::src::deflate::EXTRA_STATE
        || status == crate::src::deflate::NAME_STATE
        || status == crate::src::deflate::COMMENT_STATE
        || status == crate::src::deflate::HCRC_STATE
        || status == crate::src::deflate::BUSY_STATE
        || status == crate::src::deflate::FINISH_STATE
}

// The callback allocations remain owned by `internal_state` and are released
// through its recorded zfree transaction.  This view only borrows them for a
// single stream operation; in particular, it never extends their lifetime to
// `'static` or replaces their callback provenance with a Rust allocation.
struct DeflateCallbackStorage<'stream> {
    window: Option<&'stream mut [crate::stdlib::Bytef]>,
    prev: Option<&'stream mut [crate::src::deflate::Posf]>,
    head: Option<&'stream mut [crate::src::deflate::Posf]>,
    pending: Option<&'stream mut [crate::stdlib::Bytef]>,
}

// This operation-facing owner is intentionally pointer-free: the persistent
// callback owner proves which allocations exist and their exact geometry,
// while the stream boundary lends it ordinary bounded slices for one call.
struct DeflateDictionaryStorage<'stream> {
    window: &'stream mut [crate::stdlib::Bytef],
    prev: &'stream mut [crate::src::deflate::Posf],
    head: &'stream mut [crate::src::deflate::Posf],
}

enum DeflateStorageProjection<'request> {
    None,
    // Bit priming needs only the callback-owned pending bytes.  Project that
    // view at the shared stream/state boundary so the bit operation itself
    // never reconstructs a slice from an allocation handle.
    Pending,
    // A full reset only clears the hash table.  Keep that bounded callback
    // view in the shared stream/state projection, rather than reconstructing
    // it at the reset adapter after the opaque state has been borrowed.
    Hash,
    Dictionary,
    // Dictionary installation is a complete pointer-free operation once the
    // shared boundary has lent it the callback-owned views.  Keep its status
    // slot in the request so the FFI export can dispatch directly without
    // matching a stream/state projection itself.
    DictionaryInstall {
        dictionary: &'request [crate::stdlib::Bytef],
        result: &'request mut ::core::ffi::c_int,
    },
    // A dictionary query shares the same callback-owned window projection as
    // installation, but its copy and length publication remain in the
    // pointer-free dictionary owner below this boundary.
    DictionaryQuery {
        dictionary: Option<&'request mut [crate::stdlib::Bytef]>,
        dict_length: Option<&'request mut crate::stdlib::uInt>,
        result: &'request mut ::core::ffi::c_int,
    },
}

// The caller first checks and borrows the ABI stream, then this short-lived
// projection validates its opaque state.  Keeping all returned borrows tied
// to that stream borrow prevents a state or callback-buffer reference from
// escaping the ABI call.
unsafe fn deflate_stream_and_state<'stream, 'request>(
    strm: &'stream mut crate::zlib_h::z_stream_s,
    projection: DeflateStorageProjection<'request>,
) -> Option<(
    &'stream mut crate::zlib_h::z_stream_s,
    &'stream mut crate::src::deflate::deflate_state,
    DeflateCallbackStorage<'stream>,
)> {
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return None;
    }
    let state = strm
        .state?
        .cast::<crate::src::deflate::deflate_state>()
        .as_mut();
    if !deflate_state_status_is_valid(state.status) {
        return None;
    }
    let storage_layout = state.callback_storage.storage();
    let storage = match projection {
        DeflateStorageProjection::None => DeflateCallbackStorage {
            window: None,
            prev: None,
            head: None,
            pending: None,
        },
        DeflateStorageProjection::Pending => DeflateCallbackStorage {
            window: None,
            prev: None,
            head: None,
            pending: Some(::core::slice::from_raw_parts_mut(
                state
                    .pending_buf
                    .expect("initialized pending buffer")
                    .as_ptr(),
                storage_layout
                    .pending
                    .byte_len()
                    .expect("validated pending allocation geometry"),
            )),
        },
        DeflateStorageProjection::Hash => DeflateCallbackStorage {
            window: None,
            prev: None,
            head: Some(::core::slice::from_raw_parts_mut(
                state.head.expect("initialized head table").as_ptr(),
                storage_layout
                    .head
                    .element_len::<crate::src::deflate::Posf>()
                    .expect("validated head allocation geometry"),
            )),
            pending: None,
        },
        DeflateStorageProjection::Dictionary
        | DeflateStorageProjection::DictionaryInstall { .. }
        | DeflateStorageProjection::DictionaryQuery { .. } => DeflateCallbackStorage {
            window: Some(::core::slice::from_raw_parts_mut(
                state.window.expect("initialized window").as_ptr(),
                storage_layout
                    .window
                    .byte_len()
                    .expect("validated window allocation geometry"),
            )),
            prev: Some(::core::slice::from_raw_parts_mut(
                state.prev.expect("initialized prev table").as_ptr(),
                storage_layout
                    .prev
                    .element_len::<crate::src::deflate::Posf>()
                    .expect("validated prev allocation geometry"),
            )),
            head: Some(::core::slice::from_raw_parts_mut(
                state.head.expect("initialized head table").as_ptr(),
                storage_layout
                    .head
                    .element_len::<crate::src::deflate::Posf>()
                    .expect("validated head allocation geometry"),
            )),
            pending: None,
        },
    };
    if let DeflateStorageProjection::DictionaryInstall { dictionary, result } = projection {
        let Some(storage) = state.callback_storage.dictionary_storage(storage) else {
            return Some((
                strm,
                state,
                DeflateCallbackStorage {
                    window: None,
                    prev: None,
                    head: None,
                    pending: None,
                },
            ));
        };
        let mut dictionary_state = DictionaryState {
            wrap: state.wrap,
            status: state.status,
            lookahead: state.lookahead,
            w_size: state.w_size,
            slid: state.slid,
            strstart: state.strstart,
            block_start: state.block_start,
            insert: state.insert,
            ins_h: state.ins_h,
            hash_shift: state.hash_shift,
            hash_mask: state.hash_mask,
            w_mask: state.w_mask,
            prev_length: state.prev_length,
            match_length: state.match_length,
            match_available: state.match_available,
            high_water: state.high_water,
        };
        let Ok(checksum) = deflateSetDictionary(
            &mut dictionary_state,
            storage.window,
            storage.head,
            storage.prev,
            dictionary,
            strm.adler,
        ) else {
            return Some((
                strm,
                state,
                DeflateCallbackStorage {
                    window: None,
                    prev: None,
                    head: None,
                    pending: None,
                },
            ));
        };
        state.wrap = dictionary_state.wrap;
        state.slid = dictionary_state.slid;
        state.strstart = dictionary_state.strstart;
        state.block_start = dictionary_state.block_start;
        state.insert = dictionary_state.insert;
        state.ins_h = dictionary_state.ins_h;
        state.lookahead = dictionary_state.lookahead;
        state.prev_length = dictionary_state.prev_length;
        state.match_length = dictionary_state.match_length;
        state.match_available = dictionary_state.match_available;
        state.high_water = dictionary_state.high_water;
        if let Some(checksum) = checksum {
            strm.adler = checksum;
        }
        *result = crate::zlib_h::Z_OK;
        return Some((
            strm,
            state,
            DeflateCallbackStorage {
                window: None,
                prev: None,
                head: None,
                pending: None,
            },
        ));
    }
    if let DeflateStorageProjection::DictionaryQuery {
        dictionary,
        dict_length,
        result,
    } = projection
    {
        let Some(window) = storage.window.as_deref() else {
            return Some((strm, state, storage));
        };
        let mut len = state.strstart.wrapping_add(state.lookahead);
        if len > state.w_size {
            len = state.w_size;
        }
        let len = len as usize;
        let end = state.strstart.wrapping_add(state.lookahead) as usize;
        *result = deflate_get_dictionary(
            DeflateDictionaryRequest {
                window,
                start: end - len,
                len,
            },
            dictionary,
            dict_length,
        );
        return Some((strm, state, storage));
    }
    Some((strm, state, storage))
}

// Insert the initial dictionary strings into the hash chains.  The caller
// supplies bounded views of the state allocations, so this kernel can keep
// the cursor arithmetic and table updates entirely pointer-free.
fn insert_dictionary_hashes(
    window: &[crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    mut ins_h: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    mut str: crate::stdlib::uInt,
    count: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::stdlib::uInt) {
    for _ in 0..count {
        let byte_index = str
            .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
            .wrapping_sub(1) as usize;
        ins_h = ((ins_h << hash_shift) ^ window[byte_index] as crate::stdlib::uInt) & hash_mask;
        let prev_index = (str & w_mask) as usize;
        let head_index = ins_h as usize;
        prev[prev_index] = head[head_index];
        head[head_index] = str as crate::src::deflate::Pos as crate::src::deflate::Posf;
        str = str.wrapping_add(1);
    }
    (str, ins_h)
}

// Insert one current string into the hash chains through bounded allocation
// views.  The deflate modes retain their state/cursor policy, while this
// kernel owns the byte and table indexing.
fn insert_hash(
    window: &[crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    ins_h: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::src::deflate::IPos) {
    let next_hash = ((ins_h << hash_shift)
        ^ window
            [strstart.wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt - 1) as usize]
            as crate::stdlib::uInt)
        & hash_mask;
    let prev_index = (strstart & w_mask) as usize;
    let head_index = next_hash as usize;
    prev[prev_index] = head[head_index];
    let hash_head = prev[prev_index] as crate::src::deflate::IPos;
    head[head_index] = strstart as crate::src::deflate::Pos as crate::src::deflate::Posf;
    (next_hash, hash_head)
}

// The slow matcher emits either a length/distance pair or one delayed literal
// into the deferred-symbol suffix of the pending allocation.  Keep the tree
// accounting itself independent of the ABI-backed deflate state so the block
// algorithm only has to project the bounded symbol view at its boundary.
fn tally_slow_symbol(
    sym_buf: &mut [crate::stdlib::Bytef],
    sym_next: &mut crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    dyn_ltree: &mut [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &mut [crate::src::deflate::ct_data_s; 61],
    matches: &mut crate::stdlib::uInt,
    dist: ::core::ffi::c_uint,
    len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    crate::src::trees::tally_symbol(
        sym_buf, sym_next, sym_end, dyn_ltree, dyn_dtree, matches, dist, len,
    )
}

// The lazy matcher keeps a one-byte delayed decision.  Its policy and symbol
// accounting are entirely scalar/slice based; only the caller performs the
// raw allocation projections needed to refill, search, or flush a block.
struct SlowMatchState {
    prev_length: crate::stdlib::uInt,
    match_length: crate::stdlib::uInt,
    prev_match: crate::src::deflate::IPos,
    match_available: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
}

enum SlowMatchAction {
    Match {
        insert_start: crate::stdlib::uInt,
        insert_count: crate::stdlib::uInt,
        bflush: ::core::ffi::c_int,
    },
    Literal(::core::ffi::c_int),
    Defer,
}

fn advance_slow_match(
    state: &mut SlowMatchState,
    sym_buf: &mut [crate::stdlib::Bytef],
    sym_next: &mut crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    dyn_ltree: &mut [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &mut [crate::src::deflate::ct_data_s; 61],
    matches: &mut crate::stdlib::uInt,
    delayed_literal: Option<crate::zutil_h::uch>,
) -> SlowMatchAction {
    if state.prev_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
        && state.match_length <= state.prev_length
    {
        let max_insert = state
            .strstart
            .wrapping_add(state.lookahead)
            .wrapping_sub(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
        let len = state.prev_length.wrapping_sub(3) as crate::zutil_h::uch;
        let dist = (state.strstart as crate::src::deflate::IPos)
            .wrapping_sub(1)
            .wrapping_sub(state.prev_match) as crate::zutil_h::ush;
        let bflush = tally_slow_symbol(
            sym_buf,
            sym_next,
            sym_end,
            dyn_ltree,
            dyn_dtree,
            matches,
            dist as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
        );
        let insert_count = state
            .prev_length
            .wrapping_sub(2)
            .min(max_insert.wrapping_sub(state.strstart));
        let insert_start = state.strstart.wrapping_add(1);
        state.lookahead = state
            .lookahead
            .wrapping_sub(state.prev_length.wrapping_sub(1));
        state.strstart = state
            .strstart
            .wrapping_add(state.prev_length.wrapping_sub(1));
        state.prev_length = 0;
        state.match_available = 0;
        state.match_length = crate::zutil_h::MIN_MATCH as crate::stdlib::uInt - 1;
        SlowMatchAction::Match {
            insert_start,
            insert_count,
            bflush,
        }
    } else if state.match_available != 0 {
        let bflush = tally_slow_symbol(
            sym_buf,
            sym_next,
            sym_end,
            dyn_ltree,
            dyn_dtree,
            matches,
            0,
            delayed_literal.expect("delayed literal is available") as ::core::ffi::c_uint,
        );
        SlowMatchAction::Literal(bflush)
    } else {
        state.match_available = 1;
        state.strstart = state.strstart.wrapping_add(1);
        state.lookahead = state.lookahead.wrapping_sub(1);
        SlowMatchAction::Defer
    }
}

fn initial_hash(
    window: &[crate::stdlib::Bytef],
    strstart: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    (((window[strstart as usize] as crate::stdlib::uInt) << hash_shift)
        ^ window[strstart.wrapping_add(1) as usize] as crate::stdlib::uInt)
        & hash_mask
}

// The dictionary setter only needs a small scalar portion of the deflate
// state once its three callback-owned allocations have been projected.  Keep
// that policy separate so dictionary copying and hash construction never
// need to touch ABI stream cursors or raw allocations.
struct DictionaryState {
    wrap: ::core::ffi::c_int,
    status: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    slid: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    insert: crate::stdlib::uInt,
    ins_h: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    match_length: crate::stdlib::uInt,
    match_available: ::core::ffi::c_int,
    high_water: crate::zutil_h::ulg,
}

// This is the pointer-free dictionary transaction.  The stream adapter below
// is responsible for projecting callback-owned storage; this operation owns
// only the bounded views and scalar state needed to preserve zlib's dictionary
// admission, hash construction, and checksum rules.
fn deflateSetDictionary(
    state: &mut DictionaryState,
    window: &mut [crate::stdlib::Bytef],
    head: &mut [crate::src::deflate::Posf],
    prev: &mut [crate::src::deflate::Posf],
    dictionary: &[crate::stdlib::Bytef],
    checksum: crate::stdlib::uLong,
) -> Result<Option<crate::stdlib::uLong>, ()> {
    let wrap = state.wrap;
    if wrap == 2
        || wrap == 1 && state.status != crate::src::deflate::INIT_STATE
        || state.lookahead != 0
    {
        return Err(());
    }
    let checksum = (wrap == 1).then(|| crate::src::adler32::adler32(checksum, dictionary));
    state.wrap = 0;
    let dictionary = if dictionary.len() >= state.w_size as usize {
        if wrap == 0 {
            clear_hash_table(head);
            state.slid = 0;
            state.strstart = 0;
            state.block_start = 0;
            state.insert = 0;
        }
        &dictionary[dictionary.len() - state.w_size as usize..]
    } else {
        dictionary
    };
    window[..dictionary.len()].copy_from_slice(dictionary);
    state.lookahead = dictionary.len() as crate::stdlib::uInt;
    if state.lookahead.wrapping_add(state.insert)
        >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
    {
        state.ins_h = initial_hash(window, state.strstart, state.hash_shift, state.hash_mask);
    }
    while state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        let str = state.strstart;
        let count = state
            .lookahead
            .wrapping_sub((crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt);
        (state.strstart, state.ins_h) = insert_dictionary_hashes(
            window,
            head,
            prev,
            state.ins_h,
            state.hash_shift,
            state.hash_mask,
            state.w_mask,
            str,
            count,
        );
        state.lookahead = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
    }
    state.strstart = state.strstart.wrapping_add(state.lookahead);
    state.block_start = state.strstart as ::core::ffi::c_long;
    state.insert = state.lookahead;
    state.lookahead = 0;
    state.prev_length = (crate::zutil_h::MIN_MATCH - 1) as crate::stdlib::uInt;
    state.match_length = state.prev_length;
    state.match_available = 0;
    state.high_water =
        initialize_window_high_water(window, state.high_water, state.strstart, state.lookahead);
    state.wrap = wrap;
    Ok(checksum)
}

#[export_name = "deflateSetDictionary"]

pub unsafe extern "C" fn deflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(dictionary) = ::core::ptr::NonNull::new(dictionary.cast_mut()) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let dictionary =
        ::core::slice::from_raw_parts(dictionary.as_ptr().cast_const(), dictLength as usize);
    let mut result = crate::zlib_h::Z_STREAM_ERROR;
    let _ = deflate_stream_and_state(
        strm,
        DeflateStorageProjection::DictionaryInstall {
            dictionary,
            result: &mut result,
        },
    );
    result
}
// A bounded snapshot of the contiguous deflate history.  The callback-owned
// window remains borrowed only by the stream/state projection, while the
// output policy below operates exclusively on this pointer-free request.
struct DeflateDictionaryRequest<'window> {
    window: &'window [crate::stdlib::Bytef],
    start: usize,
    len: usize,
}

impl DeflateDictionaryRequest<'_> {
    fn dictionary_len(&self) -> usize {
        self.len
    }
}

fn deflate_get_dictionary(
    request: DeflateDictionaryRequest<'_>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    if let Some(output) = dictionary {
        output[..request.len]
            .copy_from_slice(&request.window[request.start..request.start + request.len]);
    }
    if let Some(dict_length) = dict_length {
        *dict_length = request.len as crate::stdlib::uInt;
    }
    crate::zlib_h::Z_OK
}
#[export_name = "deflateGetDictionary"]

pub unsafe extern "C" fn deflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // zlib guarantees that a 32 KiB output buffer is sufficient.  Form this
    // bounded optional view before stream/state dispatch, leaving the named
    // implementation to choose and copy only the live history prefix.
    let dictionary = if dictionary.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(
            dictionary,
            1usize << crate::stdlib::MAX_WBITS,
        ))
    };
    let mut result = crate::zlib_h::Z_STREAM_ERROR;
    let _ = deflate_stream_and_state(
        strm,
        DeflateStorageProjection::DictionaryQuery {
            dictionary,
            dict_length: dictLength.as_mut(),
            result: &mut result,
        },
    );
    result
}
// Reset state that contains no allocation handles or stream backlinks.  The
// ABI-facing caller projects these fields once, leaving reset policy and tree
// initialization in this pointer-free core.
fn reset_keep_core(
    pending: &mut crate::zutil_h::ulg,
    pending_out: &mut usize,
    wrap: &mut ::core::ffi::c_int,
    status: &mut ::core::ffi::c_int,
    last_flush: &mut ::core::ffi::c_int,
    dyn_ltree: &mut [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &mut [crate::src::deflate::ct_data_s; 61],
    bl_tree: &mut [crate::src::deflate::ct_data_s; 39],
    l_desc: &mut crate::src::deflate::tree_desc_s,
    d_desc: &mut crate::src::deflate::tree_desc_s,
    bl_desc: &mut crate::src::deflate::tree_desc_s,
    static_len: &mut crate::zutil_h::ulg,
    opt_len: &mut crate::zutil_h::ulg,
    matches: &mut crate::stdlib::uInt,
    sym_next: &mut crate::stdlib::uInt,
    bi_buf: &mut crate::zutil_h::ush,
    bi_valid: &mut ::core::ffi::c_int,
    bi_used: &mut ::core::ffi::c_int,
) -> crate::stdlib::uLong {
    *pending = 0;
    *pending_out = 0;
    if *wrap < 0 {
        *wrap = -*wrap;
    }
    *status = if *wrap == 2 {
        crate::src::deflate::GZIP_STATE
    } else {
        crate::src::deflate::INIT_STATE
    };
    *last_flush = -2;
    crate::src::trees::tr_init(
        dyn_ltree, dyn_dtree, bl_tree, l_desc, d_desc, bl_desc, static_len, opt_len, matches,
        sym_next, bi_buf, bi_valid, bi_used,
    );
    if *wrap == 2 {
        crate::src::crc32::crc32_z(0, None)
    } else {
        crate::src::adler32::adler32_z(0, None)
    }
}

// The full reset policy is independent of the ABI stream and of the callback
// allocation handles.  Keep it over ordinary references so initialization and
// reset share exactly the same lifecycle once their callers have made the one
// bounded head-table view.
struct DeflateResetCore<'a> {
    window_size: &'a mut crate::zutil_h::ulg,
    slid: &'a mut ::core::ffi::c_int,
    max_lazy_match: &'a mut crate::stdlib::uInt,
    good_match: &'a mut crate::stdlib::uInt,
    nice_match: &'a mut ::core::ffi::c_int,
    max_chain_length: &'a mut crate::stdlib::uInt,
    strstart: &'a mut crate::stdlib::uInt,
    block_start: &'a mut ::core::ffi::c_long,
    lookahead: &'a mut crate::stdlib::uInt,
    insert: &'a mut crate::stdlib::uInt,
    prev_length: &'a mut crate::stdlib::uInt,
    match_length: &'a mut crate::stdlib::uInt,
    match_available: &'a mut ::core::ffi::c_int,
    ins_h: &'a mut crate::stdlib::uInt,
}

impl DeflateResetCore<'_> {
    fn reset_after_keep(
        &mut self,
        head: &mut [crate::src::deflate::Posf],
        w_size: crate::stdlib::uInt,
        config: &config,
    ) {
        *self.window_size = (2 as ::core::ffi::c_long as crate::zutil_h::ulg)
            .wrapping_mul(w_size as crate::zutil_h::ulg);
        clear_hash_table(head);
        *self.slid = 0;
        *self.max_lazy_match = config.max_lazy as crate::stdlib::uInt;
        *self.good_match = config.good_length as crate::stdlib::uInt;
        *self.nice_match = config.nice_length as ::core::ffi::c_int;
        *self.max_chain_length = config.max_chain as crate::stdlib::uInt;
        *self.strstart = 0;
        *self.block_start = 0;
        *self.lookahead = 0;
        *self.insert = 0;
        *self.prev_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        *self.match_length = *self.prev_length;
        *self.match_available = 0;
        *self.ins_h = 0;
    }
}

// This owner deliberately exposes only the state slots reset by
// `deflateResetKeep()`.  Its signature is pointer-free, allowing reset policy
// to stay safe while the stream-lifetime-bound opaque-state projection remains
// in the non-FFI adapter below.
struct DeflateResetKeepOwner<'state> {
    data_type: &'state mut ::core::ffi::c_int,
    pending: &'state mut crate::zutil_h::ulg,
    pending_out: &'state mut usize,
    wrap: &'state mut ::core::ffi::c_int,
    status: &'state mut ::core::ffi::c_int,
    last_flush: &'state mut ::core::ffi::c_int,
    dyn_ltree: &'state mut [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &'state mut [crate::src::deflate::ct_data_s; 61],
    bl_tree: &'state mut [crate::src::deflate::ct_data_s; 39],
    l_desc: &'state mut crate::src::deflate::tree_desc_s,
    d_desc: &'state mut crate::src::deflate::tree_desc_s,
    bl_desc: &'state mut crate::src::deflate::tree_desc_s,
    static_len: &'state mut crate::zutil_h::ulg,
    opt_len: &'state mut crate::zutil_h::ulg,
    matches: &'state mut crate::stdlib::uInt,
    sym_next: &'state mut crate::stdlib::uInt,
    bi_buf: &'state mut crate::zutil_h::ush,
    bi_valid: &'state mut ::core::ffi::c_int,
    bi_used: &'state mut ::core::ffi::c_int,
}

// The reset implementation has no ABI handles or allocation views.  Keep it
// safe so both stream reset variants share the exact state transition.
fn deflateResetKeep(owner: DeflateResetKeepOwner<'_>) -> crate::stdlib::uLong {
    *owner.data_type = crate::zlib_h::Z_UNKNOWN;
    reset_keep_core(
        owner.pending,
        owner.pending_out,
        owner.wrap,
        owner.status,
        owner.last_flush,
        owner.dyn_ltree,
        owner.dyn_dtree,
        owner.bl_tree,
        owner.l_desc,
        owner.d_desc,
        owner.bl_desc,
        owner.static_len,
        owner.opt_len,
        owner.matches,
        owner.sym_next,
        owner.bi_buf,
        owner.bi_valid,
        owner.bi_used,
    )
}

// The reset variants differ only after their common keep-reset transition.
// Keep both phases under one stream/state projection: reopening `strm.state`
// after the keep reset would create a second raw-state transaction for the
// full reset.
pub(crate) enum DeflateResetKind {
    Keep,
    Full,
}

// The FFI wrapper validates and borrows the stream handle.  This adapter owns
// the opaque-state projection and stream publication, leaving both reset
// cores free of raw-pointer-carrying types.
pub(crate) unsafe fn deflate_reset_keep_from_stream(
    strm: &mut crate::zlib_h::z_stream_s,
    kind: DeflateResetKind,
) -> ::core::ffi::c_int {
    let projection = match kind {
        DeflateResetKind::Keep => DeflateStorageProjection::None,
        DeflateResetKind::Full => DeflateStorageProjection::Hash,
    };
    let Some((strm, state, mut storage)) = deflate_stream_and_state(strm, projection) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let adler = deflateResetKeep(DeflateResetKeepOwner {
        data_type: &mut state.data_type,
        pending: &mut state.pending,
        pending_out: &mut state.pending_out,
        wrap: &mut state.wrap,
        status: &mut state.status,
        last_flush: &mut state.last_flush,
        dyn_ltree: &mut state.dyn_ltree,
        dyn_dtree: &mut state.dyn_dtree,
        bl_tree: &mut state.bl_tree,
        l_desc: &mut state.l_desc,
        d_desc: &mut state.d_desc,
        bl_desc: &mut state.bl_desc,
        static_len: &mut state.static_len,
        opt_len: &mut state.opt_len,
        matches: &mut state.matches,
        sym_next: &mut state.sym_next,
        bi_buf: &mut state.bi_buf,
        bi_valid: &mut state.bi_valid,
        bi_used: &mut state.bi_used,
    });
    strm.total_out = 0;
    strm.total_in = 0;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = crate::zlib_h::Z_UNKNOWN;
    strm.adler = adler;
    if matches!(kind, DeflateResetKind::Full) {
        // The shared projection validates the callback-backed table's exact
        // `hash_size` extent before this pointer-free reset core receives it.
        let head = storage.head.take().expect("full-reset hash projection");
        let w_size = state.w_size;
        let config = &configuration_table[state.level as usize];
        DeflateResetCore {
            window_size: &mut state.window_size,
            slid: &mut state.slid,
            max_lazy_match: &mut state.max_lazy_match,
            good_match: &mut state.good_match,
            nice_match: &mut state.nice_match,
            max_chain_length: &mut state.max_chain_length,
            strstart: &mut state.strstart,
            block_start: &mut state.block_start,
            lookahead: &mut state.lookahead,
            insert: &mut state.insert,
            prev_length: &mut state.prev_length,
            match_length: &mut state.match_length,
            match_available: &mut state.match_available,
            ins_h: &mut state.ins_h,
        }
        .reset_after_keep(head, w_size, config);
    }
    crate::zlib_h::Z_OK
}
#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_reset_keep_from_stream(strm, DeflateResetKind::Keep)
}
#[export_name = "deflateReset"]

pub unsafe extern "C" fn deflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_reset_keep_from_stream(strm, DeflateResetKind::Full)
}
// Header registration is ordinary owned-state policy once the ABI header has
// been copied.  Keeping the mode check and replacement here lets the boundary
// project the stream/state once, and keeps a future pointer-free deflate
// owner from having to reproduce gzip-header lifetime rules.
fn install_gzip_header(
    wrap: ::core::ffi::c_int,
    slot: &mut Option<GzipHeader>,
    header: Option<GzipHeader>,
) -> ::core::ffi::c_int {
    if wrap != 2 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    *slot = header;
    crate::zlib_h::Z_OK
}

pub unsafe fn deflateSetHeader(
    strm: &mut crate::zlib_h::z_stream_s,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some((_strm, state, _storage)) =
        deflate_stream_and_state(strm, DeflateStorageProjection::None)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Keep the state projection at this ABI boundary; header replacement
    // below is entirely pointer-free once the caller-owned bytes are copied.
    let header = if head.is_null() {
        None
    } else {
        let header = &*head;
        let extra_len = header.extra_len & 0xffff as crate::stdlib::uInt;
        Some(GzipHeader {
            text: header.text,
            time: header.time,
            os: header.os,
            extra_len: header.extra_len,
            extra: header.extra.map(|extra| {
                ::core::slice::from_raw_parts(extra.as_ptr(), extra_len as usize).into()
            }),
            name: if header.name.is_null() {
                None
            } else {
                Some(
                    ::std::ffi::CStr::from_ptr(header.name.cast())
                        .to_bytes_with_nul()
                        .into(),
                )
            },
            comment: if header.comment.is_null() {
                None
            } else {
                Some(
                    ::std::ffi::CStr::from_ptr(header.comment.cast())
                        .to_bytes_with_nul()
                        .into(),
                )
            },
            hcrc: header.hcrc != 0,
        })
    };
    install_gzip_header(state.wrap, &mut state.gzhead, header)
}
#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflateSetHeader(strm, head)
}

fn deflate_pending_impl(
    pending_count: crate::zutil_h::ulg,
    bi_valid: ::core::ffi::c_int,
) -> (::core::ffi::c_uint, ::core::ffi::c_int, ::core::ffi::c_int) {
    let pending = pending_count as ::core::ffi::c_uint;
    if pending as crate::zutil_h::ulg != pending_count {
        (
            -1 as ::core::ffi::c_int as ::core::ffi::c_uint,
            bi_valid,
            crate::zlib_h::Z_BUF_ERROR,
        )
    } else {
        (pending, bi_valid, crate::zlib_h::Z_OK)
    }
}
#[export_name = "deflatePending"]

pub unsafe extern "C" fn deflatePending_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflateTune(
        strm,
        DeflateScalarAction::Pending {
            pending: pending.as_mut(),
            bits: bits.as_mut(),
        },
    )
}

fn deflate_used_impl(
    bi_used: ::core::ffi::c_int,
    bits: Option<&mut ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    if let Some(bits) = bits {
        *bits = bi_used;
    }
    crate::zlib_h::Z_OK
}

#[export_name = "deflateUsed"]

pub unsafe extern "C" fn deflateUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflateTune(
        strm,
        DeflateScalarAction::Used {
            bits: bits.as_mut(),
        },
    )
}

fn deflate_prime_bits(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    bi_buf: &mut crate::zutil_h::ush,
    bi_valid: &mut ::core::ffi::c_int,
    pending_out: usize,
    lit_bufsize: crate::stdlib::uInt,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if bits < 0
        || bits > 16
        || pending_out.wrapping_add(((crate::src::deflate::Buf_size + 7) >> 3) as usize)
            > lit_bufsize as usize
    {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    loop {
        let mut put = crate::src::deflate::Buf_size - *bi_valid;
        if put > bits {
            put = bits;
        }
        *bi_buf = (*bi_buf as ::core::ffi::c_int
            | ((value & ((1 as ::core::ffi::c_int) << put) - 1) << *bi_valid) as crate::zutil_h::ush
                as ::core::ffi::c_int) as crate::zutil_h::ush;
        *bi_valid += put;
        crate::src::trees::flush_pending_bits(pending_buf, pending, bi_buf, bi_valid);
        value >>= put;
        bits -= put;
        if bits == 0 {
            break;
        }
    }
    crate::zlib_h::Z_OK
}

#[export_name = "deflatePrime"]

pub unsafe extern "C" fn deflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflateTune(strm, DeflateScalarAction::Prime { bits, value })
}

// Level changes have a small amount of hash-table cleanup policy, but none
// of that policy depends on ABI cursors or callback-owned pointers.  Keep it
// in a slice/scalar core so the raw allocation views stay at the caller's
// boundary.  `tables` is present exactly for the level-zero match history
// case that needs it; the caller establishes those bounded views first.
// This owner is the pointer-free half of a parameter transition.  The ABI
// adapter constructs it only after projecting the callback-backed hash
// tables; the eventual deflate storage owner can construct the same value
// without reopening the ABI state.
struct DeflateParameterOwner<'state> {
    current_level: &'state mut ::core::ffi::c_int,
    current_strategy: &'state mut ::core::ffi::c_int,
    matches: &'state mut crate::stdlib::uInt,
    slid: &'state mut ::core::ffi::c_int,
    max_lazy_match: &'state mut crate::stdlib::uInt,
    good_match: &'state mut crate::stdlib::uInt,
    nice_match: &'state mut ::core::ffi::c_int,
    max_chain_length: &'state mut crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    tables: Option<(
        &'state mut [crate::src::deflate::Posf],
        Option<&'state mut [crate::src::deflate::Posf]>,
    )>,
}

// Keep the callback-owned hash allocations together while their bounded
// views are live.  This is deliberately pointer-free: the stream adapter
// proves the callback allocation geometry once, then parameter policy can
// neither reopen the ABI state nor recover a callback handle.
struct DeflateCallbackHashStorage<'storage> {
    head: &'storage mut [crate::src::deflate::Posf],
    prev: Option<&'storage mut [crate::src::deflate::Posf]>,
}

// These are precisely the scalar state slots a level/strategy transition may
// alter.  Separating them from `internal_state` lets the policy remain safe
// even while the callback-backed storage itself is still projected at the
// stream boundary.
struct DeflateParameterScalars<'state> {
    current_level: &'state mut ::core::ffi::c_int,
    current_strategy: &'state mut ::core::ffi::c_int,
    matches: &'state mut crate::stdlib::uInt,
    slid: &'state mut ::core::ffi::c_int,
    max_lazy_match: &'state mut crate::stdlib::uInt,
    good_match: &'state mut crate::stdlib::uInt,
    nice_match: &'state mut ::core::ffi::c_int,
    max_chain_length: &'state mut crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
}

impl<'state> DeflateParameterOwner<'state> {
    fn from_callback_storage(
        scalars: DeflateParameterScalars<'state>,
        storage: Option<DeflateCallbackHashStorage<'state>>,
    ) -> Self {
        let DeflateParameterScalars {
            current_level,
            current_strategy,
            matches,
            slid,
            max_lazy_match,
            good_match,
            nice_match,
            max_chain_length,
            w_size,
        } = scalars;
        Self {
            current_level,
            current_strategy,
            matches,
            slid,
            max_lazy_match,
            good_match,
            nice_match,
            max_chain_length,
            w_size,
            tables: storage.map(|storage| (storage.head, storage.prev)),
        }
    }
}

impl DeflateParameterOwner<'_> {
    fn update(self, level: ::core::ffi::c_int, strategy: ::core::ffi::c_int) {
        let Self {
            current_level,
            current_strategy,
            matches,
            slid,
            max_lazy_match,
            good_match,
            nice_match,
            max_chain_length,
            w_size,
            tables,
        } = self;
        if *current_level != level {
            if *current_level == 0 && *matches != 0 {
                let (head, prev) = tables.expect("level-zero matches require hash tables");
                if *matches == 1 {
                    slide_hash_table(head, w_size);
                    slide_hash_table(
                        prev.expect("single level-zero match requires previous table"),
                        w_size,
                    );
                    *slid = 1;
                } else {
                    clear_hash_table(head);
                    *slid = 0;
                }
                *matches = 0;
            }
            *current_level = level;
            *max_lazy_match = configuration_table[level as usize].max_lazy as crate::stdlib::uInt;
            *good_match = configuration_table[level as usize].good_length as crate::stdlib::uInt;
            *nice_match = configuration_table[level as usize].nice_length as ::core::ffi::c_int;
            *max_chain_length =
                configuration_table[level as usize].max_chain as crate::stdlib::uInt;
        }
        *current_strategy = strategy;
    }
}

fn deflateParams(
    owner: DeflateParameterOwner<'_>,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    owner.update(level, strategy);
    crate::zlib_h::Z_OK
}

// The stream-facing adapter keeps the callback-owned table projection at the
// same boundary as opaque-state validation.  `deflateParams()` itself only
// receives the completed pointer-free owner above.
pub unsafe fn deflate_params_from_stream(
    strm: &mut crate::zlib_h::z_stream_s,
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
    // Project the ABI stream and its opaque state once for each phase.  The
    // block flush can re-enter the dispatcher, so do not retain either Rust
    // borrow across that call; reproject afterwards instead of indexing the
    // raw cursors throughout the parameter policy.
    let needs_flush = {
        let Some((_stream, state, _storage)) =
            deflate_stream_and_state(strm, DeflateStorageProjection::None)
        else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let algorithm = configuration_table[state.level as usize].algorithm;
        (strategy != state.strategy || algorithm != configuration_table[level as usize].algorithm)
            && state.last_flush != -2 as ::core::ffi::c_int
    };
    if needs_flush {
        let mut err: ::core::ffi::c_int =
            deflate_dispatch_from_abi_stream(strm, crate::zlib_h::Z_BLOCK);
        if err == crate::zlib_h::Z_STREAM_ERROR {
            return err;
        }
        let flush_left_input = {
            let Some((stream, state, _storage)) =
                deflate_stream_and_state(strm, DeflateStorageProjection::None)
            else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            stream.avail_in != 0
                || state.strstart as ::core::ffi::c_long - state.block_start
                    + state.lookahead as ::core::ffi::c_long
                    != 0
        };
        if flush_left_input {
            return crate::zlib_h::Z_BUF_ERROR;
        }
    }
    let Some((_stream, state, storage)) =
        deflate_stream_and_state(strm, DeflateStorageProjection::Dictionary)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let needs_table_cleanup = state.level != level && state.level == 0 && state.matches != 0;
    let tables = if needs_table_cleanup {
        // `head` and (for the single-match case) `prev` are the bounded
        // callback-storage views established with state validation above.
        let prev = if state.matches == 1 {
            Some(storage.prev.expect("parameter previous-table projection"))
        } else {
            None
        };
        Some(DeflateCallbackHashStorage {
            head: storage.head.expect("parameter hash-table projection"),
            prev,
        })
    } else {
        None
    };
    let scalars = DeflateParameterScalars {
        current_level: &mut state.level,
        current_strategy: &mut state.strategy,
        matches: &mut state.matches,
        slid: &mut state.slid,
        max_lazy_match: &mut state.max_lazy_match,
        good_match: &mut state.good_match,
        nice_match: &mut state.nice_match,
        max_chain_length: &mut state.max_chain_length,
        w_size: state.w_size,
    };
    deflateParams(
        DeflateParameterOwner::from_callback_storage(scalars, tables),
        level,
        strategy,
    )
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
    deflate_params_from_stream(strm, level, strategy)
}
fn deflate_tune_values(
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) -> (
    crate::stdlib::uInt,
    crate::stdlib::uInt,
    ::core::ffi::c_int,
    crate::stdlib::uInt,
) {
    (
        good_length as crate::stdlib::uInt,
        max_lazy as crate::stdlib::uInt,
        nice_length,
        max_chain as crate::stdlib::uInt,
    )
}

// The established tuning projection also covers scalar queries over the same
// validated opaque state. Each action carries only scalar inputs or an
// optional scalar output borrow, so it cannot retain the ABI stream or any
// callback-backed storage.
enum DeflateScalarAction<'a> {
    Prime {
        bits: ::core::ffi::c_int,
        value: ::core::ffi::c_int,
    },
    Pending {
        pending: Option<&'a mut ::core::ffi::c_uint>,
        bits: Option<&'a mut ::core::ffi::c_int>,
    },
    Used {
        bits: Option<&'a mut ::core::ffi::c_int>,
    },
    Tune {
        good_length: ::core::ffi::c_int,
        max_lazy: ::core::ffi::c_int,
        nice_length: ::core::ffi::c_int,
        max_chain: ::core::ffi::c_int,
    },
}

// The export wrapper owns the nullable ABI-stream conversion.  Retuning only
// mutates scalar state, so this shared implementation retains the one
// opaque-state projection and keeps each action's policy over ordinary
// values.
unsafe fn deflateTune(
    strm: &mut crate::zlib_h::z_stream_s,
    action: DeflateScalarAction<'_>,
) -> ::core::ffi::c_int {
    let projection = match action {
        DeflateScalarAction::Prime { .. } => DeflateStorageProjection::Pending,
        DeflateScalarAction::Pending { .. }
        | DeflateScalarAction::Used { .. }
        | DeflateScalarAction::Tune { .. } => DeflateStorageProjection::None,
    };
    let Some((_strm, s, storage)) = deflate_stream_and_state(strm, projection) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    match action {
        DeflateScalarAction::Prime { bits, value } => {
            let pending_buf = storage.pending.expect("initialized pending buffer");
            deflate_prime_bits(
                pending_buf,
                &mut s.pending,
                &mut s.bi_buf,
                &mut s.bi_valid,
                s.pending_out,
                s.lit_bufsize,
                bits,
                value,
            )
        }
        DeflateScalarAction::Pending { pending, bits } => {
            let (pending_value, bits_value, status) = deflate_pending_impl(s.pending, s.bi_valid);
            if let Some(bits) = bits {
                *bits = bits_value;
            }
            if let Some(pending) = pending {
                *pending = pending_value;
                status
            } else {
                crate::zlib_h::Z_OK
            }
        }
        DeflateScalarAction::Used { bits } => deflate_used_impl(s.bi_used, bits),
        DeflateScalarAction::Tune {
            good_length,
            max_lazy,
            nice_length,
            max_chain,
        } => {
            let (good_match, max_lazy_match, nice_match, max_chain_length) =
                deflate_tune_values(good_length, max_lazy, nice_length, max_chain);
            s.good_match = good_match;
            s.max_lazy_match = max_lazy_match;
            s.nice_match = nice_match;
            s.max_chain_length = max_chain_length;
            crate::zlib_h::Z_OK
        }
    }
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
    deflateTune(
        strm,
        DeflateScalarAction::Tune {
            good_length,
            max_lazy,
            nice_length,
            max_chain,
        },
    )
}
struct DeflateBoundState {
    wrap: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    gzip_extra_len: Option<crate::stdlib::uInt>,
    gzip_name_len: Option<crate::stdlib::z_size_t>,
    gzip_comment_len: Option<crate::stdlib::z_size_t>,
    gzip_hcrc: bool,
    w_bits: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
}

fn deflate_bound_impl(
    source_len: crate::stdlib::z_size_t,
    state: Option<DeflateBoundState>,
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
    let mut wraplen = 0 as crate::stdlib::z_size_t;
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
            if let Some(extra_len) = state.gzip_extra_len {
                wraplen = wraplen
                    .wrapping_add((2 as crate::stdlib::uInt).wrapping_add(extra_len)
                        as crate::stdlib::z_size_t);
            }
            if let Some(name_len) = state.gzip_name_len {
                wraplen = wraplen.wrapping_add(name_len);
            }
            if let Some(comment_len) = state.gzip_comment_len {
                wraplen = wraplen.wrapping_add(comment_len);
            }
            if state.gzip_hcrc {
                wraplen = wraplen.wrapping_add(2 as crate::stdlib::z_size_t);
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

unsafe fn deflate_bound_state_for_stream(
    stream: &crate::zlib_h::z_stream_s,
    _stream_ptr: crate::zlib_h::z_streamp,
) -> Option<DeflateBoundState> {
    if stream.zalloc.is_none() || stream.zfree.is_none() {
        return None;
    }
    let state = stream
        .state?
        .cast::<crate::src::deflate::deflate_state>()
        .as_ref();
    if state.status != crate::src::deflate::INIT_STATE
        && state.status != crate::src::deflate::GZIP_STATE
        && state.status != crate::src::deflate::EXTRA_STATE
        && state.status != crate::src::deflate::NAME_STATE
        && state.status != crate::src::deflate::COMMENT_STATE
        && state.status != crate::src::deflate::HCRC_STATE
        && state.status != crate::src::deflate::BUSY_STATE
        && state.status != crate::src::deflate::FINISH_STATE
    {
        return None;
    }
    let mut gzip_extra_len = None;
    let mut gzip_name_len = None;
    let mut gzip_comment_len = None;
    let mut gzip_hcrc = false;
    if state.wrap == 2 as ::core::ffi::c_int || state.wrap == -2 as ::core::ffi::c_int {
        if let Some(header) = state.gzhead.as_ref() {
            if header.extra.is_some() {
                gzip_extra_len = Some(header.extra_len);
            }
            gzip_name_len = header.name.as_ref().map(|name| name.len());
            gzip_comment_len = header.comment.as_ref().map(|comment| comment.len());
            gzip_hcrc = header.hcrc;
        }
    }
    Some(DeflateBoundState {
        wrap: state.wrap,
        strstart: state.strstart,
        gzip_extra_len,
        gzip_name_len,
        gzip_comment_len,
        gzip_hcrc,
        w_bits: state.w_bits,
        hash_bits: state.hash_bits,
        level: state.level,
    })
}

fn deflateBound_z(
    source_len: crate::stdlib::z_size_t,
    state: Option<DeflateBoundState>,
) -> crate::stdlib::z_size_t {
    deflate_bound_impl(source_len, state)
}
#[export_name = "deflateBound_z"]

pub unsafe extern "C" fn deflateBound_z_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    let state = strm
        .as_ref()
        .and_then(|stream| deflate_bound_state_for_stream(stream, strm));
    deflateBound_z(sourceLen, state)
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let state = strm
        .as_ref()
        .and_then(|stream| deflate_bound_state_for_stream(stream, strm));
    deflateBound_z(sourceLen as crate::stdlib::z_size_t, state) as crate::stdlib::uLong
}

fn put_short_msb_bytes(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    value: crate::stdlib::uInt,
) {
    let start = *pending as usize;
    let Some(bytes) = pending_buf.get_mut(start..start.saturating_add(2)) else {
        return;
    };
    bytes[0] = (value >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte;
    bytes[1] = (value & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte;
    *pending = pending.wrapping_add(2);
}

// The zlib wrapper header depends only on scalar compression settings and a
// bounded pending buffer.  Keeping this emission separate from the ABI stream
// lets the dispatcher retain just one storage projection while an eventual
// owner-backed deflate state can reuse the exact same header logic.
fn append_zlib_header(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    w_bits: crate::stdlib::uInt,
    strategy: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    dictionary_adler: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let mut header: crate::stdlib::uInt = (crate::zlib_h::Z_DEFLATED as crate::stdlib::uInt)
        .wrapping_add(w_bits.wrapping_sub(8 as crate::stdlib::uInt) << 4 as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int;
    let level_flags = if strategy >= crate::zlib_h::Z_HUFFMAN_ONLY || level < 2 {
        0
    } else if level < 6 {
        1
    } else if level == 6 {
        2
    } else {
        3
    };
    header |= level_flags << 6 as ::core::ffi::c_int;
    let has_dictionary = strstart != 0;
    if has_dictionary {
        header |= crate::zutil_h::PRESET_DICT as crate::stdlib::uInt;
    }
    header = header.wrapping_add(
        (31 as crate::stdlib::uInt).wrapping_sub(header.wrapping_rem(31 as crate::stdlib::uInt)),
    );
    put_short_msb_bytes(pending_buf, pending, header);
    if has_dictionary {
        put_short_msb_bytes(
            pending_buf,
            pending,
            (dictionary_adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
        );
        put_short_msb_bytes(
            pending_buf,
            pending,
            (dictionary_adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
        );
    }
    crate::src::adler32::adler32_z(0 as crate::stdlib::uLong, None)
}

// Finish either wrapper format through the bounded pending buffer.  The
// caller decides when the resulting bytes are flushed and when the wrapper is
// marked complete; this core only owns the format-specific byte ordering.
fn append_deflate_trailer(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    wrap: ::core::ffi::c_int,
    check: crate::stdlib::uLong,
    total_in: crate::stdlib::uLong,
) {
    if wrap == 2 {
        append_pending_bytes(
            pending_buf,
            pending,
            &[
                (check & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                (check >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                (check >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                (check >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                (total_in & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                (total_in >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                (total_in >> 16 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
                (total_in >> 24 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                    as crate::stdlib::Byte,
            ],
        );
    } else {
        put_short_msb_bytes(
            pending_buf,
            pending,
            (check >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
        );
        put_short_msb_bytes(
            pending_buf,
            pending,
            (check & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
        );
    }
}

fn append_pending_bytes(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    bytes: &[crate::stdlib::Bytef],
) {
    let start = *pending as usize;
    let Some(end) = start.checked_add(bytes.len()) else {
        return;
    };
    let Some(output) = pending_buf.get_mut(start..end) else {
        return;
    };
    output.copy_from_slice(bytes);
    *pending = pending.wrapping_add(bytes.len() as crate::zutil_h::ulg);
}

fn push_pending_byte(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    byte: crate::stdlib::Bytef,
) {
    let index = *pending as usize;
    pending_buf[index] = byte;
    *pending = pending.wrapping_add(1);
}

// Build the fixed and optional portions of the gzip member header using only
// owned header metadata and the already-bounded pending allocation. `None`
// preserves the compact default header path; `Some(hcrc)` records whether the
// caller must checksum the bytes written here.
fn append_gzip_header_prefix(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    gzhead: Option<&GzipHeader>,
) -> Option<bool> {
    push_pending_byte(pending_buf, pending, 31);
    push_pending_byte(pending_buf, pending, 139);
    push_pending_byte(pending_buf, pending, 8);
    let xfl = if level == 9 {
        2
    } else if strategy >= 2 || level < 2 {
        4
    } else {
        0
    };
    let Some(gzhead) = gzhead else {
        for byte in [0, 0, 0, 0, 0, xfl, 3] {
            push_pending_byte(pending_buf, pending, byte);
        }
        return None;
    };
    push_pending_byte(
        pending_buf,
        pending,
        ((if gzhead.text != 0 { 1 } else { 0 })
            + (if gzhead.hcrc { 2 } else { 0 })
            + (if gzhead.extra.is_none() { 0 } else { 4 })
            + (if gzhead.name.is_none() { 0 } else { 8 })
            + (if gzhead.comment.is_none() { 0 } else { 16 })) as crate::stdlib::Bytef,
    );
    for shift in [0, 8, 16, 24] {
        push_pending_byte(
            pending_buf,
            pending,
            (gzhead.time >> shift & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
        );
    }
    push_pending_byte(pending_buf, pending, xfl);
    push_pending_byte(
        pending_buf,
        pending,
        (gzhead.os & 0xff as ::core::ffi::c_int) as crate::stdlib::Bytef,
    );
    if gzhead.extra.is_some() {
        push_pending_byte(
            pending_buf,
            pending,
            (gzhead.extra_len & 0xff as crate::stdlib::uInt) as crate::stdlib::Bytef,
        );
        push_pending_byte(
            pending_buf,
            pending,
            (gzhead.extra_len >> 8 & 0xff as crate::stdlib::uInt) as crate::stdlib::Bytef,
        );
    }
    Some(gzhead.hcrc)
}

// Copy as much of a NUL-terminated gzip header field as fits in the current
// pending buffer. The caller keeps the cursor and flush policy in the ABI
// state layer; this kernel only operates on bounded byte views.
fn append_gzip_cstring_bytes(
    source: &[crate::stdlib::Bytef],
    source_index: &mut usize,
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    hcrc: bool,
    checksum: &mut crate::stdlib::uLong,
) -> bool {
    let Some(remaining) = source.get(*source_index..) else {
        return true;
    };
    let available = pending_buf.len().saturating_sub(*pending as usize);
    if available == 0 {
        return false;
    }
    let limit = remaining.len().min(available);
    let count = remaining[..limit]
        .iter()
        .position(|&byte| byte == 0)
        .map_or(limit, |index| index + 1);
    let start = *pending as usize;
    let copied = &remaining[..count];
    pending_buf[start..start + count].copy_from_slice(copied);
    *source_index += count;
    *pending = pending.wrapping_add(count as crate::zutil_h::ulg);
    if hcrc {
        *checksum = crate::src::crc32::crc32_z(*checksum, Some(copied));
    }
    count != 0 && remaining[count - 1] == 0
}

// Emit as much of the owned gzip extra field as fits in the current pending
// slice. The dispatcher retains the ABI-backed allocation and flushes between
// calls; this helper owns only bounded byte, cursor, and checksum accounting.
fn append_gzip_extra_chunk(
    source: &[crate::stdlib::Bytef],
    source_index: &mut usize,
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    hcrc: bool,
    checksum: &mut crate::stdlib::uLong,
) -> bool {
    let Some(remaining) = source.get(*source_index..) else {
        return true;
    };
    let available = pending_buf.len().saturating_sub(*pending as usize);
    let copy_len = remaining.len().min(available);
    if copy_len == 0 {
        return remaining.is_empty();
    }
    let start = *pending as usize;
    let copied = &remaining[..copy_len];
    pending_buf[start..start + copy_len].copy_from_slice(copied);
    *pending = pending.wrapping_add(copy_len as crate::zutil_h::ulg);
    *source_index = source_index.wrapping_add(copy_len);
    if hcrc {
        *checksum = crate::src::crc32::crc32_z(*checksum, Some(copied));
    }
    copy_len == remaining.len()
}

fn flush_pending_bytes(
    output: &mut [crate::stdlib::Bytef],
    pending_buf: &[crate::stdlib::Bytef],
    pending_out: &mut usize,
    pending_len: &mut crate::zutil_h::ulg,
) -> crate::stdlib::uInt {
    let len = if *pending_len > output.len() as crate::zutil_h::ulg {
        output.len()
    } else {
        *pending_len as usize
    };
    if len == 0 {
        return 0;
    }
    output[..len].copy_from_slice(&pending_buf[*pending_out..*pending_out + len]);
    *pending_out += len;
    *pending_len = pending_len.wrapping_sub(len as crate::zutil_h::ulg);
    if *pending_len == 0 {
        *pending_out = 0;
    }
    len as crate::stdlib::uInt
}

// The pending allocation has two logical regions: queued output and deferred
// symbols.  Keep those ranges as ordinary Rust indices so the eventual owned
// pending storage can use this same description without reconstructing raw
// cursors.
struct PendingRegions {
    queued: ::core::ops::Range<usize>,
    symbols: ::core::ops::Range<usize>,
}

// Copying a deflate stream has a fixed set of allocation extents derived from
// scalar state.  Keep that geometry pointer-free so the eventual owned-state
// copy can use the same plan, rather than reconstructing lengths from raw
// allocation bases.
struct DeflateCopyLayout {
    window_bytes: usize,
    prev_entries: usize,
    head_entries: usize,
    pending: Option<PendingRegions>,
}

impl DeflateCopyLayout {
    // Keep every range check for the callback-owned buffers in the
    // pointer-free copy plan.  `deflateCopy()` performs raw copies only after
    // this validates the scalar geometry supplied by the source state.
    fn fits_storage(&self, storage: &DeflateStorageLayout) -> bool {
        let Some(window_bytes) = storage.window.byte_len() else {
            return false;
        };
        let Some(prev_entries) = storage.prev.element_len::<crate::src::deflate::Posf>() else {
            return false;
        };
        let Some(head_entries) = storage.head.element_len::<crate::src::deflate::Posf>() else {
            return false;
        };
        let Some(pending_bytes) = storage.pending.byte_len() else {
            return false;
        };

        self.window_bytes <= window_bytes
            && self.prev_entries <= prev_entries
            && self.head_entries <= head_entries
            && self
                .pending
                .as_ref()
                .is_some_and(|regions| regions.fits_within(pending_bytes))
    }
}

// Keep every scalar-derived decision for a deep copy in one pointer-free
// value.  The callback-owned allocation handles still cross the ABI boundary,
// but a future allocation broker can consume this plan without recovering
// either allocation geometry or copy ranges from raw storage.
struct DeflateCopyPlan {
    storage: DeflateStorageLayout,
    layout: DeflateCopyLayout,
}

// A copied state can be prepared without looking at either callback-owned
// allocation.  Keep that decision separate from the boundary adapter so the
// eventual owner-backed state can enter the same copy path directly.
struct DeflateCopyPreparation {
    payload: DeflateCopyPayload,
    plan: DeflateCopyPlan,
}

impl DeflateCopyPreparation {
    // Every storage owner reaches the same checked copy commit through this
    // pointer-free operation.  In particular, a callback-pairing owner can
    // project its allocations once, then use this without duplicating the
    // layout checks or teaching the copy core about callback handles.
    fn copy_storage(
        &self,
        source: DeflateCopySourceViews<'_>,
        destination: DeflateCopyDestinationViews<'_>,
    ) -> bool {
        deflateCopy(source, destination, &self.plan.layout)
    }

    // This is the pointer-free commit point for a deep copy.  The callback
    // boundary is still responsible for preserving zalloc/zfree pairing, but
    // once it has supplied owned storage it need not repeat the geometry or
    // range validation performed while preparing the copy.  Consume the
    // preparation with the copied storage so a future callback-pairing owner
    // can keep the scalar snapshot and its allocations together.
    fn into_owned_copy(self, source: &DeflateOwnedStorage) -> Option<DeflateOwnedCopy> {
        let mut destination = self.plan.storage.allocate_owned()?;
        DeflateCopyStorage::new(source.source_views(), destination.destination_views())
            .copy(&self)
            .then_some(DeflateOwnedCopy {
                preparation: self,
                storage: destination,
            })
    }
}

// This is the fully pointer-free form of a copied deflate state.  It keeps
// the validated scalar/header/tree preparation with the independently owned
// backing buffers, so an allocation broker can later replace only storage
// acquisition without reintroducing raw state or callback handles into the
// copy kernel.
struct DeflateOwnedCopy {
    preparation: DeflateCopyPreparation,
    storage: DeflateOwnedStorage,
}

impl DeflateOwnedCopy {
    fn payload(&self) -> &DeflateCopyPayload {
        &self.preparation.payload
    }

    fn storage(&self) -> &DeflateOwnedStorage {
        &self.storage
    }
}

// This is the safe half of the eventual custom-allocation owner.  It has the
// same four independently sized regions as zlib's callback allocation path,
// but exposes only owned slices to copy/reset cores.  The ABI adapter cannot
// use it until callback allocation and release are represented by that owner;
// keeping the representation here nevertheless lets the deep-copy algorithm
// be exercised without a state record or an allocator callback.
struct DeflateOwnedStorage {
    window: Box<[crate::stdlib::Bytef]>,
    prev: Box<[crate::src::deflate::Posf]>,
    head: Box<[crate::src::deflate::Posf]>,
    pending: Box<[crate::stdlib::Bytef]>,
}

// The owner-backed copy path must not need to know whether its storage came
// from a Rust allocation or a callback-pairing broker.  Keep the copy kernel
// on typed slice views; the broker alone will establish those views from its
// owned allocations after preserving the callback-visible allocation order.
struct DeflateCopySourceViews<'a> {
    window: &'a [crate::stdlib::Bytef],
    prev: &'a [crate::src::deflate::Posf],
    head: &'a [crate::src::deflate::Posf],
    pending: &'a [crate::stdlib::Bytef],
}

struct DeflateCopyDestinationViews<'a> {
    window: &'a mut [crate::stdlib::Bytef],
    prev: &'a mut [crate::src::deflate::Posf],
    head: &'a mut [crate::src::deflate::Posf],
    pending: &'a mut [crate::stdlib::Bytef],
}

// A copy transaction always needs both complete four-region view sets. Keep
// them paired in one pointer-free facade so a callback-allocation boundary
// can lend all eight bounded slices as one operation, rather than asking the
// copy core to reconstruct either allocation independently.
struct DeflateCopyStorage<'source, 'destination> {
    source: DeflateCopySourceViews<'source>,
    destination: DeflateCopyDestinationViews<'destination>,
}

impl<'source, 'destination> DeflateCopyStorage<'source, 'destination> {
    fn new(
        source: DeflateCopySourceViews<'source>,
        destination: DeflateCopyDestinationViews<'destination>,
    ) -> Self {
        Self {
            source,
            destination,
        }
    }

    fn copy(self, preparation: &DeflateCopyPreparation) -> bool {
        deflateCopy(self.source, self.destination, &preparation.plan.layout)
    }
}

impl DeflateOwnedStorage {
    // Keep the source-side view construction with the owner as well.  A
    // callback-pairing broker can hand the copy core two typed owners without
    // teaching that core how either owner was allocated.
    fn source_views(&self) -> DeflateCopySourceViews<'_> {
        DeflateCopySourceViews {
            window: self.window.as_ref(),
            prev: self.prev.as_ref(),
            head: self.head.as_ref(),
            pending: self.pending.as_ref(),
        }
    }

    fn destination_views(&mut self) -> DeflateCopyDestinationViews<'_> {
        DeflateCopyDestinationViews {
            window: self.window.as_mut(),
            prev: self.prev.as_mut(),
            head: self.head.as_mut(),
            pending: self.pending.as_mut(),
        }
    }
}

// Copy exactly the initialized logical ranges of a deflate state.  Both
// source and destination are already bounded typed views, so this remains
// usable by the eventual custom-allocation owner without raw projections.
// This is the pointer-free deflateCopy core.  The ABI adapter below is
// responsible only for callback-paired allocation and for projecting its
// callback-owned regions into this same operation.
fn deflateCopy(
    source: DeflateCopySourceViews<'_>,
    destination: DeflateCopyDestinationViews<'_>,
    layout: &DeflateCopyLayout,
) -> bool {
    if layout.window_bytes > source.window.len()
        || layout.window_bytes > destination.window.len()
        || layout.prev_entries > source.prev.len()
        || layout.prev_entries > destination.prev.len()
        || layout.head_entries > source.head.len()
        || layout.head_entries > destination.head.len()
        || layout.pending.as_ref().is_some_and(|regions| {
            !regions.fits_within(source.pending.len())
                || !regions.fits_within(destination.pending.len())
        })
    {
        return false;
    }
    destination.window[..layout.window_bytes]
        .copy_from_slice(&source.window[..layout.window_bytes]);
    destination.prev[..layout.prev_entries].copy_from_slice(&source.prev[..layout.prev_entries]);
    destination.head[..layout.head_entries].copy_from_slice(&source.head[..layout.head_entries]);
    copy_pending_regions(source.pending, destination.pending, layout.pending.as_ref());
    true
}

impl DeflateStorageLayout {
    fn allocate_owned(&self) -> Option<DeflateOwnedStorage> {
        fn allocate_zeroed<T: Clone>(len: usize, value: T) -> Option<Box<[T]>> {
            let mut storage = Vec::new();
            storage.try_reserve_exact(len).ok()?;
            storage.resize(len, value);
            Some(storage.into_boxed_slice())
        }

        Some(DeflateOwnedStorage {
            window: allocate_zeroed(self.window.byte_len()?, 0)?,
            prev: allocate_zeroed(self.prev.element_len::<crate::src::deflate::Posf>()?, 0)?,
            head: allocate_zeroed(self.head.element_len::<crate::src::deflate::Posf>()?, 0)?,
            pending: allocate_zeroed(self.pending.byte_len()?, 0)?,
        })
    }
}

// The tree bookkeeping is independent of the callback-owned state and buffer
// allocations.  Keep its deep-copy operation in a pointer-free value so the
// copy path can eventually hand only the allocation handles to the boundary
// owner.  In particular, do not derive `Clone` for `internal_state`: that
// would silently duplicate its callback-owned allocations.
struct DeflateTreeCopy {
    dyn_ltree: [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: [crate::src::deflate::ct_data_s; 61],
    bl_tree: [crate::src::deflate::ct_data_s; 39],
    l_desc: crate::src::deflate::tree_desc_s,
    d_desc: crate::src::deflate::tree_desc_s,
    bl_desc: crate::src::deflate::tree_desc_s,
    bl_count: [crate::zutil_h::ush; 16],
    heap: [::core::ffi::c_int; 573],
    heap_len: ::core::ffi::c_int,
    heap_max: ::core::ffi::c_int,
    depth: [crate::zutil_h::uch; 573],
}

// This is the callback-allocation-independent part of a copied state.  It is
// deliberately pointer-free: the four allocation handles and the ABI
// backlink are installed only at the callback boundary below.  Keeping the
// deep-copy payload separate prevents an eventual owned allocation broker
// from having to copy an `internal_state` and then repair aliased owners.
struct DeflateCopyPayload {
    data_type: ::core::ffi::c_int,
    status: ::core::ffi::c_int,
    pending_buf_size: crate::zutil_h::ulg,
    pending_out: usize,
    pending: crate::zutil_h::ulg,
    wrap: ::core::ffi::c_int,
    gzhead: Option<GzipHeader>,
    gzindex: usize,
    method: crate::stdlib::Byte,
    last_flush: ::core::ffi::c_int,
    w_size: crate::stdlib::uInt,
    w_bits: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    window_size: crate::zutil_h::ulg,
    ins_h: crate::stdlib::uInt,
    hash_size: crate::stdlib::uInt,
    hash_bits: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    match_length: crate::stdlib::uInt,
    prev_match: crate::src::deflate::IPos,
    match_available: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    max_chain_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    tree: DeflateTreeCopy,
    sym_buf_start: usize,
    lit_bufsize: crate::stdlib::uInt,
    sym_next: crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    opt_len: crate::zutil_h::ulg,
    static_len: crate::zutil_h::ulg,
    matches: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    bi_buf: crate::zutil_h::ush,
    bi_valid: ::core::ffi::c_int,
    bi_used: ::core::ffi::c_int,
    high_water: crate::zutil_h::ulg,
    slid: ::core::ffi::c_int,
}

impl DeflateCopyPayload {
    // The callback boundary receives this fully pointer-free snapshot before
    // it requests replacement storage.  Keep the allocation geometry with
    // that snapshot so a future custom-allocation owner does not need to
    // re-read callback-owned state merely to rebuild the copy plan.
    fn copy_plan(&self) -> DeflateCopyPlan {
        deflate_copy_plan(
            self.w_size,
            self.hash_size,
            self.lit_bufsize,
            self.high_water,
            self.slid,
            self.strstart,
            self.insert,
            self.pending_out,
            self.pending as usize,
            self.sym_buf_start,
            self.sym_next as usize,
        )
    }
}

fn prepare_deflate_copy(payload: DeflateCopyPayload) -> Option<DeflateCopyPreparation> {
    if !deflate_state_status_is_valid(payload.status) {
        return None;
    }
    let plan = payload.copy_plan();
    if !plan.layout.fits_storage(&plan.storage)
        || usize::try_from(payload.window_size).ok() != plan.storage.window.byte_len()
        || usize::try_from(payload.pending_buf_size).ok() != plan.storage.pending.byte_len()
    {
        return None;
    }
    Some(DeflateCopyPreparation { payload, plan })
}

fn copy_tree_desc(desc: &crate::src::deflate::tree_desc_s) -> crate::src::deflate::tree_desc_s {
    crate::src::deflate::tree_desc_s {
        kind: match &desc.kind {
            crate::src::deflate::TreeKind::LitLen => crate::src::deflate::TreeKind::LitLen,
            crate::src::deflate::TreeKind::Dist => crate::src::deflate::TreeKind::Dist,
            crate::src::deflate::TreeKind::BitLen => crate::src::deflate::TreeKind::BitLen,
        },
        max_code: desc.max_code,
    }
}

fn copy_deflate_tree_state(
    dyn_ltree: &[crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &[crate::src::deflate::ct_data_s; 61],
    bl_tree: &[crate::src::deflate::ct_data_s; 39],
    l_desc: &crate::src::deflate::tree_desc_s,
    d_desc: &crate::src::deflate::tree_desc_s,
    bl_desc: &crate::src::deflate::tree_desc_s,
    bl_count: &[crate::zutil_h::ush; 16],
    heap: &[::core::ffi::c_int; 573],
    heap_len: ::core::ffi::c_int,
    heap_max: ::core::ffi::c_int,
    depth: &[crate::zutil_h::uch; 573],
) -> DeflateTreeCopy {
    DeflateTreeCopy {
        dyn_ltree: ::core::array::from_fn(|index| crate::src::deflate::ct_data_s {
            fc: dyn_ltree[index].fc,
            dl: dyn_ltree[index].dl,
        }),
        dyn_dtree: ::core::array::from_fn(|index| crate::src::deflate::ct_data_s {
            fc: dyn_dtree[index].fc,
            dl: dyn_dtree[index].dl,
        }),
        bl_tree: ::core::array::from_fn(|index| crate::src::deflate::ct_data_s {
            fc: bl_tree[index].fc,
            dl: bl_tree[index].dl,
        }),
        l_desc: copy_tree_desc(l_desc),
        d_desc: copy_tree_desc(d_desc),
        bl_desc: copy_tree_desc(bl_desc),
        bl_count: *bl_count,
        heap: *heap,
        heap_len,
        heap_max,
        depth: *depth,
    }
}

fn deflate_copy_layout(
    high_water: crate::zutil_h::ulg,
    slid: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    storage: &DeflateStorageLayout,
    pending_out: usize,
    pending_len: usize,
    sym_buf_start: usize,
    sym_next: usize,
) -> DeflateCopyLayout {
    let prev_capacity = storage
        .prev
        .element_len::<crate::src::deflate::Posf>()
        .expect("validated previous-table allocation geometry");
    let prev_entries =
        if slid != 0 || strstart.wrapping_sub(insert) > prev_capacity as crate::stdlib::uInt {
            prev_capacity
        } else {
            strstart.wrapping_sub(insert) as usize
        };
    DeflateCopyLayout {
        window_bytes: high_water as usize,
        prev_entries,
        head_entries: storage
            .head
            .element_len::<crate::src::deflate::Posf>()
            .expect("validated hash-table allocation geometry"),
        pending: PendingRegions::new(pending_out, pending_len, sym_buf_start, sym_next),
    }
}

fn deflate_copy_plan(
    w_size: crate::stdlib::uInt,
    hash_size: crate::stdlib::uInt,
    lit_bufsize: crate::stdlib::uInt,
    high_water: crate::zutil_h::ulg,
    slid: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    pending_out: usize,
    pending_len: usize,
    sym_buf_start: usize,
    sym_next: usize,
) -> DeflateCopyPlan {
    let storage = DeflateStorageLayout::new(w_size, hash_size, lit_bufsize);
    let layout = deflate_copy_layout(
        high_water,
        slid,
        strstart,
        insert,
        &storage,
        pending_out,
        pending_len,
        sym_buf_start,
        sym_next,
    );
    DeflateCopyPlan { storage, layout }
}

impl PendingRegions {
    fn new(
        pending_out: usize,
        pending_len: usize,
        sym_buf_start: usize,
        sym_next: usize,
    ) -> Option<Self> {
        Some(Self {
            queued: pending_out..pending_out.checked_add(pending_len)?,
            symbols: sym_buf_start..sym_buf_start.checked_add(sym_next)?,
        })
    }

    fn copy_from(&self, source: &[crate::stdlib::Bytef], destination: &mut [crate::stdlib::Bytef]) {
        let Some(queued) = source.get(self.queued.clone()) else {
            return;
        };
        let Some(symbols) = source.get(self.symbols.clone()) else {
            return;
        };
        let Some(destination_queued) = destination.get_mut(self.queued.clone()) else {
            return;
        };
        destination_queued.copy_from_slice(queued);
        let Some(destination_symbols) = destination.get_mut(self.symbols.clone()) else {
            return;
        };
        destination_symbols.copy_from_slice(symbols);
    }

    fn fits_within(&self, len: usize) -> bool {
        self.queued.end <= len && self.symbols.end <= len
    }
}

// `deflateCopy()` duplicates those two logical regions.  Allocation setup
// establishes the extents before the implementation constructs these views;
// the copy order matches the two original memcpy operations.
fn copy_pending_regions(
    source: &[crate::stdlib::Bytef],
    destination: &mut [crate::stdlib::Bytef],
    regions: Option<&PendingRegions>,
) {
    let Some(regions) = regions else {
        return;
    };
    regions.copy_from(source, destination);
}

// `_tr_stored_block()` has already reserved these four bytes after winding up
// the bit buffer.  Patch that header through the declared pending allocation
// instead of reconstructing four raw cursors in the stored-block algorithm.
fn set_stored_block_length(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: crate::zutil_h::ulg,
    len: crate::stdlib::uInt,
) {
    let Some(start) = (pending as usize).checked_sub(4) else {
        return;
    };
    let Some(header) = pending_buf.get_mut(start..start.saturating_add(4)) else {
        return;
    };
    header[0] = len as crate::stdlib::Bytef;
    header[1] = (len >> 8) as crate::stdlib::Bytef;
    header[2] = !len as crate::stdlib::Bytef;
    header[3] = (!len >> 8) as crate::stdlib::Bytef;
}

// Pending flushing is a bounded byte operation.  The ABI dispatcher owns
// cursor publication; this kernel needs only the scalar bit/pending state and
// the two call-scoped storage views.
fn flush_pending(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    bi_buf: &mut crate::zutil_h::ush,
    bi_valid: &mut ::core::ffi::c_int,
    pending_out: &mut usize,
    output: &mut [crate::stdlib::Bytef],
    output_pos: &mut usize,
) -> ::core::ffi::c_uint {
    crate::src::trees::flush_pending_bits(pending_buf, pending, bi_buf, bi_valid);
    let len = (*pending).min(output.len().saturating_sub(*output_pos) as crate::zutil_h::ulg)
        as ::core::ffi::c_uint;
    if len == 0 as ::core::ffi::c_uint {
        return 0;
    }
    let len = flush_pending_bytes(
        &mut output[*output_pos..*output_pos + len as usize],
        pending_buf,
        pending_out,
        pending,
    );
    *output_pos += len as usize;
    len
}

// A deflate call accepts only the public flush range.  Keep validation and
// the no-progress ordering comparison in a pointer-free request so the
// eventual stream/state owner can make this decision before it projects ABI
// cursors or callback-backed storage.
#[derive(Clone, Copy)]
struct DeflateFlush(::core::ffi::c_int);

impl DeflateFlush {
    fn parse(flush: ::core::ffi::c_int) -> Option<Self> {
        (0..=crate::zlib_h::Z_BLOCK)
            .contains(&flush)
            .then_some(Self(flush))
    }

    fn raw(self) -> ::core::ffi::c_int {
        self.0
    }

    fn repeats_without_input(self, previous: ::core::ffi::c_int) -> bool {
        let rank = |flush: ::core::ffi::c_int| {
            flush * 2
                - if flush > crate::zlib_h::Z_FINISH {
                    9
                } else {
                    0
                }
        };
        rank(self.0) <= rank(previous) && self.0 != crate::zlib_h::Z_FINISH
    }
}

// One deflate invocation is expressed without ABI cursors or callback
// handles.  The stream adapter forms this view once, then publishes its
// consumed/produced offsets after the dispatch has released every slice.
struct DeflateDispatchStream<'input, 'output> {
    input: &'input [crate::stdlib::Bytef],
    output: &'output mut [crate::stdlib::Bytef],
    next_in: usize,
    next_out: usize,
    avail_in: crate::stdlib::uInt,
    avail_out: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    data_type: ::core::ffi::c_int,
    message: Option<::core::ffi::c_int>,
}

// This is deliberately a view rather than an owner of the callback-backed
// allocations.  Its fields are bounded slices or scalar state only; the
// provenance-carrying allocation handles remain in `internal_state` at the
// stream adapter.
struct DeflateDispatchState<'state> {
    pending_buf: &'state mut [crate::stdlib::Bytef],
    window: &'state mut [crate::stdlib::Bytef],
    prev: &'state mut [crate::src::deflate::Posf],
    head: &'state mut [crate::src::deflate::Posf],
    status: ::core::ffi::c_int,
    pending_buf_size: crate::zutil_h::ulg,
    pending_out: usize,
    pending: crate::zutil_h::ulg,
    wrap: ::core::ffi::c_int,
    gzhead: &'state mut Option<GzipHeader>,
    gzindex: usize,
    last_flush: ::core::ffi::c_int,
    w_size: crate::stdlib::uInt,
    w_bits: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    hash_size: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    match_length: crate::stdlib::uInt,
    prev_match: crate::src::deflate::IPos,
    match_available: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    max_chain_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    dyn_ltree: &'state mut [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &'state mut [crate::src::deflate::ct_data_s; 61],
    bl_tree: &'state mut [crate::src::deflate::ct_data_s; 39],
    l_desc: &'state mut crate::src::deflate::tree_desc_s,
    d_desc: &'state mut crate::src::deflate::tree_desc_s,
    bl_desc: &'state mut crate::src::deflate::tree_desc_s,
    bl_count: &'state mut [crate::zutil_h::ush; 16],
    heap: &'state mut [::core::ffi::c_int; 573],
    heap_len: ::core::ffi::c_int,
    heap_max: ::core::ffi::c_int,
    depth: &'state mut [crate::zutil_h::uch; 573],
    sym_buf_start: usize,
    sym_next: crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    opt_len: crate::zutil_h::ulg,
    static_len: crate::zutil_h::ulg,
    matches: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    ins_h: crate::stdlib::uInt,
    bi_buf: crate::zutil_h::ush,
    bi_valid: ::core::ffi::c_int,
    bi_used: ::core::ffi::c_int,
    high_water: crate::zutil_h::ulg,
    slid: ::core::ffi::c_int,
}

struct DeflateDispatch<'input, 'output, 'state> {
    flush: DeflateFlush,
    stream: DeflateDispatchStream<'input, 'output>,
    state: DeflateDispatchState<'state>,
}

// A completed dispatch must be detached from all bounded caller and
// callback-storage views before the ABI stream or opaque state is updated.
// Keep that completion as scalars: the stream adapter is then the sole place
// that publishes cursors, and a future callback-paired storage owner can
// consume this result without inheriting any of those temporary borrows.
struct DeflateDispatchStreamUpdate {
    next_in: usize,
    next_out: usize,
    avail_in: crate::stdlib::uInt,
    avail_out: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    data_type: ::core::ffi::c_int,
    message: Option<::core::ffi::c_int>,
}

struct DeflateDispatchStateUpdate {
    status: ::core::ffi::c_int,
    pending_out: usize,
    pending: crate::zutil_h::ulg,
    wrap: ::core::ffi::c_int,
    gzindex: usize,
    last_flush: ::core::ffi::c_int,
    block_start: ::core::ffi::c_long,
    match_length: crate::stdlib::uInt,
    prev_match: crate::src::deflate::IPos,
    match_available: ::core::ffi::c_int,
    strstart: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    max_chain_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    heap_len: ::core::ffi::c_int,
    heap_max: ::core::ffi::c_int,
    sym_next: crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    opt_len: crate::zutil_h::ulg,
    static_len: crate::zutil_h::ulg,
    matches: crate::stdlib::uInt,
    insert: crate::stdlib::uInt,
    ins_h: crate::stdlib::uInt,
    bi_buf: crate::zutil_h::ush,
    bi_valid: ::core::ffi::c_int,
    bi_used: ::core::ffi::c_int,
    high_water: crate::zutil_h::ulg,
    slid: ::core::ffi::c_int,
}

struct DeflateDispatchCompletion {
    result: ::core::ffi::c_int,
    stream: DeflateDispatchStreamUpdate,
    state: DeflateDispatchStateUpdate,
}

impl DeflateDispatch<'_, '_, '_> {
    fn complete(self, result: ::core::ffi::c_int) -> DeflateDispatchCompletion {
        DeflateDispatchCompletion {
            result,
            stream: DeflateDispatchStreamUpdate {
                next_in: self.stream.next_in,
                next_out: self.stream.next_out,
                avail_in: self.stream.avail_in,
                avail_out: self.stream.avail_out,
                total_in: self.stream.total_in,
                total_out: self.stream.total_out,
                adler: self.stream.adler,
                data_type: self.stream.data_type,
                message: self.stream.message,
            },
            state: DeflateDispatchStateUpdate {
                status: self.state.status,
                pending_out: self.state.pending_out,
                pending: self.state.pending,
                wrap: self.state.wrap,
                gzindex: self.state.gzindex,
                last_flush: self.state.last_flush,
                block_start: self.state.block_start,
                match_length: self.state.match_length,
                prev_match: self.state.prev_match,
                match_available: self.state.match_available,
                strstart: self.state.strstart,
                match_start: self.state.match_start,
                lookahead: self.state.lookahead,
                prev_length: self.state.prev_length,
                max_chain_length: self.state.max_chain_length,
                max_lazy_match: self.state.max_lazy_match,
                level: self.state.level,
                strategy: self.state.strategy,
                good_match: self.state.good_match,
                nice_match: self.state.nice_match,
                heap_len: self.state.heap_len,
                heap_max: self.state.heap_max,
                sym_next: self.state.sym_next,
                sym_end: self.state.sym_end,
                opt_len: self.state.opt_len,
                static_len: self.state.static_len,
                matches: self.state.matches,
                insert: self.state.insert,
                ins_h: self.state.ins_h,
                bi_buf: self.state.bi_buf,
                bi_valid: self.state.bi_valid,
                bi_used: self.state.bi_used,
                high_water: self.state.high_water,
                slid: self.state.slid,
            },
        }
    }
}

// The complete deflate state machine operates on the bounded dispatch owner.
// In particular, it never observes ABI pointers or callback allocation
// handles; the ABI adapter below is the sole projection/publication site.
// Consuming the view before returning makes the completion independent of the
// caller cursor and callback-storage borrows that were used to construct it.
fn deflate(dispatch: &mut DeflateDispatch<'_, '_, '_>) -> ::core::ffi::c_int {
    let strm = &mut dispatch.stream;
    let s = &mut dispatch.state;
    let flush_request = dispatch.flush;
    let flush = flush_request.raw();
    let old_flush;
    if s.status == crate::src::deflate::FINISH_STATE && flush != crate::zlib_h::Z_FINISH {
        strm.message = Some(crate::zlib_h::Z_STREAM_ERROR);
        return -2 as ::core::ffi::c_int;
    }
    if strm.avail_out == 0 as crate::stdlib::uInt {
        strm.message = Some(crate::zlib_h::Z_BUF_ERROR);
        return -5 as ::core::ffi::c_int;
    }
    let input = strm.input;
    let output = &mut *strm.output;
    let mut output_pos = strm.next_out;
    let pending_buf = &mut *s.pending_buf;
    old_flush = s.last_flush;
    s.last_flush = flush;
    if s.pending != 0 as crate::zutil_h::ulg {
        let len = flush_pending(
            pending_buf,
            &mut s.pending,
            &mut s.bi_buf,
            &mut s.bi_valid,
            &mut s.pending_out,
            output,
            &mut output_pos,
        );
        strm.next_out = strm.next_out.wrapping_add(len as usize);
        strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
        strm.avail_out = strm.avail_out.wrapping_sub(len);
        if strm.avail_out == 0 as crate::stdlib::uInt {
            s.last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    } else if strm.avail_in == 0 as crate::stdlib::uInt
        && flush_request.repeats_without_input(old_flush)
    {
        strm.message = Some(crate::zlib_h::Z_BUF_ERROR);
        return -5 as ::core::ffi::c_int;
    }
    if s.status == crate::src::deflate::FINISH_STATE && strm.avail_in != 0 as crate::stdlib::uInt {
        strm.message = Some(crate::zlib_h::Z_BUF_ERROR);
        return -5 as ::core::ffi::c_int;
    }
    if s.status == crate::src::deflate::INIT_STATE && s.wrap == 0 as ::core::ffi::c_int {
        s.status = crate::src::deflate::BUSY_STATE;
    }
    if s.status == crate::src::deflate::INIT_STATE {
        strm.adler = append_zlib_header(
            pending_buf,
            &mut s.pending,
            s.w_bits,
            s.strategy,
            s.level,
            s.strstart,
            strm.adler,
        );
        s.status = crate::src::deflate::BUSY_STATE;
        let len = flush_pending(
            pending_buf,
            &mut s.pending,
            &mut s.bi_buf,
            &mut s.bi_valid,
            &mut s.pending_out,
            output,
            &mut output_pos,
        );
        strm.next_out = strm.next_out.wrapping_add(len as usize);
        strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
        strm.avail_out = strm.avail_out.wrapping_sub(len);
        if s.pending != 0 as crate::zutil_h::ulg {
            s.last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if s.status == crate::src::deflate::GZIP_STATE {
        strm.adler = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None);
        // This initial gzip header always fits in the pending allocation. Keep
        // a single exact-capacity view for the contiguous write sequence.
        match append_gzip_header_prefix(
            pending_buf,
            &mut s.pending,
            s.level,
            s.strategy,
            s.gzhead.as_ref(),
        ) {
            None => {
                s.status = crate::src::deflate::BUSY_STATE;
                let len = flush_pending(
                    pending_buf,
                    &mut s.pending,
                    &mut s.bi_buf,
                    &mut s.bi_valid,
                    &mut s.pending_out,
                    output,
                    &mut output_pos,
                );
                strm.next_out = strm.next_out.wrapping_add(len as usize);
                strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
                strm.avail_out = strm.avail_out.wrapping_sub(len);
                if s.pending != 0 as crate::zutil_h::ulg {
                    s.last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
            Some(hcrc) => {
                if hcrc {
                    strm.adler = crate::src::crc32::crc32_z(
                        strm.adler,
                        Some(&pending_buf[..s.pending as usize]),
                    );
                }
                s.gzindex = 0;
                s.status = crate::src::deflate::EXTRA_STATE;
            }
        }
    }
    if s.status == crate::src::deflate::EXTRA_STATE {
        if s.gzhead
            .as_ref()
            .is_some_and(|header| header.extra.is_some())
        {
            loop {
                let complete = {
                    let header = s.gzhead.as_ref().expect("gzip header was checked");
                    let extra = header.extra.as_deref().expect("gzip extra was checked");
                    append_gzip_extra_chunk(
                        extra,
                        &mut s.gzindex,
                        pending_buf,
                        &mut s.pending,
                        header.hcrc,
                        &mut strm.adler,
                    )
                };
                if complete {
                    break;
                }
                let len = flush_pending(
                    pending_buf,
                    &mut s.pending,
                    &mut s.bi_buf,
                    &mut s.bi_valid,
                    &mut s.pending_out,
                    output,
                    &mut output_pos,
                );
                strm.next_out = strm.next_out.wrapping_add(len as usize);
                strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
                strm.avail_out = strm.avail_out.wrapping_sub(len);
                if s.pending != 0 as crate::zutil_h::ulg {
                    s.last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
            s.gzindex = 0;
        }
        s.status = crate::src::deflate::NAME_STATE;
    }
    if s.status == crate::src::deflate::NAME_STATE {
        if s.gzhead
            .as_ref()
            .is_some_and(|header| header.name.is_some())
        {
            // The header is owned by the state.  Copy this retained byte
            // string before advancing the pending cursor so the safe state
            // borrow does not overlap its mutable pending-buffer updates.
            let (name, hcrc) = {
                let gzhead = s.gzhead.as_ref().expect("gzip header was checked");
                (
                    gzhead
                        .name
                        .as_deref()
                        .expect("gzip name was checked")
                        .to_vec(),
                    gzhead.hcrc,
                )
            };
            loop {
                if s.pending == s.pending_buf_size {
                    let len = flush_pending(
                        pending_buf,
                        &mut s.pending,
                        &mut s.bi_buf,
                        &mut s.bi_valid,
                        &mut s.pending_out,
                        output,
                        &mut output_pos,
                    );
                    strm.next_out = strm.next_out.wrapping_add(len as usize);
                    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
                    strm.avail_out = strm.avail_out.wrapping_sub(len);
                    if s.pending != 0 as crate::zutil_h::ulg {
                        s.last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                }
                let complete = {
                    let mut source_index = s.gzindex;
                    let complete = append_gzip_cstring_bytes(
                        &name,
                        &mut source_index,
                        pending_buf,
                        &mut s.pending,
                        hcrc,
                        &mut strm.adler,
                    );
                    s.gzindex = source_index;
                    complete
                };
                if complete {
                    break;
                }
            }
            s.gzindex = 0;
        }
        s.status = crate::src::deflate::COMMENT_STATE;
    }
    if s.status == crate::src::deflate::COMMENT_STATE {
        if s.gzhead
            .as_ref()
            .is_some_and(|header| header.comment.is_some())
        {
            // See the NAME-state copy above: retain an independent byte view
            // while the pending cursor advances through this header field.
            let (comment, hcrc) = {
                let gzhead = s.gzhead.as_ref().expect("gzip header was checked");
                (
                    gzhead
                        .comment
                        .as_deref()
                        .expect("gzip comment was checked")
                        .to_vec(),
                    gzhead.hcrc,
                )
            };
            loop {
                if s.pending == s.pending_buf_size {
                    let len = flush_pending(
                        pending_buf,
                        &mut s.pending,
                        &mut s.bi_buf,
                        &mut s.bi_valid,
                        &mut s.pending_out,
                        output,
                        &mut output_pos,
                    );
                    strm.next_out = strm.next_out.wrapping_add(len as usize);
                    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
                    strm.avail_out = strm.avail_out.wrapping_sub(len);
                    if s.pending != 0 as crate::zutil_h::ulg {
                        s.last_flush = -1 as ::core::ffi::c_int;
                        return crate::zlib_h::Z_OK;
                    }
                }
                let complete = {
                    let mut source_index = s.gzindex;
                    let complete = append_gzip_cstring_bytes(
                        &comment,
                        &mut source_index,
                        pending_buf,
                        &mut s.pending,
                        hcrc,
                        &mut strm.adler,
                    );
                    s.gzindex = source_index;
                    complete
                };
                if complete {
                    break;
                }
            }
        }
        s.status = crate::src::deflate::HCRC_STATE;
    }
    if s.status == crate::src::deflate::HCRC_STATE {
        if s.gzhead.as_ref().is_some_and(|header| header.hcrc) {
            if s.pending.wrapping_add(2 as crate::zutil_h::ulg) > s.pending_buf_size {
                let len = flush_pending(
                    pending_buf,
                    &mut s.pending,
                    &mut s.bi_buf,
                    &mut s.bi_valid,
                    &mut s.pending_out,
                    output,
                    &mut output_pos,
                );
                strm.next_out = strm.next_out.wrapping_add(len as usize);
                strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
                strm.avail_out = strm.avail_out.wrapping_sub(len);
                if s.pending != 0 as crate::zutil_h::ulg {
                    s.last_flush = -1 as ::core::ffi::c_int;
                    return crate::zlib_h::Z_OK;
                }
            }
            let hcrc = strm.adler;
            // The preceding capacity check ensures that both HCRC bytes fit
            // in this exact pending allocation.
            append_pending_bytes(
                pending_buf,
                &mut s.pending,
                &[
                    (hcrc & 0xff as crate::stdlib::uLong) as crate::stdlib::Byte,
                    (hcrc >> 8 as ::core::ffi::c_int & 0xff as crate::stdlib::uLong)
                        as crate::stdlib::Byte,
                ],
            );
            strm.adler = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None);
        }
        s.status = crate::src::deflate::BUSY_STATE;
        let len = flush_pending(
            pending_buf,
            &mut s.pending,
            &mut s.bi_buf,
            &mut s.bi_valid,
            &mut s.pending_out,
            output,
            &mut output_pos,
        );
        strm.next_out = strm.next_out.wrapping_add(len as usize);
        strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
        strm.avail_out = strm.avail_out.wrapping_sub(len);
        if s.pending != 0 as crate::zutil_h::ulg {
            s.last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if strm.avail_in != 0 as crate::stdlib::uInt
        || s.lookahead != 0 as crate::stdlib::uInt
        || flush != crate::zlib_h::Z_NO_FLUSH && s.status != crate::src::deflate::FINISH_STATE
    {
        let mut bstate: block_state = need_more;
        bstate = (if s.level == 0 as ::core::ffi::c_int {
            // Level zero has no parser-specific state.  Project its three
            // callback-owned buffers once here, then keep the stored-block
            // policy entirely in the slice-based core.
            let output = &mut output[output_pos..];
            let window = &mut *s.window;
            let (result, input_pos, produced, total_in, total_out, adler) = {
                let mut stored_stream = DeflateStoredStream {
                    input,
                    input_pos: 0,
                    output,
                    output_pos: 0,
                    total_in: strm.total_in,
                    total_out: strm.total_out,
                    adler: strm.adler,
                };
                let mut stored_state = DeflateStoredState {
                    window,
                    pending_buf,
                    pending: &mut s.pending,
                    pending_out: &mut s.pending_out,
                    bi_buf: &mut s.bi_buf,
                    bi_valid: &mut s.bi_valid,
                    bi_used: &mut s.bi_used,
                    w_size: s.w_size,
                    strstart: &mut s.strstart,
                    block_start: &mut s.block_start,
                    wrap: s.wrap,
                    matches: &mut s.matches,
                    insert: &mut s.insert,
                    high_water: &mut s.high_water,
                };
                let result =
                    deflate_stored_from_views(&mut stored_state, &mut stored_stream, flush);
                (
                    result,
                    stored_stream.input_pos,
                    stored_stream.output_pos,
                    stored_stream.total_in,
                    stored_stream.total_out,
                    stored_stream.adler,
                )
            };
            if input_pos != 0 {
                strm.next_in = strm.next_in.wrapping_add(input_pos);
            }
            strm.avail_in = strm.avail_in.wrapping_sub(input_pos as crate::stdlib::uInt);
            output_pos += produced;
            strm.next_out = strm.next_out.wrapping_add(produced);
            strm.avail_out = strm.avail_out.wrapping_sub(produced as crate::stdlib::uInt);
            strm.total_in = total_in;
            strm.total_out = total_out;
            strm.adler = adler;
            result as ::core::ffi::c_uint
        } else {
            // All non-stored modes share the same bounded stream and state
            // projections.  Establish them once at the ABI boundary, then
            // dispatch through the slice-based matcher.  In particular, do
            // not make each algorithm reconstruct a raw view of the same
            // callback-owned allocations.
            let run = if s.strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
                deflate_huff_from_views
            } else if s.strategy == crate::zlib_h::Z_RLE {
                deflate_rle_from_views
            } else {
                match configuration_table[s.level as usize].algorithm {
                    DeflateAlgorithm::Stored => unreachable!("level zero is handled above"),
                    DeflateAlgorithm::Fast => deflate_fast_from_views,
                    DeflateAlgorithm::Slow => deflate_slow_from_views,
                }
            };
            let output = &mut output[output_pos..];
            let window = &mut *s.window;
            let prev = &mut *s.prev;
            let head = &mut *s.head;
            let (result, progress) = {
                let mut matched = DeflateFastState {
                    window,
                    prev,
                    head,
                    pending_buf,
                    pending_out: s.pending_out,
                    pending: s.pending,
                    bi_buf: s.bi_buf,
                    bi_valid: s.bi_valid,
                    bi_used: s.bi_used,
                    dyn_ltree: &mut s.dyn_ltree,
                    dyn_dtree: &mut s.dyn_dtree,
                    bl_tree: &mut s.bl_tree,
                    l_desc: &mut s.l_desc,
                    d_desc: &mut s.d_desc,
                    bl_desc: &mut s.bl_desc,
                    heap: &mut s.heap,
                    heap_len: s.heap_len,
                    heap_max: s.heap_max,
                    depth: &mut s.depth,
                    bl_count: &mut s.bl_count,
                    opt_len: s.opt_len,
                    static_len: s.static_len,
                    sym_buf_start: s.sym_buf_start,
                    sym_next: s.sym_next,
                    sym_end: s.sym_end,
                    matches: s.matches,
                    lookahead: s.lookahead,
                    strstart: s.strstart,
                    block_start: s.block_start,
                    insert: s.insert,
                    ins_h: s.ins_h,
                    match_length: s.match_length,
                    match_start: s.match_start,
                    prev_length: s.prev_length,
                    prev_match: s.prev_match,
                    match_available: s.match_available,
                    max_chain_length: s.max_chain_length,
                    max_lazy_match: s.max_lazy_match,
                    good_match: s.good_match,
                    nice_match: s.nice_match,
                    level: s.level,
                    strategy: s.strategy,
                    w_size: s.w_size,
                    w_mask: s.w_mask,
                    hash_shift: s.hash_shift,
                    hash_mask: s.hash_mask,
                    wrap: s.wrap,
                    slid: s.slid,
                    high_water: s.high_water,
                };
                let mut matched_stream = DeflateFastStream {
                    input,
                    input_pos: 0,
                    output,
                    output_pos: 0,
                    avail_out: strm.avail_out,
                    total_in: strm.total_in,
                    total_out: strm.total_out,
                    adler: strm.adler,
                    data_type: &mut strm.data_type,
                };
                let result =
                    deflate_match_from_views(&mut matched, &mut matched_stream, flush, run);
                let progress = DeflateMatchProgress::from_views(&matched, &matched_stream);
                (result, progress)
            };
            strm.next_in = strm.next_in.wrapping_add(progress.input_consumed);
            strm.avail_in = strm
                .avail_in
                .wrapping_sub(progress.input_consumed as crate::stdlib::uInt);
            output_pos += progress.output_produced;
            strm.next_out = strm.next_out.wrapping_add(progress.output_produced);
            strm.avail_out = progress.avail_out;
            strm.total_in = progress.total_in;
            strm.total_out = progress.total_out;
            strm.adler = progress.adler;
            s.pending_out = progress.pending_out;
            s.pending = progress.pending;
            s.bi_buf = progress.bi_buf;
            s.bi_valid = progress.bi_valid;
            s.bi_used = progress.bi_used;
            s.heap_len = progress.heap_len;
            s.heap_max = progress.heap_max;
            s.opt_len = progress.opt_len;
            s.static_len = progress.static_len;
            s.sym_next = progress.sym_next;
            s.matches = progress.matches;
            s.lookahead = progress.lookahead;
            s.strstart = progress.strstart;
            s.block_start = progress.block_start;
            s.insert = progress.insert;
            s.ins_h = progress.ins_h;
            s.match_length = progress.match_length;
            s.match_start = progress.match_start;
            s.prev_length = progress.prev_length;
            s.prev_match = progress.prev_match;
            s.match_available = progress.match_available;
            s.slid = progress.slid;
            s.high_water = progress.high_water;
            result as ::core::ffi::c_uint
        }) as block_state;
        if bstate as ::core::ffi::c_uint
            == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
            || bstate as ::core::ffi::c_uint
                == finish_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            s.status = crate::src::deflate::FINISH_STATE;
        }
        if bstate as ::core::ffi::c_uint == need_more as ::core::ffi::c_int as ::core::ffi::c_uint
            || bstate as ::core::ffi::c_uint
                == finish_started as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if strm.avail_out == 0 as crate::stdlib::uInt {
                s.last_flush = -1 as ::core::ffi::c_int;
            }
            return crate::zlib_h::Z_OK;
        }
        if bstate as ::core::ffi::c_uint == block_done as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if flush == crate::zlib_h::Z_PARTIAL_FLUSH {
                crate::src::trees::bit_output(
                    crate::src::trees::BitOutputState {
                        pending_buf,
                        pending: &mut s.pending,
                        bi_buf: &mut s.bi_buf,
                        bi_valid: &mut s.bi_valid,
                        bi_used: &mut s.bi_used,
                    },
                    crate::src::trees::BitOutputAction::Align,
                );
            } else if flush != crate::zlib_h::Z_BLOCK {
                // This flush emits the same empty stored block as
                // `_tr_stored_block()`, but the dispatcher already has the
                // opaque state and can pass its one bounded pending view
                // directly to the slice-based tree core.  In particular,
                // keep the zero-length input as a real empty slice rather
                // than reconstructing the C null cursor accepted by the ABI
                // adapter.
                crate::src::trees::stored_block_bytes(
                    pending_buf,
                    &mut s.pending,
                    &mut s.bi_buf,
                    &mut s.bi_valid,
                    &mut s.bi_used,
                    &[],
                    0 as crate::zutil_h::ulg,
                    0 as ::core::ffi::c_int,
                );
                if flush == crate::zlib_h::Z_FULL_FLUSH {
                    // `head` has exactly `hash_size` elements from
                    // `deflateInit2_()` or `deflateCopy()`.
                    let head = &mut *s.head;
                    clear_hash_table(head);
                    s.slid = 0 as ::core::ffi::c_int;
                    if s.lookahead == 0 as crate::stdlib::uInt {
                        s.strstart = 0 as crate::stdlib::uInt;
                        s.block_start = 0 as ::core::ffi::c_long;
                        s.insert = 0 as crate::stdlib::uInt;
                    }
                }
            }
            let len = flush_pending(
                pending_buf,
                &mut s.pending,
                &mut s.bi_buf,
                &mut s.bi_valid,
                &mut s.pending_out,
                output,
                &mut output_pos,
            );
            strm.next_out = strm.next_out.wrapping_add(len as usize);
            strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
            strm.avail_out = strm.avail_out.wrapping_sub(len);
            if strm.avail_out == 0 as crate::stdlib::uInt {
                s.last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        }
    }
    if flush != crate::zlib_h::Z_FINISH {
        return crate::zlib_h::Z_OK;
    }
    if s.wrap <= 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_END;
    }
    // Both trailers write through the one callback-owned pending allocation;
    // keep its projection at this boundary and hand the format choice to the
    // pointer-free encoder above.
    append_deflate_trailer(
        pending_buf,
        &mut s.pending,
        s.wrap,
        strm.adler,
        strm.total_in,
    );
    let len = flush_pending(
        pending_buf,
        &mut s.pending,
        &mut s.bi_buf,
        &mut s.bi_valid,
        &mut s.pending_out,
        output,
        &mut output_pos,
    );
    strm.next_out = strm.next_out.wrapping_add(len as usize);
    strm.total_out = strm.total_out.wrapping_add(len as crate::stdlib::uLong);
    strm.avail_out = strm.avail_out.wrapping_sub(len);
    if s.wrap > 0 as ::core::ffi::c_int {
        s.wrap = -s.wrap;
    }
    return if s.pending != 0 as crate::zutil_h::ulg {
        crate::zlib_h::Z_OK
    } else {
        crate::zlib_h::Z_STREAM_END
    };
}

// This is the callback-paired dispatch owner.  It owns every bounded caller
// and callback-storage view until the state machine has completed, and then
// returns only scalar cursor and state updates.  In particular, neither the
// core nor its completion can retain an ABI pointer or allocation handle.
fn deflate_from_stream(mut dispatch: DeflateDispatch<'_, '_, '_>) -> DeflateDispatchCompletion {
    let result = deflate(&mut dispatch);
    dispatch.complete(result)
}

// This is the only raw deflate-call boundary. It validates the ABI cursors,
// forms the callback-paired dispatch owner once, and publishes its scalar
// completion only after every temporary slice borrow has ended.
pub unsafe fn deflate_dispatch_from_abi_stream(
    strm: &mut crate::zlib_h::z_stream_s,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(flush) = DeflateFlush::parse(flush) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Dispatch needs the same three bounded history views as dictionary
    // handling. Reuse that single callback-storage projection instead of
    // rebuilding window, prev, and head slices at this boundary.
    let Some((strm, state, storage)) =
        deflate_stream_and_state(strm, DeflateStorageProjection::Dictionary)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if strm.next_out.is_null() || (strm.avail_in != 0 && strm.next_in.is_null()) {
        strm.msg = crate::src::zutil::zError(crate::zlib_h::Z_STREAM_ERROR)
            .as_ptr()
            .cast_mut()
            .cast();
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let input = if strm.avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
    };
    let output = ::core::slice::from_raw_parts_mut(strm.next_out, strm.avail_out as usize);
    let pending_buf = ::core::slice::from_raw_parts_mut(
        state
            .pending_buf
            .expect("initialized pending buffer")
            .as_ptr(),
        state.pending_buf_size as usize,
    );
    let window = storage.window.expect("dispatch window projection");
    let prev = storage.prev.expect("dispatch prev-table projection");
    let head = storage.head.expect("dispatch hash-table projection");
    let dispatch = DeflateDispatch {
        flush,
        stream: DeflateDispatchStream {
            input,
            output,
            next_in: 0,
            next_out: 0,
            avail_in: strm.avail_in,
            avail_out: strm.avail_out,
            total_in: strm.total_in,
            total_out: strm.total_out,
            adler: strm.adler,
            data_type: strm.data_type,
            message: None,
        },
        state: DeflateDispatchState {
            pending_buf,
            window,
            prev,
            head,
            status: state.status,
            pending_buf_size: state.pending_buf_size,
            pending_out: state.pending_out,
            pending: state.pending,
            wrap: state.wrap,
            gzhead: &mut state.gzhead,
            gzindex: state.gzindex,
            last_flush: state.last_flush,
            w_size: state.w_size,
            w_bits: state.w_bits,
            w_mask: state.w_mask,
            hash_size: state.hash_size,
            hash_mask: state.hash_mask,
            hash_shift: state.hash_shift,
            block_start: state.block_start,
            match_length: state.match_length,
            prev_match: state.prev_match,
            match_available: state.match_available,
            strstart: state.strstart,
            match_start: state.match_start,
            lookahead: state.lookahead,
            prev_length: state.prev_length,
            max_chain_length: state.max_chain_length,
            max_lazy_match: state.max_lazy_match,
            level: state.level,
            strategy: state.strategy,
            good_match: state.good_match,
            nice_match: state.nice_match,
            dyn_ltree: &mut state.dyn_ltree,
            dyn_dtree: &mut state.dyn_dtree,
            bl_tree: &mut state.bl_tree,
            l_desc: &mut state.l_desc,
            d_desc: &mut state.d_desc,
            bl_desc: &mut state.bl_desc,
            bl_count: &mut state.bl_count,
            heap: &mut state.heap,
            heap_len: state.heap_len,
            heap_max: state.heap_max,
            depth: &mut state.depth,
            sym_buf_start: state.sym_buf_start,
            sym_next: state.sym_next,
            sym_end: state.sym_end,
            opt_len: state.opt_len,
            static_len: state.static_len,
            matches: state.matches,
            insert: state.insert,
            ins_h: state.ins_h,
            bi_buf: state.bi_buf,
            bi_valid: state.bi_valid,
            bi_used: state.bi_used,
            high_water: state.high_water,
            slid: state.slid,
        },
    };
    let DeflateDispatchCompletion {
        result,
        stream:
            DeflateDispatchStreamUpdate {
                next_in,
                next_out,
                avail_in,
                avail_out,
                total_in,
                total_out,
                adler,
                data_type,
                message,
            },
        state:
            DeflateDispatchStateUpdate {
                status,
                pending_out,
                pending,
                wrap,
                gzindex,
                last_flush,
                block_start,
                match_length,
                prev_match,
                match_available,
                strstart,
                match_start,
                lookahead,
                prev_length,
                max_chain_length,
                max_lazy_match,
                level,
                strategy,
                good_match,
                nice_match,
                heap_len,
                heap_max,
                sym_next,
                sym_end,
                opt_len,
                static_len,
                matches,
                insert,
                ins_h,
                bi_buf,
                bi_valid,
                bi_used,
                high_water,
                slid,
            },
    } = deflate_from_stream(dispatch);
    strm.next_in = strm.next_in.wrapping_add(next_in);
    strm.next_out = strm.next_out.wrapping_add(next_out);
    strm.avail_in = avail_in;
    strm.avail_out = avail_out;
    strm.total_in = total_in;
    strm.total_out = total_out;
    strm.adler = adler;
    strm.data_type = data_type;
    if let Some(message) = message {
        strm.msg = crate::src::zutil::zError(message)
            .as_ptr()
            .cast_mut()
            .cast();
    }
    state.status = status;
    state.pending_out = pending_out;
    state.pending = pending;
    state.wrap = wrap;
    state.gzindex = gzindex;
    state.last_flush = last_flush;
    state.block_start = block_start;
    state.match_length = match_length;
    state.prev_match = prev_match;
    state.match_available = match_available;
    state.strstart = strstart;
    state.match_start = match_start;
    state.lookahead = lookahead;
    state.prev_length = prev_length;
    state.max_chain_length = max_chain_length;
    state.max_lazy_match = max_lazy_match;
    state.level = level;
    state.strategy = strategy;
    state.good_match = good_match;
    state.nice_match = nice_match;
    state.heap_len = heap_len;
    state.heap_max = heap_max;
    state.sym_next = sym_next;
    state.sym_end = sym_end;
    state.opt_len = opt_len;
    state.static_len = static_len;
    state.matches = matches;
    state.insert = insert;
    state.ins_h = ins_h;
    state.bi_buf = bi_buf;
    state.bi_valid = bi_valid;
    state.bi_used = bi_used;
    state.high_water = high_water;
    state.slid = slid;
    result
}
#[export_name = "deflate"]

pub unsafe extern "C" fn deflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_dispatch_from_abi_stream(strm, flush)
}
// The callback-backed release transaction consumes a validated stream handle.
// Embedded users (notably gzip close) can form that handle from their existing
// stream borrow instead of reconstructing a raw stream pointer. The callback
// provenance and every release remain together here.
pub unsafe fn deflateEnd(
    mut strm: ::core::ptr::NonNull<crate::zlib_h::z_stream_s>,
) -> ::core::ffi::c_int {
    let strm = strm.as_mut();
    let Some((strm, state, _storage)) =
        deflate_stream_and_state(strm, DeflateStorageProjection::None)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Keep the ABI projection at the release boundary.  Everything after
    // this uses the two scoped views and the exact callback pairing that was
    // established by deflateInit2_().
    let state_ptr = state as *mut crate::src::deflate::deflate_state;
    let zfree = strm.zfree.expect("validated by deflate_stream_and_state");
    let opaque = strm.opaque;
    // Snapshot every callback-owned allocation before the first release.
    // Besides retaining zlib's pending/head/prev/window/state release order,
    // this ends the mutable state projection before a re-entrant zfree()
    // callback can observe the stream.
    let (release_plan, allocations) = {
        let release_plan = state.callback_storage.take_release_plan(state.status);
        // The state itself is released through the caller's zfree callback,
        // so drop the owned gzip-header snapshot before that allocation.
        drop(state.gzhead.take());
        (
            release_plan,
            [
                if release_plan.pending {
                    state.pending_buf.map(|allocation| allocation.cast())
                } else {
                    None
                },
                if release_plan.head {
                    state.head.map(|allocation| allocation.cast())
                } else {
                    None
                },
                if release_plan.prev {
                    state.prev.map(|allocation| allocation.cast())
                } else {
                    None
                },
                if release_plan.window {
                    state.window.map(|allocation| allocation.cast())
                } else {
                    None
                },
                if release_plan.state {
                    ::core::ptr::NonNull::new(state_ptr.cast())
                } else {
                    None
                },
            ],
        )
    };
    for allocation in allocations.into_iter().flatten() {
        zfree(opaque, allocation.as_ptr());
    }
    strm.state = None;
    return release_plan.result();
}
#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = ::core::ptr::NonNull::new(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflateEnd(strm)
}
// This is the pointer-free handoff from an ABI state snapshot to the deep
// copy plan.  Keeping it separate from the callback transaction lets an
// owned callback broker reuse the exact validation and safe slice core.
fn deflate_copy_from_abi(payload: DeflateCopyPayload) -> Option<DeflateCopyPreparation> {
    prepare_deflate_copy(payload)
}

// The C ABI still gives us callback-owned allocations and opaque stream
// pointers.  Keep that projection out of `deflate_copy_from_abi()`: its safe
// typed-slice core is also the path used by the eventual allocation owner.
unsafe fn deflate_copy_from_abi_boundary(
    mut dest: ::core::ptr::NonNull<crate::zlib_h::z_stream_s>,
    source: ::core::ptr::NonNull<crate::zlib_h::z_stream_s>,
) -> ::core::ffi::c_int {
    let source = source.as_ref();
    if source.zalloc.is_none() || source.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(source_state) = source.state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let source_state = source_state.cast::<crate::src::deflate::deflate_state>();
    let ss = &*source_state.as_ptr();
    let dest = dest.as_mut();
    let payload = DeflateCopyPayload {
        data_type: source.data_type,
        status: ss.status,
        pending_buf_size: ss.pending_buf_size,
        pending_out: ss.pending_out,
        pending: ss.pending,
        wrap: ss.wrap,
        gzhead: ss.gzhead.as_ref().map(copy_gzip_header),
        gzindex: ss.gzindex,
        method: ss.method,
        last_flush: ss.last_flush,
        w_size: ss.w_size,
        w_bits: ss.w_bits,
        w_mask: ss.w_mask,
        window_size: ss.window_size,
        ins_h: ss.ins_h,
        hash_size: ss.hash_size,
        hash_bits: ss.hash_bits,
        hash_mask: ss.hash_mask,
        hash_shift: ss.hash_shift,
        block_start: ss.block_start,
        match_length: ss.match_length,
        prev_match: ss.prev_match,
        match_available: ss.match_available,
        strstart: ss.strstart,
        match_start: ss.match_start,
        lookahead: ss.lookahead,
        prev_length: ss.prev_length,
        max_chain_length: ss.max_chain_length,
        max_lazy_match: ss.max_lazy_match,
        level: ss.level,
        strategy: ss.strategy,
        good_match: ss.good_match,
        nice_match: ss.nice_match,
        tree: copy_deflate_tree_state(
            &ss.dyn_ltree,
            &ss.dyn_dtree,
            &ss.bl_tree,
            &ss.l_desc,
            &ss.d_desc,
            &ss.bl_desc,
            &ss.bl_count,
            &ss.heap,
            ss.heap_len,
            ss.heap_max,
            &ss.depth,
        ),
        sym_buf_start: ss.sym_buf_start,
        lit_bufsize: ss.lit_bufsize,
        sym_next: ss.sym_next,
        sym_end: ss.sym_end,
        opt_len: ss.opt_len,
        static_len: ss.static_len,
        matches: ss.matches,
        insert: ss.insert,
        bi_buf: ss.bi_buf,
        bi_valid: ss.bi_valid,
        bi_used: ss.bi_used,
        high_water: ss.high_water,
        slid: ss.slid,
    };
    let Some(DeflateCopyPreparation {
        payload,
        plan: DeflateCopyPlan {
            storage,
            layout: copy_layout,
        },
    }) = deflate_copy_from_abi(payload)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };

    // Do not byte-copy the ABI stream: that made this boundary depend on the
    // layout of a caller-visible owner and obscured which fields are retained
    // by the copied stream.  The explicit snapshot preserves all ABI fields
    // (including callbacks and caller cursors), while the state allocation
    // below replaces its temporary source-state pointer before return.
    *dest = crate::zlib_h::z_stream_s {
        next_in: source.next_in,
        avail_in: source.avail_in,
        total_in: source.total_in,
        next_out: source.next_out,
        avail_out: source.avail_out,
        total_out: source.total_out,
        msg: source.msg,
        state: source.state,
        zalloc: source.zalloc,
        zfree: source.zfree,
        opaque: source.opaque,
        data_type: source.data_type,
        adler: source.adler,
        reserved: source.reserved,
    };
    let ds = Some(dest.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        dest.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::deflate::deflate_state;
    let Some(mut ds) = ::core::ptr::NonNull::new(ds) else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    dest.state = Some(ds.cast());
    // Publish an explicit initialized snapshot rather than byte-copying a
    // Rust value out of callback-owned storage.  The allocation handles are
    // retained until their replacements are installed below, preserving the
    // C copy path's callback-visible allocation order.  Header registration
    // is independently deep-copied before any callback can observe `ds`.
    ds.write(crate::src::deflate::internal_state {
        data_type: payload.data_type,
        status: payload.status,
        pending_buf: ss.pending_buf,
        pending_buf_size: payload.pending_buf_size,
        pending_out: payload.pending_out,
        pending: payload.pending,
        callback_storage: DeflateCallbackStorageOwner::new_state(storage),
        wrap: payload.wrap,
        gzhead: payload.gzhead,
        gzindex: payload.gzindex,
        method: payload.method,
        last_flush: payload.last_flush,
        w_size: payload.w_size,
        w_bits: payload.w_bits,
        w_mask: payload.w_mask,
        window: ss.window,
        window_size: payload.window_size,
        prev: ss.prev,
        head: ss.head,
        ins_h: payload.ins_h,
        hash_size: payload.hash_size,
        hash_bits: payload.hash_bits,
        hash_mask: payload.hash_mask,
        hash_shift: payload.hash_shift,
        block_start: payload.block_start,
        match_length: payload.match_length,
        prev_match: payload.prev_match,
        match_available: payload.match_available,
        strstart: payload.strstart,
        match_start: payload.match_start,
        lookahead: payload.lookahead,
        prev_length: payload.prev_length,
        max_chain_length: payload.max_chain_length,
        max_lazy_match: payload.max_lazy_match,
        level: payload.level,
        strategy: payload.strategy,
        good_match: payload.good_match,
        nice_match: payload.nice_match,
        dyn_ltree: payload.tree.dyn_ltree,
        dyn_dtree: payload.tree.dyn_dtree,
        bl_tree: payload.tree.bl_tree,
        l_desc: payload.tree.l_desc,
        d_desc: payload.tree.d_desc,
        bl_desc: payload.tree.bl_desc,
        bl_count: payload.tree.bl_count,
        heap: payload.tree.heap,
        heap_len: payload.tree.heap_len,
        heap_max: payload.tree.heap_max,
        depth: payload.tree.depth,
        sym_buf_start: payload.sym_buf_start,
        lit_bufsize: payload.lit_bufsize,
        sym_next: payload.sym_next,
        sym_end: payload.sym_end,
        opt_len: payload.opt_len,
        static_len: payload.static_len,
        matches: payload.matches,
        insert: payload.insert,
        bi_buf: payload.bi_buf,
        bi_valid: payload.bi_valid,
        bi_used: payload.bi_used,
        high_water: payload.high_water,
        slid: payload.slid,
    });
    let ds = &mut *ds.as_ptr();
    // Preserve the source implementation's callback-visible order.  Reload
    // the callback and opaque value for every request: a re-entrant custom
    // allocator is allowed to inspect or update the stream between calls.
    let storage_results = request_deflate_storage(&storage, |slot, request| {
        let allocation = Some(dest.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            dest.opaque, request.items, request.size
        );
        match slot {
            DeflateStorageSlot::Window => {
                ds.window = ::core::ptr::NonNull::new(allocation.cast());
            }
            DeflateStorageSlot::Prev => {
                ds.prev = ::core::ptr::NonNull::new(allocation.cast());
            }
            DeflateStorageSlot::Head => {
                ds.head = ::core::ptr::NonNull::new(allocation.cast());
            }
            DeflateStorageSlot::Pending => {
                ds.pending_buf = ::core::ptr::NonNull::new(allocation.cast());
            }
        }
        let allocated = !allocation.is_null();
        ds.callback_storage.record_storage(*slot, allocated);
        allocated
    });
    if !storage_results.is_complete()
        || ds.window.is_none()
        || ds.prev.is_none()
        || ds.head.is_none()
        || ds.pending_buf.is_none()
    {
        deflateEnd(::core::ptr::NonNull::from(dest));
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // The two callback lifecycles now own all allocations.  Validate their
    // immutable descriptors once, then keep raw storage access at this one
    // boundary.  The five copies are the original logical deflate-copy
    // regions; the pointer-free plan supplies every bounded extent and range.
    assert!(ss
        .callback_storage
        .copy_geometry(&ds.callback_storage, &copy_layout));
    ::core::ptr::copy_nonoverlapping(
        ss.window.expect("initialized window").as_ptr(),
        ds.window.expect("initialized window").as_ptr(),
        copy_layout.window_bytes,
    );
    ::core::ptr::copy_nonoverlapping(
        ss.prev.expect("initialized prev table").as_ptr(),
        ds.prev.expect("initialized prev table").as_ptr(),
        copy_layout.prev_entries,
    );
    ::core::ptr::copy_nonoverlapping(
        ss.head.expect("initialized head table").as_ptr(),
        ds.head.expect("initialized head table").as_ptr(),
        copy_layout.head_entries,
    );
    if let Some(regions) = copy_layout.pending.as_ref() {
        ::core::ptr::copy_nonoverlapping(
            ss.pending_buf
                .expect("initialized pending buffer")
                .as_ptr()
                .wrapping_add(regions.queued.start),
            ds.pending_buf
                .expect("initialized pending buffer")
                .as_ptr()
                .wrapping_add(regions.queued.start),
            regions.queued.len(),
        );
        ::core::ptr::copy_nonoverlapping(
            ss.pending_buf
                .expect("initialized pending buffer")
                .as_ptr()
                .wrapping_add(regions.symbols.start),
            ds.pending_buf
                .expect("initialized pending buffer")
                .as_ptr()
                .wrapping_add(regions.symbols.start),
            regions.symbols.len(),
        );
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateCopy"]

pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(dest) = ::core::ptr::NonNull::new(dest) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(source) = ::core::ptr::NonNull::new(source) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_copy_from_abi_boundary(dest, source)
}
struct LongestMatchInput {
    max_chain_length: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    w_size: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    lookahead: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
}

struct LongestMatchResult {
    length: crate::stdlib::uInt,
    start: crate::stdlib::uInt,
}

fn longest_match_core(
    window: &[crate::stdlib::Bytef],
    prev: &[crate::src::deflate::Posf],
    input: &LongestMatchInput,
    mut cur_match: crate::src::deflate::IPos,
) -> LongestMatchResult {
    let mut chain_length = input.max_chain_length;
    let mut best_length = input.prev_length.min(input.lookahead);
    let mut best_start = input.match_start;
    let mut nice_match = input.nice_match.max(0) as crate::stdlib::uInt;
    let limit = if input.strstart
        > input
            .w_size
            .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
    {
        input.strstart.wrapping_sub(
            input
                .w_size
                .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
        ) as crate::src::deflate::IPos
    } else {
        NIL as crate::src::deflate::IPos
    };
    let max_match = crate::zutil_h::MAX_MATCH as usize;
    let scan_start = input.strstart as usize;

    if input.prev_length >= input.good_match {
        chain_length >>= 2;
    }
    nice_match = nice_match.min(input.lookahead);

    while scan_start
        .checked_add(max_match)
        .is_some_and(|end| end <= window.len())
    {
        let candidate = cur_match as usize;
        let Some(candidate_end) = candidate.checked_add(max_match) else {
            break;
        };
        if candidate_end > window.len() {
            break;
        }

        let mut length = 0usize;
        while length < max_match && window[candidate + length] == window[scan_start + length] {
            length += 1;
        }
        if length > best_length as usize {
            best_length = length as crate::stdlib::uInt;
            best_start = cur_match as crate::stdlib::uInt;
            if best_length >= nice_match {
                break;
            }
        }

        let prev_index = (cur_match as crate::stdlib::uInt & input.w_mask) as usize;
        let Some(&next_match) = prev.get(prev_index) else {
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

    LongestMatchResult {
        length: best_length.min(input.lookahead),
        start: best_start,
    }
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

// Stored-mode has no reason to retain the ABI stream or the callback-owned
// allocations.  Keep those projections in its small unsafe adapter and run
// the block policy on bounded views/cursors instead.
struct DeflateStoredState<'a> {
    window: &'a mut [crate::stdlib::Bytef],
    pending_buf: &'a mut [crate::stdlib::Bytef],
    pending: &'a mut crate::zutil_h::ulg,
    pending_out: &'a mut usize,
    bi_buf: &'a mut crate::zutil_h::ush,
    bi_valid: &'a mut ::core::ffi::c_int,
    bi_used: &'a mut ::core::ffi::c_int,
    w_size: crate::stdlib::uInt,
    strstart: &'a mut crate::stdlib::uInt,
    block_start: &'a mut ::core::ffi::c_long,
    wrap: ::core::ffi::c_int,
    matches: &'a mut crate::stdlib::uInt,
    insert: &'a mut crate::stdlib::uInt,
    high_water: &'a mut crate::zutil_h::ulg,
}

struct DeflateStoredStream<'a> {
    input: &'a [crate::stdlib::Bytef],
    input_pos: usize,
    output: &'a mut [crate::stdlib::Bytef],
    output_pos: usize,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
}

impl DeflateStoredStream<'_> {
    fn avail_in(&self) -> crate::stdlib::uInt {
        self.input.len().saturating_sub(self.input_pos) as crate::stdlib::uInt
    }

    fn avail_out(&self) -> crate::stdlib::uInt {
        self.output.len().saturating_sub(self.output_pos) as crate::stdlib::uInt
    }

    fn copy_input(
        &mut self,
        len: ::core::ffi::c_uint,
        output: &mut [crate::stdlib::Bytef],
        wrap: ::core::ffi::c_int,
    ) {
        let len = len as usize;
        let input = &self.input[self.input_pos..self.input_pos + len];
        self.adler = read_buf_bytes(input, output, self.adler, wrap);
        self.input_pos += len;
        self.total_in = self.total_in.wrapping_add(len as crate::stdlib::uLong);
    }

    fn copy_input_to_output(&mut self, len: ::core::ffi::c_uint, wrap: ::core::ffi::c_int) {
        let len = len as usize;
        let input = &self.input[self.input_pos..self.input_pos + len];
        let output = &mut self.output[self.output_pos..self.output_pos + len];
        self.adler = read_buf_bytes(input, output, self.adler, wrap);
        self.input_pos += len;
        self.output_pos += len;
        self.total_in = self.total_in.wrapping_add(len as crate::stdlib::uLong);
        self.total_out = self.total_out.wrapping_add(len as crate::stdlib::uLong);
    }
}

fn flush_stored_pending(
    pending_buf: &mut [crate::stdlib::Bytef],
    pending: &mut crate::zutil_h::ulg,
    pending_out: &mut usize,
    bi_buf: &mut crate::zutil_h::ush,
    bi_valid: &mut ::core::ffi::c_int,
    stream: &mut DeflateStoredStream<'_>,
) -> crate::stdlib::uInt {
    crate::src::trees::flush_pending_bits(pending_buf, pending, bi_buf, bi_valid);
    let len = (*pending).min(stream.avail_out() as crate::zutil_h::ulg) as usize;
    if len == 0 {
        return stream.avail_out();
    }
    let output = &mut stream.output[stream.output_pos..stream.output_pos + len];
    let len = flush_pending_bytes(output, pending_buf, pending_out, pending);
    stream.output_pos += len as usize;
    stream.total_out = stream.total_out.wrapping_add(len as crate::stdlib::uLong);
    stream.avail_out()
}

fn deflate_stored_from_views(
    state: &mut DeflateStoredState<'_>,
    stream: &mut DeflateStoredStream<'_>,
    flush: ::core::ffi::c_int,
) -> block_state {
    // The dispatcher has already validated and projected the ABI stream and
    // opaque state. Stored-mode therefore receives those scoped borrows
    // directly rather than rebuilding either from the legacy backlink.
    // The backing window has the exact `window_size` established by
    // `deflateInit2_()` and retained by `deflateCopy()`. Retain this one
    // bounded view while stored blocks copy or slide its contents.
    let pending_len = state.pending_buf.len() as crate::zutil_h::ulg;
    let window = &mut *state.window;
    let pending_buf = &mut *state.pending_buf;
    let mut min_block: ::core::ffi::c_uint = (if pending_len.wrapping_sub(5 as crate::zutil_h::ulg)
        > state.w_size as crate::zutil_h::ulg
    {
        state.w_size as crate::zutil_h::ulg
    } else {
        pending_len.wrapping_sub(5 as crate::zutil_h::ulg)
    }) as ::core::ffi::c_uint;
    let mut last: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut used: ::core::ffi::c_uint = stream.avail_in() as ::core::ffi::c_uint;
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        have = (*state.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
            >> 3 as ::core::ffi::c_int;
        if stream.avail_out() < have {
            break;
        }
        have = (stream.avail_out() as ::core::ffi::c_uint).wrapping_sub(have);
        left = (*state.strstart as ::core::ffi::c_long - *state.block_start) as ::core::ffi::c_uint;
        if len as crate::zutil_h::ulg
            > (left as crate::zutil_h::ulg).wrapping_add(stream.avail_in() as crate::zutil_h::ulg)
        {
            len = (left as crate::stdlib::uInt).wrapping_add(stream.avail_in())
                as ::core::ffi::c_uint;
        }
        if len > have {
            len = have;
        }
        if len < min_block
            && (len == 0 as ::core::ffi::c_uint && flush != crate::zlib_h::Z_FINISH
                || flush == crate::zlib_h::Z_NO_FLUSH
                || len != (left as crate::stdlib::uInt).wrapping_add(stream.avail_in()))
        {
            break;
        }
        last = if flush == crate::zlib_h::Z_FINISH
            && len == (left as crate::stdlib::uInt).wrapping_add(stream.avail_in())
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        crate::src::trees::stored_block_bytes(
            pending_buf,
            state.pending,
            state.bi_buf,
            state.bi_valid,
            state.bi_used,
            &[],
            0 as crate::zutil_h::ulg,
            last,
        );
        set_stored_block_length(pending_buf, *state.pending, len);
        flush_stored_pending(
            pending_buf,
            state.pending,
            state.pending_out,
            state.bi_buf,
            state.bi_valid,
            stream,
        );
        if left != 0 {
            if left > len {
                left = len;
            }
            let output = &mut stream.output[stream.output_pos..stream.output_pos + left as usize];
            let start = *state.block_start as usize;
            output.copy_from_slice(&window[start..start + left as usize]);
            stream.output_pos += left as usize;
            stream.total_out = stream.total_out.wrapping_add(left as crate::stdlib::uLong);
            *state.block_start += left as ::core::ffi::c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            stream.copy_input_to_output(len, state.wrap);
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub(stream.avail_in() as ::core::ffi::c_uint);
    if used != 0 {
        if used >= state.w_size {
            *state.matches = 2 as crate::stdlib::uInt;
            let input_end = stream.input_pos;
            window[..state.w_size as usize]
                .copy_from_slice(&stream.input[input_end - state.w_size as usize..input_end]);
            *state.strstart = state.w_size;
            *state.insert = *state.strstart;
        } else {
            if (window.len() as crate::zutil_h::ulg)
                .wrapping_sub(*state.strstart as crate::zutil_h::ulg)
                <= used as crate::zutil_h::ulg
            {
                *state.strstart = state.strstart.wrapping_sub(state.w_size);
                window.copy_within(
                    state.w_size as usize..state.w_size as usize + *state.strstart as usize,
                    0,
                );
                if *state.matches < 2 as crate::stdlib::uInt {
                    *state.matches = state.matches.wrapping_add(1);
                }
                if *state.insert > *state.strstart {
                    *state.insert = *state.strstart;
                }
            }
            let input_end = stream.input_pos;
            let start = *state.strstart as usize;
            window[start..start + used as usize]
                .copy_from_slice(&stream.input[input_end - used as usize..input_end]);
            *state.strstart = state.strstart.wrapping_add(used);
            *state.insert =
                state
                    .insert
                    .wrapping_add(if used > state.w_size.wrapping_sub(*state.insert) {
                        (state.w_size as ::core::ffi::c_uint)
                            .wrapping_sub(*state.insert as ::core::ffi::c_uint)
                    } else {
                        used
                    });
        }
        *state.block_start = *state.strstart as ::core::ffi::c_long;
    }
    if *state.high_water < *state.strstart as crate::zutil_h::ulg {
        *state.high_water = *state.strstart as crate::zutil_h::ulg;
    }
    if last != 0 {
        *state.bi_used = 8 as ::core::ffi::c_int;
        return finish_done;
    }
    if flush != crate::zlib_h::Z_NO_FLUSH
        && flush != crate::zlib_h::Z_FINISH
        && stream.avail_in() == 0 as crate::stdlib::uInt
        && *state.strstart as ::core::ffi::c_long == *state.block_start
    {
        return block_done;
    }
    have = (window.len() as crate::zutil_h::ulg)
        .wrapping_sub(*state.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
    if stream.avail_in() > have && *state.block_start >= state.w_size as ::core::ffi::c_long {
        *state.block_start -= state.w_size as ::core::ffi::c_long;
        *state.strstart = state.strstart.wrapping_sub(state.w_size);
        window.copy_within(
            state.w_size as usize..state.w_size as usize + *state.strstart as usize,
            0,
        );
        if *state.matches < 2 as crate::stdlib::uInt {
            *state.matches = state.matches.wrapping_add(1);
        }
        have = have.wrapping_add(state.w_size as ::core::ffi::c_uint);
        if *state.insert > *state.strstart {
            *state.insert = *state.strstart;
        }
    }
    if have > stream.avail_in() {
        have = stream.avail_in() as ::core::ffi::c_uint;
    }
    if have != 0 {
        let start = *state.strstart as usize;
        let output = &mut window[start..start + have as usize];
        stream.copy_input(have, output, state.wrap);
        *state.strstart = state.strstart.wrapping_add(have);
        *state.insert =
            state
                .insert
                .wrapping_add(if have > state.w_size.wrapping_sub(*state.insert) {
                    (state.w_size as ::core::ffi::c_uint)
                        .wrapping_sub(*state.insert as ::core::ffi::c_uint)
                } else {
                    have
                });
    }
    if *state.high_water < *state.strstart as crate::zutil_h::ulg {
        *state.high_water = *state.strstart as crate::zutil_h::ulg;
    }
    have = (*state.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
        >> 3 as ::core::ffi::c_int;
    have = (if pending_len.wrapping_sub(have as crate::zutil_h::ulg) > 65535 as crate::zutil_h::ulg
    {
        65535 as crate::zutil_h::ulg
    } else {
        pending_len.wrapping_sub(have as crate::zutil_h::ulg)
    }) as ::core::ffi::c_uint;
    min_block = if have > state.w_size {
        state.w_size as ::core::ffi::c_uint
    } else {
        have
    };
    left = (*state.strstart as ::core::ffi::c_long - *state.block_start) as ::core::ffi::c_uint;
    if left >= min_block
        || (left != 0 || flush == crate::zlib_h::Z_FINISH)
            && flush != crate::zlib_h::Z_NO_FLUSH
            && stream.avail_in() == 0 as crate::stdlib::uInt
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == crate::zlib_h::Z_FINISH
            && stream.avail_in() == 0 as crate::stdlib::uInt
            && len == left
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let start = *state.block_start as usize;
        let input = &window[start..start + len as usize];
        crate::src::trees::stored_block_bytes(
            pending_buf,
            state.pending,
            state.bi_buf,
            state.bi_valid,
            state.bi_used,
            input,
            len as crate::zutil_h::ulg,
            last,
        );
        *state.block_start += len as ::core::ffi::c_long;
        flush_stored_pending(
            pending_buf,
            state.pending,
            state.pending_out,
            state.bi_buf,
            state.bi_valid,
            stream,
        );
    }
    if last != 0 {
        *state.bi_used = 8 as ::core::ffi::c_int;
    }
    return (if last != 0 {
        finish_started as ::core::ffi::c_int
    } else {
        need_more as ::core::ffi::c_int
    }) as block_state;
}

// The fast parser only needs bounded storage and stream cursors.  Keep the
// callback-owned allocation and ABI cursor projections in `deflate_fast()`;
// this view is deliberately pointer-free so the hot loop can stay safe.
struct DeflateFastState<'a> {
    window: &'a mut [crate::stdlib::Bytef],
    prev: &'a mut [crate::src::deflate::Posf],
    head: &'a mut [crate::src::deflate::Posf],
    pending_buf: &'a mut [crate::stdlib::Bytef],
    pending_out: usize,
    pending: crate::zutil_h::ulg,
    bi_buf: crate::zutil_h::ush,
    bi_valid: ::core::ffi::c_int,
    bi_used: ::core::ffi::c_int,
    dyn_ltree: &'a mut [crate::src::deflate::ct_data_s; 573],
    dyn_dtree: &'a mut [crate::src::deflate::ct_data_s; 61],
    bl_tree: &'a mut [crate::src::deflate::ct_data_s; 39],
    l_desc: &'a mut crate::src::deflate::tree_desc_s,
    d_desc: &'a mut crate::src::deflate::tree_desc_s,
    bl_desc: &'a mut crate::src::deflate::tree_desc_s,
    heap: &'a mut [::core::ffi::c_int; 573],
    heap_len: ::core::ffi::c_int,
    heap_max: ::core::ffi::c_int,
    depth: &'a mut [crate::zutil_h::uch; 573],
    bl_count: &'a mut [crate::zutil_h::ush; 16],
    opt_len: crate::zutil_h::ulg,
    static_len: crate::zutil_h::ulg,
    sym_buf_start: usize,
    sym_next: crate::stdlib::uInt,
    sym_end: crate::stdlib::uInt,
    matches: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    insert: crate::stdlib::uInt,
    ins_h: crate::stdlib::uInt,
    match_length: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    prev_match: crate::src::deflate::IPos,
    match_available: ::core::ffi::c_int,
    max_chain_length: crate::stdlib::uInt,
    max_lazy_match: crate::stdlib::uInt,
    good_match: crate::stdlib::uInt,
    nice_match: ::core::ffi::c_int,
    level: ::core::ffi::c_int,
    strategy: ::core::ffi::c_int,
    w_size: crate::stdlib::uInt,
    w_mask: crate::stdlib::uInt,
    hash_shift: crate::stdlib::uInt,
    hash_mask: crate::stdlib::uInt,
    wrap: ::core::ffi::c_int,
    slid: ::core::ffi::c_int,
    high_water: crate::zutil_h::ulg,
}

struct DeflateFastStream<'a> {
    input: &'a [crate::stdlib::Bytef],
    input_pos: usize,
    output: &'a mut [crate::stdlib::Bytef],
    output_pos: usize,
    avail_out: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    data_type: &'a mut ::core::ffi::c_int,
}

// This is the pointer-free result of one bounded match-parser dispatch.  The
// ABI adapter below is still responsible for publishing it today, but keeping
// the progress independent of `z_stream_s` and `deflate_state` gives a future
// owned stream facade one complete handoff instead of a second cursor scan.
struct DeflateMatchProgress {
    input_consumed: usize,
    output_produced: usize,
    avail_out: crate::stdlib::uInt,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    pending_out: usize,
    pending: crate::zutil_h::ulg,
    bi_buf: crate::zutil_h::ush,
    bi_valid: ::core::ffi::c_int,
    bi_used: ::core::ffi::c_int,
    heap_len: ::core::ffi::c_int,
    heap_max: ::core::ffi::c_int,
    opt_len: crate::zutil_h::ulg,
    static_len: crate::zutil_h::ulg,
    sym_next: crate::stdlib::uInt,
    matches: crate::stdlib::uInt,
    lookahead: crate::stdlib::uInt,
    strstart: crate::stdlib::uInt,
    block_start: ::core::ffi::c_long,
    insert: crate::stdlib::uInt,
    ins_h: crate::stdlib::uInt,
    match_length: crate::stdlib::uInt,
    match_start: crate::stdlib::uInt,
    prev_length: crate::stdlib::uInt,
    prev_match: crate::src::deflate::IPos,
    match_available: ::core::ffi::c_int,
    slid: ::core::ffi::c_int,
    high_water: crate::zutil_h::ulg,
}

impl DeflateMatchProgress {
    fn from_views(state: &DeflateFastState<'_>, stream: &DeflateFastStream<'_>) -> Self {
        Self {
            input_consumed: stream.input_pos,
            output_produced: stream.output_pos,
            avail_out: stream.avail_out,
            total_in: stream.total_in,
            total_out: stream.total_out,
            adler: stream.adler,
            pending_out: state.pending_out,
            pending: state.pending,
            bi_buf: state.bi_buf,
            bi_valid: state.bi_valid,
            bi_used: state.bi_used,
            heap_len: state.heap_len,
            heap_max: state.heap_max,
            opt_len: state.opt_len,
            static_len: state.static_len,
            sym_next: state.sym_next,
            matches: state.matches,
            lookahead: state.lookahead,
            strstart: state.strstart,
            block_start: state.block_start,
            insert: state.insert,
            ins_h: state.ins_h,
            match_length: state.match_length,
            match_start: state.match_start,
            prev_length: state.prev_length,
            prev_match: state.prev_match,
            match_available: state.match_available,
            slid: state.slid,
            high_water: state.high_water,
        }
    }
}

fn fill_fast_window(state: &mut DeflateFastState<'_>, stream: &mut DeflateFastStream<'_>) {
    let mut input = DeflateInputCursor {
        input: &stream.input[stream.input_pos..],
        consumed: 0,
        checksum: stream.adler,
        total_in: stream.total_in,
    };
    fill_window_from_views(
        &mut FillWindowState {
            window: state.window,
            prev: state.prev,
            head: state.head,
            w_size: state.w_size,
            hash_shift: state.hash_shift,
            hash_mask: state.hash_mask,
            w_mask: state.w_mask,
            wrap: state.wrap,
            lookahead: &mut state.lookahead,
            strstart: &mut state.strstart,
            match_start: &mut state.match_start,
            block_start: &mut state.block_start,
            insert: &mut state.insert,
            slid: &mut state.slid,
            ins_h: &mut state.ins_h,
            high_water: &mut state.high_water,
        },
        &mut input,
    );
    stream.input_pos += input.consumed;
    stream.adler = input.checksum;
    stream.total_in = input.total_in;
}

fn flush_fast_pending(state: &mut DeflateFastState<'_>, stream: &mut DeflateFastStream<'_>) {
    crate::src::trees::flush_pending_bits(
        state.pending_buf,
        &mut state.pending,
        &mut state.bi_buf,
        &mut state.bi_valid,
    );
    let len = (state.pending as crate::stdlib::uInt).min(stream.avail_out) as usize;
    if len == 0 {
        return;
    }
    let copied = flush_pending_bytes(
        &mut stream.output[stream.output_pos..stream.output_pos + len],
        state.pending_buf,
        &mut state.pending_out,
        &mut state.pending,
    );
    stream.output_pos += copied as usize;
    stream.total_out = stream
        .total_out
        .wrapping_add(copied as crate::stdlib::uLong);
    stream.avail_out = stream.avail_out.wrapping_sub(copied);
}

// All match-oriented strategies flush through the same bounded storage and
// output cursors.  Keeping this operation here lets RLE and Huffman mode use
// the safe tree core as well, instead of reconstructing a state pointer for
// the legacy `_tr_flush_block()` adapter.
fn flush_match_block(
    state: &mut DeflateFastState<'_>,
    stream: &mut DeflateFastStream<'_>,
    last: ::core::ffi::c_int,
) {
    let stored_len =
        (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg;
    let input = if state.block_start >= 0 {
        let start = state.block_start as usize;
        Some(&state.window[start..start + stored_len as usize])
    } else {
        None
    };
    crate::src::trees::flush_block_from_views(
        if state.level > 0 {
            Some(&mut stream.data_type)
        } else {
            None
        },
        crate::src::trees::BlockFlushState {
            level: state.level,
            strategy: state.strategy,
            pending_buf: state.pending_buf,
            pending: &mut state.pending,
            bi_buf: &mut state.bi_buf,
            bi_valid: &mut state.bi_valid,
            bi_used: &mut state.bi_used,
            dyn_ltree: &mut state.dyn_ltree,
            dyn_dtree: &mut state.dyn_dtree,
            bl_tree: &mut state.bl_tree,
            l_desc: &mut state.l_desc,
            d_desc: &mut state.d_desc,
            bl_desc: &mut state.bl_desc,
            heap: &mut state.heap,
            heap_len: &mut state.heap_len,
            heap_max: &mut state.heap_max,
            depth: &mut state.depth,
            bl_count: &mut state.bl_count,
            opt_len: &mut state.opt_len,
            static_len: &mut state.static_len,
            matches: &mut state.matches,
            sym_buf_start: state.sym_buf_start,
            sym_next: &mut state.sym_next,
        },
        input,
        stored_len,
        last,
    );
    state.block_start = state.strstart as ::core::ffi::c_long;
    flush_fast_pending(state, stream);
}

fn deflate_fast_from_views(
    state: &mut DeflateFastState<'_>,
    stream: &mut DeflateFastStream<'_>,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    let sym_buf_start = state.sym_buf_start;
    // `pending_buf` is the full allocation; symbols occupy its suffix after
    // the literal area. The adapter supplied this one full-capacity view.
    loop {
        if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_fast_window(state, stream);
            if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt
                && flush == crate::zlib_h::Z_NO_FLUSH
            {
                return need_more;
            }
            if state.lookahead == 0 as crate::stdlib::uInt {
                break;
            }
        }
        // The bounded refill above is the only operation in this iteration
        // that can change the window. Keep one read view for the remainder
        // of the match/flush work, then drop it before the next refill.
        let window = &*state.window;
        hash_head = NIL as crate::src::deflate::IPos;
        if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let head = &mut *state.head;
            let prev = &mut *state.prev;
            (state.ins_h, hash_head) = insert_hash(
                window,
                head,
                prev,
                state.ins_h,
                state.hash_shift,
                state.hash_mask,
                state.w_mask,
                state.strstart,
            );
        }
        if hash_head != NIL as crate::src::deflate::IPos
            && (state.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                <= state
                    .w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
        {
            // The hash insertion view above has ended.  Reborrow the exact
            // previous-chain allocation as an immutable bounded slice for
            // the matcher, leaving the match algorithm itself pointer-free.
            let prev = &*state.prev;
            let result = longest_match_core(
                window,
                prev,
                &LongestMatchInput {
                    max_chain_length: state.max_chain_length,
                    strstart: state.strstart,
                    w_size: state.w_size,
                    w_mask: state.w_mask,
                    prev_length: state.prev_length,
                    good_match: state.good_match,
                    nice_match: state.nice_match,
                    lookahead: state.lookahead,
                    match_start: state.match_start,
                },
                hash_head,
            );
            state.match_start = result.start;
            state.match_length = result.length;
        }
        if state.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len: crate::zutil_h::uch =
                state.match_length.wrapping_sub(3 as crate::stdlib::uInt) as crate::zutil_h::uch;
            let dist: crate::zutil_h::ush =
                state.strstart.wrapping_sub(state.match_start) as crate::zutil_h::ush;
            bflush = crate::src::trees::tally_symbol(
                &mut state.pending_buf[sym_buf_start..],
                &mut state.sym_next,
                state.sym_end,
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                &mut state.matches,
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
                    let head = &mut *state.head;
                    let prev = &mut *state.prev;
                    (state.ins_h, hash_head) = insert_hash(
                        window,
                        head,
                        prev,
                        state.ins_h,
                        state.hash_shift,
                        state.hash_mask,
                        state.w_mask,
                        state.strstart,
                    );
                    state.match_length = state.match_length.wrapping_sub(1);
                    if state.match_length == 0 as crate::stdlib::uInt {
                        break;
                    }
                }
                state.strstart = state.strstart.wrapping_add(1);
            } else {
                state.strstart = state.strstart.wrapping_add(state.match_length);
                state.match_length = 0 as crate::stdlib::uInt;
                state.ins_h =
                    initial_hash(window, state.strstart, state.hash_shift, state.hash_mask);
            }
        } else {
            let cc: crate::zutil_h::uch = window[state.strstart as usize] as crate::zutil_h::uch;
            bflush = crate::src::trees::tally_symbol(
                &mut state.pending_buf[sym_buf_start..],
                &mut state.sym_next,
                state.sym_end,
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                &mut state.matches,
                0,
                cc as ::core::ffi::c_uint,
            );
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            let stored_len =
                (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg;
            let input = if state.block_start >= 0 {
                let start = state.block_start as usize;
                Some(&window[start..start + stored_len as usize])
            } else {
                None
            };
            crate::src::trees::flush_block_from_views(
                if state.level > 0 {
                    Some(&mut stream.data_type)
                } else {
                    None
                },
                crate::src::trees::BlockFlushState {
                    level: state.level,
                    strategy: state.strategy,
                    pending_buf: state.pending_buf,
                    pending: &mut state.pending,
                    bi_buf: &mut state.bi_buf,
                    bi_valid: &mut state.bi_valid,
                    bi_used: &mut state.bi_used,
                    dyn_ltree: &mut state.dyn_ltree,
                    dyn_dtree: &mut state.dyn_dtree,
                    bl_tree: &mut state.bl_tree,
                    l_desc: &mut state.l_desc,
                    d_desc: &mut state.d_desc,
                    bl_desc: &mut state.bl_desc,
                    heap: &mut state.heap,
                    heap_len: &mut state.heap_len,
                    heap_max: &mut state.heap_max,
                    depth: &mut state.depth,
                    bl_count: &mut state.bl_count,
                    opt_len: &mut state.opt_len,
                    static_len: &mut state.static_len,
                    matches: &mut state.matches,
                    sym_buf_start: state.sym_buf_start,
                    sym_next: &mut state.sym_next,
                },
                input,
                stored_len,
                0,
            );
            state.block_start = state.strstart as ::core::ffi::c_long;
            flush_fast_pending(state, stream);
            if stream.avail_out == 0 as crate::stdlib::uInt {
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
        let stored_len =
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg;
        let window = &*state.window;
        let input = if state.block_start >= 0 {
            let start = state.block_start as usize;
            Some(&window[start..start + stored_len as usize])
        } else {
            None
        };
        crate::src::trees::flush_block_from_views(
            if state.level > 0 {
                Some(&mut stream.data_type)
            } else {
                None
            },
            crate::src::trees::BlockFlushState {
                level: state.level,
                strategy: state.strategy,
                pending_buf: state.pending_buf,
                pending: &mut state.pending,
                bi_buf: &mut state.bi_buf,
                bi_valid: &mut state.bi_valid,
                bi_used: &mut state.bi_used,
                dyn_ltree: &mut state.dyn_ltree,
                dyn_dtree: &mut state.dyn_dtree,
                bl_tree: &mut state.bl_tree,
                l_desc: &mut state.l_desc,
                d_desc: &mut state.d_desc,
                bl_desc: &mut state.bl_desc,
                heap: &mut state.heap,
                heap_len: &mut state.heap_len,
                heap_max: &mut state.heap_max,
                depth: &mut state.depth,
                bl_count: &mut state.bl_count,
                opt_len: &mut state.opt_len,
                static_len: &mut state.static_len,
                matches: &mut state.matches,
                sym_buf_start: state.sym_buf_start,
                sym_next: &mut state.sym_next,
            },
            input,
            stored_len,
            1,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        flush_fast_pending(state, stream);
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        let stored_len =
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg;
        let window = &*state.window;
        let input = if state.block_start >= 0 {
            let start = state.block_start as usize;
            Some(&window[start..start + stored_len as usize])
        } else {
            None
        };
        crate::src::trees::flush_block_from_views(
            if state.level > 0 {
                Some(&mut stream.data_type)
            } else {
                None
            },
            crate::src::trees::BlockFlushState {
                level: state.level,
                strategy: state.strategy,
                pending_buf: state.pending_buf,
                pending: &mut state.pending,
                bi_buf: &mut state.bi_buf,
                bi_valid: &mut state.bi_valid,
                bi_used: &mut state.bi_used,
                dyn_ltree: &mut state.dyn_ltree,
                dyn_dtree: &mut state.dyn_dtree,
                bl_tree: &mut state.bl_tree,
                l_desc: &mut state.l_desc,
                d_desc: &mut state.d_desc,
                bl_desc: &mut state.bl_desc,
                heap: &mut state.heap,
                heap_len: &mut state.heap_len,
                heap_max: &mut state.heap_max,
                depth: &mut state.depth,
                bl_count: &mut state.bl_count,
                opt_len: &mut state.opt_len,
                static_len: &mut state.static_len,
                matches: &mut state.matches,
                sym_buf_start: state.sym_buf_start,
                sym_next: &mut state.sym_next,
            },
            input,
            stored_len,
            0,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        flush_fast_pending(state, stream);
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

fn deflate_slow_from_views(
    state: &mut DeflateFastState<'_>,
    stream: &mut DeflateFastStream<'_>,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    // The dispatcher supplies the scoped ABI stream and state borrows, so
    // the lazy-match loop need not rebuild either from raw pointers.
    let sym_buf_start = state.sym_buf_start;
    // `pending_buf` is the full allocation; symbols occupy its suffix after
    // the literal area. Keeping one full-capacity view avoids a raw cursor.
    loop {
        if state.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_fast_window(state, stream);
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
        // The bounded refill above established the initialized extent for
        // this iteration. Reuse one read view for both hashing and a
        // possible delayed literal instead of rebuilding raw views for each.
        let window = &*state.window;
        // The insertion step does not inspect the previous match fields, so
        // establish the lazy-match candidate before borrowing `prev`. This
        // lets the safe matcher reuse that same bounded hash-table view.
        state.prev_length = state.match_length;
        state.prev_match = state.match_start as crate::src::deflate::IPos;
        state.match_length =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        if state.lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let head = &mut *state.head;
            let prev = &mut *state.prev;
            (state.ins_h, hash_head) = insert_hash(
                window,
                head,
                prev,
                state.ins_h,
                state.hash_shift,
                state.hash_mask,
                state.w_mask,
                state.strstart,
            );
            if hash_head != NIL as crate::src::deflate::IPos
                && state.prev_length < state.max_lazy_match
                && (state.strstart as crate::src::deflate::IPos).wrapping_sub(hash_head)
                    <= state
                        .w_size
                        .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt)
            {
                let result = longest_match_core(
                    window,
                    prev,
                    &LongestMatchInput {
                        max_chain_length: state.max_chain_length,
                        strstart: state.strstart,
                        w_size: state.w_size,
                        w_mask: state.w_mask,
                        prev_length: state.prev_length,
                        good_match: state.good_match,
                        nice_match: state.nice_match,
                        lookahead: state.lookahead,
                        match_start: state.match_start,
                    },
                    hash_head,
                );
                state.match_start = result.start;
                state.match_length = result.length;
            }
        }
        if state.match_length <= 5 as crate::stdlib::uInt
            && (state.strategy == crate::zlib_h::Z_FILTERED
                || state.match_length == crate::zutil_h::MIN_MATCH as crate::stdlib::uInt
                    && state.strstart.wrapping_sub(state.match_start)
                        > TOO_FAR as crate::stdlib::uInt)
        {
            state.match_length =
                (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        }
        let delayed_literal = if state.match_available != 0 {
            Some(window[state.strstart.wrapping_sub(1) as usize] as crate::zutil_h::uch)
        } else {
            None
        };
        let mut slow_state = SlowMatchState {
            prev_length: state.prev_length,
            match_length: state.match_length,
            prev_match: state.prev_match,
            match_available: state.match_available,
            strstart: state.strstart,
            lookahead: state.lookahead,
        };
        let action = {
            let sym_buf = &mut state.pending_buf[sym_buf_start..];
            advance_slow_match(
                &mut slow_state,
                sym_buf,
                &mut state.sym_next,
                state.sym_end,
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                &mut state.matches,
                delayed_literal,
            )
        };
        state.prev_length = slow_state.prev_length;
        state.match_length = slow_state.match_length;
        state.match_available = slow_state.match_available;
        state.strstart = slow_state.strstart;
        state.lookahead = slow_state.lookahead;
        match action {
            SlowMatchAction::Match {
                insert_start,
                insert_count,
                bflush: next_bflush,
            } => {
                bflush = next_bflush;
                for offset in 0..insert_count {
                    let window = &*state.window;
                    let head = &mut *state.head;
                    let prev = &mut *state.prev;
                    (state.ins_h, hash_head) = insert_hash(
                        window,
                        head,
                        prev,
                        state.ins_h,
                        state.hash_shift,
                        state.hash_mask,
                        state.w_mask,
                        insert_start.wrapping_add(offset),
                    );
                }
                if bflush != 0 {
                    let stored_len = (state.strstart as ::core::ffi::c_long - state.block_start)
                        as crate::zutil_h::ulg;
                    let input = if state.block_start >= 0 {
                        let start = state.block_start as usize;
                        Some(&window[start..start + stored_len as usize])
                    } else {
                        None
                    };
                    crate::src::trees::flush_block_from_views(
                        if state.level > 0 {
                            Some(&mut stream.data_type)
                        } else {
                            None
                        },
                        crate::src::trees::BlockFlushState {
                            level: state.level,
                            strategy: state.strategy,
                            pending_buf: state.pending_buf,
                            pending: &mut state.pending,
                            bi_buf: &mut state.bi_buf,
                            bi_valid: &mut state.bi_valid,
                            bi_used: &mut state.bi_used,
                            dyn_ltree: &mut state.dyn_ltree,
                            dyn_dtree: &mut state.dyn_dtree,
                            bl_tree: &mut state.bl_tree,
                            l_desc: &mut state.l_desc,
                            d_desc: &mut state.d_desc,
                            bl_desc: &mut state.bl_desc,
                            heap: &mut state.heap,
                            heap_len: &mut state.heap_len,
                            heap_max: &mut state.heap_max,
                            depth: &mut state.depth,
                            bl_count: &mut state.bl_count,
                            opt_len: &mut state.opt_len,
                            static_len: &mut state.static_len,
                            matches: &mut state.matches,
                            sym_buf_start: state.sym_buf_start,
                            sym_next: &mut state.sym_next,
                        },
                        input,
                        stored_len,
                        0,
                    );
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_fast_pending(state, stream);
                    if stream.avail_out == 0 as crate::stdlib::uInt {
                        return need_more;
                    }
                }
            }
            SlowMatchAction::Literal(next_bflush) => {
                bflush = next_bflush;
                if bflush != 0 {
                    let stored_len = (state.strstart as ::core::ffi::c_long - state.block_start)
                        as crate::zutil_h::ulg;
                    let input = if state.block_start >= 0 {
                        let start = state.block_start as usize;
                        Some(&window[start..start + stored_len as usize])
                    } else {
                        None
                    };
                    crate::src::trees::flush_block_from_views(
                        if state.level > 0 {
                            Some(&mut stream.data_type)
                        } else {
                            None
                        },
                        crate::src::trees::BlockFlushState {
                            level: state.level,
                            strategy: state.strategy,
                            pending_buf: state.pending_buf,
                            pending: &mut state.pending,
                            bi_buf: &mut state.bi_buf,
                            bi_valid: &mut state.bi_valid,
                            bi_used: &mut state.bi_used,
                            dyn_ltree: &mut state.dyn_ltree,
                            dyn_dtree: &mut state.dyn_dtree,
                            bl_tree: &mut state.bl_tree,
                            l_desc: &mut state.l_desc,
                            d_desc: &mut state.d_desc,
                            bl_desc: &mut state.bl_desc,
                            heap: &mut state.heap,
                            heap_len: &mut state.heap_len,
                            heap_max: &mut state.heap_max,
                            depth: &mut state.depth,
                            bl_count: &mut state.bl_count,
                            opt_len: &mut state.opt_len,
                            static_len: &mut state.static_len,
                            matches: &mut state.matches,
                            sym_buf_start: state.sym_buf_start,
                            sym_next: &mut state.sym_next,
                        },
                        input,
                        stored_len,
                        0,
                    );
                    state.block_start = state.strstart as ::core::ffi::c_long;
                    flush_fast_pending(state, stream);
                }
                state.strstart = state.strstart.wrapping_add(1);
                state.lookahead = state.lookahead.wrapping_sub(1);
                if stream.avail_out == 0 as crate::stdlib::uInt {
                    return need_more;
                }
            }
            SlowMatchAction::Defer => {}
        }
    }
    if state.match_available != 0 {
        let window = &*state.window;
        let cc_0 = window[state.strstart.wrapping_sub(1) as usize] as crate::zutil_h::uch;
        bflush = {
            let sym_buf = &mut state.pending_buf[sym_buf_start..];
            tally_slow_symbol(
                sym_buf,
                &mut state.sym_next,
                state.sym_end,
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                &mut state.matches,
                0,
                cc_0 as ::core::ffi::c_uint,
            )
        };
        state.match_available = 0;
    }
    state.insert = if state.strstart
        < (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    {
        state.strstart
    } else {
        (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt
    };
    if flush == crate::zlib_h::Z_FINISH {
        let stored_len =
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg;
        let window = &*state.window;
        let input = if state.block_start >= 0 {
            let start = state.block_start as usize;
            Some(&window[start..start + stored_len as usize])
        } else {
            None
        };
        crate::src::trees::flush_block_from_views(
            if state.level > 0 {
                Some(&mut stream.data_type)
            } else {
                None
            },
            crate::src::trees::BlockFlushState {
                level: state.level,
                strategy: state.strategy,
                pending_buf: state.pending_buf,
                pending: &mut state.pending,
                bi_buf: &mut state.bi_buf,
                bi_valid: &mut state.bi_valid,
                bi_used: &mut state.bi_used,
                dyn_ltree: &mut state.dyn_ltree,
                dyn_dtree: &mut state.dyn_dtree,
                bl_tree: &mut state.bl_tree,
                l_desc: &mut state.l_desc,
                d_desc: &mut state.d_desc,
                bl_desc: &mut state.bl_desc,
                heap: &mut state.heap,
                heap_len: &mut state.heap_len,
                heap_max: &mut state.heap_max,
                depth: &mut state.depth,
                bl_count: &mut state.bl_count,
                opt_len: &mut state.opt_len,
                static_len: &mut state.static_len,
                matches: &mut state.matches,
                sym_buf_start: state.sym_buf_start,
                sym_next: &mut state.sym_next,
            },
            input,
            stored_len,
            1,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        flush_fast_pending(state, stream);
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        let stored_len =
            (state.strstart as ::core::ffi::c_long - state.block_start) as crate::zutil_h::ulg;
        let window = &*state.window;
        let input = if state.block_start >= 0 {
            let start = state.block_start as usize;
            Some(&window[start..start + stored_len as usize])
        } else {
            None
        };
        crate::src::trees::flush_block_from_views(
            if state.level > 0 {
                Some(&mut stream.data_type)
            } else {
                None
            },
            crate::src::trees::BlockFlushState {
                level: state.level,
                strategy: state.strategy,
                pending_buf: state.pending_buf,
                pending: &mut state.pending,
                bi_buf: &mut state.bi_buf,
                bi_valid: &mut state.bi_valid,
                bi_used: &mut state.bi_used,
                dyn_ltree: &mut state.dyn_ltree,
                dyn_dtree: &mut state.dyn_dtree,
                bl_tree: &mut state.bl_tree,
                l_desc: &mut state.l_desc,
                d_desc: &mut state.d_desc,
                bl_desc: &mut state.bl_desc,
                heap: &mut state.heap,
                heap_len: &mut state.heap_len,
                heap_max: &mut state.heap_max,
                depth: &mut state.depth,
                bl_count: &mut state.bl_count,
                opt_len: &mut state.opt_len,
                static_len: &mut state.static_len,
                matches: &mut state.matches,
                sym_buf_start: state.sym_buf_start,
                sym_next: &mut state.sym_next,
            },
            input,
            stored_len,
            0,
        );
        state.block_start = state.strstart as ::core::ffi::c_long;
        flush_fast_pending(state, stream);
        if stream.avail_out == 0 as crate::stdlib::uInt {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

// Find a repeated-byte match using offsets into the fully allocated sliding
// window. The bounded refill maintains the initialized extent for this
// slice-based kernel.
fn rle_match_length(
    window: &[crate::stdlib::Bytef],
    strstart: usize,
    lookahead: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    if lookahead < crate::zutil_h::MIN_MATCH as crate::stdlib::uInt || strstart == 0 {
        return 0;
    }

    let Some(&previous) = window.get(strstart - 1) else {
        return 0;
    };
    let mut scan = strstart - 1;
    for _ in 0..3 {
        scan += 1;
        if window.get(scan) != Some(&previous) {
            return 0;
        }
    }

    let strend = strstart + crate::zutil_h::MAX_MATCH as usize;
    loop {
        for _ in 0..8 {
            scan += 1;
            if window.get(scan) != Some(&previous) {
                let length = (crate::zutil_h::MAX_MATCH as usize)
                    .saturating_sub(strend.saturating_sub(scan));
                return (length as crate::stdlib::uInt).min(lookahead);
            }
        }
        if scan >= strend {
            return (crate::zutil_h::MAX_MATCH as crate::stdlib::uInt).min(lookahead);
        }
    }
}

fn deflate_rle_from_views(
    state: &mut DeflateFastState<'_>,
    stream: &mut DeflateFastStream<'_>,
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if state.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_fast_window(state, stream);
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
        let window = &*state.window;
        state.match_length = rle_match_length(window, state.strstart as usize, state.lookahead);
        if state.match_length >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let len = state.match_length.wrapping_sub(3 as crate::stdlib::uInt);
            bflush = crate::src::trees::tally_symbol(
                &mut state.pending_buf[state.sym_buf_start..],
                &mut state.sym_next,
                state.sym_end,
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                &mut state.matches,
                1,
                len,
            );
            state.lookahead = state.lookahead.wrapping_sub(state.match_length);
            state.strstart = state.strstart.wrapping_add(state.match_length);
            state.match_length = 0 as crate::stdlib::uInt;
        } else {
            let cc = window[state.strstart as usize] as crate::zutil_h::uch;
            bflush = crate::src::trees::tally_symbol(
                &mut state.pending_buf[state.sym_buf_start..],
                &mut state.sym_next,
                state.sym_end,
                &mut state.dyn_ltree,
                &mut state.dyn_dtree,
                &mut state.matches,
                0,
                cc as ::core::ffi::c_uint,
            );
            state.lookahead = state.lookahead.wrapping_sub(1);
            state.strstart = state.strstart.wrapping_add(1);
        }
        if bflush != 0 {
            flush_match_block(state, stream, 0);
            if stream.avail_out == 0 {
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
        flush_match_block(state, stream, 1);
        if stream.avail_out == 0 {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_match_block(state, stream, 0);
        if stream.avail_out == 0 {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

fn deflate_huff_from_views(
    state: &mut DeflateFastState<'_>,
    stream: &mut DeflateFastStream<'_>,
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if state.lookahead == 0 as crate::stdlib::uInt {
            fill_fast_window(state, stream);
            if state.lookahead == 0 as crate::stdlib::uInt {
                if flush == crate::zlib_h::Z_NO_FLUSH {
                    return need_more;
                }
                break;
            }
        }
        state.match_length = 0 as crate::stdlib::uInt;
        let window = &*state.window;
        let cc = window[state.strstart as usize] as crate::zutil_h::uch;
        bflush = crate::src::trees::tally_symbol(
            &mut state.pending_buf[state.sym_buf_start..],
            &mut state.sym_next,
            state.sym_end,
            &mut state.dyn_ltree,
            &mut state.dyn_dtree,
            &mut state.matches,
            0,
            cc as ::core::ffi::c_uint,
        );
        state.lookahead = state.lookahead.wrapping_sub(1);
        state.strstart = state.strstart.wrapping_add(1);
        if bflush != 0 {
            flush_match_block(state, stream, 0);
            if stream.avail_out == 0 {
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
        flush_match_block(state, stream, 1);
        if stream.avail_out == 0 {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if state.sym_next != 0 {
        flush_match_block(state, stream, 0);
        if stream.avail_out == 0 {
            return (if false {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
    }
    return block_done;
}

// RLE and Huffman mode differ only in their symbol-selection loops.  The
// caller constructs the bounded stream and allocation views once, leaving
// this dispatch entirely pointer-free.
fn deflate_match_from_views(
    state: &mut DeflateFastState<'_>,
    stream: &mut DeflateFastStream<'_>,
    flush: ::core::ffi::c_int,
    run: fn(
        &mut DeflateFastState<'_>,
        &mut DeflateFastStream<'_>,
        ::core::ffi::c_int,
    ) -> block_state,
) -> block_state {
    run(state, stream, flush)
}
