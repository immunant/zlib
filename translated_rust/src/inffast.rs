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
            InflateFastError::DistanceTooFarBack => {
                crate::src::inflate::INFLATE_MSG_DISTANCE_TOO_FAR_BACK.as_ptr()
            }
            InflateFastError::InvalidDistanceCode => {
                crate::src::inflate::INFLATE_MSG_INVALID_DISTANCE_CODE.as_ptr()
            }
            InflateFastError::InvalidLengthCode => {
                crate::src::inflate::INFLATE_MSG_INVALID_LITERAL_LENGTH_CODE.as_ptr()
            }
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
                        hold =
                            hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
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
                            hold = hold
                                .wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
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
                            let mut from =
                                (state.wsize.wrapping_add(state.wnext).wrapping_sub(needed)
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

// `inflate_fast()` is normally entered only from an inflater that has at
// least the fast loop's six input bytes and 258 output bytes available. Keep
// that internal contract explicit before the raw cursor adapter constructs
// its views. In addition to documenting the loop's indexing assumptions,
// this prevents a direct ABI call with incomplete cursors from doing cursor
// arithmetic at all.
struct InflateFastCursors {
    used: usize,
    output_len: usize,
}

// `from_raw_parts()` requires a Rust slice length, not merely a zlib `uInt`.
// Keep that platform limit in the safe preflight so the remaining cursor
// adapter never tries to construct an unrepresentable foreign view.
fn inflate_fast_view_len_is_valid(len: usize) -> bool {
    len <= isize::MAX as usize
}

fn inflate_fast_cursor_lengths(
    strm: &crate::zlib_h::z_stream,
    start: ::core::ffi::c_uint,
) -> Option<InflateFastCursors> {
    if strm.avail_in < 6 || strm.avail_out < 258 {
        return None;
    }
    let used = start.checked_sub(strm.avail_out)? as usize;
    let output_len = used.checked_add(strm.avail_out as usize)?;
    if !inflate_fast_view_len_is_valid(strm.avail_in as usize)
        || !inflate_fast_view_len_is_valid(output_len)
    {
        return None;
    }
    Some(InflateFastCursors { used, output_len })
}

// A regular inflater establishes these scalar invariants before entering the
// fast loop. Keep them explicit for the raw adapter as well: unlike the
// bounded table views below, a malformed root width or window cursor would
// otherwise reach a shift or modulo operation before it can be rejected.
fn inflate_fast_state_is_usable(state: &crate::src::inflate::inflate_state) -> bool {
    let word_bits = ::core::ffi::c_uint::BITS;
    if state.bits > 32 || state.lenbits == 0 || state.distbits == 0 {
        return false;
    }
    if state.lenbits >= word_bits || state.distbits >= word_bits {
        return false;
    }
    if state.wsize == 0 {
        // Without a window, the normal distance check must reject every
        // reference that reaches behind the output produced in this pass.
        // An undermined stream deliberately bypasses that check, so it still
        // needs an initialized window before the fast path can run.
        return state.whave == 0 && state.sane != 0;
    }
    state.wnext < state.wsize
}

// Once its caller has bound the stream cursors, the fast decoder is entirely
// reference- and slice-based. Keeping the cursor binding in the adapter
// below removes raw pointer work from this core implementation.
fn inflate_fast_slices(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    used: usize,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) {
    // The cursor adapter already validated the stream/state pair. Snapshot
    // the scalar fast-loop state before its bounded decode pass.
    let mut fast_state = InflateFastState::from(&*state);
    let result = crate::src::inflate::updatewindow(
        strm,
        state,
        crate::src::inflate::InflateWindowAccess::Existing,
        |_, state, window| {
            let window = window.as_deref().unwrap_or(&[]);
            // Fixed tables are static; dynamic tables are checked subranges
            // of the state-owned workspace.  Resolve both through the
            // inflater's cursor validator so this slice-only core does not
            // repeat raw table-address arithmetic.
            let Some(lcode) = crate::src::inflate::inflate_code_table(
                state,
                crate::src::inflate::InflateCodeTable::Length,
            ) else {
                return None;
            };
            let Some(dcode) = crate::src::inflate::inflate_code_table(
                state,
                crate::src::inflate::InflateCodeTable::Distance,
            ) else {
                return None;
            };
            Some(inflate_fast_bound(
                &mut fast_state,
                input,
                output,
                window,
                lcode,
                dcode,
                used,
            ))
        },
    )
    .expect("existing-window access cannot allocate or fail");
    // `inflate_code_table()` also validates the state-owned dynamic cursor.
    // A direct call with a malformed state must leave the stream untouched,
    // rather than turning that invariant failure into a Rust panic.
    let Some(result) = result else {
        return;
    };
    finish_inflate_fast(strm, state, fast_state, input, output, result);
}

// Once a caller has bound the two caller-owned cursor ranges, fast inflation
// has no raw-pointer work left. Keep that dispatch separate from the cursor
// adapter so regular and inflateBack callers share the same slice-only
// decoder path.
fn inflate_fast_dispatch(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    cursors: InflateFastCursors,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) {
    inflate_fast_slices(strm, state, cursors.used, input, output)
}

// Regular `inflate()` has already bound its complete input and output ranges.
// Reuse those checked views for the fast loop so it does not recreate raw
// cursor slices after the main decoder has validated them.
pub(crate) fn inflate_fast_bound_cursors(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    start: ::core::ffi::c_uint,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) {
    let Some(cursors) = inflate_fast_cursor_lengths(strm, start) else {
        return;
    };
    if !inflate_fast_state_is_usable(state)
        || input.len() != strm.avail_in as usize
        || output.len() != cursors.output_len
    {
        return;
    }
    inflate_fast_dispatch(strm, state, cursors, input, output)
}

// `inflate_fast` is an internal C symbol, not an application entry point.
// Regular inflation reaches the bounded fast core above after it has already
// borrowed both caller cursors. A direct internal-symbol call has no such
// views, so use the reference-bound full inflater instead of recreating a
// second raw cursor adapter. Direct internal-symbol callers therefore use
// the full inflater as a functional fallback, while the ABI forwarder stays
// thin.
pub fn inflate_fast(
    strm: Option<&mut crate::zlib_h::z_stream>,
    start: ::core::ffi::c_uint,
    input: &[crate::stdlib::Bytef],
    output: &mut [crate::stdlib::Bytef],
) {
    let Some(strm) = strm else {
        return;
    };
    let _ = start;
    let _ = crate::src::inflate::inflate_stream(
        strm,
        crate::zlib_h::Z_NO_FLUSH,
        Some(input),
        output,
        None,
    );
}

#[export_name = "inflate_fast"]
pub unsafe extern "C" fn inflate_fast_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    // SAFETY: the ABI adapter only binds the optional stream reference. The
    // implementation owns the fallback dispatch and stream validation.
    let strm = unsafe { strm.as_mut() };
    let Some(strm) = strm else {
        return;
    };
    if strm.next_out.is_null()
        || strm.next_in.is_null() && strm.avail_in != 0 as crate::stdlib::uInt
    {
        return;
    }
    // SAFETY: `inflate_fast` shares inflate's public cursor contract. This
    // thin ABI adapter binds both advertised cursor ranges.
    let input = if strm.avail_in == 0 {
        &[]
    } else {
        unsafe { ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize) }
    };
    let output =
        unsafe { ::core::slice::from_raw_parts_mut(strm.next_out, strm.avail_out as usize) };
    inflate_fast(Some(strm), start, input, output)
}
