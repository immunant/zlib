// =============== BEGIN inflate_h ================
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

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

/// Identifies the distance decode table without retaining a pointer into the
/// state-owned arena.  A dynamic table starts at an entry in `codes`; fixed
/// distance tables are immutable module data.
#[derive(Copy, Clone)]
pub enum distance_table {
    Fixed,
    Dynamic(usize),
}

impl distance_table {
    pub fn entry(
        self,
        codes: &[crate::src::inftrees::code; 1444],
        index: usize,
    ) -> crate::src::inftrees::code {
        const INVALID: crate::src::inftrees::code = crate::src::inftrees::code {
            op: 64,
            bits: 0,
            val: 0,
        };

        match self {
            distance_table::Fixed => crate::src::inftrees::distfix
                .get(index)
                .copied()
                .unwrap_or(INVALID),
            distance_table::Dynamic(start) => start
                .checked_add(index)
                .and_then(|entry| codes.get(entry))
                .copied()
                .unwrap_or(INVALID),
        }
    }
}

#[derive(Copy, Clone)]
pub enum length_table {
    Fixed,
    Dynamic(usize),
}

impl length_table {
    pub fn entry(
        self,
        codes: &[crate::src::inftrees::code; 1444],
        index: usize,
    ) -> crate::src::inftrees::code {
        const INVALID: crate::src::inftrees::code = crate::src::inftrees::code {
            op: 64,
            bits: 0,
            val: 0,
        };

        match self {
            length_table::Fixed => crate::src::inftrees::lenfix
                .get(index)
                .copied()
                .unwrap_or(INVALID),
            length_table::Dynamic(start) => start
                .checked_add(index)
                .and_then(|entry| codes.get(entry))
                .copied()
                .unwrap_or(INVALID),
        }
    }
}

#[derive(Clone)]
#[repr(C)]

pub struct inflate_state {
    pub mode: crate::src::inflate::inflate_mode,
    pub last: ::core::ffi::c_int,
    pub wrap: ::core::ffi::c_int,
    pub havedict: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub dmax: ::core::ffi::c_uint,
    pub check: ::core::ffi::c_ulong,
    pub total: ::core::ffi::c_ulong,
    // `inflateGetHeader()` registers a caller-owned ABI header for later
    // `inflate()` calls.  Decoder state only records that such a header is
    // attached; the ABI boundary owns the address needed to materialize its
    // bounded output views for a single call.
    pub head: bool,
    pub wbits: ::core::ffi::c_uint,
    pub wsize: ::core::ffi::c_uint,
    pub whave: ::core::ffi::c_uint,
    pub wnext: ::core::ffi::c_uint,
    /// The ordinary inflater's history ring.  Back-mode receives its caller
    /// window only for the duration of `inflateBack()`, so it uses this same
    /// owner without retaining the caller's raw pointer.
    pub window: Option<Vec<::core::ffi::c_uchar>>,
    pub hold: ::core::ffi::c_ulong,
    pub bits: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub extra: ::core::ffi::c_uint,
    pub lencode: crate::src::inflate::length_table,
    pub distcode: crate::src::inflate::distance_table,
    pub lenbits: ::core::ffi::c_uint,
    pub distbits: ::core::ffi::c_uint,
    pub ncode: ::core::ffi::c_uint,
    pub nlen: ::core::ffi::c_uint,
    pub ndist: ::core::ffi::c_uint,
    pub have: ::core::ffi::c_uint,
    // The next unused entry in `codes`.  Keeping this as an index prevents an
    // interior pointer from escaping the table arena.
    pub next: usize,
    pub lens: [::core::ffi::c_ushort; 320],
    pub work: [::core::ffi::c_ushort; 288],
    pub codes: [crate::src::inftrees::code; 1444],
    pub sane: ::core::ffi::c_int,
    pub back: ::core::ffi::c_int,
    pub was: ::core::ffi::c_uint,
}

/// Own the Rust state separately from the opaque allocation obtained through
/// a caller-provided ABI allocator.  The latter is just a token: it must be
/// returned to the same allocator, but it must never be treated as initialized
/// Rust storage.
enum InflateStateOwner {
    Default(Box<inflate_state>),
    Callback {
        state: Box<inflate_state>,
        // The allocator token is recorded only after the callback succeeds.
        // Reserving the Rust owner first means a registry allocation failure
        // never leaves an ABI allocation that needs a cleanup callback.
        allocation_address: Option<usize>,
    },
}

static INFLATE_STATE_OWNERS: OnceLock<Mutex<HashMap<usize, InflateStateOwner>>> = OnceLock::new();

fn inflate_state_owners() -> &'static Mutex<HashMap<usize, InflateStateOwner>> {
    INFLATE_STATE_OWNERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn retain_inflate_state_owner(address: usize, owner: InflateStateOwner) -> bool {
    let mut states = inflate_state_owners()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if states.try_reserve(1).is_err() {
        return false;
    }
    states.insert(address, owner);
    true
}

pub(crate) fn retain_default_inflate_state(state: Box<inflate_state>) -> bool {
    let address = core::ptr::from_ref(state.as_ref()).addr();
    retain_inflate_state_owner(address, InflateStateOwner::Default(state))
}

/// Retain a callback-backed state before requesting its opaque ABI allocation.
/// The returned address is the stable Rust owner address exposed through the
/// stream; the callback allocation is only a token recorded afterwards.
pub(crate) fn retain_callback_inflate_state(state: Box<inflate_state>) -> Option<usize> {
    let address = core::ptr::from_ref(state.as_ref()).addr();
    retain_inflate_state_owner(
        address,
        InflateStateOwner::Callback {
            state,
            allocation_address: None,
        },
    )
    .then_some(address)
}

/// Associate a successfully allocated ABI token with its pre-retained Rust
/// owner.  The owner is inserted before calling the allocator, so this update
/// cannot allocate or fail under normal operation.
pub(crate) fn set_callback_inflate_state_allocation(address: usize, allocation_address: usize) {
    let mut states = inflate_state_owners()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(InflateStateOwner::Callback {
        allocation_address: slot,
        ..
    }) = states.get_mut(&address)
    else {
        debug_assert!(false, "callback state owner must be retained first");
        return;
    };
    debug_assert!(slot.is_none(), "callback allocation token recorded twice");
    *slot = Some(allocation_address);
}

/// Drop the Rust state owner and return the callback allocation token, when
/// there is one, for the ABI boundary to return to `zfree`.
pub(crate) fn release_inflate_state_owner(address: usize) -> Option<usize> {
    let owner = inflate_state_owners()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&address)?;
    match owner {
        InflateStateOwner::Default(state) => {
            drop(state);
            None
        }
        InflateStateOwner::Callback {
            state,
            allocation_address,
        } => {
            drop(state);
            allocation_address
        }
    }
}

/// Release a state known to have used the default Rust allocator.  The
/// inflate-back path shares that ownership registry but has no ABI allocation
/// token to return.
pub(crate) fn release_default_inflate_state(address: usize) {
    debug_assert!(release_inflate_state_owner(address).is_none());
}

pub(crate) fn new_inflate_state() -> inflate_state {
    inflate_state {
        mode: crate::src::inflate::HEAD,
        last: 0,
        wrap: 0,
        havedict: 0,
        flags: 0,
        dmax: 0,
        check: 0,
        total: 0,
        head: false,
        wbits: 0,
        wsize: 0,
        whave: 0,
        wnext: 0,
        window: None,
        hold: 0,
        bits: 0,
        length: 0,
        offset: 0,
        extra: 0,
        lencode: length_table::Dynamic(0),
        distcode: distance_table::Dynamic(0),
        lenbits: 0,
        distbits: 0,
        ncode: 0,
        nlen: 0,
        ndist: 0,
        have: 0,
        next: 0,
        lens: [0; 320],
        work: [0; 288],
        codes: [crate::src::inftrees::code {
            op: 0,
            bits: 0,
            val: 0,
        }; 1444],
        sane: 0,
        back: 0,
        was: 0,
    }
}
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::adler32::adler32;
pub use crate::src::crc32::crc32;
pub use crate::src::deflate::internal_state;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::inflate_fixed;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::ENOUGH;
pub use crate::src::inftrees::ENOUGH_DISTS;
pub use crate::src::inftrees::ENOUGH_LENS;
pub use crate::src::inftrees::LENS;
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

fn inflate_state_valid(
    strm: &crate::zlib_h::z_stream_s,
    state: &crate::src::inflate::inflate_state,
) -> bool {
    inflate_state_mode_valid(state)
        && (strm.state.is_null() || inflate_stream_has_state_allocation(strm))
}

fn inflate_stream_has_state_allocation(strm: &crate::zlib_h::z_stream_s) -> bool {
    (strm.zalloc.is_some() && strm.zfree.is_some())
        || (strm.zalloc.is_none() && strm.zfree.is_none())
}

fn inflate_state_mode_valid(state: &crate::src::inflate::inflate_state) -> bool {
    state.mode >= crate::src::inflate::HEAD && state.mode <= crate::src::inflate::SYNC
}

fn inflate_reset_keep_state(
    state: &mut crate::src::inflate::inflate_state,
) -> Option<crate::stdlib::uLong> {
    state.total = 0 as ::core::ffi::c_ulong;
    let adler =
        (state.wrap != 0).then_some((state.wrap & 1 as ::core::ffi::c_int) as crate::stdlib::uLong);
    state.mode = crate::src::inflate::HEAD;
    state.last = 0 as ::core::ffi::c_int;
    state.havedict = 0 as ::core::ffi::c_int;
    state.flags = -1 as ::core::ffi::c_int;
    state.dmax = 32768 as ::core::ffi::c_uint;
    if state.head {
        remove_inflate_header(state);
    }
    state.head = false;
    state.hold = 0 as ::core::ffi::c_ulong;
    state.bits = 0 as ::core::ffi::c_uint;
    state.next = 0;
    state.distcode = crate::src::inflate::distance_table::Dynamic(0);
    state.lencode = crate::src::inflate::length_table::Dynamic(0);
    state.sane = 1 as ::core::ffi::c_int;
    state.back = -1 as ::core::ffi::c_int;
    adler
}

fn inflate_reset_keep_stream(
    strm: &mut crate::zlib_h::z_stream_s,
    adler: Option<crate::stdlib::uLong>,
) {
    strm.total_out = 0;
    strm.total_in = 0;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = 0;
    if let Some(adler) = adler {
        strm.adler = adler;
    }
}

fn inflate_reset_keep_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    inflate_reset_keep_stream(strm, inflate_reset_keep_state(state));
    crate::zlib_h::Z_OK
}

fn inflate_reset_keep_from_stream(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    let adler = {
        let Some(state) = inflate_validate_state(strm, state) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        inflate_reset_keep_state(state)
    };
    inflate_reset_keep_stream(strm, adler);
    crate::zlib_h::Z_OK
}
#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = strm
        .state
        .cast::<crate::src::inflate::inflate_state>()
        .as_mut()
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_reset_keep_from_stream(strm, state)
}
fn inflate_reset_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.wsize = 0 as ::core::ffi::c_uint;
    state.whave = 0 as ::core::ffi::c_uint;
    state.wnext = 0 as ::core::ffi::c_uint;
    inflate_reset_keep_impl(strm, state)
}

/// Reset an already-initialized inflate stream held by an internal owner.
///
/// The gzip reader resolves the ABI state link before this call.  Reset then
/// operates only on ordinary Rust borrows of the stream carrier and decoder
/// state.
pub(crate) fn inflate_reset_gzip(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    let adler = {
        let state = if strm.state.is_null() {
            inflate_state_mode_valid(state).then_some(state)
        } else {
            inflate_validate_state(strm, state)
        };
        let Some(state) = state else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        state.wsize = 0;
        state.whave = 0;
        state.wnext = 0;
        inflate_reset_keep_state(state)
    };
    inflate_reset_keep_stream(strm, adler);
    crate::zlib_h::Z_OK
}

/// Initialize the decoder state owned by a gzip reader.
///
/// Unlike a public inflate stream, this state is held directly by `gz_state`,
/// so it has no allocator callbacks or ABI `state` link to validate.  The
/// stream carrier still receives the same counters and checksum setup that
/// `inflateInit2_(..., 15 + 16, ...)` would establish.
pub(crate) fn new_gzip_inflate_state() -> crate::src::inflate::inflate_state {
    let mut state = new_inflate_state();
    state.wrap = 6;
    state.wbits = 15;
    state.wsize = 0;
    state.whave = 0;
    state.wnext = 0;
    inflate_reset_keep_state(&mut state);
    state
}

pub(crate) fn initialize_gzip_stream(strm: &mut crate::zlib_h::z_stream_s) {
    inflate_reset_keep_stream(strm, Some(0));
}

#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = strm
        .state
        .cast::<crate::src::inflate::inflate_state>()
        .as_mut()
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_reset_gzip(strm, state)
}
fn inflate_reset2_window_bits(
    mut window_bits: ::core::ffi::c_int,
) -> Option<(::core::ffi::c_int, ::core::ffi::c_uint)> {
    let wrap;
    if window_bits < 0 {
        if window_bits < -15 {
            return None;
        }
        wrap = 0;
        window_bits = -window_bits;
    } else {
        wrap = (window_bits >> 4) + 5;
        if window_bits < 48 {
            window_bits &= 15;
        }
    }
    if window_bits != 0 && !(8..=15).contains(&window_bits) {
        return None;
    }
    Some((wrap, window_bits as ::core::ffi::c_uint))
}

fn inflate_reset2_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut InflateState,
    window_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some((wrap, window_bits)) = inflate_reset2_window_bits(window_bits) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if state.window.is_some() && state.wbits != window_bits {
        state.window = None;
    }
    state.wrap = wrap;
    state.wbits = window_bits;
    state.wsize = 0;
    state.whave = 0;
    state.wnext = 0;
    inflate_reset_keep_impl(strm, state)
}

#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let state = strm.state.cast::<crate::src::inflate::inflate_state>();
    let Some(state) = (unsafe { state.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let mut state = InflateState(state);
    inflate_reset2_impl(strm, &mut state, windowBits)
}
/// The final operation to perform after an inflater state has been installed
/// in its ABI-visible storage.
///
/// The caller supplies a fully initialized Rust value.  Initialization that
/// needs the stream's state link (the public init path) and the copied-header
/// association (the copy path) happen only after that value has been written
/// to its final storage.
#[derive(Copy, Clone)]
enum InflateStateInstallation<'a> {
    Initialize(::core::ffi::c_int),
    Copy {
        source_stream: &'a crate::zlib_h::z_stream_s,
        source_state: &'a crate::src::inflate::inflate_state,
    },
}

/// Allocate, initialize, and install an inflater state through the stream's
/// ABI allocator.
///
/// This is the single callback-owned state-storage handoff shared by public
/// initialization and copying.  Callers construct codec-owned data first;
/// this helper alone invokes the paired allocator and writes the resulting
/// state slot.
unsafe fn inflate_allocate_state(
    strm: &mut crate::zlib_h::z_stream_s,
    initialized: crate::src::inflate::inflate_state,
    installation: InflateStateInstallation<'_>,
) -> ::core::ffi::c_int {
    let (source_stream, copied_header, zalloc, zfree, opaque) = match installation {
        InflateStateInstallation::Initialize(_) => {
            (None, None, strm.zalloc, strm.zfree, strm.opaque)
        }
        InflateStateInstallation::Copy {
            source_stream,
            source_state,
        } => (
            Some(source_stream),
            Some(source_state),
            source_stream.zalloc,
            source_stream.zfree,
            source_stream.opaque,
        ),
    };
    if zalloc.is_none() && zfree.is_none() {
        let mut state = Box::new(initialized);
        let state_pointer = core::ptr::from_mut(state.as_mut());
        if source_stream.is_none() {
            strm.state = state_pointer.cast();
        }
        let ret = match installation {
            InflateStateInstallation::Initialize(window_bits) => {
                inflate_reset2_impl(strm, &mut InflateState(state.as_mut()), window_bits)
            }
            InflateStateInstallation::Copy { .. } => crate::zlib_h::Z_OK,
        };
        if ret != crate::zlib_h::Z_OK {
            inflate_release_owned_state(state.as_mut());
            strm.state = core::ptr::null_mut();
            return ret;
        }
        if retain_default_inflate_state(state) {
            if let Some(source) = source_stream {
                *strm = *source;
                strm.state = state_pointer.cast();
            }
            if let Some(source) = copied_header {
                copy_inflate_header_registration(source, strm.state.addr());
            }
            return ret;
        }
        if source_stream.is_none() {
            strm.state = core::ptr::null_mut();
        }
        return crate::zlib_h::Z_MEM_ERROR;
    }
    let (Some(zalloc), Some(_)) = (zalloc, zfree) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let allocation = zalloc(
        opaque,
        1,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    );
    if allocation.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // Callback storage is opaque ABI-owned memory, not Rust storage.  Keep
    // the initialized state in a Box and retain the allocation address only
    // so `inflateEnd` can return it through the paired callback.
    let mut state = Some(Box::new(initialized));
    let state_pointer = core::ptr::from_mut(state.as_deref_mut().expect("new state"));
    if source_stream.is_none() {
        strm.state = state_pointer.cast::<crate::src::deflate::internal_state>();
    }
    let ret = match installation {
        InflateStateInstallation::Initialize(window_bits) => inflate_reset2_impl(
            strm,
            &mut InflateState(state.as_deref_mut().expect("new state")),
            window_bits,
        ),
        InflateStateInstallation::Copy { .. } => crate::zlib_h::Z_OK,
    };
    let retained = if ret == crate::zlib_h::Z_OK {
        let mut owners = inflate_state_owners()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if owners.try_reserve(1).is_err() {
            false
        } else {
            owners.insert(
                state_pointer.addr(),
                InflateStateOwner::Callback {
                    state: state.take().expect("new state"),
                    allocation_address: Some(allocation.addr()),
                },
            );
            true
        }
    } else {
        false
    };
    if !retained {
        inflate_release_owned_state(state.as_deref_mut().expect("unretained state"));
        zfree.expect("callback pair is validated")(opaque, allocation);
        if source_stream.is_none() {
            strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
        }
        return if ret == crate::zlib_h::Z_OK {
            crate::zlib_h::Z_MEM_ERROR
        } else {
            ret
        };
    } else {
        if let Some(source) = source_stream {
            *strm = *source;
            strm.state = state_pointer.cast();
        }
        if let Some(source) = copied_header {
            copy_inflate_header_registration(source, state_pointer.addr());
        }
    }
    ret
}

/// Drop and return an inflater state to the allocator that created it.
///
/// Callers invoke this only for a state installed by `inflate_allocate_state`,
/// after ensuring that the stream still carries a matching free callback.
fn inflate_release_owned_state(state: &mut crate::src::inflate::inflate_state) {
    // The history window is the state allocation's only owned field.  Clear it
    // before returning the surrounding ABI allocation to its callback, rather
    // than dropping that callback-owned allocation in place through a raw
    // pointer.
    state.window = None;
    if state.head {
        remove_inflate_header(state);
    }
    state.head = false;
}

/// The validated, fully Rust-owned portion of public inflater initialization.
///
/// Allocator selection and ownership transfer intentionally happen later in
/// `inflate_allocate_state()`: only that boundary can pair a caller's `zalloc`
/// and `zfree` callbacks.  Keeping validation and state construction here
/// leaves that boundary with one narrow responsibility.
struct InflateInitPreparation {
    state: crate::src::inflate::inflate_state,
    window_bits: ::core::ffi::c_int,
}

/// A stream prepared for the public inflater-initialization implementation.
///
/// This keeps the ABI carrier at the boundary of initialization.  In
/// particular, the implementation owns the allocator validation and the
/// state-storage handoff instead of making either exported entry point manage
/// callback-owned memory.
struct InflateInitStream<'a> {
    stream: &'a mut crate::zlib_h::z_stream_s,
}

impl<'a> InflateInitStream<'a> {
    fn new(stream: &'a mut crate::zlib_h::z_stream_s) -> Self {
        Self { stream }
    }

    fn clear_message(&mut self) {
        self.stream.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }

    fn install(&mut self, preparation: InflateInitPreparation) -> ::core::ffi::c_int {
        // `InflateInitStream` is constructed only after the ABI wrapper has
        // checked the stream pointer.  The remaining unsafety is limited to
        // the paired allocator callback and its typed-state handoff.
        unsafe {
            inflate_allocate_state(
                self.stream,
                preparation.state,
                InflateStateInstallation::Initialize(preparation.window_bits),
            )
        }
    }
}

fn prepare_inflate_init(
    window_bits: ::core::ffi::c_int,
    version: Option<::core::ffi::c_char>,
    stream_size: ::core::ffi::c_int,
) -> Result<InflateInitPreparation, ::core::ffi::c_int> {
    if version != Some(crate::zlib_h::ZLIB_VERSION[0])
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return Err(crate::zlib_h::Z_VERSION_ERROR);
    }
    let mut state = new_inflate_state();
    state.mode = crate::src::inflate::HEAD;
    Ok(InflateInitPreparation { state, window_bits })
}

fn inflate_init2_impl(
    mut stream: InflateInitStream<'_>,
    window_bits: ::core::ffi::c_int,
    version: Option<::core::ffi::c_char>,
    stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let preparation = match prepare_inflate_init(window_bits, version, stream_size) {
        Ok(preparation) => preparation,
        Err(error) => return error,
    };
    stream.clear_message();
    stream.install(preparation)
}

pub unsafe fn inflateInit2_(
    strm: &mut crate::zlib_h::z_stream_s,
    windowBits: ::core::ffi::c_int,
    version: Option<::core::ffi::c_char>,
    stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_init2_impl(
        InflateInitStream::new(strm),
        windowBits,
        version,
        stream_size,
    )
}
#[export_name = "inflateInit2_"]

pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_init2_impl(
        InflateInitStream::new(strm),
        windowBits,
        version.as_ref().copied(),
        stream_size,
    )
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_init2_impl(
        InflateInitStream::new(strm),
        crate::zutil_h::DEF_WBITS,
        version.as_ref().copied(),
        stream_size,
    )
}
fn inflate_prime_impl(
    mode: crate::src::inflate::inflate_mode,
    hold: &mut ::core::ffi::c_ulong,
    held_bits: &mut ::core::ffi::c_uint,
    bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if mode < crate::src::inflate::HEAD || mode > crate::src::inflate::SYNC {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if bits == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_OK;
    }
    if bits < 0 as ::core::ffi::c_int {
        *hold = 0 as ::core::ffi::c_ulong;
        *held_bits = 0 as ::core::ffi::c_uint;
        return crate::zlib_h::Z_OK;
    }
    if bits > 16 as ::core::ffi::c_int
        || (*held_bits as crate::stdlib::uInt).wrapping_add(bits as crate::stdlib::uInt)
            > 32 as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    value = (value as ::core::ffi::c_long
        & ((1 as ::core::ffi::c_long) << bits) - 1 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
    *hold = hold.wrapping_add((value as ::core::ffi::c_ulong) << *held_bits);
    *held_bits = held_bits.wrapping_add(bits as crate::stdlib::uInt as ::core::ffi::c_uint);
    return crate::zlib_h::Z_OK;
}

fn inflate_prime_from_state(
    allocators_present: bool,
    state: &mut crate::src::inflate::inflate_state,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !allocators_present {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_prime_impl(state.mode, &mut state.hold, &mut state.bits, bits, value)
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
    let allocators_present = inflate_stream_has_state_allocation(strm);
    let state = strm.state.cast::<crate::src::inflate::inflate_state>();
    let Some(state) = state.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_prime_from_state(allocators_present, state, bits, value)
}
fn updatewindow(
    window: &mut Option<Vec<crate::stdlib::Bytef>>,
    wbits: crate::stdlib::uInt,
    wsize: &mut crate::stdlib::uInt,
    wnext: &mut crate::stdlib::uInt,
    whave: &mut crate::stdlib::uInt,
    end: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if window.is_none() {
        let window_size = 1usize << wbits;
        let mut buffer = Vec::new();
        if buffer.try_reserve_exact(window_size).is_err() {
            return 1 as ::core::ffi::c_int;
        }
        buffer.resize(window_size, 0);
        *window = Some(buffer);
    }
    if *wsize == 0 as ::core::ffi::c_uint {
        *wsize = (1 as ::core::ffi::c_uint) << wbits;
        *wnext = 0 as ::core::ffi::c_uint;
        *whave = 0 as ::core::ffi::c_uint;
    }
    let window_size = *wsize as usize;
    let window_next = *wnext as usize;
    if window_next > window_size {
        return 1 as ::core::ffi::c_int;
    }
    let Some(window) = window.as_deref_mut() else {
        return 1 as ::core::ffi::c_int;
    };
    if window.len() != window_size {
        return 1 as ::core::ffi::c_int;
    }
    if end.len() >= window_size {
        window.copy_from_slice(&end[end.len() - window_size..]);
        *wnext = 0 as ::core::ffi::c_uint;
        *whave = *wsize;
    } else {
        let dist = (window_size - window_next).min(end.len());
        window[window_next..window_next + dist].copy_from_slice(&end[..dist]);
        let copy = end.len() - dist;
        if copy != 0 {
            window[..copy].copy_from_slice(&end[dist..]);
            *wnext = copy as ::core::ffi::c_uint;
            *whave = *wsize;
        } else {
            *wnext = (*wnext).wrapping_add(dist as ::core::ffi::c_uint);
            if *wnext == *wsize {
                *wnext = 0 as ::core::ffi::c_uint;
            }
            if *whave < *wsize {
                *whave = (*whave).wrapping_add(dist as ::core::ffi::c_uint);
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
struct InflateStream<'a>(&'a mut crate::zlib_h::z_stream_s);

impl ::core::ops::Deref for InflateStream<'_> {
    type Target = crate::zlib_h::z_stream_s;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl ::core::ops::DerefMut for InflateStream<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

struct InflateState<'a>(&'a mut crate::src::inflate::inflate_state);

impl ::core::ops::Deref for InflateState<'_> {
    type Target = crate::src::inflate::inflate_state;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl ::core::ops::DerefMut for InflateState<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

/// Returns the bytes most recently consumed from the input buffer.
///
/// `inflate()` keeps its C-compatible cursor as a raw pointer while decoding,
/// but checksum updates only need the bounded portion that has already been
/// consumed.  Keeping that range calculation slice-based avoids rebuilding a
/// raw slice at each header field.
fn inflate_consumed_input(
    input: &[crate::stdlib::Bytef],
    initial_avail: crate::stdlib::uInt,
    remaining_avail: crate::stdlib::uInt,
    count: crate::stdlib::uInt,
) -> &[crate::stdlib::Bytef] {
    let start = initial_avail.saturating_sub(remaining_avail) as usize;
    let end = start.saturating_add(count as usize);
    input.get(start..end).unwrap_or(&[])
}

/// A bounded input cursor used by the ordinary inflate state machine.
///
/// The translated C decoder advanced a raw pointer for every bit refill.
/// Keeping the base slice and its offset together makes those advances and
/// reads bounds-checked; stream pointers are materialized only when committing
/// progress to the ABI stream.
#[derive(Copy, Clone)]
struct InflateInput<'a> {
    bytes: &'a [crate::stdlib::Bytef],
    index: usize,
}

/// Selects the bounded source for the ordinary decoder's match-copy loop.
///
/// A match may refer either to the history window or to bytes already emitted
/// in the current output buffer.  Keeping the source as an index preserves
/// the byte-at-a-time, overlapping-copy behavior without rebuilding an
/// interior raw pointer into either owner.
enum InflateCopySource {
    Window(usize),
    Output(usize),
}

/// Borrowed gzip-header output buffers, validated at the ABI boundary.
///
/// zlib retains the header carrier in the inflater state, but each `inflate`
/// call only needs bounded, caller-owned destinations for the progressive
/// extra, name, and comment fields.  Keeping those destinations as slices
/// prevents the decoder from rebuilding interior raw pointers while it
/// consumes the header.
pub struct InflateHeader<'a> {
    header: &'a mut crate::zlib_h::gz_header_s,
    extra: Option<&'a mut [crate::stdlib::Bytef]>,
    name: Option<&'a mut [crate::stdlib::Bytef]>,
    comment: Option<&'a mut [crate::stdlib::Bytef]>,
}

/// The exported header API keeps caller memory alive across `inflate()`
/// calls.  Keep that ABI-only association outside decoder state: the decoder
/// receives ordinary bounded borrows for the duration of each call, and the
/// state itself never retains a pointer into caller memory.
struct InflateHeaderRegistration {
    state_address: usize,
    header_address: usize,
}

static INFLATE_HEADER_REGISTRATIONS: std::sync::OnceLock<
    std::sync::Mutex<Vec<InflateHeaderRegistration>>,
> = std::sync::OnceLock::new();

fn inflate_header_registrations() -> &'static std::sync::Mutex<Vec<InflateHeaderRegistration>> {
    INFLATE_HEADER_REGISTRATIONS.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

fn inflate_state_address(state: &crate::src::inflate::inflate_state) -> usize {
    core::ptr::from_ref(state).addr()
}

fn register_inflate_header(
    state: &crate::src::inflate::inflate_state,
    header: &crate::zlib_h::gz_header_s,
) -> bool {
    let mut registrations = inflate_header_registrations()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let state_address = inflate_state_address(state);
    if let Some(registration) = registrations
        .iter_mut()
        .find(|registration| registration.state_address == state_address)
    {
        registration.header_address = core::ptr::from_ref(header).addr();
        return true;
    }
    if registrations.try_reserve(1).is_err() {
        return false;
    }
    registrations.push(InflateHeaderRegistration {
        state_address,
        header_address: core::ptr::from_ref(header).addr(),
    });
    true
}

fn registered_inflate_header(state: &crate::src::inflate::inflate_state) -> Option<usize> {
    let registrations = inflate_header_registrations()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let state_address = inflate_state_address(state);
    registrations
        .iter()
        .find(|registration| registration.state_address == state_address)
        .map(|registration| registration.header_address)
}

fn remove_inflate_header(state: &crate::src::inflate::inflate_state) {
    remove_inflate_header_address(inflate_state_address(state));
}

fn remove_inflate_header_address(state_address: usize) {
    let mut registrations = inflate_header_registrations()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(index) = registrations
        .iter()
        .position(|registration| registration.state_address == state_address)
    {
        registrations.swap_remove(index);
    }
}

fn copy_inflate_header_registration(
    source: &crate::src::inflate::inflate_state,
    destination_address: usize,
) {
    let Some(header_address) = registered_inflate_header(source) else {
        return;
    };
    let mut registrations = inflate_header_registrations()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(registration) = registrations
        .iter_mut()
        .find(|registration| registration.state_address == destination_address)
    {
        registration.header_address = header_address;
    } else if registrations.try_reserve(1).is_ok() {
        registrations.push(InflateHeaderRegistration {
            state_address: destination_address,
            header_address,
        });
    }
}

impl<'a> InflateInput<'a> {
    fn new(bytes: &'a [crate::stdlib::Bytef]) -> Self {
        Self { bytes, index: 0 }
    }

    fn at(bytes: &'a [crate::stdlib::Bytef], index: usize) -> Self {
        Self { bytes, index }
    }

    fn wrapping_offset(self, offset: isize) -> Self {
        Self {
            bytes: self.bytes,
            index: self.index.wrapping_add_signed(offset),
        }
    }
}

impl ::core::ops::Deref for InflateInput<'_> {
    type Target = crate::stdlib::Bytef;

    fn deref(&self) -> &Self::Target {
        &self.bytes[self.index]
    }
}

/// Set the ABI-visible diagnostic and retain a borrowed safe view for the
/// current decode call.  Inflater diagnostics are static C strings, so the
/// view does not borrow caller memory.
fn inflate_set_message(
    strm: &mut InflateStream,
    error_message: &mut Option<&'static ::core::ffi::CStr>,
    message: &'static ::core::ffi::CStr,
) {
    strm.msg = message.as_ptr().cast_mut();
    *error_message = Some(message);
}

/// Build one dynamic decode table using the state-owned, bounded arenas.
///
/// Copying the short lens range lets the mutable table cursor and work arena
/// be borrowed independently without recreating the old interior pointers.
pub(crate) fn inflate_table_from_state(
    state: &mut crate::src::inflate::inflate_state,
    type_0: crate::src::inftrees::codetype,
    lens_start: usize,
    count: usize,
    distance_bits: bool,
) -> ::core::ffi::c_int {
    let Some(lens_end) = lens_start.checked_add(count) else {
        return 1;
    };
    let Some(source) = state.lens.get(lens_start..lens_end) else {
        return 1;
    };
    let mut lens = [0; 320];
    let Some(destination) = lens.get_mut(..count) else {
        return 1;
    };
    destination.copy_from_slice(source);
    let bits = if distance_bits {
        &mut state.distbits
    } else {
        &mut state.lenbits
    };
    crate::src::inftrees::inflate_table_slice(
        type_0,
        &lens[..count],
        count,
        &mut state.codes,
        &mut state.next,
        bits,
        &mut state.work,
    )
}

/// Decode one bounded input/output segment using an already-validated
/// inflater state.  Raw ABI state conversion stays at the call boundary;
/// the decoder itself works only with ordinary Rust borrows.
pub fn inflate_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
    mut flush: ::core::ffi::c_int,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    mut header: Option<InflateHeader<'_>>,
    message: &mut Option<&'static ::core::ffi::CStr>,
) -> ::core::ffi::c_int {
    let mut strm = InflateStream(strm);
    *message = None;
    let mut error_message = None;
    let mut next = InflateInput::new(input);
    let mut put = 0usize;
    let mut have: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut in_0: ::core::ffi::c_uint = 0;
    let mut out: ::core::ffi::c_uint = 0;
    let mut copy: ::core::ffi::c_uint = 0;
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
    if input.len() != strm.avail_in as usize || output.len() != strm.avail_out as usize {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !inflate_state_valid(&strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut state = InflateState(state);
    // The exported boundary resolves the caller-owned progressive gzip header
    // once for this call.  The decoder itself only observes this bounded
    // borrow, rather than following the state-held ABI pointer.
    if (*state).mode as ::core::ffi::c_uint
        == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*state).mode = crate::src::inflate::TYPEDO;
    }
    put = 0;
    left = output.len() as ::core::ffi::c_uint;
    next = InflateInput::new(input);
    have = input.len() as ::core::ffi::c_uint;
    hold = (*state).hold;
    bits = (*state).bits;
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
                                                                                            match (*state).mode as ::core::ffi::c_uint {
                                                                                                16180 => {
                                                                                                    if (*state).wrap == 0 as ::core::ffi::c_int {
                                                                                                        (*state).mode = crate::src::inflate::TYPEDO;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if have == 0 as ::core::ffi::c_uint {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            have = have.wrapping_sub(1);
                                                                                                            let c2rust_fresh0 = next;
                                                                                                            next = next.wrapping_offset(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (*c2rust_fresh0 as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        if (*state).wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if (*state).wbits == 0 as ::core::ffi::c_uint {
                                                                                                                (*state).wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            (*state).check = crate::src::crc32::crc32(
                                                                                                                0 as crate::stdlib::uLong,
                                                                                                                &[],
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            (*state).check = crate::src::crc32::crc32(
                                                                                                                (*state).check as crate::stdlib::uLong,
                                                                                                                &hbuf[..2],
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            (*state).mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            if let Some(header) = header.as_mut() {
                                                                                                                header.header.done = -1 as ::core::ffi::c_int;
                                                                                                            }
                                                                                                            if (*state).wrap & 1 as ::core::ffi::c_int == 0
                                                                                                                || (((hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                                                                                                    .wrapping_rem(31 as ::core::ffi::c_ulong) != 0
                                                                                                            {
                                                                                                                inflate_set_message(&mut strm, &mut error_message, c"incorrect header check");
                                                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                inflate_set_message(&mut strm, &mut error_message, c"unknown compression method");
                                                                                                                (*state).mode = crate::src::inflate::BAD;
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
                                                                                                                if (*state).wbits == 0 as ::core::ffi::c_uint {
                                                                                                                    (*state).wbits = len;
                                                                                                                }
                                                                                                                if len > 15 as ::core::ffi::c_uint || len > (*state).wbits {
                                                                                                                    inflate_set_message(&mut strm, &mut error_message, c"invalid window size");
                                                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    (*state).dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    (*state).flags = 0 as ::core::ffi::c_int;
                                                                                                                    (*state).check = crate::src::adler32::adler32_z(
                                                                                                                        1 as crate::stdlib::uLong,
                                                                                                                        &[],
                                                                                                                    ) as ::core::ffi::c_ulong;
                                                                                                                    (*strm).adler = (*state).check as crate::stdlib::uLong;
                                                                                                                    (*state).mode = (if hold & 0x200 as ::core::ffi::c_ulong
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
                                                                                                        next = next.wrapping_offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh1 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    (*state).flags = hold as ::core::ffi::c_int;
                                                                                                    if (*state).flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        inflate_set_message(&mut strm, &mut error_message, c"unknown compression method");
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if (*state).flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        inflate_set_message(&mut strm, &mut error_message, c"unknown header flags set");
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if let Some(header) = header.as_mut() {
                                                                                                            header.header.text = (hold >> 8 as ::core::ffi::c_int
                                                                                                                & 1 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
                                                                                                        }
                                                                                                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                                                                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                        {
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            (*state).check = crate::src::crc32::crc32(
                                                                                                                (*state).check as crate::stdlib::uLong,
                                                                                                                &hbuf[..2],
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                        }
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        (*state).mode = crate::src::inflate::TIME;
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
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh10 = next;
                                                                                                        next = next.wrapping_offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh10 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    (*state).check = (hold >> 24 as ::core::ffi::c_int
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
                                                                                                    (*strm).adler = (*state).check as crate::stdlib::uLong;
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                    (*state).mode = crate::src::inflate::DICT;
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
                                                                                                    hold >>= bits & 7 as ::core::ffi::c_uint;
                                                                                                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh12 = next;
                                                                                                        next = next.wrapping_offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh12 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    if hold & 0xffff as ::core::ffi::c_ulong
                                                                                                        != hold >> 16 as ::core::ffi::c_int
                                                                                                            ^ 0xffff as ::core::ffi::c_ulong
                                                                                                    {
                                                                                                        inflate_set_message(&mut strm, &mut error_message, c"invalid stored block lengths");
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        (*state).length = hold as ::core::ffi::c_uint
                                                                                                            & 0xffff as ::core::ffi::c_uint;
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        (*state).mode = crate::src::inflate::COPY_;
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
                                                                                                        next = next.wrapping_offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh13 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    (*state).nlen = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(257 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    (*state).ndist = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(1 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    (*state).ncode = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(4 as ::core::ffi::c_uint);
                                                                                                    hold >>= 4 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            4 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    if (*state).nlen > 286 as ::core::ffi::c_uint
                                                                                                        || (*state).ndist > 30 as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        inflate_set_message(&mut strm, &mut error_message, c"too many length or distance symbols");
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        (*state).have = 0 as ::core::ffi::c_uint;
                                                                                                        (*state).mode = crate::src::inflate::LENLENS;
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
                                                                                                    output[put] = (*state).length as ::core::ffi::c_uchar;
                                                                                                    put += 1;
                                                                                                    left = left.wrapping_sub(1);
                                                                                                    (*state).mode = crate::src::inflate::LEN;
                                                                                                    continue '_inf_leave;
                                                                                                }
                                                                                                16206 => {
                                                                                                    if (*state).wrap != 0 {
                                                                                                        while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if have == 0 as ::core::ffi::c_uint {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            have = have.wrapping_sub(1);
                                                                                                            let c2rust_fresh33 = next;
                                                                                                            next = next.wrapping_offset(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (*c2rust_fresh33 as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        out = out.wrapping_sub(left);
                                                                                                        (*strm).total_out = (*strm)
                                                                                                            .total_out
                                                                                                            .wrapping_add(out as crate::stdlib::uLong);
                                                                                                        (*state).total = (*state)
                                                                                                            .total
                                                                                                            .wrapping_add(out as ::core::ffi::c_ulong);
                                                                                                        if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0
                                                                                                        {
                                                                                                           (*state).check = (if (*state).flags != 0 {
                                                                                                               crate::src::crc32::crc32(
                                                                                                                   (*state).check as crate::stdlib::uLong,
                                                                                                                    &output[..out as usize],
                                                                                                               )
                                                                                                           } else {
                                                                                                                crate::src::adler32::adler32_z(
                                                                                                                   (*state).check as crate::stdlib::uLong,
                                                                                                                    &output[..out as usize],
                                                                                                               )
                                                                                                            }) as ::core::ffi::c_ulong;
                                                                                                            (*strm).adler = (*state).check as crate::stdlib::uLong;
                                                                                                        }
                                                                                                        out = left;
                                                                                                        if (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                            && (if (*state).flags != 0 {
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
                                                                                                            }) != (*state).check
                                                                                                        {
                                                                                                        inflate_set_message(&mut strm, &mut error_message, c"incorrect data check");
                                                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                        }
                                                                                                    }
                                                                                                    (*state).mode = crate::src::inflate::LENGTH;
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
                                                                                            if (*state).wrap != 0 && (*state).flags != 0 {
                                                                                                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                {
                                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                    have = have.wrapping_sub(1);
                                                                                                    let c2rust_fresh34 = next;
                                                                                                    next = next.wrapping_offset(1);
                                                                                                    hold = hold
                                                                                                        .wrapping_add(
                                                                                                            (*c2rust_fresh34 as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                }
                                                                                                if (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                    && hold
                                                                                                        != (*state).total & 0xffffffff as ::core::ffi::c_ulong
                                                                                                {
                                                                                                    inflate_set_message(&mut strm, &mut error_message, c"incorrect length check");
                                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                                    continue '_inf_leave;
                                                                                                } else {
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                }
                                                                                            }
                                                                                            (*state).mode = crate::src::inflate::DONE;
                                                                                            break 'c_2443;
                                                                                        }
                                                                                        while (*state).have < (*state).ncode {
                                                                                            while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                            {
                                                                                                if have == 0 as ::core::ffi::c_uint {
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                have = have.wrapping_sub(1);
                                                                                                let c2rust_fresh14 = next;
                                                                                                next = next.wrapping_offset(1);
                                                                                                hold = hold
                                                                                                    .wrapping_add(
                                                                                                        (*c2rust_fresh14 as ::core::ffi::c_ulong) << bits,
                                                                                                    );
                                                                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                            }
                                                                                            let c2rust_fresh15 = (*state).have;
                                                                                            (*state).have = (*state).have.wrapping_add(1);
                                                                                            (*state).lens[ORDER[c2rust_fresh15 as usize] as usize] = (hold
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
                                                                                        while (*state).have < 19 as ::core::ffi::c_uint {
                                                                                            let c2rust_fresh16 = (*state).have;
                                                                                            (*state).have = (*state).have.wrapping_add(1);
                                                                                            (*state).lens[ORDER[c2rust_fresh16 as usize] as usize] = 0
                                                                                                as ::core::ffi::c_ushort;
                                                                                        }
                                                                                        (*state)
                                                                                            .next =
                                                                                            0;
                                                                                        (*state).distcode = crate::src::inflate::distance_table::Dynamic(0);
                                                                                        (*state).lencode = crate::src::inflate::length_table::Dynamic(0);
                                                                                        (*state).lenbits = 7 as ::core::ffi::c_uint;
                                                                                        ret = inflate_table_from_state(
                                                                                            &mut state,
                                                                                            crate::src::inftrees::CODES,
                                                                                            0,
                                                                                            19,
                                                                                            false,
                                                                                        );
                                                                                        if ret != 0
                                                                                        {
                                                                                            inflate_set_message(&mut strm, &mut error_message, c"invalid code lengths set");
                                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                                            continue '_inf_leave;
                                                                                        } else {
                                                                                            (*state).have = 0 as ::core::ffi::c_uint;
                                                                                            (*state).mode = crate::src::inflate::CODELENS;
                                                                                            break 's_1689;
                                                                                        }
                                                                                    }
                                                                                    if (*state).havedict == 0 as ::core::ffi::c_int {
                                                                                        (*strm).next_out = output.as_mut_ptr().wrapping_add(put);
                                                                                        (*strm).avail_out = left as crate::stdlib::uInt;
                                                                                        (*strm).next_in = input.as_ptr().wrapping_add(next.index) as *mut crate::stdlib::Bytef;
                                                                                        (*strm).avail_in = have as crate::stdlib::uInt;
                                                                                        (*state).hold = hold;
                                                                                        (*state).bits = bits;
                                                                                        return crate::zlib_h::Z_NEED_DICT;
                                                                                    }
                                                                                    (*state).check = crate::src::adler32::adler32_z(
                                                                                        1 as crate::stdlib::uLong,
                                                                                        &[],
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                    (*strm).adler =
                                                                                        (*state)
                                                                                            .check
                                                                                            as crate::stdlib::uLong;
                                                                                    (*state).mode =
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
                                                                                    next = next.wrapping_offset(1);
                                                                                    hold = hold
                                                                                        .wrapping_add(
                                                                                            (*c2rust_fresh2 as ::core::ffi::c_ulong) << bits,
                                                                                        );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                if let Some(
                                                                                    header,
                                                                                ) =
                                                                                    header.as_mut()
                                                                                {
                                                                                    header.header.time = hold
                                                                                        as crate::stdlib::uLong;
                                                                                }
                                                                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                {
                                                                                    hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                    hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[2 as usize] = (hold >> 16 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[3 as usize] = (hold >> 24 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    (*state).check = crate::src::crc32::crc32(
                                                                                        (*state).check as crate::stdlib::uLong,
                                                                                        &hbuf,
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                }
                                                                                hold = 0 as ::core::ffi::c_ulong;
                                                                                bits = 0 as ::core::ffi::c_uint;
                                                                                (*state).mode = crate::src::inflate::OS;
                                                                                break 's_519;
                                                                            }
                                                                            (*state).mode = crate::src::inflate::COPY_1;
                                                                            break 'c_2356;
                                                                        }
                                                                        while (*state).have
                                                                            < (*state)
                                                                                .nlen
                                                                                .wrapping_add(
                                                                                    (*state).ndist,
                                                                                )
                                                                        {
                                                                            loop {
                                                                                here = (*state).lencode.entry(
                                                                                    &(*state).codes,
                                                                                    (hold as ::core::ffi::c_uint
                                                                                        & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
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
                                                                                next =
                                                                                    next.wrapping_offset(1);
                                                                                hold = hold
                                                                                    .wrapping_add(
                                                                                        (*c2rust_fresh17 as ::core::ffi::c_ulong) << bits,
                                                                                    );
                                                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                            }
                                                                            if (here.val as ::core::ffi::c_int)
                                                                                < 16 as ::core::ffi::c_int
                                                                            {
                                                                                hold >>= here.bits as ::core::ffi::c_int;
                                                                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                let c2rust_fresh18 = (*state).have;
                                                                                (*state).have = (*state).have.wrapping_add(1);
                                                                                (*state).lens[c2rust_fresh18 as usize] = here.val;
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
                                                                                        next = next.wrapping_offset(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (*c2rust_fresh19 as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    if (*state).have == 0 as ::core::ffi::c_uint {
                                                                                        inflate_set_message(&mut strm, &mut error_message, c"invalid bit length repeat");
                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                        break;
                                                                                    } else {
                                                                                        len = (*state)
                                                                                            .lens[(*state).have.wrapping_sub(1 as ::core::ffi::c_uint)
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
                                                                                        next = next.wrapping_offset(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (*c2rust_fresh20 as ::core::ffi::c_ulong) << bits,
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
                                                                                        next = next.wrapping_offset(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (*c2rust_fresh21 as ::core::ffi::c_ulong) << bits,
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
                                                                                if (*state).have.wrapping_add(copy)
                                                                                    > (*state).nlen.wrapping_add((*state).ndist)
                                                                                {
                                                                                    inflate_set_message(&mut strm, &mut error_message, c"invalid bit length repeat");
                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                    break;
                                                                                } else {
                                                                                    loop {
                                                                                        let c2rust_fresh22 = copy;
                                                                                        copy = copy.wrapping_sub(1);
                                                                                        if c2rust_fresh22 == 0 {
                                                                                            break;
                                                                                        }
                                                                                        let c2rust_fresh23 = (*state).have;
                                                                                        (*state).have = (*state).have.wrapping_add(1);
                                                                                        (*state).lens[c2rust_fresh23 as usize] = len
                                                                                            as ::core::ffi::c_ushort;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        if (*state).mode as ::core::ffi::c_uint
                                                                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            continue '_inf_leave;
                                                                        }
                                                                        if (*state).lens[256 as usize] as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            inflate_set_message(&mut strm, &mut error_message, c"invalid code -- missing end-of-block");
                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            (*state).next = 0;
                                                                            (*state).lencode = crate::src::inflate::length_table::Dynamic(0);
                                                                            (*state).lenbits = 9 as ::core::ffi::c_uint;
                                                                            let nlen = (*state).nlen as usize;
                                                                            ret = inflate_table_from_state(
                                                                                &mut state,
                                                                                crate::src::inftrees::LENS,
                                                                                0,
                                                                                nlen,
                                                                                false,
                                                                            );
                                                                            if ret != 0 {
                                                                                inflate_set_message(&mut strm, &mut error_message, c"invalid literal/lengths set");
                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                (*state).distcode = crate::src::inflate::distance_table::Dynamic((*state).next);
                                                                                (*state).distbits = 6 as ::core::ffi::c_uint;
                                                                                let nlen = (*state).nlen as usize;
                                                                                let ndist = (*state).ndist as usize;
                                                                                ret = inflate_table_from_state(
                                                                                    &mut state,
                                                                                    crate::src::inftrees::DISTS,
                                                                                    nlen,
                                                                                    ndist,
                                                                                    true,
                                                                                );
                                                                                if ret != 0 {
                                                                                    inflate_set_message(&mut strm, &mut error_message, c"invalid distances set");
                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                    continue '_inf_leave;
                                                                                } else {
                                                                                    (*state).mode = crate::src::inflate::LEN_;
                                                                                    if flush == crate::zlib_h::Z_TREES {
                                                                                        break '_inf_leave;
                                                                                    } else {
                                                                                        break 'c_2397;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    copy = (*state).length;
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
                                                                        let input_start = input
                                                                            .len()
                                                                            - have as usize;
                                                                        let output_start = output
                                                                            .len()
                                                                            - left as usize;
                                                                        output[output_start..output_start + copy as usize]
                                                                            .copy_from_slice(&input[input_start..input_start + copy as usize]);
                                                                        have =
                                                                            have.wrapping_sub(copy);
                                                                        next = next
                                                                            .wrapping_offset(
                                                                                copy as isize,
                                                                            );
                                                                        left =
                                                                            left.wrapping_sub(copy);
                                                                        put += copy as usize;
                                                                        (*state).length = (*state)
                                                                            .length
                                                                            .wrapping_sub(copy);
                                                                        continue '_inf_leave;
                                                                    } else {
                                                                        (*state).mode = crate::src::inflate::TYPE;
                                                                        continue '_inf_leave;
                                                                    }
                                                                }
                                                                while bits
                                                                    < 16 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_uint
                                                                {
                                                                    if have
                                                                        == 0 as ::core::ffi::c_uint
                                                                    {
                                                                        break '_inf_leave;
                                                                    }
                                                                    have = have.wrapping_sub(1);
                                                                    let c2rust_fresh3 = next;
                                                                    next = next.wrapping_offset(1);
                                                                    hold = hold.wrapping_add(
                                                                        (*c2rust_fresh3
                                                                            as ::core::ffi::c_ulong)
                                                                            << bits,
                                                                    );
                                                                    bits = bits.wrapping_add(
                                                                        8 as ::core::ffi::c_uint,
                                                                    );
                                                                }
                                                                if let Some(header) =
                                                                    header.as_mut()
                                                                {
                                                                    header.header.xflags = (hold
                                                                        & 0xff
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int;
                                                                    header.header.os = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_int;
                                                                }
                                                                if (*state).flags
                                                                    & 0x200 as ::core::ffi::c_int
                                                                    != 0
                                                                    && (*state).wrap
                                                                        & 4 as ::core::ffi::c_int
                                                                        != 0
                                                                {
                                                                    hbuf[0 as usize] = hold
                                                                        as ::core::ffi::c_uchar;
                                                                    hbuf[1 as usize] = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_uchar;
                                                                    (*state).check = crate::src::crc32::crc32(
                                                                        (*state).check as crate::stdlib::uLong,
                                                                        &hbuf[..2],
                                                                    ) as ::core::ffi::c_ulong;
                                                                }
                                                                hold = 0 as ::core::ffi::c_ulong;
                                                                bits = 0 as ::core::ffi::c_uint;
                                                                (*state).mode =
                                                                    crate::src::inflate::EXLEN;
                                                                break 'c_2317;
                                                            }
                                                            if flush == crate::zlib_h::Z_BLOCK
                                                                || flush == crate::zlib_h::Z_TREES
                                                            {
                                                                break '_inf_leave;
                                                            } else {
                                                                break 'c_2340;
                                                            }
                                                        }
                                                        ret = crate::zlib_h::Z_STREAM_END;
                                                        break '_inf_leave;
                                                    }
                                                    if (*state).last != 0 {
                                                        hold >>= bits & 7 as ::core::ffi::c_uint;
                                                        bits = bits.wrapping_sub(
                                                            bits & 7 as ::core::ffi::c_uint,
                                                        );
                                                        (*state).mode = crate::src::inflate::CHECK;
                                                        continue '_inf_leave;
                                                    } else {
                                                        while bits
                                                            < 3 as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                        {
                                                            if have == 0 as ::core::ffi::c_uint {
                                                                break '_inf_leave;
                                                            }
                                                            have = have.wrapping_sub(1);
                                                            let c2rust_fresh11 = next;
                                                            next = next.wrapping_offset(1);
                                                            hold = hold.wrapping_add(
                                                                (*c2rust_fresh11
                                                                    as ::core::ffi::c_ulong)
                                                                    << bits,
                                                            );
                                                            bits = bits.wrapping_add(
                                                                8 as ::core::ffi::c_uint,
                                                            );
                                                        }
                                                        (*state).last = (hold
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
                                                                (*state).mode =
                                                                    crate::src::inflate::STORED;
                                                            }
                                                            1 => {
                                                                let state = &mut *state;
                                                                crate::src::inftrees::inflate_fixed(
                                                                    &mut state.lencode,
                                                                    &mut state.lenbits,
                                                                    &mut state.distcode,
                                                                    &mut state.distbits,
                                                                );
                                                                state.mode =
                                                                    crate::src::inflate::LEN_;
                                                                if flush == crate::zlib_h::Z_TREES {
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
                                                                (*state).mode =
                                                                    crate::src::inflate::TABLE;
                                                            }
                                                            _ => {
                                                                inflate_set_message(
                                                                    &mut strm,
                                                                    &mut error_message,
                                                                    c"invalid block type",
                                                                );
                                                                (*state).mode =
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
                                                if (*state).flags & 0x400 as ::core::ffi::c_int != 0
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
                                                        next = next.wrapping_offset(1);
                                                        hold = hold.wrapping_add(
                                                            (*c2rust_fresh4
                                                                as ::core::ffi::c_ulong)
                                                                << bits,
                                                        );
                                                        bits = bits
                                                            .wrapping_add(8 as ::core::ffi::c_uint);
                                                    }
                                                    (*state).length = hold as ::core::ffi::c_uint;
                                                    if let Some(header) = header.as_mut() {
                                                        header.header.extra_len = hold
                                                            as ::core::ffi::c_uint
                                                            as crate::stdlib::uInt;
                                                    }
                                                    if (*state).flags & 0x200 as ::core::ffi::c_int
                                                        != 0
                                                        && (*state).wrap & 4 as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        hbuf[0 as usize] =
                                                            hold as ::core::ffi::c_uchar;
                                                        hbuf[1 as usize] = (hold
                                                            >> 8 as ::core::ffi::c_int)
                                                            as ::core::ffi::c_uchar;
                                                        (*state).check = crate::src::crc32::crc32(
                                                            (*state).check as crate::stdlib::uLong,
                                                            &hbuf[..2],
                                                        )
                                                            as ::core::ffi::c_ulong;
                                                    }
                                                    hold = 0 as ::core::ffi::c_ulong;
                                                    bits = 0 as ::core::ffi::c_uint;
                                                } else if let Some(header) = header.as_mut() {
                                                    header.header.extra = ::core::ptr::null_mut::<
                                                        crate::stdlib::Bytef,
                                                    >(
                                                    );
                                                }
                                                (*state).mode = crate::src::inflate::EXTRA;
                                                break 'c_2319;
                                            }
                                            (*state).mode = crate::src::inflate::LEN;
                                        }
                                        // The regular decoder below uses bounded slice cursors.
                                        // It is also the semantic fallback for the former raw
                                        // fast-path, so keeping one path avoids handing the ABI
                                        // stream back to the pointer-based implementation here.
                                        (*state).back = 0 as ::core::ffi::c_int;
                                        loop {
                                            here = (*state).lencode.entry(
                                                &(*state).codes,
                                                (hold as ::core::ffi::c_uint
                                                    & ((1 as ::core::ffi::c_uint)
                                                        << (*state).lenbits)
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
                                            let c2rust_fresh24 = next;
                                            next = next.wrapping_offset(1);
                                            hold = hold.wrapping_add(
                                                (*c2rust_fresh24 as ::core::ffi::c_ulong) << bits,
                                            );
                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                        }
                                        if here.op as ::core::ffi::c_int != 0
                                            && here.op as ::core::ffi::c_int
                                                & 0xf0 as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                        {
                                            last = here;
                                            loop {
                                                here = (*state).lencode.entry(
                                                    &(*state).codes,
                                                    (last.val as ::core::ffi::c_uint).wrapping_add(
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
                                                next = next.wrapping_offset(1);
                                                hold = hold.wrapping_add(
                                                    (*c2rust_fresh25 as ::core::ffi::c_ulong)
                                                        << bits,
                                                );
                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                            }
                                            hold >>= last.bits as ::core::ffi::c_int;
                                            bits =
                                                bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                                            (*state).back += last.bits as ::core::ffi::c_int;
                                        }
                                        hold >>= here.bits as ::core::ffi::c_int;
                                        bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                        (*state).back += here.bits as ::core::ffi::c_int;
                                        (*state).length = here.val as ::core::ffi::c_uint;
                                        if here.op as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                        {
                                            (*state).mode = crate::src::inflate::LIT;
                                            continue '_inf_leave;
                                        } else if here.op as ::core::ffi::c_int
                                            & 32 as ::core::ffi::c_int
                                            != 0
                                        {
                                            (*state).back = -1 as ::core::ffi::c_int;
                                            (*state).mode = crate::src::inflate::TYPE;
                                            continue '_inf_leave;
                                        } else if here.op as ::core::ffi::c_int
                                            & 64 as ::core::ffi::c_int
                                            != 0
                                        {
                                            inflate_set_message(
                                                &mut strm,
                                                &mut error_message,
                                                c"invalid literal/length code",
                                            );
                                            (*state).mode = crate::src::inflate::BAD;
                                            continue '_inf_leave;
                                        } else {
                                            (*state).extra = here.op as ::core::ffi::c_uint
                                                & 15 as ::core::ffi::c_uint;
                                            (*state).mode = crate::src::inflate::LENEXT;
                                            break 'c_2410;
                                        }
                                    }
                                    if (*state).flags & 0x400 as ::core::ffi::c_int != 0 {
                                        copy = (*state).length;
                                        if copy > have {
                                            copy = have;
                                        }
                                        if copy != 0 {
                                            if let Some(header) = header.as_mut() {
                                                if let Some(extra) = header.extra.as_deref_mut() {
                                                    len = (header.header.extra_len
                                                        as ::core::ffi::c_uint)
                                                        .wrapping_sub((*state).length);
                                                    if len < extra.len() as ::core::ffi::c_uint {
                                                        let extra_copy = (copy as usize)
                                                            .min(extra.len() - len as usize);
                                                        let input_start =
                                                            input.len() - have as usize;
                                                        extra[len as usize
                                                            ..len as usize + extra_copy]
                                                            .copy_from_slice(
                                                                &input[input_start
                                                                    ..input_start + extra_copy],
                                                            );
                                                    }
                                                }
                                            }
                                            if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                            {
                                                (*state).check = crate::src::crc32::crc32(
                                                    (*state).check as crate::stdlib::uLong,
                                                    &input[input.len() - have as usize
                                                        ..input.len() - have as usize
                                                            + copy as usize],
                                                )
                                                    as ::core::ffi::c_ulong;
                                            }
                                            have = have.wrapping_sub(copy);
                                            next = next.wrapping_offset(copy as isize);
                                            (*state).length = (*state).length.wrapping_sub(copy);
                                        }
                                        if (*state).length != 0 {
                                            break '_inf_leave;
                                        }
                                    }
                                    (*state).length = 0 as ::core::ffi::c_uint;
                                    (*state).mode = crate::src::inflate::NAME;
                                    break 'c_2322;
                                }
                                if (*state).extra != 0 {
                                    while bits < (*state).extra {
                                        if have == 0 as ::core::ffi::c_uint {
                                            break '_inf_leave;
                                        }
                                        have = have.wrapping_sub(1);
                                        let c2rust_fresh26 = next;
                                        next = next.wrapping_offset(1);
                                        hold = hold.wrapping_add(
                                            (*c2rust_fresh26 as ::core::ffi::c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    (*state).length = (*state).length.wrapping_add(
                                        hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                    );
                                    hold >>= (*state).extra;
                                    bits = bits.wrapping_sub((*state).extra);
                                    (*state).back = ((*state).back as ::core::ffi::c_uint)
                                        .wrapping_add((*state).extra)
                                        as ::core::ffi::c_int;
                                }
                                (*state).was = (*state).length;
                                (*state).mode = crate::src::inflate::DIST;
                                break 's_2462;
                            }
                            if (*state).flags & 0x800 as ::core::ffi::c_int != 0 {
                                if have == 0 as ::core::ffi::c_uint {
                                    break '_inf_leave;
                                }
                                copy = 0 as ::core::ffi::c_uint;
                                loop {
                                    let c2rust_fresh5 = copy;
                                    copy = copy.wrapping_add(1);
                                    len = *next.wrapping_offset(c2rust_fresh5 as isize)
                                        as ::core::ffi::c_uint;
                                    if let Some(header) = header.as_mut() {
                                        if let Some(name) = header.name.as_deref_mut() {
                                            if (*state).length < name.len() as ::core::ffi::c_uint {
                                                let c2rust_fresh6 = (*state).length;
                                                (*state).length = (*state).length.wrapping_add(1);
                                                name[c2rust_fresh6 as usize] =
                                                    len as crate::stdlib::Bytef;
                                            }
                                        }
                                    }
                                    if !(len != 0 && copy < have) {
                                        break;
                                    }
                                }
                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                {
                                    (*state).check = crate::src::crc32::crc32(
                                        (*state).check as crate::stdlib::uLong,
                                        inflate_consumed_input(input, in_0, have, copy),
                                    )
                                        as ::core::ffi::c_ulong;
                                }
                                have = have.wrapping_sub(copy);
                                next = next.wrapping_offset(copy as isize);
                                if len != 0 {
                                    break '_inf_leave;
                                }
                            } else if let Some(header) = header.as_mut() {
                                header.header.name =
                                    ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                            }
                            (*state).length = 0 as ::core::ffi::c_uint;
                            (*state).mode = crate::src::inflate::COMMENT;
                            break 'c_2325;
                        }
                        loop {
                            here = (*state).distcode.entry(
                                &(*state).codes,
                                (hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint) << (*state).distbits)
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
                            next = next.wrapping_offset(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh27 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            last = here;
                            loop {
                                here = (*state).distcode.entry(
                                    &(*state).codes,
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
                                next = next.wrapping_offset(1);
                                hold = hold.wrapping_add(
                                    (*c2rust_fresh28 as ::core::ffi::c_ulong) << bits,
                                );
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            hold >>= last.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                            (*state).back += last.bits as ::core::ffi::c_int;
                        }
                        hold >>= here.bits as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                        (*state).back += here.bits as ::core::ffi::c_int;
                        if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                            inflate_set_message(
                                &mut strm,
                                &mut error_message,
                                c"invalid distance code",
                            );
                            (*state).mode = crate::src::inflate::BAD;
                            continue '_inf_leave;
                        } else {
                            (*state).offset = here.val as ::core::ffi::c_uint;
                            (*state).extra =
                                here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                            (*state).mode = crate::src::inflate::DISTEXT;
                            break 'c_2422;
                        }
                    }
                    if (*state).flags & 0x1000 as ::core::ffi::c_int != 0 {
                        if have == 0 as ::core::ffi::c_uint {
                            break '_inf_leave;
                        }
                        copy = 0 as ::core::ffi::c_uint;
                        loop {
                            let c2rust_fresh7 = copy;
                            copy = copy.wrapping_add(1);
                            len = *next.wrapping_offset(c2rust_fresh7 as isize)
                                as ::core::ffi::c_uint;
                            if let Some(header) = header.as_mut() {
                                if let Some(comment) = header.comment.as_deref_mut() {
                                    if (*state).length < comment.len() as ::core::ffi::c_uint {
                                        let c2rust_fresh8 = (*state).length;
                                        (*state).length = (*state).length.wrapping_add(1);
                                        comment[c2rust_fresh8 as usize] =
                                            len as crate::stdlib::Bytef;
                                    }
                                }
                            }
                            if !(len != 0 && copy < have) {
                                break;
                            }
                        }
                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                        {
                            (*state).check = crate::src::crc32::crc32(
                                (*state).check as crate::stdlib::uLong,
                                inflate_consumed_input(input, in_0, have, copy),
                            ) as ::core::ffi::c_ulong;
                        }
                        have = have.wrapping_sub(copy);
                        next = next.wrapping_offset(copy as isize);
                        if len != 0 {
                            break '_inf_leave;
                        }
                    } else if let Some(header) = header.as_mut() {
                        header.header.comment = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                    }
                    (*state).mode = crate::src::inflate::HCRC;
                    break 'c_2327;
                }
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as ::core::ffi::c_uint {
                            break '_inf_leave;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh29 = next;
                        next = next.wrapping_offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh29 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    (*state).offset = (*state).offset.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state).back = ((*state).back as ::core::ffi::c_uint)
                        .wrapping_add((*state).extra)
                        as ::core::ffi::c_int;
                }
                (*state).mode = crate::src::inflate::MATCH;
                break 'c_2425;
            }
            if (*state).flags & 0x200 as ::core::ffi::c_int != 0 {
                while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break '_inf_leave;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh9 = next;
                    next = next.wrapping_offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh9 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if (*state).wrap & 4 as ::core::ffi::c_int != 0
                    && hold != (*state).check & 0xffff as ::core::ffi::c_ulong
                {
                    inflate_set_message(&mut strm, &mut error_message, c"header crc mismatch");
                    (*state).mode = crate::src::inflate::BAD;
                    continue '_inf_leave;
                } else {
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                }
            }
            if let Some(header) = header.as_mut() {
                header.header.hcrc =
                    (*state).flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                header.header.done = 1 as ::core::ffi::c_int;
            }
            (*state).check =
                crate::src::crc32::crc32(0 as crate::stdlib::uLong, &[]) as ::core::ffi::c_ulong;
            (*strm).adler = (*state).check as crate::stdlib::uLong;
            (*state).mode = crate::src::inflate::TYPE;
            continue '_inf_leave;
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        copy = out.wrapping_sub(left);
        let source = if (*state).offset > copy {
            copy = (*state).offset.wrapping_sub(copy);
            if copy > (*state).whave {
                if (*state).sane != 0 {
                    inflate_set_message(
                        &mut strm,
                        &mut error_message,
                        c"invalid distance too far back",
                    );
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                }
            }
            let start = if copy > (*state).wnext {
                copy = copy.wrapping_sub((*state).wnext);
                (*state).wsize.wrapping_sub(copy) as usize
            } else {
                (*state).wnext.wrapping_sub(copy) as usize
            };
            if copy > (*state).length {
                copy = (*state).length;
            }
            InflateCopySource::Window(start)
        } else {
            copy = (*state).length;
            InflateCopySource::Output(put - (*state).offset as usize)
        };
        if copy > left {
            copy = left;
        }
        left = left.wrapping_sub(copy);
        (*state).length = (*state).length.wrapping_sub(copy);
        let copy_len = copy as usize;
        for copied in 0..copy_len {
            let byte = match source {
                InflateCopySource::Window(start) => (*state)
                    .window
                    .as_deref()
                    .and_then(|window| window.get(start + copied))
                    .copied()
                    .expect("inflate window source"),
                InflateCopySource::Output(start) => output[start + copied],
            };
            output[put + copied] = byte;
        }
        put += copy_len;
        if (*state).length == 0 as ::core::ffi::c_uint {
            (*state).mode = crate::src::inflate::LEN;
        }
    }
    (*strm).next_out = output.as_mut_ptr().wrapping_add(put);
    (*strm).avail_out = left as crate::stdlib::uInt;
    (*strm).next_in = input.as_ptr().wrapping_add(next.index) as *mut crate::stdlib::Bytef;
    (*strm).avail_in = have as crate::stdlib::uInt;
    (*state).hold = hold;
    (*state).bits = bits;
    if (*state).wsize != 0
        || out != (*strm).avail_out
            && ((*state).mode as ::core::ffi::c_uint)
                < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
            && (((*state).mode as ::core::ffi::c_uint)
                < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                || flush != crate::zlib_h::Z_FINISH)
    {
        let copied = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint) as usize;
        let state = &mut *state;
        let (window, wbits, wsize, wnext, whave) = (
            &mut state.window,
            state.wbits,
            &mut state.wsize,
            &mut state.wnext,
            &mut state.whave,
        );
        if updatewindow(
            window,
            wbits,
            wsize,
            wnext,
            whave,
            &output[put - copied..put],
        ) != 0
        {
            (*state).mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in as ::core::ffi::c_uint);
    out = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
    (*strm).total_in = (*strm).total_in.wrapping_add(in_0 as crate::stdlib::uLong);
    (*strm).total_out = (*strm).total_out.wrapping_add(out as crate::stdlib::uLong);
    (*state).total = (*state).total.wrapping_add(out as ::core::ffi::c_ulong);
    if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
        (*state).check = (if (*state).flags != 0 {
            crate::src::crc32::crc32(
                (*state).check as crate::stdlib::uLong,
                &output[..out as usize],
            )
        } else {
            crate::src::adler32::adler32_z(
                (*state).check as crate::stdlib::uLong,
                &output[..out as usize],
            )
        }) as ::core::ffi::c_ulong;
        (*strm).adler = (*state).check as crate::stdlib::uLong;
    }
    (*strm).data_type = (*state).bits as ::core::ffi::c_int
        + (if (*state).last != 0 {
            64 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
        + (if (*state).mode as ::core::ffi::c_uint
            == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            128 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
        + (if (*state).mode as ::core::ffi::c_uint
            == crate::src::inflate::LEN_ as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*state).mode as ::core::ffi::c_uint
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
    *message = error_message;
    return ret;
}
#[export_name = "inflate"]
pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if strm.next_out.is_null() || (strm.next_in.is_null() && strm.avail_in != 0) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let input = if strm.avail_in == 0 {
        &[]
    } else {
        // `next_in` was checked above and the caller owns the advertised
        // range for the duration of this FFI call.
        ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
    };
    // zlib requires `next_out` even for a zero-sized output range.
    let output = ::core::slice::from_raw_parts_mut(strm.next_out, strm.avail_out as usize);
    // `inflateGetHeader()` retains this caller-owned pointer in the opaque
    // stream state.  Resolve its three writable ranges at the ABI boundary so
    // the decoder only sees bounded slices.
    let state = strm.state.cast::<crate::src::inflate::inflate_state>();
    let Some(state) = (unsafe { state.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let header = state
        .head
        .then(|| registered_inflate_header(state))
        .flatten()
        .map(|address| unsafe {
            let header =
                core::ptr::with_exposed_provenance_mut::<crate::zlib_h::gz_header_s>(address);
            let header = &mut *header;
            let extra = (!header.extra.is_null()).then(|| {
                ::core::slice::from_raw_parts_mut(header.extra, header.extra_max as usize)
            });
            let name = (!header.name.is_null())
                .then(|| ::core::slice::from_raw_parts_mut(header.name, header.name_max as usize));
            let comment = (!header.comment.is_null()).then(|| {
                ::core::slice::from_raw_parts_mut(header.comment, header.comm_max as usize)
            });
            InflateHeader {
                header,
                extra,
                name,
                comment,
            }
        });
    let mut message = None;
    inflate_impl(strm, state, flush, input, output, header, &mut message)
}
/// Release the Rust-owned portions of an initialized inflater state.
///
/// The state allocation itself belongs to the ABI allocator that created it,
/// so the exported boundary returns that allocation only after this safe
/// cleanup has released the history-window owner.
pub struct InflateEndState<'a> {
    mode: crate::src::inflate::inflate_mode,
    window: &'a mut Option<Vec<crate::stdlib::Bytef>>,
    header_registered: &'a mut bool,
    state_address: usize,
}

fn inflate_end_state(state: &mut crate::src::inflate::inflate_state) -> InflateEndState<'_> {
    let state_address = inflate_state_address(state);
    InflateEndState {
        mode: state.mode,
        window: &mut state.window,
        header_registered: &mut state.head,
        state_address,
    }
}

pub fn inflateEnd(mut state: InflateEndState<'_>) -> ::core::ffi::c_int {
    if state.mode < crate::src::inflate::HEAD || state.mode > crate::src::inflate::SYNC {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    *state.window = None;
    if *state.header_registered {
        remove_inflate_header_address(state.state_address);
        *state.header_registered = false;
    }
    crate::zlib_h::Z_OK
}

/// Validate the allocator pairing before safely releasing state-owned data.
fn inflate_end_impl(state: InflateEndState<'_>, allocators_present: bool) -> ::core::ffi::c_int {
    if !allocators_present {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflateEnd(state)
}

fn clear_inflate_state(strm: &mut crate::zlib_h::z_stream_s) {
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
}

#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = strm
        .state
        .cast::<crate::src::inflate::inflate_state>()
        .as_mut()
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let end = inflate_end_impl(
        inflate_end_state(state),
        inflate_stream_has_state_allocation(strm),
    );
    if end != crate::zlib_h::Z_STREAM_ERROR {
        // The state pointer always identifies a Rust owner.  A custom
        // allocator additionally has an opaque token recorded alongside that
        // owner; only that token is returned to the paired callback.
        let allocation = release_inflate_state_owner(core::ptr::from_mut(state).addr());
        if let Some(allocation) = allocation {
            let allocation =
                core::ptr::with_exposed_provenance_mut::<core::ffi::c_void>(allocation);
            if let Some(zfree) = strm.zfree {
                zfree(strm.opaque, allocation);
            }
        }
        clear_inflate_state(strm);
    }
    end
}
pub fn inflateGetDictionary(
    whave: crate::stdlib::uInt,
    wnext: crate::stdlib::uInt,
    window: Option<&[crate::stdlib::Bytef]>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dictLength: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    let whave = whave as usize;
    let wnext = wnext as usize;

    if wnext > whave {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    if let Some(dictionary) = dictionary {
        let Some(window) = window else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let split = whave - wnext;
        if dictionary.len() < whave || window.len() < whave {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        dictionary[..split].copy_from_slice(&window[wnext..whave]);
        dictionary[split..whave].copy_from_slice(&window[..wnext]);
    }
    if let Some(dictLength) = dictLength {
        *dictLength = whave as crate::stdlib::uInt;
    }
    crate::zlib_h::Z_OK
}
#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &*strm;
    let state = strm.state as *const crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*state;
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let whave = state.whave as usize;
    let wnext = state.wnext as usize;
    if wnext > whave || whave > state.wsize as usize {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let window = if whave == 0 {
        None
    } else {
        let Some(window) = state.window.as_deref() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        Some(window)
    };
    let dictionary = if whave == 0 || dictionary.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(dictionary, whave))
    };
    let dictLength = if dictLength.is_null() {
        None
    } else {
        Some(&mut *dictLength)
    };

    inflateGetDictionary(state.whave, state.wnext, window, dictionary, dictLength)
}
/// The safe pieces of an ABI stream needed to apply a preset dictionary.
///
/// The FFI wrapper resolves the opaque state pointer once.  This carrier
/// deliberately retains only ordinary Rust references and scalar validation
/// inputs, so dictionary handling never needs to follow an ABI pointer.
struct InflateDictionaryTarget<'a> {
    allocators_present: bool,
    state: Option<&'a mut crate::src::inflate::inflate_state>,
}

fn inflate_set_dictionary(
    target: InflateDictionaryTarget<'_>,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if crate::stdlib::uInt::try_from(dictionary.len()).is_err() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = target.state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !target.allocators_present || !inflate_state_mode_valid(state) {
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
        // `adler32(0, NULL, 0)` produces the initial Adler value of one.
        let dictid = crate::src::adler32::adler32_z(1, dictionary) as ::core::ffi::c_ulong;
        if dictid != state.check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }
    if updatewindow(
        &mut state.window,
        state.wbits,
        &mut state.wsize,
        &mut state.wnext,
        &mut state.whave,
        dictionary,
    ) != 0
    {
        state.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    state.havedict = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSetDictionary"]

pub unsafe extern "C" fn inflateSetDictionary_ffi(
    strm: crate::zlib_h::z_streamp,
    dictionary: *const crate::stdlib::Bytef,
    dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Ok(dict_length) = usize::try_from(dictLength) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let dictionary = if dict_length == 0 {
        &[]
    } else {
        if dictionary.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        core::slice::from_raw_parts(dictionary, dict_length)
    };
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let target = InflateDictionaryTarget {
        allocators_present: inflate_stream_has_state_allocation(strm),
        state: strm
            .state
            .cast::<crate::src::inflate::inflate_state>()
            .as_mut(),
    };
    inflate_set_dictionary(target, dictionary)
}
/// Attach the caller's header result to an initialized gzip-capable inflater.
///
/// The ABI wrapper has already turned the three opaque pointers into borrows.
/// Keeping validation and state mutation here lets the exported entry point
/// remain only a conversion-and-dispatch boundary.
fn inflate_get_header_impl(
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
    if !register_inflate_header(state, head) {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    state.head = true;
    head.done = 0 as ::core::ffi::c_int;
    crate::zlib_h::Z_OK
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = strm
        .state
        .cast::<crate::src::inflate::inflate_state>()
        .as_mut()
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(head) = head.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_get_header_impl(strm, state, head)
}
fn syncsearch(have: &mut ::core::ffi::c_uint, buf: &[u8]) -> ::core::ffi::c_uint {
    let mut got = *have;
    let mut next = 0usize;
    while next < buf.len() && got < 4 {
        if buf[next] as ::core::ffi::c_int == if got < 2 { 0 } else { 0xff } {
            got = got.wrapping_add(1);
        } else if buf[next] != 0 {
            got = 0;
        } else {
            got = 4u32.wrapping_sub(got);
        }
        next += 1;
    }
    *have = got;
    next as ::core::ffi::c_uint
}

/// Search the current bit buffer and supplied input for an empty stored
/// block, then reset the inflater after finding one.  The ABI wrapper owns
/// conversion of the caller's pointer-and-length input into `input`; this
/// implementation only advances ordinary counters and state.
fn inflate_sync_impl(
    stream: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
    input: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_uint = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut in_0: ::core::ffi::c_ulong = 0;
    let mut out: ::core::ffi::c_ulong = 0;
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    if inflate_validate_state(stream, state).is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if stream.avail_in == 0 as crate::stdlib::uInt && state.bits < 8 as ::core::ffi::c_uint {
        return crate::zlib_h::Z_BUF_ERROR;
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
    stream.avail_in = stream.avail_in.wrapping_sub(len);
    if len != 0 {
        stream.next_in = stream.next_in.wrapping_add(len as usize);
    }
    stream.total_in = stream.total_in.wrapping_add(len as crate::stdlib::uLong);
    if state.have != 4 as ::core::ffi::c_uint {
        return crate::zlib_h::Z_DATA_ERROR;
    }
    if state.flags == -1 as ::core::ffi::c_int {
        state.wrap = 0 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    flags = state.flags;
    in_0 = stream.total_in as ::core::ffi::c_ulong;
    out = stream.total_out as ::core::ffi::c_ulong;
    inflate_reset_gzip(stream, state);
    stream.total_in = in_0 as crate::stdlib::uLong;
    stream.total_out = out as crate::stdlib::uLong;
    state.flags = flags;
    state.mode = crate::src::inflate::TYPE;
    crate::zlib_h::Z_OK
}
#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(stream) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = stream
        .state
        .cast::<crate::src::inflate::inflate_state>()
        .as_mut()
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let input_start = stream.next_in;
    let input_len = stream.avail_in as usize;
    if input_len > isize::MAX as usize || (input_len != 0 && input_start.is_null()) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let input = if input_len == 0 {
        &[]
    } else {
        // `next_in` is a caller-owned range that is valid for this FFI call.
        ::core::slice::from_raw_parts(input_start, input_len)
    };
    inflate_sync_impl(stream, state, input)
}
fn inflate_sync_point_impl(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_int {
    (state.mode as ::core::ffi::c_uint
        == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint
        && state.bits == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
}

fn inflate_sync_point_from_stream(
    strm: &crate::zlib_h::z_stream_s,
    state: &crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    let Some(state) = inflate_mark_state(strm, state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_sync_point_impl(state)
}
#[export_name = "inflateSyncPoint"]

pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_ref() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = (unsafe {
        strm.state
            .cast::<crate::src::inflate::inflate_state>()
            .as_ref()
    }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_sync_point_from_stream(strm, state)
}
/// Safe input prepared after the ABI boundary has resolved the source state.
/// Allocator validation stays here, before any state storage is requested.
struct InflateCopyRequest<'a> {
    state: &'a crate::src::inflate::inflate_state,
    allocators_present: bool,
}

/// Validate and clone the decoder state without touching ABI allocator
/// storage.  Keeping this separate means an allocation failure cannot leave a
/// partially installed destination stream behind.
fn inflate_copy_impl(
    source: InflateCopyRequest<'_>,
) -> Result<crate::src::inflate::inflate_state, ::core::ffi::c_int> {
    if !source.allocators_present || !inflate_state_mode_valid(source.state) {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok(source.state.clone())
}

#[export_name = "inflateCopy"]

pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if dest.is_null() || source.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = *source;
    let state = source.state as *const crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*state;
    let copied_state = match inflate_copy_impl(InflateCopyRequest {
        state,
        allocators_present: inflate_stream_has_state_allocation(&source),
    }) {
        Ok(copy) => copy,
        Err(error) => return error,
    };
    inflate_allocate_state(
        &mut *dest,
        copied_state,
        InflateStateInstallation::Copy {
            source_stream: &source,
            source_state: state,
        },
    )
}
fn inflate_undermine_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    let Some(state) = inflate_validate_state(strm, state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    state.sane = 1;
    crate::zlib_h::Z_DATA_ERROR
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = strm
        .state
        .cast::<crate::src::inflate::inflate_state>()
        .as_mut()
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_undermine_impl(strm, state)
}
fn inflate_validate_state<'a>(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &'a mut crate::src::inflate::inflate_state,
) -> Option<&'a mut crate::src::inflate::inflate_state> {
    if !inflate_stream_has_state_allocation(strm) {
        return None;
    }
    inflate_state_mode_valid(state).then_some(state)
}

fn inflate_validate_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = inflate_validate_state(strm, state) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if check != 0 && state.wrap != 0 {
        state.wrap |= 4 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    crate::zlib_h::Z_OK
}
#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(state) = strm
        .state
        .cast::<crate::src::inflate::inflate_state>()
        .as_mut()
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_validate_impl(strm, state, check)
}
fn inflate_mark_impl(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_long {
    ((state.back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int)
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
        }) as ::core::ffi::c_long
}

/// Recover the initialized inflate state after the FFI boundary has checked
/// and borrowed the stream itself.  State validation remains an
/// implementation concern, keeping the exported mark accessor to one input
/// conversion and one safe dispatch.
fn inflate_mark_state<'a>(
    strm: &crate::zlib_h::z_stream_s,
    state: &'a crate::src::inflate::inflate_state,
) -> Option<&'a crate::src::inflate::inflate_state> {
    if !inflate_stream_has_state_allocation(strm) {
        return None;
    }
    inflate_state_mode_valid(state).then_some(state)
}

#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    let invalid_mark = -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    let Some(strm) = (unsafe { strm.as_ref() }) else {
        return invalid_mark;
    };
    let Some(state) = (unsafe {
        strm.state
            .cast::<crate::src::inflate::inflate_state>()
            .as_ref()
    }) else {
        return invalid_mark;
    };
    let Some(state) = inflate_mark_state(strm, state) else {
        return invalid_mark;
    };
    inflate_mark_impl(state)
}
fn inflate_codes_used_impl(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_ulong {
    state.next as ::core::ffi::c_ulong
}

fn inflate_codes_used_from_stream(
    strm: &crate::zlib_h::z_stream_s,
    state: &crate::src::inflate::inflate_state,
) -> ::core::ffi::c_ulong {
    let Some(state) = inflate_mark_state(strm, state) else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    inflate_codes_used_impl(state)
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    let Some(strm) = (unsafe { strm.as_ref() }) else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    let Some(state) = (unsafe {
        strm.state
            .cast::<crate::src::inflate::inflate_state>()
            .as_ref()
    }) else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    inflate_codes_used_from_stream(strm, state)
}
