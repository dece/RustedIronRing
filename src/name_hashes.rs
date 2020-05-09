use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Error};

use num_bigint::BigUint;
use num_traits::identities::Zero;

/// Compute the weird hash for a string. Same mechanic since DeS.
pub fn hash(s: &str) -> u32 {
    let s = s.to_lowercase();
    let mut val = BigUint::zero();
    for c in s.chars() {
        val *= 37u8;
        val += c as u32;
    }
    val.to_u32_digits()[0]
}

/// Get the string representation for this hash.
pub fn hash_as_string(h: u32) -> String {
    format!("{:08X}", h)
}

//#[no_mangle]
//pub extern "C" fn nam_hash_as_string(h: u32) -> *mut libc::c_char {
//    hash_as_string(h)
//}

/// Load a namelist file into a map.
///
/// Format for the input file should be the following for every line:
/// CAFECAFE: /chr/whatever.ext
pub fn load_name_map(path: &str) -> Result<HashMap<String, String>, Error> {
    let mut names = HashMap::new();
    let namefile = fs::File::open(path)?;
    for line_ in BufReader::new(namefile).lines() {
        if let Ok(line) = line_ {
            let (hash, name) = line.split_at(8);
            names.insert(hash.to_string(), name[2..].to_string());
        }
    }
    Ok(names)
}

mod rir_ffi {
    use std::ffi;
    use super::*;

    #[no_mangle]
    pub extern "C" fn nam_hash(s: *const libc::c_char) -> u32 {
        let c_s = unsafe { assert!(!s.is_null()); ffi::CStr::from_ptr(s) };
        hash(c_s.to_str().unwrap())
    }

    #[no_mangle]
    pub extern "C" fn nam_hash_as_string(h: u32) -> *mut libc::c_char {
        ffi::CString::new(hash_as_string(h)).unwrap().into_raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("/chr/c0000.anibnd.dcx"), 0xF8630FB1);
        assert_eq!(hash("/param/DrawParam/default_DrawParam.parambnd.dcx"), 0xD9209D30);
    }

    #[test]
    fn test_hash_as_string() {
        assert_eq!(hash_as_string(0xCAFECAFE), "CAFECAFE");
        assert_eq!(hash_as_string(0xDECE), "0000DECE");
    }
}
