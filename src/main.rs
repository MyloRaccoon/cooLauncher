use coolauncher::conf::Conf;
use coolauncher::domain::Application;
use coolauncher::saver::{Saver, LauncherSave};
use eframe::egui;
use egui::{CentralPanel, ScrollArea, SidePanel, TopBottomPanel, ViewportBuilder, Visuals, Window};
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
struct AddAppPage {
    open: bool,
    c_app_name: String,
    c_app_command: String,
    c_app_arg: String,
}

#[derive(Debug, Default)]
struct AddWineAppPage {
    open: bool,
    c_app_name: String,
    c_file_exe: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

#[derive(Debug, Default)]
struct SettingsPage {
    open: bool,
    wine_file: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

#[derive(Debug, Default)]
pub struct Launcher {
    conf: Conf,
    setting_page: SettingsPage,
    apps: Vec<Application>,
    add_app_page: AddAppPage,
    add_wine_app_page: AddWineAppPage,
    current_app: Application,
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
        self.add_app_page.open || self.add_wine_app_page.open
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

        Window::new("Settings")
            .open(&mut self.setting_page.open)
            .show(ctx, |ui| {
                ui.heading("Settings");
                ui.horizontal(|ui| {
                    ui.label("Wine Path: ");
                    match self.setting_page.wine_file.clone() {
                        Some(path_buf) => ui.label(path_buf.as_path().to_str().unwrap()),
                        None => ui.label("Please set a wine file"),
                    };
                    if ui.button("Choose a file").clicked() {
                        let mut dialog = FileDialog::open_file(self.setting_page.wine_file.clone());
                        dialog.open();
                        self.setting_page.open_file_dialog = Some(dialog);
                    }
                    if let Some(dialog) = &mut self.setting_page.open_file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.setting_page.wine_file = Some(file.to_path_buf());
                            }
                        }
                    }
                });
                if ui.button("Save Changes").clicked() {
                    self.conf.wine_path = self.setting_page.wine_file.clone().unwrap().display().to_string();
                    let _ = Saver::save(self.apps.clone(), self.conf.clone());
                }
            });

        Window::new("Add a Custom App")
            .open(&mut self.add_app_page.open)
            .show(ctx, |ui| {
                ui.heading("Manual command");
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.add_space(250.);
                    ui.label("Command: ");
                    ui.add_space(223.);
                    ui.label("Argument: ");
                });

                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.add_app_page.c_app_name));
                    ui.add(egui::TextEdit::singleline(&mut self.add_app_page.c_app_command));
                    ui.add(egui::TextEdit::singleline(&mut self.add_app_page.c_app_arg));
                });
                let mut arg_vec = Vec::new();
                if self.add_app_page.c_app_arg != String::default() {
                    arg_vec.push(self.add_app_page.c_app_arg.clone());
                }
                if ui.button("+ Add application").clicked() {
                    if is_name_taken(self.apps.clone(), self.add_app_page.c_app_name.clone()) {
                        println!("name taken");
                    } else {
                        self.apps.push(Application::from_strings(self.add_app_page.c_app_name.clone(), self.add_app_page.c_app_command.clone(), &arg_vec));
                        let _ = Saver::save(self.apps.clone(), self.conf.clone());
                        // CLOSE WINDOW
                    }
                }
            });

        Window::new("Add a Wine App")
            .open(&mut self.add_wine_app_page.open)
            .show(ctx, |ui| {
                ui.heading("Wine Application");
                if self.conf.is_wine_path_default() {
                    ui.label("/!\\ Please set your wine path in your settings");
                } else {
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.add_space(250.);
                        ui.label(".exe file:");
                    });

                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.add_wine_app_page.c_app_name));
                        match self.add_wine_app_page.c_file_exe.clone() {
                            Some(path_buf) => ui.label(path_buf.as_path().to_str().unwrap()),
                            None => ui.label("Please choose a .exe file"),
                        };
                        
                        if ui.button("Choose a .exe").clicked() {
                            let filter = Box::new({
                                let ext = Some(OsStr::new("exe"));
                                move |path: &Path| -> bool { path.extension() == ext }
                            });
                            let mut dialog = FileDialog::open_file(self.add_wine_app_page.c_file_exe.clone()).show_files_filter(filter);
                            dialog.open();
                            self.add_wine_app_page.open_file_dialog = Some(dialog);
                        }

                        if let Some(dialog) = &mut self.add_wine_app_page.open_file_dialog {
                            if dialog.show(ctx).selected() {
                                if let Some(file) = dialog.path() {
                                    self.add_wine_app_page.c_file_exe = Some(file.to_path_buf());
                                }
                            }
                        }
                    });
                    if ui.button("+ Add application").clicked() {
                        let app_name = self.add_wine_app_page.c_app_name.clone();
                        let file_path_buf = self.add_wine_app_page.c_file_exe.clone().unwrap();
                        let file_path = file_path_buf.as_path();
                        let exe_name = file_path.file_name().unwrap();
                        let dir_path = file_path.parent().unwrap();
                        self.apps.push(
                            Application::wine_app(
                                app_name,
                                dir_path.to_str().unwrap().to_string(),
                                exe_name.to_str().unwrap().to_string()
                            )
                        );
                        let _ = Saver::save(self.apps.clone(), self.conf.clone());
                        // CLOSE WINDOW
                    }
                }
            });

        TopBottomPanel::top("top_panel0").show(ctx, |ui| {
            if self.is_page_open() {
                ui.disable();
            }
            ui.add_space(10.);
            ui.heading("cooLauncher");
            ui.add_space(10.);
            ui.horizontal(|ui| {
                if ui.button("+ Add a Custom App").clicked() {
                    self.add_app_page.c_app_name = String::new();
                    self.add_app_page.c_app_command = String::new();
                    self.add_app_page.c_app_arg = String::new();
                    self.add_app_page.open = true;
                }

                if ui.button("+ Add a Wine App").clicked() {
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
                for app in self.apps.iter_mut() {
                    if ui.button(app.name.clone()).clicked() {
                        self.is_c_app = true;
                        self.current_app = app.clone();
                    }
                }
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            if self.is_page_open() {
                ui.disable();
            }
            if self.is_c_app {
                ui.heading(self.current_app.name.clone());
                ui.horizontal(|ui| {
                    if ui.button("Run").clicked() {
                        self.app_running = true;
                        self.current_app.launch(self.conf.clone());
                        self.app_running = false;
                    }
                    if ui.button("Remove").clicked() {
                        self.is_c_app = false;
                        self.remove_app(self.current_app.clone());
                   }
                });
            }
        });
    }
}