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

/// Construct the exact all-zero state that `inflateInit2_()` historically
/// obtained from `memset`, with the few fields it initialized immediately
/// afterwards already set.  Keeping this as an ordinary value makes the
/// initial state explicit; the allocator-owned pointer is written only by
/// the export boundary.
pub(crate) fn inflate_initial_state() -> inflate_state {
    let empty_code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    inflate_state {
        strm: ::core::ptr::null_mut(),
        mode: crate::src::inflate::HEAD,
        last: 0,
        wrap: 0,
        havedict: 0,
        flags: 0,
        dmax: 0,
        check: 0,
        total: 0,
        head: ::core::ptr::null_mut::<crate::zlib_h::gz_header>(),
        wbits: 0,
        wsize: 0,
        whave: 0,
        wnext: 0,
        window: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
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
        codes: [empty_code; 1444],
        sane: 0,
        back: 0,
        was: 0,
    }
}
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
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

/// Update the wrapper checksum for bytes that the decoder has already
/// produced.  The surrounding codec boundary owns the temporary output
/// view; keeping the choice of checksum here avoids duplicating that raw
/// view across the gzip and zlib branches.
fn inflate_output_checksum(
    check: crate::stdlib::uLong,
    flags: ::core::ffi::c_int,
    output: &[u8],
) -> ::core::ffi::c_ulong {
    if flags != 0 {
        crate::src::crc32::crc32_z(check, output) as ::core::ffi::c_ulong
    } else {
        crate::src::adler32::adler32_z(check, output) as ::core::ffi::c_ulong
    }
}

/// Decide and compute the checksum publication for ordinary inflate's exit.
/// The decoder boundary owns the temporary output view; this core makes the
/// wrapper bit and empty-output behavior explicit without consulting ABI
/// state.
fn inflate_exit_needs_checksum(wrap: ::core::ffi::c_int, output_len: usize) -> bool {
    wrap & 4 as ::core::ffi::c_int != 0 && output_len != 0
}

fn inflate_exit_checksum(
    wrap: ::core::ffi::c_int,
    check: crate::stdlib::uLong,
    flags: ::core::ffi::c_int,
    output: &[u8],
) -> Option<::core::ffi::c_ulong> {
    if !inflate_exit_needs_checksum(wrap, output.len()) {
        None
    } else {
        Some(inflate_output_checksum(check, flags, output))
    }
}

/// Update the gzip-header CRC from an already-bounded byte span.  The
/// transitional decoder owns any raw cursor lending; header parsing itself
/// only carries this scalar checksum and a safe byte slice.
fn inflate_header_crc_update(check: ::core::ffi::c_ulong, bytes: &[u8]) -> ::core::ffi::c_ulong {
    crate::src::crc32::crc32_z(check as crate::stdlib::uLong, bytes) as ::core::ffi::c_ulong
}

// Keep all inflate diagnostics in one immutable table.  Besides making their
// storage explicit, this lets gzip retain an inflate error without treating
// `strm.msg` as an arbitrary foreign C string.
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

/// Validate the scalar portion of an inflate stream/state relationship.
/// Pointer validation remains at the ABI adapter, while this core documents
/// the complete set of modes accepted by zlib without pointer access.
pub(crate) fn inflate_state_values_are_valid(
    has_zalloc: bool,
    has_zfree: bool,
    state_matches_stream: bool,
    mode: crate::src::inflate::inflate_mode,
) -> bool {
    has_zalloc
        && has_zfree
        && state_matches_stream
        && (crate::src::inflate::HEAD..=crate::src::inflate::SYNC).contains(&mode)
}

/// Bound one stored-block transfer by the decoder's remaining input and
/// output. The ABI loops keep their cursors at the boundary; this core owns
/// only the scalar progress calculation.
pub(crate) fn inflate_stored_copy_len(
    remaining: ::core::ffi::c_uint,
    available_input: ::core::ffi::c_uint,
    available_output: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    remaining.min(available_input).min(available_output)
}

// This is expanded only in existing ABI/codec boundaries. It keeps raw
// stream/state adoption at those boundaries, while the validity decision
// itself remains the pointer-free scalar core above.
macro_rules! inflate_state_check_at_boundary {
    ($strm:expr) => {{
        let strm = $strm;
        if strm.is_null() {
            1
        } else {
            let strm_ref = &*strm;
            let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
            if state.is_null() {
                1
            } else {
                let state = &*state;
                (!crate::src::inflate::inflate_state_values_are_valid(
                    strm_ref.zalloc.is_some(),
                    strm_ref.zfree.is_some(),
                    state.strm == strm,
                    state.mode,
                )) as ::core::ffi::c_int
            }
        }
    }};
}
pub(crate) use inflate_state_check_at_boundary;
// These reset/init transitions manipulate caller-owned stream state and may
// invoke its allocator.  Expand them only at ABI boundaries until that state
// has an owned Rust representation.
macro_rules! inflate_reset_keep_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = (*strm).state as *mut crate::src::inflate::inflate_state;
            (*state).total = 0 as ::core::ffi::c_ulong;
            (*strm).total_out = (*state).total as crate::stdlib::uLong;
            (*strm).total_in = (*strm).total_out;
            (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*strm).data_type = 0 as ::core::ffi::c_int;
            if (*state).wrap != 0 {
                (*strm).adler = ((*state).wrap & 1 as ::core::ffi::c_int) as crate::stdlib::uLong;
            }
            (*state).mode = crate::src::inflate::HEAD;
            (*state).last = 0 as ::core::ffi::c_int;
            (*state).havedict = 0 as ::core::ffi::c_int;
            (*state).flags = -1 as ::core::ffi::c_int;
            (*state).dmax = 32768 as ::core::ffi::c_uint;
            (*state).head = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
            (*state).hold = 0 as ::core::ffi::c_ulong;
            (*state).bits = 0 as ::core::ffi::c_uint;
            (*state).next = &raw mut (*state).codes as *mut crate::src::inftrees::code;
            (*state).distcode = (*state).next;
            (*state).lencode = (*state).distcode;
            (*state).sane = 1 as ::core::ffi::c_int;
            (*state).back = -1 as ::core::ffi::c_int;
            crate::zlib_h::Z_OK
        }
    }};
}
pub(crate) use inflate_reset_keep_at_boundary;

macro_rules! inflate_reset_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = (*strm).state as *mut crate::src::inflate::inflate_state;
            (*state).wsize = 0 as ::core::ffi::c_uint;
            (*state).whave = 0 as ::core::ffi::c_uint;
            (*state).wnext = 0 as ::core::ffi::c_uint;
            crate::src::inflate::inflate_reset_keep_at_boundary!(strm)
        }
    }};
}
pub(crate) use inflate_reset_at_boundary;

macro_rules! inflate_reset2_at_boundary {
    ($strm:expr, $window_bits:expr $(,)?) => {{
        let strm = $strm;
        let mut window_bits = $window_bits;
        if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else if window_bits < -15 as ::core::ffi::c_int {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = (*strm).state as *mut crate::src::inflate::inflate_state;
            let wrap: ::core::ffi::c_int;
            if window_bits < 0 as ::core::ffi::c_int {
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
                crate::zlib_h::Z_STREAM_ERROR
            } else {
                if !(*state).window.is_null()
                    && (*state).wbits != window_bits as ::core::ffi::c_uint
                {
                    Some((*strm).zfree.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        (*strm).opaque,
                        (*state).window as crate::stdlib::voidpf,
                    );
                    (*state).window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
                (*state).wrap = wrap;
                (*state).wbits = window_bits as ::core::ffi::c_uint;
                crate::src::inflate::inflate_reset_at_boundary!(strm)
            }
        }
    }};
}
pub(crate) use inflate_reset2_at_boundary;

macro_rules! inflate_init2_at_boundary {
    ($strm:expr, $window_bits:expr, $version:expr, $stream_size:expr $(,)?) => {{
        let strm = $strm;
        let window_bits = $window_bits;
        let version = $version;
        let stream_size = $stream_size;
        if version.is_null()
            || *version as ::core::ffi::c_int
                != crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
            || stream_size
                != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
        {
            crate::zlib_h::Z_VERSION_ERROR
        } else if strm.is_null() {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if (*strm).zalloc.is_none() {
                (*strm).zalloc = Some(
                    crate::src::zutil::zcalloc_ffi
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
                    crate::src::zutil::zcfree_ffi
                        as unsafe extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> (),
                ) as crate::zlib_h::free_func;
            }
            let state = Some((*strm).zalloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*strm).opaque,
                1 as crate::stdlib::uInt,
                ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
            ) as *mut crate::src::inflate::inflate_state;
            if state.is_null() {
                crate::zlib_h::Z_MEM_ERROR
            } else {
                ::core::ptr::write(state, crate::src::inflate::inflate_initial_state());
                (*strm).state = state as *mut crate::src::deflate::internal_state;
                (*state).strm = strm;
                let ret = crate::src::inflate::inflate_reset2_at_boundary!(strm, window_bits);
                if ret != crate::zlib_h::Z_OK {
                    Some((*strm).zfree.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        (*strm).opaque,
                        state as crate::stdlib::voidpf,
                    );
                    (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
                }
                ret
            }
        }
    }};
}
pub(crate) use inflate_init2_at_boundary;

#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflate_reset_keep_at_boundary!(strm)
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflate_reset_at_boundary!(strm)
}
#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_reset2_at_boundary!(strm, windowBits)
}
#[export_name = "inflateInit2_"]

pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_init2_at_boundary!(strm, windowBits, version, stream_size)
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_init2_at_boundary!(strm, crate::zutil_h::DEF_WBITS, version, stream_size)
}
/// Apply an `inflatePrime()` request to scalar bit-buffer state.  The export
/// boundary validates and owns the stream state; this core preserves zlib's
/// bit masking and wrapping arithmetic without accessing raw state.
fn inflate_prime_update(
    hold: ::core::ffi::c_ulong,
    held_bits: ::core::ffi::c_uint,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> Result<(::core::ffi::c_ulong, ::core::ffi::c_uint), ()> {
    if bits == 0 {
        return Ok((hold, held_bits));
    }
    if bits < 0 {
        return Ok((0, 0));
    }
    if bits > 16 || held_bits.wrapping_add(bits as ::core::ffi::c_uint) > 32 {
        return Err(());
    }
    let value = (value as ::core::ffi::c_long
        & ((1 as ::core::ffi::c_long) << bits) - 1 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
    Ok((
        hold.wrapping_add((value as ::core::ffi::c_ulong) << held_bits),
        held_bits.wrapping_add(bits as ::core::ffi::c_uint),
    ))
}

#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    let Ok((hold, held_bits)) = inflate_prime_update(state.hold, state.bits, bits, value) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    state.hold = hold;
    state.bits = held_bits;
    crate::zlib_h::Z_OK
}
/// The two copy operations needed to append decoded bytes to inflate's
/// circular history window.  This is deliberately pointer-free: the caller
/// owns the ABI allocation and performs the copies only after this plan has
/// proved the window cursor and lengths are coherent.
#[derive(Copy, Clone)]
struct InflateWindowCopyPlan {
    first_dest: usize,
    first_from_end: usize,
    first_len: usize,
    second_from_end: usize,
    second_len: usize,
    next: usize,
    have: usize,
}

impl InflateWindowCopyPlan {
    fn cursor_values(self) -> Option<(::core::ffi::c_uint, ::core::ffi::c_uint)> {
        Some((
            ::core::ffi::c_uint::try_from(self.next).ok()?,
            ::core::ffi::c_uint::try_from(self.have).ok()?,
        ))
    }
}

/// Compute the circular-window update without touching the ABI window
/// pointer.  `copy` is the number of bytes immediately preceding `end`.
/// Invalid internal cursor state is rejected before the boundary performs a
/// pointer offset or memory copy.
fn inflate_window_copy_plan(
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    copy: ::core::ffi::c_uint,
) -> Option<InflateWindowCopyPlan> {
    let wsize = usize::try_from(wsize).ok()?;
    let wnext = usize::try_from(wnext).ok()?;
    let whave = usize::try_from(whave).ok()?;
    let copy = usize::try_from(copy).ok()?;
    if wsize == 0 || wnext > wsize || whave > wsize {
        return None;
    }

    if copy >= wsize {
        return Some(InflateWindowCopyPlan {
            first_dest: 0,
            first_from_end: wsize,
            first_len: wsize,
            second_from_end: 0,
            second_len: 0,
            next: 0,
            have: wsize,
        });
    }

    let first_len = wsize.checked_sub(wnext)?.min(copy);
    let remaining = copy.checked_sub(first_len)?;
    let (next, have) = if remaining != 0 {
        (remaining, wsize)
    } else {
        let next = wnext.checked_add(first_len)?;
        let next = if next == wsize { 0 } else { next };
        let have = whave.checked_add(first_len)?.min(wsize);
        (next, have)
    };

    Some(InflateWindowCopyPlan {
        first_dest: wnext,
        first_from_end: copy,
        first_len,
        second_from_end: remaining,
        second_len: remaining,
        next,
        have,
    })
}

/// Copy the just-produced output tail into the circular history window.  The
/// codec boundary lends the two validated slices; all tail/destination range
/// arithmetic remains here, where it is checked before either copy.
fn inflate_window_copy(
    window: &mut [u8],
    produced: &[u8],
    plan: InflateWindowCopyPlan,
) -> Option<()> {
    let first_start = produced.len().checked_sub(plan.first_from_end)?;
    let first_end = first_start.checked_add(plan.first_len)?;
    let first = produced.get(first_start..first_end)?;
    let first_dest_end = plan.first_dest.checked_add(first.len())?;
    window
        .get_mut(plan.first_dest..first_dest_end)?
        .copy_from_slice(first);

    let second_start = produced.len().checked_sub(plan.second_from_end)?;
    let second_end = second_start.checked_add(plan.second_len)?;
    let second = produced.get(second_start..second_end)?;
    window.get_mut(..second.len())?.copy_from_slice(second);
    Some(())
}

/// Resolve the physical history-window length before the ABI boundary lends
/// the allocation as a slice.  A zero `wsize` is the pre-allocation state;
/// once a window exists, its recorded size must still fit in `usize`.
///
/// zlib only supports 8--15 window bits.  Keeping that bound here prevents a
/// malformed opaque state from requesting a wrapped or impractically large
/// compatibility allocation before the safe copy core gets a chance to
/// validate it.
fn inflate_window_len(wbits: ::core::ffi::c_uint, wsize: ::core::ffi::c_uint) -> Option<usize> {
    if !(8..=15).contains(&wbits) {
        return None;
    }
    if wsize == 0 {
        usize::try_from(1_u32.checked_shl(wbits)?).ok()
    } else {
        usize::try_from(wsize).ok()
    }
}

/// The scalar state to publish after a successful circular history update.
/// Keeping this separate from the copy lets the safe core validate and copy
/// before its caller commits any compatibility-state fields.
#[derive(Copy, Clone)]
struct InflateWindowUpdate {
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
}

/// Update the circular history window after a decoder call. Allocation and
/// ABI-owned buffer lending stay at the codec boundary; this core owns the
/// window sizing, cursor planning, and bounded copies, returning the scalar
/// state to commit only after all validation and copying succeeds.
fn inflate_window_update(
    window: &mut [u8],
    produced: &[u8],
    wbits: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
) -> Option<InflateWindowUpdate> {
    let wsize = if wsize == 0 {
        ::core::ffi::c_uint::try_from(inflate_window_len(wbits, 0)?).ok()?
    } else {
        wsize
    };
    let wsize_len = inflate_window_len(wbits, wsize)?;
    if window.len() != wsize_len {
        return None;
    }
    let copy = ::core::ffi::c_uint::try_from(produced.len()).ok()?;
    let plan = inflate_window_copy_plan(wsize, wnext, whave, copy)?;
    let (next, have) = plan.cursor_values()?;
    inflate_window_copy(window, produced, plan)?;
    Some(InflateWindowUpdate {
        wsize,
        wnext: next,
        whave: have,
    })
}

/// Resolve a dynamic decode-table cursor to a bounded tail of `codes`.
/// `cursor` is only an address token here; no implementation dereferences it.
pub(crate) fn inflate_fast_dynamic_table(
    codes: &[crate::src::inftrees::code],
    cursor: usize,
) -> Option<&[crate::src::inftrees::code]> {
    let code_size = ::core::mem::size_of::<crate::src::inftrees::code>();
    let code_start = codes.as_ptr() as usize;
    let code_end = code_start.checked_add(::core::mem::size_of_val(codes))?;
    cursor
        .checked_sub(code_start)
        .filter(|offset| code_size != 0 && *offset % code_size == 0 && cursor <= code_end)
        .and_then(|offset| codes.get(offset / code_size..))
}

/// Turn a compatibility table cursor into an index without relying on raw
/// same-allocation pointer-distance operations.  Callers keep the raw cursor
/// validation at their ABI boundary and pass only address tokens here.
fn inflate_code_index(code_start: usize, code_len: usize, cursor: usize) -> Option<usize> {
    let code_size = ::core::mem::size_of::<crate::src::inftrees::code>();
    let byte_len = code_len.checked_mul(code_size)?;
    let code_end = code_start.checked_add(byte_len)?;
    let offset = cursor.checked_sub(code_start)?;
    if cursor > code_end || code_size == 0 || offset % code_size != 0 {
        return None;
    }
    let index = offset / code_size;
    (index <= code_len).then_some(index)
}

/// Describe how an `inflateCopy()` boundary must rebase its decode-table
/// cursors after copying the owning state record.  The compatibility fields
/// remain address tokens at the ABI boundary; this plan performs their range
/// validation and index calculation without relational raw-pointer
/// comparisons or raw-pointer arithmetic.
#[derive(Copy, Clone)]
struct InflateCopyCodeCursors {
    lencode: Option<usize>,
    distcode: Option<usize>,
    next: usize,
}

fn inflate_copy_code_cursors(
    source_code_start: usize,
    code_len: usize,
    lencode: usize,
    distcode: usize,
    next: usize,
) -> InflateCopyCodeCursors {
    // The original cursor-range test accepted only an actual table element
    // for `lencode`, not the one-past-end sentinel.  Preserve that detail
    // before attempting to rebase the paired distance-table cursor.
    let lencode =
        inflate_code_index(source_code_start, code_len, lencode).filter(|index| *index < code_len);
    let distcode = lencode.and_then(|_| inflate_code_index(source_code_start, code_len, distcode));
    InflateCopyCodeCursors {
        // zlib's copied state retains its original table tokens when the
        // paired cursor is malformed.  Only publish rebased table cursors
        // when both indices passed the existing boundary validation.
        lencode: distcode.and(lencode),
        distcode,
        // The translated implementation reset an invalid `next` cursor to
        // the start of `codes`; retain that compatibility behavior.
        next: inflate_code_index(source_code_start, code_len, next).unwrap_or(0),
    }
}

/// Resolve the two active decode-table cursors to their bounded table tails.
/// Fixed tables have stable static storage; dynamic tables live in `codes`.
/// The codec boundary uses this only to lend the safe fast decoder its table
/// views, never to dereference either raw compatibility cursor directly.
pub(crate) fn inflate_fast_tables(
    codes: &[crate::src::inftrees::code],
    lencode: usize,
    distcode: usize,
) -> Option<(&[crate::src::inftrees::code], &[crate::src::inftrees::code])> {
    let lcode = inflate_fast_table(
        codes,
        lencode,
        crate::src::inftrees::inffixed_h::lenfix.as_ptr() as usize,
        &crate::src::inftrees::inffixed_h::lenfix,
    );
    let dcode = inflate_fast_table(
        codes,
        distcode,
        crate::src::inftrees::inffixed_h::distfix.as_ptr() as usize,
        &crate::src::inftrees::inffixed_h::distfix,
    );
    Some((lcode?, dcode?))
}

/// Resolve one compatibility table cursor to a bounded table view.  The
/// cursor remains an address token: all table access after this point is by
/// checked slice indexing.
fn inflate_fast_table<'a>(
    codes: &'a [crate::src::inftrees::code],
    cursor: usize,
    fixed_cursor: usize,
    fixed: &'a [crate::src::inftrees::code],
) -> Option<&'a [crate::src::inftrees::code]> {
    if cursor == fixed_cursor {
        Some(fixed)
    } else {
        inflate_fast_dynamic_table(codes, cursor)
    }
}

/// Run normal inflate's bounded fast decoder after its ABI boundary has
/// validated and lent the input, output, and history spans.  Table-token
/// resolution and the complete scalar decoder configuration live here so the
/// legacy loop only performs the temporary boundary conversion and commits
/// the resulting progress.
///
/// `output` and `window` must not overlap.  Normal inflate owns a separate
/// history window; unlike inflateBack, it can therefore lend both views to
/// the slice-only core at once.
#[allow(clippy::too_many_arguments)]
fn inflate_fast_normal(
    input: &[u8],
    output: &mut [u8],
    output_start: usize,
    window: &[u8],
    lcode: &[crate::src::inftrees::code],
    dcode: &[crate::src::inftrees::code],
    wsize: usize,
    whave: usize,
    wnext: usize,
    sane: bool,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
    lenbits: ::core::ffi::c_uint,
    distbits: ::core::ffi::c_uint,
    start: ::core::ffi::c_uint,
) -> crate::src::inffast::InflateFastProgress {
    crate::src::inffast::inflate_fast_core(crate::src::inffast::InflateFastViews {
        input,
        output,
        output_start,
        history: crate::src::inffast::InflateFastHistory::Separate(window),
        lcode,
        dcode,
        wsize,
        whave,
        wnext,
        sane,
        hold,
        bits,
        lenbits,
        distbits,
        start,
    })
}

/// The scalar shape of ordinary inflate's fast-decoder lends.  The legacy
/// decoder still owns the ABI cursor conversion, but it must establish all
/// of these bounds before making its temporary input, output, and history
/// views.
#[derive(Copy, Clone)]
struct InflateFastSpan {
    input_len: usize,
    output_start: usize,
    output_len: usize,
    window_len: usize,
}

/// Validate the spans used by normal inflate's bounded fast path without
/// looking through compatibility pointers.  `out` is the output capacity at
/// entry and `left` is the remaining capacity at the fast-path boundary, so
/// their checked difference is the output already produced by the slow path.
fn inflate_fast_span(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    out: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
) -> Option<InflateFastSpan> {
    if have < 6 || left < 258 {
        return None;
    }
    let output_start = usize::try_from(out.checked_sub(left)?).ok()?;
    let output_len = usize::try_from(out).ok()?;
    if output_start > output_len {
        return None;
    }
    Some(InflateFastSpan {
        input_len: usize::try_from(have).ok()?,
        output_start,
        output_len,
        window_len: usize::try_from(wsize).ok()?,
    })
}

/// A checked, pointer-free commit plan for one bounded fast-decode call.
/// The transitional decoder keeps compatibility cursors at its boundary, but
/// it can validate all scalar progress before publishing any of them.
struct InflateFastCommit {
    input_used: ::core::ffi::c_uint,
    output_used: ::core::ffi::c_uint,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
    mode: Option<inflate_mode>,
    error: Option<usize>,
}

fn inflate_fast_commit(
    progress: crate::src::inffast::InflateFastProgress,
    available_input: ::core::ffi::c_uint,
    available_output: ::core::ffi::c_uint,
) -> Option<InflateFastCommit> {
    let input_used = ::core::ffi::c_uint::try_from(progress.input_used).ok()?;
    let output_used = ::core::ffi::c_uint::try_from(progress.output_used).ok()?;
    if input_used > available_input || output_used > available_output {
        return None;
    }
    Some(InflateFastCommit {
        input_used,
        output_used,
        hold: crate::stdlib::uLong::try_from(progress.hold).ok()?,
        bits: ::core::ffi::c_uint::try_from(progress.bits).ok()?,
        mode: progress.mode,
        error: progress.error,
    })
}

/// Scalar state published when one ordinary `inflate()` call leaves its
/// legacy cursor loop.  Keeping these calculations independent of ABI
/// records makes the exit boundary responsible only for cursor lending,
/// checksum input, and field commits.
#[derive(Copy, Clone)]
struct InflateExitProgress {
    input_used: ::core::ffi::c_uint,
    output_used: ::core::ffi::c_uint,
    update_window: bool,
    data_type: ::core::ffi::c_int,
}

/// Preserve ordinary inflate's exit accounting without looking through the
/// compatibility stream or state records.  The decoder only decrements the
/// two availability counters, so wrapping subtraction retains the translated
/// ABI behavior even if a malformed caller supplied unusual scalar values.
fn inflate_exit_progress(
    initial_input: ::core::ffi::c_uint,
    remaining_input: ::core::ffi::c_uint,
    initial_output: ::core::ffi::c_uint,
    remaining_output: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    mode: inflate_mode,
    flush: ::core::ffi::c_int,
    bits: ::core::ffi::c_uint,
    last: ::core::ffi::c_int,
) -> InflateExitProgress {
    let input_used = initial_input.wrapping_sub(remaining_input);
    let output_used = initial_output.wrapping_sub(remaining_output);
    let mode_value = mode as ::core::ffi::c_uint;
    let update_window = wsize != 0
        || output_used != 0
            && mode_value < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
            && (mode_value
                < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                || flush != crate::zlib_h::Z_FINISH);
    let data_type = bits as ::core::ffi::c_int
        + if last != 0 { 64 } else { 0 }
        + if mode_value == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint {
            128
        } else {
            0
        }
        + if mode_value == crate::src::inflate::LEN_ as ::core::ffi::c_int as ::core::ffi::c_uint
            || mode_value == crate::src::inflate::COPY_ as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            256
        } else {
            0
        };
    InflateExitProgress {
        input_used,
        output_used,
        update_window,
        data_type,
    }
}

unsafe fn updatewindow(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    end: *const crate::stdlib::Bytef,
    copy: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    // The legacy decoder already validated and adopted these records at its
    // boundary.  This helper is still unsafe because it invokes the caller
    // allocator and lends the ABI-owned window/output spans below.
    if state.window.is_null() {
        let Some(window_len) = inflate_window_len(state.wbits, 0) else {
            return 1 as ::core::ffi::c_int;
        };
        let Ok(requested_wsize) = ::core::ffi::c_uint::try_from(window_len) else {
            return 1 as ::core::ffi::c_int;
        };
        // `inflate()` normally reaches this boundary only after init has
        // installed zalloc.  Treat a malformed compatibility stream as the
        // same allocation failure that a null allocator result represents,
        // rather than panicking across the C ABI.
        let Some(zalloc) = strm.zalloc else {
            return 1 as ::core::ffi::c_int;
        };
        state.window = zalloc(
            strm.opaque,
            requested_wsize,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if state.window.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    let Some(window_len) = inflate_window_len(state.wbits, state.wsize) else {
        return 1 as ::core::ffi::c_int;
    };
    let Ok(copy_len) = usize::try_from(copy) else {
        return 1 as ::core::ffi::c_int;
    };
    let window = ::core::slice::from_raw_parts_mut(state.window, window_len);
    let produced = if copy_len == 0 {
        &[]
    } else {
        if end.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        // Preserve the translated cursor movement without making pointer
        // arithmetic itself an unsafe operation. The boundary still lends a
        // `copy_len` span only after validating the source pointer above.
        ::core::slice::from_raw_parts(end.wrapping_sub(copy_len), copy_len)
    };
    let Some(update) = inflate_window_update(
        window,
        produced,
        state.wbits,
        state.wsize,
        state.wnext,
        state.whave,
    ) else {
        return 1 as ::core::ffi::c_int;
    };
    state.wsize = update.wsize;
    state.wnext = update.wnext;
    state.whave = update.whave;
    return 0 as ::core::ffi::c_int;
}

pub unsafe fn inflate(
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
    // Keep the dynamic literal/length table size as a scalar so the distance
    // table can borrow the following owned portion of `codes` without
    // reconstructing an interior raw cursor.
    let mut table_used: usize = 0;
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
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // This is the legacy decoder's one stream/state adoption.  Keep the
    // pointer checks here, then hand the scalar relationship to the safe
    // validator shared by the smaller inflate boundaries.
    let strm_ref = &mut *strm;
    let state_ref = {
        let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
        if state.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        &mut *state
    };
    if !inflate_state_values_are_valid(
        strm_ref.zalloc.is_some(),
        strm_ref.zfree.is_some(),
        state_ref.strm == strm,
        state_ref.mode,
    ) || strm_ref.next_out.is_null()
        || strm_ref.next_in.is_null() && strm_ref.avail_in != 0 as crate::stdlib::uInt
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = state_ref as *mut crate::src::inflate::inflate_state;
    if state_ref.mode as ::core::ffi::c_uint
        == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        state_ref.mode = crate::src::inflate::TYPEDO;
    }
    put = strm_ref.next_out as *mut ::core::ffi::c_uchar;
    left = strm_ref.avail_out as ::core::ffi::c_uint;
    next = strm_ref.next_in as *mut ::core::ffi::c_uchar;
    have = strm_ref.avail_in as ::core::ffi::c_uint;
    hold = state_ref.hold;
    bits = state_ref.bits;
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
                                                                                                    // The decoder entry already validated both
                                                                                                    // compatibility records.  Keep the header
                                                                                                    // transition on one short-lived borrow instead
                                                                                                    // of repeatedly traversing their raw pointers.
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
                                                                                                    if state_ref.wrap == 0 as ::core::ffi::c_int {
                                                                                                        state_ref.mode = crate::src::inflate::TYPEDO;
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
                                                                                                                    (*c2rust_fresh0 as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        if state_ref.wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if state_ref.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                state_ref.wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            state_ref.check = inflate_header_crc_update(0, &[]);
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            state_ref.check = inflate_header_crc_update(
                                                                                                                state_ref.check,
                                                                                                                &hbuf[..2],
                                                                                                            );
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            state_ref.mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            if !state_ref.head.is_null() {
                                                                                                                (*state_ref.head).done = -1 as ::core::ffi::c_int;
                                                                                                            }
                                                                                                            if state_ref.wrap & 1 as ::core::ffi::c_int == 0
                                                                                                                || (((hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                                                                                                    .wrapping_rem(31 as ::core::ffi::c_ulong) != 0
                                                                                                            {
                                                                                                                strm_ref.msg = INFLATE_ERROR_MESSAGES[0].as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                strm_ref.msg = INFLATE_ERROR_MESSAGES[1].as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                state_ref.mode = crate::src::inflate::BAD;
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
                                                                                                                if state_ref.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                    state_ref.wbits = len;
                                                                                                                }
                                                                                                                if len > 15 as ::core::ffi::c_uint || len > state_ref.wbits {
                                                                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[2].as_ptr()
                                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                    state_ref.mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    state_ref.dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    state_ref.flags = 0 as ::core::ffi::c_int;
                                                                                                                    // zlib defines the checksum of an empty
                                                                                                                    // stream directly.  Do not route this
                                                                                                                    // through the raw-pointer ABI adapter.
                                                                                                                    state_ref.check = crate::src::adler32::ADLER32_INITIAL
                                                                                                                        as ::core::ffi::c_ulong;
                                                                                                                    strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                                                                                                                    state_ref.mode = (if hold & 0x200 as ::core::ffi::c_ulong
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
                                                                                                    // FLAGS follows HEAD without any cursor lend or
                                                                                                    // callback.  Keep its state/message commits on
                                                                                                    // the same kind of short-lived boundary borrow.
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
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
                                                                                                                (*c2rust_fresh1 as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    state_ref.flags = hold as ::core::ffi::c_int;
                                                                                                    if state_ref.flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[1].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state_ref.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if state_ref.flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[3].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state_ref.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if !state_ref.head.is_null() {
                                                                                                            (*state_ref.head).text = (hold >> 8 as ::core::ffi::c_int
                                                                                                                & 1 as ::core::ffi::c_ulong) as ::core::ffi::c_int;
                                                                                                        }
                                                                                                        if state_ref.flags & 0x200 as ::core::ffi::c_int != 0
                                                                                                            && state_ref.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                        {
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            state_ref.check = inflate_header_crc_update(
                                                                                                                state_ref.check,
                                                                                                                &hbuf[..2],
                                                                                                            );
                                                                                                        }
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        state_ref.mode = crate::src::inflate::TIME;
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
                                                                                                        next = next.wrapping_add(1);
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
                                                                                                        next = next.wrapping_add(1);
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
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        let c2rust_fresh13 = next;
                                                                                                        next = next.wrapping_add(1);
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
                                                                                                    let c2rust_fresh32 = put;
                                                                                                    put = put.wrapping_add(1);
                                                                                                    *c2rust_fresh32 = (*state).length as ::core::ffi::c_uchar;
                                                                                                    left = left.wrapping_sub(1);
                                                                                                    (*state).mode = crate::src::inflate::LEN;
                                                                                                    continue '_inf_leave;
                                                                                                }
                                                                                                16206 => {
                                                                                                    // The decoder entry already adopted both
                                                                                                    // compatibility records. Keep trailer
                                                                                                    // accounting and the checksum commit in
                                                                                                    // this transition-local borrow rather than
                                                                                                    // repeatedly traversing the raw pointers.
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
                                                                                                    if state_ref.wrap != 0 {
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
                                                                                                                    (*c2rust_fresh33 as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        out = out.wrapping_sub(left);
                                                                                                        strm_ref.total_out = strm_ref
                                                                                                            .total_out
                                                                                                            .wrapping_add(out as crate::stdlib::uLong);
                                                                                                        state_ref.total = state_ref
                                                                                                            .total
                                                                                                            .wrapping_add(out as ::core::ffi::c_ulong);
                                                                                                        if state_ref.wrap & 4 as ::core::ffi::c_int != 0 && out != 0
                                                                                                        {
                                                                                                            let output = core::slice::from_raw_parts(
                                                                                                                put.wrapping_sub(out as usize),
                                                                                                                out as usize,
                                                                                                            );
                                                                                                            state_ref.check = inflate_output_checksum(
                                                                                                                state_ref.check as crate::stdlib::uLong,
                                                                                                                state_ref.flags,
                                                                                                                output,
                                                                                                            );
                                                                                                            strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                                                                                                        }
                                                                                                        out = left;
                                                                                                        if state_ref.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                            && (if state_ref.flags != 0 {
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
                                                                                                            }) != state_ref.check
                                                                                                        {
                                                                                                            strm_ref.msg = INFLATE_ERROR_MESSAGES[6].as_ptr()
                                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                            state_ref.mode = crate::src::inflate::BAD;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                        }
                                                                                                    }
                                                                                                    state_ref.mode = crate::src::inflate::LENGTH;
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
                                                                                                    next = next.wrapping_add(1);
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
                                                                                                if have == 0 as ::core::ffi::c_uint {
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                have = have.wrapping_sub(1);
                                                                                                let c2rust_fresh14 = next;
                                                                                                next = next.wrapping_add(1);
                                                                                                hold = hold
                                                                                                    .wrapping_add(
                                                                                                        (*c2rust_fresh14 as ::core::ffi::c_ulong) << bits,
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
                                                                                        ret = {
                                                                                            let state = &mut *state;
                                                                                            state.next = &raw mut state.codes as *mut crate::src::inftrees::code;
                                                                                            state.distcode = state.next as *const crate::src::inftrees::code;
                                                                                            state.lencode = state.distcode;
                                                                                            state.lenbits = 7 as ::core::ffi::c_uint;
                                                                                            match crate::src::inftrees::inflate_table_into(
                                                                                                crate::src::inftrees::CODES,
                                                                                                &state.lens[..19],
                                                                                                &mut state.codes[..crate::src::inftrees::ENOUGH as usize],
                                                                                                &mut state.work,
                                                                                                state.lenbits,
                                                                                            ) {
                                                                                                Ok((used, root)) => {
                                                                                                    state.next = state.next.wrapping_add(used);
                                                                                                    state.lenbits = root;
                                                                                                    0
                                                                                                }
                                                                                                Err(status) => status,
                                                                                            }
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
                                                                                    // The dictionary transition only publishes
                                                                                    // already-established cursors and scalar state.
                                                                                    // Adopt both compatibility records once rather
                                                                                    // than repeatedly traversing their raw pointers.
                                                                                    let strm_ref = &mut *strm;
                                                                                    let state_ref = &mut *state;
                                                                                    if state_ref.havedict == 0 as ::core::ffi::c_int {
                                                                                        strm_ref.next_out = put as *mut crate::stdlib::Bytef;
                                                                                        strm_ref.avail_out = left as crate::stdlib::uInt;
                                                                                        strm_ref.next_in = next as *mut crate::stdlib::Bytef;
                                                                                        strm_ref.avail_in = have as crate::stdlib::uInt;
                                                                                        state_ref.hold = hold;
                                                                                        state_ref.bits = bits;
                                                                                        return crate::zlib_h::Z_NEED_DICT;
                                                                                    }
                                                                                    state_ref.check = crate::src::adler32::ADLER32_INITIAL
                                                                                        as ::core::ffi::c_ulong;
                                                                                    strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                                                                                    state_ref.mode =
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
                                                                                            (*c2rust_fresh2 as ::core::ffi::c_ulong) << bits,
                                                                                        );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                // TIME has no cursor lend or callback. Keep the
                                                                                // retained header update and checksum commit on
                                                                                // one short-lived state borrow.
                                                                                let state_ref = &mut *state;
                                                                                if !state_ref.head.is_null()
                                                                                {
                                                                                    (*state_ref
                                                                                        .head)
                                                                                        .time = hold
                                                                                        as crate::stdlib::uLong;
                                                                                }
                                                                                if state_ref.flags & 0x200 as ::core::ffi::c_int != 0
                                                                                    && state_ref.wrap & 4 as ::core::ffi::c_int != 0
                                                                                {
                                                                                    hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                        >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                    hbuf[2 as ::core::ffi::c_int as usize] = (hold
                                                                                        >> 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                    hbuf[3 as ::core::ffi::c_int as usize] = (hold
                                                                                        >> 24 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                    state_ref.check = inflate_header_crc_update(
                                                                                        state_ref.check,
                                                                                        &hbuf[..4],
                                                                                    );
                                                                                }
                                                                                hold = 0 as ::core::ffi::c_ulong;
                                                                                bits = 0 as ::core::ffi::c_uint;
                                                                                state_ref.mode = crate::src::inflate::OS;
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
                                                                                here = *(*state)
                                                                                    .lencode
                                                                                    .wrapping_add(
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
                                                                                next = next
                                                                                    .wrapping_add(
                                                                                        1,
                                                                                    );
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
                                                                                        next = next.wrapping_add(1);
                                                                                        hold = hold
                                                                                            .wrapping_add(
                                                                                                (*c2rust_fresh19 as ::core::ffi::c_ulong) << bits,
                                                                                            );
                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
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
                                                                                        have = have.wrapping_sub(1);
                                                                                        let c2rust_fresh20 = next;
                                                                                        next = next.wrapping_add(1);
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
                                                                                        next = next.wrapping_add(1);
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
                                                                        // Code-length decoding has already consumed all
                                                                        // input for this transition.  Keep the two
                                                                        // compatibility records adopted while we validate
                                                                        // the completed lens and build its bounded tables,
                                                                        // rather than re-traversing their raw cursors for
                                                                        // each error and mode commit.
                                                                        let strm_ref = &mut *strm;
                                                                        let state_ref = &mut *state;
                                                                        if state_ref.mode as ::core::ffi::c_uint
                                                                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            continue '_inf_leave;
                                                                        }
                                                                        if state_ref.lens[256 as ::core::ffi::c_int as usize]
                                                                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                                        {
                                                                            strm_ref.msg = INFLATE_ERROR_MESSAGES[10]
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            state_ref.mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            ret = {
                                                                                let nlen = state_ref.nlen as usize;
                                                                                state_ref.next = &raw mut state_ref.codes as *mut crate::src::inftrees::code;
                                                                                state_ref.lencode = state_ref.next as *const crate::src::inftrees::code;
                                                                                state_ref.lenbits = 9 as ::core::ffi::c_uint;
                                                                                match state_ref.lens.get(..nlen) {
                                                                                    Some(lens) => match crate::src::inftrees::inflate_table_into(
                                                                                        crate::src::inftrees::LENS,
                                                                                        lens,
                                                                                        &mut state_ref.codes[..crate::src::inftrees::ENOUGH_LENS as usize],
                                                                                        &mut state_ref.work,
                                                                                        state_ref.lenbits,
                                                                                    ) {
                                                                                        Ok((used, root)) => {
                                                                                            table_used = used;
                                                                                            state_ref.next = state_ref.next.wrapping_add(used);
                                                                                            state_ref.lenbits = root;
                                                                                            0
                                                                                        }
                                                                                        Err(status) => status,
                                                                                    },
                                                                                    None => 1,
                                                                                }
                                                                            };
                                                                            if ret != 0 {
                                                                                strm_ref.msg = INFLATE_ERROR_MESSAGES[11].as_ptr()
                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                ret = {
                                                                                    let nlen = state_ref.nlen as usize;
                                                                                    let ndist = state_ref.ndist as usize;
                                                                                    state_ref.distcode = state_ref.next as *const crate::src::inftrees::code;
                                                                                    state_ref.distbits = 6 as ::core::ffi::c_uint;
                                                                                    let end = match table_used.checked_add(
                                                                                        crate::src::inftrees::ENOUGH_DISTS as usize,
                                                                                    ) {
                                                                                        Some(end) => end,
                                                                                        None => 0,
                                                                                    };
                                                                                    match (
                                                                                        state_ref.lens.get(nlen..nlen.saturating_add(ndist)),
                                                                                        state_ref.codes.get_mut(table_used..end),
                                                                                    ) {
                                                                                        (Some(lens), Some(table)) => match crate::src::inftrees::inflate_table_into(
                                                                                            crate::src::inftrees::DISTS,
                                                                                            lens,
                                                                                            table,
                                                                                            &mut state_ref.work,
                                                                                            state_ref.distbits,
                                                                                        ) {
                                                                                            Ok((used, root)) => {
                                                                                                state_ref.next = state_ref.next.wrapping_add(used);
                                                                                                state_ref.distbits = root;
                                                                                                0
                                                                                            }
                                                                                            Err(status) => status,
                                                                                        },
                                                                                        _ => 1,
                                                                                    }
                                                                                };
                                                                                if ret != 0 {
                                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[12].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state_ref.mode = crate::src::inflate::BAD;
                                                                                    continue '_inf_leave;
                                                                                } else {
                                                                                    state_ref.mode = crate::src::inflate::LEN_;
                                                                                    if flush == crate::zlib_h::Z_TREES {
                                                                                        break '_inf_leave;
                                                                                    } else {
                                                                                        break 'c_2397;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    // Stored-block copying needs no callbacks or new
                                                                    // ABI views.  Keep its scalar state commits on one
                                                                    // short-lived adopted state record instead of
                                                                    // repeatedly traversing the compatibility pointer.
                                                                    let state_ref = &mut *state;
                                                                    copy = state_ref.length;
                                                                    if copy != 0 {
                                                                        copy =
                                                                            inflate_stored_copy_len(
                                                                                copy, have, left,
                                                                            );
                                                                        if copy == 0 {
                                                                            break '_inf_leave;
                                                                        }
                                                                        // The bounded stored-block
                                                                        // copy is intentionally
                                                                        // bytewise here: this legacy
                                                                        // decoder boundary cannot lend
                                                                        // new raw slices without
                                                                        // increasing implementation
                                                                        // unsafety.
                                                                        let mut copied = 0usize;
                                                                        while copied < copy as usize
                                                                        {
                                                                            *put.wrapping_add(
                                                                                copied,
                                                                            ) = *next.wrapping_add(
                                                                                copied,
                                                                            );
                                                                            copied += 1;
                                                                        }
                                                                        have =
                                                                            have.wrapping_sub(copy);
                                                                        next = next.wrapping_add(
                                                                            copy as usize,
                                                                        );
                                                                        left =
                                                                            left.wrapping_sub(copy);
                                                                        put = put.wrapping_add(
                                                                            copy as usize,
                                                                        );
                                                                        state_ref.length =
                                                                            state_ref
                                                                                .length
                                                                                .wrapping_sub(copy);
                                                                        continue '_inf_leave;
                                                                    } else {
                                                                        state_ref.mode = crate::src::inflate::TYPE;
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
                                                                    next = next.wrapping_add(1);
                                                                    hold = hold.wrapping_add(
                                                                        (*c2rust_fresh3
                                                                            as ::core::ffi::c_ulong)
                                                                            << bits,
                                                                    );
                                                                    bits = bits.wrapping_add(
                                                                        8 as ::core::ffi::c_uint,
                                                                    );
                                                                }
                                                                // OS follows TIME without any callback or cursor lend.
                                                                // Reuse one adopted state record for the retained header,
                                                                // optional header CRC, and mode transition.
                                                                let state_ref = &mut *state;
                                                                if !state_ref.head.is_null() {
                                                                    (*state_ref.head).xflags = (hold
                                                                        & 0xff
                                                                            as ::core::ffi::c_ulong)
                                                                        as ::core::ffi::c_int;
                                                                    (*state_ref.head).os = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_int;
                                                                }
                                                                if state_ref.flags
                                                                    & 0x200 as ::core::ffi::c_int
                                                                    != 0
                                                                    && state_ref.wrap
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
                                                                    state_ref.check =
                                                                        inflate_header_crc_update(
                                                                            state_ref.check,
                                                                            &hbuf[..2],
                                                                        );
                                                                }
                                                                hold = 0 as ::core::ffi::c_ulong;
                                                                bits = 0 as ::core::ffi::c_uint;
                                                                state_ref.mode =
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
                                                    // The decoder entry has already validated both
                                                    // compatibility records. Keep the block-header
                                                    // state transition on those short-lived borrows
                                                    // instead of repeatedly traversing raw pointers.
                                                    let strm_ref = &mut *strm;
                                                    let state_ref = &mut *state;
                                                    if state_ref.last != 0 {
                                                        hold >>= bits & 7 as ::core::ffi::c_uint;
                                                        bits = bits.wrapping_sub(
                                                            bits & 7 as ::core::ffi::c_uint,
                                                        );
                                                        state_ref.mode = crate::src::inflate::CHECK;
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
                                                            next = next.wrapping_add(1);
                                                            hold = hold.wrapping_add(
                                                                (*c2rust_fresh11
                                                                    as ::core::ffi::c_ulong)
                                                                    << bits,
                                                            );
                                                            bits = bits.wrapping_add(
                                                                8 as ::core::ffi::c_uint,
                                                            );
                                                        }
                                                        state_ref.last = (hold
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
                                                                state_ref.mode =
                                                                    crate::src::inflate::STORED;
                                                            }
                                                            1 => {
                                                                crate::src::inftrees::inflate_fixed_state(
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
                                                                state_ref.mode =
                                                                    crate::src::inflate::TABLE;
                                                            }
                                                            _ => {
                                                                strm_ref.msg = INFLATE_ERROR_MESSAGES
                                                                    [13]
                                                                .as_ptr()
                                                                    as *const ::core::ffi::c_char
                                                                    as *mut ::core::ffi::c_char;
                                                                state_ref.mode =
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
                                                // EXLEN only consumes the already-validated input
                                                // cursor and updates retained header/scalar state.
                                                // Keep those commits on one short-lived state borrow
                                                // instead of repeatedly traversing the compatibility
                                                // state pointer.
                                                let state_ref = &mut *state;
                                                if state_ref.flags & 0x400 as ::core::ffi::c_int
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
                                                            (*c2rust_fresh4
                                                                as ::core::ffi::c_ulong)
                                                                << bits,
                                                        );
                                                        bits = bits
                                                            .wrapping_add(8 as ::core::ffi::c_uint);
                                                    }
                                                    state_ref.length = hold as ::core::ffi::c_uint;
                                                    if !state_ref.head.is_null() {
                                                        (*state_ref.head).extra_len = hold
                                                            as ::core::ffi::c_uint
                                                            as crate::stdlib::uInt;
                                                    }
                                                    if state_ref.flags & 0x200 as ::core::ffi::c_int
                                                        != 0
                                                        && state_ref.wrap & 4 as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        hbuf[0 as ::core::ffi::c_int as usize] =
                                                            hold as ::core::ffi::c_uchar;
                                                        hbuf[1 as ::core::ffi::c_int as usize] =
                                                            (hold >> 8 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uchar;
                                                        state_ref.check = inflate_header_crc_update(
                                                            state_ref.check,
                                                            &hbuf[..2],
                                                        );
                                                    }
                                                    hold = 0 as ::core::ffi::c_ulong;
                                                    bits = 0 as ::core::ffi::c_uint;
                                                } else if !state_ref.head.is_null() {
                                                    (*state_ref.head).extra = ::core::ptr::null_mut::<
                                                        crate::stdlib::Bytef,
                                                    >(
                                                    );
                                                }
                                                state_ref.mode = crate::src::inflate::EXTRA;
                                                break 'c_2319;
                                            }
                                            (*state).mode = crate::src::inflate::LEN;
                                        }
                                        let strm_ref = &mut *strm;
                                        let state_ref = &mut *state;
                                        let lcode = inflate_fast_table(
                                            &state_ref.codes,
                                            state_ref.lencode as usize,
                                            crate::src::inftrees::inffixed_h::lenfix.as_ptr()
                                                as usize,
                                            &crate::src::inftrees::inffixed_h::lenfix,
                                        );
                                        let fast = if let Some(span) =
                                            inflate_fast_span(have, left, out, state_ref.wsize)
                                        {
                                            // `out` is the output capacity at the beginning of
                                            // this inflate call, while `put` has already advanced
                                            // over any bytes decoded by the slow path. Lend the
                                            // complete original span so the fast decoder keeps
                                            // zlib's distance accounting relative to `out`.
                                            let window = if span.window_len == 0 {
                                                Some(&[][..])
                                            } else if state_ref.window.is_null() {
                                                None
                                            } else {
                                                Some(::core::slice::from_raw_parts(
                                                    state_ref.window,
                                                    span.window_len,
                                                ))
                                            };
                                            if let Some(window) = window {
                                                let input = ::core::slice::from_raw_parts(
                                                    next,
                                                    span.input_len,
                                                );
                                                let output = ::core::slice::from_raw_parts_mut(
                                                    put.wrapping_sub(span.output_start),
                                                    span.output_len,
                                                );
                                                inflate_fast_tables(
                                                    &state_ref.codes,
                                                    state_ref.lencode as usize,
                                                    state_ref.distcode as usize,
                                                )
                                                .map(|(lcode, dcode)| {
                                                    inflate_fast_normal(
                                                        input,
                                                        output,
                                                        span.output_start,
                                                        window,
                                                        lcode,
                                                        dcode,
                                                        span.window_len,
                                                        state_ref.whave as usize,
                                                        state_ref.wnext as usize,
                                                        state_ref.sane != 0,
                                                        hold,
                                                        bits,
                                                        state_ref.lenbits,
                                                        state_ref.distbits,
                                                        out,
                                                    )
                                                })
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        };
                                        if let Some(fast) = fast {
                                            let Some(fast) = inflate_fast_commit(fast, have, left)
                                            else {
                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                break '_inf_leave;
                                            };
                                            next = next.wrapping_add(fast.input_used as usize);
                                            have = have.wrapping_sub(fast.input_used);
                                            put = put.wrapping_add(fast.output_used as usize);
                                            left = left.wrapping_sub(fast.output_used);
                                            hold = fast.hold;
                                            bits = fast.bits;
                                            if let Some(mode) = fast.mode {
                                                state_ref.mode = mode;
                                                strm_ref.msg = match fast.error {
                                                    Some(14) => {
                                                        b"invalid literal/length code\0".as_ptr()
                                                    }
                                                    Some(15) => b"invalid distance code\0".as_ptr(),
                                                    Some(17) => {
                                                        b"invalid distance too far back\0".as_ptr()
                                                    }
                                                    _ => ::core::ptr::null(),
                                                }
                                                    as *mut ::core::ffi::c_char;
                                            }
                                            if state_ref.mode as ::core::ffi::c_uint
                                                == crate::src::inflate::TYPE as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint
                                            {
                                                state_ref.back = -1 as ::core::ffi::c_int;
                                            }
                                            continue '_inf_leave;
                                        } else {
                                            // This slow-path fallback is still inside the
                                            // short-lived boundary borrow established for the
                                            // fast dispatch.  Keep scalar state updates on that
                                            // borrow instead of re-traversing the raw state
                                            // pointer.
                                            let Some(lcode) = lcode else {
                                                strm_ref.msg = INFLATE_ERROR_MESSAGES[14].as_ptr()
                                                    as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char;
                                                state_ref.mode = crate::src::inflate::BAD;
                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                break '_inf_leave;
                                            };
                                            state_ref.back = 0 as ::core::ffi::c_int;
                                            loop {
                                                let index = (hold as ::core::ffi::c_uint
                                                    & ((1 as ::core::ffi::c_uint)
                                                        << state_ref.lenbits)
                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                    as usize;
                                                let Some(code) = lcode.get(index) else {
                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[14]
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char;
                                                    state_ref.mode = crate::src::inflate::BAD;
                                                    ret = crate::zlib_h::Z_DATA_ERROR;
                                                    break '_inf_leave;
                                                };
                                                here = *code;
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
                                                    (*c2rust_fresh24 as ::core::ffi::c_ulong)
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
                                                    let index = (last.val as ::core::ffi::c_uint)
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
                                                        as usize;
                                                    let Some(code) = lcode.get(index) else {
                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[14]
                                                            .as_ptr()
                                                            as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char;
                                                        state_ref.mode = crate::src::inflate::BAD;
                                                        ret = crate::zlib_h::Z_DATA_ERROR;
                                                        break '_inf_leave;
                                                    };
                                                    here = *code;
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
                                                        (*c2rust_fresh25 as ::core::ffi::c_ulong)
                                                            << bits,
                                                    );
                                                    bits =
                                                        bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                }
                                                hold >>= last.bits as ::core::ffi::c_int;
                                                bits = bits
                                                    .wrapping_sub(last.bits as ::core::ffi::c_uint);
                                                state_ref.back += last.bits as ::core::ffi::c_int;
                                            }
                                            hold >>= here.bits as ::core::ffi::c_int;
                                            bits =
                                                bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                            state_ref.back += here.bits as ::core::ffi::c_int;
                                            state_ref.length = here.val as ::core::ffi::c_uint;
                                            if here.op as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                state_ref.mode = crate::src::inflate::LIT;
                                                continue '_inf_leave;
                                            } else if here.op as ::core::ffi::c_int
                                                & 32 as ::core::ffi::c_int
                                                != 0
                                            {
                                                state_ref.back = -1 as ::core::ffi::c_int;
                                                state_ref.mode = crate::src::inflate::TYPE;
                                                continue '_inf_leave;
                                            } else if here.op as ::core::ffi::c_int
                                                & 64 as ::core::ffi::c_int
                                                != 0
                                            {
                                                strm_ref.msg = INFLATE_ERROR_MESSAGES[14].as_ptr()
                                                    as *const ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_char;
                                                state_ref.mode = crate::src::inflate::BAD;
                                                continue '_inf_leave;
                                            } else {
                                                state_ref.extra = here.op as ::core::ffi::c_uint
                                                    & 15 as ::core::ffi::c_uint;
                                                state_ref.mode = crate::src::inflate::LENEXT;
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
                                                let header_copy = if len.wrapping_add(copy)
                                                    > (*(*state).head).extra_max
                                                {
                                                    ((*(*state).head).extra_max
                                                        as ::core::ffi::c_uint)
                                                        .wrapping_sub(len)
                                                } else {
                                                    copy
                                                };
                                                let mut copied = 0usize;
                                                while copied < header_copy as usize {
                                                    *(*(*state).head)
                                                        .extra
                                                        .wrapping_add(len as usize + copied) =
                                                        *next.wrapping_add(copied);
                                                    copied += 1;
                                                }
                                            }
                                            if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                                && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                            {
                                                let mut checked =
                                                    (*state).check as crate::stdlib::uLong;
                                                let mut checked_at = 0usize;
                                                while checked_at < copy as usize {
                                                    let byte = *next.wrapping_add(checked_at);
                                                    checked = inflate_header_crc_update(
                                                        checked as ::core::ffi::c_ulong,
                                                        &[byte],
                                                    )
                                                        as crate::stdlib::uLong;
                                                    checked_at += 1;
                                                }
                                                (*state).check = checked as ::core::ffi::c_ulong;
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
                                        let c2rust_fresh26 = next;
                                        next = next.wrapping_add(1);
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
                                    len = *next.wrapping_add(c2rust_fresh5 as usize)
                                        as ::core::ffi::c_uint;
                                    if !(*state).head.is_null()
                                        && !(*(*state).head).name.is_null()
                                        && (*state).length < (*(*state).head).name_max
                                    {
                                        let c2rust_fresh6 = (*state).length;
                                        (*state).length = (*state).length.wrapping_add(1);
                                        *(*(*state).head)
                                            .name
                                            .wrapping_add(c2rust_fresh6 as usize) =
                                            len as crate::stdlib::Bytef;
                                    }
                                    if !(len != 0 && copy < have) {
                                        break;
                                    }
                                }
                                if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                                    && (*state).wrap & 4 as ::core::ffi::c_int != 0
                                {
                                    (*state).check = inflate_header_crc_update(
                                        (*state).check,
                                        core::slice::from_raw_parts(next, copy as usize),
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
                            here = *(*state).distcode.wrapping_add(
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
                            next = next.wrapping_add(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh27 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            last = here;
                            loop {
                                here = *(*state).distcode.wrapping_add(
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
                            len = *next.wrapping_add(c2rust_fresh7 as usize) as ::core::ffi::c_uint;
                            if !(*state).head.is_null()
                                && !(*(*state).head).comment.is_null()
                                && (*state).length < (*(*state).head).comm_max
                            {
                                let c2rust_fresh8 = (*state).length;
                                (*state).length = (*state).length.wrapping_add(1);
                                *(*(*state).head)
                                    .comment
                                    .wrapping_add(c2rust_fresh8 as usize) =
                                    len as crate::stdlib::Bytef;
                            }
                            if !(len != 0 && copy < have) {
                                break;
                            }
                        }
                        if (*state).flags & 0x200 as ::core::ffi::c_int != 0
                            && (*state).wrap & 4 as ::core::ffi::c_int != 0
                        {
                            (*state).check = inflate_header_crc_update(
                                (*state).check,
                                core::slice::from_raw_parts(next, copy as usize),
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
                        let c2rust_fresh29 = next;
                        next = next.wrapping_add(1);
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
            // Header-CRC completion only consumes the existing decoder
            // cursor and updates stream/header scalars.  Keep those commits
            // on one short-lived boundary borrow instead of repeatedly
            // traversing the compatibility records.
            let strm_ref = &mut *strm;
            let state_ref = &mut *state;
            if state_ref.flags & 0x200 as ::core::ffi::c_int != 0 {
                while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        break '_inf_leave;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh9 = next;
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh9 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if state_ref.wrap & 4 as ::core::ffi::c_int != 0
                    && hold != state_ref.check & 0xffff as ::core::ffi::c_ulong
                {
                    strm_ref.msg = INFLATE_ERROR_MESSAGES[16].as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state_ref.mode = crate::src::inflate::BAD;
                    continue '_inf_leave;
                } else {
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                }
            }
            if !state_ref.head.is_null() {
                (*state_ref.head).hcrc =
                    state_ref.flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
                (*state_ref.head).done = 1 as ::core::ffi::c_int;
            }
            state_ref.check = inflate_header_crc_update(0, &[]);
            strm_ref.adler = state_ref.check as crate::stdlib::uLong;
            state_ref.mode = crate::src::inflate::TYPE;
            continue '_inf_leave;
        }
        if left == 0 as ::core::ffi::c_uint {
            break;
        }
        // This match-copy transition does not allocate or invoke callbacks,
        // so keep its scalar state bookkeeping on one short-lived adopted
        // state record. The cursor copy below remains in the transitional
        // decoder boundary.
        let state_ref = &mut *state;
        copy = out.wrapping_sub(left);
        if state_ref.offset > copy {
            copy = state_ref.offset.wrapping_sub(copy);
            if copy > state_ref.whave {
                if state_ref.sane != 0 {
                    (*strm).msg = INFLATE_ERROR_MESSAGES[17].as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state_ref.mode = crate::src::inflate::BAD;
                    continue;
                }
            }
            if copy > state_ref.wnext {
                copy = copy.wrapping_sub(state_ref.wnext);
                from = state_ref
                    .window
                    .wrapping_add(state_ref.wsize.wrapping_sub(copy) as usize);
            } else {
                from = state_ref
                    .window
                    .wrapping_add(state_ref.wnext.wrapping_sub(copy) as usize);
            }
            if copy > state_ref.length {
                copy = state_ref.length;
            }
        } else {
            from = put.wrapping_sub(state_ref.offset as usize);
            copy = state_ref.length;
        }
        if copy > left {
            copy = left;
        }
        left = left.wrapping_sub(copy);
        state_ref.length = state_ref.length.wrapping_sub(copy);
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
        if state_ref.length == 0 as ::core::ffi::c_uint {
            state_ref.mode = crate::src::inflate::LEN;
        }
    }
    // Keep the decoder loop's raw cursors local to that loop.  The exit
    // commit adopts each ABI record once, so cursor publication, history
    // planning, totals, and checksum state use ordinary field access.
    // Compute one checked exit plan before history maintenance.  Updating the
    // circular window may change history cursors, but it does not change the
    // decoded input/output progress, mode, or bit state this call publishes.
    // Reusing this plan keeps the history decision and final ABI accounting
    // tied to the same scalar snapshot.
    let (exit, window_error) = {
        let strm_ref = &mut *strm;
        let state_ref = &mut *state;
        strm_ref.next_out = put as *mut crate::stdlib::Bytef;
        strm_ref.avail_out = left as crate::stdlib::uInt;
        strm_ref.next_in = next as *mut crate::stdlib::Bytef;
        strm_ref.avail_in = have as crate::stdlib::uInt;
        state_ref.hold = hold;
        state_ref.bits = bits;
        let exit = inflate_exit_progress(
            in_0,
            strm_ref.avail_in as ::core::ffi::c_uint,
            out,
            strm_ref.avail_out as ::core::ffi::c_uint,
            state_ref.wsize,
            state_ref.mode,
            flush,
            state_ref.bits,
            state_ref.last,
        );
        let window_error = if exit.update_window {
            updatewindow(
                strm_ref,
                state_ref,
                strm_ref.next_out as *const crate::stdlib::Bytef,
                exit.output_used,
            ) != 0
        } else {
            false
        };
        (exit, window_error)
    };
    if window_error {
        let state_ref = &mut *state;
        state_ref.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    {
        let strm_ref = &mut *strm;
        let state_ref = &mut *state;
        in_0 = exit.input_used;
        out = exit.output_used;
        strm_ref.total_in = strm_ref
            .total_in
            .wrapping_add(exit.input_used as crate::stdlib::uLong);
        strm_ref.total_out = strm_ref
            .total_out
            .wrapping_add(exit.output_used as crate::stdlib::uLong);
        state_ref.total = state_ref
            .total
            .wrapping_add(exit.output_used as ::core::ffi::c_ulong);
        if inflate_exit_needs_checksum(state_ref.wrap, exit.output_used as usize) {
            let output = core::slice::from_raw_parts(
                strm_ref.next_out.wrapping_sub(exit.output_used as usize),
                exit.output_used as usize,
            );
            if let Some(check) = inflate_exit_checksum(
                state_ref.wrap,
                state_ref.check as crate::stdlib::uLong,
                state_ref.flags,
                output,
            ) {
                state_ref.check = check;
                strm_ref.adler = state_ref.check as crate::stdlib::uLong;
            }
        }
        strm_ref.data_type = exit.data_type;
    }
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
// This expands only in export-attributed ABI functions (including the
// boundary macros used by gzip and one-shot decompression).  Destruction
// invokes caller-provided allocation callbacks, so it must remain at that
// boundary until inflate state owns its allocation safely.
macro_rules! inflate_end_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let state = (*strm).state as *mut crate::src::inflate::inflate_state;
            if !(*state).window.is_null() {
                Some((*strm).zfree.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    (*strm).opaque,
                    (*state).window as crate::stdlib::voidpf,
                );
            }
            Some((*strm).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*strm).opaque,
                (*strm).state as crate::stdlib::voidpf,
            );
            (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
            crate::zlib_h::Z_OK
        }
    }};
}
pub(crate) use inflate_end_at_boundary;
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    inflate_end_at_boundary!(strm)
}

/// Copy the logical inflate dictionary out of its circular history buffer.
///
/// The ABI wrapper validates and lends both buffers for this call.  Keeping
/// the wraparound arithmetic here makes the two copies bounds-checked slice
/// operations instead of pointer offsets at the boundary.
fn inflate_dictionary_copy(
    window: &[crate::stdlib::Byte],
    wnext: usize,
    whave: usize,
    dictionary: &mut [crate::stdlib::Byte],
) -> Option<()> {
    if whave > window.len() || wnext > whave || dictionary.len() != whave {
        return None;
    }
    let (tail, head) = dictionary.split_at_mut(whave.checked_sub(wnext)?);
    tail.copy_from_slice(window.get(wnext..whave)?);
    head.copy_from_slice(window.get(..wnext)?);
    Some(())
}

/// Check the non-overlap precondition before lending an ABI history window
/// and caller dictionary as Rust slices.  zlib's original copies have
/// `memcpy` semantics, so an overlapping caller destination was never a
/// supported operation; rejecting it here prevents simultaneous aliased
/// shared and mutable slice views at the boundary.
fn inflate_spans_are_disjoint(
    left_address: usize,
    left_len: usize,
    right_address: usize,
    right_len: usize,
) -> bool {
    if left_len == 0 || right_len == 0 {
        return true;
    }
    let Some(left_end) = left_address.checked_add(left_len) else {
        return false;
    };
    let Some(right_end) = right_address.checked_add(right_len) else {
        return false;
    };
    left_end <= right_address || right_end <= left_address
}

#[export_name = "inflateGetDictionary"]
pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    if state.whave != 0 && !dictionary.is_null() {
        let Ok(window_len) = usize::try_from(state.wsize) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Ok(wnext) = usize::try_from(state.wnext) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Ok(whave) = usize::try_from(state.whave) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if state.window.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if !inflate_spans_are_disjoint(
            state.window as usize,
            window_len,
            dictionary as usize,
            whave,
        ) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let window = ::core::slice::from_raw_parts(state.window, window_len);
        let output = ::core::slice::from_raw_parts_mut(dictionary, whave);
        if inflate_dictionary_copy(window, wnext, whave, output).is_none() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
    }
    if !dictLength.is_null() {
        *dictLength = state.whave as crate::stdlib::uInt;
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSetDictionary"]
pub unsafe extern "C" fn inflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut dictid: ::core::ffi::c_ulong = 0;
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = if dictLength == 0 {
        &[][..]
    } else if dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    } else {
        core::slice::from_raw_parts(dictionary, dictLength as usize)
    };
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
        dictid = crate::src::adler32::ADLER32_INITIAL as ::core::ffi::c_ulong;
        dictid = crate::src::adler32::adler32_z(dictid as crate::stdlib::uLong, dictionary)
            as ::core::ffi::c_ulong;
        if dictid != (*state).check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }

    // This exported boundary owns the allocator callback and the raw window
    // lend.  The circular-buffer planning and copying stay in the checked
    // slice core, so dictionary setup no longer enters the private raw
    // `updatewindow` codec adapter.
    if (*state).window.is_null() {
        (*state).window = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            (1 as crate::stdlib::uInt) << (*state).wbits,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if (*state).window.is_null() {
            (*state).mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    if (*state).wsize == 0 {
        (*state).wsize = (1 as ::core::ffi::c_uint) << (*state).wbits;
        (*state).wnext = 0;
        (*state).whave = 0;
    }
    let Some(plan) =
        inflate_window_copy_plan((*state).wsize, (*state).wnext, (*state).whave, dictLength)
    else {
        (*state).mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let Some((next, have)) = plan.cursor_values() else {
        (*state).mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let Ok(window_len) = usize::try_from((*state).wsize) else {
        (*state).mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    };
    let window = core::slice::from_raw_parts_mut((*state).window, window_len);
    if inflate_window_copy(window, dictionary, plan).is_none() {
        (*state).mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*state).wnext = next;
    (*state).whave = have;
    (*state).havedict = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateGetHeader"]
pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    if state.wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if head.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.head = head;
    (*head).done = 0 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
/// Search a safe compressed-input view for zlib's sync marker.  The export
/// boundary owns conversion of the ABI cursor into this call-scoped slice.
fn syncsearch(have: &mut ::core::ffi::c_uint, buf: &[u8]) -> ::core::ffi::c_uint {
    let mut got = *have;
    let mut next = 0;
    for &byte in buf {
        if got == 4 {
            break;
        }
        if byte == if got < 2 { 0 } else { 0xff } {
            got = got.wrapping_add(1);
        } else if byte != 0 {
            got = 0;
        } else {
            got = 4u32.wrapping_sub(got);
        }
        next += 1;
    }
    *have = got;
    next
}

/// Discard the partial byte in the bit accumulator and expose the remaining
/// whole bytes for the sync-marker search.  The legacy state stores at most
/// 32 bits, so a larger count is an incoherent internal state rather than a
/// reason to index past this fixed scratch buffer.
fn inflate_sync_aligned_bytes(
    mut hold: ::core::ffi::c_ulong,
    mut bits: ::core::ffi::c_uint,
) -> Option<(::core::ffi::c_ulong, ::core::ffi::c_uint, [u8; 4], usize)> {
    hold >>= bits & 7;
    bits = bits.wrapping_sub(bits & 7);
    let mut bytes = [0; 4];
    let mut len = 0;
    while bits >= 8 {
        let slot = bytes.get_mut(len)?;
        *slot = hold as u8;
        hold >>= 8;
        bits = bits.wrapping_sub(8);
        len += 1;
    }
    Some((hold, bits, bytes, len))
}

/// The pointer-free portion of an `inflateSync()` attempt.  The ABI wrapper
/// lends its current input span and commits the returned scalars only after
/// this has bounded the marker scan.
struct InflateSyncPlan {
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    consumed: ::core::ffi::c_uint,
}

/// Align a partially-read deflate stream, if necessary, then search the
/// supplied compressed bytes for zlib's four-byte synchronization marker.
/// `was_sync` reflects the incoming mode; callers still publish `SYNC` at
/// their ABI boundary before invoking this core, matching zlib's error-path
/// state transition when the bit accumulator is malformed.
fn inflate_sync_plan(
    input: &[u8],
    was_sync: bool,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
) -> Result<InflateSyncPlan, ::core::ffi::c_int> {
    if input.is_empty() && bits < 8 {
        return Err(crate::zlib_h::Z_BUF_ERROR);
    }

    let (hold, bits, mut have) = if was_sync {
        (hold, bits, have)
    } else {
        let Some((hold, bits, aligned, aligned_len)) = inflate_sync_aligned_bytes(hold, bits)
        else {
            return Err(crate::zlib_h::Z_STREAM_ERROR);
        };
        let mut have = 0;
        syncsearch(&mut have, &aligned[..aligned_len]);
        (hold, bits, have)
    };
    let consumed = syncsearch(&mut have, input);
    Ok(InflateSyncPlan {
        hold,
        bits,
        have,
        consumed,
    })
}

fn inflate_sync_point(
    mode: crate::src::inflate::inflate_mode,
    bits: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    (mode == crate::src::inflate::STORED && bits == 0) as ::core::ffi::c_int
}

fn inflate_validate_wrap(
    wrap: ::core::ffi::c_int,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if check != 0 && wrap != 0 {
        wrap | 4
    } else {
        wrap & !4
    }
}

fn inflate_mark(
    back: ::core::ffi::c_int,
    mode: crate::src::inflate::inflate_mode,
    length: ::core::ffi::c_uint,
    was: ::core::ffi::c_uint,
) -> ::core::ffi::c_long {
    (((back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16) as ::core::ffi::c_long)
        + if mode == crate::src::inflate::COPY_1 {
            length as ::core::ffi::c_long
        } else if mode == crate::src::inflate::MATCH {
            was.wrapping_sub(length) as ::core::ffi::c_long
        } else {
            0
        }
}

/// Convert the dynamic-code cursor into the public `inflateCodesUsed()`
/// count.  The ABI wrapper supplies addresses after validating the stream;
/// keeping the subtraction here scalar avoids deriving a raw-pointer offset
/// from a possibly incoherent internal cursor.
fn inflate_codes_used(
    codes_start: usize,
    codes_len: usize,
    next: usize,
) -> Option<::core::ffi::c_ulong> {
    let code_size = ::core::mem::size_of::<crate::src::inftrees::code>();
    let bytes = codes_len.checked_mul(code_size)?;
    let codes_end = codes_start.checked_add(bytes)?;
    if next < codes_start || next > codes_end {
        return None;
    }
    let offset = next.checked_sub(codes_start)?;
    if offset % code_size != 0 {
        return None;
    }
    ::core::ffi::c_ulong::try_from(offset / code_size).ok()
}

#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let (flags, in_0, out) = {
        let strm_ref = &mut *strm;
        let state = &mut *(strm_ref.state as *mut crate::src::inflate::inflate_state);
        let input_len = strm_ref.avail_in as usize;
        let input = if input_len == 0 {
            &[]
        } else if strm_ref.next_in.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        } else {
            ::core::slice::from_raw_parts(strm_ref.next_in as *const u8, input_len)
        };
        // zlib reports this no-progress condition before entering SYNC.  Do
        // it at the boundary before publishing the mode transition below.
        if input.is_empty() && state.bits < 8 {
            return crate::zlib_h::Z_BUF_ERROR;
        }
        let was_sync = state.mode == crate::src::inflate::SYNC;
        if !was_sync {
            // Preserve zlib's transition even if `inflate_sync_plan()` finds
            // an incoherent bit count and reports a stream error.
            state.mode = crate::src::inflate::SYNC;
        }
        let plan = match inflate_sync_plan(input, was_sync, state.hold, state.bits, state.have) {
            Ok(plan) => plan,
            Err(status) => return status,
        };
        state.hold = plan.hold;
        state.bits = plan.bits;
        state.have = plan.have;
        strm_ref.avail_in = strm_ref.avail_in.wrapping_sub(plan.consumed);
        // `consumed` was bounded by the call-scoped input slice above.
        strm_ref.next_in = strm_ref.next_in.wrapping_add(plan.consumed as usize);
        strm_ref.total_in = strm_ref
            .total_in
            .wrapping_add(plan.consumed as crate::stdlib::uLong);
        if state.have != 4 {
            return crate::zlib_h::Z_DATA_ERROR;
        }
        if state.flags == -1 {
            state.wrap = 0;
        } else {
            state.wrap &= !4;
        }
        (
            state.flags,
            strm_ref.total_in as ::core::ffi::c_ulong,
            strm_ref.total_out as ::core::ffi::c_ulong,
        )
    };
    inflate_reset_at_boundary!(strm);
    let strm_ref = &mut *strm;
    let state = &mut *(strm_ref.state as *mut crate::src::inflate::inflate_state);
    strm_ref.total_in = in_0 as crate::stdlib::uLong;
    strm_ref.total_out = out as crate::stdlib::uLong;
    state.flags = flags;
    state.mode = crate::src::inflate::TYPE;
    crate::zlib_h::Z_OK
}
#[export_name = "inflateSyncPoint"]
pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    inflate_sync_point(state.mode, state.bits)
}
#[export_name = "inflateCopy"]
pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if crate::src::inflate::inflate_state_check_at_boundary!(source) != 0 || dest.is_null() {
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
    window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !(*state).window.is_null() {
        if (*state).whave > (*state).wsize {
            Some((*source).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*source).opaque,
                copy as crate::stdlib::voidpf,
            );
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        window = Some((*source).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*source).opaque,
            (1 as crate::stdlib::uInt) << (*state).wbits,
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
    // Both ABI records are `Copy`; assigning them avoids treating their
    // typed layouts as unstructured C byte buffers at this boundary.
    *dest = *source;
    *copy = *state;
    (*copy).strm = dest;
    let source_codes = &raw mut (*state).codes as *mut crate::src::inftrees::code;
    let copy_codes = &raw mut (*copy).codes as *mut crate::src::inftrees::code;
    let cursors = inflate_copy_code_cursors(
        source_codes as usize,
        crate::src::inftrees::ENOUGH as usize,
        (*state).lencode as usize,
        (*state).distcode as usize,
        (*state).next as usize,
    );
    if let (Some(lencode), Some(distcode)) = (cursors.lencode, cursors.distcode) {
        (*copy).lencode = copy_codes.wrapping_add(lencode);
        (*copy).distcode = copy_codes.wrapping_add(distcode);
    }
    (*copy).next = copy_codes.wrapping_add(cursors.next);
    if !window.is_null() {
        let length = (*state).whave as usize;
        let source_window = ::core::slice::from_raw_parts((*state).window, length);
        let copied_window = ::core::slice::from_raw_parts_mut(window, length);
        copied_window.copy_from_slice(source_window);
    }
    (*copy).window = window;
    (*dest).state = copy as *mut crate::src::deflate::internal_state;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateUndermine"]
pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    state.sane = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_DATA_ERROR;
}
#[export_name = "inflateValidate"]
pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    state.wrap = inflate_validate_wrap(state.wrap, check);
    crate::zlib_h::Z_OK
}
#[export_name = "inflateMark"]
pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    inflate_mark(state.back, state.mode, state.length, state.was)
}
#[export_name = "inflateCodesUsed"]
pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    let codes_start = state.codes.as_mut_ptr() as usize;
    inflate_codes_used(
        codes_start,
        crate::src::inftrees::ENOUGH as usize,
        state.next as usize,
    )
    .unwrap_or(-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
}
