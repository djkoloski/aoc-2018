use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::prelude::*,
    usize,
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {}", part_1(buffer.as_str()));
    println!("Part 2: {}", part_2(buffer.as_str()));
}

fn parse_seeds(data: &str) -> Vec<(i32, i32)> {
    data.lines().map(|line| {
        let comma = line.find(",").unwrap();
        (line[..comma].parse::<i32>().unwrap(), line[comma + 2..].parse::<i32>().unwrap())
    }).collect()
}

fn bounds(seeds: &Vec<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let lower_left = seeds.iter().fold((i32::max_value(), i32::max_value()), |acc, x| (min(acc.0, x.0), min(acc.1, x.1)));
    let upper_right = seeds.iter().fold((0, 0), |acc, x| (max(acc.0, x.0), max(acc.1, x.1)));
    (lower_left, upper_right)
}

fn part_1(data: &str) -> usize {
    let seeds = parse_seeds(data);
    let (lower_left, upper_right) = bounds(&seeds);

    let mut index_to_area = HashMap::new();
    let mut border_areas = HashSet::new();

    for x in lower_left.0..=upper_right.0 {
        for y in lower_left.1..=upper_right.1 {
            let nearest_seed = seeds
                .iter()
                .map(|(sx, sy)| i32::abs(sx - x) + i32::abs(sy - y))
                .enumerate()
                .fold((None, i32::max_value()), |left, right| {
                    if right.1 < left.1 {
                        (Some(right.0), right.1)
                    } else if right.1 == left.1 {
                        (None, right.1)
                    } else {
                        left
                    }
                });
            if let (Some(nearest), _) = nearest_seed {
                *index_to_area.entry(nearest).or_insert(0) += 1;
                if x == lower_left.0 || x == upper_right.0 || y == lower_left.1 || y == upper_right.1 {
                    border_areas.insert(nearest);
                }
            }
        }
    }

    index_to_area.iter().filter(|(index, _)| !border_areas.contains(index)).map(|(_, &area)| area).max().unwrap()
}

fn part_2(data: &str) -> i32 {
    let seeds = parse_seeds(data);
    let (lower_left, upper_right) = bounds(&seeds);

    let mut total = 0;
    for x in lower_left.0..=upper_right.0 {
        for y in lower_left.1..=upper_right.1 {
            let total_distance = seeds
                .iter()
                .fold(0, |total, (sx, sy)| total + i32::abs(sx - x) + i32::abs(sy - y));
            if total_distance < 10000 {
                total += 1;
            }
        }
    }

    total
}