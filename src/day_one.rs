use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Given a list of numbers, find two that add up to 2020 and find their product
pub fn part_one(use_test: bool) -> Option<i32> {
    let input = get_input(use_test);
    for i in 0..input.len() {
        for j in 1..input.len() {
            if input[i] + input[j] == 2020 {
                return Some(input[i] * input[j]);
            }
        }
    }

    None
}

pub fn part_two(use_test: bool) -> Option<i32> {
    let input = get_input(use_test);
    for i in 0..input.len() {
        for j in 1..input.len() {
            for k in 2..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return Some(input[i] * input[j] * input[k]);
                }
            }
        }
    }

    None
}

fn get_input(use_test: bool) -> Vec<i32> {
    let mut filename = String::from("input01");

    if use_test {
        filename += "-test";
    }

    let pathstring = format!("inputs/day1/{}", filename);
    let path = Path::new(pathstring.as_str());

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut input_string = String::new();
    match file.read_to_string(&mut input_string) {
        Ok(_) => input_string
            .lines()
            .map(|s| (*s).parse::<i32>().unwrap())
            .collect(),
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
    }
}

#[test]
fn part_one_test() {
    assert_eq!(part_one(true), Some(514579));
}

#[test]
fn part_two_test() {
    assert_eq!(part_two(true), Some(241861950));
}
