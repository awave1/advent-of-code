use clap;

mod input;

mod day1;

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
		"1" => {
			let (p1_result, p2_result) = day1::solve(is_test);

			println!("day1.1: {:?}", p1_result);
			println!("day1.2: {:?}", p2_result);
		}
		_ => {
			panic!("Solutions for day{} are not implemented yet!", day);
		}
	}
}
