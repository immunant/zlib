pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::deflate;
pub use crate::src::deflate::deflateEnd;
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CompressBufferPlan {
    source_len: usize,
    dest_capacity: usize,
}

fn plan_compress2_buffers(
    source_len: crate::stdlib::z_size_t,
    dest_capacity: crate::stdlib::z_size_t,
    source_is_null: bool,
    dest_is_null: bool,
) -> Result<CompressBufferPlan, ::core::ffi::c_int> {
    if source_len > 0 && source_is_null || dest_capacity > 0 && dest_is_null {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }

    Ok(CompressBufferPlan {
        source_len,
        dest_capacity,
    })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CompressResult {
    status: ::core::ffi::c_int,
    dest_len: usize,
}

fn finish_compress(status: ::core::ffi::c_int, dest_len: usize) -> CompressResult {
    CompressResult {
        status: if status == crate::zlib_h::Z_STREAM_END {
            crate::zlib_h::Z_OK
        } else {
            status
        },
        dest_len,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CompressChunk {
    input_start: usize,
    input_len: usize,
    output_start: usize,
    output_len: usize,
    flush: ::core::ffi::c_int,
}

impl CompressChunk {
    fn record_progress(
        self,
        source_progress: &mut CompressProgress,
        dest_progress: &mut CompressProgress,
        input_available: crate::stdlib::uInt,
        output_available: crate::stdlib::uInt,
    ) {
        source_progress.record_available(self.input_len, input_available);
        dest_progress.record_available(self.output_len, output_available);
    }
}

fn next_compress_chunk(
    source_progress: CompressProgress,
    dest_progress: CompressProgress,
) -> CompressChunk {
    let input_len = source_progress.next_chunk_len();

    CompressChunk {
        input_start: source_progress.used,
        input_len,
        output_start: dest_progress.used,
        output_len: dest_progress.next_chunk_len(),
        flush: source_progress.flush_mode(input_len),
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

    let plan = match plan_compress2_buffers(sourceLen, *destLen, source.is_null(), dest.is_null()) {
        Ok(plan) => plan,
        Err(status) => return status,
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
    let init_status = crate::src::deflate::deflateInit2_(
        &mut stream,
        level,
        crate::zlib_h::Z_DEFLATED,
        crate::stdlib::MAX_WBITS,
        crate::zutil_h::DEF_MEM_LEVEL,
        crate::zlib_h::Z_DEFAULT_STRATEGY,
        crate::zlib_h::ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
    );
    if init_status != crate::zlib_h::Z_OK {
        return init_status;
    }

    let mut source_progress = CompressProgress::new(plan.source_len);
    let mut dest_progress = CompressProgress::new(plan.dest_capacity);
    let status = loop {
        let chunk = next_compress_chunk(source_progress, dest_progress);

        stream.next_in = if chunk.input_len == 0 {
            source as *mut crate::stdlib::Bytef
        } else {
            source.wrapping_add(chunk.input_start) as *mut crate::stdlib::Bytef
        };
        stream.avail_in = chunk.input_len as crate::stdlib::uInt;
        stream.next_out = if chunk.output_len == 0 {
            dest
        } else {
            dest.wrapping_add(chunk.output_start)
        };
        stream.avail_out = chunk.output_len as crate::stdlib::uInt;

        let status = crate::src::deflate::deflate(&mut stream, chunk.flush);
        chunk.record_progress(
            &mut source_progress,
            &mut dest_progress,
            stream.avail_in,
            stream.avail_out,
        );

        if status != crate::zlib_h::Z_OK {
            break status;
        }
    };

    let result = finish_compress(status, dest_progress.used);
    *destLen = result.dest_len;
    crate::src::deflate::deflateEnd(&mut stream);
    result.status
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
pub extern "C" fn compressBound_z_ffi(
    sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    compress_bound_z_impl(sourceLen)
}

#[export_name = "compressBound"]
pub extern "C" fn compressBound_ffi(sourceLen: crate::stdlib::uLong) -> crate::stdlib::uLong {
    compress_bound(sourceLen)
}

#[cfg(test)]
mod tests {
    use super::{
        compress_bound, compress_bound_z_impl, finish_compress, next_compress_chunk,
        plan_compress2_buffers, CompressBufferPlan, CompressProgress, MAX_CHUNK,
    };

    #[test]
    fn compress2_buffer_validation_allows_null_for_empty_buffers() {
        assert_eq!(
            plan_compress2_buffers(0, 0, true, true),
            Ok(CompressBufferPlan {
                source_len: 0,
                dest_capacity: 0,
            })
        );
        assert!(plan_compress2_buffers(0, 0, false, false).is_ok());
    }

    #[test]
    fn compress2_buffer_validation_rejects_null_nonempty_source() {
        assert_eq!(
            plan_compress2_buffers(1, 0, true, false),
            Err(crate::zlib_h::Z_STREAM_ERROR)
        );
        assert_eq!(
            plan_compress2_buffers(1, 0, true, true),
            Err(crate::zlib_h::Z_STREAM_ERROR)
        );
    }

    #[test]
    fn compress2_buffer_validation_rejects_null_nonempty_destination() {
        assert_eq!(
            plan_compress2_buffers(0, 1, false, true),
            Err(crate::zlib_h::Z_STREAM_ERROR)
        );
        assert_eq!(
            plan_compress2_buffers(0, 1, true, true),
            Err(crate::zlib_h::Z_STREAM_ERROR)
        );
    }

    #[test]
    fn compress2_buffer_validation_accepts_present_nonempty_buffers() {
        assert_eq!(
            plan_compress2_buffers(1, 1, false, false),
            Ok(CompressBufferPlan {
                source_len: 1,
                dest_capacity: 1,
            })
        );
    }

    #[test]
    fn compress_chunk_ranges_follow_progress_and_handle_empty_windows() {
        let source = b"abcdef";
        let source_progress = CompressProgress {
            total: source.len(),
            used: 2,
        };
        let dest_progress = CompressProgress { total: 8, used: 3 };
        let chunk = next_compress_chunk(source_progress, dest_progress);
        assert_eq!(chunk.input_start, 2);
        assert_eq!(chunk.input_len, 4);
        assert_eq!(chunk.output_start, 3);
        assert_eq!(chunk.output_len, 5);

        let source_progress = CompressProgress {
            total: source.len(),
            used: source.len(),
        };
        let dest_progress = CompressProgress { total: 8, used: 8 };
        let chunk = next_compress_chunk(source_progress, dest_progress);
        assert_eq!(chunk.input_len, 0);
        assert_eq!(chunk.output_len, 0);
    }

    #[test]
    fn chunk_plan_keeps_input_output_and_flush_selection_together() {
        let source_progress = CompressProgress { total: 5, used: 2 };
        let dest_progress = CompressProgress { total: 8, used: 3 };
        assert_eq!(
            next_compress_chunk(source_progress, dest_progress),
            super::CompressChunk {
                input_start: 2,
                input_len: 3,
                output_start: 3,
                output_len: 5,
                flush: crate::zlib_h::Z_FINISH,
            }
        );

        let Some(total) = MAX_CHUNK.checked_add(1) else {
            return;
        };
        assert_eq!(
            next_compress_chunk(CompressProgress::new(total), CompressProgress::new(0)).flush,
            crate::zlib_h::Z_NO_FLUSH
        );
    }

    #[test]
    fn chunk_records_matching_input_and_output_progress() {
        let mut source_progress = CompressProgress::new(10);
        let mut dest_progress = CompressProgress::new(12);
        let chunk = super::CompressChunk {
            input_start: 0,
            input_len: 7,
            output_start: 0,
            output_len: 9,
            flush: crate::zlib_h::Z_NO_FLUSH,
        };

        chunk.record_progress(&mut source_progress, &mut dest_progress, 2, 3);

        assert_eq!(source_progress.used, 5);
        assert_eq!(dest_progress.used, 6);
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
    fn compress_result_normalizes_only_stream_end_and_preserves_length() {
        assert_eq!(
            finish_compress(crate::zlib_h::Z_STREAM_END, 7),
            super::CompressResult {
                status: crate::zlib_h::Z_OK,
                dest_len: 7,
            }
        );
        assert_eq!(
            finish_compress(crate::zlib_h::Z_STREAM_ERROR, 3),
            super::CompressResult {
                status: crate::zlib_h::Z_STREAM_ERROR,
                dest_len: 3,
            }
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
