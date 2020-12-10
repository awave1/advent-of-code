mod day01;
mod day02;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_test = (args.get(2).unwrap_or(&String::from(""))).eq("test");

    match args.get(1) {
        Some(day) => match day.as_str() {
            "1" => {
                println!("day 1.1: {:?}", day01::part_one(is_test));
                println!("day 1.2: {:?}", day01::part_two(is_test));
            }
            "2" => {
                println!("day 2.1: {:?}", day02::part_one(is_test));
                println!("day 2.2: {:?}", day02::part_two(is_test));
            }
            _ => panic!("day is not supported"),
        },
        None => panic!("day is not specified"),
    }
}
