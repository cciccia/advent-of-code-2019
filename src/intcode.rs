use std::collections::HashMap;
use crate::BoxResult;
use std::sync::mpsc::{Sender, Receiver};


fn get_instruction_or_param(commands: &HashMap<i64, i64>, idx: i64) -> BoxResult<i64> {
    Ok(*commands.get(&idx).unwrap())
}

fn get_data(commands: &HashMap<i64, i64>, addr: i64) -> BoxResult<i64> {
    Ok(*(commands.get(&addr).or(Some(&0))).unwrap())
}

fn get_value(commands: &HashMap<i64, i64>, relative_base: i64, idx: i64, mode: i64) -> BoxResult<i64> {
    match mode {
        0 => {
            let addr = get_instruction_or_param(&commands, idx).unwrap();
            Ok(get_data(&commands, addr).unwrap())
        },
        1 => {
            Ok(get_instruction_or_param(&commands, idx).unwrap())
        },
        2 => {
            let addr = get_instruction_or_param(&commands, idx).unwrap() + relative_base;
            Ok(get_data(&commands, addr).unwrap())
        },
        f => {
            bail!("What the hell is {}", f)
        }
    }
}

fn set_value(commands: &mut HashMap<i64, i64>, relative_base: i64, idx: i64, mode: i64, value: i64) -> BoxResult<i64> {
    match mode {
        0 => {
            let addr = get_instruction_or_param(&commands, idx).unwrap();
            commands.insert(addr, value);
            Ok(1)
        },
        2 => {
            let addr = get_instruction_or_param(&commands, idx).unwrap() + relative_base;
            commands.insert(addr, value);
            Ok(1)
        },
        f => {
            bail!("What the hell is {}", f)
        }
    }
}

pub fn calc_output(commands: &mut HashMap<i64, i64>, in_chan: &Receiver<i64>, out_chan: &Sender<i64>) -> BoxResult<String> {
    let mut i = 0;
    let mut relative_base = 0;

    loop {
        let instruction_code = *commands.get(&i).unwrap();
        let opcode = instruction_code % 100;

        match opcode {
            1 => {
                let a = get_value(&commands, relative_base,i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, relative_base, i+2, instruction_code / 1000 % 10).unwrap();
                set_value(commands, relative_base, i+3, instruction_code / 10000 % 10, a+b).unwrap();
                i = i + 4;
            }
            2 => {
                let a = get_value(&commands, relative_base,i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, relative_base,i+2, instruction_code / 1000 % 10).unwrap();
                set_value(commands, relative_base, i+3, instruction_code / 10000 % 10, a*b).unwrap();
                i = i + 4;
            }
            3 => {
                let received = in_chan.recv().unwrap();
                set_value(commands, relative_base, i+1, instruction_code / 100 % 10, received).unwrap();
                i = i + 2;
            }
            4 => {
                let sent = get_value(&commands, relative_base, i+1, instruction_code / 100 % 10).unwrap();
                out_chan.send(sent).unwrap();
                i = i + 2;
            }
            5 => {
                let a = get_value(&commands, relative_base, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, relative_base, i+2, instruction_code / 1000 % 10).unwrap();
                if a != 0 {
                    i = b;
                } else {
                    i = i + 3;
                }
            }
            6 => {
                let a = get_value(&commands, relative_base, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, relative_base, i+2, instruction_code / 1000 % 10).unwrap();
                if a == 0 {
                    i = b;
                } else {
                    i = i + 3;
                }
            }
            7 => {
                let a = get_value(&commands, relative_base, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, relative_base, i+2, instruction_code / 1000 % 10).unwrap();
                set_value(commands, relative_base, i+3, instruction_code / 10000 % 10, if a < b {1} else {0}).unwrap();
                i = i + 4;
            }
            8 => {
                let a = get_value(&commands, relative_base, i+1, instruction_code / 100 % 10).unwrap();
                let b = get_value(&commands, relative_base, i+2, instruction_code / 1000 % 10).unwrap();
                set_value(commands, relative_base, i+3, instruction_code / 10000 % 10, if a == b {1} else {0}).unwrap();
                i = i + 4;
            }
            9 => {
                let a = get_value(&commands, relative_base, i+1, instruction_code / 100 % 10).unwrap();
                relative_base = relative_base + a;
                i = i + 2;
            }
            99 => {
                return Ok("done".to_string())
            }
            f => bail!("What the hell is {}", f),
        }
    }
}