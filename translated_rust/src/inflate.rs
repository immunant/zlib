// =============== BEGIN inflate_h ================
pub type inflate_mode = ::core::ffi::c_uint;

pub const HEAD: crate::src::inflate::inflate_mode = 16180;

pub const FLAGS: crate::src::inflate::inflate_mode = 16181;

pub const TIME: crate::src::inflate::inflate_mode = 16182;

pub const OS: crate::src::inflate::inflate_mode = 16183;

pub const EXLEN: crate::src::inflate::inflate_mode = 16184;

pub const EXTRA: crate::src::inflate::inflate_mode = 16185;

pub const NAME: crate::src::inflate::inflate_mode = 16186;

pub const COMMENT: crate::src::inflate::inflate_mode = 16187;

pub const HCRC: crate::src::inflate::inflate_mode = 16188;

pub const DICTID: crate::src::inflate::inflate_mode = 16189;

pub const DICT: crate::src::inflate::inflate_mode = 16190;

pub const TYPE: crate::src::inflate::inflate_mode = 16191;

pub const TYPEDO: crate::src::inflate::inflate_mode = 16192;

pub const STORED: crate::src::inflate::inflate_mode = 16193;

pub const COPY_: crate::src::inflate::inflate_mode = 16194;

pub const COPY_1: crate::src::inflate::inflate_mode = 16195;

pub const TABLE: crate::src::inflate::inflate_mode = 16196;

pub const LENLENS: crate::src::inflate::inflate_mode = 16197;

pub const CODELENS: crate::src::inflate::inflate_mode = 16198;

pub const LEN_: crate::src::inflate::inflate_mode = 16199;

pub const LEN: crate::src::inflate::inflate_mode = 16200;

pub const LENEXT: crate::src::inflate::inflate_mode = 16201;

pub const DIST: crate::src::inflate::inflate_mode = 16202;

pub const DISTEXT: crate::src::inflate::inflate_mode = 16203;

pub const MATCH: crate::src::inflate::inflate_mode = 16204;

pub const LIT: crate::src::inflate::inflate_mode = 16205;

pub const CHECK: crate::src::inflate::inflate_mode = 16206;

pub const LENGTH: crate::src::inflate::inflate_mode = 16207;

pub const DONE: crate::src::inflate::inflate_mode = 16208;

pub const BAD: crate::src::inflate::inflate_mode = 16209;

pub const MEM: crate::src::inflate::inflate_mode = 16210;

pub const SYNC: crate::src::inflate::inflate_mode = 16211;

// Inflate's diagnostics are fixed literals.  Keep their backing storage in one
// place so internal users can retain the diagnostic without dereferencing the
// ABI message pointer.
const INFLATE_MESSAGES: [&::std::ffi::CStr; 18] = [
    c"incorrect header check",
    c"unknown compression method",
    c"invalid window size",
    c"unknown header flags set",
    c"invalid stored block lengths",
    c"too many length or distance symbols",
    c"incorrect data check",
    c"incorrect length check",
    c"invalid code lengths set",
    c"invalid bit length repeat",
    c"invalid code -- missing end-of-block",
    c"invalid literal/lengths set",
    c"invalid distances set",
    c"invalid block type",
    c"invalid literal/length code",
    c"invalid distance code",
    c"header crc mismatch",
    c"invalid distance too far back",
];

pub fn inflate_error_message(strm: &crate::zlib_h::z_stream) -> Option<&'static ::std::ffi::CStr> {
    INFLATE_MESSAGES
        .iter()
        .copied()
        .find(|message| strm.msg == message.as_ptr().cast_mut())
}

/// Return a non-dereferenceable identity for an owning stream.
///
/// `inflate_state` is opaque at the ABI boundary, so its former raw stream
/// back-pointer only served the malformed-stream association check. Retaining
/// the address identity preserves that check without storing a second raw
/// pointer that implementation code could follow.
fn stream_identity(strm: &crate::zlib_h::z_stream) -> usize {
    ::core::ptr::from_ref(strm).addr()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum InflateTableRef {
    FixedLiteralLength,
    FixedDistance,
    Dynamic(usize),
}

#[repr(C)]
pub struct inflate_state {
    pub strm: usize,
    // Pointer-free allocator origin retained for the future owned-storage
    // switch.  The live allocation still follows the ABI callbacks.
    pub(crate) allocator_provenance: crate::src::zutil::AllocatorProvenance,
    pub mode: crate::src::inflate::inflate_mode,
    pub last: ::core::ffi::c_int,
    pub wrap: ::core::ffi::c_int,
    pub havedict: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub dmax: ::core::ffi::c_uint,
    pub check: ::core::ffi::c_ulong,
    pub total: ::core::ffi::c_ulong,
    pub head: crate::zlib_h::gz_headerp,
    pub wbits: ::core::ffi::c_uint,
    pub wsize: ::core::ffi::c_uint,
    pub whave: ::core::ffi::c_uint,
    pub wnext: ::core::ffi::c_uint,
    // The opaque state exposes a pointer-sized nullable handle, while Rust
    // implementation code keeps the raw allocation access at explicit
    // borrowing boundaries.
    pub window: Option<::core::ptr::NonNull<::core::ffi::c_uchar>>,
    // Only streams for which zlib installed both allocator callbacks may
    // retain their history in Rust-owned storage. Custom and mixed callback
    // pairs continue to use `window`'s callback allocation exactly as before.
    owned_window: Option<InflateOwnedWindow>,
    pub hold: ::core::ffi::c_ulong,
    pub bits: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub extra: ::core::ffi::c_uint,
    pub(crate) lencode: InflateTableRef,
    pub(crate) distcode: InflateTableRef,
    pub lenbits: ::core::ffi::c_uint,
    pub distbits: ::core::ffi::c_uint,
    pub ncode: ::core::ffi::c_uint,
    pub nlen: ::core::ffi::c_uint,
    pub ndist: ::core::ffi::c_uint,
    pub have: ::core::ffi::c_uint,
    pub next: usize,
    pub lens: [::core::ffi::c_ushort; 320],
    pub work: [::core::ffi::c_ushort; 288],
    pub codes: [crate::src::inftrees::code; 1444],
    pub sane: ::core::ffi::c_int,
    pub back: ::core::ffi::c_int,
    pub was: ::core::ffi::c_uint,
}

/// The caller-visible inflate state still retains the legacy allocation
/// handle, but the size of that allocation is pure state metadata.  Keeping
/// it in pointer-free data lets allocation and copy paths agree now, and is
/// the shape an owned window can later take without rediscovering the ABI
/// sizing rule.
struct InflateWindowLayout {
    allocation_items: crate::stdlib::uInt,
    len: usize,
}

/// Safe owner for a default-allocator inflate history window. Custom and
/// mixed allocator streams keep their callback-provided allocation.
#[derive(Clone)]
struct InflateOwnedWindow {
    bytes: Vec<crate::stdlib::Bytef>,
}

/// A validated mutable history-window view.  Both the legacy allocation
/// bridge and the eventual owned window hand the update core this same safe
/// facade, so the update algorithm never needs to know how the bytes are
/// stored.
struct InflateWindow<'a> {
    bytes: &'a mut [crate::stdlib::Bytef],
}

impl InflateWindowLayout {
    /// Derive the one allocation size used by inflate's legacy window.
    ///
    /// `wbits` is normally established by `inflateReset2`, but state copies
    /// and the legacy allocation handles still need a single checked authority
    /// before they turn that metadata into an allocation or a borrowed span.
    /// Keeping the byte length beside the allocator's `uInt` item count is the
    /// bridge an owned window can reuse without reconstructing this shift.
    fn from_state(state: &inflate_state) -> Option<Self> {
        let allocation_items = (1 as crate::stdlib::uInt).checked_shl(state.wbits)?;
        let len = usize::try_from(allocation_items).ok()?;
        Some(Self {
            allocation_items,
            len,
        })
    }

    fn matches_state_window(&self, state: &inflate_state) -> bool {
        state.wsize == 0 || usize::try_from(state.wsize).ok() == Some(self.len)
    }

    /// Allocate an owned default-allocator window with the same fallible,
    /// checked size used by the legacy callback path.
    fn try_owned(&self) -> Option<InflateOwnedWindow> {
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(self.len).ok()?;
        bytes.resize(self.len, 0);
        Some(InflateOwnedWindow { bytes })
    }
}

impl InflateOwnedWindow {
    /// Confirm that an owned allocation still matches the scalar state before
    /// handing it to the resumable window-update core.
    fn matches_state(&self, state: &inflate_state) -> bool {
        let Some(layout) = InflateWindowLayout::from_state(state) else {
            return false;
        };
        layout.matches_state_window(state)
            && state.whave <= layout.allocation_items
            && state.wnext <= layout.allocation_items
            && self.bytes.len() == layout.len
    }

    /// Hand the owned bytes to the resumable window-update core.
    fn window(&mut self, state: &inflate_state) -> Option<InflateWindow<'_>> {
        self.matches_state(state).then_some(InflateWindow {
            bytes: &mut self.bytes,
        })
    }

    /// Attach this owned history allocation to an already-initialized state.
    /// The checked scalar/window agreement is established before exposing the
    /// legacy nullable handle, and the owning vector remains in place for the
    /// rest of the stream lifetime.
    fn bind_state_window(&mut self, state: &mut inflate_state) -> bool {
        if !self.matches_state(state) {
            return false;
        }
        state.window = ::core::ptr::NonNull::new(self.bytes.as_mut_ptr());
        state.window.is_some()
    }

    /// Deep-copy exactly the initialized history retained by `inflateCopy`.
    fn try_copy_for_state(&self, state: &inflate_state) -> Option<Self> {
        if !self.matches_state(state) {
            return None;
        }
        let layout = InflateWindowLayout::from_state(state)?;
        let history_len = usize::try_from(state.whave).ok()?;
        if history_len > layout.len {
            return None;
        }

        let mut duplicate = layout.try_owned()?;
        duplicate
            .bytes
            .get_mut(..history_len)?
            .copy_from_slice(self.bytes.get(..history_len)?);
        Some(duplicate)
    }
}

impl<'a> InflateWindow<'a> {
    fn borrowed(bytes: &'a mut [crate::stdlib::Bytef]) -> Self {
        Self { bytes }
    }

    fn update(
        &mut self,
        state: &mut crate::src::inflate::inflate_state,
        input: &[crate::stdlib::Bytef],
    ) -> Result<(), ()> {
        update_window(state, self.bytes, input)
    }
}

fn inflate_window_layout_valid(state: &inflate_state) -> bool {
    let Some(layout) = InflateWindowLayout::from_state(state) else {
        return false;
    };
    layout.matches_state_window(state)
        && state.whave <= layout.allocation_items
        && state.wnext <= layout.allocation_items
}

/// Construct the initialized state value installed into an allocator-provided
/// state slot.  This replaces byte-wise zeroing of a Rust value (whose table
/// selector is an enum) with ordinary Rust initialization before reset fills
/// in the stream-specific fields.
pub(crate) fn empty_inflate_state() -> inflate_state {
    inflate_state {
        strm: 0,
        allocator_provenance: crate::src::zutil::UNKNOWN_ALLOCATOR_PROVENANCE,
        mode: crate::src::inflate::HEAD,
        last: 0,
        wrap: 0,
        havedict: 0,
        flags: 0,
        dmax: 0,
        check: 0,
        total: 0,
        head: ::core::ptr::null_mut(),
        wbits: 0,
        wsize: 0,
        whave: 0,
        wnext: 0,
        window: None,
        owned_window: None,
        hold: 0,
        bits: 0,
        length: 0,
        offset: 0,
        extra: 0,
        lencode: InflateTableRef::Dynamic(0),
        distcode: InflateTableRef::Dynamic(0),
        lenbits: 0,
        distbits: 0,
        ncode: 0,
        nlen: 0,
        ndist: 0,
        have: 0,
        next: 0,
        lens: [0; 320],
        work: [0; 288],
        codes: ::core::array::from_fn(|_| crate::src::inftrees::code {
            op: 0,
            bits: 0,
            val: 0,
        }),
        sane: 0,
        back: 0,
        was: 0,
    }
}

fn copy_inflate_state(source: &inflate_state) -> inflate_state {
    inflate_state {
        strm: source.strm,
        allocator_provenance: source.allocator_provenance.clone(),
        mode: source.mode,
        last: source.last,
        wrap: source.wrap,
        havedict: source.havedict,
        flags: source.flags,
        dmax: source.dmax,
        check: source.check,
        total: source.total,
        head: source.head,
        wbits: source.wbits,
        wsize: source.wsize,
        whave: source.whave,
        wnext: source.wnext,
        // `initialize_inflate_copy` installs either an independent
        // callback-owned allocation or an independent owned-window clone.
        // Never temporarily retain the source's window handle here.
        window: None,
        owned_window: None,
        hold: source.hold,
        bits: source.bits,
        length: source.length,
        offset: source.offset,
        extra: source.extra,
        lencode: source.lencode.clone(),
        distcode: source.distcode.clone(),
        lenbits: source.lenbits,
        distbits: source.distbits,
        ncode: source.ncode,
        nlen: source.nlen,
        ndist: source.ndist,
        have: source.have,
        next: source.next,
        lens: source.lens,
        work: source.work,
        codes: ::core::array::from_fn(|index| {
            crate::src::inftrees::copy_code(&source.codes[index])
        }),
        sane: source.sane,
        back: source.back,
        was: source.was,
    }
}
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::adler32::adler32;
pub use crate::src::crc32::crc32;
pub use crate::src::deflate::internal_state;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::inflate_fixed;
pub use crate::src::inftrees::inflate_table;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::ENOUGH;
pub use crate::src::inftrees::ENOUGH_DISTS;
pub use crate::src::inftrees::ENOUGH_LENS;
pub use crate::src::inftrees::LENS;
pub use crate::src::zutil::zcalloc;
pub use crate::src::zutil::zcfree;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
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
pub use crate::zlib_h::Z_DEFLATED;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub use crate::zlib_h::Z_TREES;
pub use crate::zlib_h::Z_VERSION_ERROR;
pub use crate::zutil_h::DEF_WBITS;

fn inflate_stream_has_allocators(strm: &crate::zlib_h::z_stream) -> bool {
    // Check the allocator pair before following `state`.  This preserves the
    // C short-circuit for malformed streams with a stale state handle.
    strm.zalloc.is_some() && strm.zfree.is_some()
}
fn inflate_state_valid(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::inflate::inflate_state,
) -> bool {
    inflate_stream_has_allocators(strm)
        && state.strm == stream_identity(strm)
        && state.mode >= crate::src::inflate::HEAD
        && state.mode <= crate::src::inflate::SYNC
}

pub fn inflateResetKeep(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.total = 0 as ::core::ffi::c_ulong;
    strm.total_out = state.total as crate::stdlib::uLong;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = 0 as ::core::ffi::c_int;
    if state.wrap != 0 {
        strm.adler = (state.wrap & 1 as ::core::ffi::c_int) as crate::stdlib::uLong;
    }
    state.mode = crate::src::inflate::HEAD;
    state.last = 0 as ::core::ffi::c_int;
    state.havedict = 0 as ::core::ffi::c_int;
    state.flags = -1 as ::core::ffi::c_int;
    state.dmax = 32768 as ::core::ffi::c_uint;
    state.head = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
    state.hold = 0 as ::core::ffi::c_ulong;
    state.bits = 0 as ::core::ffi::c_uint;
    state.next = 0;
    state.distcode = InflateTableRef::Dynamic(state.next);
    state.lencode = state.distcode.clone();
    state.sane = 1 as ::core::ffi::c_int;
    state.back = -1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateResetKeep(strm, state)
}
pub fn inflateReset(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.wsize = 0 as ::core::ffi::c_uint;
    state.whave = 0 as ::core::ffi::c_uint;
    state.wnext = 0 as ::core::ffi::c_uint;
    return inflateResetKeep(strm, state);
}

/// Reset a stream whose opaque state is still represented by the ABI handle.
///
/// This is the sole internal bridge for users that own a validated stream but
/// not its typed inflate state.  Keeping the conversion here leaves gzip
/// setup in terms of the same named reset implementation as other callers.
pub(crate) fn inflate_reset_stream(
    strm: &mut crate::zlib_h::z_stream,
) -> ::core::ffi::c_int {
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) =
        (unsafe { (strm.state as *mut crate::src::inflate::inflate_state).as_mut() })
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateReset(strm, state)
}

#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateReset(strm, state)
}
/// The safe reset decision, including a callback-owned window that the ABI
/// boundary must release before the reset mutates the stream state.
struct InflateReset2Plan {
    wrap: ::core::ffi::c_int,
    window_bits: ::core::ffi::c_int,
    release_window: bool,
    release_callback_window: bool,
}

/// Validate and describe an `inflateReset2` operation without invoking the
/// caller's allocator callback.  The FFI wrapper performs that one ABI action
/// before handing this plan back to the safe reset implementation.
fn prepare_inflate_reset2(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    mut windowBits: ::core::ffi::c_int,
) -> Result<InflateReset2Plan, ::core::ffi::c_int> {
    let mut wrap: ::core::ffi::c_int = 0;
    if !inflate_state_valid(strm, state) {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    if windowBits < 0 as ::core::ffi::c_int {
        if windowBits < -15 as ::core::ffi::c_int {
            return Err(crate::zlib_h::Z_STREAM_ERROR);
        }
        wrap = 0 as ::core::ffi::c_int;
        windowBits = -windowBits;
    } else {
        wrap = (windowBits >> 4 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int;
        if windowBits < 48 as ::core::ffi::c_int {
            windowBits &= 15 as ::core::ffi::c_int;
        }
    }
    if windowBits != 0
        && (windowBits < 8 as ::core::ffi::c_int || windowBits > 15 as ::core::ffi::c_int)
    {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok(InflateReset2Plan {
        wrap,
        window_bits: windowBits,
        release_window: state.window.is_some() && state.wbits != windowBits as ::core::ffi::c_uint,
        release_callback_window: state.window.is_some()
            && state.owned_window.is_none()
            && state.wbits != windowBits as ::core::ffi::c_uint,
    })
}

/// Apply a previously validated reset after its optional callback-owned
/// window has been released at the ABI boundary.
fn apply_inflate_reset2(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    plan: InflateReset2Plan,
) -> ::core::ffi::c_int {
    if plan.release_window {
        state.window = None;
        state.owned_window = None;
    }
    state.wrap = plan.wrap;
    state.wbits = plan.window_bits as crate::stdlib::uInt;
    inflateReset(strm, state)
}
#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let plan = match prepare_inflate_reset2(strm, state, windowBits) {
        Ok(plan) => plan,
        Err(error) => return error,
    };
    if plan.release_callback_window {
        let window = state.window.expect("plan requires an installed window");
        // This is the existing custom-allocator boundary.  It deliberately
        // precedes the safe reset mutation, matching zlib's callback order.
        strm.zfree.expect("checked allocator")(strm.opaque, window.as_ptr().cast());
    }
    apply_inflate_reset2(strm, state, plan)
}

fn initialize_inflate_state_base(
    state: &mut crate::src::inflate::inflate_state,
    strm: &mut crate::zlib_h::z_stream,
    allocator_provenance: crate::src::zutil::AllocatorProvenance,
) {
    state.strm = stream_identity(strm);
    state.allocator_provenance = allocator_provenance;
    state.window = None;
    state.owned_window = None;
    state.mode = crate::src::inflate::HEAD;
}

/// Allocate one uninitialized opaque inflate-state slot and expose it only
/// after storing its first Rust value.
///
/// The arbitrary callback allocation remains the one local unsafe boundary.
/// Initialization and reset policy receive only a typed state reference, so
/// they cannot form an initialized reference before the value exists.
fn with_callback_inflate_state_slot<R>(
    strm: &mut crate::zlib_h::z_stream,
    value: crate::src::inflate::inflate_state,
    initialize: impl FnOnce(
        &mut crate::zlib_h::z_stream,
        &mut crate::src::inflate::inflate_state,
    ) -> R,
) -> Option<R> {
    let allocation = (strm.zalloc?)(
        strm.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut ::core::mem::MaybeUninit<crate::src::inflate::inflate_state>;
    let mut slot = ::core::ptr::NonNull::new(allocation)?;
    // The callback allocation is live and uniquely owned by stream setup.
    // Write the first value before exposing a typed state reference.
    let state = unsafe { slot.as_mut().write(value) };
    Some(initialize(strm, state))
}

/// Allocate, initialize, and install the opaque state through one named
/// implementation boundary.  The stream takes ownership only after its ABI
/// state field has been installed.
fn initialize_allocated_inflate_state(
    strm: &mut crate::zlib_h::z_stream,
    window_bits: ::core::ffi::c_int,
    allocator_provenance: crate::src::zutil::AllocatorProvenance,
) -> ::core::ffi::c_int {
    let ret = with_callback_inflate_state_slot(strm, empty_inflate_state(), |strm, state| {
        strm.state = ::core::ptr::from_mut(state).cast::<crate::src::deflate::internal_state>();
        initialize_inflate_state_base(state, strm, allocator_provenance);
        match prepare_inflate_reset2(strm, state, window_bits) {
            Ok(plan) => {
                debug_assert!(!plan.release_window);
                apply_inflate_reset2(strm, state, plan)
            }
            Err(error) => error,
        }
    })
    .unwrap_or(crate::zlib_h::Z_MEM_ERROR);
    if ret != crate::zlib_h::Z_OK {
        Some(strm.zfree.expect("non-null function pointer"))
            .expect("non-null function pointer")(strm.opaque, strm.state.cast());
        strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    }
    ret
}

pub fn inflateInit2_(
    strm: &mut crate::zlib_h::z_stream,
    mut windowBits: ::core::ffi::c_int,
    version: &::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if *version as ::core::ffi::c_int
        != crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let allocator_provenance = crate::src::zutil::install_default_allocators(strm);
    initialize_allocated_inflate_state(strm, windowBits, allocator_provenance)
}
#[export_name = "inflateInit2_"]

pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(version) = version.as_ref() else {
        return crate::zlib_h::Z_VERSION_ERROR;
    };
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateInit2_(strm, windowBits, version, stream_size)
}
pub enum InflateInitMode {
    Zlib,
    Gzip,
}

pub fn inflateInit_(
    strm: Option<&mut crate::zlib_h::z_stream>,
    version: Option<&::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
    mode: InflateInitMode,
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
    let window_bits = match mode {
        InflateInitMode::Zlib => crate::zutil_h::DEF_WBITS,
        InflateInitMode::Gzip => 15 + 16,
    };
    // The safe inputs above establish the version, stream, and wrapper-mode invariants that
    // the legacy allocator/state bridge expects.
    inflateInit2_(
        strm,
        window_bits,
        version,
        stream_size,
    )
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit_(
        strm.as_mut(),
        version.as_ref(),
        stream_size,
        InflateInitMode::Zlib,
    )
}
fn inflate_prime(
    state: &mut crate::src::inflate::inflate_state,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if bits == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_OK;
    }
    if bits < 0 as ::core::ffi::c_int {
        state.hold = 0 as ::core::ffi::c_ulong;
        state.bits = 0 as ::core::ffi::c_uint;
        return crate::zlib_h::Z_OK;
    }
    if bits > 16 as ::core::ffi::c_int
        || (state.bits as crate::stdlib::uInt).wrapping_add(bits as crate::stdlib::uInt)
            > 32 as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    value = (value as ::core::ffi::c_long
        & ((1 as ::core::ffi::c_long) << bits) - 1 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
    state.hold = state
        .hold
        .wrapping_add((value as ::core::ffi::c_ulong) << state.bits);
    state.bits = state
        .bits
        .wrapping_add(bits as crate::stdlib::uInt as ::core::ffi::c_uint);
    return crate::zlib_h::Z_OK;
}
fn inflate_prime_stream(
    strm: &crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_prime(state, bits, value)
}
#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_prime_stream(strm, state, bits, value)
}
fn update_window(
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [u8],
    input: &[u8],
) -> Result<(), ()> {
    let wsize = state.wsize as usize;
    let mut copy = input.len();
    let mut wnext = state.wnext as usize;
    let mut whave = state.whave as usize;
    if wsize == 0 || window.len() != wsize || wnext > wsize || whave > wsize {
        return Err(());
    }
    if copy >= wsize {
        let start = input.len().checked_sub(wsize).ok_or(())?;
        window.copy_from_slice(input.get(start..).ok_or(())?);
        state.wnext = 0;
        state.whave = state.wsize;
        return Ok(());
    }
    let dist = wsize.checked_sub(wnext).ok_or(())?.min(copy);
    let start = input.len().checked_sub(copy).ok_or(())?;
    let end = start.checked_add(dist).ok_or(())?;
    window
        .get_mut(wnext..wnext.checked_add(dist).ok_or(())?)
        .ok_or(())?
        .copy_from_slice(input.get(start..end).ok_or(())?);
    copy -= dist;
    if copy != 0 {
        let start = input.len().checked_sub(copy).ok_or(())?;
        window
            .get_mut(..copy)
            .ok_or(())?
            .copy_from_slice(input.get(start..).ok_or(())?);
        state.wnext = copy as crate::stdlib::uInt;
        state.whave = state.wsize;
    } else {
        wnext = wnext.checked_add(dist).ok_or(())?;
        if wnext == wsize {
            wnext = 0;
        }
        if whave < wsize {
            whave = whave.checked_add(dist).ok_or(())?.min(wsize);
        }
        state.wnext = wnext as crate::stdlib::uInt;
        state.whave = whave as crate::stdlib::uInt;
    }
    Ok(())
}

fn updatewindow(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    input: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let Some(layout) = InflateWindowLayout::from_state(state) else {
        return 1 as ::core::ffi::c_int;
    };
    if !inflate_window_layout_valid(state) {
        return 1 as ::core::ffi::c_int;
    }
    if state.window.is_none() {
        if crate::src::zutil::allocator_pair_is_fully_default(&state.allocator_provenance) {
            let Some(owned_window) = layout.try_owned() else {
                return 1 as ::core::ffi::c_int;
            };
            install_owned_inflate_window(state, owned_window);
        } else {
            let Some(zalloc) = strm.zalloc else {
                return 1 as ::core::ffi::c_int;
            };
            state.window = ::core::ptr::NonNull::new(
                zalloc(
                    strm.opaque,
                    layout.allocation_items,
                    ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
                ) as *mut ::core::ffi::c_uchar
            );
            if state.window.is_none() {
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    if state.wsize == 0 as ::core::ffi::c_uint {
        state.wsize = layout.allocation_items;
        state.wnext = 0 as ::core::ffi::c_uint;
        state.whave = 0 as ::core::ffi::c_uint;
    }
    if state.wnext > state.wsize || state.whave > state.wsize {
        return 1 as ::core::ffi::c_int;
    }
    let result = if state.owned_window.is_some() {
        update_owned_inflate_window(state, input)
    } else {
        // The callback allocation establishes the window's `wsize` bytes for
        // this initialized stream state. Keep its raw handle behind the named
        // borrowing boundary; callers use only references and slices.
        update_callback_owned_inflate_window(state, layout.len, input)
    };
    result.is_err() as ::core::ffi::c_int
}

/// Bind a fully default-allocator window without exposing its allocation to
/// implementation callers. The vector's buffer is stable while the state
/// retains the owner, and the ABI handle remains available to legacy engines.
fn install_owned_inflate_window(
    state: &mut crate::src::inflate::inflate_state,
    mut owned_window: InflateOwnedWindow,
) {
    debug_assert!(owned_window.matches_state(state));
    state.window = ::core::ptr::NonNull::new(owned_window.bytes.as_mut_ptr());
    debug_assert!(state.window.is_some());
    state.owned_window = Some(owned_window);
}

/// Update a fully default-allocator window through its owner. Temporarily
/// taking the owner avoids aliasing the state while retaining the stable ABI
/// handle that the legacy decoder uses between calls.
fn update_owned_inflate_window(
    state: &mut crate::src::inflate::inflate_state,
    input: &[crate::stdlib::Bytef],
) -> Result<(), ()> {
    let mut owned_window = state.owned_window.take().ok_or(())?;
    let result = owned_window
        .window(state)
        .ok_or(())?
        .update(state, input);
    state.owned_window = Some(owned_window);
    result
}

/// Update exactly the validated callback-owned inflate window.
///
/// Callers must establish the allocation layout first. This keeps allocator-
/// derived slice formation out of the safe update algorithm and gives an
/// eventual owned-storage facade one interchange point.
fn update_callback_owned_inflate_window(
    state: &mut crate::src::inflate::inflate_state,
    len: usize,
    input: &[crate::stdlib::Bytef],
) -> Result<(), ()> {
    let window = state.window.expect("validated inflate window");
    let window = unsafe { ::core::slice::from_raw_parts_mut(window.as_ptr(), len) };
    InflateWindow::borrowed(window).update(state, input)
}

fn copy_literal_block(input: &[crate::stdlib::Bytef], output: &mut [crate::stdlib::Bytef]) {
    // The translated caller has already limited both views to the same
    // non-zero `copy` length.  As with the original `memcpy`, input and
    // output must not overlap.
    output.copy_from_slice(input);
}

/// Return the portion of the caller's output buffer consumed by one inflate
/// invocation.  Keep this arithmetic checked before the legacy engine turns
/// the resulting span into a borrowed slice for history and checksums.
fn produced_output_len(
    initial_available: crate::stdlib::uInt,
    remaining_available: crate::stdlib::uInt,
) -> Option<usize> {
    usize::try_from(initial_available.checked_sub(remaining_available)?).ok()
}

/// A checked immutable view of the input advertised by the current stream
/// call.  The legacy decoder retains raw cursors while it is translated, but
/// byte reads themselves need not dereference those cursors directly.
struct InflateInput<'a> {
    bytes: &'a [crate::stdlib::Bytef],
    start: usize,
}

impl InflateInput<'_> {
    fn byte_at(&self, address: usize) -> Option<crate::stdlib::Bytef> {
        address
            .checked_sub(self.start)
            .and_then(|index| self.bytes.get(index))
            .copied()
    }

    fn slice_at(&self, address: usize, len: usize) -> Option<&[crate::stdlib::Bytef]> {
        let start = address.checked_sub(self.start)?;
        let end = start.checked_add(len)?;
        self.bytes.get(start..end)
    }
}

pub fn inflate(
    strm: &mut crate::zlib_h::z_stream,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        // The exported wrapper and internal callers provide a live stream
        // reference. Keep the translated raw-state implementation below local
        // until stream ownership is converted.
        if !inflate_stream_has_allocators(strm)
            || strm.next_out.is_null()
            || strm.next_in.is_null() && strm.avail_in != 0 as crate::stdlib::uInt
        {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        // Check the allocator pair before following `state`, then retain the
        // validated pointer for the translated engine below.
        let Some(state_ref) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut()
        else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if !inflate_state_valid(strm, state_ref) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        // `inflateGetHeader` installs this caller-owned sink before decoding
        // begins. Validate that one ABI pointer once, then retain the
        // resulting borrow while the decoder updates its independent state.
        // The header's caller-owned byte buffers remain raw boundaries below:
        // this only removes repeated raw borrows of the header structure.
        let mut header = state_ref.head.as_mut();
        let state = state_ref;
        let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut put: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut have: ::core::ffi::c_uint = 0;
        let mut left: ::core::ffi::c_uint = 0;
        let mut hold: ::core::ffi::c_ulong = 0;
        let mut bits: ::core::ffi::c_uint = 0;
        let mut in_0: ::core::ffi::c_uint = 0;
        let mut out: ::core::ffi::c_uint = 0;
        let mut copy: ::core::ffi::c_uint = 0;
        let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut here: crate::src::inftrees::code = crate::src::inftrees::code {
            op: 0,
            bits: 0,
            val: 0,
        };
        let mut last: crate::src::inftrees::code = crate::src::inftrees::code {
            op: 0,
            bits: 0,
            val: 0,
        };
        let mut len: ::core::ffi::c_uint = 0;
        let mut ret: ::core::ffi::c_int = 0;
        let mut hbuf: [::core::ffi::c_uchar; 4] = [0; 4];
        const ORDER: [::core::ffi::c_ushort; 19] = [
            16 as ::core::ffi::c_ushort,
            17 as ::core::ffi::c_ushort,
            18 as ::core::ffi::c_ushort,
            0 as ::core::ffi::c_ushort,
            8 as ::core::ffi::c_ushort,
            7 as ::core::ffi::c_ushort,
            9 as ::core::ffi::c_ushort,
            6 as ::core::ffi::c_ushort,
            10 as ::core::ffi::c_ushort,
            5 as ::core::ffi::c_ushort,
            11 as ::core::ffi::c_ushort,
            4 as ::core::ffi::c_ushort,
            12 as ::core::ffi::c_ushort,
            3 as ::core::ffi::c_ushort,
            13 as ::core::ffi::c_ushort,
            2 as ::core::ffi::c_ushort,
            14 as ::core::ffi::c_ushort,
            1 as ::core::ffi::c_ushort,
            15 as ::core::ffi::c_ushort,
        ];
        if state.mode as ::core::ffi::c_uint
            == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            state.mode = crate::src::inflate::TYPEDO;
        }
        let output_start = strm.next_out;
        // `next_in` and `avail_in` were validated above.  Retain one bounded
        // immutable view for the byte-at-a-time decoder instead of repeatedly
        // dereferencing its moving raw cursor.
        let input = if strm.avail_in == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
        };
        let input = InflateInput {
            bytes: input,
            start: strm.next_in.addr(),
        };
        put = output_start as *mut ::core::ffi::c_uchar;
        left = strm.avail_out as ::core::ffi::c_uint;
        next = strm.next_in as *mut ::core::ffi::c_uchar;
        have = strm.avail_in as ::core::ffi::c_uint;
        hold = state.hold;
        bits = state.bits;
        in_0 = have;
        out = left;
        ret = crate::zlib_h::Z_OK;
        '_inf_leave: loop {
            'c_2425: {
                'c_2327: {
                    'c_2422: {
                        'c_2325: {
                            's_2462: {
                                'c_2322: {
                                    'c_2410: {
                                        'c_2319: {
                                            'c_2398: {
                                                'c_2397: {
                                                    'c_2317: {
                                                        'c_2340: {
                                                            'c_2443: {
                                                                'c_2339: {
                                                                    's_519: {
                                                                        'c_2356: {
                                                                            's_1689: {
                                                                                'c_2355: {
                                                                                    's_425: {
                                                                                        'c_2336: {
                                                                                            's_1582: {
                                                                                                match state.mode as ::core::ffi::c_uint {
                                                                                                16180 => {
                                                                                                    // This wrapper-selection phase only needs the
                                                                                                    // validated state reference.  Keep header output
                                                                                                    // pointers at the legacy boundary, but avoid
                                                                                                    // repeatedly dereferencing the state handle.
                                                                                                    if state.wrap == 0 as ::core::ffi::c_int {
                                                                                                        state.mode = crate::src::inflate::TYPEDO;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if have == 0 as ::core::ffi::c_uint {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            have = have.wrapping_sub(1);
                                                                                                            let c2rust_fresh0 = next;
                                                                                                            next = next.wrapping_add(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (input.byte_at(c2rust_fresh0.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        if state.wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if state.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                state.wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            state.check = crate::src::crc32::crc32(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            state.check = crate::src::crc32::crc32(
                                                                                                                state.check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            state.mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            if let Some(head) = header.as_mut() {
                                                                                                                head.done = -1 as ::core::ffi::c_int;
                                                                                                            }
                                                                                                            if state.wrap & 1 as ::core::ffi::c_int == 0
                                                                                                                || (((hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                                                                                                    .wrapping_rem(31 as ::core::ffi::c_ulong) != 0
                                                                                                            {
                                                                                                                strm.msg = INFLATE_MESSAGES[0].as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                state.mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                strm.msg = INFLATE_MESSAGES[1].as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                state.mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else {
                                                                                                                hold >>= 4 as ::core::ffi::c_int;
                                                                                                                bits = bits
                                                                                                                    .wrapping_sub(
                                                                                                                        4 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                                    );
                                                                                                                len = (hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    .wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                                if state.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                    state.wbits = len;
                                                                                                                }
                                                                                                                if len > 15 as ::core::ffi::c_uint || len > state.wbits {
                                                                                                                    strm.msg = INFLATE_MESSAGES[2].as_ptr()
                                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                    state.mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    state.dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    state.flags = 0 as ::core::ffi::c_int;
                                                                                                                    state.check = crate::src::adler32::adler32(
                                                                                                                        0 as crate::stdlib::uLong,
                                                                                                                        None,
                                                                                                                    ) as ::core::ffi::c_ulong;
                                                                                                                    strm.adler = state.check as crate::stdlib::uLong;
                                                                                                                    state.mode = (if hold & 0x200 as ::core::ffi::c_ulong
                                                                                                                        != 0
                                                                                                                    {
                                                                                                                        crate::src::inflate::DICTID as ::core::ffi::c_int
                                                                                                                    } else {
                                                                                                                        crate::src::inflate::TYPE as ::core::ffi::c_int
                                                                                                                    }) as crate::src::inflate::inflate_mode;
                                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                                    continue '_inf_leave;
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                16181 => {
                                                                                                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh1 = next;
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (input.byte_at(c2rust_fresh1.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    state.flags = hold as ::core::ffi::c_int;
                                                                                                    if state.flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        strm.msg = INFLATE_MESSAGES[1].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if state.flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        strm.msg = INFLATE_MESSAGES[3].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if let Some(head) = header.as_mut() {
                                                                                                            head.text = (hold >> 8 as ::core::ffi::c_int
                                                                                                                & 1 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
                                                                                                        }
                                                                                                        if state.flags & 0x200 as ::core::ffi::c_int != 0
                                                                                                            && state.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                        {
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            state.check = crate::src::crc32::crc32(
                                                                                                                state.check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                        }
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        state.mode = crate::src::inflate::TIME;
                                                                                                        break 's_425;
                                                                                                    }
                                                                                                }
                                                                                                16182 => {
                                                                                                    break 's_425;
                                                                                                }
                                                                                                16183 => {
                                                                                                    break 's_519;
                                                                                                }
                                                                                                16184 => {
                                                                                                    break 'c_2317;
                                                                                                }
                                                                                                16185 => {
                                                                                                    break 'c_2319;
                                                                                                }
                                                                                                16186 => {
                                                                                                    break 'c_2322;
                                                                                                }
                                                                                                16187 => {
                                                                                                    break 'c_2325;
                                                                                                }
                                                                                                16188 => {
                                                                                                    break 'c_2327;
                                                                                                }
                                                                                                16189 => {
                                                                                                    // DICTID only updates the validated inflate
                                                                                                    // state and the stream checksum.  Borrow the
                                                                                                    // state once for this bounded phase instead of
                                                                                                    // repeatedly dereferencing its raw handle.
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh10 = next;
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (input.byte_at(c2rust_fresh10.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    state.check = (hold >> 24 as ::core::ffi::c_int
                                                                                                        & 0xff as ::core::ffi::c_ulong)
                                                                                                        .wrapping_add(
                                                                                                            hold >> 8 as ::core::ffi::c_int
                                                                                                                & 0xff00 as ::core::ffi::c_ulong,
                                                                                                        )
                                                                                                        .wrapping_add(
                                                                                                            (hold & 0xff00 as ::core::ffi::c_ulong)
                                                                                                                << 8 as ::core::ffi::c_int,
                                                                                                        )
                                                                                                        .wrapping_add(
                                                                                                            (hold & 0xff as ::core::ffi::c_ulong)
                                                                                                                << 24 as ::core::ffi::c_int,
                                                                                                        );
                                                                                                    strm.adler = state.check as crate::stdlib::uLong;
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                    state.mode = crate::src::inflate::DICT;
                                                                                                    break 'c_2336;
                                                                                                }
                                                                                                16190 => {
                                                                                                    break 'c_2336;
                                                                                                }
                                                                                                16191 => {
                                                                                                    break 'c_2339;
                                                                                                }
                                                                                                16192 => {
                                                                                                    break 'c_2340;
                                                                                                }
                                                                                                16193 => {
                                                                                                    // Stored-block validation has no aliasing output
                                                                                                    // work, so keep its state updates on the
                                                                                                    // validated reference.
                                                                                                    hold >>= bits & 7 as ::core::ffi::c_uint;
                                                                                                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh12 = next;
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (input.byte_at(c2rust_fresh12.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    if hold & 0xffff as ::core::ffi::c_ulong
                                                                                                        != hold >> 16 as ::core::ffi::c_int
                                                                                                            ^ 0xffff as ::core::ffi::c_ulong
                                                                                                    {
                                                                                                        strm.msg = INFLATE_MESSAGES[4].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        state.length = hold as ::core::ffi::c_uint
                                                                                                            & 0xffff as ::core::ffi::c_uint;
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        state.mode = crate::src::inflate::COPY_;
                                                                                                        if flush == crate::zlib_h::Z_TREES {
                                                                                                            break '_inf_leave;
                                                                                                        } else {
                                                                                                            break 'c_2355;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                16194 => {
                                                                                                    break 'c_2355;
                                                                                                }
                                                                                                16195 => {
                                                                                                    break 'c_2356;
                                                                                                }
                                                                                                16196 => {
                                                                                                    while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh13 = next;
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (input.byte_at(c2rust_fresh13.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    state.nlen = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(257 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    state.ndist = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(1 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    state.ncode = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(4 as ::core::ffi::c_uint);
                                                                                                    hold >>= 4 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            4 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    if state.nlen > 286 as ::core::ffi::c_uint
                                                                                                        || state.ndist > 30 as ::core::ffi::c_uint
                                                                                                    {
                                                                                                                strm.msg = INFLATE_MESSAGES[5]
                                                                                                            .as_ptr() as *const ::core::ffi::c_char
                                                                                                            as *mut ::core::ffi::c_char;
                                                                                                        state.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        state.have = 0 as ::core::ffi::c_uint;
                                                                                                        state.mode = crate::src::inflate::LENLENS;
                                                                                                        break 's_1582;
                                                                                                    }
                                                                                                }
                                                                                                16197 => {
                                                                                                    break 's_1582;
                                                                                                }
                                                                                                16198 => {
                                                                                                    break 's_1689;
                                                                                                }
                                                                                                16199 => {
                                                                                                    break 'c_2397;
                                                                                                }
                                                                                                16200 => {
                                                                                                    break 'c_2398;
                                                                                                }
                                                                                                16201 => {
                                                                                                    break 'c_2410;
                                                                                                }
                                                                                                16202 => {
                                                                                                    break 's_2462;
                                                                                                }
                                                                                                16203 => {
                                                                                                    break 'c_2422;
                                                                                                }
                                                                                                16204 => {
                                                                                                    break 'c_2425;
                                                                                                }
                                                                                                16205 => {
                                                                                                    if left == 0 as ::core::ffi::c_uint {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                    let c2rust_fresh32 = put;
                                                                                                    put = put.wrapping_add(1);
                                                                                                    *c2rust_fresh32 = state.length as ::core::ffi::c_uchar;
                                                                                                    left = left.wrapping_sub(1);
                                                                                                    state.mode = crate::src::inflate::LEN;
                                                                                                    continue '_inf_leave;
                                                                                                }
                                                                                                16206 => {
                                                                                                    if state.wrap != 0 {
                                                                                                        while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if have == 0 as ::core::ffi::c_uint {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            have = have.wrapping_sub(1);
                                                                                                            let c2rust_fresh33 = next;
                                                                                                            next = next.wrapping_add(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (input.byte_at(c2rust_fresh33.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        let Some(produced_len) = produced_output_len(out, left) else {
                                                                                                            return crate::zlib_h::Z_STREAM_ERROR;
                                                                                                        };
                                                                                                        out = produced_len as ::core::ffi::c_uint;
                                                                                                        strm.total_out = strm
                                                                                                            .total_out
                                                                                                            .wrapping_add(out as crate::stdlib::uLong);
                                                                                                        state.total = state
                                                                                                            .total
                                                                                                            .wrapping_add(out as ::core::ffi::c_ulong);
                                                                                                        if state.wrap & 4 as ::core::ffi::c_int != 0 && out != 0
                                                                                                        {
                                                                                                            // `produced_len` was checked against the call's original
                                                                                                            // output availability before forming this one temporary
                                                                                                            // checksum view. It starts at the validated ABI output
                                                                                                            // pointer, rather than deriving a new span from `put`.
                                                                                                            let produced_output = ::core::slice::from_raw_parts(
                                                                                                                output_start,
                                                                                                                produced_len,
                                                                                                            );
                                                                                                            state.check = (if state.flags != 0 {
                                                                                                                crate::src::crc32::crc32(
                                                                                                                    state.check as crate::stdlib::uLong,
                                                                                                                    Some(produced_output),
                                                                                                                )
                                                                                                            } else {
                                                                                                                crate::src::adler32::adler32(
                                                                                                                    state.check as crate::stdlib::uLong,
                                                                                                                    Some(produced_output),
                                                                                                                )
                                                                                                            }) as ::core::ffi::c_ulong;
                                                                                                            strm.adler = state.check as crate::stdlib::uLong;
                                                                                                        }
                                                                                                        out = left;
                                                                                                        if state.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                            && (if state.flags != 0 {
                                                                                                                hold
                                                                                                            } else {
                                                                                                                (hold >> 24 as ::core::ffi::c_int
                                                                                                                    & 0xff as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(
                                                                                                                        hold >> 8 as ::core::ffi::c_int
                                                                                                                            & 0xff00 as ::core::ffi::c_ulong,
                                                                                                                    )
                                                                                                                    .wrapping_add(
                                                                                                                        (hold & 0xff00 as ::core::ffi::c_ulong)
                                                                                                                            << 8 as ::core::ffi::c_int,
                                                                                                                    )
                                                                                                                    .wrapping_add(
                                                                                                                        (hold & 0xff as ::core::ffi::c_ulong)
                                                                                                                            << 24 as ::core::ffi::c_int,
                                                                                                                    )
                                                                                                            }) != state.check
                                                                                                        {
                                                                                                    strm.msg = INFLATE_MESSAGES[6].as_ptr()
                                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                            state.mode = crate::src::inflate::BAD;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                        }
                                                                                                    }
                                                                                                    state.mode = crate::src::inflate::LENGTH;
                                                                                                }
                                                                                                16207 => {}
                                                                                                16208 => {
                                                                                                    break 'c_2443;
                                                                                                }
                                                                                                16209 => {
                                                                                                    ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                16210 => return crate::zlib_h::Z_MEM_ERROR,
                                                                                                16211 | _ => return crate::zlib_h::Z_STREAM_ERROR,
                                                                                            }
                                                                                                if state.wrap != 0 && state.flags != 0 {
                                                                                                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                {
                                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                    have = have.wrapping_sub(1);
                                                                                                    let c2rust_fresh34 = next;
                                                                                                    next = next.wrapping_add(1);
                                                                                                    hold = hold
                                                                                                        .wrapping_add(
                                                                                                            (input.byte_at(c2rust_fresh34.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                }
                                                                                                if state.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                    && hold
                                                                                                        != state.total & 0xffffffff as ::core::ffi::c_ulong
                                                                                                {
                                                                                            strm.msg = INFLATE_MESSAGES[7].as_ptr()
                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                    state.mode = crate::src::inflate::BAD;
                                                                                                    continue '_inf_leave;
                                                                                                } else {
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                }
                                                                                            }
                                                                                                state.mode = crate::src::inflate::DONE;
                                                                                                break 'c_2443;
                                                                                            }
                                                                                            while state.have < state.ncode {
                                                                                            while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                            {
                                                                                                if have == 0 as ::core::ffi::c_uint {
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                have = have.wrapping_sub(1);
                                                                                                let c2rust_fresh14 = next;
                                                                                                next = next.wrapping_add(1);
                                                                                                hold = hold
                                                                                                    .wrapping_add(
                                                                                                        (input.byte_at(c2rust_fresh14.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                                    );
                                                                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                            }
                                                                                            let c2rust_fresh15 = state.have;
                                                                                            state.have = state.have.wrapping_add(1);
                                                                                            state.lens[ORDER[c2rust_fresh15 as usize] as usize] = (hold
                                                                                                as ::core::ffi::c_uint
                                                                                                & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                as ::core::ffi::c_ushort;
                                                                                            hold >>= 3 as ::core::ffi::c_int;
                                                                                            bits = bits
                                                                                                .wrapping_sub(
                                                                                                    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                );
                                                                                        }
                                                                                            while state.have < 19 as ::core::ffi::c_uint {
                                                                                            let c2rust_fresh16 = state.have;
                                                                                            state.have = state.have.wrapping_add(1);
                                                                                            state.lens[ORDER[c2rust_fresh16 as usize] as usize] = 0
                                                                                                as ::core::ffi::c_ushort;
                                                                                        }
                                                                                            state.next = 0;
                                                                                            state.distcode = InflateTableRef::Dynamic(state.next);
                                                                                            state.lencode = state.distcode.clone();
                                                                                            state.lenbits = 7 as ::core::ffi::c_uint;
                                                                                            ret = match crate::src::inftrees::inflate_table(
                                                                                            crate::src::inftrees::CODES,
                                                                                            &(&state.lens)[..19],
                                                                                            &mut state.codes,
                                                                                            &mut state.lenbits,
                                                                                            &mut (&mut state.work)[..19],
                                                                                        ) {
                                                                                            Ok(used) => {
                                                                                                state.next = used;
                                                                                                0
                                                                                            }
                                                                                            Err(error) => error,
                                                                                        };
                                                                                            if ret
                                                                                                != 0
                                                                                            {
                                                                                                strm.msg = INFLATE_MESSAGES[8].as_ptr()
                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                state.mode = crate::src::inflate::BAD;
                                                                                                continue '_inf_leave;
                                                                                            } else {
                                                                                                state.have = 0 as ::core::ffi::c_uint;
                                                                                                state.mode = crate::src::inflate::CODELENS;
                                                                                                break 's_1689;
                                                                                            }
                                                                                        }
                                                                                        if state.havedict == 0 as ::core::ffi::c_int {
                                                                                        strm.next_out = put as *mut crate::stdlib::Bytef;
                                                                                        strm.avail_out = left as crate::stdlib::uInt;
                                                                                        strm.next_in = next as *mut crate::stdlib::Bytef;
                                                                                        strm.avail_in = have as crate::stdlib::uInt;
                                                                                        state.hold = hold;
                                                                                        state.bits = bits;
                                                                                        return crate::zlib_h::Z_NEED_DICT;
                                                                                    }
                                                                                        state.check = crate::src::adler32::adler32(
                                                                                        0 as crate::stdlib::uLong,
                                                                                        None,
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                        strm.adler =
                                                                                        state
                                                                                            .check
                                                                                            as crate::stdlib::uLong;
                                                                                        state.mode =
                                                                                        crate::src::inflate::TYPE;
                                                                                        break 'c_2339;
                                                                                    }
                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                {
                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                        break '_inf_leave;
                                                                                    }
                                                                                    have = have.wrapping_sub(1);
                                                                                    let c2rust_fresh2 = next;
                                                                                    next = next.wrapping_add(1);
                                                                                    hold = hold
                                                                                        .wrapping_add(
                                                                                            (input.byte_at(c2rust_fresh2.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                        );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                    if let Some(head) = header.as_mut() {
                                                                                        head.time = hold
                                                                                            as crate::stdlib::uLong;
                                                                                    }
                                                                                    if state.flags & 0x200 as ::core::ffi::c_int != 0
                                                                                    && state.wrap & 4 as ::core::ffi::c_int != 0
                                                                                {
                                                                                    hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                        >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                    hbuf[2 as ::core::ffi::c_int as usize] = (hold
                                                                                        >> 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                    hbuf[3 as ::core::ffi::c_int as usize] = (hold
                                                                                        >> 24 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                    state.check = crate::src::crc32::crc32(
                                                                                        state.check as crate::stdlib::uLong,
                                                                                        Some(&hbuf[..4]),
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                }
                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                    state.mode = crate::src::inflate::OS;
                                                                                    break 's_519;
                                                                                }
                                                                                state.mode = crate::src::inflate::COPY_1;
                                                                                break 'c_2356;
                                                                            }
                                                                            while state.have
                                                                                < state
                                                                                    .nlen
                                                                                    .wrapping_add(
                                                                                        state
                                                                                            .ndist,
                                                                                    )
                                                                            {
                                                                                loop {
                                                                                    here = crate::src::inffast::decode_table_entry_or_invalid(
                                                                                        &*state,
                                                                                        crate::src::inffast::DecodeTable::LiteralLength,
                                                                                        (hold as ::core::ffi::c_uint
                                                                                            & ((1 as ::core::ffi::c_uint) << state.lenbits)
                                                                                                .wrapping_sub(1 as ::core::ffi::c_uint)) as usize,
                                                                                    );
                                                                                    if here.bits as ::core::ffi::c_uint <= bits {
                                                                                    break;
                                                                                }
                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                    break '_inf_leave;
                                                                                }
                                                                                    have = have
                                                                                    .wrapping_sub(
                                                                                        1,
                                                                                    );
                                                                                    let c2rust_fresh17 =
                                                                                        next;
                                                                                    next = next
                                                                                        .wrapping_add(1);
                                                                                    hold = hold
                                                                                    .wrapping_add(
                                                                                        (input.byte_at(c2rust_fresh17.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                    );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                if (here.val as ::core::ffi::c_int)
                                                                                < 16 as ::core::ffi::c_int
                                                                            {
                                                                                hold >>= here.bits as ::core::ffi::c_int;
                                                                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                let c2rust_fresh18 = state.have;
                                                                                state.have = state.have.wrapping_add(1);
                                                                                state.lens[c2rust_fresh18 as usize] = here.val;
                                                                            } else {
                                                                                if here.val as ::core::ffi::c_int
                                                                                    == 16 as ::core::ffi::c_int
                                                                                {
                                                                                    while bits
                                                                                        < (here.bits as ::core::ffi::c_int
                                                                                            + 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                                                                    {
                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                            break '_inf_leave;
                                                                                        }
                                                                                        have = have.wrapping_sub(1);
                                                                                        let c2rust_fresh19 = next;
                                                                                        next = next.wrapping_add(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (input.byte_at(c2rust_fresh19.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    if state.have == 0 as ::core::ffi::c_uint {
                                                                                        strm.msg = INFLATE_MESSAGES[9].as_ptr()
                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                        state.mode = crate::src::inflate::BAD;
                                                                                        break;
                                                                                    } else {
                                                                                        len = state
                                                                                            .lens[state.have.wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                            as usize] as ::core::ffi::c_uint;
                                                                                        copy = (3 as ::core::ffi::c_uint)
                                                                                            .wrapping_add(
                                                                                                hold as ::core::ffi::c_uint
                                                                                                    & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint),
                                                                                            );
                                                                                        hold >>= 2 as ::core::ffi::c_int;
                                                                                        bits = bits
                                                                                            .wrapping_sub(
                                                                                                2 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                            );
                                                                                    }
                                                                                } else if here.val as ::core::ffi::c_int
                                                                                    == 17 as ::core::ffi::c_int
                                                                                {
                                                                                    while bits
                                                                                        < (here.bits as ::core::ffi::c_int
                                                                                            + 3 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                                                                    {
                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                            break '_inf_leave;
                                                                                        }
                                                                                        have = have.wrapping_sub(1);
                                                                                        let c2rust_fresh20 = next;
                                                                                        next = next.wrapping_add(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (input.byte_at(c2rust_fresh20.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    len = 0 as ::core::ffi::c_uint;
                                                                                    copy = (3 as ::core::ffi::c_uint)
                                                                                        .wrapping_add(
                                                                                            hold as ::core::ffi::c_uint
                                                                                                & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                                                                        );
                                                                                    hold >>= 3 as ::core::ffi::c_int;
                                                                                    bits = bits
                                                                                        .wrapping_sub(
                                                                                            3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                        );
                                                                                } else {
                                                                                    while bits
                                                                                        < (here.bits as ::core::ffi::c_int
                                                                                            + 7 as ::core::ffi::c_int) as ::core::ffi::c_uint
                                                                                    {
                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                            break '_inf_leave;
                                                                                        }
                                                                                        have = have.wrapping_sub(1);
                                                                                        let c2rust_fresh21 = next;
                                                                                        next = next.wrapping_add(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (input.byte_at(c2rust_fresh21.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    len = 0 as ::core::ffi::c_uint;
                                                                                    copy = (11 as ::core::ffi::c_uint)
                                                                                        .wrapping_add(
                                                                                            hold as ::core::ffi::c_uint
                                                                                                & ((1 as ::core::ffi::c_uint) << 7 as ::core::ffi::c_int)
                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                                                                        );
                                                                                    hold >>= 7 as ::core::ffi::c_int;
                                                                                    bits = bits
                                                                                        .wrapping_sub(
                                                                                            7 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                        );
                                                                                }
                                                                                if state.have.wrapping_add(copy)
                                                                                    > state.nlen.wrapping_add(state.ndist)
                                                                                {
                                                                            strm.msg = INFLATE_MESSAGES[9].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state.mode = crate::src::inflate::BAD;
                                                                                    break;
                                                                                } else {
                                                                                    loop {
                                                                                        let c2rust_fresh22 = copy;
                                                                                        copy = copy.wrapping_sub(1);
                                                                                        if c2rust_fresh22 == 0 {
                                                                                            break;
                                                                                        }
                                                                                        let c2rust_fresh23 = state.have;
                                                                                        state.have = state.have.wrapping_add(1);
                                                                                        state.lens[c2rust_fresh23 as usize] = len
                                                                                            as ::core::ffi::c_ushort;
                                                                                    }
                                                                                }
                                                                            }
                                                                            }
                                                                            if state.mode as ::core::ffi::c_uint
                                                                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            continue '_inf_leave;
                                                                        }
                                                                            if state.lens[256 as ::core::ffi::c_int as usize]
                                                                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                                        {
                                                                            strm.msg = INFLATE_MESSAGES[10]
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            state.mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            state.next = 0;
                                                                            state.lencode = InflateTableRef::Dynamic(state.next);
                                                                            state.lenbits = 9 as ::core::ffi::c_uint;
                                                                            ret = match crate::src::inftrees::inflate_table(
                                                                                crate::src::inftrees::LENS,
                                                                                &(&state.lens)[..state.nlen as usize],
                                                                                &mut state.codes,
                                                                                &mut state.lenbits,
                                                                                &mut (&mut state.work)[..state.nlen as usize],
                                                                            ) {
                                                                                Ok(used) => {
                                                                                    state.next = used;
                                                                                    0
                                                                                }
                                                                                Err(error) => error,
                                                                            };
                                                                            if ret != 0 {
                                                                                strm.msg = INFLATE_MESSAGES[11].as_ptr()
                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                state.mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                state.distcode = InflateTableRef::Dynamic(state.next);
                                                                                state.distbits = 6 as ::core::ffi::c_uint;
                                                                                let table_used = state.next;
                                                                                ret = match crate::src::inftrees::inflate_table(
                                                                                    crate::src::inftrees::DISTS,
                                                                                    &(&state.lens)[state.nlen as usize..(state.nlen + state.ndist) as usize],
                                                                                    &mut (&mut state.codes)[table_used..],
                                                                                    &mut state.distbits,
                                                                                    &mut (&mut state.work)[..state.ndist as usize],
                                                                                ) {
                                                                                    Ok(used) => {
                                                                                        state.next = table_used + used;
                                                                                        0
                                                                                    }
                                                                                    Err(error) => error,
                                                                                };
                                                                                if ret != 0 {
                                                                                    strm.msg = INFLATE_MESSAGES[12].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state.mode = crate::src::inflate::BAD;
                                                                                    continue '_inf_leave;
                                                                                } else {
                                                                                    state.mode = crate::src::inflate::LEN_;
                                                                                    if flush == crate::zlib_h::Z_TREES {
                                                                                        break '_inf_leave;
                                                                                    } else {
                                                                                        break 'c_2397;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        }
                                                                        // Stored-block copying only updates the
                                                                        // already-validated inflate state around
                                                                        // its existing raw input/output boundary.
                                                                        // Keep one local state borrow for this
                                                                        // bounded phase instead of repeatedly
                                                                        // dereferencing the legacy state handle.
                                                                        copy = state.length;
                                                                        if copy != 0 {
                                                                            if copy > have {
                                                                                copy = have;
                                                                            }
                                                                            if copy > left {
                                                                                copy = left;
                                                                            }
                                                                            if copy == 0
                                                                            as ::core::ffi::c_uint
                                                                        {
                                                                            break '_inf_leave;
                                                                        }
                                                                            let Some(stored_input) = input.slice_at(
                                                                                next.addr(),
                                                                                copy as usize,
                                                                            ) else {
                                                                                return crate::zlib_h::Z_STREAM_ERROR;
                                                                            };
                                                                            let output = ::core::slice::from_raw_parts_mut(
                                                                                put,
                                                                                copy as usize,
                                                                            );
                                                                            copy_literal_block(
                                                                                stored_input, output,
                                                                            );
                                                                            have = have
                                                                                .wrapping_sub(copy);
                                                                            next = next
                                                                                .wrapping_add(
                                                                                    copy as usize,
                                                                                );
                                                                            left = left
                                                                                .wrapping_sub(copy);
                                                                            put = put.wrapping_add(
                                                                                copy as usize,
                                                                            );
                                                                            state.length = state
                                                                                .length
                                                                                .wrapping_sub(copy);
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            state.mode = crate::src::inflate::TYPE;
                                                                            continue '_inf_leave;
                                                                        }
                                                                    }
                                                                    while bits
                                                                        < 16 as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        if have == 0
                                                                            as ::core::ffi::c_uint
                                                                        {
                                                                            break '_inf_leave;
                                                                        }
                                                                        have = have.wrapping_sub(1);
                                                                        let c2rust_fresh3 = next;
                                                                        next = next.wrapping_add(1);
                                                                        hold = hold.wrapping_add(
                                                                        (input.byte_at(c2rust_fresh3.addr()).unwrap_or(0)
                                                                            as ::core::ffi::c_ulong)
                                                                            << bits,
                                                                    );
                                                                        bits = bits.wrapping_add(
                                                                        8 as ::core::ffi::c_uint,
                                                                    );
                                                                    }
                                                                    if let Some(head) = header.as_mut() {
                                                                        head.xflags = (hold
                                                                        & 0xff
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int;
                                                                        head.os = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_int;
                                                                    }
                                                                    if state.flags
                                                                    & 0x200 as ::core::ffi::c_int
                                                                    != 0
                                                                    && state.wrap
                                                                        & 4 as ::core::ffi::c_int
                                                                        != 0
                                                                {
                                                                    hbuf[0 as ::core::ffi::c_int
                                                                        as usize] = hold
                                                                        as ::core::ffi::c_uchar;
                                                                    hbuf[1 as ::core::ffi::c_int
                                                                        as usize] = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_uchar;
                                                                    state.check = crate::src::crc32::crc32(
                                                                        state.check as crate::stdlib::uLong,
                                                                        Some(&hbuf[..2]),
                                                                    ) as ::core::ffi::c_ulong;
                                                                }
                                                                    hold =
                                                                        0 as ::core::ffi::c_ulong;
                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                    state.mode =
                                                                        crate::src::inflate::EXLEN;
                                                                    break 'c_2317;
                                                                }
                                                                if flush == crate::zlib_h::Z_BLOCK
                                                                    || flush
                                                                        == crate::zlib_h::Z_TREES
                                                                {
                                                                    break '_inf_leave;
                                                                } else {
                                                                    break 'c_2340;
                                                                }
                                                            }
                                                            ret = crate::zlib_h::Z_STREAM_END;
                                                            break '_inf_leave;
                                                        }
                                                        if state.last != 0 {
                                                            hold >>=
                                                                bits & 7 as ::core::ffi::c_uint;
                                                            bits = bits.wrapping_sub(
                                                                bits & 7 as ::core::ffi::c_uint,
                                                            );
                                                            state.mode =
                                                                crate::src::inflate::CHECK;
                                                            continue '_inf_leave;
                                                        } else {
                                                            while bits
                                                                < 3 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_uint
                                                            {
                                                                if have == 0 as ::core::ffi::c_uint
                                                                {
                                                                    break '_inf_leave;
                                                                }
                                                                have = have.wrapping_sub(1);
                                                                let c2rust_fresh11 = next;
                                                                next = next.wrapping_add(1);
                                                                hold = hold.wrapping_add(
                                                                    (input.byte_at(c2rust_fresh11.addr()).unwrap_or(0)
                                                                        as ::core::ffi::c_ulong)
                                                                        << bits,
                                                                );
                                                                bits = bits.wrapping_add(
                                                                    8 as ::core::ffi::c_uint,
                                                                );
                                                            }
                                                            state.last = (hold
                                                                as ::core::ffi::c_uint
                                                                & ((1 as ::core::ffi::c_uint)
                                                                    << 1 as ::core::ffi::c_int)
                                                                    .wrapping_sub(
                                                                        1 as ::core::ffi::c_uint,
                                                                    ))
                                                                as ::core::ffi::c_int;
                                                            hold >>= 1 as ::core::ffi::c_int;
                                                            bits = bits.wrapping_sub(
                                                                1 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_uint,
                                                            );
                                                            match hold as ::core::ffi::c_uint
                                                                & ((1 as ::core::ffi::c_uint)
                                                                    << 2 as ::core::ffi::c_int)
                                                                    .wrapping_sub(
                                                                        1 as ::core::ffi::c_uint,
                                                                    ) {
                                                                0 => {
                                                                    state.mode =
                                                                        crate::src::inflate::STORED;
                                                                }
                                                                1 => {
                                                                    crate::src::inftrees::inflate_fixed(state);
                                                                    state.mode =
                                                                        crate::src::inflate::LEN_;
                                                                    if flush
                                                                        == crate::zlib_h::Z_TREES
                                                                    {
                                                                        hold >>=
                                                                            2 as ::core::ffi::c_int;
                                                                        bits = bits.wrapping_sub(
                                                                        2 as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint,
                                                                    );
                                                                        break '_inf_leave;
                                                                    }
                                                                }
                                                                2 => {
                                                                    state.mode =
                                                                        crate::src::inflate::TABLE;
                                                                }
                                                                _ => {
                                                                    strm.msg = INFLATE_MESSAGES[13].as_ptr()
                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                    state.mode =
                                                                        crate::src::inflate::BAD;
                                                                }
                                                            }
                                                            hold >>= 2 as ::core::ffi::c_int;
                                                            bits = bits.wrapping_sub(
                                                                2 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_uint,
                                                            );
                                                            continue '_inf_leave;
                                                        }
                                                    }
                                                    if state.flags & 0x400 as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        while bits
                                                            < 16 as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                        {
                                                            if have == 0 as ::core::ffi::c_uint {
                                                                break '_inf_leave;
                                                            }
                                                            have = have.wrapping_sub(1);
                                                            let c2rust_fresh4 = next;
                                                            next = next.wrapping_add(1);
                                                            hold = hold.wrapping_add(
                                                                (input.byte_at(c2rust_fresh4.addr()).unwrap_or(0)
                                                                    as ::core::ffi::c_ulong)
                                                                    << bits,
                                                            );
                                                            bits = bits.wrapping_add(
                                                                8 as ::core::ffi::c_uint,
                                                            );
                                                        }
                                                        state.length =
                                                            hold as ::core::ffi::c_uint;
                                                        if let Some(head) = header.as_mut() {
                                                            head.extra_len = hold
                                                                as ::core::ffi::c_uint
                                                                as crate::stdlib::uInt;
                                                        }
                                                        if state.flags
                                                            & 0x200 as ::core::ffi::c_int
                                                            != 0
                                                            && state.wrap
                                                                & 4 as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            hbuf[0 as ::core::ffi::c_int
                                                                as usize] =
                                                                hold as ::core::ffi::c_uchar;
                                                            hbuf[1 as ::core::ffi::c_int
                                                                as usize] = (hold
                                                                >> 8 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uchar;
                                                            state.check =
                                                                crate::src::crc32::crc32(
                                                                    state.check
                                                                        as crate::stdlib::uLong,
                                                                    Some(&hbuf[..2]),
                                                                )
                                                                    as ::core::ffi::c_ulong;
                                                        }
                                                        hold = 0 as ::core::ffi::c_ulong;
                                                        bits = 0 as ::core::ffi::c_uint;
                                                    } else if let Some(head) = header.as_mut() {
                                                        head.extra =
                                                            ::core::ptr::null_mut::<
                                                                crate::stdlib::Bytef,
                                                            >(
                                                            );
                                                    }
                                                    state.mode = crate::src::inflate::EXTRA;
                                                    break 'c_2319;
                                                }
                                                state.mode = crate::src::inflate::LEN;
                                            }
                                            if have >= 6 as ::core::ffi::c_uint
                                                && left >= 258 as ::core::ffi::c_uint
                                            {
                                                strm.next_out = put as *mut crate::stdlib::Bytef;
                                                strm.avail_out = left as crate::stdlib::uInt;
                                                strm.next_in = next as *mut crate::stdlib::Bytef;
                                                strm.avail_in = have as crate::stdlib::uInt;
                                                state.hold = hold;
                                                state.bits = bits;
                                                let history = if state.window.is_none()
                                                    || state.wsize == 0
                                                {
                                                    None
                                                } else {
                                                    Some(::core::slice::from_raw_parts(
                                                        state.window
                                                            .expect("checked non-null window")
                                                            .as_ptr(),
                                                        state.wsize as usize,
                                                    ))
                                                };
                                                // `input` remains the one checked span for this
                                                // inflate call. `next` and `have` select its live
                                                // suffix, while `out` is the original caller output
                                                // capacity and therefore bounds the fast loop's
                                                // temporary full-output view.
                                                let Some(fast_input) = input.slice_at(
                                                    next.addr(),
                                                    have as usize,
                                                ) else {
                                                    state.mode = crate::src::inflate::BAD;
                                                    break 'c_2322;
                                                };
                                                let fast_output = ::core::slice::from_raw_parts_mut(
                                                    output_start,
                                                    out as usize,
                                                );
                                                crate::src::inffast::inflate_fast(
                                                    strm,
                                                    state,
                                                    out,
                                                    history,
                                                    false,
                                                    fast_input,
                                                    fast_output,
                                                );
                                                put = strm.next_out as *mut ::core::ffi::c_uchar;
                                                left = strm.avail_out as ::core::ffi::c_uint;
                                                next = strm.next_in as *mut ::core::ffi::c_uchar;
                                                have = strm.avail_in as ::core::ffi::c_uint;
                                                hold = state.hold;
                                                bits = state.bits;
                                                if state.mode as ::core::ffi::c_uint
                                                    == crate::src::inflate::TYPE
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    state.back = -1 as ::core::ffi::c_int;
                                                }
                                                continue '_inf_leave;
                                            } else {
                                                state.back = 0 as ::core::ffi::c_int;
                                                loop {
                                                    here = crate::src::inffast::decode_table_entry_or_invalid(
                                                        &*state,
                                                        crate::src::inffast::DecodeTable::LiteralLength,
                                                        (hold as ::core::ffi::c_uint
                                                            & ((1 as ::core::ffi::c_uint)
                                                                << state.lenbits)
                                                                .wrapping_sub(
                                                                    1 as ::core::ffi::c_uint,
                                                                ))
                                                            as usize,
                                                    );
                                                    if here.bits as ::core::ffi::c_uint <= bits {
                                                        break;
                                                    }
                                                    if have == 0 as ::core::ffi::c_uint {
                                                        break '_inf_leave;
                                                    }
                                                    have = have.wrapping_sub(1);
                                                    let c2rust_fresh24 = next;
                                                    next = next.wrapping_add(1);
                                                    hold = hold.wrapping_add(
                                                        (input.byte_at(c2rust_fresh24.addr()).unwrap_or(0) as ::core::ffi::c_ulong)
                                                            << bits,
                                                    );
                                                    bits =
                                                        bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                }
                                                if here.op as ::core::ffi::c_int != 0
                                                    && here.op as ::core::ffi::c_int
                                                        & 0xf0 as ::core::ffi::c_int
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    last = here;
                                                    loop {
                                                        here = crate::src::inffast::decode_table_entry_or_invalid(
                                                        &*state,
                                                        crate::src::inffast::DecodeTable::LiteralLength,
                                                        (last.val as ::core::ffi::c_uint)
                                                            .wrapping_add(
                                                            (hold as ::core::ffi::c_uint
                                                                & ((1 as ::core::ffi::c_uint)
                                                                    << last.bits
                                                                        as ::core::ffi::c_int
                                                                        + last.op
                                                                            as ::core::ffi::c_int)
                                                                    .wrapping_sub(
                                                                        1 as ::core::ffi::c_uint,
                                                                    ))
                                                                >> last.bits as ::core::ffi::c_int,
                                                        )
                                                            as usize,
                                                        );
                                                        if (last.bits as ::core::ffi::c_int
                                                            + here.bits as ::core::ffi::c_int)
                                                            as ::core::ffi::c_uint
                                                            <= bits
                                                        {
                                                            break;
                                                        }
                                                        if have == 0 as ::core::ffi::c_uint {
                                                            break '_inf_leave;
                                                        }
                                                        have = have.wrapping_sub(1);
                                                        let c2rust_fresh25 = next;
                                                        next = next.wrapping_add(1);
                                                        hold = hold.wrapping_add(
                                                            (input.byte_at(c2rust_fresh25.addr()).unwrap_or(0)
                                                                as ::core::ffi::c_ulong)
                                                                << bits,
                                                        );
                                                        bits = bits
                                                            .wrapping_add(8 as ::core::ffi::c_uint);
                                                    }
                                                    hold >>= last.bits as ::core::ffi::c_int;
                                                    bits = bits.wrapping_sub(
                                                        last.bits as ::core::ffi::c_uint,
                                                    );
                                                    state.back +=
                                                        last.bits as ::core::ffi::c_int;
                                                }
                                                hold >>= here.bits as ::core::ffi::c_int;
                                                bits = bits
                                                    .wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                state.back += here.bits as ::core::ffi::c_int;
                                                state.length = here.val as ::core::ffi::c_uint;
                                                if here.op as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                {
                                                    state.mode = crate::src::inflate::LIT;
                                                    continue '_inf_leave;
                                                } else if here.op as ::core::ffi::c_int
                                                    & 32 as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    state.back = -1 as ::core::ffi::c_int;
                                                    state.mode = crate::src::inflate::TYPE;
                                                    continue '_inf_leave;
                                                } else if here.op as ::core::ffi::c_int
                                                    & 64 as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    strm.msg = INFLATE_MESSAGES[14].as_ptr()
                                                        as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char;
                                                    state.mode = crate::src::inflate::BAD;
                                                    continue '_inf_leave;
                                                } else {
                                                    state.extra = here.op as ::core::ffi::c_uint
                                                        & 15 as ::core::ffi::c_uint;
                                                    state.mode = crate::src::inflate::LENEXT;
                                                    break 'c_2410;
                                                }
                                            }
                                        }
                                        if state.flags & 0x400 as ::core::ffi::c_int != 0 {
                                            copy = state.length;
                                            if copy > have {
                                                copy = have;
                                            }
                                            if copy != 0 {
                                                if let Some(head) = header.as_mut() {
                                                    if !head.extra.is_null()
                                                        && {
                                                            len = (head.extra_len
                                                                as ::core::ffi::c_uint)
                                                                .wrapping_sub(state.length);
                                                            len < head.extra_max
                                                        }
                                                    {
                                                        crate::stdlib::memcpy(
                                                            head.extra.wrapping_add(len as usize)
                                                                as *mut ::core::ffi::c_void,
                                                            next as *const ::core::ffi::c_void,
                                                            (if len.wrapping_add(copy) > head.extra_max {
                                                                (head.extra_max as ::core::ffi::c_uint)
                                                                    .wrapping_sub(len)
                                                            } else {
                                                                copy
                                                            })
                                                                as crate::__stddef_size_t_h::size_t,
                                                        );
                                                    }
                                                }
                                                if state.flags & 0x200 as ::core::ffi::c_int != 0
                                                    && state.wrap & 4 as ::core::ffi::c_int != 0
                                                {
                                                    let Some(extra_input) = input.slice_at(
                                                        next.addr(),
                                                        copy as usize,
                                                    ) else {
                                                        return crate::zlib_h::Z_STREAM_ERROR;
                                                    };
                                                    state.check = crate::src::crc32::crc32(
                                                        state.check as crate::stdlib::uLong,
                                                        Some(extra_input),
                                                    )
                                                        as ::core::ffi::c_ulong;
                                                }
                                                have = have.wrapping_sub(copy);
                                                next = next.wrapping_add(copy as usize);
                                                state.length =
                                                    state.length.wrapping_sub(copy);
                                            }
                                            if state.length != 0 {
                                                break '_inf_leave;
                                            }
                                        }
                                        state.length = 0 as ::core::ffi::c_uint;
                                        state.mode = crate::src::inflate::NAME;
                                        break 'c_2322;
                                    }
                                    if state.extra != 0 {
                                        while bits < state.extra {
                                            if have == 0 as ::core::ffi::c_uint {
                                                break '_inf_leave;
                                            }
                                            have = have.wrapping_sub(1);
                                            let c2rust_fresh26 = next;
                                            next = next.wrapping_add(1);
                                            hold = hold.wrapping_add(
                                                (input.byte_at(c2rust_fresh26.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                            );
                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                        }
                                        state.length = state.length.wrapping_add(
                                            hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint) << state.extra)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                        );
                                        hold >>= state.extra;
                                        bits = bits.wrapping_sub(state.extra);
                                        state.back = (state.back as ::core::ffi::c_uint)
                                            .wrapping_add(state.extra)
                                            as ::core::ffi::c_int;
                                    }
                                    state.was = state.length;
                                    state.mode = crate::src::inflate::DIST;
                                    break 's_2462;
                                }
                                if state.flags & 0x800 as ::core::ffi::c_int != 0 {
                                    if have == 0 as ::core::ffi::c_uint {
                                        break '_inf_leave;
                                    }
                                    // `have` bounds the current input cursor for this
                                    // NAME phase. Borrow it once from the call's checked
                                    // input view so byte reads and the optional header
                                    // checksum use the same span.
                                    // The caller-owned header output remains a raw boundary:
                                    // its advertised capacity is not a Rust slice length.
                                    let Some(name_input) = input.slice_at(next.addr(), have as usize)
                                    else {
                                        return crate::zlib_h::Z_STREAM_ERROR;
                                    };
                                    copy = 0 as ::core::ffi::c_uint;
                                    loop {
                                        let c2rust_fresh5 = copy;
                                        copy = copy.wrapping_add(1);
                                        len = name_input[c2rust_fresh5 as usize]
                                            as ::core::ffi::c_uint;
                                        if let Some(head) = header.as_mut() {
                                            if !head.name.is_null()
                                                && state.length < head.name_max
                                            {
                                                let c2rust_fresh6 = state.length;
                                                state.length = state.length.wrapping_add(1);
                                                *head.name.wrapping_add(c2rust_fresh6 as usize) =
                                                    len as crate::stdlib::Bytef;
                                            }
                                        }
                                        if !(len != 0 && copy < have) {
                                            break;
                                        }
                                    }
                                    if state.flags & 0x200 as ::core::ffi::c_int != 0
                                        && state.wrap & 4 as ::core::ffi::c_int != 0
                                    {
                                        state.check = crate::src::crc32::crc32(
                                            state.check as crate::stdlib::uLong,
                                            Some(&name_input[..copy as usize]),
                                        )
                                            as ::core::ffi::c_ulong;
                                    }
                                    have = have.wrapping_sub(copy);
                                    next = next.wrapping_add(copy as usize);
                                    if len != 0 {
                                        break '_inf_leave;
                                    }
                                } else if let Some(head) = header.as_mut() {
                                    head.name =
                                        ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                                }
                                state.length = 0 as ::core::ffi::c_uint;
                                state.mode = crate::src::inflate::COMMENT;
                                break 'c_2325;
                            }
                            loop {
                                here = crate::src::inffast::decode_table_entry_or_invalid(
                                    &*state,
                                    crate::src::inffast::DecodeTable::Distance,
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint) << state.distbits)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                    as usize,
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                if have == 0 as ::core::ffi::c_uint {
                                    break '_inf_leave;
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh27 = next;
                                next = next.wrapping_add(1);
                                hold = hold.wrapping_add(
                                    (input.byte_at(c2rust_fresh27.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                );
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                last = here;
                                loop {
                                    here = crate::src::inffast::decode_table_entry_or_invalid(
                                        &*state,
                                        crate::src::inffast::DecodeTable::Distance,
                                        (last.val as ::core::ffi::c_uint).wrapping_add(
                                            (hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint)
                                                    << last.bits as ::core::ffi::c_int
                                                        + last.op as ::core::ffi::c_int)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                                >> last.bits as ::core::ffi::c_int,
                                        ) as usize,
                                    );
                                    if (last.bits as ::core::ffi::c_int
                                        + here.bits as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint
                                        <= bits
                                    {
                                        break;
                                    }
                                    if have == 0 as ::core::ffi::c_uint {
                                        break '_inf_leave;
                                    }
                                    have = have.wrapping_sub(1);
                                    let c2rust_fresh28 = next;
                                    next = next.wrapping_add(1);
                                    hold = hold.wrapping_add(
                                        (input.byte_at(c2rust_fresh28.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits,
                                    );
                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                }
                                hold >>= last.bits as ::core::ffi::c_int;
                                bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                                state.back += last.bits as ::core::ffi::c_int;
                            }
                            hold >>= here.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                            state.back += here.bits as ::core::ffi::c_int;
                            if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                                strm.msg = INFLATE_MESSAGES[15].as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                state.mode = crate::src::inflate::BAD;
                                continue '_inf_leave;
                            } else {
                                state.offset = here.val as ::core::ffi::c_uint;
                                state.extra =
                                    here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                                state.mode = crate::src::inflate::DISTEXT;
                                break 'c_2422;
                            }
                        }
                        if state.flags & 0x1000 as ::core::ffi::c_int != 0 {
                            if have == 0 as ::core::ffi::c_uint {
                                break '_inf_leave;
                            }
                            // As in NAME, this is exactly the currently bounded input
                            // span. Keep COMMENT decoding/checksum reads in one safe
                            // view without constructing a slice for the caller's header
                            // output buffer.
                            let Some(comment_input) = input.slice_at(next.addr(), have as usize)
                            else {
                                return crate::zlib_h::Z_STREAM_ERROR;
                            };
                            copy = 0 as ::core::ffi::c_uint;
                            loop {
                                let c2rust_fresh7 = copy;
                                copy = copy.wrapping_add(1);
                                len = comment_input[c2rust_fresh7 as usize]
                                    as ::core::ffi::c_uint;
                                if let Some(head) = header.as_mut() {
                                    if !head.comment.is_null()
                                        && state.length < head.comm_max
                                    {
                                        let c2rust_fresh8 = state.length;
                                        state.length = state.length.wrapping_add(1);
                                        *head.comment.wrapping_add(c2rust_fresh8 as usize) =
                                            len as crate::stdlib::Bytef;
                                    }
                                }
                                if !(len != 0 && copy < have) {
                                    break;
                                }
                            }
                            if state.flags & 0x200 as ::core::ffi::c_int != 0
                                && state.wrap & 4 as ::core::ffi::c_int != 0
                            {
                                state.check = crate::src::crc32::crc32(
                                    state.check as crate::stdlib::uLong,
                                    Some(&comment_input[..copy as usize]),
                                )
                                    as ::core::ffi::c_ulong;
                            }
                            have = have.wrapping_sub(copy);
                            next = next.wrapping_add(copy as usize);
                            if len != 0 {
                                break '_inf_leave;
                            }
                        } else if let Some(head) = header.as_mut() {
                            head.comment =
                                ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                        }
                        state.mode = crate::src::inflate::HCRC;
                        break 'c_2327;
                    }
                    if state.extra != 0 {
                        while bits < state.extra {
                            if have == 0 as ::core::ffi::c_uint {
                                break '_inf_leave;
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh29 = next;
                            next = next.wrapping_add(1);
                            hold = hold
                                .wrapping_add((input.byte_at(c2rust_fresh29.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        state.offset = state.offset.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << state.extra)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= state.extra;
                        bits = bits.wrapping_sub(state.extra);
                        state.back = (state.back as ::core::ffi::c_uint)
                            .wrapping_add(state.extra)
                            as ::core::ffi::c_int;
                    }
                    state.mode = crate::src::inflate::MATCH;
                    break 'c_2425;
                }
                if state.flags & 0x200 as ::core::ffi::c_int != 0 {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break '_inf_leave;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh9 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((input.byte_at(c2rust_fresh9.addr()).unwrap_or(0) as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if state.wrap & 4 as ::core::ffi::c_int != 0
                        && hold != state.check & 0xffff as ::core::ffi::c_ulong
                    {
                        strm.msg = INFLATE_MESSAGES[16].as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.mode = crate::src::inflate::BAD;
                        continue '_inf_leave;
                    } else {
                        hold = 0 as ::core::ffi::c_ulong;
                        bits = 0 as ::core::ffi::c_uint;
                    }
                }
                if let Some(head) = header.as_mut() {
                    head.hcrc =
                        state.flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                    head.done = 1 as ::core::ffi::c_int;
                }
                state.check = crate::src::crc32::crc32(0 as crate::stdlib::uLong, None)
                    as ::core::ffi::c_ulong;
                strm.adler = state.check as crate::stdlib::uLong;
                state.mode = crate::src::inflate::TYPE;
                continue '_inf_leave;
            }
            if left == 0 as ::core::ffi::c_uint {
                break;
            }
            copy = out.wrapping_sub(left);
            if state.offset > copy {
                copy = state.offset.wrapping_sub(copy);
                if copy > state.whave {
                    if state.sane != 0 {
                        strm.msg = INFLATE_MESSAGES[17].as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.mode = crate::src::inflate::BAD;
                        continue;
                    }
                }
                let Some(window) = state.window else {
                    strm.msg = INFLATE_MESSAGES[17].as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state.mode = crate::src::inflate::BAD;
                    continue;
                };
                if copy > state.wnext {
                    copy = copy.wrapping_sub(state.wnext);
                    from = window
                        .as_ptr()
                        .wrapping_add(state.wsize.wrapping_sub(copy) as usize);
                } else {
                    from = window
                        .as_ptr()
                        .wrapping_add(state.wnext.wrapping_sub(copy) as usize);
                }
                if copy > state.length {
                    copy = state.length;
                }
            } else {
                from = put.wrapping_offset(-(state.offset as isize));
                copy = state.length;
            }
            if copy > left {
                copy = left;
            }
            left = left.wrapping_sub(copy);
            state.length = state.length.wrapping_sub(copy);
            loop {
                let c2rust_fresh30 = from;
                from = from.wrapping_add(1);
                let c2rust_fresh31 = put;
                put = put.wrapping_add(1);
                *c2rust_fresh31 = *c2rust_fresh30;
                copy = copy.wrapping_sub(1);
                if copy == 0 {
                    break;
                }
            }
            if state.length == 0 as ::core::ffi::c_uint {
                state.mode = crate::src::inflate::LEN;
            }
        }
        strm.next_out = put as *mut crate::stdlib::Bytef;
        strm.avail_out = left as crate::stdlib::uInt;
        strm.next_in = next as *mut crate::stdlib::Bytef;
        strm.avail_in = have as crate::stdlib::uInt;
        state.hold = hold;
        state.bits = bits;
        let Some(produced_len) = produced_output_len(out, strm.avail_out) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let produced = produced_len as ::core::ffi::c_uint;
        // The output cursor above now marks the end of this call's produced
        // bytes. Borrow that one range once for both history and checksum
        // updates, instead of rebuilding equivalent raw slices below.
        let produced_output = if produced == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts(output_start, produced_len)
        };
        if state.wsize != 0
            || produced != 0
                && (state.mode as ::core::ffi::c_uint)
                    < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                && ((state.mode as ::core::ffi::c_uint)
                    < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                    || flush != crate::zlib_h::Z_FINISH)
        {
            if updatewindow(strm, state, produced_output) != 0 {
                state.mode = crate::src::inflate::MEM;
                return crate::zlib_h::Z_MEM_ERROR;
            }
        }
        in_0 = in_0.wrapping_sub(strm.avail_in as ::core::ffi::c_uint);
        out = produced;
        strm.total_in = strm.total_in.wrapping_add(in_0 as crate::stdlib::uLong);
        strm.total_out = strm.total_out.wrapping_add(out as crate::stdlib::uLong);
        state.total = state.total.wrapping_add(out as ::core::ffi::c_ulong);
        if state.wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
            state.check = (if state.flags != 0 {
                crate::src::crc32::crc32(
                    state.check as crate::stdlib::uLong,
                    Some(produced_output),
                )
            } else {
                crate::src::adler32::adler32(
                    state.check as crate::stdlib::uLong,
                    Some(produced_output),
                )
            }) as ::core::ffi::c_ulong;
            strm.adler = state.check as crate::stdlib::uLong;
        }
        strm.data_type = state.bits as ::core::ffi::c_int
            + (if state.last != 0 {
                64 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            + (if state.mode as ::core::ffi::c_uint
                == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                128 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            + (if state.mode as ::core::ffi::c_uint
                == crate::src::inflate::LEN_ as ::core::ffi::c_int as ::core::ffi::c_uint
                || state.mode as ::core::ffi::c_uint
                    == crate::src::inflate::COPY_ as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                256 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            });
        if (in_0 == 0 as ::core::ffi::c_uint && out == 0 as ::core::ffi::c_uint
            || flush == crate::zlib_h::Z_FINISH)
            && ret == crate::zlib_h::Z_OK
        {
            ret = crate::zlib_h::Z_BUF_ERROR;
        }
        return ret;
    }
}
#[export_name = "inflate"]

pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate(strm, flush)
}

/// Release the window and state allocations in the order required by the C
/// allocator contract.  This is the named ownership boundary that a future
/// allocator facade can replace without spreading callback invocations across
/// stream operations.
fn release_inflate_allocations(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
) {
    let window = if state.owned_window.take().is_some() {
        state.window = None;
        ::core::ptr::null_mut()
    } else {
        match state.window {
            Some(window) => window.as_ptr().cast(),
            None => ::core::ptr::null_mut(),
        }
    };
    let allocations = [window, strm.state as crate::stdlib::voidpf];
    for allocation in allocations {
        if !allocation.is_null() {
            Some(strm.zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(strm.opaque, allocation);
        }
    }
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
}

/// Tear down an already-borrowed inflate state.
///
/// Keep validation and allocator-paired release policy in this typed core.
/// `inflateEnd` remains the compatibility bridge for callers that retain the
/// opaque state only in the ABI stream handle, while a future stream owner can
/// dispatch here without recreating that handle as a Rust reference.
fn inflate_end(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    release_inflate_allocations(strm, state);
    crate::zlib_h::Z_OK
}

pub fn inflateEnd(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The state handle is an ABI field.  Keep its one conversion at the
    // teardown boundary after validating the allocator pair above.
    let Some(state) = (unsafe { (strm.state as *mut crate::src::inflate::inflate_state).as_mut() })
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_end(strm, state)
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateEnd(strm)
}
fn inflate_get_dictionary(
    strm: Option<&crate::zlib_h::z_stream_s>,
    state: Option<&crate::src::inflate::inflate_state>,
    window: Option<&[crate::stdlib::Bytef]>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    let (Some(strm), Some(state)) = (strm, state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let have = state.whave as usize;
    let next = state.wnext as usize;
    let wsize = state.wsize as usize;
    if have > wsize || next > have {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if let Some(dictionary) = dictionary {
        let Some(destination) = dictionary.get_mut(..have) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if have != 0 {
            let Some(window) = window else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let Some(tail) = window.get(next..have) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let Some(head) = window.get(..next) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            let tail_len = tail.len();
            destination[..tail_len].copy_from_slice(tail);
            destination[tail_len..].copy_from_slice(head);
        }
    }
    if let Some(dict_length) = dict_length {
        *dict_length = state.whave;
    }
    crate::zlib_h::Z_OK
}
#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_ref() else {
        return inflate_get_dictionary(None, None, None, None, None);
    };
    if !inflate_stream_has_allocators(strm) {
        return inflate_get_dictionary(None, None, None, None, None);
    }
    let Some(state) = (strm.state as *const crate::src::inflate::inflate_state).as_ref() else {
        return inflate_get_dictionary(Some(strm), None, None, None, dictLength.as_mut());
    };
    let window = if state.window.is_none() {
        None
    } else {
        Some(::core::slice::from_raw_parts(
            state.window.expect("checked non-null window").as_ptr(),
            state.wsize as usize,
        ))
    };
    let dictionary = if dictionary.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(
            dictionary,
            state.whave as usize,
        ))
    };
    inflate_get_dictionary(
        Some(strm),
        Some(state),
        window,
        dictionary,
        dictLength.as_mut(),
    )
}
fn inflate_set_dictionary_check(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::inflate::inflate_state,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.wrap != 0 as ::core::ffi::c_int
        && state.mode as ::core::ffi::c_uint
            != crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.mode as ::core::ffi::c_uint
        == crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let dictid = crate::src::adler32::adler32(
            crate::src::adler32::adler32(0 as crate::stdlib::uLong, None),
            Some(dictionary),
        ) as ::core::ffi::c_ulong;
        if dictid != state.check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }
    crate::zlib_h::Z_OK
}

fn inflate_set_dictionary_complete(state: &mut crate::src::inflate::inflate_state) {
    state.havedict = 1 as ::core::ffi::c_int;
}

fn inflate_set_dictionary(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let check = inflate_set_dictionary_check(strm, state, dictionary);
    if check != crate::zlib_h::Z_OK {
        return check;
    }
    let ret = updatewindow(strm, state, dictionary);
    if ret != 0 {
        state.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    inflate_set_dictionary_complete(state);
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSetDictionary"]

pub unsafe extern "C" fn inflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if dictLength != 0 && dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = if dictLength == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(dictionary, dictLength as usize)
    };
    inflate_set_dictionary(strm, state, dictionary)
}
pub fn inflateGetHeader(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
    head: &mut crate::zlib_h::gz_header_s,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.head = head;
    head.done = 0 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(head) = head.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateGetHeader(strm, state, head)
}
fn syncsearch(have: &mut ::core::ffi::c_uint, buf: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let mut got = *have;
    let mut next = 0usize;
    while next < buf.len() && got < 4 as ::core::ffi::c_uint {
        if buf[next] as ::core::ffi::c_int
            == (if got < 2 as ::core::ffi::c_uint {
                0 as ::core::ffi::c_int
            } else {
                0xff as ::core::ffi::c_int
            })
        {
            got = got.wrapping_add(1);
        } else if buf[next] != 0 {
            got = 0 as ::core::ffi::c_uint;
        } else {
            got = (4 as ::core::ffi::c_uint).wrapping_sub(got);
        }
        next += 1;
    }
    *have = got;
    next as ::core::ffi::c_uint
}
fn inflate_sync(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    input: &[u8],
) -> (::core::ffi::c_int, usize) {
    let mut len: ::core::ffi::c_uint = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut in_0: ::core::ffi::c_ulong = 0;
    let mut out: ::core::ffi::c_ulong = 0;
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    if !inflate_state_valid(strm, state) {
        return (crate::zlib_h::Z_STREAM_ERROR, 0);
    }
    if input.len() != strm.avail_in as usize {
        return (crate::zlib_h::Z_STREAM_ERROR, 0);
    }
    if strm.avail_in == 0 as crate::stdlib::uInt && state.bits < 8 as ::core::ffi::c_uint {
        return (crate::zlib_h::Z_BUF_ERROR, 0);
    }
    if state.mode as ::core::ffi::c_uint
        != crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        state.mode = crate::src::inflate::SYNC;
        state.hold >>= state.bits & 7 as ::core::ffi::c_uint;
        state.bits = state
            .bits
            .wrapping_sub(state.bits & 7 as ::core::ffi::c_uint);
        len = 0 as ::core::ffi::c_uint;
        while state.bits >= 8 as ::core::ffi::c_uint {
            let c2rust_fresh35 = len;
            len = len.wrapping_add(1);
            buf[c2rust_fresh35 as usize] = state.hold as ::core::ffi::c_uchar;
            state.hold >>= 8 as ::core::ffi::c_int;
            state.bits = state.bits.wrapping_sub(8 as ::core::ffi::c_uint);
        }
        state.have = 0 as ::core::ffi::c_uint;
        syncsearch(&mut state.have, &buf[..len as usize]);
    }
    len = syncsearch(&mut state.have, input);
    strm.avail_in = strm.avail_in.wrapping_sub(len);
    strm.total_in = strm.total_in.wrapping_add(len as crate::stdlib::uLong);
    if state.have != 4 as ::core::ffi::c_uint {
        return (crate::zlib_h::Z_DATA_ERROR, len as usize);
    }
    if state.flags == -1 as ::core::ffi::c_int {
        state.wrap = 0 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    flags = state.flags;
    in_0 = strm.total_in as ::core::ffi::c_ulong;
    out = strm.total_out as ::core::ffi::c_ulong;
    inflateReset(strm, state);
    strm.total_in = in_0 as crate::stdlib::uLong;
    strm.total_out = out as crate::stdlib::uLong;
    state.flags = flags;
    state.mode = crate::src::inflate::TYPE;
    (crate::zlib_h::Z_OK, len as usize)
}
#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let input = if strm.avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
    };
    let next_in = strm.next_in;
    let (ret, consumed) = inflate_sync(strm, state, input);
    strm.next_in = next_in.wrapping_add(consumed);
    ret
}
fn inflate_sync_point(state: Option<&crate::src::inflate::inflate_state>) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    return (state.mode as ::core::ffi::c_uint
        == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint
        && state.bits == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
fn inflate_sync_point_stream(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return inflate_sync_point(None);
    }
    inflate_sync_point(Some(state))
}
#[export_name = "inflateSyncPoint"]

pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_ref() else {
        return inflate_sync_point(None);
    };
    if !inflate_stream_has_allocators(strm) {
        return inflate_sync_point(None);
    }
    let Some(state) = (strm.state as *const crate::src::inflate::inflate_state).as_ref() else {
        return inflate_sync_point(None);
    };
    inflate_sync_point_stream(strm, state)
}

/// Allocate and install a deep copy after `inflateCopy` has validated the
/// source tables.  The ABI callbacks and their untyped allocations stay
/// confined to this named ownership boundary.
fn initialize_inflate_copy(
    dest: &mut crate::zlib_h::z_stream,
    source: &crate::zlib_h::z_stream,
    source_state: &crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    // Keep allocation callbacks on a local stream copy until every fallible
    // allocation succeeds. This preserves C's observable rule that a failed
    // copy leaves `dest` untouched, while the typed slot helper publishes the
    // first initialized Rust state without a second raw conversion.
    let mut allocation_stream = crate::zlib_h::z_stream_s {
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
    with_callback_inflate_state_slot(
        &mut allocation_stream,
        copy_inflate_state(source_state),
        |allocation_stream, copy_ref| {
            // Record the new allocation only in the local stream so failure
            // cleanup can use the original callback pair without exposing a
            // half-initialized destination.
            allocation_stream.state =
                ::core::ptr::from_mut(copy_ref).cast::<crate::src::deflate::internal_state>();
            let mut owned_window = None;
            let mut owned_window_failed = false;
            if let Some(source_window) = source_state.owned_window.as_ref() {
                owned_window = source_window.try_copy_for_state(source_state);
                owned_window_failed = owned_window.is_none();
            }
            let mut window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            if source_state.window.is_some() && owned_window.is_none() && !owned_window_failed {
                let layout = InflateWindowLayout::from_state(source_state)
                    .expect("inflateCopy validated the source window layout");
                window = Some(allocation_stream.zalloc.expect("validated allocator"))
                    .expect("validated allocator")(
                    allocation_stream.opaque,
                    layout.allocation_items,
                    ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
                ) as *mut ::core::ffi::c_uchar;
            }
            if owned_window_failed
                || (source_state.window.is_some() && window.is_null() && owned_window.is_none())
            {
                Some(allocation_stream.zfree.expect("validated allocator"))
                    .expect("validated allocator")(
                    allocation_stream.opaque,
                    allocation_stream.state.cast(),
                );
                return crate::zlib_h::Z_MEM_ERROR;
            }
            if !window.is_null() {
                unsafe {
                    ::core::ptr::copy_nonoverlapping(
                        source_state.window.expect("checked non-null window").as_ptr(),
                        window,
                        source_state.whave as usize,
                    );
                }
            }
            copy_ref.strm = stream_identity(dest);
            if let Some(owned_window) = owned_window {
                install_owned_inflate_window(copy_ref, owned_window);
            } else {
                copy_ref.window = ::core::ptr::NonNull::new(window);
            }
            crate::zlib_h::copy_z_stream(dest, source);
            dest.state = allocation_stream.state;
            crate::zlib_h::Z_OK
        },
    )
    .unwrap_or(crate::zlib_h::Z_MEM_ERROR)
}

/// Copy an already validated source state into a destination stream.
///
/// The `inflateCopy` ABI wrapper owns conversion of the opaque source-state
/// handle.  Keeping this core typed prevents the copy validation and setup
/// from needing to follow an ABI raw pointer itself.
fn inflate_copy(
    dest: Option<&mut crate::zlib_h::z_stream>,
    source: Option<&crate::zlib_h::z_stream>,
    source_state: Option<&crate::src::inflate::inflate_state>,
) -> ::core::ffi::c_int {
    let Some(source_ref) = source else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(source_ref) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state_ref) = source_state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_state_valid(source_ref, state_ref) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(dest_ref) = dest else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let valid_table_ref = |table: &InflateTableRef| match table {
        InflateTableRef::FixedLiteralLength | InflateTableRef::FixedDistance => true,
        InflateTableRef::Dynamic(index) => *index < crate::src::inftrees::ENOUGH as usize,
    };
    if state_ref.next > crate::src::inftrees::ENOUGH as usize
        || !valid_table_ref(&state_ref.lencode)
        || !valid_table_ref(&state_ref.distcode)
        || !inflate_window_layout_valid(state_ref)
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    initialize_inflate_copy(dest_ref, source_ref, state_ref)
}
#[export_name = "inflateCopy"]

pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let source = source.as_ref();
    let dest = dest.as_mut();
    let source_state = source.and_then(|source| {
        if !inflate_stream_has_allocators(source) {
            return None;
        }
        (source.state as *const crate::src::inflate::inflate_state).as_ref()
    });
    inflate_copy(dest, source, source_state)
}
fn inflate_undermine(
    state: Option<&mut crate::src::inflate::inflate_state>,
    _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    state.sane = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_DATA_ERROR;
}
fn inflate_undermine_stream(
    strm: &crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return inflate_undermine(None, subvert);
    }
    inflate_undermine(Some(state), subvert)
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return inflate_undermine(None, subvert);
    };
    if !inflate_stream_has_allocators(strm) {
        return inflate_undermine(None, subvert);
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return inflate_undermine(None, subvert);
    };
    inflate_undermine_stream(strm, state, subvert)
}
fn inflate_validate(
    state: Option<&mut crate::src::inflate::inflate_state>,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if check != 0 && state.wrap != 0 {
        state.wrap |= 4 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    return crate::zlib_h::Z_OK;
}
fn inflate_validate_stream(
    strm: &crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return inflate_validate(None, check);
    }
    inflate_validate(Some(state), check)
}
#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return inflate_validate(None, check);
    };
    if !inflate_stream_has_allocators(strm) {
        return inflate_validate(None, check);
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return inflate_validate(None, check);
    };
    inflate_validate_stream(strm, state, check)
}
fn inflate_mark(state: Option<&crate::src::inflate::inflate_state>) -> ::core::ffi::c_long {
    let Some(state) = state else {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    };
    return ((state.back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int)
        as ::core::ffi::c_long
        + (if state.mode as ::core::ffi::c_uint
            == crate::src::inflate::COPY_1 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            state.length
        } else {
            if state.mode as ::core::ffi::c_uint
                == crate::src::inflate::MATCH as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                state.was.wrapping_sub(state.length)
            } else {
                0 as ::core::ffi::c_uint
            }
        }) as ::core::ffi::c_long;
}
fn inflate_mark_stream(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::inflate::inflate_state,
) -> ::core::ffi::c_long {
    if !inflate_state_valid(strm, state) {
        return inflate_mark(None);
    }
    inflate_mark(Some(state))
}
#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    let Some(strm) = strm.as_ref() else {
        return inflate_mark(None);
    };
    if !inflate_stream_has_allocators(strm) {
        return inflate_mark(None);
    }
    let Some(state) = (strm.state as *const crate::src::inflate::inflate_state).as_ref() else {
        return inflate_mark(None);
    };
    inflate_mark_stream(strm, state)
}
fn inflate_codes_used(state: Option<&crate::src::inflate::inflate_state>) -> ::core::ffi::c_ulong {
    let Some(state) = state else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    return state.next as ::core::ffi::c_ulong;
}
fn inflate_codes_used_stream(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::inflate::inflate_state,
) -> ::core::ffi::c_ulong {
    if !inflate_state_valid(strm, state) {
        return inflate_codes_used(None);
    }
    inflate_codes_used(Some(state))
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    let Some(strm) = strm.as_ref() else {
        return inflate_codes_used(None);
    };
    if !inflate_stream_has_allocators(strm) {
        return inflate_codes_used(None);
    }
    let Some(state) = (strm.state as *const crate::src::inflate::inflate_state).as_ref() else {
        return inflate_codes_used(None);
    };
    inflate_codes_used_stream(strm, state)
}
