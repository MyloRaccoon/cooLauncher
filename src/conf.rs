use serde::{Serialize, Deserialize};
use home::home_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conf {
	pub wine_path: String,
	pub alias_path: String,
	pub gnome_desktop_path: String,
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
		let mut gnome_desktop_path = home_dir().unwrap();
		gnome_desktop_path.push(".local/share/applications");
		Self {
			wine_path: String::default(),
			alias_path: String::default(),
			gnome_desktop_path: String::from(gnome_desktop_path.to_str().unwrap()),
			order: String::from("Order by add date")
		}
	}

}