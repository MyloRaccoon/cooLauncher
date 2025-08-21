use anyhow::{Error, Result};
use home::home_dir;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn delete_line(line: String, path_string: String) -> core::result::Result<(), Error> {
    let path = Path::new(&path_string);
    let temp_path = path.with_extension("tmp");

    let mut file = File::open(path)?;

    let mut content = String::default();
    file.read_to_string(&mut content)?;
    let new_content = content.replace(&line, "");

    let mut temp_file = File::create(temp_path.clone())?;
    temp_file.write_all(new_content.as_bytes())?;

    fs::rename(temp_path, path)?;
    Ok(())
}

pub fn get_main_dir() -> String {
    let mut path = home_dir().unwrap();
    path.push(".coolauncher");
    path.to_str().unwrap().to_string()
}

pub fn create_main_dir() -> Result<(), std::io::Error> {
    let path = get_main_dir();
    
    if !Path::new(&path).exists() {
        fs::create_dir(path)
    } else {
        Ok(())
    }
}