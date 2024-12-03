use std::{fs, ops::Mul, path::Path};

use regex::Regex;

fn extract_number_part_one(data: &String) -> Vec<(u64, u64)> {
    let mul_re = Regex::new(r"mul\((\d{1, 3}),(\d{1,3})\)").unwrap();
    mul_re
        .captures_iter(&data)
        .map(|c| (c[1].parse::<u64>().unwrap(), c[2].parse::<u64>().unwrap()))
        .collect::<Vec<(u64, u64)>>()
}

fn sum_uncorrupted_mul(data: &String) -> u64 {
    extract_number_part_one(&data)
        .into_iter()
        .fold(0 as u64, |acc, (n1, n2)| acc + n1.mul(n2))
}

fn main() {
    let data = match fs::read_to_string(Path::new("src/input.txt")) {
        Ok(filecontent) => filecontent,
        Err(e) => panic!("Error while opening file: {e:?}"),
    };
    let part_one = sum_uncorrupted_mul(&data);
    println!("Result part one: {}", part_one);
}
