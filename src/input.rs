use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn get_string(day: &str, part: &str, use_test: bool) -> String {
    let path = PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("input")
        .join(format!(
            "day{}_{}{}",
            day,
            part,
            if use_test { "_test" } else { "" }
        ));

    let mut file = match File::open(path.as_path()) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open file {}: {}", path.display(), why),
    };

    let mut input_string = String::new();
    match file.read_to_string(&mut input_string) {
        Ok(_) => input_string,
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
    }
}

pub fn get_input_split(day: &str, part: &str, use_test: bool, split: &str) -> Vec<String> {
    get_string(day, part, use_test)
        .split(split)
        .map(|s| String::from(s))
        .collect()
}

pub fn get_input(day: &str, part: &str, use_test: bool) -> Vec<String> {
    get_string(day, part, use_test)
        .lines()
        .map(|s| String::from(s))
        .collect()
}
