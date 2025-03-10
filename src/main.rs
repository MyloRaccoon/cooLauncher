use coolauncher::conf::{Conf};
use coolauncher::domain::{AppType, Application};
use coolauncher::saver::{Saver, LauncherSave};
use eframe::egui;
use egui::{CentralPanel, ScrollArea, SidePanel, TopBottomPanel, Ui, ViewportBuilder, Visuals, Window};
use std::{ffi::OsStr, path::{Path, PathBuf}};
use egui_file::FileDialog;
use std::str::FromStr;

fn main() -> Result<(), eframe::Error> {
    let mut launcher = Launcher::new();
    launcher.load(Saver::load());
    let options = eframe::NativeOptions{
        viewport: ViewportBuilder::default()
            .with_inner_size([1200., 720.])
            .with_position([0., 0.]),
        ..Default::default()
    };
    eframe::run_native(
        "cooLauncher",
        options,
        Box::new(|cc| {
            Ok(Box::new(launcher.set_visuals_dark(cc)))
        }),
    )
}

#[derive(Debug, Default)]
struct SettingsPage {
    open: bool,
    wine_file: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

impl SettingsPage {
    fn show(&mut self, ui: &mut Ui, ctx: &egui::Context, apps: Vec<Application>, conf: &mut Conf) {
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

#[derive(Debug, Default)]
struct AddAppPage {
    open: bool,
    err_message: String,
    c_app_name: String,
    c_app_command: String,
    c_app_arg: String,
}

impl AddAppPage {
    fn show(&mut self, ui: &mut Ui, apps: &mut Vec<Application>, conf: Conf) {
        ui.heading("Manual command");
        ui.label(self.err_message.clone());
        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.add_space(250.);
            ui.label("Command: ");
            ui.add_space(223.);
            ui.label("Argument: ");
        });

        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.c_app_name));
            ui.add(egui::TextEdit::singleline(&mut self.c_app_command));
            ui.add(egui::TextEdit::singleline(&mut self.c_app_arg));
        });
        let mut arg_vec = Vec::new();
        if self.c_app_arg != String::default() {
            arg_vec.push(self.c_app_arg.clone());
        }
        ui.horizontal(|ui| {
            if ui.button("+ Add application").clicked() {

                if is_name_taken(apps.clone(), self.c_app_name.clone()) {
                    self.err_message = "/!\\ This name is already taken".to_string();
                } else if self.c_app_name == String::new() {
                    self.err_message = "/!\\ Please enter a name".to_string();
                } else if self.c_app_command == String::new() {
                    self.err_message = "/!\\ Please enter a command".to_string();
                } else {
                    apps.push(Application::from_strings(self.c_app_name.clone(), self.c_app_command.clone(), &arg_vec));
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

#[derive(Debug, Default)]
struct AddWineAppPage {
    open: bool,
    err_message: String,
    c_app_name: String,
    c_file_exe: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

impl AddWineAppPage {
    fn show(&mut self, ui: &mut Ui, ctx: &egui::Context, apps: &mut Vec<Application>, conf: Conf) {
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

#[derive(Debug, Default)]
struct EditAppPage {
    open: bool,
    err_message: String,
    app_name: String,
    app_command: String,
    app_arg: String,
    app_file_exe: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

impl EditAppPage {

    fn set_current_app(&mut self, app: &mut Application) {
        self.app_name = app.name.clone();
        match app.app_type {
            AppType::Custom => {
                self.app_command = app.command.as_mut().unwrap().program.clone();
                self.app_arg = app.command.as_mut().unwrap().args[0].clone();
            }
            AppType::Wine => {
                self.app_file_exe = Some(PathBuf::from_str(&format!("{}/{}", app.exe_path.as_mut().unwrap(), app.exe_name.as_mut().unwrap())).unwrap());
            }
        }
    }

    fn show(&mut self, ui: &mut Ui, apps: Vec<Application>, app: &mut Application, conf: Conf, ctx: &egui::Context) {
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
                        if is_name_taken(apps.clone(), self.app_name.clone()) {
                            self.err_message = "/!\\ This name is already taken".to_string();
                        } else if self.app_name == String::new() {
                            self.err_message = "/!\\ Please enter a name".to_string();
                        } else if self.app_command == String::new() {
                            self.err_message = "/!\\ Please enter a command".to_string();
                        } else {
                            app.edit_from_strings(self.app_name.clone(), self.app_command.clone(), &[self.app_arg.clone()]);
                            let _ = Saver::save(apps.clone(), conf.clone());
                            self.open = false;
                        }
                    }
                    AppType::Wine => {
                        if is_name_taken(apps.clone(), self.app_name.clone()) {
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
                self.open = false;
            }
        });
    }
}

#[derive(Debug, Default)]
pub struct Launcher {
    conf: Conf,
    setting_page: SettingsPage,
    apps: Vec<Application>,
    add_app_page: AddAppPage,
    add_wine_app_page: AddWineAppPage,
    edit_app_page: EditAppPage,
    current_app_index: usize,
    app_running: bool,
    is_c_app: bool,
}

impl Launcher {
    fn new() -> Self {
        Default::default()
    }

    fn set_visuals_dark(self, cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        self
    }

    fn add_app(&mut self, app: Application) {
        self.apps.push(app);
    }

    fn remove_app(&mut self, app: Application) {
        self.apps.retain(|c_app| *c_app.name != app.name);
    }

    fn clear(&mut self) {
        self.apps.clear();
    } 

    fn load(&mut self, save: LauncherSave) {
        self.clear();
        for app in save.apps {
            self.add_app(app);
        }
        self.conf = save.conf.clone();
    }

    fn is_page_open(&self) -> bool {
        self.add_app_page.open || self.add_wine_app_page.open || self.edit_app_page.open
    }

    fn clone_current_app(&self) -> Application {
        self.apps[self.current_app_index].clone()
    }
}

fn is_name_taken(apps: Vec<Application>, app_name: String) -> bool {
    for app in apps {
        if app.name == app_name {
            return true;
        }
    }
    false
}

impl eframe::App for Launcher {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            let _ = Saver::save(self.apps.clone(), self.conf.clone());
        }

        if self.setting_page.open {
            Window::new("Settings")
                .show(ctx, |ui| {
                    self.setting_page.show(ui, ctx, self.apps.clone(), &mut self.conf);
                });
        }
        self.setting_page.open &= self.setting_page.open;

        if self.add_app_page.open {
            Window::new("Add a Custom App")
                .show(ctx, |ui| {
                    self.add_app_page.show(ui, &mut self.apps, self.conf.clone());
                });
        }
        self.add_app_page.open &= self.add_app_page.open;

        if self.add_wine_app_page.open {
            Window::new("Add a Wine App")
                .show(ctx, |ui| {
                    self.add_wine_app_page.show(ui, ctx, &mut self.apps, self.conf.clone());
                });
        }
        self.add_wine_app_page.open &= self.add_wine_app_page.open;

        if self.edit_app_page.open {
            Window::new("Edit App")
                .show(ctx, |ui| {
                    self.edit_app_page.show(ui, self.apps.clone(), &mut self.apps[self.current_app_index], self.conf.clone(), ctx);
                });
        }

        TopBottomPanel::top("top_panel0").show(ctx, |ui| {
            if self.is_page_open() {
                ui.disable();
            }
            ui.add_space(10.);
            ui.heading("cooLauncher");
            ui.add_space(10.);
            ui.horizontal(|ui| {
                if ui.button("+ Add a Custom App").clicked() {
                    self.add_app_page.err_message = String::new();
                    self.add_app_page.c_app_name = String::new();
                    self.add_app_page.c_app_command = String::new();
                    self.add_app_page.c_app_arg = String::new();
                    self.add_app_page.open = true;
                }

                if ui.button("+ Add a Wine App").clicked() {
                    self.add_wine_app_page.err_message = String::new();
                    self.add_wine_app_page.c_app_name = String::new();
                    self.add_wine_app_page.c_file_exe = None;
                    self.add_wine_app_page.open = true;
                }
                if ui.button("Settings").clicked() {
                    self.setting_page.wine_file = Some(PathBuf::from_str(&self.conf.wine_path).unwrap());
                    self.setting_page.open = true;
                }
            });
            ui.add_space(10.);
        });

        SidePanel::left("left_panel0").show(ctx, |ui| {
            if self.is_page_open() {
                ui.disable();
            }
            ui.add_space(10.);
            ScrollArea::vertical().show(ui, |ui| {
                for app in self.apps.clone().iter_mut() {
                    if ui.button(app.name.clone()).clicked() {
                        self.is_c_app = true;
                        self.current_app_index = self.apps.iter().position(|c_app| c_app.name == app.name.clone()).unwrap();
                    }
                }
            });
        });
        
        CentralPanel::default().show(ctx, |ui| {
            if self.is_page_open() {
                ui.disable();
            }
            if self.is_c_app {
                ui.heading(self.clone_current_app().name.clone());
                match self.clone_current_app().app_type.clone() {
                    AppType::Custom => ui.label("App type: Custom"),
                    AppType::Wine => ui.label("App type: Wine"),
                };
                ui.horizontal(|ui| {
                    if ui.button("Run").clicked() {
                        self.app_running = true;
                        self.clone_current_app().launch(self.conf.clone());
                        self.app_running = false;
                    }
                    if ui.button("Remove").clicked() {
                        self.is_c_app = false;
                        self.remove_app(self.clone_current_app().clone());
                        let _ = Saver::save(self.apps.clone(), self.conf.clone());
                    }
                    if ui.button("Edit").clicked() {
                        self.edit_app_page.set_current_app(&mut self.apps[self.current_app_index]);
                        self.edit_app_page.open = true;
                    }
                });
            }
        });
    }
}