use crate::input::get_input;

pub fn part_one(is_test: bool) -> Option<u32> {
    let input = get_input("3", "1", is_test);
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

#[test]
fn part_one_test() {
    assert_eq!(None, part_one(true));
}
