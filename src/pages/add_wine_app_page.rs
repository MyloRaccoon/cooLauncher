use egui::Ui;
use egui_file::FileDialog;
use std::{ffi::OsStr, path::{Path, PathBuf}};
use crate::{conf::Conf, domain::Application, saver::Saver, tools::is_name_taken};


#[derive(Debug, Default)]
pub struct AddWineAppPage {
    pub open: bool,
    pub err_message: String,
    pub c_app_name: String,
    pub c_file_exe: Option<PathBuf>,
    pub open_file_dialog: Option<FileDialog>,
}

impl AddWineAppPage {
    pub fn show(&mut self, ui: &mut Ui, ctx: &egui::Context, apps: &mut Vec<Application>, conf: Conf) {
        ui.heading("Wine Application");
        if conf.is_wine_path_default() {
            ui.label("/!\\ Please set your wine path in your settings");
        } else {
            ui.label(self.err_message.clone());
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.add_space(250.);
                ui.label(".exe file:");
            });

            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.c_app_name));
                match self.c_file_exe.clone() {
                    Some(path_buf) => ui.label(path_buf.as_path().to_str().unwrap()),
                    None => ui.label("Please choose a .exe file"),
                };
                
                if ui.button("Choose a .exe").clicked() {
                    let filter = Box::new({
                        let ext = Some(OsStr::new("exe"));
                        move |path: &Path| -> bool { path.extension() == ext }
                    });
                    let mut dialog = FileDialog::open_file(self.c_file_exe.clone()).show_files_filter(filter);
                    dialog.open();
                    self.open_file_dialog = Some(dialog);
                }

                if let Some(dialog) = &mut self.open_file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            self.c_file_exe = Some(file.to_path_buf());
                        }
                    }
                }
            });
            ui.horizontal(|ui| {
                if ui.button("+ Add application").clicked() {
                    if is_name_taken(apps.clone(), self.c_app_name.clone()) {
                        self.err_message = "/!\\ This name is already taken".to_string();
                    } else if self.c_app_name == String::new() {
                        self.err_message = "/!\\ Please enter a name".to_string();
                    } else if self.c_file_exe.is_none() {
                        self.err_message = "/!\\ Please choose a .exe file".to_string();
                    } else {
                        let app_name = self.c_app_name.clone();
                        let file_path_buf = self.c_file_exe.clone().unwrap();
                        let file_path = file_path_buf.as_path();
                        let exe_name = file_path.file_name().unwrap();
                        let dir_path = file_path.parent().unwrap();
                        apps.push(
                            Application::wine_app(
                                app_name,
                                dir_path.to_str().unwrap().to_string(),
                                exe_name.to_str().unwrap().to_string()
                            )
                        );
                        let _ = Saver::save(apps.clone(), conf.clone());
                        self.open = false;
                    }
                }
                if ui.button("Cancel").clicked() {
                    self.open = false;
                }
            });
        }
    }
}