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

/// Copy a DEFLATE match whose source is earlier in the current output buffer.
///
/// The byte-at-a-time order is deliberate: a match may overlap its destination,
/// and later bytes must be able to read bytes written by this same match.
fn copy_output_match(
    output: &mut [u8],
    output_index: &mut usize,
    distance: usize,
    length: usize,
) -> bool {
    let Some(source_start) = output_index.checked_sub(distance) else {
        return false;
    };
    let Some(output_end) = output_index.checked_add(length) else {
        return false;
    };
    if output_end > output.len() {
        return false;
    }

    for offset in 0..length {
        let byte = output[source_start + offset];
        output[*output_index + offset] = byte;
    }
    *output_index = output_end;
    true
}

/// Copy a non-overlapping portion of the inflate history window to output.
fn copy_window_history(
    output: &mut [u8],
    output_index: &mut usize,
    window: &[u8],
    window_index: usize,
    length: usize,
) -> bool {
    let Some(window_end) = window_index.checked_add(length) else {
        return false;
    };
    let Some(output_end) = output_index.checked_add(length) else {
        return false;
    };
    let Some(source) = window.get(window_index..window_end) else {
        return false;
    };
    let Some(destination) = output.get_mut(*output_index..output_end) else {
        return false;
    };

    destination.copy_from_slice(source);
    *output_index = output_end;
    true
}

pub unsafe fn inflate_fast(
    strm: &mut crate::zlib_h::z_stream,
    mut start: ::core::ffi::c_uint,
    history_may_alias_output: bool,
) {
    let mut in_index: usize = 0;
    let mut last: usize = 0;
    let mut out: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut beg: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut end: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut wsize: ::core::ffi::c_uint = 0;
    let mut whave: ::core::ffi::c_uint = 0;
    let mut wnext: ::core::ffi::c_uint = 0;
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut lcode: *const crate::src::inftrees::code =
        ::core::ptr::null::<crate::src::inftrees::code>();
    let mut dcode: *const crate::src::inftrees::code =
        ::core::ptr::null::<crate::src::inftrees::code>();
    let mut lmask: ::core::ffi::c_uint = 0;
    let mut dmask: ::core::ffi::c_uint = 0;
    let mut here: *const crate::src::inftrees::code =
        ::core::ptr::null::<crate::src::inftrees::code>();
    let mut op: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    let mut dist: ::core::ffi::c_uint = 0;
    let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    // The existing engine boundary validates this initialized stream before
    // entering the fast path. Keep the one raw handle conversion here, then
    // use the resulting borrow for all state access below.
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    // The fast-loop entry condition leaves at least five input bytes.  Keep
    // that existing boundary here and use an indexed view for bit-buffer
    // reads, rather than repeatedly dereferencing the raw input cursor.
    let input = ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize);
    last = input.len().wrapping_sub(5);
    out = strm.next_out as *mut ::core::ffi::c_uchar;
    beg = out
        .wrapping_offset(-((start as crate::stdlib::uInt).wrapping_sub(strm.avail_out) as isize));
    // This is the same caller output extent represented by `beg` and `start`
    // below.  Keep it local to the fast engine, which already relies on the
    // five-byte/257-byte entry bounds before indexing this range.
    let output = ::core::slice::from_raw_parts_mut(beg, start as usize);
    end = out.wrapping_offset(strm.avail_out.wrapping_sub(257 as crate::stdlib::uInt) as isize);
    wsize = state.wsize;
    whave = state.whave;
    wnext = state.wnext;
    window = state.window;
    hold = state.hold;
    bits = state.bits;
    lcode = state.lencode;
    dcode = state.distcode;
    lmask = ((1 as ::core::ffi::c_uint) << state.lenbits).wrapping_sub(1 as ::core::ffi::c_uint);
    dmask = ((1 as ::core::ffi::c_uint) << state.distbits).wrapping_sub(1 as ::core::ffi::c_uint);
    's_627: loop {
        if bits < 15 as ::core::ffi::c_uint {
            let input_byte = input[in_index];
            in_index = in_index.wrapping_add(1);
            hold = hold.wrapping_add((input_byte as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            let input_byte = input[in_index];
            in_index = in_index.wrapping_add(1);
            hold = hold.wrapping_add((input_byte as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
        }
        // The root-table mask bounds this cursor within the validated
        // literal/length decode table.  Dereferencing the table stays in this
        // legacy unsafe engine, but forming the cursor itself need not be an
        // unsafe operation.
        here = lcode.wrapping_offset((hold & lmask as ::core::ffi::c_ulong) as isize);
        's_92: loop {
            let here_code = crate::src::inftrees::copy_code(&*here);
            op = here_code.bits as ::core::ffi::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here_code.op as ::core::ffi::c_uint;
            if op == 0 as ::core::ffi::c_uint {
                let output_index = out.addr().wrapping_sub(beg.addr());
                let Some(byte) = output.get_mut(output_index) else {
                    state.mode = crate::src::inflate::BAD;
                    break 's_627;
                };
                *byte = here_code.val as ::core::ffi::c_uchar;
                out = output
                    .as_mut_ptr()
                    .wrapping_add(output_index.wrapping_add(1));
                break;
            } else if op & 16 as ::core::ffi::c_uint != 0 {
                len = here_code.val as ::core::ffi::c_uint;
                op &= 15 as ::core::ffi::c_uint;
                if op != 0 {
                    if bits < op {
                        let input_byte = input[in_index];
                        in_index = in_index.wrapping_add(1);
                        hold = hold.wrapping_add((input_byte as ::core::ffi::c_ulong) << bits);
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
                    let input_byte = input[in_index];
                    in_index = in_index.wrapping_add(1);
                    hold = hold.wrapping_add((input_byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    let input_byte = input[in_index];
                    in_index = in_index.wrapping_add(1);
                    hold = hold.wrapping_add((input_byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                here = dcode.wrapping_offset((hold & dmask as ::core::ffi::c_ulong) as isize);
                loop {
                    let here_code = crate::src::inftrees::copy_code(&*here);
                    op = here_code.bits as ::core::ffi::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here_code.op as ::core::ffi::c_uint;
                    if op & 16 as ::core::ffi::c_uint != 0 {
                        dist = here_code.val as ::core::ffi::c_uint;
                        op &= 15 as ::core::ffi::c_uint;
                        if bits < op {
                            let input_byte = input[in_index];
                            in_index = in_index.wrapping_add(1);
                            hold = hold.wrapping_add((input_byte as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            if bits < op {
                                let input_byte = input[in_index];
                                in_index = in_index.wrapping_add(1);
                                hold =
                                    hold.wrapping_add((input_byte as ::core::ffi::c_ulong) << bits);
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
                        op = out.addr().wrapping_sub(beg.addr()) as ::core::ffi::c_uint;
                        if dist > op {
                            op = dist.wrapping_sub(op);
                            if op > whave {
                                if state.sane != 0 {
                                    strm.msg = b"invalid distance too far back\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    state.mode = crate::src::inflate::BAD;
                                    break 's_627;
                                }
                            }
                            if history_may_alias_output {
                                // inflateBack uses its caller window as output. Keep the
                                // legacy raw copy here so a history view never aliases the
                                // mutable output view used by the safe fast-path helpers.
                                let mut copy_from_output = false;
                                from = window;
                                if wnext == 0 as ::core::ffi::c_uint {
                                    from = from.wrapping_add(wsize.wrapping_sub(op) as usize);
                                    if op < len {
                                        len = len.wrapping_sub(op);
                                        loop {
                                            let source = from;
                                            from = from.wrapping_add(1);
                                            let destination = out;
                                            out = out.wrapping_add(1);
                                            *destination = *source;
                                            op = op.wrapping_sub(1);
                                            if op == 0 {
                                                break;
                                            }
                                        }
                                        from = out.wrapping_offset(-(dist as isize));
                                        copy_from_output = true;
                                    }
                                } else if wnext < op {
                                    from = from.wrapping_add(
                                        wsize.wrapping_add(wnext).wrapping_sub(op) as usize,
                                    );
                                    op = op.wrapping_sub(wnext);
                                    if op < len {
                                        len = len.wrapping_sub(op);
                                        loop {
                                            let source = from;
                                            from = from.wrapping_add(1);
                                            let destination = out;
                                            out = out.wrapping_add(1);
                                            *destination = *source;
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
                                                let source = from;
                                                from = from.wrapping_add(1);
                                                let destination = out;
                                                out = out.wrapping_add(1);
                                                *destination = *source;
                                                op = op.wrapping_sub(1);
                                                if op == 0 {
                                                    break;
                                                }
                                            }
                                            from = out.wrapping_offset(-(dist as isize));
                                            copy_from_output = true;
                                        }
                                    }
                                } else {
                                    from = from.wrapping_add(wnext.wrapping_sub(op) as usize);
                                    if op < len {
                                        len = len.wrapping_sub(op);
                                        loop {
                                            let source = from;
                                            from = from.wrapping_add(1);
                                            let destination = out;
                                            out = out.wrapping_add(1);
                                            *destination = *source;
                                            op = op.wrapping_sub(1);
                                            if op == 0 {
                                                break;
                                            }
                                        }
                                        from = out.wrapping_offset(-(dist as isize));
                                        copy_from_output = true;
                                    }
                                }
                                if copy_from_output {
                                    let mut output_index = out.addr().wrapping_sub(beg.addr());
                                    if !copy_output_match(
                                        output,
                                        &mut output_index,
                                        dist as usize,
                                        len as usize,
                                    ) {
                                        state.mode = crate::src::inflate::BAD;
                                        break 's_627;
                                    }
                                    out = output.as_mut_ptr().wrapping_add(output_index);
                                } else {
                                    while len > 2 as ::core::ffi::c_uint {
                                        let source = from;
                                        from = from.wrapping_add(1);
                                        let destination = out;
                                        out = out.wrapping_add(1);
                                        *destination = *source;
                                        let source = from;
                                        from = from.wrapping_add(1);
                                        let destination = out;
                                        out = out.wrapping_add(1);
                                        *destination = *source;
                                        let source = from;
                                        from = from.wrapping_add(1);
                                        let destination = out;
                                        out = out.wrapping_add(1);
                                        *destination = *source;
                                        len = len.wrapping_sub(3 as ::core::ffi::c_uint);
                                    }
                                    if len != 0 {
                                        let source = from;
                                        from = from.wrapping_add(1);
                                        let destination = out;
                                        out = out.wrapping_add(1);
                                        *destination = *source;
                                        if len > 1 as ::core::ffi::c_uint {
                                            let source = from;
                                            let destination = out;
                                            out = out.wrapping_add(1);
                                            *destination = *source;
                                        }
                                    }
                                }
                            } else {
                            let history = if window.is_null() || wsize == 0 {
                                &[]
                            } else {
                                ::core::slice::from_raw_parts(window, wsize as usize)
                            };
                            let mut output_index = out.addr().wrapping_sub(beg.addr());
                            let mut remaining = len as usize;
                            if wnext == 0 as ::core::ffi::c_uint {
                                let count = remaining.min(op as usize);
                                if !copy_window_history(
                                    output,
                                    &mut output_index,
                                    history,
                                    wsize.wrapping_sub(op) as usize,
                                    count,
                                ) {
                                    state.mode = crate::src::inflate::BAD;
                                    break 's_627;
                                }
                                remaining = remaining.wrapping_sub(count);
                            } else if wnext < op {
                                let first = op.wrapping_sub(wnext) as usize;
                                let count = remaining.min(first);
                                if !copy_window_history(
                                    output,
                                    &mut output_index,
                                    history,
                                    wsize.wrapping_add(wnext).wrapping_sub(op) as usize,
                                    count,
                                ) {
                                    state.mode = crate::src::inflate::BAD;
                                    break 's_627;
                                }
                                remaining = remaining.wrapping_sub(count);
                                let count = remaining.min(wnext as usize);
                                if !copy_window_history(
                                    output,
                                    &mut output_index,
                                    history,
                                    0,
                                    count,
                                ) {
                                    state.mode = crate::src::inflate::BAD;
                                    break 's_627;
                                }
                                remaining = remaining.wrapping_sub(count);
                            } else {
                                let count = remaining.min(op as usize);
                                if !copy_window_history(
                                    output,
                                    &mut output_index,
                                    history,
                                    wnext.wrapping_sub(op) as usize,
                                    count,
                                ) {
                                    state.mode = crate::src::inflate::BAD;
                                    break 's_627;
                                }
                                remaining = remaining.wrapping_sub(count);
                            }
                            if remaining != 0
                                && !copy_output_match(
                                    output,
                                    &mut output_index,
                                    dist as usize,
                                    remaining,
                                )
                            {
                                state.mode = crate::src::inflate::BAD;
                                break 's_627;
                            }
                            out = output.as_mut_ptr().wrapping_add(output_index);
                            }
                            break 's_92;
                        } else {
                            // `dist <= out - beg` on this branch, so this is a
                            // same-allocation match source. Use a bounded
                            // sequential copy so overlapping matches keep
                            // their DEFLATE repeat semantics.
                            let mut output_index = out.addr().wrapping_sub(beg.addr());
                            if !copy_output_match(
                                output,
                                &mut output_index,
                                dist as usize,
                                len as usize,
                            ) {
                                state.mode = crate::src::inflate::BAD;
                                break 's_627;
                            }
                            out = output.as_mut_ptr().wrapping_add(output_index);
                            break 's_92;
                        }
                    } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                        here = dcode
                            .wrapping_offset(here_code.val as ::core::ffi::c_int as isize)
                            .wrapping_offset(
                                (hold
                                    & ((1 as ::core::ffi::c_uint) << op)
                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                        as ::core::ffi::c_ulong)
                                    as isize,
                            );
                    } else {
                        strm.msg = b"invalid distance code\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.mode = crate::src::inflate::BAD;
                        break 's_627;
                    }
                }
            } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                here = lcode
                    .wrapping_offset(here_code.val as ::core::ffi::c_int as isize)
                    .wrapping_offset(
                        (hold
                            & ((1 as ::core::ffi::c_uint) << op)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                as ::core::ffi::c_ulong) as isize,
                    );
            } else if op & 32 as ::core::ffi::c_uint != 0 {
                state.mode = crate::src::inflate::TYPE;
                break 's_627;
            } else {
                strm.msg = b"invalid literal/length code\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                state.mode = crate::src::inflate::BAD;
                break 's_627;
            }
        }
        if !(in_index < last && out < end) {
            break;
        }
    }
    len = bits >> 3 as ::core::ffi::c_int;
    // `len` is the whole-byte portion of the bits just read, so this rewind
    // remains within the input cursor range established by the fast loop.
    in_index = in_index.wrapping_sub(len as usize);
    bits = bits.wrapping_sub(len << 3 as ::core::ffi::c_int);
    hold &= ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1 as ::core::ffi::c_uint)
        as ::core::ffi::c_ulong;
    strm.next_in = input.as_ptr().wrapping_add(in_index) as *mut crate::stdlib::Bytef;
    strm.next_out = out as *mut crate::stdlib::Bytef;
    strm.avail_in = input.len().wrapping_sub(in_index) as crate::stdlib::uInt;
    strm.avail_out = (if out < end {
        (257usize).wrapping_add(end.addr().wrapping_sub(out.addr()))
    } else {
        (257usize).wrapping_sub(out.addr().wrapping_sub(end.addr()))
    }) as ::core::ffi::c_uint as crate::stdlib::uInt;
    state.hold = hold;
    state.bits = bits;
}
#[export_name = "inflate_fast"]

pub unsafe extern "C" fn inflate_fast_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    let Some(strm) = strm.as_mut() else {
        return;
    };
    unsafe { inflate_fast(strm, start, false) }
}
