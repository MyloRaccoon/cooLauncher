use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conf {
	pub wine_path: String,
	pub alias_path: String,
	pub order: String,
}

impl Conf {
	pub fn is_wine_path_default(&self) -> bool {
		self.wine_path == String::default()
	}

	pub fn is_alias_path_default(&self) -> bool {
		self.alias_path == String::default()
	}
}

impl Default for Conf {

	fn default() -> Self {
		Self {
			wine_path: String::default(),
			alias_path: String::default(),
			order: String::from("Order by add date")
		}
	}

}