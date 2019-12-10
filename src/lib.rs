#[macro_use]
extern crate simple_error;

extern crate rayon;

use std::error::Error;
use simple_error::SimpleError;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;

mod intcode;

pub mod utils {
    use std::io::{BufReader};
    use std::fs::File;
    use std::path::Path;
    use std::env;

    const DEFAULT_RESOURCES_PATH: &str = "./resources";

    pub fn read_input(filename: &str) -> Result<BufReader<File>, std::io::Error> {
        let path = Path::new(&env::var("RESOURCES_PATH").unwrap_or_else(|_| DEFAULT_RESOURCES_PATH.to_string())).join(filename);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(reader)
    }
}

type BoxResult<T> = Result<T,Box<dyn Error>>;

pub fn dispatch(day: &str, part: &str, input_or_filename: &str) -> BoxResult<String> {
    match (day.trim_start_matches('0'), part) {
        ("1", "1") => day1::p1(utils::read_input(input_or_filename).unwrap()),
        ("1", "2") => day1::p2(utils::read_input(input_or_filename).unwrap()),
        ("2", "1") => day2::p1(utils::read_input(input_or_filename).unwrap()),
        ("2", "2") => day2::p2(utils::read_input(input_or_filename).unwrap()),
        ("3", "1") => day3::p1(utils::read_input(input_or_filename).unwrap()),
        ("3", "2") => day3::p2(utils::read_input(input_or_filename).unwrap()),
        ("4", "1") => day4::p1(&input_or_filename),
        ("4", "2") => day4::p2(&input_or_filename),
        ("5", "1") => day5::p1(utils::read_input(input_or_filename).unwrap()),
        ("5", "2") => day5::p2(utils::read_input(input_or_filename).unwrap()),
        ("6", "1") => day6::p1(utils::read_input(input_or_filename).unwrap()),
        ("6", "2") => day6::p2(utils::read_input(input_or_filename).unwrap()),
        ("7", "1") => day7::p1(utils::read_input(input_or_filename).unwrap()),
        ("7", "2") => day7::p2(utils::read_input(input_or_filename).unwrap()),
        ("8", "1") => day8::p1(utils::read_input(input_or_filename).unwrap()),
        ("8", "2") => day8::p2(utils::read_input(input_or_filename).unwrap()),
        ("9", "1") => day9::p1(utils::read_input(input_or_filename).unwrap()),
        ("9", "2") => day9::p2(utils::read_input(input_or_filename).unwrap()),
        ("10", "1") => day10::p1(utils::read_input(input_or_filename).unwrap()),
        ("10", "2") => day10::p2(utils::read_input(input_or_filename).unwrap()),
        _ => Err(Box::from(SimpleError::new(format!("No day/part combo found for: {}, {}", day, part)))),
    }
}