use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
pub fn create(file_name: &str) -> std::io::Result<()> {
    File::create(file_name)?;
    Ok(())
}

pub fn write(json_file: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("csv.json")?;

    let result = json_file.as_bytes();
    let _ = file.write_all(result);
    Ok(())
}
