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
pub unsafe fn inflate_fast(
    mut strm: crate::zlib_h::z_streamp,
    state: &mut crate::src::inflate::inflate_state,
    mut start: ::core::ffi::c_uint,
) {
    let mut in_0: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut last: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut out: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut beg: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut end: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut wsize: ::core::ffi::c_uint = 0;
    let mut whave: ::core::ffi::c_uint = 0;
    let mut wnext: ::core::ffi::c_uint = 0;
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
    in_0 = crate::input_pointer!((*strm).next_in) as *mut ::core::ffi::c_uchar;
    last = in_0.offset((*strm).avail_in.wrapping_sub(5 as crate::stdlib::uInt) as isize);
    out = crate::output_pointer!((*strm).next_out) as *mut ::core::ffi::c_uchar;
    beg = out.offset(-((start as crate::stdlib::uInt).wrapping_sub((*strm).avail_out) as isize));
    end = out.offset((*strm).avail_out.wrapping_sub(257 as crate::stdlib::uInt) as isize);
    wsize = (*state).wsize;
    whave = (*state).whave;
    wnext = (*state).wnext;
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
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*c2rust_fresh0 as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            let c2rust_fresh1 = in_0;
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*c2rust_fresh1 as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
        }
        here = lcode.offset((hold & lmask as ::core::ffi::c_ulong) as isize);
        's_92: loop {
            op = (*here).bits as ::core::ffi::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = (*here).op as ::core::ffi::c_uint;
            if op == 0 as ::core::ffi::c_uint {
                let c2rust_fresh2 = out;
                out = out.offset(1);
                *c2rust_fresh2 = (*here).val as ::core::ffi::c_uchar;
                break;
            } else if op & 16 as ::core::ffi::c_uint != 0 {
                len = (*here).val as ::core::ffi::c_uint;
                op &= 15 as ::core::ffi::c_uint;
                if op != 0 {
                    if bits < op {
                        let c2rust_fresh3 = in_0;
                        in_0 = in_0.offset(1);
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
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh4 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    let c2rust_fresh5 = in_0;
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh5 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                here = dcode.offset((hold & dmask as ::core::ffi::c_ulong) as isize);
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
                            in_0 = in_0.offset(1);
                            hold =
                                hold.wrapping_add((*c2rust_fresh6 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            if bits < op {
                                let c2rust_fresh7 = in_0;
                                in_0 = in_0.offset(1);
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
                        op = out.offset_from(beg) as ::core::ffi::c_uint;
                        if dist > op {
                            op = dist.wrapping_sub(op);
                            if op > whave {
                                if (*state).sane != 0 {
                                    crate::zlib_h::set_stream_message(
                                        &mut *strm,
                                        c"invalid distance too far back",
                                    );
                                    (*state).mode = crate::src::inflate::BAD;
                                    break 's_627;
                                }
                            }
                            let window = (*state)
                                .window
                                .as_ref()
                                .expect("a referenced history has a window");
                            let mut history = op;
                            while len != 0 {
                                let byte = if history != 0 {
                                    let index = (wnext as usize + wsize as usize - history as usize)
                                        % wsize as usize;
                                    history = history.wrapping_sub(1);
                                    window[index]
                                } else {
                                    *out.offset(-(dist as isize))
                                };
                                *out = byte;
                                out = out.offset(1);
                                len = len.wrapping_sub(1);
                            }
                            break 's_92;
                        } else {
                            while len != 0 {
                                *out = *out.offset(-(dist as isize));
                                out = out.offset(1);
                                len = len.wrapping_sub(1);
                            }
                            break 's_92;
                        }
                    } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                        here = dcode
                            .offset((*here).val as ::core::ffi::c_int as isize)
                            .offset(
                                (hold
                                    & ((1 as ::core::ffi::c_uint) << op)
                                        .wrapping_sub(1 as ::core::ffi::c_uint)
                                        as ::core::ffi::c_ulong)
                                    as isize,
                            );
                    } else {
                        crate::zlib_h::set_stream_message(&mut *strm, c"invalid distance code");
                        (*state).mode = crate::src::inflate::BAD;
                        break 's_627;
                    }
                }
            } else if op & 64 as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                here = lcode
                    .offset((*here).val as ::core::ffi::c_int as isize)
                    .offset(
                        (hold
                            & ((1 as ::core::ffi::c_uint) << op)
                                .wrapping_sub(1 as ::core::ffi::c_uint)
                                as ::core::ffi::c_ulong) as isize,
                    );
            } else if op & 32 as ::core::ffi::c_uint != 0 {
                (*state).mode = crate::src::inflate::TYPE;
                break 's_627;
            } else {
                crate::zlib_h::set_stream_message(&mut *strm, c"invalid literal/length code");
                (*state).mode = crate::src::inflate::BAD;
                break 's_627;
            }
        }
        if !(in_0 < last && out < end) {
            break;
        }
    }
    len = bits >> 3 as ::core::ffi::c_int;
    in_0 = in_0.offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as ::core::ffi::c_int);
    hold &= ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1 as ::core::ffi::c_uint)
        as ::core::ffi::c_ulong;
    (*strm).next_in = crate::input_cursor!(in_0);
    (*strm).next_out = crate::output_cursor!(out);
    (*strm).avail_in = (if in_0 < last {
        5 as isize + last.offset_from(in_0)
    } else {
        5 as isize - in_0.offset_from(last)
    }) as ::core::ffi::c_uint as crate::stdlib::uInt;
    (*strm).avail_out = (if out < end {
        257 as isize + end.offset_from(out)
    } else {
        257 as isize - out.offset_from(end)
    }) as ::core::ffi::c_uint as crate::stdlib::uInt;
    (*state).hold = hold;
    (*state).bits = bits;
}
#[export_name = "inflate_fast"]

pub unsafe extern "C" fn inflate_fast_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    if let Some(state_handle) = (&*strm).inflate_state() {
        inflate_fast(strm, &mut state_handle.borrow_mut(), start)
    }
}
pub use crate::src::inflate::inflate_mode;
pub use crate::src::inflate::inflate_state;
