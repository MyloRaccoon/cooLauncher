use crate::conf::Conf;
use crate::domain::Application;
use std::fs::File;
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use serde_json::from_slice;

const FILE_PATH: &str = "save.json";

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
