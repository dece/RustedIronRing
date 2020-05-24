/// Return whether i has this flag set.
///
/// The flag value can be a combination of several flags, the function
/// will return  i has all the flags combined.
pub fn has_flag(i: u8, flag: u8) -> bool {
    i & flag == flag
}

/// Return a mask for this number of bits.
pub fn mask(bit_size: usize) -> usize {
    (1 << bit_size) - 1
}

/// Return the number of bytes to pad from ofs to alignment.
pub fn pad(ofs: usize, alignment: usize) -> usize {
    (alignment - (ofs % alignment)) % alignment
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_flag() {
        assert!(has_flag(0x0, 0x0));
        assert!(has_flag(0x80, 0x80));
        assert!(!has_flag(0x80, 0x40));
    }

    #[test]
    fn test_mask() {
        assert_eq!(mask(1), 0b00000001);
        assert_eq!(mask(2), 0b00000011);
        assert_eq!(mask(4), 0b00001111);
        assert_eq!(mask(8), 0b11111111);
        assert_eq!(mask(15), 0b01111111_11111111);
    }

    #[test]
    fn test_pad() {
        assert_eq!(pad(0, 4), 0);
        assert_eq!(pad(1, 4), 3);
        assert_eq!(pad(3, 4), 1);
        assert_eq!(pad(4, 4), 0);
        assert_eq!(pad(4, 16), 12);
        assert_eq!(pad(15, 16), 1);
        assert_eq!(pad(16, 16), 0);
        assert_eq!(pad(17, 16), 15);
    }
}
