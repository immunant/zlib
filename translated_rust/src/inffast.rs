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
pub unsafe fn inflate_fast(strm: &mut crate::zlib_h::z_stream, mut start: ::core::ffi::c_uint) {
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
    // The existing engine boundary validates this initialized stream before
    // entering the fast path. Keep the one raw handle conversion here, then
    // use the resulting borrow for all state access below.
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    in_0 = strm.next_in as *mut ::core::ffi::c_uchar;
    last = in_0.wrapping_offset(strm.avail_in.wrapping_sub(5 as crate::stdlib::uInt) as isize);
    out = strm.next_out as *mut ::core::ffi::c_uchar;
    beg = out
        .wrapping_offset(-((start as crate::stdlib::uInt).wrapping_sub(strm.avail_out) as isize));
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
            let c2rust_fresh0 = in_0;
            in_0 = in_0.wrapping_add(1);
            hold = hold.wrapping_add((*c2rust_fresh0 as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            let c2rust_fresh1 = in_0;
            in_0 = in_0.wrapping_add(1);
            hold = hold.wrapping_add((*c2rust_fresh1 as ::core::ffi::c_ulong) << bits);
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
                let c2rust_fresh2 = out;
                out = out.wrapping_add(1);
                *c2rust_fresh2 = here_code.val as ::core::ffi::c_uchar;
                break;
            } else if op & 16 as ::core::ffi::c_uint != 0 {
                len = here_code.val as ::core::ffi::c_uint;
                op &= 15 as ::core::ffi::c_uint;
                if op != 0 {
                    if bits < op {
                        let c2rust_fresh3 = in_0;
                        in_0 = in_0.wrapping_add(1);
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
                    hold = hold.wrapping_add((*c2rust_fresh4 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    let c2rust_fresh5 = in_0;
                    in_0 = in_0.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh5 as ::core::ffi::c_ulong) << bits);
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
                            let c2rust_fresh6 = in_0;
                            in_0 = in_0.wrapping_add(1);
                            hold =
                                hold.wrapping_add((*c2rust_fresh6 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            if bits < op {
                                let c2rust_fresh7 = in_0;
                                in_0 = in_0.wrapping_add(1);
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
                                        *c2rust_fresh9 = *c2rust_fresh8;
                                        op = op.wrapping_sub(1);
                                        if op == 0 {
                                            break;
                                        }
                                    }
                                    // `dist` was checked against the produced output above;
                                    // after copying the window prefix, this rewind stays within
                                    // the same output allocation.
                                    from = out.wrapping_offset(-(dist as isize));
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
                                            *c2rust_fresh13 = *c2rust_fresh12;
                                            op = op.wrapping_sub(1);
                                            if op == 0 {
                                                break;
                                            }
                                        }
                                        // See the matching history-distance check above.
                                        from = out.wrapping_offset(-(dist as isize));
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
                                        *c2rust_fresh15 = *c2rust_fresh14;
                                        op = op.wrapping_sub(1);
                                        if op == 0 {
                                            break;
                                        }
                                    }
                                    // See the matching history-distance check above.
                                    from = out.wrapping_offset(-(dist as isize));
                                }
                            }
                            while len > 2 as ::core::ffi::c_uint {
                                let c2rust_fresh16 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh17 = out;
                                out = out.wrapping_add(1);
                                *c2rust_fresh17 = *c2rust_fresh16;
                                let c2rust_fresh18 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh19 = out;
                                out = out.wrapping_add(1);
                                *c2rust_fresh19 = *c2rust_fresh18;
                                let c2rust_fresh20 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh21 = out;
                                out = out.wrapping_add(1);
                                *c2rust_fresh21 = *c2rust_fresh20;
                                len = len.wrapping_sub(3 as ::core::ffi::c_uint);
                            }
                            if len != 0 {
                                let c2rust_fresh22 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh23 = out;
                                out = out.wrapping_add(1);
                                *c2rust_fresh23 = *c2rust_fresh22;
                                if len > 1 as ::core::ffi::c_uint {
                                    let c2rust_fresh24 = from;
                                    from = from.wrapping_add(1);
                                    let c2rust_fresh25 = out;
                                    out = out.wrapping_add(1);
                                    *c2rust_fresh25 = *c2rust_fresh24;
                                }
                            }
                            break 's_92;
                        } else {
                            // `dist <= out - beg` on this branch, so this is a
                            // same-allocation match-source cursor rewind.
                            from = out.wrapping_offset(-(dist as isize));
                            loop {
                                let c2rust_fresh26 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh27 = out;
                                out = out.wrapping_add(1);
                                *c2rust_fresh27 = *c2rust_fresh26;
                                let c2rust_fresh28 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh29 = out;
                                out = out.wrapping_add(1);
                                *c2rust_fresh29 = *c2rust_fresh28;
                                let c2rust_fresh30 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh31 = out;
                                out = out.wrapping_add(1);
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
                                *c2rust_fresh33 = *c2rust_fresh32;
                                if len > 1 as ::core::ffi::c_uint {
                                    let c2rust_fresh34 = from;
                                    from = from.wrapping_add(1);
                                    let c2rust_fresh35 = out;
                                    out = out.wrapping_add(1);
                                    *c2rust_fresh35 = *c2rust_fresh34;
                                }
                            }
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
        if !(in_0 < last && out < end) {
            break;
        }
    }
    len = bits >> 3 as ::core::ffi::c_int;
    // `len` is the whole-byte portion of the bits just read, so this rewind
    // remains within the input cursor range established by the fast loop.
    in_0 = in_0.wrapping_offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as ::core::ffi::c_int);
    hold &= ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1 as ::core::ffi::c_uint)
        as ::core::ffi::c_ulong;
    strm.next_in = in_0 as *mut crate::stdlib::Bytef;
    strm.next_out = out as *mut crate::stdlib::Bytef;
    strm.avail_in = (if in_0 < last {
        (5usize).wrapping_add(last.addr().wrapping_sub(in_0.addr()))
    } else {
        (5usize).wrapping_sub(in_0.addr().wrapping_sub(last.addr()))
    }) as ::core::ffi::c_uint as crate::stdlib::uInt;
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
    unsafe { inflate_fast(strm, start) }
}
