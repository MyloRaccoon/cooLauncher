use std::process::{Command, Stdio};
use std::fs::File;
use std::io::Write;
use crate::saver::ApplicationSave;

const WINE_PATH: &str = "/opt/wine-ge-custom-opt/bin/wine";

#[derive(Debug, Clone)]
pub enum AppType {
	Custom,
	Wine,
}

#[derive(Debug)]
pub struct Application {
	pub name: String,
	pub command: Command,
	pub app_type: AppType,
}

impl Default for Application {
	
	fn default() -> Self {
		Self { name: "".to_string(), command: Command::new(""), app_type: AppType::Custom }
	}
}

impl Clone for Application {
	
	fn clone(&self) -> Self {
		let mut command = Command::new(self.command.get_program());
		for arg in self.command.get_args() {
			command.arg(arg);
		}
		Self { name: self.name.clone(), command, app_type: self.app_type.clone() }
	}
}


impl Application {

	pub fn from_strings(name: String, command: String, args: &[String]) -> Self {
		let mut command = Command::new(command);
		command.args(args);
		let app_type = AppType::Custom;
		Self { name, command, app_type }
	}

	pub fn wine_app(name: String, exe_path: String, exe_name: String) -> Self {
		let cd_command_str = &format!("cd {exe_path}");
		let command_str = &format!("{} {}", self::WINE_PATH, exe_name);
		let sh_name = name.replace(" ", "_");
		Self::create_bash_script(sh_name.clone(), vec![cd_command_str, command_str]);
		let sh_command = format!("./resources/bash_scripts/{}.sh", sh_name);
		let mut command = Command::new("sh");
		command.arg(sh_command);
		let app_type = AppType::Wine;
		Self { name, command, app_type }
	}

	pub fn from_save(save: ApplicationSave) -> Self {
		Self::from_strings(save.name, save.command, &save.args)
	}

	pub fn launch(&mut self) {
		let _ = Command::new("echo")
			.arg("Launching")
			.arg(self.name.clone())
			.spawn();
		let output = self.command
	        .stdout(Stdio::piped())
	        .output()
	        .expect("couldn't start the command");

	    let stdout = String::from_utf8(output.stdout).unwrap();

    	println!("{}", stdout);
		
	}

	pub fn show(&mut self, ui: &mut egui::Ui) {
		if ui.button(&self.name).clicked() {
			self.launch();
		}
	}

	fn create_bash_script(name: String, commands: Vec<&str>) {
		let file_path = format!("resources/bash_scripts/{name}.sh");
		let mut file = File::create(file_path.clone()).unwrap();
		for command in commands {
			let _ = file.write_all(command.as_bytes());
			let _ = file.write_all("\n".as_bytes());
		}
		let _ = Command::new("chmod")
			.arg("+x")
			.arg(file_path)
			.spawn();
	}
}