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

pub enum CodeTableRef {
    Dynamic(usize),
    FixedLen,
    FixedDist,
}

impl Copy for CodeTableRef {}

impl Clone for CodeTableRef {
    fn clone(&self) -> Self {
        match *self {
            Self::Dynamic(start) => Self::Dynamic(start),
            Self::FixedLen => Self::FixedLen,
            Self::FixedDist => Self::FixedDist,
        }
    }
}

impl CodeTableRef {
    #[inline]
    pub fn get<'a>(
        self,
        codes: &'a [crate::src::inftrees::code],
        index: isize,
    ) -> &'a crate::src::inftrees::code {
        let index = index as usize;
        match self {
            Self::Dynamic(start) => &codes[start + index],
            Self::FixedLen => &crate::src::inftrees::lenfix[index],
            Self::FixedDist => &crate::src::inftrees::distfix[index],
        }
    }
}
#[repr(C)]

pub struct inflate_state {
    // Keep the stream association check without retaining a raw backlink in
    // the codec state.  This is an identity token only; stream access is
    // always supplied by the caller.
    pub stream_identity: usize,
    pub mode: crate::src::inflate::inflate_mode,
    pub last: ::core::ffi::c_int,
    pub wrap: ::core::ffi::c_int,
    pub havedict: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub dmax: ::core::ffi::c_uint,
    pub check: ::core::ffi::c_ulong,
    pub total: ::core::ffi::c_ulong,
    // A registered caller header remains an ABI-boundary handle.  Keep
    // nullability in the state explicitly so decoder code need not carry a
    // nullable raw pointer between header fields.
    pub head: Option<::core::ptr::NonNull<crate::zlib_h::gz_header_s>>,
    pub wbits: ::core::ffi::c_uint,
    pub wsize: ::core::ffi::c_uint,
    pub whave: ::core::ffi::c_uint,
    pub wnext: ::core::ffi::c_uint,
    // The normal inflate history window is allocated lazily.  Retain that
    // absence explicitly; raw cursor projections are formed only by the
    // existing unsafe codec boundaries.
    pub window: Option<::core::ptr::NonNull<::core::ffi::c_uchar>>,
    pub hold: ::core::ffi::c_ulong,
    pub bits: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub extra: ::core::ffi::c_uint,
    pub lencode: crate::src::inflate::CodeTableRef,
    pub distcode: crate::src::inflate::CodeTableRef,
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

// `inflate()` publishes only these static diagnostics through `strm.msg`.
// Keep their storage explicit so gzip can identify an internal diagnostic by
// pointer without dereferencing an ABI pointer merely to copy the bytes.
pub(crate) static INFLATE_ERROR_MESSAGES: [&[u8]; 18] = [
    b"incorrect header check\0",
    b"unknown compression method\0",
    b"invalid window size\0",
    b"unknown header flags set\0",
    b"invalid stored block lengths\0",
    b"too many length or distance symbols\0",
    b"incorrect data check\0",
    b"incorrect length check\0",
    b"invalid code lengths set\0",
    b"invalid bit length repeat\0",
    b"invalid code -- missing end-of-block\0",
    b"invalid literal/lengths set\0",
    b"invalid distances set\0",
    b"invalid block type\0",
    b"invalid literal/length code\0",
    b"invalid distance code\0",
    b"header crc mismatch\0",
    b"invalid distance too far back\0",
];
pub use crate::zlib_h::Z_TREES;
pub use crate::zlib_h::Z_VERSION_ERROR;
pub use crate::zutil_h::DEF_WBITS;

unsafe extern "C" fn inflateStateCheck(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let strm_ref = &*strm;
    if strm_ref.zalloc.is_none() || strm_ref.zfree.is_none() {
        return 1 as ::core::ffi::c_int;
    }
    let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let state = &*state;
    if state.stream_identity != strm.addr()
        || (state.mode as ::core::ffi::c_uint)
            < crate::src::inflate::HEAD as ::core::ffi::c_int as ::core::ffi::c_uint
        || state.mode as ::core::ffi::c_uint
            > crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub unsafe extern "C" fn inflateResetKeep(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The state association was validated above.  Keep the ABI pointer
    // projections scoped here so reset itself only mutates Rust references.
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
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
    state.head = None;
    state.hold = 0 as ::core::ffi::c_ulong;
    state.bits = 0 as ::core::ffi::c_uint;
    state.next = 0;
    state.distcode = crate::src::inflate::CodeTableRef::Dynamic(0);
    state.lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
    state.sane = 1 as ::core::ffi::c_int;
    state.back = -1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateResetKeep(strm)
}
pub unsafe extern "C" fn inflateReset(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    state.wsize = 0 as ::core::ffi::c_uint;
    state.whave = 0 as ::core::ffi::c_uint;
    state.wnext = 0 as ::core::ffi::c_uint;
    return inflateResetKeep(strm);
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateReset(strm)
}
pub unsafe extern "C" fn inflateReset2(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wrap: ::core::ffi::c_int = 0;
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // `inflateStateCheck()` established the ABI association.  Project it
    // once so the reset policy below works on Rust references rather than
    // repeatedly dereferencing the raw stream and state pointers.
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
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
    if state.window.is_some() && state.wbits != windowBits as ::core::ffi::c_uint {
        Some(strm.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            strm.opaque,
            state.window.expect("window checked").as_ptr() as crate::stdlib::voidpf,
        );
        state.window = None;
    }
    state.wrap = wrap;
    state.wbits = windowBits as ::core::ffi::c_uint;
    return inflateReset(strm);
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
    let mut ret: ::core::ffi::c_int = 0;
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if version.is_null()
        || *version.offset(0 as isize) as ::core::ffi::c_int
            != crate::zlib_h::ZLIB_VERSION[0 as usize] as ::core::ffi::c_int
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
    // `inflateReset2()` below establishes every field before the new state is
    // observed, so clearing allocator-provided storage here is dead work.
    (*strm).state = state as *mut crate::src::deflate::internal_state;
    (*state).stream_identity = strm.addr();
    (*state).window = None;
    (*state).mode = crate::src::inflate::HEAD;
    ret = inflateReset2(strm, windowBits);
    if ret != crate::zlib_h::Z_OK {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            state as crate::stdlib::voidpf,
        );
        (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    }
    return ret;
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
pub unsafe extern "C" fn inflatePrime(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if bits == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_OK;
    }
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
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
#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflatePrime(strm, bits, value)
}
fn copy_history_window(
    window: &mut [u8],
    input: &[u8],
    wnext: &mut ::core::ffi::c_uint,
    whave: &mut ::core::ffi::c_uint,
) {
    let wsize = window.len() as ::core::ffi::c_uint;
    if wsize == 0 {
        *wnext = 0;
        *whave = 0;
        return;
    }
    let mut copy = input.len() as ::core::ffi::c_uint;
    if copy >= wsize {
        window.copy_from_slice(&input[input.len() - window.len()..]);
        *wnext = 0;
        *whave = wsize;
        return;
    }

    let mut dist = wsize.wrapping_sub(*wnext);
    if dist > copy {
        dist = copy;
    }
    let input_start = input.len() - copy as usize;
    let window_start = *wnext as usize;
    window[window_start..window_start + dist as usize]
        .copy_from_slice(&input[input_start..input_start + dist as usize]);
    copy = copy.wrapping_sub(dist);
    if copy != 0 {
        let input_start = input.len() - copy as usize;
        window[..copy as usize].copy_from_slice(&input[input_start..]);
        *wnext = copy;
        *whave = wsize;
    } else {
        *wnext = wnext.wrapping_add(dist);
        if *wnext == wsize {
            *wnext = 0;
        }
        if *whave < wsize {
            *whave = whave.wrapping_add(dist);
        }
    }
}

fn copy_history_dictionary(output: &mut [u8], window: &[u8], wnext: usize, whave: usize) {
    let first = whave - wnext;
    output[..first].copy_from_slice(&window[wnext..wnext + first]);
    output[first..whave].copy_from_slice(&window[..wnext]);
}

// Copy one decoded match into the current output chunk.  A match that reaches
// into output must be copied forward one byte at a time: later bytes can
// intentionally read the bytes just written (for example, a distance of one).
// History-window matches, on the other hand, are a bounded non-overlapping
// source range selected by the caller.
fn copy_inflate_match(
    output: &mut [u8],
    produced: usize,
    offset: usize,
    history: Option<&[u8]>,
    count: usize,
) {
    if let Some(history) = history {
        output[produced..produced + count].copy_from_slice(history);
    } else {
        for index in 0..count {
            output[produced + index] = output[produced - offset + index];
        }
    }
}

#[inline]
fn inflate_pull_byte(
    input: &[u8],
    initial_have: ::core::ffi::c_uint,
    have: &mut ::core::ffi::c_uint,
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
) -> bool {
    if *have == 0 {
        return false;
    }
    let index = initial_have.wrapping_sub(*have) as usize;
    *hold = hold.wrapping_add((input[index] as ::core::ffi::c_ulong) << *bits);
    *bits = bits.wrapping_add(8);
    *have = have.wrapping_sub(1);
    true
}

unsafe extern "C" fn updatewindow(
    mut strm: crate::zlib_h::z_streamp,
    mut end: *const crate::stdlib::Bytef,
    mut copy: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).window.is_none() {
        (*state).window =
            ::core::ptr::NonNull::new(Some((*strm).zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*strm).opaque,
                (1 as crate::stdlib::uInt) << (*state).wbits,
                ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
            ) as *mut ::core::ffi::c_uchar);
        if (*state).window.is_none() {
            return 1 as ::core::ffi::c_int;
        }
    }
    if (*state).wsize == 0 as ::core::ffi::c_uint {
        (*state).wsize = (1 as ::core::ffi::c_uint) << (*state).wbits;
        (*state).wnext = 0 as ::core::ffi::c_uint;
        (*state).whave = 0 as ::core::ffi::c_uint;
    }
    let window = ::core::slice::from_raw_parts_mut(
        (*state).window.expect("window allocated").as_ptr(),
        (*state).wsize as usize,
    );
    let input = if copy == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(end.sub(copy as usize), copy as usize)
    };
    copy_history_window(window, input, &mut (*state).wnext, &mut (*state).whave);
    return 0 as ::core::ffi::c_int;
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
    if inflateStateCheck(strm) != 0
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
    // Keep the caller-owned output range as one bounded view. `put` remains
    // only as the ABI cursor that is republished on return.
    let output = ::core::slice::from_raw_parts_mut(put, left as usize);
    let mut output_chunk_start = 0usize;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    have = (*strm).avail_in as ::core::ffi::c_uint;
    // Preserve the C API's null-plus-zero input convention while keeping the
    // decoder's byte pulls bounded by the caller's advertised input range.
    let input = if have == 0 {
        &[][..]
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
                                                                                                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
                                                                                                        }
                                                                                                        if (*state).wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if (*state).wbits == 0 as ::core::ffi::c_uint {
                                                                                                                (*state).wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            (*state).check = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            (*state).check = crate::src::crc32::crc32_z(
                                                                                                                (*state).check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            (*state).mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                            if let Some(head) = (*state).head {
                                                                                                                (*head.as_ptr()).done = -1 as ::core::ffi::c_int;
                                                                                                            }
                                                                                                            if (*state).wrap & 1 as ::core::ffi::c_int == 0
                                                                                                                || (((hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                                                                                                    .wrapping_rem(31 as ::core::ffi::c_ulong) != 0
                                                                                                            {
                                                                                                                (*strm).msg = INFLATE_ERROR_MESSAGES[0].as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                (*strm).msg = INFLATE_ERROR_MESSAGES[1].as_ptr()
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
                                                                                                                    (*strm).msg = INFLATE_ERROR_MESSAGES[2].as_ptr()
                                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                    (*state).mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    (*state).dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    (*state).flags = 0 as ::core::ffi::c_int;
                                                                                                                    (*state).check = crate::src::adler32::adler32_z(
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
                                                                                                        if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
                                                                                                    }
                                                                                                    (*state).flags = hold as ::core::ffi::c_int;
                                                                                                    if (*state).flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        (*strm).msg = INFLATE_ERROR_MESSAGES[1].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if (*state).flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        (*strm).msg = INFLATE_ERROR_MESSAGES[3].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        (*state).mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if let Some(head) = (*state).head {
                                                                                                            (*head.as_ptr()).text = (hold >> 8 as ::core::ffi::c_int
                                                                                                                & 1 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
                                                                                                        }
                                                                                                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                                                                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                        {
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            (*state).check = crate::src::crc32::crc32_z(
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
                                                                                                        if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
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
                                                                                                        if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
                                                                                                    }
                                                                                                    if hold & 0xffff as ::core::ffi::c_ulong
                                                                                                        != hold >> 16 as ::core::ffi::c_int
                                                                                                            ^ 0xffff as ::core::ffi::c_ulong
                                                                                                    {
                                                                                                        (*strm).msg = INFLATE_ERROR_MESSAGES[4].as_ptr()
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
                                                                                                        if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
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
                                                                                                        (*strm).msg = INFLATE_ERROR_MESSAGES[5]
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
                                                                                                    let output_index = output.len() - left as usize;
                                                                                                    output[output_index] = (*state).length
                                                                                                        as ::core::ffi::c_uchar;
                                                                                                    put = output.as_mut_ptr().wrapping_add(output_index + 1);
                                                                                                    left = left.wrapping_sub(1);
                                                                                                    (*state).mode = crate::src::inflate::LEN;
                                                                                                    continue '_inf_leave;
                                                                                                }
                                                                                                16206 => {
                                                                                                    if (*state).wrap != 0 {
                                                                                                        while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
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
                                                                                                            let produced = &output[output_chunk_start
                                                                                                                ..output_chunk_start + out as usize];
                                                                                                            (*state).check = (if (*state).flags != 0 {
                                                                                                                crate::src::crc32::crc32_z(
                                                                                                                    (*state).check as crate::stdlib::uLong,
                                                                                                                    Some(produced),
                                                                                                                )
                                                                                                            } else {
                                                                                                                crate::src::adler32::adler32(
                                                                                                                    (*state).check as crate::stdlib::uLong,
                                                                                                                    produced,
                                                                                                                )
                                                                                                            }) as ::core::ffi::c_ulong;
                                                                                                            (*strm).adler = (*state).check as crate::stdlib::uLong;
                                                                                                        }
                                                                                                        out = left;
                                                                                                        // Subsequent output is a new checksum
                                                                                                        // chunk in the caller's bounded range.
                                                                                                        output_chunk_start = output.len() - left as usize;
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
                                                                                                            (*strm).msg = INFLATE_ERROR_MESSAGES[6].as_ptr()
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
                                                                                                    if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                    next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
                                                                                                }
                                                                                                if (*state).wrap & 4 as ::core::ffi::c_int != 0
                                                                                                    && hold
                                                                                                        != (*state).total & 0xffffffff as ::core::ffi::c_ulong
                                                                                                {
                                                                                                    (*strm).msg = INFLATE_ERROR_MESSAGES[7].as_ptr()
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
                                                                                                if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
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
                                                                                        (*state)
                                                                                            .next =
                                                                                            0;
                                                                                        (*state).distcode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                                        (*state).lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                                        (*state).lenbits = 7 as ::core::ffi::c_uint;
                                                                                        ret = 'table: {
                                                                                            let state = &mut *state;
                                                                                            let table_start = state.next;
                                                                                            let Some(lens) = state.lens.get(..19) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                            let Some(table) = state.codes.get_mut(table_start..) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                            let Some(work) = state.work.get_mut(..19) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                            let (status, used) = crate::src::inftrees::inflate_table(
                                                                                                crate::src::inftrees::CODES,
                                                                                                lens,
                                                                                                table,
                                                                                                &mut state.lenbits,
                                                                                                work,
                                                                                            );
                                                                                            if status == 0 {
                                                                                                state.next += used;
                                                                                            }
                                                                                            status
                                                                                        };
                                                                                        if ret != 0
                                                                                        {
                                                                                            (*strm).msg = INFLATE_ERROR_MESSAGES[8].as_ptr()
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
                                                                                    (*state).check = crate::src::adler32::adler32_z(
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
                                                                                    if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                        break '_inf_leave;
                                                                                    }
                                                                                    next = input[in_0.wrapping_sub(have) as usize..].as_ptr() as *mut ::core::ffi::c_uchar;
                                                                                }
                                                                                if let Some(head) =
                                                                                    (*state).head
                                                                                {
                                                                                    (*head.as_ptr()).time = hold
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
                                                                                    (*state).check = crate::src::crc32::crc32_z(
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
                                                                                    (*state).ndist,
                                                                                )
                                                                        {
                                                                            loop {
                                                                                here = crate::src::inftrees::code::copied_from((*state)
                                                                                    .lencode
                                                                                    .get(&(*state).codes,
                                                                                        (hold as ::core::ffi::c_uint
                                                                                            & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                                                                                .wrapping_sub(1 as ::core::ffi::c_uint)) as isize,
                                                                                    ));
                                                                                if here.bits as ::core::ffi::c_uint <= bits {
                                                                                    break;
                                                                                }
                                                                                if have == 0 as ::core::ffi::c_uint {
                                                                                    break '_inf_leave;
                                                                                }
                                                                                inflate_pull_byte(
                                                                                    input, in_0, &mut have, &mut hold, &mut bits,
                                                                                );
                                                                                next = input[in_0.wrapping_sub(have) as usize..]
                                                                                    .as_ptr() as *mut ::core::ffi::c_uchar;
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
                                                                                        inflate_pull_byte(
                                                                                            input, in_0, &mut have, &mut hold, &mut bits,
                                                                                        );
                                                                                        next = input[in_0.wrapping_sub(have) as usize..]
                                                                                            .as_ptr() as *mut ::core::ffi::c_uchar;
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    if (*state).have == 0 as ::core::ffi::c_uint {
                                                                                        (*strm).msg = INFLATE_ERROR_MESSAGES[9].as_ptr()
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
                                                                                        inflate_pull_byte(
                                                                                            input, in_0, &mut have, &mut hold, &mut bits,
                                                                                        );
                                                                                        next = input[in_0.wrapping_sub(have) as usize..]
                                                                                            .as_ptr() as *mut ::core::ffi::c_uchar;
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
                                                                                        inflate_pull_byte(
                                                                                            input, in_0, &mut have, &mut hold, &mut bits,
                                                                                        );
                                                                                        next = input[in_0.wrapping_sub(have) as usize..]
                                                                                            .as_ptr() as *mut ::core::ffi::c_uchar;
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
                                                                                    (*strm).msg = INFLATE_ERROR_MESSAGES[9].as_ptr()
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
                                                                        if (*state).lens[256 as usize] as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            (*strm).msg = INFLATE_ERROR_MESSAGES[10]
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            (*state).next = 0;
                                                                            (*state).lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                            (*state).lenbits = 9 as ::core::ffi::c_uint;
                                                                            ret = 'table: {
                                                                                let state = &mut *state;
                                                                                let codes = state.nlen as usize;
                                                                                let table_start = state.next;
                                                                                let Some(lens) = state.lens.get(..codes) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let Some(table) = state.codes.get_mut(table_start..) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let Some(work) = state.work.get_mut(..codes) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let (status, used) = crate::src::inftrees::inflate_table(
                                                                                    crate::src::inftrees::LENS,
                                                                                    lens,
                                                                                    table,
                                                                                    &mut state.lenbits,
                                                                                    work,
                                                                                );
                                                                                if status == 0 {
                                                                                    state.next += used;
                                                                                }
                                                                                status
                                                                            };
                                                                            if ret != 0 {
                                                                                (*strm).msg = INFLATE_ERROR_MESSAGES[11].as_ptr()
                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                (*state).distcode = crate::src::inflate::CodeTableRef::Dynamic((*state).next);
                                                                                (*state).distbits = 6 as ::core::ffi::c_uint;
                                                                                ret = 'table: {
                                                                                    let state = &mut *state;
                                                                                    let lens_start = state.nlen as usize;
                                                                                    let codes = state.ndist as usize;
                                                                                    let Some(lens_end) = lens_start.checked_add(codes) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let table_start = state.next;
                                                                                    let Some(lens) = state.lens.get(lens_start..lens_end) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let Some(table) = state.codes.get_mut(table_start..) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let Some(work) = state.work.get_mut(..codes) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let (status, used) = crate::src::inftrees::inflate_table(
                                                                                        crate::src::inftrees::DISTS,
                                                                                        lens,
                                                                                        table,
                                                                                        &mut state.distbits,
                                                                                        work,
                                                                                    );
                                                                                    if status == 0 {
                                                                                        state.next += used;
                                                                                    }
                                                                                    status
                                                                                };
                                                                                if ret != 0 {
                                                                                    (*strm).msg = INFLATE_ERROR_MESSAGES[12].as_ptr()
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
                                                                        // This is the translated C `memcpy` path.
                                                                        // Input and output are distinct non-overlapping
                                                                        // caller ranges by inflate's existing contract.
                                                                        let input_start = in_0
                                                                            .wrapping_sub(have)
                                                                            as usize;
                                                                        let copy_len =
                                                                            copy as usize;
                                                                        let output_start = output.len() - left as usize;
                                                                        output[output_start..output_start + copy_len]
                                                                            .copy_from_slice(
                                                                                &input[input_start
                                                                                    ..input_start
                                                                                        + copy_len],
                                                                            );
                                                                        have =
                                                                            have.wrapping_sub(copy);
                                                                        next = input[in_0.wrapping_sub(have) as usize..]
                                                                            .as_ptr()
                                                                            as *mut ::core::ffi::c_uchar;
                                                                        left =
                                                                            left.wrapping_sub(copy);
                                                                        put = output.as_mut_ptr().wrapping_add(
                                                                            output_start + copy_len,
                                                                        );
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
                                                                    if !inflate_pull_byte(
                                                                        input, in_0, &mut have,
                                                                        &mut hold, &mut bits,
                                                                    ) {
                                                                        break '_inf_leave;
                                                                    }
                                                                    next = input[in_0
                                                                        .wrapping_sub(have)
                                                                        as usize..]
                                                                        .as_ptr()
                                                                        as *mut ::core::ffi::c_uchar;
                                                                }
                                                                if let Some(mut head) =
                                                                    (*state).head
                                                                {
                                                                    let head = &mut *head.as_ptr();
                                                                    head.xflags = (hold
                                                                        & 0xff
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int;
                                                                    head.os = (hold
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
                                                                    (*state).check = crate::src::crc32::crc32_z(
                                                                        (*state).check as crate::stdlib::uLong,
                                                                        Some(&hbuf[..2]),
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
                                                            if !inflate_pull_byte(
                                                                input, in_0, &mut have, &mut hold,
                                                                &mut bits,
                                                            ) {
                                                                break '_inf_leave;
                                                            }
                                                            next = input
                                                                [in_0.wrapping_sub(have) as usize..]
                                                                .as_ptr()
                                                                as *mut ::core::ffi::c_uchar;
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
                                                                (*strm).msg = INFLATE_ERROR_MESSAGES
                                                                    [13]
                                                                .as_ptr()
                                                                    as *const ::core::ffi::c_char
                                                                    as *mut ::core::ffi::c_char;
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
                                                        if !inflate_pull_byte(
                                                            input, in_0, &mut have, &mut hold,
                                                            &mut bits,
                                                        ) {
                                                            break '_inf_leave;
                                                        }
                                                        next = input
                                                            [in_0.wrapping_sub(have) as usize..]
                                                            .as_ptr()
                                                            as *mut ::core::ffi::c_uchar;
                                                    }
                                                    (*state).length = hold as ::core::ffi::c_uint;
                                                    if let Some(head) = (*state).head {
                                                        (*head.as_ptr()).extra_len = hold
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
                                                        (*state).check = crate::src::crc32::crc32_z(
                                                            (*state).check as crate::stdlib::uLong,
                                                            Some(&hbuf[..2]),
                                                        )
                                                            as ::core::ffi::c_ulong;
                                                    }
                                                    hold = 0 as ::core::ffi::c_ulong;
                                                    bits = 0 as ::core::ffi::c_uint;
                                                } else if let Some(head) = (*state).head {
                                                    (*head.as_ptr()).extra = ::core::ptr::null_mut::<
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
                                            // The enclosing decoder already owns bounded views
                                            // of the current input and output chunks.  Keep the
                                            // fast handoff within those views instead of
                                            // republishing cursors through the legacy raw-stream
                                            // adapter.
                                            let input_start = in_0.wrapping_sub(have) as usize;
                                            let input = &input[input_start..];
                                            let output = &mut output[output_chunk_start
                                                ..output_chunk_start + out as usize];
                                            let written = out.wrapping_sub(left) as usize;
                                            // Keep this short-lived projection local to the
                                            // legacy state owner; the fast core itself receives
                                            // no raw state or stream values.
                                            let state = &mut *state;
                                            let window = state.window.map(|window| {
                                                ::core::slice::from_raw_parts(
                                                    window.as_ptr(),
                                                    state.wsize as usize,
                                                )
                                            });
                                            let mut fast_state =
                                                crate::src::inffast::InflateFastState {
                                                    history:
                                                        crate::src::inffast::FastHistory::External(
                                                            window,
                                                        ),
                                                    wsize: state.wsize as usize,
                                                    whave: state.whave as usize,
                                                    wnext: state.wnext as usize,
                                                    hold,
                                                    bits,
                                                    lcode: state.lencode,
                                                    dcode: state.distcode,
                                                    lmask: (1u32 << state.lenbits) - 1,
                                                    dmask: (1u32 << state.distbits) - 1,
                                                    codes: &state.codes,
                                                    sane: state.sane != 0,
                                                };
                                            let result =
                                                crate::src::inffast::inflate_fast_from_views(
                                                    input,
                                                    output,
                                                    written,
                                                    &mut fast_state,
                                                );
                                            next = input.as_ptr().wrapping_add(result.input_used)
                                                as *mut ::core::ffi::c_uchar;
                                            have = input.len().wrapping_sub(result.input_used)
                                                as ::core::ffi::c_uint;
                                            put = output
                                                .as_mut_ptr()
                                                .wrapping_add(result.output_used);
                                            left = output.len().wrapping_sub(result.output_used)
                                                as ::core::ffi::c_uint;
                                            hold = fast_state.hold;
                                            bits = fast_state.bits;
                                            match result.exit {
                                                crate::src::inffast::FastExit::Continue => {}
                                                crate::src::inffast::FastExit::Type => {
                                                    state.mode = crate::src::inflate::TYPE;
                                                }
                                                crate::src::inffast::FastExit::InvalidDistance => {
                                                    let strm = &mut *strm;
                                                    strm.msg = INFLATE_ERROR_MESSAGES[17].as_ptr()
                                                        as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char;
                                                    state.mode = crate::src::inflate::BAD;
                                                }
                                                crate::src::inffast::FastExit::InvalidCode => {
                                                    let strm = &mut *strm;
                                                    strm.msg = b"invalid literal/length or distance code\0"
                                                        .as_ptr()
                                                        .cast_mut()
                                                        .cast();
                                                    state.mode = crate::src::inflate::BAD;
                                                }
                                            }
                                            if state.mode as ::core::ffi::c_uint
                                                == crate::src::inflate::TYPE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                state.back = -1 as ::core::ffi::c_int;
                                            }
                                            continue '_inf_leave;
                                        } else {
                                            (*state).back = 0 as ::core::ffi::c_int;
                                            loop {
                                                here = crate::src::inftrees::code::copied_from(
                                                    (*state).lencode.get(
                                                        &(*state).codes,
                                                        (hold as ::core::ffi::c_uint
                                                            & ((1 as ::core::ffi::c_uint)
                                                                << (*state).lenbits)
                                                                .wrapping_sub(
                                                                    1 as ::core::ffi::c_uint,
                                                                ))
                                                            as isize,
                                                    ),
                                                );
                                                if here.bits as ::core::ffi::c_uint <= bits {
                                                    break;
                                                }
                                                if !inflate_pull_byte(
                                                    input, in_0, &mut have, &mut hold, &mut bits,
                                                ) {
                                                    break '_inf_leave;
                                                }
                                                next = input[in_0.wrapping_sub(have) as usize..]
                                                    .as_ptr()
                                                    as *mut ::core::ffi::c_uchar;
                                            }
                                            if here.op as ::core::ffi::c_int != 0
                                                && here.op as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                            {
                                                last =
                                                    crate::src::inftrees::code::copied_from(&here);
                                                loop {
                                                    here = crate::src::inftrees::code::copied_from((*state).lencode.get(&(*state).codes,
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
                                                    ));
                                                    if (last.bits as ::core::ffi::c_int
                                                        + here.bits as ::core::ffi::c_int)
                                                        as ::core::ffi::c_uint
                                                        <= bits
                                                    {
                                                        break;
                                                    }
                                                    if !inflate_pull_byte(
                                                        input, in_0, &mut have, &mut hold,
                                                        &mut bits,
                                                    ) {
                                                        break '_inf_leave;
                                                    }
                                                    next = input[in_0.wrapping_sub(have) as usize..]
                                                        .as_ptr()
                                                        as *mut ::core::ffi::c_uchar;
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
                                                (*strm).msg = INFLATE_ERROR_MESSAGES[14].as_ptr()
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
                                            if let Some(mut head) = (*state).head {
                                                let head = &mut *head.as_ptr();
                                                if !head.extra.is_null() && {
                                                    len = (head.extra_len as ::core::ffi::c_uint)
                                                        .wrapping_sub((*state).length);
                                                    len < head.extra_max
                                                } {
                                                    // The caller input and the separately registered
                                                    // header-extra buffer have the original C memcpy
                                                    // non-overlap contract.
                                                    let input_start =
                                                        in_0.wrapping_sub(have) as usize;
                                                    let extra = ::core::slice::from_raw_parts_mut(
                                                        head.extra,
                                                        head.extra_max as usize,
                                                    );
                                                    let copy_len = (if len.wrapping_add(copy)
                                                        > head.extra_max
                                                    {
                                                        (head.extra_max as ::core::ffi::c_uint)
                                                            .wrapping_sub(len)
                                                    } else {
                                                        copy
                                                    })
                                                        as usize;
                                                    extra[len as usize..len as usize + copy_len]
                                                        .copy_from_slice(
                                                            &input[input_start
                                                                ..input_start + copy_len],
                                                        );
                                                }
                                            }
                                            if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                            {
                                                (*state).check = crate::src::crc32::crc32_z(
                                                    (*state).check as crate::stdlib::uLong,
                                                    Some(
                                                        &input[in_0.wrapping_sub(have) as usize
                                                            ..in_0
                                                                .wrapping_sub(have)
                                                                .wrapping_add(copy)
                                                                as usize],
                                                    ),
                                                )
                                                    as ::core::ffi::c_ulong;
                                            }
                                            have = have.wrapping_sub(copy);
                                            next = input[in_0.wrapping_sub(have) as usize..]
                                                .as_ptr()
                                                as *mut ::core::ffi::c_uchar;
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
                                        if !inflate_pull_byte(
                                            input, in_0, &mut have, &mut hold, &mut bits,
                                        ) {
                                            break '_inf_leave;
                                        }
                                        next = input[in_0.wrapping_sub(have) as usize..].as_ptr()
                                            as *mut ::core::ffi::c_uchar;
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
                                    len = input[in_0.wrapping_sub(have) as usize
                                        + c2rust_fresh5 as usize]
                                        as ::core::ffi::c_uint;
                                    if let Some(head) = (*state).head {
                                        if !(*head.as_ptr()).name.is_null()
                                            && (*state).length < (*head.as_ptr()).name_max
                                        {
                                            let c2rust_fresh6 = (*state).length;
                                            (*state).length = (*state).length.wrapping_add(1);
                                            *(*head.as_ptr()).name.offset(c2rust_fresh6 as isize) =
                                                len as crate::stdlib::Bytef;
                                        }
                                    }
                                    if !(len != 0 && copy < have) {
                                        break;
                                    }
                                }
                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                {
                                    (*state).check = crate::src::crc32::crc32_z(
                                        (*state).check as crate::stdlib::uLong,
                                        Some(&input[in_0.wrapping_sub(have) as usize
                                            ..in_0.wrapping_sub(have).wrapping_add(copy) as usize]),
                                    )
                                        as ::core::ffi::c_ulong;
                                }
                                have = have.wrapping_sub(copy);
                                next = input[in_0.wrapping_sub(have) as usize..].as_ptr()
                                    as *mut ::core::ffi::c_uchar;
                                if len != 0 {
                                    break '_inf_leave;
                                }
                            } else if let Some(head) = (*state).head {
                                (*head.as_ptr()).name =
                                    ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                            }
                            (*state).length = 0 as ::core::ffi::c_uint;
                            (*state).mode = crate::src::inflate::COMMENT;
                            break 'c_2325;
                        }
                        loop {
                            here = crate::src::inftrees::code::copied_from(
                                (*state).distcode.get(
                                    &(*state).codes,
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint) << (*state).distbits)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        as isize,
                                ),
                            );
                            if here.bits as ::core::ffi::c_uint <= bits {
                                break;
                            }
                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                break '_inf_leave;
                            }
                            next = input[in_0.wrapping_sub(have) as usize..].as_ptr()
                                as *mut ::core::ffi::c_uchar;
                        }
                        if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            last = crate::src::inftrees::code::copied_from(&here);
                            loop {
                                here = crate::src::inftrees::code::copied_from(
                                    (*state).distcode.get(
                                        &(*state).codes,
                                        (last.val as ::core::ffi::c_uint).wrapping_add(
                                            (hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint)
                                                    << last.bits as ::core::ffi::c_int
                                                        + last.op as ::core::ffi::c_int)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                                >> last.bits as ::core::ffi::c_int,
                                        ) as isize,
                                    ),
                                );
                                if (last.bits as ::core::ffi::c_int
                                    + here.bits as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint
                                    <= bits
                                {
                                    break;
                                }
                                if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits)
                                {
                                    break '_inf_leave;
                                }
                                next = input[in_0.wrapping_sub(have) as usize..].as_ptr()
                                    as *mut ::core::ffi::c_uchar;
                            }
                            hold >>= last.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                            (*state).back += last.bits as ::core::ffi::c_int;
                        }
                        hold >>= here.bits as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                        (*state).back += here.bits as ::core::ffi::c_int;
                        if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                            (*strm).msg = INFLATE_ERROR_MESSAGES[15].as_ptr()
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
                            len = input[in_0.wrapping_sub(have) as usize
                                + c2rust_fresh7 as usize]
                                as ::core::ffi::c_uint;
                            if let Some(head) = (*state).head {
                                if !(*head.as_ptr()).comment.is_null()
                                    && (*state).length < (*head.as_ptr()).comm_max
                                {
                                    let c2rust_fresh8 = (*state).length;
                                    (*state).length = (*state).length.wrapping_add(1);
                                    *(*head.as_ptr()).comment.offset(c2rust_fresh8 as isize) =
                                        len as crate::stdlib::Bytef;
                                }
                            }
                            if !(len != 0 && copy < have) {
                                break;
                            }
                        }
                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                        {
                            (*state).check = crate::src::crc32::crc32_z(
                                (*state).check as crate::stdlib::uLong,
                                Some(&input[in_0.wrapping_sub(have) as usize
                                    ..in_0.wrapping_sub(have).wrapping_add(copy) as usize]),
                            ) as ::core::ffi::c_ulong;
                        }
                        have = have.wrapping_sub(copy);
                        next = input[in_0.wrapping_sub(have) as usize..].as_ptr()
                            as *mut ::core::ffi::c_uchar;
                        if len != 0 {
                            break '_inf_leave;
                        }
                    } else if let Some(head) = (*state).head {
                        (*head.as_ptr()).comment = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                    }
                    (*state).mode = crate::src::inflate::HCRC;
                    break 'c_2327;
                }
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                            break '_inf_leave;
                        }
                        next = input[in_0.wrapping_sub(have) as usize..].as_ptr()
                            as *mut ::core::ffi::c_uchar;
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
                    if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                        break '_inf_leave;
                    }
                    next = input[in_0.wrapping_sub(have) as usize..].as_ptr()
                        as *mut ::core::ffi::c_uchar;
                }
                if (*state).wrap & 4 as ::core::ffi::c_int != 0
                    && hold != (*state).check & 0xffff as ::core::ffi::c_ulong
                {
                    (*strm).msg = INFLATE_ERROR_MESSAGES[16].as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue '_inf_leave;
                } else {
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                }
            }
            if let Some(mut head) = (*state).head {
                let head = &mut *head.as_ptr();
                head.hcrc = (*state).flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                head.done = 1 as ::core::ffi::c_int;
            }
            (*state).check =
                crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
            (*strm).adler = (*state).check as crate::stdlib::uLong;
            (*state).mode = crate::src::inflate::TYPE;
            continue '_inf_leave;
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        let produced = out.wrapping_sub(left) as usize;
        let offset = (*state).offset as usize;
        let mut history = None;
        copy = 0;
        if offset > produced {
            copy = (offset - produced) as ::core::ffi::c_uint;
            if copy > (*state).whave {
                if (*state).sane != 0 {
                    (*strm).msg = INFLATE_ERROR_MESSAGES[17].as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                }
            }
            if copy > (*state).wnext {
                copy = copy.wrapping_sub((*state).wnext);
                history = Some(((*state).wsize.wrapping_sub(copy) as usize, copy));
            } else {
                history = Some(((*state).wnext.wrapping_sub(copy) as usize, copy));
            }
            if copy > (*state).length {
                copy = (*state).length;
            }
        } else {
            copy = (*state).length;
        }
        if copy > left {
            copy = left;
        }
        let copy_len = copy as usize;
        // `out` is the capacity since the last checksum boundary and
        // `produced + copy_len` is bounded by it after the `left` clamp.
        let output = &mut output[output_chunk_start..output_chunk_start + out as usize];
        if let Some((start, history_limit)) = history {
            let window = ::core::slice::from_raw_parts(
                (*state)
                    .window
                    .expect("history exists when distance reaches window")
                    .as_ptr(),
                (*state).wsize as usize,
            );
            let history = &window[start..start + copy_len.min(history_limit as usize)];
            copy_inflate_match(output, produced, offset, Some(history), copy_len);
        } else {
            copy_inflate_match(output, produced, offset, None, copy_len);
        }
        put = output.as_mut_ptr().wrapping_add(produced + copy_len);
        left = left.wrapping_sub(copy);
        (*state).length = (*state).length.wrapping_sub(copy);
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
        if updatewindow(
            strm,
            (*strm).next_out,
            out.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint),
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
        let produced = &output[output_chunk_start..output_chunk_start + out as usize];
        (*state).check = (if (*state).flags != 0 {
            crate::src::crc32::crc32_z((*state).check as crate::stdlib::uLong, Some(produced))
        } else {
            crate::src::adler32::adler32((*state).check as crate::stdlib::uLong, produced)
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
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if let Some(window) = (*state).window {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            window.as_ptr() as crate::stdlib::voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as crate::stdlib::voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    inflateEnd(strm)
}
pub unsafe extern "C" fn inflateGetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).whave != 0 && !dictionary.is_null() {
        // The caller dictionary buffer and internal history allocation are
        // distinct, as required by the translated C memcpy operations. Form
        // bounded views once, then keep the ring-order copy pointer-free.
        let whave = (*state).whave as usize;
        let window = ::core::slice::from_raw_parts(
            (*state).window.expect("history exists").as_ptr(),
            (*state).wsize as usize,
        );
        let output = ::core::slice::from_raw_parts_mut(dictionary, whave);
        copy_history_dictionary(output, window, (*state).wnext as usize, whave);
    }
    if !dictLength.is_null() {
        *dictLength = (*state).whave as crate::stdlib::uInt;
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    inflateGetDictionary(strm, dictionary, dictLength)
}
pub unsafe extern "C" fn inflateSetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut dictid: ::core::ffi::c_ulong = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).wrap != 0 as ::core::ffi::c_int
        && (*state).mode as ::core::ffi::c_uint
            != crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if (*state).mode as ::core::ffi::c_uint
        == crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        dictid =
            crate::src::adler32::adler32_z(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
        dictid = crate::src::adler32::adler32(
            dictid as crate::stdlib::uLong,
            ::core::slice::from_raw_parts(dictionary, dictLength as usize),
        ) as ::core::ffi::c_ulong;
        if dictid != (*state).check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }
    ret = updatewindow(
        strm,
        dictionary.offset(dictLength as isize),
        dictLength as ::core::ffi::c_uint,
    );
    if ret != 0 {
        (*state).mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*state).havedict = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSetDictionary"]

pub unsafe extern "C" fn inflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    inflateSetDictionary(strm, dictionary, dictLength)
}
pub unsafe extern "C" fn inflateGetHeader(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*state).wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*state).head = ::core::ptr::NonNull::new(head);
    (*head).done = 0 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    inflateGetHeader(strm, head)
}
fn syncsearch(have: &mut ::core::ffi::c_uint, buf: &[::core::ffi::c_uchar]) -> ::core::ffi::c_uint {
    let mut got: ::core::ffi::c_uint = 0;
    let mut next: ::core::ffi::c_uint = 0;
    got = *have;
    next = 0 as ::core::ffi::c_uint;
    while (next as usize) < buf.len() && got < 4 as ::core::ffi::c_uint {
        if buf[next as usize] as ::core::ffi::c_int
            == (if got < 2 as ::core::ffi::c_uint {
                0 as ::core::ffi::c_int
            } else {
                0xff as ::core::ffi::c_int
            })
        {
            got = got.wrapping_add(1);
        } else if buf[next as usize] != 0 {
            got = 0 as ::core::ffi::c_uint;
        } else {
            got = (4 as ::core::ffi::c_uint).wrapping_sub(got);
        }
        next = next.wrapping_add(1);
    }
    *have = got;
    return next;
}
pub unsafe extern "C" fn inflateSync(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_uint = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut in_0: ::core::ffi::c_ulong = 0;
    let mut out: ::core::ffi::c_ulong = 0;
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    if (*strm).avail_in == 0 as crate::stdlib::uInt && (*state).bits < 8 as ::core::ffi::c_uint {
        return crate::zlib_h::Z_BUF_ERROR;
    }
    if (*state).mode as ::core::ffi::c_uint
        != crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*state).mode = crate::src::inflate::SYNC;
        (*state).hold >>= (*state).bits & 7 as ::core::ffi::c_uint;
        (*state).bits = (*state)
            .bits
            .wrapping_sub((*state).bits & 7 as ::core::ffi::c_uint);
        len = 0 as ::core::ffi::c_uint;
        while (*state).bits >= 8 as ::core::ffi::c_uint {
            let c2rust_fresh35 = len;
            len = len.wrapping_add(1);
            buf[c2rust_fresh35 as usize] = (*state).hold as ::core::ffi::c_uchar;
            (*state).hold >>= 8 as ::core::ffi::c_int;
            (*state).bits = (*state).bits.wrapping_sub(8 as ::core::ffi::c_uint);
        }
        (*state).have = 0 as ::core::ffi::c_uint;
        let state_ref = &mut *state;
        syncsearch(&mut state_ref.have, &buf[..len as usize]);
    }
    let input_len = (*strm).avail_in as usize;
    let input = if input_len == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts((*strm).next_in, input_len)
    };
    let state_ref = &mut *state;
    len = syncsearch(&mut state_ref.have, input);
    (*strm).avail_in = (*strm).avail_in.wrapping_sub(len);
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in = (*strm).total_in.wrapping_add(len as crate::stdlib::uLong);
    if (*state).have != 4 as ::core::ffi::c_uint {
        return crate::zlib_h::Z_DATA_ERROR;
    }
    if (*state).flags == -1 as ::core::ffi::c_int {
        (*state).wrap = 0 as ::core::ffi::c_int;
    } else {
        (*state).wrap &= !(4 as ::core::ffi::c_int);
    }
    flags = (*state).flags;
    in_0 = (*strm).total_in as ::core::ffi::c_ulong;
    out = (*strm).total_out as ::core::ffi::c_ulong;
    inflateReset(strm);
    (*strm).total_in = in_0 as crate::stdlib::uLong;
    (*strm).total_out = out as crate::stdlib::uLong;
    (*state).flags = flags;
    (*state).mode = crate::src::inflate::TYPE;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    inflateSync(strm)
}

fn inflate_sync_point(
    mode: crate::src::inflate::inflate_mode,
    bits: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    (mode == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint
        && bits == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
}

pub unsafe extern "C" fn inflateSyncPoint(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &*((*strm).state as *const crate::src::inflate::inflate_state);
    inflate_sync_point(state.mode, state.bits)
}
#[export_name = "inflateSyncPoint"]

pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateSyncPoint(strm)
}
pub unsafe extern "C" fn inflateCopy(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: Option<::core::ptr::NonNull<::core::ffi::c_uchar>> = None;
    if inflateStateCheck(source) != 0 || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*source).state as *mut crate::src::inflate::inflate_state;
    copy = Some((*source).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*source).opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if copy.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // The allocation is not observed before the complete state copy below.
    // Clearing it here would be dead work and only adds an unsafe foreign
    // memory call; the copy establishes every state byte before use.
    if (*state).window.is_some() {
        window =
            ::core::ptr::NonNull::new(Some((*source).zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*source).opaque,
                (1 as crate::stdlib::uInt) << (*state).wbits,
                ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
            ) as *mut ::core::ffi::c_uchar);
        if window.is_none() {
            Some((*source).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*source).opaque,
                copy as crate::stdlib::voidpf,
            );
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    // `dest` and `source`, and their separately allocated states, are
    // distinct by inflateCopy's existing C contract.
    ::core::ptr::copy_nonoverlapping(source, dest, 1);
    ::core::ptr::copy_nonoverlapping(state, copy, 1);
    (*copy).stream_identity = dest.addr();
    (*copy).lencode = (*state).lencode;
    (*copy).distcode = (*state).distcode;
    (*copy).next = (*state).next;
    if let Some(window) = window {
        ::core::ptr::copy_nonoverlapping(
            (*state).window.expect("history exists").as_ptr(),
            window.as_ptr(),
            (*state).whave as usize,
        );
    }
    (*copy).window = window;
    (*dest).state = copy as *mut crate::src::deflate::internal_state;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateCopy"]

pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateCopy(dest, source)
}

fn inflate_undermine_sane(sane: &mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    *sane = 1 as ::core::ffi::c_int;
    crate::zlib_h::Z_DATA_ERROR
}

pub unsafe extern "C" fn inflateUndermine(
    mut strm: crate::zlib_h::z_streamp,
    _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    inflate_undermine_sane(&mut state.sane)
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateUndermine(strm, subvert)
}

fn inflate_validate_wrap(
    wrap: &mut ::core::ffi::c_int,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if check != 0 && *wrap != 0 {
        *wrap |= 4 as ::core::ffi::c_int;
    } else {
        *wrap &= !(4 as ::core::ffi::c_int);
    }
    crate::zlib_h::Z_OK
}

pub unsafe extern "C" fn inflateValidate(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    inflate_validate_wrap(&mut state.wrap, check)
}
#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateValidate(strm, check)
}

fn inflate_mark_value(
    back: ::core::ffi::c_int,
    mode: crate::src::inflate::inflate_mode,
    length: ::core::ffi::c_uint,
    was: ::core::ffi::c_uint,
) -> ::core::ffi::c_long {
    ((back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int)
        as ::core::ffi::c_long
        + (if mode == crate::src::inflate::COPY_1 as ::core::ffi::c_int as ::core::ffi::c_uint {
            length
        } else if mode == crate::src::inflate::MATCH as ::core::ffi::c_int as ::core::ffi::c_uint {
            was.wrapping_sub(length)
        } else {
            0 as ::core::ffi::c_uint
        }) as ::core::ffi::c_long
}

pub unsafe extern "C" fn inflateMark(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_long {
    if inflateStateCheck(strm) != 0 {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    }
    let state = &*((*strm).state as *const crate::src::inflate::inflate_state);
    inflate_mark_value(state.back, state.mode, state.length, state.was)
}
#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    inflateMark(strm)
}

fn inflate_codes_used(next: usize) -> ::core::ffi::c_ulong {
    next as ::core::ffi::c_ulong
}

pub unsafe extern "C" fn inflateCodesUsed(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    if inflateStateCheck(strm) != 0 {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    }
    let state = &*((*strm).state as *const crate::src::inflate::inflate_state);
    inflate_codes_used(state.next)
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    inflateCodesUsed(strm)
}
