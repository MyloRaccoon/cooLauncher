use std::process::{Command, Output};
use std::fs::OpenOptions;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use crate::conf::Conf;
use crate::tools::delete_line;

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

    pub fn get_string(&self) -> String {
    	let mut res = self.program.clone();
    	for arg in self.args.clone() {
    		res += " ";
    		res += &arg.clone();
    	}
    	res
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WineCommand {
	pub exe_path: String,
	pub exe_name: String,
}

impl WineCommand {
	pub fn new(exe_path: String, exe_name: String) -> Self {
		Self { exe_path, exe_name }
	}
	pub fn spawn(&self, conf: Conf) -> std::process::Output {
		Command::new(conf.wine_path)
			.arg(self.exe_name.clone())
			.current_dir(self.exe_path.clone())
			.output().expect("Error: couldn't launch this wine application")
	}

	pub fn get_string(&self, conf: Conf) -> String {
		format!("cd \"{}\" && {} \"{}\"", self.exe_path, conf.wine_path, self.exe_name)
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
	pub alias: Vec<String>,
	pub command: Option<CoolCommand>,
	pub wine_command: Option<WineCommand>,
	pub exe_path: Option<String>,
	pub exe_name: Option<String>,
}

impl Default for Application {
	
	fn default() -> Self {
		Self { 
			name: "".to_string(), 
			app_type: AppType::Custom,
			alias: vec![],
			command: Some(CoolCommand::new("".to_string(), &[])),
			wine_command: None,
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
			alias: vec![],
			command: Some(command),
			wine_command: None,
			exe_path: None,
			exe_name: None,
		}
	}

	pub fn edit_from_strings(&mut self, name: String, command: String, args: &[String]) {
		self.name = name;
		self.command = Some(CoolCommand::new(command, args));
	}

	pub fn wine_app(name: String, exe_path: String, exe_name: String) -> Self {
		Self {
			name,
			app_type: AppType::Wine,
			alias: vec![],
			command: None,
			wine_command: Some(WineCommand::new(exe_path.clone(), exe_name.clone())),
			exe_path: Some(exe_path),
			exe_name: Some(exe_name),
		}
	}

	pub fn launch(&mut self, conf: Conf) {
		println!("Launching {}", self.name.clone());
		let output: Output = match self.app_type {
			AppType::Custom => {
				self.command.clone().expect("couldn't spawn command").spawn()
			}
			AppType::Wine => {
				self.wine_command.clone().expect("couldn't spawn command").spawn(conf)
			}
		};
		let _ = io::stdout().write_all(&output.stdout);
        let _ = io::stderr().write_all(&output.stderr);
	}

	pub fn create_alias(&mut self, alias: String, conf: Conf) {
		let line = self.get_alias_line(alias.clone(), conf.clone());

		let mut file = OpenOptions::new()
			.append(true)
			.open(conf.clone().alias_path)
			.expect("cant open file");

		file.write_all(line.as_bytes()).expect("write failed");
		self.alias.push(alias);
	}

	pub fn get_alias_line(&self, alias: String, conf: Conf) -> String {
		match self.app_type.clone() {
			AppType::Custom => format!("\nalias {}='{}'", alias.clone(), self.command.as_ref().unwrap().get_string()),
    		AppType::Wine => format!("\nalias {}='{}'", alias.clone(), self.wine_command.as_ref().unwrap().get_string(conf)),
		}
	}

	pub fn delete_alias(&mut self, alias: String, conf: Conf) {
		self.alias.retain(|c_alias| c_alias != &alias);
		delete_line(self.get_alias_line(alias, conf.clone()), conf.clone().alias_path).expect("couln't delete line");
	}
}