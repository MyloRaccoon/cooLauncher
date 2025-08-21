use egui::Ui;

use crate::{conf::Conf, domain::Application, launcher::is_app_name_taken, saver::Saver};

#[derive(Debug, Default)]
pub struct AddAppPage {
    pub is_open: bool,
    err_message: String,
    c_app_name: String,
    c_app_command: String,
    c_app_arg: String,
}

impl AddAppPage {
    pub fn open(&mut self) {
        self.err_message = String::new();
        self.c_app_name = String::new();
        self.c_app_command = String::new();
        self.c_app_arg = String::new();
        self.is_open = true;
    }

    pub fn show(&mut self, ui: &mut Ui, apps: &mut Vec<Application>, conf: Conf) {
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

                if is_app_name_taken(apps.clone(), self.c_app_name.clone()) {
                    self.err_message = "/!\\ This name is already taken".to_string();
                } else if self.c_app_name == String::new() {
                    self.err_message = "/!\\ Please enter a name".to_string();
                } else if self.c_app_command == String::new() {
                    self.err_message = "/!\\ Please enter a command".to_string();
                } else {
                    apps.push(Application::from_strings(self.c_app_name.clone(), self.c_app_command.clone(), &arg_vec));
                    let _ = Saver::save(apps.clone(), conf.clone());
                    self.is_open = false;
                }
            }
            if ui.button("Cancel").clicked() {
                self.is_open = false;
            }
        });
    }
}