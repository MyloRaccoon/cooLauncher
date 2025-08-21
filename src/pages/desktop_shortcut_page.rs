use egui::Ui;
use crate::{conf::Conf, domain::Application, tools::{create_desktop_shortcut, remove_desktop_shortcut}};

#[derive(Debug, Default)]
pub struct DesktopShortcutPage {
	pub is_open: bool,
	err_message: String,
	icon_name: String,
}

impl DesktopShortcutPage {
	pub fn open(&mut self) {
		self.is_open = true;
	}

    pub fn show(&mut self, ui: &mut Ui, app: &mut Application, conf: Conf) {

    	ui.heading(app.name.clone());
    	ui.label(self.err_message.clone());

    	if app.desktop_shortcut_exists(conf.clone()) {
    		ui.label("a desktop file already exist for this app's name");

    		ui.horizontal(|ui| {
    			if ui.button("Remove it").clicked() {
    				if remove_desktop_shortcut(app.name.clone(), conf.clone()).is_err() {
    					self.err_message = String::from("Error: couln't remove desktop file");
    				} else {
    					self.is_open = false;
    				}
    			}
    			if ui.button("Close").clicked() {
		    		self.is_open = false;
		    	}
    		});
    	} else {
    		ui.horizontal(|ui| {
	    		ui.label("Icon: ");
	    		ui.add(egui::TextEdit::singleline(&mut self.icon_name));
	    	});

	    	ui.horizontal(|ui| {
	    		if ui.button("Create").clicked() {
	    			let icon = match !self.icon_name.is_empty() {
	    				true => Some(self.icon_name.clone()),
	    				false => None,
	    			};
	    			let res = create_desktop_shortcut(app.name.clone(), icon, app.create_script(conf.clone()), false, conf.clone());
		    		if res.is_err() {
		    			self.err_message = String::from("Error, couln't create gnome desktop");
		    		} else {
		    			self.is_open = false;
		    		}
		    	}

		    	if ui.button("Close").clicked() {
		    		self.is_open = false;
		    	}
	    	});
    	}
    }
}