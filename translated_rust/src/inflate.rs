// =============== BEGIN inflate_h ================
pub type inflate_mode = ::core::ffi::c_uint;

pub const HEAD: crate::src::inflate::inflate_mode = 16180;

pub const FLAGS: crate::src::inflate::inflate_mode = 16181;

pub const TIME: crate::src::inflate::inflate_mode = 16182;

pub const OS: crate::src::inflate::inflate_mode = 16183;

pub const EXLEN: crate::src::inflate::inflate_mode = 16184;

pub const EXTRA: crate::src::inflate::inflate_mode = 16185;

pub const NAME: crate::src::inflate::inflate_mode = 16186;

pub const COMMENT: crate::src::inflate::inflate_mode = 16187;

pub const HCRC: crate::src::inflate::inflate_mode = 16188;

pub const DICTID: crate::src::inflate::inflate_mode = 16189;

pub const DICT: crate::src::inflate::inflate_mode = 16190;

pub const TYPE: crate::src::inflate::inflate_mode = 16191;

pub const TYPEDO: crate::src::inflate::inflate_mode = 16192;

pub const STORED: crate::src::inflate::inflate_mode = 16193;

pub const COPY_: crate::src::inflate::inflate_mode = 16194;

pub const COPY_1: crate::src::inflate::inflate_mode = 16195;

pub const TABLE: crate::src::inflate::inflate_mode = 16196;

pub const LENLENS: crate::src::inflate::inflate_mode = 16197;

pub const CODELENS: crate::src::inflate::inflate_mode = 16198;

pub const LEN_: crate::src::inflate::inflate_mode = 16199;

pub const LEN: crate::src::inflate::inflate_mode = 16200;

pub const LENEXT: crate::src::inflate::inflate_mode = 16201;

pub const DIST: crate::src::inflate::inflate_mode = 16202;

pub const DISTEXT: crate::src::inflate::inflate_mode = 16203;

pub const MATCH: crate::src::inflate::inflate_mode = 16204;

pub const LIT: crate::src::inflate::inflate_mode = 16205;

pub const CHECK: crate::src::inflate::inflate_mode = 16206;

pub const LENGTH: crate::src::inflate::inflate_mode = 16207;

pub const DONE: crate::src::inflate::inflate_mode = 16208;

pub const BAD: crate::src::inflate::inflate_mode = 16209;

pub const MEM: crate::src::inflate::inflate_mode = 16210;

pub const SYNC: crate::src::inflate::inflate_mode = 16211;
#[derive(Copy, Clone)]
pub enum InflateCodeTable {
    Empty,
    FixedLens,
    FixedDists,
    Dynamic(usize),
}

#[derive(Copy, Clone)]
pub struct inflate_state {
    /// The stream that owns this opaque state is an identity token only. It is
    /// never converted back to a pointer or dereferenced; ABI boundaries adopt
    /// the caller stream independently for each call.
    pub stream_token: usize,
    pub mode: crate::src::inflate::inflate_mode,
    pub last: ::core::ffi::c_int,
    pub wrap: ::core::ffi::c_int,
    pub havedict: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub dmax: ::core::ffi::c_uint,
    pub check: ::core::ffi::c_ulong,
    pub total: ::core::ffi::c_ulong,
    pub head: crate::zlib_h::gz_headerp,
    pub wbits: ::core::ffi::c_uint,
    pub wsize: ::core::ffi::c_uint,
    pub whave: ::core::ffi::c_uint,
    pub wnext: ::core::ffi::c_uint,
    pub window: *mut ::core::ffi::c_uchar,
    pub hold: ::core::ffi::c_ulong,
    pub bits: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub extra: ::core::ffi::c_uint,
    pub lencode: InflateCodeTable,
    pub distcode: InflateCodeTable,
    pub lenbits: ::core::ffi::c_uint,
    pub distbits: ::core::ffi::c_uint,
    pub ncode: ::core::ffi::c_uint,
    pub nlen: ::core::ffi::c_uint,
    pub ndist: ::core::ffi::c_uint,
    pub have: ::core::ffi::c_uint,
    /// Number of table entries populated in `codes`.  This used to be an
    /// interior pointer into the owned table, but no decoder operation needs
    /// an address to describe that progress.
    pub next: usize,
    pub lens: [::core::ffi::c_ushort; 320],
    pub work: [::core::ffi::c_ushort; 288],
    pub codes: [crate::src::inftrees::code; 1444],
    pub sane: ::core::ffi::c_int,
    pub back: ::core::ffi::c_int,
    pub was: ::core::ffi::c_uint,
}

/// Construct the exact all-zero state that `inflateInit2_()` historically
/// obtained from `memset`, with the few fields it initialized immediately
/// afterwards already set.  Keeping this as an ordinary value makes the
/// initial state explicit; the allocator-owned pointer is written only by
/// the export boundary.
pub(crate) fn inflate_initial_state() -> inflate_state {
    let empty_code = crate::src::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    };
    inflate_state {
        stream_token: 0,
        mode: crate::src::inflate::HEAD,
        last: 0,
        wrap: 0,
        havedict: 0,
        flags: 0,
        dmax: 0,
        check: 0,
        total: 0,
        head: ::core::ptr::null_mut::<crate::zlib_h::gz_header>(),
        wbits: 0,
        wsize: 0,
        whave: 0,
        wnext: 0,
        window: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        hold: 0,
        bits: 0,
        length: 0,
        offset: 0,
        extra: 0,
        lencode: InflateCodeTable::Empty,
        distcode: InflateCodeTable::Empty,
        lenbits: 0,
        distbits: 0,
        ncode: 0,
        nlen: 0,
        ndist: 0,
        have: 0,
        next: 0,
        lens: [0; 320],
        work: [0; 288],
        codes: [empty_code; 1444],
        sane: 0,
        back: 0,
        was: 0,
    }
}
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::ENOUGH;
pub use crate::src::inftrees::ENOUGH_DISTS;
pub use crate::src::inftrees::ENOUGH_LENS;
pub use crate::src::inftrees::LENS;

pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;

pub use crate::stdlib::voidpf;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::stdlib::MAX_WBITS;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gz_header;
pub use crate::zlib_h::gz_header_s;
pub use crate::zlib_h::gz_headerp;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BLOCK;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_DEFLATED;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub use crate::zlib_h::Z_TREES;
pub use crate::zlib_h::Z_VERSION_ERROR;
pub use crate::zutil_h::DEF_WBITS;

/// Update the wrapper checksum for bytes that the decoder has already
/// produced.  The surrounding codec boundary owns the temporary output
/// view; keeping the choice of checksum here avoids duplicating that raw
/// view across the gzip and zlib branches.
fn inflate_output_checksum(
    check: crate::stdlib::uLong,
    flags: ::core::ffi::c_int,
    output: &[u8],
) -> ::core::ffi::c_ulong {
    if flags != 0 {
        crate::src::crc32::crc32_z(check, output) as ::core::ffi::c_ulong
    } else {
        crate::src::adler32::adler32_z(check, output) as ::core::ffi::c_ulong
    }
}

/// Decide and compute the checksum publication for ordinary inflate's exit.
/// The decoder boundary owns the temporary output view; this core makes the
/// wrapper bit and empty-output behavior explicit without consulting ABI
/// state.
fn inflate_exit_needs_checksum(wrap: ::core::ffi::c_int, output_len: usize) -> bool {
    wrap & 4 as ::core::ffi::c_int != 0 && output_len != 0
}

fn inflate_exit_checksum(
    wrap: ::core::ffi::c_int,
    check: crate::stdlib::uLong,
    flags: ::core::ffi::c_int,
    output: &[u8],
) -> Option<::core::ffi::c_ulong> {
    if !inflate_exit_needs_checksum(wrap, output.len()) {
        None
    } else {
        Some(inflate_output_checksum(check, flags, output))
    }
}

/// Normalize the checksum word from an inflate trailer before comparing it to
/// the checksum accumulated by the decoder.  Gzip carries the word in native
/// little-endian bit-buffer order; zlib carries it in network order.  Keeping
/// this scalar-only conversion out of the cursor loop makes the trailer
/// decision independent of ABI records and raw output lends.
fn inflate_trailer_checksum(
    flags: ::core::ffi::c_int,
    hold: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    if flags != 0 {
        hold
    } else {
        (hold >> 24 & 0xff as ::core::ffi::c_ulong)
            .wrapping_add(hold >> 8 & 0xff00 as ::core::ffi::c_ulong)
            .wrapping_add((hold & 0xff00 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int)
            .wrapping_add((hold & 0xff as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int)
    }
}

/// Convert the four little-endian dictionary-id bytes gathered by the
/// bit-buffer into zlib's network-order Adler-32 value.  This uses the same
/// scalar byte-order rule as a zlib trailer; cursor consumption and ABI
/// publication remain at the ordinary-inflate boundary.
fn inflate_dictionary_id(hold: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    inflate_trailer_checksum(0, hold)
}

/// Validate a stored block's little-endian length/complement pair and return
/// the length on success. The cursor loop supplies exactly four aligned
/// bytes; keeping the complement test here makes malformed-block policy a
/// pointer-free scalar decision.
fn inflate_stored_block_len(hold: ::core::ffi::c_ulong) -> Option<::core::ffi::c_uint> {
    let length = hold as ::core::ffi::c_uint & 0xffff;
    let complement = (hold >> 16) as ::core::ffi::c_uint & 0xffff;
    if length ^ complement == 0xffff {
        Some(length)
    } else {
        None
    }
}

/// The kind of DEFLATE block selected by the three bits at the front of a
/// block. This remains scalar-only: the decoder boundary installs fixed
/// tables and publishes the existing invalid-block diagnostic.
#[derive(Copy, Clone)]
enum InflateBlockKind {
    Stored,
    Fixed,
    Dynamic,
    Invalid,
}

/// Consume a complete ordinary-inflate block header without touching stream
/// or state records. The caller must first gather three bits from its input
/// cursor; invalid or incoherent bit counts are rejected instead of allowing
/// a malformed state to underflow the bit counter.
#[derive(Copy, Clone)]
struct InflateBlockHeaderPlan {
    last: ::core::ffi::c_int,
    kind: InflateBlockKind,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
}

fn inflate_block_header_plan(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> Option<InflateBlockHeaderPlan> {
    let bits = bits.checked_sub(3)?;
    let last = (hold & 1) as ::core::ffi::c_int;
    let kind = match (hold >> 1) & 3 {
        0 => InflateBlockKind::Stored,
        1 => InflateBlockKind::Fixed,
        2 => InflateBlockKind::Dynamic,
        _ => InflateBlockKind::Invalid,
    };
    Some(InflateBlockHeaderPlan {
        last,
        kind,
        hold: hold >> 3,
        bits,
    })
}

/// Scalar result of parsing the 14-bit dynamic-Huffman table header. The
/// ordinary decoder has already accumulated those bits; this keeps its
/// compatibility cursor and diagnostics at the boundary while making the
/// field extraction and DEFLATE limits independently checked.
struct InflateTableHeaderPlan {
    nlen: ::core::ffi::c_uint,
    ndist: ::core::ffi::c_uint,
    ncode: ::core::ffi::c_uint,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
}

/// Parse a complete dynamic-Huffman table header. Return `None` for the
/// original "too many length or distance symbols" condition.
fn inflate_table_header_plan(
    mut hold: ::core::ffi::c_ulong,
    mut bits: ::core::ffi::c_uint,
) -> Option<InflateTableHeaderPlan> {
    let nlen = (hold as ::core::ffi::c_uint & 0x1f).wrapping_add(257);
    hold >>= 5;
    bits = bits.wrapping_sub(5);
    let ndist = (hold as ::core::ffi::c_uint & 0x1f).wrapping_add(1);
    hold >>= 5;
    bits = bits.wrapping_sub(5);
    let ncode = (hold as ::core::ffi::c_uint & 0x0f).wrapping_add(4);
    hold >>= 4;
    bits = bits.wrapping_sub(4);
    if nlen > 286 || ndist > 30 {
        return None;
    }
    Some(InflateTableHeaderPlan {
        nlen,
        ndist,
        ncode,
        hold,
        bits,
    })
}

/// The completed dynamic-code lengths live entirely in the owned inflate
/// state. Build both decode tables there, preserving the legacy order of
/// partial state publication while keeping table ranges and table-source
/// selection out of the cursor-driven decoder loop.
///
/// The caller remains responsible for the ABI-visible diagnostic and mode
/// transition.  Distinguishing the two failures preserves their existing
/// messages without requiring this safe helper to inspect a stream record.
enum InflateDynamicTableError {
    Lengths,
    Distances,
}

fn inflate_build_dynamic_tables(
    state: &mut crate::src::inflate::inflate_state,
) -> Result<(), InflateDynamicTableError> {
    let nlen = state.nlen as usize;
    state.next = 0;
    state.lencode = InflateCodeTable::Dynamic(0);
    state.lenbits = 9;
    let lenbits = state.lenbits;
    let (lens_used, root) = {
        let (lens, codes, work) = (&state.lens, &mut state.codes, &mut state.work);
        let Some(lens) = lens.get(..nlen) else {
            return Err(InflateDynamicTableError::Lengths);
        };
        let Some(table) = codes.get_mut(..crate::src::inftrees::ENOUGH_LENS as usize) else {
            return Err(InflateDynamicTableError::Lengths);
        };
        crate::src::inftrees::inflate_table_into(
            crate::src::inftrees::LENS,
            lens,
            table,
            work,
            lenbits,
        )
        .map_err(|_| InflateDynamicTableError::Lengths)?
    };
    state.next = lens_used;
    state.lenbits = root;

    let ndist = state.ndist as usize;
    state.distcode = InflateCodeTable::Dynamic(lens_used);
    state.distbits = 6;
    let distbits = state.distbits;
    let end = lens_used
        .checked_add(crate::src::inftrees::ENOUGH_DISTS as usize)
        .ok_or(InflateDynamicTableError::Distances)?;
    let (dist_used, root) = {
        let (lens, codes, work) = (&state.lens, &mut state.codes, &mut state.work);
        let Some(lens) = lens.get(nlen..nlen.saturating_add(ndist)) else {
            return Err(InflateDynamicTableError::Distances);
        };
        let Some(table) = codes.get_mut(lens_used..end) else {
            return Err(InflateDynamicTableError::Distances);
        };
        crate::src::inftrees::inflate_table_into(
            crate::src::inftrees::DISTS,
            lens,
            table,
            work,
            distbits,
        )
        .map_err(|_| InflateDynamicTableError::Distances)?
    };
    state.next = lens_used.saturating_add(dist_used);
    state.distbits = root;
    Ok(())
}

/// Ordinary inflate only validates a trailer when the active wrapper has a
/// checksum.  This preserves the raw decoder's no-wrapper path while keeping
/// the comparison as a safe scalar operation.
fn inflate_trailer_checksum_matches(
    wrap: ::core::ffi::c_int,
    flags: ::core::ffi::c_int,
    hold: ::core::ffi::c_ulong,
    check: ::core::ffi::c_ulong,
) -> bool {
    wrap & 4 as ::core::ffi::c_int == 0 || inflate_trailer_checksum(flags, hold) == check
}

/// Validate gzip's final uncompressed-size word after the decoder boundary
/// has gathered it.  Wrapper selection and byte gathering remain at that
/// boundary; this scalar core preserves zlib's low-32-bit comparison without
/// consulting retained ABI state.
fn inflate_trailer_length_matches(
    wrap: ::core::ffi::c_int,
    hold: ::core::ffi::c_ulong,
    total: ::core::ffi::c_ulong,
) -> bool {
    wrap & 4 as ::core::ffi::c_int == 0 || hold == total & 0xffffffff as ::core::ffi::c_ulong
}

/// Update the gzip-header CRC from an already-bounded byte span.  The
/// transitional decoder owns any raw cursor lending; header parsing itself
/// only carries this scalar checksum and a safe byte slice.
fn inflate_header_crc_update(check: ::core::ffi::c_ulong, bytes: &[u8]) -> ::core::ffi::c_ulong {
    crate::src::crc32::crc32_z(check as crate::stdlib::uLong, bytes) as ::core::ffi::c_ulong
}

/// Plan the bounded copy into a caller-provided gzip extra-field buffer.
/// `extra_len` is the decoded total length and `remaining` is the portion
/// still unread, so their wrapping difference deliberately retains zlib's
/// compatibility arithmetic for malformed retained headers.  Pointer access
/// and the actual copy stay at the decoder boundary.
fn inflate_header_extra_copy_plan(
    extra_len: crate::stdlib::uInt,
    extra_max: crate::stdlib::uInt,
    remaining: ::core::ffi::c_uint,
    available: ::core::ffi::c_uint,
) -> Option<(usize, usize)> {
    let offset = extra_len.wrapping_sub(remaining);
    if offset >= extra_max {
        return None;
    }
    let copied = if offset.wrapping_add(available) > extra_max {
        extra_max.wrapping_sub(offset)
    } else {
        available
    };
    Some((offset as usize, copied as usize))
}

/// Plan gzip's optional extra-field length transition without touching the
/// retained ABI header. The decoder boundary still owns cursor consumption and
/// header publication; this keeps the flag-dependent scalar policy explicit.
struct InflateExtraLengthPlan {
    length: ::core::ffi::c_uint,
    update_crc: bool,
}

fn inflate_extra_length_plan(
    flags: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
    hold: ::core::ffi::c_ulong,
) -> InflateExtraLengthPlan {
    let has_extra = flags & 0x400 != 0;
    InflateExtraLengthPlan {
        length: hold as ::core::ffi::c_uint,
        update_crc: has_extra && flags & 0x200 != 0 && wrap & 4 != 0,
    }
}

/// Validate the low 16 bits carried by a gzip header CRC.  The wrapper bit,
/// accumulated CRC, and bit-buffer value are all scalar state, so this policy
/// does not need to remain in the transitional cursor loop.
fn inflate_header_crc_matches(
    wrap: ::core::ffi::c_int,
    check: ::core::ffi::c_ulong,
    hold: ::core::ffi::c_ulong,
) -> bool {
    wrap & 4 as ::core::ffi::c_int == 0 || hold == check & 0xffff as ::core::ffi::c_ulong
}

/// Commit policy for gzip's optional header-CRC word after the decoder
/// boundary has gathered it.  The cursor and retained ABI header remain at
/// that boundary; this plan only decides whether the gathered word is valid
/// and which scalar bit-buffer/header fields follow it.
struct InflateGzipHeaderCrcPlan {
    matches: bool,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    hcrc: ::core::ffi::c_int,
}

fn inflate_gzip_header_crc_plan(
    flags: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
    check: ::core::ffi::c_ulong,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> InflateGzipHeaderCrcPlan {
    let has_crc = flags & 0x200 != 0;
    let matches = !has_crc || inflate_header_crc_matches(wrap, check, hold);
    InflateGzipHeaderCrcPlan {
        matches,
        hold: if has_crc && matches { 0 } else { hold },
        bits: if has_crc && matches { 0 } else { bits },
        hcrc: (flags >> 9) & 1,
    }
}

/// Identify the diagnostic for an ordinary zlib header word.  This is only
/// the scalar validation performed after the cursor loop has supplied two
/// bytes; consuming those bytes and publishing the error remain at the ABI
/// boundary.
fn inflate_zlib_header_error(
    wrap: ::core::ffi::c_int,
    hold: ::core::ffi::c_ulong,
) -> Option<usize> {
    let header = hold as ::core::ffi::c_uint;
    let check = ((header & 0xff) << 8) as ::core::ffi::c_ulong;
    if wrap & 1 == 0 || check.wrapping_add(hold >> 8).wrapping_rem(31) != 0 {
        Some(0)
    } else if header & 0x0f != crate::zlib_h::Z_DEFLATED as ::core::ffi::c_uint {
        Some(1)
    } else {
        None
    }
}

/// The scalar state selected by a valid zlib header.  Cursor consumption and
/// ABI error publication remain at the ordinary-inflate boundary; this plan
/// makes the window and dictionary decisions independently checked.
struct InflateZlibHeaderPlan {
    wbits: ::core::ffi::c_uint,
    dmax: ::core::ffi::c_uint,
    mode: inflate_mode,
}

/// Validate a zlib header and select its post-header decoder state.  `hold`
/// is intentionally left unmodified: the compatibility cursor loop retains
/// responsibility for dropping its method/flags bits at the original commit
/// point.
fn inflate_zlib_header_plan(
    wrap: ::core::ffi::c_int,
    configured_wbits: ::core::ffi::c_uint,
    hold: ::core::ffi::c_ulong,
) -> Result<InflateZlibHeaderPlan, usize> {
    if let Some(error) = inflate_zlib_header_error(wrap, hold) {
        return Err(error);
    }

    let header = hold >> 4;
    let header_wbits = (header as ::core::ffi::c_uint & 0x0f).wrapping_add(8);
    let wbits = if configured_wbits == 0 {
        header_wbits
    } else {
        configured_wbits
    };
    if header_wbits > 15 || header_wbits > wbits {
        return Err(2);
    }

    Ok(InflateZlibHeaderPlan {
        wbits,
        dmax: 1u32.wrapping_shl(header_wbits) as ::core::ffi::c_uint,
        mode: if header & 0x200 != 0 {
            crate::src::inflate::DICTID
        } else {
            crate::src::inflate::TYPE
        },
    })
}

/// The scalar state selected by a valid gzip flags word.  The ordinary
/// decoder still owns consuming that word and, when present, writing the
/// caller's retained header; this plan only validates the gzip-defined bits
/// and selects the safe scalar commits.
struct InflateGzipFlagsPlan {
    flags: ::core::ffi::c_int,
    text: ::core::ffi::c_int,
    update_crc: bool,
}

/// Validate the two-byte gzip method/flags word after the cursor loop has
/// assembled it.  Keep `hold` intact so the transitional boundary preserves
/// its exact CRC byte order and cursor reset behavior.
fn inflate_gzip_flags_plan(
    wrap: ::core::ffi::c_int,
    hold: ::core::ffi::c_ulong,
) -> Result<InflateGzipFlagsPlan, usize> {
    let flags = hold as ::core::ffi::c_int;
    if flags & 0xff != crate::zlib_h::Z_DEFLATED {
        return Err(1);
    }
    if flags & 0xe000 != 0 {
        return Err(3);
    }
    Ok(InflateGzipFlagsPlan {
        flags,
        text: ((hold >> 8) & 1) as ::core::ffi::c_int,
        update_crc: flags & 0x200 != 0 && wrap & 4 != 0,
    })
}

/// Scalar commit for gzip's four-byte modification-time field.  The
/// transitional decoder still owns the input cursor and the retained ABI
/// header destination; this plan keeps the little-endian CRC byte order and
/// wrapper/flag admission out of that raw boundary.
struct InflateGzipTimePlan {
    time: crate::stdlib::uLong,
    crc_bytes: Option<[u8; 4]>,
}

fn inflate_gzip_time_plan(
    flags: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
    hold: ::core::ffi::c_ulong,
) -> InflateGzipTimePlan {
    let crc_bytes = (flags & 0x200 != 0 && wrap & 4 != 0).then_some([
        hold as u8,
        (hold >> 8) as u8,
        (hold >> 16) as u8,
        (hold >> 24) as u8,
    ]);
    InflateGzipTimePlan {
        time: hold as crate::stdlib::uLong,
        crc_bytes,
    }
}

/// Commit the pointer-free portion of gzip's modification-time transition.
/// The retained header itself is caller-owned ABI storage, so its write stays
/// at the decoder boundary immediately before this state commit.
fn inflate_gzip_time_commit(
    state: &mut crate::src::inflate::inflate_state,
    plan: InflateGzipTimePlan,
) {
    if let Some(crc_bytes) = plan.crc_bytes {
        state.check = inflate_header_crc_update(state.check, &crc_bytes);
    }
    state.mode = crate::src::inflate::OS;
}

/// Scalar commit for gzip's XFL and OS bytes.  The cursor and retained ABI
/// header remain at the decoder boundary; this plan keeps byte extraction and
/// optional little-endian header-CRC input pointer-free.
struct InflateGzipOsPlan {
    xflags: ::core::ffi::c_int,
    os: ::core::ffi::c_int,
    crc_bytes: Option<[u8; 2]>,
}

fn inflate_gzip_os_plan(
    flags: ::core::ffi::c_int,
    wrap: ::core::ffi::c_int,
    hold: ::core::ffi::c_ulong,
) -> InflateGzipOsPlan {
    InflateGzipOsPlan {
        xflags: (hold & 0xff) as ::core::ffi::c_int,
        os: (hold >> 8) as ::core::ffi::c_int,
        crc_bytes: (flags & 0x200 != 0 && wrap & 4 != 0).then_some([hold as u8, (hold >> 8) as u8]),
    }
}

/// Commit the pointer-free portion of gzip's XFL/OS transition.  As with the
/// TIME field, the caller-owned retained header is updated at the raw codec
/// boundary before this safe state transition runs.
fn inflate_gzip_os_commit(
    state: &mut crate::src::inflate::inflate_state,
    plan: InflateGzipOsPlan,
) {
    if let Some(crc_bytes) = plan.crc_bytes {
        state.check = inflate_header_crc_update(state.check, &crc_bytes);
    }
    state.mode = crate::src::inflate::EXLEN;
}

/// Preserve zlib's final no-progress/finish result mapping independently of
/// the ABI cursor commit that precedes it.
fn inflate_exit_status(
    input_used: ::core::ffi::c_uint,
    output_used: ::core::ffi::c_uint,
    flush: ::core::ffi::c_int,
    status: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (input_used == 0 && output_used == 0 || flush == crate::zlib_h::Z_FINISH)
        && status == crate::zlib_h::Z_OK
    {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        status
    }
}

// Keep all inflate diagnostics in one immutable table.  Besides making their
// storage explicit, this lets gzip retain an inflate error without treating
// `strm.msg` as an arbitrary foreign C string.
pub(crate) static INFLATE_ERROR_MESSAGES: [&[u8]; 18] = [
    b"incorrect header check\0",
    b"unknown compression method\0",
    b"invalid window size\0",
    b"unknown header flags set\0",
    b"invalid stored block lengths\0",
    b"too many length or distance symbols\0",
    b"incorrect data check\0",
    b"incorrect length check\0",
    b"invalid code lengths set\0",
    b"invalid bit length repeat\0",
    b"invalid code -- missing end-of-block\0",
    b"invalid literal/lengths set\0",
    b"invalid distances set\0",
    b"invalid block type\0",
    b"invalid literal/length code\0",
    b"invalid distance code\0",
    b"header crc mismatch\0",
    b"invalid distance too far back\0",
];

/// Validate the scalar portion of an inflate stream/state relationship.
/// Pointer validation remains at the ABI adapter, while this core documents
/// the complete set of modes accepted by zlib without pointer access.
pub(crate) fn inflate_state_values_are_valid(
    has_zalloc: bool,
    has_zfree: bool,
    state_matches_stream: bool,
    mode: crate::src::inflate::inflate_mode,
) -> bool {
    has_zalloc
        && has_zfree
        && state_matches_stream
        && (crate::src::inflate::HEAD..=crate::src::inflate::SYNC).contains(&mode)
}

/// Bound one stored-block transfer by the decoder's remaining input and
/// output. The ABI loops keep their cursors at the boundary; this core owns
/// only the scalar progress calculation.
pub(crate) fn inflate_stored_copy_len(
    remaining: ::core::ffi::c_uint,
    available_input: ::core::ffi::c_uint,
    available_output: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    remaining.min(available_input).min(available_output)
}

/// The scalar commit after `DISTEXT` has collected all of a distance code's
/// extra bits.  Cursor consumption stays at the legacy decoder boundary, but
/// this keeps the mask, shift, and wrapping compatibility counters out of the
/// raw cursor loop.  Invalid opaque-state scalars are rejected before they can
/// request a Rust shift wider than the distance word.
#[derive(Copy, Clone)]
struct InflateDistanceExtraPlan {
    offset: ::core::ffi::c_uint,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    back: ::core::ffi::c_int,
}

fn inflate_distance_extra_plan(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    extra: ::core::ffi::c_uint,
    offset: ::core::ffi::c_uint,
    back: ::core::ffi::c_int,
) -> Option<InflateDistanceExtraPlan> {
    if extra > bits {
        return None;
    }
    let mask = (1 as ::core::ffi::c_uint)
        .checked_shl(extra)?
        .wrapping_sub(1);
    Some(InflateDistanceExtraPlan {
        offset: offset.wrapping_add(hold as ::core::ffi::c_uint & mask),
        hold: hold >> extra,
        bits: bits.wrapping_sub(extra),
        back: (back as ::core::ffi::c_uint).wrapping_add(extra) as ::core::ffi::c_int,
    })
}

/// The scalar commit after `LENEXT` has collected the length code's extra
/// bits.  As with distance extras, consuming the ABI cursor remains in the
/// transitional decoder boundary.  Keeping the mask and shifts here makes an
/// incoherent opaque-state `extra` value an ordinary decode failure instead of
/// an oversized Rust shift.
#[derive(Copy, Clone)]
struct InflateLengthExtraPlan {
    length: ::core::ffi::c_uint,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    back: ::core::ffi::c_int,
}

fn inflate_length_extra_plan(
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    extra: ::core::ffi::c_uint,
    length: ::core::ffi::c_uint,
    back: ::core::ffi::c_int,
) -> Option<InflateLengthExtraPlan> {
    if extra > bits {
        return None;
    }
    let mask = (1 as ::core::ffi::c_uint)
        .checked_shl(extra)?
        .wrapping_sub(1);
    Some(InflateLengthExtraPlan {
        length: length.wrapping_add(hold as ::core::ffi::c_uint & mask),
        hold: hold >> extra,
        bits: bits.wrapping_sub(extra),
        back: (back as ::core::ffi::c_uint).wrapping_add(extra) as ::core::ffi::c_int,
    })
}

/// The scalar part of a dynamic-Huffman code-length repeat after the legacy
/// cursor has consumed the repeat symbol itself.  The cursor boundary still
/// gathers the required extra bits and publishes diagnostics; this helper
/// keeps the repeat count and bit-buffer commit independent of ABI state.
#[derive(Copy, Clone)]
struct InflateCodeLengthRepeatPlan {
    copy: ::core::ffi::c_uint,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
}

fn inflate_code_length_repeat_plan(
    symbol: ::core::ffi::c_ushort,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
) -> Option<InflateCodeLengthRepeatPlan> {
    let (base, extra): (::core::ffi::c_uint, ::core::ffi::c_uint) =
        match symbol as ::core::ffi::c_uint {
            16 => (3, 2),
            17 => (3, 3),
            // The table builder only emits 16 through 18 here.  Preserve the
            // translated decoder's historical fallback for an incoherent table
            // entry, while still bounding the shift below.
            _ => (11, 7),
        };
    if bits < extra {
        return None;
    }
    let mask = (1 as ::core::ffi::c_uint)
        .checked_shl(extra)?
        .wrapping_sub(1);
    Some(InflateCodeLengthRepeatPlan {
        copy: base.wrapping_add(hold as ::core::ffi::c_uint & mask),
        hold: hold.checked_shr(extra)?,
        bits: bits.wrapping_sub(extra),
    })
}

/// Validate and select the destination range for a completed code-length
/// repeat.  This retains zlib's wrapping count comparison, but prevents a
/// malformed opaque state from turning the following slice fill into an
/// out-of-bounds panic.
fn inflate_code_length_repeat_range(
    have: ::core::ffi::c_uint,
    copy: ::core::ffi::c_uint,
    total: ::core::ffi::c_uint,
    lens_len: usize,
) -> Option<::core::ops::Range<usize>> {
    let end = have.wrapping_add(copy);
    if end > total {
        return None;
    }
    let start = usize::try_from(have).ok()?;
    let end = usize::try_from(end).ok()?;
    (end <= lens_len).then_some(start..end)
}

/// Compute a bounded root-table index from the current bit buffer.  The
/// ordinary decoder retains cursor gathering and diagnostic publication, but
/// all Huffman table lookups share this checked mask calculation so malformed
/// opaque root widths cannot request an oversized Rust shift.
fn inflate_root_code_index(
    hold: ::core::ffi::c_ulong,
    root_bits: ::core::ffi::c_uint,
) -> Option<usize> {
    let mask = (1 as ::core::ffi::c_uint)
        .checked_shl(root_bits)?
        .wrapping_sub(1);
    usize::try_from(hold as ::core::ffi::c_uint & mask).ok()
}

/// Compute a bounded subtable index after a root-table entry selected a
/// second-level table.  Table reads remain checked slice accesses at the
/// decoder boundary; this helper only preserves zlib's wrapping table-base
/// arithmetic while validating the two shift widths.
fn inflate_subtable_code_index(
    hold: ::core::ffi::c_ulong,
    last: crate::src::inftrees::code,
) -> Option<usize> {
    let root_bits = last.bits as ::core::ffi::c_uint;
    let total_bits = root_bits.checked_add(last.op as ::core::ffi::c_uint)?;
    let mask = (1 as ::core::ffi::c_uint)
        .checked_shl(total_bits)?
        .wrapping_sub(1);
    let suffix = (hold as ::core::ffi::c_uint & mask).checked_shr(root_bits)?;
    usize::try_from((last.val as ::core::ffi::c_uint).wrapping_add(suffix)).ok()
}

/// Select the source and bounded progress for one ordinary-inflate match.
/// The legacy decoder still owns its ABI cursor lends and the bytewise copy
/// (output-backed matches deliberately overlap), but the distance and
/// circular-history arithmetic is ordinary checked scalar logic.  A malformed
/// compatibility window cannot therefore wrap into an arbitrary cursor before
/// the boundary has a chance to report the existing distance error.
#[derive(Copy, Clone)]
enum InflateMatchSource {
    Window { index: usize },
    Output { offset: usize },
}

#[derive(Copy, Clone)]
struct InflateMatchCopyPlan {
    source: InflateMatchSource,
    copy: ::core::ffi::c_uint,
    remaining_length: ::core::ffi::c_uint,
}

fn inflate_match_copy_plan(
    initial_output: ::core::ffi::c_uint,
    available_output: ::core::ffi::c_uint,
    offset: ::core::ffi::c_uint,
    length: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    window_present: bool,
    sane: bool,
) -> Option<InflateMatchCopyPlan> {
    let produced = initial_output.checked_sub(available_output)?;
    let (source, copy) = if offset > produced {
        if !window_present || wsize == 0 || wnext >= wsize || whave > wsize {
            return None;
        }
        let back = offset.checked_sub(produced)?;
        if sane && back > whave {
            return None;
        }
        let (index, copy) = if back > wnext {
            let copy = back.checked_sub(wnext)?;
            (wsize.checked_sub(copy)?, copy)
        } else {
            (wnext.checked_sub(back)?, back)
        };
        let wsize = usize::try_from(wsize).ok()?;
        let index = usize::try_from(index).ok()?;
        if index >= wsize {
            return None;
        }
        (InflateMatchSource::Window { index }, copy)
    } else {
        if offset == 0 {
            return None;
        }
        (
            InflateMatchSource::Output {
                offset: usize::try_from(offset).ok()?,
            },
            length,
        )
    };
    let copy = copy.min(length).min(available_output);
    if copy == 0 {
        return None;
    }
    Some(InflateMatchCopyPlan {
        source,
        copy,
        remaining_length: length.checked_sub(copy)?,
    })
}

/// Copy an output-backed match in distance-sized chunks. A single bulk copy
/// would not preserve deflate's repeated-pattern behavior when `len` exceeds
/// `distance`, so every source range must end at the output cursor that was
/// valid before the corresponding chunk is written.
pub(crate) fn inflate_output_match_copy(
    output: &mut [u8],
    distance: usize,
    len: usize,
) -> Option<()> {
    if distance == 0 || distance > output.len() {
        return None;
    }

    let mut output_at = distance;
    let mut remaining = len;
    while remaining != 0 {
        let take = remaining.min(distance);
        let source = output_at.checked_sub(distance)?;
        let source_end = source.checked_add(take)?;
        let destination_end = output_at.checked_add(take)?;
        if source_end > output_at || destination_end > output.len() {
            return None;
        }
        output.copy_within(source..source_end, output_at);
        output_at = destination_end;
        remaining = remaining.checked_sub(take)?;
    }
    Some(())
}

/// Decode zlib's overloaded `windowBits` argument without touching the ABI
/// stream or inflate state.  The boundary remains responsible for freeing a
/// mismatched history allocation and publishing the accepted settings.
pub(crate) fn inflate_window_bits(
    window_bits: ::core::ffi::c_int,
) -> Option<(::core::ffi::c_int, ::core::ffi::c_uint)> {
    if window_bits < -15 {
        return None;
    }
    let (wrap, bits) = if window_bits < 0 {
        (0, window_bits.checked_neg()?)
    } else {
        let wrap = (window_bits >> 4) + 5;
        let bits = if window_bits < 48 {
            window_bits & 15
        } else {
            window_bits
        };
        (wrap, bits)
    };
    (bits == 0 || (8..=15).contains(&bits)).then_some((wrap, bits as ::core::ffi::c_uint))
}

// This is expanded only in existing ABI/codec boundaries. It keeps raw
// stream/state adoption at those boundaries, while the validity decision
// itself remains the pointer-free scalar core above.
macro_rules! inflate_state_check_at_boundary {
    ($strm:expr) => {{
        let strm = $strm;
        if strm.is_null() {
            1
        } else {
            let strm_ref = &*strm;
            let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
            if state.is_null() {
                1
            } else {
                let state = &*state;
                (!crate::src::inflate::inflate_state_values_are_valid(
                    strm_ref.zalloc.is_some(),
                    strm_ref.zfree.is_some(),
                    state.stream_token == strm as usize,
                    state.mode,
                )) as ::core::ffi::c_int
            }
        }
    }};
}
pub(crate) use inflate_state_check_at_boundary;
// These reset/init transitions manipulate caller-owned stream state and may
// invoke its allocator.  Expand them only at ABI boundaries until that state
// has an owned Rust representation.
macro_rules! inflate_reset_keep_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let strm_ref = &mut *strm;
            let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
            let state_ref = &mut *state;
            state_ref.total = 0 as ::core::ffi::c_ulong;
            strm_ref.total_out = state_ref.total as crate::stdlib::uLong;
            strm_ref.total_in = strm_ref.total_out;
            strm_ref.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            strm_ref.data_type = 0 as ::core::ffi::c_int;
            if state_ref.wrap != 0 {
                strm_ref.adler = (state_ref.wrap & 1 as ::core::ffi::c_int) as crate::stdlib::uLong;
            }
            state_ref.mode = crate::src::inflate::HEAD;
            state_ref.last = 0 as ::core::ffi::c_int;
            state_ref.havedict = 0 as ::core::ffi::c_int;
            state_ref.flags = -1 as ::core::ffi::c_int;
            state_ref.dmax = 32768 as ::core::ffi::c_uint;
            state_ref.head = ::core::ptr::null_mut::<crate::zlib_h::gz_header>();
            state_ref.hold = 0 as ::core::ffi::c_ulong;
            state_ref.bits = 0 as ::core::ffi::c_uint;
            state_ref.next = 0;
            state_ref.distcode = crate::src::inflate::InflateCodeTable::Dynamic(0);
            state_ref.lencode = crate::src::inflate::InflateCodeTable::Dynamic(0);
            state_ref.sane = 1 as ::core::ffi::c_int;
            state_ref.back = -1 as ::core::ffi::c_int;
            crate::zlib_h::Z_OK
        }
    }};
}
pub(crate) use inflate_reset_keep_at_boundary;

macro_rules! inflate_reset_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            let strm_ref = &mut *strm;
            let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
            let state_ref = &mut *state;
            state_ref.wsize = 0 as ::core::ffi::c_uint;
            state_ref.whave = 0 as ::core::ffi::c_uint;
            state_ref.wnext = 0 as ::core::ffi::c_uint;
            crate::src::inflate::inflate_reset_keep_at_boundary!(strm)
        }
    }};
}
pub(crate) use inflate_reset_at_boundary;

macro_rules! inflate_reset2_at_boundary {
    ($strm:expr, $window_bits:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            match crate::src::inflate::inflate_window_bits($window_bits) {
                None => crate::zlib_h::Z_STREAM_ERROR,
                Some((wrap, window_bits)) => {
                    let (old_window, zfree, opaque) = {
                        let strm_ref = &mut *strm;
                        let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
                        let state_ref = &mut *state;
                        if !state_ref.window.is_null() && state_ref.wbits != window_bits {
                            (state_ref.window, strm_ref.zfree, strm_ref.opaque)
                        } else {
                            (
                                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                                None,
                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            )
                        }
                    };
                    if !old_window.is_null() {
                        if let Some(zfree) = zfree {
                            zfree(opaque, old_window as crate::stdlib::voidpf);
                            let strm_ref = &mut *strm;
                            let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
                            (&mut *state).window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        }
                    }
                    let strm_ref = &mut *strm;
                    let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
                    let state_ref = &mut *state;
                    state_ref.wrap = wrap;
                    state_ref.wbits = window_bits;
                    crate::src::inflate::inflate_reset_at_boundary!(strm)
                }
            }
        }
    }};
}
pub(crate) use inflate_reset2_at_boundary;

macro_rules! inflate_init2_at_boundary {
    ($strm:expr, $window_bits:expr, $version:expr, $stream_size:expr $(,)?) => {{
        let strm = $strm;
        let window_bits = $window_bits;
        let version = $version;
        let stream_size = $stream_size;
        if version.is_null()
            || *version as ::core::ffi::c_int
                != crate::zlib_h::ZLIB_VERSION[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
            || stream_size
                != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
        {
            crate::zlib_h::Z_VERSION_ERROR
        } else if strm.is_null() {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            // Snapshot callbacks before invoking either of them.  A custom
            // allocator is allowed to observe the public stream, so no Rust
            // borrow of it may remain live across the callback.
            let callbacks = {
                let strm_ref = &mut *strm;
                strm_ref.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if strm_ref.zalloc.is_none() {
                    strm_ref.zalloc = Some(
                        crate::src::zutil::zcalloc_ffi
                            as unsafe extern "C" fn(
                                crate::stdlib::voidpf,
                                ::core::ffi::c_uint,
                                ::core::ffi::c_uint,
                            )
                                -> crate::stdlib::voidpf,
                    ) as crate::zlib_h::alloc_func;
                    strm_ref.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
                }
                if strm_ref.zfree.is_none() {
                    strm_ref.zfree = Some(
                        crate::src::zutil::zcfree_ffi
                            as unsafe extern "C" fn(
                                crate::stdlib::voidpf,
                                crate::stdlib::voidpf,
                            ) -> (),
                    ) as crate::zlib_h::free_func;
                }
                match (strm_ref.zalloc, strm_ref.zfree) {
                    (Some(zalloc), Some(zfree)) => Some((zalloc, zfree, strm_ref.opaque)),
                    _ => None,
                }
            };
            if let Some((zalloc, zfree, opaque)) = callbacks {
                let state = zalloc(
                    opaque,
                    1 as crate::stdlib::uInt,
                    ::core::mem::size_of::<crate::src::inflate::inflate_state>()
                        as crate::stdlib::uInt,
                ) as *mut crate::src::inflate::inflate_state;
                if state.is_null() {
                    crate::zlib_h::Z_MEM_ERROR
                } else {
                    ::core::ptr::write(state, crate::src::inflate::inflate_initial_state());
                    {
                        let strm_ref = &mut *strm;
                        let state_ref = &mut *state;
                        strm_ref.state = state as *mut crate::src::deflate::internal_state;
                        state_ref.stream_token = strm as usize;
                    }
                    let ret = crate::src::inflate::inflate_reset2_at_boundary!(strm, window_bits);
                    if ret != crate::zlib_h::Z_OK {
                        zfree(opaque, state as crate::stdlib::voidpf);
                        (&mut *strm).state =
                            ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
                    }
                    ret
                }
            } else {
                crate::zlib_h::Z_STREAM_ERROR
            }
        }
    }};
}
pub(crate) use inflate_init2_at_boundary;

#[export_name = "inflateResetKeep"]

pub unsafe extern "C" fn inflateResetKeep_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflate_reset_keep_at_boundary!(strm)
}
#[export_name = "inflateReset"]

pub unsafe extern "C" fn inflateReset_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    inflate_reset_at_boundary!(strm)
}
#[export_name = "inflateReset2"]

pub unsafe extern "C" fn inflateReset2_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_reset2_at_boundary!(strm, windowBits)
}
#[export_name = "inflateInit2_"]

pub unsafe extern "C" fn inflateInit2__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_init2_at_boundary!(strm, windowBits, version, stream_size)
}
#[export_name = "inflateInit_"]

pub unsafe extern "C" fn inflateInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    inflate_init2_at_boundary!(strm, crate::zutil_h::DEF_WBITS, version, stream_size)
}
/// Apply an `inflatePrime()` request to scalar bit-buffer state.  The export
/// boundary validates and owns the stream state; this core preserves zlib's
/// bit masking and wrapping arithmetic without accessing raw state.
fn inflate_prime_update(
    hold: ::core::ffi::c_ulong,
    held_bits: ::core::ffi::c_uint,
    bits: ::core::ffi::c_int,
    value: ::core::ffi::c_int,
) -> Result<(::core::ffi::c_ulong, ::core::ffi::c_uint), ()> {
    if bits == 0 {
        return Ok((hold, held_bits));
    }
    if bits < 0 {
        return Ok((0, 0));
    }
    if bits > 16 || held_bits.wrapping_add(bits as ::core::ffi::c_uint) > 32 {
        return Err(());
    }
    let value = (value as ::core::ffi::c_long
        & ((1 as ::core::ffi::c_long) << bits) - 1 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
    Ok((
        hold.wrapping_add((value as ::core::ffi::c_ulong) << held_bits),
        held_bits.wrapping_add(bits as ::core::ffi::c_uint),
    ))
}

#[export_name = "inflatePrime"]

pub unsafe extern "C" fn inflatePrime_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    let Ok((hold, held_bits)) = inflate_prime_update(state.hold, state.bits, bits, value) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    state.hold = hold;
    state.bits = held_bits;
    crate::zlib_h::Z_OK
}
/// The two copy operations needed to append decoded bytes to inflate's
/// circular history window.  This is deliberately pointer-free: the caller
/// owns the ABI allocation and performs the copies only after this plan has
/// proved the window cursor and lengths are coherent.
#[derive(Copy, Clone)]
struct InflateWindowCopyPlan {
    first_dest: usize,
    first_from_end: usize,
    first_len: usize,
    second_from_end: usize,
    second_len: usize,
    next: usize,
    have: usize,
}

impl InflateWindowCopyPlan {
    fn cursor_values(self) -> Option<(::core::ffi::c_uint, ::core::ffi::c_uint)> {
        Some((
            ::core::ffi::c_uint::try_from(self.next).ok()?,
            ::core::ffi::c_uint::try_from(self.have).ok()?,
        ))
    }
}

/// Compute the circular-window update without touching the ABI window
/// pointer.  `copy` is the number of bytes immediately preceding `end`.
/// Invalid internal cursor state is rejected before the boundary performs a
/// pointer offset or memory copy.
fn inflate_window_copy_plan(
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    copy: ::core::ffi::c_uint,
) -> Option<InflateWindowCopyPlan> {
    let wsize = usize::try_from(wsize).ok()?;
    let wnext = usize::try_from(wnext).ok()?;
    let whave = usize::try_from(whave).ok()?;
    let copy = usize::try_from(copy).ok()?;
    if wsize == 0 || wnext > wsize || whave > wsize {
        return None;
    }

    if copy >= wsize {
        return Some(InflateWindowCopyPlan {
            first_dest: 0,
            first_from_end: wsize,
            first_len: wsize,
            second_from_end: 0,
            second_len: 0,
            next: 0,
            have: wsize,
        });
    }

    let first_len = wsize.checked_sub(wnext)?.min(copy);
    let remaining = copy.checked_sub(first_len)?;
    let (next, have) = if remaining != 0 {
        (remaining, wsize)
    } else {
        let next = wnext.checked_add(first_len)?;
        let next = if next == wsize { 0 } else { next };
        let have = whave.checked_add(first_len)?.min(wsize);
        (next, have)
    };

    Some(InflateWindowCopyPlan {
        first_dest: wnext,
        first_from_end: copy,
        first_len,
        second_from_end: remaining,
        second_len: remaining,
        next,
        have,
    })
}

/// Copy the just-produced output tail into the circular history window.  The
/// codec boundary lends the two validated slices; all tail/destination range
/// arithmetic remains here, where it is checked before either copy.
fn inflate_window_copy(
    window: &mut [u8],
    produced: &[u8],
    plan: InflateWindowCopyPlan,
) -> Option<()> {
    let first_start = produced.len().checked_sub(plan.first_from_end)?;
    let first_end = first_start.checked_add(plan.first_len)?;
    let first = produced.get(first_start..first_end)?;
    let first_dest_end = plan.first_dest.checked_add(first.len())?;
    window
        .get_mut(plan.first_dest..first_dest_end)?
        .copy_from_slice(first);

    let second_start = produced.len().checked_sub(plan.second_from_end)?;
    let second_end = second_start.checked_add(plan.second_len)?;
    let second = produced.get(second_start..second_end)?;
    window.get_mut(..second.len())?.copy_from_slice(second);
    Some(())
}

/// Resolve the physical history-window length before the ABI boundary lends
/// the allocation as a slice.  A zero `wsize` is the pre-allocation state;
/// once a window exists, its recorded size must still fit in `usize`.
///
/// zlib only supports 8--15 window bits.  Keeping that bound here prevents a
/// malformed opaque state from requesting a wrapped or impractically large
/// compatibility allocation before the safe copy core gets a chance to
/// validate it.
fn inflate_window_len(wbits: ::core::ffi::c_uint, wsize: ::core::ffi::c_uint) -> Option<usize> {
    if !(8..=15).contains(&wbits) {
        return None;
    }
    if wsize == 0 {
        usize::try_from(1_u32.checked_shl(wbits)?).ok()
    } else {
        usize::try_from(wsize).ok()
    }
}

/// The scalar state to publish after a successful circular history update.
/// Keeping this separate from the copy lets the safe core validate and copy
/// before its caller commits any compatibility-state fields.
#[derive(Copy, Clone)]
struct InflateWindowUpdate {
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
}

/// The complete scalar admission record for the transitional history-window
/// boundary.  In particular, a missing allocation is only compatible with
/// the pre-allocation `wsize == 0` state.  Keeping that relationship out of
/// the raw callback/slice bridge prevents a malformed state from allocating
/// one span and then lending a differently sized prefix of it.
#[derive(Copy, Clone)]
struct InflateWindowBoundaryPlan {
    window_len: usize,
    copy_len: usize,
    allocate: bool,
}

fn inflate_window_boundary_plan(
    window_is_null: bool,
    wbits: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    copy: ::core::ffi::c_uint,
) -> Option<InflateWindowBoundaryPlan> {
    let window_len = if window_is_null {
        // A nonzero compatibility size promises that a matching allocation
        // has already been published.  Do not replace it with a fresh,
        // differently sized allocation when that promise is broken.
        (wsize == 0).then(|| inflate_window_len(wbits, 0))??
    } else {
        inflate_window_len(wbits, wsize)?
    };
    Some(InflateWindowBoundaryPlan {
        window_len,
        copy_len: usize::try_from(copy).ok()?,
        allocate: window_is_null,
    })
}

/// Update the circular history window after a decoder call. Allocation and
/// ABI-owned buffer lending stay at the codec boundary; this core owns the
/// window sizing, cursor planning, and bounded copies, returning the scalar
/// state to commit only after all validation and copying succeeds.
fn inflate_window_update(
    window: &mut [u8],
    produced: &[u8],
    wbits: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
) -> Option<InflateWindowUpdate> {
    let wsize = if wsize == 0 {
        ::core::ffi::c_uint::try_from(inflate_window_len(wbits, 0)?).ok()?
    } else {
        wsize
    };
    let wsize_len = inflate_window_len(wbits, wsize)?;
    if window.len() != wsize_len {
        return None;
    }
    let copy = ::core::ffi::c_uint::try_from(produced.len()).ok()?;
    let plan = inflate_window_copy_plan(wsize, wnext, whave, copy)?;
    let (next, have) = plan.cursor_values()?;
    inflate_window_copy(window, produced, plan)?;
    Some(InflateWindowUpdate {
        wsize,
        wnext: next,
        whave: have,
    })
}

/// Retain a decoder call's output in its circular history window and publish
/// the new history cursors as one safe commit.  The caller still owns the ABI
/// allocation and the temporary slice lend, but cannot publish a partially
/// updated history state if the checked copy rejects malformed cursors.
fn inflate_window_update_state(
    state: &mut crate::src::inflate::inflate_state,
    window: &mut [u8],
    produced: &[u8],
) -> Option<()> {
    let update = inflate_window_update(
        window,
        produced,
        state.wbits,
        state.wsize,
        state.wnext,
        state.whave,
    )?;
    state.wsize = update.wsize;
    state.wnext = update.wnext;
    state.whave = update.whave;
    Some(())
}

/// Resolve a table source to a bounded view. Dynamic tables live in `codes`;
/// fixed tables are immutable static data.
pub(crate) fn inflate_table_view(
    codes: &[crate::src::inftrees::code],
    table: InflateCodeTable,
) -> Option<&[crate::src::inftrees::code]> {
    match table {
        InflateCodeTable::Empty => None,
        InflateCodeTable::FixedLens => Some(&crate::src::inftrees::inffixed_h::lenfix),
        InflateCodeTable::FixedDists => Some(&crate::src::inftrees::inffixed_h::distfix),
        InflateCodeTable::Dynamic(index) => codes.get(index..),
    }
}

/// Read one active decode-table entry without reconstructing an interior
/// pointer into the state-owned dynamic table.
pub(crate) fn inflate_table_entry(
    codes: &[crate::src::inftrees::code],
    table: InflateCodeTable,
    index: usize,
) -> Option<crate::src::inftrees::code> {
    inflate_table_view(codes, table)?.get(index).copied()
}

/// Resolve the two active decode-table sources to their bounded table tails.
/// The codec boundary uses this only to lend the safe fast decoder table views.
pub(crate) fn inflate_fast_tables(
    codes: &[crate::src::inftrees::code],
    lencode: InflateCodeTable,
    distcode: InflateCodeTable,
) -> Option<(&[crate::src::inftrees::code], &[crate::src::inftrees::code])> {
    Some((
        inflate_table_view(codes, lencode)?,
        inflate_table_view(codes, distcode)?,
    ))
}

/// Run normal inflate's bounded fast decoder after its ABI boundary has
/// validated and lent the input, output, and history spans.  Table-token
/// resolution and the complete scalar decoder configuration live here so the
/// legacy loop only performs the temporary boundary conversion and commits
/// the resulting progress.
///
/// `output` and `window` must not overlap.  Normal inflate owns a separate
/// history window; unlike inflateBack, it can therefore lend both views to
/// the slice-only core at once.
#[allow(clippy::too_many_arguments)]
fn inflate_fast_normal(
    input: &[u8],
    output: &mut [u8],
    output_start: usize,
    window: &[u8],
    lcode: &[crate::src::inftrees::code],
    dcode: &[crate::src::inftrees::code],
    wsize: usize,
    whave: usize,
    wnext: usize,
    sane: bool,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
    lenbits: ::core::ffi::c_uint,
    distbits: ::core::ffi::c_uint,
    start: ::core::ffi::c_uint,
) -> crate::src::inffast::InflateFastProgress {
    crate::src::inffast::inflate_fast_core(crate::src::inffast::InflateFastViews {
        input,
        output,
        output_start,
        history: crate::src::inffast::InflateFastHistory::Separate(window),
        lcode,
        dcode,
        wsize,
        whave,
        wnext,
        sane,
        hold,
        bits,
        lenbits,
        distbits,
        start,
    })
}

/// The scalar shape of ordinary inflate's fast-decoder lends.  The legacy
/// decoder still owns the ABI cursor conversion, but it must establish all
/// of these bounds before making its temporary input, output, and history
/// views.
#[derive(Copy, Clone)]
struct InflateFastSpan {
    input_len: usize,
    output_start: usize,
    output_len: usize,
    window_len: usize,
}

/// Validate the spans used by normal inflate's bounded fast path without
/// looking through compatibility pointers.  `out` is the output capacity at
/// entry and `left` is the remaining capacity at the fast-path boundary, so
/// their checked difference is the output already produced by the slow path.
fn inflate_fast_span(
    have: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
    out: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    whave: ::core::ffi::c_uint,
    wnext: ::core::ffi::c_uint,
) -> Option<InflateFastSpan> {
    if have < 6 || left < 258 {
        return None;
    }
    let output_start = usize::try_from(out.checked_sub(left)?).ok()?;
    let output_len = usize::try_from(out).ok()?;
    let window_len = usize::try_from(wsize).ok()?;
    let history_len = usize::try_from(whave).ok()?;
    let history_next = usize::try_from(wnext).ok()?;
    if output_start > output_len {
        return None;
    }
    // Validate the history cursors before the transitional boundary lends its
    // callback-owned window. `inflate_fast_core()` enforces the same
    // invariants, but doing the scalar check first avoids forming a temporary
    // view for a malformed compatibility state.
    if history_len > window_len
        || window_len == 0 && history_next != 0
        || window_len != 0 && history_next >= window_len
    {
        return None;
    }
    Some(InflateFastSpan {
        input_len: usize::try_from(have).ok()?,
        output_start,
        output_len,
        window_len,
    })
}

/// A checked, pointer-free commit plan for one bounded fast-decode call.
/// The transitional decoder keeps compatibility cursors at its boundary, but
/// it can validate all scalar progress before publishing any of them.
struct InflateFastCommit {
    input_used: ::core::ffi::c_uint,
    output_used: ::core::ffi::c_uint,
    hold: crate::stdlib::uLong,
    bits: ::core::ffi::c_uint,
    mode: Option<inflate_mode>,
    error: Option<usize>,
}

fn inflate_fast_commit(
    progress: crate::src::inffast::InflateFastProgress,
    available_input: ::core::ffi::c_uint,
    available_output: ::core::ffi::c_uint,
) -> Option<InflateFastCommit> {
    let input_used = ::core::ffi::c_uint::try_from(progress.input_used).ok()?;
    let output_used = ::core::ffi::c_uint::try_from(progress.output_used).ok()?;
    if input_used > available_input || output_used > available_output {
        return None;
    }
    Some(InflateFastCommit {
        input_used,
        output_used,
        hold: crate::stdlib::uLong::try_from(progress.hold).ok()?,
        bits: ::core::ffi::c_uint::try_from(progress.bits).ok()?,
        mode: progress.mode,
        error: progress.error,
    })
}

/// Scalar state published when one ordinary `inflate()` call leaves its
/// legacy cursor loop.  Keeping these calculations independent of ABI
/// records makes the exit boundary responsible only for cursor lending,
/// checksum input, and field commits.
#[derive(Copy, Clone)]
struct InflateExitProgress {
    input_used: ::core::ffi::c_uint,
    output_used: ::core::ffi::c_uint,
    update_window: bool,
    data_type: ::core::ffi::c_int,
}

/// Preserve ordinary inflate's exit accounting without looking through the
/// compatibility stream or state records.  The decoder only decrements the
/// two availability counters, so wrapping subtraction retains the translated
/// ABI behavior even if a malformed caller supplied unusual scalar values.
fn inflate_exit_progress(
    initial_input: ::core::ffi::c_uint,
    remaining_input: ::core::ffi::c_uint,
    initial_output: ::core::ffi::c_uint,
    remaining_output: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
    mode: inflate_mode,
    flush: ::core::ffi::c_int,
    bits: ::core::ffi::c_uint,
    last: ::core::ffi::c_int,
) -> InflateExitProgress {
    let input_used = initial_input.wrapping_sub(remaining_input);
    let output_used = initial_output.wrapping_sub(remaining_output);
    let mode_value = mode as ::core::ffi::c_uint;
    let update_window = wsize != 0
        || output_used != 0
            && mode_value < crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
            && (mode_value
                < crate::src::inflate::CHECK as ::core::ffi::c_int as ::core::ffi::c_uint
                || flush != crate::zlib_h::Z_FINISH);
    let data_type = bits as ::core::ffi::c_int
        + if last != 0 { 64 } else { 0 }
        + if mode_value == crate::src::inflate::TYPE as ::core::ffi::c_int as ::core::ffi::c_uint {
            128
        } else {
            0
        }
        + if mode_value == crate::src::inflate::LEN_ as ::core::ffi::c_int as ::core::ffi::c_uint
            || mode_value == crate::src::inflate::COPY_ as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            256
        } else {
            0
        };
    InflateExitProgress {
        input_used,
        output_used,
        update_window,
        data_type,
    }
}

/// Validate ordinary `inflate()`'s scalar entry relationship before the
/// legacy cursor loop adopts its compatibility records.  Keeping this policy
/// free of ABI pointers makes the boundary's only jobs pointer validation,
/// temporary record adoption, and cursor setup.
fn inflate_entry_mode(
    has_zalloc: bool,
    has_zfree: bool,
    state_belongs_to_stream: bool,
    state_mode: inflate_mode,
    output_present: bool,
    input_is_valid: bool,
) -> Option<inflate_mode> {
    if !inflate_state_values_are_valid(has_zalloc, has_zfree, state_belongs_to_stream, state_mode)
        || !output_present
        || !input_is_valid
    {
        return None;
    }
    Some(if state_mode == crate::src::inflate::TYPE {
        crate::src::inflate::TYPEDO
    } else {
        state_mode
    })
}

/// Read the byte just consumed by ordinary inflate's bit cursor. `have` never
/// exceeds the availability snapshot and is reduced only after the decoder has
/// established that a byte is present. Keeping that relationship as slice
/// indexing lets the header and bit cursor avoid raw dereferences.
fn inflate_input_at(
    input: &[crate::stdlib::Bytef],
    initial_have: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    offset: usize,
) -> crate::stdlib::Bytef {
    input[(initial_have as usize - have as usize - 1) + offset]
}

/// Read from the current input cursor before that cursor's availability has
/// been reduced, for bounded header-copy and checksum scans.
fn inflate_input_from_cursor(
    input: &[crate::stdlib::Bytef],
    initial_have: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    offset: usize,
) -> crate::stdlib::Bytef {
    input[(initial_have as usize - have as usize) + offset]
}

pub fn inflate(
    strm_ref: &mut crate::zlib_h::z_stream,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // Retain a raw alias only inside this transitional codec boundary. Rust
    // callers pass the validated stream reference directly; ABI pointer
    // adoption is confined to `inflate_ffi()`.
    let mut strm = strm_ref as *mut crate::zlib_h::z_stream;
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut put: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut have: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut in_0: ::core::ffi::c_uint = 0;
    let mut out: ::core::ffi::c_uint = 0;
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
    let mut hbuf: [::core::ffi::c_uchar; 4] = [0; 4];
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
    // This transitional compatibility boundary owns the legacy decoder's
    // raw stream/state adoption and cursor work.  Keeping it explicit means
    // Rust callers of the dispatcher do not inherit an unsafe-function
    // contract while the safe owned/slice core is still being extracted.
    unsafe {
        // Keep the pointer checks here, then hand the scalar relationship to the
        // safe validator shared by the smaller inflate boundaries.
        let strm_ref = &mut *strm;
        let state_ref = {
            let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
            if state.is_null() {
                return crate::zlib_h::Z_STREAM_ERROR;
            }
            &mut *state
        };
        let Some(entry_mode) = inflate_entry_mode(
            strm_ref.zalloc.is_some(),
            strm_ref.zfree.is_some(),
            state_ref.stream_token == strm as usize,
            state_ref.mode,
            !strm_ref.next_out.is_null(),
            !strm_ref.next_in.is_null() || strm_ref.avail_in == 0 as crate::stdlib::uInt,
        ) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        state = state_ref as *mut crate::src::inflate::inflate_state;
        state_ref.mode = entry_mode;
        put = strm_ref.next_out as *mut ::core::ffi::c_uchar;
        left = strm_ref.avail_out as ::core::ffi::c_uint;
        next = strm_ref.next_in as *mut ::core::ffi::c_uchar;
        have = strm_ref.avail_in as ::core::ffi::c_uint;
        hold = state_ref.hold;
        bits = state_ref.bits;
        in_0 = have;
        out = left;
        ret = crate::zlib_h::Z_OK;
        {
            // Lend the immutable input, mutable output, and separate history
            // allocation once for this decoder invocation.  The loop neither
            // invokes callbacks nor reallocates the history window.  This
            // scope ends before the exit boundary, where allocation callbacks
            // are again possible.
            let input = ::core::slice::from_raw_parts(next, have as usize);
            let output = ::core::slice::from_raw_parts_mut(put, left as usize);
            let window = if state_ref.window.is_null() {
                None
            } else {
                Some(::core::slice::from_raw_parts(
                    state_ref.window,
                    state_ref.wsize as usize,
                ))
            };
            '_inf_leave: loop {
            'c_2425: {
                'c_2327: {
                    'c_2422: {
                        'c_2325: {
                            's_2462: {
                                'c_2322: {
                                    'c_2410: {
                                        'c_2319: {
                                            'c_2398: {
                                                'c_2397: {
                                                    'c_2317: {
                                                        'c_2340: {
                                                            'c_2443: {
                                                                'c_2339: {
                                                                    's_519: {
                                                                        'c_2356: {
                                                                            's_1689: {
                                                                                'c_2355: {
                                                                                    's_425: {
                                                                                        'c_2336: {
                                                                                            's_1582: {
                                                                                                // Snapshot the dispatcher mode through a
                                                                                                // short-lived state borrow.  Individual
                                                                                                // transitions continue to adopt the state
                                                                                                // only for their own commits.
                                                                                                let mode = {
                                                                                                let state_ref = &*state;
                                                                                                state_ref.mode
                                                                                            };
                                                                                                match mode as ::core::ffi::c_uint {
                                                                                                16180 => {
                                                                                                    // The decoder entry already validated both
                                                                                                    // compatibility records.  Keep the header
                                                                                                    // transition on one short-lived borrow instead
                                                                                                    // of repeatedly traversing their raw pointers.
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
                                                                                                    if state_ref.wrap == 0 as ::core::ffi::c_int {
                                                                                                        state_ref.mode = crate::src::inflate::TYPEDO;
                                                                                                        continue '_inf_leave;
                                                                                                    } else {
                                                                                                        while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if have == 0 as ::core::ffi::c_uint {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            have = have.wrapping_sub(1);
                                                                                                            next = next.wrapping_add(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                    (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        if state_ref.wrap & 2 as ::core::ffi::c_int != 0
                                                                                                            && hold == 0x8b1f as ::core::ffi::c_ulong
                                                                                                        {
                                                                                                            if state_ref.wbits == 0 as ::core::ffi::c_uint {
                                                                                                                state_ref.wbits = 15 as ::core::ffi::c_uint;
                                                                                                            }
                                                                                                            state_ref.check = inflate_header_crc_update(0, &[]);
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            state_ref.check = inflate_header_crc_update(
                                                                                                                state_ref.check,
                                                                                                                &hbuf[..2],
                                                                                                            );
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                            state_ref.mode = crate::src::inflate::FLAGS;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            if !state_ref.head.is_null() {
                                                                                                                // The retained gzip header remains an ABI
                                                                                                                // destination, but this no-callback transition
                                                                                                                // only needs a short-lived borrow to publish its
                                                                                                                // initial incomplete status.
                                                                                                                let head = &mut *state_ref.head;
                                                                                                                head.done = -1 as ::core::ffi::c_int;
                                                                                                            }
                                                                                                            let header_plan = match inflate_zlib_header_plan(
                                                                                                               state_ref.wrap,
                                                                                                               state_ref.wbits,
                                                                                                               hold,
                                                                                                            ) {
                                                                                                                Ok(plan) => plan,
                                                                                                                Err(error) => {
                                                                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[error].as_ptr()
                                                                                                                   as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                                    state_ref.mode = crate::src::inflate::BAD;
                                                                                                                    continue '_inf_leave;
                                                                                                                }
                                                                                                            };
                                                                                                               hold >>= 4 as ::core::ffi::c_int;
                                                                                                               bits = bits
                                                                                                                   .wrapping_sub(
                                                                                                                       4 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                                   );
                                                                                                                state_ref.wbits = header_plan.wbits;
                                                                                                                state_ref.dmax = header_plan.dmax;
                                                                                                                state_ref.flags = 0 as ::core::ffi::c_int;
                                                                                                                // zlib defines the checksum of an empty
                                                                                                                // stream directly.  Do not route this
                                                                                                                // through the raw-pointer ABI adapter.
                                                                                                                state_ref.check = crate::src::adler32::ADLER32_INITIAL
                                                                                                                    as ::core::ffi::c_ulong;
                                                                                                                strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                                                                                                               state_ref.mode = header_plan.mode;
                                                                                                               hold = 0 as ::core::ffi::c_ulong;
                                                                                                               bits = 0 as ::core::ffi::c_uint;
                                                                                                               continue '_inf_leave;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                16181 => {
                                                                                                    // FLAGS follows HEAD without any cursor lend or
                                                                                                    // callback.  Keep its state/message commits on
                                                                                                    // the same kind of short-lived boundary borrow.
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
                                                                                                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                                            );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    let flags_plan = match inflate_gzip_flags_plan(
                                                                                                        state_ref.wrap,
                                                                                                        hold,
                                                                                                    ) {
                                                                                                        Ok(plan) => plan,
                                                                                                        Err(error) => {
                                                                                                            strm_ref.msg = INFLATE_ERROR_MESSAGES[error].as_ptr()
                                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                            state_ref.mode = crate::src::inflate::BAD;
                                                                                                            continue '_inf_leave;
                                                                                                        }
                                                                                                    };
                                                                                                    state_ref.flags = flags_plan.flags;
                                                                                                    {
                                                                                                       if !state_ref.head.is_null() {
                                                                                                            let head = &mut *state_ref.head;
                                                                                                            head.text = flags_plan.text;
                                                                                                       }
                                                                                                        if flags_plan.update_crc {
                                                                                                            hbuf[0 as ::core::ffi::c_int as usize] = hold
                                                                                                                as ::core::ffi::c_uchar;
                                                                                                            hbuf[1 as ::core::ffi::c_int as usize] = (hold
                                                                                                                >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                                                                                                            state_ref.check = inflate_header_crc_update(
                                                                                                                state_ref.check,
                                                                                                                &hbuf[..2],
                                                                                                            );
                                                                                                        }
                                                                                                        hold = 0 as ::core::ffi::c_ulong;
                                                                                                        bits = 0 as ::core::ffi::c_uint;
                                                                                                        state_ref.mode = crate::src::inflate::TIME;
                                                                                                        break 's_425;
                                                                                                    }
                                                                                                }
                                                                                                16182 => {
                                                                                                    break 's_425;
                                                                                                }
                                                                                                16183 => {
                                                                                                    break 's_519;
                                                                                                }
                                                                                                16184 => {
                                                                                                    break 'c_2317;
                                                                                                }
                                                                                                16185 => {
                                                                                                    break 'c_2319;
                                                                                                }
                                                                                                16186 => {
                                                                                                    break 'c_2322;
                                                                                                }
                                                                                                16187 => {
                                                                                                    break 'c_2325;
                                                                                                }
                                                                                                16188 => {
                                                                                                    break 'c_2327;
                                                                                                }
                                                                                                16189 => {
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    // This transition only commits the
                                                                                                    // already-consumed dictionary id. Keep the
                                                                                                    // stream/state adoption local instead of
                                                                                                    // repeatedly traversing both ABI records.
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
                                                                                                    state_ref.check = inflate_dictionary_id(hold);
                                                                                                    strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                    state_ref.mode = crate::src::inflate::DICT;
                                                                                                    break 'c_2336;
                                                                                                }
                                                                                                16190 => {
                                                                                                    break 'c_2336;
                                                                                                }
                                                                                                16191 => {
                                                                                                    break 'c_2339;
                                                                                                }
                                                                                                16192 => {
                                                                                                    break 'c_2340;
                                                                                                }
                                                                                                16193 => {
                                                                                                    hold >>= bits & 7 as ::core::ffi::c_uint;
                                                                                                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
                                                                                                    let Some(length) = inflate_stored_block_len(hold) else {
                                                                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[4].as_ptr()
                                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                        state_ref.mode = crate::src::inflate::BAD;
                                                                                                        continue '_inf_leave;
                                                                                                    };
                                                                                                    state_ref.length = length;
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                    state_ref.mode = crate::src::inflate::COPY_;
                                                                                                    if flush == crate::zlib_h::Z_TREES {
                                                                                                        break '_inf_leave;
                                                                                                    } else {
                                                                                                        break 'c_2355;
                                                                                                    }
                                                                                                }
                                                                                                16194 => {
                                                                                                    break 'c_2355;
                                                                                                }
                                                                                                16195 => {
                                                                                                    break 'c_2356;
                                                                                                }
                                                                                                16196 => {
                                                                                                    while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                    {
                                                                                                        if have == 0 as ::core::ffi::c_uint {
                                                                                                            break '_inf_leave;
                                                                                                        }
                                                                                                        have = have.wrapping_sub(1);
                                                                                                        next = next.wrapping_add(1);
                                                                                                        hold = hold
                                                                                                            .wrapping_add(
                                                                                                                (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                    }
                                                                                                    // Dynamic-table header parsing has no
                                                                                                    // callbacks or buffer lends. Adopt the two
                                                                                                    // compatibility records once for all of its
                                                                                                    // scalar commits and diagnostics.
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
                                                                                                    let Some(plan) = inflate_table_header_plan(hold, bits) else {
                                                                                                       strm_ref.msg = INFLATE_ERROR_MESSAGES[5]
                                                                                                           .as_ptr() as *const ::core::ffi::c_char
                                                                                                           as *mut ::core::ffi::c_char;
                                                                                                       state_ref.mode = crate::src::inflate::BAD;
                                                                                                       continue '_inf_leave;
                                                                                                    };
                                                                                                    state_ref.nlen = plan.nlen;
                                                                                                    state_ref.ndist = plan.ndist;
                                                                                                    state_ref.ncode = plan.ncode;
                                                                                                    hold = plan.hold;
                                                                                                    bits = plan.bits;
                                                                                                    state_ref.have = 0 as ::core::ffi::c_uint;
                                                                                                    state_ref.mode = crate::src::inflate::LENLENS;
                                                                                                    break 's_1582;
                                                                                                }
                                                                                                16197 => {
                                                                                                    break 's_1582;
                                                                                                }
                                                                                                16198 => {
                                                                                                    break 's_1689;
                                                                                                }
                                                                                                16199 => {
                                                                                                    break 'c_2397;
                                                                                                }
                                                                                                16200 => {
                                                                                                    break 'c_2398;
                                                                                                }
                                                                                                16201 => {
                                                                                                    break 'c_2410;
                                                                                                }
                                                                                                16202 => {
                                                                                                    break 's_2462;
                                                                                                }
                                                                                                16203 => {
                                                                                                    break 'c_2422;
                                                                                                }
                                                                                                16204 => {
                                                                                                    break 'c_2425;
                                                                                                }
                                                                                                16205 => {
                                                                                                    if left == 0 as ::core::ffi::c_uint {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                    // Literal emission only touches the existing
                                                                                                    // output cursor and decoder scalar state. Keep
                                                                                                    // its state read/commit on one transition-local
                                                                                                    // borrow instead of traversing the compatibility
                                                                                                    // pointer for each field.
                                                                                                    let state_ref = &mut *state;
                                                                                                    let Some(output_index) = out
                                                                                                        .checked_sub(left)
                                                                                                        .and_then(|offset| usize::try_from(offset).ok())
                                                                                                    else {
                                                                                                        state_ref.mode = crate::src::inflate::BAD;
                                                                                                        ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                                        break '_inf_leave;
                                                                                                    };
                                                                                                    let Some(destination) = output.get_mut(output_index) else {
                                                                                                        state_ref.mode = crate::src::inflate::BAD;
                                                                                                        ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                                        break '_inf_leave;
                                                                                                    };
                                                                                                    *destination = state_ref.length as ::core::ffi::c_uchar;
                                                                                                    put = put.wrapping_add(1);
                                                                                                    left = left.wrapping_sub(1);
                                                                                                    state_ref.mode = crate::src::inflate::LEN;
                                                                                                    continue '_inf_leave;
                                                                                                }
                                                                                                16206 => {
                                                                                                    // The decoder entry already adopted both
                                                                                                    // compatibility records. Keep trailer
                                                                                                    // accounting and the checksum commit in
                                                                                                    // this transition-local borrow rather than
                                                                                                    // repeatedly traversing the raw pointers.
                                                                                                    let strm_ref = &mut *strm;
                                                                                                    let state_ref = &mut *state;
                                                                                                    if state_ref.wrap != 0 {
                                                                                                        while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                        {
                                                                                                            if have == 0 as ::core::ffi::c_uint {
                                                                                                                break '_inf_leave;
                                                                                                            }
                                                                                                            have = have.wrapping_sub(1);
                                                                                                            next = next.wrapping_add(1);
                                                                                                            hold = hold
                                                                                                                .wrapping_add(
                                                                                                                    (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                                                );
                                                                                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                        }
                                                                                                        out = out.wrapping_sub(left);
                                                                                                        strm_ref.total_out = strm_ref
                                                                                                            .total_out
                                                                                                            .wrapping_add(out as crate::stdlib::uLong);
                                                                                                        state_ref.total = state_ref
                                                                                                            .total
                                                                                                            .wrapping_add(out as ::core::ffi::c_ulong);
                                                                                                        if state_ref.wrap & 4 as ::core::ffi::c_int != 0
                                                                                                            && out != 0
                                                                                                        {
                                                                                                            let Some(output) = output.get(..out as usize) else {
                                                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                                                break '_inf_leave;
                                                                                                            };
                                                                                                            if let Some(check) = inflate_exit_checksum(
                                                                                                                state_ref.wrap,
                                                                                                                state_ref.check as crate::stdlib::uLong,
                                                                                                                state_ref.flags,
                                                                                                                output,
                                                                                                            ) {
                                                                                                                state_ref.check = check;
                                                                                                                strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                                                                                                            }
                                                                                                        }
                                                                                                        out = left;
                                                                                                        if !inflate_trailer_checksum_matches(
                                                                                                            state_ref.wrap,
                                                                                                            state_ref.flags,
                                                                                                            hold,
                                                                                                            state_ref.check,
                                                                                                        ) {
                                                                                                            strm_ref.msg = INFLATE_ERROR_MESSAGES[6].as_ptr()
                                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                            state_ref.mode = crate::src::inflate::BAD;
                                                                                                            continue '_inf_leave;
                                                                                                        } else {
                                                                                                            hold = 0 as ::core::ffi::c_ulong;
                                                                                                            bits = 0 as ::core::ffi::c_uint;
                                                                                                        }
                                                                                                    }
                                                                                                    state_ref.mode = crate::src::inflate::LENGTH;
                                                                                                }
                                                                                                16207 => {}
                                                                                                16208 => {
                                                                                                    break 'c_2443;
                                                                                                }
                                                                                                16209 => {
                                                                                                    ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                16210 => return crate::zlib_h::Z_MEM_ERROR,
                                                                                                16211 | _ => return crate::zlib_h::Z_STREAM_ERROR,
                                                                                            }
                                                                                                // This gzip-length and code-length-table
                                                                                                // transition only updates decoder scalars
                                                                                                // after consuming its local cursor. Adopt
                                                                                                // the compatibility records once rather
                                                                                                // than repeatedly traversing raw state.
                                                                                                let strm_ref = &mut *strm;
                                                                                                let state_ref = &mut *state;
                                                                                                if state_ref.wrap != 0 && state_ref.flags != 0 {
                                                                                                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                                {
                                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                                        break '_inf_leave;
                                                                                                    }
                                                                                                    have = have.wrapping_sub(1);
                                                                                                    next = next.wrapping_add(1);
                                                                                                    hold = hold
                                                                                                        .wrapping_add(
                                                                                                            (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                                        );
                                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                                }
                                                                                                if !inflate_trailer_length_matches(
                                                                                                    state_ref.wrap,
                                                                                                    hold,
                                                                                                    state_ref.total,
                                                                                                ) {
                                                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[7].as_ptr()
                                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                    state_ref.mode = crate::src::inflate::BAD;
                                                                                                    continue '_inf_leave;
                                                                                                } else {
                                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                                }
                                                                                            }
                                                                                                state_ref.mode = crate::src::inflate::DONE;
                                                                                                break 'c_2443;
                                                                                            }
                                                                                            while state_ref.have < state_ref.ncode {
                                                                                            while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                            {
                                                                                                if have == 0 as ::core::ffi::c_uint {
                                                                                                    break '_inf_leave;
                                                                                                }
                                                                                                have = have.wrapping_sub(1);
                                                                                                next = next.wrapping_add(1);
                                                                                                hold = hold
                                                                                                    .wrapping_add(
                                                                                                        (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                                    );
                                                                                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                            }
                                                                                            let c2rust_fresh15 = state_ref.have;
                                                                                            state_ref.have = state_ref.have.wrapping_add(1);
                                                                                            state_ref.lens[order[c2rust_fresh15 as usize] as usize] = (hold
                                                                                                as ::core::ffi::c_uint
                                                                                                & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                                                                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                                                                                as ::core::ffi::c_ushort;
                                                                                            hold >>= 3 as ::core::ffi::c_int;
                                                                                            bits = bits
                                                                                                .wrapping_sub(
                                                                                                    3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                                                                                );
                                                                                        }
                                                                                            while state_ref.have < 19 as ::core::ffi::c_uint {
                                                                                            let c2rust_fresh16 = state_ref.have;
                                                                                            state_ref.have = state_ref.have.wrapping_add(1);
                                                                                            state_ref.lens[order[c2rust_fresh16 as usize] as usize] = 0
                                                                                                as ::core::ffi::c_ushort;
                                                                                        }
                                                                                            ret = {
                                                                                                state_ref.next = 0;
                                                                                                state_ref.distcode = InflateCodeTable::Dynamic(0);
                                                                                                state_ref.lencode = InflateCodeTable::Dynamic(0);
                                                                                                state_ref.lenbits = 7 as ::core::ffi::c_uint;
                                                                                                match crate::src::inftrees::inflate_table_into(
                                                                                                crate::src::inftrees::CODES,
                                                                                                &state_ref.lens[..19],
                                                                                                &mut state_ref.codes[..crate::src::inftrees::ENOUGH as usize],
                                                                                                &mut state_ref.work,
                                                                                                state_ref.lenbits,
                                                                                            ) {
                                                                                                Ok((used, root)) => {
                                                                                                    state_ref.next = used;
                                                                                                    state_ref.lenbits = root;
                                                                                                    0
                                                                                                }
                                                                                                Err(status) => status,
                                                                                            }
                                                                                            };
                                                                                            if ret
                                                                                                != 0
                                                                                            {
                                                                                                strm_ref.msg = INFLATE_ERROR_MESSAGES[8].as_ptr()
                                                                                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                                continue '_inf_leave;
                                                                                            } else {
                                                                                                state_ref.have = 0 as ::core::ffi::c_uint;
                                                                                                state_ref.mode = crate::src::inflate::CODELENS;
                                                                                                break 's_1689;
                                                                                            }
                                                                                        }
                                                                                        // The dictionary transition only publishes
                                                                                        // already-established cursors and scalar state.
                                                                                        // Adopt both compatibility records once rather
                                                                                        // than repeatedly traversing their raw pointers.
                                                                                        let strm_ref =
                                                                                        &mut *strm;
                                                                                        let state_ref =
                                                                                        &mut *state;
                                                                                        if state_ref.havedict == 0 as ::core::ffi::c_int {
                                                                                        strm_ref.next_out = put as *mut crate::stdlib::Bytef;
                                                                                        strm_ref.avail_out = left as crate::stdlib::uInt;
                                                                                        strm_ref.next_in = next as *mut crate::stdlib::Bytef;
                                                                                        strm_ref.avail_in = have as crate::stdlib::uInt;
                                                                                        state_ref.hold = hold;
                                                                                        state_ref.bits = bits;
                                                                                        return crate::zlib_h::Z_NEED_DICT;
                                                                                    }
                                                                                        state_ref.check = crate::src::adler32::ADLER32_INITIAL
                                                                                        as ::core::ffi::c_ulong;
                                                                                        strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                                                                                        state_ref.mode =
                                                                                        crate::src::inflate::TYPE;
                                                                                        break 'c_2339;
                                                                                    }
                                                                                    while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                                {
                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                        break '_inf_leave;
                                                                                    }
                                                                                    have = have.wrapping_sub(1);
                                                                                    next = next.wrapping_add(1);
                                                                                    hold = hold
                                                                                        .wrapping_add(
                                                                                            (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                        );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                    // TIME has no cursor lend or callback. Keep the
                                                                                    // retained header update and checksum commit on
                                                                                    // one short-lived state borrow.
                                                                                    let state_ref =
                                                                                        &mut *state;
                                                                                    let time_plan = inflate_gzip_time_plan(
                                                                                        state_ref.flags,
                                                                                        state_ref.wrap,
                                                                                        hold,
                                                                                    );
                                                                                    if !state_ref
                                                                                        .head
                                                                                        .is_null()
                                                                                    {
                                                                                        (*state_ref
                                                                                        .head)
                                                                                        .time = time_plan.time;
                                                                                    }
                                                                                    inflate_gzip_time_commit(
                                                                                        state_ref,
                                                                                        time_plan,
                                                                                    );
                                                                                    hold = 0 as ::core::ffi::c_ulong;
                                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                                    break 's_519;
                                                                                }
                                                                                state_ref.mode = crate::src::inflate::COPY_1;
                                                                                break 'c_2356;
                                                                            }
                                                                            // This no-callback transition uses the decoder
                                                                            // entry's validated stream/state records for its
                                                                            // code-length cursor and error publication. Resolve
                                                                            // its compatibility cursor once to a bounded table
                                                                            // view, so every subsequent table read is checked
                                                                            // slice access rather than raw interior-pointer
                                                                            // arithmetic.
                                                                            let Some(lcode) =
                                                                                inflate_table_view(
                                                                                    &state_ref
                                                                                        .codes,
                                                                                    state_ref
                                                                                        .lencode,
                                                                                )
                                                                            else {
                                                                                strm_ref.msg = INFLATE_ERROR_MESSAGES[8].as_ptr()
                                                                                    as *const ::core::ffi::c_char
                                                                                    as *mut ::core::ffi::c_char;
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                continue '_inf_leave;
                                                                            };
                                                                            while state_ref.have
                                                                                < state_ref
                                                                                    .nlen
                                                                                    .wrapping_add(
                                                                                        state_ref
                                                                                            .ndist,
                                                                                    )
                                                                            {
                                                                                loop {
                                                                                    let Some(index) = inflate_root_code_index(
                                                                                        hold,
                                                                                        state_ref.lenbits,
                                                                                    ) else {
                                                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[8]
                                                                                            .as_ptr()
                                                                                            as *const ::core::ffi::c_char
                                                                                            as *mut ::core::ffi::c_char;
                                                                                        state_ref.mode = crate::src::inflate::BAD;
                                                                                        continue '_inf_leave;
                                                                                    };
                                                                                    let Some(code) = lcode.get(index).copied() else {
                                                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[8]
                                                                                            .as_ptr()
                                                                                            as *const ::core::ffi::c_char
                                                                                            as *mut ::core::ffi::c_char;
                                                                                        state_ref.mode = crate::src::inflate::BAD;
                                                                                        continue '_inf_leave;
                                                                                    };
                                                                                    here = code;
                                                                                    if here.bits as ::core::ffi::c_uint <= bits {
                                                                                    break;
                                                                                }
                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                    break '_inf_leave;
                                                                                }
                                                                                    have = have
                                                                                    .wrapping_sub(
                                                                                        1,
                                                                                    );
                                                                                    next = next
                                                                                    .wrapping_add(
                                                                                        1,
                                                                                    );
                                                                                    hold = hold
                                                                                    .wrapping_add(
                                                                                        (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                    );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                if (here.val as ::core::ffi::c_int)
                                                                                < 16 as ::core::ffi::c_int
                                                                            {
                                                                                hold >>= here.bits as ::core::ffi::c_int;
                                                                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                let c2rust_fresh18 = state_ref.have;
                                                                                state_ref.have = state_ref.have.wrapping_add(1);
                                                                                state_ref.lens[c2rust_fresh18 as usize] = here.val;
                                                                            } else {
                                                                                let extra = match here.val as ::core::ffi::c_uint {
                                                                                    16 => 2,
                                                                                    17 => 3,
                                                                                    _ => 7,
                                                                                };
                                                                                while bits
                                                                                    < (here.bits as ::core::ffi::c_uint)
                                                                                        .wrapping_add(extra)
                                                                                {
                                                                                    if have == 0 as ::core::ffi::c_uint {
                                                                                        break '_inf_leave;
                                                                                    }
                                                                                    have = have.wrapping_sub(1);
                                                                                    next = next.wrapping_add(1);
                                                                                    hold = hold.wrapping_add(
                                                                                        (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                                                                    );
                                                                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                                                }
                                                                                hold >>= here.bits as ::core::ffi::c_int;
                                                                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                                                if here.val as ::core::ffi::c_int
                                                                                    == 16 as ::core::ffi::c_int
                                                                                {
                                                                                    let Some(previous) = state_ref
                                                                                        .have
                                                                                        .checked_sub(1)
                                                                                        .and_then(|index| state_ref.lens.get(index as usize))
                                                                                        .copied()
                                                                                    else {
                                                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[9].as_ptr()
                                                                                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                        state_ref.mode = crate::src::inflate::BAD;
                                                                                        break;
                                                                                    };
                                                                                    len = previous as ::core::ffi::c_uint;
                                                                                } else {
                                                                                    len = 0 as ::core::ffi::c_uint;
                                                                                }
                                                                                let Some(repeat) = inflate_code_length_repeat_plan(
                                                                                    here.val, hold, bits,
                                                                                ) else {
                                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[9].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state_ref.mode = crate::src::inflate::BAD;
                                                                                    break;
                                                                                };
                                                                                hold = repeat.hold;
                                                                                bits = repeat.bits;
                                                                                copy = repeat.copy;
                                                                                let Some(range) = inflate_code_length_repeat_range(
                                                                                    state_ref.have,
                                                                                    copy,
                                                                                    state_ref.nlen.wrapping_add(state_ref.ndist),
                                                                                    state_ref.lens.len(),
                                                                                ) else {
                                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[9].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state_ref.mode = crate::src::inflate::BAD;
                                                                                    break;
                                                                                };
                                                                                state_ref.lens[range].fill(len as ::core::ffi::c_ushort);
                                                                                state_ref.have = state_ref.have.wrapping_add(copy);
                                                                                copy = 0;
                                                                            }
                                                                            }
                                                                            // Code-length decoding has already consumed all
                                                                            // input for this transition.  Keep the two
                                                                            // compatibility records adopted while we validate
                                                                            // the completed lens and build its bounded tables,
                                                                            // rather than re-traversing their raw cursors for
                                                                            // each error and mode commit.
                                                                            let strm_ref =
                                                                                &mut *strm;
                                                                            let state_ref =
                                                                                &mut *state;
                                                                            if state_ref.mode as ::core::ffi::c_uint
                                                                            == crate::src::inflate::BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                                                                        {
                                                                            continue '_inf_leave;
                                                                        }
                                                                            if state_ref.lens[256 as ::core::ffi::c_int as usize]
                                                                            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                                        {
                                                                            strm_ref.msg = INFLATE_ERROR_MESSAGES[10]
                                                                                .as_ptr() as *const ::core::ffi::c_char
                                                                                as *mut ::core::ffi::c_char;
                                                                            state_ref.mode = crate::src::inflate::BAD;
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            match inflate_build_dynamic_tables(state_ref) {
                                                                                Ok(()) => {
                                                                                    state_ref.mode = crate::src::inflate::LEN_;
                                                                                    if flush == crate::zlib_h::Z_TREES {
                                                                                        break '_inf_leave;
                                                                                    } else {
                                                                                        break 'c_2397;
                                                                                    }
                                                                                }
                                                                                Err(InflateDynamicTableError::Lengths) => {
                                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[11].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state_ref.mode = crate::src::inflate::BAD;
                                                                                    continue '_inf_leave;
                                                                                }
                                                                                Err(InflateDynamicTableError::Distances) => {
                                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[12].as_ptr()
                                                                                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                                                                                    state_ref.mode = crate::src::inflate::BAD;
                                                                                    continue '_inf_leave;
                                                                                }
                                                                            }
                                                                        }
                                                                        }
                                                                        // Stored-block copying needs no callbacks or new
                                                                        // ABI views.  Keep its scalar state commits on one
                                                                        // short-lived adopted state record instead of
                                                                        // repeatedly traversing the compatibility pointer.
                                                                        let state_ref = &mut *state;
                                                                        copy = state_ref.length;
                                                                        if copy != 0 {
                                                                            copy =
                                                                            inflate_stored_copy_len(
                                                                                copy, have, left,
                                                                            );
                                                                            if copy == 0 {
                                                                                break '_inf_leave;
                                                                            }
                                                                            let Some(input_start) = in_0
                                                                                .checked_sub(have)
                                                                                .and_then(|offset| usize::try_from(offset).ok())
                                                                            else {
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                break '_inf_leave;
                                                                            };
                                                                            let Some(output_start) = out
                                                                                .checked_sub(left)
                                                                                .and_then(|offset| usize::try_from(offset).ok())
                                                                            else {
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                break '_inf_leave;
                                                                            };
                                                                            let Ok(copy_len) = usize::try_from(copy) else {
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                break '_inf_leave;
                                                                            };
                                                                            let Some(input_end) = input_start.checked_add(copy_len) else {
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                break '_inf_leave;
                                                                            };
                                                                            let Some(output_end) = output_start.checked_add(copy_len) else {
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                break '_inf_leave;
                                                                            };
                                                                            let Some(source) = input.get(input_start..input_end) else {
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                break '_inf_leave;
                                                                            };
                                                                            let Some(destination) = output.get_mut(output_start..output_end) else {
                                                                                state_ref.mode = crate::src::inflate::BAD;
                                                                                ret = crate::zlib_h::Z_DATA_ERROR;
                                                                                break '_inf_leave;
                                                                            };
                                                                            destination.copy_from_slice(source);
                                                                            have = have
                                                                                .wrapping_sub(copy);
                                                                            next = next
                                                                                .wrapping_add(
                                                                                    copy as usize,
                                                                                );
                                                                            left = left
                                                                                .wrapping_sub(copy);
                                                                            put = put.wrapping_add(
                                                                                copy as usize,
                                                                            );
                                                                            state_ref.length =
                                                                                state_ref
                                                                                    .length
                                                                                    .wrapping_sub(
                                                                                        copy,
                                                                                    );
                                                                            continue '_inf_leave;
                                                                        } else {
                                                                            state_ref.mode = crate::src::inflate::TYPE;
                                                                            continue '_inf_leave;
                                                                        }
                                                                    }
                                                                    while bits
                                                                        < 16 as ::core::ffi::c_int
                                                                            as ::core::ffi::c_uint
                                                                    {
                                                                        if have == 0
                                                                            as ::core::ffi::c_uint
                                                                        {
                                                                            break '_inf_leave;
                                                                        }
                                                                        have = have.wrapping_sub(1);
                                                                        next = next.wrapping_add(1);
                                                                        hold = hold.wrapping_add(
                                                                        (inflate_input_at(input, in_0, have, 0)
                                                                            as ::core::ffi::c_ulong)
                                                                            << bits,
                                                                    );
                                                                        bits = bits.wrapping_add(
                                                                        8 as ::core::ffi::c_uint,
                                                                    );
                                                                    }
                                                                    // OS follows TIME without any callback or cursor lend.
                                                                    // Reuse one adopted state record for the retained header,
                                                                    // optional header CRC, and mode transition.
                                                                    let state_ref = &mut *state;
                                                                    let os_plan =
                                                                        inflate_gzip_os_plan(
                                                                            state_ref.flags,
                                                                            state_ref.wrap,
                                                                            hold,
                                                                        );
                                                                    if !state_ref.head.is_null() {
                                                                        let head =
                                                                            &mut *state_ref.head;
                                                                        head.xflags =
                                                                            os_plan.xflags;
                                                                        head.os = os_plan.os;
                                                                    }
                                                                    inflate_gzip_os_commit(
                                                                        state_ref,
                                                                        os_plan,
                                                                    );
                                                                    hold =
                                                                        0 as ::core::ffi::c_ulong;
                                                                    bits = 0 as ::core::ffi::c_uint;
                                                                    break 'c_2317;
                                                                }
                                                                if flush == crate::zlib_h::Z_BLOCK
                                                                    || flush
                                                                        == crate::zlib_h::Z_TREES
                                                                {
                                                                    break '_inf_leave;
                                                                } else {
                                                                    break 'c_2340;
                                                                }
                                                            }
                                                            ret = crate::zlib_h::Z_STREAM_END;
                                                            break '_inf_leave;
                                                        }
                                                        // The decoder entry has already validated both
                                                        // compatibility records. Keep the block-header
                                                        // state transition on those short-lived borrows
                                                        // instead of repeatedly traversing raw pointers.
                                                        let strm_ref = &mut *strm;
                                                        let state_ref = &mut *state;
                                                        if state_ref.last != 0 {
                                                            hold >>=
                                                                bits & 7 as ::core::ffi::c_uint;
                                                            bits = bits.wrapping_sub(
                                                                bits & 7 as ::core::ffi::c_uint,
                                                            );
                                                            state_ref.mode =
                                                                crate::src::inflate::CHECK;
                                                            continue '_inf_leave;
                                                        } else {
                                                            while bits
                                                                < 3 as ::core::ffi::c_int
                                                                    as ::core::ffi::c_uint
                                                            {
                                                                if have == 0 as ::core::ffi::c_uint
                                                                {
                                                                    break '_inf_leave;
                                                                }
                                                                have = have.wrapping_sub(1);
                                                                next = next.wrapping_add(1);
                                                                hold = hold.wrapping_add(
                                                                    (inflate_input_at(input, in_0, have, 0)
                                                                        as ::core::ffi::c_ulong)
                                                                        << bits,
                                                                );
                                                                bits = bits.wrapping_add(
                                                                    8 as ::core::ffi::c_uint,
                                                                );
                                                            }
                                                            let Some(plan) =
                                                                inflate_block_header_plan(
                                                                    hold, bits,
                                                                )
                                                            else {
                                                                strm_ref.msg = INFLATE_ERROR_MESSAGES
                                                                    [13]
                                                                .as_ptr()
                                                                    as *const ::core::ffi::c_char
                                                                    as *mut ::core::ffi::c_char;
                                                                state_ref.mode =
                                                                    crate::src::inflate::BAD;
                                                                continue '_inf_leave;
                                                            };
                                                            state_ref.last = plan.last;
                                                            hold = plan.hold;
                                                            bits = plan.bits;
                                                            match plan.kind {
                                                                InflateBlockKind::Stored => {
                                                                    state_ref.mode =
                                                                        crate::src::inflate::STORED;
                                                                }
                                                                InflateBlockKind::Fixed => {
                                                                    crate::src::inftrees::inflate_fixed_state(
                                                                        state_ref,
                                                                    );
                                                                    state_ref.mode =
                                                                        crate::src::inflate::LEN_;
                                                                    if flush
                                                                        == crate::zlib_h::Z_TREES
                                                                    {
                                                                        break '_inf_leave;
                                                                    }
                                                                }
                                                                InflateBlockKind::Dynamic => {
                                                                    state_ref.mode =
                                                                        crate::src::inflate::TABLE;
                                                                }
                                                                InflateBlockKind::Invalid => {
                                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES
                                                                    [13]
                                                                .as_ptr()
                                                                    as *const ::core::ffi::c_char
                                                                    as *mut ::core::ffi::c_char;
                                                                    state_ref.mode =
                                                                        crate::src::inflate::BAD;
                                                                }
                                                            }
                                                            continue '_inf_leave;
                                                        }
                                                    }
                                                    // EXLEN only consumes the already-validated input
                                                    // cursor and updates retained header/scalar state.
                                                    // Keep those commits on one short-lived state borrow
                                                    // instead of repeatedly traversing the compatibility
                                                    // state pointer.
                                                    let state_ref = &mut *state;
                                                    if state_ref.flags & 0x400 as ::core::ffi::c_int
                                                        != 0
                                                    {
                                                        while bits
                                                            < 16 as ::core::ffi::c_int
                                                                as ::core::ffi::c_uint
                                                        {
                                                            if have == 0 as ::core::ffi::c_uint {
                                                                break '_inf_leave;
                                                            }
                                                            have = have.wrapping_sub(1);
                                                            next = next.wrapping_add(1);
                                                            hold = hold.wrapping_add(
                                                                (inflate_input_at(input, in_0, have, 0)
                                                                    as ::core::ffi::c_ulong)
                                                                    << bits,
                                                            );
                                                            bits = bits.wrapping_add(
                                                                8 as ::core::ffi::c_uint,
                                                            );
                                                        }
                                                        let extra_plan = inflate_extra_length_plan(
                                                            state_ref.flags,
                                                            state_ref.wrap,
                                                            hold,
                                                        );
                                                        state_ref.length = extra_plan.length;
                                                        if !state_ref.head.is_null() {
                                                            let head = &mut *state_ref.head;
                                                            head.extra_len = extra_plan.length
                                                                as crate::stdlib::uInt;
                                                        }
                                                        if extra_plan.update_crc {
                                                            hbuf[0 as ::core::ffi::c_int
                                                                as usize] =
                                                                hold as ::core::ffi::c_uchar;
                                                            hbuf[1 as ::core::ffi::c_int
                                                                as usize] = (hold
                                                                >> 8 as ::core::ffi::c_int)
                                                                as ::core::ffi::c_uchar;
                                                            state_ref.check =
                                                                inflate_header_crc_update(
                                                                    state_ref.check,
                                                                    &hbuf[..2],
                                                                );
                                                        }
                                                        hold = 0 as ::core::ffi::c_ulong;
                                                        bits = 0 as ::core::ffi::c_uint;
                                                    } else if !state_ref.head.is_null() {
                                                        let head = &mut *state_ref.head;
                                                        head.extra = ::core::ptr::null_mut::<
                                                            crate::stdlib::Bytef,
                                                        >(
                                                        );
                                                    }
                                                    state_ref.mode = crate::src::inflate::EXTRA;
                                                    break 'c_2319;
                                                }
                                                // The EXLEN branch above owns a scoped state borrow.
                                                // Re-adopt the already-validated compatibility record
                                                // only for this fall-through mode commit.
                                                let state_ref = &mut *state;
                                                state_ref.mode = crate::src::inflate::LEN;
                                            }
                                            let strm_ref = &mut *strm;
                                            let state_ref = &mut *state;
                                            let lcode = inflate_table_view(
                                                &state_ref.codes,
                                                state_ref.lencode,
                                            );
                                            let fast = if let Some(span) = inflate_fast_span(
                                                have,
                                                left,
                                                out,
                                                state_ref.wsize,
                                                state_ref.whave,
                                                state_ref.wnext,
                                            ) {
                                                // `out` is the output capacity at the beginning of
                                                // this inflate call, while `put` has already advanced
                                                // over any bytes decoded by the slow path. Lend the
                                                // complete original span so the fast decoder keeps
                                                // zlib's distance accounting relative to `out`.
                                                let window = if span.window_len == 0 {
                                                    Some(&[][..])
                                                } else {
                                                    window.and_then(|window| {
                                                        window.get(..span.window_len)
                                                    })
                                                };
                                                let input_start = in_0.checked_sub(have)
                                                    .and_then(|offset| usize::try_from(offset).ok());
                                                let input = input_start.and_then(|start| {
                                                    start.checked_add(span.input_len)
                                                        .and_then(|end| input.get(start..end))
                                                });
                                                if let (Some(window), Some(input)) = (window, input) {
                                                    inflate_fast_tables(
                                                        &state_ref.codes,
                                                        state_ref.lencode,
                                                        state_ref.distcode,
                                                    )
                                                    .map(|(lcode, dcode)| {
                                                        inflate_fast_normal(
                                                            input,
                                                            output,
                                                            span.output_start,
                                                            window,
                                                            lcode,
                                                            dcode,
                                                            span.window_len,
                                                            state_ref.whave as usize,
                                                            state_ref.wnext as usize,
                                                            state_ref.sane != 0,
                                                            hold,
                                                            bits,
                                                            state_ref.lenbits,
                                                            state_ref.distbits,
                                                            out,
                                                        )
                                                    })
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            };
                                            if let Some(fast) = fast {
                                                let Some(fast) =
                                                    inflate_fast_commit(fast, have, left)
                                                else {
                                                    ret = crate::zlib_h::Z_DATA_ERROR;
                                                    break '_inf_leave;
                                                };
                                                next = next.wrapping_add(fast.input_used as usize);
                                                have = have.wrapping_sub(fast.input_used);
                                                put = put.wrapping_add(fast.output_used as usize);
                                                left = left.wrapping_sub(fast.output_used);
                                                hold = fast.hold;
                                                bits = fast.bits;
                                                if let Some(mode) = fast.mode {
                                                    state_ref.mode = mode;
                                                    strm_ref.msg = match fast.error {
                                                        Some(14) => {
                                                            b"invalid literal/length code\0"
                                                                .as_ptr()
                                                        }
                                                        Some(15) => {
                                                            b"invalid distance code\0".as_ptr()
                                                        }
                                                        Some(17) => {
                                                            b"invalid distance too far back\0"
                                                                .as_ptr()
                                                        }
                                                        _ => ::core::ptr::null(),
                                                    }
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                if state_ref.mode as ::core::ffi::c_uint
                                                    == crate::src::inflate::TYPE
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                                {
                                                    state_ref.back = -1 as ::core::ffi::c_int;
                                                }
                                                continue '_inf_leave;
                                            } else {
                                                // This slow-path fallback is still inside the
                                                // short-lived boundary borrow established for the
                                                // fast dispatch.  Keep scalar state updates on that
                                                // borrow instead of re-traversing the raw state
                                                // pointer.
                                                let Some(lcode) = lcode else {
                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[14]
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char;
                                                    state_ref.mode = crate::src::inflate::BAD;
                                                    ret = crate::zlib_h::Z_DATA_ERROR;
                                                    break '_inf_leave;
                                                };
                                                state_ref.back = 0 as ::core::ffi::c_int;
                                                loop {
                                                    let Some(index) = inflate_root_code_index(
                                                        hold,
                                                        state_ref.lenbits,
                                                    ) else {
                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[14]
                                                            .as_ptr()
                                                            as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char;
                                                        state_ref.mode = crate::src::inflate::BAD;
                                                        ret = crate::zlib_h::Z_DATA_ERROR;
                                                        break '_inf_leave;
                                                    };
                                                    let Some(code) = lcode.get(index) else {
                                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[14]
                                                            .as_ptr()
                                                            as *const ::core::ffi::c_char
                                                            as *mut ::core::ffi::c_char;
                                                        state_ref.mode = crate::src::inflate::BAD;
                                                        ret = crate::zlib_h::Z_DATA_ERROR;
                                                        break '_inf_leave;
                                                    };
                                                    here = *code;
                                                    if here.bits as ::core::ffi::c_uint <= bits {
                                                        break;
                                                    }
                                                    if have == 0 as ::core::ffi::c_uint {
                                                        break '_inf_leave;
                                                    }
                                                    have = have.wrapping_sub(1);
                                                    next = next.wrapping_add(1);
                                                    hold = hold.wrapping_add(
                                                        (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong)
                                                            << bits,
                                                    );
                                                    bits =
                                                        bits.wrapping_add(8 as ::core::ffi::c_uint);
                                                }
                                                if here.op as ::core::ffi::c_int != 0
                                                    && here.op as ::core::ffi::c_int
                                                        & 0xf0 as ::core::ffi::c_int
                                                        == 0 as ::core::ffi::c_int
                                                {
                                                    last = here;
                                                    loop {
                                                        let Some(index) =
                                                            inflate_subtable_code_index(hold, last)
                                                        else {
                                                            strm_ref.msg =
                                                                INFLATE_ERROR_MESSAGES[14].as_ptr()
                                                                    as *const ::core::ffi::c_char
                                                                    as *mut ::core::ffi::c_char;
                                                            state_ref.mode =
                                                                crate::src::inflate::BAD;
                                                            ret = crate::zlib_h::Z_DATA_ERROR;
                                                            break '_inf_leave;
                                                        };
                                                        let Some(code) = lcode.get(index) else {
                                                            strm_ref.msg =
                                                                INFLATE_ERROR_MESSAGES[14].as_ptr()
                                                                    as *const ::core::ffi::c_char
                                                                    as *mut ::core::ffi::c_char;
                                                            state_ref.mode =
                                                                crate::src::inflate::BAD;
                                                            ret = crate::zlib_h::Z_DATA_ERROR;
                                                            break '_inf_leave;
                                                        };
                                                        here = *code;
                                                        if (last.bits as ::core::ffi::c_int
                                                            + here.bits as ::core::ffi::c_int)
                                                            as ::core::ffi::c_uint
                                                            <= bits
                                                        {
                                                            break;
                                                        }
                                                        if have == 0 as ::core::ffi::c_uint {
                                                            break '_inf_leave;
                                                        }
                                                        have = have.wrapping_sub(1);
                                                        next = next.wrapping_add(1);
                                                        hold = hold.wrapping_add(
                                                            (inflate_input_at(input, in_0, have, 0)
                                                                as ::core::ffi::c_ulong)
                                                                << bits,
                                                        );
                                                        bits = bits
                                                            .wrapping_add(8 as ::core::ffi::c_uint);
                                                    }
                                                    hold >>= last.bits as ::core::ffi::c_int;
                                                    bits = bits.wrapping_sub(
                                                        last.bits as ::core::ffi::c_uint,
                                                    );
                                                    state_ref.back +=
                                                        last.bits as ::core::ffi::c_int;
                                                }
                                                hold >>= here.bits as ::core::ffi::c_int;
                                                bits = bits
                                                    .wrapping_sub(here.bits as ::core::ffi::c_uint);
                                                state_ref.back += here.bits as ::core::ffi::c_int;
                                                state_ref.length = here.val as ::core::ffi::c_uint;
                                                if here.op as ::core::ffi::c_int
                                                    == 0 as ::core::ffi::c_int
                                                {
                                                    state_ref.mode = crate::src::inflate::LIT;
                                                    continue '_inf_leave;
                                                } else if here.op as ::core::ffi::c_int
                                                    & 32 as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    state_ref.back = -1 as ::core::ffi::c_int;
                                                    state_ref.mode = crate::src::inflate::TYPE;
                                                    continue '_inf_leave;
                                                } else if here.op as ::core::ffi::c_int
                                                    & 64 as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[14]
                                                        .as_ptr()
                                                        as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char;
                                                    state_ref.mode = crate::src::inflate::BAD;
                                                    continue '_inf_leave;
                                                } else {
                                                    state_ref.extra = here.op
                                                        as ::core::ffi::c_uint
                                                        & 15 as ::core::ffi::c_uint;
                                                    state_ref.mode = crate::src::inflate::LENEXT;
                                                    break 'c_2410;
                                                }
                                            }
                                        }
                                        // The gzip extra-field transition only consumes the
                                        // established decoder cursor and updates retained header
                                        // state.  Adopt the decoder state once for this transition
                                        // instead of repeatedly traversing its raw compatibility
                                        // pointer.
                                        let state_ref = &mut *state;
                                        if state_ref.flags & 0x400 as ::core::ffi::c_int != 0 {
                                            copy = state_ref.length;
                                            if copy > have {
                                                copy = have;
                                            }
                                            if copy != 0 {
                                                if !state_ref.head.is_null() {
                                                    // The retained header is an ABI destination, but
                                                    // this transition only needs one temporary borrow
                                                    // to bound and copy its extra bytes.
                                                    let head = &mut *state_ref.head;
                                                    if !head.extra.is_null() {
                                                        if let Some((header_offset, header_copy)) =
                                                            inflate_header_extra_copy_plan(
                                                                head.extra_len,
                                                                head.extra_max,
                                                                state_ref.length,
                                                                copy,
                                                            )
                                                        {
                                                            let mut copied = 0usize;
                                                            while copied < header_copy {
                                                                *head.extra.wrapping_add(
                                                                    header_offset + copied,
                                                                ) = inflate_input_from_cursor(input, in_0, have, copied);
                                                                copied += 1;
                                                            }
                                                        }
                                                    }
                                                }
                                                if state_ref.flags & 0x200 as ::core::ffi::c_int
                                                    != 0
                                                    && state_ref.wrap & 4 as ::core::ffi::c_int != 0
                                                {
                                                    let mut checked =
                                                        state_ref.check as crate::stdlib::uLong;
                                                    let mut checked_at = 0usize;
                                                    while checked_at < copy as usize {
                                                        let byte = inflate_input_from_cursor(input, in_0, have, checked_at);
                                                        checked = inflate_header_crc_update(
                                                            checked as ::core::ffi::c_ulong,
                                                            &[byte],
                                                        )
                                                            as crate::stdlib::uLong;
                                                        checked_at += 1;
                                                    }
                                                    state_ref.check =
                                                        checked as ::core::ffi::c_ulong;
                                                }
                                                have = have.wrapping_sub(copy);
                                                next = next.wrapping_add(copy as usize);
                                                state_ref.length =
                                                    state_ref.length.wrapping_sub(copy);
                                            }
                                            if state_ref.length != 0 {
                                                break '_inf_leave;
                                            }
                                        }
                                        state_ref.length = 0 as ::core::ffi::c_uint;
                                        state_ref.mode = crate::src::inflate::NAME;
                                        break 'c_2322;
                                    }
                                    // Length extra bits only consume the decoder cursor and
                                    // update scalar match state. Keep the compatibility-state
                                    // access in one short-lived transition borrow, matching
                                    // the distance-extra transition below.
                                    let state_ref = &mut *state;
                                    if state_ref.extra != 0 {
                                        let extra = state_ref.extra;
                                        while bits < extra {
                                            if have == 0 as ::core::ffi::c_uint {
                                                break '_inf_leave;
                                            }
                                            have = have.wrapping_sub(1);
                                            next = next.wrapping_add(1);
                                            hold = hold.wrapping_add(
                                                (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                            );
                                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                        }
                                        let Some(plan) = inflate_length_extra_plan(
                                            hold,
                                            bits,
                                            extra,
                                            state_ref.length,
                                            state_ref.back,
                                        ) else {
                                            state_ref.mode = crate::src::inflate::BAD;
                                            continue '_inf_leave;
                                        };
                                        state_ref.length = plan.length;
                                        hold = plan.hold;
                                        bits = plan.bits;
                                        state_ref.back = plan.back;
                                    }
                                    state_ref.was = state_ref.length;
                                    state_ref.mode = crate::src::inflate::DIST;
                                    break 's_2462;
                                }
                                // NAME consumes only the current input cursor and retained header
                                // destination.  Keep its scalar state updates on one adopted
                                // decoder-state borrow.
                                let state_ref = &mut *state;
                                if state_ref.flags & 0x800 as ::core::ffi::c_int != 0 {
                                    if have == 0 as ::core::ffi::c_uint {
                                        break '_inf_leave;
                                    }
                                    let header_crc = state_ref.flags & 0x200 as ::core::ffi::c_int
                                        != 0
                                        && state_ref.wrap & 4 as ::core::ffi::c_int != 0;
                                    copy = 0 as ::core::ffi::c_uint;
                                    loop {
                                        let c2rust_fresh5 = copy;
                                        copy = copy.wrapping_add(1);
                                        len = inflate_input_from_cursor(input, in_0, have, c2rust_fresh5 as usize)
                                            as ::core::ffi::c_uint;
                                        if header_crc {
                                            // Feed the byte while it is already available as a
                                            // scalar, avoiding a second raw input lend solely for
                                            // the header checksum after the name scan.
                                            state_ref.check = inflate_header_crc_update(
                                                state_ref.check,
                                                &[len as crate::stdlib::Bytef],
                                            );
                                        }
                                        if !state_ref.head.is_null() {
                                            // `head` remains an ABI-owned retained destination, but
                                            // this transition has already established that it is
                                            // non-null. Adopt it once rather than re-dereferencing
                                            // the compatibility pointer for its bounds and byte
                                            // commit.
                                            let head = &mut *state_ref.head;
                                            if !head.name.is_null()
                                                && state_ref.length < head.name_max
                                            {
                                                let c2rust_fresh6 = state_ref.length;
                                                state_ref.length = state_ref.length.wrapping_add(1);
                                                *head.name.wrapping_add(c2rust_fresh6 as usize) =
                                                    len as crate::stdlib::Bytef;
                                            }
                                        }
                                        if !(len != 0 && copy < have) {
                                            break;
                                        }
                                    }
                                    have = have.wrapping_sub(copy);
                                    next = next.wrapping_add(copy as usize);
                                    if len != 0 {
                                        break '_inf_leave;
                                    }
                                } else if !state_ref.head.is_null() {
                                    let head = &mut *state_ref.head;
                                    head.name = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                                }
                                state_ref.length = 0 as ::core::ffi::c_uint;
                                state_ref.mode = crate::src::inflate::COMMENT;
                                break 'c_2325;
                            }
                            // Keep distance-table state and its error publication on
                            // the decoder entry's validated records. The table cursor
                            // is resolved once to a bounded view, so both root and
                            // subtable reads stay checked slice accesses.
                            let strm_ref = &mut *strm;
                            let state_ref = &mut *state;
                            let Some(dcode) =
                                inflate_table_view(&state_ref.codes, state_ref.distcode)
                            else {
                                strm_ref.msg = INFLATE_ERROR_MESSAGES[15].as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                state_ref.mode = crate::src::inflate::BAD;
                                continue '_inf_leave;
                            };
                            loop {
                                let Some(index) = inflate_root_code_index(hold, state_ref.distbits)
                                else {
                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[15].as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    state_ref.mode = crate::src::inflate::BAD;
                                    continue '_inf_leave;
                                };
                                let Some(code) = dcode.get(index).copied() else {
                                    strm_ref.msg = INFLATE_ERROR_MESSAGES[15].as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                    state_ref.mode = crate::src::inflate::BAD;
                                    continue '_inf_leave;
                                };
                                here = code;
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                if have == 0 as ::core::ffi::c_uint {
                                    break '_inf_leave;
                                }
                                have = have.wrapping_sub(1);
                                next = next.wrapping_add(1);
                                hold = hold.wrapping_add(
                                    (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                );
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                last = here;
                                loop {
                                    let Some(index) = inflate_subtable_code_index(hold, last)
                                    else {
                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[15].as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char;
                                        state_ref.mode = crate::src::inflate::BAD;
                                        continue '_inf_leave;
                                    };
                                    let Some(code) = dcode.get(index).copied() else {
                                        strm_ref.msg = INFLATE_ERROR_MESSAGES[15].as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char;
                                        state_ref.mode = crate::src::inflate::BAD;
                                        continue '_inf_leave;
                                    };
                                    here = code;
                                    if (last.bits as ::core::ffi::c_int
                                        + here.bits as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint
                                        <= bits
                                    {
                                        break;
                                    }
                                    if have == 0 as ::core::ffi::c_uint {
                                        break '_inf_leave;
                                    }
                                    have = have.wrapping_sub(1);
                                    next = next.wrapping_add(1);
                                    hold = hold.wrapping_add(
                                        (inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits,
                                    );
                                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                }
                                hold >>= last.bits as ::core::ffi::c_int;
                                bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                                state_ref.back += last.bits as ::core::ffi::c_int;
                            }
                            hold >>= here.bits as ::core::ffi::c_int;
                            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                            state_ref.back += here.bits as ::core::ffi::c_int;
                            if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                                strm_ref.msg = INFLATE_ERROR_MESSAGES[15].as_ptr()
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                state_ref.mode = crate::src::inflate::BAD;
                                continue '_inf_leave;
                            } else {
                                state_ref.offset = here.val as ::core::ffi::c_uint;
                                state_ref.extra =
                                    here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                                state_ref.mode = crate::src::inflate::DISTEXT;
                                break 'c_2422;
                            }
                        }
                        // COMMENT consumes only the current input cursor and retained
                        // header destination. Keep its scalar state updates on one
                        // adopted decoder-state borrow, matching the NAME transition.
                        let state_ref = &mut *state;
                        if state_ref.flags & 0x1000 as ::core::ffi::c_int != 0 {
                            if have == 0 as ::core::ffi::c_uint {
                                break '_inf_leave;
                            }
                            let header_crc = state_ref.flags & 0x200 as ::core::ffi::c_int != 0
                                && state_ref.wrap & 4 as ::core::ffi::c_int != 0;
                            copy = 0 as ::core::ffi::c_uint;
                            loop {
                                let c2rust_fresh7 = copy;
                                copy = copy.wrapping_add(1);
                                len = inflate_input_from_cursor(input, in_0, have, c2rust_fresh7 as usize)
                                    as ::core::ffi::c_uint;
                                if header_crc {
                                    // The byte is already available as a scalar while scanning the
                                    // NUL-terminated comment.  Feed it directly instead of forming
                                    // a second raw input view after the scan.
                                    state_ref.check = inflate_header_crc_update(
                                        state_ref.check,
                                        &[len as crate::stdlib::Bytef],
                                    );
                                }
                                if !state_ref.head.is_null() {
                                    // As in NAME, use one transition-local borrow of the retained
                                    // ABI header destination for the bounds check and byte commit.
                                    let head = &mut *state_ref.head;
                                    if !head.comment.is_null() && state_ref.length < head.comm_max {
                                        let c2rust_fresh8 = state_ref.length;
                                        state_ref.length = state_ref.length.wrapping_add(1);
                                        *head.comment.wrapping_add(c2rust_fresh8 as usize) =
                                            len as crate::stdlib::Bytef;
                                    }
                                }
                                if !(len != 0 && copy < have) {
                                    break;
                                }
                            }
                            have = have.wrapping_sub(copy);
                            next = next.wrapping_add(copy as usize);
                            if len != 0 {
                                break '_inf_leave;
                            }
                        } else if !state_ref.head.is_null() {
                            let head = &mut *state_ref.head;
                            head.comment = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                        }
                        state_ref.mode = crate::src::inflate::HCRC;
                        break 'c_2327;
                    }
                    // Distance extra bits only consume the decoder cursor and
                    // update scalar match state. Keep the compatibility-state
                    // access in one short-lived transition borrow; raw input
                    // cursor handling remains in this legacy boundary.
                    let state_ref = &mut *state;
                    if state_ref.extra != 0 {
                        let extra = state_ref.extra;
                        while bits < extra {
                            if have == 0 as ::core::ffi::c_uint {
                                break '_inf_leave;
                            }
                            have = have.wrapping_sub(1);
                            next = next.wrapping_add(1);
                            hold = hold
                                .wrapping_add((inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        let Some(plan) = inflate_distance_extra_plan(
                            hold,
                            bits,
                            extra,
                            state_ref.offset,
                            state_ref.back,
                        ) else {
                            state_ref.mode = crate::src::inflate::BAD;
                            continue '_inf_leave;
                        };
                        state_ref.offset = plan.offset;
                        hold = plan.hold;
                        bits = plan.bits;
                        state_ref.back = plan.back;
                    }
                    state_ref.mode = crate::src::inflate::MATCH;
                    break 'c_2425;
                }
                // Header-CRC completion only consumes the existing decoder
                // cursor and updates stream/header scalars.  Keep those commits
                // on one short-lived boundary borrow instead of repeatedly
                // traversing the compatibility records.
                let strm_ref = &mut *strm;
                let state_ref = &mut *state;
                if state_ref.flags & 0x200 as ::core::ffi::c_int != 0 {
                    while bits < 16 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            break '_inf_leave;
                        }
                        have = have.wrapping_sub(1);
                        next = next.wrapping_add(1);
                        hold = hold.wrapping_add((inflate_input_at(input, in_0, have, 0) as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                }
                let plan = inflate_gzip_header_crc_plan(
                    state_ref.flags,
                    state_ref.wrap,
                    state_ref.check,
                    hold,
                    bits,
                );
                if !plan.matches {
                    strm_ref.msg = INFLATE_ERROR_MESSAGES[16].as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    state_ref.mode = crate::src::inflate::BAD;
                    continue '_inf_leave;
                }
                hold = plan.hold;
                bits = plan.bits;
                if !state_ref.head.is_null() {
                    let head = &mut *state_ref.head;
                    head.hcrc = plan.hcrc;
                    head.done = 1 as ::core::ffi::c_int;
                }
                state_ref.check = inflate_header_crc_update(0, &[]);
                strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                state_ref.mode = crate::src::inflate::TYPE;
                continue '_inf_leave;
            }
            if left == 0 as ::core::ffi::c_uint {
                break;
            }
            // This match-copy transition does not allocate or invoke callbacks,
            // so keep its scalar state bookkeeping on one short-lived adopted
            // state record. The cursor copy below remains in the transitional
            // decoder boundary.
            let strm_ref = &mut *strm;
            let state_ref = &mut *state;
            let Some(plan) = inflate_match_copy_plan(
                out,
                left,
                state_ref.offset,
                state_ref.length,
                state_ref.whave,
                state_ref.wnext,
                state_ref.wsize,
                !state_ref.window.is_null(),
                state_ref.sane != 0,
            ) else {
                strm_ref.msg = INFLATE_ERROR_MESSAGES[17].as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                state_ref.mode = crate::src::inflate::BAD;
                continue;
            };
            copy = plan.copy;
            let copied = (|| {
                let output_start = usize::try_from(out.checked_sub(left)?).ok()?;
                let copy_len = usize::try_from(copy).ok()?;
                let output_end = output_start.checked_add(copy_len)?;
                match plan.source {
                    InflateMatchSource::Window { index } => {
                        let source_end = index.checked_add(copy_len)?;
                        let source = window?.get(index..source_end)?;
                        let destination = output.get_mut(output_start..output_end)?;
                        destination.copy_from_slice(source);
                        Some(())
                    }
                    InflateMatchSource::Output { offset } => {
                        let output_copy_start = output_start.checked_sub(offset)?;
                        let copied = output.get_mut(output_copy_start..output_end)?;
                        inflate_output_match_copy(copied, offset, copy_len)
                    }
                }
            })();
            if copied.is_none() {
                strm_ref.msg = INFLATE_ERROR_MESSAGES[17].as_ptr()
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
                state_ref.mode = crate::src::inflate::BAD;
                continue;
            }
            left = left.wrapping_sub(copy);
            put = put.wrapping_add(copy as usize);
            state_ref.length = plan.remaining_length;
            if state_ref.length == 0 as ::core::ffi::c_uint {
                state_ref.mode = crate::src::inflate::LEN;
            }
        }
        }
        // The invocation-local ABI views above have ended before this exit
        // boundary can invoke an allocator callback.
        // Keep the decoder loop's raw cursors local to that loop.  The exit
        // commit adopts each ABI record once, so cursor publication, history
        // planning, totals, and checksum state use ordinary field access.
        // Compute one checked exit plan before history maintenance.  Updating the
        // circular window may change history cursors, but it does not change the
        // decoded input/output progress, mode, or bit state this call publishes.
        // Reusing this plan keeps the history decision and final ABI accounting
        // tied to the same scalar snapshot.
        {
            let strm_ref = &mut *strm;
            let state_ref = &mut *state;
            strm_ref.next_out = put as *mut crate::stdlib::Bytef;
            strm_ref.avail_out = left as crate::stdlib::uInt;
            strm_ref.next_in = next as *mut crate::stdlib::Bytef;
            strm_ref.avail_in = have as crate::stdlib::uInt;
            state_ref.hold = hold;
            state_ref.bits = bits;
            let exit = inflate_exit_progress(
                in_0,
                strm_ref.avail_in as ::core::ffi::c_uint,
                out,
                strm_ref.avail_out as ::core::ffi::c_uint,
                state_ref.wsize,
                state_ref.mode,
                flush,
                state_ref.bits,
                state_ref.last,
            );
            let (window_error, produced): (bool, &[crate::stdlib::Bytef]) = 'window: {
                // Invoke a possible allocator callback before lending the caller's
                // completed-output span. History retention and final checksum then
                // consume that one exact bounded view.
                let wrap = state_ref.wrap;
                let window = if exit.update_window {
                    // The decoder boundary owns callback invocation and the one
                    // temporary ABI-window lend.  The plan and the subsequent
                    // history update remain slice/scalar-only, so no private
                    // unsafe window adapter is needed.
                    let Some(plan) = inflate_window_boundary_plan(
                        state_ref.window.is_null(),
                        state_ref.wbits,
                        state_ref.wsize,
                        exit.output_used,
                    ) else {
                        break 'window (true, &[]);
                    };
                    if plan.allocate {
                        let Ok(requested_wsize) = ::core::ffi::c_uint::try_from(plan.window_len)
                        else {
                            break 'window (true, &[]);
                        };
                        // `inflate()` normally reaches this boundary only after
                        // init has installed zalloc. Treat malformed callback
                        // state as allocation failure instead of panicking.
                        let Some(zalloc) = strm_ref.zalloc else {
                            break 'window (true, &[]);
                        };
                        state_ref.window = zalloc(
                            strm_ref.opaque,
                            requested_wsize,
                            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
                        ) as *mut ::core::ffi::c_uchar;
                        if state_ref.window.is_null() {
                            break 'window (true, &[]);
                        }
                    }
                    Some(::core::slice::from_raw_parts_mut(
                        state_ref.window,
                        plan.window_len,
                    ))
                } else {
                    None
                };
                if exit.update_window && window.is_none() {
                    break 'window (true, &[]);
                }
                let needs_output_view = exit.update_window
                    || inflate_exit_needs_checksum(wrap, exit.output_used as usize);
                let produced = if needs_output_view && exit.output_used != 0 {
                    ::core::slice::from_raw_parts(
                        strm_ref.next_out.wrapping_sub(exit.output_used as usize),
                        exit.output_used as usize,
                    )
                } else {
                    &[]
                };
                if let Some(window) = window {
                    if inflate_window_update_state(state_ref, window, produced).is_none() {
                        break 'window (true, &[]);
                    }
                }
                (false, produced)
            };
            if window_error {
                state_ref.mode = crate::src::inflate::MEM;
                return crate::zlib_h::Z_MEM_ERROR;
            }
            in_0 = exit.input_used;
            out = exit.output_used;
            strm_ref.total_in = strm_ref
                .total_in
                .wrapping_add(exit.input_used as crate::stdlib::uLong);
            strm_ref.total_out = strm_ref
                .total_out
                .wrapping_add(exit.output_used as crate::stdlib::uLong);
            state_ref.total = state_ref
                .total
                .wrapping_add(exit.output_used as ::core::ffi::c_ulong);
            if inflate_exit_needs_checksum(state_ref.wrap, exit.output_used as usize) {
                if let Some(check) = inflate_exit_checksum(
                    state_ref.wrap,
                    state_ref.check as crate::stdlib::uLong,
                    state_ref.flags,
                    produced,
                ) {
                    state_ref.check = check;
                    strm_ref.adler = state_ref.check as crate::stdlib::uLong;
                }
            }
            strm_ref.data_type = exit.data_type;
        }
        inflate_exit_status(in_0, out, flush, ret)
    }
}
#[export_name = "inflate"]

pub unsafe extern "C" fn inflate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(strm_ref) = strm.as_mut() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflate(strm_ref, flush)
}
// This expands only in export-attributed ABI functions (including the
// boundary macros used by gzip and one-shot decompression).  Destruction
// invokes caller-provided allocation callbacks, so it must remain at that
// boundary until inflate state owns its allocation safely.
macro_rules! inflate_end_at_boundary {
    ($strm:expr $(,)?) => {{
        let strm = $strm;
        if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
            crate::zlib_h::Z_STREAM_ERROR
        } else {
            // Snapshot exactly one callback invocation at a time. A custom
            // allocator may observe or mutate the public stream, so no Rust
            // borrow of it can survive either callback. This remains an ABI
            // boundary: the safe inflate core never owns callback-allocated
            // state or invokes the caller's free function.
            let (window, zfree, opaque) = {
                let strm_ref = &mut *strm;
                let state = strm_ref.state as *mut crate::src::inflate::inflate_state;
                let state_ref = &mut *state;
                match strm_ref.zfree {
                    Some(zfree) => (state_ref.window, zfree, strm_ref.opaque),
                    None => return crate::zlib_h::Z_STREAM_ERROR,
                }
            };
            if !window.is_null() {
                zfree(opaque, window as crate::stdlib::voidpf);
            }
            let (state, zfree, opaque) = {
                let strm_ref = &mut *strm;
                match strm_ref.zfree {
                    Some(zfree) => (strm_ref.state, zfree, strm_ref.opaque),
                    None => return crate::zlib_h::Z_STREAM_ERROR,
                }
            };
            zfree(opaque, state as crate::stdlib::voidpf);
            let strm_ref = &mut *strm;
            strm_ref.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
            crate::zlib_h::Z_OK
        }
    }};
}
pub(crate) use inflate_end_at_boundary;
#[export_name = "inflateEnd"]

pub unsafe extern "C" fn inflateEnd_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    inflate_end_at_boundary!(strm)
}

/// Copy the logical inflate dictionary out of its circular history buffer.
///
/// The ABI wrapper validates and lends both buffers for this call.  Keeping
/// the wraparound arithmetic here makes the two copies bounds-checked slice
/// operations instead of pointer offsets at the boundary.
fn inflate_dictionary_copy(
    window: &[crate::stdlib::Byte],
    wnext: usize,
    whave: usize,
    dictionary: &mut [crate::stdlib::Byte],
) -> Option<()> {
    if whave > window.len() || wnext > whave || dictionary.len() != whave {
        return None;
    }
    let (tail, head) = dictionary.split_at_mut(whave.checked_sub(wnext)?);
    tail.copy_from_slice(window.get(wnext..whave)?);
    head.copy_from_slice(window.get(..wnext)?);
    Some(())
}

/// Classify and size dictionary installation before the ABI boundary invokes
/// an allocator or lends the history allocation.  The checksum and mode
/// rules are ordinary inflate state transitions; only the callback and raw
/// window conversion belong to the exported wrapper.
fn inflate_dictionary_admission(
    state: &inflate_state,
    dictionary: &[crate::stdlib::Byte],
) -> Result<InflateWindowBoundaryPlan, ::core::ffi::c_int> {
    if state.wrap != 0 && state.mode != crate::src::inflate::DICT {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    if state.mode == crate::src::inflate::DICT {
        let dictid = crate::src::adler32::adler32_z(
            crate::src::adler32::ADLER32_INITIAL as crate::stdlib::uLong,
            dictionary,
        ) as ::core::ffi::c_ulong;
        if dictid != state.check {
            return Err(crate::zlib_h::Z_DATA_ERROR);
        }
    }
    let copy =
        ::core::ffi::c_uint::try_from(dictionary.len()).map_err(|_| crate::zlib_h::Z_MEM_ERROR)?;
    inflate_window_boundary_plan(state.window.is_null(), state.wbits, state.wsize, copy)
        .ok_or(crate::zlib_h::Z_MEM_ERROR)
}

/// Commit a validated preset dictionary using an already-lent history slice.
/// All state and slice work stays safe, so the export boundary only has to
/// allocate the optional window and create this temporary view.
fn inflate_dictionary_commit(
    state: &mut inflate_state,
    dictionary: &[crate::stdlib::Byte],
    window: &mut [crate::stdlib::Byte],
) -> Result<(), ::core::ffi::c_int> {
    let admission = inflate_dictionary_admission(state, dictionary)?;
    if admission.allocate
        || admission.window_len != window.len()
        || admission.copy_len != dictionary.len()
    {
        return Err(crate::zlib_h::Z_MEM_ERROR);
    }
    let update = inflate_window_update(
        window,
        dictionary,
        state.wbits,
        state.wsize,
        state.wnext,
        state.whave,
    )
    .ok_or(crate::zlib_h::Z_MEM_ERROR)?;
    state.wsize = update.wsize;
    state.wnext = update.wnext;
    state.whave = update.whave;
    state.havedict = 1;
    Ok(())
}

/// Check the non-overlap precondition before lending an ABI history window
/// and caller dictionary as Rust slices.  zlib's original copies have
/// `memcpy` semantics, so an overlapping caller destination was never a
/// supported operation; rejecting it here prevents simultaneous aliased
/// shared and mutable slice views at the boundary.
fn inflate_spans_are_disjoint(
    left_address: usize,
    left_len: usize,
    right_address: usize,
    right_len: usize,
) -> bool {
    if left_len == 0 || right_len == 0 {
        return true;
    }
    let Some(left_end) = left_address.checked_add(left_len) else {
        return false;
    };
    let Some(right_end) = right_address.checked_add(right_len) else {
        return false;
    };
    left_end <= right_address || right_end <= left_address
}

#[export_name = "inflateGetDictionary"]
pub unsafe extern "C" fn inflateGetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *mut crate::stdlib::Bytef,
    mut dictLength: *mut crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    if state.whave != 0 && !dictionary.is_null() {
        let Ok(window_len) = usize::try_from(state.wsize) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Ok(wnext) = usize::try_from(state.wnext) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Ok(whave) = usize::try_from(state.whave) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        if state.window.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        if !inflate_spans_are_disjoint(
            state.window as usize,
            window_len,
            dictionary as usize,
            whave,
        ) {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let window = ::core::slice::from_raw_parts(state.window, window_len);
        let output = ::core::slice::from_raw_parts_mut(dictionary, whave);
        if inflate_dictionary_copy(window, wnext, whave, output).is_none() {
            return crate::zlib_h::Z_STREAM_ERROR;
        }
    }
    if !dictLength.is_null() {
        *dictLength = state.whave as crate::stdlib::uInt;
    }
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateSetDictionary"]
pub unsafe extern "C" fn inflateSetDictionary_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::stdlib::Bytef,
    mut dictLength: crate::stdlib::uInt,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dictionary = if dictLength == 0 {
        &[][..]
    } else if dictionary.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    } else {
        core::slice::from_raw_parts(dictionary, dictLength as usize)
    };
    let (admission, zalloc, opaque) = {
        let strm_ref = &mut *strm;
        let state = &mut *(strm_ref.state as *mut crate::src::inflate::inflate_state);
        let admission = match inflate_dictionary_admission(state, dictionary) {
            Ok(admission) => admission,
            Err(status) => return status,
        };
        (admission, strm_ref.zalloc, strm_ref.opaque)
    };

    // This exported boundary owns the allocator callback and the raw window
    // lend. End the state borrow before invoking the callback: it may inspect
    // the public stream record. The circular-buffer planning and copying stay
    // in the checked slice core, so dictionary setup does not enter the
    // private raw `updatewindow` codec adapter.
    let allocated_window = if admission.allocate {
        let Ok(window_len) = crate::stdlib::uInt::try_from(admission.window_len) else {
            return crate::zlib_h::Z_MEM_ERROR;
        };
        // `inflate_state_check_at_boundary!` normally guarantees this, but
        // keep a malformed callback slot from turning an ABI error into a
        // Rust panic before the allocator boundary is reached.
        let Some(zalloc) = zalloc else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        Some(zalloc(
            opaque,
            window_len,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar)
    } else {
        None
    };

    let strm_ref = &mut *strm;
    let state = &mut *(strm_ref.state as *mut crate::src::inflate::inflate_state);
    if let Some(window) = allocated_window {
        state.window = window;
        if state.window.is_null() {
            state.mode = crate::src::inflate::MEM;
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    let admission = match inflate_dictionary_admission(state, dictionary) {
        Ok(admission) => admission,
        Err(status) => {
            if status == crate::zlib_h::Z_MEM_ERROR {
                state.mode = crate::src::inflate::MEM;
            }
            return status;
        }
    };
    if admission.allocate || state.window.is_null() {
        state.mode = crate::src::inflate::MEM;
        return crate::zlib_h::Z_MEM_ERROR;
    }
    let window = core::slice::from_raw_parts_mut(state.window, admission.window_len);
    match inflate_dictionary_commit(state, dictionary, window) {
        Ok(()) => crate::zlib_h::Z_OK,
        Err(status) => {
            if status == crate::zlib_h::Z_MEM_ERROR {
                state.mode = crate::src::inflate::MEM;
            }
            status
        }
    }
}
#[export_name = "inflateGetHeader"]
pub unsafe extern "C" fn inflateGetHeader_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    if state.wrap & 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if head.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    state.head = head;
    (*head).done = 0 as ::core::ffi::c_int;
    return crate::zlib_h::Z_OK;
}
/// Search a safe compressed-input view for zlib's sync marker.  The export
/// boundary owns conversion of the ABI cursor into this call-scoped slice.
fn syncsearch(have: &mut ::core::ffi::c_uint, buf: &[u8]) -> ::core::ffi::c_uint {
    let mut got = *have;
    let mut next = 0;
    for &byte in buf {
        if got == 4 {
            break;
        }
        if byte == if got < 2 { 0 } else { 0xff } {
            got = got.wrapping_add(1);
        } else if byte != 0 {
            got = 0;
        } else {
            got = 4u32.wrapping_sub(got);
        }
        next += 1;
    }
    *have = got;
    next
}

/// Discard the partial byte in the bit accumulator and expose the remaining
/// whole bytes for the sync-marker search.  The legacy state stores at most
/// 32 bits, so a larger count is an incoherent internal state rather than a
/// reason to index past this fixed scratch buffer.
fn inflate_sync_aligned_bytes(
    mut hold: ::core::ffi::c_ulong,
    mut bits: ::core::ffi::c_uint,
) -> Option<(::core::ffi::c_ulong, ::core::ffi::c_uint, [u8; 4], usize)> {
    hold >>= bits & 7;
    bits = bits.wrapping_sub(bits & 7);
    let mut bytes = [0; 4];
    let mut len = 0;
    while bits >= 8 {
        let slot = bytes.get_mut(len)?;
        *slot = hold as u8;
        hold >>= 8;
        bits = bits.wrapping_sub(8);
        len += 1;
    }
    Some((hold, bits, bytes, len))
}

/// The pointer-free portion of an `inflateSync()` attempt.  The ABI wrapper
/// lends its current input span and commits the returned scalars only after
/// this has bounded the marker scan.
struct InflateSyncPlan {
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
    consumed: ::core::ffi::c_uint,
}

/// Align a partially-read deflate stream, if necessary, then search the
/// supplied compressed bytes for zlib's four-byte synchronization marker.
/// `was_sync` reflects the incoming mode; callers still publish `SYNC` at
/// their ABI boundary before invoking this core, matching zlib's error-path
/// state transition when the bit accumulator is malformed.
fn inflate_sync_plan(
    input: &[u8],
    was_sync: bool,
    hold: ::core::ffi::c_ulong,
    bits: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
) -> Result<InflateSyncPlan, ::core::ffi::c_int> {
    if input.is_empty() && bits < 8 {
        return Err(crate::zlib_h::Z_BUF_ERROR);
    }

    let (hold, bits, mut have) = if was_sync {
        (hold, bits, have)
    } else {
        let Some((hold, bits, aligned, aligned_len)) = inflate_sync_aligned_bytes(hold, bits)
        else {
            return Err(crate::zlib_h::Z_STREAM_ERROR);
        };
        let mut have = 0;
        syncsearch(&mut have, &aligned[..aligned_len]);
        (hold, bits, have)
    };
    let consumed = syncsearch(&mut have, input);
    Ok(InflateSyncPlan {
        hold,
        bits,
        have,
        consumed,
    })
}

fn inflate_sync_point(
    mode: crate::src::inflate::inflate_mode,
    bits: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    (mode == crate::src::inflate::STORED && bits == 0) as ::core::ffi::c_int
}

fn inflate_validate_wrap(
    wrap: ::core::ffi::c_int,
    check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if check != 0 && wrap != 0 {
        wrap | 4
    } else {
        wrap & !4
    }
}

fn inflate_mark(
    back: ::core::ffi::c_int,
    mode: crate::src::inflate::inflate_mode,
    length: ::core::ffi::c_uint,
    was: ::core::ffi::c_uint,
) -> ::core::ffi::c_long {
    (((back as ::core::ffi::c_long as ::core::ffi::c_ulong) << 16) as ::core::ffi::c_long)
        + if mode == crate::src::inflate::COPY_1 {
            length as ::core::ffi::c_long
        } else if mode == crate::src::inflate::MATCH {
            was.wrapping_sub(length) as ::core::ffi::c_long
        } else {
            0
        }
}

/// Convert the dynamic-code cursor into the public `inflateCodesUsed()`
/// count.  The ABI wrapper supplies addresses after validating the stream;
/// keeping the subtraction here scalar avoids deriving a raw-pointer offset
/// from a possibly incoherent internal cursor.
fn inflate_codes_used(codes_len: usize, next: usize) -> Option<::core::ffi::c_ulong> {
    (next <= codes_len)
        .then_some(next)
        .and_then(|next| ::core::ffi::c_ulong::try_from(next).ok())
}

#[export_name = "inflateSync"]

pub unsafe extern "C" fn inflateSync_ffi(mut strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let (flags, in_0, out) = {
        let strm_ref = &mut *strm;
        let state = &mut *(strm_ref.state as *mut crate::src::inflate::inflate_state);
        let input_len = strm_ref.avail_in as usize;
        let input = if input_len == 0 {
            &[]
        } else if strm_ref.next_in.is_null() {
            return crate::zlib_h::Z_STREAM_ERROR;
        } else {
            ::core::slice::from_raw_parts(strm_ref.next_in as *const u8, input_len)
        };
        // zlib reports this no-progress condition before entering SYNC.  Do
        // it at the boundary before publishing the mode transition below.
        if input.is_empty() && state.bits < 8 {
            return crate::zlib_h::Z_BUF_ERROR;
        }
        let was_sync = state.mode == crate::src::inflate::SYNC;
        if !was_sync {
            // Preserve zlib's transition even if `inflate_sync_plan()` finds
            // an incoherent bit count and reports a stream error.
            state.mode = crate::src::inflate::SYNC;
        }
        let plan = match inflate_sync_plan(input, was_sync, state.hold, state.bits, state.have) {
            Ok(plan) => plan,
            Err(status) => return status,
        };
        state.hold = plan.hold;
        state.bits = plan.bits;
        state.have = plan.have;
        strm_ref.avail_in = strm_ref.avail_in.wrapping_sub(plan.consumed);
        // `consumed` was bounded by the call-scoped input slice above.
        strm_ref.next_in = strm_ref.next_in.wrapping_add(plan.consumed as usize);
        strm_ref.total_in = strm_ref
            .total_in
            .wrapping_add(plan.consumed as crate::stdlib::uLong);
        if state.have != 4 {
            return crate::zlib_h::Z_DATA_ERROR;
        }
        if state.flags == -1 {
            state.wrap = 0;
        } else {
            state.wrap &= !4;
        }
        (
            state.flags,
            strm_ref.total_in as ::core::ffi::c_ulong,
            strm_ref.total_out as ::core::ffi::c_ulong,
        )
    };
    inflate_reset_at_boundary!(strm);
    let strm_ref = &mut *strm;
    let state = &mut *(strm_ref.state as *mut crate::src::inflate::inflate_state);
    strm_ref.total_in = in_0 as crate::stdlib::uLong;
    strm_ref.total_out = out as crate::stdlib::uLong;
    state.flags = flags;
    state.mode = crate::src::inflate::TYPE;
    crate::zlib_h::Z_OK
}
#[export_name = "inflateSyncPoint"]
pub unsafe extern "C" fn inflateSyncPoint_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    inflate_sync_point(state.mode, state.bits)
}
#[export_name = "inflateCopy"]
pub unsafe extern "C" fn inflateCopy_ffi(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    let mut state: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut copy: *mut crate::src::inflate::inflate_state =
        ::core::ptr::null_mut::<crate::src::inflate::inflate_state>();
    let mut window: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if crate::src::inflate::inflate_state_check_at_boundary!(source) != 0 || dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Snapshot callback inputs and allocation shape before invoking either
    // callback. A custom allocator can inspect the public source stream, so
    // no temporary Rust borrow of it may remain live across those calls.
    let (zalloc, zfree, opaque, has_window, whave, wsize, wbits) = {
        let source_ref = &mut *source;
        state = source_ref.state as *mut crate::src::inflate::inflate_state;
        let state_ref = &mut *state;
        (
            source_ref.zalloc,
            source_ref.zfree,
            source_ref.opaque,
            !state_ref.window.is_null(),
            state_ref.whave,
            state_ref.wsize,
            state_ref.wbits,
        )
    };
    // The state check above establishes both callbacks for normal streams.
    // Preserve that ABI error result for malformed callback state instead of
    // panicking while trying to clone it.
    let (Some(zalloc), Some(zfree)) = (zalloc, zfree) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    copy = zalloc(
        opaque,
        1 as crate::stdlib::uInt,
        ::core::mem::size_of::<crate::src::inflate::inflate_state>() as crate::stdlib::uInt,
    ) as *mut crate::src::inflate::inflate_state;
    if copy.is_null() {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    window = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if has_window {
        if whave > wsize {
            zfree(opaque, copy as crate::stdlib::voidpf);
            return crate::zlib_h::Z_STREAM_ERROR;
        }
        let Some(window_len) = inflate_window_len(wbits, 0) else {
            zfree(opaque, copy as crate::stdlib::voidpf);
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let Ok(window_len) = crate::stdlib::uInt::try_from(window_len) else {
            zfree(opaque, copy as crate::stdlib::voidpf);
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        window = zalloc(
            opaque,
            window_len,
            ::core::mem::size_of::<::core::ffi::c_uchar>() as crate::stdlib::uInt,
        ) as *mut ::core::ffi::c_uchar;
        if window.is_null() {
            zfree(opaque, copy as crate::stdlib::voidpf);
            return crate::zlib_h::Z_MEM_ERROR;
        }
    }
    // Both ABI records are `Copy`; assigning them avoids treating their
    // typed layouts as unstructured C byte buffers at this boundary.
    let source_ref = &mut *source;
    let state_ref = &mut *state;
    let dest_ref = &mut *dest;
    let copy_ref = &mut *copy;
    *dest_ref = *source_ref;
    *copy_ref = *state_ref;
    copy_ref.stream_token = dest as usize;
    // Decode tables are now described by fixed-table selectors or indices
    // into the owned `codes` array, so the ordinary record copy above keeps
    // both table sources valid without address rebasing.
    copy_ref.next = copy_ref.next.min(crate::src::inftrees::ENOUGH as usize);
    if !window.is_null() {
        let length = state_ref.whave as usize;
        let source_window = ::core::slice::from_raw_parts(state_ref.window, length);
        let copied_window = ::core::slice::from_raw_parts_mut(window, length);
        copied_window.copy_from_slice(source_window);
    }
    copy_ref.window = window;
    dest_ref.state = copy as *mut crate::src::deflate::internal_state;
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateUndermine"]
pub unsafe extern "C" fn inflateUndermine_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut _subvert: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    state.sane = 1 as ::core::ffi::c_int;
    return crate::zlib_h::Z_DATA_ERROR;
}
#[export_name = "inflateValidate"]
pub unsafe extern "C" fn inflateValidate_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    state.wrap = inflate_validate_wrap(state.wrap, check);
    crate::zlib_h::Z_OK
}
#[export_name = "inflateMark"]
pub unsafe extern "C" fn inflateMark_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_long {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return -((1 as ::core::ffi::c_long) << 16 as ::core::ffi::c_int);
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    inflate_mark(state.back, state.mode, state.length, state.was)
}
#[export_name = "inflateCodesUsed"]
pub unsafe extern "C" fn inflateCodesUsed_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_ulong {
    if crate::src::inflate::inflate_state_check_at_boundary!(strm) != 0 {
        return -1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
    }
    let strm = &mut *strm;
    let state = &mut *(strm.state as *mut crate::src::inflate::inflate_state);
    inflate_codes_used(crate::src::inftrees::ENOUGH as usize, state.next)
        .unwrap_or(-1 as ::core::ffi::c_int as ::core::ffi::c_ulong)
}
