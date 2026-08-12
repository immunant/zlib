pub use crate::src::inflate::{BAD, TYPE};

use crate::src::inflate::inflate_state;

/// The effect of one fast inflate pass on its caller's buffers.
pub struct InflateFastResult {
    pub input_used: usize,
    pub output_used: usize,
    pub message: Option<&'static ::core::ffi::CStr>,
}

/// Inflate as many literal/length codes as can be decoded without refilling
/// `input` or growing `output`.
///
/// `output[..out]` is the data already emitted in the current window and
/// `output[out..]` is free space.  Keeping both in one slice is important:
/// matches may refer to data emitted before this call as well as to the
/// sliding history window in `state`.
pub fn inflate_fast(
    input: &[u8],
    output: &mut [u8],
    mut out: usize,
    state: &mut inflate_state,
    history_is_output: bool,
) -> InflateFastResult {
    let input_len = input.len();
    let out_start = out;
    let mut input_at = 0usize;
    let mut hold = state.hold;
    let mut bits = state.bits;
    let lcode = state.lencode;
    let dcode = state.distcode;
    let lmask = 1u32.checked_shl(state.lenbits).unwrap_or(0).wrapping_sub(1);
    let dmask = 1u32
        .checked_shl(state.distbits)
        .unwrap_or(0)
        .wrapping_sub(1);
    let mut message = None;

    // The C fast path needs these reserves before its first iteration.  The
    // regular decoder handles smaller buffers.
    if input_len < 6 || output.len().saturating_sub(out) < 258 {
        return InflateFastResult {
            input_used: 0,
            output_used: 0,
            message,
        };
    }
    let last = input_len - 5;
    let end = output.len() - 257;

    'decode: loop {
        if bits < 15 {
            let Some(&byte) = input.get(input_at) else {
                break 'decode;
            };
            input_at += 1;
            hold = hold.wrapping_add((byte as u64) << bits);
            bits += 8;
            let Some(&byte) = input.get(input_at) else {
                break 'decode;
            };
            input_at += 1;
            hold = hold.wrapping_add((byte as u64) << bits);
            bits += 8;
        }

        let mut here = state.code_at(lcode, (hold as u32 & lmask) as usize);
        loop {
            let mut op = here.bits as u32;
            hold >>= op;
            bits -= op;
            op = here.op as u32;
            if op == 0 {
                if out == output.len() {
                    break 'decode;
                }
                output[out] = here.val as u8;
                out += 1;
                break;
            }

            if op & 16 != 0 {
                let mut len = here.val as u32;
                op &= 15;
                if op != 0 {
                    if bits < op {
                        let Some(&byte) = input.get(input_at) else {
                            break 'decode;
                        };
                        input_at += 1;
                        hold = hold.wrapping_add((byte as u64) << bits);
                        bits += 8;
                    }
                    len = len.wrapping_add(hold as u32 & ((1u32 << op) - 1));
                    hold >>= op;
                    bits -= op;
                }

                if bits < 15 {
                    let Some(&byte) = input.get(input_at) else {
                        break 'decode;
                    };
                    input_at += 1;
                    hold = hold.wrapping_add((byte as u64) << bits);
                    bits += 8;
                    let Some(&byte) = input.get(input_at) else {
                        break 'decode;
                    };
                    input_at += 1;
                    hold = hold.wrapping_add((byte as u64) << bits);
                    bits += 8;
                }

                here = state.code_at(dcode, (hold as u32 & dmask) as usize);
                loop {
                    op = here.bits as u32;
                    hold >>= op;
                    bits -= op;
                    op = here.op as u32;
                    if op & 16 != 0 {
                        let mut dist = here.val as u32;
                        op &= 15;
                        if bits < op {
                            let Some(&byte) = input.get(input_at) else {
                                break 'decode;
                            };
                            input_at += 1;
                            hold = hold.wrapping_add((byte as u64) << bits);
                            bits += 8;
                            if bits < op {
                                let Some(&byte) = input.get(input_at) else {
                                    break 'decode;
                                };
                                input_at += 1;
                                hold = hold.wrapping_add((byte as u64) << bits);
                                bits += 8;
                            }
                        }
                        dist = dist.wrapping_add(hold as u32 & ((1u32 << op) - 1));
                        hold >>= op;
                        bits -= op;

                        let distance = dist as usize;
                        if distance > out {
                            let mut history = distance - out;
                            if history > state.whave as usize && state.sane != 0 {
                                state.mode = BAD;
                                message = Some(c"invalid distance too far back");
                                break 'decode;
                            }
                            while len != 0 {
                                if out == output.len() {
                                    break 'decode;
                                }
                                let byte = if history != 0 {
                                    let wsize = state.wsize as usize;
                                    let Some(index) = wsize
                                        .checked_add(state.wnext as usize)
                                        .and_then(|index| index.checked_sub(history))
                                        .map(|index| index % wsize)
                                    else {
                                        state.mode = BAD;
                                        message = Some(c"invalid distance too far back");
                                        break 'decode;
                                    };
                                    let byte = if history_is_output {
                                        let Some(&byte) = output.get(index) else {
                                            state.mode = BAD;
                                            message = Some(c"invalid distance too far back");
                                            break 'decode;
                                        };
                                        byte
                                    } else {
                                        let Some(window) = state.window.as_ref() else {
                                            state.mode = BAD;
                                            message = Some(c"invalid distance too far back");
                                            break 'decode;
                                        };
                                        let Some(&byte) = window.get(index) else {
                                            state.mode = BAD;
                                            message = Some(c"invalid distance too far back");
                                            break 'decode;
                                        };
                                        byte
                                    };
                                    history -= 1;
                                    byte
                                } else {
                                    let Some(source) = out.checked_sub(distance) else {
                                        state.mode = BAD;
                                        message = Some(c"invalid distance too far back");
                                        break 'decode;
                                    };
                                    output[source]
                                };
                                output[out] = byte;
                                out += 1;
                                len -= 1;
                            }
                            break;
                        }

                        while len != 0 {
                            if out == output.len() {
                                break 'decode;
                            }
                            output[out] = output[out - distance];
                            out += 1;
                            len -= 1;
                        }
                        break;
                    }
                    if op & 64 == 0 {
                        here = state.code_at(
                            dcode,
                            here.val as usize + (hold as u32 & ((1u32 << op) - 1)) as usize,
                        );
                    } else {
                        state.mode = BAD;
                        message = Some(c"invalid distance code");
                        break 'decode;
                    }
                }
                break;
            }

            if op & 64 == 0 {
                here = state.code_at(
                    lcode,
                    here.val as usize + (hold as u32 & ((1u32 << op) - 1)) as usize,
                );
            } else if op & 32 != 0 {
                state.mode = TYPE;
                break 'decode;
            } else {
                state.mode = BAD;
                message = Some(c"invalid literal/length code");
                break 'decode;
            }
        }

        if input_at >= last || out >= end {
            break;
        }
    }

    let unused = (bits >> 3) as usize;
    input_at = input_at.saturating_sub(unused);
    bits -= (unused as u32) << 3;
    hold &= (1u64.checked_shl(bits).unwrap_or(0)).wrapping_sub(1);
    state.hold = hold;
    state.bits = bits;
    InflateFastResult {
        input_used: input_at.min(input_len),
        output_used: out.saturating_sub(out_start),
        message,
    }
}

#[export_name = "inflate_fast"]
pub unsafe extern "C" fn inflate_fast_ffi(
    strm: crate::zlib_h::z_streamp,
    start: ::core::ffi::c_uint,
) {
    let Some(strm) = strm.as_mut() else {
        return;
    };
    let Some(state_handle) = strm.inflate_state() else {
        return;
    };
    let mut state = state_handle.borrow_mut();
    let input_len = strm.avail_in as usize;
    let output_free = strm.avail_out as usize;
    let start = start as usize;
    let written = start.saturating_sub(output_free);
    let input = if input_len == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(crate::input_pointer!(strm.next_in), input_len)
    };
    let output_start = crate::output_pointer!(strm.next_out).sub(written);
    let output = ::core::slice::from_raw_parts_mut(output_start, start);
    let result = inflate_fast(input, output, written, &mut state, false);
    strm.next_in = strm.next_in.advance(result.input_used);
    strm.avail_in = (input_len - result.input_used) as crate::stdlib::uInt;
    strm.next_out = strm.next_out.advance(result.output_used);
    strm.avail_out = (output_free - result.output_used) as crate::stdlib::uInt;
    if let Some(message) = result.message {
        crate::zlib_h::set_stream_message(strm, message);
    }
}
