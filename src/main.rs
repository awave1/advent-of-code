mod day01;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1) {
        Some(day) => match day.as_str() {
            "1" => {
                println!("day 1.1: {:?}", day01::part_one(false));
                println!("day 1.2: {:?}", day01::part_two(false));
            }
            _ => panic!("day is not supported"),
        },
        None => panic!("day is not specified"),
    }
}
