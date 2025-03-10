use egui::Ui;

use crate::{conf::Conf, domain::Application, saver::Saver, tools::is_name_taken};

#[derive(Debug, Default)]
pub struct AddAppPage {
    pub open: bool,
    pub err_message: String,
    pub c_app_name: String,
    pub c_app_command: String,
    pub c_app_arg: String,
}

impl AddAppPage {
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