use std::env;

extern crate simple_error;

use std::error::Error;
use simple_error::SimpleError;

mod day1;

mod utils {
    use std::io::{BufReader};
    use std::fs::File;
    use std::path::Path;
    use std::env;

    const DEFAULT_RESOURCES_PATH: &str = "./resources";

    pub fn read_input(filename: &str) -> Result<BufReader<File>, std::io::Error> {
        if env::var("RESOURCES_PATH").is_err() {
            env::set_var("RESOURCES_PATH", "./resources");
        }

        let path = Path::new(&env::var("RESOURCES_PATH").unwrap_or_else(|_| DEFAULT_RESOURCES_PATH.to_string())).join(filename);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(reader)
    }
}

type BoxResult<T> = Result<T,Box<dyn Error>>;

fn dispatch(day: &str, part: &str, input_or_filename: &str) {
    let result = match (day.trim_start_matches('0'), part) {
        ("1", "1") => day1::p1(utils::read_input(input_or_filename).unwrap()),
        ("1", "2") => day1::p2(utils::read_input(input_or_filename).unwrap()),
        _ => Err(Box::from(SimpleError::new(format!("No day/part combo found for: {}, {}", day, part)))),
    };

    println!("Result for Day {} Part {} was: {}", day, part, result.unwrap())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let part = &args[2];
    let input_or_file = &args[3];

    dispatch(day, part, input_or_file)
}
