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

fn bit_mask(bit_count: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    ((1 as ::core::ffi::c_uint) << bit_count).wrapping_sub(1 as ::core::ffi::c_uint)
}

fn low_bits(hold: ::core::ffi::c_ulong, bit_count: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    hold as ::core::ffi::c_uint & bit_mask(bit_count)
}

fn consume_bits(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    bit_count: ::core::ffi::c_uint,
) -> (::core::ffi::c_ulong, ::core::ffi::c_uint) {
    (hold >> bit_count, bits.wrapping_sub(bit_count))
}

fn append_input_byte(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    byte: ::core::ffi::c_uchar,
) -> (::core::ffi::c_ulong, ::core::ffi::c_uint) {
    (
        hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits),
        bits.wrapping_add(8 as ::core::ffi::c_uint),
    )
}

fn subtable_offset(entry: code, hold: ::core::ffi::c_ulong) -> isize {
    entry.val as ::core::ffi::c_int as isize
        + (hold & bit_mask(entry.op as ::core::ffi::c_uint) as ::core::ffi::c_ulong) as isize
}

fn unread_bit_state(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> (
    ::core::ffi::c_ulong,
    ::core::ffi::c_uint,
    ::core::ffi::c_uint,
) {
    let unread_bytes = bits >> 3 as ::core::ffi::c_int;
    let unread_bits = bits.wrapping_sub(unread_bytes << 3 as ::core::ffi::c_int);
    (
        hold & bit_mask(unread_bits) as ::core::ffi::c_ulong,
        unread_bits,
        unread_bytes,
    )
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FastLitLenAction {
    Literal,
    Length { extra_bits: ::core::ffi::c_uint },
    Subtable,
    End,
    Invalid,
}

fn fast_litlen_action(op: ::core::ffi::c_uint) -> FastLitLenAction {
    if op == 0 {
        FastLitLenAction::Literal
    } else if op & 16 != 0 {
        FastLitLenAction::Length {
            extra_bits: op & 15,
        }
    } else if op & 64 == 0 {
        FastLitLenAction::Subtable
    } else if op & 32 != 0 {
        FastLitLenAction::End
    } else {
        FastLitLenAction::Invalid
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FastDistAction {
    Distance { extra_bits: ::core::ffi::c_uint },
    Subtable,
    Invalid,
}

fn fast_dist_action(op: ::core::ffi::c_uint) -> FastDistAction {
    if op & 16 != 0 {
        FastDistAction::Distance {
            extra_bits: op & 15,
        }
    } else if op & 64 == 0 {
        FastDistAction::Subtable
    } else {
        FastDistAction::Invalid
    }
}

pub unsafe extern "C" fn inflate_fast(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut in_0: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut last: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
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
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    in_0 = (*strm).next_in as *mut ::core::ffi::c_uchar;
    last = in_0.offset((*strm).avail_in.wrapping_sub(5 as crate::stdlib::uInt) as isize);
    out = (*strm).next_out as *mut ::core::ffi::c_uchar;
    beg = out.offset(-((start as crate::stdlib::uInt).wrapping_sub((*strm).avail_out) as isize));
    end = out.offset((*strm).avail_out.wrapping_sub(257 as crate::stdlib::uInt) as isize);
    wsize = (*state).wsize;
    whave = (*state).whave;
    wnext = (*state).wnext;
    window = (*state).window;
    hold = (*state).hold;
    bits = (*state).bits;
    lcode = (*state).lencode;
    dcode = (*state).distcode;
    lmask = bit_mask((*state).lenbits);
    dmask = bit_mask((*state).distbits);
    let mut c2rust_current_block_141: u64;
    's_94: loop {
        if bits < 15 as ::core::ffi::c_uint {
            let c2rust_fresh0 = in_0;
            in_0 = in_0.offset(1);
            (hold, bits) = append_input_byte(hold, bits, *c2rust_fresh0);
            let c2rust_fresh1 = in_0;
            in_0 = in_0.offset(1);
            (hold, bits) = append_input_byte(hold, bits, *c2rust_fresh1);
        }
        here = lcode.offset((hold & lmask as ::core::ffi::c_ulong) as isize);
        loop {
            op = (*here).bits as ::core::ffi::c_uint;
            (hold, bits) = consume_bits(hold, bits, op);
            op = (*here).op as ::core::ffi::c_uint;
            match fast_litlen_action(op) {
                FastLitLenAction::Literal => {
                    let c2rust_fresh2 = out;
                    out = out.offset(1);
                    *c2rust_fresh2 = (*here).val as ::core::ffi::c_uchar;
                    c2rust_current_block_141 = 5689001924483802034;
                    break;
                }
                FastLitLenAction::Length { extra_bits } => {
                    len = (*here).val as ::core::ffi::c_uint;
                    if extra_bits != 0 {
                        if bits < extra_bits {
                            let c2rust_fresh3 = in_0;
                            in_0 = in_0.offset(1);
                            (hold, bits) = append_input_byte(hold, bits, *c2rust_fresh3);
                        }
                        len = len.wrapping_add(low_bits(hold, extra_bits));
                        (hold, bits) = consume_bits(hold, bits, extra_bits);
                    }
                    if bits < 15 as ::core::ffi::c_uint {
                        let c2rust_fresh4 = in_0;
                        in_0 = in_0.offset(1);
                        (hold, bits) = append_input_byte(hold, bits, *c2rust_fresh4);
                        let c2rust_fresh5 = in_0;
                        in_0 = in_0.offset(1);
                        (hold, bits) = append_input_byte(hold, bits, *c2rust_fresh5);
                    }
                    here = dcode.offset((hold & dmask as ::core::ffi::c_ulong) as isize);
                    c2rust_current_block_141 = 3217834059723038609;
                    break;
                }
                FastLitLenAction::Subtable => {
                    here = lcode.offset(subtable_offset(*here, hold));
                }
                FastLitLenAction::End => {
                    c2rust_current_block_141 = 13505557363059842426;
                    break;
                }
                FastLitLenAction::Invalid => {
                    c2rust_current_block_141 = 9180031981464905198;
                    break;
                }
            }
        }
        match c2rust_current_block_141 {
            3217834059723038609 => {
                loop {
                    op = (*here).bits as ::core::ffi::c_uint;
                    (hold, bits) = consume_bits(hold, bits, op);
                    op = (*here).op as ::core::ffi::c_uint;
                    match fast_dist_action(op) {
                        FastDistAction::Distance { extra_bits } => {
                            dist = (*here).val as ::core::ffi::c_uint;
                            if bits < extra_bits {
                                let c2rust_fresh6 = in_0;
                                in_0 = in_0.offset(1);
                                (hold, bits) = append_input_byte(hold, bits, *c2rust_fresh6);
                                if bits < extra_bits {
                                    let c2rust_fresh7 = in_0;
                                    in_0 = in_0.offset(1);
                                    (hold, bits) = append_input_byte(hold, bits, *c2rust_fresh7);
                                }
                            }
                            dist = dist.wrapping_add(low_bits(hold, extra_bits));
                            (hold, bits) = consume_bits(hold, bits, extra_bits);
                            op = out.offset_from(beg) as ::core::ffi::c_long
                                as ::core::ffi::c_uint;
                            if dist > op {
                                c2rust_current_block_141 = 5235537862154438448;
                                break;
                            } else {
                                c2rust_current_block_141 = 6072622540298447352;
                                break;
                            }
                        }
                        FastDistAction::Subtable => {
                            here = dcode.offset(subtable_offset(*here, hold));
                        }
                        FastDistAction::Invalid => {
                            (*strm).msg = b"invalid distance code\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                            break 's_94;
                        }
                    }
                }
                match c2rust_current_block_141 {
                    6072622540298447352 => {
                        from = out.offset(-(dist as isize));
                        loop {
                            let c2rust_fresh26 = from;
                            from = from.offset(1);
                            let c2rust_fresh27 = out;
                            out = out.offset(1);
                            *c2rust_fresh27 = *c2rust_fresh26;
                            let c2rust_fresh28 = from;
                            from = from.offset(1);
                            let c2rust_fresh29 = out;
                            out = out.offset(1);
                            *c2rust_fresh29 = *c2rust_fresh28;
                            let c2rust_fresh30 = from;
                            from = from.offset(1);
                            let c2rust_fresh31 = out;
                            out = out.offset(1);
                            *c2rust_fresh31 = *c2rust_fresh30;
                            len = len.wrapping_sub(3 as ::core::ffi::c_uint);
                            if !(len > 2 as ::core::ffi::c_uint) {
                                break;
                            }
                        }
                        if len != 0 {
                            let c2rust_fresh32 = from;
                            from = from.offset(1);
                            let c2rust_fresh33 = out;
                            out = out.offset(1);
                            *c2rust_fresh33 = *c2rust_fresh32;
                            if len > 1 as ::core::ffi::c_uint {
                                let c2rust_fresh34 = from;
                                from = from.offset(1);
                                let c2rust_fresh35 = out;
                                out = out.offset(1);
                                *c2rust_fresh35 = *c2rust_fresh34;
                            }
                        }
                    }
                    _ => {
                        op = dist.wrapping_sub(op);
                        if op > whave {
                            if (*state).sane != 0 {
                                (*strm).msg = b"invalid distance too far back\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                break;
                            }
                        }
                        from = window;
                        if wnext == 0 as ::core::ffi::c_uint {
                            from = from.offset(wsize.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    let c2rust_fresh8 = from;
                                    from = from.offset(1);
                                    let c2rust_fresh9 = out;
                                    out = out.offset(1);
                                    *c2rust_fresh9 = *c2rust_fresh8;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize));
                            }
                        } else if wnext < op {
                            from = from.offset(wsize.wrapping_add(wnext).wrapping_sub(op) as isize);
                            op = op.wrapping_sub(wnext);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    let c2rust_fresh10 = from;
                                    from = from.offset(1);
                                    let c2rust_fresh11 = out;
                                    out = out.offset(1);
                                    *c2rust_fresh11 = *c2rust_fresh10;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = window;
                                if wnext < len {
                                    op = wnext;
                                    len = len.wrapping_sub(op);
                                    loop {
                                        let c2rust_fresh12 = from;
                                        from = from.offset(1);
                                        let c2rust_fresh13 = out;
                                        out = out.offset(1);
                                        *c2rust_fresh13 = *c2rust_fresh12;
                                        op = op.wrapping_sub(1);
                                        if !(op != 0) {
                                            break;
                                        }
                                    }
                                    from = out.offset(-(dist as isize));
                                }
                            }
                        } else {
                            from = from.offset(wnext.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    let c2rust_fresh14 = from;
                                    from = from.offset(1);
                                    let c2rust_fresh15 = out;
                                    out = out.offset(1);
                                    *c2rust_fresh15 = *c2rust_fresh14;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize));
                            }
                        }
                        while len > 2 as ::core::ffi::c_uint {
                            let c2rust_fresh16 = from;
                            from = from.offset(1);
                            let c2rust_fresh17 = out;
                            out = out.offset(1);
                            *c2rust_fresh17 = *c2rust_fresh16;
                            let c2rust_fresh18 = from;
                            from = from.offset(1);
                            let c2rust_fresh19 = out;
                            out = out.offset(1);
                            *c2rust_fresh19 = *c2rust_fresh18;
                            let c2rust_fresh20 = from;
                            from = from.offset(1);
                            let c2rust_fresh21 = out;
                            out = out.offset(1);
                            *c2rust_fresh21 = *c2rust_fresh20;
                            len = len.wrapping_sub(3 as ::core::ffi::c_uint);
                        }
                        if len != 0 {
                            let c2rust_fresh22 = from;
                            from = from.offset(1);
                            let c2rust_fresh23 = out;
                            out = out.offset(1);
                            *c2rust_fresh23 = *c2rust_fresh22;
                            if len > 1 as ::core::ffi::c_uint {
                                let c2rust_fresh24 = from;
                                from = from.offset(1);
                                let c2rust_fresh25 = out;
                                out = out.offset(1);
                                *c2rust_fresh25 = *c2rust_fresh24;
                            }
                        }
                    }
                }
            }
            9180031981464905198 => {
                (*strm).msg = b"invalid literal/length code\0".as_ptr()
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                (*state).mode = crate::src::inflate::BAD;
                break;
            }
            13505557363059842426 => {
                (*state).mode = crate::src::inflate::TYPE;
                break;
            }
            _ => {}
        }
        if !(in_0 < last && out < end) {
            break;
        }
    }
    (hold, bits, len) = unread_bit_state(hold, bits);
    in_0 = in_0.offset(-(len as isize));
    (*strm).next_in = in_0 as *mut crate::stdlib::Bytef;
    (*strm).next_out = out as *mut crate::stdlib::Bytef;
    (*strm).avail_in = (if in_0 < last {
        5 as ::core::ffi::c_long + last.offset_from(in_0) as ::core::ffi::c_long
    } else {
        5 as ::core::ffi::c_long - in_0.offset_from(last) as ::core::ffi::c_long
    }) as ::core::ffi::c_uint as crate::stdlib::uInt;
    (*strm).avail_out = (if out < end {
        257 as ::core::ffi::c_long + end.offset_from(out) as ::core::ffi::c_long
    } else {
        257 as ::core::ffi::c_long - out.offset_from(end) as ::core::ffi::c_long
    }) as ::core::ffi::c_uint as crate::stdlib::uInt;
    (*state).hold = hold;
    (*state).bits = bits;
}
#[export_name = "inflate_fast"]

pub unsafe extern "C" fn inflate_fast_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    inflate_fast(strm, start)
}

#[cfg(test)]
mod tests {
    use super::{
        append_input_byte, bit_mask, code, consume_bits, fast_dist_action, fast_litlen_action,
        low_bits, subtable_offset, unread_bit_state, FastDistAction, FastLitLenAction,
    };

    #[test]
    fn bit_mask_selects_requested_low_bits() {
        assert_eq!(bit_mask(0), 0);
        assert_eq!(bit_mask(1), 1);
        assert_eq!(bit_mask(5), 0b1_1111);
        assert_eq!(bit_mask(15), 0x7fff);
    }

    #[test]
    fn low_bits_masks_the_low_word_of_the_bit_buffer() {
        assert_eq!(low_bits(0xffff_ffff_0000_001b, 5), 0x1b);
        assert_eq!(low_bits(0xfeed, 0), 0);
        assert_eq!(low_bits(0xfeed, 15), 0x7eed);
    }

    #[test]
    fn consume_bits_discards_low_bits_and_updates_count() {
        assert_eq!(consume_bits(0b1011_0101, 8, 3), (0b1_0110, 5));
    }

    #[test]
    fn consume_bits_preserves_zero_count_and_wrapping_subtraction() {
        assert_eq!(consume_bits(0xfeed, 9, 0), (0xfeed, 9));
        assert_eq!(consume_bits(1, 0, 1), (0, ::core::ffi::c_uint::MAX));
    }

    #[test]
    fn append_input_byte_packs_bytes_low_bit_first() {
        let (hold, bits) = append_input_byte(0, 0, 0xab);
        assert_eq!((hold, bits), (0xab, 8));
        assert_eq!(append_input_byte(hold, bits, 0xcd), (0xcdab, 16));
    }

    #[test]
    fn append_input_byte_preserves_existing_bits() {
        assert_eq!(append_input_byte(0b101, 3, 0b11), (0b1_1101, 11));
    }

    #[test]
    fn subtable_offset_combines_base_and_low_bit_index() {
        let entry = code {
            op: 5,
            bits: 0,
            val: 96,
        };
        assert_eq!(subtable_offset(entry, 0b1_1011), 123);
    }

    #[test]
    fn litlen_opcode_actions_preserve_deflate_dispatch_precedence() {
        assert_eq!(fast_litlen_action(0), FastLitLenAction::Literal);
        assert_eq!(
            fast_litlen_action(31),
            FastLitLenAction::Length { extra_bits: 15 }
        );
        assert_eq!(fast_litlen_action(1), FastLitLenAction::Subtable);
        assert_eq!(fast_litlen_action(32), FastLitLenAction::Subtable);
        assert_eq!(fast_litlen_action(96), FastLitLenAction::End);
        assert_eq!(fast_litlen_action(64), FastLitLenAction::Invalid);
        assert_eq!(
            fast_litlen_action(80),
            FastLitLenAction::Length { extra_bits: 0 }
        );
        assert_eq!(
            fast_litlen_action(112),
            FastLitLenAction::Length { extra_bits: 0 }
        );
    }

    #[test]
    fn distance_opcode_actions_preserve_base_and_invalid_precedence() {
        assert_eq!(
            fast_dist_action(16),
            FastDistAction::Distance { extra_bits: 0 }
        );
        assert_eq!(
            fast_dist_action(31),
            FastDistAction::Distance { extra_bits: 15 }
        );
        assert_eq!(fast_dist_action(1), FastDistAction::Subtable);
        assert_eq!(fast_dist_action(32), FastDistAction::Subtable);
        assert_eq!(fast_dist_action(64), FastDistAction::Invalid);
        assert_eq!(
            fast_dist_action(80),
            FastDistAction::Distance { extra_bits: 0 }
        );
    }

    #[test]
    fn unread_bit_state_rewinds_full_bytes_and_retains_remaining_bits() {
        assert_eq!(unread_bit_state(0xdead_beef, 21), (0x0f, 5, 2));
    }

    #[test]
    fn unread_bit_state_clears_aligned_and_preserves_sub_byte_buffers() {
        assert_eq!(unread_bit_state(0xfeed, 16), (0, 0, 2));
        assert_eq!(unread_bit_state(0xff, 7), (0x7f, 7, 0));
    }
}
