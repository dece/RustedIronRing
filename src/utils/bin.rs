/// Return whether i has this flag set.
///
/// The flag value can be a combination of several flags, the function
/// will return  i has all the flags combined.
pub fn has_flag(i: u8, flag: u8) -> bool {
    i & flag == flag
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
}
