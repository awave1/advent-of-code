use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn get_string(day: &str, use_test: bool) -> String {
	let path = PathBuf::new()
		.join(env!("CARGO_MANIFEST_DIR"))
		.join("src")
		.join(format!("day{}", day))
		.join(if use_test { "test.txt" } else { "input.txt" });

	let mut input_file = match File::open(path.as_path()) {
		Ok(file) => file,
		Err(why) => panic!("Couldn't open file at path {}: {}", path.display(), why),
	};

	let mut input_string = String::new();
	match input_file.read_to_string(&mut input_string) {
		Ok(_) => input_string,
		Err(why) => panic!("Failed to read from file {}: {}", path.display(), why),
	}
}

pub fn get_input(day: &str, use_test: bool) -> Vec<String> {
	get_string(day, use_test)
		.lines()
		.map(String::from)
		.collect()
}
