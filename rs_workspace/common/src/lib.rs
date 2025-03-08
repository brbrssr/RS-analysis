use std::ffi::CString;
use std::fs::File;
use std::io::{self, Write};
use std::os::raw::c_char;

pub fn file_clean(path: String) -> io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(b"")?;

    Ok(())
}

pub fn rust_string_to_c(s: &str) -> *mut c_char {
    CString::new(s).unwrap_or_default().into_raw()
}
