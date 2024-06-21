use std::{fs, path::Path};

use super::model::Parser;

impl Parser {
	pub fn parse_dir(&mut self, dir: impl AsRef<Path>) {
		for item in fs::read_dir(dir).unwrap().flatten() {
			if !item.file_type().unwrap().is_file() {
				continue;
			}
			let file = item.file_name();
			let file = file.to_string_lossy();
			if !file.starts_with("fyrox-") && file.ends_with(".rs") {
				continue;
			}
			let crate_name = file[0..file.len() - ".rs".len()]
				.replace('-', "_")
				.to_string();
			let a_crate = syn::parse_file(item.path().to_string_lossy().to_string().as_str()).unwrap();
			self.parse_crate(&a_crate, crate_name.as_str());
		}
	}
}