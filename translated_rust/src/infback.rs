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

fn inflate_back_window_bits_are_valid(window_bits: ::core::ffi::c_int) -> bool {
    window_bits >= 8 as ::core::ffi::c_int && window_bits <= 15 as ::core::ffi::c_int
}

fn inflate_back_window_size(window_bits: ::core::ffi::c_int) -> Option<::core::ffi::c_uint> {
    if inflate_back_window_bits_are_valid(window_bits) {
        Some((1 as ::core::ffi::c_uint) << window_bits)
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InflateBackInitPlan {
    window_bits: ::core::ffi::c_int,
    window_size: ::core::ffi::c_uint,
    install_default_zalloc: bool,
    install_default_zfree: bool,
}

fn inflate_back_init_plan(
    window_bits: ::core::ffi::c_int,
    has_zalloc: bool,
    has_zfree: bool,
) -> Option<InflateBackInitPlan> {
    Some(InflateBackInitPlan {
        window_bits,
        window_size: inflate_back_window_size(window_bits)?,
        install_default_zalloc: !has_zalloc,
        install_default_zfree: !has_zfree,
    })
}

fn inflate_back_apply_allocator_defaults(
    stream: &mut crate::zlib_h::z_stream_s,
    plan: &InflateBackInitPlan,
) {
    stream.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if plan.install_default_zalloc {
        stream.zalloc = Some(crate::src::zutil::zcalloc_ffi);
        stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if plan.install_default_zfree {
        stream.zfree = Some(crate::src::zutil::zcfree_ffi);
    }
}

fn inflate_back_init_metadata_is_valid(
    version_first_byte: Option<::core::ffi::c_int>,
    stream_size: ::core::ffi::c_int,
) -> bool {
    version_first_byte == Some(crate::zlib_h::ZLIB_VERSION[0 as usize] as ::core::ffi::c_int)
        && stream_size == ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
}

fn inflate_back_stored_block_length(hold: ::core::ffi::c_ulong) -> Option<::core::ffi::c_uint> {
    let length = hold & 0xffff as ::core::ffi::c_ulong;
    if length == hold >> 16 as ::core::ffi::c_int ^ 0xffff as ::core::ffi::c_ulong {
        Some(length as ::core::ffi::c_uint)
    } else {
        None
    }
}

fn inflate_back_copy_count(
    requested: ::core::ffi::c_uint,
    available_input: ::core::ffi::c_uint,
    available_output: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    requested.min(available_input).min(available_output)
}

fn inflate_back_initial_input_count(
    input_is_present: bool,
    available_input: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    if input_is_present {
        available_input as ::core::ffi::c_uint
    } else {
        0
    }
}

fn inflate_back_finish_flush_status(
    ret: ::core::ffi::c_int,
    output_failed: bool,
) -> ::core::ffi::c_int {
    if output_failed && ret == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        ret
    }
}

fn inflate_back_consume_input_byte(
    have: ::core::ffi::c_uint,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    byte: ::core::ffi::c_uchar,
) -> (
    ::core::ffi::c_uint,
    ::core::ffi::c_ulong,
    ::core::ffi::c_uint,
) {
    (
        have.wrapping_sub(1),
        hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits),
        bits.wrapping_add(8),
    )
}

fn inflate_back_low_bits(
    hold: ::core::ffi::c_ulong,
    count: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << count).wrapping_sub(1 as ::core::ffi::c_uint)
}

fn inflate_back_root_table_index(
    hold: ::core::ffi::c_ulong,
    table_bits: ::core::ffi::c_uint,
) -> usize {
    inflate_back_low_bits(hold, table_bits) as usize
}

fn inflate_back_subtable_index(
    hold: ::core::ffi::c_ulong,
    last_val: ::core::ffi::c_ushort,
    last_bits: ::core::ffi::c_uchar,
    last_op: ::core::ffi::c_uchar,
) -> usize {
    (last_val as ::core::ffi::c_uint).wrapping_add(
        (hold as ::core::ffi::c_uint
            & ((1 as ::core::ffi::c_uint)
                << last_bits as ::core::ffi::c_int + last_op as ::core::ffi::c_int)
                .wrapping_sub(1 as ::core::ffi::c_uint))
            >> last_bits as ::core::ffi::c_int,
    ) as usize
}

fn inflate_back_take_bits(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
) -> (
    ::core::ffi::c_uint,
    ::core::ffi::c_ulong,
    ::core::ffi::c_uint,
) {
    let value = hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << count).wrapping_sub(1 as ::core::ffi::c_uint);
    (value, hold >> count, bits.wrapping_sub(count))
}

fn inflate_back_discard_bits(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
) -> (::core::ffi::c_ulong, ::core::ffi::c_uint) {
    (hold >> count, bits.wrapping_sub(count))
}

fn inflate_back_fill_code_length_run(
    lens: &mut [::core::ffi::c_ushort],
    start: usize,
    count: usize,
    length: ::core::ffi::c_ushort,
) -> usize {
    let end = start + count;
    lens[start..end].fill(length);
    end
}

fn inflate_back_align_to_byte_boundary(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> (::core::ffi::c_ulong, ::core::ffi::c_uint) {
    let discarded_bits = bits & 7 as ::core::ffi::c_uint;
    (hold >> discarded_bits, bits.wrapping_sub(discarded_bits))
}

fn inflate_back_distance_exceeds_window(
    offset: ::core::ffi::c_uint,
    window_size: ::core::ffi::c_uint,
    window_have: ::core::ffi::c_uint,
    window_left: ::core::ffi::c_uint,
) -> bool {
    offset
        > window_size.wrapping_sub(if window_have < window_size {
            window_left
        } else {
            0
        })
}

#[derive(Debug, PartialEq, Eq)]
enum InflateBackBlockKind {
    Stored,
    Fixed,
    Dynamic,
    Invalid,
}

struct InflateBackBlockHeader {
    last: ::core::ffi::c_int,
    kind: InflateBackBlockKind,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
}

#[derive(Debug, PartialEq, Eq)]
enum InflateBackMatchSource {
    Ahead(usize),
    Behind(usize),
}

#[derive(Debug, Eq, PartialEq)]
enum InflateBackLitLenAction {
    Literal,
    End,
    Invalid,
    Length { extra_bits: ::core::ffi::c_uchar },
}

fn inflate_back_litlen_action(op: ::core::ffi::c_uchar) -> InflateBackLitLenAction {
    if op == 0 {
        InflateBackLitLenAction::Literal
    } else if op & 32 != 0 {
        InflateBackLitLenAction::End
    } else if op & 64 != 0 {
        InflateBackLitLenAction::Invalid
    } else {
        InflateBackLitLenAction::Length {
            extra_bits: op & 15,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum InflateBackCodeLengthRepeat {
    Previous {
        extra_bits: ::core::ffi::c_uint,
    },
    Zero {
        extra_bits: ::core::ffi::c_uint,
        base: ::core::ffi::c_uint,
    },
}

fn inflate_back_code_length_repeat(symbol: ::core::ffi::c_ushort) -> InflateBackCodeLengthRepeat {
    match symbol {
        16 => InflateBackCodeLengthRepeat::Previous { extra_bits: 2 },
        17 => InflateBackCodeLengthRepeat::Zero {
            extra_bits: 3,
            base: 3,
        },
        _ => InflateBackCodeLengthRepeat::Zero {
            extra_bits: 7,
            base: 11,
        },
    }
}

fn inflate_back_code_length_repeat_extra_bits(
    repeat: &InflateBackCodeLengthRepeat,
) -> ::core::ffi::c_uint {
    match repeat {
        InflateBackCodeLengthRepeat::Previous { extra_bits }
        | InflateBackCodeLengthRepeat::Zero { extra_bits, .. } => *extra_bits,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InflateBackCodeLengthRepeatPlan {
    length: ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
}

fn inflate_back_code_length_repeat_plan(
    repeat: &InflateBackCodeLengthRepeat,
    previous_length: Option<::core::ffi::c_ushort>,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> Option<InflateBackCodeLengthRepeatPlan> {
    let (length, base, extra_bits) = match repeat {
        InflateBackCodeLengthRepeat::Previous { extra_bits } => {
            (previous_length? as ::core::ffi::c_uint, 3, *extra_bits)
        }
        InflateBackCodeLengthRepeat::Zero { extra_bits, base } => (0, *base, *extra_bits),
    };
    let (extra, hold, bits) = inflate_back_take_bits(hold, bits, extra_bits);

    Some(InflateBackCodeLengthRepeatPlan {
        length,
        count: base.wrapping_add(extra),
        hold,
        bits,
    })
}

fn inflate_back_match_copy_plan(
    window_size: ::core::ffi::c_uint,
    offset: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    length: ::core::ffi::c_uint,
) -> (InflateBackMatchSource, ::core::ffi::c_uint) {
    let copy = window_size.wrapping_sub(offset);
    if copy < left {
        (
            InflateBackMatchSource::Ahead(copy as usize),
            left.wrapping_sub(copy).min(length),
        )
    } else {
        (
            InflateBackMatchSource::Behind(offset as usize),
            left.min(length),
        )
    }
}

fn inflate_back_copy_match(
    window: &mut [::core::ffi::c_uchar],
    destination: usize,
    source: InflateBackMatchSource,
    count: ::core::ffi::c_uint,
) -> Option<()> {
    let count = usize::try_from(count).ok()?;
    let source = match source {
        InflateBackMatchSource::Ahead(distance) => destination.checked_add(distance)?,
        InflateBackMatchSource::Behind(distance) => destination.checked_sub(distance)?,
    };
    destination
        .checked_add(count)?
        .le(&window.len())
        .then_some(())?;
    source.checked_add(count)?.le(&window.len()).then_some(())?;

    for offset in 0..count {
        let byte = *window.get(source.checked_add(offset)?)?;
        *window.get_mut(destination.checked_add(offset)?)? = byte;
    }
    Some(())
}

fn inflate_back_block_header(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> InflateBackBlockHeader {
    let kind = match (hold >> 1) & 3 {
        0 => InflateBackBlockKind::Stored,
        1 => InflateBackBlockKind::Fixed,
        2 => InflateBackBlockKind::Dynamic,
        _ => InflateBackBlockKind::Invalid,
    };

    InflateBackBlockHeader {
        last: (hold & 1) as ::core::ffi::c_int,
        kind,
        hold: hold >> 3,
        bits: bits.wrapping_sub(3),
    }
}

fn inflate_back_initialize_state(
    stream: &mut crate::zlib_h::z_stream_s,
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [::core::ffi::c_uchar],
    window_bits: ::core::ffi::c_int,
) {
    stream.state = state as *mut crate::src::inflate::inflate_state as *mut ::core::ffi::c_void;
    state.dmax = 32768;
    state.wbits = window_bits as crate::stdlib::uInt as ::core::ffi::c_uint;
    state.wsize = window.len() as ::core::ffi::c_uint;
    state.window = window.as_mut_ptr();
    state.window_ownership = crate::src::inflate::WindowOwnership::CallerBorrowed.raw();
    state.wnext = 0;
    state.whave = 0;
    state.sane = 1;
}
#[export_name = "inflateBackInit_"]

pub unsafe extern "C" fn inflateBackInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let version_first_byte = if version.is_null() {
        None
    } else {
        Some(*version as ::core::ffi::c_int)
    };
    if !inflate_back_init_metadata_is_valid(version_first_byte, stream_size) {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if strm.is_null() || window.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let plan = match inflate_back_init_plan(
        windowBits,
        (*strm).zalloc.is_some(),
        (*strm).zfree.is_some(),
    ) {
        Some(plan) => plan,
        None => return crate::zlib_h::Z_STREAM_ERROR,
    };

    let stream = &mut *strm;
    inflate_back_apply_allocator_defaults(stream, &plan);
    let state = Some(stream.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        stream.opaque,
        1,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    // The allocation callback supplies uninitialized storage.  Initialize the
    // complete opaque state before converting it to a mutable reference.
    core::ptr::write(state, crate::src::inflate::inflate_state::newly_allocated());
    let state = &mut *state;
    let window = core::slice::from_raw_parts_mut(window, plan.window_size as usize);
    inflate_back_initialize_state(stream, state, window, plan.window_bits);
    crate::zlib_h::Z_OK
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
    const ORDER: [::core::ffi::c_ushort; 19] = [
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    ];
    if strm.is_null() || (*strm).state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut crate::src::inflate::inflate_state;
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).mode = crate::src::inflate::TYPE;
    (*state).last = 0 as ::core::ffi::c_int;
    (*state).whave = 0 as ::core::ffi::c_uint;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    have = inflate_back_initial_input_count(!next.is_null(), (*strm).avail_in);
    hold = 0 as ::core::ffi::c_ulong;
    bits = 0 as ::core::ffi::c_uint;
    put = (*state).window;
    left = (*state).wsize;
    's_69: loop {
        match (*state).mode as ::core::ffi::c_uint {
            16191 => {
                if (*state).last != 0 {
                    (hold, bits) = inflate_back_align_to_byte_boundary(hold, bits);
                    (*state).mode = crate::src::inflate::DONE;
                    continue;
                } else {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        let input_byte = *next;
                        next = next.wrapping_add(1);
                        (have, hold, bits) =
                            inflate_back_consume_input_byte(have, hold, bits, input_byte);
                    }
                    let header = inflate_back_block_header(hold, bits);
                    (*state).last = header.last;
                    hold = header.hold;
                    bits = header.bits;
                    match header.kind {
                        InflateBackBlockKind::Stored => {
                            (*state).mode = crate::src::inflate::STORED;
                        }
                        InflateBackBlockKind::Fixed => {
                            crate::src::inftrees::inflate_fixed(&mut *state);
                            (*state).mode = crate::src::inflate::LEN;
                        }
                        InflateBackBlockKind::Dynamic => {
                            (*state).mode = crate::src::inflate::TABLE;
                        }
                        InflateBackBlockKind::Invalid => {
                            (*strm).msg = b"invalid block type\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                        }
                    }
                    continue;
                }
            }
            16193 => {
                (hold, bits) = inflate_back_align_to_byte_boundary(hold, bits);
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    let input_byte = *next;
                    next = next.wrapping_add(1);
                    (have, hold, bits) =
                        inflate_back_consume_input_byte(have, hold, bits, input_byte);
                }
                let length = match inflate_back_stored_block_length(hold) {
                    Some(length) => length,
                    None => {
                        (*strm).msg = b"invalid stored block lengths\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                        continue;
                    }
                };
                (*state).length = length;
                hold = 0 as ::core::ffi::c_ulong;
                bits = 0 as ::core::ffi::c_uint;
                while (*state).length != 0 as ::core::ffi::c_uint {
                    copy = (*state).length;
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    if left == 0 as ::core::ffi::c_uint {
                        put = (*state).window;
                        left = (*state).wsize;
                        (*state).whave = left;
                        if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    copy = inflate_back_copy_count(copy, have, left);
                    crate::stdlib::memcpy(
                        put as *mut ::core::ffi::c_void,
                        next as *const ::core::ffi::c_void,
                        copy as crate::__stddef_size_t_h::size_t,
                    );
                    have = have.wrapping_sub(copy);
                    next = next.wrapping_add(copy as usize);
                    left = left.wrapping_sub(copy);
                    put = put.wrapping_add(copy as usize);
                    (*state).length = (*state).length.wrapping_sub(copy);
                }
                (*state).mode = crate::src::inflate::TYPE;
                continue;
            }
            16196 => {
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    let input_byte = *next;
                    next = next.wrapping_add(1);
                    (have, hold, bits) =
                        inflate_back_consume_input_byte(have, hold, bits, input_byte);
                }
                let counts =
                    crate::src::inflate::dynamic_header_counts(hold as ::core::ffi::c_uint);
                (*state).nlen = counts.nlen;
                (*state).ndist = counts.ndist;
                (*state).ncode = counts.ncode;
                (hold, bits) = inflate_back_discard_bits(hold, bits, 14);
                if !counts.is_valid() {
                    (*strm).msg = b"too many length or distance symbols\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                    continue;
                } else {
                    (*state).have = 0 as ::core::ffi::c_uint;
                    while (*state).have < (*state).ncode {
                        while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                            if have == 0 as ::core::ffi::c_uint {
                                have = in_0.expect("non-null function pointer")(
                                    in_desc,
                                    &raw mut next,
                                );
                                if have == 0 as ::core::ffi::c_uint {
                                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break 's_69;
                                }
                            }
                            let input_byte = *next;
                            next = next.wrapping_add(1);
                            (have, hold, bits) =
                                inflate_back_consume_input_byte(have, hold, bits, input_byte);
                        }
                        let c2rust_fresh4 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[ORDER[c2rust_fresh4 as usize] as usize] =
                            inflate_back_low_bits(hold, 3) as ::core::ffi::c_ushort;
                        (hold, bits) = inflate_back_discard_bits(hold, bits, 3);
                    }
                    while (*state).have < 19 as ::core::ffi::c_uint {
                        let c2rust_fresh5 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[ORDER[c2rust_fresh5 as usize] as usize] =
                            0 as ::core::ffi::c_ushort;
                    }
                    let state = &mut *state;
                    let mut table_cursor = 0usize;
                    state.next = 0;
                    state.lencode = crate::src::inflate::DecodeTableLocation::dynamic(0);
                    state.lenbits = 7 as ::core::ffi::c_uint;
                    ret = crate::src::inftrees::inflate_table_safe(
                        crate::src::inftrees::CODES,
                        &state.lens[..19],
                        &mut state.codes,
                        &mut table_cursor,
                        &mut state.lenbits,
                        &mut state.work[..19],
                    );
                    if ret == 0 {
                        state.next = table_cursor;
                    }
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
                                here = crate::src::inflate::inflate_decode_table_entry(
                                    &*state,
                                    (*state).lencode,
                                    inflate_back_root_table_index(hold, (*state).lenbits),
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
                                        break 's_69;
                                    }
                                }
                                let input_byte = *next;
                                next = next.wrapping_add(1);
                                (have, hold, bits) =
                                    inflate_back_consume_input_byte(have, hold, bits, input_byte);
                            }
                            if (here.val as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                                (hold, bits) = inflate_back_discard_bits(
                                    hold,
                                    bits,
                                    here.bits as ::core::ffi::c_uint,
                                );
                                let c2rust_fresh7 = (*state).have;
                                (*state).have = (*state).have.wrapping_add(1);
                                (*state).lens[c2rust_fresh7 as usize] = here.val;
                            } else {
                                let repeat = inflate_back_code_length_repeat(here.val);
                                let extra_bits =
                                    inflate_back_code_length_repeat_extra_bits(&repeat);
                                while bits < here.bits as ::core::ffi::c_uint + extra_bits {
                                    if have == 0 as ::core::ffi::c_uint {
                                        have = in_0.expect("non-null function pointer")(
                                            in_desc,
                                            &raw mut next,
                                        );
                                        if have == 0 as ::core::ffi::c_uint {
                                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break 's_69;
                                        }
                                    }
                                    let input_byte = *next;
                                    next = next.wrapping_add(1);
                                    (have, hold, bits) = inflate_back_consume_input_byte(
                                        have, hold, bits, input_byte,
                                    );
                                }
                                (hold, bits) = inflate_back_discard_bits(
                                    hold,
                                    bits,
                                    here.bits as ::core::ffi::c_uint,
                                );
                                let previous_length = if (*state).have == 0 {
                                    None
                                } else {
                                    Some((*state).lens[(*state).have.wrapping_sub(1) as usize])
                                };
                                let Some(plan) = inflate_back_code_length_repeat_plan(
                                    &repeat,
                                    previous_length,
                                    hold,
                                    bits,
                                ) else {
                                    (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    (*state).mode = crate::src::inflate::BAD;
                                    break;
                                };
                                len = plan.length;
                                copy = plan.count;
                                hold = plan.hold;
                                bits = plan.bits;
                                if (*state).have.wrapping_add(copy)
                                    > (*state).nlen.wrapping_add((*state).ndist)
                                {
                                    (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    (*state).mode = crate::src::inflate::BAD;
                                    break;
                                } else {
                                    let state = &mut *state;
                                    state.have = inflate_back_fill_code_length_run(
                                        &mut state.lens,
                                        state.have as usize,
                                        copy as usize,
                                        len as ::core::ffi::c_ushort,
                                    )
                                        as ::core::ffi::c_uint;
                                }
                            }
                        }
                        if (*state).mode as ::core::ffi::c_uint
                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        if (*state).lens[256 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*strm).msg = b"invalid code -- missing end-of-block\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                            continue;
                        } else {
                            let state = &mut *state;
                            let mut table_cursor = 0usize;
                            state.next = 0;
                            state.lencode = crate::src::inflate::DecodeTableLocation::dynamic(0);
                            state.lenbits = 9 as ::core::ffi::c_uint;
                            ret = crate::src::inftrees::inflate_table_safe(
                                crate::src::inftrees::LENS,
                                &state.lens[..state.nlen as usize],
                                &mut state.codes,
                                &mut table_cursor,
                                &mut state.lenbits,
                                &mut state.work[..state.nlen as usize],
                            );
                            if ret == 0 {
                                state.next = table_cursor;
                            }
                            if ret != 0 {
                                (*strm).msg = b"invalid literal/lengths set\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = crate::src::inflate::BAD;
                                continue;
                            } else {
                                state.distcode =
                                    crate::src::inflate::DecodeTableLocation::dynamic(state.next);
                                state.distbits = 6 as ::core::ffi::c_uint;
                                ret = crate::src::inftrees::inflate_table_safe(
                                    crate::src::inftrees::DISTS,
                                    &state.lens[state.nlen as usize
                                        ..state.nlen.wrapping_add(state.ndist) as usize],
                                    &mut state.codes,
                                    &mut table_cursor,
                                    &mut state.distbits,
                                    &mut state.work[..state.ndist as usize],
                                );
                                if ret == 0 {
                                    state.next = table_cursor;
                                }
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
            (*strm).next_out = put as *mut crate::stdlib::Bytef;
            (*strm).avail_out = left as crate::stdlib::uInt;
            (*strm).next_in = next as *mut crate::stdlib::Bytef;
            (*strm).avail_in = have as crate::stdlib::uInt;
            (*state).hold = hold;
            (*state).bits = bits;
            crate::src::inffast::inflate_fast(
                strm as *mut crate::zlib_h::z_stream_s,
                (*state).wsize,
            );
            put = (*strm).next_out as *mut ::core::ffi::c_uchar;
            left = (*strm).avail_out as ::core::ffi::c_uint;
            next = (*strm).next_in as *mut ::core::ffi::c_uchar;
            have = (*strm).avail_in as ::core::ffi::c_uint;
            hold = (*state).hold;
            bits = (*state).bits;
        } else {
            loop {
                here = crate::src::inflate::inflate_decode_table_entry(
                    &*state,
                    (*state).lencode,
                    inflate_back_root_table_index(hold, (*state).lenbits),
                );
                if here.bits as ::core::ffi::c_uint <= bits {
                    break;
                }
                if have == 0 as ::core::ffi::c_uint {
                    have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                    if have == 0 as ::core::ffi::c_uint {
                        next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break 's_69;
                    }
                }
                let input_byte = *next;
                next = next.wrapping_add(1);
                (have, hold, bits) = inflate_back_consume_input_byte(have, hold, bits, input_byte);
            }
            if here.op as ::core::ffi::c_int != 0
                && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                last = here;
                loop {
                    here = crate::src::inflate::inflate_decode_table_entry(
                        &*state,
                        (*state).lencode,
                        inflate_back_subtable_index(hold, last.val, last.bits, last.op),
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
                            break 's_69;
                        }
                    }
                    let input_byte = *next;
                    next = next.wrapping_add(1);
                    (have, hold, bits) =
                        inflate_back_consume_input_byte(have, hold, bits, input_byte);
                }
                (hold, bits) =
                    inflate_back_discard_bits(hold, bits, last.bits as ::core::ffi::c_uint);
            }
            (hold, bits) = inflate_back_discard_bits(hold, bits, here.bits as ::core::ffi::c_uint);
            (*state).length = here.val as ::core::ffi::c_uint;
            match inflate_back_litlen_action(here.op) {
                InflateBackLitLenAction::Literal => {
                    if left == 0 as ::core::ffi::c_uint {
                        put = (*state).window;
                        left = (*state).wsize;
                        (*state).whave = left;
                        if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break;
                        }
                    }
                    let c2rust_fresh15 = put;
                    put = put.wrapping_add(1);
                    *c2rust_fresh15 = (*state).length as ::core::ffi::c_uchar;
                    left = left.wrapping_sub(1);
                    (*state).mode = crate::src::inflate::LEN;
                }
                InflateBackLitLenAction::End => {
                    (*state).mode = crate::src::inflate::TYPE;
                }
                InflateBackLitLenAction::Invalid => {
                    (*strm).msg = b"invalid literal/length code\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = crate::src::inflate::BAD;
                }
                InflateBackLitLenAction::Length { extra_bits } => {
                    (*state).extra = extra_bits as ::core::ffi::c_uint;
                    if (*state).extra != 0 as ::core::ffi::c_uint {
                        while bits < (*state).extra {
                            if have == 0 as ::core::ffi::c_uint {
                                have = in_0.expect("non-null function pointer")(
                                    in_desc,
                                    &raw mut next,
                                );
                                if have == 0 as ::core::ffi::c_uint {
                                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break 's_69;
                                }
                            }
                            let input_byte = *next;
                            next = next.wrapping_add(1);
                            (have, hold, bits) =
                                inflate_back_consume_input_byte(have, hold, bits, input_byte);
                        }
                        let (extra, remaining_hold, remaining_bits) =
                            inflate_back_take_bits(hold, bits, (*state).extra);
                        (*state).length = (*state).length.wrapping_add(extra);
                        hold = remaining_hold;
                        bits = remaining_bits;
                    }
                    loop {
                        here = crate::src::inflate::inflate_decode_table_entry(
                            &*state,
                            (*state).distcode,
                            inflate_back_root_table_index(hold, (*state).distbits),
                        );
                        if here.bits as ::core::ffi::c_uint <= bits {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        let input_byte = *next;
                        next = next.wrapping_add(1);
                        (have, hold, bits) =
                            inflate_back_consume_input_byte(have, hold, bits, input_byte);
                    }
                    if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        last = here;
                        loop {
                            here = crate::src::inflate::inflate_decode_table_entry(
                                &*state,
                                (*state).distcode,
                                inflate_back_subtable_index(hold, last.val, last.bits, last.op),
                            );
                            if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                <= bits
                            {
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
                                    break 's_69;
                                }
                            }
                            let input_byte = *next;
                            next = next.wrapping_add(1);
                            (have, hold, bits) =
                                inflate_back_consume_input_byte(have, hold, bits, input_byte);
                        }
                        (hold, bits) =
                            inflate_back_discard_bits(hold, bits, last.bits as ::core::ffi::c_uint);
                    }
                    (hold, bits) =
                        inflate_back_discard_bits(hold, bits, here.bits as ::core::ffi::c_uint);
                    if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                        (*strm).msg = b"invalid distance code\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = crate::src::inflate::BAD;
                    } else {
                        (*state).offset = here.val as ::core::ffi::c_uint;
                        (*state).extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                        if (*state).extra != 0 as ::core::ffi::c_uint {
                            while bits < (*state).extra {
                                if have == 0 as ::core::ffi::c_uint {
                                    have = in_0.expect("non-null function pointer")(
                                        in_desc,
                                        &raw mut next,
                                    );
                                    if have == 0 as ::core::ffi::c_uint {
                                        next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                        ret = crate::zlib_h::Z_BUF_ERROR;
                                        break 's_69;
                                    }
                                }
                                let input_byte = *next;
                                next = next.wrapping_add(1);
                                (have, hold, bits) =
                                    inflate_back_consume_input_byte(have, hold, bits, input_byte);
                            }
                            let (extra, remaining_hold, remaining_bits) =
                                inflate_back_take_bits(hold, bits, (*state).extra);
                            (*state).offset = (*state).offset.wrapping_add(extra);
                            hold = remaining_hold;
                            bits = remaining_bits;
                        }
                        if inflate_back_distance_exceeds_window(
                            (*state).offset,
                            (*state).wsize,
                            (*state).whave,
                            left,
                        ) {
                            (*strm).msg = b"invalid distance too far back\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = crate::src::inflate::BAD;
                        } else {
                            loop {
                                if left == 0 as ::core::ffi::c_uint {
                                    put = (*state).window;
                                    left = (*state).wsize;
                                    (*state).whave = left;
                                    if out.expect("non-null function pointer")(out_desc, put, left)
                                        != 0
                                    {
                                        ret = crate::zlib_h::Z_BUF_ERROR;
                                        break 's_69;
                                    }
                                }
                                let state = &mut *state;
                                let (source, planned_copy) = inflate_back_match_copy_plan(
                                    state.wsize,
                                    state.offset,
                                    left,
                                    state.length,
                                );
                                let destination = state.wsize as usize - left as usize;
                                let window = &mut *core::ptr::slice_from_raw_parts_mut(
                                    state.window,
                                    state.wsize as usize,
                                );
                                if inflate_back_copy_match(
                                    window,
                                    destination,
                                    source,
                                    planned_copy,
                                )
                                .is_none()
                                {
                                    state.mode = crate::src::inflate::BAD;
                                    break;
                                }
                                put = put.wrapping_add(planned_copy as usize);
                                state.length = state.length.wrapping_sub(planned_copy);
                                left = left.wrapping_sub(planned_copy);
                                if state.length == 0 as ::core::ffi::c_uint {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if left < (*state).wsize {
        let output_failed = out.expect("non-null function pointer")(
            out_desc,
            (*state).window,
            (*state).wsize.wrapping_sub(left),
        ) != 0;
        ret = inflate_back_finish_flush_status(ret, output_failed);
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
    if strm.is_null() || (*strm).state.is_null() || (*strm).zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as crate::stdlib::voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<::core::ffi::c_void>();
    crate::zlib_h::Z_OK
}

#[cfg(test)]
mod tests {
    use super::{
        inflate_back_align_to_byte_boundary, inflate_back_block_header,
        inflate_back_code_length_repeat, inflate_back_code_length_repeat_extra_bits,
        inflate_back_code_length_repeat_plan, inflate_back_consume_input_byte,
        inflate_back_copy_count, inflate_back_copy_match, inflate_back_discard_bits,
        inflate_back_distance_exceeds_window, inflate_back_fill_code_length_run,
        inflate_back_finish_flush_status, inflate_back_init_metadata_is_valid,
        inflate_back_initialize_state,
        inflate_back_init_plan, inflate_back_initial_input_count, inflate_back_litlen_action,
        inflate_back_low_bits, inflate_back_match_copy_plan, inflate_back_root_table_index,
        inflate_back_stored_block_length, inflate_back_subtable_index, inflate_back_take_bits,
        inflate_back_window_bits_are_valid, inflate_back_window_size, InflateBackBlockKind,
        InflateBackCodeLengthRepeat, InflateBackCodeLengthRepeatPlan, InflateBackInitPlan,
        InflateBackLitLenAction, InflateBackMatchSource,
    };

    #[test]
    fn inflate_back_litlen_action_preserves_opcode_precedence() {
        assert_eq!(
            inflate_back_litlen_action(0),
            InflateBackLitLenAction::Literal
        );
        assert_eq!(
            inflate_back_litlen_action(0x10),
            InflateBackLitLenAction::Length { extra_bits: 0 }
        );
        assert_eq!(
            inflate_back_litlen_action(0x1f),
            InflateBackLitLenAction::Length { extra_bits: 15 }
        );
        assert_eq!(
            inflate_back_litlen_action(0x20),
            InflateBackLitLenAction::End
        );
        assert_eq!(
            inflate_back_litlen_action(0x40),
            InflateBackLitLenAction::Invalid
        );
        assert_eq!(
            inflate_back_litlen_action(0x60),
            InflateBackLitLenAction::End
        );
    }

    #[test]
    fn inflate_back_consume_input_byte_updates_bit_buffer_and_availability() {
        assert_eq!(
            inflate_back_consume_input_byte(3, 0x12, 8, 0x34),
            (2, 0x3412, 16),
        );
        assert_eq!(
            inflate_back_consume_input_byte(0, 0, 60, 0xff),
            (::core::ffi::c_uint::MAX, 0xf000_0000_0000_0000, 68),
        );
    }

    #[test]
    fn inflate_back_take_bits_returns_low_bits_and_advances_buffer() {
        assert_eq!(inflate_back_take_bits(0b101101, 6, 3), (0b101, 0b101, 3));
        assert_eq!(inflate_back_take_bits(0x1234, 16, 4), (0x4, 0x123, 12));
    }

    #[test]
    fn inflate_back_discard_bits_advances_the_bit_buffer() {
        assert_eq!(inflate_back_discard_bits(0b101101, 6, 3), (0b101, 3));
        assert_eq!(inflate_back_discard_bits(0b101101, 6, 0), (0b101101, 6));
    }

    #[test]
    fn inflate_back_fill_code_length_run_updates_only_requested_entries() {
        let mut lens = [1, 2, 3, 4, 5, 6];

        assert_eq!(inflate_back_fill_code_length_run(&mut lens, 2, 3, 9), 5);
        assert_eq!(lens, [1, 2, 9, 9, 9, 6]);
    }

    #[test]
    fn inflate_back_low_bits_masks_only_requested_bits() {
        assert_eq!(inflate_back_low_bits(0x1234, 0), 0);
        assert_eq!(inflate_back_low_bits(0b101101, 3), 0b101);
        assert_eq!(inflate_back_low_bits(0x1234, 12), 0x234);
    }

    #[test]
    fn inflate_back_byte_alignment_discards_only_partial_bytes() {
        assert_eq!(inflate_back_align_to_byte_boundary(0x1234, 8), (0x1234, 8));
        assert_eq!(inflate_back_align_to_byte_boundary(0, 0), (0, 0));
        assert_eq!(
            inflate_back_align_to_byte_boundary(0b101_101, 11),
            (0b101, 8)
        );
        assert_eq!(inflate_back_align_to_byte_boundary(0xfe, 7), (1, 0));
    }

    #[test]
    fn inflate_back_window_bits_validation_accepts_only_supported_range() {
        for window_bits in 8..=15 {
            assert!(inflate_back_window_bits_are_valid(window_bits));
        }
        assert!(!inflate_back_window_bits_are_valid(7));
        assert!(!inflate_back_window_bits_are_valid(16));
    }

    #[test]
    fn inflate_back_window_size_matches_each_supported_bit_width() {
        for window_bits in 8..=15 {
            assert_eq!(
                inflate_back_window_size(window_bits),
                Some(1 << window_bits)
            );
        }
    }

    #[test]
    fn inflate_back_window_size_rejects_unsupported_bit_widths() {
        for window_bits in [::core::ffi::c_int::MIN, -1, 7, 16, ::core::ffi::c_int::MAX] {
            assert_eq!(inflate_back_window_size(window_bits), None);
        }
    }

    #[test]
    fn inflate_back_init_plan_preserves_window_and_allocator_decisions() {
        assert_eq!(
            inflate_back_init_plan(15, false, false),
            Some(InflateBackInitPlan {
                window_bits: 15,
                window_size: 32768,
                install_default_zalloc: true,
                install_default_zfree: true,
            })
        );
        assert_eq!(
            inflate_back_init_plan(8, true, true),
            Some(InflateBackInitPlan {
                window_bits: 8,
                window_size: 256,
                install_default_zalloc: false,
                install_default_zfree: false,
            })
        );
        assert_eq!(inflate_back_init_plan(7, false, false), None);
    }

    #[test]
    fn inflate_back_init_metadata_validation_requires_matching_version_and_size() {
        let version_first_byte = crate::zlib_h::ZLIB_VERSION[0 as usize] as ::core::ffi::c_int;
        let stream_size = ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int;

        assert!(inflate_back_init_metadata_is_valid(
            Some(version_first_byte),
            stream_size,
        ));
        assert!(!inflate_back_init_metadata_is_valid(
            Some(version_first_byte.wrapping_add(1)),
            stream_size,
        ));
        assert!(!inflate_back_init_metadata_is_valid(
            Some(version_first_byte),
            stream_size.wrapping_sub(1),
        ));
        assert!(!inflate_back_init_metadata_is_valid(None, stream_size));
    }

    #[test]
    fn inflate_back_initialization_overwrites_a_complete_safe_state() {
        let mut stream = crate::zlib_h::z_stream {
            next_in: ::core::ptr::null_mut(),
            avail_in: 0,
            total_in: 0,
            next_out: ::core::ptr::null_mut(),
            avail_out: 0,
            total_out: 0,
            msg: ::core::ptr::null_mut(),
            state: ::core::ptr::null_mut(),
            zalloc: None,
            zfree: None,
            opaque: ::core::ptr::null_mut(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        };
        let mut state = crate::src::inflate::inflate_state::newly_allocated();
        let state_address =
            (&mut state as *mut crate::src::inflate::inflate_state) as *mut ::core::ffi::c_void;
        let mut window = [0_u8; 256];

        inflate_back_initialize_state(&mut stream, &mut state, &mut window, 8);

        assert_eq!(stream.state, state_address);
        assert_eq!(state.dmax, 32768);
        assert_eq!(state.wbits, 8);
        assert_eq!(state.wsize, 256);
        assert_eq!(state.window, window.as_mut_ptr());
        assert_eq!(
            state.window_ownership,
            crate::src::inflate::WindowOwnership::CallerBorrowed.raw()
        );
        assert_eq!(state.wnext, 0);
        assert_eq!(state.whave, 0);
        assert_eq!(state.sane, 1);
        assert!(state.strm.is_null());
        assert!(state.head.is_null());
        assert_eq!(state.wrap, 0);
        assert_eq!(state.check, 0);
    }

    #[test]
    fn inflate_back_stored_block_length_requires_complementary_words() {
        assert_eq!(inflate_back_stored_block_length(0xedcb1234), Some(0x1234));
        assert_eq!(inflate_back_stored_block_length(0xffff0000), Some(0));
        assert_eq!(inflate_back_stored_block_length(0xedca1234), None);
    }

    #[test]
    fn inflate_back_copy_count_limits_requested_input_and_output() {
        assert_eq!(inflate_back_copy_count(8, 12, 16), 8);
        assert_eq!(inflate_back_copy_count(20, 12, 16), 12);
        assert_eq!(inflate_back_copy_count(20, 24, 16), 16);
        assert_eq!(inflate_back_copy_count(20, 0, 16), 0);
    }

    #[test]
    fn inflate_back_initial_input_count_requires_an_input_pointer() {
        assert_eq!(inflate_back_initial_input_count(true, 17), 17);
        assert_eq!(inflate_back_initial_input_count(false, 17), 0);
    }

    #[test]
    fn inflate_back_finish_flush_status_preserves_stream_end_after_successful_output() {
        assert_eq!(
            inflate_back_finish_flush_status(crate::zlib_h::Z_STREAM_END, false),
            crate::zlib_h::Z_STREAM_END
        );
    }

    #[test]
    fn inflate_back_finish_flush_status_maps_stream_end_output_failure_to_buffer_error() {
        assert_eq!(
            inflate_back_finish_flush_status(crate::zlib_h::Z_STREAM_END, true),
            crate::zlib_h::Z_BUF_ERROR
        );
    }

    #[test]
    fn inflate_back_finish_flush_status_preserves_non_stream_end_errors() {
        assert_eq!(
            inflate_back_finish_flush_status(crate::zlib_h::Z_DATA_ERROR, true),
            crate::zlib_h::Z_DATA_ERROR
        );
    }

    #[test]
    fn inflate_back_match_copy_plan_selects_window_side_and_count() {
        assert_eq!(
            inflate_back_match_copy_plan(32, 20, 16, 7),
            (InflateBackMatchSource::Ahead(12), 4)
        );
        assert_eq!(
            inflate_back_match_copy_plan(32, 20, 12, 7),
            (InflateBackMatchSource::Behind(20), 7)
        );
        assert_eq!(
            inflate_back_match_copy_plan(32, 20, 0, 7),
            (InflateBackMatchSource::Behind(20), 0)
        );
    }

    #[test]
    fn inflate_back_copy_match_reuses_new_output_for_overlapping_back_references() {
        let mut window = [b'a', b'b', 0, 0, 0, 0, 0, 0];

        assert!(
            inflate_back_copy_match(&mut window, 2, InflateBackMatchSource::Behind(2), 6).is_some()
        );

        assert_eq!(window, [b'a', b'b', b'a', b'b', b'a', b'b', b'a', b'b']);
    }

    #[test]
    fn inflate_back_copy_match_reads_wrapped_history_ahead_of_output() {
        let mut window = [0, 0, 0, 0, b'w', b'x', b'y', b'z'];

        assert!(
            inflate_back_copy_match(&mut window, 0, InflateBackMatchSource::Ahead(4), 4).is_some()
        );

        assert_eq!(window, [b'w', b'x', b'y', b'z', b'w', b'x', b'y', b'z']);
    }

    #[test]
    fn inflate_back_copy_match_rejects_ranges_without_mutating_the_window() {
        let original = [b'a', b'b', b'c', b'd'];

        for (destination, source, count) in [
            (1, InflateBackMatchSource::Behind(2), 1),
            (1, InflateBackMatchSource::Ahead(3), 1),
            (3, InflateBackMatchSource::Behind(1), 2),
        ] {
            let mut window = original;
            assert_eq!(
                inflate_back_copy_match(&mut window, destination, source, count),
                None
            );
            assert_eq!(window, original);
        }
    }

    #[test]
    fn inflate_back_code_length_repeat_maps_repeat_symbols() {
        assert_eq!(
            inflate_back_code_length_repeat(16),
            InflateBackCodeLengthRepeat::Previous { extra_bits: 2 }
        );
        assert_eq!(
            inflate_back_code_length_repeat(17),
            InflateBackCodeLengthRepeat::Zero {
                extra_bits: 3,
                base: 3,
            }
        );
        assert_eq!(
            inflate_back_code_length_repeat(18),
            InflateBackCodeLengthRepeat::Zero {
                extra_bits: 7,
                base: 11,
            }
        );
    }

    #[test]
    fn inflate_back_code_length_repeat_preserves_other_symbol_fallback() {
        assert_eq!(
            inflate_back_code_length_repeat(19),
            InflateBackCodeLengthRepeat::Zero {
                extra_bits: 7,
                base: 11,
            }
        );
    }

    #[test]
    fn inflate_back_code_length_repeat_extra_bits_preserves_repeat_width() {
        let previous = InflateBackCodeLengthRepeat::Previous { extra_bits: 2 };
        let short_zero_run = InflateBackCodeLengthRepeat::Zero {
            extra_bits: 3,
            base: 3,
        };
        let long_zero_run = InflateBackCodeLengthRepeat::Zero {
            extra_bits: 7,
            base: 11,
        };

        assert_eq!(inflate_back_code_length_repeat_extra_bits(&previous), 2);
        assert_eq!(
            inflate_back_code_length_repeat_extra_bits(&short_zero_run),
            3
        );
        assert_eq!(
            inflate_back_code_length_repeat_extra_bits(&long_zero_run),
            7
        );
    }

    #[test]
    fn inflate_back_code_length_repeat_plan_uses_previous_length_and_consumes_extra_bits() {
        assert_eq!(
            inflate_back_code_length_repeat_plan(
                &InflateBackCodeLengthRepeat::Previous { extra_bits: 2 },
                Some(9),
                0b110_10,
                5,
            ),
            Some(InflateBackCodeLengthRepeatPlan {
                length: 9,
                count: 5,
                hold: 0b110,
                bits: 3,
            })
        );
    }

    #[test]
    fn inflate_back_code_length_repeat_plan_rejects_previous_repeat_without_history() {
        assert_eq!(
            inflate_back_code_length_repeat_plan(
                &InflateBackCodeLengthRepeat::Previous { extra_bits: 2 },
                None,
                0,
                2,
            ),
            None
        );
    }

    #[test]
    fn inflate_back_code_length_repeat_plan_builds_zero_run_from_base_and_extra_bits() {
        assert_eq!(
            inflate_back_code_length_repeat_plan(
                &InflateBackCodeLengthRepeat::Zero {
                    extra_bits: 3,
                    base: 3,
                },
                None,
                0b1_101,
                4,
            ),
            Some(InflateBackCodeLengthRepeatPlan {
                length: 0,
                count: 8,
                hold: 1,
                bits: 1,
            })
        );
    }

    #[test]
    fn inflate_back_distance_window_limit_accounts_for_available_history() {
        assert!(!inflate_back_distance_exceeds_window(
            32768, 32768, 32768, 64
        ));
        assert!(inflate_back_distance_exceeds_window(
            32769, 32768, 32768, 64
        ));

        assert!(!inflate_back_distance_exceeds_window(32704, 32768, 32, 64));
        assert!(inflate_back_distance_exceeds_window(32705, 32768, 32, 64));
    }

    #[test]
    fn inflate_back_block_header_decodes_all_block_kinds() {
        let stored = inflate_back_block_header(0b000, 3);
        assert_eq!(stored.last, 0);
        assert_eq!(stored.kind, InflateBackBlockKind::Stored);
        assert_eq!(stored.hold, 0);
        assert_eq!(stored.bits, 0);

        let fixed = inflate_back_block_header(0b011, 3);
        assert_eq!(fixed.last, 1);
        assert_eq!(fixed.kind, InflateBackBlockKind::Fixed);

        assert_eq!(
            inflate_back_block_header(0b100, 3).kind,
            InflateBackBlockKind::Dynamic
        );
        assert_eq!(
            inflate_back_block_header(0b110, 3).kind,
            InflateBackBlockKind::Invalid
        );
    }

    #[test]
    fn inflate_back_block_header_preserves_remaining_bits() {
        let header = inflate_back_block_header(0b101_101, 11);

        assert_eq!(header.last, 1);
        assert_eq!(header.kind, InflateBackBlockKind::Dynamic);
        assert_eq!(header.hold, 0b101);
        assert_eq!(header.bits, 8);
    }

    #[test]
    fn inflate_back_root_table_index_uses_only_low_table_bits() {
        assert_eq!(inflate_back_root_table_index(0b1011_0010, 0), 0);
        assert_eq!(inflate_back_root_table_index(0b1011_0010, 3), 2);
        assert_eq!(inflate_back_root_table_index(0b1011_0010, 7), 0b011_0010);
    }

    #[test]
    fn inflate_back_subtable_index_preserves_base_mask_and_shift() {
        assert_eq!(inflate_back_subtable_index(0b11_101_10, 17, 2, 3), 22);
        assert_eq!(inflate_back_subtable_index(0xffff_ffff, 3, 5, 2), 6);
    }
}
