use crate::src::inflate::{inflate_state, CodeTableRef, BAD, TYPE};
use crate::src::inftrees::code;

enum FastExit {
    Continue,
    Type,
    InvalidDistance,
    InvalidCode,
}

struct FastResult {
    input_used: usize,
    output_used: usize,
    hold: u64,
    bits: u32,
    exit: FastExit,
}

#[inline]
fn table_entry(table: CodeTableRef, codes: &[code], index: usize) -> code {
    code::copied_from(table.get(codes, index as isize))
}

#[inline]
fn pull_byte(input: &[u8], input_pos: &mut usize, hold: &mut u64, bits: &mut u32) {
    *hold = hold.wrapping_add((input[*input_pos] as u64) << *bits);
    *input_pos += 1;
    *bits += 8;
}

// This decoder deliberately takes only bounded views and scalar state.  The
// caller creates those views from the ABI stream, and publishes the resulting
// cursor progress after the fast loop returns.
fn inflate_fast_core(
    input: &[u8],
    output: &mut [u8],
    mut output_pos: usize,
    window: Option<&[u8]>,
    wsize: usize,
    whave: usize,
    wnext: usize,
    mut hold: u64,
    mut bits: u32,
    lcode: CodeTableRef,
    dcode: CodeTableRef,
    lmask: u32,
    dmask: u32,
    codes: &[code],
    sane: bool,
) -> FastResult {
    let mut input_pos = 0;
    let last = input.len().saturating_sub(5);
    let end = output.len().saturating_sub(257);
    let mut exit = FastExit::Continue;

    'outer: loop {
        if bits < 15 {
            pull_byte(input, &mut input_pos, &mut hold, &mut bits);
            pull_byte(input, &mut input_pos, &mut hold, &mut bits);
        }
        let mut here = table_entry(lcode, codes, (hold as u32 & lmask) as usize);
        'literal: loop {
            let mut op = here.bits as u32;
            hold >>= op;
            bits -= op;
            op = here.op as u32;
            if op == 0 {
                output[output_pos] = here.val as u8;
                output_pos += 1;
                break;
            }
            if op & 16 != 0 {
                let mut len = here.val as usize;
                op &= 15;
                if op != 0 {
                    if bits < op {
                        pull_byte(input, &mut input_pos, &mut hold, &mut bits);
                    }
                    len += (hold as u32 & ((1u32 << op) - 1)) as usize;
                    hold >>= op;
                    bits -= op;
                }
                if bits < 15 {
                    pull_byte(input, &mut input_pos, &mut hold, &mut bits);
                    pull_byte(input, &mut input_pos, &mut hold, &mut bits);
                }
                here = table_entry(dcode, codes, (hold as u32 & dmask) as usize);
                loop {
                    op = here.bits as u32;
                    hold >>= op;
                    bits -= op;
                    op = here.op as u32;
                    if op & 16 != 0 {
                        let mut dist = here.val as usize;
                        op &= 15;
                        if bits < op {
                            pull_byte(input, &mut input_pos, &mut hold, &mut bits);
                            if bits < op {
                                pull_byte(input, &mut input_pos, &mut hold, &mut bits);
                            }
                        }
                        dist += (hold as u32 & ((1u32 << op) - 1)) as usize;
                        hold >>= op;
                        bits -= op;
                        let produced = output_pos;
                        if dist > produced {
                            let missing = dist - produced;
                            if missing > whave && sane {
                                exit = FastExit::InvalidDistance;
                                break 'outer;
                            }
                            let history = window.unwrap_or(&[]);
                            let (first_from, first_len) = if wnext == 0 {
                                (wsize - missing, missing)
                            } else if wnext < missing {
                                (wsize + wnext - missing, missing - wnext)
                            } else {
                                (wnext - missing, missing)
                            };
                            let first = first_len.min(len);
                            for offset in 0..first {
                                let from = first_from + offset;
                                output[output_pos] = history[from];
                                output_pos += 1;
                            }
                            len -= first;
                            if len != 0 && wnext != 0 && wnext < missing {
                                let second = wnext.min(len);
                                for from in 0..second {
                                    output[output_pos] = history[from];
                                    output_pos += 1;
                                }
                                len -= second;
                            }
                        }
                        if len != 0 {
                            let mut from = output_pos - dist;
                            for _ in 0..len {
                                let byte = output[from];
                                output[output_pos] = byte;
                                from += 1;
                                output_pos += 1;
                            }
                        }
                        break 'literal;
                    }
                    if op & 64 == 0 {
                        here = table_entry(
                            dcode,
                            codes,
                            here.val as usize + (hold as u32 & ((1u32 << op) - 1)) as usize,
                        );
                    } else {
                        exit = FastExit::InvalidCode;
                        break 'outer;
                    }
                }
            } else if op & 64 == 0 {
                here = table_entry(
                    lcode,
                    codes,
                    here.val as usize + (hold as u32 & ((1u32 << op) - 1)) as usize,
                );
            } else if op & 32 != 0 {
                exit = FastExit::Type;
                break 'outer;
            } else {
                exit = FastExit::InvalidCode;
                break 'outer;
            }
        }
        if input_pos >= last || output_pos >= end {
            break;
        }
    }
    let unused = (bits >> 3) as usize;
    input_pos -= unused;
    bits -= (unused as u32) << 3;
    hold &= (1u64 << bits) - 1;
    FastResult {
        input_used: input_pos,
        output_used: output_pos,
        hold,
        bits,
        exit,
    }
}

pub unsafe extern "C" fn inflate_fast(strm: crate::zlib_h::z_streamp, start: ::core::ffi::c_uint) {
    // Project the ABI stream and state once.  All subsequent decoder work is
    // on bounded slices and ordinary Rust references; this adapter only
    // creates and republishes the caller-owned cursor views.
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut inflate_state);
    let input = core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize);
    let written = start.wrapping_sub(strm.avail_out) as usize;
    let output_start = strm.next_out.wrapping_sub(written);
    let output = core::slice::from_raw_parts_mut(output_start, start as usize);
    let window = state
        .window
        .map(|window| core::slice::from_raw_parts(window.as_ptr(), state.wsize as usize));
    let result = inflate_fast_core(
        input,
        output,
        written,
        window,
        state.wsize as usize,
        state.whave as usize,
        state.wnext as usize,
        state.hold,
        state.bits,
        state.lencode,
        state.distcode,
        (1u32 << state.lenbits) - 1,
        (1u32 << state.distbits) - 1,
        &state.codes,
        state.sane != 0,
    );
    strm.next_in = strm.next_in.wrapping_add(result.input_used);
    strm.avail_in = input.len().wrapping_sub(result.input_used) as crate::stdlib::uInt;
    strm.next_out = output_start.wrapping_add(result.output_used);
    strm.avail_out = output.len().wrapping_sub(result.output_used) as crate::stdlib::uInt;
    state.hold = result.hold;
    state.bits = result.bits;
    match result.exit {
        FastExit::Continue => {}
        FastExit::Type => state.mode = TYPE,
        FastExit::InvalidDistance => {
            strm.msg = b"invalid distance too far back\0"
                .as_ptr()
                .cast_mut()
                .cast();
            state.mode = BAD;
        }
        FastExit::InvalidCode => {
            strm.msg = b"invalid literal/length or distance code\0"
                .as_ptr()
                .cast_mut()
                .cast();
            state.mode = BAD;
        }
    }
}

#[export_name = "inflate_fast"]
pub unsafe extern "C" fn inflate_fast_ffi(
    strm: crate::zlib_h::z_streamp,
    start: ::core::ffi::c_uint,
) {
    inflate_fast(strm, start)
}
