pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::deflate::internal_state;
pub use crate::src::inflate::inflateEnd;
pub use crate::src::inflate::inflate_impl;
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

struct BitReader<'a> {
    input: &'a [u8],
    byte: usize,
    bits: u32,
    held: u32,
}

impl<'a> BitReader<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            byte: 0,
            bits: 0,
            held: 0,
        }
    }

    fn read(&mut self, count: u32) -> Option<u32> {
        while self.bits < count {
            let next = *self.input.get(self.byte)? as u32;
            self.byte += 1;
            self.held |= next << self.bits;
            self.bits += 8;
        }
        let value = self.held & ((1u32 << count) - 1);
        self.held >>= count;
        self.bits -= count;
        Some(value)
    }

    fn align_byte(&mut self) {
        self.held = 0;
        self.bits = 0;
    }

    fn read_byte(&mut self) -> Option<u8> {
        self.align_byte();
        let byte = *self.input.get(self.byte)?;
        self.byte += 1;
        Some(byte)
    }
}

struct HuffmanCode {
    bits: u16,
    len: u8,
    symbol: u16,
}

struct Huffman {
    codes: Vec<HuffmanCode>,
}

fn reverse_bits(mut value: u16, count: u8) -> u16 {
    let mut reversed = 0;
    for _ in 0..count {
        reversed = (reversed << 1) | value & 1;
        value >>= 1;
    }
    reversed
}

impl Huffman {
    fn from_lengths(lengths: &[u8]) -> Option<Self> {
        let mut counts = [0u16; 16];
        for &len in lengths {
            if len > 15 {
                return None;
            }
            if len != 0 {
                counts[len as usize] += 1;
            }
        }
        let mut next = [0u16; 16];
        let mut code = 0u16;
        for len in 1..=15 {
            code = (code + counts[len - 1]) << 1;
            next[len] = code;
        }
        let mut codes = Vec::with_capacity(lengths.len());
        for (symbol, &len) in lengths.iter().enumerate() {
            if len != 0 {
                let code = next[len as usize];
                next[len as usize] += 1;
                codes.push(HuffmanCode {
                    bits: reverse_bits(code, len),
                    len,
                    symbol: symbol as u16,
                });
            }
        }
        (!codes.is_empty()).then_some(Self { codes })
    }

    fn decode(&self, bits: &mut BitReader<'_>) -> Option<u16> {
        let mut code = 0u16;
        for len in 1..=15 {
            code |= (bits.read(1)? as u16) << (len - 1);
            if let Some(entry) = self
                .codes
                .iter()
                .find(|entry| entry.len == len && entry.bits == code)
            {
                return Some(entry.symbol);
            }
        }
        None
    }
}

const LENGTH_BASE: [usize; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LENGTH_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [usize; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

fn fixed_trees() -> (Huffman, Huffman) {
    let mut literal_lengths = [0u8; 288];
    literal_lengths[..144].fill(8);
    literal_lengths[144..256].fill(9);
    literal_lengths[256..280].fill(7);
    literal_lengths[280..].fill(8);
    let distance_lengths = [5u8; 32];
    (
        Huffman::from_lengths(&literal_lengths).expect("fixed literal tree is valid"),
        Huffman::from_lengths(&distance_lengths).expect("fixed distance tree is valid"),
    )
}

fn dynamic_trees(bits: &mut BitReader<'_>) -> Option<(Huffman, Huffman)> {
    let literal_count = bits.read(5)? as usize + 257;
    let distance_count = bits.read(5)? as usize + 1;
    let code_count = bits.read(4)? as usize + 4;
    if literal_count > 286 || distance_count > 32 {
        return None;
    }
    const ORDER: [usize; 19] = [
        16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
    ];
    let mut code_lengths = [0u8; 19];
    for &index in &ORDER[..code_count] {
        code_lengths[index] = bits.read(3)? as u8;
    }
    let code_tree = Huffman::from_lengths(&code_lengths)?;
    let mut lengths = Vec::with_capacity(literal_count + distance_count);
    while lengths.len() < literal_count + distance_count {
        match code_tree.decode(bits)? {
            value @ 0..=15 => lengths.push(value as u8),
            16 => {
                let previous = *lengths.last()?;
                let repeat = bits.read(2)? as usize + 3;
                if lengths.len() + repeat > literal_count + distance_count {
                    return None;
                }
                lengths.extend(std::iter::repeat_n(previous, repeat));
            }
            17 => {
                let repeat = bits.read(3)? as usize + 3;
                if lengths.len() + repeat > literal_count + distance_count {
                    return None;
                }
                lengths.extend(std::iter::repeat_n(0, repeat));
            }
            18 => {
                let repeat = bits.read(7)? as usize + 11;
                if lengths.len() + repeat > literal_count + distance_count {
                    return None;
                }
                lengths.extend(std::iter::repeat_n(0, repeat));
            }
            _ => return None,
        }
    }
    let literals = Huffman::from_lengths(&lengths[..literal_count])?;
    if lengths[256] == 0 {
        return None;
    }
    let distances = Huffman::from_lengths(&lengths[literal_count..])?;
    Some((literals, distances))
}

fn decode_huffman_block(
    bits: &mut BitReader<'_>,
    literals: &Huffman,
    distances: &Huffman,
    dest: &mut [u8],
    written: &mut usize,
) -> Result<(), ::core::ffi::c_int> {
    loop {
        let symbol = literals.decode(bits).ok_or(crate::zlib_h::Z_DATA_ERROR)?;
        match symbol {
            0..=255 => {
                let slot = dest.get_mut(*written).ok_or(crate::zlib_h::Z_BUF_ERROR)?;
                *slot = symbol as u8;
                *written += 1;
            }
            256 => return Ok(()),
            257..=285 => {
                let index = (symbol - 257) as usize;
                let length = LENGTH_BASE[index]
                    + bits
                        .read(LENGTH_EXTRA[index] as u32)
                        .ok_or(crate::zlib_h::Z_DATA_ERROR)? as usize;
                let distance_symbol =
                    distances.decode(bits).ok_or(crate::zlib_h::Z_DATA_ERROR)? as usize;
                let Some(&base) = DIST_BASE.get(distance_symbol) else {
                    return Err(crate::zlib_h::Z_DATA_ERROR);
                };
                let distance = base
                    + bits
                        .read(DIST_EXTRA[distance_symbol] as u32)
                        .ok_or(crate::zlib_h::Z_DATA_ERROR)? as usize;
                if distance > *written {
                    return Err(crate::zlib_h::Z_DATA_ERROR);
                }
                for _ in 0..length {
                    let byte = dest[*written - distance];
                    let slot = dest.get_mut(*written).ok_or(crate::zlib_h::Z_BUF_ERROR)?;
                    *slot = byte;
                    *written += 1;
                }
            }
            _ => return Err(crate::zlib_h::Z_DATA_ERROR),
        }
    }
}

struct DecodeResult {
    status: ::core::ffi::c_int,
    written: usize,
    consumed: usize,
}

fn decode_zlib(dest: &mut [u8], source: &[u8]) -> DecodeResult {
    let Some((&cmf, &flg)) = source.first().zip(source.get(1)) else {
        return DecodeResult {
            status: crate::zlib_h::Z_DATA_ERROR,
            written: 0,
            consumed: source.len(),
        };
    };
    if cmf & 15 != 8
        || cmf >> 4 > 7
        || (u16::from(cmf) << 8 | u16::from(flg)) % 31 != 0
        || flg & 32 != 0
    {
        return DecodeResult {
            status: crate::zlib_h::Z_DATA_ERROR,
            written: 0,
            consumed: 2,
        };
    }
    let mut bits = BitReader::new(&source[2..]);
    let mut written = 0usize;
    loop {
        let Some(last) = bits.read(1) else {
            return DecodeResult {
                status: crate::zlib_h::Z_DATA_ERROR,
                written,
                consumed: bits.byte + 2,
            };
        };
        let Some(kind) = bits.read(2) else {
            return DecodeResult {
                status: crate::zlib_h::Z_DATA_ERROR,
                written,
                consumed: bits.byte + 2,
            };
        };
        let result = match kind {
            0 => {
                bits.align_byte();
                let Some(len) = bits
                    .read_byte()
                    .zip(bits.read_byte())
                    .map(|(low, high)| u16::from_le_bytes([low, high]))
                else {
                    return DecodeResult {
                        status: crate::zlib_h::Z_DATA_ERROR,
                        written,
                        consumed: bits.byte + 2,
                    };
                };
                let Some(nlen) = bits
                    .read_byte()
                    .zip(bits.read_byte())
                    .map(|(low, high)| u16::from_le_bytes([low, high]))
                else {
                    return DecodeResult {
                        status: crate::zlib_h::Z_DATA_ERROR,
                        written,
                        consumed: bits.byte + 2,
                    };
                };
                if len != !nlen {
                    return DecodeResult {
                        status: crate::zlib_h::Z_DATA_ERROR,
                        written,
                        consumed: bits.byte + 2,
                    };
                }
                for _ in 0..len {
                    let Some(byte) = bits.read_byte() else {
                        return DecodeResult {
                            status: crate::zlib_h::Z_DATA_ERROR,
                            written,
                            consumed: bits.byte + 2,
                        };
                    };
                    let Some(slot) = dest.get_mut(written) else {
                        return DecodeResult {
                            status: crate::zlib_h::Z_BUF_ERROR,
                            written,
                            consumed: bits.byte + 2,
                        };
                    };
                    *slot = byte;
                    written += 1;
                }
                Ok(())
            }
            1 => {
                let (literals, distances) = fixed_trees();
                decode_huffman_block(&mut bits, &literals, &distances, dest, &mut written)
            }
            2 => match dynamic_trees(&mut bits) {
                Some((literals, distances)) => {
                    decode_huffman_block(&mut bits, &literals, &distances, dest, &mut written)
                }
                None => Err(crate::zlib_h::Z_DATA_ERROR),
            },
            _ => Err(crate::zlib_h::Z_DATA_ERROR),
        };
        if let Err(error) = result {
            return DecodeResult {
                status: error,
                written,
                consumed: bits.byte + 2,
            };
        }
        if last != 0 {
            break;
        }
    }
    bits.align_byte();
    let Some(trailer) = bits
        .read_byte()
        .zip(bits.read_byte())
        .zip(bits.read_byte())
        .zip(bits.read_byte())
        .map(|(((a, b), c), d)| u32::from_be_bytes([a, b, c, d]))
    else {
        return DecodeResult {
            status: crate::zlib_h::Z_DATA_ERROR,
            written,
            consumed: bits.byte + 2,
        };
    };
    if crate::src::adler32::adler32_z(1, &dest[..written]) as u32 != trailer {
        return DecodeResult {
            status: crate::zlib_h::Z_DATA_ERROR,
            written,
            consumed: bits.byte + 2,
        };
    }
    DecodeResult {
        status: crate::zlib_h::Z_OK,
        written,
        consumed: bits.byte + 2,
    }
}
fn uncompress2_z_impl(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> DecodeResult {
    decode_zlib(dest, source)
}
#[export_name = "uncompress2_z"]

pub unsafe extern "C" fn uncompress2_z_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::z_size_t,
    source: *const crate::stdlib::Bytef,
    sourceLen: *mut crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    let Some(source_len) = (unsafe { sourceLen.as_ref() }).copied() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(dest_len) = (unsafe { destLen.as_ref() }).copied() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if source_len > isize::MAX as crate::stdlib::z_size_t
        || dest_len > isize::MAX as crate::stdlib::z_size_t
        || (source_len != 0 && source.is_null())
        || (dest_len != 0 && dest.is_null())
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if source_len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(source, source_len) }
    };
    let dest = if dest_len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dest, dest_len) }
    };
    let result = uncompress2_z_impl(dest, source);
    unsafe {
        *sourceLen = result.consumed;
        *destLen = result.written;
    }
    result.status
}
#[export_name = "uncompress2"]

pub unsafe extern "C" fn uncompress2_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::uLongf,
    source: *const crate::stdlib::Bytef,
    sourceLen: *mut crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    let Some(source_len) = (unsafe { sourceLen.as_ref() }).copied() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Some(dest_len) = (unsafe { destLen.as_ref() }).copied() else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Ok(source_len) = usize::try_from(source_len) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    let Ok(dest_len) = usize::try_from(dest_len) else {
        return crate::zlib_h::Z_STREAM_ERROR;
    };
    if source_len > isize::MAX as usize
        || dest_len > isize::MAX as usize
        || (source_len != 0 && source.is_null())
        || (dest_len != 0 && dest.is_null())
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if source_len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(source, source_len) }
    };
    let dest = if dest_len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dest, dest_len) }
    };
    let result = uncompress2_z_impl(dest, source);
    unsafe {
        *sourceLen = result.consumed as crate::stdlib::uLong;
        *destLen = result.written as crate::stdlib::uLongf;
    }
    result.status
}
pub fn uncompress_z(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> (::core::ffi::c_int, crate::stdlib::z_size_t) {
    let result = decode_zlib(dest, source);
    (result.status, result.written)
}
#[export_name = "uncompress_z"]

pub unsafe extern "C" fn uncompress_z_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::z_size_t,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::z_size_t,
) -> ::core::ffi::c_int {
    if destLen.is_null()
        || sourceLen > isize::MAX as crate::stdlib::z_size_t
        || (sourceLen != 0 && source.is_null())
    {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = unsafe { *destLen };
    if dest_len > isize::MAX as crate::stdlib::z_size_t || (dest_len != 0 && dest.is_null()) {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source = if sourceLen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(source, sourceLen) }
    };
    let dest = if dest_len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dest, dest_len) }
    };
    let (result, written) = uncompress_z(dest, source);
    unsafe { *destLen = written };
    result
}
pub fn uncompress(
    dest: &mut [crate::stdlib::Bytef],
    source: &[crate::stdlib::Bytef],
) -> (::core::ffi::c_int, crate::stdlib::uLongf) {
    let result = decode_zlib(dest, source);
    (result.status, result.written as crate::stdlib::uLongf)
}
#[export_name = "uncompress"]

pub unsafe extern "C" fn uncompress_ffi(
    dest: *mut crate::stdlib::Bytef,
    destLen: *mut crate::stdlib::uLongf,
    source: *const crate::stdlib::Bytef,
    sourceLen: crate::stdlib::uLong,
) -> ::core::ffi::c_int {
    if (sourceLen != 0 && source.is_null()) || destLen.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let dest_len = match usize::try_from(unsafe { *destLen }) {
        Ok(length) => length,
        Err(_) => return crate::zlib_h::Z_STREAM_ERROR,
    };
    if dest_len != 0 && dest.is_null() {
        return crate::zlib_h::Z_STREAM_ERROR;
    }
    let source_len = match usize::try_from(sourceLen) {
        Ok(length) => length,
        Err(_) => return crate::zlib_h::Z_STREAM_ERROR,
    };
    let source = if sourceLen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(source, source_len) }
    };
    let dest = if dest_len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dest, dest_len) }
    };
    let (result, written) = uncompress(dest, source);
    unsafe { *destLen = written };
    result
}
