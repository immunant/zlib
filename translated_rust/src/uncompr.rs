pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflateEnd_ffi;
pub use crate::src::inflate::inflate_ffi;
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

fn uncompress_chunk(remaining: &mut crate::stdlib::z_size_t) -> crate::stdlib::uInt {
    let max = crate::stdlib::uInt::MAX as crate::stdlib::z_size_t;
    let chunk = if *remaining > max {
        crate::stdlib::uInt::MAX
    } else {
        *remaining as crate::stdlib::uInt
    };
    *remaining = (*remaining).wrapping_sub(chunk as crate::stdlib::z_size_t);
    chunk
}

fn uncompress2_final_status(
    err: ::core::ffi::c_int,
    remaining_input: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if err == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_OK
    } else if err == crate::zlib_h::Z_NEED_DICT {
        crate::zlib_h::Z_DATA_ERROR
    } else if err == crate::zlib_h::Z_BUF_ERROR && remaining_input == 0 as crate::stdlib::z_size_t {
        crate::zlib_h::Z_DATA_ERROR
    } else {
        err
    }
}

struct Uncompress2ArgPlan {
    input_left: crate::stdlib::z_size_t,
    output_left: crate::stdlib::z_size_t,
    use_reserved_out: bool,
}

fn uncompress2_arg_plan(
    source_len: Option<crate::stdlib::z_size_t>,
    source_is_null: bool,
    dest_len: Option<crate::stdlib::z_size_t>,
    dest_is_null: bool,
) -> Option<Uncompress2ArgPlan> {
    let input_left = source_len?;
    let output_left = dest_len?;
    if input_left > 0 as crate::stdlib::z_size_t && source_is_null
        || output_left > 0 as crate::stdlib::z_size_t && dest_is_null
    {
        None
    } else {
        Some(Uncompress2ArgPlan {
            input_left,
            output_left,
            use_reserved_out: output_left == 0 as crate::stdlib::z_size_t && dest_is_null,
        })
    }
}

struct Uncompress2Account {
    source_used: crate::stdlib::z_size_t,
    dest_produced: crate::stdlib::z_size_t,
    remaining_input: crate::stdlib::z_size_t,
}

fn uncompress2_account(
    source_capacity: crate::stdlib::z_size_t,
    dest_capacity: crate::stdlib::z_size_t,
    unissued_input: crate::stdlib::z_size_t,
    unissued_output: crate::stdlib::z_size_t,
    avail_in: crate::stdlib::uInt,
    avail_out: crate::stdlib::uInt,
) -> Uncompress2Account {
    let remaining_input = unissued_input.wrapping_add(avail_in as crate::stdlib::z_size_t);
    let remaining_output = unissued_output.wrapping_add(avail_out as crate::stdlib::z_size_t);
    Uncompress2Account {
        source_used: source_capacity.wrapping_sub(remaining_input),
        dest_produced: dest_capacity.wrapping_sub(remaining_output),
        remaining_input,
    }
}

macro_rules! uncompress2_z_body {
    ($dest:expr, $destLen:expr, $source:expr, $sourceLen:expr) => {{
        let mut dest = $dest;
        let dest_len = $destLen;
        let source = $source;
        let source_len = $sourceLen;
        let source_capacity = if source_len.is_null() {
            None
        } else {
            Some(*source_len)
        };
        let dest_capacity = if dest_len.is_null() {
            None
        } else {
            Some(*dest_len)
        };
        match uncompress2_arg_plan(
            source_capacity,
            source.is_null(),
            dest_capacity,
            dest.is_null(),
        ) {
            None => crate::zlib_h::Z_STREAM_ERROR,
            Some(plan) => {
                let mut stream: crate::zlib_h::z_stream = crate::zlib_h::z_stream_s {
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
                let mut len = plan.input_left;
                let mut left = plan.output_left;
                if plan.use_reserved_out {
                    dest = &raw mut stream.reserved as *mut crate::stdlib::Bytef;
                }
                stream.next_in = source as *mut crate::stdlib::Bytef;
                stream.avail_in = 0 as crate::stdlib::uInt;
                stream.zalloc = None;
                stream.zfree = None;
                stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
                let mut err = crate::src::inflate::inflateInit__ffi(
                    &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
                    crate::zlib_h::ZLIB_VERSION.as_ptr(),
                    ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int,
                );
                if err == crate::zlib_h::Z_OK {
                    stream.next_out = dest;
                    stream.avail_out = 0 as crate::stdlib::uInt;
                    loop {
                        if stream.avail_out == 0 as crate::stdlib::uInt {
                            stream.avail_out = uncompress_chunk(&mut left);
                        }
                        if stream.avail_in == 0 as crate::stdlib::uInt {
                            stream.avail_in = uncompress_chunk(&mut len);
                        }
                        err = crate::src::inflate::inflate_ffi(
                            &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
                            crate::zlib_h::Z_NO_FLUSH,
                        );
                        if !(err == crate::zlib_h::Z_OK) {
                            break;
                        }
                    }
                    let accounting = uncompress2_account(
                        plan.input_left,
                        plan.output_left,
                        len,
                        left,
                        stream.avail_in,
                        stream.avail_out,
                    );
                    *source_len = accounting.source_used;
                    *dest_len = accounting.dest_produced;
                    crate::src::inflate::inflateEnd_ffi(
                        &raw mut stream as *mut _ as *mut crate::zlib_h::z_stream_s,
                    );
                    uncompress2_final_status(err, accounting.remaining_input)
                } else {
                    err
                }
            }
        }
    }};
}

#[export_name = "uncompress2_z"]
pub unsafe extern "C" fn uncompress2_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    uncompress2_z_body!(dest, destLen, source, sourceLen)
}
#[export_name = "uncompress2"]
pub unsafe extern "C" fn uncompress2_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: *mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut got: crate::stdlib::z_size_t = *destLen as crate::stdlib::z_size_t;
    let mut used: crate::stdlib::z_size_t = *sourceLen as crate::stdlib::z_size_t;
    ret = uncompress2_z_body!(dest, &raw mut got, source, &raw mut used);
    *sourceLen = used as crate::stdlib::uLong;
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    return ret;
}
#[export_name = "uncompress_z"]
pub unsafe extern "C" fn uncompress_z_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::z_size_t,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let mut used: crate::stdlib::z_size_t = sourceLen;
    return uncompress2_z_body!(dest, destLen, source, &raw mut used);
}
#[export_name = "uncompress"]
pub unsafe extern "C" fn uncompress_ffi(
    mut dest: *mut crate::stdlib::Bytef,
    mut destLen: *mut crate::stdlib::uLongf,
    mut source: *const crate::stdlib::Bytef,
    mut sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let mut used: crate::stdlib::uLong = sourceLen;
    let mut ret: ::core::ffi::c_int = 0;
    let mut got: crate::stdlib::z_size_t = *destLen as crate::stdlib::z_size_t;
    let mut used_z: crate::stdlib::z_size_t = used as crate::stdlib::z_size_t;
    ret = uncompress2_z_body!(dest, &raw mut got, source, &raw mut used_z);
    used = used_z as crate::stdlib::uLong;
    *destLen = got as crate::stdlib::uLong as crate::stdlib::uLongf;
    return ret;
}
