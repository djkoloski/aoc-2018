use std::{
    collections::HashSet,
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

fn part_1(data: &str) -> i32 {
    data.lines().map(|n| n.parse::<i32>().unwrap()).sum()
}

fn part_2(data: &str) -> i32 {
    let mut visited = HashSet::new();
    let mut total = 0;
    visited.insert(total);

    for n in data.lines().map(|n| n.parse::<i32>().unwrap()).cycle() {
        total += n;
        if !visited.insert(total) {
            break;
        }
    }

    total
}