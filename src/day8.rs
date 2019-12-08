use std::io::{BufReader, Read};
use std::fs::File;
use crate::BoxResult;
use std::str::from_utf8_unchecked;
use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const IMAGE_SIZE: usize = WIDTH * HEIGHT;

const BLACK_SQUARE: [u8; 3] = [0xE2, 0x96, 0xA0];
const WHITE_SQUARE: [u8; 3] = [0xE2, 0x96, 0xA1];
const NOTHING_YET: [u8; 3] = [0xE2, 0x88, 0x85];

struct OneTwo {
    ones: i32,
    twos: i32,
}

pub fn p1(mut input: BufReader<File>) -> BoxResult<String> {
    let mut image: String = String::new();
    let _size = input.read_to_string(&mut image).unwrap();

    let layers: Vec<&str> = image.as_bytes()
        .chunks(IMAGE_SIZE)
        .map(|buf| unsafe { from_utf8_unchecked(buf) })
        .sorted_by_key(|layer| {
            layer.chars().filter(|c| *c == '0').count()
        })
        .collect();

    let onetwo = layers.first().unwrap().chars().fold(OneTwo {ones: 0, twos: 0}, |acc, c| {
        match c {
            '1' => OneTwo {ones: acc.ones + 1, twos: acc.twos},
            '2' => OneTwo {ones: acc.ones, twos: acc.twos + 1},
            _ => acc
        }
    });

    Ok(format!("{}", onetwo.ones * onetwo.twos))
}

pub fn p2(mut input: BufReader<File>) -> BoxResult<String> {
    let mut image: String = String::new();
    let _size = input.read_to_string(&mut image).unwrap();

    let mut actual_image: Vec<String> = vec![String::from_utf8(NOTHING_YET.to_vec()).unwrap(); IMAGE_SIZE];

    for (i, c) in image.chars().enumerate() {
        actual_image[i % IMAGE_SIZE] = if actual_image[i % IMAGE_SIZE] == String::from_utf8(NOTHING_YET.to_vec()).unwrap() {
            match c {
                '0' => String::from_utf8(BLACK_SQUARE.to_vec()).unwrap(),
                '1' => String::from_utf8(WHITE_SQUARE.to_vec()).unwrap(),
                _ => String::from_utf8(NOTHING_YET.to_vec()).unwrap(),
            }
        } else {actual_image[i % IMAGE_SIZE].clone()};
    }

    let mut out = String::new();
    out.push_str("\n");
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            out.push_str(&actual_image[i * WIDTH + j]);
        }
        out.push_str("\n");
    }

    Ok(format!("{}", out))

}