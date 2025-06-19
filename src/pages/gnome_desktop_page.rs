use egui::Ui;

use crate::{conf::Conf, domain::Application, tools::create_gnome_desktop};

#[derive(Debug, Default)]
pub struct GnomeDesktopPage {
	pub open: bool,
	err_message: String,
}

impl GnomeDesktopPage {
    pub fn show(&mut self, ui: &mut Ui, app: &mut Application, conf: Conf) {
    	ui.heading(app.name.clone());
    	ui.label(self.err_message.clone());

    	if ui.button("Create").clicked() {
    		match create_gnome_desktop(app.name.clone(), Some("coolauncher".to_string()), app.create_script(conf), false) {
    			Ok(_) => println!("ok"),
    			Err(e) => println!("{e}"),
    		};
    		self.open = false;
    	}

    	if ui.button("Close").clicked() {
    		self.open = false;
    	}
    }
}