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

fn copy_match_bytes(window: &mut [u8], source: usize, destination: usize, count: usize) -> bool {
    let Some(source_end) = source.checked_add(count) else {
        return false;
    };
    let Some(destination_end) = destination.checked_add(count) else {
        return false;
    };
    if source_end > window.len() || destination_end > window.len() {
        return false;
    }
    for offset in 0..count {
        window[destination + offset] = window[source + offset];
    }
    true
}

/// A call-scoped input source for `inflateBack()`.
///
/// The FFI wrapper turns each callback result into a borrowed slice.  The
/// decoder never retains that slice across another refill, matching zlib's
/// callback lifetime rule without carrying a raw cursor through the decoder.
struct BackInput<'a, F> {
    refill: F,
    current: Option<&'a [u8]>,
}

impl<'a, F> BackInput<'a, F>
where
    F: FnMut() -> Option<&'a [u8]>,
{
    fn new(refill: F, current: Option<&'a [u8]>) -> Self {
        Self { refill, current }
    }

    fn refill_if_empty(&mut self) -> bool {
        if self.current.is_none_or(<[u8]>::is_empty) {
            self.current = (self.refill)();
        }
        self.current.is_some()
    }

    fn next_byte(&mut self) -> Option<u8> {
        if !self.refill_if_empty() {
            return None;
        }
        let current = self.current.as_mut().expect("refill succeeded");
        let byte = current[0];
        *current = &current[1..];
        Some(byte)
    }

    fn available(&mut self) -> Option<usize> {
        self.refill_if_empty()
            .then(|| self.current.expect("refill succeeded").len())
    }

    fn copy_to(&mut self, output: &mut [u8]) {
        let current = self.current.as_mut().expect("input is available");
        debug_assert!(output.len() <= current.len());
        output.copy_from_slice(&current[..output.len()]);
        *current = &current[output.len()..];
    }

    fn remaining(&self) -> Option<&'a [u8]> {
        self.current
    }
}

/// A call-scoped output sink for `inflateBack()`.
struct BackOutput<F> {
    write: F,
}

impl<F> BackOutput<F>
where
    F: FnMut(&[u8]) -> bool,
{
    fn new(write: F) -> Self {
        Self { write }
    }

    fn flush(&mut self, output: &[u8]) -> bool {
        (self.write)(output)
    }
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
    let window_len = 1usize << windowBits;
    let mut window_storage = Vec::new();
    if window_storage.try_reserve_exact(window_len).is_err() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    window_storage.resize(window_len, 0);
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
    state.write(crate::src::inflate::new_inflate_state());
    (*strm).state = state as *mut crate::src::deflate::internal_state;
    (*state).dmax = 32768 as ::core::ffi::c_uint;
    (*state).wbits = windowBits as crate::stdlib::uInt as ::core::ffi::c_uint;
    (*state).wsize = (1 as ::core::ffi::c_uint) << windowBits;
    (*state).window = Some(window_storage);
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
fn inflate_back_impl<'a, I, O>(
    strm: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [u8],
    input: &mut BackInput<'a, I>,
    output: &mut BackOutput<O>,
) -> ::core::ffi::c_int
where
    I: FnMut() -> Option<&'a [u8]>,
    O: FnMut(&[u8]) -> bool,
{
    let mut put = 0usize;
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
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    state.mode = crate::src::inflate::TYPE;
    state.last = 0 as ::core::ffi::c_int;
    state.whave = 0 as ::core::ffi::c_uint;
    hold = 0 as ::core::ffi::c_ulong;
    bits = 0 as ::core::ffi::c_uint;
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
                        let Some(byte) = input.next_byte() else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                                &mut state.lencode,
                                &mut state.lenbits,
                                &mut state.distcode,
                                &mut state.distbits,
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
                    let Some(byte) = input.next_byte() else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                        if left == 0 as ::core::ffi::c_uint {
                            put = 0;
                            left = state.wsize;
                            state.whave = left;
                            if output.flush(&window[..left as usize]) {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        let Some(have) = input.available() else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        if copy > have as ::core::ffi::c_uint {
                            copy = have as ::core::ffi::c_uint;
                        }
                        if copy > left {
                            copy = left;
                        }
                        input.copy_to(&mut window[put..put + copy as usize]);
                        left = left.wrapping_sub(copy);
                        put += copy as usize;
                        state.length = state.length.wrapping_sub(copy);
                    }
                    state.mode = crate::src::inflate::TYPE;
                    continue;
                }
            }
            16196 => {
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    let Some(byte) = input.next_byte() else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                            let Some(byte) = input.next_byte() else {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            };
                            hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                    state.lencode = crate::src::inflate::length_table::Dynamic(0);
                    state.lenbits = 7 as ::core::ffi::c_uint;
                    ret = unsafe {
                        crate::src::inftrees::inflate_table(
                            crate::src::inftrees::CODES,
                            &raw mut state.lens as *mut ::core::ffi::c_ushort,
                            19 as ::core::ffi::c_uint,
                            &raw mut table,
                            &raw mut state.lenbits,
                            &raw mut state.work as *mut ::core::ffi::c_ushort,
                        )
                    };
                    state.next = table.addr().wrapping_sub(
                        (&raw mut state.codes as *mut crate::src::inftrees::code).addr(),
                    ) / ::core::mem::size_of::<crate::src::inftrees::code>();
                    if ret != 0 {
                        set_back_error(strm, b"invalid code lengths set\0");
                        state.mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        state.have = 0 as ::core::ffi::c_uint;
                        while state.have < state.nlen.wrapping_add(state.ndist) {
                            loop {
                                here = state.lencode.entry(
                                    &state.codes,
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint) << state.lenbits)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        as usize,
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                let Some(byte) = input.next_byte() else {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                };
                                hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                                        let Some(byte) = input.next_byte() else {
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break '_inf_leave;
                                        };
                                        hold = hold
                                            .wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                                        let Some(byte) = input.next_byte() else {
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
                                        let Some(byte) = input.next_byte() else {
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
                        if state.lens[256 as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            set_back_error(strm, b"invalid code -- missing end-of-block\0");
                            state.mode = crate::src::inflate::BAD;
                            continue;
                        } else {
                            state.next = 0;
                            let mut table = &raw mut state.codes as *mut crate::src::inftrees::code;
                            state.lencode = crate::src::inflate::length_table::Dynamic(0);
                            state.lenbits = 9 as ::core::ffi::c_uint;
                            ret = unsafe {
                                crate::src::inftrees::inflate_table(
                                    crate::src::inftrees::LENS,
                                    &raw mut state.lens as *mut ::core::ffi::c_ushort,
                                    state.nlen,
                                    &raw mut table,
                                    &raw mut state.lenbits,
                                    &raw mut state.work as *mut ::core::ffi::c_ushort,
                                )
                            };
                            state.next = table.addr().wrapping_sub(
                                (&raw mut state.codes as *mut crate::src::inftrees::code).addr(),
                            ) / ::core::mem::size_of::<crate::src::inftrees::code>();
                            if ret != 0 {
                                set_back_error(strm, b"invalid literal/lengths set\0");
                                state.mode = crate::src::inflate::BAD;
                                continue;
                            } else {
                                state.distcode =
                                    crate::src::inflate::distance_table::Dynamic(state.next);
                                state.distbits = 6 as ::core::ffi::c_uint;
                                ret = unsafe {
                                    crate::src::inftrees::inflate_table(
                                        crate::src::inftrees::DISTS,
                                        (&raw mut state.lens as *mut ::core::ffi::c_ushort)
                                            .offset(state.nlen as isize),
                                        state.ndist,
                                        &raw mut table,
                                        &raw mut state.distbits,
                                        &raw mut state.work as *mut ::core::ffi::c_ushort,
                                    )
                                };
                                state.next = table.addr().wrapping_sub(
                                    (&raw mut state.codes as *mut crate::src::inftrees::code)
                                        .addr(),
                                ) / ::core::mem::size_of::<crate::src::inftrees::code>(
                                );
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
        // The ordinary decoder below implements the same transition sequence as
        // the optional fast path.  Keeping this path local avoids borrowing the
        // stream through the raw-pointer fast decoder in back-streaming mode.
        loop {
            here = state.lencode.entry(
                &state.codes,
                (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << state.lenbits)
                        .wrapping_sub(1 as ::core::ffi::c_uint)) as usize,
            );
            if here.bits as ::core::ffi::c_uint <= bits {
                break;
            }
            let Some(byte) = input.next_byte() else {
                ret = crate::zlib_h::Z_BUF_ERROR;
                break '_inf_leave;
            };
            hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
        }
        if here.op as ::core::ffi::c_int != 0
            && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            last = here;
            loop {
                here = state.lencode.entry(
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
                let Some(byte) = input.next_byte() else {
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
        state.length = here.val as ::core::ffi::c_uint;
        if here.op as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if left == 0 as ::core::ffi::c_uint {
                put = 0;
                left = state.wsize;
                state.whave = left;
                if output.flush(&window[..left as usize]) {
                    ret = crate::zlib_h::Z_BUF_ERROR;
                    break;
                }
            }
            window[put] = state.length as u8;
            put += 1;
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
                    let Some(byte) = input.next_byte() else {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    };
                    hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                            .wrapping_sub(1 as ::core::ffi::c_uint)) as usize,
                );
                if here.bits as ::core::ffi::c_uint <= bits {
                    break;
                }
                let Some(byte) = input.next_byte() else {
                    ret = crate::zlib_h::Z_BUF_ERROR;
                    break '_inf_leave;
                };
                hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            }
            if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
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
                    let Some(byte) = input.next_byte() else {
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
                set_back_error(strm, b"invalid distance code\0");
                state.mode = crate::src::inflate::BAD;
            } else {
                state.offset = here.val as ::core::ffi::c_uint;
                state.extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                if state.extra != 0 as ::core::ffi::c_uint {
                    while bits < state.extra {
                        let Some(byte) = input.next_byte() else {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        };
                        hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits);
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
                    > state.wsize.wrapping_sub(if state.whave < state.wsize {
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
                            put = 0;
                            left = state.wsize;
                            state.whave = left;
                            if output.flush(&window[..left as usize]) {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        let put_index = put;
                        copy = state.wsize.wrapping_sub(state.offset);
                        let from_index: usize;
                        if copy < left {
                            from_index = put_index + copy as usize;
                            copy = left.wrapping_sub(copy);
                        } else {
                            from_index = put_index.wrapping_sub(state.offset as usize);
                            copy = left;
                        }
                        if copy > state.length {
                            copy = state.length;
                        }
                        if !copy_match_bytes(window, from_index, put_index, copy as usize) {
                            ret = crate::zlib_h::Z_STREAM_ERROR;
                            break '_inf_leave;
                        }
                        state.length = state.length.wrapping_sub(copy);
                        left = left.wrapping_sub(copy);
                        put = put_index + copy as usize;
                        if state.length == 0 as ::core::ffi::c_uint {
                            break;
                        }
                    }
                }
            }
        }
    }
    if left < state.wsize {
        if output.flush(&window[..state.wsize.wrapping_sub(left) as usize])
            && ret == crate::zlib_h::Z_STREAM_END
        {
            ret = crate::zlib_h::Z_BUF_ERROR;
        }
    }
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
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = strm.state as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let state = &mut *state;
    if state.window.is_none() || state.wsize == 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let initial_input = if strm.next_in.is_null() {
        None
    } else {
        Some(::core::slice::from_raw_parts(
            strm.next_in,
            strm.avail_in as usize,
        ))
    };
    let Some(mut window) = state.window.take() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if window.len() != state.wsize as usize {
        state.window = Some(window);
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut input = BackInput::new(
        move || {
            let Some(callback) = in_0 else {
                return None;
            };
            let mut next = ::core::ptr::null_mut();
            let have = callback(in_desc, &mut next);
            if have == 0 || next.is_null() {
                None
            } else {
                Some(::core::slice::from_raw_parts(next, have as usize))
            }
        },
        initial_input,
    );
    let mut output = BackOutput::new(move |bytes: &[u8]| {
        let Some(callback) = out else {
            return true;
        };
        callback(
            out_desc,
            bytes.as_ptr() as *mut ::core::ffi::c_uchar,
            bytes.len() as u32,
        ) != 0
    });
    let result = inflate_back_impl(strm, state, &mut window, &mut input, &mut output);
    state.window = Some(window);
    match input.remaining() {
        Some(remaining) => {
            strm.next_in = remaining.as_ptr() as *mut crate::stdlib::Bytef;
            strm.avail_in = remaining.len() as crate::stdlib::uInt;
        }
        None => {
            strm.next_in = ::core::ptr::null_mut();
            strm.avail_in = 0;
        }
    }
    result
}
pub unsafe extern "C" fn inflateBackEnd(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() || (*strm).state.is_null() || (*strm).zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    ::core::ptr::drop_in_place((*strm).state as *mut crate::src::inflate::inflate_state);
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
