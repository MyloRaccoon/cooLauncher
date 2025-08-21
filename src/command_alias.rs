use crate::domain::{Application, AppType};
use crate::conf::Conf;
use crate::tools::delete_line;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read};
use regex::Regex;

pub fn is_alias_taken(alias: String, file: String) -> bool {
    let mut file = File::open(file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let regex = Regex::new(format!("alias *{}[ =]", alias.clone()).as_str()).unwrap();
    regex.is_match(&content)
}

impl Application {
	pub fn get_alias_line(&self, alias: String, conf: Conf) -> String {
		match self.app_type.clone() {
			AppType::Custom => format!("\nalias {}='{}'", alias.clone(), self.command.as_ref().unwrap().get_string()),
    		AppType::Wine => format!("\nalias {}='{}'", alias.clone(), self.wine_command.as_ref().unwrap().get_string(conf)),
		}
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

	pub fn delete_alias(&mut self, alias: String, conf: Conf) {
		self.alias.retain(|c_alias| c_alias != &alias);
		delete_line(self.get_alias_line(alias, conf.clone()), conf.clone().alias_path).expect("couln't delete line");
	}
}