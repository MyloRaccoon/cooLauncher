use coolauncher::domain::Application;
use coolauncher::saver::{Saver, LauncherSave};
use eframe::egui;
use egui::{CentralPanel, ScrollArea, SidePanel, TopBottomPanel, ViewportBuilder, Visuals, Window};

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

#[derive(Default)]
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
    c_app_exe_name: String,
    c_app_exe_path: String,
}

#[derive(Default)]
pub struct Launcher {
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
        for app_save in save.apps {
            self.add_app(Application::from_save(app_save));
        }
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
            let _ = Saver::save(&mut self.apps);
        }

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
                        // CLOSE WINDOW
                    }
                }
            });

        Window::new("Add a Wine App")
            .open(&mut self.add_wine_app_page.open)
            .show(ctx, |ui| {
                ui.heading("Wine Application");
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.add_space(250.);
                    ui.label(".exe name: ");
                    ui.add_space(223.);
                    ui.label("path/to/file: ");
                });

                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.add_wine_app_page.c_app_name));
                    ui.add(egui::TextEdit::singleline(&mut self.add_wine_app_page.c_app_exe_name));
                    ui.add(egui::TextEdit::singleline(&mut self.add_wine_app_page.c_app_exe_path));
                });
                if ui.button("+ Add application").clicked() {
                    self.apps.push(
                        Application::wine_app(
                            self.add_wine_app_page.c_app_name.clone(), 
                            self.add_wine_app_page.c_app_exe_path.clone(),
                            self.add_wine_app_page.c_app_exe_name.clone() 
                        )
                    );
                    // CLOSE WINDOW
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
                    self.add_app_page.open = true;
                }

                if ui.button("+ Add a Wine App").clicked() {
                    self.add_wine_app_page.open = true;
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
                        self.current_app.launch();
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