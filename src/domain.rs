use std::process::{Command, Output};
use serde::{Serialize, Deserialize};
use std::io::{self, Write};

const WINE_PATH: &str = "/opt/wine-ge-custom-opt/bin/wine";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoolCommand {
	pub program: String,
	pub args: Vec<String>,
}

impl CoolCommand {
    	
    pub fn new(program: String, args: &[String]) -> Self {
    	let mut args_vec: Vec<String> = Vec::new();
    	for arg in args {
    		args_vec.push(arg.clone());
    	}
    	Self { program: program.to_string(), args: args_vec }
    }

    pub fn spawn(&self) -> Output {
    	let mut command = Command::new(self.program.clone());
    	for arg in self.args.clone() {
    		command.arg(arg);
    	}
    	command.output().expect("idk fuck u")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppType {
	Custom,
	Wine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
	pub name: String,
	pub app_type: AppType,
	pub command: Option<CoolCommand>,
	pub exe_path: Option<String>,
	pub exe_name: Option<String>,
}

pub struct WineApp {
	pub name: String,
	pub exe_path: String,
	pub exe_name: String,
}

impl Default for Application {
	
	fn default() -> Self {
		Self { 
			name: "".to_string(), 
			app_type: AppType::Custom,
			command: Some(CoolCommand::new("".to_string(), &[])), 
			exe_path: None,
			exe_name: None,
		}
	}
}


impl Application {

	pub fn from_strings(name: String, command: String, args: &[String]) -> Self {
		let command = CoolCommand::new(command, args);
		let app_type = AppType::Custom;
		Self { 
			name,
			app_type,
			command: Some(command),
			exe_path: None,
			exe_name: None,
		}
	}

	pub fn wine_app(name: String, exe_path: String, exe_name: String) -> Self {
		Self {
			name,
			app_type: AppType::Wine,
			command: None,
			exe_path: Some(exe_path),
			exe_name: Some(exe_name),
		}
	}

	pub fn launch(&mut self) {
		println!("Launching {}", self.name.clone());
		let output: Output = match self.app_type {
			AppType::Custom => {
				self.command.clone().expect("couldn't spawn command").spawn()
			}
			AppType::Wine => {
				Command::new(self::WINE_PATH)
					.arg(self.exe_name.clone().expect("couldn't get exe"))
					.current_dir(self.exe_path.clone().expect("couldn't get directory"))
					.output().expect("Error: couldn't launch this wine application")
			}
		};
		let _ = io::stdout().write_all(&output.stdout);
        let _ = io::stderr().write_all(&output.stderr);
	}

	pub fn show(&mut self, ui: &mut egui::Ui) {
		if ui.button(&self.name).clicked() {
			self.launch();
		}
	}
}