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

// This raw-state adapter is used only by translated Rust implementations.
// It validates the two associated allocations and then returns their bound
// references, so small inflater operations do not need to repeat raw stream
// and state dereferences after checking them.
unsafe fn inflateStateCheck<'a>(
    mut strm: crate::zlib_h::z_streamp,
) -> Option<(
    &'a mut crate::zlib_h::z_stream,
    &'a mut crate::src::inflate::inflate_state,
)> {
    if strm.is_null() {
        return None;
    }
    let strm_ref = &mut *strm;
    let state_ptr = strm_ref.state as *mut crate::src::inflate::inflate_state;
    if state_ptr.is_null() {
        return None;
    }
    let state = &mut *state_ptr;
    if !inflate_state_is_valid(strm_ref, state, state.strm == strm) {
        return None;
    }
    Some((strm_ref, state))
}

fn inflate_state_is_valid(
    strm: &crate::zlib_h::z_stream,
    state: &crate::src::inflate::inflate_state,
    points_back_to_stream: bool,
) -> bool {
    strm.zalloc.is_some()
        && strm.zfree.is_some()
        && points_back_to_stream
        && state.mode >= crate::src::inflate::HEAD
        && state.mode <= crate::src::inflate::SYNC
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
#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_reset_keep(strm, state)
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_reset_bound(strm, state)
}
pub unsafe extern "C" fn inflateReset2(
    mut strm: crate::zlib_h::z_streamp,
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
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            strm.opaque,
            state.window as crate::stdlib::voidpf,
        );
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
#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateReset2(strm, windowBits)
}
pub unsafe extern "C" fn inflateInit2_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if version.is_null()
        || *version as ::core::ffi::c_int
            != crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
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
    state = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    crate::stdlib::memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>(),
    );
    (*strm).state = state as *mut crate::src::deflate::internal_state;
    let strm = &mut *strm;
    let state = &mut *state;
    state.strm = strm;
    let ret = inflate_initialize_state(strm, state, windowBits);
    if ret != crate::zlib_h::Z_OK {
        Some(strm.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            strm.opaque,
            state as *mut crate::src::inflate::inflate_state as crate::stdlib::voidpf,
        );
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
    inflateInit2_(strm, windowBits, version, stream_size)
}
pub unsafe extern "C" fn inflateInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return inflateInit2_(strm, crate::zutil_h::DEF_WBITS, version, stream_size);
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit_(strm, version, stream_size)
}
#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_prime(state, bits, value)
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

fn update_window(
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [crate::stdlib::Bytef],
    end: &[crate::stdlib::Bytef],
) {
    if state.wsize == 0 {
        state.wsize = (1 as ::core::ffi::c_uint) << state.wbits;
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

// Updating a bound inflater window is an internal operation, not an ABI
// entry point. A first call with no output ensures the allocation exists; a
// second call consumes the produced output after that allocator callback has
// returned.
fn updatewindow(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    output: Option<&[crate::stdlib::Bytef]>,
) -> ::core::ffi::c_int {
    if state.window.is_null() {
        // SAFETY: zlib's initialized allocator is invoked with the same
        // window size and element count as the C implementation.
        state.window = unsafe {
            Some(stream.zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                stream.opaque,
                (1 as crate::stdlib::uInt) << state.wbits,
                ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
            ) as *mut ::core::ffi::c_uchar
        };
        if state.window.is_null() {
            return 1;
        }
    }
    let Some(output) = output else {
        return 0;
    };
    // SAFETY: a successful allocation above (or the initialized existing
    // window) has exactly the configured window length.
    let window = unsafe {
        ::core::slice::from_raw_parts_mut(
            state.window,
            ((1 as ::core::ffi::c_uint) << state.wbits) as usize,
        )
    };
    update_window(state, window, output);
    0
}
pub unsafe extern "C" fn inflate(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
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
    let mut from_window: bool = false;
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
    let mut output_start: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
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
    if inflateStateCheck(strm).is_none()
        || (*strm).next_out.is_null()
        || (*strm).next_in.is_null() && (*strm).avail_in != 0 as crate::stdlib::uInt
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).mode as ::core::ffi::c_uint
        == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*state).mode = crate::src::inflate::TYPEDO;
    }
    put = (*strm).next_out as *mut ::core::ffi::c_uchar;
    left = (*strm).avail_out as ::core::ffi::c_uint;
    output_start = put;
    output_capacity = left as usize;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    have = (*strm).avail_in as ::core::ffi::c_uint;
    let input = if have == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(next, have as usize)
    };
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
                                                                                                            next = next.wrapping_add(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        if (*state).wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if (*state).wbits == 0 as ::core::ffi::c_uint {
                                                                                                                (*state).wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            (*state).check = crate::src::crc32::crc32_bytes(
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
                                                                                                            (*state).mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            if !(*state).head.is_null() {
                                                                                                                (*(*state).head).done = -1 as ::core::ffi::c_int;
                                                                                                            }
                                                                                                            if (*state).wrap & 1 as ::core::ffi::c_int == 0
                                                                                                                || (((hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                                                                                                    .wrapping_rem(31 as ::core::ffi::c_ulong) != 0
                                                                                                            {
                                                                                                                (*strm).msg = b"incorrect header check\0".as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                (*strm).msg = b"unknown compression method\0".as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                                                                                                                    (*strm).msg = b"invalid window size\0".as_ptr()
                                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    (*state).dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    (*state).flags = 0 as ::core::ffi::c_int;
                                                                                                                    (*state).check = crate::src::adler32::adler32_buffer(
                                                                                                                        0 as crate::stdlib::uLong,
                                                                                                                        None,
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
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    (*state).flags = hold as ::core::ffi::c_int;
                                                                                                    if (*state).flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        (*strm).msg = b"unknown compression method\0".as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if (*state).flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        (*strm).msg = b"unknown header flags set\0".as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if !(*state).head.is_null() {
                                                                                                            (*(*state).head).text = (hold >> 8 as ::core::ffi::c_int
                                                                                                                & 1 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
                                                                                                        }
                                                                                                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                                                                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                        {
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            inflate_update_header_crc!(state, &hbuf[..2]);
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
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
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
                                                                                                        (*strm).msg = b"invalid stored block lengths\0".as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
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
                                                                                                        (*strm).msg = b"too many length or distance symbols\0"
                                                                                                            .as_ptr() as *const ::core::ffi::c_char
                                                                                                            as *mut ::core::ffi::c_char;
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
                                                                                                    let output = ::core::slice::from_raw_parts_mut(
                                                                                                        put,
                                                                                                        left as usize,
                                                                                                    );
                                                                                                    output[0] = (*state).length as ::core::ffi::c_uchar;
                                                                                                    put = output[1..].as_mut_ptr();
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
                                                                                                            next = next.wrapping_add(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
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
                                                                                                            let output = ::core::slice::from_raw_parts(
                                                                                                                output_start,
                                                                                                                output_capacity,
                                                                                                            );
                                                                                                            let output_start = output_capacity
                                                                                                                .wrapping_sub(left as usize)
                                                                                                                .wrapping_sub(out as usize);
                                                                                                            let output = &output[output_start..output_start + out as usize];
                                                                                                            (*state).check = (if (*state).flags != 0 {
                                                                                                                crate::src::crc32::crc32_bytes(
                                                                                                                    (*state).check as crate::stdlib::uLong,
                                                                                                                    output,
                                                                                                                )
                                                                                                            } else {
                                                                                                                crate::src::adler32::adler32_bytes(
                                                                                                                    (*state).check as crate::stdlib::uLong,
                                                                                                                    output,
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
                                                                                                            (*strm).msg = b"incorrect data check\0".as_ptr()
                                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                                                                                                    next = next.wrapping_add(1);
                                                                                                    hold = hold
                                                                                                        .wrapping_add(
                                                                                                            (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                }
                                                                                                if (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                    && hold
                                                                                                        != (*state).total & 0xffffffff as ::core::ffi::c_ulong
                                                                                                {
                                                                                                    (*strm).msg = b"incorrect length check\0".as_ptr()
                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                                                                                                next = next.wrapping_add(1);
                                                                                                hold = hold
                                                                                                    .wrapping_add(
                                                                                                        (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                                    );
                                                                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                            }
                                                                                            let c2rust_fresh15 = (*state).have;
                                                                                            (*state).have = (*state).have.wrapping_add(1);
                                                                                            (*state).lens[order[c2rust_fresh15 as usize] as usize] = (hold
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
                                                                                            (*state).lens[order[c2rust_fresh16 as usize] as usize] = 0
                                                                                                as ::core::ffi::c_ushort;
                                                                                        }
                                                                                        (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                                        (*state).distcode = (*state).next as *const crate::src::inftrees::code;
                                                                                        (*state).lencode = (*state).distcode;
                                                                                        (*state).lenbits = 7 as ::core::ffi::c_uint;
                                                                                        ret = crate::src::inftrees::inflate_table(
                                                                                            crate::src::inftrees::CODES,
                                                                                            &raw mut (*state).lens as *mut ::core::ffi::c_ushort,
                                                                                            19 as ::core::ffi::c_uint,
                                                                                            
                                                                                            &raw mut (*state).next as *mut _ as *mut *mut crate::src::inftrees::code,
                                                                                            &raw mut (*state).lenbits,
                                                                                            &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                                        );
                                                                                        if ret != 0
                                                                                        {
                                                                                            (*strm).msg = b"invalid code lengths set\0".as_ptr()
                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                                            continue '_inf_leave;
                                                                                        } else {
                                                                                            (*state).have = 0 as ::core::ffi::c_uint;
                                                                                            (*state).mode = crate::src::inflate::CODELENS;
                                                                                            break 's_1689;
                                                                                        }
                                                                                    }
                                                                                    if (*state).havedict == 0 as ::core::ffi::c_int {
                                                                                        (*strm).next_out = put as *mut crate::stdlib::Bytef;
                                                                                        (*strm).avail_out = left as crate::stdlib::uInt;
                                                                                        (*strm).next_in = next as *mut crate::stdlib::Bytef;
                                                                                        (*strm).avail_in = have as crate::stdlib::uInt;
                                                                                        (*state).hold = hold;
                                                                                        (*state).bits = bits;
                                                                                        return crate::zlib_h::Z_NEED_DICT;
                                                                                    }
                                                                                    (*state).check = crate::src::adler32::adler32_buffer(
                                                                                        0 as crate::stdlib::uLong,
                                                                                        None,
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
                                                                                    next = next.wrapping_add(1);
                                                                                    hold = hold
                                                                                        .wrapping_add(
                                                                                            (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                        );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                if !(*state)
                                                                                    .head
                                                                                    .is_null()
                                                                                {
                                                                                    (*(*state)
                                                                                        .head)
                                                                                        .time = hold
                                                                                        as crate::stdlib::uLong;
                                                                                }
                                                                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
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
                                                                                here = inflate_lencode!(
                                                                                    state,
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
                                                                                        next = next.wrapping_add(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    if (*state).have == 0 as ::core::ffi::c_uint {
                                                                                        (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                                                                                if (*state).have.wrapping_add(copy)
                                                                                    > (*state).nlen.wrapping_add((*state).ndist)
                                                                                {
                                                                                    (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                                                                        if (*state).lens[256 as ::core::ffi::c_int as usize]
                                                                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                                        {
                                                                            (*strm).msg = b"invalid code -- missing end-of-block\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                            (*state).lencode = (*state).next as *const crate::src::inftrees::code;
                                                                            (*state).lenbits = 9 as ::core::ffi::c_uint;
                                                                            ret = crate::src::inftrees::inflate_table(
                                                                                crate::src::inftrees::LENS,
                                                                                &raw mut (*state).lens as *mut ::core::ffi::c_ushort,
                                                                                (*state).nlen,
                                                                                
                                                                                &raw mut (*state).next as *mut _ as *mut *mut crate::src::inftrees::code,
                                                                                &raw mut (*state).lenbits,
                                                                                &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                            );
                                                                            if ret != 0 {
                                                                                (*strm).msg = b"invalid literal/lengths set\0".as_ptr()
                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                (*state).distcode = (*state).next as *const crate::src::inftrees::code;
                                                                                (*state).distbits = 6 as ::core::ffi::c_uint;
                                                                                ret = crate::src::inftrees::inflate_table(
                                                                                    crate::src::inftrees::DISTS,
                                                                                    // `nlen` is checked against the
                                                                                    // literal/length-code limit before
                                                                                    // reaching this state. Bind the
                                                                                    // remaining fixed state array instead
                                                                                    // of deriving its cursor with raw
                                                                                    // pointer arithmetic.
                                                                                    (&mut (*state).lens)[(*state).nlen as usize..]
                                                                                        .as_mut_ptr(),
                                                                                    (*state).ndist,
                                                                                    
                                                                                    &raw mut (*state).next as *mut _ as *mut *mut crate::src::inftrees::code,
                                                                                    &raw mut (*state).distbits,
                                                                                    &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                                );
                                                                                if ret != 0 {
                                                                                    (*strm).msg = b"invalid distances set\0".as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                                                                        let input_start = in_0
                                                                            .wrapping_sub(have)
                                                                            as usize;
                                                                        let output = ::core::slice::from_raw_parts_mut(
                                                                            put,
                                                                            copy as usize,
                                                                        );
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
                                                                if !(*state).head.is_null() {
                                                                    (*(*state).head).xflags = (hold
                                                                        & 0xff
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int;
                                                                    (*(*state).head).os = (hold
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
                                                                (*state).mode =
                                                                    crate::src::inflate::TABLE;
                                                            }
                                                            _ => {
                                                                (*strm).msg = b"invalid block type\0".as_ptr()
                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
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
                                                        next = next.wrapping_add(1);
                                                        hold = hold.wrapping_add(
                                                            (inflate_input_byte!(input, in_0, have)
                                                                as ::core::ffi::c_ulong)
                                                                << bits,
                                                        );
                                                        bits = bits
                                                            .wrapping_add(8 as ::core::ffi::c_uint);
                                                    }
                                                    (*state).length = hold as ::core::ffi::c_uint;
                                                    if !(*state).head.is_null() {
                                                        (*(*state).head).extra_len = hold
                                                            as ::core::ffi::c_uint
                                                            as crate::stdlib::uInt;
                                                    }
                                                    if (*state).flags & 0x200 as ::core::ffi::c_int
                                                        != 0
                                                        && (*state).wrap & 4 as ::core::ffi::c_int
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
                                                } else if !(*state).head.is_null() {
                                                    (*(*state).head).extra = ::core::ptr::null_mut::<
                                                        crate::stdlib::Bytef,
                                                    >(
                                                    );
                                                }
                                                (*state).mode = crate::src::inflate::EXTRA;
                                                break 'c_2319;
                                            }
                                            (*state).mode = crate::src::inflate::LEN;
                                        }
                                        if have >= 6 as ::core::ffi::c_uint
                                            && left >= 258 as ::core::ffi::c_uint
                                        {
                                            (*strm).next_out = put as *mut crate::stdlib::Bytef;
                                            (*strm).avail_out = left as crate::stdlib::uInt;
                                            (*strm).next_in = next as *mut crate::stdlib::Bytef;
                                            (*strm).avail_in = have as crate::stdlib::uInt;
                                            (*state).hold = hold;
                                            (*state).bits = bits;
                                            crate::src::inffast::inflate_fast(
                                                strm as *mut crate::zlib_h::z_stream_s,
                                                out,
                                            );
                                            put = (*strm).next_out as *mut ::core::ffi::c_uchar;
                                            left = (*strm).avail_out as ::core::ffi::c_uint;
                                            next = (*strm).next_in as *mut ::core::ffi::c_uchar;
                                            have = (*strm).avail_in as ::core::ffi::c_uint;
                                            hold = (*state).hold;
                                            bits = (*state).bits;
                                            if (*state).mode as ::core::ffi::c_uint
                                                == crate::src::inflate::TYPE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                (*state).back = -1 as ::core::ffi::c_int;
                                            }
                                            continue '_inf_leave;
                                        } else {
                                            (*state).back = 0 as ::core::ffi::c_int;
                                            loop {
                                                here = inflate_lencode!(
                                                    state,
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
                                                (*state).back += last.bits as ::core::ffi::c_int;
                                            }
                                            hold >>= here.bits as ::core::ffi::c_int;
                                            bits =
                                                bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                            (*state).back += here.bits as ::core::ffi::c_int;
                                            (*state).length = here.val as ::core::ffi::c_uint;
                                            if here.op as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
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
                                                (*strm).msg = b"invalid literal/length code\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char;
                                                (*state).mode = crate::src::inflate::BAD;
                                                continue '_inf_leave;
                                            } else {
                                                (*state).extra = here.op as ::core::ffi::c_uint
                                                    & 15 as ::core::ffi::c_uint;
                                                (*state).mode = crate::src::inflate::LENEXT;
                                                break 'c_2410;
                                            }
                                        }
                                    }
                                    if (*state).flags & 0x400 as ::core::ffi::c_int != 0 {
                                        copy = (*state).length;
                                        if copy > have {
                                            copy = have;
                                        }
                                        if copy != 0 {
                                            if !(*state).head.is_null() {
                                                let head = &mut *(*state).head;
                                                let extra_offset = (head.extra_len
                                                    as ::core::ffi::c_uint)
                                                    .wrapping_sub((*state).length);
                                                if !head.extra.is_null()
                                                    && extra_offset < head.extra_max
                                                {
                                                    let copy_len = copy.min(
                                                        (head.extra_max as ::core::ffi::c_uint)
                                                            .wrapping_sub(extra_offset),
                                                    )
                                                        as usize;
                                                    let extra = ::core::slice::from_raw_parts_mut(
                                                        head.extra,
                                                        head.extra_max as usize,
                                                    );
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
                                            if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                && (*state).wrap & 4 as ::core::ffi::c_int != 0
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
                                        next = next.wrapping_add(1);
                                        hold = hold.wrapping_add(
                                            (inflate_input_byte!(input, in_0, have)
                                                as ::core::ffi::c_ulong)
                                                << bits,
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
                                    len = input[in_0.wrapping_sub(have).wrapping_add(c2rust_fresh5)
                                        as usize]
                                        as ::core::ffi::c_uint;
                                    if !(*state).head.is_null() {
                                        let head = &mut *(*state).head;
                                        if !head.name.is_null() && (*state).length < head.name_max {
                                            let name = ::core::slice::from_raw_parts_mut(
                                                head.name,
                                                head.name_max as usize,
                                            );
                                            name[(*state).length as usize] =
                                                len as crate::stdlib::Bytef;
                                            (*state).length = (*state).length.wrapping_add(1);
                                        }
                                    }
                                    if !(len != 0 && copy < have) {
                                        break;
                                    }
                                }
                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
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
                            } else if !(*state).head.is_null() {
                                (*(*state).head).name =
                                    ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                            }
                            (*state).length = 0 as ::core::ffi::c_uint;
                            (*state).mode = crate::src::inflate::COMMENT;
                            break 'c_2325;
                        }
                        loop {
                            here = inflate_distcode!(
                                state,
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
                            (*state).back += last.bits as ::core::ffi::c_int;
                        }
                        hold >>= here.bits as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                        (*state).back += here.bits as ::core::ffi::c_int;
                        if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                            (*strm).msg = b"invalid distance code\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
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
                            len = input
                                [in_0.wrapping_sub(have).wrapping_add(c2rust_fresh7) as usize]
                                as ::core::ffi::c_uint;
                            if !(*state).head.is_null() {
                                let head = &mut *(*state).head;
                                if !head.comment.is_null() && (*state).length < head.comm_max {
                                    let comment = ::core::slice::from_raw_parts_mut(
                                        head.comment,
                                        head.comm_max as usize,
                                    );
                                    comment[(*state).length as usize] = len as crate::stdlib::Bytef;
                                    (*state).length = (*state).length.wrapping_add(1);
                                }
                            }
                            if !(len != 0 && copy < have) {
                                break;
                            }
                        }
                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
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
                    } else if !(*state).head.is_null() {
                        (*(*state).head).comment = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
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
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add(
                            (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong)
                                << bits,
                        );
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
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add(
                        (inflate_input_byte!(input, in_0, have) as ::core::ffi::c_ulong) << bits,
                    );
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if (*state).wrap & 4 as ::core::ffi::c_int != 0
                    && hold != (*state).check & 0xffff as ::core::ffi::c_ulong
                {
                    (*strm).msg = b"header crc mismatch\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue '_inf_leave;
                } else {
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                }
            }
            if !(*state).head.is_null() {
                (*(*state).head).hcrc =
                    (*state).flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                (*(*state).head).done = 1 as ::core::ffi::c_int;
            }
            (*state).check = crate::src::crc32::crc32_bytes(0 as crate::stdlib::uLong, &[])
                as ::core::ffi::c_ulong;
            (*strm).adler = (*state).check as crate::stdlib::uLong;
            (*state).mode = crate::src::inflate::TYPE;
            continue '_inf_leave;
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        copy = out.wrapping_sub(left);
        from_window = false;
        if (*state).offset > copy {
            from_window = true;
            copy = (*state).offset.wrapping_sub(copy);
            if copy > (*state).whave {
                if (*state).sane != 0 {
                    (*strm).msg = b"invalid distance too far back\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                }
            }
            if copy > (*state).wnext {
                copy = copy.wrapping_sub((*state).wnext);
                from = (*state)
                    .window
                    .wrapping_add((*state).wsize.wrapping_sub(copy) as usize);
            } else {
                from = (*state)
                    .window
                    .wrapping_add((*state).wnext.wrapping_sub(copy) as usize);
            }
            if copy > (*state).length {
                copy = (*state).length;
            }
        } else {
            from = put.wrapping_sub((*state).offset as usize);
            copy = (*state).length;
        }
        if copy > left {
            copy = left;
        }
        left = left.wrapping_sub(copy);
        (*state).length = (*state).length.wrapping_sub(copy);
        let copy_len = copy as usize;
        if from_window {
            let source = ::core::slice::from_raw_parts(from, copy_len);
            let output = ::core::slice::from_raw_parts_mut(put, copy_len);
            output.copy_from_slice(source);
            put = output[copy_len..].as_mut_ptr();
        } else {
            let distance = (*state).offset as usize;
            let output = ::core::slice::from_raw_parts_mut(from, distance + copy_len);
            for index in distance..distance + copy_len {
                output[index] = output[index - distance];
            }
            put = output[distance + copy_len..].as_mut_ptr();
        }
        if (*state).length == 0 as ::core::ffi::c_uint {
            (*state).mode = crate::src::inflate::LEN;
        }
    }
    (*strm).next_out = put as *mut crate::stdlib::Bytef;
    (*strm).avail_out = left as crate::stdlib::uInt;
    (*strm).next_in = next as *mut crate::stdlib::Bytef;
    (*strm).avail_in = have as crate::stdlib::uInt;
    (*state).hold = hold;
    (*state).bits = bits;
    let produced = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
    let update_window = (*state).wsize != 0
        || out != (*strm).avail_out
            && ((*state).mode as ::core::ffi::c_uint)
                < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
            && (((*state).mode as ::core::ffi::c_uint)
                < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                || flush != crate::zlib_h::Z_FINISH);
    let window_binding = if update_window {
        let stream = &mut *strm;
        let state = &mut *state;
        if updatewindow(stream, state, None) != 0 {
            state.mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
        Some((stream, state))
    } else {
        None
    };
    let output = if produced == 0 {
        &[]
    } else {
        // SAFETY: the validated output cursor advanced by exactly `produced`
        // bytes during this call, so this is the completed output range.
        ::core::slice::from_raw_parts(put.wrapping_sub(produced as usize), produced as usize)
    };
    if let Some((stream, state)) = window_binding {
        updatewindow(stream, state, Some(output));
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in as ::core::ffi::c_uint);
    out = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
    (*strm).total_in = (*strm).total_in.wrapping_add(in_0 as crate::stdlib::uLong);
    (*strm).total_out = (*strm).total_out.wrapping_add(out as crate::stdlib::uLong);
    (*state).total = (*state).total.wrapping_add(out as ::core::ffi::c_ulong);
    if (*state).wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
        (*state).check = (if (*state).flags != 0 {
            crate::src::crc32::crc32_bytes((*state).check as crate::stdlib::uLong, output)
        } else {
            crate::src::adler32::adler32_bytes((*state).check as crate::stdlib::uLong, output)
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
    return ret;
}
#[export_name = "inflate"]

pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate(strm, flush)
}
pub unsafe extern "C" fn inflateEnd(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
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
        Some(zfree).expect("non-null function pointer")(opaque, window as crate::stdlib::voidpf);
    }
    Some(zfree).expect("non-null function pointer")(opaque, state_ptr as crate::stdlib::voidpf);
    clear_inflate_state(strm);
    crate::zlib_h::Z_OK
}

fn clear_inflate_state(strm: &mut crate::zlib_h::z_stream) {
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    inflateEnd(strm)
}
fn inflate_get_dictionary(
    state: &crate::src::inflate::inflate_state,
    window: Option<&[crate::stdlib::Bytef]>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    if let (Some(window), Some(dictionary)) = (window, dictionary) {
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
}
#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    // The ABI boundary validates the stream and binds optional caller output
    // storage. The named implementation only operates on those references.
    let Some((_strm, state)) = inflateStateCheck(strm) else {
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
    let window = if state.whave != 0 {
        Some(::core::slice::from_raw_parts(
            state.window,
            state.wsize as usize,
        ))
    } else {
        None
    };
    let dict_length = if dictLength.is_null() {
        None
    } else {
        Some(&mut *dictLength)
    };
    inflate_get_dictionary(state, window, dictionary, dict_length)
}
pub unsafe extern "C" fn inflateSetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_dictionary_input_is_valid(dictionary.is_null(), dictLength) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = if dictLength == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(dictionary, dictLength as usize)
    };
    if let Err(error) = inflate_dictionary_check(state, dictionary) {
        return error;
    }
    if state.window.is_null() {
        state.window = Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
            (1 as crate::stdlib::uInt) << state.wbits,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
    }
    if state.window.is_null() {
        state.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    let window = ::core::slice::from_raw_parts_mut(
        state.window,
        ((1 as ::core::ffi::c_uint) << state.wbits) as usize,
    );
    inflate_set_dictionary(state, window, dictionary)
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

fn inflate_set_dictionary(
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [crate::stdlib::Bytef],
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    update_window(state, window, dictionary);
    state.havedict = 1;
    crate::zlib_h::Z_OK
}
#[export_name = "inflateSetDictionary"]

pub unsafe extern "C" fn inflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    inflateSetDictionary(strm, dictionary, dictLength)
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    // This is the C ABI boundary: validate and bind the stream before
    // binding the caller-provided header storage.
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_get_header(state, &mut *head)
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
    if state.flags == -1 as ::core::ffi::c_int {
        state.wrap = 0 as ::core::ffi::c_int;
    } else {
        state.wrap &= !(4 as ::core::ffi::c_int);
    }
    let flags = state.flags;
    let in_0 = strm.total_in;
    let out = strm.total_out;
    inflate_reset(strm, state);
    strm.total_in = in_0;
    strm.total_out = out;
    state.flags = flags;
    state.mode = crate::src::inflate::TYPE;
    crate::zlib_h::Z_OK
}
#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
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
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_sync_point(state)
}
pub unsafe extern "C" fn inflateCopy(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if inflateStateCheck(source).is_none() || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*source).state as *mut crate::src::inflate::inflate_state;
    let state = &*state;
    let plan = inflate_copy_plan(state);
    copy = Some((*source).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*source).opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if copy.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    crate::stdlib::memset(
        copy as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>(),
    );
    window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if let Some(window_len) = plan.window_len {
        window = Some((*source).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*source).opaque,
            window_len as crate::stdlib::uInt,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if window.is_null() {
            Some((*source).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*source).opaque,
                copy as crate::stdlib::voidpf,
            );
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    let source = &*source;
    let dest = &mut *dest;
    let copy = &mut *copy;
    let window = if window.is_null() {
        None
    } else {
        Some((
            ::core::slice::from_raw_parts(state.window, state.whave as usize),
            ::core::slice::from_raw_parts_mut(
                window,
                plan.window_len.expect("window allocation has a length"),
            ),
        ))
    };
    inflate_copy_state(dest, source, copy, state, window);
    dest.state =
        copy as *mut crate::src::inflate::inflate_state as *mut crate::src::deflate::internal_state;
    return crate::zlib_h::Z_OK;
}

struct InflateCopyPlan {
    window_len: Option<usize>,
}

// Everything needed to decide whether a copied inflater owns a window is
// ordinary state inspection.  The surrounding `inflateCopy()` keeps the
// allocator and raw storage bindings at its existing ABI boundary.
fn inflate_copy_plan(state: &crate::src::inflate::inflate_state) -> InflateCopyPlan {
    InflateCopyPlan {
        window_len: (!state.window.is_null()).then_some((1usize) << state.wbits),
    }
}

fn inflate_copy_state(
    dest: &mut crate::zlib_h::z_stream,
    source: &crate::zlib_h::z_stream,
    copy: &mut crate::src::inflate::inflate_state,
    state: &crate::src::inflate::inflate_state,
    window: Option<(&[crate::stdlib::Bytef], &mut [crate::stdlib::Bytef])>,
) {
    *dest = *source;
    *copy = *state;
    copy.strm = dest;

    let code_size = ::core::mem::size_of::<crate::src::inftrees::code>();
    let source_codes = state.codes.as_ptr().addr();
    let source_end = source_codes.wrapping_add(state.codes.len().wrapping_mul(code_size));
    let lencode = state.lencode.addr();
    if lencode >= source_codes && lencode < source_end {
        let lencode_index = lencode.wrapping_sub(source_codes).wrapping_div(code_size);
        let distcode_index = state
            .distcode
            .addr()
            .wrapping_sub(source_codes)
            .wrapping_div(code_size);
        copy.lencode = copy.codes.as_ptr().wrapping_add(lencode_index);
        copy.distcode = copy.codes.as_ptr().wrapping_add(distcode_index);
    }
    let next_index = state
        .next
        .addr()
        .wrapping_sub(source_codes)
        .wrapping_div(code_size);
    copy.next = copy.codes.as_mut_ptr().wrapping_add(next_index);

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
    inflateCopy(dest, source)
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    let Some((_strm, state)) = inflateStateCheck(strm) else {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    };
    inflate_mark(state)
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
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
