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

fn bit_mask(bit_count: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    ((1 as ::core::ffi::c_uint) << bit_count).wrapping_sub(1 as ::core::ffi::c_uint)
}

fn low_bits(hold: ::core::ffi::c_ulong, bit_count: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    hold as ::core::ffi::c_uint & bit_mask(bit_count)
}

fn consume_bits(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    bit_count: ::core::ffi::c_uint,
) -> (::core::ffi::c_ulong, ::core::ffi::c_uint) {
    (hold >> bit_count, bits.wrapping_sub(bit_count))
}

fn add_and_consume_extra_bits(
    value: ::core::ffi::c_uint,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
    extra_bits: ::core::ffi::c_uint,
) -> (
    ::core::ffi::c_uint,
    crate::stdlib::uLong,
    ::core::ffi::c_uint,
) {
    let value = value.wrapping_add(low_bits(hold, extra_bits));
    let (hold, bits) = consume_bits(hold, bits, extra_bits);
    (value, hold, bits)
}

fn append_input_byte(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    byte: ::core::ffi::c_uchar,
) -> (::core::ffi::c_ulong, ::core::ffi::c_uint) {
    (
        hold.wrapping_add((byte as ::core::ffi::c_ulong) << bits),
        bits.wrapping_add(8 as ::core::ffi::c_uint),
    )
}

fn refill_input_byte(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    input_remaining: crate::stdlib::uInt,
    byte: ::core::ffi::c_uchar,
) -> (
    ::core::ffi::c_ulong,
    ::core::ffi::c_uint,
    crate::stdlib::uInt,
) {
    let (hold, bits) = append_input_byte(hold, bits, byte);
    (hold, bits, input_remaining_after_read(input_remaining))
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FastCodeEntry {
    op: ::core::ffi::c_uint,
    bits: ::core::ffi::c_uint,
    value: ::core::ffi::c_uint,
}

fn fast_code_entry(entry: code) -> FastCodeEntry {
    FastCodeEntry {
        op: entry.op as ::core::ffi::c_uint,
        bits: entry.bits as ::core::ffi::c_uint,
        value: entry.val as ::core::ffi::c_uint,
    }
}

fn subtable_index(entry: FastCodeEntry, hold: ::core::ffi::c_ulong) -> usize {
    entry.value as usize + (hold & bit_mask(entry.op) as ::core::ffi::c_ulong) as usize
}

fn table_index(hold: ::core::ffi::c_ulong, mask: ::core::ffi::c_uint) -> usize {
    (hold & mask as ::core::ffi::c_ulong) as usize
}

fn unread_input_state(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    input_remaining: crate::stdlib::uInt,
) -> (
    ::core::ffi::c_ulong,
    ::core::ffi::c_uint,
    ::core::ffi::c_uint,
    crate::stdlib::uInt,
) {
    let unread_bytes = bits >> 3 as ::core::ffi::c_int;
    let unread_bits = bits.wrapping_sub(unread_bytes << 3 as ::core::ffi::c_int);
    (
        hold & bit_mask(unread_bits) as ::core::ffi::c_ulong,
        unread_bits,
        unread_bytes,
        input_remaining.wrapping_add(unread_bytes as crate::stdlib::uInt),
    )
}

/// The scalar portion of the fast-path commit after backing up bytes still
/// held in the bit buffer.  Keeping this independent of stream pointers is a
/// small but important boundary: the eventual slice-based fast decoder can
/// use the same commit without reconstructing raw input cursors.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FastPathCommit {
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    unread_bytes: ::core::ffi::c_uint,
    input_remaining: crate::stdlib::uInt,
}

fn fast_path_commit(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    input_remaining: crate::stdlib::uInt,
) -> FastPathCommit {
    let (hold, bits, unread_bytes, input_remaining) =
        unread_input_state(hold, bits, input_remaining);
    FastPathCommit {
        hold,
        bits,
        unread_bytes,
        input_remaining,
    }
}

fn input_remaining_after_read(input_remaining: crate::stdlib::uInt) -> crate::stdlib::uInt {
    input_remaining.wrapping_sub(1)
}

fn input_bytes_needed(
    bits: ::core::ffi::c_uint,
    required_bits: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let missing_bits = required_bits.saturating_sub(bits);
    missing_bits / 8 + (missing_bits % 8 != 0) as ::core::ffi::c_uint
}

fn fast_decode_needs_prefetch(bits: ::core::ffi::c_uint) -> bool {
    bits < 15 as ::core::ffi::c_uint
}

fn fast_decode_prefetch_byte_count(bits: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    if fast_decode_needs_prefetch(bits) {
        2
    } else {
        0
    }
}

fn fast_length_extra_bits_refill_byte_count(
    bits: ::core::ffi::c_uint,
    extra_bits: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    if extra_bits != 0 && bits < extra_bits {
        1
    } else {
        0
    }
}

fn output_cursor_after_write(
    output_produced: crate::stdlib::uInt,
    output_remaining: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::stdlib::uInt) {
    output_cursor_after_writes(output_produced, output_remaining, 1)
}

fn output_cursor_after_writes(
    output_produced: crate::stdlib::uInt,
    output_remaining: crate::stdlib::uInt,
    write_count: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::stdlib::uInt) {
    (
        output_produced.wrapping_add(write_count),
        output_remaining.wrapping_sub(write_count),
    )
}

fn output_produced_at_fast_path_start(
    start: crate::stdlib::uInt,
    output_remaining: crate::stdlib::uInt,
) -> crate::stdlib::uInt {
    start.wrapping_sub(output_remaining)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FastLitLenAction {
    Literal,
    Length { extra_bits: ::core::ffi::c_uint },
    Subtable,
    End,
    Invalid,
}

fn fast_litlen_action(op: ::core::ffi::c_uint) -> FastLitLenAction {
    if op == 0 {
        FastLitLenAction::Literal
    } else if op & 16 != 0 {
        FastLitLenAction::Length {
            extra_bits: op & 15,
        }
    } else if op & 64 == 0 {
        FastLitLenAction::Subtable
    } else if op & 32 != 0 {
        FastLitLenAction::End
    } else {
        FastLitLenAction::Invalid
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FastDistAction {
    Distance { extra_bits: ::core::ffi::c_uint },
    Subtable,
    Invalid,
}

fn fast_dist_action(op: ::core::ffi::c_uint) -> FastDistAction {
    if op & 16 != 0 {
        FastDistAction::Distance {
            extra_bits: op & 15,
        }
    } else if op & 64 == 0 {
        FastDistAction::Subtable
    } else {
        FastDistAction::Invalid
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FastDecodeError {
    InvalidDistanceCode,
    DistanceTooFarBack,
    InvalidLiteralLengthCode,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FastDecodeFailure {
    error: FastDecodeError,
}

fn fast_decode_failure(error: FastDecodeError) -> FastDecodeFailure {
    FastDecodeFailure { error }
}

fn fast_decode_error_message(error: FastDecodeError) -> &'static [u8] {
    match error {
        FastDecodeError::InvalidDistanceCode => b"invalid distance code\0",
        FastDecodeError::DistanceTooFarBack => b"invalid distance too far back\0",
        FastDecodeError::InvalidLiteralLengthCode => b"invalid literal/length code\0",
    }
}

fn fast_window_distance_is_invalid(
    distance_from_window: ::core::ffi::c_uint,
    window_available: ::core::ffi::c_uint,
    sane: bool,
) -> bool {
    distance_from_window > window_available && sane
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FastWindowDistance {
    Valid { distance_back: ::core::ffi::c_uint },
    Invalid,
}

fn validate_fast_window_distance(
    distance: ::core::ffi::c_uint,
    output_produced: crate::stdlib::uInt,
    window_available: ::core::ffi::c_uint,
    sane: bool,
) -> FastWindowDistance {
    let distance_back = distance.wrapping_sub(output_produced);
    if fast_window_distance_is_invalid(distance_back, window_available, sane) {
        FastWindowDistance::Invalid
    } else {
        FastWindowDistance::Valid { distance_back }
    }
}

fn fast_window_distance_back(
    distance: ::core::ffi::c_uint,
    output_produced: crate::stdlib::uInt,
    window_available: ::core::ffi::c_uint,
    sane: bool,
) -> Result<::core::ffi::c_uint, FastDecodeFailure> {
    match validate_fast_window_distance(distance, output_produced, window_available, sane) {
        FastWindowDistance::Valid { distance_back } => Ok(distance_back),
        FastWindowDistance::Invalid => {
            Err(fast_decode_failure(FastDecodeError::DistanceTooFarBack))
        }
    }
}

fn fast_match_uses_window(
    distance: ::core::ffi::c_uint,
    output_produced: crate::stdlib::uInt,
) -> bool {
    distance > output_produced
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FastDistanceSource {
    Output,
    Window,
}

fn fast_distance_requires_window_copy(source: FastDistanceSource) -> bool {
    source == FastDistanceSource::Window
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FastDistance {
    distance: ::core::ffi::c_uint,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
    source: FastDistanceSource,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FastWindowContinuationSource {
    Window,
    Output,
}

fn fast_window_copy_continues_from_output(
    continuation_source: FastWindowContinuationSource,
) -> bool {
    continuation_source == FastWindowContinuationSource::Output
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FastWindowCopyPlan {
    first_window_start: ::core::ffi::c_uint,
    first_window_length: ::core::ffi::c_uint,
    wrap_window_start_length: Option<::core::ffi::c_uint>,
    remaining_length: ::core::ffi::c_uint,
    continuation_source: FastWindowContinuationSource,
}

fn fast_window_copy_plan(
    window_size: ::core::ffi::c_uint,
    window_next: ::core::ffi::c_uint,
    distance_back: ::core::ffi::c_uint,
    match_length: ::core::ffi::c_uint,
) -> FastWindowCopyPlan {
    if window_next == 0 {
        let first_window_start = window_size.wrapping_sub(distance_back);
        if distance_back < match_length {
            FastWindowCopyPlan {
                first_window_start,
                first_window_length: distance_back,
                wrap_window_start_length: None,
                remaining_length: match_length.wrapping_sub(distance_back),
                continuation_source: FastWindowContinuationSource::Output,
            }
        } else {
            FastWindowCopyPlan {
                first_window_start,
                first_window_length: 0,
                wrap_window_start_length: None,
                remaining_length: match_length,
                continuation_source: FastWindowContinuationSource::Window,
            }
        }
    } else if window_next < distance_back {
        let first_window_start = window_size
            .wrapping_add(window_next)
            .wrapping_sub(distance_back);
        let first_window_length = distance_back.wrapping_sub(window_next);
        if first_window_length < match_length {
            let remaining_length = match_length.wrapping_sub(first_window_length);
            if window_next < remaining_length {
                FastWindowCopyPlan {
                    first_window_start,
                    first_window_length,
                    wrap_window_start_length: Some(window_next),
                    remaining_length: remaining_length.wrapping_sub(window_next),
                    continuation_source: FastWindowContinuationSource::Output,
                }
            } else {
                FastWindowCopyPlan {
                    first_window_start,
                    first_window_length,
                    wrap_window_start_length: Some(0),
                    remaining_length,
                    continuation_source: FastWindowContinuationSource::Window,
                }
            }
        } else {
            FastWindowCopyPlan {
                first_window_start,
                first_window_length: 0,
                wrap_window_start_length: None,
                remaining_length: match_length,
                continuation_source: FastWindowContinuationSource::Window,
            }
        }
    } else {
        let first_window_start = window_next.wrapping_sub(distance_back);
        if distance_back < match_length {
            FastWindowCopyPlan {
                first_window_start,
                first_window_length: distance_back,
                wrap_window_start_length: None,
                remaining_length: match_length.wrapping_sub(distance_back),
                continuation_source: FastWindowContinuationSource::Output,
            }
        } else {
            FastWindowCopyPlan {
                first_window_start,
                first_window_length: 0,
                wrap_window_start_length: None,
                remaining_length: match_length,
                continuation_source: FastWindowContinuationSource::Window,
            }
        }
    }
}

fn match_copy_layout(
    match_length: ::core::ffi::c_uint,
) -> (::core::ffi::c_uint, ::core::ffi::c_uint) {
    (match_length / 3, match_length % 3)
}

fn trailing_match_copy_byte_count(trailing_bytes: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    trailing_bytes.min(2)
}

fn trailing_match_copy_needs_second_byte(trailing_copy_byte_count: ::core::ffi::c_uint) -> bool {
    trailing_copy_byte_count > 1
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FastMatchCopyLayout {
    final_copy_triplets: ::core::ffi::c_uint,
    final_trailing_bytes: ::core::ffi::c_uint,
    window_copy_plan: Option<FastWindowCopyPlan>,
}

fn fast_match_copy_layout(
    uses_window: bool,
    window_size: ::core::ffi::c_uint,
    window_next: ::core::ffi::c_uint,
    distance_back: ::core::ffi::c_uint,
    match_length: ::core::ffi::c_uint,
) -> FastMatchCopyLayout {
    let window_copy_plan = uses_window
        .then(|| fast_window_copy_plan(window_size, window_next, distance_back, match_length));
    let final_copy_length = window_copy_plan
        .map(|copy_plan| copy_plan.remaining_length)
        .unwrap_or(match_length);
    let (final_copy_triplets, final_trailing_bytes) = match_copy_layout(final_copy_length);

    FastMatchCopyLayout {
        final_copy_triplets,
        final_trailing_bytes,
        window_copy_plan,
    }
}

/// Copy a match whose source is already in the current output buffer.
///
/// This deliberately copies one byte at a time. When the match exceeds its
/// distance, each byte just written is source for the next byte, which is
/// deflate's required overlapping-copy behavior.
fn fast_copy_from_output(
    output: &mut [crate::stdlib::Bytef],
    output_produced: crate::stdlib::uInt,
    distance: ::core::ffi::c_uint,
    match_length: ::core::ffi::c_uint,
) -> Option<()> {
    let write_start = usize::try_from(output_produced).ok()?;
    let distance = usize::try_from(distance).ok()?;
    let match_length = usize::try_from(match_length).ok()?;
    if distance == 0 || distance > write_start {
        return None;
    }
    let write_end = write_start.checked_add(match_length)?;
    if write_end > output.len() {
        return None;
    }

    for offset in 0..match_length {
        let source = write_start.checked_sub(distance)?.checked_add(offset)?;
        let destination = write_start.checked_add(offset)?;
        let byte = *output.get(source)?;
        *output.get_mut(destination)? = byte;
    }
    Some(())
}

fn finish_fast_distance(
    base_distance: ::core::ffi::c_uint,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
    extra_bits: ::core::ffi::c_uint,
    output_produced: crate::stdlib::uInt,
) -> FastDistance {
    let (distance, hold, bits) = add_and_consume_extra_bits(base_distance, hold, bits, extra_bits);
    let source = if fast_match_uses_window(distance, output_produced) {
        FastDistanceSource::Window
    } else {
        FastDistanceSource::Output
    };
    FastDistance {
        distance,
        hold,
        bits,
        source,
    }
}

pub unsafe extern "C" fn inflate_fast(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    let mut in_0: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut input_remaining: crate::stdlib::uInt = 0;
    let mut out: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut output_produced: crate::stdlib::uInt = 0;
    let mut output_remaining: crate::stdlib::uInt = 0;
    let mut wsize: ::core::ffi::c_uint = 0;
    let mut whave: ::core::ffi::c_uint = 0;
    let mut wnext: ::core::ffi::c_uint = 0;
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut lcode = crate::src::inflate::DecodeTableLocation::dynamic(0);
    let mut dcode = crate::src::inflate::DecodeTableLocation::dynamic(0);
    let mut lmask: ::core::ffi::c_uint = 0;
    let mut dmask: ::core::ffi::c_uint = 0;
    let mut here = crate::src::inflate::INVALID_DECODE_CODE;
    let mut op: ::core::ffi::c_uint = 0;
    let mut len: ::core::ffi::c_uint = 0;
    let mut dist: ::core::ffi::c_uint = 0;
    let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut pending_failure: Option<FastDecodeFailure> = None;
    let state = &mut *((*strm).state as *mut crate::src::inflate::inflate_state);
    in_0 = (*strm).next_in as *mut ::core::ffi::c_uchar;
    input_remaining = (*strm).avail_in;
    out = (*strm).next_out as *mut ::core::ffi::c_uchar;
    output_produced =
        output_produced_at_fast_path_start(start as crate::stdlib::uInt, (*strm).avail_out);
    output_remaining = (*strm).avail_out;
    wsize = state.wsize;
    whave = state.whave;
    wnext = state.wnext;
    window = state.window;
    hold = state.hold;
    bits = state.bits;
    lcode = state.lencode;
    dcode = state.distcode;
    lmask = bit_mask(state.lenbits);
    dmask = bit_mask(state.distbits);
    let mut c2rust_current_block_141: u64;
    's_94: loop {
        for _ in 0..fast_decode_prefetch_byte_count(bits) {
            let input_byte = in_0;
            in_0 = in_0.wrapping_add(1);
            (hold, bits, input_remaining) =
                refill_input_byte(hold, bits, input_remaining, *input_byte);
        }
        here =
            crate::src::inflate::inflate_decode_table_entry(state, lcode, table_index(hold, lmask));
        loop {
            let entry = fast_code_entry(here);
            op = entry.bits;
            (hold, bits) = consume_bits(hold, bits, op);
            op = entry.op;
            match fast_litlen_action(op) {
                FastLitLenAction::Literal => {
                    let c2rust_fresh2 = out;
                    out = out.wrapping_add(1);
                    (output_produced, output_remaining) =
                        output_cursor_after_write(output_produced, output_remaining);
                    *c2rust_fresh2 = entry.value as ::core::ffi::c_uchar;
                    c2rust_current_block_141 = 5689001924483802034;
                    break;
                }
                FastLitLenAction::Length { extra_bits } => {
                    len = entry.value;
                    if extra_bits != 0 {
                        for _ in 0..fast_length_extra_bits_refill_byte_count(bits, extra_bits) {
                            let c2rust_fresh3 = in_0;
                            in_0 = in_0.wrapping_add(1);
                            (hold, bits, input_remaining) =
                                refill_input_byte(hold, bits, input_remaining, *c2rust_fresh3);
                        }
                        (len, hold, bits) = add_and_consume_extra_bits(len, hold, bits, extra_bits);
                    }
                    for _ in 0..fast_decode_prefetch_byte_count(bits) {
                        let input_byte = in_0;
                        in_0 = in_0.wrapping_add(1);
                        (hold, bits, input_remaining) =
                            refill_input_byte(hold, bits, input_remaining, *input_byte);
                    }
                    here = crate::src::inflate::inflate_decode_table_entry(
                        state,
                        dcode,
                        table_index(hold, dmask),
                    );
                    c2rust_current_block_141 = 3217834059723038609;
                    break;
                }
                FastLitLenAction::Subtable => {
                    here = crate::src::inflate::inflate_decode_table_entry(
                        state,
                        lcode,
                        subtable_index(entry, hold),
                    );
                }
                FastLitLenAction::End => {
                    c2rust_current_block_141 = 13505557363059842426;
                    break;
                }
                FastLitLenAction::Invalid => {
                    c2rust_current_block_141 = 9180031981464905198;
                    break;
                }
            }
        }
        match c2rust_current_block_141 {
            3217834059723038609 => {
                loop {
                    let entry = fast_code_entry(here);
                    op = entry.bits;
                    (hold, bits) = consume_bits(hold, bits, op);
                    op = entry.op;
                    match fast_dist_action(op) {
                        FastDistAction::Distance { extra_bits } => {
                            dist = entry.value;
                            for _ in 0..input_bytes_needed(bits, extra_bits) {
                                let c2rust_fresh6 = in_0;
                                in_0 = in_0.wrapping_add(1);
                                (hold, bits, input_remaining) =
                                    refill_input_byte(hold, bits, input_remaining, *c2rust_fresh6);
                            }
                            let distance =
                                finish_fast_distance(dist, hold, bits, extra_bits, output_produced);
                            dist = distance.distance;
                            hold = distance.hold;
                            bits = distance.bits;
                            if fast_distance_requires_window_copy(distance.source) {
                                c2rust_current_block_141 = 5235537862154438448;
                                break;
                            } else {
                                c2rust_current_block_141 = 6072622540298447352;
                                break;
                            }
                        }
                        FastDistAction::Subtable => {
                            here = crate::src::inflate::inflate_decode_table_entry(
                                state,
                                dcode,
                                subtable_index(entry, hold),
                            );
                        }
                        FastDistAction::Invalid => {
                            pending_failure =
                                Some(fast_decode_failure(FastDecodeError::InvalidDistanceCode));
                            break 's_94;
                        }
                    }
                }
                match c2rust_current_block_141 {
                    6072622540298447352 => {
                        from = out.wrapping_sub(dist as usize);
                        let copy_layout = fast_match_copy_layout(false, 0, 0, 0, len);
                        for _ in 0..copy_layout.final_copy_triplets {
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
                            (output_produced, output_remaining) =
                                output_cursor_after_writes(output_produced, output_remaining, 3);
                        }
                        let trailing_copy_byte_count =
                            trailing_match_copy_byte_count(copy_layout.final_trailing_bytes);
                        if trailing_copy_byte_count != 0 {
                            let c2rust_fresh32 = from;
                            from = from.wrapping_add(1);
                            let c2rust_fresh33 = out;
                            out = out.wrapping_add(1);
                            (output_produced, output_remaining) =
                                output_cursor_after_write(output_produced, output_remaining);
                            *c2rust_fresh33 = *c2rust_fresh32;
                            if trailing_match_copy_needs_second_byte(trailing_copy_byte_count) {
                                let c2rust_fresh34 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh35 = out;
                                out = out.wrapping_add(1);
                                (output_produced, output_remaining) =
                                    output_cursor_after_write(output_produced, output_remaining);
                                *c2rust_fresh35 = *c2rust_fresh34;
                            }
                        }
                    }
                    _ => {
                        let distance_back = match fast_window_distance_back(
                            dist,
                            output_produced,
                            whave,
                            state.sane != 0,
                        ) {
                            Ok(distance_back) => distance_back,
                            Err(failure) => {
                                pending_failure = Some(failure);
                                break;
                            }
                        };
                        let copy_layout =
                            fast_match_copy_layout(true, wsize, wnext, distance_back, len);
                        let copy_plan = copy_layout.window_copy_plan.unwrap();
                        from = window.wrapping_add(copy_plan.first_window_start as usize);
                        if copy_plan.first_window_length != 0 {
                            op = copy_plan.first_window_length;
                            loop {
                                let c2rust_fresh8 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh9 = out;
                                out = out.wrapping_add(1);
                                (output_produced, output_remaining) =
                                    output_cursor_after_write(output_produced, output_remaining);
                                *c2rust_fresh9 = *c2rust_fresh8;
                                op = op.wrapping_sub(1);
                                if !(op != 0) {
                                    break;
                                }
                            }
                        }
                        if let Some(wrap_window_start_length) = copy_plan.wrap_window_start_length {
                            from = window;
                            if wrap_window_start_length != 0 {
                                op = wrap_window_start_length;
                                loop {
                                    let c2rust_fresh10 = from;
                                    from = from.wrapping_add(1);
                                    let c2rust_fresh11 = out;
                                    out = out.wrapping_add(1);
                                    (output_produced, output_remaining) = output_cursor_after_write(
                                        output_produced,
                                        output_remaining,
                                    );
                                    *c2rust_fresh11 = *c2rust_fresh10;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                            }
                        }
                        len = copy_plan.remaining_length;
                        if fast_window_copy_continues_from_output(copy_plan.continuation_source) {
                            from = out.wrapping_sub(dist as usize);
                        }
                        for _ in 0..copy_layout.final_copy_triplets {
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
                            (output_produced, output_remaining) =
                                output_cursor_after_writes(output_produced, output_remaining, 3);
                        }
                        let trailing_copy_byte_count =
                            trailing_match_copy_byte_count(copy_layout.final_trailing_bytes);
                        if trailing_copy_byte_count != 0 {
                            let c2rust_fresh22 = from;
                            from = from.wrapping_add(1);
                            let c2rust_fresh23 = out;
                            out = out.wrapping_add(1);
                            (output_produced, output_remaining) =
                                output_cursor_after_write(output_produced, output_remaining);
                            *c2rust_fresh23 = *c2rust_fresh22;
                            if trailing_match_copy_needs_second_byte(trailing_copy_byte_count) {
                                let c2rust_fresh24 = from;
                                from = from.wrapping_add(1);
                                let c2rust_fresh25 = out;
                                out = out.wrapping_add(1);
                                (output_produced, output_remaining) =
                                    output_cursor_after_write(output_produced, output_remaining);
                                *c2rust_fresh25 = *c2rust_fresh24;
                            }
                        }
                    }
                }
            }
            9180031981464905198 => {
                pending_failure = Some(fast_decode_failure(
                    FastDecodeError::InvalidLiteralLengthCode,
                ));
                break;
            }
            13505557363059842426 => {
                state.mode = crate::src::inflate::TYPE;
                break;
            }
            _ => {}
        }
        if !crate::src::inflate::inflate_can_use_fast_path(input_remaining, output_remaining) {
            break;
        }
    }
    if let Some(failure) = pending_failure {
        (*strm).msg = fast_decode_error_message(failure.error).as_ptr()
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        state.mode = crate::src::inflate::BAD;
    }
    let commit = fast_path_commit(hold, bits, input_remaining);
    in_0 = in_0.wrapping_sub(commit.unread_bytes as usize);
    (*strm).next_in = in_0 as *mut crate::stdlib::Bytef;
    (*strm).next_out = out as *mut crate::stdlib::Bytef;
    (*strm).avail_in = commit.input_remaining;
    (*strm).avail_out = output_remaining;
    state.hold = commit.hold;
    state.bits = commit.bits;
}
#[export_name = "inflate_fast"]

pub unsafe extern "C" fn inflate_fast_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut start: ::core::ffi::c_uint,
) {
    inflate_fast(strm, start)
}
#[cfg(test)]
mod tests {
    use super::{
        add_and_consume_extra_bits, append_input_byte, bit_mask, code, consume_bits,
        fast_code_entry, fast_copy_from_output, fast_decode_error_message, fast_decode_failure,
        fast_decode_needs_prefetch, fast_decode_prefetch_byte_count, fast_dist_action,
        fast_distance_requires_window_copy, fast_length_extra_bits_refill_byte_count,
        fast_litlen_action, fast_match_copy_layout, fast_match_uses_window, fast_path_commit,
        fast_window_copy_plan, fast_window_distance_back, fast_window_distance_is_invalid,
        finish_fast_distance, input_bytes_needed, input_remaining_after_read, low_bits,
        output_cursor_after_write, output_cursor_after_writes, output_produced_at_fast_path_start,
        refill_input_byte, subtable_index, table_index, trailing_match_copy_byte_count,
        trailing_match_copy_needs_second_byte, unread_input_state, validate_fast_window_distance,
        FastCodeEntry, FastDecodeError, FastDecodeFailure, FastDistAction, FastDistance,
        FastDistanceSource, FastLitLenAction, FastMatchCopyLayout, FastWindowContinuationSource,
        FastWindowCopyPlan, FastWindowDistance,
    };

    #[test]
    fn bit_mask_selects_requested_low_bits() {
        assert_eq!(bit_mask(0), 0);
        assert_eq!(bit_mask(1), 1);
        assert_eq!(bit_mask(5), 0b1_1111);
        assert_eq!(bit_mask(15), 0x7fff);
    }

    #[test]
    fn fast_code_entry_snapshots_code_fields() {
        let entry = code {
            op: 0b1_0101,
            bits: 7,
            val: 123,
        };

        assert_eq!(
            fast_code_entry(entry),
            FastCodeEntry {
                op: 0b1_0101,
                bits: 7,
                value: 123,
            }
        );
    }

    #[test]
    fn fast_decode_error_messages_are_exact_nul_terminated_strings() {
        assert_eq!(
            fast_decode_error_message(FastDecodeError::InvalidDistanceCode),
            b"invalid distance code\0"
        );
        assert_eq!(
            fast_decode_error_message(FastDecodeError::DistanceTooFarBack),
            b"invalid distance too far back\0"
        );
        assert_eq!(
            fast_decode_error_message(FastDecodeError::InvalidLiteralLengthCode),
            b"invalid literal/length code\0"
        );
    }

    #[test]
    fn fast_decode_failure_preserves_the_selected_error() {
        assert_eq!(
            fast_decode_failure(FastDecodeError::DistanceTooFarBack),
            FastDecodeFailure {
                error: FastDecodeError::DistanceTooFarBack,
            }
        );
    }

    #[test]
    fn fast_window_distance_validation_preserves_strict_sane_rule() {
        assert!(!fast_window_distance_is_invalid(4, 4, true));
        assert!(!fast_window_distance_is_invalid(3, 4, true));
        assert!(fast_window_distance_is_invalid(5, 4, true));
        assert!(!fast_window_distance_is_invalid(5, 4, false));
    }

    #[test]
    fn fast_window_distance_validation_preserves_wrapping_and_error_boundaries() {
        assert_eq!(
            validate_fast_window_distance(9, 4, 5, true),
            FastWindowDistance::Valid { distance_back: 5 }
        );
        assert_eq!(
            validate_fast_window_distance(10, 4, 5, true),
            FastWindowDistance::Invalid
        );
        assert_eq!(
            validate_fast_window_distance(10, 4, 5, false),
            FastWindowDistance::Valid { distance_back: 6 }
        );
        assert_eq!(
            validate_fast_window_distance(0, 1, ::core::ffi::c_uint::MAX, true),
            FastWindowDistance::Valid {
                distance_back: ::core::ffi::c_uint::MAX
            }
        );
    }

    #[test]
    fn fast_match_uses_window_only_beyond_produced_output() {
        assert!(!fast_match_uses_window(4, 4));
        assert!(!fast_match_uses_window(3, 4));
        assert!(fast_match_uses_window(5, 4));
    }

    #[test]
    fn fast_distance_requires_window_copy_only_for_window_source() {
        assert!(!fast_distance_requires_window_copy(
            FastDistanceSource::Output
        ));
        assert!(fast_distance_requires_window_copy(
            FastDistanceSource::Window
        ));
    }

    #[test]
    fn fast_window_copy_continues_from_output_only_for_output_source() {
        assert!(!super::fast_window_copy_continues_from_output(
            FastWindowContinuationSource::Window
        ));
        assert!(super::fast_window_copy_continues_from_output(
            FastWindowContinuationSource::Output
        ));
    }

    #[test]
    fn fast_distance_finalization_consumes_bits_and_selects_copy_source() {
        assert_eq!(
            finish_fast_distance(4, 0x12, 8, 0, 4),
            FastDistance {
                distance: 4,
                hold: 0x12,
                bits: 8,
                source: FastDistanceSource::Output,
            }
        );
        assert_eq!(
            finish_fast_distance(7, 0b1011_0101, 8, 3, 11),
            FastDistance {
                distance: 12,
                hold: 0b1_0110,
                bits: 5,
                source: FastDistanceSource::Window,
            }
        );
    }

    #[test]
    fn fast_distance_finalization_preserves_strict_boundary_and_wrapping() {
        assert_eq!(
            finish_fast_distance(5, 0, 0, 0, 5).source,
            FastDistanceSource::Output
        );
        assert_eq!(
            finish_fast_distance(6, 0, 0, 0, 5).source,
            FastDistanceSource::Window
        );
        assert_eq!(
            finish_fast_distance(::core::ffi::c_uint::MAX, 1, 0, 1, 0),
            FastDistance {
                distance: 0,
                hold: 0,
                bits: ::core::ffi::c_uint::MAX,
                source: FastDistanceSource::Output,
            }
        );
    }

    #[test]
    fn low_bits_masks_the_low_word_of_the_bit_buffer() {
        assert_eq!(low_bits(0xffff_ffff_0000_001b, 5), 0x1b);
        assert_eq!(low_bits(0xfeed, 0), 0);
        assert_eq!(low_bits(0xfeed, 15), 0x7eed);
    }

    #[test]
    fn consume_bits_discards_low_bits_and_updates_count() {
        assert_eq!(consume_bits(0b1011_0101, 8, 3), (0b1_0110, 5));
    }

    #[test]
    fn consume_bits_preserves_zero_count_and_wrapping_subtraction() {
        assert_eq!(consume_bits(0xfeed, 9, 0), (0xfeed, 9));
        assert_eq!(consume_bits(1, 0, 1), (0, ::core::ffi::c_uint::MAX));
    }

    #[test]
    fn extra_bits_update_preserves_value_and_bit_count_wrapping() {
        assert_eq!(
            add_and_consume_extra_bits(7, 0b1011_0101, 8, 3),
            (12, 0b1_0110, 5)
        );
        assert_eq!(
            add_and_consume_extra_bits(::core::ffi::c_uint::MAX, 0, 0, 0),
            (::core::ffi::c_uint::MAX, 0, 0)
        );
        assert_eq!(
            add_and_consume_extra_bits(::core::ffi::c_uint::MAX, 1, 0, 1),
            (0, 0, ::core::ffi::c_uint::MAX)
        );
    }

    #[test]
    fn append_input_byte_packs_bytes_low_bit_first() {
        let (hold, bits) = append_input_byte(0, 0, 0xab);
        assert_eq!((hold, bits), (0xab, 8));
        assert_eq!(append_input_byte(hold, bits, 0xcd), (0xcdab, 16));
    }

    #[test]
    fn append_input_byte_preserves_existing_bits() {
        assert_eq!(append_input_byte(0b101, 3, 0b11), (0b1_1101, 11));
    }

    #[test]
    fn refill_input_byte_updates_bit_buffer_and_remaining_input() {
        assert_eq!(refill_input_byte(0b101, 3, 4, 0b11), (0b1_1101, 11, 3));
        assert_eq!(
            refill_input_byte(0, 0, 0, 0xab),
            (0xab, 8, ::core::ffi::c_uint::MAX)
        );
    }

    #[test]
    fn subtable_index_combines_base_and_low_bit_index() {
        let entry = fast_code_entry(code {
            op: 5,
            bits: 0,
            val: 96,
        });
        assert_eq!(subtable_index(entry, 0b1_1011), 123);

        let entry = fast_code_entry(code {
            op: 15,
            bits: 0,
            val: ::core::ffi::c_ushort::MAX,
        });
        assert_eq!(subtable_index(entry, ::core::ffi::c_ulong::MAX), 98_302);
    }

    #[test]
    fn table_index_selects_only_masked_low_bits() {
        assert_eq!(table_index(0b1101_1011, 0b1_1111), 27);
        assert_eq!(table_index(::core::ffi::c_ulong::MAX, 0), 0);
        assert_eq!(
            table_index(::core::ffi::c_ulong::MAX, ::core::ffi::c_uint::MAX),
            ::core::ffi::c_uint::MAX as usize,
        );
    }

    #[test]
    fn litlen_opcode_actions_preserve_deflate_dispatch_precedence() {
        assert_eq!(fast_litlen_action(0), FastLitLenAction::Literal);
        assert_eq!(
            fast_litlen_action(31),
            FastLitLenAction::Length { extra_bits: 15 }
        );
        assert_eq!(fast_litlen_action(1), FastLitLenAction::Subtable);
        assert_eq!(fast_litlen_action(32), FastLitLenAction::Subtable);
        assert_eq!(fast_litlen_action(96), FastLitLenAction::End);
        assert_eq!(fast_litlen_action(64), FastLitLenAction::Invalid);
        assert_eq!(
            fast_litlen_action(80),
            FastLitLenAction::Length { extra_bits: 0 }
        );
        assert_eq!(
            fast_litlen_action(112),
            FastLitLenAction::Length { extra_bits: 0 }
        );
    }

    #[test]
    fn distance_opcode_actions_preserve_base_and_invalid_precedence() {
        assert_eq!(
            fast_dist_action(16),
            FastDistAction::Distance { extra_bits: 0 }
        );
        assert_eq!(
            fast_dist_action(31),
            FastDistAction::Distance { extra_bits: 15 }
        );
        assert_eq!(fast_dist_action(1), FastDistAction::Subtable);
        assert_eq!(fast_dist_action(32), FastDistAction::Subtable);
        assert_eq!(fast_dist_action(64), FastDistAction::Invalid);
        assert_eq!(
            fast_dist_action(80),
            FastDistAction::Distance { extra_bits: 0 }
        );
    }

    #[test]
    fn unread_input_state_rewinds_full_bytes_and_restores_input() {
        assert_eq!(unread_input_state(0xdead_beef, 21, 4), (0x0f, 5, 2, 6));
    }

    #[test]
    fn fast_path_commit_preserves_unread_input_and_bit_state() {
        assert_eq!(
            fast_path_commit(0xdead_beef, 21, 4),
            super::FastPathCommit {
                hold: 0x0f,
                bits: 5,
                unread_bytes: 2,
                input_remaining: 6,
            }
        );
    }

    #[test]
    fn unread_input_state_clears_aligned_and_preserves_sub_byte_buffers() {
        assert_eq!(unread_input_state(0xfeed, 16, 4), (0, 0, 2, 6));
        assert_eq!(unread_input_state(0xff, 7, 4), (0x7f, 7, 0, 4));
    }

    #[test]
    fn input_remaining_after_read_preserves_wrapping_decrement() {
        assert_eq!(input_remaining_after_read(6), 5);
        assert_eq!(input_remaining_after_read(0), ::core::ffi::c_uint::MAX);
    }

    #[test]
    fn output_cursor_after_writes_preserves_triplet_accounting_and_wrapping() {
        assert_eq!(output_cursor_after_writes(7, 10, 3), (10, 7));
        assert_eq!(
            output_cursor_after_writes(::core::ffi::c_uint::MAX, 1, 3),
            (2, ::core::ffi::c_uint::MAX - 1)
        );
        assert_eq!(
            output_cursor_after_write(7, 10),
            output_cursor_after_writes(7, 10, 1)
        );
    }

    #[test]
    fn fast_window_distance_back_returns_distance_or_decode_failure() {
        assert_eq!(fast_window_distance_back(12, 7, 5, true), Ok(5));
        assert_eq!(
            fast_window_distance_back(13, 7, 5, true),
            Err(fast_decode_failure(FastDecodeError::DistanceTooFarBack))
        );
        assert_eq!(fast_window_distance_back(13, 7, 5, false), Ok(6));
    }

    #[test]
    fn input_bytes_needed_refills_only_the_missing_bits() {
        assert_eq!(input_bytes_needed(15, 13), 0);
        assert_eq!(input_bytes_needed(8, 13), 1);
        assert_eq!(input_bytes_needed(0, 13), 2);
        assert_eq!(input_bytes_needed(0, 0), 0);
        assert_eq!(input_bytes_needed(0, ::core::ffi::c_uint::MAX), 536_870_912);
    }

    #[test]
    fn input_bytes_needed_rounds_partial_missing_bytes_up() {
        assert_eq!(input_bytes_needed(0, 8), 1);
        assert_eq!(input_bytes_needed(0, 9), 2);
        assert_eq!(input_bytes_needed(1, 9), 1);
    }

    #[test]
    fn fast_decode_prefetch_preserves_fifteen_bit_threshold() {
        assert!(fast_decode_needs_prefetch(0));
        assert!(fast_decode_needs_prefetch(14));
        assert!(!fast_decode_needs_prefetch(15));
        assert!(!fast_decode_needs_prefetch(::core::ffi::c_uint::MAX));
    }

    #[test]
    fn fast_decode_prefetch_count_preserves_two_byte_refill() {
        assert_eq!(fast_decode_prefetch_byte_count(0), 2);
        assert_eq!(fast_decode_prefetch_byte_count(14), 2);
        assert_eq!(fast_decode_prefetch_byte_count(15), 0);
        assert_eq!(fast_decode_prefetch_byte_count(::core::ffi::c_uint::MAX), 0);
    }

    #[test]
    fn fast_length_extra_bits_refill_count_requires_nonzero_missing_bits() {
        assert_eq!(fast_length_extra_bits_refill_byte_count(0, 0), 0);
        assert_eq!(fast_length_extra_bits_refill_byte_count(0, 1), 1);
        assert_eq!(fast_length_extra_bits_refill_byte_count(4, 5), 1);
        assert_eq!(fast_length_extra_bits_refill_byte_count(5, 5), 0);
        assert_eq!(
            fast_length_extra_bits_refill_byte_count(::core::ffi::c_uint::MAX, 15),
            0
        );
    }

    #[test]
    fn unread_input_state_wraps_restored_input() {
        assert_eq!(
            unread_input_state(::core::ffi::c_ulong::MAX, 8, ::core::ffi::c_uint::MAX),
            (0, 0, 1, 0),
        );
    }

    #[test]
    fn fast_output_cursor_tracks_writes() {
        assert_eq!(output_cursor_after_write(4, 3), (5, 2));
        assert_eq!(
            output_cursor_after_write(::core::ffi::c_uint::MAX, 0),
            (0, ::core::ffi::c_uint::MAX),
        );
    }

    #[test]
    fn fast_output_match_copy_preserves_overlapping_history() {
        let mut output = *b"abc______";

        assert_eq!(fast_copy_from_output(&mut output, 3, 3, 6), Some(()));
        assert_eq!(output, *b"abcabcabc");
    }

    #[test]
    fn fast_output_match_copy_rejects_invalid_ranges_without_writing() {
        let mut output = *b"abc___";

        assert_eq!(fast_copy_from_output(&mut output, 3, 0, 2), None);
        assert_eq!(output, *b"abc___");
        assert_eq!(fast_copy_from_output(&mut output, 3, 4, 2), None);
        assert_eq!(output, *b"abc___");
        assert_eq!(fast_copy_from_output(&mut output, 3, 3, 4), None);
        assert_eq!(output, *b"abc___");
    }

    #[test]
    fn fast_path_start_output_count_preserves_wrapping_subtraction() {
        assert_eq!(output_produced_at_fast_path_start(10, 4), 6);
        assert_eq!(
            output_produced_at_fast_path_start(0, 1),
            ::core::ffi::c_uint::MAX
        );
    }

    #[test]
    fn trailing_match_copy_byte_count_preserves_two_byte_copy_limit() {
        assert_eq!(trailing_match_copy_byte_count(0), 0);
        assert_eq!(trailing_match_copy_byte_count(1), 1);
        assert_eq!(trailing_match_copy_byte_count(2), 2);
        assert_eq!(trailing_match_copy_byte_count(::core::ffi::c_uint::MAX), 2);
    }

    #[test]
    fn trailing_match_copy_second_byte_guard_preserves_boundaries() {
        assert!(!trailing_match_copy_needs_second_byte(0));
        assert!(!trailing_match_copy_needs_second_byte(1));
        assert!(trailing_match_copy_needs_second_byte(2));
        assert!(trailing_match_copy_needs_second_byte(
            ::core::ffi::c_uint::MAX
        ));
    }

    #[test]
    fn fast_match_copy_layout_preserves_output_history_triplets() {
        assert_eq!(
            fast_match_copy_layout(false, 0, 0, 0, 8),
            FastMatchCopyLayout {
                final_copy_triplets: 2,
                final_trailing_bytes: 2,
                window_copy_plan: None,
            }
        );
    }

    #[test]
    fn fast_match_copy_layout_preserves_window_wrap_and_output_continuation() {
        let copy_plan = fast_window_copy_plan(32, 7, 12, 13);
        assert_eq!(
            fast_match_copy_layout(true, 32, 7, 12, 13),
            FastMatchCopyLayout {
                final_copy_triplets: 0,
                final_trailing_bytes: 1,
                window_copy_plan: Some(copy_plan),
            }
        );
    }

    #[test]
    fn fast_window_copy_plan_preserves_exact_first_segment_boundaries() {
        assert_eq!(
            fast_window_copy_plan(32, 0, 5, 5),
            FastWindowCopyPlan {
                first_window_start: 27,
                first_window_length: 0,
                wrap_window_start_length: None,
                remaining_length: 5,
                continuation_source: FastWindowContinuationSource::Window,
            }
        );
        assert_eq!(
            fast_window_copy_plan(32, 0, 5, 6),
            FastWindowCopyPlan {
                first_window_start: 27,
                first_window_length: 5,
                wrap_window_start_length: None,
                remaining_length: 1,
                continuation_source: FastWindowContinuationSource::Output,
            }
        );
        assert_eq!(
            fast_window_copy_plan(32, 12, 5, 5),
            FastWindowCopyPlan {
                first_window_start: 7,
                first_window_length: 0,
                wrap_window_start_length: None,
                remaining_length: 5,
                continuation_source: FastWindowContinuationSource::Window,
            }
        );
        assert_eq!(
            fast_window_copy_plan(32, 12, 5, 6),
            FastWindowCopyPlan {
                first_window_start: 7,
                first_window_length: 5,
                wrap_window_start_length: None,
                remaining_length: 1,
                continuation_source: FastWindowContinuationSource::Output,
            }
        );
    }

    #[test]
    fn fast_window_copy_plan_preserves_wrap_segments_and_boundaries() {
        assert_eq!(
            fast_window_copy_plan(32, 7, 12, 5),
            FastWindowCopyPlan {
                first_window_start: 27,
                first_window_length: 0,
                wrap_window_start_length: None,
                remaining_length: 5,
                continuation_source: FastWindowContinuationSource::Window,
            }
        );
        assert_eq!(
            fast_window_copy_plan(32, 7, 12, 6),
            FastWindowCopyPlan {
                first_window_start: 27,
                first_window_length: 5,
                wrap_window_start_length: Some(0),
                remaining_length: 1,
                continuation_source: FastWindowContinuationSource::Window,
            }
        );
        assert_eq!(
            fast_window_copy_plan(32, 7, 12, 7),
            FastWindowCopyPlan {
                first_window_start: 27,
                first_window_length: 5,
                wrap_window_start_length: Some(0),
                remaining_length: 2,
                continuation_source: FastWindowContinuationSource::Window,
            }
        );
        assert_eq!(
            fast_window_copy_plan(32, 7, 12, 12),
            FastWindowCopyPlan {
                first_window_start: 27,
                first_window_length: 5,
                wrap_window_start_length: Some(0),
                remaining_length: 7,
                continuation_source: FastWindowContinuationSource::Window,
            }
        );
        assert_eq!(
            fast_window_copy_plan(32, 7, 12, 13),
            FastWindowCopyPlan {
                first_window_start: 27,
                first_window_length: 5,
                wrap_window_start_length: Some(7),
                remaining_length: 1,
                continuation_source: FastWindowContinuationSource::Output,
            }
        );
    }

    #[test]
    fn fast_window_copy_plan_preserves_wrapping_start_arithmetic() {
        assert_eq!(
            fast_window_copy_plan(::core::ffi::c_uint::MAX, 0, 1, 1),
            FastWindowCopyPlan {
                first_window_start: ::core::ffi::c_uint::MAX - 1,
                first_window_length: 0,
                wrap_window_start_length: None,
                remaining_length: 1,
                continuation_source: FastWindowContinuationSource::Window,
            }
        );
        assert_eq!(
            fast_window_copy_plan(0, 1, 2, 1),
            FastWindowCopyPlan {
                first_window_start: ::core::ffi::c_uint::MAX,
                first_window_length: 0,
                wrap_window_start_length: None,
                remaining_length: 1,
                continuation_source: FastWindowContinuationSource::Window,
            }
        );
    }
}
