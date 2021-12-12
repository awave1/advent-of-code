/// https://adventofcode.com/2021/day/1
use crate::input::get_input;

pub fn solve(use_test: bool) -> (i32, i32) {
	(p1(use_test), p2(use_test))
}

/// count the number of times a depth measurement increases from the previous measurement
pub fn p1(use_test: bool) -> i32 {
	// 1. get the input
	// 2. slice it into tuples of 2  using the .windows function
	// 3. map it to i32
	// 4. convert back to vector
	get_input("1", use_test)
		.windows(2)
		.map(|line| {
			(
				line[0].parse::<i32>().unwrap(),
				line[1].parse::<i32>().unwrap(),
			)
		})
		.fold(0, |mut increase_count, (d1, d2)| {
			if d2 > d1 {
				increase_count += 1;
			}

			increase_count
		})
}

pub fn p2(use_test: bool) -> i32 {
	let lines = get_input("1", use_test)
		.into_iter()
		.map(|line| line.parse::<i32>().unwrap())
		.collect::<Vec<_>>();

	let mut window_start = 0;
	let mut window_end = 3;

	let mut window_sums: Vec<i32> = vec![];

	while window_end != lines.len() + 1 {
		let mut window_sum = 0;

		for i in window_start..window_end {
			window_sum += lines[i];
		}

		window_start += 1;
		window_end += 1;

		window_sums.push(window_sum);
	}

	window_sums.windows(2).fold(0, |mut increase_count, sums| {
		if sums[0] < sums[1] {
			increase_count += 1;
		}

		increase_count
	})
}

#[test]
fn p1_test() {
	let result = p1(true);

	assert_eq!(result, 7);
}

#[test]
fn p2_test() {
	let result = p2(true);

	assert_eq!(result, 5);
}
