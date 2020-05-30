use std::fmt;
use std::io;

use encoding_rs::SHIFT_JIS;
use nom::IResult;
use nom::bytes::complete::take_while;

/// Trait for structs that are easy to pack to bytes.
pub trait Pack {
    /// Write the entirety of `self` as bytes to the write buffer `f`.
    fn write(&self, f: &mut dyn io::Write) -> io::Result<usize>;
}

/// Parse a zero-terminated string from the slice.
pub fn take_cstring(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c != b'\0')(i)
}

/// Parse a zero-terminated string from the slice, discarding the rest.
///
/// The cstring will be parsed from the first max_length bytes of the
/// slice, and on success the parser will discard exactly max_length
/// bytes from the input, regardless of the parsed string length.
pub fn take_cstring_from(i: &[u8], max_length: usize) -> IResult<&[u8], &[u8]> {
    take_cstring(i).map(|(_, s)| Ok((&i[max_length..], s)) )?
}

/// Decode a Shift JIS encoded byte slice.
pub fn sjis_to_string(i: &[u8]) -> Option<String> {
    let (cow, _, has_errors) = SHIFT_JIS.decode(i);
    if has_errors {
        return None
    }
    Some(cow.to_string())
}

/// Decode a Shift JIS encoded byte slice or hex representation.
pub fn sjis_to_string_lossy(i: &[u8]) -> String {
    sjis_to_string(i).unwrap_or(format!("{:x?}", i))
}

/// Represent an integer that can be 32 or 64 bits,
/// depending on the platform and flags used.
pub union VarSizeInt {
    pub vu32: u32,
    pub vu64: u64,
}

impl VarSizeInt {
    /// Set u64 value if condition is true, else the u32 as u64.
    pub fn u64_if(&self, c: bool) -> u64 {
        if c { unsafe { self.vu64 } } else { unsafe { self.vu32 as u64 } }
    }
}

impl fmt::Debug for VarSizeInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VarSizeInt: {{ {}: u32, {}: u64 }}", unsafe { self.vu32 }, unsafe { self.vu64 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_cstring() {
        assert_eq!(take_cstring(b"ABC\0\xFF"), Ok((b"\0\xFF".as_ref(), b"ABC".as_ref())));
        assert_eq!(take_cstring(b"\0"), Ok((b"\0".as_ref(), b"".as_ref())));
    }

    #[test]
    fn test_take_cstring_from() {
        // Take cstring from the whole slice; nothing remains.
        assert_eq!(
            take_cstring_from(
                b"ABC\0\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20",  // 0x10 bytes
                0x10,
            ),
            Ok((b"".as_ref(), b"ABC".as_ref()))
        );

        // Take cstring from the first half of the slice; the second half remains.
        assert_eq!(
            take_cstring_from(
                b"ABC\0\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20",  // 0x10 bytes
                0x8,
            ),
            Ok((b"\x20\x20\x20\x20\x20\x20\x20\x20".as_ref(), b"ABC".as_ref()))
        );
    }
}
