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

struct InflateBackStateConfig {
    dmax: ::core::ffi::c_uint,
    wbits: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
}

// An inflateBack input callback is allowed to signal exhaustion with zero.
// Any nonzero length, however, must come with a buffer before the decoder
// advances and dereferences `next`.  Keep that callback contract check at
// every refill site in the raw decoder rather than relying on a later byte
// load to discover an invalid callback result.
macro_rules! inflate_back_refill {
    ($input:expr, $input_desc:expr, $next:ident) => {{
        let have = $input.expect("non-null function pointer")($input_desc, &raw mut $next);
        if have != 0 && $next.is_null() {
            0
        } else {
            have
        }
    }};
}

// Keep the public initializer's validation order independent of its raw
// stream and window bindings.  In particular, a bad version must win over
// every other error, as it does in zlib.
fn inflate_back_init_config(
    version_first: Option<::core::ffi::c_char>,
    stream_size: ::core::ffi::c_int,
    has_stream: bool,
    has_window: bool,
    window_bits: ::core::ffi::c_int,
) -> Result<InflateBackStateConfig, ::core::ffi::c_int> {
    if version_first != Some(crate::zlib_h::ZLIB_VERSION[0])
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return Err(crate::zlib_h::Z_VERSION_ERROR);
    }
    if !has_stream || !has_window || !(8..=15).contains(&window_bits) {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok(inflate_back_state_config(window_bits))
}

#[derive(Copy, Clone)]
enum InflateBackBlockType {
    Stored,
    Fixed,
    Dynamic,
    Invalid,
}

#[derive(Copy, Clone)]
struct InflateBackDynamicHeader {
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
    ncode: ::core::ffi::c_uint,
}

const INFLATE_BACK_CODE_LENGTH_ORDER: [::core::ffi::c_ushort; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

#[derive(Copy, Clone)]
enum InflateBackCodeLengthRepeat {
    Previous,
    Zero,
}

#[derive(Copy, Clone)]
enum InflateBackLengthCode {
    Literal,
    End,
    Invalid,
    Match {
        extra: ::core::ffi::c_uint,
    },
}

#[derive(Copy, Clone)]
enum InflateBackDistanceCode {
    Invalid,
    Distance {
        offset: ::core::ffi::c_uint,
        extra: ::core::ffi::c_uint,
    },
}

#[derive(Copy, Clone)]
enum InflateBackCodeTable {
    Length,
    Distance,
}

// The three tables built for a dynamic block have fixed root widths.  Keep
// their selection and the `lens` partitioning value-only, so the raw
// `inflate_table` call only receives an already-validated construction plan.
struct InflateBackDynamicTableSpec {
    table_type: crate::src::inftrees::codetype,
    code_count: ::core::ffi::c_uint,
    root_bits: ::core::ffi::c_uint,
    lens_offset: usize,
}

#[derive(Copy, Clone)]
enum InflateBackDynamicTableTarget {
    CodeLengths,
    LiteralLengths,
    Distances,
}

fn inflate_back_code_length_table_spec() -> InflateBackDynamicTableSpec {
    InflateBackDynamicTableSpec {
        table_type: crate::src::inftrees::CODES,
        code_count: 19,
        root_bits: 7,
        lens_offset: 0,
    }
}

fn inflate_back_literal_length_table_spec(
    nlen: ::core::ffi::c_uint,
) -> InflateBackDynamicTableSpec {
    InflateBackDynamicTableSpec {
        table_type: crate::src::inftrees::LENS,
        code_count: nlen,
        root_bits: 9,
        lens_offset: 0,
    }
}

fn inflate_back_distance_table_spec(
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
) -> InflateBackDynamicTableSpec {
    InflateBackDynamicTableSpec {
        table_type: crate::src::inftrees::DISTS,
        code_count: ndist,
        root_bits: 6,
        lens_offset: nlen as usize,
    }
}

// The table builder itself is an FFI boundary, but selecting the destination
// table and publishing its root width are ordinary decoder state changes.
// Keep those changes reference-based before the caller makes that raw call.
fn inflate_back_prepare_dynamic_table(
    state: &mut crate::src::inflate::inflate_state,
    target: InflateBackDynamicTableTarget,
    spec: &InflateBackDynamicTableSpec,
) {
    match target {
        InflateBackDynamicTableTarget::CodeLengths
        | InflateBackDynamicTableTarget::LiteralLengths => {
            state.next = state.codes.as_mut_ptr();
            state.lencode = state.next as *const crate::src::inftrees::code;
            state.lenbits = spec.root_bits;
        }
        InflateBackDynamicTableTarget::Distances => {
            state.distcode = state.next as *const crate::src::inftrees::code;
            state.distbits = spec.root_bits;
        }
    }
}

fn inflate_back_state_config(window_bits: ::core::ffi::c_int) -> InflateBackStateConfig {
    InflateBackStateConfig {
        dmax: 32768 as ::core::ffi::c_uint,
        wbits: window_bits as crate::stdlib::uInt as ::core::ffi::c_uint,
        wsize: (1 as ::core::ffi::c_uint) << window_bits,
    }
}

// `inflateBack()` starts each decode by resetting only the fields whose
// lifetime is confined to that operation.  Keep that state transition
// reference-based once the FFI entry point has established the state binding.
fn inflate_back_reset(state: &mut crate::src::inflate::inflate_state) {
    state.mode = crate::src::inflate::TYPE;
    state.last = 0;
    state.whave = 0;
}

fn inflate_back_block_header(
    hold: ::core::ffi::c_ulong,
) -> (::core::ffi::c_int, InflateBackBlockType) {
    let last = (hold & 1) as ::core::ffi::c_int;
    let block_type = match (hold >> 1) & 3 {
        0 => InflateBackBlockType::Stored,
        1 => InflateBackBlockType::Fixed,
        2 => InflateBackBlockType::Dynamic,
        _ => InflateBackBlockType::Invalid,
    };
    (last, block_type)
}

// Applying a valid block header changes only decoder-owned state.  Keep that
// transition reference-based after the FFI loop has decoded the header; the
// invalid case is left to the caller so it can publish the diagnostic before
// entering BAD, exactly as the original control flow does.
fn inflate_back_start_block(
    state: &mut crate::src::inflate::inflate_state,
    block_type: InflateBackBlockType,
) -> bool {
    match block_type {
        InflateBackBlockType::Stored => {
            state.mode = crate::src::inflate::STORED;
            false
        }
        InflateBackBlockType::Fixed => {
            crate::src::inftrees::inflate_fixed(state);
            state.mode = crate::src::inflate::LEN;
            false
        }
        InflateBackBlockType::Dynamic => {
            state.mode = crate::src::inflate::TABLE;
            false
        }
        InflateBackBlockType::Invalid => true,
    }
}

fn inflate_back_stored_length(hold: ::core::ffi::c_ulong) -> Option<::core::ffi::c_uint> {
    let length = hold as ::core::ffi::c_uint & 0xffff;
    if hold & 0xffff == hold >> 16 ^ 0xffff {
        Some(length)
    } else {
        None
    }
}

// A validated stored-block length begins a decoder-owned copy operation.  The
// caller still owns the bit-buffer reset and the raw input/output boundary.
fn inflate_back_start_stored_copy(
    state: &mut crate::src::inflate::inflate_state,
    length: ::core::ffi::c_uint,
) {
    state.length = length;
}

// These transitions happen around output callbacks, but do not themselves
// touch the callback or its raw window pointer.  Keeping them here makes the
// ownership boundary explicit: the caller publishes bytes, while this helper
// maintains decoder bookkeeping and modes.
fn inflate_back_reset_output_window(state: &mut crate::src::inflate::inflate_state) {
    state.whave = state.wsize;
}

// An output callback consumes a whole window, after which the decoder starts
// filling that same window again.  The caller retains the raw window binding
// and performs the callback; this helper owns only the decoder bookkeeping.
fn inflate_back_reopen_output_window(
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_uint {
    inflate_back_reset_output_window(state);
    state.wsize
}

// Decide whether the decoder must publish a full output window before more
// bytes can be written.  The caller keeps the raw window pointer and invokes
// the callback; this helper performs only state and count bookkeeping.
fn inflate_back_prepare_output_window(
    state: &mut crate::src::inflate::inflate_state,
    left: &mut ::core::ffi::c_uint,
) -> bool {
    if *left != 0 {
        return false;
    }
    *left = inflate_back_reopen_output_window(state);
    true
}

fn inflate_back_finish_stored_block(state: &mut crate::src::inflate::inflate_state) {
    state.mode = crate::src::inflate::TYPE;
}

fn inflate_back_finish_literal(state: &mut crate::src::inflate::inflate_state) {
    state.mode = crate::src::inflate::LEN;
}

fn inflate_back_literal_byte(state: &crate::src::inflate::inflate_state) -> ::core::ffi::c_uchar {
    state.length as ::core::ffi::c_uchar
}

fn inflate_back_finish_end_code(state: &mut crate::src::inflate::inflate_state) {
    state.mode = crate::src::inflate::TYPE;
}

fn inflate_back_finish_stream(state: &mut crate::src::inflate::inflate_state) {
    state.mode = crate::src::inflate::DONE;
}

fn inflate_back_enter_bad(state: &mut crate::src::inflate::inflate_state) {
    state.mode = crate::src::inflate::BAD;
}

fn inflate_back_dynamic_header(hold: ::core::ffi::c_ulong) -> InflateBackDynamicHeader {
    InflateBackDynamicHeader {
        nlen: (hold as ::core::ffi::c_uint & 31).wrapping_add(257),
        ndist: ((hold >> 5) as ::core::ffi::c_uint & 31).wrapping_add(1),
        ncode: ((hold >> 10) as ::core::ffi::c_uint & 15).wrapping_add(4),
    }
}

fn inflate_back_dynamic_header_is_valid(header: InflateBackDynamicHeader) -> bool {
    header.nlen <= 286 && header.ndist <= 30
}

fn inflate_back_set_dynamic_header(
    state: &mut crate::src::inflate::inflate_state,
    header: InflateBackDynamicHeader,
) {
    state.nlen = header.nlen;
    state.ndist = header.ndist;
    state.ncode = header.ncode;
}

fn inflate_back_start_code_length_order(state: &mut crate::src::inflate::inflate_state) {
    state.have = 0;
}

fn inflate_back_code_length_repeat(
    code: ::core::ffi::c_ushort,
) -> (InflateBackCodeLengthRepeat, ::core::ffi::c_uint, ::core::ffi::c_uint) {
    match code {
        16 => (InflateBackCodeLengthRepeat::Previous, 3, 2),
        17 => (InflateBackCodeLengthRepeat::Zero, 3, 3),
        _ => (InflateBackCodeLengthRepeat::Zero, 11, 7),
    }
}

fn inflate_back_length_code(code: crate::src::inftrees::code) -> InflateBackLengthCode {
    let op = code.op as ::core::ffi::c_uint;
    if op == 0 {
        InflateBackLengthCode::Literal
    } else if op & 32 != 0 {
        InflateBackLengthCode::End
    } else if op & 64 != 0 {
        InflateBackLengthCode::Invalid
    } else {
        InflateBackLengthCode::Match {
            extra: op & 15,
        }
    }
}

fn inflate_back_distance_code(code: crate::src::inftrees::code) -> InflateBackDistanceCode {
    let op = code.op as ::core::ffi::c_uint;
    if op & 64 != 0 {
        InflateBackDistanceCode::Invalid
    } else {
        InflateBackDistanceCode::Distance {
            offset: code.val as ::core::ffi::c_uint,
            extra: op & 15,
        }
    }
}

// Decode-table entries carry both their semantic meaning and the initial
// length or distance value.  Apply that value-only transition through a
// state reference before the FFI-facing loop performs any callback or byte
// access.  In particular, an invalid distance entry must leave `offset` and
// `extra` untouched, matching the original control flow.
fn inflate_back_start_length_code(
    state: &mut crate::src::inflate::inflate_state,
    code: crate::src::inftrees::code,
) -> InflateBackLengthCode {
    let decoded = inflate_back_length_code(code);
    state.length = code.val as ::core::ffi::c_uint;
    if let InflateBackLengthCode::Match { extra } = decoded {
        state.extra = extra;
    }
    decoded
}

fn inflate_back_start_distance_code(
    state: &mut crate::src::inflate::inflate_state,
    code: crate::src::inftrees::code,
) -> InflateBackDistanceCode {
    let decoded = inflate_back_distance_code(code);
    if let InflateBackDistanceCode::Distance { offset, extra } = decoded {
        state.offset = offset;
        state.extra = extra;
    }
    decoded
}

fn inflate_back_add_length_extra(
    state: &mut crate::src::inflate::inflate_state,
    extra: ::core::ffi::c_uint,
) {
    state.length = state.length.wrapping_add(extra);
}

fn inflate_back_add_distance_extra(
    state: &mut crate::src::inflate::inflate_state,
    extra: ::core::ffi::c_uint,
) {
    state.offset = state.offset.wrapping_add(extra);
}

fn inflate_back_finish_dynamic_tables(state: &mut crate::src::inflate::inflate_state) {
    state.mode = crate::src::inflate::LEN;
}

fn inflate_back_start_dynamic_code_lengths(state: &mut crate::src::inflate::inflate_state) {
    state.have = 0;
}

fn inflate_back_has_end_code(state: &crate::src::inflate::inflate_state) -> bool {
    state.lens[256] != 0
}

fn inflate_back_length_code_needs_subtable(code: crate::src::inftrees::code) -> bool {
    let op = code.op as ::core::ffi::c_uint;
    op != 0 && op & 0xf0 == 0
}

fn inflate_back_distance_code_needs_subtable(code: crate::src::inftrees::code) -> bool {
    (code.op as ::core::ffi::c_uint) & 0xf0 == 0
}

// A decode table is either one of the immutable fixed tables or a range in
// `state.codes`.  Keep the pointer-to-index conversion here so callers only
// select a table and an already-decoded index.
fn inflate_back_code_table_entry(
    state: &crate::src::inflate::inflate_state,
    table: InflateBackCodeTable,
    index: usize,
) -> crate::src::inftrees::code {
    let (code_table, fixed_table) = match table {
        InflateBackCodeTable::Length => (
            state.lencode,
            crate::src::inftrees::inffixed_h::lenfix.as_ptr(),
        ),
        InflateBackCodeTable::Distance => (
            state.distcode,
            crate::src::inftrees::inffixed_h::distfix.as_ptr(),
        ),
    };
    if ::core::ptr::eq(code_table, fixed_table) {
        match table {
            InflateBackCodeTable::Length => crate::src::inftrees::inffixed_h::lenfix[index],
            InflateBackCodeTable::Distance => crate::src::inftrees::inffixed_h::distfix[index],
        }
    } else {
        let base = state.codes.as_ptr().addr();
        let start = code_table
            .addr()
            .wrapping_sub(base)
            .wrapping_div(::core::mem::size_of::<crate::src::inftrees::code>());
        state.codes[start.wrapping_add(index)]
    }
}

fn inflate_back_low_bits(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    hold as ::core::ffi::c_uint
        & ((1 as ::core::ffi::c_uint) << bits).wrapping_sub(1 as ::core::ffi::c_uint)
}

fn inflate_back_drop_bits(
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
) {
    *hold >>= count;
    *bits = bits.wrapping_sub(count);
}

fn inflate_back_take_bits(
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let value = inflate_back_low_bits(*hold, count);
    inflate_back_drop_bits(hold, bits, count);
    value
}

fn inflate_back_table_index(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> isize {
    inflate_back_low_bits(hold, bits) as isize
}

fn inflate_back_subtable_index(
    hold: ::core::ffi::c_ulong,
    last: crate::src::inftrees::code,
) -> isize {
    (last.val as ::core::ffi::c_uint).wrapping_add(
        inflate_back_low_bits(
            hold,
            (last.bits as ::core::ffi::c_int + last.op as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        ) >> last.bits as ::core::ffi::c_int,
    ) as isize
}

fn inflate_back_distance_fits(
    offset: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
) -> bool {
    offset <= wsize.wrapping_sub(if whave < wsize { left } else { 0 })
}

struct InflateBackMatchCopy {
    from_offset: isize,
    count: ::core::ffi::c_uint,
}

fn inflate_back_match_copy(
    wsize: ::core::ffi::c_uint,
    offset: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    length: ::core::ffi::c_uint,
) -> InflateBackMatchCopy {
    let distance_to_end = wsize.wrapping_sub(offset);
    let (from_offset, available) = if distance_to_end < left {
        (distance_to_end as isize, left.wrapping_sub(distance_to_end))
    } else {
        (-(offset as isize), left)
    };
    InflateBackMatchCopy {
        from_offset,
        count: available.min(length),
    }
}

fn inflate_back_stored_copy_count(
    length: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    length.min(have).min(left)
}

// Both stored and match copies consume decoded bytes from the same output
// window.  The actual pointer advances remain in the caller; this transition
// keeps their length/window accounting reference-based.
fn inflate_back_consume_output_copy(
    state: &mut crate::src::inflate::inflate_state,
    left: &mut ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
) {
    state.length = state.length.wrapping_sub(count);
    *left = left.wrapping_sub(count);
}

fn inflate_back_can_use_fast_path(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
) -> bool {
    have >= 6 && left >= 258
}

fn inflate_back_pending_output(
    wsize: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    (left < wsize).then(|| wsize.wrapping_sub(left))
}

fn inflate_back_push_code_length(
    lens: &mut [::core::ffi::c_ushort; 320],
    have: &mut ::core::ffi::c_uint,
    length: ::core::ffi::c_ushort,
) {
    lens[*have as usize] = length;
    *have = have.wrapping_add(1);
}

fn inflate_back_set_code_length_order(
    lens: &mut [::core::ffi::c_ushort; 320],
    have: &mut ::core::ffi::c_uint,
    hold: ::core::ffi::c_ulong,
) {
    let order_index = INFLATE_BACK_CODE_LENGTH_ORDER[*have as usize] as usize;
    lens[order_index] = inflate_back_low_bits(hold, 3) as ::core::ffi::c_ushort;
    *have = have.wrapping_add(1);
}

fn inflate_back_finish_code_length_order(
    lens: &mut [::core::ffi::c_ushort; 320],
    have: &mut ::core::ffi::c_uint,
) {
    while *have < INFLATE_BACK_CODE_LENGTH_ORDER.len() as ::core::ffi::c_uint {
        let order_index = INFLATE_BACK_CODE_LENGTH_ORDER[*have as usize] as usize;
        lens[order_index] = 0;
        *have = have.wrapping_add(1);
    }
}

fn inflate_back_repeat_length(
    lens: &[::core::ffi::c_ushort; 320],
    have: ::core::ffi::c_uint,
    repeat_kind: InflateBackCodeLengthRepeat,
) -> Option<::core::ffi::c_uint> {
    match repeat_kind {
        InflateBackCodeLengthRepeat::Previous => have
            .checked_sub(1)
            .map(|index| lens[index as usize] as ::core::ffi::c_uint),
        InflateBackCodeLengthRepeat::Zero => Some(0),
    }
}

fn inflate_back_repeat_fits(
    have: ::core::ffi::c_uint,
    repeat: ::core::ffi::c_uint,
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
) -> bool {
    have.wrapping_add(repeat) <= nlen.wrapping_add(ndist)
}

fn inflate_back_push_repeated_code_length(
    lens: &mut [::core::ffi::c_ushort; 320],
    have: &mut ::core::ffi::c_uint,
    length: ::core::ffi::c_uint,
    repeat: ::core::ffi::c_uint,
) {
    let end = have.wrapping_add(repeat) as usize;
    lens[*have as usize..end].fill(length as ::core::ffi::c_ushort);
    *have = end as ::core::ffi::c_uint;
}

// The caller has validated the stream binding before reaching this helper.
// Default allocator selection and error-message reset are ordinary stream
// state transitions; keep them reference-based so the allocation boundary in
// `inflateBackInit_` only handles the still-uninitialized state object.
fn inflate_back_prepare_stream(strm: &mut crate::zlib_h::z_stream) {
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
}

// Once the allocation has been bound, initializing the rest of an
// inflateBack state is ordinary reference-bound state setup. Keeping this
// separate leaves the allocation callback and caller-window binding in
// `inflateBackInit_`'s narrow implementation boundary.
fn inflate_back_init_state(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    config: InflateBackStateConfig,
) {
    strm.state = state as *mut crate::src::inflate::inflate_state
        as *mut crate::src::deflate::internal_state;
    state.dmax = config.dmax;
    state.wbits = config.wbits;
    state.wsize = config.wsize;
    state.wnext = 0;
    state.whave = 0;
    state.sane = 1;
}

pub unsafe fn inflateBackInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    version_first: Option<::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let config = match inflate_back_init_config(
        version_first,
        stream_size,
        !strm.is_null(),
        !window.is_null(),
        windowBits,
    ) {
        Ok(config) => config,
        Err(error) => return error,
    };
    let strm_ref = &mut *strm;
    inflate_back_prepare_stream(strm_ref);
    state = Some(strm_ref.zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        strm_ref.opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if state.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    let state_ref = &mut *state;
    state_ref.window = window;
    inflate_back_init_state(strm_ref, state_ref, config);
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
    let version_first = version.as_ref().copied();
    inflateBackInit_(strm, windowBits, window, version_first, stream_size)
}
pub unsafe extern "C" fn inflateBack(
    strm: &mut crate::zlib_h::z_stream,
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
    if strm.state.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // The stream owns this initialized allocation for the whole decode. Bind
    // it once after the null check so decoder bookkeeping stays reference-based.
    let state = (*strm).state as *mut crate::src::inflate::inflate_state;
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let state_ref = &mut *state;
    inflate_back_reset(state_ref);
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    have = (if !next.is_null() {
        (*strm).avail_in
    } else {
        0 as crate::stdlib::uInt
    }) as ::core::ffi::c_uint;
    hold = 0 as ::core::ffi::c_ulong;
    bits = 0 as ::core::ffi::c_uint;
    put = state_ref.window;
    left = state_ref.wsize;
    '_inf_leave: loop {
        match state_ref.mode as ::core::ffi::c_uint {
            16191 => {
                if state_ref.last != 0 {
                    let padding = bits & 7;
                    inflate_back_drop_bits(&mut hold, &mut bits, padding);
                    inflate_back_finish_stream(state_ref);
                    continue;
                } else {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            have = inflate_back_refill!(in_0, in_desc, next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh0 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh0 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    let (last, block_type) = inflate_back_block_header(hold);
                    state_ref.last = last;
                    inflate_back_drop_bits(&mut hold, &mut bits, 3);
                    if inflate_back_start_block(state_ref, block_type) {
                        (*strm).msg = b"invalid block type\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        inflate_back_enter_bad(state_ref);
                    }
                    continue;
                }
            }
            16193 => {
                let padding = bits & 7;
                inflate_back_drop_bits(&mut hold, &mut bits, padding);
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        have = inflate_back_refill!(in_0, in_desc, next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh1 = next;
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh1 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if let Some(length) = inflate_back_stored_length(hold) {
                    inflate_back_start_stored_copy(state_ref, length);
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    while state_ref.length != 0 as ::core::ffi::c_uint {
                        copy = state_ref.length;
                        if have == 0 as ::core::ffi::c_uint {
                            have = inflate_back_refill!(in_0, in_desc, next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        if inflate_back_prepare_output_window(state_ref, &mut left) {
                            put = state_ref.window;
                            if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        copy = inflate_back_stored_copy_count(copy, have, left);
                        crate::stdlib::memcpy(
                            put as *mut ::core::ffi::c_void,
                            next as *const ::core::ffi::c_void,
                            copy as crate::__stddef_size_t_h::size_t,
                        );
                        have = have.wrapping_sub(copy);
                        next = next.wrapping_add(copy as usize);
                        put = put.wrapping_add(copy as usize);
                        inflate_back_consume_output_copy(state_ref, &mut left, copy);
                    }
                    inflate_back_finish_stored_block(state_ref);
                    continue;
                } else {
                    (*strm).msg = b"invalid stored block lengths\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    inflate_back_enter_bad(state_ref);
                    continue;
                }
            }
            16196 => {
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        have = inflate_back_refill!(in_0, in_desc, next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh2 = next;
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh2 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                let header = inflate_back_dynamic_header(hold);
                inflate_back_set_dynamic_header(state_ref, header);
                inflate_back_drop_bits(&mut hold, &mut bits, 14);
                if !inflate_back_dynamic_header_is_valid(header) {
                    (*strm).msg = b"too many length or distance symbols\0".as_ptr()
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    inflate_back_enter_bad(state_ref);
                    continue;
                } else {
                    inflate_back_start_code_length_order(state_ref);
                    while state_ref.have < state_ref.ncode {
                        while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                            if have == 0 as ::core::ffi::c_uint {
                                have = inflate_back_refill!(in_0, in_desc, next);
                                if have == 0 as ::core::ffi::c_uint {
                                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh3 = next;
                            next = next.wrapping_add(1);
                            hold =
                                hold.wrapping_add((*c2rust_fresh3 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        inflate_back_set_code_length_order(
                            &mut state_ref.lens,
                            &mut state_ref.have,
                            hold,
                        );
                        inflate_back_drop_bits(&mut hold, &mut bits, 3);
                    }
                    inflate_back_finish_code_length_order(
                        &mut state_ref.lens,
                        &mut state_ref.have,
                    );
                    let code_length_table = inflate_back_code_length_table_spec();
                    inflate_back_prepare_dynamic_table(
                        state_ref,
                        InflateBackDynamicTableTarget::CodeLengths,
                        &code_length_table,
                    );
                    ret = crate::src::inftrees::inflate_table(
                        code_length_table.table_type,
                        &raw mut state_ref.lens as *mut ::core::ffi::c_ushort,
                        code_length_table.code_count,
                        &raw mut state_ref.next as *mut _ as *mut *mut crate::src::inftrees::code,
                        &raw mut state_ref.lenbits,
                        &raw mut state_ref.work as *mut ::core::ffi::c_ushort,
                    );
                    if ret != 0 {
                        (*strm).msg = b"invalid code lengths set\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        inflate_back_enter_bad(state_ref);
                        continue;
                    } else {
                        inflate_back_start_dynamic_code_lengths(state_ref);
                        while state_ref.have < state_ref.nlen.wrapping_add(state_ref.ndist) {
                            loop {
                                here = inflate_back_code_table_entry(
                                    state_ref,
                                    InflateBackCodeTable::Length,
                                    inflate_back_table_index(hold, state_ref.lenbits) as usize,
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                if have == 0 as ::core::ffi::c_uint {
                                    have = inflate_back_refill!(in_0, in_desc, next);
                                    if have == 0 as ::core::ffi::c_uint {
                                        next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                        ret = crate::zlib_h::Z_BUF_ERROR;
                                        break '_inf_leave;
                                    }
                                }
                                have = have.wrapping_sub(1);
                                let c2rust_fresh6 = next;
                                next = next.wrapping_add(1);
                                hold = hold
                                    .wrapping_add((*c2rust_fresh6 as ::core::ffi::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            if (here.val as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                                inflate_back_drop_bits(
                                    &mut hold,
                                    &mut bits,
                                    here.bits as ::core::ffi::c_uint,
                                );
                                inflate_back_push_code_length(
                                    &mut state_ref.lens,
                                    &mut state_ref.have,
                                    here.val,
                                );
                            } else {
                                let (repeat_kind, repeat_base, repeat_bits) =
                                    inflate_back_code_length_repeat(here.val);
                                while bits < here.bits as ::core::ffi::c_uint + repeat_bits {
                                    if have == 0 as ::core::ffi::c_uint {
                                        have = inflate_back_refill!(in_0, in_desc, next);
                                        if have == 0 as ::core::ffi::c_uint {
                                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                            ret = crate::zlib_h::Z_BUF_ERROR;
                                            break '_inf_leave;
                                        }
                                    }
                                    have = have.wrapping_sub(1);
                                    let c2rust_fresh8 = next;
                                    next = next.wrapping_add(1);
                                    hold = hold.wrapping_add(
                                        (*c2rust_fresh8 as ::core::ffi::c_ulong) << bits,
                                    );
                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                }
                                inflate_back_drop_bits(
                                    &mut hold,
                                    &mut bits,
                                    here.bits as ::core::ffi::c_uint,
                                );
                                let Some(repeated_length) = inflate_back_repeat_length(
                                    &state_ref.lens,
                                    state_ref.have,
                                    repeat_kind,
                                ) else {
                                    (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    inflate_back_enter_bad(state_ref);
                                    break;
                                };
                                len = repeated_length;
                                copy = repeat_base.wrapping_add(inflate_back_take_bits(
                                    &mut hold,
                                    &mut bits,
                                    repeat_bits,
                                ));
                                if !inflate_back_repeat_fits(
                                    state_ref.have,
                                    copy,
                                    state_ref.nlen,
                                    state_ref.ndist,
                                ) {
                                    (*strm).msg = b"invalid bit length repeat\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    inflate_back_enter_bad(state_ref);
                                    break;
                                } else {
                                    inflate_back_push_repeated_code_length(
                                        &mut state_ref.lens,
                                        &mut state_ref.have,
                                        len,
                                        copy,
                                    );
                                }
                            }
                        }
                        if state_ref.mode as ::core::ffi::c_uint
                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        if !inflate_back_has_end_code(state_ref) {
                            (*strm).msg = b"invalid code -- missing end-of-block\0".as_ptr()
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            inflate_back_enter_bad(state_ref);
                            continue;
                        } else {
                            let literal_length_table =
                                inflate_back_literal_length_table_spec(state_ref.nlen);
                            inflate_back_prepare_dynamic_table(
                                state_ref,
                                InflateBackDynamicTableTarget::LiteralLengths,
                                &literal_length_table,
                            );
                            ret = crate::src::inftrees::inflate_table(
                                literal_length_table.table_type,
                                &raw mut state_ref.lens as *mut ::core::ffi::c_ushort,
                                literal_length_table.code_count,
                                &raw mut state_ref.next as *mut _
                                    as *mut *mut crate::src::inftrees::code,
                                &raw mut state_ref.lenbits,
                                &raw mut state_ref.work as *mut ::core::ffi::c_ushort,
                            );
                            if ret != 0 {
                                (*strm).msg = b"invalid literal/lengths set\0".as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                inflate_back_enter_bad(state_ref);
                                continue;
                            } else {
                                let distance_table = inflate_back_distance_table_spec(
                                    state_ref.nlen,
                                    state_ref.ndist,
                                );
                                inflate_back_prepare_dynamic_table(
                                    state_ref,
                                    InflateBackDynamicTableTarget::Distances,
                                    &distance_table,
                                );
                                ret = crate::src::inftrees::inflate_table(
                                    distance_table.table_type,
                                    (&raw mut state_ref.lens as *mut ::core::ffi::c_ushort)
                                        .wrapping_add(distance_table.lens_offset),
                                    distance_table.code_count,
                                    &raw mut state_ref.next as *mut _
                                        as *mut *mut crate::src::inftrees::code,
                                    &raw mut state_ref.distbits,
                                    &raw mut state_ref.work as *mut ::core::ffi::c_ushort,
                                );
                                if ret != 0 {
                                    (*strm).msg = b"invalid distances set\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    inflate_back_enter_bad(state_ref);
                                    continue;
                                } else {
                                    inflate_back_finish_dynamic_tables(state_ref);
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
        if inflate_back_can_use_fast_path(have, left) {
            (*strm).next_out = put as *mut crate::stdlib::Bytef;
            (*strm).avail_out = left as crate::stdlib::uInt;
            (*strm).next_in = next as *mut crate::stdlib::Bytef;
            (*strm).avail_in = have as crate::stdlib::uInt;
            state_ref.hold = hold;
            state_ref.bits = bits;
            crate::src::inffast::inflate_fast(
                strm as *mut crate::zlib_h::z_stream_s,
                state_ref.wsize,
            );
            put = (*strm).next_out as *mut ::core::ffi::c_uchar;
            left = (*strm).avail_out as ::core::ffi::c_uint;
            next = (*strm).next_in as *mut ::core::ffi::c_uchar;
            have = (*strm).avail_in as ::core::ffi::c_uint;
            hold = state_ref.hold;
            bits = state_ref.bits;
        } else {
            loop {
                here = inflate_back_code_table_entry(
                    state_ref,
                    InflateBackCodeTable::Length,
                    inflate_back_table_index(hold, state_ref.lenbits) as usize,
                );
                if here.bits as ::core::ffi::c_uint <= bits {
                    break;
                }
                if have == 0 as ::core::ffi::c_uint {
                    have = inflate_back_refill!(in_0, in_desc, next);
                    if have == 0 as ::core::ffi::c_uint {
                        next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break '_inf_leave;
                    }
                }
                have = have.wrapping_sub(1);
                let c2rust_fresh13 = next;
                next = next.wrapping_add(1);
                hold = hold.wrapping_add((*c2rust_fresh13 as ::core::ffi::c_ulong) << bits);
                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            }
            if inflate_back_length_code_needs_subtable(here) {
                last = here;
                loop {
                    here = inflate_back_code_table_entry(
                        state_ref,
                        InflateBackCodeTable::Length,
                        inflate_back_subtable_index(hold, last) as usize,
                    );
                    if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        <= bits
                    {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        have = inflate_back_refill!(in_0, in_desc, next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh14 = next;
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh14 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                inflate_back_drop_bits(
                    &mut hold,
                    &mut bits,
                    last.bits as ::core::ffi::c_uint,
                );
            }
            inflate_back_drop_bits(
                &mut hold,
                &mut bits,
                here.bits as ::core::ffi::c_uint,
            );
            match inflate_back_start_length_code(state_ref, here) {
                InflateBackLengthCode::Literal => {
                if inflate_back_prepare_output_window(state_ref, &mut left) {
                    put = state_ref.window;
                    if out.expect("non-null function pointer")(out_desc, put, left) != 0 {
                        ret = crate::zlib_h::Z_BUF_ERROR;
                        break;
                    }
                }
                let c2rust_fresh15 = put;
                put = put.wrapping_add(1);
                *c2rust_fresh15 = inflate_back_literal_byte(state_ref);
                left = left.wrapping_sub(1);
                inflate_back_finish_literal(state_ref);
                }
                InflateBackLengthCode::End => {
                inflate_back_finish_end_code(state_ref);
                }
                InflateBackLengthCode::Invalid => {
                (*strm).msg = b"invalid literal/length code\0".as_ptr()
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                inflate_back_enter_bad(state_ref);
                }
                InflateBackLengthCode::Match { .. } => {
                if state_ref.extra != 0 as ::core::ffi::c_uint {
                    while bits < state_ref.extra {
                        if have == 0 as ::core::ffi::c_uint {
                            have = inflate_back_refill!(in_0, in_desc, next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh16 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh16 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    let extra = inflate_back_take_bits(&mut hold, &mut bits, state_ref.extra);
                    inflate_back_add_length_extra(state_ref, extra);
                }
                loop {
                    here = inflate_back_code_table_entry(
                        state_ref,
                        InflateBackCodeTable::Distance,
                        inflate_back_table_index(hold, state_ref.distbits) as usize,
                    );
                    if here.bits as ::core::ffi::c_uint <= bits {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        have = inflate_back_refill!(in_0, in_desc, next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break '_inf_leave;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let c2rust_fresh17 = next;
                    next = next.wrapping_add(1);
                    hold = hold.wrapping_add((*c2rust_fresh17 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if inflate_back_distance_code_needs_subtable(here) {
                    last = here;
                    loop {
                        here = inflate_back_code_table_entry(
                            state_ref,
                            InflateBackCodeTable::Distance,
                            inflate_back_subtable_index(hold, last) as usize,
                        );
                        if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            <= bits
                        {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            have = inflate_back_refill!(in_0, in_desc, next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let c2rust_fresh18 = next;
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((*c2rust_fresh18 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    inflate_back_drop_bits(
                        &mut hold,
                        &mut bits,
                        last.bits as ::core::ffi::c_uint,
                    );
                }
                inflate_back_drop_bits(
                    &mut hold,
                    &mut bits,
                    here.bits as ::core::ffi::c_uint,
                );
                match inflate_back_start_distance_code(state_ref, here) {
                    InflateBackDistanceCode::Invalid => {
                    (*strm).msg = b"invalid distance code\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    inflate_back_enter_bad(state_ref);
                    }
                    InflateBackDistanceCode::Distance { .. } => {
                    if state_ref.extra != 0 as ::core::ffi::c_uint {
                        while bits < state_ref.extra {
                            if have == 0 as ::core::ffi::c_uint {
                                have = inflate_back_refill!(in_0, in_desc, next);
                                if have == 0 as ::core::ffi::c_uint {
                                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let c2rust_fresh19 = next;
                            next = next.wrapping_add(1);
                            hold = hold
                                .wrapping_add((*c2rust_fresh19 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        let extra =
                            inflate_back_take_bits(&mut hold, &mut bits, state_ref.extra);
                        inflate_back_add_distance_extra(state_ref, extra);
                    }
                    if !inflate_back_distance_fits(
                        state_ref.offset,
                        state_ref.wsize,
                        state_ref.whave,
                        left,
                    ) {
                        (*strm).msg = b"invalid distance too far back\0".as_ptr()
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        inflate_back_enter_bad(state_ref);
                    } else {
                        loop {
                            if inflate_back_prepare_output_window(state_ref, &mut left) {
                                put = state_ref.window;
                                if out.expect("non-null function pointer")(out_desc, put, left) != 0
                                {
                                    ret = crate::zlib_h::Z_BUF_ERROR;
                                    break '_inf_leave;
                                }
                            }
                            let match_copy = inflate_back_match_copy(
                                state_ref.wsize,
                                state_ref.offset,
                                left,
                                state_ref.length,
                            );
                            // `inflate_back_distance_fits()` above bounds this
                            // window-relative back-reference. Keep the cursor
                            // calculation non-unsafe; the bounded copy below
                            // performs the actual accesses.
                            from = put.wrapping_offset(match_copy.from_offset);
                            copy = match_copy.count;
                            inflate_back_consume_output_copy(
                                state_ref,
                                &mut left,
                                match_copy.count,
                            );
                            loop {
                                let c2rust_fresh20 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh21 = put;
                                put = put.wrapping_add(1);
                                *c2rust_fresh21 = *c2rust_fresh20;
                                copy = copy.wrapping_sub(1);
                                if copy == 0 {
                                    break;
                                }
                            }
                            if state_ref.length == 0 as ::core::ffi::c_uint {
                                break;
                            }
                        }
                    }
                    }
                }
                }
            }
        }
    }
    if let Some(pending) = inflate_back_pending_output(state_ref.wsize, left) {
        if out.expect("non-null function pointer")(
            out_desc,
            state_ref.window,
            pending,
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
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflateBack(&mut *strm, in_0, in_desc, out, out_desc)
}
// The FFI wrapper has already established that `strm` is a valid mutable
// stream. Keep the teardown state transition in a reference-based helper;
// only the configured C deallocator remains an unsafe boundary here.
pub fn inflateBackEnd(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    if strm.state.is_null() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // SAFETY: `inflateBackInit_` obtained `state` from this stream's `zalloc`,
    // and this validated callback is the matching deallocator configured on
    // the same stream. This is the final use of that allocation.
    unsafe {
        Some(strm.zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            strm.opaque,
            strm.state as crate::stdlib::voidpf,
        );
    }
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    crate::zlib_h::Z_OK
}
#[export_name = "inflateBackEnd"]

pub unsafe extern "C" fn inflateBackEnd_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflateBackEnd(&mut *strm)
}
