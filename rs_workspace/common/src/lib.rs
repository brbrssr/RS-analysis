use std::fs::File;
use std::io::{self, Write};

pub fn file_clean(path: String) -> io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(b"")?;

    Ok(())
}
