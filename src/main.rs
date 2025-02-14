use std::process::Command;
use coolauncher::{domain, saver};
use domain::Application;
use saver::{Saver, LauncherSave};
use eframe::egui;
use egui::{CentralPanel, ScrollArea, TopBottomPanel, ViewportBuilder, Visuals};

fn main() -> Result<(), eframe::Error> {
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
            Ok(Box::new(Launcher::new(cc)))
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
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        Default::default()
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
        TopBottomPanel::top("top_panel0").show(ctx, |ui| {
            ui.heading("cooLauncher");
            
            ui.horizontal(|ui| {
                if ui.button("save").clicked() {
                    let _ = Saver::save(&mut self.apps);
                }
                if ui.button("load").clicked() {
                    self.load(Saver::load());
                }
            });

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

                if ui.button("+ Add application").clicked() {
                    self.add_app(Application::from_strings(self.c_app_name.clone(), self.c_app_command.clone(), &[self.c_app_arg.clone()]));
                }
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for app in self.apps.iter_mut() {
                    app.show(ui)
                }
            });
            // ui.ctx().request_repaint();
            // ui.label("press/hold/release A");
            // ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            //     ui.label(&self.text);
            // });
            // if ctx.input(|i| i.key_pressed(Key::C)) {
            //     self.text.clear();
            // } else if ctx.input(|i| i.key_pressed(Key::A)) {
            //     self.text.push_str("\nPressed");
            // } else if ctx.input(|i| i.key_down(Key::A)) {
            //     self.text.push_str("\nDown");
            //     // update is not called every frame but only when repaint is needed
            //     // we can force it with this: 
                
            // } else if ctx.input(|i| i.key_released(Key::A)) {
            //     self.text.push_str("\nReleased");
            // }
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