extern crate advent_of_code_2019;

use std::env;
use advent_of_code_2019::dispatch;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let part = &args[2];
    let input_or_file = &args[3];

    let result = dispatch(day, part, input_or_file).unwrap();

    println!("Result for Day {} Part {} was: {}", day, part, result);
}