use serde_json;
use std::ffi::CString;
use std::fs;
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

pub fn write_data(json_data: serde_json::Value, path: String) -> Result<(), String> {
    let json_string = serde_json::to_string_pretty(&json_data)
        .map_err(|e| format!("Error: failed to serialize JSON: {}", e))?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("Error: failed to open file: {}", e))?;

    writeln!(file, "{}", json_string).map_err(|e| format!("Error: failed to write data: {}", e))?;

    Ok(())
}
