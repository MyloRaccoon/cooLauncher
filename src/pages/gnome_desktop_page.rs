use egui::Ui;
use egui_file::FileDialog;
use std::path::PathBuf;
use crate::{conf::Conf, domain::Application, tools::{create_gnome_desktop, remove_gnome_desktop}};

#[derive(Debug, Default)]
pub struct GnomeDesktopPage {
	pub open: bool,
	err_message: String,
	pub img_file: Option<PathBuf>,
    pub open_file_dialog: Option<FileDialog>
}

impl GnomeDesktopPage {
    pub fn show(&mut self, ui: &mut Ui, app: &mut Application, conf: Conf) {

    	ui.heading(app.name.clone());
    	ui.label(self.err_message.clone());

    	if app.gnome_desktop_exists() {
    		ui.label("a desktop file already exist for this app's name");

    		ui.horizontal(|ui| {
    			if ui.button("Remove it").clicked() {
    				if remove_gnome_desktop(app.name.clone()).is_err() {
    					self.err_message = String::from("Error: couln't remove desktop file");
    				} else {
    					self.open = false;
    				}
    			}
    			if ui.button("Close").clicked() {
		    		self.open = false;
		    	}
    		});
    	} else {
    		ui.horizontal(|ui| {
	    		ui.label("Icon: ");
	    		ui.label(
	    			match &self.img_file {
	    				Some(path) => path.to_str().unwrap(),
	    				None => "default"
	    			}
	    		);
	    		ui.button("import image");
	    	});

	    	ui.horizontal(|ui| {
	    		if ui.button("Create").clicked() {
	    			let res = create_gnome_desktop(app.name.clone(), Some("coolauncher".to_string()), app.create_script(conf), false);
		    		if res.is_err() {
		    			self.err_message = String::from("Error, couln't create gnome desktop");
		    		} else {
		    			self.open = false;
		    		}
		    	}

		    	if ui.button("Close").clicked() {
		    		self.open = false;
		    	}
	    	});
    	}
    }
}