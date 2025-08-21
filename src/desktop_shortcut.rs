use crate::{conf::Conf, domain::Application};
use std::path::Path;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use anyhow::Result;

impl Application {
    pub fn get_desktop_shortcut_path(&self, conf: Conf) -> String {
        format!("{}/{}.desktop", conf.gnome_desktop_path, self.name.clone())
    }

    pub fn desktop_shortcut_exists(&self, conf: Conf) -> bool {
        Path::new(&self.get_desktop_shortcut_path(conf)).exists()
    }

    pub fn create_desktop_shortcut(&self, icon: Option<String>, exec: String, terminal:bool, conf: Conf) -> Result<()> {
        let name = self.name.clone();
        let path_str = self.get_desktop_shortcut_path(conf);
        let path = Path::new(&path_str);
        if path.exists() {
            panic!("Error: Desktop Shortcut already exists for the name {}", name);
        }
        let content = match icon {
            Some(icon_name) => format!("
[Desktop Entry]
Name={name}
Terminal={terminal}
Type=Application
Icon={icon_name}
Exec=\"{exec}\"
Categories=Game;
"),
        None => format!("
[Desktop Entry]
Name={name}
Terminal={terminal}
Type=Application
Icon=coolauncher
Exec=\"{exec}\"
Categories=Game;
"),
        };

        let mut file = File::create(path)?;
        Ok(file.write_all(content.as_bytes())?)
    }

    pub fn remove_desktop_shortcut(&self, conf: Conf) -> Result<()> {
        Command::new("rm").arg(self.get_desktop_shortcut_path(conf)).spawn()?;
        Ok(())
    }
}