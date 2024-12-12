use std::collections::HashMap;

use crate::common_functions::open_file;

fn part_one(left_side: &mut Vec<i64>, right_side: &mut Vec<i64>) {
    let mut total_difference: i64 = 0;
    left_side.sort();
    right_side.sort();

    for i in 0..left_side.len() {
        let between = left_side[i] - right_side[i];
        if between < 0 {
            total_difference -= between
        } else {
            total_difference += between
        }
    }

    println!("Total Differnce: {:?}", total_difference)
}

fn part_two(left_side: &mut Vec<i64>, right_side: &mut Vec<i64>) {
    let mut freq_right_side: HashMap<i64, i64> = HashMap::new();
    for i in 0..right_side.len() {
        if !freq_right_side.contains_key(&right_side[i]) {
            freq_right_side.insert(right_side[i], 1);
        } else {
            match freq_right_side.get_mut(&right_side[i]) {
                Some(val) => *val += 1,
                None => {}
            }
        }
    }

    let mut total_sim: i64 = 0;

    for i in 0..left_side.len() {
        if !freq_right_side.contains_key(&left_side[i]) {
            continue;
        } else {
            let freq: i64 = match freq_right_side.get(&left_side[i]) {
                Some(val) => *val,
                None => 0,
            };
            total_sim += left_side[i] * freq
        }
    }

    println!("Simularity Score: {:?}", total_sim)
}

pub fn main() {
    let input = open_file("inputs/day_one.txt");
    let split_lines: Vec<&str> = input.split("\r\n").collect();
    let mut left_side: Vec<i64> = Vec::new();
    let mut right_side: Vec<i64> = Vec::new();

    for i in 0..split_lines.len() - 1 {
        let split_line: Vec<&str> = split_lines[i].split("   ").collect();
        left_side.push(split_line[0].parse::<i64>().unwrap());
        right_side.push(split_line[1].parse::<i64>().unwrap());
    }

    part_one(left_side.clone().as_mut(), right_side.clone().as_mut());
    part_two(left_side.clone().as_mut(), right_side.clone().as_mut());
}
