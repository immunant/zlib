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

// Inflater diagnostics are static zlib strings. Keep their storage named so
// internal users can carry the message as bytes without traversing a raw C
// string that the inflater itself produced.
const INFLATE_MSG_INCORRECT_HEADER_CHECK: &[u8] = b"incorrect header check\0";
const INFLATE_MSG_UNKNOWN_COMPRESSION_METHOD: &[u8] = b"unknown compression method\0";
const INFLATE_MSG_INVALID_WINDOW_SIZE: &[u8] = b"invalid window size\0";
const INFLATE_MSG_UNKNOWN_HEADER_FLAGS: &[u8] = b"unknown header flags set\0";
const INFLATE_MSG_INVALID_STORED_BLOCK_LENGTHS: &[u8] = b"invalid stored block lengths\0";
const INFLATE_MSG_TOO_MANY_SYMBOLS: &[u8] = b"too many length or distance symbols\0";
const INFLATE_MSG_INCORRECT_DATA_CHECK: &[u8] = b"incorrect data check\0";
const INFLATE_MSG_INCORRECT_LENGTH_CHECK: &[u8] = b"incorrect length check\0";
const INFLATE_MSG_INVALID_CODE_LENGTHS: &[u8] = b"invalid code lengths set\0";
const INFLATE_MSG_INVALID_BIT_LENGTH_REPEAT: &[u8] = b"invalid bit length repeat\0";
const INFLATE_MSG_MISSING_END_OF_BLOCK: &[u8] = b"invalid code -- missing end-of-block\0";
const INFLATE_MSG_INVALID_LITERAL_LENGTHS: &[u8] = b"invalid literal/lengths set\0";
const INFLATE_MSG_INVALID_DISTANCES: &[u8] = b"invalid distances set\0";
const INFLATE_MSG_INVALID_BLOCK_TYPE: &[u8] = b"invalid block type\0";
pub(crate) const INFLATE_MSG_INVALID_LITERAL_LENGTH_CODE: &[u8] = b"invalid literal/length code\0";
pub(crate) const INFLATE_MSG_INVALID_DISTANCE_CODE: &[u8] = b"invalid distance code\0";
const INFLATE_MSG_HEADER_CRC_MISMATCH: &[u8] = b"header crc mismatch\0";
pub(crate) const INFLATE_MSG_DISTANCE_TOO_FAR_BACK: &[u8] = b"invalid distance too far back\0";

pub(crate) fn inflate_error_message(
    strm: &crate::zlib_h::z_stream,
) -> Option<&'static [u8]> {
    [
        INFLATE_MSG_INCORRECT_HEADER_CHECK,
        INFLATE_MSG_UNKNOWN_COMPRESSION_METHOD,
        INFLATE_MSG_INVALID_WINDOW_SIZE,
        INFLATE_MSG_UNKNOWN_HEADER_FLAGS,
        INFLATE_MSG_INVALID_STORED_BLOCK_LENGTHS,
        INFLATE_MSG_TOO_MANY_SYMBOLS,
        INFLATE_MSG_INCORRECT_DATA_CHECK,
        INFLATE_MSG_INCORRECT_LENGTH_CHECK,
        INFLATE_MSG_INVALID_CODE_LENGTHS,
        INFLATE_MSG_INVALID_BIT_LENGTH_REPEAT,
        INFLATE_MSG_MISSING_END_OF_BLOCK,
        INFLATE_MSG_INVALID_LITERAL_LENGTHS,
        INFLATE_MSG_INVALID_DISTANCES,
        INFLATE_MSG_INVALID_BLOCK_TYPE,
        INFLATE_MSG_INVALID_LITERAL_LENGTH_CODE,
        INFLATE_MSG_INVALID_DISTANCE_CODE,
        INFLATE_MSG_HEADER_CRC_MISMATCH,
        INFLATE_MSG_DISTANCE_TOO_FAR_BACK,
    ]
    .into_iter()
    .find(|known| ::core::ptr::eq(strm.msg, known.as_ptr().cast()))
}

macro_rules! inflate_input_byte {
    ($input:expr, $initial_have:expr, $have:expr) => {
        $input[$initial_have.wrapping_sub($have).wrapping_sub(1) as usize]
    };
}

macro_rules! inflate_lencode {
    ($state:expr, $index:expr $(,)?) => {{
        let state = &*$state;
        let index = $index;
        if ::core::ptr::eq(
            state.lencode,
            crate::src::inftrees::inffixed_h::lenfix.as_ptr(),
        ) {
            crate::src::inftrees::inffixed_h::lenfix[index]
        } else {
            state.codes[index]
        }
    }};
}

macro_rules! inflate_distcode {
    ($state:expr, $index:expr $(,)?) => {{
        let state = &*$state;
        let index = $index;
        if ::core::ptr::eq(
            state.distcode,
            crate::src::inftrees::inffixed_h::distfix.as_ptr(),
        ) {
            crate::src::inftrees::inffixed_h::distfix[index]
        } else {
            let base = state.codes.as_ptr().addr();
            let start = state
                .distcode
                .addr()
                .wrapping_sub(base)
                .wrapping_div(::core::mem::size_of::<crate::src::inftrees::code>());
            state.codes[start.wrapping_add(index)]
        }
    }};
}

macro_rules! inflate_update_header_crc {
    ($state:expr, $bytes:expr $(,)?) => {{
        let state = &mut *$state;
        state.check = crate::src::crc32::crc32_bytes(state.check as crate::stdlib::uLong, $bytes)
            as ::core::ffi::c_ulong;
    }};
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct inflate_state {
    pub strm: crate::zlib_h::z_streamp,
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
    pub window: *mut ::core::ffi::c_uchar,
    pub hold: ::core::ffi::c_ulong,
    pub bits: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub extra: ::core::ffi::c_uint,
    pub lencode: *const crate::src::inftrees::code,
    pub distcode: *const crate::src::inftrees::code,
    pub lenbits: ::core::ffi::c_uint,
    pub distbits: ::core::ffi::c_uint,
    pub ncode: ::core::ffi::c_uint,
    pub nlen: ::core::ffi::c_uint,
    pub ndist: ::core::ffi::c_uint,
    pub have: ::core::ffi::c_uint,
    pub next: *mut crate::src::inftrees::code,
    pub lens: [::core::ffi::c_ushort; 320],
    pub work: [::core::ffi::c_ushort; 288],
    pub codes: [crate::src::inftrees::code; 1444],
    pub sane: ::core::ffi::c_int,
    pub back: ::core::ffi::c_int,
    pub was: ::core::ffi::c_uint,
}

// State allocations come from zlib's configurable allocator and are therefore
// initially uninitialized. Construct the exact field-wise zero value before
// exposing such an allocation as `inflate_state`, rather than relying on C's
// bytewise initialization after allocation.
pub(crate) fn inflate_state_zero_value() -> inflate_state {
    let zero_code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    inflate_state {
        strm: ::core::ptr::null_mut(),
        mode: 0,
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
        window: ::core::ptr::null_mut(),
        hold: 0,
        bits: 0,
        length: 0,
        offset: 0,
        extra: 0,
        lencode: ::core::ptr::null(),
        distcode: ::core::ptr::null(),
        lenbits: 0,
        distbits: 0,
        ncode: 0,
        nlen: 0,
        ndist: 0,
        have: 0,
        next: ::core::ptr::null_mut(),
        lens: [0; 320],
        work: [0; 288],
        codes: [zero_code; 1444],
        sane: 0,
        back: 0,
        was: 0,
    }
}
pub use crate::__stddef_size_t_h::size_t;

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

// This state adapter is used only by translated Rust implementations.  Its
// callers have already bound the stream at their ABI boundary; it validates
// the associated state allocation before returning both references.
pub(crate) fn inflateStateCheck<'a>(
    strm: &'a mut crate::zlib_h::z_stream,
) -> Option<(
    &'a mut crate::zlib_h::z_stream,
    &'a mut crate::src::inflate::inflate_state,
)> {
    let strm_ptr = ::core::ptr::from_mut(strm);
    let state_ptr = strm.state as *mut crate::src::inflate::inflate_state;
    if state_ptr.is_null() {
        return None;
    }
    // SAFETY: the stream reference is already bound by the caller. Its
    // non-null state pointer is checked for the reciprocal stream link and
    // state invariants before the reference is exposed.
    let state = unsafe { &mut *state_ptr };
    if !inflate_state_is_valid(strm, state, state.strm == strm_ptr) {
        return None;
    }
    Some((strm, state))
}

fn inflate_state_is_valid(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::inflate::inflate_state,
    points_back_to_stream: bool,
) -> bool {
    strm.zalloc.is_some()
        && strm.zfree.is_some()
        && points_back_to_stream
        && inflate_state_mode_is_valid(state.mode)
}

// Keep the scalar state-range check separate from raw stream/state binding.
// This makes the latter responsible only for establishing the two references
// and their reciprocal link.
fn inflate_state_mode_is_valid(mode: ::core::ffi::c_uint) -> bool {
    mode >= crate::src::inflate::HEAD && mode <= crate::src::inflate::SYNC
}
fn inflate_reset_keep(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
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
    state.next = state.codes.as_mut_ptr();
    state.distcode = state.next;
    state.lencode = state.distcode;
    state.sane = 1 as ::core::ffi::c_int;
    state.back = -1 as ::core::ffi::c_int;
    crate::zlib_h::Z_OK
}

fn inflate_reset(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    state.wsize = 0 as ::core::ffi::c_uint;
    state.whave = 0 as ::core::ffi::c_uint;
    state.wnext = 0 as ::core::ffi::c_uint;
    inflate_reset_keep(strm, state)
}

// Callers that have already validated and bound an inflater can reset it
// without routing through the raw C-ABI entry point.  This keeps ordinary
// reset state changes reference-bound at internal call sites.
pub(crate) fn inflate_reset_bound(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    inflate_reset(strm, state)
}

// Internal callers that own a stream but not its raw state pointer can use
// the validated stream/state binder above instead of dereferencing that
// pointer themselves. This keeps the reset transition reference-bound at
// those call sites while preserving inflateStateCheck as the one raw adapter.
pub(crate) fn inflate_reset_stream_bound(
    strm: &mut crate::zlib_h::z_stream,
) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_reset_bound(strm, state)
}

// Keep reset validation in named implementations so the exported ABI
// forwarders below only bind the caller's stream pointer and dispatch.
pub fn inflateResetKeep(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_reset_keep(strm, state)
}

pub fn inflateReset(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    inflate_reset_stream_bound(strm)
}

#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // SAFETY: this is the ABI boundary that binds the caller's optional
    // stream pointer; the named implementation owns state validation.
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateResetKeep(strm)
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // SAFETY: this is the ABI boundary that binds the caller's optional
    // stream pointer; the named implementation owns state validation.
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateReset(strm)
}
// Resetting an already-bound stream keeps all state validation and teardown
// in the implementation. The exported ABI wrapper below only establishes the
// foreign stream reference before dispatching here.
pub fn inflateReset2(
    strm: &mut crate::zlib_h::z_stream,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let (wrap, window_bits) = match inflate_window_bits(windowBits) {
        Ok(window_bits) => window_bits,
        Err(error) => return error,
    };
    if inflate_reset2_releases_window(state, window_bits) {
        // Snapshot the user callback and its arguments before releasing the
        // old window.  The stream and state are already bound above, so use
        // those same bindings after the callback instead of revisiting the
        // raw stream pointer.
        let (zfree, opaque, window) = (
            strm.zfree.expect("non-null function pointer"),
            strm.opaque,
            state.window,
        );
        Some(zfree).expect("non-null function pointer")(opaque, window as crate::stdlib::voidpf);
        state.window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    inflate_reset_with_window_bits(strm, state, wrap, window_bits)
}

// This is the state-only part of `inflateReset2()`.  Its caller retains the
// existing ABI bindings and callback ordering.
fn inflate_reset2_releases_window(
    state: &crate::src::inflate::inflate_state,
    window_bits: ::core::ffi::c_int,
) -> bool {
    !state.window.is_null() && state.wbits != window_bits as ::core::ffi::c_uint
}

fn inflate_window_bits(
    mut window_bits: ::core::ffi::c_int,
) -> Result<(::core::ffi::c_int, ::core::ffi::c_int), ::core::ffi::c_int> {
    let wrap;
    if window_bits < 0 as ::core::ffi::c_int {
        if window_bits < -15 as ::core::ffi::c_int {
            return Err(crate::zlib_h::Z_STREAM_ERROR);
        }
        wrap = 0 as ::core::ffi::c_int;
        window_bits = -window_bits;
    } else {
        wrap = (window_bits >> 4 as ::core::ffi::c_int) + 5 as ::core::ffi::c_int;
        if window_bits < 48 as ::core::ffi::c_int {
            window_bits &= 15 as ::core::ffi::c_int;
        }
    }
    if window_bits != 0
        && (window_bits < 8 as ::core::ffi::c_int || window_bits > 15 as ::core::ffi::c_int)
    {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok((wrap, window_bits))
}

fn inflate_reset_with_window_bits(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    wrap: ::core::ffi::c_int,
    window_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    state.wrap = wrap;
    state.wbits = window_bits as ::core::ffi::c_uint;
    inflate_reset(strm, state)
}

fn inflate_initialize_state(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    window_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    state.window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    state.mode = crate::src::inflate::HEAD;
    let (wrap, window_bits) = match inflate_window_bits(window_bits) {
        Ok(window_bits) => window_bits,
        Err(error) => return error,
    };
    inflate_reset_with_window_bits(strm, state, wrap, window_bits)
}

// Default callbacks belong to initialization, after the ABI entry point has
// bound the stream. This keeps callback selection out of the exported
// forwarding wrapper and leaves the allocation sequence below unchanged.
fn inflate_prepare_stream(strm: &mut crate::zlib_h::z_stream) -> bool {
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    crate::src::zutil::prepare_stream_allocator(strm)
}

#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateReset2(strm, windowBits)
}
// Version and stream-layout validation precedes the optional stream binding:
// zlib reports a version error even when the stream pointer is null. Keep the
// scalar check shared by both initializers without moving their raw stream or
// allocator work out of the established ABI boundary.
fn inflate_init_version_and_size_valid(
    version: Option<::core::ffi::c_char>,
    stream_size: ::core::ffi::c_int,
) -> bool {
    let Some(version) = version else {
        return false;
    };
    version as ::core::ffi::c_int == crate::zlib_h::ZLIB_VERSION[0] as ::core::ffi::c_int
        && stream_size == ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
}

// The ABI adapter binds the optional stream and version byte before reaching
// this implementation. Keeping the initializer reference- and value-based
// removes the raw-pointer contract from the core allocation and reset path.
pub(crate) fn inflateInit2_(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut windowBits: ::core::ffi::c_int,
    version: Option<::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if !inflate_init_version_and_size_valid(version, stream_size) {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // The remaining initialization is ordinary stream/state work, including
    // default callback selection.
    let uses_default_allocator = inflate_prepare_stream(strm);
    state = if uses_default_allocator {
        crate::src::zutil::zcalloc(
            strm.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::inflate::inflate_state
    } else {
        Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::inflate::inflate_state
    };
    if state.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // SAFETY: the allocator returned a non-null allocation large enough for
    // one `inflate_state`. Bind it as uninitialized storage only long enough
    // to write the complete safe zero value, then retain the initialized
    // reference for the rest of this function.
    let state_ref = unsafe {
        (&mut *state.cast::<::core::mem::MaybeUninit<crate::src::inflate::inflate_state>>())
            .write(inflate_state_zero_value())
    };
    strm.state = state as *mut crate::src::deflate::internal_state;
    // The allocator returned a non-null `inflate_state` above. It is owned by
    // this stream until the matching release below.
    state_ref.strm = strm;
    let ret = inflate_initialize_state(strm, state_ref, windowBits);
    if ret != crate::zlib_h::Z_OK {
        Some(strm.zfree.expect("non-null function pointer"))
            .expect("non-null function pointer")(strm.opaque, state as crate::stdlib::voidpf);
        strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    }
    ret
}
#[export_name = "inflateInit2_"]

pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter alone binds the optional foreign stream and
    // version pointers. Validation and initialization stay in `inflateInit2_`.
    let strm = unsafe { strm.as_mut() };
    let version = unsafe { version.as_ref().copied() };
    inflateInit2_(strm, windowBits, version, stream_size)
}
// This internal dispatcher only supplies zlib's default window size and
// accepts references already bound by its callers. Keep the initialization
// contract contained in `inflateInit2_`.
pub fn inflateInit_(
    strm: Option<&mut crate::zlib_h::z_stream>,
    version: Option<&::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // `inflateInit2_()` owns the shared version-before-null validation.  Do
    // not duplicate it here, since both initializer spellings must retain
    // exactly the same error ordering.
    inflateInit2_(
        strm,
        crate::zutil_h::DEF_WBITS,
        version.copied(),
        stream_size,
    )
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // SAFETY: this is the ABI boundary that binds optional foreign pointers.
    // The safe dispatcher preserves `inflateInit2_`'s validation order.
    let strm = unsafe { strm.as_mut() };
    let version = unsafe { version.as_ref() };
    inflateInit_(strm, version, stream_size)
}
// Priming an already-bound stream only updates inflater state. Keep the
// state validation and transition here so internal callers do not need the
// raw C-ABI entry point.
pub fn inflatePrime(
    strm: &mut crate::zlib_h::z_stream,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_prime(state, bits, value)
}

#[export_name = "inflatePrime"]
pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // SAFETY: the C ABI supplies the optional foreign stream pointer; the
    // named dispatcher owns all inflater-state validation and updates.
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflatePrime(strm, bits, value)
}

fn inflate_prime(
    state: &mut crate::src::inflate::inflate_state,
    bits: ::core::ffi::c_int,
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
    crate::zlib_h::Z_OK
}

// All callers reach this only after zlib's window-bit validation. Keeping the
// byte length and allocator request together makes window allocation and
// slice capacities agree without adding another raw-pointer boundary.
struct InflateWindowLayout {
    len: usize,
    alloc_items: crate::stdlib::uInt,
    alloc_size: crate::stdlib::uInt,
}

fn inflate_window_layout(wbits: crate::stdlib::uInt) -> InflateWindowLayout {
    let len = 1usize << wbits;
    InflateWindowLayout {
        len,
        alloc_items: len as crate::stdlib::uInt,
        alloc_size: ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
    }
}

fn update_window(
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [crate::stdlib::Bytef],
    end: &[crate::stdlib::Bytef],
) {
    if state.wsize == 0 {
        state.wsize = inflate_window_layout(state.wbits).len as ::core::ffi::c_uint;
        state.wnext = 0;
        state.whave = 0;
    }
    let wsize = state.wsize as usize;
    let copy = end.len();
    if copy >= wsize {
        window[..wsize].copy_from_slice(&end[copy - wsize..]);
        state.wnext = 0;
        state.whave = state.wsize;
    } else {
        let mut dist = wsize - state.wnext as usize;
        if dist > copy {
            dist = copy;
        }
        let first = copy - dist;
        let wnext = state.wnext as usize;
        window[wnext..wnext + dist].copy_from_slice(&end[..dist]);
        if first != 0 {
            window[..first].copy_from_slice(&end[dist..]);
            state.wnext = first as ::core::ffi::c_uint;
            state.whave = state.wsize;
        } else {
            state.wnext = state.wnext.wrapping_add(dist as ::core::ffi::c_uint);
            if state.wnext == state.wsize {
                state.wnext = 0;
            }
            if state.whave < state.wsize {
                state.whave = state.whave.wrapping_add(dist as ::core::ffi::c_uint);
            }
        }
    }
}

// Copy one pending match into the caller's output buffer.  The decoder loop
// supplies only reference-bound state and already-validated buffer slices,
// keeping the match-distance arithmetic independent of its raw cursors.
enum InflateMatchCopy {
    Copied(usize),
    InvalidDistance,
}

fn inflate_match_copy(
    state: &mut crate::src::inflate::inflate_state,
    output: &mut [crate::stdlib::Bytef],
    written: usize,
    window: Option<&[crate::stdlib::Bytef]>,
) -> InflateMatchCopy {
    let mut copy = state.offset.wrapping_sub(written as ::core::ffi::c_uint);
    let from_window = state.offset > written as ::core::ffi::c_uint;
    let source_start;
    if from_window {
        if copy > state.whave && state.sane != 0 {
            return InflateMatchCopy::InvalidDistance;
        }
        if copy > state.wnext {
            copy = copy.wrapping_sub(state.wnext);
            source_start = state.wsize.wrapping_sub(copy) as usize;
        } else {
            source_start = state.wnext.wrapping_sub(copy) as usize;
        }
        if copy > state.length {
            copy = state.length;
        }
    } else {
        source_start = 0;
        copy = state.length;
    }
    copy = copy.min((output.len() - written) as ::core::ffi::c_uint);
    let copy_len = copy as usize;
    if from_window {
        let window = window.expect("window-backed match requires a window");
        output[written..written + copy_len]
            .copy_from_slice(&window[source_start..source_start + copy_len]);
    } else {
        let distance = state.offset as usize;
        for index in written..written + copy_len {
            output[index] = output[index - distance];
        }
    }
    state.length = state.length.wrapping_sub(copy);
    if state.length == 0 {
        state.mode = crate::src::inflate::LEN;
    }
    InflateMatchCopy::Copied(copy_len)
}

// A match can read from the inflater's retained window. Reuse the common
// window binder here instead of reopening that state-owned allocation with a
// raw slice in the decoder loop.
fn inflate_match_copy_from_state_window(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    output: &mut [crate::stdlib::Bytef],
    written: usize,
) -> InflateMatchCopy {
    updatewindow(stream, state, InflateWindowAccess::Existing, |state, window| {
        inflate_match_copy(state, output, written, window.as_deref())
    })
    .expect("existing-window access cannot allocate or fail")
}

// The inflater's window is an internal allocation.  Keep allocation, update,
// and read-only inspection behind this existing implementation boundary so
// ABI wrappers never need to bind that state-owned raw pointer themselves.
pub(crate) enum InflateWindowAccess<'a> {
    Ensure,
    Update(&'a [crate::stdlib::Bytef]),
    // `inflateCopy()` needs to populate an already-allocated destination
    // history buffer without changing the copied state's window cursors.
    // Keep that byte transfer at this existing owned-window binding boundary.
    CopyFrom(&'a [crate::stdlib::Bytef]),
    Inspect,
    Existing,
}

// A first `Ensure` call keeps the allocator callback separate from use of the
// resulting slice. `Update` consumes produced output, while `Inspect` lends
// the bound window to a reference-only implementation such as dictionary
// retrieval. `Existing` is the no-allocation counterpart for a decoder that
// can use a prior window when one is already present.
pub(crate) fn updatewindow<T>(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    access: InflateWindowAccess<'_>,
    operation: impl FnOnce(
        &mut crate::src::inflate::inflate_state,
        Option<&mut [crate::stdlib::Bytef]>,
    ) -> T,
) -> Result<T, ()> {
    let layout = inflate_window_layout(state.wbits);
    if state.window.is_null() && !matches!(access, InflateWindowAccess::Existing) {
        state.window = Some(stream.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            stream.opaque,
            layout.alloc_items,
            layout.alloc_size,
        ) as *mut ::core::ffi::c_uchar;
        if state.window.is_null() {
            return Err(());
        }
    }
    let needs_window = !matches!(access, InflateWindowAccess::Ensure) && !state.window.is_null();
    // SAFETY: a successful allocation above (or the initialized existing
    // window) has exactly the configured window length. `Ensure` does not
    // need to expose that allocation at all.
    let window = if needs_window {
        Some(unsafe { ::core::slice::from_raw_parts_mut(state.window, layout.len) })
    } else {
        None
    };
    match access {
        InflateWindowAccess::Ensure => Ok(operation(state, None)),
        InflateWindowAccess::Update(output) => {
            update_window(
                state,
                window.expect("window updates require a bound window"),
                output,
            );
            Ok(operation(state, None))
        }
        InflateWindowAccess::CopyFrom(source) => {
            let window = window.expect("window copies require a bound window");
            // `inflateCopy()` preserves only the initialized history
            // (`whave` bytes), exactly as zlib's zmemcpy does.  In
            // particular, a custom allocator is not required to initialize
            // the rest of either window allocation.
            window[..source.len()].copy_from_slice(source);
            Ok(operation(state, None))
        }
        InflateWindowAccess::Inspect => {
            Ok(operation(state, window))
        }
        InflateWindowAccess::Existing => Ok(operation(state, window)),
    }
}
// The checked stream/state binding below, followed by the null cursor guard,
// keeps the implementation's Rust-facing contract reference-bound. Its raw
// decoder operations stay internal to this implementation.
pub fn inflate(
    strm: &mut crate::zlib_h::z_stream,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // The state adapter and cursor validation below bind the stream before
    // the translated decoder runs. Keep ordinary state transitions on those
    // references; the few foreign-buffer and retained-header bindings use
    // their own narrow unsafe blocks.
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut put: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
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
    let mut output_capacity: usize = 0;
    static order: [::core::ffi::c_ushort; 19] = [
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
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if strm.next_out.is_null()
        || strm.next_in.is_null() && strm.avail_in != 0 as crate::stdlib::uInt
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.mode as ::core::ffi::c_uint
        == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        state.mode = crate::src::inflate::TYPEDO;
    }
    put = strm.next_out as *mut ::core::ffi::c_uchar;
    left = strm.avail_out as ::core::ffi::c_uint;
    output_capacity = left as usize;
    next = strm.next_in as *mut ::core::ffi::c_uchar;
    have = strm.avail_in as ::core::ffi::c_uint;
    // SAFETY: the entry guard above established the C cursor contracts for
    // both ranges. Bind them together once before decoding; all subsequent
    // cursor movement is bounded slice/reference work.
    let (input, mut output_storage) = unsafe {
        let input = if have == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts(next, have as usize)
        };
        let output = if left == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut(put, output_capacity)
        };
        (input, output)
    };
    // `inflateGetHeader()` retains this optional caller-owned structure for
    // the duration of inflate. Bind it once for this decode call, so gzip
    // header publication below does not repeatedly dereference the same raw
    // pointer. The variable-length buffers it contains remain bound only at
    // their existing copy sites.
    let mut head = if state.head.is_null() {
        None
    } else {
        // A registered gzip header remains caller-owned and live for this
        // synchronous call, as required by `inflateGetHeader()`.
        Some(unsafe { &mut *state.head })
    };
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
                                                                                                            next = next.wrapping_add(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        if state.wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if state.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                state.wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            state.check = crate::src::crc32::crc32_bytes(
                                                                                                                0 as crate::stdlib::uLong,
                                                                                                                &[],
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            inflate_update_header_crc!(state, &hbuf[..2]);
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            state.mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            if let Some(head) = head.as_deref_mut() {
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
                                                                                                                strm.msg = INFLATE_MSG_INCORRECT_HEADER_CHECK.as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                state.mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                strm.msg = INFLATE_MSG_UNKNOWN_COMPRESSION_METHOD.as_ptr()
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
                                                                                                                    strm.msg = INFLATE_MSG_INVALID_WINDOW_SIZE.as_ptr()
                                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                    state.mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    state.dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    state.flags = 0 as ::core::ffi::c_int;
                                                                                                                    state.check = crate::src::adler32::adler32_buffer(
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
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    state.flags = hold as ::core::ffi::c_int;
                                                                                                    if state.flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        strm.msg = INFLATE_MSG_UNKNOWN_COMPRESSION_METHOD.as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if state.flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        strm.msg = INFLATE_MSG_UNKNOWN_HEADER_FLAGS.as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if let Some(head) = head.as_deref_mut() {
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
                                                                                                            inflate_update_header_crc!(state, &hbuf[..2]);
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
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
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
                                                                                                    hold >>= bits & 7 as ::core::ffi::c_uint;
                                                                                                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    if hold & 0xffff as ::core::ffi::c_ulong
                                                                                                        != hold >> 16 as ::core::ffi::c_int
                                                                                                            ^ 0xffff as ::core::ffi::c_ulong
                                                                                                    {
                                                                                                        strm.msg = INFLATE_MSG_INVALID_STORED_BLOCK_LENGTHS.as_ptr()
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
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
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
                                                                                                        strm.msg = INFLATE_MSG_TOO_MANY_SYMBOLS
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
                                                                                                    let output = &mut output_storage[output_capacity - left as usize..];
                                                                                                    output[0] = state.length as ::core::ffi::c_uchar;
                                                                                                    put = output[1..].as_mut_ptr();
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
                                                                                                            next = next.wrapping_add(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        out = out.wrapping_sub(left);
                                                                                                        strm.total_out = (*strm)
                                                                                                            .total_out
                                                                                                            .wrapping_add(out as crate::stdlib::uLong);
                                                                                                        state.total = (*state)
                                                                                                            .total
                                                                                                            .wrapping_add(out as ::core::ffi::c_ulong);
                                                                                                        if state.wrap & 4 as ::core::ffi::c_int != 0 && out != 0
                                                                                                        {
                                                                                                            let output = &output_storage[..];
                                                                                                            let output_start = output_capacity
                                                                                                                .wrapping_sub(left as usize)
                                                                                                                .wrapping_sub(out as usize);
                                                                                                            let output = &output[output_start..output_start + out as usize];
                                                                                                            state.check = (if state.flags != 0 {
                                                                                                                crate::src::crc32::crc32_bytes(
                                                                                                                    state.check as crate::stdlib::uLong,
                                                                                                                    output,
                                                                                                                )
                                                                                                            } else {
                                                                                                                crate::src::adler32::adler32_bytes(
                                                                                                                    state.check as crate::stdlib::uLong,
                                                                                                                    output,
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
                                                                                                            strm.msg = INFLATE_MSG_INCORRECT_DATA_CHECK.as_ptr()
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
                                                                                                    next = next.wrapping_add(1);
                                                                                                    hold = hold
                                                                                                        .wrapping_add(
                                                                                                            (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                }
                                                                                                if state.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                    && hold
                                                                                                        != state.total & 0xffffffff as ::core::ffi::c_ulong
                                                                                                {
                                                                                                    strm.msg = INFLATE_MSG_INCORRECT_LENGTH_CHECK.as_ptr()
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
                                                                                                next = next.wrapping_add(1);
                                                                                                hold = hold
                                                                                                    .wrapping_add(
                                                                                                        (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                    );
                                                                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                            }
                                                                                            let c2rust_fresh15 = state.have;
                                                                                            state.have = state.have.wrapping_add(1);
                                                                                            state.lens[order[c2rust_fresh15 as usize] as usize] = (hold
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
                                                                                            state.lens[order[c2rust_fresh16 as usize] as usize] = 0
                                                                                                as ::core::ffi::c_ushort;
                                                                                        }
                                                                                        state.next = &raw mut state.codes as *mut crate::src::inftrees::code;
                                                                                        state.distcode = state.next as *const crate::src::inftrees::code;
                                                                                        state.lencode = state.distcode;
                                                                                        state.lenbits = 7 as ::core::ffi::c_uint;
                                                                                        ret = match inflate_build_dynamic_table(
                                                                                            &mut *state,
                                                                                            crate::src::inftrees::CODES,
                                                                                            0,
                                                                                            19 as ::core::ffi::c_uint,
                                                                                            7 as ::core::ffi::c_uint,
                                                                                        ) {
                                                                                            Ok(bits) => {
                                                                                                state.lenbits = bits;
                                                                                                0
                                                                                            }
                                                                                            Err(error) => error,
                                                                                        };
                                                                                        if ret != 0
                                                                                        {
                                                                                            strm.msg = INFLATE_MSG_INVALID_CODE_LENGTHS.as_ptr()
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
                                                                                    state.check = crate::src::adler32::adler32_buffer(
                                                                                        0 as crate::stdlib::uLong,
                                                                                        None,
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                    strm.adler =
                                                                                        (*state)
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
                                                                                    next = next.wrapping_add(1);
                                                                                    hold = hold
                                                                                        .wrapping_add(
                                                                                            (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                        );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                if let Some(head) = head.as_deref_mut() {
                                                                                    head.time = hold as crate::stdlib::uLong;
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
                                                                                    inflate_update_header_crc!(state, &hbuf);
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
                                                                            < (*state)
                                                                                .nlen
                                                                                .wrapping_add(
                                                                                    state.ndist,
                                                                                )
                                                                        {
                                                                            loop {
                                                                                here = inflate_lencode!(
                                                                                    state,
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
                                                                                next = next
                                                                                    .wrapping_add(
                                                                                        1,
                                                                                    );
                                                                                hold = hold
                                                                                    .wrapping_add(
                                                                                        (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
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
                                                                                        next = next.wrapping_add(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    if state.have == 0 as ::core::ffi::c_uint {
                                                                                        strm.msg = INFLATE_MSG_INVALID_BIT_LENGTH_REPEAT.as_ptr()
                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                        state.mode = crate::src::inflate::BAD;
                                                                                        break;
                                                                                    } else {
                                                                                        len = (*state)
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
                                                                                        next = next.wrapping_add(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
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
                                                                                        next = next.wrapping_add(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
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
                                                                                    strm.msg = INFLATE_MSG_INVALID_BIT_LENGTH_REPEAT.as_ptr()
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
                                                                            strm.msg = INFLATE_MSG_MISSING_END_OF_BLOCK
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            state.mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            state.next = &raw mut state.codes as *mut crate::src::inftrees::code;
                                                                            state.lencode = state.next as *const crate::src::inftrees::code;
                                                                            state.lenbits = 9 as ::core::ffi::c_uint;
                                                                            let nlen = state.nlen;
                                                                            ret = match inflate_build_dynamic_table(
                                                                                &mut *state,
                                                                                crate::src::inftrees::LENS,
                                                                                0,
                                                                                nlen,
                                                                                9 as ::core::ffi::c_uint,
                                                                            ) {
                                                                                Ok(bits) => {
                                                                                    state.lenbits = bits;
                                                                                    0
                                                                                }
                                                                                Err(error) => error,
                                                                            };
                                                                            if ret != 0 {
                                                                                strm.msg = INFLATE_MSG_INVALID_LITERAL_LENGTHS.as_ptr()
                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                state.mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                state.distcode = state.next as *const crate::src::inftrees::code;
                                                                                state.distbits = 6 as ::core::ffi::c_uint;
                                                                                let (nlen, ndist) = (state.nlen, state.ndist);
                                                                                ret = match inflate_build_dynamic_table(
                                                                                    &mut *state,
                                                                                    crate::src::inftrees::DISTS,
                                                                                    nlen as usize,
                                                                                    ndist,
                                                                                    6 as ::core::ffi::c_uint,
                                                                                ) {
                                                                                    Ok(bits) => {
                                                                                        state.distbits = bits;
                                                                                        0
                                                                                    }
                                                                                    Err(error) => error,
                                                                                };
                                                                                if ret != 0 {
                                                                                    strm.msg = INFLATE_MSG_INVALID_DISTANCES.as_ptr()
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
                                                                        let input_start = in_0
                                                                            .wrapping_sub(have)
                                                                            as usize;
                                                                        let output_start = output_capacity - left as usize;
                                                                        let output = &mut output_storage[output_start
                                                                            ..output_start + copy as usize];
                                                                        output.copy_from_slice(
                                                                            &input[input_start
                                                                                ..input_start
                                                                                    + copy
                                                                                        as usize],
                                                                        );
                                                                        have =
                                                                            have.wrapping_sub(copy);
                                                                        // `copy` is bounded by the input slice above.
                                                                        // Advancing this cursor therefore need not use
                                                                        // `offset`'s in-bounds raw-pointer operation.
                                                                        next = next.wrapping_add(
                                                                            copy as usize,
                                                                        );
                                                                        left =
                                                                            left.wrapping_sub(copy);
                                                                        put = output
                                                                            [copy as usize..]
                                                                            .as_mut_ptr();
                                                                        state.length = (*state)
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
                                                                    if have
                                                                        == 0 as ::core::ffi::c_uint
                                                                    {
                                                                        break '_inf_leave;
                                                                    }
                                                                    have = have.wrapping_sub(1);
                                                                    next = next.wrapping_add(1);
                                                                    hold = hold.wrapping_add(
                                                                        (inflate_input_byte!(
                                                                            input, in_0, have
                                                                        )
                                                                            as ::core::ffi::c_ulong)
                                                                            << bits,
                                                                    );
                                                                    bits = bits.wrapping_add(
                                                                        8 as ::core::ffi::c_uint,
                                                                    );
                                                                }
                                                                if let Some(head) = head.as_deref_mut() {
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
                                                                    inflate_update_header_crc!(
                                                                        state,
                                                                        &hbuf[..2]
                                                                    );
                                                                }
                                                                hold = 0 as ::core::ffi::c_ulong;
                                                                bits = 0 as ::core::ffi::c_uint;
                                                                state.mode =
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
                                                    if state.last != 0 {
                                                        hold >>= bits & 7 as ::core::ffi::c_uint;
                                                        bits = bits.wrapping_sub(
                                                            bits & 7 as ::core::ffi::c_uint,
                                                        );
                                                        state.mode = crate::src::inflate::CHECK;
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
                                                            next = next.wrapping_add(1);
                                                            hold = hold.wrapping_add(
                                                                (inflate_input_byte!(
                                                                    input, in_0, have
                                                                )
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
                                                                let state_ref = &mut *state;
                                                                crate::src::inftrees::inflate_fixed(
                                                                    state_ref,
                                                                );
                                                                state_ref.mode =
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
                                                                state.mode =
                                                                    crate::src::inflate::TABLE;
                                                            }
                                                            _ => {
                                                                strm.msg = INFLATE_MSG_INVALID_BLOCK_TYPE.as_ptr()
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
                                                if state.flags & 0x400 as ::core::ffi::c_int != 0
                                                {
                                                    while bits
                                                        < 16 as ::core::ffi::c_int
                                                            as ::core::ffi::c_uint
                                                    {
                                                        if have == 0 as ::core::ffi::c_uint {
                                                            break '_inf_leave;
                                                        }
                                                        have = have.wrapping_sub(1);
                                                        next = next.wrapping_add(1);
                                                        hold = hold.wrapping_add(
                                                            (inflate_input_byte!(input, in_0, have)
                                                                as ::core::ffi::c_ulong)
                                                                << bits,
                                                        );
                                                        bits = bits
                                                            .wrapping_add(8 as ::core::ffi::c_uint);
                                                    }
                                                    state.length = hold as ::core::ffi::c_uint;
                                                    if let Some(head) = head.as_deref_mut() {
                                                        head.extra_len = hold
                                                            as ::core::ffi::c_uint
                                                            as crate::stdlib::uInt;
                                                    }
                                                    if state.flags & 0x200 as ::core::ffi::c_int
                                                        != 0
                                                        && state.wrap & 4 as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        hbuf[0 as ::core::ffi::c_int as usize] =
                                                            hold as ::core::ffi::c_uchar;
                                                        hbuf[1 as ::core::ffi::c_int as usize] =
                                                            (hold >> 8 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uchar;
                                                        inflate_update_header_crc!(
                                                            state,
                                                            &hbuf[..2]
                                                        );
                                                    }
                                                    hold = 0 as ::core::ffi::c_ulong;
                                                    bits = 0 as ::core::ffi::c_uint;
                                                } else if let Some(head) = head.as_deref_mut() {
                                                    head.extra = ::core::ptr::null_mut::<
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
                                            let input_start = in_0.wrapping_sub(have) as usize;
                                            let output_start = output_capacity - out as usize;
                                            crate::src::inffast::inflate_fast_bound_cursors(
                                                strm,
                                                state,
                                                out,
                                                &input[input_start..],
                                                &mut output_storage[output_start..],
                                            );
                                            put = strm.next_out as *mut ::core::ffi::c_uchar;
                                            left = strm.avail_out as ::core::ffi::c_uint;
                                            next = strm.next_in as *mut ::core::ffi::c_uchar;
                                            have = strm.avail_in as ::core::ffi::c_uint;
                                            hold = state.hold;
                                            bits = state.bits;
                                            if state.mode as ::core::ffi::c_uint
                                                == crate::src::inflate::TYPE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                state.back = -1 as ::core::ffi::c_int;
                                            }
                                            continue '_inf_leave;
                                        } else {
                                            state.back = 0 as ::core::ffi::c_int;
                                            loop {
                                                here = inflate_lencode!(
                                                    state,
                                                    (hold as ::core::ffi::c_uint
                                                        & ((1 as ::core::ffi::c_uint)
                                                            << state.lenbits)
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
                                                next = next.wrapping_add(1);
                                                hold = hold.wrapping_add(
                                                    (inflate_input_byte!(input, in_0, have)
                                                        as ::core::ffi::c_ulong)
                                                        << bits,
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
                                                    here = inflate_lencode!(
                                                        state,
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
                                                    next = next.wrapping_add(1);
                                                    hold = hold.wrapping_add(
                                                        (inflate_input_byte!(input, in_0, have)
                                                            as ::core::ffi::c_ulong)
                                                            << bits,
                                                    );
                                                    bits =
                                                        bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                }
                                                hold >>= last.bits as ::core::ffi::c_int;
                                                bits = bits
                                                    .wrapping_sub(last.bits as ::core::ffi::c_uint);
                                                state.back += last.bits as ::core::ffi::c_int;
                                            }
                                            hold >>= here.bits as ::core::ffi::c_int;
                                            bits =
                                                bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
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
                                                strm.msg = INFLATE_MSG_INVALID_LITERAL_LENGTH_CODE
                                                    .as_ptr()
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
                                            if let Some(head) = head.as_deref_mut() {
                                                let extra_offset = (head.extra_len
                                                    as ::core::ffi::c_uint)
                                                    .wrapping_sub(state.length);
                                                if !head.extra.is_null()
                                                    && extra_offset < head.extra_max
                                                {
                                                    let copy_len = copy.min(
                                                        (head.extra_max as ::core::ffi::c_uint)
                                                            .wrapping_sub(extra_offset),
                                                    )
                                                        as usize;
                                                    // `extra_offset` and
                                                    // `copy_len` were checked
                                                    // against the caller's
                                                    // advertised header range.
                                                    let extra = unsafe {
                                                        ::core::slice::from_raw_parts_mut(
                                                            head.extra,
                                                            head.extra_max as usize,
                                                        )
                                                    };
                                                    let input_start =
                                                        in_0.wrapping_sub(have) as usize;
                                                    extra[extra_offset as usize
                                                        ..extra_offset as usize + copy_len]
                                                        .copy_from_slice(
                                                            &input[input_start
                                                                ..input_start + copy_len],
                                                        );
                                                }
                                            }
                                            if state.flags & 0x200 as ::core::ffi::c_int != 0
                                                && state.wrap & 4 as ::core::ffi::c_int != 0
                                            {
                                                let input_start = in_0.wrapping_sub(have) as usize;
                                                inflate_update_header_crc!(
                                                    state,
                                                    &input
                                                        [input_start..input_start + copy as usize],
                                                );
                                            }
                                            have = have.wrapping_sub(copy);
                                            next = next.wrapping_add(copy as usize);
                                            state.length = state.length.wrapping_sub(copy);
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
                                        next = next.wrapping_add(1);
                                        hold = hold.wrapping_add(
                                            (inflate_input_byte!(input, in_0, have)
                                                as ::core::ffi::c_ulong)
                                                << bits,
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
                                copy = 0 as ::core::ffi::c_uint;
                                loop {
                                    let c2rust_fresh5 = copy;
                                    copy = copy.wrapping_add(1);
                                    len = input[in_0.wrapping_sub(have).wrapping_add(c2rust_fresh5)
                                        as usize]
                                        as ::core::ffi::c_uint;
                                    if let Some(head) = head.as_deref_mut() {
                                        if !head.name.is_null() && state.length < head.name_max {
                                            // The non-null cursor and length
                                            // check above bind this one
                                            // caller-owned header range.
                                            let name = unsafe {
                                                ::core::slice::from_raw_parts_mut(
                                                    head.name,
                                                    head.name_max as usize,
                                                )
                                            };
                                            name[state.length as usize] =
                                                len as crate::stdlib::Bytef;
                                            state.length = state.length.wrapping_add(1);
                                        }
                                    }
                                    if !(len != 0 && copy < have) {
                                        break;
                                    }
                                }
                                if state.flags & 0x200 as ::core::ffi::c_int != 0
                                    && state.wrap & 4 as ::core::ffi::c_int != 0
                                {
                                    let input_start = in_0.wrapping_sub(have) as usize;
                                    inflate_update_header_crc!(
                                        state,
                                        &input[input_start..input_start + copy as usize],
                                    );
                                }
                                have = have.wrapping_sub(copy);
                                next = next.wrapping_add(copy as usize);
                                if len != 0 {
                                    break '_inf_leave;
                                }
                            } else if let Some(head) = head.as_deref_mut() {
                                head.name =
                                    ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                            }
                            state.length = 0 as ::core::ffi::c_uint;
                            state.mode = crate::src::inflate::COMMENT;
                            break 'c_2325;
                        }
                        loop {
                            here = inflate_distcode!(
                                state,
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
                            next = next.wrapping_add(1);
                            hold = hold.wrapping_add(
                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong)
                                    << bits,
                            );
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            last = here;
                            loop {
                                here = inflate_distcode!(
                                    state,
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
                                next = next.wrapping_add(1);
                                hold = hold.wrapping_add(
                                    (inflate_input_byte!(input, in_0, have)
                                        as ::core::ffi::c_ulong)
                                        << bits,
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
                            strm.msg = INFLATE_MSG_INVALID_DISTANCE_CODE.as_ptr()
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
                        copy = 0 as ::core::ffi::c_uint;
                        loop {
                            let c2rust_fresh7 = copy;
                            copy = copy.wrapping_add(1);
                            len = input
                                [in_0.wrapping_sub(have).wrapping_add(c2rust_fresh7) as usize]
                                as ::core::ffi::c_uint;
                            if let Some(head) = head.as_deref_mut() {
                                if !head.comment.is_null() && state.length < head.comm_max {
                                    // The non-null cursor and length check
                                    // above bind this one caller-owned
                                    // header range.
                                    let comment = unsafe {
                                        ::core::slice::from_raw_parts_mut(
                                            head.comment,
                                            head.comm_max as usize,
                                        )
                                    };
                                    comment[state.length as usize] = len as crate::stdlib::Bytef;
                                    state.length = state.length.wrapping_add(1);
                                }
                            }
                            if !(len != 0 && copy < have) {
                                break;
                            }
                        }
                        if state.flags & 0x200 as ::core::ffi::c_int != 0
                            && state.wrap & 4 as ::core::ffi::c_int != 0
                        {
                            let input_start = in_0.wrapping_sub(have) as usize;
                            inflate_update_header_crc!(
                                state,
                                &input[input_start..input_start + copy as usize],
                            );
                        }
                        have = have.wrapping_sub(copy);
                        next = next.wrapping_add(copy as usize);
                        if len != 0 {
                            break '_inf_leave;
                        }
                    } else if let Some(head) = head.as_deref_mut() {
                        head.comment = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
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
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add(
                            (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong)
                                << bits,
                        );
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
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add(
                        (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                    );
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if state.wrap & 4 as ::core::ffi::c_int != 0
                    && hold != state.check & 0xffff as ::core::ffi::c_ulong
                {
                    strm.msg = INFLATE_MSG_HEADER_CRC_MISMATCH.as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state.mode = crate::src::inflate::BAD;
                    continue '_inf_leave;
                } else {
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                }
            }
            if let Some(head) = head.as_deref_mut() {
                head.hcrc =
                    state.flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                head.done = 1 as ::core::ffi::c_int;
            }
            state.check = crate::src::crc32::crc32_bytes(0 as crate::stdlib::uLong, &[])
                as ::core::ffi::c_ulong;
            strm.adler = state.check as crate::stdlib::uLong;
            state.mode = crate::src::inflate::TYPE;
            continue '_inf_leave;
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        let written = output_capacity - left as usize;
        let copied =
            inflate_match_copy_from_state_window(strm, state, &mut output_storage, written);
        match copied {
            InflateMatchCopy::Copied(copied) => {
                left = left.wrapping_sub(copied as ::core::ffi::c_uint);
                put = output_storage[written + copied..].as_mut_ptr();
            }
            InflateMatchCopy::InvalidDistance => {
                strm.msg = INFLATE_MSG_DISTANCE_TOO_FAR_BACK.as_ptr()
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                state.mode = crate::src::inflate::BAD;
                continue '_inf_leave;
            }
        }
    }
    // The decoder loop is complete. Bind the already-validated stream/state
    // pair once for the publication tail, so cursor updates, the optional
    // window allocation, and final accounting remain reference-bound.
    strm.next_out = put as *mut crate::stdlib::Bytef;
    strm.avail_out = left as crate::stdlib::uInt;
    strm.next_in = next as *mut crate::stdlib::Bytef;
    strm.avail_in = have as crate::stdlib::uInt;
    state.hold = hold;
    state.bits = bits;
    let produced = out.wrapping_sub(strm.avail_out as ::core::ffi::c_uint);
    let update_window = state.wsize != 0
        || out != strm.avail_out
            && (state.mode as ::core::ffi::c_uint)
                < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
            && ((state.mode as ::core::ffi::c_uint)
                < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                || flush != crate::zlib_h::Z_FINISH);
    if update_window {
        if updatewindow(strm, state, InflateWindowAccess::Ensure, |_, _| ()).is_err() {
            state.mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    let output = &output_storage[output_capacity - out as usize
        ..output_capacity - out as usize + produced as usize];
    if update_window {
        let _ = updatewindow(strm, state, InflateWindowAccess::Update(output), |_, _| ());
    }
    in_0 = in_0.wrapping_sub(strm.avail_in as ::core::ffi::c_uint);
    out = out.wrapping_sub(strm.avail_out as ::core::ffi::c_uint);
    strm.total_in = strm.total_in.wrapping_add(in_0 as crate::stdlib::uLong);
    strm.total_out = strm.total_out.wrapping_add(out as crate::stdlib::uLong);
    state.total = state.total.wrapping_add(out as ::core::ffi::c_ulong);
    if state.wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
        state.check = (if state.flags != 0 {
            crate::src::crc32::crc32_bytes(state.check as crate::stdlib::uLong, output)
        } else {
            crate::src::adler32::adler32_bytes(state.check as crate::stdlib::uLong, output)
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
#[export_name = "inflate"]

pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate(strm, flush)
}
pub fn inflateEnd(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Snapshot the release plan before invoking a user-supplied deallocator,
    // since it is allowed to invalidate allocations reachable from the stream.
    let (zfree, opaque, window, state_ptr) = (
        strm.zfree.expect("non-null function pointer"),
        strm.opaque,
        state.window,
        strm.state,
    );
    if !window.is_null() {
        zfree(opaque, window as crate::stdlib::voidpf);
    }
    zfree(opaque, state_ptr as crate::stdlib::voidpf);
    inflate_end_complete(strm)
}

// Gzip initializes its private inflater with zlib's default callbacks.  Its
// close path can therefore release that private state without crossing the
// user-callback boundary used by the public `inflateEnd()` ABI.  Keep the
// validation in the existing raw-state adapter, then snapshot everything
// needed before either allocation is released.
pub(crate) fn inflate_end_default_bound(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    let (window, state_ptr) = {
        let Some((bound_strm, state)) = inflateStateCheck(strm) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        (state.window, bound_strm.state)
    };
    if !window.is_null() {
        crate::src::zutil::zcfree(::core::ptr::null_mut(), window as crate::stdlib::voidpf);
    }
    crate::src::zutil::zcfree(::core::ptr::null_mut(), state_ptr as crate::stdlib::voidpf);
    clear_inflate_state(strm);
    crate::zlib_h::Z_OK
}

fn clear_inflate_state(strm: &mut crate::zlib_h::z_stream) {
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
}

// Keep the only post-release stream transition in a reference-bound helper.
// The callback plan above contains all values the deallocator can observe,
// so this helper does not need to retain state or allocator references.
fn inflate_end_complete(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    clear_inflate_state(strm);
    crate::zlib_h::Z_OK
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateEnd(strm)
}
fn inflate_get_dictionary(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    if state.whave == 0 {
        if let Some(dict_length) = dict_length {
            *dict_length = 0;
        }
        return crate::zlib_h::Z_OK;
    }
    let result = updatewindow(
        strm,
        state,
        InflateWindowAccess::Inspect,
        |state, window| {
            let window = window.expect("a nonempty inflater dictionary has a window");
            if let Some(dictionary) = dictionary {
                let whave = state.whave as usize;
                let wnext = state.wnext as usize;
                let first = whave - wnext;
                dictionary[..first].copy_from_slice(&window[wnext..whave]);
                dictionary[first..whave].copy_from_slice(&window[..wnext]);
            }
            if let Some(dict_length) = dict_length {
                *dict_length = state.whave as crate::stdlib::uInt;
            }
            crate::zlib_h::Z_OK
        },
    );
    result.unwrap_or(crate::zlib_h::Z_MEM_ERROR)
}
#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    // The ABI boundary validates the stream and binds optional caller output
    // storage. The named implementation binds its own state-owned window.
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let dictionary = if !dictionary.is_null() && state.whave != 0 {
        Some(::core::slice::from_raw_parts_mut(
            dictionary,
            state.whave as usize,
        ))
    } else {
        None
    };
    let dict_length = if dictLength.is_null() {
        None
    } else {
        Some(&mut *dictLength)
    };
    inflate_get_dictionary(strm, state, dictionary, dict_length)
}
pub fn inflateSetDictionary(
    strm: &mut crate::zlib_h::z_stream,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_set_dictionary(strm, state, dictionary)
}

// Once the ABI entry point has validated the stream/state pair and bound the
// optional dictionary range, dictionary installation is only state and slice
// work. Keep that transition out of the exported raw-pointer boundary.
fn inflate_set_dictionary(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if let Err(error) = inflate_dictionary_check(state, dictionary) {
        return error;
    }
    if updatewindow(
        strm,
        state,
        InflateWindowAccess::Update(dictionary),
        |_, _| (),
    )
    .is_err()
    {
        state.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    state.havedict = 1;
    crate::zlib_h::Z_OK
}

// zlib accepts an empty dictionary with a null pointer, but a non-empty
// dictionary must be backed by caller storage before the implementation binds
// it as a slice.
fn inflate_dictionary_input_is_valid(
    dictionary_is_null: bool,
    dictionary_len: crate::stdlib::uInt,
) -> bool {
    dictionary_len == 0 || !dictionary_is_null
}

fn inflate_dictionary_check(
    state: &crate::src::inflate::inflate_state,
    dictionary: &[crate::stdlib::Bytef],
) -> Result<(), ::core::ffi::c_int> {
    if state.wrap != 0
        && state.mode as ::core::ffi::c_uint
            != crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    if state.mode as ::core::ffi::c_uint
        == crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let dictid = crate::src::adler32::adler32_bytes(1, dictionary);
        if dictid != state.check {
            return Err(crate::zlib_h::Z_DATA_ERROR);
        }
    }
    Ok(())
}

#[export_name = "inflateSetDictionary"]

pub unsafe extern "C" fn inflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_dictionary_input_is_valid(dictionary.is_null(), dictLength) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // SAFETY: a non-empty dictionary is required to have caller-provided
    // backing storage for `dictLength` bytes; an empty dictionary needs no
    // pointer binding.
    let dictionary = if dictLength == 0 {
        &[]
    } else {
        unsafe { ::core::slice::from_raw_parts(dictionary, dictLength as usize) }
    };
    inflateSetDictionary(strm, dictionary)
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    // Bind foreign arguments only. State validation stays in the named
    // dispatcher, alongside the header transition it protects.
    inflateGetHeader(strm.as_mut(), head.as_mut())
}

fn inflateGetHeader(
    strm: Option<&mut crate::zlib_h::z_stream>,
    head: Option<&mut crate::zlib_h::gz_header>,
) -> ::core::ffi::c_int {
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(head) = head else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_get_header(state, head)
}

fn inflate_get_header(
    state: &mut crate::src::inflate::inflate_state,
    head: &mut crate::zlib_h::gz_header,
) -> ::core::ffi::c_int {
    if state.wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.head = head;
    head.done = 0 as ::core::ffi::c_int;
    crate::zlib_h::Z_OK
}

fn syncsearch_byte(
    mut have: ::core::ffi::c_uint,
    byte: ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    if byte as ::core::ffi::c_int
        == if have < 2 as ::core::ffi::c_uint {
            0 as ::core::ffi::c_int
        } else {
            0xff as ::core::ffi::c_int
        }
    {
        have = have.wrapping_add(1);
    } else if byte != 0 {
        have = 0 as ::core::ffi::c_uint;
    } else {
        have = (4 as ::core::ffi::c_uint).wrapping_sub(have);
    }
    have
}

fn syncsearch_bytes(
    have: &mut ::core::ffi::c_uint,
    buf: &[::core::ffi::c_uchar],
) -> ::core::ffi::c_uint {
    let mut got = *have;
    let mut next = 0usize;
    while next < buf.len() && got < 4 as ::core::ffi::c_uint {
        got = syncsearch_byte(got, buf[next]);
        next += 1;
    }
    *have = got;
    next as ::core::ffi::c_uint
}

// The stream input range is already bound by `inflateSync_ffi()`. Publish a
// consumed prefix through its slice tail so cursor movement does not require
// raw-pointer arithmetic. A zero-byte search must leave a possibly-null C
// cursor unchanged.
fn inflate_sync_consume_input(
    strm: &mut crate::zlib_h::z_stream,
    input: &[crate::stdlib::Bytef],
    consumed: ::core::ffi::c_uint,
) {
    strm.avail_in = strm.avail_in.wrapping_sub(consumed);
    if consumed != 0 {
        strm.next_in = input[consumed as usize..].as_ptr() as *mut crate::stdlib::Bytef;
    }
    strm.total_in = strm.total_in.wrapping_add(consumed as crate::stdlib::uLong);
}

// A successful sync resets decoding state but must retain the public byte
// counters and wrapper flags accumulated before the marker. Keep that
// transition in one safe helper so the cursor-search path cannot accidentally
// expose a partially reset stream.
fn inflate_sync_finish(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) {
    if state.flags == -1 as ::core::ffi::c_int {
        state.wrap = 0 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    let flags = state.flags;
    let total_in = strm.total_in;
    let total_out = strm.total_out;
    inflate_reset(strm, state);
    strm.total_in = total_in;
    strm.total_out = total_out;
    state.flags = flags;
    state.mode = crate::src::inflate::TYPE;
}

fn inflate_sync(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    input: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_uint = 0;
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    if strm.avail_in == 0 && state.bits < 8 as ::core::ffi::c_uint {
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
        syncsearch_bytes(&mut state.have, &buf[..len as usize]);
    }
    len = syncsearch_bytes(&mut state.have, input);
    inflate_sync_consume_input(strm, input, len);
    if state.have != 4 as ::core::ffi::c_uint {
        return crate::zlib_h::Z_DATA_ERROR;
    }
    inflate_sync_finish(strm, state);
    crate::zlib_h::Z_OK
}
#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let input = if strm.avail_in == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
    };
    inflate_sync(strm, state, input)
}
#[export_name = "inflateSyncPoint"]

pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter only binds the optional foreign stream. The
    // named implementation owns inflater-state validation.
    inflateSyncPoint(unsafe { strm.as_mut() })
}

pub fn inflateSyncPoint(strm: Option<&mut crate::zlib_h::z_stream>) -> ::core::ffi::c_int {
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_sync_point(state)
}
// The exported adapter below owns the foreign-call boundary. Keep this
// implementation callable through that dispatcher without exposing its raw
// allocation and cursor work as an unsafe-function contract to Rust callers.
pub fn inflateCopy(
    dest: &mut crate::zlib_h::z_stream,
    source: &mut crate::zlib_h::z_stream,
) -> ::core::ffi::c_int {
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    // Derive all source-state information needed after allocation before
    // calling the source allocator. This value snapshot prevents a Rust
    // reference to source state from spanning that user callback.
    let (plan, mut source_state, initial_zalloc, initial_opaque) = {
        let Some((source, state)) = inflateStateCheck(source) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Some(plan) = inflate_copy_plan(state) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        // C reads these first-allocation arguments before the callback. Keep
        // that small value snapshot rather than reopening the already-bound
        // source stream through its raw pointer.
        (
            plan,
            *state,
            source.zalloc.expect("non-null function pointer"),
            source.opaque,
        )
    };
    copy = Some(initial_zalloc).expect("non-null function pointer")(
        initial_opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if copy.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if let Some(window_len) = plan.window_len {
        // The first allocation callback may have changed the source stream's
        // callbacks or opaque value. Rebind it after that callback, then
        // snapshot the next allocation request before invoking it.
        let (zalloc, opaque) = {
            let Some((source, _source_state)) = inflateStateCheck(source) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            (
                source.zalloc.expect("non-null function pointer"),
                source.opaque,
            )
        };
        window = Some(zalloc).expect("non-null function pointer")(
            opaque,
            window_len as crate::stdlib::uInt,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if window.is_null() {
            // Match zlib's post-callback free lookup: the failed allocation
            // itself may have replaced the source release callback or opaque
            // value.
            let Some((source, _source_state)) = inflateStateCheck(source) else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            Some(source.zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(source.opaque, copy as crate::stdlib::voidpf);
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    // The allocator callbacks have completed. Reuse the checked stream/state
    // binding for the final stream copy instead of reopening `source` through
    // a raw dereference.
    let Some((source, _live_state)) = inflateStateCheck(source) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // `updatewindow()` only inspects this source-state snapshot, so it cannot
    // allocate or otherwise invoke a callback. Reusing that established
    // owner-window binder keeps the source history as a checked slice instead
    // of reopening the saved raw cursor here.
    let source_stream = *source;
    // SAFETY: the configured allocator returned this non-null allocation for
    // exactly one `inflate_state`; no allocator callback can run after this
    // binding. All other stream and window access below stays reference- or
    // slice-based.
    let copy = unsafe { &mut *copy };
    if window.is_null() {
        inflate_copy_state(dest, &source_stream, copy, &source_state, &plan, None);
    } else {
        updatewindow(
            source,
            &mut source_state,
            InflateWindowAccess::Inspect,
            |source_state, source_window| {
                let source_history = &source_window
                    .expect("source copy window is bound")[..plan.window_copy_len];
                inflate_copy_state(dest, &source_stream, copy, source_state, &plan, None);
                // Preserve the copied state's exact window metadata, then
                // populate its already-allocated storage through the shared
                // owned-window binder. This avoids reopening `window` here
                // as another raw slice.
                copy.window = window;
                updatewindow(
                    dest,
                    copy,
                    InflateWindowAccess::CopyFrom(source_history),
                    |_copy, _window| {},
                )
                .expect("a copied destination window is already allocated");
            },
        )
        .expect("a copied source window is already allocated");
    }
    dest.state =
        copy as *mut crate::src::inflate::inflate_state as *mut crate::src::deflate::internal_state;
    return crate::zlib_h::Z_OK;
}

struct InflateCopyPlan {
    window_len: Option<usize>,
    window_copy_len: usize,
    lencode: InflateCopyTableCursor,
    distcode: InflateCopyTableCursor,
    next: usize,
}

// A decode table either remains one of zlib's process-wide fixed tables or
// starts at a checked element in the source state's owned `codes` workspace.
// Retaining that distinction as scalar data lets `inflateCopy()` snapshot it
// before allocator callbacks and publish it into the destination afterwards.
enum InflateCopyTableCursor {
    Fixed,
    Codes(usize),
}

// Everything needed to decide whether a copied inflater owns a window is
// ordinary state inspection.  The surrounding `inflateCopy()` keeps the
// allocator and raw storage bindings at its existing ABI boundary.
fn inflate_copy_plan(state: &crate::src::inflate::inflate_state) -> Option<InflateCopyPlan> {
    let window_len = (!state.window.is_null()).then_some(inflate_window_layout(state.wbits).len);
    let window_copy_len = state.whave as usize;
    // A copied inflater's history must fit in the allocation derived from
    // its configured window bits. Validate that scalar relationship before
    // the ABI boundary forms a raw window slice below.
    if window_len.is_none() && window_copy_len != 0 {
        return None;
    }
    if let Some(window_len) = window_len {
        if window_copy_len > window_len {
            return None;
        }
    }
    let code_size = ::core::mem::size_of::<crate::src::inftrees::code>();
    let codes_base = state.codes.as_ptr().addr();
    let lencode = if ::core::ptr::eq(
        state.lencode,
        crate::src::inftrees::inffixed_h::lenfix.as_ptr(),
    ) {
        InflateCopyTableCursor::Fixed
    } else {
        InflateCopyTableCursor::Codes(inflate_codes_cursor_index(
            state.codes.len(),
            code_size,
            codes_base,
            state.lencode.addr(),
            false,
        )?)
    };
    let distcode = if ::core::ptr::eq(
        state.distcode,
        crate::src::inftrees::inffixed_h::distfix.as_ptr(),
    ) {
        InflateCopyTableCursor::Fixed
    } else {
        InflateCopyTableCursor::Codes(inflate_codes_cursor_index(
            state.codes.len(),
            code_size,
            codes_base,
            state.distcode.addr(),
            false,
        )?)
    };
    let next = inflate_codes_cursor_index(
        state.codes.len(),
        code_size,
        codes_base,
        state.next.addr(),
        true,
    )?;
    Some(InflateCopyPlan {
        window_len,
        window_copy_len,
        lencode,
        distcode,
        next,
    })
}

// Decode-table cursors in a live inflater either refer to its fixed tables or
// to an aligned element in its owned `codes` workspace. `next` may also be
// the one-past-workspace construction cursor. Derive those positions as
// indices before copying the state, so publishing the corresponding cursor in
// the new workspace only needs a checked slice tail instead of raw pointer
// arithmetic.
fn inflate_codes_cursor_index(
    codes_len: usize,
    code_size: usize,
    codes_base: usize,
    cursor: usize,
    allow_end: bool,
) -> Option<usize> {
    let bytes = codes_len.checked_mul(code_size)?;
    let offset = cursor.checked_sub(codes_base)?;
    if offset > bytes || (!allow_end && offset == bytes) || offset % code_size != 0 {
        return None;
    }
    Some(offset / code_size)
}

// Dynamic table construction happens entirely within an inflater's fixed
// `lens`, `codes`, and `work` arrays. Keep that path slice-bound: the raw
// `inflate_table()` entry point is only needed for its C-facing ABI.
fn inflate_build_dynamic_table(
    state: &mut crate::src::inflate::inflate_state,
    type_0: crate::src::inftrees::codetype,
    lens_start: usize,
    codes: ::core::ffi::c_uint,
    mut bits: ::core::ffi::c_uint,
) -> Result<::core::ffi::c_uint, ::core::ffi::c_int> {
    let lens_end = lens_start.checked_add(codes as usize).ok_or(1)?;
    let next = inflate_codes_cursor_index(
        state.codes.len(),
        ::core::mem::size_of::<crate::src::inftrees::code>(),
        state.codes.as_ptr().addr(),
        state.next.addr(),
        true,
    )
    .ok_or(1)?;
    let used = {
        let lens = state.lens.get(lens_start..lens_end).ok_or(1)?;
        let table = state.codes.get_mut(next..).ok_or(1)?;
        crate::src::inftrees::inflate_table_bound(type_0, lens, codes, table, &mut bits, &mut state.work)?
    };
    state.next = state.codes.get_mut(next + used..).ok_or(1)?.as_mut_ptr();
    Ok(bits)
}

// A live decoder uses either zlib's fixed tables or a bounded subrange of its
// own `codes` workspace.  Keep the raw cursor identity check beside the
// existing checked cursor-index conversion, so consumers of a decode table
// only receive a slice whose complete fast-path capacity has been verified.
pub(crate) enum InflateCodeTable {
    Length,
    Distance,
}

pub(crate) fn inflate_code_table(
    state: &crate::src::inflate::inflate_state,
    table: InflateCodeTable,
) -> Option<&[crate::src::inftrees::code]> {
    let (cursor, fixed, required) = match table {
        InflateCodeTable::Length => (
            state.lencode,
            crate::src::inftrees::inffixed_h::lenfix.as_ptr(),
            crate::src::inftrees::ENOUGH_LENS as usize,
        ),
        InflateCodeTable::Distance => (
            state.distcode,
            crate::src::inftrees::inffixed_h::distfix.as_ptr(),
            crate::src::inftrees::ENOUGH_DISTS as usize,
        ),
    };
    if ::core::ptr::eq(cursor, fixed) {
        return match table {
            InflateCodeTable::Length => Some(&crate::src::inftrees::inffixed_h::lenfix[..]),
            InflateCodeTable::Distance => Some(&crate::src::inftrees::inffixed_h::distfix[..]),
        };
    }
    let start = inflate_codes_cursor_index(
        state.codes.len(),
        ::core::mem::size_of::<crate::src::inftrees::code>(),
        state.codes.as_ptr().addr(),
        cursor.addr(),
        false,
    )?;
    state.codes.get(start..start.checked_add(required)?)
}

fn inflate_copy_state(
    dest: &mut crate::zlib_h::z_stream,
    source: &crate::zlib_h::z_stream,
    copy: &mut crate::src::inflate::inflate_state,
    state: &crate::src::inflate::inflate_state,
    plan: &InflateCopyPlan,
    window: Option<(&[crate::stdlib::Bytef], &mut [crate::stdlib::Bytef])>,
) {
    *dest = *source;
    *copy = *state;
    copy.strm = dest;

    // `plan` captured these cursor identities before allocator callbacks.
    // The state value snapshot retains raw cursors into the original source
    // workspace, so do not rediscover their offsets against the snapshot
    // array after those callbacks.
    copy.lencode = match plan.lencode {
        InflateCopyTableCursor::Fixed => crate::src::inftrees::inffixed_h::lenfix.as_ptr(),
        InflateCopyTableCursor::Codes(index) => copy.codes[index..].as_ptr(),
    };
    copy.distcode = match plan.distcode {
        InflateCopyTableCursor::Fixed => crate::src::inftrees::inffixed_h::distfix.as_ptr(),
        InflateCopyTableCursor::Codes(index) => copy.codes[index..].as_ptr(),
    };
    copy.next = copy.codes[plan.next..].as_mut_ptr();

    if let Some((source_window, dest_window)) = window {
        dest_window[..source_window.len()].copy_from_slice(source_window);
        copy.window = dest_window.as_mut_ptr();
    }
}
#[export_name = "inflateCopy"]

pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // The ABI adapter binds both foreign stream pointers; the named
    // implementation owns validation of their associated inflater states.
    if dest.is_null() || source.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest = unsafe { &mut *dest };
    let source = unsafe { &mut *source };
    inflateCopy(dest, source)
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter only binds the optional foreign stream. The
    // named implementation owns inflater-state validation.
    inflateUndermine(unsafe { strm.as_mut() }, subvert)
}

pub fn inflateUndermine(
    strm: Option<&mut crate::zlib_h::z_stream>,
    _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_undermine(state)
}
#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter only binds the optional foreign stream. The
    // named implementation owns inflater-state validation.
    inflateValidate(unsafe { strm.as_mut() }, check)
}

pub fn inflateValidate(
    strm: Option<&mut crate::zlib_h::z_stream>,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_validate(state, check)
}

fn inflate_sync_point(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_int {
    (state.mode as ::core::ffi::c_uint
        == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint
        && state.bits == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
}

fn inflate_undermine(state: &mut crate::src::inflate::inflate_state) -> ::core::ffi::c_int {
    state.sane = 1 as ::core::ffi::c_int;
    crate::zlib_h::Z_DATA_ERROR
}

fn inflate_validate(
    state: &mut crate::src::inflate::inflate_state,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if check != 0 && state.wrap != 0 {
        state.wrap |= 4 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    crate::zlib_h::Z_OK
}

fn inflate_mark(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_long {
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
#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    // SAFETY: this ABI adapter only binds the optional foreign stream. The
    // named implementation owns inflater-state validation.
    inflateMark(unsafe { strm.as_mut() })
}

pub fn inflateMark(strm: Option<&mut crate::zlib_h::z_stream>) -> ::core::ffi::c_long {
    let Some(strm) = strm else {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    };
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    };
    inflate_mark(state)
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    // SAFETY: this ABI adapter only binds the optional foreign stream. The
    // named implementation owns inflater-state validation.
    inflateCodesUsed(unsafe { strm.as_mut() })
}

pub fn inflateCodesUsed(
    strm: Option<&mut crate::zlib_h::z_stream>,
) -> ::core::ffi::c_ulong {
    let Some(strm) = strm else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    inflate_codes_used(state)
}

fn inflate_codes_used(state: &crate::src::inflate::inflate_state) -> crate::stdlib::uLong {
    let base = state.codes.as_ptr().addr();
    let next = state.next.addr();
    let size = ::core::mem::size_of::<crate::src::inftrees::code>();
    if next >= base {
        next.wrapping_sub(base).wrapping_div(size) as crate::stdlib::uLong
    } else {
        (base.wrapping_sub(next).wrapping_div(size) as ::core::ffi::c_long).wrapping_neg()
            as crate::stdlib::uLong
    }
}
