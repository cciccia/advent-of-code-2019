use std::io::BufReader;
use std::fs::File;
use crate::BoxResult;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Jupiter {
    i_pos: Point,
    e_pos: Point,
    g_pos: Point,
    c_pos: Point,
    i_vel: Point,
    e_vel: Point,
    g_vel: Point,
    c_vel: Point,
}

fn energy(point: &Point) -> i64 {
    point.x.abs() + point.y.abs() + point.z.abs()
}

fn adjustment(a: i64, b: i64) -> i64 {
    if a == b { 0 } else if a < b { 1 } else { -1 }
}

fn attract(vel: &Point, a: &Point, b: &Point, c: &Point, d: &Point) -> Point {
    Point {
        x: vel.x + adjustment(a.x, b.x) + adjustment(a.x, c.x) + adjustment(a.x, d.x),
        y: vel.y + adjustment(a.y, b.y) + adjustment(a.y, c.y) + adjustment(a.y, d.y),
        z: vel.z + adjustment(a.z, b.z) + adjustment(a.z, c.z) + adjustment(a.z, d.z),
    }
}

fn next_vel(jupiter: &Jupiter) -> Jupiter {
    Jupiter {
        i_pos: jupiter.i_pos.clone(),
        e_pos: jupiter.e_pos.clone(),
        g_pos: jupiter.g_pos.clone(),
        c_pos: jupiter.c_pos.clone(),
        i_vel: attract(&jupiter.i_vel, &jupiter.i_pos, &jupiter.e_pos, &jupiter.g_pos, &jupiter.c_pos),
        e_vel: attract(&jupiter.e_vel, &jupiter.e_pos, &jupiter.i_pos, &jupiter.g_pos, &jupiter.c_pos),
        g_vel: attract(&jupiter.g_vel, &jupiter.g_pos, &jupiter.e_pos, &jupiter.i_pos, &jupiter.c_pos),
        c_vel: attract(&jupiter.c_vel, &jupiter.c_pos, &jupiter.e_pos, &jupiter.g_pos, &jupiter.i_pos),
    }
}

fn displace(point: &Point, vel: &Point) -> Point {
    Point {
        x: point.x + vel.x,
        y: point.y + vel.y,
        z: point.z + vel.z,
    }
}

fn next_pos(jupiter: &Jupiter) -> Jupiter {
    Jupiter {
        i_pos: displace(&jupiter.i_pos, &jupiter.i_vel),
        e_pos: displace(&jupiter.e_pos, &jupiter.e_vel),
        g_pos: displace(&jupiter.g_pos, &jupiter.g_vel),
        c_pos: displace(&jupiter.c_pos, &jupiter.c_vel),
        i_vel: jupiter.i_vel.clone(),
        e_vel: jupiter.e_vel.clone(),
        g_vel: jupiter.g_vel.clone(),
        c_vel: jupiter.c_vel.clone(),
    }
}

pub fn p1(_input: BufReader<File>) -> BoxResult<String> {
    let mut i = 0;
    let mut jupiter = Jupiter {
        i_pos: Point { x: 9, y: 13, z: -8 },
        e_pos: Point { x: -3, y: 16, z: -17 },
        g_pos: Point { x: -4, y: 11, z: -10 },
        c_pos: Point { x: 0, y: -2, z: -2 },
        i_vel: Point { x: 0, y: 0, z: 0 },
        e_vel: Point { x: 0, y: 0, z: 0 },
        g_vel: Point { x: 0, y: 0, z: 0 },
        c_vel: Point { x: 0, y: 0, z: 0 },
    };
    while i < 1000 {
        jupiter = next_pos(&next_vel(&jupiter));
        i = i + 1;
    }


    Ok(format!("{}",
               energy(&jupiter.i_pos) * energy(&jupiter.i_vel) +
                   energy(&jupiter.e_pos) * energy(&jupiter.e_vel) +
                   energy(&jupiter.g_pos) * energy(&jupiter.g_vel) +
                   energy(&jupiter.c_pos) * energy(&jupiter.c_vel)
    ))
}

