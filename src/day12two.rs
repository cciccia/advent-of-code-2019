use std::collections::HashSet;
use std::io::BufReader;
use std::fs::File;
use crate::BoxResult;
use std::thread;
use num::integer::lcm;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Jupiter {
    i_pos: i64,
    i_vel: i64,
    e_pos: i64,
    e_vel: i64,
    g_pos: i64,
    g_vel: i64,
    c_pos: i64,
    c_vel: i64,
}

fn adjustment(a: i64, b: i64) -> i64 {
    if a == b { 0 } else if a < b { 1 } else { -1 }
}

fn next_vel(jupiter: &Jupiter) -> Jupiter {
    Jupiter {
        i_pos: jupiter.i_pos.clone(),
        e_pos: jupiter.e_pos.clone(),
        g_pos: jupiter.g_pos.clone(),
        c_pos: jupiter.c_pos.clone(),
        i_vel: jupiter.i_vel + adjustment(jupiter.i_pos, jupiter.e_pos) + adjustment(jupiter.i_pos, jupiter.g_pos) + adjustment(jupiter.i_pos, jupiter.c_pos),
        e_vel: jupiter.e_vel + adjustment(jupiter.e_pos, jupiter.i_pos) + adjustment(jupiter.e_pos, jupiter.g_pos) + adjustment(jupiter.e_pos, jupiter.c_pos),
        g_vel: jupiter.g_vel + adjustment(jupiter.g_pos, jupiter.i_pos) + adjustment(jupiter.g_pos, jupiter.e_pos) + adjustment(jupiter.g_pos, jupiter.c_pos),
        c_vel: jupiter.c_vel + adjustment(jupiter.c_pos, jupiter.i_pos) + adjustment(jupiter.c_pos, jupiter.e_pos) + adjustment(jupiter.c_pos, jupiter.g_pos),

    }
}

fn next_pos(jupiter: &Jupiter) -> Jupiter {
    Jupiter {
        i_pos: jupiter.i_pos + jupiter.i_vel,
        e_pos: jupiter.e_pos + jupiter.e_vel,
        g_pos: jupiter.g_pos + jupiter.g_vel,
        c_pos: jupiter.c_pos + jupiter.c_vel,
        i_vel: jupiter.i_vel.clone(),
        e_vel: jupiter.e_vel.clone(),
        g_vel: jupiter.g_vel.clone(),
        c_vel: jupiter.c_vel.clone(),
    }
}

fn do_axis(i: i64, e: i64, g: i64, c: i64) -> i64 {
    let mut seen = HashSet::new();
    let mut count = 0;
    let mut jupiter = Jupiter {
        i_pos: i,
        e_pos: e,
        g_pos: g,
        c_pos: c,
        i_vel: 0,
        e_vel: 0,
        g_vel: 0,
        c_vel: 0,
    };
    seen.insert(jupiter.clone());
    loop {
        jupiter = next_pos(&next_vel(&jupiter));
        count = count + 1;
        if seen.contains(&jupiter) {
            break;
        }
        seen.insert(jupiter.clone());
    }

    count
}

pub fn p2(_input: BufReader<File>) -> BoxResult<String> {
    let mut threads = Vec::new();

    let (tx_x, rx_x): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx_y, rx_y): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx_z, rx_z): (Sender<i64>, Receiver<i64>) = mpsc::channel();


    threads.push(thread::spawn(move || {
        let x = do_axis(9, -3, -4, 0);
        tx_x.send(x).unwrap();
    }));

    threads.push(thread::spawn(move || {
        let y = do_axis(13, 16, 11, -2);
        tx_y.send(y).unwrap();
    }));

    threads.push(thread::spawn(move || {
        let z = do_axis(-8, -17, -10, -2);
        tx_z.send(z).unwrap();
    }));

    for child in threads {
        child.join().expect("fgsfdf");
    }

    let x = rx_x.recv().unwrap();
    let y = rx_y.recv().unwrap();
    let z = rx_z.recv().unwrap();


    Ok(format!("{}", lcm(lcm(x, y), z)))
}