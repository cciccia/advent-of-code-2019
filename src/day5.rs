use std::collections::HashMap;
use crate::BoxResult;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::from_utf8;

fn get_value(commands: &HashMap<i32, i32>, pos_or_value: i32, mode: i32) -> BoxResult<i32> {
    match mode {
        0 => {
            let addr = commands.get(&pos_or_value).unwrap();
            Ok(*commands.get(addr).unwrap())
        },
        1 => {
            Ok(*commands.get(&pos_or_value).unwrap())
        },
        f => {
            bail!("What the hell is {}", f)
        }
    }
}

fn calc_output(commands: &mut HashMap<i32, i32>, input: i32) -> BoxResult<String> {
    let mut i = 0;

    loop {
        let instruction_code = *commands.get(&i).unwrap();
        let opcode = instruction_code % 100;
        match opcode {
            1 => {
                let a = get_value(&commands, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, i+2, instruction_code / 1000 % 10).unwrap();
                let addr_write = get_value(&commands, i+3, 1).unwrap();
                commands.insert(addr_write, a + b);
                i = i + 4;
            }
            2 => {
                let a = get_value(&commands, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, i+2, instruction_code / 1000 % 10).unwrap();
                let addr_write = get_value(&commands, i+3, 1).unwrap();
                commands.insert(addr_write, a * b);
                i = i + 4;
            }
            3 => {
                let addr_write = get_value(&commands, i+1, 1).unwrap();
                commands.insert(addr_write, input);
                i = i + 2;
            }
            4 => {
                let v = format!("{}", get_value(&commands, i+1, 0).unwrap());
                println!("Output: {}", v);
                i = i + 2;
            }
            5 => {
                let a = get_value(&commands, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, i+2, instruction_code / 1000 % 10).unwrap();
                if a != 0 {
                    i = b;
                } else {
                    i = i + 3;
                }
            }
            6 => {
                let a = get_value(&commands, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, i+2, instruction_code / 1000 % 10).unwrap();
                if a == 0 {
                    i = b;
                } else {
                    i = i + 3;
                }
            }
            7 => {
                let a = get_value(&commands, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, i+2, instruction_code / 1000 % 10).unwrap();
                let addr_write = get_value(&commands, i+3, 1).unwrap();
                commands.insert(addr_write, if a < b {1} else {0});
                i = i + 4;
            }
            8 => {
                let a = get_value(&commands, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, i+2, instruction_code / 1000 % 10).unwrap();
                let addr_write = get_value(&commands, i+3, 1).unwrap();
                commands.insert(addr_write, if a == b {1} else {0});
                i = i + 4;
            }
            99 => {
                return Ok("done".to_string())
            }
            f => bail!("What the hell is {}", f),
        }
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

    Ok(calc_output(&mut commands.clone(), 1).unwrap())
}

pub fn p2(input: BufReader<File>) -> BoxResult<String> {
    let mut commands = HashMap::new();

    let mut i = 0;
    for command in input.split(b',') {
        let parsed = from_utf8(&command.unwrap()).unwrap().parse::<i32>().unwrap();
        commands.insert(i, parsed);
        i = i + 1;
    }

    Ok(calc_output(&mut commands.clone(), 5).unwrap())
}