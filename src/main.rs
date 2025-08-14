use coolauncher::conf::{Conf};
use coolauncher::domain::{AppType, Application};
use coolauncher::pages::alias_page::AliasPage;
use coolauncher::pages::add_app_page::AddAppPage;
use coolauncher::pages::add_wine_app_page::AddWineAppPage;
use coolauncher::pages::edit_app_page::EditAppPage;
use coolauncher::pages::gnome_desktop_page::GnomeDesktopPage;
use coolauncher::pages::setting_page::SettingsPage;
use coolauncher::saver::{Saver, LauncherSave};
use coolauncher::tools::create_main_dir;
use eframe::egui;
use egui::{CentralPanel, ScrollArea, SidePanel, TopBottomPanel, ViewportBuilder, Visuals, Window};
use tokio::task::JoinHandle;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {

    let _ = create_main_dir();

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
pub struct Launcher {
    conf: Conf,
    setting_page: SettingsPage,
    apps: Vec<Application>,
    add_app_page: AddAppPage,
    add_wine_app_page: AddWineAppPage,
    edit_app_page: EditAppPage,
    alias_page: AliasPage,
    gnome_desktop_page: GnomeDesktopPage,
    current_app_index: usize,
    app_running: bool,
    // running_apps: Vec<JoinHandle<()>>,
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

    fn get_apps_order_name(&self) -> Vec<Application> {
        let mut res = self.apps.clone();
        res.sort_by(|a, b| a.name.cmp(&b.name));
        res
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
        self.add_app_page.open || self.add_wine_app_page.open || self.edit_app_page.open || self.alias_page.open || self.gnome_desktop_page.open
    }

    fn clone_current_app(&self) -> Application {
        self.apps[self.current_app_index].clone()
    }
}

impl eframe::App for Launcher {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            // for apps in &self.running_apps {
            //     apps.abort();
            // }
            let _ = Saver::save(self.apps.clone(), self.conf.clone());
        }

        if self.setting_page.open {
            Window::new("Settings")
                .show(ctx, |ui| {
                    self.setting_page.show(ui, ctx, self.apps.clone(), &mut self.conf);
                });
        }

        if self.add_app_page.open {
            Window::new("Add a Custom App")
                .show(ctx, |ui| {
                    self.add_app_page.show(ui, &mut self.apps, self.conf.clone());
                });
        }

        if self.add_wine_app_page.open {
            Window::new("Add a Wine App")
                .show(ctx, |ui| {
                    self.add_wine_app_page.show(ui, ctx, &mut self.apps, self.conf.clone());
                });
        }

        if self.edit_app_page.open {
            Window::new("Edit App")
                .show(ctx, |ui| {
                    self.edit_app_page.show(ui, self.apps.clone(), &mut self.apps[self.current_app_index], self.conf.clone(), ctx);
                });
        }

        if self.alias_page.open {
            Window::new("Command Alias")
                .show(ctx, |ui| {
                    self.alias_page.show(ui, &mut self.apps[self.current_app_index], self.conf.clone());
                });
        }

        if self.gnome_desktop_page.open {
            Window::new("Gnome Shortcut")
                .show(ctx, |ui| {
                    self.gnome_desktop_page.show(ui, &mut self.apps[self.current_app_index], self.conf.clone());
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
                    self.setting_page.wine_file = match self.conf.is_wine_path_default() {
                        true => None,
                        false => Some(PathBuf::from(&self.conf.wine_path)),
                    };
                    self.setting_page.alias_file = match self.conf.is_alias_path_default() {
                        true => None,
                        false => Some(PathBuf::from(&self.conf.alias_path)),
                    };
                    self.setting_page.open = true;
                }
            });
            ui.add_space(10.);
        });

        SidePanel::left("left_panel0").default_width(250.0).show(ctx, |ui| {
            if self.is_page_open() {
                ui.disable();
            }
            ui.add_space(10.);
            ui.menu_button(self.conf.order.clone(), |ui| {
                if self.is_page_open() {
                    ui.disable();
                }
                if ui.button("Order by add date").clicked() {
                    self.conf.order = String::from("Order by add date");
                    ui.disable();
                }
                if ui.button("Order by name").clicked() {
                    self.conf.order = String::from("Order by name");
                    ui.disable();
                }
            });
            ui.add_space(10.);
            ScrollArea::vertical().show(ui, |ui| {
                let mut apps: Vec<Application>;
                if self.conf.order == *"Order by name" {
                    apps = self.get_apps_order_name();
                } else {
                    apps = self.apps.clone();
                }
                for app in apps.iter_mut() {
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
                let c_app = self.clone_current_app();
                ui.heading(c_app.name.clone());
                match c_app.app_type.clone() {
                    AppType::Custom => ui.label("App type: Custom"),
                    AppType::Wine => ui.label("App type: Wine"),
                };
                ui.horizontal(|ui| {
                    if ui.button("Run").clicked() {
                        self.app_running = true;

                        let mut app_to_launch = self.apps[self.current_app_index].clone();
                        let conf = self.conf.clone();

                        let handle = tokio::spawn(async move {app_to_launch.launch(conf).await});
                        // self.running_apps.push(handle);

                        self.app_running = false;
                    }
                    if ui.button("Remove").clicked() {
                        self.is_c_app = false;
                        self.remove_app(c_app.clone());
                        let _ = Saver::save(self.apps.clone(), self.conf.clone());
                    }
                    if ui.button("Edit").clicked() {
                        self.edit_app_page.set_current_app(&mut self.apps[self.current_app_index]);
                        self.edit_app_page.open = true;
                    }
                    ui.menu_button("Shortcuts", |ui| {
                        if self.is_page_open() {
                            ui.disable();
                        }
                        if ui.button("Command Alias").clicked() {
                            self.alias_page.open = true;
                        }
                        if ui.button("Gnome Shortcut").clicked() {
                            self.gnome_desktop_page.open = true;
                        }
                    });
                });
            }
        });
    }
}