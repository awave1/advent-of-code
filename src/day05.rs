use crate::input::get_input;

fn parse_input(input: Vec<char>) -> (u64, u64) {
    let row_code = input[0..7].to_vec();
    let col_code = input[7..].to_vec();

    let row_bin = row_code
        .iter()
        .map(|c| match *c {
            'F' => '0',
            'B' => '1',
            _ => panic!("failed to parse character: {}", c),
        })
        .collect::<String>();

    let col_bin = col_code
        .iter()
        .map(|c| match *c {
            'L' => '0',
            'R' => '1',
            _ => panic!("failed to parse character: {}", c),
        })
        .collect::<String>();

    let row = isize::from_str_radix(row_bin.as_str(), 2).unwrap();
    let col = isize::from_str_radix(col_bin.as_str(), 2).unwrap();

    (row as u64, col as u64)
}

pub fn part_one(use_test: bool) -> Option<u64> {
    let input = get_input("5", "1", use_test);
    let mut res = input
        .iter()
        .map(|s| parse_input(s.chars().collect()))
        .map(|(row, col)| row * 8 + col)
        .collect::<Vec<u64>>();
    res.sort_by(|a, b| b.cmp(a));

    Some(res[0])
}

pub fn part_two(use_test: bool) -> Option<u32> {
    let input = get_input("5", "2", use_test);
    None
}

#[test]
fn part_one_test() {
    assert_eq!(Some(820), part_one(true));
}

#[test]
fn part_two_test() {
    unimplemented!();
}
