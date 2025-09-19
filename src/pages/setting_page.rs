use egui::Ui;
use egui_file::FileDialog;
use home::home_dir;
use std::path::PathBuf;
use crate::{conf::Conf, domain::Application, saver::Saver};

#[derive(Debug, Default)]
pub struct SettingsPage {
    pub is_open: bool,
    wine_file: Option<PathBuf>,
    alias_file: Option<PathBuf>,
    gnome_desktop_dir: Option<PathBuf>,
    open_wine_file_dialog: Option<FileDialog>,
    open_alias_file_dialog: Option<FileDialog>,
    open_gnome_desktop_dir_dialog: Option<FileDialog>,
}

impl SettingsPage {
    pub fn open(&mut self, conf: Conf) {
        self.wine_file = match conf.is_wine_path_default() {
            true => None,
            false => Some(PathBuf::from(&conf.wine_path)),
        };
        self.alias_file = match conf.is_alias_path_default() {
            true => None,
            false => Some(PathBuf::from(&conf.alias_path)),
        };
        self.gnome_desktop_dir = match conf.gnome_desktop_path.clone().is_empty() {
            true => None,
            false => Some(PathBuf::from(&conf.gnome_desktop_path)),
        };
        self.is_open = true;
    }

    fn save(&self, conf: &mut Conf) {
        conf.wine_path = match self.wine_file.clone() {
            Some(path_buf) => path_buf.display().to_string(),
            None => String::default(),
        };
        conf.alias_path = match self.alias_file.clone() {
            Some(path_buf) => path_buf.display().to_string(),
            None => String::default(),
        };
        let mut default_gnome_desktop_path = home_dir().unwrap();
        default_gnome_desktop_path.push(".local/share/applications");
        conf.gnome_desktop_path = match self.gnome_desktop_dir.clone() {
            Some(path_buf) => path_buf.display().to_string(),
            None => default_gnome_desktop_path.display().to_string(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, ctx: &egui::Context, apps: Vec<Application>, conf: &mut Conf) {
        ui.heading("Settings");
        ui.horizontal(|ui| {
            ui.label("Wine Path: ");
            match self.wine_file.clone() {
                Some(path_buf) => ui.label(path_buf.as_path().to_str().unwrap()),
                None => ui.label("Please set a wine file"),
            };
            if ui.button("Choose a file").clicked() {
                let mut dialog = FileDialog::open_file(self.wine_file.clone());
                dialog.open();
                self.open_wine_file_dialog = Some(dialog);
            }
            if let Some(dialog) = &mut self.open_wine_file_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.wine_file = Some(file.to_path_buf());
                    }
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Alias File: ");
            match self.alias_file.clone() {
                Some(path_buf) => ui.label(path_buf.as_path().to_str().unwrap()),
                None => ui.label("Please set an alias file"),
            };
            if ui.button("Choose a file").clicked() {
                let mut dialog = FileDialog::open_file(self.alias_file.clone());
                dialog.open();
                self.open_alias_file_dialog = Some(dialog);
            }
            if let Some(dialog) = &mut self.open_alias_file_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.alias_file = Some(file.to_path_buf());
                    }
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Desktop Files Directory: ");
            match self.gnome_desktop_dir.clone() {
                Some(path_buf) => ui.label(path_buf.as_path().to_str().unwrap()),
                None => ui.label("Please set a desktop directory"),
            };
            if ui.button("Choose a directory").clicked() {
                let mut dialog = FileDialog::select_folder(self.gnome_desktop_dir.clone());
                dialog.open();
                self.open_gnome_desktop_dir_dialog = Some(dialog);
            }
            if let Some(dialog) = &mut self.open_gnome_desktop_dir_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(dir) = dialog.path() {
                        self.gnome_desktop_dir = Some(dir.to_path_buf());
                    }
                }
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Save Changes").clicked() {
                self.save(conf);
            }
            if ui.button("Cancel").clicked() {
                self.is_open = false;
            }
            if ui.button("Close and Save").clicked() {
                self.save(conf);
                let _ = Saver::save(apps.clone(), conf.clone());
                self.is_open = false;
            }
        });
    }
}