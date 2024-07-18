use std::io;
use std::{fs::File, io::Read};

pub fn file_to_str(filename: &str) -> Result<String, io::Error> {
    let mut file = String::new();

    File::open(filename)?.read_to_string(&mut file)?;

    Ok(file)
}
