use std::{
    cmp::min,
    env,
    fs::File,
    io::prelude::*,
    ops::{Add, AddAssign, Sub, SubAssign},
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {:?}", part_1(buffer.as_str()));
    println!("Part 2: {:?}", part_2(buffer.as_str()));
}

fn power(x: i32, y: i32, serial: i32) -> i32 {
    ((x + 10) * y + serial) * (x + 10) / 100 % 10 - 5
}

struct SummedAreaTable<T: Add + Sub> {
    size: (usize, usize),
    values: Vec<T>,
}

impl<T: Copy + Clone + Default + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign> SummedAreaTable<T> {
    fn new(size: (usize, usize), value: impl Fn(usize, usize) -> T) -> SummedAreaTable<T> {
        let mut values = vec![Default::default(); size.0 * size.1];

        values[0] = value(0, 0);
        for x in 1..size.0 {
            values[x] = value(x, 0) + values[x - 1];
        }
        for y in 1..size.1 {
            values[y * size.0] = value(0, y) + values[(y - 1) * size.0];
        }

        for y in 1..size.1 {
            for x in 1..size.0 {
                values[x + y * size.0] = value(x, y) + values[x - 1 + y * size.0] + values[x + (y - 1) * size.0] - values[x - 1 + (y - 1) * size.0];
            }
        }

        SummedAreaTable {
            size,
            values,
        }
    }

    fn total(&self, position: (usize, usize)) -> T {
        self.values[position.0 + position.1 * self.size.0]
    }

    fn value(&self, position: (usize, usize), size: (usize, usize)) -> T {
        let mut result = self.total((position.0 + size.0 - 1, position.1 + size.1 - 1));
        if position.0 > 0 {
            result -= self.total((position.0 - 1, position.1 + size.1 - 1));
        }
        if position.1 > 0 {
            result -= self.total((position.0 + size.0 - 1, position.1 - 1));
        }
        if position.0 > 0 && position.1 > 0 {
            result += self.total((position.0 - 1, position.1 - 1));
        }
        result
    }
}

fn part_1(data: &str) -> (usize, usize) {
    let serial = data.parse::<i32>().unwrap();
    let table = SummedAreaTable::new((300, 300), |x, y| power(x as i32, y as i32, serial));

    let mut greatest_square = i32::min_value();
    let mut greatest_coords = (0, 0);
    for y in 0..298 {
        for x in 0..298 {
            let square = table.value((x, y), (3, 3));
            if square > greatest_square {
                greatest_square = square;
                greatest_coords = (x, y);
            }
        }
    }

    greatest_coords
}

fn part_2(data: &str) -> (usize, usize, usize) {
    let serial = data.parse::<i32>().unwrap();
    let table = SummedAreaTable::new((300, 300), |x, y| power(x as i32, y as i32, serial));

    let mut greatest_square = i32::min_value();
    let mut greatest_coords = (0, 0, 0);
    for y in 0..300 {
        for x in 0..300 {
            for i in 1..=min(300 - x, 300 - y) {
                let square = table.value((x, y), (i, i));
                if square > greatest_square {
                    greatest_square = square;
                    greatest_coords = (x, y, i);
                }
            }
        }
    }

    greatest_coords
}