use std::{
    env,
    fs::File,
    io::prelude::*,
    iter::repeat,
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {}", part_1(buffer.as_str()));
    println!("Part 2: {}", part_2(buffer.as_str()));
}

fn advance<'a>(state: &Vec<bool>, rules: &[bool; 32]) -> Vec<bool> {
    let mut result = Vec::with_capacity(state.len() + 4);

    let mut window = 0;
    for value in state.iter().chain(repeat(&false).take(4)) {
        window = ((window << 1) & 0b11111) | if *value { 1 } else { 0 };
        result.push(rules[window]);
    }

    result
}

fn parse_data(data: &str) -> (Vec<bool>, [bool; 32]) {
    let mut initial_state = Vec::new();
    let mut lines = data.lines();
    for c in lines.next().unwrap()[15..].chars() {
        initial_state.push(c == '#');
    }

    lines.next();

    let mut rules = [false; 32];
    for line in lines {
        let mut index = 0;
        for c in line.chars().take(5) {
            index = (index << 1) | if c == '#' { 1 } else { 0 };
        }
        rules[index] = line.chars().nth(9).unwrap() == '#';
    }

    (initial_state, rules)
}

fn part_1(data: &str) -> i32 {
    let (mut state, rules) = parse_data(data);

    for i in 0..20 {
        state = advance(&state, &rules);
    }

    state.iter().enumerate().fold(0, |t, (i, &v)| if v { t + i as i32 - 40 } else { t })
}

fn part_2(data: &str) -> i64 {
    let (mut state, rules) = parse_data(data);

    const ITERS: i64 = 300;
    const MAX_ITERS: i64 = 50000000000;
    for i in 0..ITERS {
        state = advance(&state, &rules);
    }
    let total = state.iter().enumerate().fold(0, |t, (i, &v)| if v { t + i as i64 - ITERS as i64 * 2 } else { t });
    state = advance(&state, &rules);
    let next = state.iter().enumerate().fold(0, |t, (i, &v)| if v { t + i as i64 - (ITERS as i64 + 1) * 2 } else { t });

    total + (next - total) * (MAX_ITERS - ITERS)
}