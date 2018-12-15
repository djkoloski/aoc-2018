use std::{
    env,
    fs::File,
    io::prelude::*,
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {}", part_1(buffer.as_str()));
    println!("Part 2: {}", part_2(buffer.as_str()));
}

fn combine(digits: &mut Vec<u8>, first: &mut usize, second: &mut usize) {
    let mut sum = digits[*first] + digits[*second];
    if sum == 0 {
        digits.push(0);
    } else {
        let start = digits.len();
        while sum > 0 {
            digits.push(sum % 10);
            sum /= 10;
        }
        digits[start..].reverse();
    }

    *first = (*first + 1 + digits[*first] as usize) % digits.len();
    *second = (*second + 1 + digits[*second] as usize) % digits.len();
}

fn part_1(data: &str) -> String {
    let iterations = data.parse::<usize>().unwrap();
    let mut digits = vec![3, 7];
    let mut first = 0;
    let mut second = 1;

    for _ in 0..iterations + 10 {
        combine(&mut digits, &mut first, &mut second);
    }

    let mut result = String::new();
    for i in iterations..iterations + 10 {
        let digit = (digits[i] as u8 + '0' as u8) as char;
        result.push(digit);
    }

    result
}

fn part_2(data: &str) -> usize {
    let pattern: Vec<_> = data.chars().map(|c| c as u8 - '0' as u8).collect();
    let mut digits = vec![3, 7];
    let mut first = 0;
    let mut second = 1;

    let mut checked = 0;
    'check_all: loop {
        if digits.len() >= pattern.len() {
            while checked <= digits.len() - pattern.len() {
                if digits[checked..checked + pattern.len()].iter().zip(pattern.iter()).all(|(a, b)| a == b) {
                    break 'check_all checked;
                }
                checked += 1;
            }
        }
        combine(&mut digits, &mut first, &mut second);
    }
}