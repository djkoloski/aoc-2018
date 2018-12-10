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
    println!("Part 2: {}", part_2(buffer.as_str()));
}

fn compute(data: &str, factor: u32) -> u32 {
    let mut words = data.split(' ');
    let players = words.next().unwrap().parse::<u32>().unwrap();
    let last_marble_value = words.skip(5).next().unwrap().parse::<u32>().unwrap() * factor;

    let mut state = vec![0];
    let mut current = 0;
    let mut player = 0;
    let mut scores = HashMap::new();
    for i in 1..=last_marble_value {
        if i % 23 != 0 {
            current = (current + 2) % state.len();
            state.insert(current, i);
        } else {
            current = (current + state.len() - 7) % state.len();
            *scores.entry(player).or_insert(0) += i + state[current];
            state.remove(current);
        }
        player = (player + 1) % players;
    }

    *scores.values().max().unwrap()
}

fn part_1(data: &str) -> u32 {
    compute(data, 1)
}

fn part_2(data: &str) -> u32 {
    compute(data, 100)
}