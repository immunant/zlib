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

// The one-shot decoder keeps the zlib wrapper, bounded caller borrows, and
// uInt-sized request accounting entirely in pointer-free state.
pub(crate) fn inflate_one_shot(
    owner: &mut InflateOneShotOwner<'_, '_>,
) -> Result<InflateOneShotProgress, ::core::ffi::c_int> {
    let max = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    // One-shot decoding has no ABI-visible stream and never registers a
    // header, so it can use the same pointer-free bounded decoder that gzip
    // embeds.  This preserves the zlib wrapper selection while leaving each
    // caller buffer borrowed only for its individual decoder request.
    let mut decoder = InflateGzipOwner::with_window_bits(crate::zutil_h::DEF_WBITS);
    let mut input_offset = 0usize;
    let mut output_offset = 0usize;
    let mut input_available = 0 as crate::stdlib::uInt;
    let mut output_available = 0 as crate::stdlib::uInt;
    let status;
    loop {
        if output_available == 0 {
            output_available = owner.next_output_chunk(max);
        }
        if input_available == 0 {
            input_available = owner.next_input_chunk(max);
        }
        let (next_input_available, next_output_available, next_status) = {
            let input_end = input_offset + input_available as usize;
            let output_end = output_offset + output_available as usize;
            let result = decoder.inflate(
                &owner.input[input_offset..input_end],
                &mut owner.output[output_offset..output_end],
            );
            (
                result.input_remaining,
                result.output_remaining,
                result.status,
            )
        };
        input_offset += (input_available - next_input_available) as usize;
        output_offset += (output_available - next_output_available) as usize;
        input_available = next_input_available;
        output_available = next_output_available;
        if next_status != crate::zlib_h::Z_OK {
            status = next_status;
            break;
        }
    }
    Ok(InflateOneShotProgress {
        status,
        source_remaining: owner
            .input_remaining
            .wrapping_add(input_available as crate::stdlib::z_size_t),
        output_remaining: owner
            .output_remaining
            .wrapping_add(output_available as crate::stdlib::z_size_t),
    })
}

#[repr(C)]
pub struct inflate_state {
    // Keep the stream association check without retaining a raw backlink in
    // the codec state.  This is an identity token only; stream access is
    // always supplied by the caller.
    pub stream_identity: usize,
    // Header registration remains an ABI-boundary handle.  Back-mode's
    // workspace, on the other hand, is owned here: the caller-supplied
    // window establishes the API mode but is never read before back-mode
    // produces each byte, so it need not remain a persistent foreign borrow.
    pub head: Option<::core::ptr::NonNull<crate::zlib_h::gz_header_s>>,
    pub(crate) back_window: Option<InflateBackWindow>,
    // The normal codec and its scalar completion form one pointer-free
    // owner.  The callback-owned record retains only the two persistent ABI
    // registrations plus this owner; later stream adapters can hand the
    // owner directly to bounded decoder requests without rebuilding a
    // second scalar snapshot.
    pub(crate) decoder: InflateOwnedDecoder,
}

// `inflateBackInit_()` receives a caller-owned work area, but inflateBack's
// decoder always starts with an empty history and fills that area before any
// byte is consumed from it.  Retaining an owned, zeroed workspace therefore
// removes the cross-call foreign borrow without manufacturing a long-lived
// reference to the caller buffer.  The full maximum window is retained so
// the configured `wsize` stays a bounded prefix selected at call time.
#[derive(Clone)]
pub(crate) struct InflateBackWindow {
    pub(crate) bytes: Box<[u8; 32768]>,
}

impl InflateBackWindow {
    pub(crate) fn new() -> Self {
        Self {
            bytes: Box::new([0; 32768]),
        }
    }
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

// Construct the normal decoder payload independently of the callback-owned
// `inflate_state` record.  Gzip's embedded inflater needs this exact
// pointer-free payload as its eventual persistent codec owner; the ABI init
// boundary still chooses where the enclosing record is allocated.
fn initial_inflate_normal_state() -> InflateNormalState {
    InflateNormalState {
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
    }
}

impl InflateNormalState {
    // The fast decoder borrows only the normal decoder's owned history and
    // tables.  Keep that snapshot construction on the pointer-free state so
    // every caller uses the same selector, masks, and history policy before
    // it enters the bounded fast request.
    pub(crate) fn fast_state(&self) -> crate::src::inffast::InflateFastState<'_> {
        crate::src::inffast::InflateFastState {
            history: crate::src::inffast::FastHistory::External(self.owned_window.as_deref()),
            wsize: self.wsize as usize,
            whave: self.whave as usize,
            wnext: self.wnext as usize,
            hold: self.hold,
            bits: self.bits,
            lcode: self.lencode,
            dcode: self.distcode,
            lmask: (1u32 << self.lenbits) - 1,
            dmask: (1u32 << self.distbits) - 1,
            codes: &self.codes,
            sane: self.sane != 0,
        }
    }

    // A bounded fast request returns its cursor update separately.  Apply
    // only the resumable decoder portion here; callers still own diagnostic
    // publication in their respective stream/core completion boundaries.
    pub(crate) fn apply_fast_update(
        &mut self,
        update: &crate::src::inffast::InflateFastStreamUpdate,
    ) {
        self.hold = update.hold;
        self.bits = update.bits;
        match update.exit {
            crate::src::inffast::FastExit::Continue => {}
            crate::src::inffast::FastExit::Type => self.mode = crate::src::inflate::TYPE,
            crate::src::inffast::FastExit::InvalidDistance
            | crate::src::inffast::FastExit::InvalidCode => {
                self.mode = crate::src::inflate::BAD;
            }
        }
    }
}

// This owns only the normal decoder payload, which is already free of raw
// registrations.  It is intentionally not installed in gzip yet: callers
// still use the ABI adapter until the bounded inflate-call core can consume
// this owner directly.
// This is the persistent owner for every decoder that has no ABI cursor or
// foreign header registration.  Gzip and the one-shot APIs already use it;
// keeping the normal state and its scalar stream snapshot together makes it
// the pointer-free counterpart to the remaining ABI stream/state adapter.
//
// The owner deliberately has no callback allocation handle.  Normal inflate
// still keeps that provenance at `inflate_stream_and_state()` until the ABI
// state record can be replaced as one complete lifecycle.
pub(crate) struct InflateOwnedDecoder {
    pub(crate) normal: InflateNormalState,
    stream: InflateDecoderStream,
}

// Retain the gzip-facing name while gzip owns the decoder.  The owner itself
// is intentionally codec-generic so the ABI stream adapter can eventually
// hand off the same pointer-free state without a second representation.
pub(crate) type InflateGzipOwner = InflateOwnedDecoder;

// Gzip owns its inflater for the lifetime of the opaque gzip handle, so it
// never needs the public `z_stream` cursor or a registered header.  Expose
// exactly the bounded-call completion that gzip needs to retain that owner.
pub(crate) struct InflateGzipResult {
    pub(crate) status: ::core::ffi::c_int,
    pub(crate) input_remaining: crate::stdlib::uInt,
    pub(crate) output_remaining: crate::stdlib::uInt,
    pub(crate) total_in: crate::stdlib::uLong,
    pub(crate) total_out: crate::stdlib::uLong,
    pub(crate) data_error_message: Option<&'static [u8]>,
}

impl InflateOwnedDecoder {
    pub(crate) fn from_normal(normal: InflateNormalState) -> Self {
        Self {
            normal,
            stream: InflateDecoderStream {
                total_in: 0,
                total_out: 0,
                adler: 0,
                data_type: 0,
                message: None,
            },
        }
    }

    pub(crate) fn new() -> Self {
        Self::with_window_bits(15 + 16)
    }

    // Both gzip's embedded decoder and the temporary one-shot zlib decoder
    // own only normal decoder state.  Keep wrapper selection in this
    // pointer-free constructor so their bounded-call behavior remains one
    // implementation.
    fn with_window_bits(window_bits: ::core::ffi::c_int) -> Self {
        let mut normal = initial_inflate_normal_state();
        let reset = inflate_reset2_normal(&mut normal, window_bits)
            .expect("one-shot and gzip window bits are valid");
        Self {
            normal,
            stream: InflateDecoderStream {
                total_in: 0,
                total_out: 0,
                adler: reset.adler.unwrap_or(0),
                data_type: 0,
                message: None,
            },
        }
    }

    pub(crate) fn reset(&mut self) {
        let reset = inflate_reset_core(&mut self.normal);
        self.stream.total_in = 0;
        self.stream.total_out = 0;
        self.stream.data_type = 0;
        self.stream.message = None;
        if let Some(adler) = reset.adler {
            self.stream.adler = adler;
        }
    }

    pub(crate) fn inflate(&mut self, input: &[u8], output: &mut [u8]) -> InflateGzipResult {
        // The ABI adapter clears `strm.msg` before each invocation.  The
        // persistent owner keeps scalar counters between calls, but a prior
        // diagnostic must not be reported again after a later successful
        // bounded request.
        self.stream.message = None;
        let result = InflateStreamOwner {
            normal: &mut self.normal,
            input,
            output,
            header: None,
            stream: &mut self.stream,
            flush: crate::zlib_h::Z_NO_FLUSH,
        }
        .run();
        InflateGzipResult {
            status: result.status,
            input_remaining: result.cursor.input_remaining,
            output_remaining: result.cursor.output_remaining,
            total_in: self.stream.total_in,
            total_out: self.stream.total_out,
            data_error_message: self.stream.message.and_then(|message| match message {
                InflateMessage::Error(index) => {
                    Some(&INFLATE_ERROR_MESSAGES[index][..INFLATE_ERROR_MESSAGES[index].len() - 1])
                }
                // The former gzip adapter recognized only the stable table
                // entries above; retain its generic gzip diagnostic for this
                // non-table stream message.
                InflateMessage::InvalidCode => None,
            }),
        }
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
        || (state.decoder.normal.mode as ::core::ffi::c_uint)
            < crate::src::inflate::HEAD as ::core::ffi::c_int as ::core::ffi::c_uint
        || state.decoder.normal.mode as ::core::ffi::c_uint
            > crate::src::inflate::SYNC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return None;
    }
    Some((stream, state))
}

struct InflateResetUpdate {
    adler: Option<crate::stdlib::uLong>,
}

// The requested wrapper policy and the normal decoder's initial value are
// entirely pointer-free.  Keep them together while the callback transaction
// below decides whether that value is published or released.  In particular,
// an invalid window request still reaches the callback allocation boundary,
// matching zlib's allocation-then-reset lifecycle.
struct InflateInitOwner {
    normal: InflateNormalState,
    reset: Result<InflateResetUpdate, ::core::ffi::c_int>,
}

impl InflateInitOwner {
    fn new(window_bits: ::core::ffi::c_int) -> Self {
        let mut normal = initial_inflate_normal_state();
        let reset = inflate_reset2_normal(&mut normal, window_bits);
        Self { normal, reset }
    }
}

// This is the one callback-paired initialization transaction.  It keeps the
// allocator result's provenance at the boundary until exactly one complete
// state publication or matching zfree.  The owner it receives contains no
// callback handle, stream pointer, or foreign registration.
unsafe fn inflate_publish_callback_owner(
    strm: &mut crate::zlib_h::z_stream_s,
    window_bits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // Keep the caller's stream projection at the allocator boundary. The
    // callback-owned state is published only after it has been fully
    // initialized below, since zalloc() need not return initialized bytes.
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
    // Retain the callback result with its provenance while initialization is
    // in flight.  The raw allocation is not exposed to the pointer-free
    // decoder plan; it is consumed only by the paired publication/release
    // boundary below.
    let Some(state) = ::core::ptr::NonNull::new(
        Some(strm.zalloc.expect("non-null function pointer")).expect("non-null function pointer")(
            strm.opaque,
            1 as crate::stdlib::uInt,
            ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
        )
        .cast::<crate::src::inflate::inflate_state>(),
    ) else {
        return crate::zlib_h::Z_MEM_ERROR;
    };
    // Preserve the callback ordering: allocation comes first, then the
    // pointer-free initialization plan is built and published as one value.
    // A failing plan therefore still has a matching callback release below.
    let owner = InflateInitOwner::new(window_bits);
    // Publish one complete value into the callback-owned allocation.  Writing
    // fields piecemeal here would briefly treat uninitialized callback bytes
    // as Rust fields with drop glue.
    ::core::ptr::write(
        state.as_ptr(),
        crate::src::inflate::inflate_state {
            stream_identity: ::core::ptr::from_mut(strm).addr(),
            head: None,
            back_window: None,
            decoder: InflateOwnedDecoder::from_normal(owner.normal),
        },
    );
    strm.state = Some(state.cast());
    let update = match owner.reset {
        Ok(update) => update,
        Err(status) => {
            Some(strm.zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(strm.opaque, state.as_ptr().cast());
            strm.state = None;
            return status;
        }
    };
    strm.total_out = 0;
    strm.total_in = strm.total_out;
    strm.msg = ::core::ptr::null_mut();
    strm.data_type = 0;
    if let Some(adler) = update.adler {
        strm.adler = adler;
    }
    crate::zlib_h::Z_OK
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

// Window-bit normalization and normal-decoder reset are independent of an
// ABI stream.  Both the exported reset adapter and the future embedded gzip
// owner use this one policy, leaving only scalar stream publication at the
// ABI boundary.
fn inflate_reset2_normal(
    normal: &mut InflateNormalState,
    mut window_bits: ::core::ffi::c_int,
) -> Result<InflateResetUpdate, ::core::ffi::c_int> {
    let wrap;
    if window_bits < 0 {
        if window_bits < -15 {
            return Err(crate::zlib_h::Z_STREAM_ERROR);
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
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    if normal.owned_window.is_some() && normal.wbits != window_bits as ::core::ffi::c_uint {
        normal.owned_window = None;
    }
    normal.wrap = wrap;
    normal.wbits = window_bits as ::core::ffi::c_uint;
    Ok(inflate_reset_core(normal))
}

// The reset exports differ only in which pointer-free normal-state policy is
// selected.  Keep their common stream/state projection and scalar publication
// in this one adapter so each ABI wrapper remains a handle conversion and
// dispatch only.
pub(crate) enum InflateResetKind {
    Keep,
    Full,
    WindowBits(::core::ffi::c_int),
}

// Reset policy is independent of ABI cursor provenance.  This owner receives
// the normal decoder plus the scalar stream snapshot that reset is allowed to
// change, so the policy can be reused without reopening a stream/state
// association.
struct InflateResetOwner<'state, 'stream> {
    normal: &'state mut InflateNormalState,
    stream: &'stream mut InflateDecoderStream,
}

// Header registration remains an ABI-boundary concern, but spelling the
// release of that registration as a scalar completion keeps the reset core
// from retaining the foreign handle that backs it.
struct InflateResetCompletion {
    clear_header_registration: bool,
}

impl InflateResetOwner<'_, '_> {
    fn new<'normal, 'stream>(
        normal: &'normal mut InflateNormalState,
        stream: &'stream mut InflateDecoderStream,
    ) -> InflateResetOwner<'normal, 'stream> {
        InflateResetOwner { normal, stream }
    }

    fn reset(self, kind: InflateResetKind) -> Result<InflateResetUpdate, ::core::ffi::c_int> {
        let update = match kind {
            InflateResetKind::Keep => Ok(inflate_reset_keep_core(self.normal)),
            InflateResetKind::Full => Ok(inflate_reset_core(self.normal)),
            InflateResetKind::WindowBits(window_bits) => {
                inflate_reset2_normal(self.normal, window_bits)
            }
        }?;
        self.stream.total_out = 0;
        self.stream.total_in = self.stream.total_out;
        self.stream.message = None;
        self.stream.data_type = 0;
        if let Some(adler) = update.adler {
            self.stream.adler = adler;
        }
        Ok(update)
    }
}

// This is the pointer-free reset transaction.  It owns both reset policy and
// scalar publication, returning only the one ABI-boundary action that cannot
// be expressed without the retained header registration.
fn inflate_reset_from_stream(
    owner: InflateResetOwner<'_, '_>,
    kind: InflateResetKind,
) -> Result<InflateResetCompletion, ::core::ffi::c_int> {
    owner.reset(kind)?;
    Ok(InflateResetCompletion {
        clear_header_registration: true,
    })
}

#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_from_stream(strm, InflateStreamRequest::Reset(InflateResetKind::Keep)).status()
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_from_stream(strm, InflateStreamRequest::Reset(InflateResetKind::Full)).status()
}
#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_from_stream(
        strm,
        InflateStreamRequest::Reset(InflateResetKind::WindowBits(windowBits)),
    )
    .status()
}
pub unsafe extern "C" fn inflateInit2_(
    strm: Option<&mut crate::zlib_h::z_stream_s>,
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
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_publish_callback_owner(strm, windowBits)
}
#[export_name = "inflateInit2_"]

pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit2_(strm.as_mut(), windowBits, version, stream_size)
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateInit2_(
        strm.as_mut(),
        crate::zutil_h::DEF_WBITS,
        version,
        stream_size,
    )
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

#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_from_stream(
        strm,
        InflateStreamRequest::Scalar(InflateNormalScalarAction::Prime { bits, value }),
    )
    .scalar()
    .status()
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

fn inflate_get_dictionary(
    normal: &InflateNormalState,
    dictionary: Option<&mut [crate::stdlib::Bytef]>,
    dict_length: Option<&mut crate::stdlib::uInt>,
) -> ::core::ffi::c_int {
    let whave = normal.whave as usize;
    if let Some(output) = dictionary {
        if whave != 0 {
            let window = normal
                .owned_window
                .as_deref()
                .expect("normal inflate owns its history window");
            copy_history_dictionary(output, window, normal.wnext as usize, whave);
        }
    }
    if let Some(dict_length) = dict_length {
        *dict_length = normal.whave;
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

// A completed decoder invocation describes cursor movement only as bounded
// lengths.  The stream adapter publishes those lengths to the ABI pointers
// after the decoder has released its input and output slices; decoder policy
// must never retain an ABI cursor shadow.
#[derive(Clone, Copy)]
struct InflateDecoderCursor {
    input_used: usize,
    input_remaining: ::core::ffi::c_uint,
    output_used: usize,
    output_remaining: ::core::ffi::c_uint,
}

impl InflateDecoderCursor {
    fn from_remaining(
        input_len: usize,
        input_remaining: ::core::ffi::c_uint,
        output_len: usize,
        output_remaining: ::core::ffi::c_uint,
    ) -> Self {
        let input_remaining = input_remaining.min(input_len as ::core::ffi::c_uint);
        let output_remaining = output_remaining.min(output_len as ::core::ffi::c_uint);
        Self {
            input_used: input_len - input_remaining as usize,
            input_remaining,
            output_used: output_len - output_remaining as usize,
            output_remaining,
        }
    }
}

// The normal decoder communicates with its ABI adapter only through these
// scalar values.  In particular, diagnostics stay as a stable selection
// rather than a raw C-string pointer until the bounded request has ended.
#[derive(Clone, Copy)]
enum InflateMessage {
    Error(usize),
    InvalidCode,
}

struct InflateDecoderStream {
    total_in: crate::stdlib::uLong,
    total_out: crate::stdlib::uLong,
    adler: crate::stdlib::uLong,
    data_type: ::core::ffi::c_int,
    message: Option<InflateMessage>,
}

struct InflateDecoderResult {
    status: ::core::ffi::c_int,
    cursor: InflateDecoderCursor,
    publication: Option<HeaderPublication>,
}

// Both the public bounded decoder and the exported fast-path symbol need the
// same stream/state association.  Keep their selector pointer-free so that
// all ABI cursor construction and publication remains at this one adapter.
pub(crate) enum InflateStreamRequest<'request> {
    Decode(::core::ffi::c_int),
    Fast(::core::ffi::c_uint),
    Sync,
    Reset(InflateResetKind),
    Scalar(InflateNormalScalarAction),
    Dictionary {
        dictionary: Option<&'request mut [crate::stdlib::Bytef]>,
        dict_length: Option<&'request mut crate::stdlib::uInt>,
    },
    SetDictionary(&'request [crate::stdlib::Bytef]),
}

// The ABI adapter can service both streaming and scalar normal-inflate
// requests.  Keep the distinct scalar return types in this pointer-free
// result so exported wrappers only dispatch and extract their ABI result.
pub(crate) enum InflateStreamResult {
    Status(::core::ffi::c_int),
    Scalar(InflateNormalScalarResult),
}

impl InflateStreamResult {
    fn status(self) -> ::core::ffi::c_int {
        match self {
            Self::Status(status) => status,
            Self::Scalar(result) => result.status(),
        }
    }

    fn scalar(self) -> InflateNormalScalarResult {
        match self {
            Self::Scalar(result) => result,
            Self::Status(_) => unreachable!("scalar request must return a scalar result"),
        }
    }
}

// One normal-inflate dispatch owns every value the decoder is allowed to
// observe: the resumable Rust state, bounded caller cursors, the scoped
// header-output facade, and scalar stream accounting.  It deliberately does
// not retain the ABI stream, the opaque state handle, or the registered
// header pointer.  The stream adapter constructs this request only after all
// three foreign projections have been checked, and consumes its completion
// before publishing any ABI cursor or header changes.
struct InflateStreamOwner<'normal, 'input, 'output, 'header, 'stream> {
    normal: &'normal mut InflateNormalState,
    input: &'input [u8],
    output: &'output mut [u8],
    header: Option<InflateHeaderOutput<'header>>,
    stream: &'stream mut InflateDecoderStream,
    flush: ::core::ffi::c_int,
}

impl InflateStreamOwner<'_, '_, '_, '_, '_> {
    fn run(self) -> InflateDecoderResult {
        inflate(self)
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

// The normal decoder is pointer-free: it consumes a bounded request and
// returns scalar cursor/header publication before the ABI adapter republishes
// either.  The ABI-shaped adapter deliberately has a distinct name, so this
// remains the implementation entry point for bounded decoder requests.
fn inflate(request: InflateStreamOwner<'_, '_, '_, '_, '_>) -> InflateDecoderResult {
    let InflateStreamOwner {
        normal,
        input,
        output,
        mut header,
        stream,
        mut flush,
    } = request;
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
    let mut completion_cursor = None;
    let result = '_inflate_result: {
        if normal.mode as ::core::ffi::c_uint
            == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            normal.mode = crate::src::inflate::TYPEDO;
        }
        left = output.len() as ::core::ffi::c_uint;
        let mut output_chunk_start = 0usize;
        have = input.len() as ::core::ffi::c_uint;
        hold = normal.hold;
        bits = normal.bits;
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
                                                                                                match normal.mode as ::core::ffi::c_uint {
                                                                                                16180 => {
                                                                                                    if normal.wrap == 0 as ::core::ffi::c_int {
                                                                                                        normal.mode = crate::src::inflate::TYPEDO;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                        }
                                                                                                        if normal.wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if normal.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                normal.wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            normal.check = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            normal.check = crate::src::crc32::crc32_z(
                                                                                                                normal.check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            normal.mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                            if let Some(head) = header.as_mut() {
                                                                                                                head.set_done(-1 as ::core::ffi::c_int);
                                                                                                            }
                                                                                                            if normal.wrap & 1 as ::core::ffi::c_int == 0
                                                                                                                || (((hold as ::core::ffi::c_uint
                                                                                                                    & ((1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                                                                                                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                                                                                                                    .wrapping_add(hold >> 8 as ::core::ffi::c_int)
                                                                                                                    .wrapping_rem(31 as ::core::ffi::c_ulong) != 0
                                                                                                            {
                                                                                                                stream.message = Some(InflateMessage::Error(0));
                                                                                                                normal.mode = crate::src::inflate::BAD;
                                                                                                                continue '_inf_leave;
                                                                                                            } else if hold as ::core::ffi::c_uint
                                                                                                                & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                                                                                                != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint
                                                                                                            {
                                                                                                                stream.message = Some(InflateMessage::Error(1));
                                                                                                                normal.mode = crate::src::inflate::BAD;
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
                                                                                                                if normal.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                    normal.wbits = len;
                                                                                                                }
                                                                                                                if len > 15 as ::core::ffi::c_uint || len > normal.wbits {
                                                                                                                    stream.message = Some(InflateMessage::Error(2));
                                                                                                                    normal.mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                } else {
                                                                                                                    normal.dmax = (1 as ::core::ffi::c_uint) << len;
                                                                                                                    normal.flags = 0 as ::core::ffi::c_int;
                                                                                                                    normal.check = crate::src::adler32::adler32_z(
                                                                                                                        0 as crate::stdlib::uLong,
                                                                                                                        None,
                                                                                                                    ) as ::core::ffi::c_ulong;
                                                                                                                    stream.adler = normal.check as crate::stdlib::uLong;
                                                                                                                    normal.mode = (if hold & 0x200 as ::core::ffi::c_ulong
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
                                                                                                    normal.flags = hold as ::core::ffi::c_int;
                                                                                                    if normal.flags & 0xff as ::core::ffi::c_int != crate::zlib_h::Z_DEFLATED
                                                                                                    {
                                                                                                        stream.message = Some(InflateMessage::Error(1));
                                                                                                        normal.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else if normal.flags & 0xe000 as ::core::ffi::c_int != 0
                                                                                                    {
                                                                                                        stream.message = Some(InflateMessage::Error(3));
                                                                                                        normal.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        if let Some(head) = header.as_mut() {
                                                                                                            head.set_text((hold >> 8 as ::core::ffi::c_int
                                                                                                                & 1 as ::core::ffi::c_ulong) as ::core::ffi::c_int);
                                                                                                        }
                                                                                                        if normal.flags & 0x200 as ::core::ffi::c_int != 0
                                                                                                            && normal.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                        {
                                                                                                            hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            normal.check = crate::src::crc32::crc32_z(
                                                                                                                normal.check as crate::stdlib::uLong,
                                                                                                                Some(&hbuf[..2]),
                                                                                                            ) as ::core::ffi::c_ulong;
                                                                                                        }
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        normal.mode = crate::src::inflate::TIME;
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
                                                                                                    normal.check = (hold >> 24 as ::core::ffi::c_int
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
                                                                                                    stream.adler = normal.check as crate::stdlib::uLong;
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                    normal.mode = crate::src::inflate::DICT;
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
                                                                                                        stream.message = Some(InflateMessage::Error(4));
                                                                                                        normal.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        normal.length = hold as ::core::ffi::c_uint
                                                                                                            & 0xffff as ::core::ffi::c_uint;
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        normal.mode = crate::src::inflate::COPY_;
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
                                                                                                    normal.nlen = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(257 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    normal.ndist = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(1 as ::core::ffi::c_uint);
                                                                                                    hold >>= 5 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            5 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    normal.ncode = (hold as ::core::ffi::c_uint
                                                                                                        & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                                                                                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                        .wrapping_add(4 as ::core::ffi::c_uint);
                                                                                                    hold >>= 4 as ::core::ffi::c_int;
                                                                                                    bits = bits
                                                                                                        .wrapping_sub(
                                                                                                            4 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                        );
                                                                                                    if normal.nlen > 286 as ::core::ffi::c_uint
                                                                                                        || normal.ndist > 30 as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        stream.message = Some(InflateMessage::Error(5));
                                                                                                        normal.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        normal.have = 0 as ::core::ffi::c_uint;
                                                                                                        normal.mode = crate::src::inflate::LENLENS;
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
                                                                                                    output[output_index] = normal.length
                                                                                                        as ::core::ffi::c_uchar;
                                                                                                    left = left.wrapping_sub(1);
                                                                                                    normal.mode = crate::src::inflate::LEN;
                                                                                                    continue '_inf_leave;
                                                                                                }
                                                                                                16206 => {
                                                                                                    if normal.wrap != 0 {
                                                                                                        while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                        }
                                                                                                        out = out.wrapping_sub(left);
                                                                                                        stream.total_out = stream
                                                                                                            .total_out
                                                                                                            .wrapping_add(out as crate::stdlib::uLong);
                                                                                                        normal.total = normal
                                                                                                            .total
                                                                                                            .wrapping_add(out as ::core::ffi::c_ulong);
                                                                                                        if normal.wrap & 4 as ::core::ffi::c_int != 0 && out != 0
                                                                                                        {
                                                                                                            let produced = &output[output_chunk_start
                                                                                                                ..output_chunk_start + out as usize];
                                                                                                            normal.check = (if normal.flags != 0 {
                                                                                                                crate::src::crc32::crc32_z(
                                                                                                                    normal.check as crate::stdlib::uLong,
                                                                                                                    Some(produced),
                                                                                                                )
                                                                                                            } else {
                                                                                                                crate::src::adler32::adler32(
                                                                                                                    normal.check as crate::stdlib::uLong,
                                                                                                                    produced,
                                                                                                                )
                                                                                                            }) as ::core::ffi::c_ulong;
                                                                                                            stream.adler = normal.check as crate::stdlib::uLong;
                                                                                                        }
                                                                                                        out = left;
                                                                                                        // Subsequent output is a new checksum
                                                                                                        // chunk in the caller's bounded range.
                                                                                                        output_chunk_start = output.len() - left as usize;
                                                                                                        if normal.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                            && (if normal.flags != 0 {
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
                                                                                                            }) != normal.check
                                                                                                        {
                                                                                                            stream.message = Some(InflateMessage::Error(6));
                                                                                                            normal.mode = crate::src::inflate::BAD;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                        }
                                                                                                    }
                                                                                                    normal.mode = crate::src::inflate::LENGTH;
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
                                                                                                if normal.wrap != 0 && normal.flags != 0 {
                                                                                                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                {
                                                                                                    if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                }
                                                                                                if normal.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                    && hold
                                                                                                        != normal.total & 0xffffffff as ::core::ffi::c_ulong
                                                                                                {
                                                                                                    stream.message = Some(InflateMessage::Error(7));
                                                                                                    normal.mode = crate::src::inflate::BAD;
                                                                                                    continue '_inf_leave;
                                                                                                } else {
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                }
                                                                                            }
                                                                                                normal.mode = crate::src::inflate::DONE;
                                                                                                break 'c_2443;
                                                                                            }
                                                                                            while normal.have < normal.ncode {
                                                                                            while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                            {
                                                                                                if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                            }
                                                                                            let c2rust_fresh15 = normal.have;
                                                                                            normal.have = normal.have.wrapping_add(1);
                                                                                            normal.lens[order[c2rust_fresh15 as usize] as usize] = (hold
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
                                                                                            while normal.have < 19 as ::core::ffi::c_uint {
                                                                                            let c2rust_fresh16 = normal.have;
                                                                                            normal.have = normal.have.wrapping_add(1);
                                                                                            normal.lens[order[c2rust_fresh16 as usize] as usize] = 0
                                                                                                as ::core::ffi::c_ushort;
                                                                                        }
                                                                                            normal
                                                                                            .next =
                                                                                            0;
                                                                                            normal.distcode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                                            normal.lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                                            normal.lenbits = 7 as ::core::ffi::c_uint;
                                                                                            ret = 'table: {
                                                                                                let table_start = normal.next;
                                                                                                let Some(lens) = normal.lens.get(..19) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                                let Some(table) = normal.codes.get_mut(table_start..) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                                let Some(work) = normal.work.get_mut(..19) else {
                                                                                                break 'table 1;
                                                                                            };
                                                                                                let (status, used) = crate::src::inftrees::inflate_table(
                                                                                                crate::src::inftrees::CODES,
                                                                                                lens,
                                                                                                table,
                                                                                                &mut normal.lenbits,
                                                                                                work,
                                                                                            );
                                                                                                if status == 0 {
                                                                                                normal.next += used;
                                                                                            }
                                                                                                status
                                                                                            };
                                                                                            if ret
                                                                                                != 0
                                                                                            {
                                                                                                stream.message = Some(InflateMessage::Error(8));
                                                                                                normal.mode = crate::src::inflate::BAD;
                                                                                                continue '_inf_leave;
                                                                                            } else {
                                                                                                normal.have = 0 as ::core::ffi::c_uint;
                                                                                                normal.mode = crate::src::inflate::CODELENS;
                                                                                                break 's_1689;
                                                                                            }
                                                                                        }
                                                                                        if normal.havedict == 0 as ::core::ffi::c_int {
                                                                                        normal.hold = hold;
                                                                                        normal.bits = bits;
                                                                                        completion_cursor = Some(InflateDecoderCursor::from_remaining(
                                                                                            input.len(), have, output.len(), left,
                                                                                        ));
                                                                                        break '_inflate_result crate::zlib_h::Z_NEED_DICT;
                                                                                    }
                                                                                        normal.check = crate::src::adler32::adler32_z(
                                                                                        0 as crate::stdlib::uLong,
                                                                                        None,
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                        stream.adler =
                                                                                        normal
                                                                                            .check
                                                                                            as crate::stdlib::uLong;
                                                                                        normal.mode =
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
                                                                                    if normal.flags & 0x200 as ::core::ffi::c_int != 0
                                                                                    && normal.wrap & 4 as ::core::ffi::c_int != 0
                                                                                {
                                                                                    hbuf[0 as usize] = hold as ::core::ffi::c_uchar;
                                                                                    hbuf[1 as usize] = (hold >> 8 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[2 as usize] = (hold >> 16 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    hbuf[3 as usize] = (hold >> 24 as ::core::ffi::c_int)
                                                                                        as ::core::ffi::c_uchar;
                                                                                    normal.check = crate::src::crc32::crc32_z(
                                                                                        normal.check as crate::stdlib::uLong,
                                                                                        Some(&hbuf[..4]),
                                                                                    ) as ::core::ffi::c_ulong;
                                                                                }
                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                    normal.mode = crate::src::inflate::OS;
                                                                                    break 's_519;
                                                                                }
                                                                                normal.mode = crate::src::inflate::COPY_1;
                                                                                break 'c_2356;
                                                                            }
                                                                            while normal.have
                                                                                < normal
                                                                                    .nlen
                                                                                    .wrapping_add(
                                                                                        normal
                                                                                            .ndist,
                                                                                    )
                                                                            {
                                                                                loop {
                                                                                    here = crate::src::inftrees::code::copied_from(normal
                                                                                    .lencode
                                                                                    .get(&normal.codes,
                                                                                        (hold as ::core::ffi::c_uint
                                                                                            & ((1 as ::core::ffi::c_uint) << normal.lenbits)
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
                                                                                let c2rust_fresh18 = normal.have;
                                                                                normal.have = normal.have.wrapping_add(1);
                                                                                normal.lens[c2rust_fresh18 as usize] = here.val;
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
                                                                                    if normal.have == 0 as ::core::ffi::c_uint {
                                                                                        stream.message = Some(InflateMessage::Error(9));
                                                                                        normal.mode = crate::src::inflate::BAD;
                                                                                        break;
                                                                                    } else {
                                                                                        len = normal
                                                                                            .lens[normal.have.wrapping_sub(1 as ::core::ffi::c_uint)
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
                                                                                if normal.have.wrapping_add(copy)
                                                                                    > normal.nlen.wrapping_add(normal.ndist)
                                                                                {
                                                                                    stream.message = Some(InflateMessage::Error(9));
                                                                                    normal.mode = crate::src::inflate::BAD;
                                                                                    break;
                                                                                } else {
                                                                                    loop {
                                                                                        let c2rust_fresh22 = copy;
                                                                                        copy = copy.wrapping_sub(1);
                                                                                        if c2rust_fresh22 == 0 {
                                                                                            break;
                                                                                        }
                                                                                        let c2rust_fresh23 = normal.have;
                                                                                        normal.have = normal.have.wrapping_add(1);
                                                                                        normal.lens[c2rust_fresh23 as usize] = len
                                                                                            as ::core::ffi::c_ushort;
                                                                                    }
                                                                                }
                                                                            }
                                                                            }
                                                                            if normal.mode as ::core::ffi::c_uint
                                                                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            continue '_inf_leave;
                                                                        }
                                                                            if normal.lens[256 as usize] as ::core::ffi::c_int
                                                                            == 0 as ::core::ffi::c_int
                                                                        {
                                                                            stream.message = Some(InflateMessage::Error(10));
                                                                            normal.mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            normal.next = 0;
                                                                            normal.lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                                                                            normal.lenbits = 9 as ::core::ffi::c_uint;
                                                                            ret = 'table: {let codes = normal.nlen as usize;
                                                                                let table_start = normal.next;
                                                                                let Some(lens) = normal.lens.get(..codes) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let Some(table) = normal.codes.get_mut(table_start..) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let Some(work) = normal.work.get_mut(..codes) else {
                                                                                    break 'table 1;
                                                                                };
                                                                                let (status, used) = crate::src::inftrees::inflate_table(
                                                                                    crate::src::inftrees::LENS,
                                                                                    lens,
                                                                                    table,
                                                                                    &mut normal.lenbits,
                                                                                    work,
                                                                                );
                                                                                if status == 0 {
                                                                                    normal.next += used;
                                                                                }
                                                                                status
                                                                            };
                                                                            if ret != 0 {
                                                                                stream.message = Some(InflateMessage::Error(11));
                                                                                normal.mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            } else {
                                                                                normal.distcode = crate::src::inflate::CodeTableRef::Dynamic(normal.next);
                                                                                normal.distbits = 6 as ::core::ffi::c_uint;
                                                                                ret = 'table: {let lens_start = normal.nlen as usize;
                                                                                    let codes = normal.ndist as usize;
                                                                                    let Some(lens_end) = lens_start.checked_add(codes) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let table_start = normal.next;
                                                                                    let Some(lens) = normal.lens.get(lens_start..lens_end) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let Some(table) = normal.codes.get_mut(table_start..) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let Some(work) = normal.work.get_mut(..codes) else {
                                                                                        break 'table 1;
                                                                                    };
                                                                                    let (status, used) = crate::src::inftrees::inflate_table(
                                                                                        crate::src::inftrees::DISTS,
                                                                                        lens,
                                                                                        table,
                                                                                        &mut normal.distbits,
                                                                                        work,
                                                                                    );
                                                                                    if status == 0 {
                                                                                        normal.next += used;
                                                                                    }
                                                                                    status
                                                                                };
                                                                                if ret != 0 {
                                                                                    stream.message = Some(InflateMessage::Error(12));
                                                                                    normal.mode = crate::src::inflate::BAD;
                                                                                    continue '_inf_leave;
                                                                                } else {
                                                                                    normal.mode = crate::src::inflate::LEN_;
                                                                                    if flush == crate::zlib_h::Z_TREES {
                                                                                        break '_inf_leave;
                                                                                    } else {
                                                                                        break 'c_2397;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        }
                                                                        copy = normal.length;
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
                                                                            normal.length = normal
                                                                                .length
                                                                                .wrapping_sub(copy);
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            normal.mode = crate::src::inflate::TYPE;
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
                                                                    if normal.flags
                                                                    & 0x200 as ::core::ffi::c_int
                                                                    != 0
                                                                    && normal.wrap
                                                                        & 4 as ::core::ffi::c_int
                                                                        != 0
                                                                {
                                                                    hbuf[0 as usize] = hold
                                                                        as ::core::ffi::c_uchar;
                                                                    hbuf[1 as usize] = (hold
                                                                        >> 8 as ::core::ffi::c_int)
                                                                        as ::core::ffi::c_uchar;
                                                                    normal.check = crate::src::crc32::crc32_z(
                                                                        normal.check as crate::stdlib::uLong,
                                                                        Some(&hbuf[..2]),
                                                                    ) as ::core::ffi::c_ulong;
                                                                }
                                                                    hold =
                                                                        0 as ::core::ffi::c_ulong;
                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                    normal.mode =
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
                                                        if normal.last != 0 {
                                                            hold >>=
                                                                bits & 7 as ::core::ffi::c_uint;
                                                            bits = bits.wrapping_sub(
                                                                bits & 7 as ::core::ffi::c_uint,
                                                            );
                                                            normal.mode =
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
                                                            normal.last = (hold
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
                                                                    normal.mode =
                                                                        crate::src::inflate::STORED;
                                                                }
                                                                1 => {
                                                                    crate::src::inftrees::inflate_fixed(
                                                                    &mut normal.lencode,
                                                                    &mut normal.lenbits,
                                                                    &mut normal.distcode,
                                                                    &mut normal.distbits,
                                                                );
                                                                    normal.mode =
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
                                                                    normal.mode =
                                                                        crate::src::inflate::TABLE;
                                                                }
                                                                _ => {
                                                                    stream.message = Some(
                                                                        InflateMessage::Error(13),
                                                                    );
                                                                    normal.mode =
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
                                                    if normal.flags & 0x400 as ::core::ffi::c_int
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
                                                        normal.length = hold as ::core::ffi::c_uint;
                                                        if let Some(head) = header.as_mut() {
                                                            head.set_extra_len(
                                                                hold as ::core::ffi::c_uint
                                                                    as crate::stdlib::uInt,
                                                            );
                                                        }
                                                        if normal.flags
                                                            & 0x200 as ::core::ffi::c_int
                                                            != 0
                                                            && normal.wrap & 4 as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            hbuf[0 as usize] =
                                                                hold as ::core::ffi::c_uchar;
                                                            hbuf[1 as usize] = (hold
                                                                >> 8 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uchar;
                                                            normal.check =
                                                                crate::src::crc32::crc32_z(
                                                                    normal.check
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
                                                    normal.mode = crate::src::inflate::EXTRA;
                                                    break 'c_2319;
                                                }
                                                normal.mode = crate::src::inflate::LEN;
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
                                                // The fast core receives only this bounded
                                                // normal-state view, never an ABI stream.
                                                normal.hold = hold;
                                                normal.bits = bits;
                                                let fast_state = normal.fast_state();
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
                                                normal.apply_fast_update(&result);
                                                match result.exit {
                                                crate::src::inffast::FastExit::Continue => {}
                                                crate::src::inffast::FastExit::Type => {}
                                                crate::src::inffast::FastExit::InvalidDistance => {
                                                    stream.message = Some(InflateMessage::Error(17));
                                                }
                                                crate::src::inffast::FastExit::InvalidCode => {
                                                    stream.message = Some(InflateMessage::InvalidCode);
                                                }
                                            }
                                                if normal.mode as ::core::ffi::c_uint
                                                    == crate::src::inflate::TYPE
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    normal.back = -1 as ::core::ffi::c_int;
                                                }
                                                continue '_inf_leave;
                                            } else {
                                                normal.back = 0 as ::core::ffi::c_int;
                                                loop {
                                                    here = crate::src::inftrees::code::copied_from(
                                                        normal.lencode.get(
                                                            &normal.codes,
                                                            (hold as ::core::ffi::c_uint
                                                                & ((1 as ::core::ffi::c_uint)
                                                                    << normal.lenbits)
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
                                                        here = crate::src::inftrees::code::copied_from(normal.lencode.get(&normal.codes,
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
                                                    normal.back += last.bits as ::core::ffi::c_int;
                                                }
                                                hold >>= here.bits as ::core::ffi::c_int;
                                                bits = bits
                                                    .wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                normal.back += here.bits as ::core::ffi::c_int;
                                                normal.length = here.val as ::core::ffi::c_uint;
                                                if here.op as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                {
                                                    normal.mode = crate::src::inflate::LIT;
                                                    continue '_inf_leave;
                                                } else if here.op as ::core::ffi::c_int
                                                    & 32 as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    normal.back = -1 as ::core::ffi::c_int;
                                                    normal.mode = crate::src::inflate::TYPE;
                                                    continue '_inf_leave;
                                                } else if here.op as ::core::ffi::c_int
                                                    & 64 as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    stream.message =
                                                        Some(InflateMessage::Error(14));
                                                    normal.mode = crate::src::inflate::BAD;
                                                    continue '_inf_leave;
                                                } else {
                                                    normal.extra = here.op as ::core::ffi::c_uint
                                                        & 15 as ::core::ffi::c_uint;
                                                    normal.mode = crate::src::inflate::LENEXT;
                                                    break 'c_2410;
                                                }
                                            }
                                        }
                                        if normal.flags & 0x400 as ::core::ffi::c_int != 0 {
                                            copy = normal.length;
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
                                                        normal.length,
                                                        &input[input_start
                                                            ..input_start + copy as usize],
                                                    );
                                                }
                                                if normal.flags & 0x200 as ::core::ffi::c_int != 0
                                                    && normal.wrap & 4 as ::core::ffi::c_int != 0
                                                {
                                                    normal.check = crate::src::crc32::crc32_z(
                                                        normal.check as crate::stdlib::uLong,
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
                                                normal.length = normal.length.wrapping_sub(copy);
                                            }
                                            if normal.length != 0 {
                                                break '_inf_leave;
                                            }
                                        }
                                        normal.length = 0 as ::core::ffi::c_uint;
                                        normal.mode = crate::src::inflate::NAME;
                                        break 'c_2322;
                                    }
                                    if normal.extra != 0 {
                                        while bits < normal.extra {
                                            if !inflate_pull_byte(
                                                input, in_0, &mut have, &mut hold, &mut bits,
                                            ) {
                                                break '_inf_leave;
                                            }
                                        }
                                        normal.length = normal.length.wrapping_add(
                                            hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint) << normal.extra)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                        );
                                        hold >>= normal.extra;
                                        bits = bits.wrapping_sub(normal.extra);
                                        normal.back = (normal.back as ::core::ffi::c_uint)
                                            .wrapping_add(normal.extra)
                                            as ::core::ffi::c_int;
                                    }
                                    normal.was = normal.length;
                                    normal.mode = crate::src::inflate::DIST;
                                    break 's_2462;
                                }
                                if normal.flags & 0x800 as ::core::ffi::c_int != 0 {
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
                                            if head.push_name(normal.length as usize, len as u8) {
                                                normal.length = normal.length.wrapping_add(1);
                                            }
                                        }
                                        if !(len != 0 && copy < have) {
                                            break;
                                        }
                                    }
                                    if normal.flags & 0x200 as ::core::ffi::c_int != 0
                                        && normal.wrap & 4 as ::core::ffi::c_int != 0
                                    {
                                        normal.check = crate::src::crc32::crc32_z(
                                            normal.check as crate::stdlib::uLong,
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
                                normal.length = 0 as ::core::ffi::c_uint;
                                normal.mode = crate::src::inflate::COMMENT;
                                break 'c_2325;
                            }
                            loop {
                                here = crate::src::inftrees::code::copied_from(
                                    normal.distcode.get(
                                        &normal.codes,
                                        (hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint) << normal.distbits)
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
                                        normal.distcode.get(
                                            &normal.codes,
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
                                normal.back += last.bits as ::core::ffi::c_int;
                            }
                            hold >>= here.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                            normal.back += here.bits as ::core::ffi::c_int;
                            if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                                stream.message = Some(InflateMessage::Error(15));
                                normal.mode = crate::src::inflate::BAD;
                                continue '_inf_leave;
                            } else {
                                normal.offset = here.val as ::core::ffi::c_uint;
                                normal.extra =
                                    here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                                normal.mode = crate::src::inflate::DISTEXT;
                                break 'c_2422;
                            }
                        }
                        if normal.flags & 0x1000 as ::core::ffi::c_int != 0 {
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
                                    if head.push_comment(normal.length as usize, len as u8) {
                                        normal.length = normal.length.wrapping_add(1);
                                    }
                                }
                                if !(len != 0 && copy < have) {
                                    break;
                                }
                            }
                            if normal.flags & 0x200 as ::core::ffi::c_int != 0
                                && normal.wrap & 4 as ::core::ffi::c_int != 0
                            {
                                normal.check = crate::src::crc32::crc32_z(
                                    normal.check as crate::stdlib::uLong,
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
                        normal.mode = crate::src::inflate::HCRC;
                        break 'c_2327;
                    }
                    if normal.extra != 0 {
                        while bits < normal.extra {
                            if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                                break '_inf_leave;
                            }
                        }
                        normal.offset = normal.offset.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << normal.extra)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= normal.extra;
                        bits = bits.wrapping_sub(normal.extra);
                        normal.back = (normal.back as ::core::ffi::c_uint)
                            .wrapping_add(normal.extra)
                            as ::core::ffi::c_int;
                    }
                    normal.mode = crate::src::inflate::MATCH;
                    break 'c_2425;
                }
                if normal.flags & 0x200 as ::core::ffi::c_int != 0 {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if !inflate_pull_byte(input, in_0, &mut have, &mut hold, &mut bits) {
                            break '_inf_leave;
                        }
                    }
                    if normal.wrap & 4 as ::core::ffi::c_int != 0
                        && hold != normal.check & 0xffff as ::core::ffi::c_ulong
                    {
                        stream.message = Some(InflateMessage::Error(16));
                        normal.mode = crate::src::inflate::BAD;
                        continue '_inf_leave;
                    } else {
                        hold = 0 as ::core::ffi::c_ulong;
                        bits = 0 as ::core::ffi::c_uint;
                    }
                }
                if let Some(head) = header.as_mut() {
                    head.set_hcrc_and_done(
                        normal.flags >> 9 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
                    );
                }
                normal.check = crate::src::crc32::crc32_z(0 as crate::stdlib::uLong, None)
                    as ::core::ffi::c_ulong;
                stream.adler = normal.check as crate::stdlib::uLong;
                normal.mode = crate::src::inflate::TYPE;
                continue '_inf_leave;
            }
            if left == 0 as ::core::ffi::c_uint {
                break;
            }
            let produced = out.wrapping_sub(left) as usize;
            let offset = normal.offset as usize;
            let mut history = None;
            copy = 0;
            if offset > produced {
                copy = (offset - produced) as ::core::ffi::c_uint;
                if copy > normal.whave {
                    if normal.sane != 0 {
                        stream.message = Some(InflateMessage::Error(17));
                        normal.mode = crate::src::inflate::BAD;
                        continue;
                    }
                }
                if copy > normal.wnext {
                    copy = copy.wrapping_sub(normal.wnext);
                    history = Some((normal.wsize.wrapping_sub(copy) as usize, copy));
                } else {
                    history = Some((normal.wnext.wrapping_sub(copy) as usize, copy));
                }
                if copy > normal.length {
                    copy = normal.length;
                }
            } else {
                copy = normal.length;
            }
            if copy > left {
                copy = left;
            }
            let copy_len = copy as usize;
            // `out` is the capacity since the last checksum boundary and
            // `produced + copy_len` is bounded by it after the `left` clamp.
            let output = &mut output[output_chunk_start..output_chunk_start + out as usize];
            if let Some((start, history_limit)) = history {
                let window = normal
                    .owned_window
                    .as_deref()
                    .expect("normal inflate owns its history window");
                let history = &window[start..start + copy_len.min(history_limit as usize)];
                copy_inflate_match(output, produced, offset, Some(history), copy_len);
            } else {
                copy_inflate_match(output, produced, offset, None, copy_len);
            }
            left = left.wrapping_sub(copy);
            normal.length = normal.length.wrapping_sub(copy);
            if normal.length == 0 as ::core::ffi::c_uint {
                normal.mode = crate::src::inflate::LEN;
            }
        }
        // Consume the bounded cursor transaction before the ABI adapter
        // republishes it.  The slices above are the sole authority for this
        // call's movement.
        let cursor = InflateDecoderCursor::from_remaining(input.len(), have, output.len(), left);
        completion_cursor = Some(cursor);
        normal.hold = hold;
        normal.bits = bits;
        if normal.wsize != 0
            || out != cursor.output_remaining
                && (normal.mode as ::core::ffi::c_uint)
                    < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                && ((normal.mode as ::core::ffi::c_uint)
                    < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                    || flush != crate::zlib_h::Z_FINISH)
        {
            // `output` is the bounded caller range retained for this call, and
            // `output_chunk_start` marks the same post-checksum chunk that the
            // former cursor adapter passed to `updatewindow()`.  Keep history
            // updates in the slice core instead of rebuilding raw cursors from
            // the ABI stream after all decoding has completed.
            let produced = out.wrapping_sub(cursor.output_remaining) as usize;
            let produced_output = &output[output_chunk_start..output_chunk_start + produced];
            if update_window_from_slice(
                &mut normal.owned_window,
                normal.wbits,
                &mut normal.wsize,
                &mut normal.wnext,
                &mut normal.whave,
                produced_output,
            ) != 0
            {
                normal.mode = crate::src::inflate::MEM;
                break '_inflate_result crate::zlib_h::Z_MEM_ERROR;
            }
        }
        in_0 = in_0.wrapping_sub(cursor.input_remaining);
        out = out.wrapping_sub(cursor.output_remaining);
        stream.total_in = stream.total_in.wrapping_add(in_0 as crate::stdlib::uLong);
        stream.total_out = stream.total_out.wrapping_add(out as crate::stdlib::uLong);
        normal.total = normal.total.wrapping_add(out as ::core::ffi::c_ulong);
        if normal.wrap & 4 as ::core::ffi::c_int != 0 && out != 0 {
            let produced = &output[output_chunk_start..output_chunk_start + out as usize];
            normal.check = (if normal.flags != 0 {
                crate::src::crc32::crc32_z(normal.check as crate::stdlib::uLong, Some(produced))
            } else {
                crate::src::adler32::adler32(normal.check as crate::stdlib::uLong, produced)
            }) as ::core::ffi::c_ulong;
            stream.adler = normal.check as crate::stdlib::uLong;
        }
        stream.data_type = normal.bits as ::core::ffi::c_int
            + (if normal.last != 0 {
                64 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            + (if normal.mode as ::core::ffi::c_uint
                == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                128 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            })
            + (if normal.mode as ::core::ffi::c_uint
                == crate::src::inflate::LEN_ as ::core::ffi::c_int as ::core::ffi::c_uint
                || normal.mode as ::core::ffi::c_uint
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
    InflateDecoderResult {
        status: result,
        cursor: completion_cursor.expect("every inflate decoder exit has a cursor completion"),
        publication: header.map(|output| output.publication),
    }
}

// The stream/state association, foreign header registration, and caller
// cursor views remain at this ABI-shaped boundary.  The decoder itself sees
// only pointer-free normal state, bounded slices, and scalar publication.
// Keep this projection separate from `inflate()`, which is the pointer-free
// decoder core consumed by direct bounded callers.  The owner constructed
// below consumes every scoped cursor and registered-header view before this
// adapter republishes ABI state.
pub(crate) unsafe fn inflate_from_stream(
    strm: &mut crate::zlib_h::z_stream_s,
    request: InflateStreamRequest<'_>,
) -> InflateStreamResult {
    let Some((strm, state)) = inflate_stream_and_state(strm) else {
        return match request {
            InflateStreamRequest::Scalar(action) => {
                InflateStreamResult::Scalar(inflate_normal_scalar_stream_error(action))
            }
            InflateStreamRequest::Decode(_)
            | InflateStreamRequest::Fast(_)
            | InflateStreamRequest::Sync
            | InflateStreamRequest::Reset(_)
            | InflateStreamRequest::Dictionary { .. }
            | InflateStreamRequest::SetDictionary(_) => {
                InflateStreamResult::Status(crate::zlib_h::Z_STREAM_ERROR)
            }
        };
    };
    if let InflateStreamRequest::Scalar(action) = request {
        let mut owner = InflateNormalStateOwner::new(&mut state.decoder.normal);
        return InflateStreamResult::Scalar(inflate_normal_scalar(&mut owner, action));
    };
    if let InflateStreamRequest::Dictionary {
        dictionary,
        dict_length,
    } = request
    {
        return InflateStreamResult::Status(inflate_get_dictionary(
            &state.decoder.normal,
            dictionary,
            dict_length,
        ));
    }
    if let InflateStreamRequest::SetDictionary(dictionary) = request {
        let mut owner = InflateNormalStateOwner::new(&mut state.decoder.normal);
        return InflateStreamResult::Status(inflateSetDictionary(&mut owner, dictionary));
    }
    if let InflateStreamRequest::Reset(kind) = request {
        // Reset shares this established stream/state projection with normal
        // inflate requests. Refresh and publish the scalar snapshot here,
        // keeping header-registration release at the sole association
        // boundary rather than recreating a reset-specific unsafe adapter.
        state.decoder.stream = InflateDecoderStream {
            total_in: strm.total_in,
            total_out: strm.total_out,
            adler: strm.adler,
            data_type: strm.data_type,
            message: None,
        };
        let completion = match inflate_reset_from_stream(
            InflateResetOwner::new(&mut state.decoder.normal, &mut state.decoder.stream),
            kind,
        ) {
            Ok(completion) => completion,
            Err(status) => return InflateStreamResult::Status(status),
        };
        strm.total_out = state.decoder.stream.total_out;
        strm.total_in = state.decoder.stream.total_in;
        strm.msg = ::core::ptr::null_mut();
        strm.data_type = state.decoder.stream.data_type;
        strm.adler = state.decoder.stream.adler;
        if completion.clear_header_registration {
            state.head = None;
        }
        return InflateStreamResult::Status(crate::zlib_h::Z_OK);
    }
    if strm.avail_in != 0 && strm.next_in.is_null() {
        return InflateStreamResult::Status(crate::zlib_h::Z_STREAM_ERROR);
    }
    // All normal inflate requests consume the same checked ABI input view.
    // Keep it at this projection boundary so sync does not recreate a second
    // raw cursor adapter beside the decoder.
    let input = if strm.avail_in == 0 {
        &[][..]
    } else {
        ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
    };
    if let InflateStreamRequest::Fast(start) = request {
        let written = start.wrapping_sub(strm.avail_out) as usize;
        let output_start = strm.next_out.wrapping_sub(written);
        let output = if start == 0 {
            &mut []
        } else {
            if output_start.is_null() {
                return InflateStreamResult::Status(crate::zlib_h::Z_STREAM_ERROR);
            }
            ::core::slice::from_raw_parts_mut(output_start, start as usize)
        };
        // The direct fast export now shares the same pointer-free normal
        // owner as the full decoder.  This adapter owns its one ABI cursor
        // projection and publishes only after the bounded request ends.
        let fast_state = state.decoder.normal.fast_state();
        let Some(owner) = InflateNormalStreamOwner::new(input, output, written, fast_state) else {
            return InflateStreamResult::Status(crate::zlib_h::Z_STREAM_ERROR);
        };
        let update = owner.run_fast();
        strm.next_in = strm.next_in.wrapping_add(update.input_used);
        strm.avail_in = update.input_remaining as crate::stdlib::uInt;
        strm.next_out = output_start.wrapping_add(update.output_used);
        strm.avail_out = update.output_remaining as crate::stdlib::uInt;
        state.decoder.normal.apply_fast_update(&update);
        strm.msg = match update.exit {
            crate::src::inffast::FastExit::Continue | crate::src::inffast::FastExit::Type => {
                strm.msg
            }
            crate::src::inffast::FastExit::InvalidDistance => {
                INFLATE_ERROR_MESSAGES[17].as_ptr().cast_mut().cast()
            }
            crate::src::inffast::FastExit::InvalidCode => {
                b"invalid literal/length or distance code\0"
                    .as_ptr()
                    .cast_mut()
                    .cast()
            }
        };
        return InflateStreamResult::Status(crate::zlib_h::Z_OK);
    }
    if let InflateStreamRequest::Sync = request {
        let (flags, in_0, out) = {
            let sync_state = InflateSyncState {
                mode: state.decoder.normal.mode,
                hold: state.decoder.normal.hold,
                bits: state.decoder.normal.bits,
                have: state.decoder.normal.have,
                flags: state.decoder.normal.flags,
                wrap: state.decoder.normal.wrap,
            };
            let (sync_state, consumed, status) = inflate_sync_core(sync_state, input);
            state.decoder.normal.mode = sync_state.mode;
            state.decoder.normal.hold = sync_state.hold;
            state.decoder.normal.bits = sync_state.bits;
            state.decoder.normal.have = sync_state.have;
            state.decoder.normal.flags = sync_state.flags;
            state.decoder.normal.wrap = sync_state.wrap;
            strm.avail_in = strm
                .avail_in
                .wrapping_sub(consumed as crate::stdlib::uInt);
            strm.next_in = strm.next_in.wrapping_add(consumed);
            strm.total_in = strm
                .total_in
                .wrapping_add(consumed as crate::stdlib::uLong);
            if status != crate::zlib_h::Z_OK {
                return InflateStreamResult::Status(status);
            }
            (state.decoder.normal.flags, strm.total_in, strm.total_out)
        };
        let update = inflate_reset_core(&mut state.decoder.normal);
        strm.total_out = 0;
        strm.total_in = strm.total_out;
        strm.msg = ::core::ptr::null_mut();
        strm.data_type = 0;
        if let Some(adler) = update.adler {
            strm.adler = adler;
        }
        state.head = None;
        strm.total_in = in_0;
        strm.total_out = out;
        state.decoder.normal.flags = flags;
        state.decoder.normal.mode = crate::src::inflate::TYPE;
        return InflateStreamResult::Status(crate::zlib_h::Z_OK);
    }
    let InflateStreamRequest::Decode(flush) = request else {
        unreachable!("non-decoder request returned from its ABI projection");
    };
    if strm.next_out.is_null() {
        return InflateStreamResult::Status(crate::zlib_h::Z_STREAM_ERROR);
    }
    // Convert the registered handle once for this entire request.  The
    // resulting borrow carries the original caller provenance through both
    // the bounded header-output view and its later scalar publication; do
    // not reconstruct a second reference from the handle after decoding.
    let mut registered_header = state.head.map(|registered| &mut *registered.as_ptr());
    let header = registered_header.as_deref_mut().map(|header| {
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
    let output = ::core::slice::from_raw_parts_mut(strm.next_out, strm.avail_out as usize);
    // Keep stream scalars inside the same pointer-free decoder owner as the
    // resumable normal state.  The ABI adapter refreshes it only while the
    // cursor/header projections are live, then publishes this completion.
    state.decoder.stream = InflateDecoderStream {
        total_in: strm.total_in,
        total_out: strm.total_out,
        adler: strm.adler,
        data_type: strm.data_type,
        message: None,
    };
    let result = InflateStreamOwner {
        normal: &mut state.decoder.normal,
        input,
        output,
        header,
        stream: &mut state.decoder.stream,
        flush,
    }
    .run();
    strm.next_out = strm.next_out.wrapping_add(result.cursor.output_used);
    strm.avail_out = result.cursor.output_remaining;
    strm.next_in = strm.next_in.wrapping_add(result.cursor.input_used);
    strm.avail_in = result.cursor.input_remaining;
    strm.total_in = state.decoder.stream.total_in;
    strm.total_out = state.decoder.stream.total_out;
    strm.adler = state.decoder.stream.adler;
    strm.data_type = state.decoder.stream.data_type;
    if let Some(message) = state.decoder.stream.message {
        strm.msg = match message {
            InflateMessage::Error(index) => {
                INFLATE_ERROR_MESSAGES[index].as_ptr().cast_mut().cast()
            }
            InflateMessage::InvalidCode => b"invalid literal/length or distance code\0"
                .as_ptr()
                .cast_mut()
                .cast(),
        };
    }
    if let (Some(header), Some(publication)) = (registered_header, result.publication) {
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
    InflateStreamResult::Status(result.status)
}
#[export_name = "inflate"]

pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_from_stream(strm, InflateStreamRequest::Decode(flush)).status()
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
    InflateEndOwner::from_normal(&mut state.decoder.normal).release();
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
    // zlib documents that 32768 bytes always suffice.  Form this bounded
    // caller view before dispatch, leaving the normal-state owner to select
    // the current history length and perform the copy.
    let dictionary = if dictionary.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts_mut(
            dictionary,
            1usize << crate::stdlib::MAX_WBITS,
        ))
    };
    inflate_from_stream(
        stream,
        InflateStreamRequest::Dictionary {
            dictionary,
            dict_length: dictLength.as_mut(),
        },
    )
    .status()
}
// Dictionary installation is entirely a normal-decoder-state transition.
// Keeping it on the pointer-free owner lets embedded callers reuse the same
// policy without reopening the ABI stream/state boundary.
fn inflateSetDictionary(
    owner: &mut InflateNormalStateOwner<'_>,
    dictionary: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    let mut dictid: ::core::ffi::c_ulong = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let normal = &mut *owner.normal;
    if normal.wrap != 0 as ::core::ffi::c_int
        && normal.mode as ::core::ffi::c_uint
            != crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if normal.mode as ::core::ffi::c_uint
        == crate::src::inflate::DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        dictid =
            crate::src::adler32::adler32_z(0 as crate::stdlib::uLong, None) as ::core::ffi::c_ulong;
        dictid = crate::src::adler32::adler32(dictid as crate::stdlib::uLong, dictionary)
            as ::core::ffi::c_ulong;
        if dictid != normal.check {
            return crate::zlib_h::Z_DATA_ERROR;
        }
    }
    ret = update_window_from_slice(
        &mut normal.owned_window,
        normal.wbits,
        &mut normal.wsize,
        &mut normal.wnext,
        &mut normal.whave,
        dictionary,
    );
    if ret != 0 {
        normal.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    normal.havedict = 1 as ::core::ffi::c_int;
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
    inflate_from_stream(strm, InflateStreamRequest::SetDictionary(dictionary)).status()
}
// A registered gzip header must retain the original pointer's provenance for
// later decoder calls.  The export boundary forms that handle after checking
// the stream; this adapter owns the stream-bound opaque-state projection and
// the one immediate `done` publication.
pub unsafe fn inflateGetHeader(
    strm: &mut crate::zlib_h::z_stream_s,
    mut head: Option<&mut crate::zlib_h::gz_header_s>,
) -> ::core::ffi::c_int {
    let Some((_strm, state)) = inflate_stream_and_state(strm) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if state.decoder.normal.wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.head = head.as_deref_mut().map(::core::ptr::NonNull::from);
    if let Some(head) = head {
        head.done = 0 as ::core::ffi::c_int;
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
    inflateGetHeader(strm, head.as_mut())
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

#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_from_stream(strm, InflateStreamRequest::Sync).status()
}

fn inflate_sync_point(
    mode: crate::src::inflate::inflate_mode,
    bits: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    (mode == crate::src::inflate::STORED as ::core::ffi::c_int as ::core::ffi::c_uint
        && bits == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
}

// Scalar controls all need the same stream-bound opaque-state association.
// Keep that one projection in this adapter; the action and result are fully
// pointer-free, so each operation's policy remains independently testable
// without reopening the ABI stream boundary.
#[derive(Clone, Copy)]
pub(crate) enum InflateNormalScalarAction {
    Prime {
        bits: ::core::ffi::c_int,
        value: ::core::ffi::c_int,
    },
    SyncPoint,
    Undermine,
    Validate(::core::ffi::c_int),
    Mark,
    CodesUsed,
}

pub(crate) enum InflateNormalScalarResult {
    Status(::core::ffi::c_int),
    Mark(::core::ffi::c_long),
    CodesUsed(::core::ffi::c_ulong),
}

impl InflateNormalScalarResult {
    fn status(self) -> ::core::ffi::c_int {
        match self {
            Self::Status(status) => status,
            Self::Mark(_) | Self::CodesUsed(_) => unreachable!("status scalar action"),
        }
    }

    fn mark(self) -> ::core::ffi::c_long {
        match self {
            Self::Mark(mark) => mark,
            Self::Status(_) | Self::CodesUsed(_) => unreachable!("mark scalar action"),
        }
    }

    fn codes_used(self) -> ::core::ffi::c_ulong {
        match self {
            Self::CodesUsed(used) => used,
            Self::Status(_) | Self::Mark(_) => unreachable!("codes-used scalar action"),
        }
    }
}

fn inflate_normal_scalar(
    owner: &mut InflateNormalStateOwner<'_>,
    action: InflateNormalScalarAction,
) -> InflateNormalScalarResult {
    match action {
        InflateNormalScalarAction::Prime { bits, value } => {
            InflateNormalScalarResult::Status(inflatePrime(owner, bits, value))
        }
        InflateNormalScalarAction::SyncPoint => InflateNormalScalarResult::Status(
            inflate_sync_point(owner.normal.mode, owner.normal.bits),
        ),
        InflateNormalScalarAction::Undermine => {
            InflateNormalScalarResult::Status(inflate_undermine_sane(&mut owner.normal.sane))
        }
        InflateNormalScalarAction::Validate(check) => {
            InflateNormalScalarResult::Status(inflateValidate(owner.normal, check))
        }
        InflateNormalScalarAction::Mark => InflateNormalScalarResult::Mark(inflate_mark_value(
            owner.normal.back,
            owner.normal.mode,
            owner.normal.length,
            owner.normal.was,
        )),
        InflateNormalScalarAction::CodesUsed => {
            InflateNormalScalarResult::CodesUsed(inflate_codes_used(owner.normal.next))
        }
    }
}

fn inflate_normal_scalar_stream_error(
    action: InflateNormalScalarAction,
) -> InflateNormalScalarResult {
    match action {
        InflateNormalScalarAction::Mark => InflateNormalScalarResult::Mark(
            -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int),
        ),
        InflateNormalScalarAction::CodesUsed => {
            InflateNormalScalarResult::CodesUsed(-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
        }
        InflateNormalScalarAction::Prime { .. }
        | InflateNormalScalarAction::SyncPoint
        | InflateNormalScalarAction::Undermine
        | InflateNormalScalarAction::Validate(_) => {
            InflateNormalScalarResult::Status(crate::zlib_h::Z_STREAM_ERROR)
        }
    }
}

#[export_name = "inflateSyncPoint"]

pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_from_stream(strm, InflateStreamRequest::Scalar(InflateNormalScalarAction::SyncPoint))
        .scalar()
        .status()
}

// The export validates the source handle and creates this scoped borrow.  The
// destination remains a non-borrowing handle until the source projection has
// ended, since zlib permits source and destination to alias.
pub unsafe fn inflateCopy(
    mut dest: crate::zlib_h::z_streamp,
    source: &mut crate::zlib_h::z_stream_s,
) -> ::core::ffi::c_int {
    if dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_identity = dest.addr();
    // Build the replacement before borrowing the destination.  This retains
    // C's behavior even for a source/destination alias while all state
    // access remains scoped to the checked source stream.
    let (copy, state_copy, destination_stream) = {
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
        let owned_window = match state.decoder.normal.owned_window.as_deref() {
            Some(source_window) => {
                let Some(mut window) = allocate_inflate_window(source_window.len()) else {
                    Some(source.zfree.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        source.opaque, copy.as_ptr().cast()
                    );
                    return crate::zlib_h::Z_MEM_ERROR;
                };
                window[..state.decoder.normal.whave as usize]
                    .copy_from_slice(&source_window[..state.decoder.normal.whave as usize]);
                Some(window)
            }
            None => None,
        };
        let state_copy = inflate_state {
            stream_identity: dest_identity,
            head: state.head,
            back_window: state.back_window.clone(),
            decoder: InflateOwnedDecoder {
                normal: InflateNormalState {
                mode: state.decoder.normal.mode,
                last: state.decoder.normal.last,
                wrap: state.decoder.normal.wrap,
                havedict: state.decoder.normal.havedict,
                flags: state.decoder.normal.flags,
                dmax: state.decoder.normal.dmax,
                check: state.decoder.normal.check,
                total: state.decoder.normal.total,
                wbits: state.decoder.normal.wbits,
                wsize: state.decoder.normal.wsize,
                whave: state.decoder.normal.whave,
                wnext: state.decoder.normal.wnext,
                owned_window,
                hold: state.decoder.normal.hold,
                bits: state.decoder.normal.bits,
                length: state.decoder.normal.length,
                offset: state.decoder.normal.offset,
                extra: state.decoder.normal.extra,
                lencode: state.decoder.normal.lencode,
                distcode: state.decoder.normal.distcode,
                lenbits: state.decoder.normal.lenbits,
                distbits: state.decoder.normal.distbits,
                ncode: state.decoder.normal.ncode,
                nlen: state.decoder.normal.nlen,
                ndist: state.decoder.normal.ndist,
                have: state.decoder.normal.have,
                next: state.decoder.normal.next,
                lens: state.decoder.normal.lens,
                work: state.decoder.normal.work,
                codes: core::array::from_fn(|index| {
                    crate::src::inftrees::code::copied_from(&state.decoder.normal.codes[index])
                }),
                sane: state.decoder.normal.sane,
                back: state.decoder.normal.back,
                was: state.decoder.normal.was,
                },
                stream: InflateDecoderStream {
                    total_in: state.decoder.stream.total_in,
                    total_out: state.decoder.stream.total_out,
                    adler: state.decoder.stream.adler,
                    data_type: state.decoder.stream.data_type,
                    message: state.decoder.stream.message,
                },
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
    if dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(source) = source.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateCopy(dest, source)
}

fn inflate_undermine_sane(sane: &mut ::core::ffi::c_int) -> ::core::ffi::c_int {
    *sane = 1 as ::core::ffi::c_int;
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
    inflate_from_stream(strm, InflateStreamRequest::Scalar(InflateNormalScalarAction::Undermine))
        .scalar()
        .status()
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

fn inflateValidate(
    normal: &mut InflateNormalState,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_validate_wrap(&mut normal.wrap, check)
}

#[export_name = "inflateValidate"]

pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_from_stream(
        strm,
        InflateStreamRequest::Scalar(InflateNormalScalarAction::Validate(check)),
    )
    .scalar()
    .status()
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

#[export_name = "inflateMark"]

pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    let Some(strm) = strm.as_mut() else {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    };
    inflate_from_stream(strm, InflateStreamRequest::Scalar(InflateNormalScalarAction::Mark))
        .scalar()
        .mark()
}

fn inflate_codes_used(next: usize) -> ::core::ffi::c_ulong {
    next as ::core::ffi::c_ulong
}

#[export_name = "inflateCodesUsed"]

pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    let Some(strm) = strm.as_mut() else {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    };
    inflate_from_stream(
        strm,
        InflateStreamRequest::Scalar(InflateNormalScalarAction::CodesUsed),
    )
    .scalar()
    .codes_used()
}
