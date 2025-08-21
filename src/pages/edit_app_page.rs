use egui::Ui;
use egui_file::FileDialog;

use crate::{conf::Conf, domain::{AppType, Application}, launcher::is_app_name_taken, saver::Saver};
use std::{ffi::OsStr, path::{Path, PathBuf}};

#[derive(Debug, Default)]
pub struct EditAppPage {
    pub is_open: bool,
    err_message: String,
    app_name: String,
    app_command: String,
    app_arg: String,
    app_file_exe: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

impl EditAppPage {
    pub fn open(&mut self, app: &mut Application) {
        self.set_current_app(app);
        self.is_open = true;
    }

    pub fn set_current_app(&mut self, app: &mut Application) {
        self.app_name = app.name.clone();
        match app.app_type {
            AppType::Custom => {
                self.app_command = app.command.as_mut().unwrap().program.clone();
                self.app_arg = app.command.as_mut().unwrap().args[0].clone();
            }
            AppType::Wine => {
                self.app_file_exe = Some(PathBuf::from(&format!("{}/{}", app.exe_path.as_mut().unwrap(), app.exe_name.as_mut().unwrap())));
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui, apps: Vec<Application>, app: &mut Application, conf: Conf, ctx: &egui::Context) {
        ui.heading(app.name.clone());
        ui.label(self.err_message.clone());
        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.add(egui::TextEdit::singleline(&mut self.app_name));
        });
        match app.app_type {
            AppType::Custom => {
                ui.horizontal(|ui| {
                    ui.label("Command: ");
                    ui.add(egui::TextEdit::singleline(&mut self.app_command));
                });
                ui.horizontal(|ui| {
                    ui.label("Args: ");
                    ui.add(egui::TextEdit::singleline(&mut self.app_arg));
                });
            }
            AppType::Wine => {
                ui.horizontal(|ui| {
                    ui.label(".exe file: ");
                    ui.label(self.app_file_exe.as_mut().unwrap().as_path().to_str().unwrap());

                    if ui.button("Choose a .exe").clicked() {
                        let filter = Box::new({
                            let ext = Some(OsStr::new("exe"));
                            move |path: &Path| -> bool { path.extension() == ext }
                        });
                        let mut dialog = FileDialog::open_file(self.app_file_exe.clone()).show_files_filter(filter);
                        dialog.open();
                        self.open_file_dialog = Some(dialog);
                    }

                    if let Some(dialog) = &mut self.open_file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.app_file_exe = Some(file.to_path_buf());
                            }
                        }
                    }
                });
            }
        }
        ui.horizontal(|ui| {
            if ui.button("Save and close").clicked() {
                match app.app_type {
                    AppType::Custom => {
                        if is_app_name_taken(apps.clone(), self.app_name.clone()) {
                            self.err_message = "/!\\ This name is already taken".to_string();
                        } else if self.app_name == String::new() {
                            self.err_message = "/!\\ Please enter a name".to_string();
                        } else if self.app_command == String::new() {
                            self.err_message = "/!\\ Please enter a command".to_string();
                        } else {
                            app.edit_from_strings(self.app_name.clone(), self.app_command.clone(), std::slice::from_ref(&self.app_arg));
                            let _ = Saver::save(apps.clone(), conf.clone());
                            self.is_open = false;
                        }
                    }
                    AppType::Wine => {
                        if is_app_name_taken(apps.clone(), self.app_name.clone()) {
                            self.err_message = "/!\\ This name is already taken".to_string();
                        } else if self.app_name == String::new() {
                            self.err_message = "/!\\ Please enter a name".to_string();
                        } else {
                            app.name = self.app_name.clone();
                        }
                    }
                }
            }
            if ui.button("Cancel").clicked() {
                self.is_open = false;
            }
        });
    }
}