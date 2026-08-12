pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::inflate::inflate_mode;
pub use crate::src::inflate::inflate_state;
pub use crate::src::inflate::BAD;
pub use crate::src::inflate::CHECK;
pub use crate::src::inflate::CODELENS;
pub use crate::src::inflate::COMMENT;
pub use crate::src::inflate::COPY_;
pub use crate::src::inflate::COPY_1;
pub use crate::src::inflate::DICT;
pub use crate::src::inflate::DICTID;
pub use crate::src::inflate::DIST;
pub use crate::src::inflate::DISTEXT;
pub use crate::src::inflate::DONE;
pub use crate::src::inflate::EXLEN;
pub use crate::src::inflate::EXTRA;
pub use crate::src::inflate::FLAGS;
pub use crate::src::inflate::HCRC;
pub use crate::src::inflate::HEAD;
pub use crate::src::inflate::LEN;
pub use crate::src::inflate::LENEXT;
pub use crate::src::inflate::LENGTH;
pub use crate::src::inflate::LENLENS;
pub use crate::src::inflate::LEN_;
pub use crate::src::inflate::LIT;
pub use crate::src::inflate::MATCH;
pub use crate::src::inflate::MEM;
pub use crate::src::inflate::NAME;
pub use crate::src::inflate::OS;
pub use crate::src::inflate::STORED;
pub use crate::src::inflate::SYNC;
pub use crate::src::inflate::TABLE;
pub use crate::src::inflate::TIME;
pub use crate::src::inflate::TYPE;
pub use crate::src::inflate::TYPEDO;
pub use crate::src::inftrees::code;
pub use crate::src::inftrees::codetype;
pub use crate::src::inftrees::inflate_fixed;
pub use crate::src::inftrees::inflate_table;
pub use crate::src::inftrees::CODES;
pub use crate::src::inftrees::DISTS;
pub use crate::src::inftrees::LENS;

pub use crate::src::deflate::internal_state;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gz_header;
pub use crate::zlib_h::gz_header_s;
pub use crate::zlib_h::gz_headerp;
pub use crate::zlib_h::in_func;
pub use crate::zlib_h::out_func;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
pub use crate::zlib_h::ZLIB_VERSION;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DATA_ERROR;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_NULL;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_STREAM_END;
pub use crate::zlib_h::Z_STREAM_ERROR;
pub use crate::zlib_h::Z_VERSION_ERROR;
pub fn inflateBackInit_(
    strm: &mut crate::zlib_h::z_stream,
    windowBits: ::core::ffi::c_int,
    window: &mut [crate::stdlib::Bytef],
    version_first: ::core::ffi::c_char,
    stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if version_first as ::core::ffi::c_int
        != crate::zlib_h::ZLIB_VERSION[0 as usize] as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    if windowBits < 8 as ::core::ffi::c_int || windowBits > 15 as ::core::ffi::c_int {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let window_size = 1usize << windowBits;
    if window.len() < window_size {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::zlib_h::clear_stream_message(strm);
    if strm.zalloc.is_none() {
        strm.zalloc = Some(crate::zlib_h::default_stream_allocator());
        strm.opaque = crate::zlib_h::Opaque::default();
    }
    if strm.zfree.is_none() {
        strm.zfree = Some(crate::zlib_h::default_stream_allocator());
    }
    let mut state = crate::src::inflate::inflate_state::default();
    state.dmax = 32768 as ::core::ffi::c_uint;
    state.wbits = windowBits as crate::stdlib::uInt as ::core::ffi::c_uint;
    state.wsize = (1 as ::core::ffi::c_uint) << windowBits;
    let mut owned_window = Vec::new();
    if owned_window
        .try_reserve_exact(state.wsize as usize)
        .is_err()
    {
        return crate::zlib_h::Z_MEM_ERROR;
    }
    owned_window.resize(state.wsize as usize, 0);
    state.window = Some(owned_window);
    state.sane = 1 as ::core::ffi::c_int;
    strm.set_inflate_state(state);
    return crate::zlib_h::Z_OK;
}
#[export_name = "inflateBackInit_"]

pub unsafe extern "C" fn inflateBackInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(version) = (unsafe { version.as_ref() }) else {
        return crate::zlib_h::Z_VERSION_ERROR;
    };
    if *version as ::core::ffi::c_int
        != crate::zlib_h::ZLIB_VERSION[0 as usize] as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_VERSION_ERROR;
    }
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if window.is_null()
        || windowBits < 8 as ::core::ffi::c_int
        || windowBits > 15 as ::core::ffi::c_int
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let window_size = 1usize << windowBits;
    let window = unsafe { ::core::slice::from_raw_parts_mut(window, window_size) };
    inflateBackInit_(strm, windowBits, window, *version, stream_size)
}

/// Result of one safe, bounded inflateBack decoding step.
pub struct InflateBackStep {
    pub code: ::core::ffi::c_int,
    pub needs_input: bool,
    pub flush_output: bool,
}

/// Reset an initialized inflateBack stream before feeding it safe input slices.
pub fn inflateBackStart(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    let Some(state_handle) = strm.inflate_state() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let mut state = state_handle.borrow_mut();
    if state.window.is_none() || state.wsize == 0 {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    crate::zlib_h::clear_stream_message(strm);
    state.mode = crate::src::inflate::TYPE;
    state.last = 0;
    state.whave = 0;
    crate::zlib_h::Z_OK
}

/// Decode one input range into the caller-owned output window.
///
/// `output` is retained across calls.  When `flush_output` is set, its bytes
/// are ready for a safe output sink, after which the caller clears it and
/// invokes this function again.  This makes the callback protocol available
/// without raw cursors or function pointers in implementation code.
pub fn inflateBack(
    strm: &mut crate::zlib_h::z_stream,
    input: &[crate::stdlib::Bytef],
    output: &mut Vec<crate::stdlib::Bytef>,
) -> InflateBackStep {
    let Some(state_handle) = strm.inflate_state() else {
        return InflateBackStep {
            code: crate::zlib_h::Z_STREAM_ERROR,
            needs_input: false,
            flush_output: false,
        };
    };
    let window_size = {
        let state = state_handle.borrow();
        if state.window.is_none() || state.wsize == 0 {
            return InflateBackStep {
                code: crate::zlib_h::Z_STREAM_ERROR,
                needs_input: false,
                flush_output: false,
            };
        }
        state.wsize as usize
    };
    if output.len() > window_size {
        return InflateBackStep {
            code: crate::zlib_h::Z_STREAM_ERROR,
            needs_input: false,
            flush_output: false,
        };
    }

    let saved_total_in = strm.total_in;
    let saved_total_out = strm.total_out;
    let saved_adler = strm.adler;
    let saved_data_type = strm.data_type;
    let saved_state_total = state_handle.borrow().total;
    let output_start = output.len();
    output.resize(window_size, 0);
    let ret = crate::src::inflate::inflate(
        strm,
        input,
        &mut output[output_start..],
        crate::zlib_h::Z_NO_FLUSH,
    );
    let written = window_size - strm.avail_out as usize;
    output.truncate(output_start + written);

    // inflateBack exposes only unused input, not the regular streaming
    // accounting side effects of inflate().
    strm.total_in = saved_total_in;
    strm.total_out = saved_total_out;
    strm.adler = saved_adler;
    strm.data_type = saved_data_type;
    state_handle.borrow_mut().total = saved_state_total;

    let terminal = matches!(
        ret,
        crate::zlib_h::Z_STREAM_END
            | crate::zlib_h::Z_DATA_ERROR
            | crate::zlib_h::Z_STREAM_ERROR
            | crate::zlib_h::Z_MEM_ERROR
    );
    InflateBackStep {
        code: if terminal { ret } else { crate::zlib_h::Z_OK },
        needs_input: !terminal && strm.avail_in == 0,
        flush_output: output.len() == window_size || (terminal && !output.is_empty()),
    }
}

#[export_name = "inflateBack"]
pub unsafe extern "C" fn inflateBack_ffi(
    strm: crate::zlib_h::z_streamp,
    in_0: crate::zlib_h::in_func,
    in_desc: *mut ::core::ffi::c_void,
    out: crate::zlib_h::out_func,
    out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if inflateBackStart(strm) != crate::zlib_h::Z_OK {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut input_cursor = strm.next_in;
    let mut input_len = if input_cursor.0.is_some() {
        strm.avail_in as usize
    } else {
        0
    };
    let mut input = if input_len == 0 {
        Vec::new()
    } else {
        unsafe {
            ::core::slice::from_raw_parts(crate::input_pointer!(input_cursor), input_len).to_vec()
        }
    };
    let window_size = strm
        .inflate_state()
        .expect("inflateBackStart verified the state")
        .borrow()
        .wsize as usize;
    let mut output = Vec::with_capacity(window_size);

    loop {
        let step = inflateBack(strm, &input, &mut output);
        let consumed = input_len.saturating_sub(strm.avail_in as usize);
        if step.flush_output {
            let output_failed = match out {
                Some(callback) => unsafe {
                    callback(
                        out_desc,
                        output.as_mut_ptr(),
                        output.len() as ::core::ffi::c_uint,
                    ) != 0
                },
                None => true,
            };
            if output_failed && step.code == crate::zlib_h::Z_STREAM_END {
                strm.next_in = input_cursor.advance(consumed);
                return crate::zlib_h::Z_BUF_ERROR;
            }
            if output_failed && step.code == crate::zlib_h::Z_OK {
                strm.next_in = input_cursor.advance(consumed);
                return crate::zlib_h::Z_BUF_ERROR;
            }
            output.clear();
        }
        if step.code != crate::zlib_h::Z_OK {
            strm.next_in = input_cursor.advance(consumed);
            return step.code;
        }
        if !step.needs_input {
            strm.next_in = input_cursor.advance(consumed);
            return crate::zlib_h::Z_STREAM_ERROR;
        }

        let Some(callback) = in_0 else {
            strm.next_in = crate::zlib_h::InputBuffer::default();
            strm.avail_in = 0;
            return crate::zlib_h::Z_BUF_ERROR;
        };
        let mut next = ::core::ptr::null_mut();
        let have = unsafe { callback(in_desc, &mut next) } as usize;
        if have == 0 || next.is_null() {
            strm.next_in = crate::zlib_h::InputBuffer::default();
            strm.avail_in = 0;
            return crate::zlib_h::Z_BUF_ERROR;
        }
        input_cursor = crate::input_cursor!(next);
        input_len = have;
        input = unsafe { ::core::slice::from_raw_parts(next, have).to_vec() };
    }
}

pub fn inflateBackEnd(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    if strm.inflate_state().is_none() || strm.zfree.is_none() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    strm.state = None;
    crate::zlib_h::Z_OK
}

#[export_name = "inflateBackEnd"]
pub unsafe extern "C" fn inflateBackEnd_ffi(strm: crate::zlib_h::z_streamp) -> ::core::ffi::c_int {
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    inflateBackEnd(strm)
}
