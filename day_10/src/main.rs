use std::{
    cmp::{max, min},
    env,
    fs::File,
    io::prelude::*,
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    solve(buffer.as_str());
}

struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn evaluate(&self, time: i32) -> (i32, i32) {
        (self.x + self.vx * time, self.y + self.vy * time)
    }
}

fn parse_data(data: &str) -> Vec<Point> {
    data
        .lines()
        .map(|line|
            Point {
                x: line[10..16].trim().parse::<i32>().unwrap(),
                y: line[18..24].trim().parse::<i32>().unwrap(),
                vx: line[36..38].trim().parse::<i32>().unwrap(),
                vy: line[40..42].trim().parse::<i32>().unwrap(),
            }
        )
        .collect()
}

fn bounding_box(points: &Vec<Point>, time: i32) -> ((i32, i32), (i32, i32)) {
    let lower_left = points
        .iter()
        .map(|p| p.evaluate(time))
        .fold((i32::max_value(), i32::max_value()), |least, x| (min(least.0, x.0), min(least.1, x.1)));
    let upper_right = points
        .iter()
        .map(|p| p.evaluate(time))
        .fold((i32::min_value(), i32::min_value()), |greatest, x| (max(greatest.0, x.0), max(greatest.1, x.1)));
    (lower_left, upper_right)
}

fn print_points(points: &Vec<Point>, time: i32) {
    let (lower_left, upper_right) = bounding_box(points, time);

    let mut lines = Vec::new();
    for y in lower_left.1..=upper_right.1 {
        let mut line = Vec::new();
        line.resize((upper_right.0 - lower_left.0 + 1) as usize, '.');
        lines.push(line);
    }

    for point in points {
        let p = point.evaluate(time);
        lines[(p.1 - lower_left.1) as usize][(p.0 - lower_left.0) as usize] = '#';
    }

    for line in lines {
        println!("{}", line.iter().collect::<String>());
    }
}

fn solve(data: &str) {
    let points = parse_data(data);

    let mut least_area = i32::max_value();
    let mut least_time = 0;
    for i in 8000..12000 {
        let (lower_left, upper_right) = bounding_box(&points, i);
        let area = (upper_right.0 - lower_left.0) * (upper_right.1 - lower_left.1);
        if area < least_area {
            least_area = area;
            least_time = i;
        }
    }

    println!("Part 1:");
    print_points(&points, least_time);
    println!("Part 2: {}", least_time);
}