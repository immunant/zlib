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

// The one-shot APIs keep the temporary ABI stream entirely inside this
// adapter. The caller owns only bounded slice borrows and the uInt-sized
// chunk accounting, so it never retains an ABI cursor between codec calls.
pub(crate) struct InflateOneShotOwner<'input, 'output> {
    input: &'input [crate::stdlib::Bytef],
    output: &'output mut [crate::stdlib::Bytef],
    input_remaining: crate::stdlib::z_size_t,
    output_remaining: crate::stdlib::z_size_t,
}

pub(crate) struct InflateOneShotProgress {
    pub(crate) status: ::core::ffi::c_int,
    pub(crate) source_remaining: crate::stdlib::z_size_t,
    pub(crate) output_remaining: crate::stdlib::z_size_t,
}

impl<'input, 'output> InflateOneShotOwner<'input, 'output> {
    pub(crate) fn new(
        input: &'input [crate::stdlib::Bytef],
        output: &'output mut [crate::stdlib::Bytef],
    ) -> Self {
        let output_remaining = output.len();
        Self {
            input,
            output,
            input_remaining: input.len(),
            output_remaining,
        }
    }

    fn next_input_chunk(&mut self, max: crate::stdlib::uInt) -> crate::stdlib::uInt {
        let chunk = self.input_remaining.min(max as crate::stdlib::z_size_t) as crate::stdlib::uInt;
        self.input_remaining = self
            .input_remaining
            .wrapping_sub(chunk as crate::stdlib::z_size_t);
        chunk
    }

    fn next_output_chunk(&mut self, max: crate::stdlib::uInt) -> crate::stdlib::uInt {
        let chunk =
            self.output_remaining.min(max as crate::stdlib::z_size_t) as crate::stdlib::uInt;
        self.output_remaining = self
            .output_remaining
            .wrapping_sub(chunk as crate::stdlib::z_size_t);
        chunk
    }
}

// This is the local ABI adapter for uncompress*. Initialization failure
// returns before cleanup, while every initialized stream is ended after its
// final inflate call. The slice-backed owner keeps policy and accounting safe.
pub(crate) fn inflate_one_shot(
    owner: &mut InflateOneShotOwner<'_, '_>,
) -> Result<InflateOneShotProgress, ::core::ffi::c_int> {
    let mut stream = crate::zlib_h::z_stream {
        next_in: owner.input.as_ptr().cast_mut(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut(),
        state: None,
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let max = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut status = unsafe {
        inflateInit2_(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
            crate::zutil_h::DEF_WBITS,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        )
    };
    if status != crate::zlib_h::Z_OK {
        return Err(status);
    }
    stream.next_out = if owner.output.is_empty() {
        &raw mut stream.reserved as *mut crate::stdlib::Bytef
    } else {
        owner.output.as_mut_ptr()
    };
    loop {
        if stream.avail_out == 0 {
            stream.avail_out = owner.next_output_chunk(max);
        }
        if stream.avail_in == 0 {
            stream.avail_in = owner.next_input_chunk(max);
        }
        status = unsafe { inflate(&mut stream, crate::zlib_h::Z_NO_FLUSH) };
        if status != crate::zlib_h::Z_OK {
            break;
        }
    }
    let progress = InflateOneShotProgress {
        status,
        source_remaining: owner
            .input_remaining
            .wrapping_add(stream.avail_in as crate::stdlib::z_size_t),
        output_remaining: owner
            .output_remaining
            .wrapping_add(stream.avail_out as crate::stdlib::z_size_t),
    };
    unsafe {
        inflateEnd(&mut stream);
    }
    Ok(progress)
}

#[repr(C)]
pub struct inflate_state {
    // Keep the stream association check without retaining a raw backlink in
    // the codec state.  This is an identity token only; stream access is
    // always supplied by the caller.
    pub stream_identity: usize,
    // These two handles are ABI-boundary state.  Normal inflate never needs
    // to carry them through its decoder core: `head` is projected for one
    // call and `window` belongs exclusively to inflateBack().
    pub head: Option<::core::ptr::NonNull<crate::zlib_h::gz_header_s>>,
    pub window: Option<::core::ptr::NonNull<::core::ffi::c_uchar>>,
    pub normal: InflateNormalState,
}

// All resumable normal-inflate data is pointer-free.  Keeping it separate
// from the two ABI registrations above lets normal decoder requests use a
// safe state reference without smuggling a persistent foreign handle through
// their signature.
#[repr(C)]
pub struct InflateNormalState {
    pub mode: crate::src::inflate::inflate_mode,
    pub last: ::core::ffi::c_int,
    pub wrap: ::core::ffi::c_int,
    pub havedict: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub dmax: ::core::ffi::c_uint,
    pub check: ::core::ffi::c_ulong,
    pub total: ::core::ffi::c_ulong,
    pub wbits: ::core::ffi::c_uint,
    pub wsize: ::core::ffi::c_uint,
    pub whave: ::core::ffi::c_uint,
    pub wnext: ::core::ffi::c_uint,
    // Normal inflate history is Rust-owned. This is intentionally separate
    // from the foreign back-mode window above.
    pub owned_window: Option<Box<[u8]>>,
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

// Keep the state projection tied to the exclusive stream borrow.  Callers
// first validate the raw ABI pointer with `as_mut()` and then use this helper
// for the association check, so no projected state reference can outlive the
// stream it belongs to.
pub(crate) unsafe fn inflate_stream_and_state<'stream>(
    stream: &'stream mut crate::zlib_h::z_stream_s,
) -> Option<(
    &'stream mut crate::zlib_h::z_stream_s,
    &'stream mut crate::src::inflate::inflate_state,
)> {
    if stream.zalloc.is_none() || stream.zfree.is_none() {
        return None;
    }
    let identity = ::core::ptr::from_mut(stream).addr();
    let state = stream
        .state?
        .cast::<crate::src::inflate::inflate_state>()
        .as_mut();
    if state.stream_identity != identity
        || (state.normal.mode as ::core::ffi::c_uint)
            < crate::src::inflate::HEAD as ::core::ffi::c_int as ::core::ffi::c_uint
        || state.normal.mode as ::core::ffi::c_uint
            > crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return None;
    }
    Some((stream, state))
}

struct InflateResetUpdate {
    adler: Option<crate::stdlib::uLong>,
}

// Resetting the resumable decoder state is independent of the ABI stream.
// Keep the stream publication at the state projection boundary, while the
// scalar reset itself remains usable by all reset variants.
fn inflate_reset_keep_core(normal: &mut InflateNormalState) -> InflateResetUpdate {
    normal.total = 0;
    let adler = (normal.wrap != 0).then_some((normal.wrap & 1) as crate::stdlib::uLong);
    normal.mode = crate::src::inflate::HEAD;
    normal.last = 0;
    normal.havedict = 0;
    normal.flags = -1;
    normal.dmax = 32768;
    normal.hold = 0;
    normal.bits = 0;
    normal.next = 0;
    normal.distcode = crate::src::inflate::CodeTableRef::Dynamic(0);
    normal.lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
    normal.sane = 1;
    normal.back = -1;
    InflateResetUpdate { adler }
}

fn inflate_reset_core(normal: &mut InflateNormalState) -> InflateResetUpdate {
    normal.wsize = 0;
    normal.whave = 0;
    normal.wnext = 0;
    inflate_reset_keep_core(normal)
}

pub unsafe fn inflateResetKeep(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let update = inflate_reset_keep_core(&mut state.normal);
    strm.total_out = 0;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strm.data_type = 0;
    if let Some(adler) = update.adler {
        strm.adler = adler;
    }
    state.head = None;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateResetKeep(strm)
}
pub unsafe fn inflateReset(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    let Some((strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let update = inflate_reset_core(&mut state.normal);
    strm.total_out = 0;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut();
    strm.data_type = 0;
    if let Some(adler) = update.adler {
        strm.adler = adler;
    }
    state.head = None;
    crate::zlib_h::Z_OK
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
pub unsafe fn inflateReset2(
    strm: &mut crate::zlib_h::z_stream_s,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wrap: ::core::ffi::c_int = 0;
    let Some((strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
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
    if state.normal.owned_window.is_some()
        && state.normal.wbits != windowBits as ::core::ffi::c_uint
    {
        state.normal.owned_window = None;
    }
    state.normal.wrap = wrap;
    state.normal.wbits = windowBits as ::core::ffi::c_uint;
    // This variant already owns the validated stream/state projection.  Do
    // not re-enter `inflateReset()` merely to repeat that projection: apply
    // the same pointer-free reset core and publish its stream scalars while
    // both borrows are still in scope.
    let update = inflate_reset_core(&mut state.normal);
    strm.total_out = 0;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut();
    strm.data_type = 0;
    if let Some(adler) = update.adler {
        strm.adler = adler;
    }
    state.head = None;
    crate::zlib_h::Z_OK
}
#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateReset2(strm, windowBits)
}
pub unsafe extern "C" fn inflateInit2_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    // Keep the caller's stream projection at the allocator boundary. The
    // callback-owned state is published only after it has been fully
    // initialized below, since zalloc() need not return initialized bytes.
    let strm = &mut *strm;
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
    let state = Some(strm.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        strm.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // Publish one complete value into the callback-owned allocation.  The
    // reset below then applies the requested wrapper/window policy.  Writing
    // fields piecemeal here would briefly treat uninitialized callback bytes
    // as Rust fields with drop glue.
    ::core::ptr::write(
        state,
        crate::src::inflate::inflate_state {
            stream_identity: ::core::ptr::from_mut(strm).addr(),
            head: None,
            window: None,
            normal: InflateNormalState {
                mode: crate::src::inflate::HEAD,
                last: 0,
                wrap: 0,
                havedict: 0,
                flags: 0,
                dmax: 0,
                check: 0,
                total: 0,
                wbits: 0,
                wsize: 0,
                whave: 0,
                wnext: 0,
                owned_window: None,
                hold: 0,
                bits: 0,
                length: 0,
                offset: 0,
                extra: 0,
                lencode: crate::src::inflate::CodeTableRef::Dynamic(0),
                distcode: crate::src::inflate::CodeTableRef::Dynamic(0),
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
                sane: 1,
                back: -1,
                was: 0,
            },
        },
    );
    strm.state = Some(
        ::core::ptr::NonNull::new(state)
            .expect("checked state allocation")
            .cast(),
    );
    let ret = inflateReset2(strm, windowBits);
    if ret != crate::zlib_h::Z_OK {
        Some(strm.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            strm.opaque,
            state as crate::stdlib::voidpf,
        );
        strm.state = None;
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
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit2_(strm, crate::zutil_h::DEF_WBITS, version, stream_size)
}
// Bit priming is normal-decoder state only.  Keep its validation and update
// independent of the ABI stream so the stream/state adapter retains the one
// unsafe projection needed to reach the opaque state.
fn inflate_prime_bits(
    normal: &mut InflateNormalState,
    bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if bits == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_OK;
    }
    if bits < 0 as ::core::ffi::c_int {
        normal.hold = 0 as ::core::ffi::c_ulong;
        normal.bits = 0 as ::core::ffi::c_uint;
        return crate::zlib_h::Z_OK;
    }
    if bits > 16 as ::core::ffi::c_int
        || (normal.bits as crate::stdlib::uInt).wrapping_add(bits as crate::stdlib::uInt)
            > 32 as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    value = (value as ::core::ffi::c_long
        & ((1 as ::core::ffi::c_long) << bits) - 1 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
    normal.hold = normal
        .hold
        .wrapping_add((value as ::core::ffi::c_ulong) << normal.bits);
    normal.bits = normal
        .bits
        .wrapping_add(bits as crate::stdlib::uInt as ::core::ffi::c_uint);
    crate::zlib_h::Z_OK
}

// A normal-inflate state borrow is pointer-free once the stream-bound opaque
// state projection has completed.  Keep scalar controls on this owner, so
// their cores cannot accidentally regain access to ABI stream fields.
struct InflateNormalStateOwner<'state> {
    normal: &'state mut InflateNormalState,
}

impl<'state> InflateNormalStateOwner<'state> {
    fn new(normal: &'state mut InflateNormalState) -> Self {
        Self { normal }
    }
}

// Bit priming is a pure normal-state operation.  Its signature intentionally
// contains no ABI stream or opaque-state handle.
fn inflatePrime(
    owner: &mut InflateNormalStateOwner<'_>,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_prime_bits(owner.normal, bits, value)
}

// The export wrapper owns stream validation/conversion; this adapter retains
// the stream-lifetime-bound opaque-state projection and then produces the
// pointer-free owner consumed by the scalar core above.
unsafe fn inflate_prime_stream(
    strm: &mut crate::zlib_h::z_stream_s,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let mut owner = InflateNormalStateOwner::new(&mut state.normal);
    inflatePrime(&mut owner, bits, value)
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
    inflate_prime_stream(strm, bits, value)
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

fn allocate_inflate_window(size: usize) -> Option<Box<[u8]>> {
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).ok()?;
    bytes.resize(size, 0);
    Some(bytes.into_boxed_slice())
}

fn copy_history_dictionary(output: &mut [u8], window: &[u8], wnext: usize, whave: usize) {
    let first = whave - wnext;
    output[..first].copy_from_slice(&window[wnext..wnext + first]);
    output[first..whave].copy_from_slice(&window[..wnext]);
}

// A read-only, pointer-free view of the history state used by
// inflateGetDictionary().  Keeping the history borrow and its ring metadata
// together lets the ABI wrapper turn its optional output pointer into a
// bounded slice before it dispatches to the copy core.
struct InflateDictionaryRequest<'window> {
    window: &'window [crate::stdlib::Bytef],
    wnext: usize,
    whave: usize,
}

impl InflateDictionaryRequest<'_> {
    fn dictionary_len(&self) -> usize {
        self.whave
    }
}

// The ABI stream carries the opaque state pointer.  Project it once and keep
// all of the resulting data in the pointer-free request consumed below.
unsafe fn inflateGetDictionary<'stream>(
    stream: &'stream mut crate::zlib_h::z_stream_s,
) -> Option<InflateDictionaryRequest<'stream>> {
    let (_, state) = inflate_stream_and_state(stream)?;
    let whave = state.normal.whave as usize;
    let window = if whave == 0 {
        &[]
    } else {
        state
            .normal
            .owned_window
            .as_deref()
            .expect("normal inflate owns its history window")
    };
    Some(InflateDictionaryRequest {
        window,
        wnext: state.normal.wnext as usize,
        whave,
    })
}

fn inflate_get_dictionary(
    request: InflateDictionaryRequest<'_>,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    if let Some(output) = dictionary {
        copy_history_dictionary(output, request.window, request.wnext, request.whave);
    }
    if let Some(dict_length) = dict_length {
        *dict_length = request.whave as crate::stdlib::uInt;
    }
    crate::zlib_h::Z_OK
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

// Updating normal-inflate history is independent of the ABI stream cursor.
// Keep allocation, ring initialization, and copying in this slice-based core
// so callers that already own a bounded input view do not need to re-enter the
// raw cursor adapter.
fn update_window_from_slice(
    owned_window: &mut Option<Box<[u8]>>,
    wbits: ::core::ffi::c_uint,
    wsize: &mut ::core::ffi::c_uint,
    wnext: &mut ::core::ffi::c_uint,
    whave: &mut ::core::ffi::c_uint,
    input: &[u8],
) -> ::core::ffi::c_int {
    if owned_window.is_none() {
        *owned_window = allocate_inflate_window((1usize) << wbits);
        if owned_window.is_none() {
            return 1;
        }
    }
    if *wsize == 0 {
        *wsize = 1u32 << wbits;
        *wnext = 0;
        *whave = 0;
    }
    let window = owned_window
        .as_deref_mut()
        .expect("normal inflate owns its history window");
    copy_history_window(window, input, wnext, whave);
    0
}

// The normal decoder keeps a call's cursor arithmetic in this bounded owner
// before it enters the fast path.  It deliberately contains neither the ABI
// stream nor the opaque inflate state: the projection adapter supplies the
// short-lived history/table view, and only this pointer-free completion is
// used to update the surrounding decoder cursors.
pub(crate) struct InflateNormalStreamOwner<'input, 'output> {
    input: &'input [u8],
    output: &'output mut [u8],
    output_pos: usize,
    state: crate::src::inffast::InflateFastState<'input>,
}

impl<'input, 'output> InflateNormalStreamOwner<'input, 'output> {
    pub(crate) fn new(
        input: &'input [u8],
        output: &'output mut [u8],
        output_pos: usize,
        state: crate::src::inffast::InflateFastState<'input>,
    ) -> Option<Self> {
        output.get(output_pos..)?;
        Some(Self {
            input,
            output,
            output_pos,
            state,
        })
    }

    pub(crate) fn run_fast(self) -> crate::src::inffast::InflateFastStreamUpdate {
        let request = crate::src::inffast::InflateFastRequest::new(
            self.input,
            self.output,
            self.output_pos,
            self.state,
        )
        .expect("normal inflate checked its fast output cursor");
        crate::src::inffast::inflate_fast(request).into_stream_update()
    }
}

// Header registration is persistent ABI state, but each inflate call needs
// only bounded output buffers and a small set of scalar updates.  Keep those
// decoder-facing values pointer-free.  The registration itself remains a
// provenance-carrying `NonNull` in `inflate_state` and is projected only at
// the call boundary below.
#[derive(Default)]
struct HeaderPublication {
    text: Option<::core::ffi::c_int>,
    time: Option<crate::stdlib::uLong>,
    xflags: Option<::core::ffi::c_int>,
    os: Option<::core::ffi::c_int>,
    extra_len: Option<crate::stdlib::uInt>,
    hcrc: Option<::core::ffi::c_int>,
    done: Option<::core::ffi::c_int>,
    clear_extra: bool,
    clear_name: bool,
    clear_comment: bool,
}

struct InflateHeaderOutput<'scope> {
    extra: Option<&'scope mut [u8]>,
    name: Option<&'scope mut [u8]>,
    comment: Option<&'scope mut [u8]>,
    extra_len: ::core::ffi::c_uint,
    publication: HeaderPublication,
}

impl InflateHeaderOutput<'_> {
    fn set_text(&mut self, text: ::core::ffi::c_int) {
        self.publication.text = Some(text);
    }

    fn set_time(&mut self, time: crate::stdlib::uLong) {
        self.publication.time = Some(time);
    }

    fn set_xflags_and_os(&mut self, xflags: ::core::ffi::c_int, os: ::core::ffi::c_int) {
        self.publication.xflags = Some(xflags);
        self.publication.os = Some(os);
    }

    fn set_extra_len(&mut self, extra_len: crate::stdlib::uInt) {
        self.extra_len = extra_len as ::core::ffi::c_uint;
        self.publication.extra_len = Some(extra_len);
    }

    fn set_hcrc_and_done(&mut self, hcrc: ::core::ffi::c_int) {
        self.publication.hcrc = Some(hcrc);
        self.publication.done = Some(1);
    }

    fn set_done(&mut self, done: ::core::ffi::c_int) {
        self.publication.done = Some(done);
    }

    fn clear_extra(&mut self) {
        self.publication.clear_extra = true;
    }

    fn clear_name(&mut self) {
        self.publication.clear_name = true;
    }

    fn clear_comment(&mut self) {
        self.publication.clear_comment = true;
    }

    fn copy_extra(&mut self, remaining: ::core::ffi::c_uint, input: &[u8]) {
        let Some(extra) = self.extra.as_deref_mut() else {
            return;
        };
        let start = self.extra_len.wrapping_sub(remaining) as usize;
        let Some(destination) = extra.get_mut(start..) else {
            return;
        };
        let copy = destination.len().min(input.len());
        destination[..copy].copy_from_slice(&input[..copy]);
    }

    fn push_name(&mut self, index: usize, byte: u8) -> bool {
        let Some(name) = self.name.as_deref_mut() else {
            return false;
        };
        let Some(slot) = name.get_mut(index) else {
            return false;
        };
        *slot = byte;
        true
    }

    fn push_comment(&mut self, index: usize, byte: u8) -> bool {
        let Some(comment) = self.comment.as_deref_mut() else {
            return false;
        };
        let Some(slot) = comment.get_mut(index) else {
            return false;
        };
        *slot = byte;
        true
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

pub unsafe fn inflate(
    strm: &mut crate::zlib_h::z_stream_s,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    let Some((strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if strm.next_out.is_null()
        || strm.next_in.is_null() && strm.avail_in != 0 as crate::stdlib::uInt
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The ABI state and stream have now passed their association and cursor
    // checks; the decoder below works through ordinary Rust references.
    // The decoder receives only call-scoped slices and scalar publication
    // state.  Keep the registered `NonNull` itself in `inflate_state` so its
    // provenance survives until writeback.
    let registered_header = state.head;
    let mut header = None;
    let result = '_inflate_result: {
        // The registration retains `NonNull` provenance from `inflateGetHeader()`.
        // Project its selected caller buffers only for this decoder invocation.
        header = registered_header.map(|registered| {
            let header = &mut *registered.as_ptr();
            let extra = if header.extra.is_none() || header.extra_max == 0 {
                None
            } else {
                Some(::core::slice::from_raw_parts_mut(
                    header.extra.expect("extra was checked").as_ptr(),
                    header.extra_max as usize,
                ))
            };
            let name = if header.name.is_null() || header.name_max == 0 {
                None
            } else {
                Some(::core::slice::from_raw_parts_mut(
                    header.name,
                    header.name_max as usize,
                ))
            };
            let comment = if header.comment.is_null() || header.comm_max == 0 {
                None
            } else {
                Some(::core::slice::from_raw_parts_mut(
                    header.comment,
                    header.comm_max as usize,
                ))
            };
            InflateHeaderOutput {
                extra,
                name,
                comment,
                extra_len: header.extra_len as ::core::ffi::c_uint,
                publication: HeaderPublication::default(),
            }
        });
        if state.normal.mode as ::core::ffi::c_uint
            == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            state.normal.mode = crate::src::inflate::TYPEDO;
        }
        left = strm.avail_out as ::core::ffi::c_uint;
        // Keep the caller-owned output range as one bounded view. Cursor
        // publication below derives its progress from this bounded range.
        let output = ::core::slice::from_raw_parts_mut(strm.next_out, left as usize);
        let mut output_chunk_start = 0usize;
        have = strm.avail_in as ::core::ffi::c_uint;
        // Preserve the C API's null-plus-zero input convention while keeping the
        // decoder's byte pulls bounded by the caller's advertised input range.
        let input = if have == 0 {
            &[][..]
        } else {
            ::core::slice::from_raw_parts(strm.next_in, have as usize)
        };
        hold = state.normal.hold;
        bits = state.normal.bits;
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
                                                                                                match state.normal.mode as ::core::ffi::c_uint {
                                                                                                16180 => {
                                                                                                    if state.normal.wrap == 0 as ::core::ffi::c_int {
                                                                                                        state.normal.mode = crate::src::inflate::TYPEDO;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                        }
                                                                                                        if state.normal.wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if state.normal.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                state.normal.wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            state.normal.check = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            state.normal.check = crate::src::crc32::crc32_z(
                                                                                                                state.normal.check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            state.normal.mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                            if let Some(head) = header.as_mut() {
                                                                                                                head.set_done(-1 as ::core::ffi::c_int);
                                                                                                            }
                                                                                                            if state.normal.wrap & 1 as ::core::ffi::c_int == 0
                                                                                                                || (((hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                                                                                                    .wrapping_rem(31 as ::core::ffi::c_ulong) != 0
                                                                                                            {
                                                                                                                strm.msg = INFLATE_ERROR_MESSAGES[0].as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                state.normal.mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                strm.msg = INFLATE_ERROR_MESSAGES[1].as_ptr()
                                                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                state.normal.mode = crate::src::inflate::BAD;
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
                                                                                                                if state.normal.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                    state.normal.wbits = len;
                                                                                                                }
                                                                                                                if len > 15 as ::core::ffi::c_uint || len > state.normal.wbits {
                                                                                                                    strm.msg = INFLATE_ERROR_MESSAGES[2].as_ptr()
                                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                    state.normal.mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    state.normal.dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    state.normal.flags = 0 as ::core::ffi::c_int;
                                                                                                                    state.normal.check = crate::src::adler32::adler32_z(
                                                                                                                        0 as crate::stdlib::uLong,
                                                                                                                        None,
                                                                                                                    ) as ::core::ffi::c_ulong;
                                                                                                                    strm.adler = state.normal.check as crate::stdlib::uLong;
                                                                                                                    state.normal.mode = (if hold & 0x200 as ::core::ffi::c_ulong
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
                                                                                                    }
                                                                                                    state.normal.flags = hold as ::core::ffi::c_int;
                                                                                                    if state.normal.flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        strm.msg = INFLATE_ERROR_MESSAGES[1].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state.normal.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if state.normal.flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        strm.msg = INFLATE_ERROR_MESSAGES[3].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state.normal.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if let Some(head) = header.as_mut() {
                                                                                                            head.set_text((hold >> 8 as ::core::ffi::c_int
                                                                                                                & 1 as ::core::ffi::c_ulong) as ::core::ffi::c_int);
                                                                                                        }
                                                                                                        if state.normal.flags & 0x200 as ::core::ffi::c_int != 0
                                                                                                            && state.normal.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                        {
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            state.normal.check = crate::src::crc32::crc32_z(
                                                                                                                state.normal.check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                        }
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        state.normal.mode = crate::src::inflate::TIME;
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
                                                                                                    }
                                                                                                    state.normal.check = (hold >> 24 as ::core::ffi::c_int
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
                                                                                                    strm.adler = state.normal.check as crate::stdlib::uLong;
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                    state.normal.mode = crate::src::inflate::DICT;
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
                                                                                                    }
                                                                                                    if hold & 0xffff as ::core::ffi::c_ulong
                                                                                                        != hold >> 16 as ::core::ffi::c_int
                                                                                                            ^ 0xffff as ::core::ffi::c_ulong
                                                                                                    {
                                                                                                        strm.msg = INFLATE_ERROR_MESSAGES[4].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state.normal.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        state.normal.length = hold as ::core::ffi::c_uint
                                                                                                            & 0xffff as ::core::ffi::c_uint;
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        state.normal.mode = crate::src::inflate::COPY_;
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
                                                                                                    }
                                                                                                    state.normal.nlen = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(257 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    state.normal.ndist = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(1 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    state.normal.ncode = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(4 as ::core::ffi::c_uint);
                                                                                                    hold >>= 4 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            4 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    if state.normal.nlen > 286 as ::core::ffi::c_uint
                                                                                                        || state.normal.ndist > 30 as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        strm.msg = INFLATE_ERROR_MESSAGES[5]
                                                                                                            .as_ptr() as *const ::core::ffi::c_char
                                                                                                            as *mut ::core::ffi::c_char;
                                                                                                        state.normal.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        state.normal.have = 0 as ::core::ffi::c_uint;
                                                                                                        state.normal.mode = crate::src::inflate::LENLENS;
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
                                                                                                    output[output_index] = state.normal.length
                                                                                                        as ::core::ffi::c_uchar;
                                                                                                    left = left.wrapping_sub(1);
                                                                                                    state.normal.mode = crate::src::inflate::LEN;
                                                                                                    continue '_inf_leave;
                                                                                                }
                                                                                                16206 => {
                                                                                                    if state.normal.wrap != 0 {
                                                                                                        while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                        }
                                                                                                        out = out.wrapping_sub(left);
                                                                                                        strm.total_out = (*strm)
                                                                                                            .total_out
                                                                                                            .wrapping_add(out as crate::stdlib::uLong);
                                                                                                        state.normal.total = (*state)
                                                                                                            .normal
                                                                                                            .total
                                                                                                            .wrapping_add(out as ::core::ffi::c_ulong);
                                                                                                        if state.normal.wrap & 4 as ::core::ffi::c_int != 0 && out != 0
                                                                                                        {
                                                                                                            let produced = &output[output_chunk_start
                                                                                                                ..output_chunk_start + out as usize];
                                                                                                            state.normal.check = (if state.normal.flags != 0 {
                                                                                                                crate::src::crc32::crc32_z(
                                                                                                                    state.normal.check as crate::stdlib::uLong,
                                                                                                                    Some(produced),
                                                                                                                )
                                                                                                            } else {
                                                                                                                crate::src::adler32::adler32(
                                                                                                                    state.normal.check as crate::stdlib::uLong,
                                                                                                                    produced,
                                                                                                                )
                                                                                                            }) as ::core::ffi::c_ulong;
                                                                                                            strm.adler = state.normal.check as crate::stdlib::uLong;
                                                                                                        }
                                                                                                        out = left;
                                                                                                        // Subsequent output is a new checksum
                                                                                                        // chunk in the caller's bounded range.
                                                                                                        output_chunk_start = output.len() - left as usize;
                                                                                                        if state.normal.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                            && (if state.normal.flags != 0 {
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
                                                                                                            }) != state.normal.check
                                                                                                        {
                                                                                                            strm.msg = INFLATE_ERROR_MESSAGES[6].as_ptr()
                                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                            state.normal.mode = crate::src::inflate::BAD;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                        }
                                                                                                    }
                                                                                                    state.normal.mode = crate::src::inflate::LENGTH;
                                                                                                }
                                                                                                16207 => {}
                                                                                                16208 => {
                                                                                                    break 'c_2443;
                                                                                                }
                                                                                                16209 => {
                                                                                                    ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                16210 => {
                                                                                                    break '_inflate_result crate::zlib_h::Z_MEM_ERROR;
                                                                                                }
                                                                                                16211 | _ => {
                                                                                                    break '_inflate_result crate::zlib_h::Z_STREAM_ERROR;
                                                                                                }
                                                                                            }
                                                                                                if state.normal.wrap != 0 && state.normal.flags != 0 {
                                                                                                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                {
                                                                                                    if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                }
                                                                                                if state.normal.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                    && hold
                                                                                                        != state.normal.total & 0xffffffff as ::core::ffi::c_ulong
                                                                                                {
                                                                                                    strm.msg = INFLATE_ERROR_MESSAGES[7].as_ptr()
                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                    state.normal.mode = crate::src::inflate::BAD;
                                                                                                    continue '_inf_leave;
                                                                                                } else {
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                }
                                                                                            }
                                                                                                state.normal.mode = crate::src::inflate::DONE;
                                                                                                break 'c_2443;
                                                                                            }
                                                                                            while state.normal.have < state.normal.ncode {
                                                                                            while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                            {
                                                                                                if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                            }
                                                                                            let c2rust_fresh15 = state.normal.have;
                                                                                            state.normal.have = state.normal.have.wrapping_add(1);
                                                                                            state.normal.lens[order[c2rust_fresh15 as usize] as usize] = (hold
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
                                                                                            while state.normal.have < 19 as ::core::ffi::c_uint {
                                                                                            let c2rust_fresh16 = state.normal.have;
                                                                                            state.normal.have = state.normal.have.wrapping_add(1);
                                                                                            state.normal.lens[order[c2rust_fresh16 as usize] as usize] = 0
                                                                                                as ::core::ffi::c_ushort;
                                                                                        }
                                                                                            (*state)
                                                                                            .normal
                                                                                            .next =
                                                                                            0;
                                                                                            state.normal.distcode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                                            state.normal.lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                                            state.normal.lenbits = 7 as ::core::ffi::c_uint;
                                                                                            ret = 'table: {
                                                                                                let state = &mut *state;
                                                                                                let table_start = state.normal.next;
                                                                                                let Some(lens) = state.normal.lens.get(..19) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                                let Some(table) = state.normal.codes.get_mut(table_start..) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                                let Some(work) = state.normal.work.get_mut(..19) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                                let (status, used) = crate::src::inftrees::inflate_table(
                                                                                                crate::src::inftrees::CODES,
                                                                                                lens,
                                                                                                table,
                                                                                                &mut state.normal.lenbits,
                                                                                                work,
                                                                                            );
                                                                                                if status == 0 {
                                                                                                state.normal.next += used;
                                                                                            }
                                                                                                status
                                                                                            };
                                                                                            if ret
                                                                                                != 0
                                                                                            {
                                                                                                strm.msg = INFLATE_ERROR_MESSAGES[8].as_ptr()
                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                state.normal.mode = crate::src::inflate::BAD;
                                                                                                continue '_inf_leave;
                                                                                            } else {
                                                                                                state.normal.have = 0 as ::core::ffi::c_uint;
                                                                                                state.normal.mode = crate::src::inflate::CODELENS;
                                                                                                break 's_1689;
                                                                                            }
                                                                                        }
                                                                                        if state.normal.havedict == 0 as ::core::ffi::c_int {
                                                                                        strm.next_out = strm
                                                                                            .next_out
                                                                                            .wrapping_add(output.len() - left as usize);
                                                                                        strm.avail_out = left as crate::stdlib::uInt;
                                                                                        strm.next_in = strm
                                                                                            .next_in
                                                                                            .wrapping_add(input.len() - have as usize);
                                                                                        strm.avail_in = have as crate::stdlib::uInt;
                                                                                        state.normal.hold = hold;
                                                                                        state.normal.bits = bits;
                                                                                        break '_inflate_result crate::zlib_h::Z_NEED_DICT;
                                                                                    }
                                                                                        state.normal.check = crate::src::adler32::adler32_z(
                                                                                        0 as crate::stdlib::uLong,
                                                                                        None,
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                        strm.adler =
                                                                                        (*state)
                                                                                            .normal
                                                                                            .check
                                                                                            as crate::stdlib::uLong;
                                                                                        state.normal.mode =
                                                                                        crate::src::inflate::TYPE;
                                                                                        break 'c_2339;
                                                                                    }
                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                {
                                                                                    if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                        break '_inf_leave;
                                                                                    }
                                                                                }
                                                                                    if let Some(
                                                                                        head,
                                                                                    ) = header
                                                                                        .as_mut()
                                                                                    {
                                                                                        head.set_time(hold as crate::stdlib::uLong);
                                                                                    }
                                                                                    if state.normal.flags & 0x200 as ::core::ffi::c_int != 0
                                                                                    && state.normal.wrap & 4 as ::core::ffi::c_int != 0
                                                                                {
                                                                                    hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                    hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[2 as usize] = (hold >> 16 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[3 as usize] = (hold >> 24 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    state.normal.check = crate::src::crc32::crc32_z(
                                                                                        state.normal.check as crate::stdlib::uLong,
                                                                                        Some(&hbuf[..4]),
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                }
                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                    state.normal.mode = crate::src::inflate::OS;
                                                                                    break 's_519;
                                                                                }
                                                                                state.normal.mode = crate::src::inflate::COPY_1;
                                                                                break 'c_2356;
                                                                            }
                                                                            while state.normal.have
                                                                                < (*state)
                                                                                    .normal
                                                                                    .nlen
                                                                                    .wrapping_add(
                                                                                        state
                                                                                            .normal
                                                                                            .ndist,
                                                                                    )
                                                                            {
                                                                                loop {
                                                                                    here = crate::src::inftrees::code::copied_from((*state)
                                                                                    .normal
                                                                                    .lencode
                                                                                    .get(&state.normal.codes,
                                                                                        (hold as ::core::ffi::c_uint
                                                                                            & ((1 as ::core::ffi::c_uint) << state.normal.lenbits)
                                                                                                .wrapping_sub(1 as ::core::ffi::c_uint)) as isize,
                                                                                    ));
                                                                                    if here.bits as ::core::ffi::c_uint <= bits {
                                                                                    break;
                                                                                }
                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                    break '_inf_leave;
                                                                                }
                                                                                    inflate_pull_byte(
                                                                                    input, in_0,
                                                                                    &mut have,
                                                                                    &mut hold,
                                                                                    &mut bits,
                                                                                );
                                                                                }
                                                                                if (here.val as ::core::ffi::c_int)
                                                                                < 16 as ::core::ffi::c_int
                                                                            {
                                                                                hold >>= here.bits as ::core::ffi::c_int;
                                                                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                let c2rust_fresh18 = state.normal.have;
                                                                                state.normal.have = state.normal.have.wrapping_add(1);
                                                                                state.normal.lens[c2rust_fresh18 as usize] = here.val;
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
                                                                                    }
                                                                                    hold >>= here.bits as ::core::ffi::c_int;
                                                                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                    if state.normal.have == 0 as ::core::ffi::c_uint {
                                                                                        strm.msg = INFLATE_ERROR_MESSAGES[9].as_ptr()
                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                        state.normal.mode = crate::src::inflate::BAD;
                                                                                        break;
                                                                                    } else {
                                                                                        len = (*state)
                                                                                            .normal
                                                                                            .lens[state.normal.have.wrapping_sub(1 as ::core::ffi::c_uint)
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
                                                                                if state.normal.have.wrapping_add(copy)
                                                                                    > state.normal.nlen.wrapping_add(state.normal.ndist)
                                                                                {
                                                                                    strm.msg = INFLATE_ERROR_MESSAGES[9].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state.normal.mode = crate::src::inflate::BAD;
                                                                                    break;
                                                                                } else {
                                                                                    loop {
                                                                                        let c2rust_fresh22 = copy;
                                                                                        copy = copy.wrapping_sub(1);
                                                                                        if c2rust_fresh22 == 0 {
                                                                                            break;
                                                                                        }
                                                                                        let c2rust_fresh23 = state.normal.have;
                                                                                        state.normal.have = state.normal.have.wrapping_add(1);
                                                                                        state.normal.lens[c2rust_fresh23 as usize] = len
                                                                                            as ::core::ffi::c_ushort;
                                                                                    }
                                                                                }
                                                                            }
                                                                            }
                                                                            if state.normal.mode as ::core::ffi::c_uint
                                                                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            continue '_inf_leave;
                                                                        }
                                                                            if state.normal.lens[256 as usize] as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            strm.msg = INFLATE_ERROR_MESSAGES[10]
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            state.normal.mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            state.normal.next = 0;
                                                                            state.normal.lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                            state.normal.lenbits = 9 as ::core::ffi::c_uint;
                                                                            ret = 'table: {
                                                                                let state = &mut *state;
                                                                                let codes = state.normal.nlen as usize;
                                                                                let table_start = state.normal.next;
                                                                                let Some(lens) = state.normal.lens.get(..codes) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let Some(table) = state.normal.codes.get_mut(table_start..) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let Some(work) = state.normal.work.get_mut(..codes) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let (status, used) = crate::src::inftrees::inflate_table(
                                                                                    crate::src::inftrees::LENS,
                                                                                    lens,
                                                                                    table,
                                                                                    &mut state.normal.lenbits,
                                                                                    work,
                                                                                );
                                                                                if status == 0 {
                                                                                    state.normal.next += used;
                                                                                }
                                                                                status
                                                                            };
                                                                            if ret != 0 {
                                                                                strm.msg = INFLATE_ERROR_MESSAGES[11].as_ptr()
                                                                                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                state.normal.mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                state.normal.distcode = crate::src::inflate::CodeTableRef::Dynamic(state.normal.next);
                                                                                state.normal.distbits = 6 as ::core::ffi::c_uint;
                                                                                ret = 'table: {
                                                                                    let state = &mut *state;
                                                                                    let lens_start = state.normal.nlen as usize;
                                                                                    let codes = state.normal.ndist as usize;
                                                                                    let Some(lens_end) = lens_start.checked_add(codes) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let table_start = state.normal.next;
                                                                                    let Some(lens) = state.normal.lens.get(lens_start..lens_end) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let Some(table) = state.normal.codes.get_mut(table_start..) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let Some(work) = state.normal.work.get_mut(..codes) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let (status, used) = crate::src::inftrees::inflate_table(
                                                                                        crate::src::inftrees::DISTS,
                                                                                        lens,
                                                                                        table,
                                                                                        &mut state.normal.distbits,
                                                                                        work,
                                                                                    );
                                                                                    if status == 0 {
                                                                                        state.normal.next += used;
                                                                                    }
                                                                                    status
                                                                                };
                                                                                if ret != 0 {
                                                                                    strm.msg = INFLATE_ERROR_MESSAGES[12].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state.normal.mode = crate::src::inflate::BAD;
                                                                                    continue '_inf_leave;
                                                                                } else {
                                                                                    state.normal.mode = crate::src::inflate::LEN_;
                                                                                    if flush == crate::zlib_h::Z_TREES {
                                                                                        break '_inf_leave;
                                                                                    } else {
                                                                                        break 'c_2397;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        }
                                                                        copy = state.normal.length;
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
                                                                            let output_start =
                                                                                output.len()
                                                                                    - left as usize;
                                                                            output[output_start
                                                                            ..output_start
                                                                                + copy_len]
                                                                            .copy_from_slice(
                                                                                &input[input_start
                                                                                    ..input_start
                                                                                        + copy_len],
                                                                            );
                                                                            have = have
                                                                                .wrapping_sub(copy);
                                                                            left = left
                                                                                .wrapping_sub(copy);
                                                                            state.normal.length =
                                                                                (*state)
                                                                                    .normal
                                                                                    .length
                                                                                    .wrapping_sub(
                                                                                        copy,
                                                                                    );
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            state.normal.mode = crate::src::inflate::TYPE;
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
                                                                    }
                                                                    if let Some(head) =
                                                                        header.as_mut()
                                                                    {
                                                                        head.set_xflags_and_os(
                                                                        (hold & 0xff
                                                                            as ::core::ffi::c_ulong)
                                                                            as ::core::ffi::c_int,
                                                                        (hold >> 8
                                                                            as ::core::ffi::c_int)
                                                                            as ::core::ffi::c_int,
                                                                    );
                                                                    }
                                                                    if state.normal.flags
                                                                    & 0x200 as ::core::ffi::c_int
                                                                    != 0
                                                                    && state.normal.wrap
                                                                        & 4 as ::core::ffi::c_int
                                                                        != 0
                                                                {
                                                                    hbuf[0 as usize] = hold
                                                                        as ::core::ffi::c_uchar;
                                                                    hbuf[1 as usize] = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_uchar;
                                                                    state.normal.check = crate::src::crc32::crc32_z(
                                                                        state.normal.check as crate::stdlib::uLong,
                                                                        Some(&hbuf[..2]),
                                                                    ) as ::core::ffi::c_ulong;
                                                                }
                                                                    hold =
                                                                        0 as ::core::ffi::c_ulong;
                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                    state.normal.mode =
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
                                                        if state.normal.last != 0 {
                                                            hold >>=
                                                                bits & 7 as ::core::ffi::c_uint;
                                                            bits = bits.wrapping_sub(
                                                                bits & 7 as ::core::ffi::c_uint,
                                                            );
                                                            state.normal.mode =
                                                                crate::src::inflate::CHECK;
                                                            continue '_inf_leave;
                                                        } else {
                                                            while bits
                                                                < 3 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_uint
                                                            {
                                                                if !inflate_pull_byte(
                                                                    input, in_0, &mut have,
                                                                    &mut hold, &mut bits,
                                                                ) {
                                                                    break '_inf_leave;
                                                                }
                                                            }
                                                            state.normal.last = (hold
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
                                                                    state.normal.mode =
                                                                        crate::src::inflate::STORED;
                                                                }
                                                                1 => {
                                                                    let state = &mut *state;
                                                                    crate::src::inftrees::inflate_fixed(
                                                                    &mut state.normal.lencode,
                                                                    &mut state.normal.lenbits,
                                                                    &mut state.normal.distcode,
                                                                    &mut state.normal.distbits,
                                                                );
                                                                    state.normal.mode =
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
                                                                    state.normal.mode =
                                                                        crate::src::inflate::TABLE;
                                                                }
                                                                _ => {
                                                                    strm.msg = INFLATE_ERROR_MESSAGES
                                                                    [13]
                                                                .as_ptr()
                                                                    as *const ::core::ffi::c_char
                                                                    as *mut ::core::ffi::c_char;
                                                                    state.normal.mode =
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
                                                    if state.normal.flags
                                                        & 0x400 as ::core::ffi::c_int
                                                        != 0
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
                                                        }
                                                        state.normal.length =
                                                            hold as ::core::ffi::c_uint;
                                                        if let Some(head) = header.as_mut() {
                                                            head.set_extra_len(
                                                                hold as ::core::ffi::c_uint
                                                                    as crate::stdlib::uInt,
                                                            );
                                                        }
                                                        if state.normal.flags
                                                            & 0x200 as ::core::ffi::c_int
                                                            != 0
                                                            && state.normal.wrap
                                                                & 4 as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            hbuf[0 as usize] =
                                                                hold as ::core::ffi::c_uchar;
                                                            hbuf[1 as usize] = (hold
                                                                >> 8 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uchar;
                                                            state.normal.check =
                                                                crate::src::crc32::crc32_z(
                                                                    state.normal.check
                                                                        as crate::stdlib::uLong,
                                                                    Some(&hbuf[..2]),
                                                                )
                                                                    as ::core::ffi::c_ulong;
                                                        }
                                                        hold = 0 as ::core::ffi::c_ulong;
                                                        bits = 0 as ::core::ffi::c_uint;
                                                    } else if let Some(head) = header.as_mut() {
                                                        head.clear_extra();
                                                    }
                                                    state.normal.mode = crate::src::inflate::EXTRA;
                                                    break 'c_2319;
                                                }
                                                state.normal.mode = crate::src::inflate::LEN;
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
                                                let window = state.normal.owned_window.as_deref();
                                                let fast_state =
                                                crate::src::inffast::InflateFastState {
                                                    history:
                                                        crate::src::inffast::FastHistory::External(
                                                            window,
                                                        ),
                                                    wsize: state.normal.wsize as usize,
                                                    whave: state.normal.whave as usize,
                                                    wnext: state.normal.wnext as usize,
                                                    hold,
                                                    bits,
                                                    lcode: state.normal.lencode,
                                                    dcode: state.normal.distcode,
                                                    lmask: (1u32 << state.normal.lenbits) - 1,
                                                    dmask: (1u32 << state.normal.distbits) - 1,
                                                    codes: &state.normal.codes,
                                                    sane: state.normal.sane != 0,
                                                };
                                                let owner = InflateNormalStreamOwner::new(
                                                    input, output, written, fast_state,
                                                )
                                                .expect(
                                                    "normal inflate checked its fast output cursor",
                                                );
                                                let result = owner.run_fast();
                                                have =
                                                    result.input_remaining as ::core::ffi::c_uint;
                                                left =
                                                    result.output_remaining as ::core::ffi::c_uint;
                                                hold = result.hold;
                                                bits = result.bits;
                                                match result.exit {
                                                crate::src::inffast::FastExit::Continue => {}
                                                crate::src::inffast::FastExit::Type => {
                                                    state.normal.mode = crate::src::inflate::TYPE;
                                                }
                                                crate::src::inffast::FastExit::InvalidDistance => {
                                                    let strm = &mut *strm;
                                                    strm.msg = INFLATE_ERROR_MESSAGES[17].as_ptr()
                                                        as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char;
                                                    state.normal.mode = crate::src::inflate::BAD;
                                                }
                                                crate::src::inffast::FastExit::InvalidCode => {
                                                    let strm = &mut *strm;
                                                    strm.msg = b"invalid literal/length or distance code\0"
                                                        .as_ptr()
                                                        .cast_mut()
                                                        .cast();
                                                    state.normal.mode = crate::src::inflate::BAD;
                                                }
                                            }
                                                if state.normal.mode as ::core::ffi::c_uint
                                                    == crate::src::inflate::TYPE
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    state.normal.back = -1 as ::core::ffi::c_int;
                                                }
                                                continue '_inf_leave;
                                            } else {
                                                state.normal.back = 0 as ::core::ffi::c_int;
                                                loop {
                                                    here = crate::src::inftrees::code::copied_from(
                                                        state.normal.lencode.get(
                                                            &state.normal.codes,
                                                            (hold as ::core::ffi::c_uint
                                                                & ((1 as ::core::ffi::c_uint)
                                                                    << state.normal.lenbits)
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
                                                        input, in_0, &mut have, &mut hold,
                                                        &mut bits,
                                                    ) {
                                                        break '_inf_leave;
                                                    }
                                                }
                                                if here.op as ::core::ffi::c_int != 0
                                                    && here.op as ::core::ffi::c_int
                                                        & 0xf0 as ::core::ffi::c_int
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    last = crate::src::inftrees::code::copied_from(
                                                        &here,
                                                    );
                                                    loop {
                                                        here = crate::src::inftrees::code::copied_from(state.normal.lencode.get(&state.normal.codes,
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
                                                    }
                                                    hold >>= last.bits as ::core::ffi::c_int;
                                                    bits = bits.wrapping_sub(
                                                        last.bits as ::core::ffi::c_uint,
                                                    );
                                                    state.normal.back +=
                                                        last.bits as ::core::ffi::c_int;
                                                }
                                                hold >>= here.bits as ::core::ffi::c_int;
                                                bits = bits
                                                    .wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                state.normal.back +=
                                                    here.bits as ::core::ffi::c_int;
                                                state.normal.length =
                                                    here.val as ::core::ffi::c_uint;
                                                if here.op as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                {
                                                    state.normal.mode = crate::src::inflate::LIT;
                                                    continue '_inf_leave;
                                                } else if here.op as ::core::ffi::c_int
                                                    & 32 as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    state.normal.back = -1 as ::core::ffi::c_int;
                                                    state.normal.mode = crate::src::inflate::TYPE;
                                                    continue '_inf_leave;
                                                } else if here.op as ::core::ffi::c_int
                                                    & 64 as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    strm.msg = INFLATE_ERROR_MESSAGES[14].as_ptr()
                                                        as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char;
                                                    state.normal.mode = crate::src::inflate::BAD;
                                                    continue '_inf_leave;
                                                } else {
                                                    state.normal.extra = here.op
                                                        as ::core::ffi::c_uint
                                                        & 15 as ::core::ffi::c_uint;
                                                    state.normal.mode = crate::src::inflate::LENEXT;
                                                    break 'c_2410;
                                                }
                                            }
                                        }
                                        if state.normal.flags & 0x400 as ::core::ffi::c_int != 0 {
                                            copy = state.normal.length;
                                            if copy > have {
                                                copy = have;
                                            }
                                            if copy != 0 {
                                                if let Some(head) = header.as_mut() {
                                                    // The caller input and the separately registered
                                                    // header-extra buffer have the original C memcpy
                                                    // non-overlap contract. `copy_extra()` clips at the
                                                    // advertised output capacity just as zlib does.
                                                    let input_start =
                                                        in_0.wrapping_sub(have) as usize;
                                                    head.copy_extra(
                                                        state.normal.length,
                                                        &input[input_start
                                                            ..input_start + copy as usize],
                                                    );
                                                }
                                                if state.normal.flags & 0x200 as ::core::ffi::c_int
                                                    != 0
                                                    && state.normal.wrap & 4 as ::core::ffi::c_int
                                                        != 0
                                                {
                                                    state.normal.check = crate::src::crc32::crc32_z(
                                                        state.normal.check as crate::stdlib::uLong,
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
                                                state.normal.length =
                                                    state.normal.length.wrapping_sub(copy);
                                            }
                                            if state.normal.length != 0 {
                                                break '_inf_leave;
                                            }
                                        }
                                        state.normal.length = 0 as ::core::ffi::c_uint;
                                        state.normal.mode = crate::src::inflate::NAME;
                                        break 'c_2322;
                                    }
                                    if state.normal.extra != 0 {
                                        while bits < state.normal.extra {
                                            if !inflate_pull_byte(
                                                input, in_0, &mut have, &mut hold, &mut bits,
                                            ) {
                                                break '_inf_leave;
                                            }
                                        }
                                        state.normal.length = state.normal.length.wrapping_add(
                                            hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint)
                                                    << state.normal.extra)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                        );
                                        hold >>= state.normal.extra;
                                        bits = bits.wrapping_sub(state.normal.extra);
                                        state.normal.back = (state.normal.back
                                            as ::core::ffi::c_uint)
                                            .wrapping_add(state.normal.extra)
                                            as ::core::ffi::c_int;
                                    }
                                    state.normal.was = state.normal.length;
                                    state.normal.mode = crate::src::inflate::DIST;
                                    break 's_2462;
                                }
                                if state.normal.flags & 0x800 as ::core::ffi::c_int != 0 {
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
                                        if let Some(head) = header.as_mut() {
                                            if head
                                                .push_name(state.normal.length as usize, len as u8)
                                            {
                                                state.normal.length =
                                                    state.normal.length.wrapping_add(1);
                                            }
                                        }
                                        if !(len != 0 && copy < have) {
                                            break;
                                        }
                                    }
                                    if state.normal.flags & 0x200 as ::core::ffi::c_int != 0
                                        && state.normal.wrap & 4 as ::core::ffi::c_int != 0
                                    {
                                        state.normal.check = crate::src::crc32::crc32_z(
                                            state.normal.check as crate::stdlib::uLong,
                                            Some(
                                                &input[in_0.wrapping_sub(have) as usize
                                                    ..in_0.wrapping_sub(have).wrapping_add(copy)
                                                        as usize],
                                            ),
                                        )
                                            as ::core::ffi::c_ulong;
                                    }
                                    have = have.wrapping_sub(copy);
                                    if len != 0 {
                                        break '_inf_leave;
                                    }
                                } else if let Some(head) = header.as_mut() {
                                    head.clear_name();
                                }
                                state.normal.length = 0 as ::core::ffi::c_uint;
                                state.normal.mode = crate::src::inflate::COMMENT;
                                break 'c_2325;
                            }
                            loop {
                                here = crate::src::inftrees::code::copied_from(
                                    state.normal.distcode.get(
                                        &state.normal.codes,
                                        (hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint) << state.normal.distbits)
                                                .wrapping_sub(1 as ::core::ffi::c_uint))
                                            as isize,
                                    ),
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits)
                                {
                                    break '_inf_leave;
                                }
                            }
                            if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                last = crate::src::inftrees::code::copied_from(&here);
                                loop {
                                    here = crate::src::inftrees::code::copied_from(
                                        state.normal.distcode.get(
                                            &state.normal.codes,
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
                                    if !inflate_pull_byte(
                                        input, in_0, &mut have, &mut hold, &mut bits,
                                    ) {
                                        break '_inf_leave;
                                    }
                                }
                                hold >>= last.bits as ::core::ffi::c_int;
                                bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                                state.normal.back += last.bits as ::core::ffi::c_int;
                            }
                            hold >>= here.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                            state.normal.back += here.bits as ::core::ffi::c_int;
                            if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                                strm.msg = INFLATE_ERROR_MESSAGES[15].as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                state.normal.mode = crate::src::inflate::BAD;
                                continue '_inf_leave;
                            } else {
                                state.normal.offset = here.val as ::core::ffi::c_uint;
                                state.normal.extra =
                                    here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                                state.normal.mode = crate::src::inflate::DISTEXT;
                                break 'c_2422;
                            }
                        }
                        if state.normal.flags & 0x1000 as ::core::ffi::c_int != 0 {
                            if have == 0 as ::core::ffi::c_uint {
                                break '_inf_leave;
                            }
                            copy = 0 as ::core::ffi::c_uint;
                            loop {
                                let c2rust_fresh7 = copy;
                                copy = copy.wrapping_add(1);
                                len = input
                                    [in_0.wrapping_sub(have) as usize + c2rust_fresh7 as usize]
                                    as ::core::ffi::c_uint;
                                if let Some(head) = header.as_mut() {
                                    if head.push_comment(state.normal.length as usize, len as u8) {
                                        state.normal.length = state.normal.length.wrapping_add(1);
                                    }
                                }
                                if !(len != 0 && copy < have) {
                                    break;
                                }
                            }
                            if state.normal.flags & 0x200 as ::core::ffi::c_int != 0
                                && state.normal.wrap & 4 as ::core::ffi::c_int != 0
                            {
                                state.normal.check = crate::src::crc32::crc32_z(
                                    state.normal.check as crate::stdlib::uLong,
                                    Some(
                                        &input[in_0.wrapping_sub(have) as usize
                                            ..in_0.wrapping_sub(have).wrapping_add(copy) as usize],
                                    ),
                                )
                                    as ::core::ffi::c_ulong;
                            }
                            have = have.wrapping_sub(copy);
                            if len != 0 {
                                break '_inf_leave;
                            }
                        } else if let Some(head) = header.as_mut() {
                            head.clear_comment();
                        }
                        state.normal.mode = crate::src::inflate::HCRC;
                        break 'c_2327;
                    }
                    if state.normal.extra != 0 {
                        while bits < state.normal.extra {
                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                break '_inf_leave;
                            }
                        }
                        state.normal.offset = state.normal.offset.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << state.normal.extra)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= state.normal.extra;
                        bits = bits.wrapping_sub(state.normal.extra);
                        state.normal.back = (state.normal.back as ::core::ffi::c_uint)
                            .wrapping_add(state.normal.extra)
                            as ::core::ffi::c_int;
                    }
                    state.normal.mode = crate::src::inflate::MATCH;
                    break 'c_2425;
                }
                if state.normal.flags & 0x200 as ::core::ffi::c_int != 0 {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                            break '_inf_leave;
                        }
                    }
                    if state.normal.wrap & 4 as ::core::ffi::c_int != 0
                        && hold != state.normal.check & 0xffff as ::core::ffi::c_ulong
                    {
                        strm.msg = INFLATE_ERROR_MESSAGES[16].as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.normal.mode = crate::src::inflate::BAD;
                        continue '_inf_leave;
                    } else {
                        hold = 0 as ::core::ffi::c_ulong;
                        bits = 0 as ::core::ffi::c_uint;
                    }
                }
                if let Some(head) = header.as_mut() {
                    head.set_hcrc_and_done(
                        state.normal.flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
                    );
                }
                state.normal.check = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None)
                    as ::core::ffi::c_ulong;
                strm.adler = state.normal.check as crate::stdlib::uLong;
                state.normal.mode = crate::src::inflate::TYPE;
                continue '_inf_leave;
            }
            if left == 0 as ::core::ffi::c_uint {
                break;
            }
            let produced = out.wrapping_sub(left) as usize;
            let offset = state.normal.offset as usize;
            let mut history = None;
            copy = 0;
            if offset > produced {
                copy = (offset - produced) as ::core::ffi::c_uint;
                if copy > state.normal.whave {
                    if state.normal.sane != 0 {
                        strm.msg = INFLATE_ERROR_MESSAGES[17].as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.normal.mode = crate::src::inflate::BAD;
                        continue;
                    }
                }
                if copy > state.normal.wnext {
                    copy = copy.wrapping_sub(state.normal.wnext);
                    history = Some((state.normal.wsize.wrapping_sub(copy) as usize, copy));
                } else {
                    history = Some((state.normal.wnext.wrapping_sub(copy) as usize, copy));
                }
                if copy > state.normal.length {
                    copy = state.normal.length;
                }
            } else {
                copy = state.normal.length;
            }
            if copy > left {
                copy = left;
            }
            let copy_len = copy as usize;
            // `out` is the capacity since the last checksum boundary and
            // `produced + copy_len` is bounded by it after the `left` clamp.
            let output = &mut output[output_chunk_start..output_chunk_start + out as usize];
            if let Some((start, history_limit)) = history {
                let window = (*state)
                    .normal
                    .owned_window
                    .as_deref()
                    .expect("normal inflate owns its history window");
                let history = &window[start..start + copy_len.min(history_limit as usize)];
                copy_inflate_match(output, produced, offset, Some(history), copy_len);
            } else {
                copy_inflate_match(output, produced, offset, None, copy_len);
            }
            left = left.wrapping_sub(copy);
            state.normal.length = state.normal.length.wrapping_sub(copy);
            if state.normal.length == 0 as ::core::ffi::c_uint {
                state.normal.mode = crate::src::inflate::LEN;
            }
        }
        strm.next_out = strm.next_out.wrapping_add(output.len() - left as usize);
        strm.avail_out = left as crate::stdlib::uInt;
        strm.next_in = strm.next_in.wrapping_add(input.len() - have as usize);
        strm.avail_in = have as crate::stdlib::uInt;
        state.normal.hold = hold;
        state.normal.bits = bits;
        if state.normal.wsize != 0
            || out != strm.avail_out
                && (state.normal.mode as ::core::ffi::c_uint)
                    < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                && ((state.normal.mode as ::core::ffi::c_uint)
                    < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                    || flush != crate::zlib_h::Z_FINISH)
        {
            // `output` is the bounded caller range retained for this call, and
            // `output_chunk_start` marks the same post-checksum chunk that the
            // former cursor adapter passed to `updatewindow()`.  Keep history
            // updates in the slice core instead of rebuilding raw cursors from
            // the ABI stream after all decoding has completed.
            let produced = out.wrapping_sub(strm.avail_out as ::core::ffi::c_uint) as usize;
            let produced_output = &output[output_chunk_start..output_chunk_start + produced];
            if update_window_from_slice(
                &mut state.normal.owned_window,
                state.normal.wbits,
                &mut state.normal.wsize,
                &mut state.normal.wnext,
                &mut state.normal.whave,
                produced_output,
            ) != 0
            {
                state.normal.mode = crate::src::inflate::MEM;
                break '_inflate_result crate::zlib_h::Z_MEM_ERROR;
            }
        }
        in_0 = in_0.wrapping_sub(strm.avail_in as ::core::ffi::c_uint);
        out = out.wrapping_sub(strm.avail_out as ::core::ffi::c_uint);
        strm.total_in = strm.total_in.wrapping_add(in_0 as crate::stdlib::uLong);
        strm.total_out = strm.total_out.wrapping_add(out as crate::stdlib::uLong);
        state.normal.total = state.normal.total.wrapping_add(out as ::core::ffi::c_ulong);
        if state.normal.wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
            let produced = &output[output_chunk_start..output_chunk_start + out as usize];
            state.normal.check = (if state.normal.flags != 0 {
                crate::src::crc32::crc32_z(
                    state.normal.check as crate::stdlib::uLong,
                    Some(produced),
                )
            } else {
                crate::src::adler32::adler32(state.normal.check as crate::stdlib::uLong, produced)
            }) as ::core::ffi::c_ulong;
            strm.adler = state.normal.check as crate::stdlib::uLong;
        }
        strm.data_type = state.normal.bits as ::core::ffi::c_int
            + (if state.normal.last != 0 {
                64 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            + (if state.normal.mode as ::core::ffi::c_uint
                == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                128 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            + (if state.normal.mode as ::core::ffi::c_uint
                == crate::src::inflate::LEN_ as ::core::ffi::c_int as ::core::ffi::c_uint
                || state.normal.mode as ::core::ffi::c_uint
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
        ret
    };
    // Move the scalar completion out before publishing it.  This drops the
    // bounded extra/name/comment borrows first, so the retained registration
    // is reborrowed only after the decoder's header view has ended.
    let header_publication = header.map(|output| output.publication);
    // Publish only scalar/pointer-slot changes after the pointer-free decoder
    // has finished with its call-scoped slices.  Use the retained `NonNull`
    // directly so provenance is never reconstructed from an address token.
    if let (Some(registered), Some(publication)) = (registered_header, header_publication) {
        let header = &mut *registered.as_ptr();
        if let Some(value) = publication.text {
            header.text = value;
        }
        if let Some(value) = publication.time {
            header.time = value;
        }
        if let Some(value) = publication.xflags {
            header.xflags = value;
        }
        if let Some(value) = publication.os {
            header.os = value;
        }
        if let Some(value) = publication.extra_len {
            header.extra_len = value;
        }
        if let Some(value) = publication.hcrc {
            header.hcrc = value;
        }
        if let Some(value) = publication.done {
            header.done = value;
        }
        if publication.clear_extra {
            header.extra = None;
        }
        if publication.clear_name {
            header.name = ::core::ptr::null_mut();
        }
        if publication.clear_comment {
            header.comment = ::core::ptr::null_mut();
        }
    }
    return result;
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
// Ending a normal inflate stream has a pointer-free half: consume the Rust
// history owner before the ABI adapter releases the callback-owned state
// record.  Keeping that ordering in this owner makes the callback boundary
// responsible only for its original allocation handle and paired `zfree`.
struct InflateEndOwner<'state> {
    normal: &'state mut InflateNormalState,
}

impl<'state> InflateEndOwner<'state> {
    fn from_normal(normal: &'state mut InflateNormalState) -> Self {
        Self { normal }
    }

    fn release(self) {
        drop(self.normal.owned_window.take());
    }
}

pub unsafe fn inflateEnd(stream: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    // Snapshot the opaque allocation handle before borrowing its typed
    // contents.  This preserves the callback's provenance without deriving
    // a new raw address from the projected state reference.
    let Some(state_handle) = stream.state else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Keep the ABI projections at the callback-release boundary.  The stream
    // must continue to point at that state during `zfree`, matching C's
    // observable release order.
    let Some((stream, state)) = inflate_stream_and_state(stream) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    InflateEndOwner::from_normal(&mut state.normal).release();
    let zfree = stream.zfree.expect("non-null function pointer");
    let opaque = stream.opaque;
    zfree(opaque, state_handle.as_ptr().cast());
    stream.state = None;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(stream) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateEnd(stream)
}
#[export_name = "inflateGetDictionary"]

pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    let Some(stream) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(request) = inflateGetDictionary(stream) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // C defines the dictionary output only when history exists.  Convert the
    // raw optional output at this boundary, using the bounded request's
    // length, so the implementation receives no raw output pointer.
    let dictionary = if dictionary.is_null() || request.dictionary_len() == 0 {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(
            dictionary,
            request.dictionary_len(),
        ))
    };
    let dict_length = dictLength.as_mut();
    inflate_get_dictionary(request, dictionary, dict_length)
}
pub unsafe fn inflateSetDictionary(
    strm: &mut crate::zlib_h::z_stream_s,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let mut dictid: ::core::ffi::c_ulong = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if state.normal.wrap != 0 as ::core::ffi::c_int
        && state.normal.mode as ::core::ffi::c_uint
            != crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if state.normal.mode as ::core::ffi::c_uint
        == crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        dictid =
            crate::src::adler32::adler32_z(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
        dictid = crate::src::adler32::adler32(dictid as crate::stdlib::uLong, dictionary)
            as ::core::ffi::c_ulong;
        if dictid != state.normal.check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }
    ret = update_window_from_slice(
        &mut state.normal.owned_window,
        state.normal.wbits,
        &mut state.normal.wsize,
        &mut state.normal.wnext,
        &mut state.normal.whave,
        dictionary,
    );
    if ret != 0 {
        state.normal.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    state.normal.havedict = 1 as ::core::ffi::c_int;
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
    if dictionary.is_null() && dictLength != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = if dictLength == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(dictionary, dictLength as usize)
    };
    inflateSetDictionary(strm, dictionary)
}
// A registered gzip header must retain the original pointer's provenance for
// later decoder calls.  The export boundary forms that handle after checking
// the stream; this adapter owns the stream-bound opaque-state projection and
// the one immediate `done` publication.
pub unsafe fn inflateGetHeader(
    strm: &mut crate::zlib_h::z_stream_s,
    mut head: Option<::core::ptr::NonNull<crate::zlib_h::gz_header_s>>,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if state.normal.wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.head = head;
    if let Some(head) = head.as_mut() {
        head.as_mut().done = 0 as ::core::ffi::c_int;
    }
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
    inflateGetHeader(strm, ::core::ptr::NonNull::new(head))
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

// The sync marker scan has no need for an ABI stream or an allocation-backed
// inflate state.  Keep that policy in a small value so the boundary only has
// to project the caller input and publish the resulting cursors.
struct InflateSyncState {
    mode: crate::src::inflate::inflate_mode,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    flags: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
}

fn inflate_sync_core(
    mut state: InflateSyncState,
    input: &[::core::ffi::c_uchar],
) -> (InflateSyncState, usize, ::core::ffi::c_int) {
    if input.is_empty() && state.bits < 8 {
        return (state, 0, crate::zlib_h::Z_BUF_ERROR);
    }
    if state.mode != crate::src::inflate::SYNC {
        state.mode = crate::src::inflate::SYNC;
        state.hold >>= state.bits & 7;
        state.bits = state.bits.wrapping_sub(state.bits & 7);
        let mut len = 0usize;
        let mut buf = [0; 4];
        while state.bits >= 8 {
            buf[len] = state.hold as ::core::ffi::c_uchar;
            len += 1;
            state.hold >>= 8;
            state.bits = state.bits.wrapping_sub(8);
        }
        state.have = 0;
        syncsearch(&mut state.have, &buf[..len]);
    }
    let consumed = syncsearch(&mut state.have, input) as usize;
    if state.have != 4 {
        return (state, consumed, crate::zlib_h::Z_DATA_ERROR);
    }
    if state.flags == -1 {
        state.wrap = 0;
    } else {
        state.wrap &= !4;
    }
    (state, consumed, crate::zlib_h::Z_OK)
}

pub unsafe extern "C" fn inflateSync(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm_ref) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((strm_ref, state)) = inflate_stream_and_state(strm_ref) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let input_len = strm_ref.avail_in as usize;
    let input = if input_len == 0 {
        &[][..]
    } else {
        ::core::slice::from_raw_parts(strm_ref.next_in, input_len)
    };
    let (flags, in_0, out) = {
        let sync_state = InflateSyncState {
            mode: state.normal.mode,
            hold: state.normal.hold,
            bits: state.normal.bits,
            have: state.normal.have,
            flags: state.normal.flags,
            wrap: state.normal.wrap,
        };
        let (sync_state, consumed, status) = inflate_sync_core(sync_state, input);
        state.normal.mode = sync_state.mode;
        state.normal.hold = sync_state.hold;
        state.normal.bits = sync_state.bits;
        state.normal.have = sync_state.have;
        state.normal.flags = sync_state.flags;
        state.normal.wrap = sync_state.wrap;
        strm_ref.avail_in = strm_ref
            .avail_in
            .wrapping_sub(consumed as crate::stdlib::uInt);
        strm_ref.next_in = strm_ref.next_in.wrapping_add(consumed);
        strm_ref.total_in = strm_ref
            .total_in
            .wrapping_add(consumed as crate::stdlib::uLong);
        if status != crate::zlib_h::Z_OK {
            return status;
        }
        (state.normal.flags, strm_ref.total_in, strm_ref.total_out)
    };
    let update = inflate_reset_core(&mut state.normal);
    strm_ref.total_out = 0;
    strm_ref.total_in = strm_ref.total_out;
    strm_ref.msg = ::core::ptr::null_mut();
    strm_ref.data_type = 0;
    if let Some(adler) = update.adler {
        strm_ref.adler = adler;
    }
    state.head = None;
    strm_ref.total_in = in_0;
    strm_ref.total_out = out;
    state.normal.flags = flags;
    state.normal.mode = crate::src::inflate::TYPE;
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

// The export boundary validates the ABI handle before this state adapter is
// entered.  Keeping the projection here makes the implementation's borrow
// explicitly live for the full state access without retaining a raw stream
// cursor in the core API.
pub unsafe fn inflateSyncPoint(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_int {
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_sync_point(state.normal.mode, state.normal.bits)
}
#[export_name = "inflateSyncPoint"]

pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateSyncPoint(strm)
}

pub unsafe extern "C" fn inflateCopy(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_identity = dest.addr();
    // Build the replacement before borrowing the destination.  This retains
    // C's behavior even for a source/destination alias while all state
    // access remains scoped to the checked source stream.
    let (copy, state_copy, destination_stream) = {
        let Some(source) = source.as_mut() else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Some((source, state)) = inflate_stream_and_state(source) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let copy = Some(source.zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            source.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
        ) as *mut crate::src::inflate::inflate_state;
        let Some(copy) = ::core::ptr::NonNull::new(copy) else {
            return crate::zlib_h::Z_MEM_ERROR;
        };
        // The ABI state has already been projected above.  Copy its ordinary
        // fields directly here so the deep-copy operation does not need a
        // separate unsafe helper carrying the raw-pointer-bearing state type.
        let owned_window = match state.normal.owned_window.as_deref() {
            Some(source_window) => {
                let Some(mut window) = allocate_inflate_window(source_window.len()) else {
                    Some(source.zfree.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        source.opaque, copy.as_ptr().cast()
                    );
                    return crate::zlib_h::Z_MEM_ERROR;
                };
                window[..state.normal.whave as usize]
                    .copy_from_slice(&source_window[..state.normal.whave as usize]);
                Some(window)
            }
            None => None,
        };
        let state_copy = inflate_state {
            stream_identity: dest_identity,
            head: state.head,
            window: state.window,
            normal: InflateNormalState {
                mode: state.normal.mode,
                last: state.normal.last,
                wrap: state.normal.wrap,
                havedict: state.normal.havedict,
                flags: state.normal.flags,
                dmax: state.normal.dmax,
                check: state.normal.check,
                total: state.normal.total,
                wbits: state.normal.wbits,
                wsize: state.normal.wsize,
                whave: state.normal.whave,
                wnext: state.normal.wnext,
                owned_window,
                hold: state.normal.hold,
                bits: state.normal.bits,
                length: state.normal.length,
                offset: state.normal.offset,
                extra: state.normal.extra,
                lencode: state.normal.lencode,
                distcode: state.normal.distcode,
                lenbits: state.normal.lenbits,
                distbits: state.normal.distbits,
                ncode: state.normal.ncode,
                nlen: state.normal.nlen,
                ndist: state.normal.ndist,
                have: state.normal.have,
                next: state.normal.next,
                lens: state.normal.lens,
                work: state.normal.work,
                codes: core::array::from_fn(|index| {
                    crate::src::inftrees::code::copied_from(&state.normal.codes[index])
                }),
                sane: state.normal.sane,
                back: state.normal.back,
                was: state.normal.was,
            },
        };
        let destination_stream = crate::zlib_h::z_stream_s {
            next_in: source.next_in,
            avail_in: source.avail_in,
            total_in: source.total_in,
            next_out: source.next_out,
            avail_out: source.avail_out,
            total_out: source.total_out,
            msg: source.msg,
            state: Some(::core::ptr::NonNull::from(copy).cast()),
            zalloc: source.zalloc,
            zfree: source.zfree,
            opaque: source.opaque,
            data_type: source.data_type,
            adler: source.adler,
            reserved: source.reserved,
        };
        (copy, state_copy, destination_stream)
    };
    // zalloc() returns uninitialized storage.  Publish a fully initialized
    // state in one write, then mirror the source stream exactly with only its
    // opaque state handle changed.
    copy.as_ptr().write(state_copy);
    *dest = destination_stream;
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

pub unsafe fn inflateUndermine(
    strm: &mut crate::zlib_h::z_stream_s,
    _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_undermine_sane(&mut state.normal.sane)
}
#[export_name = "inflateUndermine"]

pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
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
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_validate_wrap(&mut state.normal.wrap, check)
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

// The export boundary validates the nullable stream handle.  The named
// implementation keeps the stream-bound opaque-state projection together
// with the scalar query, so the wrapper remains a conversion and dispatch.
pub unsafe fn inflateMark(strm: &mut crate::zlib_h::z_stream_s) -> ::core::ffi::c_long {
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    };
    inflate_mark_value(
        state.normal.back,
        state.normal.mode,
        state.normal.length,
        state.normal.was,
    )
}
#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    let Some(strm) = strm.as_mut() else {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    };
    inflateMark(strm)
}

fn inflate_codes_used(next: usize) -> ::core::ffi::c_ulong {
    next as ::core::ffi::c_ulong
}

pub unsafe extern "C" fn inflateCodesUsed(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    let Some(strm) = strm.as_mut() else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    inflate_codes_used(state.normal.next)
}
#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    inflateCodesUsed(strm)
}
