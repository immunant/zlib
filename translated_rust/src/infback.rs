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

// The raw decoder owns callback and cursor control flow, but choosing the
// decoder arm from the reference-bound inflater mode is ordinary state
// bookkeeping.  Keep the terminal return mapping here as well, so the raw
// loop does not have to encode the public status for completed or bad state.
#[derive(Copy, Clone)]
enum InflateBackDecodeMode {
    Type,
    Stored,
    Table,
    Length,
    Terminal(::core::ffi::c_int),
}

fn inflate_back_decode_mode(mode: crate::src::inflate::inflate_mode) -> InflateBackDecodeMode {
    match mode {
        crate::src::inflate::TYPE => InflateBackDecodeMode::Type,
        crate::src::inflate::STORED => InflateBackDecodeMode::Stored,
        crate::src::inflate::TABLE => InflateBackDecodeMode::Table,
        crate::src::inflate::LEN => InflateBackDecodeMode::Length,
        crate::src::inflate::DONE => InflateBackDecodeMode::Terminal(crate::zlib_h::Z_STREAM_END),
        crate::src::inflate::BAD => InflateBackDecodeMode::Terminal(crate::zlib_h::Z_DATA_ERROR),
        _ => InflateBackDecodeMode::Terminal(crate::zlib_h::Z_STREAM_ERROR),
    }
}

// A decode error changes only the stream diagnostic and the decoder mode.
// Keep that publication reference-bound so the raw callback/cursor loop only
// selects which decoder error occurred.
#[derive(Copy, Clone)]
enum InflateBackError {
    InvalidBlockType,
    InvalidStoredBlockLengths,
    TooManyLengthOrDistanceSymbols,
    InvalidCodeLengthsSet,
    InvalidBitLengthRepeat,
    MissingEndOfBlock,
    InvalidLiteralLengthsSet,
    InvalidDistancesSet,
    InvalidLiteralLengthCode,
    InvalidDistanceCode,
    InvalidDistanceTooFarBack,
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
    Match { extra: ::core::ffi::c_uint },
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

// Selecting the destination table and publishing its root width are ordinary
// decoder state changes. The bounded builder below keeps its owned workspace
// reference-based as well.
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

fn inflate_back_build_dynamic_table(
    state: &mut crate::src::inflate::inflate_state,
    target: InflateBackDynamicTableTarget,
    spec: &InflateBackDynamicTableSpec,
) -> ::core::ffi::c_int {
    let code_size = ::core::mem::size_of::<crate::src::inftrees::code>();
    let code_base = state.codes.as_ptr().addr();
    let byte_offset = match state.next.addr().checked_sub(code_base) {
        Some(offset) if offset % code_size == 0 => offset,
        _ => return 1,
    };
    let table_start = byte_offset / code_size;
    let lens_start = spec.lens_offset;
    let lens_end = match lens_start.checked_add(spec.code_count as usize) {
        Some(end) if end <= state.lens.len() => end,
        _ => return 1,
    };
    if table_start > state.codes.len() {
        return 1;
    }
    let mut root_bits = match target {
        InflateBackDynamicTableTarget::CodeLengths
        | InflateBackDynamicTableTarget::LiteralLengths => state.lenbits,
        InflateBackDynamicTableTarget::Distances => state.distbits,
    };
    let used = {
        let lens = &state.lens[lens_start..lens_end];
        let table = &mut state.codes[table_start..];
        match crate::src::inftrees::inflate_table_bound(
            spec.table_type,
            lens,
            spec.code_count,
            table,
            &mut root_bits,
            &mut state.work,
        ) {
            Ok(used) => used,
            Err(error) => return error,
        }
    };
    let Some(next) = table_start.checked_add(used) else {
        return 1;
    };
    if next > state.codes.len() {
        return 1;
    }
    state.next = state.codes[next..].as_mut_ptr();
    match target {
        InflateBackDynamicTableTarget::CodeLengths
        | InflateBackDynamicTableTarget::LiteralLengths => state.lenbits = root_bits,
        InflateBackDynamicTableTarget::Distances => state.distbits = root_bits,
    }
    0
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

// Starting an inflateBack decode clears the public diagnostic and resets the
// decoder-owned operation state together.  The caller still owns the raw
// stream/state binding and the callback/window cursors; this helper only
// returns the configured window capacity for that boundary to use.
fn inflate_back_begin_decode(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_uint {
    stream.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    inflate_back_reset(state);
    state.wsize
}

// `next_in` itself remains a raw cursor owned by the decoder.  Once it has
// established whether that cursor is present, deriving the accompanying
// available-byte count is ordinary stream bookkeeping.
fn inflate_back_initial_input_available(
    stream: &crate::zlib_h::z_stream,
    has_next: bool,
) -> ::core::ffi::c_uint {
    if has_next {
        stream.avail_in as ::core::ffi::c_uint
    } else {
        0
    }
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

// Decoding the three block-header bits and applying their meaning are two
// separate concerns: the caller owns the raw input cursor, while this helper
// owns the decoder state change that follows a successfully loaded header.
// Keeping the bit-buffer update with that state change prevents individual
// decoder arms from publishing a partially applied header.
fn inflate_back_apply_block_header(
    state: &mut crate::src::inflate::inflate_state,
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
) -> bool {
    let (last, block_type) = inflate_back_block_header(*hold);
    state.last = last;
    inflate_back_drop_bits(hold, bits, 3);
    inflate_back_start_block(state, block_type)
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

// A stored block discards its byte-aligned length descriptor before copying
// payload bytes.  This is decoder bookkeeping only; input and output cursors
// remain at the raw callback boundary.
fn inflate_back_begin_stored_copy(
    state: &mut crate::src::inflate::inflate_state,
    length: ::core::ffi::c_uint,
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
) {
    inflate_back_start_stored_copy(state, length);
    *hold = 0;
    *bits = 0;
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

// Literal publication consumes exactly one byte from the active output
// window.  The byte store itself stays in the raw decoder, while its state
// accounting is reference-bound here.
fn inflate_back_commit_literal(
    state: &mut crate::src::inflate::inflate_state,
    left: &mut ::core::ffi::c_uint,
) {
    *left = left.wrapping_sub(1);
    inflate_back_finish_literal(state);
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

fn inflate_back_report_error(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    error: InflateBackError,
) {
    strm.msg = match error {
        InflateBackError::InvalidBlockType => b"invalid block type\0".as_ptr(),
        InflateBackError::InvalidStoredBlockLengths => b"invalid stored block lengths\0".as_ptr(),
        InflateBackError::TooManyLengthOrDistanceSymbols => {
            b"too many length or distance symbols\0".as_ptr()
        }
        InflateBackError::InvalidCodeLengthsSet => b"invalid code lengths set\0".as_ptr(),
        InflateBackError::InvalidBitLengthRepeat => b"invalid bit length repeat\0".as_ptr(),
        InflateBackError::MissingEndOfBlock => b"invalid code -- missing end-of-block\0".as_ptr(),
        InflateBackError::InvalidLiteralLengthsSet => b"invalid literal/lengths set\0".as_ptr(),
        InflateBackError::InvalidDistancesSet => b"invalid distances set\0".as_ptr(),
        InflateBackError::InvalidLiteralLengthCode => b"invalid literal/length code\0".as_ptr(),
        InflateBackError::InvalidDistanceCode => b"invalid distance code\0".as_ptr(),
        InflateBackError::InvalidDistanceTooFarBack => b"invalid distance too far back\0".as_ptr(),
    } as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    inflate_back_enter_bad(state);
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

// Once the caller has loaded a complete dynamic-block header, all remaining
// work is local decoder state and bit-buffer bookkeeping.
fn inflate_back_apply_dynamic_header(
    state: &mut crate::src::inflate::inflate_state,
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
) -> InflateBackDynamicHeader {
    let header = inflate_back_dynamic_header(*hold);
    inflate_back_set_dynamic_header(state, header);
    inflate_back_drop_bits(hold, bits, 14);
    header
}

fn inflate_back_start_code_length_order(state: &mut crate::src::inflate::inflate_state) {
    state.have = 0;
}

fn inflate_back_code_length_repeat(
    code: ::core::ffi::c_ushort,
) -> (
    InflateBackCodeLengthRepeat,
    ::core::ffi::c_uint,
    ::core::ffi::c_uint,
) {
    match code {
        16 => (InflateBackCodeLengthRepeat::Previous, 3, 2),
        17 => (InflateBackCodeLengthRepeat::Zero, 3, 3),
        _ => (InflateBackCodeLengthRepeat::Zero, 11, 7),
    }
}

// A repeat carries bits after the resolved table entry. Keep the raw refill
// loop responsible for obtaining those bits, and leave the entry's effect on
// owned decoder state to the helper below.
fn inflate_back_code_length_bits_required(code: crate::src::inftrees::code) -> ::core::ffi::c_uint {
    if code.val < 16 {
        code.bits as ::core::ffi::c_uint
    } else {
        let (_, _, repeat_bits) = inflate_back_code_length_repeat(code.val);
        code.bits as ::core::ffi::c_uint + repeat_bits
    }
}

// This runs only after the caller has refilled the number of bits returned by
// `inflate_back_code_length_bits_required()`. It deliberately does not touch
// input cursors or callbacks: a dynamic code-length entry changes only the
// decoder's owned lens array, count, and bit-buffer bookkeeping.
fn inflate_back_apply_code_length(
    state: &mut crate::src::inflate::inflate_state,
    code: crate::src::inftrees::code,
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
) -> Result<(), InflateBackError> {
    inflate_back_drop_bits(hold, bits, code.bits as ::core::ffi::c_uint);
    if code.val < 16 {
        inflate_back_push_code_length(&mut state.lens, &mut state.have, code.val);
        return Ok(());
    }

    let (repeat_kind, repeat_base, repeat_bits) = inflate_back_code_length_repeat(code.val);
    let Some(length) = inflate_back_repeat_length(&state.lens, state.have, repeat_kind) else {
        return Err(InflateBackError::InvalidBitLengthRepeat);
    };
    let repeat = repeat_base.wrapping_add(inflate_back_take_bits(hold, bits, repeat_bits));
    if !inflate_back_repeat_fits(state.have, repeat, state.nlen, state.ndist) {
        return Err(InflateBackError::InvalidBitLengthRepeat);
    }
    inflate_back_push_repeated_code_length(&mut state.lens, &mut state.have, length, repeat);
    Ok(())
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
        InflateBackLengthCode::Match { extra: op & 15 }
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

// A decode table is either one of the immutable fixed tables or a checked
// range in `state.codes`.  Reuse the inflater's central resolver rather than
// deriving an index from a raw table cursor here.  That keeps inflateBack's
// dynamic-table reads bounded by the same validation used by inflate_fast.
fn inflate_back_code_table_entry(
    state: &crate::src::inflate::inflate_state,
    table: InflateBackCodeTable,
    index: usize,
) -> crate::src::inftrees::code {
    let table = match table {
        InflateBackCodeTable::Length => crate::src::inflate::InflateCodeTable::Length,
        InflateBackCodeTable::Distance => crate::src::inflate::InflateCodeTable::Distance,
    };
    crate::src::inflate::inflate_code_table(state, table)
        .and_then(|entries| entries.get(index))
        .copied()
        .expect("live inflateBack decode cursor has a complete table")
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

// Loading a byte is part of the raw input-cursor boundary, but incorporating
// that already-read value into the inflater's bit buffer is ordinary decoder
// bookkeeping. Keep the latter value-only so every refill site has the same
// bit-buffer transition without widening the raw-pointer surface.
fn inflate_back_append_input_byte(
    hold: &mut ::core::ffi::c_ulong,
    bits: &mut ::core::ffi::c_uint,
    byte: ::core::ffi::c_uchar,
) {
    *hold = hold.wrapping_add((byte as ::core::ffi::c_ulong) << *bits);
    *bits = bits.wrapping_add(8);
}

fn inflate_back_table_index(hold: ::core::ffi::c_ulong, bits: ::core::ffi::c_uint) -> isize {
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

#[derive(Copy, Clone)]
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

// A match may overlap the bytes it is producing, so copy it forward one byte
// at a time just as the raw decoder did.  The caller derives both indices
// from the active window's checked capacity before borrowing the window.
// This keeps the overlap semantics in slice code instead of dereferencing
// the raw `put` and `from` cursors in the decoder loop.
fn inflate_back_copy_match_window(
    window: &mut [crate::stdlib::Bytef],
    write_index: usize,
    copy: InflateBackMatchCopy,
) {
    let source_index = if copy.from_offset < 0 {
        write_index
            .checked_sub((-copy.from_offset) as usize)
            .expect("validated match source precedes output")
    } else {
        write_index
            .checked_add(copy.from_offset as usize)
            .expect("validated match source stays in window")
    };
    let count = copy.count as usize;
    let source_end = source_index
        .checked_add(count)
        .expect("validated match source range stays in window");
    let write_end = write_index
        .checked_add(count)
        .expect("validated match output range stays in window");
    assert!(source_end <= window.len() && write_end <= window.len());
    for index in 0..count {
        let byte = window[source_index + index];
        window[write_index + index] = byte;
    }
}

// Literal output follows the same configured-window path as match output.
// The actual byte store is bounded by the active window rather than using a
// raw output cursor in the decoder loop.
fn inflate_back_write_literal(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    left: ::core::ffi::c_uint,
    byte: crate::stdlib::Bytef,
) {
    let write_index = state
        .wsize
        .checked_sub(left)
        .expect("active inflateBack output space fits its window") as usize;
    crate::src::inflate::updatewindow(
        strm,
        state,
        crate::src::inflate::InflateWindowAccess::Existing,
        |_, window| {
            let window = window.expect("inflateBack has a configured output window");
            window[write_index] = byte;
        },
    )
    .expect("inflateBack existing window access cannot fail");
}

fn inflate_back_stored_copy_count(
    length: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    length.min(have).min(left)
}

// Stored bytes occupy the same checked output window as literals and
// matches.  Copy through that bounded view instead of the raw `put` cursor;
// the decoder keeps ownership of the callback-facing cursor updates below.
fn inflate_back_copy_stored_window(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    left: ::core::ffi::c_uint,
    source: &[crate::stdlib::Bytef],
) {
    let write_index = state
        .wsize
        .checked_sub(left)
        .expect("active inflateBack output space fits its window") as usize;
    let write_end = write_index
        .checked_add(source.len())
        .expect("stored copy fits active inflateBack output window");
    crate::src::inflate::updatewindow(
        strm,
        state,
        crate::src::inflate::InflateWindowAccess::Existing,
        |_, window| {
            let window = window.expect("inflateBack has a configured output window");
            assert!(write_end <= window.len());
            window[write_index..write_end].copy_from_slice(source);
        },
    )
    .expect("inflateBack existing window access cannot fail");
}

// Both stored and match copies consume decoded bytes from the same output
// window.  This transition keeps their length/window accounting
// reference-based.
fn inflate_back_consume_output_copy(
    state: &mut crate::src::inflate::inflate_state,
    left: &mut ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
) {
    state.length = state.length.wrapping_sub(count);
    *left = left.wrapping_sub(count);
}

fn inflate_back_pending_output(
    wsize: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    (left < wsize).then(|| wsize.wrapping_sub(left))
}

// The final output callback is still owned by the raw callback boundary, but
// its result changes only the decoder's return value.  Keep that decision
// value-only so the boundary does not also have to encode zlib's special
// successful-end-of-stream rule.
fn inflate_back_finish_pending_output(
    ret: ::core::ffi::c_int,
    output_failed: bool,
) -> ::core::ffi::c_int {
    if output_failed && ret == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        ret
    }
}

// `next_in` remains a raw cursor publication at the callback boundary.  The
// paired available-byte count is ordinary stream bookkeeping, so isolate it
// in a reference-bound helper.
fn inflate_back_publish_available_input(
    strm: &mut crate::zlib_h::z_stream,
    have: ::core::ffi::c_uint,
) {
    strm.avail_in = have as crate::stdlib::uInt;
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
fn inflate_back_prepare_stream(strm: &mut crate::zlib_h::z_stream) -> bool {
    strm.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    crate::src::zutil::prepare_stream_allocator(strm)
}

// Once the shared initializer has allocated the state, configuring it for
// inflateBack is ordinary reference-bound setup. Keeping this separate leaves
// the allocation callback and caller-window binding in `inflateBackInit_`'s
// narrow implementation boundary.
fn inflate_back_init_state(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    window: &mut ::core::ffi::c_uchar,
    config: InflateBackStateConfig,
) {
    strm.state = state as *mut crate::src::inflate::inflate_state
        as *mut crate::src::deflate::internal_state;
    state.dmax = config.dmax;
    // `inflateInit2_()` initializes a wrapped stream. `inflateBack()` works
    // on raw deflate input, matching the zero-initialized `wrap` field used
    // by zlib's dedicated initializer.
    state.wrap = 0;
    state.wbits = config.wbits;
    state.wsize = config.wsize;
    state.wnext = 0;
    state.whave = 0;
    state.sane = 1;
    // Keep the same reciprocal stream/state relationship as the regular
    // inflater.  `inflateBack()` can then use the established checked binder
    // instead of reopening this raw state pointer itself.
    state.strm = strm as *mut crate::zlib_h::z_stream;
    state.window = window;
    state.mode = crate::src::inflate::TYPE;
}

// This is deliberately the private implementation target for the exported
// initializer below. The ABI wrapper binds its foreign arguments and
// dispatches here; validation, allocation, and state setup remain outside
// the exported entry point.
fn inflateBackInit_(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut windowBits: ::core::ffi::c_int,
    window: Option<&mut ::core::ffi::c_uchar>,
    version_first: Option<::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let config = match inflate_back_init_config(
        version_first,
        stream_size,
        strm.is_some(),
        window.is_some(),
        windowBits,
    ) {
        Ok(config) => config,
        Err(error) => return error,
    };
    let strm = strm.expect("configuration preflight requires a stream");
    let window = window.expect("configuration preflight requires a window");
    // Unlike `inflateInit2_()`, zlib's `inflateBackInit_()` does not reset
    // these public accounting fields. Preserve them while reusing the common
    // allocator and state initialization path.
    let public_fields = (strm.total_in, strm.total_out, strm.data_type, strm.adler);
    let ret = crate::src::inflate::inflateInit2_(
        Some(strm),
        windowBits,
        version_first,
        stream_size,
    );
    if ret != crate::zlib_h::Z_OK {
        return ret;
    }
    let Some((strm, state)) =
        crate::src::inflate::inflateStateCheck(strm as *mut crate::zlib_h::z_stream)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    (strm.total_in, strm.total_out, strm.data_type, strm.adler) = public_fields;
    inflate_back_init_state(strm, state, window, config);
    crate::zlib_h::Z_OK
}
#[export_name = "inflateBackInit_"]

pub unsafe extern "C" fn inflateBackInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter only binds optional caller-owned arguments.
    // The named implementation retains validation and all initialization
    // work, including allocation and persistent state setup.
    let strm = unsafe { strm.as_mut() };
    let window = unsafe { window.as_mut() };
    let version_first = unsafe { version.as_ref().copied() };
    inflateBackInit_(strm, windowBits, window, version_first, stream_size)
}
pub(crate) fn inflateBack(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut in_0: crate::zlib_h::in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: crate::zlib_h::out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    // SAFETY: this decoder is reached only after the ABI adapter has bound
    // the stream. Its callback and cursor protocol is validated at the
    // existing refill, output, and state-check sites below; keeping those raw
    // operations scoped here avoids exposing an unsafe function contract to
    // Rust callers of the implementation.
    unsafe {
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
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
    let mut ret: ::core::ffi::c_int = 0;
    // Keep stream validation in this named implementation. The exported ABI
    // forwarder only binds its foreign stream reference and dispatches here.
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // `inflateBackInit_()` establishes the reciprocal stream/state binding
    // above. Reuse the shared checked binder so this decoder's state setup is
    // reference-bound; the raw callback and window cursors remain below.
    let Some((strm, state_ref)) =
        crate::src::inflate::inflateStateCheck(strm as *mut crate::zlib_h::z_stream)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    left = inflate_back_begin_decode(strm, state_ref);
    next = strm.next_in as *mut ::core::ffi::c_uchar;
    have = inflate_back_initial_input_available(strm, !next.is_null());
    hold = 0 as ::core::ffi::c_ulong;
    bits = 0 as ::core::ffi::c_uint;
    '_inf_leave: loop {
        match inflate_back_decode_mode(state_ref.mode) {
            InflateBackDecodeMode::Type => {
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
                        inflate_back_append_input_byte(&mut hold, &mut bits, *c2rust_fresh0);
                    }
                    if inflate_back_apply_block_header(state_ref, &mut hold, &mut bits) {
                        inflate_back_report_error(
                            strm,
                            state_ref,
                            InflateBackError::InvalidBlockType,
                        );
                    }
                    continue;
                }
            }
            InflateBackDecodeMode::Stored => {
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
                    inflate_back_append_input_byte(&mut hold, &mut bits, *c2rust_fresh1);
                }
                if let Some(length) = inflate_back_stored_length(hold) {
                    inflate_back_begin_stored_copy(state_ref, length, &mut hold, &mut bits);
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
                            if out.expect("non-null function pointer")(out_desc, state_ref.window, left) != 0 {
                                ret = crate::zlib_h::Z_BUF_ERROR;
                                break '_inf_leave;
                            }
                        }
                        copy = inflate_back_stored_copy_count(copy, have, left);
                        // `have` and `copy` bound the callback-provided input
                        // range, while `left` bounds the configured output
                        // window.  Bind the input once; the transfer itself is
                        // checked slice work in `inflate_back_copy_stored_window`.
                        let source = ::core::slice::from_raw_parts(next, copy as usize);
                        inflate_back_copy_stored_window(strm, state_ref, left, source);
                        have = have.wrapping_sub(copy);
                        next = next.wrapping_add(copy as usize);
                        inflate_back_consume_output_copy(state_ref, &mut left, copy);
                    }
                    inflate_back_finish_stored_block(state_ref);
                    continue;
                } else {
                    inflate_back_report_error(
                        strm,
                        state_ref,
                        InflateBackError::InvalidStoredBlockLengths,
                    );
                    continue;
                }
            }
            InflateBackDecodeMode::Table => {
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
                    inflate_back_append_input_byte(&mut hold, &mut bits, *c2rust_fresh2);
                }
                let header = inflate_back_apply_dynamic_header(state_ref, &mut hold, &mut bits);
                if !inflate_back_dynamic_header_is_valid(header) {
                    inflate_back_report_error(
                        strm,
                        state_ref,
                        InflateBackError::TooManyLengthOrDistanceSymbols,
                    );
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
                            inflate_back_append_input_byte(&mut hold, &mut bits, *c2rust_fresh3);
                        }
                        inflate_back_set_code_length_order(
                            &mut state_ref.lens,
                            &mut state_ref.have,
                            hold,
                        );
                        inflate_back_drop_bits(&mut hold, &mut bits, 3);
                    }
                    inflate_back_finish_code_length_order(&mut state_ref.lens, &mut state_ref.have);
                    let code_length_table = inflate_back_code_length_table_spec();
                    inflate_back_prepare_dynamic_table(
                        state_ref,
                        InflateBackDynamicTableTarget::CodeLengths,
                        &code_length_table,
                    );
                    ret = inflate_back_build_dynamic_table(
                        state_ref,
                        InflateBackDynamicTableTarget::CodeLengths,
                        &code_length_table,
                    );
                    if ret != 0 {
                        inflate_back_report_error(
                            strm,
                            state_ref,
                            InflateBackError::InvalidCodeLengthsSet,
                        );
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
                                inflate_back_append_input_byte(
                                    &mut hold,
                                    &mut bits,
                                    *c2rust_fresh6,
                                );
                            }
                            if here.val >= 16 {
                                while bits < inflate_back_code_length_bits_required(here) {
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
                                    inflate_back_append_input_byte(
                                        &mut hold,
                                        &mut bits,
                                        *c2rust_fresh8,
                                    );
                                }
                            }
                            if let Err(error) = inflate_back_apply_code_length(
                                state_ref,
                                here,
                                &mut hold,
                                &mut bits,
                            ) {
                                inflate_back_report_error(strm, state_ref, error);
                                break;
                            }
                        }
                        if state_ref.mode as ::core::ffi::c_uint
                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        if !inflate_back_has_end_code(state_ref) {
                            inflate_back_report_error(
                                strm,
                                state_ref,
                                InflateBackError::MissingEndOfBlock,
                            );
                            continue;
                        } else {
                            let literal_length_table =
                                inflate_back_literal_length_table_spec(state_ref.nlen);
                            inflate_back_prepare_dynamic_table(
                                state_ref,
                                InflateBackDynamicTableTarget::LiteralLengths,
                                &literal_length_table,
                            );
                            ret = inflate_back_build_dynamic_table(
                                state_ref,
                                InflateBackDynamicTableTarget::LiteralLengths,
                                &literal_length_table,
                            );
                            if ret != 0 {
                                inflate_back_report_error(
                                    strm,
                                    state_ref,
                                    InflateBackError::InvalidLiteralLengthsSet,
                                );
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
                                ret = inflate_back_build_dynamic_table(
                                    state_ref,
                                    InflateBackDynamicTableTarget::Distances,
                                    &distance_table,
                                );
                                if ret != 0 {
                                    inflate_back_report_error(
                                        strm,
                                        state_ref,
                                        InflateBackError::InvalidDistancesSet,
                                    );
                                    continue;
                                } else {
                                    inflate_back_finish_dynamic_tables(state_ref);
                                }
                            }
                        }
                    }
                }
            }
            InflateBackDecodeMode::Length => {}
            InflateBackDecodeMode::Terminal(status) => {
                ret = status;
                break;
            }
        }
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
                inflate_back_append_input_byte(&mut hold, &mut bits, *c2rust_fresh13);
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
                    inflate_back_append_input_byte(&mut hold, &mut bits, *c2rust_fresh14);
                }
                inflate_back_drop_bits(&mut hold, &mut bits, last.bits as ::core::ffi::c_uint);
            }
            inflate_back_drop_bits(&mut hold, &mut bits, here.bits as ::core::ffi::c_uint);
            match inflate_back_start_length_code(state_ref, here) {
                InflateBackLengthCode::Literal => {
                    if inflate_back_prepare_output_window(state_ref, &mut left) {
                        if out.expect("non-null function pointer")(out_desc, state_ref.window, left) != 0 {
                            ret = crate::zlib_h::Z_BUF_ERROR;
                            break;
                        }
                    }
                    inflate_back_write_literal(
                        strm,
                        state_ref,
                        left,
                        inflate_back_literal_byte(state_ref),
                    );
                    inflate_back_commit_literal(state_ref, &mut left);
                }
                InflateBackLengthCode::End => {
                    inflate_back_finish_end_code(state_ref);
                }
                InflateBackLengthCode::Invalid => {
                    inflate_back_report_error(
                        strm,
                        state_ref,
                        InflateBackError::InvalidLiteralLengthCode,
                    );
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
                            inflate_back_append_input_byte(
                                &mut hold,
                                &mut bits,
                                *c2rust_fresh16,
                            );
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
                        inflate_back_append_input_byte(&mut hold, &mut bits, *c2rust_fresh17);
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
                            inflate_back_append_input_byte(
                                &mut hold,
                                &mut bits,
                                *c2rust_fresh18,
                            );
                        }
                        inflate_back_drop_bits(
                            &mut hold,
                            &mut bits,
                            last.bits as ::core::ffi::c_uint,
                        );
                    }
                    inflate_back_drop_bits(&mut hold, &mut bits, here.bits as ::core::ffi::c_uint);
                    match inflate_back_start_distance_code(state_ref, here) {
                        InflateBackDistanceCode::Invalid => {
                            inflate_back_report_error(
                                strm,
                                state_ref,
                                InflateBackError::InvalidDistanceCode,
                            );
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
                                    inflate_back_append_input_byte(
                                        &mut hold,
                                        &mut bits,
                                        *c2rust_fresh19,
                                    );
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
                                inflate_back_report_error(
                                    strm,
                                    state_ref,
                                    InflateBackError::InvalidDistanceTooFarBack,
                                );
                            } else {
                                loop {
                                    if inflate_back_prepare_output_window(state_ref, &mut left) {
                                        if out.expect("non-null function pointer")(
                                            out_desc, state_ref.window, left,
                                        ) != 0
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
                                    // The distance check above establishes both the
                                    // source and destination ranges in the active
                                    // window. Borrow that already-configured window
                                    // through the inflater's bounded access path so
                                    // overlapping LZ copies need no raw dereferences.
                                    let write_index =
                                        state_ref.wsize.wrapping_sub(left) as usize;
                                    crate::src::inflate::updatewindow(
                                        strm,
                                        state_ref,
                                        crate::src::inflate::InflateWindowAccess::Existing,
                                        |_, window| {
                                            inflate_back_copy_match_window(
                                                window.expect(
                                                    "inflateBack has a configured output window",
                                                ),
                                                write_index,
                                                match_copy,
                                            );
                                        },
                                    )
                                    .expect("inflateBack existing window access cannot fail");
                                    inflate_back_consume_output_copy(
                                        state_ref,
                                        &mut left,
                                        match_copy.count,
                                    );
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
    if let Some(pending) = inflate_back_pending_output(state_ref.wsize, left) {
        let output_failed =
            out.expect("non-null function pointer")(out_desc, state_ref.window, pending) != 0;
        ret = inflate_back_finish_pending_output(ret, output_failed);
    }
    strm.next_in = next as *mut crate::stdlib::Bytef;
    inflate_back_publish_available_input(strm, have);
    return ret;
    }
}
#[export_name = "inflateBack"]

pub unsafe extern "C" fn inflateBack_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut in_0: crate::zlib_h::in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: crate::zlib_h::out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter only binds the optional foreign stream
    // reference. The implementation retains callback validation and all
    // callback-owned cursor handling.
    let strm = unsafe { strm.as_mut() };
    inflateBack(strm, in_0, in_desc, out, out_desc)
}
// The ABI forwarder only binds the foreign stream reference. Keep validation
// and the post-release transition reference-bound; only the configured C
// deallocator remains an unsafe boundary here.
fn inflate_back_end_can_release(strm: &crate::zlib_h::z_stream) -> bool {
    !strm.state.is_null() && strm.zfree.is_some()
}

fn inflate_back_end_complete(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    crate::zlib_h::Z_OK
}

pub fn inflateBackEnd(strm: Option<&mut crate::zlib_h::z_stream>) -> ::core::ffi::c_int {
    // As with inflateBack(), the export binds the foreign reference and this
    // implementation owns validation and teardown decisions.
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_back_end_can_release(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // A user deallocator may re-enter unrelated code, so capture all values
    // it receives before crossing that callback boundary. In particular, do
    // not read the stream-owned callback or allocation through `strm` while
    // the callback is active.
    let opaque = strm.opaque;
    let state = strm.state as crate::stdlib::voidpf;
    let zfree = strm.zfree.expect("non-null function pointer");
    Some(zfree).expect("non-null function pointer")(opaque, state);
    inflate_back_end_complete(strm)
}
#[export_name = "inflateBackEnd"]

pub unsafe extern "C" fn inflateBackEnd_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter only binds the optional foreign stream
    // reference before the reference-based teardown dispatcher validates it.
    let strm = unsafe { strm.as_mut() };
    inflateBackEnd(strm)
}
