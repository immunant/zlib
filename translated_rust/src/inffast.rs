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
pub unsafe extern "C" fn inflate_fast(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    let input = ::core::slice::from_raw_parts((*strm).next_in, (*strm).avail_in as usize);
    let used = start.wrapping_sub((*strm).avail_out) as usize;
    let output = ::core::slice::from_raw_parts_mut(
        (*strm).next_out.wrapping_sub(used),
        used + (*strm).avail_out as usize,
    );
    let window = if state.wsize == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(state.window, state.wsize as usize)
    };
    let lcode_len = if ::core::ptr::eq(
        state.lencode,
        crate::src::inftrees::inffixed_h::lenfix.as_ptr(),
    ) {
        crate::src::inftrees::inffixed_h::lenfix.len()
    } else {
        crate::src::inftrees::ENOUGH_LENS as usize
    };
    let dcode_len = if ::core::ptr::eq(
        state.distcode,
        crate::src::inftrees::inffixed_h::distfix.as_ptr(),
    ) {
        crate::src::inftrees::inffixed_h::distfix.len()
    } else {
        crate::src::inftrees::ENOUGH_DISTS as usize
    };
    let lcode = ::core::slice::from_raw_parts(state.lencode, lcode_len);
    let dcode = ::core::slice::from_raw_parts(state.distcode, dcode_len);
    let mut input_index = 0usize;
    let mut output_index = used;
    let mut hold = state.hold;
    let mut bits = state.bits;
    let lmask = ((1 as ::core::ffi::c_uint) << state.lenbits).wrapping_sub(1);
    let dmask = ((1 as ::core::ffi::c_uint) << state.distbits).wrapping_sub(1);
    'outer: loop {
        if bits < 15 {
            hold = hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
            input_index += 1;
            bits = bits.wrapping_add(8);
            hold = hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
            input_index += 1;
            bits = bits.wrapping_add(8);
        }
        let mut here_index = (hold & lmask as ::core::ffi::c_ulong) as usize;
        'literal: loop {
            let mut here = lcode[here_index];
            let mut op = here.bits as ::core::ffi::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here.op as ::core::ffi::c_uint;
            if op == 0 {
                output[output_index] = here.val as ::core::ffi::c_uchar;
                output_index += 1;
                break;
            }
            if op & 16 != 0 {
                let mut len = here.val as ::core::ffi::c_uint;
                op &= 15;
                if op != 0 {
                    if bits < op {
                        hold = hold.wrapping_add(
                            (input[input_index] as ::core::ffi::c_ulong) << bits,
                        );
                        input_index += 1;
                        bits = bits.wrapping_add(8);
                    }
                    len = len.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1),
                    );
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                }
                if bits < 15 {
                    hold = hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
                    input_index += 1;
                    bits = bits.wrapping_add(8);
                    hold = hold.wrapping_add((input[input_index] as ::core::ffi::c_ulong) << bits);
                    input_index += 1;
                    bits = bits.wrapping_add(8);
                }
                here_index = (hold & dmask as ::core::ffi::c_ulong) as usize;
                loop {
                    here = dcode[here_index];
                    op = here.bits as ::core::ffi::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here.op as ::core::ffi::c_uint;
                    if op & 16 != 0 {
                        let mut dist = here.val as ::core::ffi::c_uint;
                        op &= 15;
                        if bits < op {
                            hold = hold.wrapping_add(
                                (input[input_index] as ::core::ffi::c_ulong) << bits,
                            );
                            input_index += 1;
                            bits = bits.wrapping_add(8);
                            if bits < op {
                                hold = hold.wrapping_add(
                                    (input[input_index] as ::core::ffi::c_ulong) << bits,
                                );
                                input_index += 1;
                                bits = bits.wrapping_add(8);
                            }
                        }
                        dist = dist.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1),
                        );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        let produced = output_index as ::core::ffi::c_uint;
                        if dist > produced {
                            let needed = dist.wrapping_sub(produced);
                            if needed > state.whave && state.sane != 0 {
                                (*strm).msg = b"invalid distance too far back\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                state.mode = crate::src::inflate::BAD;
                                break 'outer;
                            }
                            let mut from = (state.wsize.wrapping_add(state.wnext)
                                .wrapping_sub(needed)
                                % state.wsize) as usize;
                            let from_window = len.min(needed) as usize;
                            for _ in 0..from_window {
                                output[output_index] = window[from];
                                output_index += 1;
                                from += 1;
                                if from == window.len() {
                                    from = 0;
                                }
                            }
                            len = len.wrapping_sub(from_window as ::core::ffi::c_uint);
                        }
                        while len != 0 {
                            output[output_index] = output[output_index - dist as usize];
                            output_index += 1;
                            len = len.wrapping_sub(1);
                        }
                        break 'literal;
                    }
                    if op & 64 == 0 {
                        here_index = here.val as usize
                            + (hold
                                & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1)
                                    as ::core::ffi::c_ulong) as usize;
                    } else {
                        (*strm).msg = b"invalid distance code\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.mode = crate::src::inflate::BAD;
                        break 'outer;
                    }
                }
            } else if op & 64 == 0 {
                here_index = here.val as usize
                    + (hold
                        & ((1 as ::core::ffi::c_uint) << op).wrapping_sub(1)
                            as ::core::ffi::c_ulong) as usize;
            } else if op & 32 != 0 {
                state.mode = crate::src::inflate::TYPE;
                break 'outer;
            } else {
                (*strm).msg = b"invalid literal/length code\0".as_ptr()
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                state.mode = crate::src::inflate::BAD;
                break 'outer;
            }
        }
        if input_index >= input.len() - 5 || output_index + 257 > output.len() {
            break;
        }
    }
    let back = (bits >> 3) as usize;
    input_index -= back;
    bits = bits.wrapping_sub((back as ::core::ffi::c_uint) << 3);
    hold &= ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1) as ::core::ffi::c_ulong;
    (*strm).next_in = input.as_ptr().wrapping_add(input_index) as *mut crate::stdlib::Bytef;
    (*strm).next_out = output.as_mut_ptr().wrapping_add(output_index) as *mut crate::stdlib::Bytef;
    (*strm).avail_in = (input.len() - input_index) as crate::stdlib::uInt;
    (*strm).avail_out = (output.len() - output_index) as crate::stdlib::uInt;
    state.hold = hold;
    state.bits = bits;
}
#[export_name = "inflate_fast"]

pub unsafe extern "C" fn inflate_fast_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    inflate_fast(strm, start)
}
