pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

use crate::src::compress::OneShotCursor;
pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflate;
pub use crate::src::inflate::inflateEnd;
pub use crate::src::inflate::inflateInit_;
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

struct UncompressProgress {
    status: ::core::ffi::c_int,
    source_remaining: crate::stdlib::z_size_t,
    output_remaining: crate::stdlib::z_size_t,
}

fn uncompress2_z(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> Result<UncompressProgress, ::core::ffi::c_int> {
    let mut stream: crate::zlib_h::z_stream = crate::zlib_h::z_stream {
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
    let mut err: ::core::ffi::c_int;
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut input = OneShotCursor::new(source.len());
    let mut output = OneShotCursor::new(dest.len());
    stream.next_in = source.as_ptr().cast_mut();
    stream.avail_in = 0 as crate::stdlib::uInt;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    // The stream is an ABI mirror used only while this slice-backed loop is
    // active.  Empty output still needs a non-null cursor for inflate's ABI.
    unsafe {
        err = crate::src::inflate::inflateInit_(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
    }
    if err != crate::zlib_h::Z_OK {
        return Err(err);
    }
    stream.next_out = if dest.is_empty() {
        &raw mut stream.reserved as *mut crate::stdlib::Bytef
    } else {
        dest.as_mut_ptr()
    };
    stream.avail_out = 0 as crate::stdlib::uInt;
    loop {
        if stream.avail_out == 0 as crate::stdlib::uInt {
            stream.avail_out = output.next_chunk(max);
        }
        if stream.avail_in == 0 as crate::stdlib::uInt {
            stream.avail_in = input.next_chunk(max);
        }
        unsafe {
            err = crate::src::inflate::inflate(
                &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
                crate::zlib_h::Z_NO_FLUSH,
            );
        }
        if err != crate::zlib_h::Z_OK {
            break;
        }
    }
    let len = input
        .remaining()
        .wrapping_add(stream.avail_in as crate::stdlib::z_size_t);
    let left = output
        .remaining()
        .wrapping_add(stream.avail_out as crate::stdlib::z_size_t);
    unsafe {
        crate::src::inflate::inflateEnd(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
    }
    Ok(UncompressProgress {
        status: if err == crate::zlib_h::Z_STREAM_END {
            crate::zlib_h::Z_OK
        } else if err == crate::zlib_h::Z_NEED_DICT {
            crate::zlib_h::Z_DATA_ERROR
        } else if err == crate::zlib_h::Z_BUF_ERROR && len == 0 as crate::stdlib::z_size_t {
            crate::zlib_h::Z_DATA_ERROR
        } else {
            err
        },
        source_remaining: len,
        output_remaining: left,
    })
}
#[export_name = "uncompress2_z"]

pub unsafe extern "C" fn uncompress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if sourceLen.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source_capacity = *sourceLen;
    let dest_capacity = *destLen;
    if source_capacity != 0 && source.is_null() || dest_capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if source_capacity == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, source_capacity)
    };
    let dest = if dest_capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_capacity)
    };
    let progress = match uncompress2_z(dest, source) {
        Ok(progress) => progress,
        Err(err) => return err,
    };
    if sourceLen == destLen {
        // Preserve the existing alias case: C length publication is an ABI
        // boundary operation, so the core returns both progress values.
        *sourceLen = source_capacity
            .wrapping_sub(progress.source_remaining)
            .wrapping_sub(progress.output_remaining);
    } else {
        *sourceLen = source_capacity.wrapping_sub(progress.source_remaining);
        *destLen = dest_capacity.wrapping_sub(progress.output_remaining);
    }
    progress.status
}
fn uncompress2(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> Result<UncompressProgress, ::core::ffi::c_int> {
    uncompress2_z(dest, source)
}
#[export_name = "uncompress2"]

pub unsafe extern "C" fn uncompress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if sourceLen.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source_capacity = *sourceLen as crate::stdlib::z_size_t;
    let dest_capacity = *destLen as crate::stdlib::z_size_t;
    if source_capacity != 0 && source.is_null() || dest_capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if source_capacity == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, source_capacity)
    };
    let dest = if dest_capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_capacity)
    };
    let progress = match uncompress2(dest, source) {
        Ok(progress) => progress,
        Err(err) => return err,
    };
    *sourceLen = source_capacity.wrapping_sub(progress.source_remaining) as crate::stdlib::uLong;
    *destLen = dest_capacity.wrapping_sub(progress.output_remaining) as crate::stdlib::uLongf;
    progress.status
}
fn uncompress_z(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> Result<UncompressProgress, ::core::ffi::c_int> {
    uncompress2_z(dest, source)
}
#[export_name = "uncompress_z"]

pub unsafe extern "C" fn uncompress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_capacity = *destLen;
    if sourceLen != 0 && source.is_null() || dest_capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen)
    };
    let dest = if dest_capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_capacity)
    };
    let progress = match uncompress_z(dest, source) {
        Ok(progress) => progress,
        Err(err) => return err,
    };
    *destLen = dest_capacity.wrapping_sub(progress.output_remaining);
    progress.status
}
fn uncompress(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> Result<UncompressProgress, ::core::ffi::c_int> {
    uncompress2(dest, source)
}
#[export_name = "uncompress"]

pub unsafe extern "C" fn uncompress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_capacity = *destLen as crate::stdlib::z_size_t;
    let source_capacity = sourceLen as crate::stdlib::z_size_t;
    if source_capacity != 0 && source.is_null() || dest_capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if source_capacity == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, source_capacity)
    };
    let dest = if dest_capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, dest_capacity)
    };
    let progress = match uncompress(dest, source) {
        Ok(progress) => progress,
        Err(err) => return err,
    };
    *destLen = dest_capacity.wrapping_sub(progress.output_remaining) as crate::stdlib::uLongf;
    progress.status
}
