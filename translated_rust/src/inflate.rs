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

#[derive(Copy, Clone)]
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

unsafe extern "C" fn inflateStateCheck(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let strm = &*strm;
    let state = strm.state as *const crate::src::inflate::inflate_state;
    if state.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let state = &*state;
    (!inflate_state_valid(strm, state)) as ::core::ffi::c_int
}

fn inflate_state_valid(
    strm: &crate::zlib_h::z_stream_s,
    state: &crate::src::inflate::inflate_state,
) -> bool {
    strm.zalloc.is_some() && strm.zfree.is_some() && inflate_state_mode_valid(state)
}

fn inflate_state_mode_valid(state: &crate::src::inflate::inflate_state) -> bool {
    state.mode >= crate::src::inflate::HEAD && state.mode <= crate::src::inflate::SYNC
}

fn inflate_reset_keep_impl(
    strm: &mut crate::zlib_h::z_stream_s,
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
    state.next = 0;
    state.distcode = crate::src::inflate::distance_table::Dynamic(0);
    state.lencode = crate::src::inflate::length_table::Dynamic(0);
    state.sane = 1 as ::core::ffi::c_int;
    state.back = -1 as ::core::ffi::c_int;
    crate::zlib_h::Z_OK
}

pub unsafe extern "C" fn inflateResetKeep(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = strm.state as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state;
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_reset_keep_impl(strm, state)
}
#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = strm.state as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state;
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_reset_keep_impl(strm, state)
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
/// The gzip reader owns the stream for the duration of the call, so it can
/// borrow the ABI carrier directly.  Keep the one unavoidable state-pointer
/// conversion here, after checking it, instead of making each caller invoke
/// the raw-pointer API.
pub(crate) fn inflate_reset_gzip(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = strm.state as *mut crate::src::inflate::inflate_state;
    let Some(state) = (unsafe { state.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_reset_impl(strm, state)
}

pub unsafe fn inflateReset(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    inflate_reset_gzip(strm)
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateReset(strm)
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
    window_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = strm.state as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = unsafe { &mut *state };
    if !inflate_state_mode_valid(state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some((wrap, window_bits)) = inflate_reset2_window_bits(window_bits) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !state.window.is_null() && state.wbits != window_bits {
        let Some(zfree) = strm.zfree else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        unsafe { zfree(strm.opaque, state.window as crate::stdlib::voidpf) };
        state.window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    state.wrap = wrap;
    state.wbits = window_bits;
    state.wsize = 0;
    state.whave = 0;
    state.wnext = 0;
    inflate_reset_keep_impl(strm, state)
}

pub unsafe fn inflateReset2(
    strm: crate::zlib_h::z_streamp,
    window_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_reset2_impl(&mut *strm, window_bits)
}

#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_reset2_impl(&mut *strm, windowBits)
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
    crate::stdlib::memset(
        state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>(),
    );
    (*strm).state = state as *mut crate::src::deflate::internal_state;
    (*state).window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
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
    let allocators_present = strm.zalloc.is_some() && strm.zfree.is_some();
    let state = strm.state.cast::<crate::src::inflate::inflate_state>();
    let Some(state) = state.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_prime_from_state(allocators_present, state, bits, value)
}
unsafe fn updatewindow(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
    end: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if state.window.is_null() {
        state.window = Some(strm.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            strm.opaque,
            (1 as crate::stdlib::uInt) << state.wbits,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if state.window.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    if state.wsize == 0 as ::core::ffi::c_uint {
        state.wsize = (1 as ::core::ffi::c_uint) << state.wbits;
        state.wnext = 0 as ::core::ffi::c_uint;
        state.whave = 0 as ::core::ffi::c_uint;
    }
    let wsize = state.wsize as usize;
    let wnext = state.wnext as usize;
    if wnext > wsize {
        return 1 as ::core::ffi::c_int;
    }
    // `window` is allocated above (or supplied by inflateBackInit_) with
    // exactly `wsize` bytes.  Its lifetime is managed by the stream state.
    let window = ::core::slice::from_raw_parts_mut(state.window, wsize);
    if end.len() >= wsize {
        window.copy_from_slice(&end[end.len() - wsize..]);
        state.wnext = 0 as ::core::ffi::c_uint;
        state.whave = state.wsize;
    } else {
        let dist = (wsize - wnext).min(end.len());
        window[wnext..wnext + dist].copy_from_slice(&end[..dist]);
        let copy = end.len() - dist;
        if copy != 0 {
            window[..copy].copy_from_slice(&end[dist..]);
            state.wnext = copy as ::core::ffi::c_uint;
            state.whave = state.wsize;
        } else {
            state.wnext = state.wnext.wrapping_add(dist as ::core::ffi::c_uint);
            if state.wnext == state.wsize {
                state.wnext = 0 as ::core::ffi::c_uint;
            }
            if state.whave < state.wsize {
                state.whave = state.whave.wrapping_add(dist as ::core::ffi::c_uint);
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

pub unsafe fn inflate(
    strm: &mut crate::zlib_h::z_stream_s,
    mut flush: ::core::ffi::c_int,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let mut strm = InflateStream(strm);
    let mut next = InflateInput::new(input);
    let mut put = 0usize;
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
    if inflateStateCheck(strm.0) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if input.len() != (*strm).avail_in as usize || output.len() != (*strm).avail_out as usize {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut state = InflateState(&mut *((*strm).state as *mut crate::src::inflate::inflate_state));
    // The stream state retains this caller-owned header for the duration of
    // inflate().  Borrow it once, so header field access below stays within
    // the safe decoder state machine rather than repeatedly dereferencing the
    // ABI pointer.
    let mut header = if state.head.is_null() {
        None
    } else {
        Some(&mut *state.head)
    };
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
                                                                                                            if let Some(header) = header.as_deref_mut() {
                                                                                                                header.done = -1 as ::core::ffi::c_int;
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
                                                                                                        if let Some(header) = header.as_deref_mut() {
                                                                                                            header.text = (hold >> 8 as ::core::ffi::c_int
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
                                                                                        let mut table = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                                        (*state).distcode = crate::src::inflate::distance_table::Dynamic(0);
                                                                                        (*state).lencode = crate::src::inflate::length_table::Dynamic(0);
                                                                                        (*state).lenbits = 7 as ::core::ffi::c_uint;
                                                                                        ret = crate::src::inftrees::inflate_table(
                                                                                            crate::src::inftrees::CODES,
                                                                                            &raw mut (*state).lens as *mut ::core::ffi::c_ushort,
                                                                                            19 as ::core::ffi::c_uint,
                                                                                            
                                                                                            &raw mut table,
                                                                                            &raw mut (*state).lenbits,
                                                                                            &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                                        );
                                                                                        (*state).next = table.addr().wrapping_sub((&raw mut (*state).codes as *mut crate::src::inftrees::code).addr()) / ::core::mem::size_of::<crate::src::inftrees::code>();
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
                                                                                ) = header
                                                                                    .as_deref_mut()
                                                                                {
                                                                                    header.time = hold
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
                                                                        if (*state).lens[256 as usize] as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            (*strm).msg = b"invalid code -- missing end-of-block\0"
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            (*state).mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            (*state).next = 0;
                                                                            let mut table = &raw mut (*state).codes as *mut crate::src::inftrees::code;
                                                                            (*state).lencode = crate::src::inflate::length_table::Dynamic(0);
                                                                            (*state).lenbits = 9 as ::core::ffi::c_uint;
                                                                            ret = crate::src::inftrees::inflate_table(
                                                                                crate::src::inftrees::LENS,
                                                                                &raw mut (*state).lens as *mut ::core::ffi::c_ushort,
                                                                                (*state).nlen,
                                                                                
                                                                                &raw mut table,
                                                                                &raw mut (*state).lenbits,
                                                                                &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                            );
                                                                            (*state).next = table.addr().wrapping_sub((&raw mut (*state).codes as *mut crate::src::inftrees::code).addr()) / ::core::mem::size_of::<crate::src::inftrees::code>();
                                                                            if ret != 0 {
                                                                                (*strm).msg = b"invalid literal/lengths set\0".as_ptr()
                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                (*state).mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                (*state).distcode = crate::src::inflate::distance_table::Dynamic((*state).next);
                                                                                (*state).distbits = 6 as ::core::ffi::c_uint;
                                                                                ret = crate::src::inftrees::inflate_table(
                                                                                    crate::src::inftrees::DISTS,
                                                                                    (&raw mut (*state).lens as *mut ::core::ffi::c_ushort)
                                                                                        .wrapping_offset((*state).nlen as isize),
                                                                                    (*state).ndist,
                                                                                    
                                                                                    &raw mut table,
                                                                                    &raw mut (*state).distbits,
                                                                                    &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                                                                );
                                                                                (*state).next = table.addr().wrapping_sub((&raw mut (*state).codes as *mut crate::src::inftrees::code).addr()) / ::core::mem::size_of::<crate::src::inftrees::code>();
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
                                                                    header.as_deref_mut()
                                                                {
                                                                    header.xflags = (hold
                                                                        & 0xff
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int;
                                                                    header.os = (hold
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
                                                                crate::src::inftrees::inflate_fixed(
                                                                    &mut *state,
                                                                );
                                                                (*state).mode =
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
                                                    if let Some(header) = header.as_deref_mut() {
                                                        header.extra_len = hold
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
                                                } else if let Some(header) = header.as_deref_mut() {
                                                    header.extra = ::core::ptr::null_mut::<
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
                                            (*strm).msg = b"invalid literal/length code\0".as_ptr()
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
                                    if (*state).flags & 0x400 as ::core::ffi::c_int != 0 {
                                        copy = (*state).length;
                                        if copy > have {
                                            copy = have;
                                        }
                                        if copy != 0 {
                                            if let Some(header) = header.as_deref_mut() {
                                                if !header.extra.is_null() && {
                                                    len = (header.extra_len as ::core::ffi::c_uint)
                                                        .wrapping_sub((*state).length);
                                                    len < header.extra_max
                                                } {
                                                    let extra_copy = if len.wrapping_add(copy)
                                                        > header.extra_max
                                                    {
                                                        (header.extra_max as ::core::ffi::c_uint)
                                                            .wrapping_sub(len)
                                                    } else {
                                                        copy
                                                    }
                                                        as usize;
                                                    let input_start = input.len() - have as usize;
                                                    ::core::slice::from_raw_parts_mut(
                                                        header.extra.wrapping_add(len as usize),
                                                        extra_copy,
                                                    )
                                                    .copy_from_slice(
                                                        &input
                                                            [input_start..input_start + extra_copy],
                                                    );
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
                                    if let Some(header) = header.as_deref_mut() {
                                        if !header.name.is_null()
                                            && (*state).length < header.name_max
                                        {
                                            let c2rust_fresh6 = (*state).length;
                                            (*state).length = (*state).length.wrapping_add(1);
                                            *header.name.wrapping_offset(c2rust_fresh6 as isize) =
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
                            } else if let Some(header) = header.as_deref_mut() {
                                header.name = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
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
                            len = *next.wrapping_offset(c2rust_fresh7 as isize)
                                as ::core::ffi::c_uint;
                            if let Some(header) = header.as_deref_mut() {
                                if !header.comment.is_null() && (*state).length < header.comm_max {
                                    let c2rust_fresh8 = (*state).length;
                                    (*state).length = (*state).length.wrapping_add(1);
                                    *header.comment.wrapping_offset(c2rust_fresh8 as isize) =
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
                    } else if let Some(header) = header.as_deref_mut() {
                        header.comment = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
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
                    (*strm).msg = b"header crc mismatch\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue '_inf_leave;
                } else {
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                }
            }
            if let Some(header) = header.as_deref_mut() {
                header.hcrc = (*state).flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                header.done = 1 as ::core::ffi::c_int;
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
        if (*state).offset > copy {
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
                    .wrapping_offset((*state).wsize.wrapping_sub(copy) as isize);
            } else {
                from = (*state)
                    .window
                    .wrapping_offset((*state).wnext.wrapping_sub(copy) as isize);
            }
            if copy > (*state).length {
                copy = (*state).length;
            }
        } else {
            from = output
                .as_mut_ptr()
                .wrapping_add(put - (*state).offset as usize);
            copy = (*state).length;
        }
        if copy > left {
            copy = left;
        }
        left = left.wrapping_sub(copy);
        (*state).length = (*state).length.wrapping_sub(copy);
        loop {
            let c2rust_fresh30 = from;
            from = from.wrapping_offset(1);
            output[put] = *c2rust_fresh30;
            put += 1;
            copy = copy.wrapping_sub(1);
            if copy == 0 {
                break;
            }
        }
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
        if updatewindow(strm.0, &mut *state, &output[put - copied..put]) != 0 {
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
    return ret;
}
/// Converts the ABI stream's current buffers once before entering the
/// slice-based decoder.  Keeping this boundary out of the exported wrapper
/// leaves the latter responsible only for validating and borrowing `strm`.
fn inflate_from_stream(
    strm: &mut crate::zlib_h::z_stream_s,
    flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strm.next_out.is_null() || (strm.next_in.is_null() && strm.avail_in != 0) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let input = if strm.avail_in == 0 {
        &[]
    } else {
        // `next_in` was checked above and the caller owns the advertised
        // range for the duration of this FFI call.
        unsafe { ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize) }
    };
    // zlib requires `next_out` even for a zero-sized output range.
    let output =
        unsafe { ::core::slice::from_raw_parts_mut(strm.next_out, strm.avail_out as usize) };
    unsafe { inflate(strm, flush, input, output) }
}

#[export_name = "inflate"]
pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_from_stream(&mut *strm, flush)
}
pub unsafe fn inflateEnd(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    let state = strm.state as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state;
    if !inflate_state_valid(strm, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if !state.window.is_null() {
        Some(strm.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            strm.opaque,
            state.window as crate::stdlib::voidpf,
        );
    }
    Some(strm.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        strm.opaque,
        strm.state as crate::stdlib::voidpf,
    );
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    crate::zlib_h::Z_OK
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflateEnd(&mut *strm)
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
        if state.window.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        Some(::core::slice::from_raw_parts(
            state.window,
            state.wsize as usize,
        ))
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
pub unsafe fn inflateSetDictionary(
    strm: &mut crate::zlib_h::z_stream_s,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    if crate::stdlib::uInt::try_from(dictionary.len()).is_err() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Validate the stream before borrowing the state behind its raw link.
    if inflateStateCheck(strm as *mut crate::zlib_h::z_stream_s) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
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
    if updatewindow(strm, state, dictionary) != 0 {
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
    inflateSetDictionary(strm, dictionary)
}
fn inflate_get_header_impl(wrap: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::zlib_h::Z_OK
}

pub unsafe fn inflateGetHeader(
    strm: crate::zlib_h::z_streamp,
    head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Validate the stream before borrowing its state through the ABI link.
    if inflateStateCheck(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(state) = (strm.state as *mut crate::src::inflate::inflate_state).as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let ret = inflate_get_header_impl(state.wrap);
    if ret != crate::zlib_h::Z_OK {
        return ret;
    }
    let Some(head) = head.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    state.head = head;
    head.done = 0 as ::core::ffi::c_int;
    ret
}
#[export_name = "inflateGetHeader"]

pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    inflateGetHeader(strm, head)
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
        syncsearch(&mut (*state).have, &buf[..len as usize]);
    }
    let input_start = (*strm).next_in;
    let input_len = (*strm).avail_in as usize;
    if input_len > isize::MAX as usize || (input_len != 0 && input_start.is_null()) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let input = if input_len == 0 {
        &[]
    } else {
        // `next_in` is part of the stream's caller-provided input range.
        // The length is bounded above before constructing this transient
        // slice, satisfying `from_raw_parts`' platform-size requirement.
        ::core::slice::from_raw_parts(input_start, input_len)
    };
    len = syncsearch(&mut (*state).have, input);
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
    inflateReset(&mut *strm);
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
fn inflate_sync_point_impl(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_int {
    (state.mode as ::core::ffi::c_uint
        == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint
        && state.bits == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
}

fn inflate_sync_point_from_stream(strm: &crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    let Some(state) = inflate_mark_state(strm) else {
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
    inflate_sync_point_from_stream(strm)
}
unsafe fn inflate_copy_impl(
    dest: &mut crate::zlib_h::z_stream_s,
    source: &crate::zlib_h::z_stream_s,
    state: &crate::src::inflate::inflate_state,
) -> ::core::ffi::c_int {
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    copy = Some(source.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        source.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if copy.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !state.window.is_null() {
        window = Some(source.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            source.opaque,
            (1 as crate::stdlib::uInt) << state.wbits,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if window.is_null() {
            Some(source.zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                source.opaque, copy as crate::stdlib::voidpf
            );
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    *dest = *source;
    *copy = *state;
    (*copy).next = state.next;
    if !window.is_null() {
        ::core::ptr::copy_nonoverlapping(state.window, window, state.whave as usize);
    }
    (*copy).window = window;
    dest.state = copy as *mut crate::src::deflate::internal_state;
    return crate::zlib_h::Z_OK;
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
    if !inflate_state_valid(&source, state) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_copy_impl(&mut *dest, &source, state)
}
fn inflate_undermine_impl(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    let Some(state) = inflate_validate_state(strm) else {
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
    inflate_undermine_impl(strm)
}
fn inflate_validate_state(
    strm: &mut crate::zlib_h::z_stream_s,
) -> Option<&mut crate::src::inflate::inflate_state> {
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return None;
    }
    let state = strm.state.cast::<crate::src::inflate::inflate_state>();
    let state = unsafe { state.as_mut() }?;
    inflate_state_mode_valid(state).then_some(state)
}

fn inflate_validate_impl(
    strm: &mut crate::zlib_h::z_stream_s,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(state) = inflate_validate_state(strm) else {
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
    inflate_validate_impl(strm, check)
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
fn inflate_mark_state(
    strm: &crate::zlib_h::z_stream_s,
) -> Option<&crate::src::inflate::inflate_state> {
    if strm.zalloc.is_none() || strm.zfree.is_none() {
        return None;
    }
    let state = strm.state.cast::<crate::src::inflate::inflate_state>();
    let state = unsafe { state.as_ref() }?;
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
    let Some(state) = inflate_mark_state(strm) else {
        return invalid_mark;
    };
    inflate_mark_impl(state)
}
fn inflate_codes_used_impl(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_ulong {
    state.next as ::core::ffi::c_ulong
}

fn inflate_codes_used_from_stream(strm: &crate::zlib_h::z_stream_s) -> ::core::ffi::c_ulong {
    let Some(state) = inflate_mark_state(strm) else {
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
    inflate_codes_used_from_stream(strm)
}
