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
#[derive(Copy, Clone)]
struct InflateFastProgress {
    input_used: usize,
    output_used: usize,
    hold: u64,
    bits: u32,
    mode: Option<crate::src::inflate::inflate_mode>,
    error: Option<usize>,
}

/// All bounded state the fast decoder needs for one invocation.  A future C3
/// stream boundary can construct this view after validating the ABI cursors;
/// the decoder itself only sees ordinary slices and scalar cursors.
struct InflateFastViews<'a> {
    input: &'a [u8],
    output: &'a mut [u8],
    window: &'a [u8],
    lcode: &'a [crate::src::inftrees::code],
    dcode: &'a [crate::src::inftrees::code],
    wsize: usize,
    whave: usize,
    wnext: usize,
    hold: u64,
    bits: u32,
    lenbits: u32,
    distbits: u32,
    start: u32,
}

impl InflateFastViews<'_> {
    /// The fast loop is optional: callers fall back to the regular decoder
    /// when either of its documented six-input-byte or 258-output-byte
    /// reserves is unavailable.
    fn validate(&self) -> Result<(), InflateFastProgress> {
        if self.input.len() < 6 || self.output.len() < 258 {
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
        if self.wsize > self.window.len()
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

/// The legacy ABI adapter keeps its table masks at zlib's `uInt` width.  Do
/// the shift through a checked scalar helper so malformed state cannot turn a
/// mask setup into an invalid Rust shift before the bounded core replaces the
/// adapter entirely.
fn inflate_fast_u32_mask(bits: u32) -> Option<u32> {
    1u32.checked_shl(bits).map(|mask| mask.wrapping_sub(1))
}

/// Number of root entries addressed by a DEFLATE decode table.  Keep this
/// checked even though normal inflate state constrains the widths: the safe
/// core must reject a malformed borrowed view before it consumes any input.
fn inflate_fast_table_root(bits: u32) -> Option<usize> {
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

/// Decode the fast-path portion of a deflate stream using only bounded
/// buffers.  The ABI adapter owns construction of these views and commits the
/// resulting cursors, so this core cannot retain or dereference foreign
/// pointers.
fn inflate_fast_core(mut views: InflateFastViews<'_>) -> InflateFastProgress {
    if let Err(progress) = views.validate() {
        return progress;
    }
    let InflateFastViews {
        input,
        output,
        window,
        lcode,
        dcode,
        wsize,
        whave,
        wnext,
        mut hold,
        mut bits,
        lenbits,
        distbits,
        ..
    } = views;
    let mut input_at = 0usize;
    let mut output_at = 0usize;
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
            if here_bits > bits {
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
                    if here_bits > bits {
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
                    if back > whave || back > wsize {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(17);
                        break 'fast;
                    }
                    let Some(window_end) = wnext.checked_add(wsize) else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(17);
                        break 'fast;
                    };
                    let Some(mut from) = window_end.checked_sub(back).map(|index| index % wsize)
                    else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(17);
                        break 'fast;
                    };
                    let take = back.min(len);
                    for _ in 0..take {
                        let Some(&byte) = window.get(from) else {
                            mode = Some(crate::src::inflate::BAD);
                            error = Some(17);
                            break 'fast;
                        };
                        let Some(slot) = output.get_mut(output_at) else {
                            break 'fast;
                        };
                        *slot = byte;
                        from += 1;
                        if from == wsize {
                            from = 0;
                        }
                        output_at += 1;
                    }
                    len -= take;
                }
                while len != 0 {
                    if output_at < dist {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(17);
                        break 'fast;
                    }
                    let Some(&byte) = output.get(output_at - dist) else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(17);
                        break 'fast;
                    };
                    let Some(slot) = output.get_mut(output_at) else {
                        break 'fast;
                    };
                    *slot = byte;
                    output_at += 1;
                    len -= 1;
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
        output_used: output_at,
        hold,
        bits,
        mode,
        error,
    }
}
pub unsafe extern "C" fn inflate_fast(mut strm: z_streamp, mut start: ::core::ffi::c_uint) {
    let mut input_remaining: uInt = 0;
    let mut output_remaining: uInt = 0;
    let mut in_0: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut out: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut wsize: ::core::ffi::c_uint = 0;
    let mut whave: ::core::ffi::c_uint = 0;
    let mut wnext: ::core::ffi::c_uint = 0;
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut lcode: *const code = ::core::ptr::null::<code>();
    let mut dcode: *const code = ::core::ptr::null::<code>();
    let mut lmask: ::core::ffi::c_uint = 0;
    let mut dmask: ::core::ffi::c_uint = 0;
    let mut here: *const code = ::core::ptr::null::<code>();
    let mut op: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    let mut dist: ::core::ffi::c_uint = 0;
    let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    // The decoder's raw cursors remain transitional, but borrow the two
    // validated ABI records once rather than repeatedly dereferencing their
    // raw handles throughout the loop.
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut inflate_state);
    in_0 = strm.next_in as *mut ::core::ffi::c_uchar;
    input_remaining = strm.avail_in;
    out = strm.next_out as *mut ::core::ffi::c_uchar;
    output_remaining = strm.avail_out;
    wsize = state.wsize;
    whave = state.whave;
    wnext = state.wnext;
    window = state.window;
    hold = state.hold;
    bits = state.bits;
    lcode = state.lencode;
    dcode = state.distcode;
    let Some(mask) = inflate_fast_u32_mask(state.lenbits) else {
        strm.msg = b"invalid literal/length code\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
        state.mode = BAD;
        return;
    };
    lmask = mask;
    let Some(mask) = inflate_fast_u32_mask(state.distbits) else {
        strm.msg = b"invalid distance code\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
        state.mode = BAD;
        return;
    };
    dmask = mask;
    's_627: loop {
        if bits < 15 as ::core::ffi::c_uint {
            let c2rust_fresh0 = in_0;
            in_0 = in_0.wrapping_add(1);
            input_remaining = input_remaining.wrapping_sub(1);
            hold = hold.wrapping_add((*c2rust_fresh0 as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            let c2rust_fresh1 = in_0;
            in_0 = in_0.wrapping_add(1);
            input_remaining = input_remaining.wrapping_sub(1);
            hold = hold.wrapping_add((*c2rust_fresh1 as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
        }
        here = lcode.wrapping_add((hold & lmask as ::core::ffi::c_ulong) as usize);
        's_92: loop {
            let here_code = &*here;
            op = here_code.bits as ::core::ffi::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here_code.op as ::core::ffi::c_uint;
            if op == 0 as ::core::ffi::c_uint {
                let c2rust_fresh2 = out;
                out = out.wrapping_add(1);
                output_remaining = output_remaining.wrapping_sub(1);
                *c2rust_fresh2 = here_code.val as ::core::ffi::c_uchar;
                break;
            } else if op & 16 as ::core::ffi::c_uint != 0 {
                len = here_code.val as ::core::ffi::c_uint;
                op &= 15 as ::core::ffi::c_uint;
                if op != 0 {
                    if bits < op {
                        let c2rust_fresh3 = in_0;
                        in_0 = in_0.wrapping_add(1);
                        input_remaining = input_remaining.wrapping_sub(1);
                        hold = hold.wrapping_add((*c2rust_fresh3 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    len = len.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << op)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                }
                if bits < 15 as ::core::ffi::c_uint {
                    let c2rust_fresh4 = in_0;
                    in_0 = in_0.wrapping_add(1);
                    input_remaining = input_remaining.wrapping_sub(1);
                    hold = hold.wrapping_add((*c2rust_fresh4 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    let c2rust_fresh5 = in_0;
                    in_0 = in_0.wrapping_add(1);
                    input_remaining = input_remaining.wrapping_sub(1);
                    hold = hold.wrapping_add((*c2rust_fresh5 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                here = dcode.wrapping_add((hold & dmask as ::core::ffi::c_ulong) as usize);
                loop {
                    let here_code = &*here;
                    op = here_code.bits as ::core::ffi::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here_code.op as ::core::ffi::c_uint;
                    if op & 16 as ::core::ffi::c_uint != 0 {
                        dist = here_code.val as ::core::ffi::c_uint;
                        op &= 15 as ::core::ffi::c_uint;
                        if bits < op {
                            let c2rust_fresh6 = in_0;
                            in_0 = in_0.wrapping_add(1);
                            input_remaining = input_remaining.wrapping_sub(1);
                            hold =
                                hold.wrapping_add((*c2rust_fresh6 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            if bits < op {
                                let c2rust_fresh7 = in_0;
                                in_0 = in_0.wrapping_add(1);
                                input_remaining = input_remaining.wrapping_sub(1);
                                hold = hold
                                    .wrapping_add((*c2rust_fresh7 as ::core::ffi::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                        }
                        dist = dist.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << op)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        op = start.wrapping_sub(output_remaining);
                        if dist > op {
                            op = dist.wrapping_sub(op);
                            if op > whave {
                                if state.sane != 0 {
                                    strm.msg = b"invalid distance too far back\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    state.mode = BAD;
                                    break 's_627;
                                }
                            }
                            from = window;
                            if wnext == 0 as ::core::ffi::c_uint {
                                from = from.wrapping_add(wsize.wrapping_sub(op) as usize);
                                if op < len {
                                    len = len.wrapping_sub(op);
                                    loop {
                                        let c2rust_fresh8 = from;
                                        from = from.wrapping_add(1);
                                        let c2rust_fresh9 = out;
                                        out = out.wrapping_add(1);
                                        output_remaining = output_remaining.wrapping_sub(1);
                                        *c2rust_fresh9 = *c2rust_fresh8;
                                        op = op.wrapping_sub(1);
                                        if op == 0 {
                                            break;
                                        }
                                    }
                                    from = out.wrapping_sub(dist as usize);
                                }
                            } else if wnext < op {
                                from = from.wrapping_add(
                                    wsize.wrapping_add(wnext).wrapping_sub(op) as usize,
                                );
                                op = op.wrapping_sub(wnext);
                                if op < len {
                                    len = len.wrapping_sub(op);
                                    loop {
                                        let c2rust_fresh10 = from;
                                        from = from.wrapping_add(1);
                                        let c2rust_fresh11 = out;
                                        out = out.wrapping_add(1);
                                        output_remaining = output_remaining.wrapping_sub(1);
                                        *c2rust_fresh11 = *c2rust_fresh10;
                                        op = op.wrapping_sub(1);
                                        if op == 0 {
                                            break;
                                        }
                                    }
                                    from = window;
                                    if wnext < len {
                                        op = wnext;
                                        len = len.wrapping_sub(op);
                                        loop {
                                            let c2rust_fresh12 = from;
                                            from = from.wrapping_add(1);
                                            let c2rust_fresh13 = out;
                                            out = out.wrapping_add(1);
                                            output_remaining = output_remaining.wrapping_sub(1);
                                            *c2rust_fresh13 = *c2rust_fresh12;
                                            op = op.wrapping_sub(1);
                                            if op == 0 {
                                                break;
                                            }
                                        }
                                        from = out.wrapping_sub(dist as usize);
                                    }
                                }
                            } else {
                                from = from.wrapping_add(wnext.wrapping_sub(op) as usize);
                                if op < len {
                                    len = len.wrapping_sub(op);
                                    loop {
                                        let c2rust_fresh14 = from;
                                        from = from.wrapping_add(1);
                                        let c2rust_fresh15 = out;
                                        out = out.wrapping_add(1);
                                        output_remaining = output_remaining.wrapping_sub(1);
                                        *c2rust_fresh15 = *c2rust_fresh14;
                                        op = op.wrapping_sub(1);
                                        if op == 0 {
                                            break;
                                        }
                                    }
                                    from = out.wrapping_sub(dist as usize);
                                }
                            }
                            while len > 2 as ::core::ffi::c_uint {
                                let c2rust_fresh16 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh17 = out;
                                out = out.wrapping_add(1);
                                output_remaining = output_remaining.wrapping_sub(1);
                                *c2rust_fresh17 = *c2rust_fresh16;
                                let c2rust_fresh18 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh19 = out;
                                out = out.wrapping_add(1);
                                output_remaining = output_remaining.wrapping_sub(1);
                                *c2rust_fresh19 = *c2rust_fresh18;
                                let c2rust_fresh20 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh21 = out;
                                out = out.wrapping_add(1);
                                output_remaining = output_remaining.wrapping_sub(1);
                                *c2rust_fresh21 = *c2rust_fresh20;
                                len = len.wrapping_sub(3 as ::core::ffi::c_uint);
                            }
                            if len != 0 {
                                let c2rust_fresh22 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh23 = out;
                                out = out.wrapping_add(1);
                                output_remaining = output_remaining.wrapping_sub(1);
                                *c2rust_fresh23 = *c2rust_fresh22;
                                if len > 1 as ::core::ffi::c_uint {
                                    let c2rust_fresh24 = from;
                                    from = from.wrapping_add(1);
                                    let c2rust_fresh25 = out;
                                    out = out.wrapping_add(1);
                                    output_remaining = output_remaining.wrapping_sub(1);
                                    *c2rust_fresh25 = *c2rust_fresh24;
                                }
                            }
                            break 's_92;
                        } else {
                            from = out.wrapping_sub(dist as usize);
                            loop {
                                let c2rust_fresh26 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh27 = out;
                                out = out.wrapping_add(1);
                                output_remaining = output_remaining.wrapping_sub(1);
                                *c2rust_fresh27 = *c2rust_fresh26;
                                let c2rust_fresh28 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh29 = out;
                                out = out.wrapping_add(1);
                                output_remaining = output_remaining.wrapping_sub(1);
                                *c2rust_fresh29 = *c2rust_fresh28;
                                let c2rust_fresh30 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh31 = out;
                                out = out.wrapping_add(1);
                                output_remaining = output_remaining.wrapping_sub(1);
                                *c2rust_fresh31 = *c2rust_fresh30;
                                len = len.wrapping_sub(3 as ::core::ffi::c_uint);
                                if len <= 2 as ::core::ffi::c_uint {
                                    break;
                                }
                            }
                            if len != 0 {
                                let c2rust_fresh32 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh33 = out;
                                out = out.wrapping_add(1);
                                output_remaining = output_remaining.wrapping_sub(1);
                                *c2rust_fresh33 = *c2rust_fresh32;
                                if len > 1 as ::core::ffi::c_uint {
                                    let c2rust_fresh34 = from;
                                    from = from.wrapping_add(1);
                                    let c2rust_fresh35 = out;
                                    out = out.wrapping_add(1);
                                    output_remaining = output_remaining.wrapping_sub(1);
                                    *c2rust_fresh35 = *c2rust_fresh34;
                                }
                            }
                            break 's_92;
                        }
                    } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                        here = dcode
                            .wrapping_add(here_code.val as usize)
                            .wrapping_add(
                                (hold
                                    & ((1 as ::core::ffi::c_uint) << op)
                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                        as ::core::ffi::c_ulong)
                                    as usize,
                            );
                    } else {
                        strm.msg = b"invalid distance code\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.mode = BAD;
                        break 's_627;
                    }
                }
            } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                here = lcode
                    .wrapping_add(here_code.val as usize)
                    .wrapping_add(
                        (hold
                            & ((1 as ::core::ffi::c_uint) << op)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                as ::core::ffi::c_ulong) as usize,
                    );
            } else if op & 32 as ::core::ffi::c_uint != 0 {
                state.mode = TYPE;
                break 's_627;
            } else {
                strm.msg = b"invalid literal/length code\0".as_ptr()
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                state.mode = BAD;
                break 's_627;
            }
        }
        if !(input_remaining > 5 && output_remaining > 257) {
            break;
        }
    }
    len = bits >> 3 as ::core::ffi::c_int;
    in_0 = in_0.wrapping_sub(len as usize);
    input_remaining = input_remaining.wrapping_add(len);
    bits = bits.wrapping_sub(len << 3 as ::core::ffi::c_int);
    if let Some(mask) = inflate_fast_mask(bits) {
        hold &= mask as ::core::ffi::c_ulong;
    } else {
        strm.msg = b"invalid literal/length code\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
        state.mode = BAD;
    }
    strm.next_in = in_0 as *mut Bytef;
    strm.next_out = out as *mut Bytef;
    strm.avail_in = input_remaining;
    strm.avail_out = output_remaining;
    state.hold = hold;
    state.bits = bits;
}
