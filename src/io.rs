#[macro_export]
macro_rules! get_input {
    (day: String, use_test: bool) => {
        let mut filename = String::from("input1");

        if use_test {
            filename += "-test";
        }

        let pathstring = format!("src/input/{}-{}", day, filename);
        let path = Path::new(pathstring.as_str());

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open file {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut input_string = String::new();
        match file.read_to_string(&mut input_string) {
            Ok(_) => input_string.lines().map(|s| String::from(s)).collect(),
            Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        }
    };
}
