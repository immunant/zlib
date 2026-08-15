use crate::src::inflate::{inflate_state, length_table, BAD, TYPE};
use crate::src::inftrees::code;

#[derive(Copy, Clone)]
enum FastError {
    DistanceTooFarBack,
    InvalidDistanceCode,
    InvalidLiteralLengthCode,
}

pub(crate) struct FastProgress {
    input_consumed: usize,
    output_written: usize,
    error: Option<FastError>,
}

/// The part of an inflate state used by the fast decoder.  This deliberately
/// excludes ABI-only fields such as the retained gzip-header pointer, allowing
/// the decoder itself to be an ordinary safe function.
struct FastState<'a> {
    mode: &'a mut crate::src::inflate::inflate_mode,
    wsize: usize,
    whave: usize,
    wnext: usize,
    window: Option<&'a [u8]>,
    hold: &'a mut crate::stdlib::uLong,
    bits: &'a mut crate::stdlib::uInt,
    lencode: length_table,
    distcode: crate::src::inflate::distance_table,
    lenbits: u32,
    distbits: u32,
    codes: &'a [code; 1444],
    sane: ::core::ffi::c_int,
}

fn bit_mask(bits: u32) -> u64 {
    if bits >= u64::BITS {
        u64::MAX
    } else if bits == 0 {
        0
    } else {
        (1_u64 << bits) - 1
    }
}

fn pull_byte(input: &[u8], input_at: &mut usize, hold: &mut u64, bits: &mut u32) -> bool {
    let Some(&byte) = input.get(*input_at) else {
        return false;
    };
    *input_at += 1;
    *hold = hold.wrapping_add((byte as u64) << *bits);
    *bits += 8;
    true
}

/// Copy a decoded match from the output already produced in this `inflate()`
/// call or from the history ring.  `output_at` is an index in the complete
/// output arena, rather than an interior pointer, so matches that reach bytes
/// produced before the fast path remain bounded.
fn copy_match(
    state: &FastState<'_>,
    output: &mut [u8],
    output_at: &mut usize,
    length: usize,
    distance: usize,
) -> bool {
    if distance == 0 || *output_at > output.len() || length > output.len() - *output_at {
        return false;
    }

    let before = *output_at;
    let window_needed = distance.saturating_sub(before);
    if window_needed != 0 {
        let wsize = state.wsize;
        let whave = state.whave;
        let wnext = state.wnext;
        let Some(window) = state.window else {
            return false;
        };
        if wsize == 0 || window.len() < wsize || wnext >= wsize {
            return false;
        }
        // With `sane` disabled, zlib deliberately permits references older
        // than `whave`; the allocated ring still supplies those bytes.
        if window_needed > whave && state.sane != 0 {
            return false;
        }
        for copied in 0..length {
            let byte = if copied < window_needed {
                let source = (wnext + wsize - (window_needed % wsize) + copied) % wsize;
                window[source]
            } else {
                output[before + copied - distance]
            };
            output[before + copied] = byte;
        }
    } else {
        for copied in 0..length {
            let byte = output[before + copied - distance];
            output[before + copied] = byte;
        }
    }
    *output_at += length;
    true
}

/// Decode the fast-path portion of an inflate block using bounded cursors.
///
/// `output_at` identifies the current output cursor within the complete
/// buffer supplied to the surrounding `inflate()` call.  This retains access
/// to output produced before the fast path without reconstructing an interior
/// pointer.
fn inflate_fast_impl(
    state: &mut FastState<'_>,
    input: &[u8],
    output: &mut [u8],
    mut output_at: usize,
) -> FastProgress {
    let initial_output_at = output_at;
    let mut input_at = 0usize;
    let mut hold = *state.hold as u64;
    let mut bits = *state.bits as u32;
    let mut error = None;

    let Some(input_limit) = input.len().checked_sub(5) else {
        return FastProgress {
            input_consumed: 0,
            output_written: 0,
            error,
        };
    };
    let Some(output_limit) = output.len().checked_sub(257) else {
        return FastProgress {
            input_consumed: 0,
            output_written: 0,
            error,
        };
    };
    if output_at > output_limit {
        return FastProgress {
            input_consumed: 0,
            output_written: 0,
            error,
        };
    }

    let lcode = state.lencode;
    let lmask = bit_mask(state.lenbits);
    let dmask = bit_mask(state.distbits);

    'decode: while input_at < input_limit && output_at < output_limit {
        if bits < 15 {
            if !pull_byte(input, &mut input_at, &mut hold, &mut bits)
                || !pull_byte(input, &mut input_at, &mut hold, &mut bits)
            {
                break;
            }
        }
        let mut here: code = lcode.entry(state.codes, (hold & lmask) as usize);
        loop {
            let op = here.bits as u32;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            let operation = here.op as u32;
            if operation == 0 {
                let Some(slot) = output.get_mut(output_at) else {
                    break 'decode;
                };
                *slot = here.val as u8;
                output_at += 1;
                break;
            }
            if operation & 16 != 0 {
                let extra = operation & 15;
                let mut length = here.val as u32;
                if extra != 0 {
                    if bits < extra && !pull_byte(input, &mut input_at, &mut hold, &mut bits) {
                        break 'decode;
                    }
                    length = length.wrapping_add((hold & bit_mask(extra)) as u32);
                    hold >>= extra;
                    bits = bits.wrapping_sub(extra);
                }
                if bits < 15 {
                    if !pull_byte(input, &mut input_at, &mut hold, &mut bits)
                        || !pull_byte(input, &mut input_at, &mut hold, &mut bits)
                    {
                        break 'decode;
                    }
                }
                let mut dist_here: code =
                    state.distcode.entry(state.codes, (hold & dmask) as usize);
                let distance = loop {
                    let op = dist_here.bits as u32;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    let operation = dist_here.op as u32;
                    if operation & 16 != 0 {
                        let extra = operation & 15;
                        let mut distance = dist_here.val as u32;
                        if bits < extra {
                            if !pull_byte(input, &mut input_at, &mut hold, &mut bits) {
                                break 'decode;
                            }
                            if bits < extra
                                && !pull_byte(input, &mut input_at, &mut hold, &mut bits)
                            {
                                break 'decode;
                            }
                        }
                        distance = distance.wrapping_add((hold & bit_mask(extra)) as u32);
                        hold >>= extra;
                        bits = bits.wrapping_sub(extra);
                        break distance as usize;
                    }
                    if operation & 64 == 0 {
                        let index = dist_here.val as usize + (hold & bit_mask(operation)) as usize;
                        dist_here = state.distcode.entry(state.codes, index);
                    } else {
                        *state.mode = BAD;
                        error = Some(FastError::InvalidDistanceCode);
                        break 'decode;
                    }
                };

                let produced = output_at;
                let window_needed = distance.saturating_sub(produced);
                if window_needed > state.whave && state.sane != 0 {
                    *state.mode = BAD;
                    error = Some(FastError::DistanceTooFarBack);
                    break 'decode;
                }
                if !copy_match(state, output, &mut output_at, length as usize, distance) {
                    *state.mode = BAD;
                    error = Some(FastError::DistanceTooFarBack);
                    break 'decode;
                }
                break;
            }
            if operation & 64 == 0 {
                let index = here.val as usize + (hold & bit_mask(operation)) as usize;
                here = lcode.entry(state.codes, index);
            } else if operation & 32 != 0 {
                *state.mode = TYPE;
                break 'decode;
            } else {
                *state.mode = BAD;
                error = Some(FastError::InvalidLiteralLengthCode);
                break 'decode;
            }
        }
    }

    let whole_bytes = (bits >> 3) as usize;
    input_at = input_at.saturating_sub(whole_bytes);
    bits -= (whole_bytes as u32) << 3;
    hold &= bit_mask(bits);
    *state.hold = hold as _;
    *state.bits = bits as _;
    FastProgress {
        input_consumed: input_at,
        output_written: output_at - initial_output_at,
        error,
    }
}

/// Adapt the ABI-carrying inflate state to the pointer-free fast decoder.
fn inflate_fast_dispatch(
    state: &mut inflate_state,
    input: &[u8],
    output: &mut [u8],
    output_at: usize,
) -> FastProgress {
    let mut fast_state = FastState {
        mode: &mut state.mode,
        wsize: state.wsize as usize,
        whave: state.whave as usize,
        wnext: state.wnext as usize,
        window: state.window.as_deref(),
        hold: &mut state.hold,
        bits: &mut state.bits,
        lencode: state.lencode,
        distcode: state.distcode,
        lenbits: state.lenbits,
        distbits: state.distbits,
        codes: &state.codes,
        sane: state.sane,
    };
    inflate_fast_impl(&mut fast_state, input, output, output_at)
}

#[export_name = "inflate_fast"]
pub unsafe extern "C" fn inflate_fast_ffi(
    strm: crate::zlib_h::z_streamp,
    start: ::core::ffi::c_uint,
) {
    let Some(strm) = strm.as_mut() else {
        return;
    };
    let state = strm.state.cast::<inflate_state>();
    let Some(state) = state.as_mut() else {
        return;
    };
    let input = if strm.avail_in == 0 {
        &[]
    } else {
        if strm.next_in.is_null() {
            return;
        }
        ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
    };
    let total_output = start as usize;
    let available_output = strm.avail_out as usize;
    let Some(output_at) = total_output.checked_sub(available_output) else {
        return;
    };
    let output: &mut [u8] = if total_output == 0 {
        &mut []
    } else {
        if strm.next_out.is_null() {
            return;
        }
        let base = strm.next_out.sub(output_at);
        ::core::slice::from_raw_parts_mut(base, total_output)
    };
    let progress = inflate_fast_dispatch(state, input, output, output_at);
    if progress.input_consumed != 0 {
        strm.next_in = strm.next_in.add(progress.input_consumed);
        strm.avail_in -= progress.input_consumed as crate::stdlib::uInt;
    }
    if progress.output_written != 0 {
        strm.next_out = strm.next_out.add(progress.output_written);
        strm.avail_out -= progress.output_written as crate::stdlib::uInt;
    }
    if let Some(error) = progress.error {
        strm.msg = match error {
            FastError::DistanceTooFarBack => b"invalid distance too far back\0".as_ptr(),
            FastError::InvalidDistanceCode => b"invalid distance code\0".as_ptr(),
            FastError::InvalidLiteralLengthCode => b"invalid literal/length code\0".as_ptr(),
        } as *mut ::core::ffi::c_char;
    }
}
