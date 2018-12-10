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

fn node_total(numbers: &mut impl Iterator<Item = u32>) -> u32 {
    let mut total = 0;

    let child_nodes = numbers.next().unwrap();
    let metadata_entries = numbers.next().unwrap();

    for _ in 0..child_nodes {
        total += node_total(numbers);
    }

    total + numbers.take(metadata_entries as usize).sum::<u32>()
}

fn part_1(data: &str) -> u32 {
    node_total(&mut data.split(' ').map(|x| x.parse().unwrap()))
}

fn node_value(numbers: &mut impl Iterator<Item = u32>) -> u32 {
    let child_nodes = numbers.next().unwrap();
    let metadata_entries = numbers.next().unwrap();

    if child_nodes == 0 {
        numbers.take(metadata_entries as usize).sum::<u32>()
    } else {
        let child_values: Vec<_> = (0..child_nodes).map(|_| node_value(numbers)).collect();
        let mut total = 0;
        for _ in 0..metadata_entries {
            let index = numbers.next().unwrap();
            if index > 0 && index <= child_nodes {
                total += child_values[index as usize - 1];
            }
        }
        total
    }
}

fn part_2(data: &str) -> u32 {
    node_value(&mut data.split(' ').map(|x| x.parse().unwrap()))
}