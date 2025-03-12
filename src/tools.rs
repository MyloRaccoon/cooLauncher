use anyhow::Error;

use crate::domain::Application;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn is_name_taken(apps: Vec<Application>, app_name: String) -> bool {
    for app in apps {
        if app.name == app_name {
            return true;
        }
    }
    false
}

pub fn is_alias_taken(alias: String, file: String) -> bool {
    let mut file = File::open(file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content.contains(format!("alias {}", alias.clone()).as_str())
}

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