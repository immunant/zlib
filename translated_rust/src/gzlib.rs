pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::stdlib::__O_LARGEFILE;

pub use crate::gzguts_h::gz_state;
pub use crate::gzguts_h::gz_statep;
pub use crate::gzguts_h::COPY;
pub use crate::gzguts_h::GZBUFSIZE;
pub use crate::gzguts_h::GZ_APPEND;
pub use crate::gzguts_h::GZ_NONE;
pub use crate::gzguts_h::GZ_READ;
pub use crate::gzguts_h::GZ_WRITE;
pub use crate::gzguts_h::LOOK;
pub use crate::internal::__INT_MAX__;
pub use crate::limits_h::INT_MAX;
pub use crate::stdlib::fcntl;
pub use crate::stdlib::open;
pub use crate::stdlib::__O_CLOEXEC;
pub use crate::stdlib::F_GETFD;
pub use crate::stdlib::F_GETFL;
pub use crate::stdlib::F_SETFD;
pub use crate::stdlib::F_SETFL;
pub use crate::stdlib::O_APPEND;
pub use crate::stdlib::O_CLOEXEC;
pub use crate::stdlib::O_CREAT;
pub use crate::stdlib::O_EXCL;
pub use crate::stdlib::O_LARGEFILE;
pub use crate::stdlib::O_NONBLOCK;
pub use crate::stdlib::O_RDONLY;
pub use crate::stdlib::O_TRUNC;
pub use crate::stdlib::O_WRONLY;
pub use crate::stdlib::SEEK_CUR;
pub use crate::stdlib::SEEK_END;
pub use crate::stdlib::SEEK_SET;

pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off64_t;
pub use crate::stdlib::off_t;

pub use crate::src::deflate::internal_state;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::z_size_t;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gzFile;
pub use crate::zlib_h::gzFile_s;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::Z_BUF_ERROR;
pub use crate::zlib_h::Z_DEFAULT_COMPRESSION;
pub use crate::zlib_h::Z_DEFAULT_STRATEGY;
pub use crate::zlib_h::Z_FILTERED;
pub use crate::zlib_h::Z_FIXED;
pub use crate::zlib_h::Z_HUFFMAN_ONLY;
pub use crate::zlib_h::Z_MEM_ERROR;
pub use crate::zlib_h::Z_OK;
pub use crate::zlib_h::Z_RLE;

// Keep gzip I/O requests within the unsigned-int sizes used by zlib's stream
// fields and the POSIX read/write adapters.
pub fn gz_stream_chunk(len: crate::stdlib::z_size_t) -> ::core::ffi::c_uint {
    let max = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    if max as crate::stdlib::z_size_t > len {
        len as ::core::ffi::c_uint
    } else {
        max
    }
}

pub fn gz_syscall_chunk(len: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let max = (-1 as ::core::ffi::c_int as ::core::ffi::c_uint >> 2)
        .wrapping_add(1 as ::core::ffi::c_uint);
    if len > max { max } else { len }
}

// Keep byte-count arithmetic out of the raw read/write adapters.  These use
// wrapping operations to retain the translated C behavior for corrupt state.
pub(crate) fn gz_load_request(
    len: ::core::ffi::c_uint,
    have: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    gz_syscall_chunk(len.wrapping_sub(have))
}

pub(crate) fn gz_add_received(
    have: ::core::ffi::c_uint,
    received: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    have.wrapping_add(received)
}

// The read and write adapters own the descriptor, errno, and raw buffer
// pointers.  Keep their common state transitions here, where they can be
// checked without expanding either raw I/O boundary.
pub(crate) fn gz_begin_io(state: &mut crate::gzguts_h::gz_state) {
    state.again = 0;
}

pub(crate) fn gz_io_result(
    state: &mut crate::gzguts_h::gz_state,
    result: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> Result<::core::ffi::c_uint, ::core::ffi::c_int> {
    match gz_syscall_result(result, errno) {
        Ok(count) => Ok(count),
        Err(again) => {
            if again {
                state.again = 1;
            }
            Err(errno)
        }
    }
}

pub(crate) fn gz_load_result(
    state: &mut crate::gzguts_h::gz_state,
    result: ::core::ffi::c_int,
    have: ::core::ffi::c_uint,
    errno: ::core::ffi::c_int,
) -> Result<(), ::core::ffi::c_int> {
    if result < 0 {
        if let Err(errno) = gz_io_result(state, result, errno) {
            if state.again != 0 && have != 0 {
                return Ok(());
            }
            return Err(errno);
        }
    }
    if result == 0 {
        state.eof = 1;
    }
    Ok(())
}

pub(crate) enum GzAvailPlan {
    Done,
    Load {
        buffered: ::core::ffi::c_uint,
        requested: ::core::ffi::c_uint,
    },
}

// Decide whether the raw gzip input adapter needs another read.  Pointer
// compaction and the descriptor call remain in that adapter; this helper owns
// the state checks and count arithmetic.
pub(crate) fn gz_avail_plan(
    state: &crate::gzguts_h::gz_state,
) -> Result<GzAvailPlan, ()> {
    if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
        return Err(());
    }
    if state.eof != 0 {
        return Ok(GzAvailPlan::Done);
    }
    let buffered = state.strm.avail_in as ::core::ffi::c_uint;
    Ok(GzAvailPlan::Load {
        buffered,
        requested: state.size.wrapping_sub(buffered),
    })
}

pub(crate) fn gz_avail_after_load(
    state: &mut crate::gzguts_h::gz_state,
    received: ::core::ffi::c_uint,
) {
    state.strm.avail_in = state.strm.avail_in.wrapping_add(received);
    state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
}

pub(crate) fn gz_set_copy_input(
    state: &mut crate::gzguts_h::gz_state,
    copied: ::core::ffi::c_uint,
) {
    state.x.next = state.out;
    state.x.have = copied;
    state.strm.avail_in = 0;
    state.how = crate::gzguts_h::COPY;
}

// Once gzip input is available, the look adapter either waits for a complete
// signature, starts a gzip member, or retains the input as a transparent
// copy.  Signature access remains at the raw input-buffer adapter; this
// helper only receives the scalar facts needed for classification.
pub(crate) enum GzLookPlan {
    NeedMore,
    Gzip,
    Copy { copied: ::core::ffi::c_uint },
}

pub(crate) fn gz_look_plan(
    available: ::core::ffi::c_uint,
    stalled: bool,
    gzip_header: bool,
) -> GzLookPlan {
    if available == 0 || (stalled && available < 4) {
        GzLookPlan::NeedMore
    } else if gzip_header {
        GzLookPlan::Gzip
    } else {
        GzLookPlan::Copy { copied: available }
    }
}

pub(crate) fn gz_is_gzip_header(header: [::core::ffi::c_uchar; 4]) -> bool {
    header == [31, 139, 8, header[3]] && header[3] < 32
}

pub(crate) fn gz_set_gzip_input(state: &mut crate::gzguts_h::gz_state, junk: bool) {
    state.how = crate::gzguts_h::GZIP;
    state.junk = junk as ::core::ffi::c_int;
    state.direct = 0;
}

pub(crate) fn gz_reset_output_buffer(state: &mut crate::gzguts_h::gz_state) {
    state.strm.avail_out = state.size;
}

pub(crate) fn gz_remaining_after_write(
    available: crate::stdlib::uInt,
    written: ::core::ffi::c_uint,
) -> crate::stdlib::uInt {
    available.wrapping_sub(written)
}

pub(crate) fn gz_produced(
    available_before: ::core::ffi::c_uint,
    available_after: crate::stdlib::uInt,
) -> ::core::ffi::c_uint {
    available_before.wrapping_sub(available_after as ::core::ffi::c_uint)
}

// Account for bytes appended to the gzip input buffer without involving its
// raw buffer pointer.  The caller has already copied exactly `added` bytes.
pub(crate) fn gz_append_input(
    state: &mut crate::gzguts_h::gz_state,
    added: ::core::ffi::c_uint,
) {
    state.strm.avail_in = state.strm.avail_in.wrapping_add(added);
    gz_advance_pos(state, added);
}

// Account for the portion of a stream input request consumed by deflate.
// `avail_in` remains the source of truth for the raw stream adapter.
pub(crate) fn gz_consume_stream_input(
    state: &mut crate::gzguts_h::gz_state,
    requested: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let consumed = gz_produced(requested, state.strm.avail_in);
    gz_advance_pos(state, consumed);
    consumed
}

// Classify a POSIX I/O result without coupling the decision to the raw
// descriptor and buffer adapters.  A non-negative result is a byte count;
// a negative result preserves whether a non-blocking operation stalled.
pub(crate) fn gz_syscall_result(
    result: ::core::ffi::c_int,
    errno: ::core::ffi::c_int,
) -> Result<::core::ffi::c_uint, bool> {
    if result < 0 {
        Err(errno == crate::stdlib::EAGAIN || errno == crate::stdlib::EWOULDBLOCK)
    } else {
        Ok(result as ::core::ffi::c_uint)
    }
}

// gz_comp writes a completed output buffer, or writes while flushing except
// before Z_FINISH reaches the end of the stream.
pub(crate) fn gz_comp_needs_write(
    avail_out: ::core::ffi::c_uint,
    flush: ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> bool {
    avail_out == 0
        || (flush != crate::zlib_h::Z_NO_FLUSH
            && (flush != crate::zlib_h::Z_FINISH || ret == crate::zlib_h::Z_STREAM_END))
}

// Describe a compression-output drain before the raw write adapter touches
// the descriptor or advances its buffer pointer.  Address arithmetic avoids
// making a provenance-dependent pointer subtraction part of that adapter.
pub(crate) struct GzCompOutputPlan {
    pub reset: bool,
}

pub(crate) fn gz_comp_output_pending(
    state: &crate::gzguts_h::gz_state,
) -> ::core::ffi::c_uint {
    state
        .strm
        .next_out
        .addr()
        .wrapping_sub(state.x.next.addr()) as ::core::ffi::c_uint
}

// Keep the request sizing for the two gz_comp write adapters with the
// associated state accounting.  The adapters retain their distinct raw input
// pointers and descriptor calls.
pub(crate) fn gz_comp_direct_write_request(
    state: &crate::gzguts_h::gz_state,
) -> ::core::ffi::c_uint {
    gz_syscall_chunk(state.strm.avail_in)
}

pub(crate) fn gz_comp_output_write_request(
    state: &crate::gzguts_h::gz_state,
) -> ::core::ffi::c_uint {
    gz_syscall_chunk(gz_comp_output_pending(state))
}

pub(crate) fn gz_comp_output_write_progress(
    state: &mut crate::gzguts_h::gz_state,
    written: ::core::ffi::c_uint,
) {
    state.x.next = state.x.next.wrapping_add(written as usize);
}

pub(crate) fn gz_comp_output_plan(
    state: &crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> Option<GzCompOutputPlan> {
    if !gz_comp_needs_write(state.strm.avail_out, flush, ret) {
        return None;
    }
    Some(GzCompOutputPlan {
        reset: state.strm.avail_out == 0,
    })
}

pub(crate) fn gz_comp_reset_output(state: &mut crate::gzguts_h::gz_state) {
    gz_reset_output_buffer(state);
    state.strm.next_out = state.out as *mut crate::stdlib::Bytef;
    state.x.next = state.out;
}

pub(crate) fn gz_comp_should_reset(flush: ::core::ffi::c_int) -> bool {
    flush == crate::zlib_h::Z_FINISH
}

// Select the compression-path state transition before entering either the raw
// direct-write adapter or deflate.  The adapter retains descriptor calls and
// pointer rebasing; this helper owns the state-only decisions.
pub(crate) enum GzCompMode {
    Direct,
    Idle,
    Reset,
    Deflate,
}

pub(crate) fn gz_comp_mode(
    state: &crate::gzguts_h::gz_state,
    flush: ::core::ffi::c_int,
) -> GzCompMode {
    if state.direct != 0 {
        GzCompMode::Direct
    } else if state.reset != 0 {
        if state.strm.avail_in == 0 && flush == crate::zlib_h::Z_NO_FLUSH {
            GzCompMode::Idle
        } else {
            GzCompMode::Reset
        }
    } else {
        GzCompMode::Deflate
    }
}

pub(crate) fn gz_comp_reset_complete(state: &mut crate::gzguts_h::gz_state) {
    state.reset = 0;
}

pub(crate) fn gz_comp_finish(state: &mut crate::gzguts_h::gz_state, flush: ::core::ffi::c_int) {
    if gz_comp_should_reset(flush) {
        state.reset = 1;
    }
}

// Advance only the count portion of a direct write.  The raw adapter advances
// `next_in`, since it is the sole owner of that pointer.
pub(crate) fn gz_direct_write_progress(
    state: &mut crate::gzguts_h::gz_state,
    written: ::core::ffi::c_uint,
) {
    state.strm.avail_in = gz_remaining_after_write(state.strm.avail_in, written);
}

// On a non-blocking write failure, gzip reports only the input consumed so
// far.  Other write failures report no input consumed.
pub(crate) fn gz_write_result(
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
    stalled: bool,
) -> crate::stdlib::z_size_t {
    if stalled {
        requested.wrapping_sub(remaining)
    } else {
        0
    }
}

// Keep the write-path choices and byte accounting independent of the raw
// source and destination buffers.  The callers still own the actual copy and
// stream-pointer rebasing at the FFI boundary.
pub(crate) fn gz_write_needs_init(state: &crate::gzguts_h::gz_state) -> bool {
    state.size == 0
}

// Choose the next state-only step for gz_write().  Initialization and sparse
// seek handling can change the state, so the raw buffer adapter asks again
// after each succeeds before it selects a copy or stream operation.
pub(crate) enum GzWritePlan {
    Empty,
    Initialize,
    Zero,
    Buffered,
    Stream,
}

pub(crate) fn gz_write_plan(
    state: &crate::gzguts_h::gz_state,
    remaining: crate::stdlib::z_size_t,
) -> GzWritePlan {
    if remaining == 0 {
        GzWritePlan::Empty
    } else if gz_write_needs_init(state) {
        GzWritePlan::Initialize
    } else if state.skip != 0 {
        GzWritePlan::Zero
    } else if remaining < state.size as crate::stdlib::z_size_t {
        GzWritePlan::Buffered
    } else {
        GzWritePlan::Stream
    }
}

pub(crate) struct GzBufferedCopyPlan {
    pub offset: ::core::ffi::c_uint,
    pub len: ::core::ffi::c_uint,
}

// The gzip input buffer is contiguous.  Keep its cursor setup and byte count
// out of the copy adapter, which is the only write-path code that needs the
// raw source and destination pointers.
pub(crate) fn gz_buffered_input_len(state: &mut crate::gzguts_h::gz_state) -> ::core::ffi::c_uint {
    if state.strm.avail_in == 0 {
        state.strm.next_in = state.in_0 as *mut crate::stdlib::Bytef;
    }
    state
        .strm
        .next_in
        .addr()
        .wrapping_add(state.strm.avail_in as usize)
        .wrapping_sub(state.in_0.addr()) as ::core::ffi::c_uint
}

// Select the destination offset and copy size before the raw copy adapter
// touches either source or destination memory.
pub(crate) fn gz_buffered_copy_plan(
    state: &mut crate::gzguts_h::gz_state,
    remaining: crate::stdlib::z_size_t,
) -> GzBufferedCopyPlan {
    let offset = gz_buffered_input_len(state);
    GzBufferedCopyPlan {
        offset,
        len: gz_buffer_space(state.size, offset, remaining),
    }
}

pub(crate) fn gz_buffered_copy_progress(
    state: &mut crate::gzguts_h::gz_state,
    remaining: &mut crate::stdlib::z_size_t,
    copied: ::core::ffi::c_uint,
) {
    gz_append_input(state, copied);
    *remaining = remaining.wrapping_sub(copied as crate::stdlib::z_size_t);
}

pub(crate) fn gz_putc_buffered_progress(state: &mut crate::gzguts_h::gz_state) {
    gz_append_input(state, 1);
}

pub(crate) fn gz_stream_write_progress(
    state: &mut crate::gzguts_h::gz_state,
    remaining: &mut crate::stdlib::z_size_t,
    offered: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let consumed = gz_consume_stream_input(state, offered);
    *remaining = remaining.wrapping_sub(consumed as crate::stdlib::z_size_t);
    consumed
}

pub(crate) fn gz_write_error_result(
    state: &crate::gzguts_h::gz_state,
    requested: crate::stdlib::z_size_t,
    remaining: crate::stdlib::z_size_t,
) -> crate::stdlib::z_size_t {
    gz_write_result(requested, remaining, state.again != 0)
}

pub(crate) fn gz_zero_progress(
    state: &mut crate::gzguts_h::gz_state,
    offered: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let consumed = gz_produced(offered, state.strm.avail_in);
    gz_advance_pos(state, consumed);
    state.skip -= consumed as crate::stdlib::off64_t;
    consumed
}

// Choose the amount a bulk gzip operation may handle in one stream request.
// `available` is only a bound when data is already buffered.
pub(crate) fn gz_buffered_chunk(
    len: crate::stdlib::z_size_t,
    available: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let chunk = gz_stream_chunk(len);
    if chunk > available { available } else { chunk }
}

// Select the next read operation without touching either the caller's buffer
// or gzip's raw output buffer.  The read adapter keeps those pointer-based
// operations at its FFI boundary.
pub(crate) enum GzReadPlan {
    Buffered(::core::ffi::c_uint),
    End,
    Fetch,
    Copy(::core::ffi::c_uint),
    Decompress(::core::ffi::c_uint),
}

pub(crate) fn gz_read_plan(
    state: &crate::gzguts_h::gz_state,
    remaining: crate::stdlib::z_size_t,
) -> GzReadPlan {
    let chunk = gz_stream_chunk(remaining);
    if state.x.have != 0 {
        return GzReadPlan::Buffered(gz_buffered_chunk(remaining, state.x.have));
    }
    if state.eof != 0 && state.strm.avail_in == 0 {
        return GzReadPlan::End;
    }
    if state.how == crate::gzguts_h::LOOK || chunk < state.size.wrapping_shl(1) {
        return GzReadPlan::Fetch;
    }
    if state.how == crate::gzguts_h::COPY {
        GzReadPlan::Copy(chunk)
    } else {
        GzReadPlan::Decompress(chunk)
    }
}

// A buffered read already advances position in gz_consume().  Direct reads
// need the same accounting, but must not update it twice.
pub(crate) fn gz_read_progress(
    state: &mut crate::gzguts_h::gz_state,
    remaining: &mut crate::stdlib::z_size_t,
    received: &mut crate::stdlib::z_size_t,
    count: ::core::ffi::c_uint,
    was_buffered: bool,
) {
    *remaining = remaining.wrapping_sub(count as crate::stdlib::z_size_t);
    *received = received.wrapping_add(count as crate::stdlib::z_size_t);
    if !was_buffered {
        gz_advance_pos(state, count);
    }
}

pub(crate) fn gz_read_mark_past(state: &mut crate::gzguts_h::gz_state, remaining: crate::stdlib::z_size_t) {
    if remaining != 0 && state.eof != 0 {
        state.past = 1;
    }
}

// Return how much input fits in the gzip input buffer.  Valid gzip state has
// `buffered <= size`; wrapping preserves the translated C arithmetic if a
// corrupt state reaches this internal path.
pub(crate) fn gz_buffer_space(
    size: ::core::ffi::c_uint,
    buffered: ::core::ffi::c_uint,
    remaining: crate::stdlib::z_size_t,
) -> ::core::ffi::c_uint {
    let space = size.wrapping_sub(buffered);
    if space as crate::stdlib::z_size_t > remaining {
        remaining as ::core::ffi::c_uint
    } else {
        space
    }
}

// Keep the logical gzip position update independent of the raw buffer
// adapters used by the read and write paths.
pub(crate) fn gz_advance_pos(state: &mut crate::gzguts_h::gz_state, count: crate::stdlib::uInt) {
    state.x.pos += count as crate::stdlib::off64_t;
}

fn gz_reset(state: &mut crate::gzguts_h::gz_state) {
    state.x.have = 0 as ::core::ffi::c_uint;
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0 as ::core::ffi::c_int;
        state.past = 0 as ::core::ffi::c_int;
        state.how = crate::gzguts_h::LOOK;
        state.junk = -1 as ::core::ffi::c_int;
    } else {
        state.reset = 0 as ::core::ffi::c_int;
    }
    state.again = 0 as ::core::ffi::c_int;
    state.skip = 0 as crate::stdlib::off64_t;
    state.x.pos = 0 as crate::stdlib::off64_t;
    state.strm.avail_in = 0 as crate::stdlib::uInt;
}

struct GzOpenMode {
    oflag: ::core::ffi::c_int,
    exclusive: ::core::ffi::c_int,
}

// Keep the constructor's state-only setup separate from allocation and C
// string traversal in gz_open().
fn gz_open_init(state: &mut crate::gzguts_h::gz_state) {
    state.size = 0;
    state.want = crate::gzguts_h::GZBUFSIZE as ::core::ffi::c_uint;
    state.err = crate::zlib_h::Z_OK;
    state.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    state.mode = crate::gzguts_h::GZ_NONE;
    state.level = crate::zlib_h::Z_DEFAULT_COMPRESSION;
    state.strategy = crate::zlib_h::Z_DEFAULT_STRATEGY;
    state.direct = 0;
}

// Apply one mode character without coupling interpretation to the raw mode
// string cursor used by the public FFI constructor.
fn gz_open_mode_byte(
    state: &mut crate::gzguts_h::gz_state,
    options: &mut GzOpenMode,
    mode: ::core::ffi::c_uchar,
) -> bool {
    if mode >= b'0' && mode <= b'9' {
        state.level = (mode - b'0') as ::core::ffi::c_int;
        return true;
    }
    match mode {
        b'r' => state.mode = crate::gzguts_h::GZ_READ,
        b'w' => state.mode = crate::gzguts_h::GZ_WRITE,
        b'a' => state.mode = crate::gzguts_h::GZ_APPEND,
        b'+' => return false,
        b'e' => options.oflag |= crate::stdlib::O_CLOEXEC,
        b'x' => options.exclusive = 1,
        b'f' => state.strategy = crate::zlib_h::Z_FILTERED,
        b'h' => state.strategy = crate::zlib_h::Z_HUFFMAN_ONLY,
        b'R' => state.strategy = crate::zlib_h::Z_RLE,
        b'F' => state.strategy = crate::zlib_h::Z_FIXED,
        b'G' => state.direct = -1,
        b'N' => options.oflag |= crate::stdlib::O_NONBLOCK,
        b'T' => state.direct = 1,
        _ => {}
    }
    true
}

// Validate the parsed mode and apply the read-mode transparent default.
fn gz_open_finish_mode(state: &mut crate::gzguts_h::gz_state) -> bool {
    if state.mode == crate::gzguts_h::GZ_NONE {
        return false;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        if state.direct == 1 {
            return false;
        }
        if state.direct == 0 {
            state.direct = 1;
        }
    } else if state.direct == -1 {
        return false;
    }
    true
}

fn gz_open_flags(
    state: &crate::gzguts_h::gz_state,
    options: &GzOpenMode,
) -> ::core::ffi::c_int {
    options.oflag
        | crate::stdlib::O_LARGEFILE
        | if state.mode == crate::gzguts_h::GZ_READ {
            crate::stdlib::O_RDONLY
        } else {
            crate::stdlib::O_WRONLY
                | crate::stdlib::O_CREAT
                | if options.exclusive != 0 {
                    crate::stdlib::O_EXCL
                } else {
                    0
                }
                | if state.mode == crate::gzguts_h::GZ_WRITE {
                    crate::stdlib::O_TRUNC
                } else {
                    crate::stdlib::O_APPEND
                }
        }
}

unsafe extern "C" fn gz_open(
    mut path: *const ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut state: crate::gzguts_h::gz_statep =
        ::core::ptr::null_mut::<crate::gzguts_h::gz_state>();
    let mut len: crate::stdlib::z_size_t = 0;
    let mut options = GzOpenMode {
        oflag: 0,
        exclusive: 0,
    };
    if path.is_null() || mode.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    state = crate::stdlib::malloc(::core::mem::size_of::<crate::gzguts_h::gz_state>())
        as crate::gzguts_h::gz_statep;
    if state.is_null() {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    let state_ref = &mut *state;
    gz_open_init(state_ref);
    while *mode != 0 {
        if !gz_open_mode_byte(state_ref, &mut options, *mode as ::core::ffi::c_uchar) {
            crate::stdlib::free(state as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
        }
        mode = mode.offset(1);
    }
    if !gz_open_finish_mode(state_ref) {
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    len = crate::stdlib::strlen(path as *const ::core::ffi::c_char) as crate::stdlib::z_size_t;
    state_ref.path = crate::stdlib::malloc(
        (len as crate::__stddef_size_t_h::size_t)
            .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
    ) as *mut ::core::ffi::c_char;
    if state_ref.path.is_null() {
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    crate::stdlib::snprintf(
        state_ref.path,
        (len as crate::__stddef_size_t_h::size_t)
            .wrapping_add(1 as crate::__stddef_size_t_h::size_t),
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        path as *const ::core::ffi::c_char,
    );
    let oflag = gz_open_flags(state_ref, &options);
    if fd == -1 as ::core::ffi::c_int {
        state_ref.fd = crate::stdlib::open(
            path as *const ::core::ffi::c_char,
            oflag,
            0o666 as ::core::ffi::c_int,
        );
    } else {
        if oflag & crate::stdlib::O_NONBLOCK != 0 {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFL,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFL) | crate::stdlib::O_NONBLOCK,
            );
        }
        if oflag & crate::stdlib::O_CLOEXEC != 0 {
            crate::stdlib::fcntl(
                fd,
                crate::stdlib::F_SETFD,
                crate::stdlib::fcntl(fd, crate::stdlib::F_GETFD) | crate::stdlib::O_CLOEXEC,
            );
        }
        state_ref.fd = fd;
    }
    if state_ref.fd == -1 as ::core::ffi::c_int {
        crate::stdlib::free(state_ref.path as *mut ::core::ffi::c_void);
        crate::stdlib::free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    if state_ref.mode == crate::gzguts_h::GZ_APPEND {
        crate::stdlib::lseek64(
            state_ref.fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_END,
        );
        state_ref.mode = crate::gzguts_h::GZ_WRITE;
    }
    if state_ref.mode == crate::gzguts_h::GZ_READ {
        state_ref.start = crate::stdlib::lseek64(
            state_ref.fd,
            0 as crate::stdlib::__off64_t,
            crate::stdlib::SEEK_CUR,
        ) as crate::stdlib::off64_t;
        if state_ref.start == -1 as crate::stdlib::off64_t {
            state_ref.start = 0 as crate::stdlib::off64_t;
        }
    }
    gz_reset(state_ref);
    gz_error(
        state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return state as crate::zlib_h::gzFile;
}
pub unsafe extern "C" fn gzopen(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    return gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    );
}
#[export_name = "gzopen"]

pub unsafe extern "C" fn gzopen_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzopen(path, mode)
}
pub unsafe extern "C" fn gzopen64(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    return gz_open(
        path as *const ::core::ffi::c_void,
        -1 as ::core::ffi::c_int,
        mode,
    );
}
#[export_name = "gzopen64"]

pub unsafe extern "C" fn gzopen64_ffi(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzopen64(path, mode)
}
pub unsafe extern "C" fn gzdopen(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gz: crate::zlib_h::gzFile = ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    if fd == -1 as ::core::ffi::c_int || {
        path = crate::stdlib::malloc(
            (7 as crate::__stddef_size_t_h::size_t).wrapping_add(
                (3 as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
            ),
        ) as *mut ::core::ffi::c_char;
        path.is_null()
    } {
        return ::core::ptr::null_mut::<crate::zlib_h::gzFile_s>();
    }
    crate::stdlib::snprintf(
        path,
        (7 as crate::__stddef_size_t_h::size_t).wrapping_add(
            (3 as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>()),
        ),
        b"<fd:%d>\0".as_ptr() as *const ::core::ffi::c_char,
        fd,
    );
    gz = gz_open(path as *const ::core::ffi::c_void, fd, mode);
    crate::stdlib::free(path as *mut ::core::ffi::c_void);
    return gz;
}
#[export_name = "gzdopen"]

pub unsafe extern "C" fn gzdopen_ffi(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> crate::zlib_h::gzFile {
    gzdopen(fd, mode)
}
pub unsafe extern "C" fn gzbuffer(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    gz_buffer(state, size)
}

fn gz_buffer(
    state: &mut crate::gzguts_h::gz_state,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1 as ::core::ffi::c_int;
    }
    if state.size != 0 as ::core::ffi::c_uint {
        return -1 as ::core::ffi::c_int;
    }
    if (size << 1 as ::core::ffi::c_int) < size {
        return -1 as ::core::ffi::c_int;
    }
    if size < 8 as ::core::ffi::c_uint {
        size = 8 as ::core::ffi::c_uint;
    }
    state.want = size;
    0 as ::core::ffi::c_int
}
#[export_name = "gzbuffer"]

pub unsafe extern "C" fn gzbuffer_ffi(
    mut file: crate::zlib_h::gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    gzbuffer(file, size)
}
pub unsafe extern "C" fn gzrewind(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ
        || state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR
    {
        return -1 as ::core::ffi::c_int;
    }
    if crate::stdlib::lseek64(
        state.fd,
        state.start as crate::stdlib::off64_t,
        crate::stdlib::SEEK_SET,
    ) == -1 as crate::stdlib::__off64_t
    {
        return -1 as ::core::ffi::c_int;
    }
    gz_reset(state);
    gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    return 0 as ::core::ffi::c_int;
}
#[export_name = "gzrewind"]

pub unsafe extern "C" fn gzrewind_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzrewind(file)
}
pub unsafe extern "C" fn gzseek64(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    let mut n: ::core::ffi::c_uint = 0;
    let mut copy_seek = false;
    let mut rewind = false;
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    {
        let state = &mut *(file as crate::gzguts_h::gz_statep);
        if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
            return -1 as crate::stdlib::off64_t;
        }
        if state.err != crate::zlib_h::Z_OK && state.err != crate::zlib_h::Z_BUF_ERROR {
            return -1 as crate::stdlib::off64_t;
        }
        if whence != crate::stdlib::SEEK_SET && whence != crate::stdlib::SEEK_CUR {
            return -1 as crate::stdlib::off64_t;
        }
        if whence == crate::stdlib::SEEK_SET {
            offset -= state.x.pos;
        } else {
            offset += if state.past != 0 {
                0 as crate::stdlib::off64_t
            } else {
                state.skip
            };
            state.skip = 0 as crate::stdlib::off64_t;
        }
        if state.mode == crate::gzguts_h::GZ_READ
            && state.how == crate::gzguts_h::COPY
            && state.x.pos + offset >= 0 as crate::stdlib::off64_t
        {
            if crate::stdlib::lseek64(
                state.fd,
                offset as crate::stdlib::__off64_t - state.x.have as crate::stdlib::__off64_t,
                crate::stdlib::SEEK_CUR,
            ) == -1 as crate::stdlib::__off64_t
            {
                return -1 as crate::stdlib::off64_t;
            }
            state.x.have = 0 as ::core::ffi::c_uint;
            state.eof = 0 as ::core::ffi::c_int;
            state.past = 0 as ::core::ffi::c_int;
            state.skip = 0 as crate::stdlib::off64_t;
            copy_seek = true;
        } else if offset < 0 as crate::stdlib::off64_t {
            if state.mode != crate::gzguts_h::GZ_READ {
                return -1 as crate::stdlib::off64_t;
            }
            offset += state.x.pos;
            if offset < 0 as crate::stdlib::off64_t {
                return -1 as crate::stdlib::off64_t;
            }
            rewind = true;
        }
    }
    if copy_seek {
        gz_error(
            file as crate::gzguts_h::gz_statep,
            crate::zlib_h::Z_OK,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let state = &mut *(file as crate::gzguts_h::gz_statep);
        state.strm.avail_in = 0 as crate::stdlib::uInt;
        state.x.pos += offset;
        return state.x.pos;
    }
    if rewind {
        if gzrewind(file) == -1 as ::core::ffi::c_int {
            return -1 as crate::stdlib::off64_t;
        }
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode == crate::gzguts_h::GZ_READ {
        n = crate::src::gzread::gz_consume(state, offset);
        offset -= n as crate::stdlib::off64_t;
    }
    state.skip = offset;
    return state.x.pos + offset;
}
#[export_name = "gzseek64"]

pub unsafe extern "C" fn gzseek64_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off64_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off64_t {
    gzseek64(file, offset, whence)
}
pub unsafe extern "C" fn gzseek(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzseek64(file, offset, whence);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzseek"]

pub unsafe extern "C" fn gzseek_ffi(
    mut file: crate::zlib_h::gzFile,
    mut offset: crate::stdlib::off_t,
    mut whence: ::core::ffi::c_int,
) -> crate::stdlib::off_t {
    gzseek(file, offset, whence)
}
pub unsafe extern "C" fn gztell64(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    gz_tell(state)
}

fn gz_tell(state: &crate::gzguts_h::gz_state) -> crate::stdlib::off64_t {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1 as crate::stdlib::off64_t;
    }
    state.x.pos
        + (if state.past != 0 {
            0 as crate::stdlib::off64_t
        } else {
            state.skip
        })
}
#[export_name = "gztell64"]

pub unsafe extern "C" fn gztell64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    gztell64(file)
}
pub unsafe extern "C" fn gztell(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gztell64(file);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gztell"]

pub unsafe extern "C" fn gztell_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    gztell(file)
}
pub unsafe extern "C" fn gzoffset64(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    let mut offset: crate::stdlib::off64_t = 0;
    if file.is_null() {
        return -1 as crate::stdlib::off64_t;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return -1 as crate::stdlib::off64_t;
    }
    offset = crate::stdlib::lseek64(
        state.fd,
        0 as crate::stdlib::__off64_t,
        crate::stdlib::SEEK_CUR,
    ) as crate::stdlib::off64_t;
    if offset == -1 as crate::stdlib::off64_t {
        return -1 as crate::stdlib::off64_t;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        offset -= state.strm.avail_in as crate::stdlib::off64_t;
    }
    return offset;
}
#[export_name = "gzoffset64"]

pub unsafe extern "C" fn gzoffset64_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off64_t {
    gzoffset64(file)
}
pub unsafe extern "C" fn gzoffset(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    let mut ret: crate::stdlib::off64_t = 0;
    ret = gzoffset64(file);
    return if ret == ret {
        ret
    } else {
        -1 as crate::stdlib::off_t
    };
}
#[export_name = "gzoffset"]

pub unsafe extern "C" fn gzoffset_ffi(mut file: crate::zlib_h::gzFile) -> crate::stdlib::off_t {
    gzoffset(file)
}
pub unsafe extern "C" fn gzeof(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let state = &*(file as crate::gzguts_h::gz_statep);
    gz_eof(state)
}

fn gz_eof(state: &crate::gzguts_h::gz_state) -> ::core::ffi::c_int {
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return 0 as ::core::ffi::c_int;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.past
    } else {
        0 as ::core::ffi::c_int
    }
}
#[export_name = "gzeof"]

pub unsafe extern "C" fn gzeof_ffi(mut file: crate::zlib_h::gzFile) -> ::core::ffi::c_int {
    gzeof(file)
}
pub unsafe extern "C" fn gzerror(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if file.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !errnum.is_null() {
        *errnum = state.err;
    }
    return if state.err == crate::zlib_h::Z_MEM_ERROR {
        b"out of memory\0".as_ptr() as *const ::core::ffi::c_char
    } else if state.msg.is_null() {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        state.msg as *const ::core::ffi::c_char
    };
}
#[export_name = "gzerror"]

pub unsafe extern "C" fn gzerror_ffi(
    mut file: crate::zlib_h::gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    gzerror(file, errnum)
}
pub unsafe extern "C" fn gzclearerr(mut file: crate::zlib_h::gzFile) {
    if file.is_null() {
        return;
    }
    let state = &mut *(file as crate::gzguts_h::gz_statep);
    if state.mode != crate::gzguts_h::GZ_READ && state.mode != crate::gzguts_h::GZ_WRITE {
        return;
    }
    if state.mode == crate::gzguts_h::GZ_READ {
        state.eof = 0 as ::core::ffi::c_int;
        state.past = 0 as ::core::ffi::c_int;
    }
    gz_error(
        state as *mut crate::gzguts_h::gz_state,
        crate::zlib_h::Z_OK,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[export_name = "gzclearerr"]

pub unsafe extern "C" fn gzclearerr_ffi(mut file: crate::zlib_h::gzFile) {
    gzclearerr(file)
}
pub unsafe extern "C" fn gz_error(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    let state_ref = &mut *state;
    if !state_ref.msg.is_null() {
        if state_ref.err != crate::zlib_h::Z_MEM_ERROR {
            crate::stdlib::free(state_ref.msg as *mut ::core::ffi::c_void);
        }
        state_ref.msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if err != crate::zlib_h::Z_OK && err != crate::zlib_h::Z_BUF_ERROR && state_ref.again == 0 {
        state_ref.x.have = 0 as ::core::ffi::c_uint;
    }
    state_ref.err = err;
    if msg.is_null() {
        return;
    }
    if err == crate::zlib_h::Z_MEM_ERROR {
        return;
    }
    state_ref.msg = crate::stdlib::malloc(
        crate::stdlib::strlen(state_ref.path)
            .wrapping_add(crate::stdlib::strlen(msg))
            .wrapping_add(3 as crate::__stddef_size_t_h::size_t),
    ) as *mut ::core::ffi::c_char;
    if state_ref.msg.is_null() {
        state_ref.err = crate::zlib_h::Z_MEM_ERROR;
        return;
    }
    crate::stdlib::snprintf(
        state_ref.msg,
        crate::stdlib::strlen(state_ref.path)
            .wrapping_add(crate::stdlib::strlen(msg))
            .wrapping_add(3 as crate::__stddef_size_t_h::size_t),
        b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        state_ref.path,
        b": \0".as_ptr() as *const ::core::ffi::c_char,
        msg,
    );
}
#[export_name = "gz_error"]

pub unsafe extern "C" fn gz_error_ffi(
    mut state: crate::gzguts_h::gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    gz_error(state, err, msg)
}

pub fn gz_skip_chunk(
    available: ::core::ffi::c_uint,
    skip: crate::stdlib::off64_t,
) -> ::core::ffi::c_uint {
    if (::core::mem::size_of::<::core::ffi::c_int>()
        == ::core::mem::size_of::<crate::stdlib::off64_t>()
        && available > gz_intmax())
        || available as crate::stdlib::off64_t > skip
    {
        skip as ::core::ffi::c_uint
    } else {
        available
    }
}

pub extern "C" fn gz_intmax() -> ::core::ffi::c_uint {
    return crate::limits_h::INT_MAX as ::core::ffi::c_uint;
}
#[export_name = "gz_intmax"]

pub unsafe extern "C" fn gz_intmax_ffi() -> ::core::ffi::c_uint {
    gz_intmax()
}
