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
fn initialize_inflate_back_state(
    state: &mut crate::src::inflate::inflate_state,
    window_bits: ::core::ffi::c_int,
) {
    state.dmax = 32768 as ::core::ffi::c_uint;
    state.wbits = window_bits as crate::stdlib::uInt as ::core::ffi::c_uint;
    state.wsize = (1 as ::core::ffi::c_uint) << window_bits;
    state.wnext = 0 as ::core::ffi::c_uint;
    state.whave = 0 as ::core::ffi::c_uint;
    state.sane = 1 as ::core::ffi::c_int;
}

/// Validate the one caller-owned window span established by
/// `inflateBackInit_`.  The FFI boundary constructs that span once; the
/// decoder shares it for both history and output instead of rebuilding it from
/// opaque state metadata.
fn inflate_back_window<'a>(
    state: &crate::src::inflate::inflate_state,
    window: &'a mut [u8],
    put: usize,
    length: usize,
) -> Option<&'a mut [u8]> {
    let window_bits = usize::try_from(state.wbits).ok()?;
    if !(8..=15).contains(&window_bits) {
        return None;
    }
    let window_len = 1usize.checked_shl(window_bits as u32)?;
    if usize::try_from(state.wsize).ok()? != window_len || window.len() != window_len {
        return None;
    }
    if put.checked_add(length)? > window_len {
        return None;
    }
    Some(window)
}

/// Copy a possibly overlapping match within the caller-owned inflateBack
/// window.  Byte order intentionally matches DEFLATE's forward expansion.
fn copy_inflate_back_match(
    window: &mut [u8],
    put: usize,
    distance: usize,
    length: usize,
) -> Option<()> {
    if distance == 0 || distance > window.len() {
        return None;
    }
    let end = put.checked_add(length)?;
    if end > window.len() {
        return None;
    }
    let from = if distance <= put {
        put.checked_sub(distance)?
    } else {
        put.checked_add(window.len().checked_sub(distance)?)?
    };
    if from.checked_add(length)? > window.len() {
        return None;
    }

    for index in 0..length {
        let byte = window[from + index];
        window[put + index] = byte;
    }
    Some(())
}

/// Refill the decoder's owned callback-input buffer.  The FFI wrapper adapts
/// the C callback to this synchronous closure, so the decoder never needs to
/// dereference the callback's transient buffer directly.
fn refill_inflate_back_input(
    input: &mut Vec<u8>,
    refill: &mut impl FnMut(&mut Vec<u8>) -> bool,
) -> bool {
    input.clear();
    if !refill(input) || input.is_empty() {
        return false;
    }
    true
}

/// Read a byte selected by the decoder's owned-input cursor.
fn inflate_back_input_byte(input: &[u8], next_index: usize) -> Option<u8> {
    input.get(next_index).copied()
}

fn inflate_back_input_slice(input: &[u8], next_index: usize, len: usize) -> Option<&[u8]> {
    let end = next_index.checked_add(len)?;
    input.get(next_index..end)
}

/// Allocate, initialize, and install the inflateBack state through the shared
/// typed callback-state facade.
///
/// The callback allocation itself remains contained by that facade. This
/// initialization only receives a live typed state and the checked
/// caller-owned window, leaving the FFI wrapper to marshal ABI inputs.
fn initialize_allocated_inflate_back_state(
    strm: &mut crate::zlib_h::z_stream,
    window_bits: ::core::ffi::c_int,
    window: &mut [::core::ffi::c_uchar],
    allocator_provenance: crate::src::zutil::AllocatorProvenance,
) -> ::core::ffi::c_int {
    crate::src::zutil::allocate_callback_owned_state(
        strm,
        crate::src::inflate::inflate_states(),
        crate::src::inflate::empty_inflate_state(),
        |_strm, state| {
            state.allocator_provenance = allocator_provenance;
            initialize_inflate_back_state(state, window_bits);
            crate::src::inflate::bind_inflate_back_window(state, window);
            crate::zlib_h::Z_OK
        },
    )
    .unwrap_or(crate::zlib_h::Z_MEM_ERROR)
}

struct InflateBackInit {
    allocator_provenance: crate::src::zutil::AllocatorProvenance,
}

/// Validate ABI-independent initialization inputs before the wrapper invokes
/// the caller's allocator callback.
fn prepare_inflate_back_init(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut windowBits: ::core::ffi::c_int,
    window: Option<&mut [::core::ffi::c_uchar]>,
    version: Option<&::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> Result<InflateBackInit, ::core::ffi::c_int> {
    if version.is_none()
        || *version.expect("checked non-null version") as ::core::ffi::c_int
            != crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return Err(crate::zlib_h::Z_VERSION_ERROR);
    }
    if strm.is_none()
        || window.is_none()
        || windowBits < 8 as ::core::ffi::c_int
        || windowBits > 15 as ::core::ffi::c_int
    {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    let strm = strm.expect("checked non-null stream");
    let window = window.expect("checked non-null window");
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let allocator_provenance = crate::src::zutil::install_default_allocators(strm);
    // The full window span was validated at the FFI boundary. It remains the
    // single output/history span installed after the callback allocation.
    let _ = window;
    Ok(InflateBackInit {
        allocator_provenance,
    })
}
#[export_name = "inflateBackInit_"]

pub unsafe extern "C" fn inflateBackInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let version = version.as_ref();
    let mut strm = strm.as_mut();
    let mut window = if windowBits >= 8 as ::core::ffi::c_int
        && windowBits <= 15 as ::core::ffi::c_int
        && !window.is_null()
    {
        Some(::core::slice::from_raw_parts_mut(
            window,
            (1usize) << windowBits,
        ))
    } else {
        None
    };
    let init = match prepare_inflate_back_init(
        strm.as_deref_mut(),
        windowBits,
        window.as_deref_mut(),
        version,
        stream_size,
    ) {
        Ok(init) => init,
        Err(error) => return error,
    };
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(window) = window else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    initialize_allocated_inflate_back_state(strm, windowBits, window, init.allocator_provenance)
}
pub fn inflateBack<F, R>(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [u8],
    mut input: Vec<u8>,
    mut refill: R,
    mut emit_window: F,
) -> ::core::ffi::c_int
where
    F: FnMut(&mut [u8], ::core::ffi::c_uint) -> bool,
    R: FnMut(&mut Vec<u8>) -> bool,
{
    // Callback input is copied into `input`, so an index fully represents the
    // decoder cursor and cannot outlive or escape that owned buffer.
    let mut next_index: usize = 0;
    // `window` is the one validated caller-owned history/output span.  Keep
    // its output cursor as an index so the decoder never advances a raw
    // pointer through that span.
    let mut put_index: usize = 0;
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
    const ORDER: [::core::ffi::c_ushort; 19] = [
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
    have = input.len() as ::core::ffi::c_uint;
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
                        if have == 0 as ::core::ffi::c_uint {
                            if !refill_inflate_back_input(&mut input, &mut refill) {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                            next_index = 0;
                            have = input.len() as ::core::ffi::c_uint;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh0 = next_index;
                        next_index = next_index.wrapping_add(1);
                        hold = hold.wrapping_add(
                            (inflate_back_input_byte(&input, c2rust_fresh0).unwrap_or(0)
                                as ::core::ffi::c_ulong)
                                << bits,
                        );
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
                            crate::src::inftrees::inflate_fixed(state);
                            state.mode = crate::src::inflate::LEN;
                        }
                        2 => {
                            state.mode = crate::src::inflate::TABLE;
                        }
                        _ => {
                            strm.msg = b"invalid block type\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
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
                        if !refill_inflate_back_input(&mut input, &mut refill) {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                        next_index = 0;
                        have = input.len() as ::core::ffi::c_uint;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh1 = next_index;
                    next_index = next_index.wrapping_add(1);
                    hold = hold.wrapping_add(
                        (inflate_back_input_byte(&input, c2rust_fresh1).unwrap_or(0)
                            as ::core::ffi::c_ulong)
                            << bits,
                    );
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if hold & 0xffff as ::core::ffi::c_ulong
                    != hold >> 16 as ::core::ffi::c_int ^ 0xffff as ::core::ffi::c_ulong
                {
                    strm.msg = b"invalid stored block lengths\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state.mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    state.length = hold as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint;
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    while state.length != 0 as ::core::ffi::c_uint {
                        copy = state.length;
                        if have == 0 as ::core::ffi::c_uint {
                            if !refill_inflate_back_input(&mut input, &mut refill) {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                            next_index = 0;
                            have = input.len() as ::core::ffi::c_uint;
                        }
                        if left == 0 as ::core::ffi::c_uint {
                            put_index = 0;
                            left = state.wsize;
                            state.whave = left;
                            if !emit_window(window, left) {
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
                        let copy_len = copy as usize;
                        let Some(destination) = put_index
                            .checked_add(copy_len)
                            .and_then(|end| window.get_mut(put_index..end))
                        else {
                            ret = crate::zlib_h::Z_STREAM_ERROR;
                            break '_inf_leave;
                        };
                        let Some(source) = inflate_back_input_slice(&input, next_index, copy_len)
                        else {
                            ret = crate::zlib_h::Z_STREAM_ERROR;
                            break '_inf_leave;
                        };
                        destination.copy_from_slice(source);
                        have = have.wrapping_sub(copy);
                        next_index = next_index.wrapping_add(copy_len);
                        left = left.wrapping_sub(copy);
                        put_index = put_index.wrapping_add(copy_len);
                        state.length = state.length.wrapping_sub(copy);
                    }
                    state.mode = crate::src::inflate::TYPE;
                    continue;
                }
            }
            16196 => {
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        if !refill_inflate_back_input(&mut input, &mut refill) {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                        next_index = 0;
                        have = input.len() as ::core::ffi::c_uint;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh2 = next_index;
                    next_index = next_index.wrapping_add(1);
                    hold = hold.wrapping_add(
                        (inflate_back_input_byte(&input, c2rust_fresh2).unwrap_or(0)
                            as ::core::ffi::c_ulong)
                            << bits,
                    );
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
                    strm.msg = b"too many length or distance symbols\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state.mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    state.have = 0 as ::core::ffi::c_uint;
                    while state.have < state.ncode {
                        while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                            if have == 0 as ::core::ffi::c_uint {
                                if !refill_inflate_back_input(&mut input, &mut refill) {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                                next_index = 0;
                                have = input.len() as ::core::ffi::c_uint;
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh3 = next_index;
                            next_index = next_index.wrapping_add(1);
                            hold = hold.wrapping_add(
                                (inflate_back_input_byte(&input, c2rust_fresh3).unwrap_or(0)
                                    as ::core::ffi::c_ulong)
                                    << bits,
                            );
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        let c2rust_fresh4 = state.have;
                        state.have = state.have.wrapping_add(1);
                        state.lens[ORDER[c2rust_fresh4 as usize] as usize] = (hold
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
                        state.lens[ORDER[c2rust_fresh5 as usize] as usize] =
                            0 as ::core::ffi::c_ushort;
                    }
                    state.next = 0;
                    state.lencode = crate::src::inflate::InflateTableRef::Dynamic(state.next);
                    state.lenbits = 7 as ::core::ffi::c_uint;
                    ret = match crate::src::inftrees::inflate_table(
                        crate::src::inftrees::CODES,
                        &(&state.lens)[..19],
                        &mut state.codes,
                        &mut state.lenbits,
                        &mut (&mut state.work)[..19],
                    ) {
                        Ok(used) => {
                            state.next = used;
                            0
                        }
                        Err(error) => error,
                    };
                    if ret != 0 {
                        strm.msg = b"invalid code lengths set\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.mode = crate::src::inflate::BAD;
                        continue;
                    } else {
                        state.have = 0 as ::core::ffi::c_uint;
                        while state.have < state.nlen.wrapping_add(state.ndist) {
                            loop {
                                let Some(table_entry) = crate::src::inffast::decode_table_entry(
                                    state,
                                    crate::src::inffast::DecodeTable::LiteralLength,
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint) << state.lenbits)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        as usize,
                                ) else {
                                    ret = crate::zlib_h::Z_STREAM_ERROR;
                                    break '_inf_leave;
                                };
                                here = table_entry;
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                if have == 0 as ::core::ffi::c_uint {
                                    if !refill_inflate_back_input(&mut input, &mut refill) {
                                        ret = crate::zlib_h::Z_BUF_ERROR;
                                        break '_inf_leave;
                                    }
                                    next_index = 0;
                                    have = input.len() as ::core::ffi::c_uint;
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh6 = next_index;
                                next_index = next_index.wrapping_add(1);
                                hold = hold.wrapping_add(
                                    (inflate_back_input_byte(&input, c2rust_fresh6).unwrap_or(0)
                                        as ::core::ffi::c_ulong)
                                        << bits,
                                );
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
                                            if !refill_inflate_back_input(&mut input, &mut refill) {
                                                ret = crate::zlib_h::Z_BUF_ERROR;
                                                break '_inf_leave;
                                            }
                                            next_index = 0;
                                            have = input.len() as ::core::ffi::c_uint;
                                        }
                                        have = have.wrapping_sub(1);
                                        let c2rust_fresh8 = next_index;
                                        next_index = next_index.wrapping_add(1);
                                        hold = hold.wrapping_add(
                                            (inflate_back_input_byte(&input, c2rust_fresh8)
                                                .unwrap_or(0)
                                                as ::core::ffi::c_ulong)
                                                << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    if state.have == 0 as ::core::ffi::c_uint {
                                        strm.msg = b"invalid bit length repeat\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char;
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
                                            if !refill_inflate_back_input(&mut input, &mut refill) {
                                                ret = crate::zlib_h::Z_BUF_ERROR;
                                                break '_inf_leave;
                                            }
                                            next_index = 0;
                                            have = input.len() as ::core::ffi::c_uint;
                                        }
                                        have = have.wrapping_sub(1);
                                        let c2rust_fresh9 = next_index;
                                        next_index = next_index.wrapping_add(1);
                                        hold = hold.wrapping_add(
                                            (inflate_back_input_byte(&input, c2rust_fresh9)
                                                .unwrap_or(0)
                                                as ::core::ffi::c_ulong)
                                                << bits,
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
                                            if !refill_inflate_back_input(&mut input, &mut refill) {
                                                ret = crate::zlib_h::Z_BUF_ERROR;
                                                break '_inf_leave;
                                            }
                                            next_index = 0;
                                            have = input.len() as ::core::ffi::c_uint;
                                        }
                                        have = have.wrapping_sub(1);
                                        let c2rust_fresh10 = next_index;
                                        next_index = next_index.wrapping_add(1);
                                        hold = hold.wrapping_add(
                                            (inflate_back_input_byte(&input, c2rust_fresh10)
                                                .unwrap_or(0)
                                                as ::core::ffi::c_ulong)
                                                << bits,
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
                                    strm.msg = b"invalid bit length repeat\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
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
                        if state.lens[256 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            strm.msg = b"invalid code -- missing end-of-block\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            state.mode = crate::src::inflate::BAD;
                            continue;
                        } else {
                            state.next = 0;
                            state.lencode =
                                crate::src::inflate::InflateTableRef::Dynamic(state.next);
                            state.lenbits = 9 as ::core::ffi::c_uint;
                            ret = match crate::src::inftrees::inflate_table(
                                crate::src::inftrees::LENS,
                                &(&state.lens)[..state.nlen as usize],
                                &mut state.codes,
                                &mut state.lenbits,
                                &mut (&mut state.work)[..state.nlen as usize],
                            ) {
                                Ok(used) => {
                                    state.next = used;
                                    0
                                }
                                Err(error) => error,
                            };
                            if ret != 0 {
                                strm.msg = b"invalid literal/lengths set\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                state.mode = crate::src::inflate::BAD;
                                continue;
                            } else {
                                state.distcode =
                                    crate::src::inflate::InflateTableRef::Dynamic(state.next);
                                state.distbits = 6 as ::core::ffi::c_uint;
                                let table_used = state.next;
                                ret = match crate::src::inftrees::inflate_table(
                                    crate::src::inftrees::DISTS,
                                    &(&state.lens)
                                        [state.nlen as usize..(state.nlen + state.ndist) as usize],
                                    &mut (&mut state.codes)[table_used..],
                                    &mut state.distbits,
                                    &mut (&mut state.work)[..state.ndist as usize],
                                ) {
                                    Ok(used) => {
                                        state.next = table_used + used;
                                        0
                                    }
                                    Err(error) => error,
                                };
                                if ret != 0 {
                                    strm.msg = b"invalid distances set\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
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
            let Some(output) = window.get_mut(put_index..) else {
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break '_inf_leave;
            };
            strm.next_out = output.as_mut_ptr() as *mut crate::stdlib::Bytef;
            strm.avail_out = left as crate::stdlib::uInt;
            strm.avail_in = have as crate::stdlib::uInt;
            state.hold = hold;
            state.bits = bits;
            let Some(fast_input) = inflate_back_input_slice(&input, next_index, have as usize)
            else {
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break '_inf_leave;
            };
            strm.next_in = fast_input.as_ptr() as *mut crate::stdlib::Bytef;
            crate::src::inffast::inflate_fast(
                strm,
                state,
                state.wsize,
                None,
                true,
                fast_input,
                window,
            );
            left = strm.avail_out as ::core::ffi::c_uint;
            let Some(produced) = usize::try_from(state.wsize.wrapping_sub(left)).ok() else {
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break '_inf_leave;
            };
            if produced > window.len() {
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break '_inf_leave;
            }
            put_index = produced;
            let remaining = strm.avail_in as ::core::ffi::c_uint;
            let Some(consumed) = have.checked_sub(remaining) else {
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break '_inf_leave;
            };
            let Some(updated_next_index) = next_index.checked_add(consumed as usize) else {
                ret = crate::zlib_h::Z_STREAM_ERROR;
                break '_inf_leave;
            };
            next_index = updated_next_index;
            have = remaining;
            hold = state.hold;
            bits = state.bits;
        } else {
            loop {
                let Some(table_entry) = crate::src::inffast::decode_table_entry(
                    state,
                    crate::src::inffast::DecodeTable::LiteralLength,
                    (hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << state.lenbits)
                            .wrapping_sub(1 as ::core::ffi::c_uint)) as usize,
                ) else {
                    ret = crate::zlib_h::Z_STREAM_ERROR;
                    break '_inf_leave;
                };
                here = table_entry;
                if here.bits as ::core::ffi::c_uint <= bits {
                    break;
                }
                if have == 0 as ::core::ffi::c_uint {
                    if !refill_inflate_back_input(&mut input, &mut refill) {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    }
                    next_index = 0;
                    have = input.len() as ::core::ffi::c_uint;
                }
                have = have.wrapping_sub(1);
                let c2rust_fresh13 = next_index;
                next_index = next_index.wrapping_add(1);
                hold = hold.wrapping_add(
                    (inflate_back_input_byte(&input, c2rust_fresh13).unwrap_or(0)
                        as ::core::ffi::c_ulong)
                        << bits,
                );
                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            }
            if here.op as ::core::ffi::c_int != 0
                && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                last = here;
                loop {
                    let Some(table_entry) = crate::src::inffast::decode_table_entry(
                        state,
                        crate::src::inffast::DecodeTable::LiteralLength,
                        (last.val as ::core::ffi::c_uint).wrapping_add(
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint)
                                    << last.bits as ::core::ffi::c_int
                                        + last.op as ::core::ffi::c_int)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                >> last.bits as ::core::ffi::c_int,
                        ) as usize,
                    ) else {
                        ret = crate::zlib_h::Z_STREAM_ERROR;
                        break '_inf_leave;
                    };
                    here = table_entry;
                    if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        <= bits
                    {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        if !refill_inflate_back_input(&mut input, &mut refill) {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                        next_index = 0;
                        have = input.len() as ::core::ffi::c_uint;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh14 = next_index;
                    next_index = next_index.wrapping_add(1);
                    hold = hold.wrapping_add(
                        (inflate_back_input_byte(&input, c2rust_fresh14).unwrap_or(0)
                            as ::core::ffi::c_ulong)
                            << bits,
                    );
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
                    put_index = 0;
                    left = state.wsize;
                    state.whave = left;
                    if !emit_window(window, left) {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break;
                    }
                }
                let Some(current_put) = state
                    .wsize
                    .checked_sub(left)
                    .and_then(|remaining| usize::try_from(remaining).ok())
                else {
                    ret = crate::zlib_h::Z_STREAM_ERROR;
                    break '_inf_leave;
                };
                let literal = state.length as ::core::ffi::c_uchar;
                let Some(put_end) = current_put.checked_add(1) else {
                    ret = crate::zlib_h::Z_STREAM_ERROR;
                    break '_inf_leave;
                };
                let Some(window) = inflate_back_window(state, window, current_put, 1) else {
                    ret = crate::zlib_h::Z_STREAM_ERROR;
                    break '_inf_leave;
                };
                let Some(destination) = window.get_mut(current_put..put_end) else {
                    ret = crate::zlib_h::Z_STREAM_ERROR;
                    break '_inf_leave;
                };
                destination.copy_from_slice(&[literal]);
                put_index = put_end;
                left = left.wrapping_sub(1);
                state.mode = crate::src::inflate::LEN;
            } else if here.op as ::core::ffi::c_int & 32 as ::core::ffi::c_int != 0 {
                state.mode = crate::src::inflate::TYPE;
            } else if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                strm.msg = b"invalid literal/length code\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                state.mode = crate::src::inflate::BAD;
            } else {
                state.extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                if state.extra != 0 as ::core::ffi::c_uint {
                    while bits < state.extra {
                        if have == 0 as ::core::ffi::c_uint {
                            if !refill_inflate_back_input(&mut input, &mut refill) {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                            next_index = 0;
                            have = input.len() as ::core::ffi::c_uint;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh16 = next_index;
                        next_index = next_index.wrapping_add(1);
                        hold = hold.wrapping_add(
                            (inflate_back_input_byte(&input, c2rust_fresh16).unwrap_or(0)
                                as ::core::ffi::c_ulong)
                                << bits,
                        );
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
                    let Some(table_entry) = crate::src::inffast::decode_table_entry(
                        state,
                        crate::src::inffast::DecodeTable::Distance,
                        (hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << state.distbits)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as usize,
                    ) else {
                        ret = crate::zlib_h::Z_STREAM_ERROR;
                        break '_inf_leave;
                    };
                    here = table_entry;
                    if here.bits as ::core::ffi::c_uint <= bits {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        if !refill_inflate_back_input(&mut input, &mut refill) {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                        next_index = 0;
                        have = input.len() as ::core::ffi::c_uint;
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh17 = next_index;
                    next_index = next_index.wrapping_add(1);
                    hold = hold.wrapping_add(
                        (inflate_back_input_byte(&input, c2rust_fresh17).unwrap_or(0)
                            as ::core::ffi::c_ulong)
                            << bits,
                    );
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    last = here;
                    loop {
                        let Some(table_entry) = crate::src::inffast::decode_table_entry(
                            state,
                            crate::src::inffast::DecodeTable::Distance,
                            (last.val as ::core::ffi::c_uint).wrapping_add(
                                (hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint)
                                        << last.bits as ::core::ffi::c_int
                                            + last.op as ::core::ffi::c_int)
                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                    >> last.bits as ::core::ffi::c_int,
                            ) as usize,
                        ) else {
                            ret = crate::zlib_h::Z_STREAM_ERROR;
                            break '_inf_leave;
                        };
                        here = table_entry;
                        if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            <= bits
                        {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            if !refill_inflate_back_input(&mut input, &mut refill) {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                            next_index = 0;
                            have = input.len() as ::core::ffi::c_uint;
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh18 = next_index;
                        next_index = next_index.wrapping_add(1);
                        hold = hold.wrapping_add(
                            (inflate_back_input_byte(&input, c2rust_fresh18).unwrap_or(0)
                                as ::core::ffi::c_ulong)
                                << bits,
                        );
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    hold >>= last.bits as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                }
                hold >>= here.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                    strm.msg = b"invalid distance code\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state.mode = crate::src::inflate::BAD;
                } else {
                    state.offset = here.val as ::core::ffi::c_uint;
                    state.extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                    if state.extra != 0 as ::core::ffi::c_uint {
                        while bits < state.extra {
                            if have == 0 as ::core::ffi::c_uint {
                                if !refill_inflate_back_input(&mut input, &mut refill) {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                                next_index = 0;
                                have = input.len() as ::core::ffi::c_uint;
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh19 = next_index;
                            next_index = next_index.wrapping_add(1);
                            hold = hold.wrapping_add(
                                (inflate_back_input_byte(&input, c2rust_fresh19).unwrap_or(0)
                                    as ::core::ffi::c_ulong)
                                    << bits,
                            );
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
                        strm.msg = b"invalid distance too far back\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        state.mode = crate::src::inflate::BAD;
                    } else {
                        loop {
                            if left == 0 as ::core::ffi::c_uint {
                                put_index = 0;
                                left = state.wsize;
                                state.whave = left;
                                if !emit_window(window, left) {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            copy = state.wsize.wrapping_sub(state.offset);
                            if copy < left {
                                copy = left.wrapping_sub(copy);
                            } else {
                                copy = left;
                            }
                            if copy > state.length {
                                copy = state.length;
                            }
                            let Some(remaining) = state.wsize.checked_sub(left) else {
                                ret = crate::zlib_h::Z_STREAM_ERROR;
                                break '_inf_leave;
                            };
                            let Some(current_put) = usize::try_from(remaining).ok() else {
                                ret = crate::zlib_h::Z_STREAM_ERROR;
                                break '_inf_leave;
                            };
                            let copy_len = copy as usize;
                            let distance = state.offset as usize;
                            let Some(window) =
                                inflate_back_window(state, window, current_put, copy_len)
                            else {
                                ret = crate::zlib_h::Z_STREAM_ERROR;
                                break '_inf_leave;
                            };
                            if copy_inflate_back_match(window, current_put, distance, copy_len)
                                .is_none()
                            {
                                ret = crate::zlib_h::Z_STREAM_ERROR;
                                break '_inf_leave;
                            }
                            state.length = state.length.wrapping_sub(copy);
                            left = left.wrapping_sub(copy);
                            put_index = current_put + copy_len;
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
        if !emit_window(window, state.wsize.wrapping_sub(left))
            && ret == crate::zlib_h::Z_STREAM_END
        {
            ret = crate::zlib_h::Z_BUF_ERROR;
        }
    }
    // The FFI wrapper restores the corresponding ABI pointer into the
    // callback's original buffer.  Only the remaining length crosses this
    // safe decoder boundary.
    strm.avail_in = have as crate::stdlib::uInt;
    return ret;
}
/// An ABI callback result after its buffer has been converted at the boundary.
enum InflateBackCallbackInput<'a> {
    Chunk(&'a [u8]),
    Exhausted,
    Invalid(usize),
}

/// The safe adapter's result, expressed as an offset into the ABI callback's
/// most recent input span.  The wrapper alone converts that offset back to a
/// raw cursor.
struct InflateBackCallbackResult {
    result: ::core::ffi::c_int,
    input_cursor: InflateBackInputCursor,
}

enum InflateBackInputCursor {
    Preserve,
    Null,
    Offset(usize),
}

/// Adapt already-validated callback chunks to the owned input buffer used by
/// the decoder.  This keeps buffering, refill sequencing, and restoration
/// decisions out of the ABI entry point.
fn inflate_back_with_callback_adapter<'a, F, R>(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [u8],
    initial_input: &[u8],
    mut source: F,
    emit_window: R,
) -> InflateBackCallbackResult
where
    F: FnMut() -> InflateBackCallbackInput<'a>,
    R: FnMut(&mut [u8], ::core::ffi::c_uint) -> bool,
{
    let window_bits = match usize::try_from(state.wbits) {
        Ok(bits) if (8..=15).contains(&bits) => bits,
        _ => {
            return InflateBackCallbackResult {
                result: crate::zlib_h::Z_STREAM_ERROR,
                input_cursor: InflateBackInputCursor::Preserve,
            };
        }
    };
    let Some(window_len) = 1usize.checked_shl(window_bits as u32) else {
        return InflateBackCallbackResult {
            result: crate::zlib_h::Z_STREAM_ERROR,
            input_cursor: InflateBackInputCursor::Preserve,
        };
    };
    if usize::try_from(state.wsize).ok() != Some(window_len) || window.len() != window_len {
        return InflateBackCallbackResult {
            result: crate::zlib_h::Z_STREAM_ERROR,
            input_cursor: InflateBackInputCursor::Preserve,
        };
    }
    let mut callback_exhausted = false;
    let mut current_input_len = initial_input.len();
    let input = initial_input.to_vec();
    let mut refill = |input: &mut Vec<u8>| {
        let chunk = match source() {
            InflateBackCallbackInput::Chunk(chunk) if !chunk.is_empty() => {
                current_input_len = chunk.len();
                chunk
            }
            InflateBackCallbackInput::Exhausted => {
                callback_exhausted = true;
                current_input_len = 0;
                return false;
            }
            InflateBackCallbackInput::Chunk(_) => {
                current_input_len = 0;
                return false;
            }
            InflateBackCallbackInput::Invalid(length) => {
                current_input_len = length;
                return false;
            }
        };
        input.extend_from_slice(chunk);
        true
    };
    let result = inflateBack(strm, state, window, input, &mut refill, emit_window);
    drop(refill);
    let input_cursor = if callback_exhausted {
        InflateBackInputCursor::Null
    } else if let Some(offset) = current_input_len.checked_sub(strm.avail_in as usize) {
        InflateBackInputCursor::Offset(offset)
    } else {
        return InflateBackCallbackResult {
            result: crate::zlib_h::Z_STREAM_ERROR,
            input_cursor: InflateBackInputCursor::Preserve,
        };
    };
    InflateBackCallbackResult { result, input_cursor }
}

#[export_name = "inflateBack"]

pub unsafe extern "C" fn inflateBack_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut in_0: crate::zlib_h::in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: crate::zlib_h::out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some((window_bits, window)) = crate::src::inflate::with_inflate_stream_state_ref(strm, |state| {
        (state.wbits, state.window.load(::core::sync::atomic::Ordering::Relaxed))
    }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let window_bits = match usize::try_from(window_bits) {
        Ok(bits) if (8..=15).contains(&bits) => bits,
        _ => return crate::zlib_h::Z_STREAM_ERROR,
    };
    let Some(window_len) = 1usize.checked_shl(window_bits as u32) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if window.is_null() || (strm.avail_in != 0 && strm.next_in.is_null()) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(in_0) = in_0 else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(out) = out else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let initial_input = if strm.avail_in == 0 {
        &[][..]
    } else {
        ::core::slice::from_raw_parts(strm.next_in, strm.avail_in as usize)
    };
    let window = ::core::slice::from_raw_parts_mut(window, window_len);
    let mut current_input = strm.next_in;
    let callback_result = crate::src::inflate::with_inflate_stream_state(strm, |strm, state| {
        inflate_back_with_callback_adapter(
            strm,
            state,
            window,
            initial_input,
            || {
                let mut callback_input = ::core::ptr::null_mut();
                let length = in_0(in_desc, &raw mut callback_input);
                current_input = callback_input;
                if length == 0 {
                    InflateBackCallbackInput::Exhausted
                } else if callback_input.is_null() {
                    InflateBackCallbackInput::Invalid(length as usize)
                } else {
                    InflateBackCallbackInput::Chunk(::core::slice::from_raw_parts(
                        callback_input,
                        length as usize,
                    ))
                }
            },
            |bytes: &mut [u8], length: ::core::ffi::c_uint| {
                out(out_desc, bytes.as_mut_ptr(), length) == 0
            },
        )
    });
    let Some(callback_result) = callback_result else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    match callback_result.input_cursor {
        InflateBackInputCursor::Preserve => {}
        InflateBackInputCursor::Null => strm.next_in = ::core::ptr::null_mut(),
        InflateBackInputCursor::Offset(offset) => strm.next_in = current_input.wrapping_add(offset),
    }
    callback_result.result
}
fn inflate_back_end(
    strm: &mut crate::zlib_h::z_stream,
    state: Option<&crate::src::inflate::inflate_state>,
) -> ::core::ffi::c_int {
    if state.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let Some(free) = strm.zfree else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // The state token is an opaque callback allocation.  Release it only
    // after the typed-state validation above; the FFI wrapper merely borrows
    // that typed state and dispatches here.
    free(strm.opaque, strm.state.cast());
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    crate::zlib_h::Z_OK
}

fn inflate_back_end_stream(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    let identity = strm.state.addr();
    let result = crate::src::inflate::with_inflate_stream_state(strm, |strm, state| {
        inflate_back_end(strm, Some(state))
    });
    if strm.state.is_null() {
        drop(crate::src::inflate::inflate_states().take(identity));
    }
    result.unwrap_or(crate::zlib_h::Z_STREAM_ERROR)
}
#[export_name = "inflateBackEnd"]

pub unsafe extern "C" fn inflateBackEnd_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let Some(strm) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate_back_end_stream(strm)
}
