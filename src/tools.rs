use anyhow::{Error, Result};
use regex::Regex;

use crate::domain::Application;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

const APP_DESKTOP_PATH: &str = "/usr/share/applications/";

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
    let regex = Regex::new(format!("alias *{}[ =]", alias.clone()).as_str()).unwrap();
    regex.is_match(&content)
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

pub fn gnome_shortcut_exists(name: String) -> bool {
    Path::new(&format!("{}{}.desktop", self::APP_DESKTOP_PATH, name)).exists()
}

pub fn create_gnome_shortcut(name: String, icon: Option<String>, exec: String) -> Result<()> {
    let path_str = format!("{}{}.desktop", self::APP_DESKTOP_PATH, name);
    let path = Path::new(&path_str);
    if path.exists() {
        panic!("Error: Gnome Desktop Shortcut already exists for the name {}", name.clone());
    }
    let content = match icon {
        Some(icon_name) => format!("
[Desktop Entry]\n
Name={name}\n
Terminal=true\n
Type=Application\n
Icon={icon_name}\n
Exec={exec}
"),
        None => format!("
[Desktop Entry]\n
Name={name}\n
Terminal=true\n
Type=Application\n
Exec={exec}
"),
    };

    let mut file = File::create(path)?;
    Ok(file.write_all(content.as_bytes())?)
}