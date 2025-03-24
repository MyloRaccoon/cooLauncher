use egui::Ui;

use crate::{conf::Conf, domain::Application, tools::is_alias_taken};

#[derive(Debug, Default)]
pub struct GnomeShortcutPage {
	pub open: bool,
	err_message: String,
}

impl GnomeShortcutPage {
    pub fn show(&mut self, ui: &mut Ui, app: &mut Application, conf: Conf) {
    	ui.heading(app.name.clone());
    	if ui.button("Close").clicked() {
    		self.open = false;
    	}
    }
}