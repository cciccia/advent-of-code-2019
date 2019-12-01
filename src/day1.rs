use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::BoxResult;
use std::collections::HashMap;
use std::cmp::max;

pub fn p1(input: BufReader<File>) -> BoxResult<String> {
    let mut result: i32 = 0;

    for line in input.lines() {
        result = result + (line?.parse::<i32>()? / 3 - 2)
    }

    Ok(format!("{}", result))
}

fn calc_fuel(cache: &mut HashMap<i32, i32>, module: i32) -> i32 {
    let mut start = module;
    let mut total = 0;
    while start > 0 {
        if cache.contains_key(&start) {
            total = total + cache.get(&start).unwrap().clone();
            return total;
        } else {
            start = max(start / 3 - 2, 0);
            total = total + start;
        }
    }

    total
}

pub fn p2(input: BufReader<File>) -> BoxResult<String> {
    let mut cache = HashMap::new();
    let mut result: i32 = 0;

    for line in input.lines() {
        let mut module = line?.parse::<i32>()?;
        result = result + calc_fuel(&mut cache, module);
    }

    Ok(format!("{}", result))
}