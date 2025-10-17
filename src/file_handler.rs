use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

// Arbitrary file write vulnerability
pub fn write_to_file(file_path: String, data: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

// Path traversal vulnerability (Absolute and Relative)
pub fn read_file(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path);
    if path.exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}
