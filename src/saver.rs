use crate::domain::Application;
use std::fs::File;
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use serde_json::from_slice;

const FILE_PATH: &str = "save.json";

#[derive(Serialize, Deserialize)]
pub struct ApplicationSave {
	pub name: String,
	pub command: String,
	pub args: Vec<String>,
}

impl ApplicationSave {
	fn from_application(app: &Application) -> Self {
		let name = app.name.clone();
		let command = app.command.get_program().to_str().unwrap().to_string();
		let mut args = Vec::new();
		for arg in app.command.get_args() {
			args.push(arg.to_str().unwrap().to_string());
		}
		Self { name, command, args }
	}
}

#[derive(Serialize, Deserialize, Default)]
pub struct LauncherSave {
	pub apps: Vec<ApplicationSave>
}

impl LauncherSave {

	fn from_vec(apps: &mut [Application]) -> Self {
		let mut app_saves = Vec::new();
		for app in apps.iter_mut() {
			app_saves.push(ApplicationSave::from_application(app));
		}
		Self { apps: app_saves }
	}
}

pub struct Saver;

impl Saver {

	pub fn save(apps: &mut [Application]) -> std::result::Result<(), std::io::Error> {
		let launcher_save = LauncherSave::from_vec(apps);
		let mut file = File::create(self::FILE_PATH).unwrap();
		let data = serde_json::to_string(&launcher_save).unwrap();
		file.write_all(data.as_bytes())
	}

	pub fn load() -> LauncherSave {
		match File::open(self::FILE_PATH) {
			Ok(mut file) => {
				let mut data_bytes = vec![];
				let _ = file.read_to_end(&mut data_bytes);
				let launcher_save = from_slice(&data_bytes);
				launcher_save.unwrap()
			}
			Err(_) => {
				LauncherSave::default()
			}
		}
	}
}
