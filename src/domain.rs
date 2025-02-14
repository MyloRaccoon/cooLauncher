use std::process::{Command, Child};
use std::io::Result;

use crate::saver::ApplicationSave;

pub struct Application {
	pub name: String,
	pub command: Command,
}

impl Application {

	pub fn new(name: String, command: Command) -> Self {
		Self { name, command }
	}

	pub fn from_strings(name: String, command: String, args: &[String]) -> Self {
		let mut command = Command::new(command);
		command.args(args);
		Self { name, command }
	}

	pub fn from_save(save: ApplicationSave) -> Self {
		Self::from_strings(save.name, save.command, &save.args)
	}

	pub fn launch(&mut self) -> Result<Child> {
		self.command.spawn()
	}

	pub fn show(&mut self, ui: &mut egui::Ui) {
		if ui.button(&self.name).clicked() {
			let _ = self.launch();
		}
	}
}