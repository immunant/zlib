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

pub use crate::stdlib::uLong;
#[derive(Copy, Clone)]
pub(crate) struct InflateFastProgress {
    pub(crate) input_used: usize,
    pub(crate) output_used: usize,
    pub(crate) hold: u64,
    pub(crate) bits: u32,
    pub(crate) mode: Option<crate::src::inflate::inflate_mode>,
    pub(crate) error: Option<usize>,
}

/// The normal inflate API keeps history in a separate circular window, while
/// inflateBack uses its output window as that history.  Keeping the latter as
/// an explicit selector avoids creating aliased immutable and mutable slices.
pub(crate) enum InflateFastHistory<'a> {
    Separate(&'a [u8]),
    Output,
}

/// All bounded state the fast decoder needs for one invocation.  A future C3
/// stream boundary can construct this view after validating the ABI cursors;
/// the decoder itself only sees ordinary slices and scalar cursors.
pub(crate) struct InflateFastViews<'a> {
    pub(crate) input: &'a [u8],
    pub(crate) output: &'a mut [u8],
    pub(crate) output_start: usize,
    pub(crate) history: InflateFastHistory<'a>,
    pub(crate) lcode: &'a [crate::src::inftrees::code],
    pub(crate) dcode: &'a [crate::src::inftrees::code],
    pub(crate) wsize: usize,
    pub(crate) whave: usize,
    pub(crate) wnext: usize,
    pub(crate) sane: bool,
    pub(crate) hold: u64,
    pub(crate) bits: u32,
    pub(crate) lenbits: u32,
    pub(crate) distbits: u32,
    pub(crate) start: u32,
}

impl InflateFastViews<'_> {
    /// The fast loop is optional: callers fall back to the regular decoder
    /// when either of its documented six-input-byte or 258-output-byte
    /// reserves is unavailable.
    fn validate(&self) -> Result<(), InflateFastProgress> {
        if self.input.len() < 6
            || self.output_start > self.output.len()
            || self.output.len().saturating_sub(self.output_start) < 258
        {
            return Err(InflateFastProgress::no_progress(self.hold, self.bits));
        }
        if self.bits >= u64::BITS {
            return Err(InflateFastProgress::invalid(self.hold, self.bits, 14));
        }
        let Some(lroot) = inflate_fast_table_root(self.lenbits) else {
            return Err(InflateFastProgress::invalid(self.hold, self.bits, 14));
        };
        if self.lcode.len() < lroot {
            return Err(InflateFastProgress::invalid(self.hold, self.bits, 14));
        }
        let Some(droot) = inflate_fast_table_root(self.distbits) else {
            return Err(InflateFastProgress::invalid(self.hold, self.bits, 15));
        };
        if self.dcode.len() < droot {
            return Err(InflateFastProgress::invalid(self.hold, self.bits, 15));
        }
        let history_len = match self.history {
            InflateFastHistory::Separate(window) => window.len(),
            InflateFastHistory::Output => self.output.len(),
        };
        if self.wsize > history_len
            || self.whave > self.wsize
            || (self.wsize == 0 && self.wnext != 0)
            || (self.wsize != 0 && self.wnext >= self.wsize)
        {
            return Err(InflateFastProgress::invalid(self.hold, self.bits, 17));
        }
        let Ok(output_capacity) = u32::try_from(self.output.len()) else {
            return Err(InflateFastProgress::invalid(self.hold, self.bits, 17));
        };
        // The legacy fast loop receives the output availability at entry as
        // `start`, so its distance accounting is relative to this output
        // slice. A bounded caller must preserve that same contract.
        if self.start != output_capacity {
            return Err(InflateFastProgress::invalid(self.hold, self.bits, 17));
        }
        Ok(())
    }
}

impl InflateFastProgress {
    /// The fast loop is optional: callers fall back to the regular decoder
    /// when either of its documented six-input-byte or 258-output-byte
    /// reserves is unavailable.
    fn no_progress(hold: u64, bits: u32) -> Self {
        Self {
            input_used: 0,
            output_used: 0,
            hold,
            bits,
            mode: None,
            error: None,
        }
    }

    /// Reject inconsistent bounded-core inputs before any table or history
    /// lookup.  The raw adapter currently establishes these invariants; the
    /// checks keep this core safe when a future stream boundary lends it
    /// ordinary slices instead.
    fn invalid(hold: u64, bits: u32, error: usize) -> Self {
        Self {
            input_used: 0,
            output_used: 0,
            hold,
            bits,
            mode: Some(crate::src::inflate::BAD),
            error: Some(error),
        }
    }
}

/// Return the low-bit mask used by a DEFLATE table entry without relying on
/// a potentially invalid shift count from malformed decoder state.
fn inflate_fast_mask(bits: u32) -> Option<u64> {
    1u64.checked_shl(bits).map(|mask| mask.wrapping_sub(1))
}

/// Number of root entries addressed by a DEFLATE decode table.  Keep this
/// checked even though normal inflate state constrains the widths: the safe
/// core must reject a malformed borrowed view before it consumes any input.
fn inflate_fast_table_root(bits: u32) -> Option<usize> {
    // Deflate codes are at most 15 bits wide.  Besides documenting the
    // format limit, this prevents a future bounded caller from treating an
    // arbitrarily wide (but machine-word-valid) shift as a table root.
    if bits > 15 {
        return None;
    }
    1usize.checked_shl(bits)
}

/// Pull one byte into the bit accumulator without permitting an invalid
/// shift.  A malformed bounded decoder state can otherwise ask a later
/// refill to shift by 64 or more; the legacy raw path relies on its ABI
/// invariants, while this safe core must reject that state explicitly.
fn inflate_fast_pull_byte(
    input: &[u8],
    input_at: &mut usize,
    hold: &mut u64,
    bits: &mut u32,
) -> Result<bool, ()> {
    // Adding a byte at bit 56 or later would leave no valid representation
    // for the accumulator's low-bit mask on the next decode step.
    if *bits > 55 {
        return Err(());
    }
    let Some(&byte) = input.get(*input_at) else {
        return Ok(false);
    };
    *input_at += 1;
    *hold = hold.wrapping_add((byte as u64) << *bits);
    *bits += 8;
    Ok(true)
}

/// Copy a match whose source is entirely in already-produced output.  Copy in
/// distance-sized chunks so each source range precedes its destination: this
/// preserves deflate's repeated-pattern behavior for matches longer than the
/// distance while still using slice-checked overlapping copies.
fn inflate_fast_copy_output_match(
    output: &mut [u8],
    output_at: &mut usize,
    dist: usize,
    len: usize,
) -> bool {
    // A match can be satisfied entirely from the history window before this
    // helper is reached.  In that case there is no output-relative source to
    // validate or copy.  In particular, `output_at` may still be smaller
    // than `dist`, which is valid because no overlapping output copy occurs.
    if len == 0 {
        return true;
    }
    if dist == 0 || *output_at < dist || len > output.len().saturating_sub(*output_at) {
        return false;
    }

    let mut remaining = len;
    while remaining != 0 {
        let copy = remaining.min(dist);
        let source = *output_at - dist;
        let Some(source_end) = source.checked_add(copy) else {
            return false;
        };
        let Some(destination_end) = output_at.checked_add(copy) else {
            return false;
        };
        if source_end > *output_at || destination_end > output.len() {
            return false;
        }
        output.copy_within(source..source_end, *output_at);
        *output_at = destination_end;
        remaining -= copy;
    }
    true
}

/// Copy the prefix of a match that precedes the current output span from the
/// circular history window.  The remaining bytes, if any, are then supplied
/// by `inflate_fast_copy_output_match()` from newly produced output.  Keeping
/// the circular cursor arithmetic here makes both history sources follow the
/// same checked rules without aliasing the output window as an immutable
/// slice in inflateBack mode.
fn inflate_fast_copy_history_prefix(
    history: &InflateFastHistory<'_>,
    output: &mut [u8],
    output_at: &mut usize,
    wsize: usize,
    wnext: usize,
    back: usize,
    len: usize,
) -> Option<usize> {
    if wsize == 0 || wnext >= wsize || back == 0 || back > wsize || *output_at > output.len() {
        return None;
    }
    let history_len = match history {
        InflateFastHistory::Separate(window) => window.len(),
        InflateFastHistory::Output => output.len(),
    };
    if history_len < wsize {
        return None;
    }

    let window_end = wnext.checked_add(wsize)?;
    let from = window_end.checked_sub(back)? % wsize;
    let take = back.min(len);
    let output_end = output_at.checked_add(take)?;
    if output_end > output.len() {
        return None;
    }
    match history {
        // Normal inflate has a separate history allocation, so the prefix
        // can be copied as at most two bounded slices across the circular
        // wrap. Unlike the output-backed inflateBack history below, this
        // source cannot overlap the destination.
        InflateFastHistory::Separate(window) => {
            let first_len = take.min(wsize.checked_sub(from)?);
            let first_end = from.checked_add(first_len)?;
            let destination_mid = output_at.checked_add(first_len)?;
            output
                .get_mut(*output_at..destination_mid)?
                .copy_from_slice(window.get(from..first_end)?);

            let second_len = take.checked_sub(first_len)?;
            if second_len != 0 {
                output
                    .get_mut(destination_mid..output_end)?
                    .copy_from_slice(window.get(..second_len)?);
            }
        }
        // inflateBack uses the output allocation as its history window. A
        // read-then-write step is required here: the source may be produced
        // by an earlier step of the same match.
        InflateFastHistory::Output => {
            let mut from = from;
            for destination in *output_at..output_end {
                let byte = output.get(from).copied()?;
                *output.get_mut(destination)? = byte;
                from = match from.checked_add(1)? {
                    next if next == wsize => 0,
                    next => next,
                };
            }
        }
    }
    *output_at = output_end;
    len.checked_sub(take)
}

/// Decode the fast-path portion of a deflate stream using only bounded
/// buffers.  The ABI adapter owns construction of these views and commits the
/// resulting cursors, so this core cannot retain or dereference foreign
/// pointers.
pub(crate) fn inflate_fast_core(mut views: InflateFastViews<'_>) -> InflateFastProgress {
    if let Err(progress) = views.validate() {
        return progress;
    }
    let InflateFastViews {
        input,
        output,
        output_start,
        history,
        lcode,
        dcode,
        wsize,
        whave,
        wnext,
        sane,
        mut hold,
        mut bits,
        lenbits,
        distbits,
        ..
    } = views;
    let mut input_at = 0usize;
    let mut output_at = output_start;
    // `validate()` established both shifts and the corresponding root table
    // spans. Keeping the masks checked here makes that relationship explicit
    // if this core is later reused independently.
    let Some(lmask) = inflate_fast_table_root(lenbits).map(|root| root.wrapping_sub(1) as u64)
    else {
        return InflateFastProgress::invalid(hold, bits, 14);
    };
    let Some(dmask) = inflate_fast_table_root(distbits).map(|root| root.wrapping_sub(1) as u64)
    else {
        return InflateFastProgress::invalid(hold, bits, 15);
    };
    let mut mode = None;
    let mut error = None;

    'fast: loop {
        if bits < 15 {
            for _ in 0..2 {
                match inflate_fast_pull_byte(input, &mut input_at, &mut hold, &mut bits) {
                    Ok(true) => {}
                    Ok(false) => break 'fast,
                    Err(()) => {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(14);
                        break 'fast;
                    }
                }
            }
        }
        let Some(&mut_here) = lcode.get((hold & lmask) as usize) else {
            mode = Some(crate::src::inflate::BAD);
            error = Some(14);
            break;
        };
        let mut here = mut_here;
        'code: loop {
            let here_bits = here.bits as u32;
            // A zero-width table entry cannot make progress through a
            // subtable chain.  Valid deflate entries always consume at least
            // one bit; reject malformed borrowed tables rather than letting
            // one loop indefinitely in this safe core.
            if here_bits == 0 || here_bits > bits {
                mode = Some(crate::src::inflate::BAD);
                error = Some(14);
                break 'fast;
            }
            hold >>= here_bits;
            bits -= here_bits;
            let mut op = here.op as u32;
            if op == 0 {
                let Some(slot) = output.get_mut(output_at) else {
                    break 'fast;
                };
                *slot = here.val as u8;
                output_at += 1;
                break;
            }
            if op & 16 != 0 {
                let mut len = here.val as usize;
                op &= 15;
                if op != 0 {
                    while bits < op {
                        match inflate_fast_pull_byte(input, &mut input_at, &mut hold, &mut bits) {
                            Ok(true) => {}
                            Ok(false) => break 'fast,
                            Err(()) => {
                                mode = Some(crate::src::inflate::BAD);
                                error = Some(14);
                                break 'fast;
                            }
                        }
                    }
                    let Some(mask) = inflate_fast_mask(op) else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(14);
                        break 'fast;
                    };
                    len = len.wrapping_add((hold & mask) as usize);
                    hold >>= op;
                    bits -= op;
                }
                if bits < 15 {
                    for _ in 0..2 {
                        match inflate_fast_pull_byte(input, &mut input_at, &mut hold, &mut bits) {
                            Ok(true) => {}
                            Ok(false) => break 'fast,
                            Err(()) => {
                                mode = Some(crate::src::inflate::BAD);
                                error = Some(15);
                                break 'fast;
                            }
                        }
                    }
                }
                let Some(&mut_dist_here) = dcode.get((hold & dmask) as usize) else {
                    mode = Some(crate::src::inflate::BAD);
                    error = Some(15);
                    break 'fast;
                };
                let mut dist_here = mut_dist_here;
                let dist = loop {
                    let here_bits = dist_here.bits as u32;
                    if here_bits == 0 || here_bits > bits {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(15);
                        break 'fast;
                    }
                    hold >>= here_bits;
                    bits -= here_bits;
                    let mut dist_op = dist_here.op as u32;
                    if dist_op & 16 != 0 {
                        let mut dist = dist_here.val as usize;
                        dist_op &= 15;
                        while bits < dist_op {
                            match inflate_fast_pull_byte(input, &mut input_at, &mut hold, &mut bits)
                            {
                                Ok(true) => {}
                                Ok(false) => break 'fast,
                                Err(()) => {
                                    mode = Some(crate::src::inflate::BAD);
                                    error = Some(15);
                                    break 'fast;
                                }
                            }
                        }
                        let Some(mask) = inflate_fast_mask(dist_op) else {
                            mode = Some(crate::src::inflate::BAD);
                            error = Some(15);
                            break 'fast;
                        };
                        dist = dist.wrapping_add((hold & mask) as usize);
                        hold >>= dist_op;
                        bits -= dist_op;
                        break dist;
                    }
                    if dist_op & 64 != 0 {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(15);
                        break 'fast;
                    }
                    let Some(mask) = inflate_fast_mask(dist_op) else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(15);
                        break 'fast;
                    };
                    let Some(index) = (dist_here.val as usize).checked_add((hold & mask) as usize)
                    else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(15);
                        break 'fast;
                    };
                    let Some(&next) = dcode.get(index) else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(15);
                        break 'fast;
                    };
                    dist_here = next;
                };

                if dist == 0 {
                    mode = Some(crate::src::inflate::BAD);
                    error = Some(17);
                    break 'fast;
                }
                if dist > output_at {
                    // The first `back` bytes precede this output span, so
                    // they come from the circular history window.  Once
                    // copied, ordinary overlapping output copying supplies
                    // any remaining match bytes.  Advancing modulo `wsize`
                    // preserves both of the legacy window-wrap branches.
                    let back = dist - output_at;
                    // `inflateUndermine()` retains zlib's permissive
                    // history behavior: only the normal sane mode rejects
                    // a distance beyond the recorded history.  A future
                    // safe boundary must still lend a fully initialized
                    // window slice for the permissive mode.
                    if back > wsize || (sane && back > whave) {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(17);
                        break 'fast;
                    }
                    let Some(remaining) = inflate_fast_copy_history_prefix(
                        &history,
                        output,
                        &mut output_at,
                        wsize,
                        wnext,
                        back,
                        len,
                    ) else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(17);
                        break 'fast;
                    };
                    len = remaining;
                }
                if !inflate_fast_copy_output_match(output, &mut output_at, dist, len) {
                    mode = Some(crate::src::inflate::BAD);
                    error = Some(17);
                    break 'fast;
                }
                break 'code;
            }
            if op & 64 == 0 {
                let Some(mask) = inflate_fast_mask(op) else {
                    mode = Some(crate::src::inflate::BAD);
                    error = Some(14);
                    break 'fast;
                };
                let Some(index) = (here.val as usize).checked_add((hold & mask) as usize) else {
                    mode = Some(crate::src::inflate::BAD);
                    error = Some(14);
                    break 'fast;
                };
                let Some(&next) = lcode.get(index) else {
                    mode = Some(crate::src::inflate::BAD);
                    error = Some(14);
                    break 'fast;
                };
                here = next;
            } else if op & 32 != 0 {
                mode = Some(crate::src::inflate::TYPE);
                break 'fast;
            } else {
                mode = Some(crate::src::inflate::BAD);
                error = Some(14);
                break 'fast;
            }
        }
        if input.len().saturating_sub(input_at) <= 5
            || output.len().saturating_sub(output_at) <= 257
        {
            break;
        }
    }
    let rollback = (bits >> 3) as usize;
    if rollback <= input_at {
        input_at -= rollback;
        bits -= (rollback as u32) << 3;
        let Some(mask) = inflate_fast_mask(bits) else {
            return InflateFastProgress::invalid(hold, bits, 14);
        };
        hold &= mask;
    } else {
        mode = Some(crate::src::inflate::BAD);
        error = Some(14);
    }
    InflateFastProgress {
        input_used: input_at,
        output_used: output_at.wrapping_sub(output_start),
        hold,
        bits,
        mode,
        error,
    }
}
