use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::BoxResult;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::f64::consts::PI;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

const F_0: f64 = 0 as f64;

fn angle(from: &Point, to: &Point) -> OrderedFloat<f64> {
    let delta_y = f64::from(to.y - from.y);
    let delta_x = f64::from(to.x - from.x);
    if delta_x == F_0 {
        if delta_y > F_0 {OrderedFloat(0 as f64)} else {OrderedFloat(std::f64::consts::PI as f64)}
    } else {
        let angle = delta_y.atan2(delta_x) + PI/(2 as f64);
        OrderedFloat(if angle < 0 as f64 {angle + PI * 2 as f64} else {angle})
    }
}

pub fn p1(input: BufReader<File>) -> BoxResult<String> {
    let mut asteroids = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let line_str = line.unwrap();
        for (j, c) in line_str.chars().enumerate() {
            if c == '#' {
                asteroids.push(Point { x: j as i32, y: i as i32 });
            }
        }
    }

    let result: (Point, i32) = asteroids.iter()
        .map(|from_asteroid| {
            let mut slopes = HashSet::new();
            for to_asteroid in asteroids.clone().iter() {
                slopes.insert(angle(from_asteroid, to_asteroid));
            }
            (from_asteroid.clone(), slopes.iter().count() as i32)
        })
        .sorted_by_key(|v| v.1)
        .last().unwrap();

    Ok(format!("{:?}", result))
}

pub fn p2(input: BufReader<File>) -> BoxResult<String> {
    let mut asteroids = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let line_str = line.unwrap();
        for (j, c) in line_str.chars().enumerate() {
            if c == '#' {
                asteroids.push(Point { x: j as i32, y: i as i32 });
            }
        }
    }

    let origin = Point {x: 11, y: 19};  // let's assume i knew this
    let mut slope_buckets = HashMap::new();

    for asteroid in asteroids.into_iter() {
        let slope = angle(&origin, &asteroid);
        if !(slope_buckets.contains_key(&slope)) {
            slope_buckets.insert(slope, Vec::new());
        }
        let mut bucket: Vec<Point> = slope_buckets.get(&slope).unwrap().to_vec();
        let seek = (asteroid.x - origin.x).abs() + (asteroid.y - origin.y).abs();
        let pos = bucket.binary_search_by(|existing_asteroid| {
            ((existing_asteroid.x - origin.x).abs() + (existing_asteroid.y - origin.y).abs()).cmp(&seek)
        }).unwrap_or_else(|e| e);
        bucket.insert(pos, asteroid);
        slope_buckets.insert(slope, bucket);
    }
    let mut i = 0;
    loop {
        for key in slope_buckets.clone().keys().sorted() {
            let mut bucket = slope_buckets.get(&key).unwrap().clone();
            if !(bucket.is_empty()) {
                let popped = bucket.remove(0);
                i = i + 1;
                if i == 200 {
                    return Ok(format!("{}", popped.x * 100 + popped.y));
                }
                slope_buckets.insert(*key, bucket);
            }
        }
    }
}