pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::deflate;
pub use crate::src::deflate::deflateEnd;
pub use crate::src::deflate::deflateInit2_;
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

// The one-shot APIs feed z_stream in uInt-sized pieces.  Keep this cursor
// pointer-free so a later slice-backed stream facade can retain the exact
// chunking and progress accounting without carrying ABI pointers into its
// core.
pub(crate) struct OneShotCursor {
    remaining: crate::stdlib::z_size_t,
}

impl OneShotCursor {
    pub(crate) fn new(remaining: crate::stdlib::z_size_t) -> Self {
        Self { remaining }
    }

    pub(crate) fn remaining(&self) -> crate::stdlib::z_size_t {
        self.remaining
    }

    pub(crate) fn next_chunk(&mut self, max: crate::stdlib::uInt) -> crate::stdlib::uInt {
        let chunk = if self.remaining > max as crate::stdlib::z_size_t {
            max
        } else {
            self.remaining as crate::stdlib::uInt
        };
        self.remaining = self
            .remaining
            .wrapping_sub(chunk as crate::stdlib::z_size_t);
        chunk
    }
}

// The one-shot API owns all of its byte-range accounting.  Keep that
// accounting separate from the temporary ABI stream: a later deflate owner
// can consume this plan without reconstructing progress from raw cursors.
struct OneShotDeflatePlan {
    input: OneShotCursor,
    output: OneShotCursor,
    output_capacity: crate::stdlib::z_size_t,
}

impl OneShotDeflatePlan {
    fn new(input_len: usize, output_capacity: crate::stdlib::z_size_t) -> Self {
        Self {
            input: OneShotCursor::new(input_len),
            output: OneShotCursor::new(output_capacity),
            output_capacity,
        }
    }

    fn flush(&self) -> ::core::ffi::c_int {
        if self.input.remaining() == 0 {
            crate::zlib_h::Z_FINISH
        } else {
            crate::zlib_h::Z_NO_FLUSH
        }
    }

    fn produced(&self, remaining_output: crate::stdlib::uInt) -> crate::stdlib::z_size_t {
        self.output_capacity.wrapping_sub(
            self.output
                .remaining()
                .wrapping_add(remaining_output as crate::stdlib::z_size_t),
        )
    }
}

fn compress2_z(
    dest: &mut [crate::stdlib::Bytef],
    dest_len: &mut crate::stdlib::z_size_t,
    source: &[crate::stdlib::Bytef],
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    let mut err: ::core::ffi::c_int = 0;
    let max: crate::stdlib::uInt = -1 as ::core::ffi::c_int as crate::stdlib::uInt;
    let mut plan = OneShotDeflatePlan::new(source.len(), *dest_len);
    *dest_len = 0 as crate::stdlib::z_size_t;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    // The stream itself is an ABI mirror used only while this slice-based
    // one-shot loop is active.
    unsafe {
        err = crate::src::deflate::deflateInit2_(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
            level,
            crate::zlib_h::Z_DEFLATED,
            crate::stdlib::MAX_WBITS,
            crate::zutil_h::DEF_MEM_LEVEL,
            crate::zlib_h::Z_DEFAULT_STRATEGY,
            crate::zlib_h::ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
        );
    }
    if err != crate::zlib_h::Z_OK {
        return err;
    }
    stream.next_out = dest.as_mut_ptr();
    stream.avail_out = 0 as crate::stdlib::uInt;
    stream.next_in = source.as_ptr().cast_mut();
    stream.avail_in = 0 as crate::stdlib::uInt;
    loop {
        if stream.avail_out == 0 as crate::stdlib::uInt {
            stream.avail_out = plan.output.next_chunk(max);
        }
        if stream.avail_in == 0 as crate::stdlib::uInt {
            stream.avail_in = plan.input.next_chunk(max);
        }
        unsafe {
            err = crate::src::deflate::deflate(
                &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
                plan.flush(),
            );
        }
        if err != crate::zlib_h::Z_OK {
            break;
        }
    }
    *dest_len = plan.produced(stream.avail_out);
    unsafe {
        crate::src::deflate::deflateEnd(
            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
        );
    }
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
    if sourceLen != 0 && source.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let capacity = *destLen;
    if capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen)
    };
    let dest = if capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, capacity)
    };
    compress2_z(dest, &mut *destLen, source, level)
}
fn compress2(
    dest: &mut [crate::stdlib::Bytef],
    dest_len: &mut crate::stdlib::uLongf,
    source: &[crate::stdlib::Bytef],
    mut level: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut got: crate::stdlib::z_size_t = *dest_len as crate::stdlib::z_size_t;
    ret = compress2_z(dest, &mut got, source, level);
    *dest_len = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    return ret;
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
    let capacity = *destLen as crate::stdlib::z_size_t;
    if sourceLen != 0 && source.is_null() || capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t)
    };
    let dest = if capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, capacity)
    };
    compress2(dest, &mut *destLen, source, level)
}
fn compress_z(
    dest: &mut [crate::stdlib::Bytef],
    dest_len: &mut crate::stdlib::z_size_t,
    source: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    return compress2_z(dest, dest_len, source, crate::zlib_h::Z_DEFAULT_COMPRESSION);
}
#[export_name = "compress_z"]

pub unsafe extern "C" fn compress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if sourceLen != 0 && source.is_null() || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let capacity = *destLen;
    if capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen)
    };
    let dest = if capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, capacity)
    };
    compress_z(dest, &mut *destLen, source)
}
fn compress(
    dest: &mut [crate::stdlib::Bytef],
    dest_len: &mut crate::stdlib::uLongf,
    source: &[crate::stdlib::Bytef],
) -> ::core::ffi::c_int {
    return compress2(dest, dest_len, source, crate::zlib_h::Z_DEFAULT_COMPRESSION);
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
    let capacity = *destLen as crate::stdlib::z_size_t;
    if sourceLen != 0 && source.is_null() || capacity != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        ::core::slice::from_raw_parts(source, sourceLen as crate::stdlib::z_size_t)
    };
    let dest = if capacity == 0 {
        &mut []
    } else {
        ::core::slice::from_raw_parts_mut(dest, capacity)
    };
    compress(dest, &mut *destLen, source)
}
pub fn compressBound_z(mut sourceLen: crate::stdlib::z_size_t) -> crate::stdlib::z_size_t {
    let mut bound: crate::stdlib::z_size_t = sourceLen
        .wrapping_add(sourceLen >> 12 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 14 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as crate::stdlib::z_size_t);
    return if bound < sourceLen {
        -1 as ::core::ffi::c_int as crate::stdlib::z_size_t
    } else {
        bound
    };
}
#[export_name = "compressBound_z"]

pub unsafe extern "C" fn compressBound_z_ffi(
    mut sourceLen: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    compressBound_z(sourceLen)
}
pub fn compressBound(mut sourceLen: crate::stdlib::uLong) -> crate::stdlib::uLong {
    let mut bound: crate::stdlib::z_size_t = compressBound_z(sourceLen as crate::stdlib::z_size_t);
    return if bound != bound {
        -1 as ::core::ffi::c_int as crate::stdlib::uLong
    } else {
        bound as crate::stdlib::uLong
    };
}
#[export_name = "compressBound"]

pub unsafe extern "C" fn compressBound_ffi(
    mut sourceLen: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    compressBound(sourceLen)
}
