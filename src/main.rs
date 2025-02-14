use coolauncher::domain::Application;
use coolauncher::saver::{Saver, LauncherSave};
use eframe::egui;
use egui::{CentralPanel, ScrollArea, TopBottomPanel, ViewportBuilder, Visuals};

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
pub struct Launcher {
    apps: Vec<Application>,
    c_app_name: String,
    c_app_command: String,
    c_app_arg: String,
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

    fn clear(&mut self) {
        self.apps.clear();
    } 

    fn load(&mut self, save: LauncherSave) {
        self.clear();
        for app_save in save.apps {
            self.add_app(Application::from_save(app_save));
        }
    }
}

impl eframe::App for Launcher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            let _ = Saver::save(&mut self.apps);
        }
        TopBottomPanel::top("top_panel0").show(ctx, |ui| {
            ui.heading("cooLauncher");

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
                let mut arg_vec = Vec::new();
                if self.c_app_arg != String::default() {
                    arg_vec.push(self.c_app_arg.clone());
                }
                if ui.button("+ Add application").clicked() {
                    self.add_app(Application::from_strings(self.c_app_name.clone(), self.c_app_command.clone(), &arg_vec));
                }
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for app in self.apps.iter_mut() {
                    app.show(ui)
                }
            });
        });
    }
}

// #[derive(Debug)]
// struct PadButton {
//     command: string,
//     image: string,
// }

// impl PadButton {
//     fn add_pad_button(&self, ui: &mut egui::Ui) {

//     }
// }

// fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
//     // Put the buttons and label on the same row:
//     ui.horizontal(|ui| {
//         if ui.button("−").clicked() {
//             *counter -= 1;
//         }
//         ui.label(counter.to_string());
//         if ui.button("+").clicked() {
//             *counter += 1;
//         }
//     });
// }