use std::ffi;

/// Free a String owned by librir.
#[no_mangle]
pub extern "C" fn ffi_free_string(s: *mut libc::c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        ffi::CString::from_raw(s)
    }
}
