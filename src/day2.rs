use std::io::{BufReader, BufRead};
use std::fs::File;
use std::i32;
use crate::BoxResult;
use std::str::{from_utf8};
use std::collections::HashMap;

fn calc_output(commands: &mut HashMap<i32, i32>) -> BoxResult<i32> {
    let mut i = 0;

    loop {
        match commands.get(&i) {
            Some(1) => {
                let addr_a = commands.get(&(i + 1)).unwrap();
                let addr_b = commands.get(&(i + 2)).unwrap();
                let addr_write = commands.get(&(i + 3)).unwrap().clone();
                let a = *commands.get(addr_a).unwrap();
                let b = *commands.get(addr_b).unwrap();
                commands.insert(addr_write, a + b);
            }
            Some(2) => {
                let addr_a = commands.get(&(i + 1)).unwrap();
                let addr_b = commands.get(&(i + 2)).unwrap();
                let addr_write = commands.get(&(i + 3)).unwrap().clone();
                let a = *commands.get(addr_a).unwrap();
                let b = *commands.get(addr_b).unwrap();
                commands.insert(addr_write, a * b);
            }
            Some(99) => {
                return Ok(*commands.get(&0).unwrap())
            }
            Some(f) => bail!("What the hell is {}", f),
            None => bail!("Noooooo")
        }
        i = i + 4;
    }
}

pub fn p1(input: BufReader<File>) -> BoxResult<String> {
    let mut commands = HashMap::new();

    let mut i = 0;
    for command in input.split(b',') {
        let parsed = from_utf8(&command.unwrap()).unwrap().parse::<i32>().unwrap();
        commands.insert(i, parsed);
        i = i + 1;
    }

    Ok(format!("{}", calc_output(&mut commands.clone()).unwrap()))
}

pub fn p2(input: BufReader<File>) -> BoxResult<String> {
    let mut commands = HashMap::new();

    let mut i = 0;
    for command in input.split(b',') {
        let parsed = from_utf8(&command.unwrap()).unwrap().parse::<i32>().unwrap();
        commands.insert(i, parsed);
        i = i + 1;
    }

    for i in 0..100 {
        for j in 0..100 {
            let mut commands_for_this_run = commands.clone();
            commands_for_this_run.insert(1, i);
            commands_for_this_run.insert(2, j);
            match calc_output(&mut commands_for_this_run) {
                Ok(19690720) => return Ok(format!("{}", 100 * i + j)),
                Ok(_) => {},
                Err(e) => bail!(e)
            }
        }
    }
    bail!("No combo found")
}