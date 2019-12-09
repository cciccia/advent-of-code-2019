use crate::intcode::calc_output;
use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::BoxResult;
use std::collections::HashMap;
use std::str::from_utf8;
use rayon::prelude::*;
use itertools::Itertools;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub fn p1(input: BufReader<File>) -> BoxResult<String> {
    let mut commands = HashMap::new();

    let mut i = 0;
    for command in input.split(b',') {
        let parsed = from_utf8(&command.unwrap()).unwrap().parse::<i64>().unwrap();
        commands.insert(i, parsed);
        i = i + 1;
    }

    let combos: Vec<Vec<i64>> = (0..5).permutations(5).collect();
    let result = combos.into_par_iter().map(|combo| {
        let mut threads = Vec::new();

        let (tx_ia, rx_ia): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_ab, rx_ab): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_bc, rx_bc): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_cd, rx_cd): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_de, rx_de): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_eo, rx_eo): (Sender<i64>, Receiver<i64>) = mpsc::channel();

        tx_ia.send(combo[0]).unwrap();
        tx_ab.send(combo[1]).unwrap();
        tx_bc.send(combo[2]).unwrap();
        tx_cd.send(combo[3]).unwrap();
        tx_de.send(combo[4]).unwrap();

        let mut commands_a = commands.clone();
        let mut commands_b = commands.clone();
        let mut commands_c = commands.clone();
        let mut commands_d = commands.clone();
        let mut commands_e = commands.clone();

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_a, &rx_ia, &tx_ab).unwrap();
        }));

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_b, &rx_ab, &tx_bc).unwrap();
        }));

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_c, &rx_bc, &tx_cd).unwrap();
        }));

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_d, &rx_cd, &tx_de).unwrap();
        }));

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_e, &rx_de, &tx_eo).unwrap();
        }));

        tx_ia.send(0).unwrap();

        rx_eo.recv().unwrap()
    }).max().unwrap();

    Ok(format!("{}", result))
}

pub fn p2(input: BufReader<File>) -> BoxResult<String> {
    let mut commands = HashMap::new();

    let mut i = 0;
    for command in input.split(b',') {
        let parsed = from_utf8(&command.unwrap()).unwrap().parse::<i64>().unwrap();
        commands.insert(i, parsed);
        i = i + 1;
    }

    let combos: Vec<Vec<i64>> = (5..10).permutations(5).collect();
    let result = combos.into_par_iter().map(|combo| {
        let mut threads = Vec::new();

        let (tx_ab, rx_ab): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_bc, rx_bc): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_cd, rx_cd): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_de, rx_de): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_ea, rx_ea): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (tx_o, rx_o): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let tx_ia = tx_ea.clone();

        tx_ea.send(combo[0]).unwrap();
        tx_ab.send(combo[1]).unwrap();
        tx_bc.send(combo[2]).unwrap();
        tx_cd.send(combo[3]).unwrap();
        tx_de.send(combo[4]).unwrap();

        let mut commands_a = commands.clone();
        let mut commands_b = commands.clone();
        let mut commands_c = commands.clone();
        let mut commands_d = commands.clone();
        let mut commands_e = commands.clone();

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_a, &rx_ea, &tx_ab).unwrap();
            tx_o.send(rx_ea.recv().unwrap()).unwrap();
        }));

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_b, &rx_ab, &tx_bc).unwrap();
        }));

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_c, &rx_bc, &tx_cd).unwrap();
        }));

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_d, &rx_cd, &tx_de).unwrap();
        }));

        threads.push(thread::spawn(move || {
            calc_output(&mut commands_e, &rx_de, &tx_ea).unwrap();
        }));

        tx_ia.send(0).unwrap();

        for child in threads {
            child.join().expect("fgsfdf");
        }
        rx_o.recv().unwrap()
    }).max().unwrap();

    Ok(format!("{}", result))
}

