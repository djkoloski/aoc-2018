use std::{
    collections::HashMap,
    env,
    fs::File,
    io::prelude::*,
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {}", part_1(buffer.as_str()));
    println!("Part 2: {}", part_2(buffer.as_str()).unwrap());
}

fn part_1(data: &str) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for name in data.lines() {
        let mut map = HashMap::new();
        for c in name.chars() {
            map.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        if map.values().any(|&x| x == 2) {
            twos += 1;
        }
        if map.values().any(|&x| x == 3) {
            threes += 1;
        }
    }

    twos * threes
}

fn part_2(data: &str) -> Option<String> {
    for (i, first) in data.lines().enumerate() {
        for second in data.lines().skip(i + 1) {
            let same = first.chars().zip(second.chars()).filter(|(a, b)| a == b).map(|(a, _)| a).collect::<String>();
            if same.len() == first.len() - 1 {
                return Some(same);
            }
        }
    }

    None
}