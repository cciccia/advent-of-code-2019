use std::io::{BufReader, BufRead};
use std::fs::File;
use crate::BoxResult;
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use crate::intcode::calc_output;

pub fn p1(input: BufReader<File>) -> BoxResult<String> {
    let mut commands = HashMap::new();

    let mut i = 0;
    for command in input.split(b',') {
        let parsed = from_utf8(&command.unwrap()).unwrap().parse::<i64>().unwrap();
        commands.insert(i, parsed);
        i = i + 1;
    }

    let (tx_i, rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx, rx_o): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    tx_i.send(1).unwrap();

    calc_output(&mut commands, &rx, &tx).unwrap();

    let result = rx_o.recv().unwrap();

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

    let (tx_i, rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx, rx_o): (Sender<i64>, Receiver<i64>) = mpsc::channel();

    tx_i.send(2).unwrap();

    calc_output(&mut commands, &rx, &tx).unwrap();

    let result = rx_o.recv().unwrap();

    Ok(format!("{}", result))
}