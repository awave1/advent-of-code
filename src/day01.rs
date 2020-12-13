use crate::input::get_input;

/// Given a list of numbers, find two that add up to 2020 and find their product
pub fn part_one(use_test: bool) -> Option<i32> {
    let mut input = get_input("1", "1", use_test)
        .iter()
        .map(|s| (*s).parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    input.sort();

    for i in input.iter() {
        for j in input.iter() {
            if i + j == 2020 {
                return Some(i * j);
            }
        }
    }

    None
}

pub fn part_two(use_test: bool) -> Option<i32> {
    let mut input = get_input("1", "2", use_test)
        .iter()
        .map(|s| (*s).parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    input.sort();

    for i in input.iter() {
        for j in input.iter() {
            for k in input.iter() {
                if i + j + k == 2020 {
                    return Some(i * j * k);
                }
            }
        }
    }

    None
}

#[test]
fn part_one_test() {
    assert_eq!(part_one(true), Some(514579));
}

#[test]
fn part_two_test() {
    assert_eq!(part_two(true), Some(241861950));
}
