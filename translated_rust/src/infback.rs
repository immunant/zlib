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

struct InflateBackStateConfig {
    dmax: ::core::ffi::c_uint,
    wbits: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
}

// Keep the public initializer's validation order independent of its raw
// stream and window bindings.  In particular, a bad version must win over
// every other error, as it does in zlib.
fn inflate_back_init_config(
    version_first: Option<::core::ffi::c_char>,
    stream_size: ::core::ffi::c_int,
    has_stream: bool,
    has_window: bool,
    window_bits: ::core::ffi::c_int,
) -> Result<InflateBackStateConfig, ::core::ffi::c_int> {
    if version_first != Some(crate::zlib_h::ZLIB_VERSION[0])
        || stream_size != ::core::mem::size_of::<crate::zlib_h::z_stream>() as ::core::ffi::c_int
    {
        return Err(crate::zlib_h::Z_VERSION_ERROR);
    }
    if !has_stream || !has_window || !(8..=15).contains(&window_bits) {
        return Err(crate::zlib_h::Z_STREAM_ERROR);
    }
    Ok(inflate_back_state_config(window_bits))
}

// Keep the callback-window shape check value-only so both the ABI adapter and
// the named decoder agree on the one valid caller-window layout.  In
// particular, do not derive a slice length from an unchecked `wbits` value at
// the ABI boundary.
fn inflate_back_callback_window_len(
    wbits: ::core::ffi::c_uint,
    wsize: ::core::ffi::c_uint,
) -> Option<usize> {
    if !(8..=15).contains(&wbits) {
        return None;
    }
    let len = 1usize.checked_shl(wbits)?;
    (len == wsize as usize).then_some(len)
}

fn inflate_back_state_config(window_bits: ::core::ffi::c_int) -> InflateBackStateConfig {
    InflateBackStateConfig {
        dmax: 32768 as ::core::ffi::c_uint,
        wbits: window_bits as crate::stdlib::uInt as ::core::ffi::c_uint,
        wsize: (1 as ::core::ffi::c_uint) << window_bits,
    }
}

// `inflateBack()` starts each decode by resetting only the fields whose
// lifetime is confined to that operation.  Keep that state transition
// reference-based once the FFI entry point has established the state binding.
fn inflate_back_reset(state: &mut crate::src::inflate::inflate_state) {
    state.mode = crate::src::inflate::TYPE;
    state.last = 0;
    state.whave = 0;
}

// Starting an inflateBack decode clears the public diagnostic and resets the
// decoder-owned operation state together.  The caller still owns the raw
// stream/state binding and the callback/window cursors; this helper only
// returns the configured window capacity for that boundary to use.
fn inflate_back_begin_decode(
    stream: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
) -> ::core::ffi::c_uint {
    stream.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    inflate_back_reset(state);
    state.wsize
}

// `inflateBack()` treats a null initial cursor as an empty initial input
// buffer, irrespective of the accompanying count.  The general inflater
// correctly rejects that pair for its public API, so normalize the callback
// API's distinct convention before dispatching to it.
fn inflate_back_normalize_initial_input(stream: &mut crate::zlib_h::z_stream) {
    if stream.next_in.is_null() {
        stream.avail_in = 0;
    }
}

fn inflate_back_pending_output(
    wsize: ::core::ffi::c_uint,
    left: ::core::ffi::c_uint,
) -> Option<::core::ffi::c_uint> {
    (left < wsize).then(|| wsize.wrapping_sub(left))
}

// The final output callback is still owned by the raw callback boundary, but
// its result changes only the decoder's return value.  Keep that decision
// value-only so the boundary does not also have to encode zlib's special
// successful-end-of-stream rule.
fn inflate_back_finish_pending_output(
    ret: ::core::ffi::c_int,
    output_failed: bool,
) -> ::core::ffi::c_int {
    if output_failed && ret == crate::zlib_h::Z_STREAM_END {
        crate::zlib_h::Z_BUF_ERROR
    } else {
        ret
    }
}

// `next_in` remains a raw cursor publication at the callback boundary.  The
// paired available-byte count is ordinary stream bookkeeping, so isolate it
// in a reference-bound helper.
fn inflate_back_publish_available_input(
    strm: &mut crate::zlib_h::z_stream,
    have: ::core::ffi::c_uint,
) {
    strm.avail_in = have as crate::stdlib::uInt;
}

// Once the shared initializer has allocated the state, configuring it for
// inflateBack is ordinary reference-bound setup. Keeping this separate leaves
// the allocation callback and caller-window binding in `inflateBackInit_`'s
// narrow implementation boundary.
fn inflate_back_init_state(
    strm: &mut crate::zlib_h::z_stream,
    state: &mut crate::src::inflate::inflate_state,
    window: &mut ::core::ffi::c_uchar,
    config: InflateBackStateConfig,
) {
    strm.state = state as *mut crate::src::inflate::inflate_state
        as *mut crate::src::deflate::internal_state;
    state.dmax = config.dmax;
    // `inflateInit2_()` initializes a wrapped stream. `inflateBack()` works
    // on raw deflate input, matching the zero-initialized `wrap` field used
    // by zlib's dedicated initializer.
    state.wrap = 0;
    state.wbits = config.wbits;
    state.wsize = config.wsize;
    state.wnext = 0;
    state.whave = 0;
    state.sane = 1;
    // Keep the same reciprocal stream/state relationship as the regular
    // inflater.  `inflateBack()` can then use the established checked binder
    // instead of reopening this raw state pointer itself.
    state.strm = strm as *mut crate::zlib_h::z_stream;
    state.window = window;
    state.mode = crate::src::inflate::TYPE;
}

// This is deliberately the private implementation target for the exported
// initializer below. The ABI wrapper binds its foreign arguments and
// dispatches here; validation, allocation, and state setup remain outside
// the exported entry point.
fn inflateBackInit_(
    strm: Option<&mut crate::zlib_h::z_stream>,
    mut windowBits: ::core::ffi::c_int,
    window: Option<&mut ::core::ffi::c_uchar>,
    version_first: Option<::core::ffi::c_char>,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let config = match inflate_back_init_config(
        version_first,
        stream_size,
        strm.is_some(),
        window.is_some(),
        windowBits,
    ) {
        Ok(config) => config,
        Err(error) => return error,
    };
    let strm = strm.expect("configuration preflight requires a stream");
    let window = window.expect("configuration preflight requires a window");
    // Unlike `inflateInit2_()`, zlib's `inflateBackInit_()` does not reset
    // these public accounting fields. Preserve them while reusing the common
    // allocator and state initialization path.
    let public_fields = (strm.total_in, strm.total_out, strm.data_type, strm.adler);
    let ret =
        crate::src::inflate::inflateInit2_(Some(strm), windowBits, version_first, stream_size);
    if ret != crate::zlib_h::Z_OK {
        return ret;
    }
    let Some(mut bound_state) =
        crate::src::inflate::inflateStateCheck::<crate::src::inflate::inflate_state>(strm, None)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let (strm, state) = bound_state.parts();
    (strm.total_in, strm.total_out, strm.data_type, strm.adler) = public_fields;
    inflate_back_init_state(strm, state, window, config);
    crate::zlib_h::Z_OK
}
#[export_name = "inflateBackInit_"]

pub unsafe extern "C" fn inflateBackInit__ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter only binds optional caller-owned arguments.
    // The named implementation retains validation and all initialization
    // work, including allocation and persistent state setup.
    let strm = unsafe { strm.as_mut() };
    let window = unsafe { window.as_mut() };
    let version_first = unsafe { version.as_ref().copied() };
    inflateBackInit_(strm, windowBits, window, version_first, stream_size)
}
// Keep the callback-facing window separate from the general inflater's
// history window.  `inflateBack()` lends the caller's window to `out`, whereas
// `inflate()` keeps an independently managed history ring.  The ABI adapter
// binds the callback-owned output window once; this core uses that slice only
// for publication and keeps decoder history in Rust-owned storage.
pub(crate) fn inflateBack(
    strm: Option<&mut crate::zlib_h::z_stream>,
    callback_window: Option<&mut [crate::stdlib::Bytef]>,
    input: crate::zlib_h::in_func,
    input_desc: *mut ::core::ffi::c_void,
    output: crate::zlib_h::out_func,
    output_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(mut bound_state) =
        crate::src::inflate::inflateStateCheck::<crate::src::inflate::inflate_state>(strm, None)
    else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let (strm, state) = bound_state.parts();
    let Some(callback_window) = callback_window else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(window_size) = inflate_back_callback_window_len(state.wbits, state.wsize) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if callback_window.len() != window_size {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    inflate_back_begin_decode(strm, state);
    inflate_back_normalize_initial_input(strm);
    let saved_next_out = strm.next_out;
    let saved_avail_out = strm.avail_out;
    let saved_public = (strm.total_in, strm.total_out, strm.data_type, strm.adler);
    let saved_bit_buffer = (state.hold, state.bits);
    if crate::src::inflate::inflate_back_install_history(state, window_size).is_err() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let mut written = 0usize;
    let mut published_full_window = false;
    let mut ret = loop {
        strm.next_out = callback_window[written..].as_mut_ptr();
        strm.avail_out = (window_size - written) as crate::stdlib::uInt;
        let status = crate::src::inflate::inflate(
            strm,
            state,
            crate::zlib_h::Z_NO_FLUSH,
            None,
            &mut callback_window[written..],
            None,
        );
        written = inflate_back_pending_output(window_size as ::core::ffi::c_uint, strm.avail_out)
            .unwrap_or(0) as usize;
        if written == window_size {
            published_full_window = true;
            if output.expect("non-null function pointer")(
                output_desc,
                callback_window.as_mut_ptr(),
                state.wsize,
            ) != 0
            {
                break crate::zlib_h::Z_BUF_ERROR;
            }
            written = 0;
            continue;
        }
        match status {
            crate::zlib_h::Z_STREAM_END
            | crate::zlib_h::Z_DATA_ERROR
            | crate::zlib_h::Z_MEM_ERROR
            | crate::zlib_h::Z_STREAM_ERROR => break status,
            crate::zlib_h::Z_OK | crate::zlib_h::Z_BUF_ERROR if strm.avail_in == 0 => {
                let mut next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                let have = input.expect("non-null function pointer")(input_desc, &raw mut next);
                if have == 0 || next.is_null() {
                    strm.next_in = ::core::ptr::null_mut::<crate::stdlib::Bytef>();
                    strm.avail_in = 0;
                    break crate::zlib_h::Z_BUF_ERROR;
                }
                strm.next_in = next;
                inflate_back_publish_available_input(strm, have);
            }
            crate::zlib_h::Z_OK | crate::zlib_h::Z_BUF_ERROR => break status,
            _ => break status,
        }
    };
    if written != 0 {
        let output_failed = output.expect("non-null function pointer")(
            output_desc,
            callback_window.as_mut_ptr(),
            written as ::core::ffi::c_uint,
        ) != 0;
        ret = inflate_back_finish_pending_output(ret, output_failed);
    }
    state.wnext = 0;
    state.whave = if published_full_window {
        state.wsize
    } else {
        0
    };
    crate::src::inflate::inflate_back_remove_history(state);
    (state.hold, state.bits) = saved_bit_buffer;
    (strm.total_in, strm.total_out, strm.data_type, strm.adler) = saved_public;
    strm.next_out = saved_next_out;
    strm.avail_out = saved_avail_out;
    ret
}

#[export_name = "inflateBack"]

pub unsafe extern "C" fn inflateBack_ffi(
    mut strm: crate::zlib_h::z_streamp,
    mut in_0: crate::zlib_h::in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: crate::zlib_h::out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter binds the foreign stream and its configured
    // callback window once. The initializer guarantees the window's length
    // from `wbits`; the safe implementation retains all decoder, callback,
    // and cursor handling.
    let Some(strm) = (unsafe { strm.as_mut() }) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    // Reuse the shared state binder to validate and snapshot the configured
    // callback window before binding its caller-owned storage.  This keeps
    // the raw `strm.state` conversion in the established inflater boundary.
    let (window, wbits, wsize) = {
        let Some(mut bound_state) = crate::src::inflate::inflateStateCheck::<
            crate::src::inflate::inflate_state,
        >(strm, None) else {
            return crate::zlib_h::Z_STREAM_ERROR;
        };
        let (_bound_strm, state) = bound_state.parts();
        (state.window, state.wbits, state.wsize)
    };
    let callback_window = match inflate_back_callback_window_len(wbits, wsize) {
        Some(len) if !window.is_null() => {
            Some(unsafe { ::core::slice::from_raw_parts_mut(window, len) })
        }
        _ => None,
    };
    inflateBack(Some(strm), callback_window, in_0, in_desc, out, out_desc)
}
// The ABI forwarder only binds the foreign stream reference. Keep validation
// and the post-release transition reference-bound; only the configured C
// deallocator remains an unsafe boundary here.
fn inflate_back_end_can_release(strm: &crate::zlib_h::z_stream) -> bool {
    !strm.state.is_null() && strm.zfree.is_some()
}

fn inflate_back_end_complete(strm: &mut crate::zlib_h::z_stream) -> ::core::ffi::c_int {
    strm.state = ::core::ptr::null_mut::<crate::src::deflate::internal_state>();
    crate::zlib_h::Z_OK
}

pub fn inflateBackEnd(strm: Option<&mut crate::zlib_h::z_stream>) -> ::core::ffi::c_int {
    // As with inflateBack(), the export binds the foreign reference and this
    // implementation owns validation and teardown decisions.
    let Some(strm) = strm else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if !inflate_back_end_can_release(strm) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    // A user deallocator may re-enter unrelated code, so capture all values
    // it receives before crossing that callback boundary. In particular, do
    // not read the stream-owned callback or allocation through `strm` while
    // the callback is active.
    let opaque = strm.opaque;
    let state = strm.state as crate::stdlib::voidpf;
    let zfree = strm.zfree.expect("non-null function pointer");
    Some(zfree).expect("non-null function pointer")(opaque, state);
    inflate_back_end_complete(strm)
}
#[export_name = "inflateBackEnd"]

pub unsafe extern "C" fn inflateBackEnd_ffi(
    mut strm: crate::zlib_h::z_streamp,
) -> ::core::ffi::c_int {
    // SAFETY: this ABI adapter only binds the optional foreign stream
    // reference before the reference-based teardown dispatcher validates it.
    let strm = unsafe { strm.as_mut() };
    inflateBackEnd(strm)
}
