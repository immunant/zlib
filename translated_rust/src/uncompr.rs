pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflateEnd_ffi as inflateEnd;
pub use crate::src::inflate::inflateInit2_;
pub use crate::src::inflate::inflate_ffi as inflate;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::uLongf;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_NEED_DICT;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ChunkedProgress {
    total: crate::stdlib::z_size_t,
    unassigned: crate::stdlib::z_size_t,
}

fn replenish_scalar(unassigned: &mut crate::stdlib::z_size_t, available: &mut crate::stdlib::uInt) {
    if *available != 0 {
        return;
    }

    *available = (*unassigned).min(crate::stdlib::uInt::MAX as crate::stdlib::z_size_t)
        as crate::stdlib::uInt;
    *unassigned = (*unassigned).wrapping_sub(*available as crate::stdlib::z_size_t);
}

impl ChunkedProgress {
    fn new(total: crate::stdlib::z_size_t) -> Self {
        Self {
            total,
            unassigned: total,
        }
    }

    fn replenish(&mut self, available: &mut crate::stdlib::uInt) {
        replenish_scalar(&mut self.unassigned, available);
    }

    fn remaining(self, available: crate::stdlib::uInt) -> crate::stdlib::z_size_t {
        self.unassigned
            .wrapping_add(available as crate::stdlib::z_size_t)
    }

    fn consumed(self, available: crate::stdlib::uInt) -> crate::stdlib::z_size_t {
        self.total.wrapping_sub(self.remaining(available))
    }
}

fn normalize_uncompress_status(
    err: ::core::ffi::c_int,
    input_remaining: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    match (err, input_remaining == 0) {
        (crate::zlib_h::Z_STREAM_END, _) => crate::zlib_h::Z_OK,
        (crate::zlib_h::Z_NEED_DICT, _) | (crate::zlib_h::Z_BUF_ERROR, true) => {
            crate::zlib_h::Z_DATA_ERROR
        }
        _ => err,
    }
}

fn has_missing_uncompress_lengths(dest_len_is_null: bool, source_len_is_null: bool) -> bool {
    dest_len_is_null || source_len_is_null
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct UncompressBufferLengths {
    dest: crate::stdlib::z_size_t,
    source: crate::stdlib::z_size_t,
}

impl UncompressBufferLengths {
    fn new(dest: crate::stdlib::z_size_t, source: crate::stdlib::z_size_t) -> Self {
        Self { dest, source }
    }

    fn buffer_setup(self, dest_is_null: bool, source_is_null: bool) -> UncompressBufferSetup {
        if self.source > 0 && source_is_null || self.dest > 0 && dest_is_null {
            UncompressBufferSetup::Invalid
        } else if self.dest == 0 && dest_is_null {
            UncompressBufferSetup::DummyOutput
        } else {
            UncompressBufferSetup::Direct
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum UncompressBufferSetup {
    Invalid,
    Direct,
    DummyOutput,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct UncompressOutcome {
    status: ::core::ffi::c_int,
    source_len: crate::stdlib::z_size_t,
    dest_len: crate::stdlib::z_size_t,
}

fn uncompress_outcome(
    err: ::core::ffi::c_int,
    input_progress: ChunkedProgress,
    input_available: crate::stdlib::uInt,
    output_progress: ChunkedProgress,
    output_available: crate::stdlib::uInt,
) -> UncompressOutcome {
    let input_remaining = input_progress.remaining(input_available);
    UncompressOutcome {
        status: normalize_uncompress_status(err, input_remaining),
        source_len: input_progress.consumed(input_available),
        dest_len: output_progress.consumed(output_available),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct LegacyUncompressLengths {
    dest: crate::stdlib::z_size_t,
    source: crate::stdlib::z_size_t,
}

impl LegacyUncompressLengths {
    fn from_legacy(dest: crate::stdlib::uLongf, source: crate::stdlib::uLong) -> Self {
        Self {
            dest: dest as crate::stdlib::z_size_t,
            source: source as crate::stdlib::z_size_t,
        }
    }

    fn from_z(dest: crate::stdlib::z_size_t, source: crate::stdlib::z_size_t) -> Self {
        Self { dest, source }
    }

    fn into_legacy(self) -> (crate::stdlib::uLongf, crate::stdlib::uLong) {
        (
            self.dest as crate::stdlib::uLong as crate::stdlib::uLongf,
            self.source as crate::stdlib::uLong,
        )
    }
}

#[export_name = "uncompress2_z"]
pub unsafe extern "C" fn uncompress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if has_missing_uncompress_lengths(destLen.is_null(), sourceLen.is_null()) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let lengths = UncompressBufferLengths::new(*destLen, *sourceLen);
    let buffer_setup = lengths.buffer_setup(dest.is_null(), source.is_null());
    if buffer_setup == UncompressBufferSetup::Invalid {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let mut stream: crate::zlib_h::z_stream = crate::zlib_h::z_stream_s {
        next_in: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        state: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut dummy = 0 as crate::stdlib::Bytef;
    if buffer_setup == UncompressBufferSetup::DummyOutput {
        dest = &raw mut dummy;
    }

    stream.next_in = source as *mut crate::stdlib::Bytef;
    stream.next_out = dest;
    let err = crate::src::inflate::inflateInit2_(
        &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
        crate::zutil_h::DEF_WBITS,
        crate::zlib_h::ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
    );
    if err != crate::zlib_h::Z_OK {
        return err;
    }

    let mut input_progress = ChunkedProgress::new(lengths.source);
    let mut output_progress = ChunkedProgress::new(lengths.dest);
    let err = loop {
        output_progress.replenish(&mut stream.avail_out);
        input_progress.replenish(&mut stream.avail_in);
        let err = crate::src::inflate::inflate_ffi(
            &raw mut stream as *mut crate::zlib_h::z_stream_s,
            crate::zlib_h::Z_NO_FLUSH,
        );
        if err != crate::zlib_h::Z_OK {
            break err;
        }
    };
    let outcome = uncompress_outcome(
        err,
        input_progress,
        stream.avail_in,
        output_progress,
        stream.avail_out,
    );
    *sourceLen = outcome.source_len;
    *destLen = outcome.dest_len;
    crate::src::inflate::inflateEnd_ffi(&raw mut stream as *mut crate::zlib_h::z_stream_s);
    outcome.status
}

#[export_name = "uncompress2"]
pub unsafe extern "C" fn uncompress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if has_missing_uncompress_lengths(destLen.is_null(), sourceLen.is_null()) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let lengths = LegacyUncompressLengths::from_legacy(*destLen, *sourceLen);
    let mut got = lengths.dest;
    let mut used = lengths.source;
    let ret = uncompress2_z_ffi(dest, &raw mut got, source, &raw mut used);
    let (dest_len, source_len) = LegacyUncompressLengths::from_z(got, used).into_legacy();
    *sourceLen = source_len;
    *destLen = dest_len;
    ret
}

#[export_name = "uncompress_z"]
pub unsafe extern "C" fn uncompress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let mut used = sourceLen;
    uncompress2_z_ffi(dest, destLen, source, &raw mut used)
}

#[export_name = "uncompress"]
pub unsafe extern "C" fn uncompress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut used = sourceLen;
    uncompress2_ffi(dest, destLen, source, &raw mut used)
}

#[cfg(test)]
mod tests {
    use super::{
        has_missing_uncompress_lengths, normalize_uncompress_status, replenish_scalar,
        uncompress_outcome, ChunkedProgress, LegacyUncompressLengths, UncompressBufferLengths,
        UncompressBufferSetup,
    };

    #[test]
    fn buffer_setup_preserves_validation_and_dummy_output_rules() {
        assert_eq!(
            UncompressBufferLengths::new(0, 0).buffer_setup(true, true),
            UncompressBufferSetup::DummyOutput
        );
        assert_eq!(
            UncompressBufferLengths::new(1, 0).buffer_setup(true, false),
            UncompressBufferSetup::Invalid
        );
        assert_eq!(
            UncompressBufferLengths::new(0, 1).buffer_setup(false, true),
            UncompressBufferSetup::Invalid
        );
        assert_eq!(
            UncompressBufferLengths::new(1, 1).buffer_setup(false, false),
            UncompressBufferSetup::Direct
        );
        assert_eq!(
            UncompressBufferLengths::new(0, 0).buffer_setup(false, false),
            UncompressBufferSetup::Direct
        );
    }

    #[test]
    fn length_validation_rejects_either_missing_length() {
        assert!(has_missing_uncompress_lengths(true, false));
        assert!(has_missing_uncompress_lengths(false, true));
        assert!(!has_missing_uncompress_lengths(false, false));
    }

    #[test]
    fn legacy_lengths_preserve_the_wrapper_cast_sequence() {
        let lengths = LegacyUncompressLengths::from_legacy(7, 11);
        assert_eq!(lengths.dest, 7);
        assert_eq!(lengths.source, 11);
        assert_eq!(
            LegacyUncompressLengths::from_z(
                crate::stdlib::z_size_t::MAX,
                crate::stdlib::z_size_t::MAX,
            )
            .into_legacy(),
            (
                crate::stdlib::z_size_t::MAX as crate::stdlib::uLong as crate::stdlib::uLongf,
                crate::stdlib::z_size_t::MAX as crate::stdlib::uLong,
            )
        );
    }

    #[test]
    fn replenish_scalar_caps_to_the_uint_bound() {
        let mut unassigned = crate::stdlib::z_size_t::MAX;
        let mut available = 0;

        replenish_scalar(&mut unassigned, &mut available);

        let expected = crate::stdlib::z_size_t::MAX
            .min(crate::stdlib::uInt::MAX as crate::stdlib::z_size_t)
            as crate::stdlib::uInt;
        assert_eq!(available, expected);
        assert_eq!(
            unassigned,
            crate::stdlib::z_size_t::MAX.wrapping_sub(expected as crate::stdlib::z_size_t)
        );
    }

    #[test]
    fn replenish_scalar_keeps_nonzero_availability() {
        let mut unassigned = 10;
        let mut available = 7;

        replenish_scalar(&mut unassigned, &mut available);

        assert_eq!(available, 7);
        assert_eq!(unassigned, 10);
    }

    #[test]
    fn progress_refills_in_uint_sized_chunks() {
        let Some(total) = (crate::stdlib::uInt::MAX as crate::stdlib::z_size_t).checked_add(3)
        else {
            return;
        };
        let mut progress = ChunkedProgress::new(total);
        let mut available = 0;
        progress.replenish(&mut available);
        assert_eq!(available, crate::stdlib::uInt::MAX);
        assert_eq!(progress.unassigned, 3);
        available = 0;
        progress.replenish(&mut available);
        assert_eq!(available, 3);
        assert_eq!(progress.unassigned, 0);
    }

    #[test]
    fn progress_keeps_scheduled_availability_and_accounts_consumption() {
        let mut progress = ChunkedProgress::new(10);
        let mut available = 0;
        progress.replenish(&mut available);
        assert_eq!(available, 10);
        available = 7;
        progress.replenish(&mut available);
        assert_eq!(available, 7);
        assert_eq!(progress.remaining(available), 7);
        assert_eq!(progress.consumed(available), 3);
    }

    #[test]
    fn progress_with_no_work_stays_empty() {
        let mut progress = ChunkedProgress::new(0);
        let mut available = 0;
        progress.replenish(&mut available);
        assert_eq!(available, 0);
        assert_eq!(progress.remaining(available), 0);
        assert_eq!(progress.consumed(available), 0);
    }

    #[test]
    fn status_normalization_preserves_all_result_cases() {
        assert_eq!(
            normalize_uncompress_status(crate::zlib_h::Z_STREAM_END, 3),
            crate::zlib_h::Z_OK
        );
        assert_eq!(
            normalize_uncompress_status(crate::zlib_h::Z_NEED_DICT, 3),
            crate::zlib_h::Z_DATA_ERROR
        );
        assert_eq!(
            normalize_uncompress_status(crate::zlib_h::Z_BUF_ERROR, 0),
            crate::zlib_h::Z_DATA_ERROR
        );
        assert_eq!(
            normalize_uncompress_status(crate::zlib_h::Z_BUF_ERROR, 1),
            crate::zlib_h::Z_BUF_ERROR
        );
        assert_eq!(
            normalize_uncompress_status(crate::zlib_h::Z_STREAM_ERROR, 0),
            crate::zlib_h::Z_STREAM_ERROR
        );
    }

    #[test]
    fn outcome_reports_normalized_status_and_progress() {
        let mut input = ChunkedProgress::new(10);
        let mut output = ChunkedProgress::new(8);
        let mut input_available = 0;
        let mut output_available = 0;
        input.replenish(&mut input_available);
        output.replenish(&mut output_available);

        let outcome = uncompress_outcome(crate::zlib_h::Z_STREAM_END, input, 4, output, 3);

        assert_eq!(outcome.status, crate::zlib_h::Z_OK);
        assert_eq!(outcome.source_len, 6);
        assert_eq!(outcome.dest_len, 5);
    }
}
