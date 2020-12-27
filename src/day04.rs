use crate::input::get_input_split;
use regex::Regex;

// The expected fields are as follows:
// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)
#[derive(Debug)]
struct Entry {
  byr: Option<String>,
  iyr: Option<String>,
  eyr: Option<String>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  cid: Option<String>,
}

impl Entry {
  fn new() -> Entry {
    Entry {
      byr: None,
      iyr: None,
      eyr: None,
      hgt: None,
      hcl: None,
      ecl: None,
      pid: None,
      cid: None,
    }
  }

  fn data_exists(self) -> bool {
    self.byr.is_some()
      && self.iyr.is_some()
      && self.eyr.is_some()
      && self.hgt.is_some()
      && self.hcl.is_some()
      && self.ecl.is_some()
      && self.pid.is_some()
  }

  fn is_valid(&self) -> bool {
    self.byr_valid()
      && self.iyr_valid()
      && self.eyr_valid()
      && self.hgt_valid()
      && self.hcl_valid()
      && self.ecl_valid()
      && self.pid_valid()
  }

  fn byr_valid(&self) -> bool {
    match &self.byr {
      Some(byr) => {
        (*byr).len() == 4
          && match (*byr).parse::<u32>() {
            Ok(num) => num >= 1920 && num <= 2002,
            Err(_) => false,
          }
      }
      None => false,
    }
  }

  fn iyr_valid(&self) -> bool {
    match &self.iyr {
      Some(iyr) => {
        (*iyr).len() == 4
          && match (*iyr).parse::<u32>() {
            Ok(num) => num >= 2010 && num <= 2020,
            Err(_) => false,
          }
      }
      None => false,
    }
  }

  fn eyr_valid(&self) -> bool {
    match &self.eyr {
      Some(eyr) => {
        (*eyr).len() == 4
          && match (*eyr).parse::<u32>() {
            Ok(num) => num >= 2020 && num <= 2030,
            Err(_) => false,
          }
      }
      None => false,
    }
  }

  fn hgt_valid(&self) -> bool {
    let re = Regex::new(r"(?P<val>\d+)(cm|in)").unwrap();

    match &self.hgt {
      Some(hgt) => match re.captures((*hgt).as_str()) {
        Some(caps) => {
          let val = &caps["val"].parse::<i32>().unwrap_or(-1);
          if *val < 0 {
            return false;
          }

          if hgt.ends_with("cm") {
            return *val >= 150 && *val <= 193;
          } else {
            return *val >= 59 && *val <= 76;
          }
        }
        None => false,
      },
      None => false,
    }
  }

  fn hcl_valid(&self) -> bool {
    let re = Regex::new(r"#[a-z0-9]{6}").unwrap();

    match &self.hcl {
      Some(hcl) => re.is_match((*hcl).as_str()),
      None => false,
    }
  }

  fn ecl_valid(&self) -> bool {
    let re = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();

    match &self.ecl {
      Some(ecl) => re.is_match((*ecl).as_str()),
      None => false,
    }
  }

  fn pid_valid(&self) -> bool {
    match &self.pid {
      Some(pid) => (*pid).len() == 9,
      None => false,
    }
  }
}

fn build_input(use_test: bool) -> Vec<String> {
  get_input_split("4", "2", use_test, "\n\n")
    .iter()
    .map(|s| s.replace("\n", " "))
    .collect()
}

fn parse_data(input: Vec<String>) -> Vec<Entry> {
  let parsed_data: Vec<Entry> = input
    .iter()
    .map(|s| {
      // declare all as None initially
      let mut entry = Entry::new();

      for e in s.split_whitespace().map(String::from) {
        let field: Vec<String> = e.split(":").map(String::from).collect();

        match field[0].as_str() {
          "byr" => entry.byr = Some(field[1].to_string()),
          "iyr" => entry.iyr = Some(field[1].to_string()),
          "eyr" => entry.eyr = Some(field[1].to_string()),
          "hgt" => entry.hgt = Some(field[1].to_string()),
          "hcl" => entry.hcl = Some(field[1].to_string()),
          "ecl" => entry.ecl = Some(field[1].to_string()),
          "pid" => entry.pid = Some(field[1].to_string()),
          "cid" => entry.cid = Some(field[1].to_string()),
          _ => panic!("Unknown field!"),
        };
      }

      entry
    })
    .collect();

  parsed_data
}

pub fn part_one(use_test: bool) -> Option<u32> {
  let input = build_input(use_test);

  let parsed_data = parse_data(input);

  let mut result = 0;
  for entry in parsed_data {
    if entry.data_exists() {
      result += 1;
    }
  }

  Some(result)
}

pub fn part_two(use_test: bool) -> Option<u32> {
  let input = build_input(use_test);
  let parsed_data = parse_data(input);

  let mut result = 0;
  for entry in parsed_data {
    if entry.is_valid() {
      result += 1;
    }
  }

  Some(result)
}

#[test]
fn part_two_test() {
  assert_eq!(Some(4), part_two(true));
}
