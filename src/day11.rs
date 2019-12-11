use std::io::{BufReader, BufRead};
use crate::BoxResult;
use std::collections::{HashMap, HashSet};
use std::str::from_utf8;
use crate::intcode::calc_output;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::fs::File;
use std::i64::{MAX, MIN};
use std::borrow::Borrow;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

const BLACK_SQUARE: [u8; 3] = [0xE2, 0x96, 0xA0];
const WHITE_SQUARE: [u8; 3] = [0xE2, 0x96, 0xA1];

fn turn_robot(dir: &Direction, turn: i64) -> BoxResult<Direction> {
    match (dir, turn) {
        (Direction::Up, 0) => Ok(Direction::Left),
        (Direction::Left, 0) => Ok(Direction::Down),
        (Direction::Down, 0) => Ok(Direction::Right),
        (Direction::Right, 0) => Ok(Direction::Up),
        (Direction::Up, 1) => Ok(Direction::Right),
        (Direction::Left, 1) => Ok(Direction::Up),
        (Direction::Down, 1) => Ok(Direction::Left),
        (Direction::Right, 1) => Ok(Direction::Down),
        (_, _) => bail!("the hell")
    }
}

fn move_robot(point: &Point, dir: &Direction) -> BoxResult<Point> {
    match dir {
        Direction::Up => Ok(Point {x: point.x, y: point.y + 1}),
        Direction::Left => Ok(Point {x: point.x - 1, y: point.y}),
        Direction::Down => Ok(Point {x: point.x, y: point.y - 1}),
        Direction::Right => Ok(Point {x: point.x + 1, y: point.y}),
    }
}

pub fn p1(input: BufReader<File>) -> BoxResult<String> {
    let mut commands = HashMap::new();

    let mut i = 0;
    for command in input.split(b',') {
        let parsed = from_utf8(&command.unwrap()).unwrap().parse::<i64>().unwrap();
        commands.insert(i, parsed);
        i = i + 1;
    }

    let (tx_in, rx_in): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx_out, rx_out): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    let mut threads = Vec::new();

    threads.push(thread::spawn(move || {
        calc_output(&mut commands, &rx_in, &tx_out).unwrap();
        tx_out.send(-1).unwrap();
    }));

    let mut painted = HashSet::new();
    let mut white = HashSet::new();

    let mut current_position = Point {x: 0, y: 0};
    let mut current_direction = Direction::Up;

    loop {
        let current_color = if white.contains(&current_position) {1} else {0};
        tx_in.send(current_color).unwrap();

        let color_to_paint = rx_out.recv().unwrap();

        match color_to_paint {
            0 => {
                painted.insert(current_position.clone());
                white.remove(&current_position);
            },
            1 => {
                painted.insert(current_position.clone());
                white.insert(current_position.clone());
            },
            -1 => {
                break;
            },
            _ => {
                bail!("y u do dis");
            },
        }

        let turn = rx_out.recv().unwrap();

        current_direction = turn_robot(&current_direction, turn).unwrap();
        current_position = move_robot(&current_position, &current_direction).unwrap();
    }

    Ok(format!("{}", painted.iter().count()))
}

pub fn p2(input: BufReader<File>) -> BoxResult<String> {
    let mut commands = HashMap::new();

    let mut i = 0;
    for command in input.split(b',') {
        let parsed = from_utf8(&command.unwrap()).unwrap().parse::<i64>().unwrap();
        commands.insert(i, parsed);
        i = i + 1;
    }

    let (tx_in, rx_in): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx_out, rx_out): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    let mut threads = Vec::new();

    threads.push(thread::spawn(move || {
        calc_output(&mut commands, &rx_in, &tx_out).unwrap();
        tx_out.send(-1).unwrap();
    }));

    let mut painted = HashSet::new();
    let mut white = HashSet::new();
    white.insert(Point {x: 0, y: 0});

    let mut current_position = Point {x: 0, y: 0};
    let mut current_direction = Direction::Up;

    let mut min_x = MAX;
    let mut min_y = MAX;
    let mut max_x = MIN;
    let mut max_y = MIN;

    loop {
        let current_color = if white.contains(&current_position) {1} else {0};
        tx_in.send(current_color).unwrap();

        let color_to_paint = rx_out.recv().unwrap();

        match color_to_paint {
            0 => {
                painted.insert(current_position.clone());
                white.remove(&current_position);
            },
            1 => {
                painted.insert(current_position.clone());
                white.insert(current_position.clone());
            },
            -1 => {
                break;
            },
            _ => {
                bail!("y u do dis");
            },
        }

        min_x = if current_position.x < min_x {current_position.x} else {min_x};
        min_y = if current_position.y < min_y {current_position.y} else {min_y};
        max_x = if current_position.x > max_x {current_position.x} else {max_x};
        max_y = if current_position.y > max_y {current_position.y} else {max_y};

        let turn = rx_out.recv().unwrap();

        current_direction = turn_robot(&current_direction, turn).unwrap();
        current_position = move_robot(&current_position, &current_direction).unwrap();
    }

    println!("minx={}, maxx={}, miny={}, maxy= {}", min_x, max_x, min_y, max_y);

    let mut out = String::new();
    out.push_str("\n");
    for y in -max_y..-min_y+1 {
        for x in min_x..max_x+1 {
            let color = if white.contains(Point { x, y: -y }.borrow()) {
                String::from_utf8(WHITE_SQUARE.to_vec()).unwrap()
            } else {
                String::from_utf8(BLACK_SQUARE.to_vec()).unwrap()
            };
            out.push_str(color.as_str());
        }
        out.push_str("\n");
    }

    Ok(format!("{}", out))
}
