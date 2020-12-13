use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn get_input(day: &str, part: &str, use_test: bool) -> Vec<String> {
    let path = PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("input")
        .join(format!(
            "day{}.{}.{}",
            day,
            part,
            if use_test { "test" } else { "" }
        ));

    let mut file = match File::open(path.as_path()) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open file {}: {}", path.display(), why),
    };

    let mut input_string = String::new();
    match file.read_to_string(&mut input_string) {
        Ok(_) => input_string.lines().map(|s| String::from(s)).collect(),
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
    }
}
