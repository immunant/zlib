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
        && state.strm.cast_const() == ::core::ptr::from_ref(strm)
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
    state.next = state.codes.as_mut_ptr();
    state.distcode = state.next;
    state.lencode = state.distcode;
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
pub fn inflateReset2(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wrap: ::core::ffi::c_int = 0;
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if windowBits < 0 as ::core::ffi::c_int {
        if windowBits < -15 as ::core::ffi::c_int {
            return crate::zlib_h::Z_STREAM_ERROR;
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
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !state.window.is_null() && state.wbits != windowBits as ::core::ffi::c_uint {
        // The validated stream owns this existing window allocation. The
        // callback remains the ABI boundary for custom allocators.
        unsafe {
            Some(strm.zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                strm.opaque,
                state.window as crate::stdlib::voidpf,
            );
        }
        state.window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    state.wrap = wrap;
    state.wbits = windowBits as ::core::ffi::c_uint;
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
    inflateReset2(strm, state, windowBits)
}
pub fn inflateInit2_(
    strm: &mut crate::zlib_h::z_stream,
    mut windowBits: ::core::ffi::c_int,
    version: &::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // This legacy allocator/state bridge is the sole raw portion of the
    // initializer. Callers use the safe named initializer below.
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        let mut state: *mut crate::src::inflate::inflate_state =
            ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
        if *version as ::core::ffi::c_int
            != crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
            || stream_size
                != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
        {
            return crate::zlib_h::Z_VERSION_ERROR;
        }
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
        state = Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
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
        strm.state = state as *mut crate::src::deflate::internal_state;
        let state_ref = &mut *state;
        state_ref.strm = ::core::ptr::from_mut(strm);
        state_ref.window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        state_ref.mode = crate::src::inflate::HEAD;
        ret = inflateReset2(strm, state_ref, windowBits);
        if ret != crate::zlib_h::Z_OK {
            Some(strm.zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                strm.opaque,
                state as crate::stdlib::voidpf,
            );
            strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
        }
        ret
    }
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
    if state.window.is_null() {
        let Some(zalloc) = strm.zalloc else {
            return 1 as ::core::ffi::c_int;
        };
        state.window = unsafe {
            zalloc(
                strm.opaque,
                (1 as crate::stdlib::uInt) << state.wbits,
                ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
            ) as *mut ::core::ffi::c_uchar
        };
        if state.window.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    if state.wsize == 0 as ::core::ffi::c_uint {
        state.wsize = (1 as ::core::ffi::c_uint) << state.wbits;
        state.wnext = 0 as ::core::ffi::c_uint;
        state.whave = 0 as ::core::ffi::c_uint;
    }
    // Allocation above establishes the window's `wsize` bytes for this
    // initialized stream state.  Keep the ABI allocation handle confined to
    // this bridge; callers use only references and slices.
    let window = unsafe { ::core::slice::from_raw_parts_mut(state.window, state.wsize as usize) };
    update_window(state, window, input).is_err() as ::core::ffi::c_int
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
        let mut state = state_ref as *mut crate::src::inflate::inflate_state;
        let strm = strm as *mut crate::zlib_h::z_stream;
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
        if (*state).mode as ::core::ffi::c_uint
            == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*state).mode = crate::src::inflate::TYPEDO;
        }
        put = (*strm).next_out as *mut ::core::ffi::c_uchar;
        left = (*strm).avail_out as ::core::ffi::c_uint;
        next = (*strm).next_in as *mut ::core::ffi::c_uchar;
        have = (*strm).avail_in as ::core::ffi::c_uint;
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
                                                                                                            next = next.offset(1);
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
                                                                                                            (*state).check = crate::src::crc32::crc32(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            (*state).check = crate::src::crc32::crc32(
                                                                                                                (*state).check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
                                                                                                            ) as ::core::ffi::c_ulong;
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
                                                                                                                (*strm).msg = INFLATE_MESSAGES[0].as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                (*strm).msg = INFLATE_MESSAGES[1].as_ptr()
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
                                                                                                                    (*strm).msg = INFLATE_MESSAGES[2].as_ptr()
                                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    (*state).dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    (*state).flags = 0 as ::core::ffi::c_int;
                                                                                                                    (*state).check = crate::src::adler32::adler32(
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
                                                                                                        let c2rust_fresh1 = next;
                                                                                                        next = next.offset(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (*c2rust_fresh1 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    (*state).flags = hold as ::core::ffi::c_int;
                                                                                                    if (*state).flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        (*strm).msg = INFLATE_MESSAGES[1].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if (*state).flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        (*strm).msg = INFLATE_MESSAGES[3].as_ptr()
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
                                                                                                            (*state).check = crate::src::crc32::crc32(
                                                                                                                (*state).check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
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
                                                                                                        next = next.offset(1);
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
                                                                                                        next = next.offset(1);
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
                                                                                                        (*strm).msg = INFLATE_MESSAGES[4].as_ptr()
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
                                                                                                        let c2rust_fresh13 = next;
                                                                                                        next = next.offset(1);
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
                                                                                                                (*strm).msg = INFLATE_MESSAGES[5]
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
                                                                                                    let c2rust_fresh32 = put;
                                                                                                    put = put.offset(1);
                                                                                                    *c2rust_fresh32 = (*state).length as ::core::ffi::c_uchar;
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
                                                                                                            next = next.offset(1);
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
                                                                                                                    Some(::core::slice::from_raw_parts(
                                                                                                                        put.offset(-(out as isize)),
                                                                                                                        out as usize,
                                                                                                                    )),
                                                                                                                )
                                                                                                            } else {
                                                                                                                crate::src::adler32::adler32(
                                                                                                                    (*state).check as crate::stdlib::uLong,
                                                                                                                    Some(::core::slice::from_raw_parts(
                                                                                                                        put.offset(-(out as isize)),
                                                                                                                        out as crate::stdlib::z_size_t,
                                                                                                                    )),
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
                                                                                                    (*strm).msg = INFLATE_MESSAGES[6].as_ptr()
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
                                                                                                    let c2rust_fresh34 = next;
                                                                                                    next = next.offset(1);
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
                                                                                            (*strm).msg = INFLATE_MESSAGES[7].as_ptr()
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
                                                                                                let c2rust_fresh14 = next;
                                                                                                next = next.offset(1);
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
                                                                                            (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                                            (*state).distcode = (*state).next as *const crate::src::inftrees::code;
                                                                                            (*state).lencode = (*state).distcode;
                                                                                            (*state).lenbits = 7 as ::core::ffi::c_uint;
                                                                                            ret = match crate::src::inftrees::inflate_table(
                                                                                            crate::src::inftrees::CODES,
                                                                                            &(&(*state).lens)[..19],
                                                                                            &mut (*state).codes,
                                                                                            &mut (*state).lenbits,
                                                                                            &mut (&mut (*state).work)[..19],
                                                                                        ) {
                                                                                            Ok(used) => {
                                                                                                (*state).next = (&raw mut (*state).codes as *mut crate::src::inftrees::code).add(used);
                                                                                                0
                                                                                            }
                                                                                            Err(error) => error,
                                                                                        };
                                                                                            if ret
                                                                                                != 0
                                                                                            {
                                                                                                (*strm).msg = INFLATE_MESSAGES[8].as_ptr()
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
                                                                                        (*state).check = crate::src::adler32::adler32(
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
                                                                                    let c2rust_fresh2 = next;
                                                                                    next = next.offset(1);
                                                                                    hold = hold
                                                                                        .wrapping_add(
                                                                                            (*c2rust_fresh2 as ::core::ffi::c_ulong) << bits,
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
                                                                                    (*state).check = crate::src::crc32::crc32(
                                                                                        (*state).check as crate::stdlib::uLong,
                                                                                        Some(&hbuf[..4]),
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
                                                                                        (*state)
                                                                                            .ndist,
                                                                                    )
                                                                            {
                                                                                loop {
                                                                                    here = *(*state)
                                                                                    .lencode
                                                                                    .offset(
                                                                                        (hold as ::core::ffi::c_uint
                                                                                            & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                                                                                .wrapping_sub(1 as ::core::ffi::c_uint)) as isize,
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
                                                                                        .offset(1);
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
                                                                                        next = next.offset(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (*c2rust_fresh19 as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    if (*state).have == 0 as ::core::ffi::c_uint {
                                                                                        (*strm).msg = INFLATE_MESSAGES[9].as_ptr()
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
                                                                                        let c2rust_fresh20 = next;
                                                                                        next = next.offset(1);
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
                                                                                        next = next.offset(1);
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
                                                                            (*strm).msg = INFLATE_MESSAGES[9].as_ptr()
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
                                                                            (*strm).msg = INFLATE_MESSAGES[10]
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                            (*state).lencode = (*state).next as *const crate::src::inftrees::code;
                                                                            (*state).lenbits = 9 as ::core::ffi::c_uint;
                                                                            ret = match crate::src::inftrees::inflate_table(
                                                                                crate::src::inftrees::LENS,
                                                                                &(&(*state).lens)[..(*state).nlen as usize],
                                                                                &mut (*state).codes,
                                                                                &mut (*state).lenbits,
                                                                                &mut (&mut (*state).work)[..(*state).nlen as usize],
                                                                            ) {
                                                                                Ok(used) => {
                                                                                    (*state).next = (&raw mut (*state).codes as *mut crate::src::inftrees::code).add(used);
                                                                                    0
                                                                                }
                                                                                Err(error) => error,
                                                                            };
                                                                            if ret != 0 {
                                                                                (*strm).msg = INFLATE_MESSAGES[11].as_ptr()
                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                (*state).distcode = (*state).next as *const crate::src::inftrees::code;
                                                                                (*state).distbits = 6 as ::core::ffi::c_uint;
                                                                                let table_start = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                                let table_used = (*state).next.offset_from(table_start) as usize;
                                                                                ret = match crate::src::inftrees::inflate_table(
                                                                                    crate::src::inftrees::DISTS,
                                                                                    &(&(*state).lens)[(*state).nlen as usize..((*state).nlen + (*state).ndist) as usize],
                                                                                    &mut (&mut (*state).codes)[table_used..],
                                                                                    &mut (*state).distbits,
                                                                                    &mut (&mut (*state).work)[..(*state).ndist as usize],
                                                                                ) {
                                                                                    Ok(used) => {
                                                                                        (*state).next = table_start.add(table_used + used);
                                                                                        0
                                                                                    }
                                                                                    Err(error) => error,
                                                                                };
                                                                                if ret != 0 {
                                                                                    (*strm).msg = INFLATE_MESSAGES[12].as_ptr()
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
                                                                            crate::stdlib::memcpy(
                                                                            put as *mut ::core::ffi::c_void,
                                                                            next as *const ::core::ffi::c_void,
                                                                            copy as crate::__stddef_size_t_h::size_t,
                                                                        );
                                                                            have = have
                                                                                .wrapping_sub(copy);
                                                                            next = next.offset(
                                                                                copy as isize,
                                                                            );
                                                                            left = left
                                                                                .wrapping_sub(copy);
                                                                            put = put.offset(
                                                                                copy as isize,
                                                                            );
                                                                            (*state).length =
                                                                                (*state)
                                                                                    .length
                                                                                    .wrapping_sub(
                                                                                        copy,
                                                                                    );
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
                                                                        if have == 0
                                                                            as ::core::ffi::c_uint
                                                                        {
                                                                            break '_inf_leave;
                                                                        }
                                                                        have = have.wrapping_sub(1);
                                                                        let c2rust_fresh3 = next;
                                                                        next = next.offset(1);
                                                                        hold = hold.wrapping_add(
                                                                        (*c2rust_fresh3
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
                                                                    (*state).check = crate::src::crc32::crc32(
                                                                        (*state).check as crate::stdlib::uLong,
                                                                        Some(&hbuf[..2]),
                                                                    ) as ::core::ffi::c_ulong;
                                                                }
                                                                    hold =
                                                                        0 as ::core::ffi::c_ulong;
                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                    (*state).mode =
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
                                                        if (*state).last != 0 {
                                                            hold >>=
                                                                bits & 7 as ::core::ffi::c_uint;
                                                            bits = bits.wrapping_sub(
                                                                bits & 7 as ::core::ffi::c_uint,
                                                            );
                                                            (*state).mode =
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
                                                                next = next.offset(1);
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
                                                                    (*state).mode =
                                                                        crate::src::inflate::TABLE;
                                                                }
                                                                _ => {
                                                                    (*strm).msg = INFLATE_MESSAGES[13].as_ptr()
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
                                                    if (*state).flags & 0x400 as ::core::ffi::c_int
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
                                                            next = next.offset(1);
                                                            hold = hold.wrapping_add(
                                                                (*c2rust_fresh4
                                                                    as ::core::ffi::c_ulong)
                                                                    << bits,
                                                            );
                                                            bits = bits.wrapping_add(
                                                                8 as ::core::ffi::c_uint,
                                                            );
                                                        }
                                                        (*state).length =
                                                            hold as ::core::ffi::c_uint;
                                                        if !(*state).head.is_null() {
                                                            (*(*state).head).extra_len = hold
                                                                as ::core::ffi::c_uint
                                                                as crate::stdlib::uInt;
                                                        }
                                                        if (*state).flags
                                                            & 0x200 as ::core::ffi::c_int
                                                            != 0
                                                            && (*state).wrap
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
                                                            (*state).check =
                                                                crate::src::crc32::crc32(
                                                                    (*state).check
                                                                        as crate::stdlib::uLong,
                                                                    Some(&hbuf[..2]),
                                                                )
                                                                    as ::core::ffi::c_ulong;
                                                        }
                                                        hold = 0 as ::core::ffi::c_ulong;
                                                        bits = 0 as ::core::ffi::c_uint;
                                                    } else if !(*state).head.is_null() {
                                                        (*(*state).head).extra =
                                                            ::core::ptr::null_mut::<
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
                                                    == crate::src::inflate::TYPE
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    (*state).back = -1 as ::core::ffi::c_int;
                                                }
                                                continue '_inf_leave;
                                            } else {
                                                (*state).back = 0 as ::core::ffi::c_int;
                                                loop {
                                                    here = *(*state).lencode.offset(
                                                        (hold as ::core::ffi::c_uint
                                                            & ((1 as ::core::ffi::c_uint)
                                                                << (*state).lenbits)
                                                                .wrapping_sub(
                                                                    1 as ::core::ffi::c_uint,
                                                                ))
                                                            as isize,
                                                    );
                                                    if here.bits as ::core::ffi::c_uint <= bits {
                                                        break;
                                                    }
                                                    if have == 0 as ::core::ffi::c_uint {
                                                        break '_inf_leave;
                                                    }
                                                    have = have.wrapping_sub(1);
                                                    let c2rust_fresh24 = next;
                                                    next = next.offset(1);
                                                    hold = hold.wrapping_add(
                                                        (*c2rust_fresh24 as ::core::ffi::c_ulong)
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
                                                        here = *(*state).lencode.offset(
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
                                                            as isize,
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
                                                        next = next.offset(1);
                                                        hold = hold.wrapping_add(
                                                            (*c2rust_fresh25
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
                                                    (*state).back +=
                                                        last.bits as ::core::ffi::c_int;
                                                }
                                                hold >>= here.bits as ::core::ffi::c_int;
                                                bits = bits
                                                    .wrapping_sub(here.bits as ::core::ffi::c_uint);
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
                                                    (*strm).msg = INFLATE_MESSAGES[14].as_ptr()
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
                                                if !(*state).head.is_null()
                                                    && !(*(*state).head).extra.is_null()
                                                    && {
                                                        len = ((*(*state).head).extra_len
                                                            as ::core::ffi::c_uint)
                                                            .wrapping_sub((*state).length);
                                                        len < (*(*state).head).extra_max
                                                    }
                                                {
                                                    crate::stdlib::memcpy(
                                                        (*(*state).head).extra.offset(len as isize)
                                                            as *mut ::core::ffi::c_void,
                                                        next as *const ::core::ffi::c_void,
                                                        (if len.wrapping_add(copy)
                                                            > (*(*state).head).extra_max
                                                        {
                                                            ((*(*state).head).extra_max
                                                                as ::core::ffi::c_uint)
                                                                .wrapping_sub(len)
                                                        } else {
                                                            copy
                                                        })
                                                            as crate::__stddef_size_t_h::size_t,
                                                    );
                                                }
                                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                {
                                                    (*state).check = crate::src::crc32::crc32(
                                                        (*state).check as crate::stdlib::uLong,
                                                        Some(::core::slice::from_raw_parts(
                                                            next,
                                                            copy as usize,
                                                        )),
                                                    )
                                                        as ::core::ffi::c_ulong;
                                                }
                                                have = have.wrapping_sub(copy);
                                                next = next.offset(copy as isize);
                                                (*state).length =
                                                    (*state).length.wrapping_sub(copy);
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
                                            next = next.offset(1);
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
                                        len = *next.offset(c2rust_fresh5 as isize)
                                            as ::core::ffi::c_uint;
                                        if !(*state).head.is_null()
                                            && !(*(*state).head).name.is_null()
                                            && (*state).length < (*(*state).head).name_max
                                        {
                                            let c2rust_fresh6 = (*state).length;
                                            (*state).length = (*state).length.wrapping_add(1);
                                            *(*(*state).head).name.offset(c2rust_fresh6 as isize) =
                                                len as crate::stdlib::Bytef;
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
                                            Some(::core::slice::from_raw_parts(
                                                next,
                                                copy as usize,
                                            )),
                                        )
                                            as ::core::ffi::c_ulong;
                                    }
                                    have = have.wrapping_sub(copy);
                                    next = next.offset(copy as isize);
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
                                here = *(*state).distcode.offset(
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint) << (*state).distbits)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        as isize,
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                if have == 0 as ::core::ffi::c_uint {
                                    break '_inf_leave;
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh27 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add(
                                    (*c2rust_fresh27 as ::core::ffi::c_ulong) << bits,
                                );
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                last = here;
                                loop {
                                    here = *(*state).distcode.offset(
                                        (last.val as ::core::ffi::c_uint).wrapping_add(
                                            (hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint)
                                                    << last.bits as ::core::ffi::c_int
                                                        + last.op as ::core::ffi::c_int)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                                >> last.bits as ::core::ffi::c_int,
                                        ) as isize,
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
                                    next = next.offset(1);
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
                                (*strm).msg = INFLATE_MESSAGES[15].as_ptr()
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
                                len = *next.offset(c2rust_fresh7 as isize) as ::core::ffi::c_uint;
                                if !(*state).head.is_null()
                                    && !(*(*state).head).comment.is_null()
                                    && (*state).length < (*(*state).head).comm_max
                                {
                                    let c2rust_fresh8 = (*state).length;
                                    (*state).length = (*state).length.wrapping_add(1);
                                    *(*(*state).head).comment.offset(c2rust_fresh8 as isize) =
                                        len as crate::stdlib::Bytef;
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
                                    Some(::core::slice::from_raw_parts(next, copy as usize)),
                                )
                                    as ::core::ffi::c_ulong;
                            }
                            have = have.wrapping_sub(copy);
                            next = next.offset(copy as isize);
                            if len != 0 {
                                break '_inf_leave;
                            }
                        } else if !(*state).head.is_null() {
                            (*(*state).head).comment =
                                ::core::ptr::null_mut::<crate::stdlib::Bytef>();
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
                            next = next.offset(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh29 as ::core::ffi::c_ulong) << bits);
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
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh9 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    if (*state).wrap & 4 as ::core::ffi::c_int != 0
                        && hold != (*state).check & 0xffff as ::core::ffi::c_ulong
                    {
                        (*strm).msg = INFLATE_MESSAGES[16].as_ptr() as *const ::core::ffi::c_char
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
                (*state).check = crate::src::crc32::crc32(0 as crate::stdlib::uLong, None)
                    as ::core::ffi::c_ulong;
                (*strm).adler = (*state).check as crate::stdlib::uLong;
                (*state).mode = crate::src::inflate::TYPE;
                continue '_inf_leave;
            }
            if left == 0 as ::core::ffi::c_uint {
                break;
            }
            copy = out.wrapping_sub(left);
            if (*state).offset > copy {
                copy = (*state).offset.wrapping_sub(copy);
                if copy > (*state).whave {
                    if (*state).sane != 0 {
                        (*strm).msg = INFLATE_MESSAGES[17].as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                        continue;
                    }
                }
                if copy > (*state).wnext {
                    copy = copy.wrapping_sub((*state).wnext);
                    from = (*state)
                        .window
                        .offset((*state).wsize.wrapping_sub(copy) as isize);
                } else {
                    from = (*state)
                        .window
                        .offset((*state).wnext.wrapping_sub(copy) as isize);
                }
                if copy > (*state).length {
                    copy = (*state).length;
                }
            } else {
                from = put.offset(-((*state).offset as isize));
                copy = (*state).length;
            }
            if copy > left {
                copy = left;
            }
            left = left.wrapping_sub(copy);
            (*state).length = (*state).length.wrapping_sub(copy);
            loop {
                let c2rust_fresh30 = from;
                from = from.offset(1);
                let c2rust_fresh31 = put;
                put = put.offset(1);
                *c2rust_fresh31 = *c2rust_fresh30;
                copy = copy.wrapping_sub(1);
                if copy == 0 {
                    break;
                }
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
        if (*state).wsize != 0
            || out != (*strm).avail_out
                && ((*state).mode as ::core::ffi::c_uint)
                    < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                && (((*state).mode as ::core::ffi::c_uint)
                    < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                    || flush != crate::zlib_h::Z_FINISH)
        {
            let copy = out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
            let input = if copy == 0 {
                &[]
            } else {
                ::core::slice::from_raw_parts(
                    (*strm).next_out.offset(-(copy as isize)),
                    copy as usize,
                )
            };
            if updatewindow(&mut *strm, &mut *state, input) != 0 {
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
                    Some(::core::slice::from_raw_parts(
                        (*strm).next_out.offset(-(out as isize)),
                        out as usize,
                    )),
                )
            } else {
                crate::src::adler32::adler32(
                    (*state).check as crate::stdlib::uLong,
                    Some(::core::slice::from_raw_parts(
                        (*strm).next_out.offset(-(out as isize)),
                        out as crate::stdlib::z_size_t,
                    )),
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
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The C allocator releases the window before the state. Keep that order
    // in one explicit callback boundary while these allocations remain raw.
    let allocations = [
        state.window as crate::stdlib::voidpf,
        strm.state as crate::stdlib::voidpf,
    ];
    for allocation in allocations {
        if !allocation.is_null() {
            unsafe {
                Some(strm.zfree.expect("non-null function pointer"))
                    .expect("non-null function pointer")(strm.opaque, allocation);
            }
        }
    }
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return crate::zlib_h::Z_OK;
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
    let window = if state.window.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts(
            state.window,
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
pub fn inflateCopy(
    dest: Option<&mut crate::zlib_h::z_stream>,
    source: Option<&crate::zlib_h::z_stream>,
) -> ::core::ffi::c_int {
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let Some(source_ref) = source else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_stream_has_allocators(source_ref) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The allocator pair above makes this the one ABI state-handle conversion
    // in the copy implementation. Keep source validation ahead of any
    // destination mutation.
    let Some(state_ref) =
        (unsafe { (source_ref.state as *const crate::src::inflate::inflate_state).as_ref() })
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_state_valid(source_ref, state_ref) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(dest_ref) = dest else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let state_codes = ::core::ptr::from_ref(&state_ref.codes).cast_mut();
    let code_size = ::core::mem::size_of::<crate::src::inftrees::code>();
    let code_index = |address: usize| {
        address
            .checked_sub(state_codes.addr())
            .filter(|bytes| bytes % code_size == 0)
            .map(|bytes| bytes / code_size)
    };
    let Some(next_index) = code_index(state_ref.next.addr())
        .filter(|index| *index <= crate::src::inftrees::ENOUGH as usize)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let table_indices = match code_index(state_ref.lencode.addr()) {
        Some(lencode_index) if lencode_index < crate::src::inftrees::ENOUGH as usize => {
            let Some(distcode_index) = code_index(state_ref.distcode.addr())
                .filter(|index| *index < crate::src::inftrees::ENOUGH as usize)
            else {
                return crate::zlib_h::Z_STREAM_ERROR;
            };
            Some((lencode_index, distcode_index))
        }
        _ => None,
    };
    copy = unsafe {
        Some(source_ref.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            source_ref.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::inflate::inflate_state
    };
    if copy.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !state_ref.window.is_null() {
        window = unsafe {
            Some(source_ref.zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                source_ref.opaque,
                (1 as crate::stdlib::uInt) << state_ref.wbits,
                ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
            ) as *mut ::core::ffi::c_uchar
        };
        if window.is_null() {
            unsafe {
                Some(source_ref.zfree.expect("non-null function pointer"))
                    .expect("non-null function pointer")(source_ref.opaque, copy.cast());
            }
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    *dest_ref = *source_ref;
    let copy_ref = unsafe { &mut *copy };
    *copy_ref = *state_ref;
    copy_ref.strm = ::core::ptr::from_mut(dest_ref);
    if let Some((lencode_index, distcode_index)) = table_indices {
        let copy_codes = ::core::ptr::from_mut(&mut copy_ref.codes).cast::<crate::src::inftrees::code>();
        copy_ref.lencode = copy_codes.wrapping_add(lencode_index);
        copy_ref.distcode = copy_codes.wrapping_add(distcode_index);
    }
    copy_ref.next = ::core::ptr::from_mut(&mut copy_ref.codes)
        .cast::<crate::src::inftrees::code>()
        .wrapping_add(next_index);
    if !window.is_null() {
        unsafe {
            ::core::ptr::copy_nonoverlapping(state_ref.window, window, state_ref.whave as usize);
        }
    }
    copy_ref.window = window;
    dest_ref.state = copy.cast::<crate::src::deflate::internal_state>();
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateCopy"]

pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let source = source.as_ref();
    let dest = dest.as_mut();
    inflateCopy(dest, source)
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
    return state
        .next
        .addr()
        .wrapping_sub(state.codes.as_ptr().addr())
        .wrapping_div(::core::mem::size_of::<crate::src::inftrees::code>())
        as ::core::ffi::c_ulong;
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
