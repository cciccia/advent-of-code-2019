use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::BoxResult;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn can_see(asteroids: &Vec<Point>, from: &Point, to: &Point) -> bool {
    let delta_x = to.x - from.x;
    let delta_y = to.y - from.y;

    from != to && !asteroids.iter().any(|asteroid| {
        if asteroid != from && asteroid != to &&
            (from.x <= asteroid.x && asteroid.x <= to.x || from.x >= asteroid.x && asteroid.x >= to.x) &&
            (from.y <= asteroid.y && asteroid.y <= to.y || from.y >= asteroid.y && asteroid.y >= to.y) &&
            match (delta_x, delta_y) {
                (0, _) => asteroid.x == from.x,
                (_, 0) => asteroid.y == from.y,
                (_, _) => f64::from(asteroid.x - from.x) / f64::from(delta_x) == f64::from(asteroid.y - from.y) / f64::from(delta_y)
            } {
        }
        asteroid != from && asteroid != to &&
            (from.x <= asteroid.x && asteroid.x <= to.x || from.x >= asteroid.x && asteroid.x >= to.x) &&
            (from.y <= asteroid.y && asteroid.y <= to.y || from.y >= asteroid.y && asteroid.y >= to.y) &&
            match (delta_x, delta_y) {
                (0, _) => asteroid.x == from.x,
                (_, 0) => asteroid.y == from.y,
                (_, _) => f64::from(asteroid.x - from.x) / f64::from(delta_x) == f64::from(asteroid.y - from.y) / f64::from(delta_y)
            }
    })
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

    let sorted: (Point, i32) = asteroids.iter().map(|from_asteroid| {
        (from_asteroid.clone(), asteroids.clone().iter().fold(0, |acc, to_asteroid| {
            if can_see(&asteroids, &from_asteroid, &to_asteroid) { acc + 1 } else { acc }
        }))
    })
        .sorted_by_key(|v| v.1)
        .last().unwrap();

    Ok(format!("{:?}", sorted))
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




}