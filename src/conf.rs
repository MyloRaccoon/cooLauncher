use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Conf {
	pub wine_path: String,
	pub alias_path: String,
}

impl Conf {
	pub fn is_wine_path_default(&self) -> bool {
		self.wine_path == String::default()
	}

	pub fn is_alias_path_default(&self) -> bool {
		self.alias_path == String::default()
	}
}