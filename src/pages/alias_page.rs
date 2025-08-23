use egui::Ui;

use crate::{conf::Conf, domain::Application, alias_manager::alias_exists};

#[derive(Debug, Default)]
pub struct AliasPage {
	pub is_open: bool,
	err_message: String,
	alias: String,
}

impl AliasPage {
	pub fn open(&mut self) {
		self.is_open = true;
		self.err_message = String::default();
		self.alias = String::default();
	}
	
    pub fn show(&mut self, ui: &mut Ui, app: &mut Application, conf: Conf) {
    	if conf.is_alias_path_default() {
    		ui.heading("Please set your alias path in settings.");
    		if ui.button("Close").clicked() {
    			self.is_open = false;
    		}
    	} else {
	    	ui.heading(app.name.clone());
    		for alias in app.alias.clone() {
    			ui.horizontal(|ui| {
    				ui.label(alias.clone());
    				if ui.button("delete").clicked() {
    					if let Err(e) = app.delete_alias(alias.clone(), conf.clone()) {
    						self.err_message = e.to_string();
    					} else {
    						self.err_message = String::default();
    					}
    				}
    			});
    		}
	    	ui.label(self.err_message.clone());
	    	ui.label("alias:");
	    	ui.add(egui::TextEdit::singleline(&mut self.alias));
	    	ui.horizontal(|ui| {
		    	if ui.button("+ Add alias").clicked() {
		    		if alias_exists(self.alias.clone()) {
		    			self.err_message = "/!\\ this alias is already taken".to_string();
		    		} else {
		    			if let Err(e) = app.create_alias(self.alias.clone(), conf) {
		    				self.err_message = e.to_string();
		    			} else {
		    				self.err_message = String::default();
		    			}
		    			self.alias = String::default();
		    		}
		    	}
		    	if ui.button("close").clicked() {
		    		self.is_open = false;
		    	}
	    	});
    	}
    }
}