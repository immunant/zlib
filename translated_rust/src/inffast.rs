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

/// Copy a match whose history and destination are different positions in the
/// same inflateBack window.
///
/// This deliberately uses indexed, byte-at-a-time reads and writes.  A match
/// may wrap around the window or overlap its destination, and the C loop's
/// forward order is part of DEFLATE's repeat semantics.  Keeping both roles
/// in one slice avoids aliasing a mutable output slice with a history slice.
fn copy_aliasing_window_history(
    output: &mut [u8],
    output_index: &mut usize,
    wsize: usize,
    wnext: usize,
    distance: usize,
    length: usize,
) -> bool {
    if wsize == 0 || output.len() != wsize || wnext > wsize || *output_index > output.len() {
        return false;
    }
    let Some(history_distance) = distance.checked_sub(*output_index) else {
        return false;
    };
    if history_distance > wsize {
        return false;
    }
    let Some(output_end) = output_index.checked_add(length) else {
        return false;
    };
    if output_end > output.len() {
        return false;
    }

    let history_length = length.min(history_distance);
    let history_start = if wnext == 0 {
        wsize - history_distance
    } else if wnext < history_distance {
        wsize - (history_distance - wnext)
    } else {
        wnext - history_distance
    };
    for offset in 0..history_length {
        let source_index = (history_start + offset) % wsize;
        let byte = output[source_index];
        output[*output_index + offset] = byte;
    }
    *output_index += history_length;

    let remaining = length - history_length;
    remaining == 0 || copy_output_match(output, output_index, distance, remaining)
}

pub(crate) enum DecodeTable {
    LiteralLength,
    Distance,
}

/// Return the active decode table as a bounded view.
///
/// Fixed tables have their own immutable storage; dynamic tables are indices
/// into the state's code arena. Keeping that distinction here avoids raw
/// interior table pointers in the decoder state.
pub(crate) fn decode_table(
    state: &crate::src::inflate::inflate_state,
    kind: DecodeTable,
) -> Option<&[crate::src::inftrees::code]> {
    let table = match kind {
        DecodeTable::LiteralLength => state.lencode.clone(),
        DecodeTable::Distance => state.distcode.clone(),
    };
    match table {
        crate::src::inflate::InflateTableRef::FixedLiteralLength => {
            Some(&crate::src::inftrees::lenfix[..])
        }
        crate::src::inflate::InflateTableRef::FixedDistance => {
            Some(&crate::src::inftrees::distfix[..])
        }
        crate::src::inflate::InflateTableRef::Dynamic(index) => state.codes.get(index..),
    }
}

/// Copy one validated decode-table entry.
///
/// Both ordinary inflate and inflateBack retain raw table selectors in their
/// ABI-compatible state.  This is the shared checked access point: fixed
/// tables use their immutable backing storage and dynamic tables are bounded
/// by the state-owned code arena.
pub(crate) fn decode_table_entry(
    state: &crate::src::inflate::inflate_state,
    kind: DecodeTable,
    index: usize,
) -> Option<crate::src::inftrees::code> {
    decode_table(state, kind)
        .and_then(|table| table.get(index))
        .map(crate::src::inftrees::copy_code)
}

/// Return zlib's invalid-code marker when an internal table selector is not
/// usable. The normal decoder then follows its existing data-error path
/// instead of allowing a panic to cross the ABI boundary.
pub(crate) fn decode_table_entry_or_invalid(
    state: &crate::src::inflate::inflate_state,
    kind: DecodeTable,
    index: usize,
) -> crate::src::inftrees::code {
    decode_table_entry(state, kind, index).unwrap_or(crate::src::inftrees::code {
        op: 64,
        bits: 0,
        val: 0,
    })
}

pub fn inflate_fast(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
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
    let mut lmask: ::core::ffi::c_uint = 0;
    let mut dmask: ::core::ffi::c_uint = 0;
    let mut op: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    let mut dist: ::core::ffi::c_uint = 0;
    // Callers establish the stream/state association before entering the fast
    // path. Keeping that typed state borrow in the caller prevents this core
    // from following the ABI state handle itself.
    // The fast-loop entry condition leaves at least five input bytes.  Keep
    // that existing boundary here and use an indexed view for bit-buffer
    // reads, rather than repeatedly dereferencing the raw input cursor.
    let input = unsafe { ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize) };
    last = input.len().wrapping_sub(5);
    out = strm.next_out as *mut ::core::ffi::c_uchar;
    beg = out
        .wrapping_offset(-((start as crate::stdlib::uInt).wrapping_sub(strm.avail_out) as isize));
    // This is the same caller output extent represented by `beg` and `start`
    // below.  Keep it local to the fast engine, which already relies on the
    // five-byte/257-byte entry bounds before indexing this range.
    let output = unsafe { ::core::slice::from_raw_parts_mut(beg, start as usize) };
    end = out.wrapping_offset(strm.avail_out.wrapping_sub(257 as crate::stdlib::uInt) as isize);
    wsize = state.wsize;
    whave = state.whave;
    wnext = state.wnext;
    window = state.window;
    hold = state.hold;
    bits = state.bits;
    let Some(lcode) = decode_table(state, DecodeTable::LiteralLength) else {
        state.mode = crate::src::inflate::BAD;
        return;
    };
    let Some(dcode) = decode_table(state, DecodeTable::Distance) else {
        state.mode = crate::src::inflate::BAD;
        return;
    };
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
        // literal/length decode table. Read it through the bounded table view
        // rather than dereferencing the legacy raw table cursor.
        let Some(mut here_code) = lcode
            .get((hold & lmask as ::core::ffi::c_ulong) as usize)
            .map(crate::src::inftrees::copy_code)
        else {
            state.mode = crate::src::inflate::BAD;
            break 's_627;
        };
        's_92: loop {
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
                let Some(mut here_code) = dcode
                    .get((hold & dmask as ::core::ffi::c_ulong) as usize)
                    .map(crate::src::inftrees::copy_code)
                else {
                    state.mode = crate::src::inflate::BAD;
                    break 's_627;
                };
                loop {
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
                                // inflateBack uses its caller window as output.  Copy from
                                // that one mutable window view with indexes, rather than
                                // creating an aliasing history slice alongside `output`.
                                let mut output_index = out.addr().wrapping_sub(beg.addr());
                                if !copy_aliasing_window_history(
                                    output,
                                    &mut output_index,
                                    wsize as usize,
                                    wnext as usize,
                                    dist as usize,
                                    len as usize,
                                ) {
                                    state.mode = crate::src::inflate::BAD;
                                    break 's_627;
                                }
                                out = output.as_mut_ptr().wrapping_add(output_index);
                            } else {
                                let history = if window.is_null() || wsize == 0 {
                                    &[]
                                } else {
                                    unsafe { ::core::slice::from_raw_parts(window, wsize as usize) }
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
                        let index = (here_code.val as usize).wrapping_add(
                            (hold
                                & ((1 as ::core::ffi::c_uint) << op)
                                    .wrapping_sub(1 as ::core::ffi::c_uint)
                                    as ::core::ffi::c_ulong) as usize,
                        );
                        let Some(next_code) = dcode.get(index).map(crate::src::inftrees::copy_code)
                        else {
                            state.mode = crate::src::inflate::BAD;
                            break 's_627;
                        };
                        here_code = next_code;
                    } else {
                        strm.msg = b"invalid distance code\0".as_ptr() as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.mode = crate::src::inflate::BAD;
                        break 's_627;
                    }
                }
            } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                let index = (here_code.val as usize).wrapping_add(
                    (hold
                        & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1 as ::core::ffi::c_uint)
                            as ::core::ffi::c_ulong) as usize,
                );
                let Some(next_code) = lcode.get(index).map(crate::src::inftrees::copy_code) else {
                    state.mode = crate::src::inflate::BAD;
                    break 's_627;
                };
                here_code = next_code;
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
    let state = unsafe { &mut *(strm.state as *mut crate::src::inflate::inflate_state) };
    inflate_fast(strm, state, start, false)
}
