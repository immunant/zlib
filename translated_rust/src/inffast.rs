use crate::src::inflate::CodeTableRef;
use crate::src::inftrees::code;

pub(crate) enum FastExit {
    Continue,
    Type,
    InvalidDistance,
    InvalidCode,
}

pub(crate) struct FastResult {
    pub(crate) input_used: usize,
    pub(crate) output_used: usize,
    pub(crate) hold: u64,
    pub(crate) bits: u32,
    pub(crate) exit: FastExit,
}

// A bounded fast-decoder request must be consumed before an ABI cursor is
// republished.  Keep every scalar needed for that publication in this
// pointer-free completion, rather than returning a loose collection of
// cursor lengths alongside the decoder result.
pub(crate) struct InflateFastCompletion {
    pub(crate) result: FastResult,
    pub(crate) hold: u64,
    pub(crate) bits: u32,
    pub(crate) input_len: usize,
    pub(crate) output_len: usize,
}

// Cursor and scalar publication must stay separate from the ABI stream
// projection.  This is deliberately a pointer-free value: both the direct
// fast entry point and a future owned inflate stream can consume the same
// completion without retaining a caller cursor or an opaque state borrow.
pub(crate) struct InflateFastStreamUpdate {
    pub(crate) input_used: usize,
    pub(crate) input_remaining: usize,
    pub(crate) output_used: usize,
    pub(crate) output_remaining: usize,
    pub(crate) hold: u64,
    pub(crate) bits: u32,
    pub(crate) exit: FastExit,
}

impl InflateFastCompletion {
    pub(crate) fn into_stream_update(self) -> InflateFastStreamUpdate {
        InflateFastStreamUpdate::from_completion(self)
    }
}

impl InflateFastStreamUpdate {
    pub(crate) fn from_completion(completion: InflateFastCompletion) -> Self {
        Self {
            input_used: completion.result.input_used,
            input_remaining: completion
                .input_len
                .wrapping_sub(completion.result.input_used),
            output_used: completion.result.output_used,
            output_remaining: completion
                .output_len
                .wrapping_sub(completion.result.output_used),
            hold: completion.hold,
            bits: completion.bits,
            exit: completion.result.exit,
        }
    }
}

// Normal inflate keeps its history allocation separate from the caller's
// output.  inflateBack, however, deliberately uses its caller window for
// both roles.  Represent that relationship explicitly instead of forming
// overlapping `&[u8]` and `&mut [u8]` views of the same window.
pub(crate) enum FastHistory<'a> {
    External(Option<&'a [u8]>),
    Output,
}

// The fast decoder's resumable state is deliberately pointer-free.  Both the
// normal inflate loop and the exported `inflate_fast` boundary build this
// view, so the decoder itself never needs an ABI stream or raw cursor.
pub(crate) struct InflateFastState<'a> {
    pub(crate) history: FastHistory<'a>,
    pub(crate) wsize: usize,
    pub(crate) whave: usize,
    pub(crate) wnext: usize,
    pub(crate) hold: u64,
    pub(crate) bits: u32,
    pub(crate) lcode: CodeTableRef,
    pub(crate) dcode: CodeTableRef,
    pub(crate) lmask: u32,
    pub(crate) dmask: u32,
    pub(crate) codes: &'a [code],
    pub(crate) sane: bool,
}

// Keep one fast-decoder invocation as a pointer-free transaction.  The ABI
// adapter is still responsible for making the initial bounded views, but it
// must not reopen cursor arithmetic once the decoder has consumed them.  This
// is also the hand-off seam for the normal-inflate owner: it needs only the
// request borrows, the output position, and the resumable decoder view.
pub(crate) struct InflateFastRequest<'input, 'output, 'state> {
    input: &'input [u8],
    output: &'output mut [u8],
    output_pos: usize,
    state: InflateFastState<'state>,
}

impl<'input, 'output, 'state> InflateFastRequest<'input, 'output, 'state> {
    pub(crate) fn new(
        input: &'input [u8],
        output: &'output mut [u8],
        output_pos: usize,
        state: InflateFastState<'state>,
    ) -> Option<Self> {
        output.get(output_pos..)?;
        Some(Self {
            input,
            output,
            output_pos,
            state,
        })
    }

    pub(crate) fn run(mut self) -> InflateFastCompletion {
        let input_len = self.input.len();
        let output_len = self.output.len();
        let result = inflate_fast_core(
            self.input,
            self.output,
            self.output_pos,
            match self.state.history {
                FastHistory::External(window) => FastHistory::External(window),
                FastHistory::Output => FastHistory::Output,
            },
            self.state.wsize,
            self.state.whave,
            self.state.wnext,
            self.state.hold,
            self.state.bits,
            self.state.lcode,
            self.state.dcode,
            self.state.lmask,
            self.state.dmask,
            self.state.codes,
            self.state.sane,
        );
        self.state.hold = result.hold;
        self.state.bits = result.bits;
        InflateFastCompletion {
            result,
            hold: self.state.hold,
            bits: self.state.bits,
            input_len,
            output_len,
        }
    }
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
    history: FastHistory<'_>,
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
                                let byte = match &history {
                                    FastHistory::External(history) => history.unwrap_or(&[])[from],
                                    FastHistory::Output => output[from],
                                };
                                output[output_pos] = byte;
                                output_pos += 1;
                            }
                            len -= first;
                            if len != 0 && wnext != 0 && wnext < missing {
                                let second = wnext.min(len);
                                for from in 0..second {
                                    let byte = match &history {
                                        FastHistory::External(history) => {
                                            history.unwrap_or(&[])[from]
                                        }
                                        FastHistory::Output => output[from],
                                    };
                                    output[output_pos] = byte;
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

pub(crate) fn inflate_fast_from_views(
    input: &[u8],
    output: &mut [u8],
    output_pos: usize,
    state: &mut InflateFastState<'_>,
) -> FastResult {
    let request = InflateFastRequest::new(
        input,
        output,
        output_pos,
        InflateFastState {
            history: match state.history {
                FastHistory::External(window) => FastHistory::External(window),
                FastHistory::Output => FastHistory::Output,
            },
            wsize: state.wsize,
            whave: state.whave,
            wnext: state.wnext,
            hold: state.hold,
            bits: state.bits,
            lcode: state.lcode,
            dcode: state.dcode,
            lmask: state.lmask,
            dmask: state.dmask,
            codes: state.codes,
            sane: state.sane,
        },
    );
    let Some(request) = request else {
        return FastResult {
            input_used: 0,
            output_used: output_pos,
            hold: state.hold,
            bits: state.bits,
            exit: FastExit::Continue,
        };
    };
    let completion = request.run();
    state.hold = completion.hold;
    state.bits = completion.bits;
    completion.result
}

// The only raw stream/state and cursor projection for the direct ABI entry
// point.  The decoder/publication seam below receives an owned bounded
// facade, so the future pointer-free inflate stream owner can replace this
// adapter without reopening the fast decoder or its cursor accounting.
pub(crate) unsafe fn inflate_fast_from_abi_stream(
    stream: &mut crate::zlib_h::z_stream_s,
    start: ::core::ffi::c_uint,
) {
    let Some((strm, state)) = crate::src::inflate::inflate_stream_and_state(stream) else {
        return;
    };
    let input = if strm.avail_in == 0 {
        &[]
    } else {
        if strm.next_in.is_null() {
            return;
        }
        core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
    };
    let written = start.wrapping_sub(strm.avail_out) as usize;
    let output_start = strm.next_out.wrapping_sub(written);
    let output = if start == 0 {
        &mut []
    } else {
        if output_start.is_null() {
            return;
        }
        core::slice::from_raw_parts_mut(output_start, start as usize)
    };
    let fast_state = state.decoder.normal.fast_state();
    // Use the same pointer-free transaction as normal inflate.  This keeps
    // the raw ABI cursor projection here while ensuring both fast callers
    // consume and account for an identical bounded request.
    let owner =
        crate::src::inflate::InflateNormalStreamOwner::new(input, output, written, fast_state);
    let Some(owner) = owner else {
        return;
    };
    let update = owner.run_fast();
    strm.next_in = strm.next_in.wrapping_add(update.input_used);
    strm.avail_in = update.input_remaining as crate::stdlib::uInt;
    strm.next_out = output_start.wrapping_add(update.output_used);
    strm.avail_out = update.output_remaining as crate::stdlib::uInt;
    state.decoder.normal.apply_fast_update(&update);
    match update.exit {
        FastExit::Continue => {}
        FastExit::Type => {}
        FastExit::InvalidDistance => {
            strm.msg = b"invalid distance too far back\0"
                .as_ptr()
                .cast_mut()
                .cast();
        }
        FastExit::InvalidCode => {
            strm.msg = b"invalid literal/length or distance code\0"
                .as_ptr()
                .cast_mut()
                .cast();
        }
    }
}

// The normal inflate owner also dispatches directly through the same
// pointer-free request/completion seam.
pub(crate) fn inflate_fast(request: InflateFastRequest<'_, '_, '_>) -> InflateFastCompletion {
    request.run()
}

#[export_name = "inflate_fast"]
pub unsafe extern "C" fn inflate_fast_ffi(
    strm: crate::zlib_h::z_streamp,
    start: ::core::ffi::c_uint,
) {
    let Some(stream) = strm.as_mut() else {
        return;
    };
    inflate_fast_from_abi_stream(stream, start)
}
