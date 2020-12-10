use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct ParsedLine {
    at_least: u32,
    at_most: u32,
    letter: String,
    password: String,
}

fn parse_line(line: &String) -> Option<ParsedLine> {
    // create a regex expression
    // \d+-\d+\s[a-zA-Z]:\s\w+
    // with named components that can be extracted and passed into ParsedLine struct
    let re =
        Regex::new(r"(?P<at_least>\d+)-(?P<at_most>\d+)\s(?P<letter>[a-zA-Z]):\s(?P<password>\w+)")
            .unwrap();
    match re.captures(line.as_str()) {
        Some(caps) => {
            let at_least = &caps["at_least"].parse::<u32>().unwrap();
            let at_most = &caps["at_most"].parse::<u32>().unwrap();
            let letter = String::from(&caps["letter"]);
            let password = String::from(&caps["password"]);

            let parsed_line = ParsedLine {
                at_least: *at_least,
                at_most: *at_most,
                letter: letter,
                password: password,
            };

            Some(parsed_line)
        }
        None => None,
    }
}

/// Each line gives the password policy and then the password.
/// The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid.
/// For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
pub fn part_one(use_test: bool) -> Option<u32> {
    let input = get_input(use_test);
    let mut count = 0;

    for line in input {
        match parse_line(&line) {
            Some(parsed_line) => {
                let password = parsed_line.password;
                let at_least = parsed_line.at_least;
                let at_most = parsed_line.at_most;

                let matches: Vec<&str> = password
                    .matches(parsed_line.letter.chars().last().unwrap())
                    .collect();
                let occurrences = matches.len() as u32;

                if occurrences >= at_least && occurrences <= at_most {
                    count += 1;
                }
            }
            None => panic!("failed to parse line: {}", line),
        }
    }

    Some(count)
}

pub fn part_two(use_test: bool) -> Option<i32> {
    let input = get_input(use_test);
    let mut count: i32 = 0;

    for line in input {
        match parse_line(&line) {
            Some(parsed_line) => {
                let ch = parsed_line.letter.chars().last().unwrap();
                let first_position = (parsed_line.at_least as usize) - 1;
                let second_position = (parsed_line.at_most as usize) - 1;

                let first_ch = match parsed_line.password.chars().nth(first_position) {
                    Some(char_found) => char_found,
                    None => panic!(
                        "can't find character at position {} (original: {}) in string {}",
                        first_position,
                        first_position + 1,
                        parsed_line.password,
                    ),
                };
                let second_ch = match parsed_line.password.chars().nth(second_position) {
                    Some(char_found) => char_found,
                    None => panic!(
                        "can't find character at position {} (original: {}) in string '{}'",
                        second_position,
                        second_position + 1,
                        parsed_line.password,
                    ),
                };

                let occurrences = vec![first_ch.eq(&ch), second_ch.eq(&ch)];
                let exactly_one_char = occurrences
                    .iter()
                    .filter(|&f| *f)
                    .collect::<Vec<&bool>>()
                    .len()
                    == 1;
                if exactly_one_char {
                    count += 1;
                }
            }
            None => panic!("failed to parse line: {}", line),
        }
    }

    Some(count)
}

fn get_input(use_test: bool) -> Vec<String> {
    let mut filename = String::from("input1");

    if use_test {
        filename += "-test";
    }

    let pathstring = format!("src/input/day2-{}", filename);
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
}

#[test]
fn part_one_test() {
    assert_eq!(Some(2), part_one(true));
}

#[test]
fn part_two_test() {
    assert_eq!(Some(1), part_two(true));
}
