pub use crate::__stddef_size_t_h::size_t;

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
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::inflate_fixed;
pub use crate::src::inftrees::inflate_table;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::LENS;

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
pub use crate::zlib_h::in_func;
pub use crate::zlib_h::out_func;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub use crate::zlib_h::Z_VERSION_ERROR;

fn set_back_error(strm: &mut crate::zlib_h::z_stream_s, message: &'static [u8]) {
    debug_assert_eq!(message.last(), Some(&0));
    strm.msg = message.as_ptr() as *mut ::core::ffi::c_char;
}

pub unsafe extern "C" fn inflateBackInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    if version.is_null()
        || *version.offset(0 as isize) as ::core::ffi::c_int
            != crate::zlib_h::ZLIB_VERSION[0 as usize] as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null()
        || window.is_null()
        || windowBits < 8 as ::core::ffi::c_int
        || windowBits > 15 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*strm).zalloc.is_none() {
        (*strm).zalloc = Some(
            crate::src::zutil::zcalloc
                as unsafe extern "C" fn(
                    crate::stdlib::voidpf,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> crate::stdlib::voidpf,
        ) as crate::zlib_h::alloc_func;
        (*strm).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*strm).zfree.is_none() {
        (*strm).zfree = Some(
            crate::src::zutil::zcfree
                as unsafe extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> (),
        ) as crate::zlib_h::free_func;
    }
    state = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    (*strm).state = state as *mut crate::src::deflate::internal_state;
    (*state).dmax = 32768 as ::core::ffi::c_uint;
    (*state).wbits = windowBits as crate::stdlib::uInt as ::core::ffi::c_uint;
    (*state).wsize = (1 as ::core::ffi::c_uint) << windowBits;
    (*state).window = window;
    (*state).wnext = 0 as ::core::ffi::c_uint;
    (*state).whave = 0 as ::core::ffi::c_uint;
    (*state).sane = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateBackInit_"]

pub unsafe extern "C" fn inflateBackInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflateBackInit_(strm, windowBits, window, version, stream_size)
}
pub unsafe extern "C" fn inflateBack(
    mut strm: crate::zlib_h::z_streamp,
    mut in_0: crate::zlib_h::in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: crate::zlib_h::out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut put: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut have: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut copy: ::core::ffi::c_uint = 0;
    let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut here: crate::src::inftrees::code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut last: crate::src::inftrees::code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    let mut len: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int = 0;
    static order: [::core::ffi::c_ushort; 19] = [
        16 as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_ushort,
        8 as ::core::ffi::c_ushort,
        7 as ::core::ffi::c_ushort,
        9 as ::core::ffi::c_ushort,
        6 as ::core::ffi::c_ushort,
        10 as ::core::ffi::c_ushort,
        5 as ::core::ffi::c_ushort,
        11 as ::core::ffi::c_ushort,
        4 as ::core::ffi::c_ushort,
        12 as ::core::ffi::c_ushort,
        3 as ::core::ffi::c_ushort,
        13 as ::core::ffi::c_ushort,
        2 as ::core::ffi::c_ushort,
        14 as ::core::ffi::c_ushort,
        1 as ::core::ffi::c_ushort,
        15 as ::core::ffi::c_ushort,
    ];
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    if strm.state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    state.mode = crate::src::inflate::TYPE;
    state.last = 0 as ::core::ffi::c_int;
    state.whave = 0 as ::core::ffi::c_uint;
    next = strm.next_in as *mut ::core::ffi::c_uchar;
    have = (if !next.is_null() {
        strm.avail_in
    } else {
        0 as crate::stdlib::uInt
    }) as ::core::ffi::c_uint;
    hold = 0 as ::core::ffi::c_ulong;
    bits = 0 as ::core::ffi::c_uint;
    put = state.window;
    left = state.wsize;
    '_inf_leave: loop {
        match state.mode as ::core::ffi::c_uint {
            16191 => {
                if state.last != 0 {
                    hold >>= bits & 7 as ::core::ffi::c_uint;
                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                    state.mode = crate::src::inflate::DONE;
                    continue;
                } else {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh0 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    state.last = (hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_int;
                    hold >>= 1 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    match hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint)
                    {
                        0 => {
                            state.mode = crate::src::inflate::STORED;
                        }
                        1 => {
                            crate::src::inftrees::inflate_fixed(
                                state as *mut crate::src::inflate::inflate_state,
                            );
                            state.mode = crate::src::inflate::LEN;
                        }
                        2 => {
                            state.mode = crate::src::inflate::TABLE;
                        }
                        _ => {
                            set_back_error(strm, b"invalid block type\0");
                            state.mode = crate::src::inflate::BAD;
                        }
                    }
                    hold >>= 2 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(2 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    continue;
                }
            }
            16193 => {
                hold >>= bits & 7 as ::core::ffi::c_uint;
                bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh1 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if hold & 0xffff as ::core::ffi::c_ulong
                    != hold >> 16 as ::core::ffi::c_int ^ 0xffff as ::core::ffi::c_ulong
                {
                    set_back_error(strm, b"invalid stored block lengths\0");
                    state.mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    state.length = hold as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint;
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    while state.length != 0 as ::core::ffi::c_uint {
                        copy = state.length;
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        if left == 0 as ::core::ffi::c_uint {
                            put = state.window;
                            left = state.wsize;
                            state.whave = left;
                            if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        if copy > have {
                            copy = have;
                        }
                        if copy > left {
                            copy = left;
                        }
                        crate::stdlib::memcpy(
                            put as *mut ::core::ffi::c_void,
                            next as *const ::core::ffi::c_void,
                            copy as crate::__stddef_size_t_h::size_t,
                        );
                        have = have.wrapping_sub(copy);
                        next = next.offset(copy as isize);
                        left = left.wrapping_sub(copy);
                        put = put.offset(copy as isize);
                        state.length = state.length.wrapping_sub(copy);
                    }
                    state.mode = crate::src::inflate::TYPE;
                    continue;
                }
            }
            16196 => {
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh2 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh2 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                state.nlen = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(257 as ::core::ffi::c_uint);
                hold >>= 5 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
                state.ndist = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(1 as ::core::ffi::c_uint);
                hold >>= 5 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
                state.ncode = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(4 as ::core::ffi::c_uint);
                hold >>= 4 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(4 as ::core::ffi::c_int as ::core::ffi::c_uint);
                if state.nlen > 286 as ::core::ffi::c_uint
                    || state.ndist > 30 as ::core::ffi::c_uint
                {
                    set_back_error(strm, b"too many length or distance symbols\0");
                    state.mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    state.have = 0 as ::core::ffi::c_uint;
                    while state.have < state.ncode {
                        while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                            if have == 0 as ::core::ffi::c_uint {
                                have = in_0.expect("non-null function pointer")(
                                    in_desc,
                                    &raw mut next,
                                );
                                if have == 0 as ::core::ffi::c_uint {
                                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh3 = next;
                            next = next.offset(1);
                            hold =
                                hold.wrapping_add((*c2rust_fresh3 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        let c2rust_fresh4 = state.have;
                        state.have = state.have.wrapping_add(1);
                        state.lens[order[c2rust_fresh4 as usize] as usize] = (hold
                            as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_ushort;
                        hold >>= 3 as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(3 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    }
                    while state.have < 19 as ::core::ffi::c_uint {
                        let c2rust_fresh5 = state.have;
                        state.have = state.have.wrapping_add(1);
                        state.lens[order[c2rust_fresh5 as usize] as usize] =
                            0 as ::core::ffi::c_ushort;
                    }
                    state.next = 0;
                    let mut table = &raw mut state.codes as *mut crate::src::inftrees::code;
                    state.lencode = table as *const crate::src::inftrees::code;
                    state.lenbits = 7 as ::core::ffi::c_uint;
                    ret = crate::src::inftrees::inflate_table(
                        crate::src::inftrees::CODES,
                        &raw mut state.lens as *mut ::core::ffi::c_ushort,
                        19 as ::core::ffi::c_uint,
                        &raw mut table,
                        &raw mut state.lenbits,
                        &raw mut state.work as *mut ::core::ffi::c_ushort,
                    );
                    state.next = table.addr().wrapping_sub((&raw mut state.codes as *mut crate::src::inftrees::code).addr()) / ::core::mem::size_of::<crate::src::inftrees::code>();
                    if ret != 0 {
                        set_back_error(strm, b"invalid code lengths set\0");
                        state.mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        state.have = 0 as ::core::ffi::c_uint;
                        while state.have < state.nlen.wrapping_add(state.ndist) {
                            loop {
                                here = *state.lencode.offset(
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint) << state.lenbits)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        as isize,
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                if have == 0 as ::core::ffi::c_uint {
                                    have = in_0.expect("non-null function pointer")(
                                        in_desc,
                                        &raw mut next,
                                    );
                                    if have == 0 as ::core::ffi::c_uint {
                                        next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                        ret = crate::zlib_h::Z_BUF_ERROR;
                                        break '_inf_leave;
                                    }
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh6 = next;
                                next = next.offset(1);
                                hold = hold
                                    .wrapping_add((*c2rust_fresh6 as ::core::ffi::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            if (here.val as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                                hold >>= here.bits as ::core::ffi::c_int;
                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                let c2rust_fresh7 = state.have;
                                state.have = state.have.wrapping_add(1);
                                state.lens[c2rust_fresh7 as usize] = here.val;
                            } else {
                                if here.val as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        if have == 0 as ::core::ffi::c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as ::core::ffi::c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                ret = crate::zlib_h::Z_BUF_ERROR;
                                                break '_inf_leave;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let c2rust_fresh8 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*c2rust_fresh8 as ::core::ffi::c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    if state.have == 0 as ::core::ffi::c_uint {
                                        set_back_error(strm, b"invalid bit length repeat\0");
                                        state.mode = crate::src::inflate::BAD;
                                        break;
                                    } else {
                                        len = state.lens[state
                                            .have
                                            .wrapping_sub(1 as ::core::ffi::c_uint)
                                            as usize]
                                            as ::core::ffi::c_uint;
                                        copy = (3 as ::core::ffi::c_uint).wrapping_add(
                                            hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint)
                                                    << 2 as ::core::ffi::c_int)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                        );
                                        hold >>= 2 as ::core::ffi::c_int;
                                        bits = bits.wrapping_sub(
                                            2 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        );
                                    }
                                } else if here.val as ::core::ffi::c_int == 17 as ::core::ffi::c_int
                                {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        if have == 0 as ::core::ffi::c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as ::core::ffi::c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                ret = crate::zlib_h::Z_BUF_ERROR;
                                                break '_inf_leave;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let c2rust_fresh9 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*c2rust_fresh9 as ::core::ffi::c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    len = 0 as ::core::ffi::c_uint;
                                    copy = (3 as ::core::ffi::c_uint).wrapping_add(
                                        hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint)
                                                << 3 as ::core::ffi::c_int)
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                    );
                                    hold >>= 3 as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(
                                        3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    );
                                } else {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 7 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        if have == 0 as ::core::ffi::c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as ::core::ffi::c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                ret = crate::zlib_h::Z_BUF_ERROR;
                                                break '_inf_leave;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let c2rust_fresh10 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*c2rust_fresh10 as ::core::ffi::c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    len = 0 as ::core::ffi::c_uint;
                                    copy = (11 as ::core::ffi::c_uint).wrapping_add(
                                        hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint)
                                                << 7 as ::core::ffi::c_int)
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                    );
                                    hold >>= 7 as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(
                                        7 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    );
                                }
                                if state.have.wrapping_add(copy)
                                    > state.nlen.wrapping_add(state.ndist)
                                {
                                    set_back_error(strm, b"invalid bit length repeat\0");
                                    state.mode = crate::src::inflate::BAD;
                                    break;
                                } else {
                                    loop {
                                        let c2rust_fresh11 = copy;
                                        copy = copy.wrapping_sub(1);
                                        if c2rust_fresh11 == 0 {
                                            break;
                                        }
                                        let c2rust_fresh12 = state.have;
                                        state.have = state.have.wrapping_add(1);
                                        state.lens[c2rust_fresh12 as usize] =
                                            len as ::core::ffi::c_ushort;
                                    }
                                }
                            }
                        }
                        if state.mode as ::core::ffi::c_uint
                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        if state.lens[256 as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            set_back_error(strm, b"invalid code -- missing end-of-block\0");
                            state.mode = crate::src::inflate::BAD;
                            continue;
                        } else {
                            state.next = 0;
                            let mut table = &raw mut state.codes as *mut crate::src::inftrees::code;
                            state.lencode = table as *const crate::src::inftrees::code;
                            state.lenbits = 9 as ::core::ffi::c_uint;
                            ret = crate::src::inftrees::inflate_table(
                                crate::src::inftrees::LENS,
                                &raw mut state.lens as *mut ::core::ffi::c_ushort,
                                state.nlen,
                                &raw mut table,
                                &raw mut state.lenbits,
                                &raw mut state.work as *mut ::core::ffi::c_ushort,
                            );
                            state.next = table.addr().wrapping_sub((&raw mut state.codes as *mut crate::src::inftrees::code).addr()) / ::core::mem::size_of::<crate::src::inftrees::code>();
                            if ret != 0 {
                                set_back_error(strm, b"invalid literal/lengths set\0");
                                state.mode = crate::src::inflate::BAD;
                                continue;
                            } else {
                                {
                                    let state_ref = &mut *state;
                                    state_ref.distcode =
                                        crate::src::inflate::distance_table::Dynamic(state_ref.next);
                                }
                                state.distbits = 6 as ::core::ffi::c_uint;
                                ret = crate::src::inftrees::inflate_table(
                                    crate::src::inftrees::DISTS,
                                    (&raw mut state.lens as *mut ::core::ffi::c_ushort)
                                        .offset(state.nlen as isize),
                                    state.ndist,
                                &raw mut table,
                                    &raw mut state.distbits,
                                    &raw mut state.work as *mut ::core::ffi::c_ushort,
                                );
                                state.next = table.addr().wrapping_sub((&raw mut state.codes as *mut crate::src::inftrees::code).addr()) / ::core::mem::size_of::<crate::src::inftrees::code>();
                                if ret != 0 {
                                    set_back_error(strm, b"invalid distances set\0");
                                    state.mode = crate::src::inflate::BAD;
                                    continue;
                                } else {
                                    state.mode = crate::src::inflate::LEN;
                                }
                            }
                        }
                    }
                }
            }
            16200 => {}
            16208 => {
                ret = crate::zlib_h::Z_STREAM_END;
                break;
            }
            16209 => {
                ret = crate::zlib_h::Z_DATA_ERROR;
                break;
            }
            _ => {
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break;
            }
        }
        if have >= 6 as ::core::ffi::c_uint && left >= 258 as ::core::ffi::c_uint {
            strm.next_out = put as *mut crate::stdlib::Bytef;
            strm.avail_out = left as crate::stdlib::uInt;
            strm.next_in = next as *mut crate::stdlib::Bytef;
            strm.avail_in = have as crate::stdlib::uInt;
            state.hold = hold;
            state.bits = bits;
            crate::src::inffast::inflate_fast(
                strm as *mut crate::zlib_h::z_stream_s,
                state.wsize,
            );
            put = strm.next_out as *mut ::core::ffi::c_uchar;
            left = strm.avail_out as ::core::ffi::c_uint;
            next = strm.next_in as *mut ::core::ffi::c_uchar;
            have = strm.avail_in as ::core::ffi::c_uint;
            hold = state.hold;
            bits = state.bits;
        } else {
            loop {
                here = *state.lencode.offset(
                    (hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << state.lenbits)
                            .wrapping_sub(1 as ::core::ffi::c_uint)) as isize,
                );
                if here.bits as ::core::ffi::c_uint <= bits {
                    break;
                }
                if have == 0 as ::core::ffi::c_uint {
                    have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                    if have == 0 as ::core::ffi::c_uint {
                        next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    }
                }
                have = have.wrapping_sub(1);
                let c2rust_fresh13 = next;
                next = next.offset(1);
                hold = hold.wrapping_add((*c2rust_fresh13 as ::core::ffi::c_ulong) << bits);
                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            }
            if here.op as ::core::ffi::c_int != 0
                && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                last = here;
                loop {
                    here = *state.lencode.offset(
                        (last.val as ::core::ffi::c_uint).wrapping_add(
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint)
                                    << last.bits as ::core::ffi::c_int
                                        + last.op as ::core::ffi::c_int)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                >> last.bits as ::core::ffi::c_int,
                        ) as isize,
                    );
                    if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        <= bits
                    {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh14 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh14 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                hold >>= last.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
            }
            hold >>= here.bits as ::core::ffi::c_int;
            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
            state.length = here.val as ::core::ffi::c_uint;
            if here.op as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if left == 0 as ::core::ffi::c_uint {
                    put = state.window;
                    left = state.wsize;
                    state.whave = left;
                    if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break;
                    }
                }
                let c2rust_fresh15 = put;
                put = put.offset(1);
                *c2rust_fresh15 = state.length as ::core::ffi::c_uchar;
                left = left.wrapping_sub(1);
                state.mode = crate::src::inflate::LEN;
            } else if here.op as ::core::ffi::c_int & 32 as ::core::ffi::c_int != 0 {
                state.mode = crate::src::inflate::TYPE;
            } else if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                set_back_error(strm, b"invalid literal/length code\0");
                state.mode = crate::src::inflate::BAD;
            } else {
                state.extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                if state.extra != 0 as ::core::ffi::c_uint {
                    while bits < state.extra {
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh16 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh16 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    state.length = state.length.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << state.extra)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= state.extra;
                    bits = bits.wrapping_sub(state.extra);
                }
                loop {
                    here = state.distcode.entry(
                        &state.codes,
                        (hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << state.distbits)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as usize,
                    );
                    if here.bits as ::core::ffi::c_uint <= bits {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh17 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*c2rust_fresh17 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    last = here;
                    loop {
                        here = state.distcode.entry(
                            &state.codes,
                            (last.val as ::core::ffi::c_uint).wrapping_add(
                                (hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint)
                                        << last.bits as ::core::ffi::c_int
                                            + last.op as ::core::ffi::c_int)
                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                    >> last.bits as ::core::ffi::c_int,
                            ) as usize,
                        );
                        if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            <= bits
                        {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh18 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*c2rust_fresh18 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    hold >>= last.bits as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                }
                hold >>= here.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                    set_back_error(strm, b"invalid distance code\0");
                    state.mode = crate::src::inflate::BAD;
                } else {
                    state.offset = here.val as ::core::ffi::c_uint;
                    state.extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                    if state.extra != 0 as ::core::ffi::c_uint {
                        while bits < state.extra {
                            if have == 0 as ::core::ffi::c_uint {
                                have = in_0.expect("non-null function pointer")(
                                    in_desc,
                                    &raw mut next,
                                );
                                if have == 0 as ::core::ffi::c_uint {
                                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh19 = next;
                            next = next.offset(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh19 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        state.offset = state.offset.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << state.extra)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= state.extra;
                        bits = bits.wrapping_sub(state.extra);
                    }
                    if state.offset
                        > state
                            .wsize
                            .wrapping_sub(if state.whave < state.wsize {
                                left
                            } else {
                                0 as ::core::ffi::c_uint
                            })
                    {
                        set_back_error(strm, b"invalid distance too far back\0");
                        state.mode = crate::src::inflate::BAD;
                    } else {
                        loop {
                            if left == 0 as ::core::ffi::c_uint {
                                put = state.window;
                                left = state.wsize;
                                state.whave = left;
                                if out.expect("non-null function pointer")(out_desc, put, left) != 0
                                {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            copy = state.wsize.wrapping_sub(state.offset);
                            if copy < left {
                                from = put.offset(copy as isize);
                                copy = left.wrapping_sub(copy);
                            } else {
                                from = put.offset(-(state.offset as isize));
                                copy = left;
                            }
                            if copy > state.length {
                                copy = state.length;
                            }
                            state.length = state.length.wrapping_sub(copy);
                            left = left.wrapping_sub(copy);
                            loop {
                                let c2rust_fresh20 = from;
                                from = from.offset(1);
                                let c2rust_fresh21 = put;
                                put = put.offset(1);
                                *c2rust_fresh21 = *c2rust_fresh20;
                                copy = copy.wrapping_sub(1);
                                if copy == 0 {
                                    break;
                                }
                            }
                            if state.length == 0 as ::core::ffi::c_uint {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    if left < state.wsize {
        if out.expect("non-null function pointer")(
            out_desc,
            state.window,
            state.wsize.wrapping_sub(left),
        ) != 0
            && ret == crate::zlib_h::Z_STREAM_END
        {
            ret = crate::zlib_h::Z_BUF_ERROR;
        }
    }
    strm.next_in = next as *mut crate::stdlib::Bytef;
    strm.avail_in = have as crate::stdlib::uInt;
    return ret;
}
#[export_name = "inflateBack"]

pub unsafe extern "C" fn inflateBack_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut in_0: crate::zlib_h::in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: crate::zlib_h::out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    inflateBack(strm, in_0, in_desc, out, out_desc)
}
pub unsafe extern "C" fn inflateBackEnd(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() || (*strm).state.is_null() || (*strm).zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as crate::stdlib::voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateBackEnd"]

pub unsafe extern "C" fn inflateBackEnd_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflateBackEnd(strm)
}
