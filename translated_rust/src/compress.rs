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

// The one-shot compressor replenishes `avail_out` from `left` whenever a
// chunk is exhausted. Together they describe the unused portion of the
// caller's original destination range, without deriving a length from raw
// cursor addresses.
fn compress_output_len(
    capacity: crate::stdlib::z_size_t,
    left: crate::stdlib::z_size_t,
    avail_out: crate::stdlib::uInt,
) -> crate::stdlib::z_size_t {
    capacity.wrapping_sub(left.wrapping_add(avail_out as crate::stdlib::z_size_t))
}

// The stream driver uses the same bounded chunking for input and output.
// Keep that arithmetic separate from its raw cursors and FFI arguments.
fn compress_chunk(
    remaining: crate::stdlib::z_size_t,
    max: crate::stdlib::uInt,
) -> (crate::stdlib::uInt, crate::stdlib::z_size_t) {
    let chunk = if remaining > max as crate::stdlib::z_size_t {
        max
    } else {
        remaining as crate::stdlib::uInt
    };
    (
        chunk,
        remaining.wrapping_sub(chunk as crate::stdlib::z_size_t),
    )
}

fn compress_buffers_valid(
    source_is_null: bool,
    source_len: crate::stdlib::z_size_t,
    dest_is_null: bool,
    dest_len: crate::stdlib::z_size_t,
) -> bool {
    !(source_len != 0 && source_is_null || dest_len != 0 && dest_is_null)
}

fn compress_flush(remaining_source: crate::stdlib::z_size_t) -> ::core::ffi::c_int {
    if remaining_source != 0 {
        crate::zlib_h::Z_NO_FLUSH
    } else {
        crate::zlib_h::Z_FINISH
    }
}

// A one-shot compressor starts with no caller buffers or allocator state
// published.  Keep this value-only construction separate from the driver so
// every exit path begins from the same inert stream configuration.
fn compress_stream() -> crate::zlib_h::z_stream {
    crate::zlib_h::z_stream {
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
    }
}

// The one-shot driver receives byte ranges already bound by its ABI adapter.
// It still preserves a null zero-capacity output cursor when publishing the
// temporary stream: `deflate()` distinguishes that C cursor state before it
// examines `avail_out`.
fn compress2_z_bound(
    mut dest: Option<&mut [crate::stdlib::Bytef]>,
    destLen: &mut crate::stdlib::z_size_t,
    source: Option<&[crate::stdlib::Bytef]>,
    mut sourceLen: crate::stdlib::z_size_t,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut stream = compress_stream();
    let mut err: ::core::ffi::c_int = 0;
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut left: crate::stdlib::z_size_t = 0;
    let mut capacity: crate::stdlib::z_size_t = 0;
    if !compress_buffers_valid(source.is_none(), sourceLen, dest.is_none(), *destLen) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    capacity = *destLen;
    left = capacity;
    *destLen = 0 as crate::stdlib::z_size_t;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = crate::src::deflate::deflateInit_(
        Some(&mut stream),
        level,
        Some(&crate::zlib_h::ZLIB_VERSION[0]),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
    );
    if err != crate::zlib_h::Z_OK {
        return err;
    }
    stream.next_out = dest
        .as_deref_mut()
        .map_or(::core::ptr::null_mut(), <[crate::stdlib::Bytef]>::as_mut_ptr);
    stream.avail_out = 0 as crate::stdlib::uInt;
    stream.next_in = source
        .map_or(::core::ptr::null(), <[crate::stdlib::Bytef]>::as_ptr)
        as *mut crate::stdlib::Bytef;
    stream.avail_in = 0 as crate::stdlib::uInt;
    loop {
        if stream.avail_out == 0 as crate::stdlib::uInt {
            (stream.avail_out, left) = compress_chunk(left, max);
        }
        if stream.avail_in == 0 as crate::stdlib::uInt {
            (stream.avail_in, sourceLen) = compress_chunk(sourceLen, max);
        }
        let input = source.and_then(|source| {
            let offset = stream.next_in.addr().checked_sub(source.as_ptr().addr())?;
            source.get(offset..offset.checked_add(stream.avail_in as usize)?)
        });
        let Some(input) = input.or_else(|| (stream.avail_in == 0).then_some(&[][..])) else {
            err = crate::zlib_h::Z_STREAM_ERROR;
            break;
        };
        let output = dest.as_deref_mut().and_then(|dest| {
            let offset = stream.next_out.addr().checked_sub(dest.as_ptr().addr())?;
            dest.get_mut(offset..offset.checked_add(stream.avail_out as usize)?)
        });
        let Some(output) = output.or_else(|| (stream.avail_out == 0).then_some(&mut [][..])) else {
            err = crate::zlib_h::Z_STREAM_ERROR;
            break;
        };
        err = crate::src::deflate::deflate(&mut stream, compress_flush(sourceLen), input, output);
        if err != crate::zlib_h::Z_OK {
            break;
        }
    }
    *destLen = compress_output_len(capacity, left, stream.avail_out);
    // This one-shot stream was initialized with zlib's default callbacks,
    // so its teardown can stay reference-bound instead of re-entering the
    // raw public `deflateEnd()` adapter.
    crate::src::deflate::deflate_end_default_bound(&mut stream);
    return if err == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else {
        err
    };
}
#[export_name = "compress2_z"]

pub unsafe extern "C" fn compress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // SAFETY: the foreign caller supplied the required destination-length
    // output pointer and its advertised byte ranges. The named driver owns
    // all compression behavior after this one ABI binding.
    let dest_len = unsafe { &mut *destLen };
    // This preflight must precede every range binding: zlib reports an
    // invalid null/nonzero pair without touching the other caller cursor.
    if !compress_buffers_valid(source.is_null(), sourceLen, dest.is_null(), *dest_len) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let destination = if dest.is_null() {
        None
    } else if *dest_len == 0 {
        Some(&mut [] as &mut [crate::stdlib::Bytef])
    } else {
        Some(unsafe { ::core::slice::from_raw_parts_mut(dest, *dest_len) })
    };
    let source = if source.is_null() {
        None
    } else if sourceLen == 0 {
        Some(&[] as &[crate::stdlib::Bytef])
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(source, sourceLen) })
    };
    compress2_z_bound(destination, dest_len, source, sourceLen, level)
}

fn compress2_legacy_bound(
    destination: Option<&mut [crate::stdlib::Bytef]>,
    dest_len: &mut crate::stdlib::uLongf,
    source: Option<&[crate::stdlib::Bytef]>,
    source_len: crate::stdlib::uLong,
    level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut got = *dest_len as crate::stdlib::z_size_t;
    let ret = compress2_z_bound(
        destination,
        &mut got,
        source,
        source_len as crate::stdlib::z_size_t,
        level,
    );
    *dest_len = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
}

#[export_name = "compress2"]
pub unsafe extern "C" fn compress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Bind the legacy output and both advertised byte ranges once. Width
    // conversion remains in the named safe dispatcher.
    let dest_len = unsafe { &mut *destLen };
    if !compress_buffers_valid(
        source.is_null(),
        sourceLen as crate::stdlib::z_size_t,
        dest.is_null(),
        *dest_len as crate::stdlib::z_size_t,
    ) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let destination = if dest.is_null() {
        None
    } else if *dest_len == 0 {
        Some(&mut [] as &mut [crate::stdlib::Bytef])
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts_mut(dest, *dest_len as crate::stdlib::z_size_t)
        })
    };
    let source = if source.is_null() {
        None
    } else if sourceLen == 0 {
        Some(&[] as &[crate::stdlib::Bytef])
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t)
        })
    };
    compress2_legacy_bound(destination, dest_len, source, sourceLen, level)
}
#[export_name = "compress_z"]

pub unsafe extern "C" fn compress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Bind the caller ranges before dispatching to the named size_t driver.
    let dest_len = unsafe { &mut *destLen };
    if !compress_buffers_valid(source.is_null(), sourceLen, dest.is_null(), *dest_len) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let destination = if dest.is_null() {
        None
    } else if *dest_len == 0 {
        Some(&mut [] as &mut [crate::stdlib::Bytef])
    } else {
        Some(unsafe { ::core::slice::from_raw_parts_mut(dest, *dest_len) })
    };
    let source = if source.is_null() {
        None
    } else if sourceLen == 0 {
        Some(&[] as &[crate::stdlib::Bytef])
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(source, sourceLen) })
    };
    compress2_z_bound(
        destination,
        dest_len,
        source,
        sourceLen,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    )
}
#[export_name = "compress"]

pub unsafe extern "C" fn compress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Bind the legacy output and byte ranges before the safe dispatcher.
    let dest_len = unsafe { &mut *destLen };
    if !compress_buffers_valid(
        source.is_null(),
        sourceLen as crate::stdlib::z_size_t,
        dest.is_null(),
        *dest_len as crate::stdlib::z_size_t,
    ) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let destination = if dest.is_null() {
        None
    } else if *dest_len == 0 {
        Some(&mut [] as &mut [crate::stdlib::Bytef])
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts_mut(dest, *dest_len as crate::stdlib::z_size_t)
        })
    };
    let source = if source.is_null() {
        None
    } else if sourceLen == 0 {
        Some(&[] as &[crate::stdlib::Bytef])
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t)
        })
    };
    compress2_legacy_bound(
        destination,
        dest_len,
        source,
        sourceLen,
        crate::zlib_h::Z_DEFAULT_COMPRESSION,
    )
}
// Keep the bound calculation value-only so the exported ABI functions only
// select their public integer width.  The wrapping arithmetic and overflow
// sentinel match zlib's unsigned C calculation.
fn compress_bound_z_value(source_len: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    let bound = source_len
        .wrapping_add(source_len >> 12 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 14 as ::core::ffi::c_int)
        .wrapping_add(source_len >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as crate::stdlib::z_size_t);
    if bound < source_len {
        -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
    } else {
        bound
    }
}

fn compress_bound_ulong_value(source_len: crate::stdlib::uLong) -> crate::stdlib::uLong {
    let bound = compress_bound_z_value(source_len as crate::stdlib::z_size_t);
    if bound as crate::stdlib::uLong as crate::stdlib::z_size_t != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        bound as crate::stdlib::uLong
    }
}

pub extern "C" fn compressBound_z(sourceLen: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    compress_bound_z_value(sourceLen)
}
#[export_name = "compressBound_z"]

pub unsafe extern "C" fn compressBound_z_ffi(
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    compressBound_z(sourceLen)
}
pub extern "C" fn compressBound(sourceLen: crate::stdlib::uLong) -> crate::stdlib::uLong {
    compress_bound_ulong_value(sourceLen)
}
#[export_name = "compressBound"]

pub unsafe extern "C" fn compressBound_ffi(
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    compressBound(sourceLen)
}
