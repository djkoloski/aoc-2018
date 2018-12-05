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

fn reduce_polymer(chars: impl Iterator<Item = char>) -> usize {
    let mut stack = Vec::<char>::new();

    for c in chars {
        if stack.len() > 0 && stack[stack.len() - 1] != c && stack[stack.len() - 1].to_ascii_uppercase() == c.to_ascii_uppercase() {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    stack.len()
}

fn part_1(data: &str) -> usize {
    reduce_polymer(data.chars())
}

fn part_2(data: &str) -> usize {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .map(|removed_type| reduce_polymer(data.chars().filter(|c| c.to_ascii_uppercase() != removed_type)))
        .min()
        .unwrap()
}