use crate::input::get_input;

pub fn solve(use_test: bool) -> (i32, i32) {
	(p1(use_test), p2(use_test))
}

pub fn p1(use_test: bool) -> i32 {
	let mut x = 0;
	let mut y = 0;

	get_input("2", use_test).into_iter().fold(0, |_, line| {
		let split = line.split(" ").collect::<Vec<&str>>();
		let direction = split[0];
		let amount = split[1].parse::<i32>().unwrap();

		match direction {
			"forward" => {
				x += amount;
			}
			"down" => {
				y += amount;
			}
			"up" => {
				y -= amount;
			}
			_ => panic!("unknown direcrtion {:?}", direction),
		}

		x * y
	})
}

pub fn p2(use_test: bool) -> i32 {
	let mut x = 0;
	let mut y = 0;
	let mut aim = 0;

	get_input("2", use_test).into_iter().fold(0, |_, line| {
		let split = line.split(" ").collect::<Vec<&str>>();
		let direction = split[0];
		let amount = split[1].parse::<i32>().unwrap();

		match direction {
			"forward" => {
				x += amount;
				y += aim * amount;
			}
			"down" => {
				aim += amount;
			}
			"up" => {
				aim -= amount;
			}
			_ => panic!("unknown direcrtion {:?}", direction),
		}

		x * y
	})
}

#[test]
fn day2_p1_test() {
	assert_eq!(p1(true), 150);
}

#[test]
fn day2_p2_test() {
	assert_eq!(p2(true), 900);
}
