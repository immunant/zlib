pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::inflate::inflate_mode;
pub use crate::src::inflate::inflate_state;
pub use crate::src::inflate::BAD;
pub use crate::src::inflate::CHECK;
pub use crate::src::inflate::CODELENS;
pub use crate::src::inflate::COMMENT;
pub use crate::src::inflate::COPY_;
pub use crate::src::inflate::COPY_1;
pub use crate::src::inflate::DICT;
pub use crate::src::inflate::DICTID;
pub use crate::src::inflate::DIST;
pub use crate::src::inflate::DISTEXT;
pub use crate::src::inflate::DONE;
pub use crate::src::inflate::EXLEN;
pub use crate::src::inflate::EXTRA;
pub use crate::src::inflate::FLAGS;
pub use crate::src::inflate::HCRC;
pub use crate::src::inflate::HEAD;
pub use crate::src::inflate::LEN;
pub use crate::src::inflate::LENEXT;
pub use crate::src::inflate::LENGTH;
pub use crate::src::inflate::LENLENS;
pub use crate::src::inflate::LEN_;
pub use crate::src::inflate::LIT;
pub use crate::src::inflate::MATCH;
pub use crate::src::inflate::MEM;
pub use crate::src::inflate::NAME;
pub use crate::src::inflate::OS;
pub use crate::src::inflate::STORED;
pub use crate::src::inflate::SYNC;
pub use crate::src::inflate::TABLE;
pub use crate::src::inflate::TIME;
pub use crate::src::inflate::TYPE;
pub use crate::src::inflate::TYPEDO;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::inflate_fixed;
pub use crate::src::inftrees::inflate_table;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::LENS;

pub use crate::src::deflate::internal_state;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gz_header;
pub use crate::zlib_h::gz_header_s;
pub use crate::zlib_h::gz_headerp;
pub use crate::zlib_h::in_func;
pub use crate::zlib_h::out_func;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub use crate::zlib_h::Z_VERSION_ERROR;

pub unsafe extern "C" fn inflateBackInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // The shared non-FFI transaction owns raw leading-version observation,
    // callback-back admission, and callback allocation/publication. This
    // adapter carries only scalar admission data and the nullable handle.
    let strm = ::core::ptr::NonNull::new(strm);
    let mut copied_state = None;
    crate::src::inflate::inflate_publish_callback_owner(
        strm,
        None,
        version,
        true,
        stream_size,
        Some(crate::src::inflate::InflateBackInitAdmission {
            window_bits: windowBits,
            window_present: !window.is_null(),
        }),
        None,
        0,
        &mut copied_state,
    )
}
#[export_name = "inflateBackInit_"]

pub unsafe extern "C" fn inflateBackInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateBackInit_(strm, windowBits, window, version, stream_size)
}

// The callback owns the raw cursor transition, but the decoder only needs a
// bounded current chunk.  This call-scoped facade keeps its public operations
// pointer-free: the boundary closure below refills and republishes the raw
// cursor, while all bit-reader sites ask only for bytes or bounded slices.
struct InflateBackInput<F> {
    visit: F,
}

impl<F> InflateBackInput<F>
where
    F: FnMut(&mut dyn FnMut(&[::core::ffi::c_uchar]) -> usize),
{
    fn new(visit: F) -> Self {
        Self { visit }
    }

    fn available(&mut self) -> usize {
        let mut available = 0;
        (self.visit)(&mut |input| {
            available = input.len();
            0
        });
        available
    }

    fn pull_byte(&mut self) -> Option<::core::ffi::c_uchar> {
        let mut byte = None;
        (self.visit)(&mut |input| {
            byte = input.first().copied();
            usize::from(byte.is_some())
        });
        byte
    }

    fn copy_to(&mut self, output: &mut [::core::ffi::c_uchar]) -> usize {
        let mut copied = 0;
        (self.visit)(&mut |input| {
            copied = input.len().min(output.len());
            output[..copied].copy_from_slice(&input[..copied]);
            copied
        });
        copied
    }

    fn with_remaining(&mut self, operation: impl FnOnce(&[::core::ffi::c_uchar]) -> usize) {
        let mut operation = Some(operation);
        (self.visit)(&mut |input| operation.take().expect("single input visit")(input));
    }
}

// The caller window is valid only for this `inflateBack()` invocation.  Keep
// its cursor as an index into a scoped slice, and let the boundary closure be
// the sole place that presents a raw window pointer to the output callback.
// In particular, no callback allocation or borrowed window escapes this
// facade as a `'static` slice.
struct InflateBackOutput<'a, F> {
    window: &'a mut [::core::ffi::c_uchar],
    written: usize,
    visit: F,
}

impl<'a, F> InflateBackOutput<'a, F>
where
    F: FnMut(&[::core::ffi::c_uchar]) -> ::core::ffi::c_int,
{
    fn new(window: &'a mut [::core::ffi::c_uchar], visit: F) -> Self {
        Self {
            window,
            written: 0,
            visit,
        }
    }

    fn remaining(&self) -> usize {
        self.window.len() - self.written
    }

    fn written(&self) -> usize {
        self.written
    }

    fn flush(&mut self) -> bool {
        if (self.visit)(&self.window[..self.written]) != 0 {
            return false;
        }
        self.written = 0;
        true
    }

    fn write_with(
        &mut self,
        operation: impl FnOnce(&mut [::core::ffi::c_uchar]) -> usize,
    ) -> usize {
        let available = self.remaining();
        let written = operation(&mut self.window[self.written..]);
        let written = written.min(available);
        self.written += written;
        written
    }

    fn write_byte(&mut self, byte: ::core::ffi::c_uchar) -> bool {
        let Some(slot) = self.window.get_mut(self.written) else {
            return false;
        };
        *slot = byte;
        self.written += 1;
        true
    }

    fn with_window<R>(
        &mut self,
        operation: impl FnOnce(&mut [::core::ffi::c_uchar], usize) -> R,
    ) -> R {
        operation(self.window, self.written)
    }

    fn set_written(&mut self, written: usize) -> bool {
        if written > self.window.len() {
            return false;
        }
        self.written = written;
        true
    }

    fn copy_match(&mut self, offset: usize, length: usize) -> Option<usize> {
        let left = self.remaining();
        let put_index = self.written;
        let back = self.window.len().checked_sub(offset)?;
        let (from_index, available) = if back < left {
            (put_index.checked_add(back)?, left.checked_sub(back)?)
        } else {
            (put_index.checked_sub(offset)?, left)
        };
        let count = available.min(length);
        let from_end = from_index.checked_add(count)?;
        let put_end = put_index.checked_add(count)?;
        if from_end > self.window.len() || put_end > self.window.len() {
            return None;
        }
        self.window.copy_within(from_index..from_end, put_index);
        self.written = put_end;
        Some(count)
    }
}

// Callback-back decoding needs most of inflate_state, but none of its ABI
// handles.  Keep the decoder-visible portion as a pointer-free view over the
// state-owned tables.  `inflateBack()` builds and writes back this view at its
// existing raw projection boundary; a subsequent extraction can therefore
// accept this type together with the input/output facades without inheriting
// the stream, header, or caller-window pointers.
struct InflateBackDecoderState<'a> {
    mode: crate::src::inflate::inflate_mode,
    last: ::core::ffi::c_int,
    wsize: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    length: ::core::ffi::c_uint,
    offset: ::core::ffi::c_uint,
    extra: ::core::ffi::c_uint,
    lencode: crate::src::inflate::CodeTableRef,
    distcode: crate::src::inflate::CodeTableRef,
    lenbits: ::core::ffi::c_uint,
    distbits: ::core::ffi::c_uint,
    ncode: ::core::ffi::c_uint,
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    next: usize,
    lens: &'a mut [::core::ffi::c_ushort; 320],
    work: &'a mut [::core::ffi::c_ushort; 288],
    codes: &'a mut [crate::src::inftrees::code; 1444],
    sane: ::core::ffi::c_int,
}

struct InflateBackDecoderScalars {
    mode: crate::src::inflate::inflate_mode,
    last: ::core::ffi::c_int,
    whave: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    length: ::core::ffi::c_uint,
    offset: ::core::ffi::c_uint,
    extra: ::core::ffi::c_uint,
    lencode: crate::src::inflate::CodeTableRef,
    distcode: crate::src::inflate::CodeTableRef,
    lenbits: ::core::ffi::c_uint,
    distbits: ::core::ffi::c_uint,
    ncode: ::core::ffi::c_uint,
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    next: usize,
}

impl InflateBackDecoderState<'_> {
    fn scalars(&self) -> InflateBackDecoderScalars {
        InflateBackDecoderScalars {
            mode: self.mode,
            last: self.last,
            whave: self.whave,
            wnext: self.wnext,
            length: self.length,
            offset: self.offset,
            extra: self.extra,
            lencode: self.lencode,
            distcode: self.distcode,
            lenbits: self.lenbits,
            distbits: self.distbits,
            ncode: self.ncode,
            nlen: self.nlen,
            ndist: self.ndist,
            have: self.have,
            next: self.next,
        }
    }
}

struct InflateBackDecodeResult {
    status: ::core::ffi::c_int,
    message: Option<&'static [u8]>,
}

// A completed callback-back request carries every decoder-visible result as
// one pointer-free value.  The ABI adapter must consume this only after the
// callback and caller-window borrows have ended, so status/diagnostics cannot
// be published ahead of the state that produced them.
struct InflateBackCompletion {
    result: InflateBackDecodeResult,
    state: InflateBackDecoderScalars,
}

// Back-mode's decoder and caller window are both call-scoped, pointer-free
// resources once the opaque inflate state has been projected. Keep their
// setup and scalar write-back together in this owner so no decoder operation
// needs to touch the raw-backed `inflate_state` directly.
struct InflateBackStateOwner<'a> {
    normal: &'a mut crate::src::inflate::InflateNormalState,
    window: &'a mut crate::src::inflate::InflateBackWindow,
}

impl<'a> InflateBackStateOwner<'a> {
    fn new(
        normal: &'a mut crate::src::inflate::InflateNormalState,
        window: &'a mut crate::src::inflate::InflateBackWindow,
    ) -> Self {
        Self { normal, window }
    }

    fn begin(&mut self) -> Option<(InflateBackDecoderState<'_>, &mut [u8])> {
        let (normal, back_window) = (&mut *self.normal, &mut *self.window);
        normal.mode = crate::src::inflate::TYPE;
        normal.last = 0;
        normal.whave = 0;
        let window = back_window
            .bytes
            .as_mut()
            .get_mut(..normal.wsize as usize)?;
        Some((
            InflateBackDecoderState {
                mode: normal.mode,
                last: normal.last,
                wsize: normal.wsize,
                whave: normal.whave,
                wnext: normal.wnext,
                length: normal.length,
                offset: normal.offset,
                extra: normal.extra,
                lencode: normal.lencode,
                distcode: normal.distcode,
                lenbits: normal.lenbits,
                distbits: normal.distbits,
                ncode: normal.ncode,
                nlen: normal.nlen,
                ndist: normal.ndist,
                have: normal.have,
                next: normal.next,
                lens: &mut normal.lens,
                work: &mut normal.work,
                codes: &mut normal.codes,
                sane: normal.sane,
            },
            window,
        ))
    }

    fn commit(&mut self, completion: InflateBackCompletion) -> InflateBackDecodeResult {
        self.normal.mode = completion.state.mode;
        self.normal.last = completion.state.last;
        self.normal.whave = completion.state.whave;
        self.normal.wnext = completion.state.wnext;
        self.normal.length = completion.state.length;
        self.normal.offset = completion.state.offset;
        self.normal.extra = completion.state.extra;
        self.normal.lencode = completion.state.lencode;
        self.normal.distcode = completion.state.distcode;
        self.normal.lenbits = completion.state.lenbits;
        self.normal.distbits = completion.state.distbits;
        self.normal.ncode = completion.state.ncode;
        self.normal.nlen = completion.state.nlen;
        self.normal.ndist = completion.state.ndist;
        self.normal.have = completion.state.have;
        self.normal.next = completion.state.next;
        completion.result
    }
}

// One callback-back operation owns all decoder-visible borrows.  In
// particular, this keeps the implementation entry point free of the ABI
// stream, callback descriptors, and caller-window address: the adapter only
// creates this call-scoped owner and publishes its scalar completion once the
// owner has been released.
struct InflateBackInvocation<'a, InputVisitor, OutputVisitor> {
    state: InflateBackDecoderState<'a>,
    input: InflateBackInput<InputVisitor>,
    output: InflateBackOutput<'a, OutputVisitor>,
}

impl<InputVisitor, OutputVisitor> InflateBackInvocation<'_, InputVisitor, OutputVisitor>
where
    InputVisitor: FnMut(&mut dyn FnMut(&[::core::ffi::c_uchar]) -> usize),
    OutputVisitor: FnMut(&[::core::ffi::c_uchar]) -> ::core::ffi::c_int,
{
    fn decode(&mut self) -> InflateBackCompletion {
        let result = inflate_back_decode(&mut self.state, &mut self.input, &mut self.output);
        InflateBackCompletion {
            result,
            state: self.state.scalars(),
        }
    }
}

// The callback decoder owns only pointer-free state and bounded caller borrows.
// Raw ABI cursor projection and diagnostic publication stay in inflateBack().
fn inflate_back_decode<InputVisitor, OutputVisitor>(
    state: &mut InflateBackDecoderState<'_>,
    input: &mut InflateBackInput<InputVisitor>,
    output: &mut InflateBackOutput<'_, OutputVisitor>,
) -> InflateBackDecodeResult
where
    InputVisitor: FnMut(&mut dyn FnMut(&[::core::ffi::c_uchar]) -> usize),
    OutputVisitor: FnMut(&[::core::ffi::c_uchar]) -> ::core::ffi::c_int,
{
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut copy: ::core::ffi::c_uint = 0;
    let mut here = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut last = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut len: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut message: Option<&'static [u8]> = None;
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
    '_inf_leave: loop {
        match state.mode as ::core::ffi::c_uint {
            16191 => {
                if state.last != 0 {
                    hold >>= bits & 7 as ::core::ffi::c_uint;
                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                    state.mode = crate::src::inflate::DONE;
                    continue;
                } else {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        let Some(byte) = input.pull_byte() else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    state.last = (hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_int;
                    hold >>= 1 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    match hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint)
                    {
                        0 => {
                            state.mode = crate::src::inflate::STORED;
                        }
                        1 => {
                            crate::src::inftrees::inflate_fixed(
                                &mut state.lencode,
                                &mut state.lenbits,
                                &mut state.distcode,
                                &mut state.distbits,
                            );
                            state.mode = crate::src::inflate::LEN;
                        }
                        2 => {
                            state.mode = crate::src::inflate::TABLE;
                        }
                        _ => {
                            message = Some(b"invalid block type\0");
                            state.mode = crate::src::inflate::BAD;
                        }
                    }
                    hold >>= 2 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(2 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    continue;
                }
            }
            16193 => {
                hold >>= bits & 7 as ::core::ffi::c_uint;
                bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    let Some(byte) = input.pull_byte() else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if hold & 0xffff as ::core::ffi::c_ulong
                    != hold >> 16 as ::core::ffi::c_int ^ 0xffff as ::core::ffi::c_ulong
                {
                    message = Some(b"invalid stored block lengths\0");
                    state.mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    state.length = hold as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint;
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    while state.length != 0 as ::core::ffi::c_uint {
                        copy = state.length;
                        if input.available() == 0 {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                        if output.remaining() == 0 {
                            state.whave = state.wsize;
                            if !output.flush() {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        if copy > input.available() as ::core::ffi::c_uint {
                            copy = input.available() as ::core::ffi::c_uint;
                        }
                        if copy > output.remaining() as ::core::ffi::c_uint {
                            copy = output.remaining() as ::core::ffi::c_uint;
                        }
                        let copied = output
                            .write_with(|bytes| input.copy_to(&mut bytes[..copy as usize]))
                            as ::core::ffi::c_uint;
                        state.length = state.length.wrapping_sub(copied);
                    }
                    state.mode = crate::src::inflate::TYPE;
                    continue;
                }
            }
            16196 => {
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    let Some(byte) = input.pull_byte() else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                state.nlen = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(257 as ::core::ffi::c_uint);
                hold >>= 5 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
                state.ndist = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(1 as ::core::ffi::c_uint);
                hold >>= 5 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
                state.ncode = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(4 as ::core::ffi::c_uint);
                hold >>= 4 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(4 as ::core::ffi::c_int as ::core::ffi::c_uint);
                if state.nlen > 286 as ::core::ffi::c_uint
                    || state.ndist > 30 as ::core::ffi::c_uint
                {
                    message = Some(b"too many length or distance symbols\0");
                    state.mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    state.have = 0 as ::core::ffi::c_uint;
                    while state.have < state.ncode {
                        while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                            let Some(byte) = input.pull_byte() else {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            };
                            hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        let c2rust_fresh4 = state.have;
                        state.have = state.have.wrapping_add(1);
                        state.lens[order[c2rust_fresh4 as usize] as usize] = (hold
                            as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_ushort;
                        hold >>= 3 as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(3 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    }
                    while state.have < 19 as ::core::ffi::c_uint {
                        let c2rust_fresh5 = state.have;
                        state.have = state.have.wrapping_add(1);
                        state.lens[order[c2rust_fresh5 as usize] as usize] =
                            0 as ::core::ffi::c_ushort;
                    }
                    state.next = 0;
                    state.lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                    state.lenbits = 7 as ::core::ffi::c_uint;
                    ret = 'table: {
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
                    if ret != 0 {
                        message = Some(b"invalid code lengths set\0");
                        state.mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        state.have = 0 as ::core::ffi::c_uint;
                        while state.have < state.nlen.wrapping_add(state.ndist) {
                            loop {
                                here = crate::src::inftrees::code::copied_from(
                                    state.lencode.get(
                                        &*state.codes,
                                        (hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint) << state.lenbits)
                                                .wrapping_sub(1 as ::core::ffi::c_uint))
                                            as isize,
                                    ),
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                let Some(byte) = input.pull_byte() else {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                };
                                hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            if (here.val as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                                hold >>= here.bits as ::core::ffi::c_int;
                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                let c2rust_fresh7 = state.have;
                                state.have = state.have.wrapping_add(1);
                                state.lens[c2rust_fresh7 as usize] = here.val;
                            } else {
                                if here.val as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        let Some(byte) = input.pull_byte() else {
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break '_inf_leave;
                                        };
                                        hold = hold
                                            .wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    if state.have == 0 as ::core::ffi::c_uint {
                                        message = Some(b"invalid bit length repeat\0");
                                        state.mode = crate::src::inflate::BAD;
                                        break;
                                    } else {
                                        len = state.lens[state
                                            .have
                                            .wrapping_sub(1 as ::core::ffi::c_uint)
                                            as usize]
                                            as ::core::ffi::c_uint;
                                        copy = (3 as ::core::ffi::c_uint).wrapping_add(
                                            hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint)
                                                    << 2 as ::core::ffi::c_int)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                        );
                                        hold >>= 2 as ::core::ffi::c_int;
                                        bits = bits.wrapping_sub(
                                            2 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        );
                                    }
                                } else if here.val as ::core::ffi::c_int == 17 as ::core::ffi::c_int
                                {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        let Some(byte) = input.pull_byte() else {
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break '_inf_leave;
                                        };
                                        hold = hold
                                            .wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    len = 0 as ::core::ffi::c_uint;
                                    copy = (3 as ::core::ffi::c_uint).wrapping_add(
                                        hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint)
                                                << 3 as ::core::ffi::c_int)
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                    );
                                    hold >>= 3 as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(
                                        3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    );
                                } else {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 7 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        let Some(byte) = input.pull_byte() else {
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break '_inf_leave;
                                        };
                                        hold = hold
                                            .wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    len = 0 as ::core::ffi::c_uint;
                                    copy = (11 as ::core::ffi::c_uint).wrapping_add(
                                        hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint)
                                                << 7 as ::core::ffi::c_int)
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                    );
                                    hold >>= 7 as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(
                                        7 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    );
                                }
                                if state.have.wrapping_add(copy)
                                    > state.nlen.wrapping_add(state.ndist)
                                {
                                    message = Some(b"invalid bit length repeat\0");
                                    state.mode = crate::src::inflate::BAD;
                                    break;
                                } else {
                                    loop {
                                        let c2rust_fresh11 = copy;
                                        copy = copy.wrapping_sub(1);
                                        if c2rust_fresh11 == 0 {
                                            break;
                                        }
                                        let c2rust_fresh12 = state.have;
                                        state.have = state.have.wrapping_add(1);
                                        state.lens[c2rust_fresh12 as usize] =
                                            len as ::core::ffi::c_ushort;
                                    }
                                }
                            }
                        }
                        if state.mode as ::core::ffi::c_uint
                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        if state.lens[256 as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            message = Some(b"invalid code -- missing end-of-block\0");
                            state.mode = crate::src::inflate::BAD;
                            continue;
                        } else {
                            state.next = 0;
                            state.lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                            state.lenbits = 9 as ::core::ffi::c_uint;
                            ret = 'table: {
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
                                message = Some(b"invalid literal/lengths set\0");
                                state.mode = crate::src::inflate::BAD;
                                continue;
                            } else {
                                state.distcode =
                                    crate::src::inflate::CodeTableRef::Dynamic(state.next);
                                state.distbits = 6 as ::core::ffi::c_uint;
                                ret = 'table: {
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
                                    message = Some(b"invalid distances set\0");
                                    state.mode = crate::src::inflate::BAD;
                                    continue;
                                } else {
                                    state.mode = crate::src::inflate::LEN;
                                }
                            }
                        }
                    }
                }
            }
            16200 => {}
            16208 => {
                ret = crate::zlib_h::Z_STREAM_END;
                break;
            }
            16209 => {
                ret = crate::zlib_h::Z_DATA_ERROR;
                break;
            }
            _ => {
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break;
            }
        }
        if input.available() >= 6 && output.remaining() >= 258 {
            // Both callback cursors are bounded for this dispatch: `have`
            // describes the input callback's current chunk, and the output
            // facade retains the checked caller-window cursor. Decode
            // directly through those views instead of republishing them via
            // the legacy raw-stream fast adapter.
            let window_size = output.written().wrapping_add(output.remaining());
            let written = output.written();
            let mut fast_state = crate::src::inffast::InflateFastState {
                history: crate::src::inffast::FastHistory::Output,
                wsize: window_size,
                whave: state.whave as usize,
                wnext: state.wnext as usize,
                hold,
                bits,
                lcode: state.lencode,
                dcode: state.distcode,
                lmask: (1u32 << state.lenbits) - 1,
                dmask: (1u32 << state.distbits) - 1,
                codes: &*state.codes,
                sane: state.sane != 0,
            };
            let mut fast_result = None;
            output.with_window(|window, _| {
                input.with_remaining(|input| {
                    let result = crate::src::inffast::inflate_fast_from_views(
                        input,
                        window,
                        written,
                        &mut fast_state,
                    );
                    let used = result.input_used;
                    fast_result = Some(result);
                    used
                });
            });
            let result = fast_result.expect("fast input visit");
            if !output.set_written(result.output_used) {
                ret = crate::zlib_h::Z_DATA_ERROR;
                break;
            }
            hold = fast_state.hold;
            bits = fast_state.bits;
            match result.exit {
                crate::src::inffast::FastExit::Continue => {}
                crate::src::inffast::FastExit::Type => state.mode = crate::src::inflate::TYPE,
                crate::src::inffast::FastExit::InvalidDistance => {
                    message = Some(b"invalid distance too far back\0");
                    state.mode = crate::src::inflate::BAD;
                }
                crate::src::inffast::FastExit::InvalidCode => {
                    message = Some(b"invalid literal/length or distance code\0");
                    state.mode = crate::src::inflate::BAD;
                }
            }
        } else {
            loop {
                here = crate::src::inftrees::code::copied_from(
                    state.lencode.get(
                        &*state.codes,
                        (hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << state.lenbits)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as isize,
                    ),
                );
                if here.bits as ::core::ffi::c_uint <= bits {
                    break;
                }
                let Some(byte) = input.pull_byte() else {
                    ret = crate::zlib_h::Z_BUF_ERROR;
                    break '_inf_leave;
                };
                hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            }
            if here.op as ::core::ffi::c_int != 0
                && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                last = crate::src::inftrees::code::copied_from(&here);
                loop {
                    here = crate::src::inftrees::code::copied_from(
                        state.lencode.get(
                            &*state.codes,
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
                    if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        <= bits
                    {
                        break;
                    }
                    let Some(byte) = input.pull_byte() else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                hold >>= last.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
            }
            hold >>= here.bits as ::core::ffi::c_int;
            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
            state.length = here.val as ::core::ffi::c_uint;
            if here.op as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if output.remaining() == 0 {
                    state.whave = state.wsize;
                    if !output.flush() {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break;
                    }
                }
                if !output.write_byte(state.length as ::core::ffi::c_uchar) {
                    ret = crate::zlib_h::Z_DATA_ERROR;
                    break;
                }
                state.mode = crate::src::inflate::LEN;
            } else if here.op as ::core::ffi::c_int & 32 as ::core::ffi::c_int != 0 {
                state.mode = crate::src::inflate::TYPE;
            } else if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                message = Some(b"invalid literal/length code\0");
                state.mode = crate::src::inflate::BAD;
            } else {
                state.extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                if state.extra != 0 as ::core::ffi::c_uint {
                    while bits < state.extra {
                        let Some(byte) = input.pull_byte() else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    state.length = state.length.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << state.extra)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= state.extra;
                    bits = bits.wrapping_sub(state.extra);
                }
                loop {
                    here = crate::src::inftrees::code::copied_from(
                        state.distcode.get(
                            &*state.codes,
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << state.distbits)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                as isize,
                        ),
                    );
                    if here.bits as ::core::ffi::c_uint <= bits {
                        break;
                    }
                    let Some(byte) = input.pull_byte() else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    last = crate::src::inftrees::code::copied_from(&here);
                    loop {
                        here = crate::src::inftrees::code::copied_from(
                            state.distcode.get(
                                &*state.codes,
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
                        if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            <= bits
                        {
                            break;
                        }
                        let Some(byte) = input.pull_byte() else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    hold >>= last.bits as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                }
                hold >>= here.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                    message = Some(b"invalid distance code\0");
                    state.mode = crate::src::inflate::BAD;
                } else {
                    state.offset = here.val as ::core::ffi::c_uint;
                    state.extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                    if state.extra != 0 as ::core::ffi::c_uint {
                        while bits < state.extra {
                            let Some(byte) = input.pull_byte() else {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            };
                            hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        state.offset = state.offset.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << state.extra)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= state.extra;
                        bits = bits.wrapping_sub(state.extra);
                    }
                    if state.offset
                        > state.wsize.wrapping_sub(if state.whave < state.wsize {
                            output.remaining() as ::core::ffi::c_uint
                        } else {
                            0 as ::core::ffi::c_uint
                        })
                    {
                        message = Some(b"invalid distance too far back\0");
                        state.mode = crate::src::inflate::BAD;
                    } else {
                        loop {
                            if output.remaining() == 0 {
                                state.whave = state.wsize;
                                if !output.flush() {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            // `offset` was checked against the amount of
                            // history represented by this caller window.
                            // The call-scoped output facade performs the
                            // overlapping copy with a checked slice range.
                            let Some(count) =
                                output.copy_match(state.offset as usize, state.length as usize)
                            else {
                                ret = crate::zlib_h::Z_DATA_ERROR;
                                break '_inf_leave;
                            };
                            state.length = state.length.wrapping_sub(count as u32);
                            if state.length == 0 as ::core::ffi::c_uint {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    if output.written() != 0 && !output.flush() && ret == crate::zlib_h::Z_STREAM_END {
        ret = crate::zlib_h::Z_BUF_ERROR;
    }
    InflateBackDecodeResult {
        status: ret,
        message,
    }
}

// One complete callback-back request is pointer-free once the stream/state
// association has supplied its normal decoder and bounded back window.  In
// particular, the callback invocation ends before the scalar completion is
// committed, so this core cannot retain either ABI cursor provenance or a
// caller window borrow.
fn inflateBack<InputVisitor, OutputVisitor>(
    owner: &mut InflateBackStateOwner<'_>,
    input_visit: InputVisitor,
    output_visit: OutputVisitor,
) -> InflateBackDecodeResult
where
    InputVisitor: FnMut(&mut dyn FnMut(&[::core::ffi::c_uchar]) -> usize),
    OutputVisitor: FnMut(&[::core::ffi::c_uchar]) -> ::core::ffi::c_int,
{
    let (state, window) = owner.begin().expect("inflateBack window geometry");
    let output = InflateBackOutput::new(window, output_visit);
    let input = InflateBackInput::new(input_visit);
    let mut invocation = InflateBackInvocation {
        state,
        input,
        output,
    };
    let completion = invocation.decode();
    // Release callback/window borrows before writing either the backing state
    // or any ABI stream.  The completion above carries only scalar state.
    drop(invocation);
    owner.commit(completion)
}

// Run a complete callback-back request after the shared normal-inflate stream
// adapter has projected the durable decoder payload and owned back window.
// This implementation owns every callback/window borrow and returns only the
// pointer-free scalar completion for that adapter to publish.
fn inflate_back_from_stream<InputVisitor, OutputVisitor>(
    normal: &mut crate::src::inflate::InflateNormalState,
    back_window: &mut crate::src::inflate::InflateBackWindow,
    input_visit: InputVisitor,
    output_visit: OutputVisitor,
) -> crate::src::inflate::InflateBackDispatchResult
where
    InputVisitor: FnMut(&mut dyn FnMut(&[::core::ffi::c_uchar]) -> usize),
    OutputVisitor: FnMut(&[::core::ffi::c_uchar]) -> ::core::ffi::c_int,
{
    let mut owner = InflateBackStateOwner::new(normal, back_window);
    let result = inflateBack(&mut owner, input_visit, output_visit);
    crate::src::inflate::InflateBackDispatchResult {
        status: result.status,
        message: result.message,
    }
}

// This bridges the callback cursor facades supplied by the export to the
// pointer-free request accepted by the shared stream adapter. Its fields are
// generic safe visitors, not persistent ABI handles; each is consumed within
// exactly one dispatch call.
struct InflateBackStreamDispatch<InputVisitor, OutputVisitor> {
    input_visit: InputVisitor,
    output_visit: OutputVisitor,
}

impl<InputVisitor, OutputVisitor> crate::src::inflate::InflateBackDispatch
    for InflateBackStreamDispatch<InputVisitor, OutputVisitor>
where
    InputVisitor: FnMut(&mut dyn FnMut(&[::core::ffi::c_uchar]) -> usize),
    OutputVisitor: FnMut(&[::core::ffi::c_uchar]) -> ::core::ffi::c_int,
{
    fn dispatch(
        &mut self,
        normal: &mut crate::src::inflate::InflateNormalState,
        back_window: &mut crate::src::inflate::InflateBackWindow,
    ) -> crate::src::inflate::InflateBackDispatchResult {
        inflate_back_from_stream(
            normal,
            back_window,
            |consume| (self.input_visit)(consume),
            |bytes| (self.output_visit)(bytes),
        )
    }
}
#[export_name = "inflateBack"]

pub unsafe extern "C" fn inflateBack_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut in_0: crate::zlib_h::in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: crate::zlib_h::out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Raw callback cursor adaptation belongs to the ABI boundary.  The named
    // decoder implementation sees only scoped input slices and output slices;
    // this wrapper retains the callback-provided cursor and republishes it
    // only after that decoder request has completed.
    let mut next = strm.next_in.cast::<::core::ffi::c_uchar>();
    let mut have = if next.is_null() {
        0
    } else {
        strm.avail_in as ::core::ffi::c_uint
    };
    let mut dispatch = InflateBackStreamDispatch {
        input_visit: |consume: &mut dyn FnMut(&[::core::ffi::c_uchar]) -> usize| {
            if have == 0 {
                have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                if have == 0 {
                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    return;
                }
            }
            // The callback cursor is exposed only for this invocation of the
            // continuation. It never escapes as a fabricated long-lived
            // slice or reference.
            let bytes = ::core::slice::from_raw_parts(next, have as usize);
            let used = consume(bytes).min(bytes.len());
            next = next.wrapping_add(used);
            have = have.wrapping_sub(used as ::core::ffi::c_uint);
        },
        output_visit: |bytes: &[::core::ffi::c_uchar]| {
            out.expect("non-null function pointer")(
                out_desc,
                bytes.as_ptr().cast_mut(),
                bytes.len() as u32,
            )
        },
    };
    let status = crate::src::inflate::inflate_from_stream(
        strm,
        crate::src::inflate::InflateStreamRequest::Back(&mut dispatch),
        None,
        None,
    );
    strm.next_in = next.cast::<crate::stdlib::Bytef>();
    strm.avail_in = have as crate::stdlib::uInt;
    status.status()
}
#[export_name = "inflateBackEnd"]

pub unsafe extern "C" fn inflateBackEnd_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    crate::src::inflate::inflate_from_stream(
        strm,
        crate::src::inflate::InflateStreamRequest::End,
        None,
        None,
    )
    .status()
}
