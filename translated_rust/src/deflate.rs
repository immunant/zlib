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
    // The LZ history window is a fixed-size allocation owned by the stream
    // state.  Store the owner directly rather than retaining an allocator
    // pointer across compression calls.
    pub window: Option<Vec<crate::stdlib::Bytef>>,
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
        let window = self.window.as_deref()?;
        if end > window_len || end > window.len() {
            return None;
        }
        Some(&window[start..end])
    }

    /// Mutable counterpart to `window_bytes()`.
    #[inline]
    fn window_bytes_mut(
        &mut self,
        start: crate::stdlib::uInt,
        len: usize,
    ) -> Option<&mut [crate::stdlib::Bytef]> {
        let start = usize::try_from(start).ok()?;
        let window_len = usize::try_from(self.window_size).ok()?;
        let end = start.checked_add(len)?;
        let window = self.window.as_deref_mut()?;
        if end > window_len || end > window.len() {
            return None;
        }
        Some(&mut window[start..end])
    }

    /// Preserve the input most recently consumed by the stored-block engine
    /// as the LZ history required by a later change of compression level.
    /// The source is synchronously copied into the owned window, so no borrow
    /// into the caller's ABI stream buffer is retained.
    fn retain_stored_history(&mut self, input: &[crate::stdlib::Bytef]) -> bool {
        let used = input.len();
        let Ok(wsize) = usize::try_from(self.w_size) else {
            return false;
        };
        let Ok(window_size) = usize::try_from(self.window_size) else {
            return false;
        };
        let Ok(strstart) = usize::try_from(self.strstart) else {
            return false;
        };
        if wsize == 0 || strstart > window_size {
            return false;
        }

        if used >= wsize {
            let Some(window) = self.window_bytes_mut(0, wsize) else {
                return false;
            };
            window.copy_from_slice(&input[used - wsize..]);
            self.matches = 2;
            self.strstart = self.w_size;
            self.insert = self.strstart;
        } else {
            if window_size.saturating_sub(strstart) <= used {
                self.strstart = self.strstart.wrapping_sub(self.w_size);
                let Ok(copy_len) = usize::try_from(self.strstart) else {
                    return false;
                };
                let Some(window) = self.window_bytes_mut(0, window_size) else {
                    return false;
                };
                let Some(copy_end) = wsize.checked_add(copy_len) else {
                    return false;
                };
                if copy_end > window.len() {
                    return false;
                }
                window.copy_within(wsize..copy_end, 0);
                if self.matches < 2 {
                    self.matches = self.matches.wrapping_add(1);
                }
                if self.insert > self.strstart {
                    self.insert = self.strstart;
                }
            }
            let Ok(start) = usize::try_from(self.strstart) else {
                return false;
            };
            let Some(end) = start.checked_add(used) else {
                return false;
            };
            if end > window_size {
                return false;
            }
            let Some(window) = self.window_bytes_mut(self.strstart, used) else {
                return false;
            };
            window.copy_from_slice(input);
            self.strstart = self.strstart.wrapping_add(used as crate::stdlib::uInt);
            self.insert = self.insert.wrapping_add(
                if used > self.w_size.wrapping_sub(self.insert) as usize {
                    self.w_size.wrapping_sub(self.insert)
                } else {
                    used as crate::stdlib::uInt
                },
            );
        }
        true
    }

    /// Append uncompressed input that was prefetched into the current window.
    /// Unlike post-block history retention, the stored engine has already
    /// decided whether a window slide is needed before this input is read.
    fn append_stored_input(&mut self, input: &[crate::stdlib::Bytef]) -> bool {
        let used = input.len();
        let Ok(start) = usize::try_from(self.strstart) else {
            return false;
        };
        let Ok(window_size) = usize::try_from(self.window_size) else {
            return false;
        };
        let Some(end) = start.checked_add(used) else {
            return false;
        };
        if end > window_size {
            return false;
        }
        let Some(window) = self.window_bytes_mut(self.strstart, used) else {
            return false;
        };
        window.copy_from_slice(input);
        self.strstart = self.strstart.wrapping_add(used as crate::stdlib::uInt);
        self.insert =
            self.insert
                .wrapping_add(if used > self.w_size.wrapping_sub(self.insert) as usize {
                    self.w_size.wrapping_sub(self.insert)
                } else {
                    used as crate::stdlib::uInt
                });
        true
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
pub use crate::src::trees::_tr_stored_block_ffi as _tr_stored_block;
pub use crate::src::zutil::z_errmsg;
pub use crate::src::zutil::zcalloc;
pub use crate::src::zutil::zcfree;

pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::stdlib::MAX_MEM_LEVEL;
pub use crate::stdlib::MAX_WBITS;
pub use crate::stdlib::charf;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::z_size_t;
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
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gz_header;
pub use crate::zlib_h::gz_header_s;
pub use crate::zlib_h::gz_headerp;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zutil_h::DEF_MEM_LEVEL;
pub use crate::zutil_h::MAX_MATCH;
pub use crate::zutil_h::MIN_MATCH;
pub use crate::zutil_h::PRESET_DICT;
pub use crate::zutil_h::uch;
pub use crate::zutil_h::uchf;
pub use crate::zutil_h::ulg;
pub use crate::zutil_h::ush;

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

fn allocate_window(len: usize) -> Option<Vec<crate::stdlib::Bytef>> {
    let mut window = Vec::new();
    window.try_reserve_exact(len).ok()?;
    window.resize(len, 0);
    Some(window)
}

/// Construct the valid empty Rust value installed in caller-allocated state
/// storage by `deflateInit2_`.  This replaces the translated C `memset`: in
/// particular, the `Option<Vec<_>>` owners must start as real `None` values,
/// rather than merely bytes that happen to look zeroed on a given layout.
fn new_deflate_state() -> crate::src::deflate::deflate_state {
    let tree = crate::src::deflate::ct_data_s { fc: 0, dl: 0 };
    crate::src::deflate::internal_state {
        status: 0,
        pending_buf: None,
        pending_buf_size: 0,
        pending_out: 0,
        pending: 0,
        wrap: 0,
        gzhead: 0,
        gzindex: 0,
        method: 0,
        last_flush: 0,
        w_size: 0,
        w_bits: 0,
        w_mask: 0,
        window: None,
        window_size: 0,
        prev: None,
        head: None,
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
        dyn_ltree: [tree; 573],
        dyn_dtree: [tree; 61],
        bl_tree: [tree; 39],
        l_desc: crate::src::deflate::tree_desc_s {
            max_code: 0,
            static_kind: 0,
        },
        d_desc: crate::src::deflate::tree_desc_s {
            max_code: 0,
            static_kind: 0,
        },
        bl_desc: crate::src::deflate::tree_desc_s {
            max_code: 0,
            static_kind: 0,
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

fn clone_head_table(head: &[crate::src::deflate::Posf]) -> Option<Vec<crate::src::deflate::Posf>> {
    let mut copy = allocate_head_table(head.len())?;
    copy.copy_from_slice(head);
    Some(copy)
}

fn clone_prev_table(prev: &[crate::src::deflate::Posf]) -> Option<Vec<crate::src::deflate::Posf>> {
    let mut copy = allocate_prev_table(prev.len())?;
    copy.copy_from_slice(prev);
    Some(copy)
}

fn slide_hash(s: &mut crate::src::deflate::deflate_state) {
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

/// Append bytes to the caller's current output range and commit the stream
/// cursor.  The pointer-to-slice conversion is centralized here, after the
/// caller has checked the requested length against `avail_out`.
fn write_stream_bytes(
    strm: &mut crate::zlib_h::z_stream_s,
    bytes: &[crate::stdlib::Bytef],
) -> bool {
    if bytes.len() > strm.avail_out as usize || (bytes.len() != 0 && strm.next_out.is_null()) {
        return false;
    }
    if !bytes.is_empty() {
        // `avail_out` describes a writable range established by the ABI
        // caller.  The length above is bounded by that range.
        let output = unsafe { core::slice::from_raw_parts_mut(strm.next_out, bytes.len()) };
        output.copy_from_slice(bytes);
        strm.next_out = strm.next_out.wrapping_add(bytes.len());
        strm.avail_out = strm
            .avail_out
            .wrapping_sub(bytes.len() as crate::stdlib::uInt);
        strm.total_out = strm
            .total_out
            .wrapping_add(bytes.len() as crate::stdlib::uLong);
    }
    true
}

/// Refill the LZ window from a bounded view of the stream's current input.
///
/// `input` starts at the stream cursor on entry.  The stream's decreasing
/// `avail_in` then selects the unconsumed suffix on subsequent loop turns,
/// keeping the window logic independent of raw buffer pointers.
fn fill_window_from_input(
    s: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream_s,
    input: Option<&[crate::stdlib::Bytef]>,
) {
    let wsize = s.w_size;
    let Ok(window_len) = usize::try_from(s.window_size) else {
        return;
    };
    if s.window
        .as_ref()
        .is_none_or(|window| window.len() != window_len)
    {
        return;
    }
    loop {
        let mut more =
            s.window_size
                .wrapping_sub(s.lookahead as crate::zutil_h::ulg)
                .wrapping_sub(s.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
        if ::core::mem::size_of::<::core::ffi::c_int>() <= 2 as usize {
            if more == 0 as ::core::ffi::c_uint
                && s.strstart == 0 as crate::stdlib::uInt
                && s.lookahead == 0 as crate::stdlib::uInt
            {
                more = wsize as ::core::ffi::c_uint;
            } else if more == -1 as ::core::ffi::c_int as ::core::ffi::c_uint {
                more = more.wrapping_sub(1);
            }
        }
        if s.strstart
            >= wsize.wrapping_add(
                s.w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            let Ok(window_base) = usize::try_from(wsize) else {
                return;
            };
            let Ok(copy_len) = usize::try_from(wsize.wrapping_sub(more)) else {
                return;
            };
            let Some(copy_end) = window_base.checked_add(copy_len) else {
                return;
            };
            if copy_end > window_len {
                return;
            }
            let Some(window) = s.window.as_deref_mut() else {
                return;
            };
            window.copy_within(window_base..copy_end, 0);
            s.match_start = s.match_start.wrapping_sub(wsize);
            s.strstart = s.strstart.wrapping_sub(wsize);
            s.block_start -= wsize as ::core::ffi::c_long;
            if s.insert > s.strstart {
                s.insert = s.strstart;
            }
            slide_hash(s);
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if strm.avail_in == 0 as crate::stdlib::uInt {
            break;
        }
        let n = strm.avail_in.min(more);
        let Ok(start) = usize::try_from(s.strstart.wrapping_add(s.lookahead)) else {
            return;
        };
        let Ok(n_usize) = usize::try_from(n) else {
            return;
        };
        let Some(end) = start.checked_add(n_usize) else {
            return;
        };
        if end > window_len {
            return;
        }
        if n != 0 {
            let Some(input) = input else {
                return;
            };
            let Some(input_start) = input.len().checked_sub(strm.avail_in as usize) else {
                return;
            };
            let Some(input_end) = input_start.checked_add(n_usize) else {
                return;
            };
            let Some(input) = input.get(input_start..input_end) else {
                return;
            };
            let Some(window) = s.window.as_deref_mut() else {
                return;
            };
            read_buf_impl(strm, input, &mut window[start..end], s.wrap);
            strm.next_in = strm.next_in.wrapping_add(n_usize);
        }
        s.lookahead = s.lookahead.wrapping_add(n);
        if s.lookahead.wrapping_add(s.insert) >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut str = s.strstart.wrapping_sub(s.insert);
            let Ok(first) = usize::try_from(str) else {
                return;
            };
            let Some(second) = first.checked_add(1) else {
                return;
            };
            let Some(window) = s.window.as_deref() else {
                return;
            };
            let (Some(&first_byte), Some(&second_byte)) = (window.get(first), window.get(second))
            else {
                return;
            };
            s.ins_h = first_byte as crate::stdlib::uInt;
            s.ins_h = (s.ins_h << s.hash_shift ^ second_byte as crate::stdlib::uInt) & s.hash_mask;
            while s.insert != 0 {
                let Ok(next) = usize::try_from(str.wrapping_add(2)) else {
                    return;
                };
                let Some(&next_byte) = window.get(next) else {
                    return;
                };
                s.ins_h =
                    (s.ins_h << s.hash_shift ^ next_byte as crate::stdlib::uInt) & s.hash_mask;
                let head_index = s.ins_h as usize;
                let Some(hash_head) = s
                    .head
                    .as_ref()
                    .and_then(|head| head.get(head_index))
                    .copied()
                else {
                    return;
                };
                let prev_index = (str & s.w_mask) as usize;
                let Some(slot) = s.prev.as_mut().and_then(|prev| prev.get_mut(prev_index)) else {
                    return;
                };
                *slot = hash_head;
                let Some(head) = s.head.as_mut() else {
                    return;
                };
                let Some(slot) = head.get_mut(head_index) else {
                    return;
                };
                *slot = str as crate::src::deflate::Pos as crate::src::deflate::Posf;
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
            && strm.avail_in != 0 as crate::stdlib::uInt)
        {
            break;
        }
    }
    if s.high_water < s.window_size {
        let curr =
            (s.strstart as crate::zutil_h::ulg).wrapping_add(s.lookahead as crate::zutil_h::ulg);
        let init = if s.high_water < curr {
            let mut init = s.window_size.wrapping_sub(curr);
            if init > crate::src::deflate::WIN_INIT as crate::zutil_h::ulg {
                init = crate::src::deflate::WIN_INIT as crate::zutil_h::ulg;
            }
            s.high_water = curr.wrapping_add(init);
            (curr, init)
        } else if s.high_water
            < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
        {
            let mut init = curr
                .wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
                .wrapping_sub(s.high_water);
            if init > s.window_size.wrapping_sub(s.high_water) {
                init = s.window_size.wrapping_sub(s.high_water);
            }
            let start = s.high_water;
            s.high_water = s.high_water.wrapping_add(init);
            (start, init)
        } else {
            (0, 0)
        };
        let (Ok(start), Ok(init)) = (usize::try_from(init.0), usize::try_from(init.1)) else {
            return;
        };
        let Some(end) = start.checked_add(init) else {
            return;
        };
        if end > window_len {
            return;
        }
        let Some(window) = s.window.as_deref_mut() else {
            return;
        };
        window[start..end].fill(0);
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
        version.as_ref().copied(),
        stream_size,
    )
}

pub use deflateInit__ffi as deflateInit_;

pub unsafe fn deflateInit2_(
    strm: Option<&mut crate::zlib_h::z_stream_s>,
    mut level: ::core::ffi::c_int,
    mut method: ::core::ffi::c_int,
    mut windowBits: ::core::ffi::c_int,
    mut memLevel: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
    version: Option<::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wrap: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    static MY_VERSION: [::core::ffi::c_char; 15] = crate::zlib_h::ZLIB_VERSION;
    if version != Some(MY_VERSION[0 as usize])
        || stream_size as usize != ::core::mem::size_of::<crate::zlib_h::z_stream>()
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
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
    unsafe {
        let s = Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::deflate::deflate_state;
        if s.is_null() {
            return crate::zlib_h::Z_MEM_ERROR;
        }
        let state_storage =
            &mut *s.cast::<::core::mem::MaybeUninit<crate::src::deflate::deflate_state>>();
        let s = state_storage.write(new_deflate_state());
        strm.state = s as *mut crate::src::deflate::internal_state;
        s.status = crate::src::deflate::INIT_STATE;
        s.wrap = wrap;
        s.gzhead = 0;
        s.w_bits = windowBits as crate::stdlib::uInt;
        s.w_size = ((1 as ::core::ffi::c_int) << s.w_bits) as crate::stdlib::uInt;
        s.w_mask = s.w_size.wrapping_sub(1 as crate::stdlib::uInt);
        s.hash_bits = (memLevel as crate::stdlib::uInt).wrapping_add(7 as crate::stdlib::uInt);
        s.hash_size = ((1 as ::core::ffi::c_int) << s.hash_bits) as crate::stdlib::uInt;
        s.hash_mask = s.hash_size.wrapping_sub(1 as crate::stdlib::uInt);
        s.hash_shift = s
            .hash_bits
            .wrapping_add(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt)
            .wrapping_sub(1 as crate::stdlib::uInt)
            .wrapping_div(crate::zutil_h::MIN_MATCH as crate::stdlib::uInt);
        s.window = allocate_window(s.w_size as usize * 2);
        s.prev = allocate_prev_table(s.w_size as usize);
        s.head = Some(allocate_head_table(s.hash_size as usize).unwrap_or_default());
        s.high_water = 0 as crate::zutil_h::ulg;
        s.lit_bufsize = ((1 as ::core::ffi::c_int) << memLevel + 6 as ::core::ffi::c_int)
            as crate::stdlib::uInt;
        s.pending_buf_size =
            (s.lit_bufsize as crate::zutil_h::ulg).wrapping_mul(4 as crate::zutil_h::ulg);
        s.pending_buf = allocate_pending_buffer(s.pending_buf_size as usize);
        if s.window.is_none()
            || s.prev.is_none()
            || s.head
                .as_ref()
                .is_none_or(|head| head.len() != s.hash_size as usize)
            || s.pending_buf.is_none()
        {
            s.status = crate::src::deflate::FINISH_STATE;
            strm.msg = crate::src::zutil::z_errmsg[(if (-4 as ::core::ffi::c_int)
                < -6 as ::core::ffi::c_int
                || -4 as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                9 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int - -4 as ::core::ffi::c_int
            }) as usize]
                .load(::core::sync::atomic::Ordering::Relaxed);
            deflate_end_release(strm);
            return crate::zlib_h::Z_MEM_ERROR;
        }
        s.sym_buf = s.lit_bufsize as usize;
        s.sym_end = s
            .lit_bufsize
            .wrapping_sub(1 as crate::stdlib::uInt)
            .wrapping_mul(3 as crate::stdlib::uInt);
        s.level = level;
        s.strategy = strategy;
        s.method = method as crate::stdlib::Byte;
        deflate_reset_state(strm, s)
    }
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
        version.as_ref().copied(),
        stream_size,
    )
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
fn deflate_params_state_is_valid(s: &crate::src::deflate::deflate_state) -> bool {
    s.status == crate::src::deflate::INIT_STATE
            || s.status == crate::src::deflate::GZIP_STATE
            || s.status == crate::src::deflate::EXTRA_STATE
            || s.status == crate::src::deflate::NAME_STATE
            || s.status == crate::src::deflate::COMMENT_STATE
            || s.status == crate::src::deflate::HCRC_STATE
            || s.status == crate::src::deflate::BUSY_STATE
            || s.status == crate::src::deflate::FINISH_STATE
}
pub unsafe extern "C" fn deflateSetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut str: crate::stdlib::uInt = 0;
    let mut n: crate::stdlib::uInt = 0;
    let mut wrap: ::core::ffi::c_int = 0;
    let mut avail: ::core::ffi::c_uint = 0;
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    if !deflate_params_stream_is_valid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(s) = strm.state.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_params_state_is_valid(s) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The state link was validated before this one borrow, so dictionary
    // setup and its window fills do not repeatedly traverse it.
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
    let dictionary_input = core::slice::from_raw_parts(dictionary, dictLength as usize);
    fill_window_from_input(s, strm, Some(dictionary_input));
    while (*s).lookahead >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
        str = (*s).strstart;
        n = (*s).lookahead.wrapping_sub(
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt,
        );
        loop {
            let Some(byte) = (*s).window_byte(
                str.wrapping_add(3 as crate::stdlib::uInt)
                    .wrapping_sub(1 as crate::stdlib::uInt),
            ) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            (*s).ins_h =
                ((*s).ins_h << (*s).hash_shift ^ byte as crate::stdlib::uInt) & (*s).hash_mask;
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
            head[head_index] = str as crate::src::deflate::Pos as crate::src::deflate::Posf;
            str = str.wrapping_add(1);
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
        (*s).strstart = str;
        (*s).lookahead =
            (crate::zutil_h::MIN_MATCH - 1 as ::core::ffi::c_int) as crate::stdlib::uInt;
        fill_window_from_input(s, strm, Some(dictionary_input));
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
    let mut len: crate::stdlib::uInt = 0;
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    if !deflate_params_stream_is_valid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(s) = strm.state.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_params_state_is_valid(s) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    len = (*s).strstart.wrapping_add((*s).lookahead);
    if len > (*s).w_size {
        len = (*s).w_size;
    }
    if !dictionary.is_null() && len != 0 {
        let start = (*s).strstart.wrapping_add((*s).lookahead).wrapping_sub(len);
        let Some(source) = (*s).window_bytes(start, len as usize) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        crate::stdlib::memcpy(
            dictionary as *mut ::core::ffi::c_void,
            source.as_ptr() as *const ::core::ffi::c_void,
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
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    if !deflate_params_stream_is_valid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(s) = strm.state.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_params_state_is_valid(s) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*strm).total_out = 0 as crate::stdlib::uLong;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*strm).data_type = crate::zlib_h::Z_UNKNOWN;
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
    crate::src::trees::tr_init(&mut *s);
    return crate::zlib_h::Z_OK;
}
#[export_name = "deflateResetKeep"]

pub unsafe extern "C" fn deflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateResetKeep(strm)
}
fn lm_init(s: &mut crate::src::deflate::deflate_state) {
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
/// Reset a stream after its state link has been validated and borrowed.
///
/// Initialization already owns both sides of this relationship, so it can
/// reuse the reset behavior without another raw state-link traversal.
fn deflate_reset_state(
    strm: &mut crate::zlib_h::z_stream_s,
    s: &mut crate::src::deflate::deflate_state,
) -> ::core::ffi::c_int {
    strm.total_out = 0 as crate::stdlib::uLong;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = crate::zlib_h::Z_UNKNOWN;
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
        crate::src::crc32::crc32(0 as crate::stdlib::uLong, &[])
    } else {
        1 as crate::stdlib::uLong
    };
    s.last_flush = -2 as ::core::ffi::c_int;
    crate::src::trees::tr_init(s);
    lm_init(s);
    crate::zlib_h::Z_OK
}

/// Reset a stream after its ABI state link has been converted at the caller
/// boundary.  Validation remains here so every caller shares the same reset
/// contract without placing stream logic in an FFI wrapper.
pub(crate) fn deflate_reset_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    state: Option<&mut crate::src::deflate::deflate_state>,
) -> ::core::ffi::c_int {
    if !deflate_params_stream_is_valid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(s) = state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_params_state_is_valid(s) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflate_reset_state(strm, s)
}
#[export_name = "deflateReset"]

pub unsafe extern "C" fn deflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let state = unsafe { strm.state.as_mut() };
    deflate_reset_impl(strm, state)
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

/// Pointer-free stream view used by the gzip-header operation.  The exported
/// boundary performs the one raw state-link conversion; validation remains
/// with the implementation so every caller receives the same error result.
struct DeflateHeaderStream<'a> {
    allocators_present: bool,
    state: Option<&'a mut crate::src::deflate::deflate_state>,
}

fn deflateSetHeader(
    stream: DeflateHeaderStream<'_>,
    header: Option<GzipHeader>,
) -> ::core::ffi::c_int {
    if !stream.allocators_present {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = stream.state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = DeflateState::validated(state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_set_header_impl(state.state, header)
}
#[export_name = "deflateSetHeader"]

pub unsafe extern "C" fn deflateSetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let stream = DeflateHeaderStream {
        allocators_present: strm.zalloc.is_some() && strm.zfree.is_some(),
        state: strm.state.as_mut(),
    };
    let header = head.as_ref().map(|head| {
        let extra = (!head.extra.is_null())
            .then(|| ::core::slice::from_raw_parts(head.extra, head.extra_len as usize));
        let name = (!head.name.is_null())
            .then(|| CStr::from_ptr(head.name.cast::<::core::ffi::c_char>()).to_bytes_with_nul());
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
    deflateSetHeader(stream, header)
}
pub unsafe extern "C" fn deflatePending(
    mut strm: crate::zlib_h::z_streamp,
    mut pending: *mut ::core::ffi::c_uint,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    if !deflate_params_stream_is_valid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = strm.state.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_params_state_is_valid(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !bits.is_null() {
        *bits = state.bi_valid;
    }
    if !pending.is_null() {
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
    deflatePending(strm, pending, bits)
}
fn deflate_used_impl(state: &crate::src::deflate::deflate_state) -> ::core::ffi::c_int {
    state.bi_used
}

/// Read the bit count from a previously validated deflate state.
///
/// ABI stream validation and conversion of its raw state link belong to the
/// exported wrapper.  The implementation only receives the typed state it
/// needs to inspect.
fn deflateUsed(
    state: &crate::src::deflate::deflate_state,
) -> Result<::core::ffi::c_int, ::core::ffi::c_int> {
    if !deflate_params_state_is_valid(state) {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok(deflate_used_impl(state))
}
#[export_name = "deflateUsed"]

pub unsafe extern "C" fn deflateUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let used = {
        let Some(strm) = strm.as_ref() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        // Validate the ABI stream before following its raw state link.
        if !deflate_params_stream_is_valid(strm) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let Some(state) = strm.state.as_ref() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        match deflateUsed(state) {
            Ok(used) => used,
            Err(error) => return error,
        }
    };
    if !bits.is_null() {
        *bits = used;
    }
    crate::zlib_h::Z_OK
}
/// A validated, borrowed deflate state.  The ABI stream owns the raw state
/// link; codec operations receive this facade after that link has been
/// checked and converted at the stream boundary.
struct DeflateState<'a> {
    state: &'a mut crate::src::deflate::deflate_state,
}

impl<'a> DeflateState<'a> {
    fn validated(state: &'a mut crate::src::deflate::deflate_state) -> Option<Self> {
        deflate_params_state_is_valid(state).then_some(Self { state })
    }
}

/// The pointer-free view of the portions of an ABI stream used by
/// `deflatePrime`.  The exported boundary converts the raw state link once;
/// validation remains with the codec operation below.
struct DeflatePrimeStream<'a> {
    allocators_present: bool,
    state: Option<&'a mut crate::src::deflate::deflate_state>,
}

/// The parameter-update path only needs to ask the main compressor for a
/// block boundary.  Keeping that dispatch behind this stream facade lets the
/// parameter logic operate on its already-borrowed state without itself
/// crossing back into the raw-stream implementation.
struct DeflateParamsStream<'a> {
    stream: &'a mut crate::zlib_h::z_stream_s,
}

/// A short-lived handle for one legacy ABI-stream compression step.
///
/// Gzip and other safe staging code use this named boundary rather than
/// invoking the legacy state-machine entry point themselves.  The remaining
/// unsafe transition stays beside the stream implementation until the ABI
/// stream's cursors and state link have owners of their own.
pub(crate) struct DeflateCall<'a> {
    stream: &'a mut crate::zlib_h::z_stream_s,
}

impl<'a> DeflateCall<'a> {
    pub(crate) fn new(stream: &'a mut crate::zlib_h::z_stream_s) -> Self {
        Self { stream }
    }

    pub(crate) fn compress(&mut self, flush: ::core::ffi::c_int) -> ::core::ffi::c_int {
        unsafe { deflate(self.stream, flush) }
    }
}

impl DeflateParamsStream<'_> {
    fn flush_block(&mut self) -> ::core::ffi::c_int {
        // `deflate` owns the remaining ABI cursor conversions for a complete
        // compression step.  This facade is intentionally the only
        // parameter-update route into that implementation.
        unsafe { deflate(self.stream, crate::zlib_h::Z_BLOCK) }
    }
}

/// Validate an already-converted stream view and apply pending bits.
fn deflate_prime_impl(
    stream: DeflatePrimeStream<'_>,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !stream.allocators_present {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = stream.state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = DeflateState::validated(state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_prime_state(state, bits, value)
}

/// Apply pending bits to an already-validated state facade.
fn deflate_prime_state(
    state: DeflateState<'_>,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut put: ::core::ffi::c_int = 0;
    let s = state.state;
    if bits < 0 as ::core::ffi::c_int
        || bits > 16 as ::core::ffi::c_int
        || s.sym_buf
            < s.pending_out.wrapping_add(
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
        crate::src::trees::flush_bits_impl(s);
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
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let stream = DeflatePrimeStream {
        allocators_present: strm.zalloc.is_some() && strm.zfree.is_some(),
        state: unsafe { strm.state.as_mut() },
    };
    deflate_prime_impl(stream, bits, value)
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
    if !deflate_params_state_is_valid(s) {
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
        let err = DeflateParamsStream { stream: strm }.flush_block();
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
                slide_hash(s);
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
/// Tune an already-validated deflate state.  This state-only operation has no
/// ABI pointers, so stream validation and the one state-link crossing stay in
/// its caller.
fn deflate_tune_state(
    state: &mut crate::src::deflate::deflate_state,
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deflate_params_state_is_valid(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.good_match = good_length as crate::stdlib::uInt;
    state.max_lazy_match = max_lazy as crate::stdlib::uInt;
    state.nice_match = nice_length;
    state.max_chain_length = max_chain as crate::stdlib::uInt;
    return crate::zlib_h::Z_OK;
}

/// Validate the ABI stream before borrowing the deflate state it owns.
///
/// The exported wrapper only converts its raw stream argument and dispatches
/// here; this adapter retains the state-specific validation outside the FFI
/// entry point.
pub unsafe fn deflateTune(
    strm: &mut crate::zlib_h::z_stream_s,
    good_length: ::core::ffi::c_int,
    max_lazy: ::core::ffi::c_int,
    nice_length: ::core::ffi::c_int,
    max_chain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !deflate_params_stream_is_valid(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *strm.state;
    deflate_tune_state(state, good_length, max_lazy, nice_length, max_chain)
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
    deflateTune(strm, good_length, max_lazy, nice_length, max_chain)
}
/// Compute a deflate bound after the ABI stream link has been converted at
/// the caller boundary.  Keeping validation here lets both exported bounds
/// share the exact state checks without retaining or dereferencing a raw
/// pointer in the calculation itself.
fn deflate_bound_z_impl(
    stream: Option<(
        &crate::zlib_h::z_stream_s,
        &crate::src::deflate::deflate_state,
    )>,
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
    let state = stream.and_then(|(strm, state)| {
        (deflate_params_stream_is_valid(strm)
            && core::ptr::eq(strm.state.cast_const(), core::ptr::from_ref(state))
            && deflate_params_state_is_valid(state))
        .then_some(state)
    });
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
    let stream = strm.as_ref();
    let state = stream.and_then(|strm| strm.state.as_ref());
    deflate_bound_z_impl(stream.zip(state), sourceLen)
}
#[export_name = "deflateBound"]

pub unsafe extern "C" fn deflateBound_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let stream = strm.as_ref();
    let state = stream.and_then(|strm| strm.state.as_ref());
    deflate_bound_z_impl(stream.zip(state), sourceLen as crate::stdlib::z_size_t)
        as crate::stdlib::uLong
}
fn put_short_msb(
    s: &mut crate::src::deflate::deflate_state,
    b: crate::stdlib::uInt,
) {
    s.put_pending_byte((b >> 8 as ::core::ffi::c_int) as crate::stdlib::Byte);
    s.put_pending_byte((b & 0xff as crate::stdlib::uInt) as crate::stdlib::Byte);
}

/// Move pending compressed bytes into the caller's bounded output range.
///
/// The pending bytes are owned by `s`; `write_stream_bytes()` is the sole
/// stream-output boundary and validates the ABI cursor before creating its
/// short-lived output slice.
fn flush_pending_impl(
    s: &mut crate::src::deflate::deflate_state,
    strm: &mut crate::zlib_h::z_stream_s,
) {
    let mut len: ::core::ffi::c_uint = 0;
    crate::src::trees::flush_bits_impl(s);
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
    if !write_stream_bytes(strm, source) {
        return;
    }
    s.pending_out = s.pending_out.wrapping_add(len as usize);
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
    // Validate the ABI carrier before following its state link.  The state
    // machine validation then operates on the borrowed state below, rather
    // than dispatching through the raw-pointer checker.
    if !deflate_params_stream_is_valid(strm)
        || flush > crate::zlib_h::Z_BLOCK
        || flush < 0 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Validate the state link before borrowing it.  The resulting reference
    // carries the state through this call, avoiding repeated raw-state
    // dereferences in the compression state machine.
    let s = &mut *strm.state;
    if !deflate_params_state_is_valid(s) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
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
        flush_pending_impl(s, strm);
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
        put_short_msb(s, header);
        if (*s).strstart != 0 as crate::stdlib::uInt {
            put_short_msb(
                s,
                (strm.adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
            );
            put_short_msb(
                s,
                (strm.adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
            );
        }
        // `adler32(NULL, 0)` returns the initial Adler value of one.  The
        // slice API has no null sentinel, so supply that exact seed for the
        // equivalent empty update.
        strm.adler = crate::src::adler32::adler32_z(1, &[]);
        (*s).status = crate::src::deflate::BUSY_STATE;
        flush_pending_impl(s, strm);
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
            flush_pending_impl(s, strm);
            if (*s).pending != 0 as crate::zutil_h::ulg {
                (*s).last_flush = -1 as ::core::ffi::c_int;
                return crate::zlib_h::Z_OK;
            }
        } else {
            let gzhead = gzhead.as_ref().expect("gzip header disappeared");
            (*s).put_pending_byte(
                (if gzhead.text != 0 { 1 } else { 0 })
                    + (if gzhead.hcrc != 0 { 2 } else { 0 })
                    + (if gzhead.extra.is_none() { 0 } else { 4 })
                    + (if gzhead.name.is_none() { 0 } else { 8 })
                    + (if gzhead.comment.is_none() { 0 } else { 16 }),
            );
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
                (*s).put_pending_byte(
                    (extra.len() as crate::stdlib::uInt & 0xff) as crate::stdlib::Bytef,
                );
                (*s).put_pending_byte(
                    (extra.len() as crate::stdlib::uInt >> 8 & 0xff) as crate::stdlib::Bytef,
                );
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
                let (Some(destination), Some(source)) = (
                    pending.get_mut(start..end),
                    extra.get(extra_start..extra_end),
                ) else {
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
                flush_pending_impl(s, strm);
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
            let (Some(destination), Some(source)) = (
                pending.get_mut(start..end),
                extra.get(extra_start..extra_end),
            ) else {
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
                    flush_pending_impl(s, strm);
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
                    flush_pending_impl(s, strm);
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
                flush_pending_impl(s, strm);
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
        flush_pending_impl(s, strm);
        if (*s).pending != 0 as crate::zutil_h::ulg {
            (*s).last_flush = -1 as ::core::ffi::c_int;
            return crate::zlib_h::Z_OK;
        }
    }
    if strm.avail_in != 0 as crate::stdlib::uInt
        || (*s).lookahead != 0 as crate::stdlib::uInt
        || flush != crate::zlib_h::Z_NO_FLUSH && (*s).status != crate::src::deflate::FINISH_STATE
    {
        // The stream and its input cursor were validated before entering the
        // compression state machine.  Every engine receives this one bounded
        // view and tracks progress through the ABI cursor it updates.
        let input = if strm.avail_in == 0 {
            &[]
        } else {
            core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
        };
        // `avail_out` and `next_out` were checked on entry.  All compression
        // engines below use the same bounded output view, so form it once
        // instead of repeating the ABI conversion in each dispatch arm.
        let output_len = strm.avail_out as usize;
        let output = core::slice::from_raw_parts_mut(strm.next_out, output_len);
        let mut bstate: block_state = need_more;
        bstate = (if (*s).level == 0 as ::core::ffi::c_int
            || matches!(
                configuration_table[(*s).level as usize].func,
                CompressionEngine::Stored
            )
        {
            let input_len = input.len();
            let mut io = DeflateStoredIo {
                input,
                input_pos: 0,
                output,
                output_pos: 0,
                total_in: strm.total_in,
                total_out: strm.total_out,
                adler: strm.adler,
            };
            let result = deflate_stored(s, &mut io, flush);
            if input_len != 0 {
                strm.next_in =
                    io.input.as_ptr().wrapping_add(io.input_pos) as *mut crate::stdlib::Bytef;
            }
            strm.avail_in = io.avail_in();
            strm.next_out = io.output.as_mut_ptr().wrapping_add(io.output_pos);
            strm.avail_out = io.avail_out();
            strm.total_in = io.total_in;
            strm.total_out = io.total_out;
            strm.adler = io.adler;
            result as ::core::ffi::c_uint
        } else if (*s).strategy == crate::zlib_h::Z_HUFFMAN_ONLY {
            let input_len = input.len();
            let mut io = DeflateFastIo {
                input,
                input_pos: 0,
                output,
                output_pos: 0,
                total_in: strm.total_in,
                total_out: strm.total_out,
                adler: strm.adler,
                data_type: strm.data_type,
            };
            let result = deflate_huff(&mut *s, &mut io, flush);
            if input_len != 0 {
                strm.next_in = io.input.as_ptr().wrapping_add(io.input_pos)
                    as *mut crate::stdlib::Bytef;
            }
            strm.avail_in = io.avail_in();
            strm.next_out = io.output.as_mut_ptr().wrapping_add(io.output_pos);
            strm.avail_out = io.avail_out();
            strm.total_in = io.total_in;
            strm.total_out = io.total_out;
            strm.adler = io.adler;
            strm.data_type = io.data_type;
            result as ::core::ffi::c_uint
        } else if (*s).strategy == crate::zlib_h::Z_RLE {
            let input_len = input.len();
            let mut io = DeflateFastIo {
                input,
                input_pos: 0,
                output,
                output_pos: 0,
                total_in: strm.total_in,
                total_out: strm.total_out,
                adler: strm.adler,
                data_type: strm.data_type,
            };
            let result = deflate_rle(s, &mut io, flush);
            if input_len != 0 {
                strm.next_in = io.input.as_ptr().wrapping_add(io.input_pos)
                    as *mut crate::stdlib::Bytef;
            }
            strm.avail_in = io.avail_in();
            strm.next_out = io.output.as_mut_ptr().wrapping_add(io.output_pos);
            strm.avail_out = io.avail_out();
            strm.total_in = io.total_in;
            strm.total_out = io.total_out;
            strm.adler = io.adler;
            strm.data_type = io.data_type;
            result as ::core::ffi::c_uint
        } else {
            (match configuration_table[(*s).level as usize].func {
                CompressionEngine::Stored => {
                    unreachable!("stored levels use the slice-based engine")
                }
                CompressionEngine::Fast => {
                    let input_len = input.len();
                    let mut io = DeflateFastIo {
                        input,
                        input_pos: 0,
                        output,
                        output_pos: 0,
                        total_in: strm.total_in,
                        total_out: strm.total_out,
                        adler: strm.adler,
                        data_type: strm.data_type,
                    };
                    let result = deflate_fast(s, &mut io, flush);
                    if input_len != 0 {
                        strm.next_in = io.input.as_ptr().wrapping_add(io.input_pos)
                            as *mut crate::stdlib::Bytef;
                    }
                    strm.avail_in = io.avail_in();
                    strm.next_out = io.output.as_mut_ptr().wrapping_add(io.output_pos);
                    strm.avail_out = io.avail_out();
                    strm.total_in = io.total_in;
                    strm.total_out = io.total_out;
                    strm.adler = io.adler;
                    strm.data_type = io.data_type;
                    result
                }
                CompressionEngine::Slow => {
                    let input_len = input.len();
                    let mut io = DeflateFastIo {
                        input,
                        input_pos: 0,
                        output,
                        output_pos: 0,
                        total_in: strm.total_in,
                        total_out: strm.total_out,
                        adler: strm.adler,
                        data_type: strm.data_type,
                    };
                    let result = deflate_slow(&mut *s, &mut io, flush);
                    if input_len != 0 {
                        strm.next_in = io.input.as_ptr().wrapping_add(io.input_pos)
                            as *mut crate::stdlib::Bytef;
                    }
                    strm.avail_in = io.avail_in();
                    strm.next_out = io.output.as_mut_ptr().wrapping_add(io.output_pos);
                    strm.avail_out = io.avail_out();
                    strm.total_in = io.total_in;
                    strm.total_out = io.total_out;
                    strm.adler = io.adler;
                    strm.data_type = io.data_type;
                    result
                }
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
                crate::src::trees::tr_align(s);
            } else if flush != crate::zlib_h::Z_BLOCK {
                crate::src::trees::tr_stored_block(
                    &mut *s,
                    &[],
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
            flush_pending_impl(s, strm);
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
        put_short_msb(
            s,
            (strm.adler >> 16 as ::core::ffi::c_int) as crate::stdlib::uInt,
        );
        put_short_msb(
            s,
            (strm.adler & 0xffff as crate::stdlib::uLong) as crate::stdlib::uInt,
        );
    }
    flush_pending_impl(s, strm);
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
/// Tear down the owned portions of a validated deflate state.
///
/// The state allocation itself remains the responsibility of
/// `deflate_end_release()`: it was obtained through the stream's ABI
/// allocator and must be returned through that same allocator.
pub fn deflateEnd(
    state: &mut crate::src::deflate::deflate_state,
) -> ::core::ffi::c_int {
    let (status, gzhead, head, prev, pending, allocations) = {
        if state.status != crate::src::deflate::INIT_STATE
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
        (
            state.status,
            state.gzhead,
            state.head.take(),
            state.prev.take(),
            state.pending_buf.take(),
            state.window.take(),
        )
    };
    gzip_header_remove(gzhead);
    drop(head);
    drop(prev);
    drop(pending);
    drop(allocations);
    return if status == crate::src::deflate::BUSY_STATE {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        crate::zlib_h::Z_OK
    };
}

/// Release the ABI allocation around the safe state teardown.  The pointer
/// was installed by the initializer using this stream's allocator, so this
/// is the one boundary that may return it through that allocator.
pub(crate) unsafe fn deflate_end_release(
    strm: &mut crate::zlib_h::z_stream_s,
) -> ::core::ffi::c_int {
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state_allocation = strm.state.cast::<::core::ffi::c_void>();
    // Validate the stream before borrowing its state.  The initializer owns
    // this allocation, and only this boundary returns it through `zfree`.
    let Some(state) = strm.state.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let end = deflateEnd(state);
    if end != crate::zlib_h::Z_STREAM_ERROR {
        let zfree = strm.zfree.expect("deflate_end_release validates zfree");
        zfree(strm.opaque, state_allocation);
        strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    }
    end
}

#[export_name = "deflateEnd"]

pub unsafe extern "C" fn deflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    deflate_end_release(strm)
}
fn deflate_copy_state_is_valid(
    strm: &crate::zlib_h::z_stream_s,
    state: &crate::src::deflate::deflate_state,
) -> bool {
    let Ok(window_size) = usize::try_from(state.window_size) else {
        return false;
    };
    let Ok(w_size) = usize::try_from(state.w_size) else {
        return false;
    };
    let Ok(hash_size) = usize::try_from(state.hash_size) else {
        return false;
    };
    let Ok(pending_size) = usize::try_from(state.pending_buf_size) else {
        return false;
    };
    let Ok(high_water) = usize::try_from(state.high_water) else {
        return false;
    };
    let Ok(pending) = usize::try_from(state.pending) else {
        return false;
    };

    deflate_params_stream_is_valid(strm)
        && deflate_params_state_is_valid(state)
        && state
            .window
            .as_ref()
            .is_some_and(|window| window.len() == window_size)
        && window_size == w_size.saturating_mul(2)
        && high_water <= window_size
        && state
            .head
            .as_ref()
            .is_some_and(|head| head.len() == hash_size)
        && state.prev.as_ref().is_some_and(|prev| prev.len() == w_size)
        && state.pending_buf.as_ref().is_some_and(|buffer| {
            state.pending_out <= buffer.len()
                && pending <= buffer.len().saturating_sub(state.pending_out)
                && buffer.len() == pending_size
        })
}

fn deflate_copy_state(
    source: &crate::src::deflate::deflate_state,
    head: Vec<crate::src::deflate::Posf>,
    prev: Vec<crate::src::deflate::Posf>,
    pending: Vec<crate::stdlib::Bytef>,
    gzhead: u64,
) -> crate::src::deflate::deflate_state {
    crate::src::deflate::internal_state {
        status: source.status,
        pending_buf: Some(pending),
        pending_buf_size: source.pending_buf_size,
        pending_out: source.pending_out,
        pending: source.pending,
        wrap: source.wrap,
        gzhead,
        gzindex: source.gzindex,
        method: source.method,
        last_flush: source.last_flush,
        w_size: source.w_size,
        w_bits: source.w_bits,
        w_mask: source.w_mask,
        window: None,
        window_size: source.window_size,
        prev: Some(prev),
        head: Some(head),
        ins_h: source.ins_h,
        hash_size: source.hash_size,
        hash_bits: source.hash_bits,
        hash_mask: source.hash_mask,
        hash_shift: source.hash_shift,
        block_start: source.block_start,
        match_length: source.match_length,
        prev_match: source.prev_match,
        match_available: source.match_available,
        strstart: source.strstart,
        match_start: source.match_start,
        lookahead: source.lookahead,
        prev_length: source.prev_length,
        max_chain_length: source.max_chain_length,
        max_lazy_match: source.max_lazy_match,
        level: source.level,
        strategy: source.strategy,
        good_match: source.good_match,
        nice_match: source.nice_match,
        dyn_ltree: source.dyn_ltree,
        dyn_dtree: source.dyn_dtree,
        bl_tree: source.bl_tree,
        l_desc: source.l_desc,
        d_desc: source.d_desc,
        bl_desc: source.bl_desc,
        bl_count: source.bl_count,
        heap: source.heap,
        heap_len: source.heap_len,
        heap_max: source.heap_max,
        depth: source.depth,
        sym_buf: source.lit_bufsize as usize,
        lit_bufsize: source.lit_bufsize,
        sym_next: source.sym_next,
        sym_end: source.sym_end,
        opt_len: source.opt_len,
        static_len: source.static_len,
        matches: source.matches,
        insert: source.insert,
        bi_buf: source.bi_buf,
        bi_valid: source.bi_valid,
        bi_used: source.bi_used,
        high_water: source.high_water,
        slid: source.slid,
    }
}

unsafe fn deflate_copy_impl(
    dest: &mut crate::zlib_h::z_stream_s,
    source: &crate::zlib_h::z_stream_s,
) -> ::core::ffi::c_int {
    // Validate the stream before following its state link.  The state borrow
    // is then retained for the complete snapshot preparation below.
    if !deflate_params_stream_is_valid(source) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(source_state) = source.state.as_ref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !deflate_copy_state_is_valid(source, source_state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let Some(head) = source_state.head.as_deref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(head_copy) = clone_head_table(head) else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let Some(prev) = source_state.prev.as_deref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(prev_copy) = clone_prev_table(prev) else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let Some(source_pending) = source_state.pending_buf.as_deref() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(mut pending_copy) = allocate_pending_buffer(source_pending.len()) else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    pending_copy.copy_from_slice(source_pending);
    let gzhead = gzip_header_clone(source_state.gzhead);

    // `z_stream_s` is an ABI carrier and intentionally Copy.  Its state
    // field is replaced below only after the duplicate allocation succeeds.
    *dest = *source;
    let zalloc = dest.zalloc.expect("validated source allocator");
    let zfree = dest.zfree.expect("validated source deallocator");
    let state_memory = zalloc(
        dest.opaque,
        1,
        ::core::mem::size_of::<crate::src::deflate::deflate_state>() as crate::stdlib::uInt,
    )
    .cast::<crate::src::deflate::deflate_state>();
    if state_memory.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    let Some(source_window) = source_state.window.as_deref() else {
        zfree(dest.opaque, state_memory.cast());
        dest.state = ::core::ptr::null_mut();
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(window) = allocate_window(source_window.len()) else {
        zfree(dest.opaque, state_memory.cast());
        dest.state = ::core::ptr::null_mut();
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let high_water = source_state.high_water as usize;
    if high_water > source_window.len() {
        zfree(dest.opaque, state_memory.cast());
        dest.state = ::core::ptr::null_mut();
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut window = window;
    window[..high_water].copy_from_slice(&source_window[..high_water]);

    let copied_state = deflate_copy_state(
        source_state,
        head_copy,
        prev_copy,
        pending_copy,
        gzip_header_store(gzhead),
    );
    let mut copied_state = copied_state;
    copied_state.window = Some(window);
    core::ptr::write(state_memory, copied_state);
    dest.state = state_memory;
    crate::zlib_h::Z_OK
}

/// Convert ABI stream pointers once before the copy implementation borrows
/// either stream.  Keeping this adapter separate leaves the exported symbol
/// as a one-call dispatch while all copy validation remains in the
/// implementation.
unsafe fn deflateCopy(
    dest: crate::zlib_h::z_streamp,
    source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if dest.is_null() || source.is_null() || dest == source {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    deflate_copy_impl(&mut *dest, &*source)
}
#[export_name = "deflateCopy"]

pub unsafe extern "C" fn deflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    deflateCopy(dest, source)
}
fn longest_match(
    s: &mut crate::src::deflate::deflate_state,
    mut cur_match: crate::src::deflate::IPos,
) -> crate::stdlib::uInt {
    let mut chain_length = s.max_chain_length as ::core::ffi::c_uint;
    let lookahead = s.lookahead;
    let Some(window) = s.window.as_deref() else {
        return lookahead;
    };
    let Ok(scan_start) = usize::try_from(s.strstart) else {
        return lookahead;
    };
    let Some(scan) = window.get(scan_start..) else {
        return lookahead;
    };
    let mut best_len = s.prev_length as ::core::ffi::c_int;
    let mut nice_match = s.nice_match;
    let limit = if s.strstart
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
    let Some(prev) = s.prev.as_deref() else {
        return lookahead;
    };
    let wmask = s.w_mask;
    if s.prev_length >= s.good_match {
        chain_length >>= 2 as ::core::ffi::c_int;
    }
    if nice_match as crate::stdlib::uInt > s.lookahead {
        nice_match = s.lookahead as ::core::ffi::c_int;
    }
    let mut best_match = None;
    loop {
        let Ok(match_start) = usize::try_from(cur_match) else {
            break;
        };
        let Some(candidate) = window.get(match_start..) else {
            break;
        };
        let len = scan
            .iter()
            .zip(candidate)
            .take(crate::zutil_h::MAX_MATCH as usize)
            .take(lookahead as usize)
            .take_while(|(scan_byte, match_byte)| scan_byte == match_byte)
            .count() as ::core::ffi::c_int;
        if len > best_len {
            best_match = Some(cur_match as crate::stdlib::uInt);
            best_len = len;
            if len >= nice_match {
                break;
            }
        }
        let Some(&previous) = prev.get((cur_match as crate::stdlib::uInt & wmask) as usize) else {
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
    if let Some(best_match) = best_match {
        s.match_start = best_match;
    }
    if best_len as crate::stdlib::uInt <= lookahead {
        return best_len as crate::stdlib::uInt;
    }
    return lookahead;
}

pub const MAX_STORED: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;

struct DeflateStoredIo<'a> {
    input: &'a [crate::stdlib::Bytef],
    input_pos: usize,
    output: &'a mut [crate::stdlib::Bytef],
    output_pos: usize,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
}

/// Bounded cursors used by the fast engine.  The ABI stream is converted to
/// this value by `deflate()` and updated again only after the engine returns.
/// Keeping the engine's cursor state here prevents the compression loop from
/// retaining an ABI carrier (and its raw pointers).
struct DeflateFastIo<'a> {
    input: &'a [crate::stdlib::Bytef],
    input_pos: usize,
    output: &'a mut [crate::stdlib::Bytef],
    output_pos: usize,
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    data_type: ::core::ffi::c_int,
}

impl DeflateFastIo<'_> {
    fn avail_in(&self) -> crate::stdlib::uInt {
        (self.input.len() - self.input_pos) as crate::stdlib::uInt
    }

    fn avail_out(&self) -> crate::stdlib::uInt {
        (self.output.len() - self.output_pos) as crate::stdlib::uInt
    }

    fn consume_input(&mut self, len: usize, wrap: ::core::ffi::c_int) -> bool {
        let Some(end) = self.input_pos.checked_add(len) else {
            return false;
        };
        let Some(bytes) = self.input.get(self.input_pos..end) else {
            return false;
        };
        if wrap == 1 {
            self.adler = crate::src::adler32::adler32_z(self.adler, bytes);
        } else if wrap == 2 {
            self.adler = crate::src::crc32::crc32(self.adler, bytes);
        }
        self.input_pos = end;
        self.total_in = self.total_in.wrapping_add(len as crate::stdlib::uLong);
        true
    }

    fn write(&mut self, bytes: &[crate::stdlib::Bytef]) -> bool {
        let Some(end) = self.output_pos.checked_add(bytes.len()) else {
            return false;
        };
        let Some(output) = self.output.get_mut(self.output_pos..end) else {
            return false;
        };
        output.copy_from_slice(bytes);
        self.output_pos = end;
        self.total_out = self
            .total_out
            .wrapping_add(bytes.len() as crate::stdlib::uLong);
        true
    }

    fn flush_pending(&mut self, s: &mut crate::src::deflate::deflate_state) {
        crate::src::trees::flush_bits_impl(s);
        let len = (s.pending as usize).min(self.avail_out() as usize);
        if len == 0 {
            return;
        }
        let start = s.pending_out;
        let Some(end) = start.checked_add(len) else {
            return;
        };
        let Some(bytes) = s
            .pending_buf
            .as_ref()
            .and_then(|pending| pending.get(start..end))
        else {
            return;
        };
        if !self.write(bytes) {
            return;
        }
        s.pending_out = end;
        s.pending = s.pending.wrapping_sub(len as crate::zutil_h::ulg);
        if s.pending == 0 {
            s.pending_out = 0;
        }
    }
}

/// Reuse the established window-filling logic with the Huffman engine's
/// slice-based cursors.  The temporary stream is local bookkeeping only: it
/// never escapes this helper, and its updated scalar cursor state is copied
/// back into `io` before the engine resumes.
fn fill_window_from_fast_io(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut DeflateFastIo<'_>,
) {
    let input = &io.input[io.input_pos..];
    let mut stream = crate::zlib_h::z_stream_s {
        next_in: input.as_ptr().cast_mut(),
        avail_in: io.avail_in(),
        total_in: io.total_in,
        next_out: ::core::ptr::null_mut(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut(),
        state: ::core::ptr::null_mut(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut(),
        data_type: 0,
        adler: io.adler,
        reserved: 0,
    };
    let available_before = stream.avail_in;
    fill_window_from_input(s, &mut stream, Some(input));
    let consumed = available_before.wrapping_sub(stream.avail_in) as usize;
    io.input_pos = io.input_pos.saturating_add(consumed).min(io.input.len());
    io.total_in = stream.total_in;
    io.adler = stream.adler;
}

impl DeflateStoredIo<'_> {
    fn avail_in(&self) -> crate::stdlib::uInt {
        (self.input.len() - self.input_pos) as crate::stdlib::uInt
    }

    fn avail_out(&self) -> crate::stdlib::uInt {
        (self.output.len() - self.output_pos) as crate::stdlib::uInt
    }

    fn write(&mut self, bytes: &[crate::stdlib::Bytef]) -> bool {
        let Some(end) = self.output_pos.checked_add(bytes.len()) else {
            return false;
        };
        let Some(output) = self.output.get_mut(self.output_pos..end) else {
            return false;
        };
        output.copy_from_slice(bytes);
        self.output_pos = end;
        self.total_out = self
            .total_out
            .wrapping_add(bytes.len() as crate::stdlib::uLong);
        true
    }

    fn copy_input_to_output(&mut self, len: crate::stdlib::uInt, wrap: ::core::ffi::c_int) -> bool {
        let len = len as usize;
        let Some(input_end) = self.input_pos.checked_add(len) else {
            return false;
        };
        let Some(output_end) = self.output_pos.checked_add(len) else {
            return false;
        };
        let Some(input) = self.input.get(self.input_pos..input_end) else {
            return false;
        };
        let Some(output) = self.output.get_mut(self.output_pos..output_end) else {
            return false;
        };
        output.copy_from_slice(input);
        if wrap == 1 {
            self.adler = crate::src::adler32::adler32_z(self.adler, output);
        } else if wrap == 2 {
            self.adler = crate::src::crc32::crc32(self.adler, output);
        }
        self.input_pos = input_end;
        self.output_pos = output_end;
        self.total_in = self.total_in.wrapping_add(len as crate::stdlib::uLong);
        self.total_out = self.total_out.wrapping_add(len as crate::stdlib::uLong);
        true
    }

    fn input_prefix(&self, len: crate::stdlib::uInt) -> Option<&[crate::stdlib::Bytef]> {
        let end = self.input_pos.checked_add(len as usize)?;
        self.input.get(self.input_pos..end)
    }

    fn consume_input(&mut self, len: crate::stdlib::uInt) -> bool {
        let Some(end) = self.input_pos.checked_add(len as usize) else {
            return false;
        };
        if end > self.input.len() {
            return false;
        }
        self.input_pos = end;
        self.total_in = self.total_in.wrapping_add(len as crate::stdlib::uLong);
        true
    }

    fn consumed_input(&self, len: crate::stdlib::uInt) -> Option<&[crate::stdlib::Bytef]> {
        let start = self.input_pos.checked_sub(len as usize)?;
        self.input.get(start..self.input_pos)
    }

    fn flush_pending(&mut self, s: &mut crate::src::deflate::deflate_state) {
        crate::src::trees::flush_bits_impl(s);
        let len = (s.pending as usize).min(self.avail_out() as usize);
        if len == 0 {
            return;
        }
        let start = s.pending_out;
        let Some(end) = start.checked_add(len) else {
            return;
        };
        let Some(source) = s
            .pending_buf
            .as_ref()
            .and_then(|pending| pending.get(start..end))
        else {
            return;
        };
        if !self.write(source) {
            return;
        }
        s.pending_out = end;
        s.pending = s.pending.wrapping_sub(len as crate::zutil_h::ulg);
        if s.pending == 0 {
            s.pending_out = 0;
        }
    }
}

fn deflate_stored(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut DeflateStoredIo<'_>,
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
    let mut used: ::core::ffi::c_uint = io.avail_in() as ::core::ffi::c_uint;
    loop {
        len = MAX_STORED as ::core::ffi::c_uint;
        have = (s.bi_valid as ::core::ffi::c_uint).wrapping_add(42 as ::core::ffi::c_uint)
            >> 3 as ::core::ffi::c_int;
        if io.avail_out() < have {
            break;
        }
        have = (io.avail_out() as ::core::ffi::c_uint).wrapping_sub(have);
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
        crate::src::trees::tr_stored_block(s, &[], 0 as crate::zutil_h::ulg, last);
        // `_tr_stored_block()` just appended the four-byte length trailer.
        // Replace its zero length with this block's actual length.
        let _ = s.set_stored_block_length(len as crate::stdlib::uInt);
        io.flush_pending(s);
        if left != 0 {
            if left > len {
                left = len;
            }
            let Some(source) = s.block_data(
                s.block_start as crate::stdlib::uInt,
                left as crate::zutil_h::ulg,
            ) else {
                break;
            };
            if !io.write(&source) {
                break;
            }
            s.block_start += left as ::core::ffi::c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            if !io.copy_input_to_output(len, s.wrap) {
                break;
            }
        }
        if last != 0 as ::core::ffi::c_int {
            break;
        }
    }
    used = used.wrapping_sub(io.avail_in() as ::core::ffi::c_uint);
    if used != 0 {
        if let Some(input) = io.consumed_input(used) {
            if s.retain_stored_history(&input) {
                s.block_start = s.strstart as ::core::ffi::c_long;
            }
        }
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
        && io.avail_in() == 0 as crate::stdlib::uInt
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
        let Ok(copy_len) = usize::try_from(s.strstart) else {
            return need_more;
        };
        let Ok(window_len) = usize::try_from(s.window_size) else {
            return need_more;
        };
        let Ok(wsize) = usize::try_from(s.w_size) else {
            return need_more;
        };
        let Some(window) = s.window_bytes_mut(0, window_len) else {
            return need_more;
        };
        let Some(copy_end) = wsize.checked_add(copy_len) else {
            return need_more;
        };
        if copy_end > window.len() {
            return need_more;
        }
        window.copy_within(wsize..copy_end, 0);
        if s.matches < 2 as crate::stdlib::uInt {
            s.matches = s.matches.wrapping_add(1);
        }
        have = have.wrapping_add(s.w_size as ::core::ffi::c_uint);
        if s.insert > s.strstart {
            s.insert = s.strstart;
        }
    }
    if have > io.avail_in() {
        have = io.avail_in() as ::core::ffi::c_uint;
    }
    if have != 0 {
        if let Some(input) = io.input_prefix(have) {
            if !s.append_stored_input(input) {
                return need_more;
            }
            if s.wrap == 1 {
                io.adler = crate::src::adler32::adler32_z(io.adler, input);
            } else if s.wrap == 2 {
                io.adler = crate::src::crc32::crc32(io.adler, input);
            }
            if !io.consume_input(have) {
                return need_more;
            }
        } else {
            return need_more;
        }
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
            && io.avail_in() == 0 as crate::stdlib::uInt
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == crate::zlib_h::Z_FINISH
            && io.avail_in() == 0 as crate::stdlib::uInt
            && len == left
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let Some(source) = s.block_data(
            s.block_start as crate::stdlib::uInt,
            len as crate::zutil_h::ulg,
        ) else {
            return need_more;
        };
        crate::src::trees::tr_stored_block(s, &source, len as crate::zutil_h::ulg, last);
        s.block_start += len as ::core::ffi::c_long;
        io.flush_pending(s);
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

/// Flush the current fast-deflate block and report whether the caller can
/// continue producing output.  All callers use the same block boundaries;
/// centralizing the tree dispatch keeps that boundary in one place.
fn fill_fast_window(s: &mut crate::src::deflate::deflate_state, io: &mut DeflateFastIo<'_>) {
    let wsize = s.w_size;
    let Ok(window_len) = usize::try_from(s.window_size) else {
        return;
    };
    if s.window
        .as_ref()
        .is_none_or(|window| window.len() != window_len)
    {
        return;
    }
    loop {
        let mut more =
            s.window_size
                .wrapping_sub(s.lookahead as crate::zutil_h::ulg)
                .wrapping_sub(s.strstart as crate::zutil_h::ulg) as ::core::ffi::c_uint;
        if ::core::mem::size_of::<::core::ffi::c_int>() <= 2 {
            if more == 0 && s.strstart == 0 && s.lookahead == 0 {
                more = wsize as ::core::ffi::c_uint;
            } else if more == -1i32 as ::core::ffi::c_uint {
                more = more.wrapping_sub(1);
            }
        }
        if s.strstart
            >= wsize.wrapping_add(
                s.w_size
                    .wrapping_sub(crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt),
            )
        {
            let Ok(window_base) = usize::try_from(wsize) else {
                return;
            };
            let Ok(copy_len) = usize::try_from(wsize.wrapping_sub(more)) else {
                return;
            };
            let Some(copy_end) = window_base.checked_add(copy_len) else {
                return;
            };
            if copy_end > window_len {
                return;
            }
            let Some(window) = s.window.as_deref_mut() else {
                return;
            };
            window.copy_within(window_base..copy_end, 0);
            s.match_start = s.match_start.wrapping_sub(wsize);
            s.strstart = s.strstart.wrapping_sub(wsize);
            s.block_start -= wsize as ::core::ffi::c_long;
            if s.insert > s.strstart {
                s.insert = s.strstart;
            }
            slide_hash(s);
            more = more.wrapping_add(wsize as ::core::ffi::c_uint);
        }
        if io.avail_in() == 0 {
            break;
        }
        let n = io.avail_in().min(more);
        let Ok(start) = usize::try_from(s.strstart.wrapping_add(s.lookahead)) else {
            return;
        };
        let Ok(n_usize) = usize::try_from(n) else {
            return;
        };
        let Some(end) = start.checked_add(n_usize) else {
            return;
        };
        if end > window_len {
            return;
        }
        if n != 0 {
            let Some(input_end) = io.input_pos.checked_add(n_usize) else {
                return;
            };
            let Some(input) = io.input.get(io.input_pos..input_end) else {
                return;
            };
            let Some(window) = s.window.as_deref_mut() else {
                return;
            };
            window[start..end].copy_from_slice(input);
            if !io.consume_input(n_usize, s.wrap) {
                return;
            }
        }
        s.lookahead = s.lookahead.wrapping_add(n);
        if s.lookahead.wrapping_add(s.insert) >= crate::zutil_h::MIN_MATCH as crate::stdlib::uInt {
            let mut str = s.strstart.wrapping_sub(s.insert);
            let Ok(first) = usize::try_from(str) else {
                return;
            };
            let Some(second) = first.checked_add(1) else {
                return;
            };
            let Some(window) = s.window.as_deref() else {
                return;
            };
            let (Some(&first_byte), Some(&second_byte)) = (window.get(first), window.get(second))
            else {
                return;
            };
            s.ins_h = first_byte as crate::stdlib::uInt;
            s.ins_h = (s.ins_h << s.hash_shift ^ second_byte as crate::stdlib::uInt) & s.hash_mask;
            while s.insert != 0 {
                let Ok(next) = usize::try_from(str.wrapping_add(2)) else {
                    return;
                };
                let Some(&next_byte) = window.get(next) else {
                    return;
                };
                s.ins_h =
                    (s.ins_h << s.hash_shift ^ next_byte as crate::stdlib::uInt) & s.hash_mask;
                let head_index = s.ins_h as usize;
                let Some(hash_head) = s
                    .head
                    .as_ref()
                    .and_then(|head| head.get(head_index))
                    .copied()
                else {
                    return;
                };
                let prev_index = (str & s.w_mask) as usize;
                let Some(slot) = s.prev.as_mut().and_then(|prev| prev.get_mut(prev_index)) else {
                    return;
                };
                *slot = hash_head;
                let Some(head) = s.head.as_mut() else { return };
                let Some(slot) = head.get_mut(head_index) else {
                    return;
                };
                *slot = str as crate::src::deflate::Pos as crate::src::deflate::Posf;
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
        let (start, init) = if s.high_water < curr {
            let init = s
                .window_size
                .wrapping_sub(curr)
                .min(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg);
            s.high_water = curr.wrapping_add(init);
            (curr, init)
        } else if s.high_water
            < curr.wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
        {
            let init = curr
                .wrapping_add(crate::src::deflate::WIN_INIT as crate::zutil_h::ulg)
                .wrapping_sub(s.high_water)
                .min(s.window_size.wrapping_sub(s.high_water));
            let start = s.high_water;
            s.high_water = s.high_water.wrapping_add(init);
            (start, init)
        } else {
            (0, 0)
        };
        let (Ok(start), Ok(init)) = (usize::try_from(start), usize::try_from(init)) else {
            return;
        };
        let Some(end) = start.checked_add(init) else {
            return;
        };
        let Some(window) = s.window.as_deref_mut() else {
            return;
        };
        let Some(initialized) = window.get_mut(start..end) else {
            return;
        };
        initialized.fill(0);
    }
}

fn flush_fast_block(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut DeflateFastIo<'_>,
    last: ::core::ffi::c_int,
) -> bool {
    let block_len = (s.strstart as ::core::ffi::c_long - s.block_start) as crate::zutil_h::ulg;
    let block = if s.block_start >= 0 {
        s.block_data(s.block_start as crate::stdlib::uInt, block_len)
    } else {
        None
    };
    // `block_data()` borrows the owned window for this synchronous tree
    // encoding operation; `tr_flush_block()` neither retains it nor the
    // stream reference.
    crate::src::trees::tr_flush_block(
        s,
        Some(&mut io.data_type),
        block.as_deref(),
        block_len,
        last,
    );
    s.block_start = s.strstart as ::core::ffi::c_long;
    io.flush_pending(s);
    io.avail_out() != 0
}

fn deflate_fast(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut DeflateFastIo<'_>,
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if s.lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_fast_window(s, io);
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
            s.match_length = longest_match(s, hash_head);
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
            if !flush_fast_block(s, io, 0) {
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
        if !flush_fast_block(s, io, 1) {
            return finish_started;
        }
        return finish_done;
    }
    if s.sym_next != 0 {
        if !flush_fast_block(s, io, 0) {
            return need_more;
        }
    }
    return block_done;
}

fn deflate_slow(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut DeflateFastIo<'_>,
    mut flush: ::core::ffi::c_int,
) -> block_state {
    let mut hash_head: crate::src::deflate::IPos = 0;
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if (*s).lookahead < crate::src::deflate::MIN_LOOKAHEAD as crate::stdlib::uInt {
            fill_window_from_fast_io(s, io);
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
                let block_len = ((*s).strstart as ::core::ffi::c_long - (*s).block_start)
                    as crate::zutil_h::ulg;
                let block = if (*s).block_start >= 0 {
                    (*s).block_data((*s).block_start as crate::stdlib::uInt, block_len)
                } else {
                    None
                };
                crate::src::trees::tr_flush_block(
                    s,
                    Some(&mut io.data_type),
                    block.as_deref(),
                    block_len,
                    0,
                );
                (*s).block_start = (*s).strstart as ::core::ffi::c_long;
                io.flush_pending(s);
                if io.avail_out() == 0 as crate::stdlib::uInt {
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
                let block_len = ((*s).strstart as ::core::ffi::c_long - (*s).block_start)
                    as crate::zutil_h::ulg;
                let block = if (*s).block_start >= 0 {
                    (*s).block_data((*s).block_start as crate::stdlib::uInt, block_len)
                } else {
                    None
                };
                crate::src::trees::tr_flush_block(
                    s,
                    Some(&mut io.data_type),
                    block.as_deref(),
                    block_len,
                    0,
                );
                (*s).block_start = (*s).strstart as ::core::ffi::c_long;
                io.flush_pending(s);
            }
            (*s).strstart = (*s).strstart.wrapping_add(1);
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            if io.avail_out() == 0 as crate::stdlib::uInt {
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
        let block_len =
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg;
        let block = if (*s).block_start >= 0 {
            (*s).block_data((*s).block_start as crate::stdlib::uInt, block_len)
        } else {
            None
        };
        crate::src::trees::tr_flush_block(
            s,
            Some(&mut io.data_type),
            block.as_deref(),
            block_len,
            1,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        io.flush_pending(s);
        if io.avail_out() == 0 as crate::stdlib::uInt {
            return (if true {
                finish_started as ::core::ffi::c_int
            } else {
                need_more as ::core::ffi::c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).sym_next != 0 {
        let block_len =
            ((*s).strstart as ::core::ffi::c_long - (*s).block_start) as crate::zutil_h::ulg;
        let block = if (*s).block_start >= 0 {
            (*s).block_data((*s).block_start as crate::stdlib::uInt, block_len)
        } else {
            None
        };
        crate::src::trees::tr_flush_block(
            s,
            Some(&mut io.data_type),
            block.as_deref(),
            block_len,
            0,
        );
        (*s).block_start = (*s).strstart as ::core::ffi::c_long;
        io.flush_pending(s);
        if io.avail_out() == 0 as crate::stdlib::uInt {
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
    s: &mut crate::src::deflate::deflate_state,
    io: &mut DeflateFastIo<'_>,
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if s.lookahead <= crate::zutil_h::MAX_MATCH as crate::stdlib::uInt {
            fill_window_from_fast_io(s, io);
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
            crate::src::trees::tr_flush_block(
                s,
                Some(&mut io.data_type),
                block.as_deref(),
                block_len,
                0,
            );
            s.block_start = s.strstart as ::core::ffi::c_long;
            io.flush_pending(s);
            if io.avail_out() == 0 {
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
        crate::src::trees::tr_flush_block(
            s,
            Some(&mut io.data_type),
            block.as_deref(),
            block_len,
            1,
        );
        s.block_start = s.strstart as ::core::ffi::c_long;
        io.flush_pending(s);
        if io.avail_out() == 0 {
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
        crate::src::trees::tr_flush_block(
            s,
            Some(&mut io.data_type),
            block.as_deref(),
            block_len,
            0,
        );
        s.block_start = s.strstart as ::core::ffi::c_long;
        io.flush_pending(s);
        if io.avail_out() == 0 {
            return need_more;
        }
    }
    return block_done;
}

fn deflate_huff(
    s: &mut crate::src::deflate::deflate_state,
    io: &mut DeflateFastIo<'_>,
    flush: ::core::ffi::c_int,
) -> block_state {
    let mut bflush: ::core::ffi::c_int = 0;
    loop {
        if s.lookahead == 0 as crate::stdlib::uInt {
            fill_window_from_fast_io(s, io);
            if s.lookahead == 0 as crate::stdlib::uInt {
                if flush == crate::zlib_h::Z_NO_FLUSH {
                    return need_more;
                }
                break;
            }
        }
        s.match_length = 0;
        let Some(cc) = s.window_byte(s.strstart) else {
            return need_more;
        };
        let cc = cc as crate::zutil_h::uch;
        let _ = s.put_symbol(0, cc as crate::stdlib::uInt);
        s.dyn_ltree[cc as usize].fc = s.dyn_ltree[cc as usize].fc.wrapping_add(1);
        bflush = (s.sym_next == s.sym_end) as ::core::ffi::c_int;
        s.lookahead = s.lookahead.wrapping_sub(1);
        s.strstart = s.strstart.wrapping_add(1);
        if bflush != 0 {
            let block_len =
                (s.strstart as ::core::ffi::c_long - s.block_start) as crate::zutil_h::ulg;
            let block = if s.block_start >= 0 {
                s.block_data(s.block_start as crate::stdlib::uInt, block_len)
            } else {
                None
            };
            crate::src::trees::tr_flush_block(
                s,
                Some(&mut io.data_type),
                block.as_deref(),
                block_len,
                0,
            );
            s.block_start = s.strstart as ::core::ffi::c_long;
            io.flush_pending(s);
            if io.avail_out() == 0 {
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
        crate::src::trees::tr_flush_block(
            s,
            Some(&mut io.data_type),
            block.as_deref(),
            block_len,
            1,
        );
        s.block_start = s.strstart as ::core::ffi::c_long;
        io.flush_pending(s);
        if io.avail_out() == 0 {
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
        crate::src::trees::tr_flush_block(
            s,
            Some(&mut io.data_type),
            block.as_deref(),
            block_len,
            0,
        );
        s.block_start = s.strstart as ::core::ffi::c_long;
        io.flush_pending(s);
        if io.avail_out() == 0 {
            return need_more;
        }
    }
    return block_done;
}
