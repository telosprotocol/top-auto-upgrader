use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::error::AuError;

pub fn read_file(file_path_str: &str) -> Result<String, AuError> {
    let file_path = Path::new(file_path_str);
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_file(file_path_str: &str, content: String) -> Result<(), AuError> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path_str)?
        .write_all(content.as_bytes())?;
    Ok(())
}
