use egui::Ui;
use egui_file::FileDialog;
use std::path::PathBuf;
use crate::{conf::Conf, domain::Application, saver::Saver};

#[derive(Debug, Default)]
pub struct SettingsPage {
    pub open: bool,
    pub wine_file: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

impl SettingsPage {
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
                self.open_file_dialog = Some(dialog);
            }
            if let Some(dialog) = &mut self.open_file_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.wine_file = Some(file.to_path_buf());
                    }
                }
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Save Changes").clicked() {
                conf.wine_path = self.wine_file.clone().unwrap().display().to_string();
                let _ = Saver::save(apps.clone(), conf.clone());
            }
            if ui.button("Cancel").clicked() {
                self.open = false;
            }
            if ui.button("Close and Save").clicked() {
                conf.wine_path = self.wine_file.clone().unwrap().display().to_string();
                let _ = Saver::save(apps.clone(), conf.clone());
                self.open = false;
            }
        });
    }
}