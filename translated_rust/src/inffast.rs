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
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
#[derive(Clone, Copy)]
enum InflateFastError {
    DistanceTooFarBack,
    InvalidDistanceCode,
    InvalidLengthCode,
}

struct InflateFastResult {
    input_index: usize,
    output_index: usize,
    error: Option<InflateFastError>,
}

// The fast loop only changes these three inflater fields.  Snapshot the
// scalars it reads as well, so its decode tables can remain borrowed from the
// state's owned workspace for the whole bounded operation.
struct InflateFastState {
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    lenbits: ::core::ffi::c_uint,
    distbits: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    sane: ::core::ffi::c_int,
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    mode: crate::src::inflate::inflate_mode,
}

impl From<&crate::src::inflate::inflate_state> for InflateFastState {
    fn from(state: &crate::src::inflate::inflate_state) -> Self {
        Self {
            hold: state.hold,
            bits: state.bits,
            lenbits: state.lenbits,
            distbits: state.distbits,
            whave: state.whave,
            sane: state.sane,
            wsize: state.wsize,
            wnext: state.wnext,
            mode: state.mode,
        }
    }
}

// The fast loop reports positions relative to its already-bound input and
// output views.  Publishing them through slice tails keeps the cursor update
// bounds-checked and separate from the raw stream binding at the ABI edge.
fn publish_inflate_fast_result(
    strm: &mut crate::zlib_h::z_stream,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    result: InflateFastResult,
) {
    if let Some(error) = result.error {
        strm.msg = match error {
            InflateFastError::DistanceTooFarBack => b"invalid distance too far back\0".as_ptr(),
            InflateFastError::InvalidDistanceCode => b"invalid distance code\0".as_ptr(),
            InflateFastError::InvalidLengthCode => b"invalid literal/length code\0".as_ptr(),
        } as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    strm.next_in = input[result.input_index..].as_ptr() as *mut crate::stdlib::Bytef;
    strm.next_out = output[result.output_index..].as_mut_ptr();
    strm.avail_in = (input.len() - result.input_index) as crate::stdlib::uInt;
    strm.avail_out = (output.len() - result.output_index) as crate::stdlib::uInt;
}

fn inflate_fast_impl(
    state: &mut InflateFastState,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    window: &[crate::stdlib::Bytef],
    lcode: &[crate::src::inftrees::code],
    dcode: &[crate::src::inftrees::code],
    used: usize,
) -> InflateFastResult {
    let mut input_index = 0usize;
    let mut output_index = used;
    let mut hold = state.hold;
    let mut bits = state.bits;
    let lmask = ((1 as ::core::ffi::c_uint) << state.lenbits).wrapping_sub(1);
    let dmask = ((1 as ::core::ffi::c_uint) << state.distbits).wrapping_sub(1);
    let mut error = None;
    'outer: loop {
        if bits < 15 {
            hold = hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
            input_index += 1;
            bits = bits.wrapping_add(8);
            hold = hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
            input_index += 1;
            bits = bits.wrapping_add(8);
        }
        let mut here_index = (hold & lmask as ::core::ffi::c_ulong) as usize;
        'literal: loop {
            let mut here = lcode[here_index];
            let mut op = here.bits as ::core::ffi::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here.op as ::core::ffi::c_uint;
            if op == 0 {
                output[output_index] = here.val as ::core::ffi::c_uchar;
                output_index += 1;
                break;
            }
            if op & 16 != 0 {
                let mut len = here.val as ::core::ffi::c_uint;
                op &= 15;
                if op != 0 {
                    if bits < op {
                        hold = hold.wrapping_add(
                            (input[input_index] as ::core::ffi::c_ulong) << bits,
                        );
                        input_index += 1;
                        bits = bits.wrapping_add(8);
                    }
                    len = len.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1),
                    );
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                }
                if bits < 15 {
                    hold = hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
                    input_index += 1;
                    bits = bits.wrapping_add(8);
                    hold = hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
                    input_index += 1;
                    bits = bits.wrapping_add(8);
                }
                here_index = (hold & dmask as ::core::ffi::c_ulong) as usize;
                loop {
                    here = dcode[here_index];
                    op = here.bits as ::core::ffi::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here.op as ::core::ffi::c_uint;
                    if op & 16 != 0 {
                        let mut dist = here.val as ::core::ffi::c_uint;
                        op &= 15;
                        if bits < op {
                            hold = hold.wrapping_add(
                                (input[input_index] as ::core::ffi::c_ulong) << bits,
                            );
                            input_index += 1;
                            bits = bits.wrapping_add(8);
                            if bits < op {
                                hold = hold.wrapping_add(
                                    (input[input_index] as ::core::ffi::c_ulong) << bits,
                                );
                                input_index += 1;
                                bits = bits.wrapping_add(8);
                            }
                        }
                        dist = dist.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1),
                        );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        let produced = output_index as ::core::ffi::c_uint;
                        if dist > produced {
                            let needed = dist.wrapping_sub(produced);
                            if needed > state.whave && state.sane != 0 {
                                state.mode = crate::src::inflate::BAD;
                                error = Some(InflateFastError::DistanceTooFarBack);
                                break 'outer;
                            }
                            let mut from = (state.wsize.wrapping_add(state.wnext)
                                .wrapping_sub(needed)
                                % state.wsize) as usize;
                            let from_window = len.min(needed) as usize;
                            for _ in 0..from_window {
                                output[output_index] = window[from];
                                output_index += 1;
                                from += 1;
                                if from == window.len() {
                                    from = 0;
                                }
                            }
                            len = len.wrapping_sub(from_window as ::core::ffi::c_uint);
                        }
                        while len != 0 {
                            output[output_index] = output[output_index - dist as usize];
                            output_index += 1;
                            len = len.wrapping_sub(1);
                        }
                        break 'literal;
                    }
                    if op & 64 == 0 {
                        here_index = here.val as usize
                            + (hold
                                & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1)
                                    as ::core::ffi::c_ulong) as usize;
                    } else {
                        state.mode = crate::src::inflate::BAD;
                        error = Some(InflateFastError::InvalidDistanceCode);
                        break 'outer;
                    }
                }
            } else if op & 64 == 0 {
                here_index = here.val as usize
                    + (hold
                        & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1)
                            as ::core::ffi::c_ulong) as usize;
            } else if op & 32 != 0 {
                state.mode = crate::src::inflate::TYPE;
                break 'outer;
            } else {
                state.mode = crate::src::inflate::BAD;
                error = Some(InflateFastError::InvalidLengthCode);
                break 'outer;
            }
        }
        if input_index >= input.len() - 5 || output_index + 257 > output.len() {
            break;
        }
    }
    let back = (bits >> 3) as usize;
    input_index -= back;
    bits = bits.wrapping_sub((back as ::core::ffi::c_uint) << 3);
    hold &= ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1) as ::core::ffi::c_ulong;
    state.hold = hold;
    state.bits = bits;
    InflateFastResult {
        input_index,
        output_index,
        error,
    }
}

// Once the stream-owned views have been bound, fast inflation is ordinary
// slice and scalar-state work. The full inflater can keep its decode-table
// workspace borrowed during this operation, rather than recreating raw views
// of those tables at the cursor adapter.
fn inflate_fast_bound(
    state: &mut InflateFastState,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    window: &[crate::stdlib::Bytef],
    lcode: &[crate::src::inftrees::code],
    dcode: &[crate::src::inftrees::code],
    used: usize,
) -> InflateFastResult {
    inflate_fast_impl(state, input, output, window, lcode, dcode, used)
}

// Apply the fast loop's state changes before publishing the advanced cursors,
// matching the original decoder ordering without retaining a raw table view.
fn finish_inflate_fast(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    fast_state: InflateFastState,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
    result: InflateFastResult,
) {
    state.hold = fast_state.hold;
    state.bits = fast_state.bits;
    state.mode = fast_state.mode;
    publish_inflate_fast_result(strm, input, output, result);
}

// The raw stream cursors are bound in one narrow scope, after which the
// decoder remains entirely reference- and slice-based in `inflate_fast_bound`.
// Keeping this adapter safe prevents internal callers from inheriting an
// unnecessary unsafe-function contract.
pub fn inflate_fast(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    // Reuse the inflater's established raw state adapter instead of
    // dereferencing the stream and state cursors a second time here.
    let Some((strm, state)) = crate::src::inflate::inflateStateCheck(strm) else {
        return;
    };
    // `inflateStateCheck()` already returned a validated reference. Snapshot
    // the scalar fast-loop state before binding the remaining raw cursors.
    let mut fast_state = InflateFastState::from(&*state);
    // SAFETY: `inflate()` invokes this adapter only after the checked
    // stream/state pair above has been validated. Its input and output
    // cursors are the bounded ranges maintained by that same state machine
    // for this call. The optional prior window is instead bound by
    // `updatewindow()` below, at its single state-owned allocation boundary.
    unsafe {
        let input = ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize);
        let used = start.wrapping_sub(strm.avail_out) as usize;
        let output = ::core::slice::from_raw_parts_mut(
            strm.next_out.wrapping_sub(used),
            used + strm.avail_out as usize,
        );
        let result = crate::src::inflate::updatewindow(
            strm,
            state,
            crate::src::inflate::InflateWindowAccess::Existing,
            |state, window| {
                let window = window.as_deref().unwrap_or(&[]);
                let lcode_fixed = ::core::ptr::eq(
                    state.lencode,
                    crate::src::inftrees::inffixed_h::lenfix.as_ptr(),
                );
                let dcode_fixed = ::core::ptr::eq(
                    state.distcode,
                    crate::src::inftrees::inffixed_h::distfix.as_ptr(),
                );
                // Dynamic decode tables are subranges of the state-owned
                // workspace. Fixed tables retain their static slices.
                let code_base = state.codes.as_ptr().addr();
                let code_size = ::core::mem::size_of::<crate::src::inftrees::code>();
                let lcode = if lcode_fixed {
                    &crate::src::inftrees::inffixed_h::lenfix[..]
                } else {
                    let start = state.lencode.addr().wrapping_sub(code_base) / code_size;
                    &state.codes[start..start + crate::src::inftrees::ENOUGH_LENS as usize]
                };
                let dcode = if dcode_fixed {
                    &crate::src::inftrees::inffixed_h::distfix[..]
                } else {
                    let start = state.distcode.addr().wrapping_sub(code_base) / code_size;
                    &state.codes[start..start + crate::src::inftrees::ENOUGH_DISTS as usize]
                };
                inflate_fast_bound(
                    &mut fast_state,
                    input,
                    output,
                    window,
                    lcode,
                    dcode,
                    used,
                )
            },
        )
        .expect("existing-window access cannot allocate or fail");
        finish_inflate_fast(strm, state, fast_state, input, output, result);
    }
}
#[export_name = "inflate_fast"]

pub unsafe extern "C" fn inflate_fast_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    inflate_fast(strm, start)
}
