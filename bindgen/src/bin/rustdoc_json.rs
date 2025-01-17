fn main() {
	let json_path = rustdoc_json::Builder::default()
    .toolchain("nightly")
    .manifest_path("engine/fyrox-core/Cargo.toml")
    .build()
    .unwrap();

	// Prints `Wrote rustdoc JSON to "/Users/martin/src/project/target/doc/project.json"`
	println!("Wrote rustdoc JSON to {:?}", &json_path);
}