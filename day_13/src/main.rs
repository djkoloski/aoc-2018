use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::prelude::*,
};

fn main() {
    let mut file = File::open(env::args().nth(1).unwrap()).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("Part 1: {:?}", part_1(buffer.as_str()));
    println!("Part 2: {:?}", part_2(buffer.as_str()));
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Turning {
    Left,
    Straight,
    Right,
}

#[derive(Eq, PartialEq)]
struct Cart {
    position: (usize, usize),
    facing: Direction,
    turning: Turning,
}

impl Cart {
    fn new(x: usize, y: usize, facing: Direction) -> Cart {
        Cart {
            position: (x, y),
            facing,
            turning: Turning::Left,
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        if self.position.1 == other.position.1 {
            if self.position.0 == other.position.0 {
                Ordering::Equal
            } else {
                self.position.0.cmp(&other.position.0)
            }
        } else {
            self.position.1.cmp(&other.position.1)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Horizontal,
    Vertical,
    RightUpLeftDown,
    RightDownLeftUp,
    Intersection,
}

struct Board {
    size: (usize, usize),
    tiles: Vec<Tile>,
}

impl Board {
    fn new(size: (usize, usize)) -> Board {
        Board {
            size,
            tiles: vec![Tile::Empty; size.0 * size.1],
        }
    }

    fn get(&self, position: (usize, usize)) -> Tile {
        self.tiles[position.0 + position.1 * self.size.0]
    }

    fn set(&mut self, position: (usize, usize), tile: Tile) {
        self.tiles[position.0 + position.1 * self.size.0] = tile;
    }
}

fn parse(data: &str) -> (Board, Vec<Cart>) {
    let mut board = Board::new((data.lines().next().unwrap().len(), data.lines().count()));
    let mut carts = Vec::new();

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                ' ' => Tile::Empty,
                '-' => Tile::Horizontal,
                '|' => Tile::Vertical,
                '/' => Tile::RightUpLeftDown,
                '\\' => Tile::RightDownLeftUp,
                '+' => Tile::Intersection,
                '>' => {
                    carts.push(Cart::new(x, y, Direction::Right));
                    Tile::Horizontal
                },
                '^' => {
                    carts.push(Cart::new(x, y, Direction::Up));
                    Tile::Vertical
                },
                '<' => {
                    carts.push(Cart::new(x, y, Direction::Left));
                    Tile::Horizontal
                },
                'v' => {
                    carts.push(Cart::new(x, y, Direction::Down));
                    Tile::Vertical
                },
                _ => panic!("Invalid input at ({}, {})", x, y),
            };
            board.set((x, y), tile);
        }
    }

    (board, carts)
}

fn move_cart(board: &Board, cart: &mut Cart) {
    cart.position = match cart.facing {
        Direction::Right => (cart.position.0 + 1, cart.position.1),
        Direction::Up => (cart.position.0, cart.position.1 - 1),
        Direction::Left => (cart.position.0 - 1, cart.position.1),
        Direction::Down => (cart.position.0, cart.position.1 + 1),
    };

    match board.get(cart.position) {
        Tile::Empty => panic!("Invalid cart position at ({}, {})", cart.position.0, cart.position.1),
        Tile::Horizontal | Tile::Vertical => (),
        Tile::RightUpLeftDown => {
            cart.facing = match cart.facing {
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Left,
            };
        },
        Tile::RightDownLeftUp => {
            cart.facing = match cart.facing {
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Right,
            };
        },
        Tile::Intersection => {
            cart.turning = match cart.turning {
                Turning::Right => {
                    cart.facing = match cart.facing {
                        Direction::Right => Direction::Down,
                        Direction::Up => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Down => Direction::Left,
                    };
                    Turning::Left
                },
                Turning::Straight => Turning::Right,
                Turning::Left => {
                    cart.facing = match cart.facing {
                        Direction::Right => Direction::Up,
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Right,
                    };
                    Turning::Straight
                },
            }
        },
    }
}

fn step(board: &Board, carts: &mut Vec<Cart>) -> Option<(usize, usize)> {
    carts.sort_unstable();

    for i in 0..carts.len() {
        move_cart(board, carts.get_mut(i).unwrap());
        if carts.iter().enumerate().any(|(j, c)| j != i && c.position == carts[i].position) {
            return Some(carts[i].position);
        }
    }

    None
}

fn step_remove(board: &Board, carts: &mut Vec<Cart>) {
    carts.sort_unstable();

    let mut i = 0;
    while i < carts.len() {
        move_cart(board, carts.get_mut(i).unwrap());
        if let Some((j, _)) = carts.iter().enumerate().find(|(j, c)| *j != i && c.position == carts[i].position) {
            carts.remove(j);
            if j < i {
                i -= 1;
            }
            carts.remove(i);
        } else {
            i += 1;
        }
    }
}

fn print(board: &Board, carts: &Vec<Cart>) {
    for y in 0..board.size.1 {
        for x in 0..board.size.0 {
            print!("{}",
                if let Some(cart) = carts.iter().find(|c| c.position == (x, y)) {
                    match cart.facing {
                        Direction::Right => '>',
                        Direction::Up => '^',
                        Direction::Left => '<',
                        Direction::Down => 'v'
                    }
                } else {
                    match board.get((x, y)) {
                        Tile::Empty => ' ',
                        Tile::Horizontal => '-',
                        Tile::Vertical => '|',
                        Tile::RightUpLeftDown => '/',
                        Tile::RightDownLeftUp => '\\',
                        Tile::Intersection => '+',
                    }
                }
            );
        }
        println!("");
    }
}

fn part_1(data: &str) -> (usize, usize) {
    let (board, mut carts) = parse(data);

    loop {
        if let Some(position) = step(&board, &mut carts) {
            break position;
        }
    }
}

fn part_2(data: &str) -> (usize, usize) {
    let (board, mut carts) = parse(data);

    loop {
        step_remove(&board, &mut carts);
        if carts.len() == 1 {
            break carts[0].position
        }
    }
}