pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflateEnd;
pub use crate::src::inflate::inflateInit_;
pub use crate::src::inflate::inflate_stream as inflate;
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

// Chunk selection is independent of the raw stream cursor.  Keep the
// one-shot decompressor's pointer-bearing implementation focused on the
// actual inflate calls.
fn uncompress_chunk(
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

fn uncompress_buffers_valid(
    source_is_null: bool,
    source_len: crate::stdlib::z_size_t,
    dest_is_null: bool,
    dest_len: crate::stdlib::z_size_t,
) -> bool {
    !(source_len != 0 && source_is_null || dest_len != 0 && dest_is_null)
}

fn uncompress_result(
    err: ::core::ffi::c_int,
    remaining_input: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if err == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else if err == crate::zlib_h::Z_NEED_DICT {
        crate::zlib_h::Z_DATA_ERROR
    } else if err == crate::zlib_h::Z_BUF_ERROR && remaining_input == 0 {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        err
    }
}

// A one-shot decompressor likewise begins with no caller buffers or allocator
// state published.  Keep this value-only setup out of the pointer-bearing
// driver so its initialization and teardown paths share one inert baseline.
fn uncompress_stream() -> crate::zlib_h::z_stream {
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

// The one-shot driver receives ranges already bound by its ABI adapter.  A
// null zero-capacity destination still gets the local sentinel that zlib uses
// for this special case; a non-null empty slice retains its C cursor value.
fn uncompress2_z_bound(
    mut dest: Option<&mut [crate::stdlib::Bytef]>,
    destLen: &mut crate::stdlib::z_size_t,
    source: Option<&[crate::stdlib::Bytef]>,
    sourceLen: &mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let mut stream = uncompress_stream();
    let mut err: ::core::ffi::c_int = 0;
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut len: crate::stdlib::z_size_t = 0;
    let mut left: crate::stdlib::z_size_t = 0;
    // zlib requires a non-null output cursor even when no output space is
    // available. Keep a real byte sentinel alive for that zero-length case
    // instead of manufacturing a byte pointer from an unrelated stream field.
    let mut empty_output = [0 as crate::stdlib::Bytef; 1];
    if !uncompress_buffers_valid(source.is_none(), *sourceLen, dest.is_none(), *destLen) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    len = *sourceLen;
    left = *destLen;
    let source = source.unwrap_or(&[]);
    let dest = dest.as_deref_mut().unwrap_or(&mut empty_output[..]);
    let mut source_offset = 0usize;
    let mut dest_offset = 0usize;
    stream.next_in = source.as_ptr() as *mut crate::stdlib::Bytef;
    stream.avail_in = 0 as crate::stdlib::uInt;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = crate::src::inflate::inflateInit_(
        Some(&mut stream),
        Some(&crate::zlib_h::ZLIB_VERSION[0]),
        ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
    );
    if err != crate::zlib_h::Z_OK {
        return err;
    }
    stream.next_out = dest.as_mut_ptr();
    stream.avail_out = 0 as crate::stdlib::uInt;
    loop {
        if stream.avail_out == 0 as crate::stdlib::uInt {
            (stream.avail_out, left) = uncompress_chunk(left, max);
        }
        if stream.avail_in == 0 as crate::stdlib::uInt {
            (stream.avail_in, len) = uncompress_chunk(len, max);
        }
        let input_end = source_offset + stream.avail_in as usize;
        let output_end = dest_offset + stream.avail_out as usize;
        let before_in = stream.avail_in;
        let before_out = stream.avail_out;
        err = crate::src::inflate::inflate_stream(
            &mut stream,
            crate::zlib_h::Z_NO_FLUSH,
            Some(&source[source_offset..input_end]),
            &mut dest[dest_offset..output_end],
            None,
        );
        source_offset += before_in.wrapping_sub(stream.avail_in) as usize;
        dest_offset += before_out.wrapping_sub(stream.avail_out) as usize;
        debug_assert!(input_end >= source_offset);
        if err != crate::zlib_h::Z_OK {
            break;
        }
    }
    len = len.wrapping_add(stream.avail_in as crate::stdlib::z_size_t);
    left = left.wrapping_add(stream.avail_out as crate::stdlib::z_size_t);
    *sourceLen = (*sourceLen).wrapping_sub(len);
    *destLen = (*destLen).wrapping_sub(left);
    // This one-shot stream was initialized with zlib's default callbacks,
    // so its teardown can stay reference-bound instead of re-entering the
    // raw public `inflateEnd()` adapter.
    crate::src::inflate::inflate_end_default_bound(&mut stream);
    uncompress_result(err, len)
}
#[export_name = "uncompress2_z"]

pub unsafe extern "C" fn uncompress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if destLen.is_null() || sourceLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // SAFETY: the foreign caller supplied both required length pointers and
    // the advertised byte ranges. The named driver owns the decode lifecycle.
    let dest_len = unsafe { &mut *destLen };
    let source_len = unsafe { &mut *sourceLen };
    // Preflight before binding either range so an invalid null/nonzero pair
    // retains zlib's error result without touching another foreign cursor.
    if !uncompress_buffers_valid(source.is_null(), *source_len, dest.is_null(), *dest_len) {
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
    } else if *source_len == 0 {
        Some(&[] as &[crate::stdlib::Bytef])
    } else {
        Some(unsafe { ::core::slice::from_raw_parts(source, *source_len) })
    };
    uncompress2_z_bound(destination, dest_len, source, source_len)
}

fn uncompress2_legacy_bound(
    destination: Option<&mut [crate::stdlib::Bytef]>,
    dest_len: &mut crate::stdlib::uLongf,
    source: Option<&[crate::stdlib::Bytef]>,
    source_len: &mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut got = *dest_len as crate::stdlib::z_size_t;
    let mut used = *source_len as crate::stdlib::z_size_t;
    let ret = uncompress2_z_bound(destination, &mut got, source, &mut used);
    *source_len = used as crate::stdlib::uLong;
    *dest_len = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
}

fn uncompress_z_bound(
    destination: Option<&mut [crate::stdlib::Bytef]>,
    dest_len: &mut crate::stdlib::z_size_t,
    source: Option<&[crate::stdlib::Bytef]>,
    source_len: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let mut used = source_len;
    uncompress2_z_bound(destination, dest_len, source, &mut used)
}

fn uncompress_legacy_bound(
    destination: Option<&mut [crate::stdlib::Bytef]>,
    dest_len: &mut crate::stdlib::uLongf,
    source: Option<&[crate::stdlib::Bytef]>,
    source_len: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut got = *dest_len as crate::stdlib::z_size_t;
    let ret = uncompress_z_bound(
        destination,
        &mut got,
        source,
        source_len as crate::stdlib::z_size_t,
    );
    *dest_len = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    ret
}

#[export_name = "uncompress2"]
pub unsafe extern "C" fn uncompress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if destLen.is_null() || sourceLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // Bind the legacy length outputs and byte ranges once, then dispatch to
    // the named safe one-shot driver.
    let dest_len = unsafe { &mut *destLen };
    let source_len = unsafe { &mut *sourceLen };
    if !uncompress_buffers_valid(
        source.is_null(),
        *source_len as crate::stdlib::z_size_t,
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
    } else if *source_len == 0 {
        Some(&[] as &[crate::stdlib::Bytef])
    } else {
        Some(unsafe {
            ::core::slice::from_raw_parts(source, *source_len as crate::stdlib::z_size_t)
        })
    };
    uncompress2_legacy_bound(destination, dest_len, source, source_len)
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
    // Bind the caller ranges before the named safe one-shot dispatcher.
    let dest_len = unsafe { &mut *destLen };
    if !uncompress_buffers_valid(source.is_null(), sourceLen, dest.is_null(), *dest_len) {
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
    uncompress_z_bound(destination, dest_len, source, sourceLen)
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
    // Bind the legacy output and byte ranges before the safe driver.
    let dest_len = unsafe { &mut *destLen };
    if !uncompress_buffers_valid(
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
        Some(unsafe { ::core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t) })
    };
    uncompress_legacy_bound(destination, dest_len, source, sourceLen)
}
