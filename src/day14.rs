use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::BoxResult;
use std::collections::{HashMap, HashSet};
use std::cmp::max;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct MatUnits(String, i32);

fn iterate_on_mats(mats: &HashMap<String, i32>, formulae: &HashMap<String, (i32, Vec<MatUnits>)>) -> HashMap<String, i32> {
    let (mat, needed_amt) = mats.iter().filter(|(k, v)| {*k != "ORE"}).next().unwrap();
    let mut mats_cloned = mats.clone();
    let (conversion_amt, new_mats) = formulae.get(mat).unwrap();
    let multiplier = (*needed_amt as f32 / *conversion_amt as f32).ceil() as i32;
    for new_mat in new_mats {
        mats_cloned.insert(new_mat.0.clone(), mats.get(&new_mat.0).or(Some(&0)).unwrap() + new_mat.1 * multiplier);
        mats_cloned.insert(mat.to_string(), max(mats.get(mat).or(Some(&0)).unwrap() - *conversion_amt * multiplier, 0));
    }
    mats_cloned.remove(mat);
    mats_cloned
}

pub fn p1(input: BufReader<File>) -> BoxResult<String> {
    let mut formulae: HashMap<String, (i32, Vec<MatUnits>)> = HashMap::new();
    let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let line_val = line.unwrap();
        let vals: Vec<&str> = line_val.split(" => ").collect();
        let result = vals[1].to_string();
        let result_vec: Vec<&str> = result.split(" ").collect();
        let components = vals[0].to_string();
        let component_vals = components.split(", ");
        let component_vals_vec: Vec<MatUnits> = component_vals.map(|mat_pair| {
            let mat_pair_vec: Vec<&str> = mat_pair.split(" ").collect();
            MatUnits {
                0: mat_pair_vec[1].to_string(),
                1: mat_pair_vec[0].parse::<i32>().unwrap(),
            }
        }).collect();

        formulae.insert(result_vec[1].to_string(), (result_vec[0].parse::<i32>().unwrap(),component_vals_vec));
    }

    let mut mats = HashMap::new();
    mats.insert("FUEL".to_string(), 1);
    mats.insert("ORE".to_string(), 0);

    loop {
        println!("mats is now={:?}", mats);
        mats = iterate_on_mats(&mats, &formulae);
        if mats.iter().filter(|(k, v)| {*k != "ORE" && **v > 0}).count() == 0 {break;}
    }

    Ok(format!("{:?}", mats.get("ORE").unwrap()))
}