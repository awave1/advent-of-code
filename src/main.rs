use clap;

mod input;

mod day1;
mod day2;

fn show_solution(day: &str, solutions: (i32, i32)) {
	println!("day{}.1: {:?}", day, solutions.0);
	println!("day{}.2: {:?}", day, solutions.1);
}

fn main() {
	let args = clap::App::new("aoc2021")
		.arg(
			clap::Arg::with_name("day")
				.short("d")
				.long("day")
				.value_name("DAY")
				.help("Specifies the day to solve")
				.takes_value(true)
				.required(true),
		)
		.arg(
			clap::Arg::with_name("test")
				.short("t")
				.long("test")
				.help("Run the specified day against test input")
				.takes_value(false)
				.required(false),
		)
		.get_matches();

	let day = args.value_of("day").unwrap();
	let is_test = args.is_present("test");

	match day {
		"1" => show_solution(day, day1::solve(is_test)),
		"2" => show_solution(day, day2::solve(is_test)),
		_ => {
			panic!("Solutions for day{} are not implemented yet!", day);
		}
	}
}
