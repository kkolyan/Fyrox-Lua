use std::fs;

use rustdoc_types::Crate;

fn main() {
	let data = fs::read("C:\\dev\\rust\\fyrox_lua\\engine\\target\\doc\\fyrox_core.json").unwrap();
	let model = serde_json::from_slice::<Crate>(&data).unwrap();
	println!("{:?}", model);
}