use clap;

mod day01;
mod day02;
mod day03;
mod input;

fn parse_args() -> clap::ArgMatches {
    clap::App::new("advent-of-code")
        .arg(
            clap::Arg::new("day")
                .short('d')
                .long("day")
                .value_name("DAY")
                .about("Specifies the day to run solutions for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::new("test")
                .short('t')
                .long("test")
                .about("Runs only test inputs"),
        )
        .get_matches()
}

fn main() {
    let args = parse_args();
    let day = args.value_of("day").unwrap();
    let is_test = args.occurrences_of("test") == 1;

    match day {
        "1" => {
            println!("day 1.1: {:?}", day01::part_one(is_test));
            println!("day 1.2: {:?}", day01::part_two(is_test));
        }
        "2" => {
            println!("day 2.1: {:?}", day02::part_one(is_test));
            println!("day 2.2: {:?}", day02::part_two(is_test));
        }
        "3" => {
            println!("day 3.1: {:?}", day03::part_one(is_test));
        }
        _ => panic!("solutions for day {} are not implemented", day),
    }
}
