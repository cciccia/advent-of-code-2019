use crate::BoxResult;

fn has_two_adjecent_same_digits_and_never_decreases(num: i32) -> bool {
    let num_str = format!("{}", num);
    let num_str_bytes = num_str.as_bytes();
    let mut has_two_adjacent_digits = false;
    for i in 0..num_str.chars().count() - 1 {
        if num_str_bytes[i] == num_str_bytes[i+1] {
            has_two_adjacent_digits = true;
        }

        if num_str_bytes[i] > num_str_bytes[i+1] {
            return false;
        }
    }
    return has_two_adjacent_digits;
}

pub fn p1(input: &str) -> BoxResult<String> {
    let input_split: Vec<&str> = input.split("-").collect();
    let l_bound = input_split[0].parse::<i32>().unwrap();
    let u_bound = input_split[1].parse::<i32>().unwrap(); //exclusive? puzzle does not say

    let mut matches = 0;

    for i in l_bound..u_bound {
        if has_two_adjecent_same_digits_and_never_decreases(i) {
            matches = matches + 1;
        }
    }

    Ok(format!("{}", matches))

}

fn has_exactly_two_adjecent_same_digits_and_never_decreases(num: i32) -> bool {
    let num_str = format!("{}", num);
    let num_str_bytes = num_str.as_bytes();
    let mut has_two_adjacent_digits = false;

    for i in 0..num_str.chars().count() - 1 {
        if (num_str_bytes[i] == num_str_bytes[i+1])
            && (i == 4 || num_str_bytes[i+1] != num_str_bytes[i+2])
            && (i == 0 || num_str_bytes[i-1] != num_str_bytes[i]) {
            has_two_adjacent_digits = true;
        }

        if num_str_bytes[i] > num_str_bytes[i+1] {
            return false;
        }
    }
    return has_two_adjacent_digits;
}

pub fn p2(input: &str) -> BoxResult<String> {
    let input_split: Vec<&str> = input.split("-").collect();
    let l_bound = input_split[0].parse::<i32>().unwrap();
    let u_bound = input_split[1].parse::<i32>().unwrap(); //exclusive? puzzle does not say

    let mut matches = 0;

    for i in l_bound..u_bound {
        if has_exactly_two_adjecent_same_digits_and_never_decreases(i) {
            matches = matches + 1;
        }
    }

    Ok(format!("{}", matches))

}