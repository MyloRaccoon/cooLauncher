use egui::Ui;
use egui_file::FileDialog;
use std::path::PathBuf;
use crate::{conf::Conf, domain::Application, saver::Saver};

#[derive(Debug, Default)]
pub struct SettingsPage {
    pub open: bool,
    pub wine_file: Option<PathBuf>,
    pub alias_file: Option<PathBuf>,
    open_wine_file_dialog: Option<FileDialog>,
    open_alias_file_dialog: Option<FileDialog>,
}

impl SettingsPage {
    fn save(&self, conf: &mut Conf) {
        conf.wine_path = match self.wine_file.clone() {
            Some(path_buf) => path_buf.display().to_string(),
            None => String::default(),
        };
        conf.alias_path = match self.alias_file.clone() {
            Some(path_buf) => path_buf.display().to_string(),
            None => String::default(),
        };
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
            if ui.button("Save Changes").clicked() {
                self.save(conf);
            }
            if ui.button("Cancel").clicked() {
                self.open = false;
            }
            if ui.button("Close and Save").clicked() {
                self.save(conf);
                let _ = Saver::save(apps.clone(), conf.clone());
                self.open = false;
            }
        });
    }
}