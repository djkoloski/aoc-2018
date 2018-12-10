mod topological_sort;

use std::{
    env,
    fs::File,
    io::prelude::*,
};
use topological_sort::TopologicalSort;

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {}", part_1(buffer.as_str()));
    println!("Part 2: {}", part_2(buffer.as_str()));
}

fn parse_data(data: &str) -> TopologicalSort<char> {
    let mut ts = TopologicalSort::<char>::new();
    for line in data.lines() {
        let before = line.chars().nth(5).unwrap();
        let after = line.chars().nth(36).unwrap();
        ts.add_dependency(before, after);
    }
    ts
}

fn advance(ts: &mut TopologicalSort<char>) -> Option<char> {
    let next = {
        let mut round = ts.peek_all();
        if round.len() == 0 {
            None
        } else {
            round.sort();
            Some(*round[0])
        }
    };
    if next.is_some() {
        ts.remove(&next.unwrap());
    }
    next
}

fn part_1(data: &str) -> String {
    let mut ts = parse_data(data);

    let mut result = String::new();
    while let Some(next) = advance(&mut ts) {
        result.push(next);
    }

    result
}

fn part_2(data: &str) -> i32 {
    let mut ts = parse_data(data);

    const MAX_TASKS: usize = 5;
    const SECONDS_PER_TASK: u8 = 61;
    let mut tasks = Vec::<(char, u8)>::new();
    let mut time = 0;
    loop {
        // Remove finished tasks
        let mut i = 0;
        while i < tasks.len() {
            if tasks[i].1 == 0 {
                ts.remove(&tasks[i].0);
                tasks.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // Assign new tasks
        while tasks.len() < MAX_TASKS {
            let round = ts.peek_all();
            if let Some(&next) = round.iter().filter(|&&a| !tasks.iter().any(|(b, _)| a == b)).min() {
                tasks.push((*next, SECONDS_PER_TASK + *next as u8 - 'A' as u8));
            } else {
                break;
            }
        }

        // Break if no tasks remain
        if tasks.len() == 0 {
            break;
        }

        // Step
        time += 1;
        for (_, ref mut time) in tasks.iter_mut() {
            *time -= 1;
        }
    }

    time
}