pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::deflate;
pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit_;
pub use crate::src::deflate::internal_state;
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
pub use crate::zlib_h::Z_DEFAULT_COMPRESSION;
pub use crate::zlib_h::Z_FINISH;
pub use crate::zlib_h::Z_NO_FLUSH;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;

const MAX_CHUNK: usize = crate::stdlib::uInt::MAX as usize;

fn chunk_len(remaining: usize) -> usize {
    remaining.min(MAX_CHUNK)
}

fn compress2_buffers_are_valid(
    source_len: crate::stdlib::z_size_t,
    dest_capacity: crate::stdlib::z_size_t,
    source_is_null: bool,
    dest_is_null: bool,
) -> bool {
    !(source_len > 0 && source_is_null || dest_capacity > 0 && dest_is_null)
}

fn normalize_compress_status(status: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if status == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else {
        status
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CompressProgress {
    total: usize,
    used: usize,
}

impl CompressProgress {
    fn new(total: usize) -> Self {
        Self { total, used: 0 }
    }

    fn remaining(self) -> usize {
        self.total.saturating_sub(self.used)
    }

    fn next_chunk_len(self) -> usize {
        chunk_len(self.remaining())
    }

    fn is_final_chunk(self, scheduled: usize) -> bool {
        scheduled == self.remaining()
    }

    fn flush_mode(self, scheduled: usize) -> ::core::ffi::c_int {
        if self.is_final_chunk(scheduled) {
            crate::zlib_h::Z_FINISH
        } else {
            crate::zlib_h::Z_NO_FLUSH
        }
    }

    fn record_available(&mut self, scheduled: usize, available: crate::stdlib::uInt) {
        self.used = self
            .used
            .wrapping_add(scheduled.wrapping_sub(available as usize));
    }
}

fn compress_bound_z_impl(source_len: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    let bound = source_len
        .wrapping_add(source_len >> 12)
        .wrapping_add(source_len >> 14)
        .wrapping_add(source_len >> 25)
        .wrapping_add(13);

    if bound < source_len {
        -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
    } else {
        bound
    }
}

fn compress_bound(source_len: crate::stdlib::uLong) -> crate::stdlib::uLong {
    let bound = compress_bound_z_impl(source_len as crate::stdlib::z_size_t);

    if (bound as crate::stdlib::uLong) as crate::stdlib::z_size_t != bound {
        crate::stdlib::uLong::MAX
    } else {
        bound as crate::stdlib::uLong
    }
}

#[export_name = "compress2_z"]
pub unsafe extern "C" fn compress2_z_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::z_size_t,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::z_size_t,
    level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let dest_capacity = *destLen;
    if !compress2_buffers_are_valid(sourceLen, dest_capacity, source.is_null(), dest.is_null()) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let source_slice = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen)
    };
    let dest_slice = if dest_capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_capacity)
    };
    *destLen = 0;

    let mut stream = crate::zlib_h::z_stream_s {
        next_in: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<crate::stdlib::Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        state: ::core::ptr::null_mut::<crate::src::deflate::internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let init_status = crate::src::deflate::deflateInit_(
        &mut stream,
        level,
        crate::zlib_h::ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
    );
    if init_status != crate::zlib_h::Z_OK {
        return init_status;
    }

    let mut source_progress = CompressProgress::new(source_slice.len());
    let mut dest_progress = CompressProgress::new(dest_slice.len());
    let status = loop {
        let input_len = source_progress.next_chunk_len();
        let output_len = dest_progress.next_chunk_len();
        let input = &source_slice[source_progress.used..source_progress.used + input_len];
        let output = &mut dest_slice[dest_progress.used..dest_progress.used + output_len];

        stream.next_in = if input.is_empty() {
            source as *mut crate::stdlib::Bytef
        } else {
            input.as_ptr() as *mut crate::stdlib::Bytef
        };
        stream.avail_in = input.len() as crate::stdlib::uInt;
        stream.next_out = if output.is_empty() {
            dest
        } else {
            output.as_mut_ptr()
        };
        stream.avail_out = output.len() as crate::stdlib::uInt;

        let status = crate::src::deflate::deflate(
            &mut stream,
            source_progress.flush_mode(input_len),
        );
        source_progress.record_available(input_len, stream.avail_in);
        dest_progress.record_available(output_len, stream.avail_out);

        if status != crate::zlib_h::Z_OK {
            break status;
        }
    };

    *destLen = dest_progress.used;
    crate::src::deflate::deflateEnd(&mut stream);
    normalize_compress_status(status)
}

#[export_name = "compress2"]
pub unsafe extern "C" fn compress2_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::uLongf,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::uLong,
    level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }

    let mut got = *destLen as crate::stdlib::z_size_t;
    let status = compress2_z_ffi(
        dest,
        &mut got,
        source,
        sourceLen as crate::stdlib::z_size_t,
        level,
    );
    *destLen = got as crate::stdlib::uLongf;
    status
}

#[export_name = "compress_z"]
pub unsafe extern "C" fn compress_z_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::z_size_t,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    compress2_z_ffi(
        dest,
        destLen,
        source,
        sourceLen,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    )
}

#[export_name = "compress"]
pub unsafe extern "C" fn compress_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::uLongf,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    compress2_ffi(
        dest,
        destLen,
        source,
        sourceLen,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    )
}

#[export_name = "compressBound_z"]
pub unsafe extern "C" fn compressBound_z_ffi(
    sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    compress_bound_z_impl(sourceLen)
}

#[export_name = "compressBound"]
pub unsafe extern "C" fn compressBound_ffi(
    sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    compress_bound(sourceLen)
}

#[cfg(test)]
mod tests {
    use super::{
        compress2_buffers_are_valid, compress_bound, compress_bound_z_impl,
        normalize_compress_status, CompressProgress, MAX_CHUNK,
    };

    #[test]
    fn compress2_buffer_validation_allows_null_for_empty_buffers() {
        assert!(compress2_buffers_are_valid(0, 0, true, true));
        assert!(compress2_buffers_are_valid(0, 0, false, false));
    }

    #[test]
    fn compress2_buffer_validation_rejects_null_nonempty_source() {
        assert!(!compress2_buffers_are_valid(1, 0, true, false));
        assert!(!compress2_buffers_are_valid(1, 0, true, true));
    }

    #[test]
    fn compress2_buffer_validation_rejects_null_nonempty_destination() {
        assert!(!compress2_buffers_are_valid(0, 1, false, true));
        assert!(!compress2_buffers_are_valid(0, 1, true, true));
    }

    #[test]
    fn compress2_buffer_validation_accepts_present_nonempty_buffers() {
        assert!(compress2_buffers_are_valid(1, 1, false, false));
    }

    #[test]
    fn progress_schedules_uint_sized_chunks_and_tracks_consumption() {
        let Some(total) = MAX_CHUNK.checked_add(3) else {
            return;
        };
        let mut progress = CompressProgress::new(total);
        assert_eq!(progress.next_chunk_len(), MAX_CHUNK);
        assert!(!progress.is_final_chunk(MAX_CHUNK));
        progress.record_available(MAX_CHUNK, 2);
        assert_eq!(progress.used, MAX_CHUNK - 2);
        assert_eq!(progress.next_chunk_len(), 5);
        assert!(progress.is_final_chunk(5));
        progress.record_available(5, 0);
        assert_eq!(progress.used, total);
        assert_eq!(progress.remaining(), 0);
    }

    #[test]
    fn progress_selects_finish_only_for_the_final_chunk() {
        let mut progress = CompressProgress::new(4);
        assert_eq!(progress.flush_mode(3), crate::zlib_h::Z_NO_FLUSH);
        assert_eq!(progress.flush_mode(4), crate::zlib_h::Z_FINISH);

        progress.record_available(4, 1);
        assert_eq!(progress.flush_mode(1), crate::zlib_h::Z_FINISH);
    }

    #[test]
    fn compress_status_normalization_maps_only_stream_end_to_ok() {
        assert_eq!(
            normalize_compress_status(crate::zlib_h::Z_STREAM_END),
            crate::zlib_h::Z_OK
        );
        assert_eq!(
            normalize_compress_status(crate::zlib_h::Z_STREAM_ERROR),
            crate::zlib_h::Z_STREAM_ERROR
        );
    }

    #[test]
    fn compress_bound_matches_the_z_size_formula() {
        assert_eq!(compress_bound(0), 13);
        assert_eq!(
            compress_bound(12_345),
            compress_bound_z_impl(12_345) as crate::stdlib::uLong
        );
    }

    #[test]
    fn compress_bound_preserves_the_overflow_sentinel() {
        assert_eq!(
            compress_bound_z_impl(crate::stdlib::z_size_t::MAX),
            crate::stdlib::z_size_t::MAX
        );
        assert_eq!(
            compress_bound(crate::stdlib::uLong::MAX),
            crate::stdlib::uLong::MAX
        );
    }
}
