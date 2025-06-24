use crate::conf::Conf;
use crate::domain::Application;
use crate::tools::get_main_dir;
use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json::{from_slice, Error};

fn get_file_path() -> String {
	let mut path = PathBuf::from(&get_main_dir());
	path.push("cooLauncher_save");
	path.set_extension("json");
	path.to_str().unwrap().to_string()
}

#[derive(Serialize, Deserialize, Default)]
pub struct LauncherSave {
	pub apps: Vec<Application>,
	pub conf: Conf,
}

impl LauncherSave {
	fn new(apps: Vec<Application>, conf: Conf) -> Self {
		Self { 
			apps, 
			conf,
		}
	}
}

pub struct Saver;

impl Saver {
	
	pub fn save(apps: Vec<Application>, conf: Conf) -> std::result::Result<(), std::io::Error> {
		let launcher_save = LauncherSave::new(apps, conf);
		let mut file = File::create(get_file_path()).unwrap();
		let data = serde_json::to_string(&launcher_save).unwrap();
		file.write_all(data.as_bytes())
	}

	pub fn load() -> LauncherSave {
		match File::open(get_file_path()) {
			Ok(mut file) => {
				let mut data_bytes = vec![];
				let _ = file.read_to_end(&mut data_bytes);
				let launcher_save: Result<LauncherSave, Error> = from_slice(&data_bytes);
				launcher_save.unwrap_or_default()
			}
			Err(_) => {
				LauncherSave::default()
			}
		}
	}

}
