use egui::Ui;

use crate::{conf::Conf, domain::Application, tools::is_alias_taken};

#[derive(Debug, Default)]
pub struct AliasPage {
	pub open: bool,
	err_message: String,
	alias: String,
}

impl AliasPage {
    pub fn show(&mut self, ui: &mut Ui, app: &mut Application, conf: Conf) {
    	if conf.is_alias_path_default() {
    		ui.heading("Please set your alias path in settings.");
    		if ui.button("Close").clicked() {
    			self.open = false;
    		}
    	} else {
	    	ui.heading(app.name.clone());
    		for alias in app.alias.clone() {
    			ui.horizontal(|ui| {
    				ui.label(alias.clone());
    				if ui.button("delete").clicked() {
    					app.delete_alias(alias.clone(), conf.clone());
    				}
    			});
    		}
	    	ui.label(self.err_message.clone());
	    	ui.label("alias:");
	    	ui.add(egui::TextEdit::singleline(&mut self.alias));
	    	ui.horizontal(|ui| {
		    	if ui.button("+ Add alias").clicked() {
		    		if is_alias_taken(self.alias.clone(), conf.alias_path.clone()) {
		    			self.err_message = "/!\\ this alias is already taken".to_string();
		    		} else {
		    			app.create_alias(self.alias.clone(), conf);
		    			self.err_message = String::new();
		    			self.alias = String::new();
		    		}
		    	}
		    	if ui.button("close").clicked() {
		    		self.open = false;
		    	}
	    	});
    	}
    }
}