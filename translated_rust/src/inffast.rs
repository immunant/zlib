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

/// Decode the fast-path portion of a deflate stream using only bounded
/// buffers.  The ABI adapter owns construction of these views and commits the
/// resulting cursors, so this core cannot retain or dereference foreign
/// pointers.
fn inflate_fast_core(
    input: &[u8],
    output: &mut [u8],
    window: &[u8],
    lcode: &[crate::src::inftrees::code],
    dcode: &[crate::src::inftrees::code],
    wsize: usize,
    whave: usize,
    wnext: usize,
    mut hold: u64,
    mut bits: u32,
    lenbits: u32,
    distbits: u32,
    sane: bool,
    start: u32,
) -> InflateFastProgress {
    // The translated raw loop relies on these reserves before it reads ahead.
    // A bounded caller with less space should use the normal decoder instead.
    if input.len() < 6 || output.len() < 258 {
        return InflateFastProgress::no_progress(hold, bits);
    }
    if lenbits >= u32::BITS || distbits >= u32::BITS {
        return InflateFastProgress::invalid(hold, bits, 14);
    }
    if wsize > window.len() || whave > wsize || wnext > wsize {
        return InflateFastProgress::invalid(hold, bits, 17);
    }
    let Ok(output_capacity) = u32::try_from(output.len()) else {
        return InflateFastProgress::invalid(hold, bits, 17);
    };
    let Some(output_origin) = start.checked_sub(output_capacity) else {
        return InflateFastProgress::invalid(hold, bits, 17);
    };
    let mut input_at = 0usize;
    let mut output_at = 0usize;
    let lmask = (1u32 << lenbits).wrapping_sub(1) as u64;
    let dmask = (1u32 << distbits).wrapping_sub(1) as u64;
    let mut mode = None;
    let mut error = None;

    'fast: loop {
        if bits < 15 {
            for _ in 0..2 {
                let Some(&byte) = input.get(input_at) else {
                    break 'fast;
                };
                input_at += 1;
                hold = hold.wrapping_add((byte as u64) << bits);
                bits += 8;
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
                        let Some(&byte) = input.get(input_at) else {
                            break 'fast;
                        };
                        input_at += 1;
                        hold = hold.wrapping_add((byte as u64) << bits);
                        bits += 8;
                    }
                    len = len.wrapping_add((hold as u32 & (1u32 << op).wrapping_sub(1)) as usize);
                    hold >>= op;
                    bits -= op;
                }
                if bits < 15 {
                    for _ in 0..2 {
                        let Some(&byte) = input.get(input_at) else {
                            break 'fast;
                        };
                        input_at += 1;
                        hold = hold.wrapping_add((byte as u64) << bits);
                        bits += 8;
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
                            let Some(&byte) = input.get(input_at) else {
                                break 'fast;
                            };
                            input_at += 1;
                            hold = hold.wrapping_add((byte as u64) << bits);
                            bits += 8;
                        }
                        dist = dist.wrapping_add(
                            (hold as u32 & (1u32 << dist_op).wrapping_sub(1)) as usize,
                        );
                        hold >>= dist_op;
                        bits -= dist_op;
                        break dist;
                    }
                    if dist_op & 64 != 0 {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(15);
                        break 'fast;
                    }
                    let index = dist_here.val as usize
                        + (hold as u32 & (1u32 << dist_op).wrapping_sub(1)) as usize;
                    let Some(&next) = dcode.get(index) else {
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(15);
                        break 'fast;
                    };
                    dist_here = next;
                };

                let produced = output_origin.wrapping_add(output_at as u32) as usize;
                if dist > produced {
                    let mut back = dist - produced;
                    if back > whave || back > wsize || wsize > window.len() {
                        if sane {
                            mode = Some(crate::src::inflate::BAD);
                            error = Some(17);
                            break 'fast;
                        }
                        mode = Some(crate::src::inflate::BAD);
                        error = Some(17);
                        break 'fast;
                    }
                    let mut from = if wnext == 0 {
                        wsize - back
                    } else if wnext < back {
                        wsize + wnext - back
                    } else {
                        wnext - back
                    };
                    let first = if wnext != 0 && wnext < back {
                        back - wnext
                    } else {
                        back
                    };
                    let take = first.min(len);
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
                        output_at += 1;
                    }
                    len -= take;
                    back -= take;
                    if len != 0 && wnext != 0 && wnext < dist - produced {
                        from = 0;
                        let take = wnext.min(len);
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
                            output_at += 1;
                        }
                        len -= take;
                    }
                    if len != 0 {
                        if output_at < dist {
                            mode = Some(crate::src::inflate::BAD);
                            error = Some(17);
                            break 'fast;
                        }
                    }
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
                let index =
                    here.val as usize + (hold as u32 & (1u32 << op).wrapping_sub(1)) as usize;
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
        hold &= (1u64 << bits).wrapping_sub(1);
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
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
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
    state = (*strm).state as *mut inflate_state;
    in_0 = (*strm).next_in as *mut ::core::ffi::c_uchar;
    input_remaining = (*strm).avail_in;
    out = (*strm).next_out as *mut ::core::ffi::c_uchar;
    output_remaining = (*strm).avail_out;
    wsize = (*state).wsize;
    whave = (*state).whave;
    wnext = (*state).wnext;
    window = (*state).window;
    hold = (*state).hold;
    bits = (*state).bits;
    lcode = (*state).lencode;
    dcode = (*state).distcode;
    lmask = ((1 as ::core::ffi::c_uint) << (*state).lenbits).wrapping_sub(1 as ::core::ffi::c_uint);
    dmask =
        ((1 as ::core::ffi::c_uint) << (*state).distbits).wrapping_sub(1 as ::core::ffi::c_uint);
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
            op = (*here).bits as ::core::ffi::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = (*here).op as ::core::ffi::c_uint;
            if op == 0 as ::core::ffi::c_uint {
                let c2rust_fresh2 = out;
                out = out.wrapping_add(1);
                output_remaining = output_remaining.wrapping_sub(1);
                *c2rust_fresh2 = (*here).val as ::core::ffi::c_uchar;
                break;
            } else if op & 16 as ::core::ffi::c_uint != 0 {
                len = (*here).val as ::core::ffi::c_uint;
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
                    op = (*here).bits as ::core::ffi::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = (*here).op as ::core::ffi::c_uint;
                    if op & 16 as ::core::ffi::c_uint != 0 {
                        dist = (*here).val as ::core::ffi::c_uint;
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
                                if (*state).sane != 0 {
                                    (*strm).msg = b"invalid distance too far back\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    (*state).mode = BAD;
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
                            .wrapping_add((*here).val as usize)
                            .wrapping_add(
                                (hold
                                    & ((1 as ::core::ffi::c_uint) << op)
                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                        as ::core::ffi::c_ulong)
                                    as usize,
                            );
                    } else {
                        (*strm).msg = b"invalid distance code\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = BAD;
                        break 's_627;
                    }
                }
            } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                here = lcode
                    .wrapping_add((*here).val as usize)
                    .wrapping_add(
                        (hold
                            & ((1 as ::core::ffi::c_uint) << op)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                as ::core::ffi::c_ulong) as usize,
                    );
            } else if op & 32 as ::core::ffi::c_uint != 0 {
                (*state).mode = TYPE;
                break 's_627;
            } else {
                (*strm).msg = b"invalid literal/length code\0".as_ptr()
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                (*state).mode = BAD;
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
    hold &= ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1 as ::core::ffi::c_uint)
        as ::core::ffi::c_ulong;
    (*strm).next_in = in_0 as *mut Bytef;
    (*strm).next_out = out as *mut Bytef;
    (*strm).avail_in = input_remaining;
    (*strm).avail_out = output_remaining;
    (*state).hold = hold;
    (*state).bits = bits;
}
