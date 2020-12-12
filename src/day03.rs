use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn part_one(is_test: bool) -> Option<u32> {
    let input = get_input(is_test);
    let grid = input
        .iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut position = (0, 0);
    //           "down", "right"
    let direction = (1, 3);

    let mut trees = 0;
    println!("");
    while position.0 != grid.len() {
        println!("{:?}", grid[position.0]);
        let mut counter = 0;
        while position.1 != grid[position.0].len() {
            if counter == direction.1 + 1 {
                break;
            }

            let current_char = grid[position.0][position.1];
            if current_char == '#' {
                trees += 1;
            }

            position.1 += 1;
            counter += 1;
        }
        position.0 += direction.0;
    }

    println!("trees = {}", trees);
    None
}

fn get_input(use_test: bool) -> Vec<String> {
    let mut filename = String::from("input1");

    if use_test {
        filename += "-test";
    }

    let pathstring = format!("src/input/day3-{}", filename);
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
    assert_eq!(None, part_one(true));
}
