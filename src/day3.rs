use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::BoxResult;
use regex::Regex;
use std::collections::{HashSet, HashMap};

enum Axis {
    X,
    Y,
}

struct Snake {
    delay: HashMap<(i32, i32), i32>,
    points: HashSet<(i32, i32)>,
}

fn build_snake_path(snake: &str) -> Snake {
    let re: Regex = Regex::new("([UDLR])(\\d+)").unwrap();
    let snake_iter = snake.split(",");
    let mut snake = Snake {
        delay: HashMap::new(),
        points: HashSet::new(),
    };
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    for next_move in snake_iter {
        let caps = re.captures(next_move).unwrap();
        let m = (&caps[1], (&caps[2]).parse::<i32>().unwrap());
        let (x_or_y, neg, val) = match m {
            ("U", val) => Ok((Axis::Y, false, val)),
            ("D", val) => Ok((Axis::Y, true, val)),
            ("L", val) => Ok((Axis::X, true, val)),
            ("R", val) => Ok((Axis::X, false, val)),
            (dir, val) => Err(format!("Encountered weird command: {}{}", dir, val))
        }.unwrap();
        match x_or_y {
            Axis::X => {
                for _i in 1..val + 1 {
                    x = if neg { x - 1 } else { x + 1 };
                    steps = steps + 1;
                    snake.points.insert((x, y));
                    if !snake.delay.contains_key(&(x, y)) {
                        snake.delay.insert((x, y), steps);
                    }
                }
            },
            Axis::Y => {
                for _i in 1..val + 1 {
                    y = if neg { y - 1 } else { y + 1 };
                    steps = steps + 1;
                    snake.points.insert((x, y));
                    if !snake.delay.contains_key(&(x, y)) {
                        snake.delay.insert((x, y), steps);
                    }
                }
            }
        };
    }
    snake
}

pub fn p1(mut input: BufReader<File>) -> BoxResult<String> {
    let mut snake_a_str = String::new();
    let mut snake_b_str = String::new();

    input.read_line(&mut snake_a_str).unwrap();
    input.read_line(&mut snake_b_str).unwrap();

    let snake_a = build_snake_path(&snake_a_str);
    let snake_b = build_snake_path(&snake_b_str);

    let intersections = snake_a.points.intersection(&snake_b.points);

    let min = intersections.into_iter().fold(std::i32::MAX, |p, c| {
        let distance = c.0.abs() + c.1.abs();
        if distance < p { distance } else { p }
    });

    Ok(format!("{}", min))
}

pub fn p2(mut input: BufReader<File>) -> BoxResult<String> {
    let mut snake_a_str = String::new();
    let mut snake_b_str = String::new();

    input.read_line(&mut snake_a_str).unwrap();
    input.read_line(&mut snake_b_str).unwrap();

    let snake_a = build_snake_path(&snake_a_str);
    let snake_b = build_snake_path(&snake_b_str);

    let intersections = snake_a.points.intersection(&snake_b.points);

    let min = intersections.into_iter().fold(std::i32::MAX, |p, c| {
        let delay = snake_a.delay.get(c).unwrap() + snake_b.delay.get(c).unwrap();
        if delay < p { delay } else { p }
    });

    Ok(format!("{}", min))
}

