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
    if sourceLen > 0 && source.is_null() || dest_capacity > 0 && dest.is_null() {
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

    let mut source_used = 0usize;
    let mut written = 0usize;
    let status = loop {
        let input_len = chunk_len(source_slice.len().saturating_sub(source_used));
        let output_len = chunk_len(dest_slice.len().saturating_sub(written));
        let input = &source_slice[source_used..source_used + input_len];
        let output = &mut dest_slice[written..written + output_len];

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
            if source_used + input_len == source_slice.len() {
                crate::zlib_h::Z_FINISH
            } else {
                crate::zlib_h::Z_NO_FLUSH
            },
        );
        source_used += input_len - stream.avail_in as usize;
        written += output_len - stream.avail_out as usize;

        if status != crate::zlib_h::Z_OK {
            break status;
        }
    };

    *destLen = written;
    crate::src::deflate::deflateEnd(&mut stream);
    if status == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else {
        status
    }
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
    let bound = compress_bound_z_impl(sourceLen as crate::stdlib::z_size_t);
    if bound != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        bound as crate::stdlib::uLong
    }
}
