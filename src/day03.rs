use crate::input::get_input;

type Grid = Vec<Vec<char>>;

fn calculate_trees(grid: &Grid, x_steps: usize, y_steps: usize) -> Option<u64> {
    let width = grid[0].len();
    let mut result = 0;

    let mut x = 0;
    let mut y = 0;

    while y < grid.len() {
        if grid[y][x] == '#' {
            result += 1;
        }

        y += y_steps;
        x = (x + x_steps) % width;
    }

    Some(result)
}

pub fn part_one(is_test: bool) -> Option<u64> {
    let input = get_input("3", "1", is_test);
    let grid = input.iter().map(|l| l.chars().collect()).collect::<Grid>();

    calculate_trees(&grid, 3, 1)
}

pub fn part_two(is_test: bool) -> Option<u64> {
    let input = get_input("3", "1", is_test);
    let grid = input.iter().map(|l| l.chars().collect()).collect::<Grid>();
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result = slopes
        .iter()
        .map(|slope| calculate_trees(&grid, slope.0, slope.1).unwrap_or(0))
        .fold(1u64, |total, val| total * val);

    Some(result)
}

#[test]
fn part_one_test() {
    assert_eq!(Some(7), part_one(true));
}

#[test]
fn part_two_test() {
    assert_eq!(Some(336), part_two(true));
}
