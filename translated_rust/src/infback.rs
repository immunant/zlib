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

// Keep the validation and scalar geometry for callback-back mode independent
// of its ABI stream and caller window.  This is the first piece of the
// pointer-free back-mode owner: a later call-scoped window/callback facade can
// consume this plan without repeating the FFI checks.
struct InflateBackInitPlan {
    wbits: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
}

impl InflateBackInitPlan {
    fn new(
        version_matches: bool,
        window_bits: ::core::ffi::c_int,
        stream_size: ::core::ffi::c_int,
    ) -> Result<Self, ::core::ffi::c_int> {
        if !version_matches
            || stream_size
                != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
        {
            return Err(crate::zlib_h::Z_VERSION_ERROR);
        }
        if !(8..=15).contains(&window_bits) {
            return Err(crate::zlib_h::Z_STREAM_ERROR);
        }
        Ok(Self {
            wbits: window_bits as ::core::ffi::c_uint,
            wsize: 1u32 << window_bits,
        })
    }
}

pub unsafe extern "C" fn inflateBackInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let version_matches = !version.is_null()
        && *version.offset(0) as ::core::ffi::c_int
            == crate::zlib_h::ZLIB_VERSION[0] as ::core::ffi::c_int;
    let plan = match InflateBackInitPlan::new(version_matches, windowBits, stream_size) {
        Ok(plan) => plan,
        Err(status) => return status,
    };
    if strm.is_null() || window.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Keep the ABI stream projection at the allocation boundary.  All
    // subsequent setup uses this scoped Rust borrow, rather than repeatedly
    // dereferencing the caller's raw stream pointer.
    let strm = &mut *strm;
    let stream_identity = ::core::ptr::from_mut(strm).addr();
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if strm.zalloc.is_none() {
        strm.zalloc = Some(
            crate::src::zutil::zcalloc
                as unsafe extern "C" fn(
                    crate::stdlib::voidpf,
                    ::core::ffi::c_uint,
                    ::core::ffi::c_uint,
                ) -> crate::stdlib::voidpf,
        ) as crate::zlib_h::alloc_func;
        strm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if strm.zfree.is_none() {
        strm.zfree = Some(
            crate::src::zutil::zcfree
                as unsafe extern "C" fn(crate::stdlib::voidpf, crate::stdlib::voidpf) -> (),
        ) as crate::zlib_h::free_func;
    }
    let state = Some(strm.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        strm.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    strm.state = state as *mut crate::src::deflate::internal_state;
    // Back-mode state uses the same allocation/release contract as normal
    // inflate.  Publish one fully initialized value into the callback-owned
    // allocation, whose returned bytes need not have been initialized. Keep
    // this construction at its sole raw publication site: a separate unsafe
    // constructor added an unsafe call and an unsafe function without making
    // the callback window any safer.
    ::core::ptr::write(
        state,
        crate::src::inflate::inflate_state {
            stream_identity,
            mode: crate::src::inflate::TYPE,
            last: 0,
            wrap: 0,
            havedict: 0,
            flags: 0,
            dmax: 32768,
            check: 0,
            total: 0,
            head: None,
            wbits: plan.wbits,
            wsize: plan.wsize,
            whave: 0,
            wnext: 0,
            window: Some(::core::ptr::NonNull::new(window).expect("validated caller window")),
            owned_window: None,
            hold: 0,
            bits: 0,
            length: 0,
            offset: 0,
            extra: 0,
            lencode: crate::src::inflate::CodeTableRef::Dynamic(0),
            distcode: crate::src::inflate::CodeTableRef::Dynamic(0),
            lenbits: 0,
            distbits: 0,
            ncode: 0,
            nlen: 0,
            ndist: 0,
            have: 0,
            next: 0,
            lens: [0; 320],
            work: [0; 288],
            codes: ::core::array::from_fn(|_| crate::src::inftrees::code {
                op: 0,
                bits: 0,
                val: 0,
            }),
            sane: 1,
            back: 0,
            was: 0,
        },
    );
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

// Callback-back input is valid only for the byte count returned by `in`.
// Keep that one raw callback/cursor transition in a named helper so the
// decoder below can consume bytes without repeating unchecked pointer work at
// every bit-reader site.  The returned byte is copied before the cursor
// advances, and a zero-length callback result keeps zlib's null-cursor
// convention for the final stream publication.
unsafe fn inflate_back_pull_byte(
    input: crate::zlib_h::in_func,
    input_desc: *mut ::core::ffi::c_void,
    next: &mut *mut ::core::ffi::c_uchar,
    have: &mut ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uchar> {
    if *have == 0 {
        *have = input.expect("non-null function pointer")(input_desc, &raw mut *next);
        if *have == 0 {
            *next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            return None;
        }
    }
    *have = have.wrapping_sub(1);
    let byte = **next;
    *next = next.offset(1);
    Some(byte)
}

pub unsafe extern "C" fn inflateBack(
    mut strm: crate::zlib_h::z_streamp,
    mut in_0: crate::zlib_h::in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: crate::zlib_h::out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut put: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut have: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut copy: ::core::ffi::c_uint = 0;
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
    if strm.is_null() || (*strm).state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    // Keep the two ABI projections at this boundary. The decoder below uses
    // scoped Rust borrows; only the callback cursors remain raw.
    let strm = &mut *strm;
    let state = &mut *state;
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).mode = crate::src::inflate::TYPE;
    (*state).last = 0 as ::core::ffi::c_int;
    (*state).whave = 0 as ::core::ffi::c_uint;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    have = (if !next.is_null() {
        (*strm).avail_in
    } else {
        0 as crate::stdlib::uInt
    }) as ::core::ffi::c_uint;
    hold = 0 as ::core::ffi::c_ulong;
    bits = 0 as ::core::ffi::c_uint;
    put = (*state).window.expect("inflateBack window").as_ptr();
    left = (*state).wsize;
    '_inf_leave: loop {
        match (*state).mode as ::core::ffi::c_uint {
            16191 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as ::core::ffi::c_uint;
                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                    (*state).mode = crate::src::inflate::DONE;
                    continue;
                } else {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        let Some(byte) =
                            inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                        else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    (*state).last = (hold as ::core::ffi::c_uint
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
                            (*state).mode = crate::src::inflate::STORED;
                        }
                        1 => {
                            crate::src::inftrees::inflate_fixed(
                                &mut state.lencode,
                                &mut state.lenbits,
                                &mut state.distcode,
                                &mut state.distbits,
                            );
                            state.mode = crate::src::inflate::LEN;
                        }
                        2 => {
                            (*state).mode = crate::src::inflate::TABLE;
                        }
                        _ => {
                            (*strm).msg = b"invalid block type\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
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
                    let Some(byte) = inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                    else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if hold & 0xffff as ::core::ffi::c_ulong
                    != hold >> 16 as ::core::ffi::c_int ^ 0xffff as ::core::ffi::c_ulong
                {
                    (*strm).msg = b"invalid stored block lengths\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    (*state).length = hold as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint;
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    while (*state).length != 0 as ::core::ffi::c_uint {
                        copy = (*state).length;
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        if left == 0 as ::core::ffi::c_uint {
                            put = (*state).window.expect("inflateBack window").as_ptr();
                            left = (*state).wsize;
                            (*state).whave = left;
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
                        // This is the translated C `memcpy` path: input and
                        // caller window are distinct callback buffers, so
                        // preserve its non-overlap requirement without a C
                        // memory call. The callback-state slice facade will
                        // eventually make this copy fully safe.
                        ::core::ptr::copy_nonoverlapping(next, put, copy as usize);
                        have = have.wrapping_sub(copy);
                        next = next.offset(copy as isize);
                        left = left.wrapping_sub(copy);
                        put = put.offset(copy as isize);
                        (*state).length = (*state).length.wrapping_sub(copy);
                    }
                    (*state).mode = crate::src::inflate::TYPE;
                    continue;
                }
            }
            16196 => {
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    let Some(byte) = inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                    else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                (*state).nlen = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(257 as ::core::ffi::c_uint);
                hold >>= 5 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
                (*state).ndist = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(1 as ::core::ffi::c_uint);
                hold >>= 5 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
                (*state).ncode = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(4 as ::core::ffi::c_uint);
                hold >>= 4 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(4 as ::core::ffi::c_int as ::core::ffi::c_uint);
                if (*state).nlen > 286 as ::core::ffi::c_uint
                    || (*state).ndist > 30 as ::core::ffi::c_uint
                {
                    (*strm).msg = b"too many length or distance symbols\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    (*state).have = 0 as ::core::ffi::c_uint;
                    while (*state).have < (*state).ncode {
                        while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                            let Some(byte) =
                                inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                            else {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            };
                            hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        let c2rust_fresh4 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[order[c2rust_fresh4 as usize] as usize] = (hold
                            as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_ushort;
                        hold >>= 3 as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(3 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    }
                    while (*state).have < 19 as ::core::ffi::c_uint {
                        let c2rust_fresh5 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[order[c2rust_fresh5 as usize] as usize] =
                            0 as ::core::ffi::c_ushort;
                    }
                    (*state).next = 0;
                    (*state).lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                    (*state).lenbits = 7 as ::core::ffi::c_uint;
                    ret = 'table: {
                        let table_start = state.next;
                        let Some(lens) = state.lens.get(..19) else {
                            break 'table 1;
                        };
                        let Some(table) = state.codes.get_mut(table_start..) else {
                            break 'table 1;
                        };
                        let Some(work) = state.work.get_mut(..19) else {
                            break 'table 1;
                        };
                        let (status, used) = crate::src::inftrees::inflate_table(
                            crate::src::inftrees::CODES,
                            lens,
                            table,
                            &mut state.lenbits,
                            work,
                        );
                        if status == 0 {
                            state.next += used;
                        }
                        status
                    };
                    if ret != 0 {
                        (*strm).msg = b"invalid code lengths set\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        (*state).have = 0 as ::core::ffi::c_uint;
                        while (*state).have < (*state).nlen.wrapping_add((*state).ndist) {
                            loop {
                                here = crate::src::inftrees::code::copied_from(
                                    (*state).lencode.get(
                                        &(*state).codes,
                                        (hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                                .wrapping_sub(1 as ::core::ffi::c_uint))
                                            as isize,
                                    ),
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                let Some(byte) =
                                    inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                                else {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                };
                                hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            if (here.val as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                                hold >>= here.bits as ::core::ffi::c_int;
                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                let c2rust_fresh7 = (*state).have;
                                (*state).have = (*state).have.wrapping_add(1);
                                (*state).lens[c2rust_fresh7 as usize] = here.val;
                            } else {
                                if here.val as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        let Some(byte) = inflate_back_pull_byte(
                                            in_0, in_desc, &mut next, &mut have,
                                        ) else {
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break '_inf_leave;
                                        };
                                        hold = hold
                                            .wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    if (*state).have == 0 as ::core::ffi::c_uint {
                                        (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char;
                                        (*state).mode = crate::src::inflate::BAD;
                                        break;
                                    } else {
                                        len = (*state).lens[(*state)
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
                                        let Some(byte) = inflate_back_pull_byte(
                                            in_0, in_desc, &mut next, &mut have,
                                        ) else {
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break '_inf_leave;
                                        };
                                        hold = hold
                                            .wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                                        let Some(byte) = inflate_back_pull_byte(
                                            in_0, in_desc, &mut next, &mut have,
                                        ) else {
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break '_inf_leave;
                                        };
                                        hold = hold
                                            .wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                                if (*state).have.wrapping_add(copy)
                                    > (*state).nlen.wrapping_add((*state).ndist)
                                {
                                    (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    (*state).mode = crate::src::inflate::BAD;
                                    break;
                                } else {
                                    loop {
                                        let c2rust_fresh11 = copy;
                                        copy = copy.wrapping_sub(1);
                                        if c2rust_fresh11 == 0 {
                                            break;
                                        }
                                        let c2rust_fresh12 = (*state).have;
                                        (*state).have = (*state).have.wrapping_add(1);
                                        (*state).lens[c2rust_fresh12 as usize] =
                                            len as ::core::ffi::c_ushort;
                                    }
                                }
                            }
                        }
                        if (*state).mode as ::core::ffi::c_uint
                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        if (*state).lens[256 as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*strm).msg = b"invalid code -- missing end-of-block\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                            continue;
                        } else {
                            (*state).next = 0;
                            (*state).lencode = crate::src::inflate::CodeTableRef::Dynamic(0);
                            (*state).lenbits = 9 as ::core::ffi::c_uint;
                            ret = 'table: {
                                let codes = state.nlen as usize;
                                let table_start = state.next;
                                let Some(lens) = state.lens.get(..codes) else {
                                    break 'table 1;
                                };
                                let Some(table) = state.codes.get_mut(table_start..) else {
                                    break 'table 1;
                                };
                                let Some(work) = state.work.get_mut(..codes) else {
                                    break 'table 1;
                                };
                                let (status, used) = crate::src::inftrees::inflate_table(
                                    crate::src::inftrees::LENS,
                                    lens,
                                    table,
                                    &mut state.lenbits,
                                    work,
                                );
                                if status == 0 {
                                    state.next += used;
                                }
                                status
                            };
                            if ret != 0 {
                                (*strm).msg = b"invalid literal/lengths set\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            } else {
                                (*state).distcode =
                                    crate::src::inflate::CodeTableRef::Dynamic((*state).next);
                                (*state).distbits = 6 as ::core::ffi::c_uint;
                                ret = 'table: {
                                    let lens_start = state.nlen as usize;
                                    let codes = state.ndist as usize;
                                    let Some(lens_end) = lens_start.checked_add(codes) else {
                                        break 'table 1;
                                    };
                                    let table_start = state.next;
                                    let Some(lens) = state.lens.get(lens_start..lens_end) else {
                                        break 'table 1;
                                    };
                                    let Some(table) = state.codes.get_mut(table_start..) else {
                                        break 'table 1;
                                    };
                                    let Some(work) = state.work.get_mut(..codes) else {
                                        break 'table 1;
                                    };
                                    let (status, used) = crate::src::inftrees::inflate_table(
                                        crate::src::inftrees::DISTS,
                                        lens,
                                        table,
                                        &mut state.distbits,
                                        work,
                                    );
                                    if status == 0 {
                                        state.next += used;
                                    }
                                    status
                                };
                                if ret != 0 {
                                    (*strm).msg = b"invalid distances set\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    (*state).mode = crate::src::inflate::BAD;
                                    continue;
                                } else {
                                    (*state).mode = crate::src::inflate::LEN;
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
            // Both callback cursors are bounded for this dispatch: `have`
            // describes the input callback's current chunk, and `put` lies
            // in the caller window with `left` bytes remaining.  Decode
            // directly through those views instead of republishing them via
            // the legacy raw-stream fast adapter.
            let input = ::core::slice::from_raw_parts(next, have as usize);
            let window = state.window.expect("inflateBack window");
            let window_size = state.wsize as usize;
            let output = ::core::slice::from_raw_parts_mut(window.as_ptr(), window_size);
            let written = window_size.wrapping_sub(left as usize);
            let mut fast_state = crate::src::inffast::InflateFastState {
                history: crate::src::inffast::FastHistory::Output,
                wsize: window_size,
                whave: state.whave as usize,
                wnext: state.wnext as usize,
                hold,
                bits,
                lcode: state.lencode,
                dcode: state.distcode,
                lmask: (1u32 << state.lenbits) - 1,
                dmask: (1u32 << state.distbits) - 1,
                codes: &state.codes,
                sane: state.sane != 0,
            };
            let result = crate::src::inffast::inflate_fast_from_views(
                input,
                output,
                written,
                &mut fast_state,
            );
            next = input.as_ptr().wrapping_add(result.input_used) as *mut ::core::ffi::c_uchar;
            have = input.len().wrapping_sub(result.input_used) as ::core::ffi::c_uint;
            put = output.as_mut_ptr().wrapping_add(result.output_used);
            left = output.len().wrapping_sub(result.output_used) as ::core::ffi::c_uint;
            hold = fast_state.hold;
            bits = fast_state.bits;
            match result.exit {
                crate::src::inffast::FastExit::Continue => {}
                crate::src::inffast::FastExit::Type => state.mode = crate::src::inflate::TYPE,
                crate::src::inffast::FastExit::InvalidDistance => {
                    (*strm).msg = b"invalid distance too far back\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state.mode = crate::src::inflate::BAD;
                }
                crate::src::inffast::FastExit::InvalidCode => {
                    (*strm).msg = b"invalid literal/length or distance code\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state.mode = crate::src::inflate::BAD;
                }
            }
        } else {
            loop {
                here = crate::src::inftrees::code::copied_from(
                    (*state).lencode.get(
                        &(*state).codes,
                        (hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as isize,
                    ),
                );
                if here.bits as ::core::ffi::c_uint <= bits {
                    break;
                }
                let Some(byte) = inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have) else {
                    ret = crate::zlib_h::Z_BUF_ERROR;
                    break '_inf_leave;
                };
                hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            }
            if here.op as ::core::ffi::c_int != 0
                && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                last = crate::src::inftrees::code::copied_from(&here);
                loop {
                    here = crate::src::inftrees::code::copied_from(
                        (*state).lencode.get(
                            &(*state).codes,
                            (last.val as ::core::ffi::c_uint).wrapping_add(
                                (hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint)
                                        << last.bits as ::core::ffi::c_int
                                            + last.op as ::core::ffi::c_int)
                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                    >> last.bits as ::core::ffi::c_int,
                            ) as isize,
                        ),
                    );
                    if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        <= bits
                    {
                        break;
                    }
                    let Some(byte) = inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                    else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                hold >>= last.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
            }
            hold >>= here.bits as ::core::ffi::c_int;
            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
            (*state).length = here.val as ::core::ffi::c_uint;
            if here.op as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if left == 0 as ::core::ffi::c_uint {
                    put = (*state).window.expect("inflateBack window").as_ptr();
                    left = (*state).wsize;
                    (*state).whave = left;
                    if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break;
                    }
                }
                let c2rust_fresh15 = put;
                put = put.offset(1);
                *c2rust_fresh15 = (*state).length as ::core::ffi::c_uchar;
                left = left.wrapping_sub(1);
                (*state).mode = crate::src::inflate::LEN;
            } else if here.op as ::core::ffi::c_int & 32 as ::core::ffi::c_int != 0 {
                (*state).mode = crate::src::inflate::TYPE;
            } else if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                (*strm).msg = b"invalid literal/length code\0".as_ptr()
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                (*state).mode = crate::src::inflate::BAD;
            } else {
                (*state).extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                if (*state).extra != 0 as ::core::ffi::c_uint {
                    while bits < (*state).extra {
                        let Some(byte) =
                            inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                        else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    (*state).length = (*state).length.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                }
                loop {
                    here = crate::src::inftrees::code::copied_from(
                        (*state).distcode.get(
                            &(*state).codes,
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << (*state).distbits)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                as isize,
                        ),
                    );
                    if here.bits as ::core::ffi::c_uint <= bits {
                        break;
                    }
                    let Some(byte) = inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                    else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    last = crate::src::inftrees::code::copied_from(&here);
                    loop {
                        here = crate::src::inftrees::code::copied_from(
                            (*state).distcode.get(
                                &(*state).codes,
                                (last.val as ::core::ffi::c_uint).wrapping_add(
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint)
                                            << last.bits as ::core::ffi::c_int
                                                + last.op as ::core::ffi::c_int)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        >> last.bits as ::core::ffi::c_int,
                                ) as isize,
                            ),
                        );
                        if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            <= bits
                        {
                            break;
                        }
                        let Some(byte) =
                            inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                        else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    hold >>= last.bits as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                }
                hold >>= here.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                    (*strm).msg = b"invalid distance code\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                } else {
                    (*state).offset = here.val as ::core::ffi::c_uint;
                    (*state).extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                    if (*state).extra != 0 as ::core::ffi::c_uint {
                        while bits < (*state).extra {
                            let Some(byte) =
                                inflate_back_pull_byte(in_0, in_desc, &mut next, &mut have)
                            else {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            };
                            hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        (*state).offset = (*state).offset.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= (*state).extra;
                        bits = bits.wrapping_sub((*state).extra);
                    }
                    if (*state).offset
                        > (*state)
                            .wsize
                            .wrapping_sub(if (*state).whave < (*state).wsize {
                                left
                            } else {
                                0 as ::core::ffi::c_uint
                            })
                    {
                        (*strm).msg = b"invalid distance too far back\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                    } else {
                        loop {
                            if left == 0 as ::core::ffi::c_uint {
                                put = (*state).window.expect("inflateBack window").as_ptr();
                                left = (*state).wsize;
                                (*state).whave = left;
                                if out.expect("non-null function pointer")(out_desc, put, left) != 0
                                {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            // `offset` was checked against the amount of
                            // history represented by this caller window.
                            // Rebuild that bounded window only for this
                            // copy, so the overlapping match uses Rust's
                            // memmove-equivalent slice operation.
                            let window = ::core::slice::from_raw_parts_mut(
                                (*state).window.expect("inflateBack window").as_ptr(),
                                (*state).wsize as usize,
                            );
                            let put_index = window.len().wrapping_sub(left as usize);
                            let back = (*state).wsize.wrapping_sub((*state).offset) as usize;
                            let (from_index, available) = if back < left as usize {
                                (
                                    put_index.wrapping_add(back),
                                    (left as usize).wrapping_sub(back),
                                )
                            } else {
                                (
                                    put_index.wrapping_sub((*state).offset as usize),
                                    left as usize,
                                )
                            };
                            let count = available.min((*state).length as usize);
                            let Some(from_end) = from_index.checked_add(count) else {
                                ret = crate::zlib_h::Z_DATA_ERROR;
                                break '_inf_leave;
                            };
                            let Some(put_end) = put_index.checked_add(count) else {
                                ret = crate::zlib_h::Z_DATA_ERROR;
                                break '_inf_leave;
                            };
                            if from_end > window.len() || put_end > window.len() {
                                ret = crate::zlib_h::Z_DATA_ERROR;
                                break '_inf_leave;
                            }
                            window.copy_within(from_index..from_end, put_index);
                            (*state).length = (*state).length.wrapping_sub(count as u32);
                            left = left.wrapping_sub(count as u32);
                            put = window.as_mut_ptr().wrapping_add(put_end);
                            if (*state).length == 0 as ::core::ffi::c_uint {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    if left < (*state).wsize {
        if out.expect("non-null function pointer")(
            out_desc,
            (*state).window.expect("inflateBack window").as_ptr(),
            (*state).wsize.wrapping_sub(left),
        ) != 0
            && ret == crate::zlib_h::Z_STREAM_END
        {
            ret = crate::zlib_h::Z_BUF_ERROR;
        }
    }
    (*strm).next_in = next as *mut crate::stdlib::Bytef;
    (*strm).avail_in = have as crate::stdlib::uInt;
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
#[export_name = "inflateBackEnd"]

pub unsafe extern "C" fn inflateBackEnd_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    crate::src::inflate::inflateEnd(strm)
}
