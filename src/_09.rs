use crate::utils::read_file;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn ordered_pair(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn distance_to(&self, other: &Point) -> f32 {
        let (x1, y1) = self.ordered_pair();
        let (x2, y2) = other.ordered_pair();

        let sum = (x1 - x2).pow(2) + (y1 - y2).pow(2);

        (sum as f32).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct State1 {
    head: Point,
    tail: Point,
    last: Point,
    visited: HashSet<Point>,
}

struct State2 {
    rope: Vec<Point>,
    visited: HashSet<Point>,
}

impl State2 {
    fn min_x(&self) -> i32 {
        self.rope.iter().map(|p| p.x).min().unwrap()
    }

    fn max_x(&self) -> i32 {
        self.rope.iter().map(|p| p.x).max().unwrap()
    }

    fn min_y(&self) -> i32 {
        self.rope.iter().map(|p| p.y).min().unwrap()
    }

    fn max_y(&self) -> i32 {
        self.rope.iter().map(|p| p.y).max().unwrap()
    }

    fn print(&self) {
        let min_x = self.min_x();
        let max_x = self.max_x();
        let min_y = self.min_y();
        let max_y = self.max_y();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let point = Point { x, y };

                if self.rope.contains(&point) {
                    print!("{}", self.rope.iter().position(|p| p == &point).unwrap());
                } else if self.visited.contains(&point) {
                    print!("O");
                } else {
                    print!(".");
                }
            }

            println!();
        }

        println!("\n");
    }
}

pub fn run() {
    let input = read_file("input/09.in");

    let directions = HashMap::from([
        ("R", Point { x: 1, y: 0 }),
        ("L", Point { x: -1, y: 0 }),
        ("U", Point { x: 0, y: 1 }),
        ("D", Point { x: 0, y: -1 }),
    ]);

    let mut state1 = State1 {
        head: Point { x: 0, y: 0 },
        tail: Point { x: 0, y: 0 },
        last: Point { x: 0, y: 0 },
        visited: HashSet::new(),
    };

    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let distance = distance.trim().parse::<i32>().unwrap();

        let direction = directions.get(direction).unwrap();

        for _ in 0..distance {
            state1.head.x += direction.x;
            state1.head.y += direction.y;

            if state1.head.distance_to(&state1.tail) >= 2.0 {
                state1.tail = state1.last;
            }

            state1.visited.insert(state1.tail);
            state1.last = state1.head;
        }
    }

    println!("Part 1: {}", state1.visited.len());

    let mut state = State2 {
        rope: (0..10).map(|_| Point { x: 0, y: 0 }).collect(),
        visited: HashSet::new(),
    };

    for line in input.lines() {
        let (direction, distance) = line.split_at(1);

        let distance = distance.trim().parse::<i32>().unwrap();

        let direction = directions.get(direction).unwrap();

        for _ in 0..distance {
            state.rope[0].x += direction.x;
            state.rope[0].y += direction.y;

            for i in 1..state.rope.len() {
                if state.rope[i - 1].distance_to(&state.rope[i]) >= 2.0 {
                    state.rope[i] = match state.rope[i - 1] - state.rope[i] {
                        Point { x: 2, y: 0 } => Point {
                            x: state.rope[i].x + 1,
                            y: state.rope[i].y,
                        },
                        Point { x: -2, y: 0 } => Point {
                            x: state.rope[i].x - 1,
                            y: state.rope[i].y,
                        },
                        Point { x: 0, y: 2 } => Point {
                            x: state.rope[i].x,
                            y: state.rope[i].y + 1,
                        },
                        Point { x: 0, y: -2 } => Point {
                            x: state.rope[i].x,
                            y: state.rope[i].y - 1,
                        },
                        Point { x: 1..=2, y: 1..=2 } => Point {
                            x: state.rope[i].x + 1,
                            y: state.rope[i].y + 1,
                        },
                        Point {
                            x: -2..=-1,
                            y: 1..=2,
                        } => Point {
                            x: state.rope[i].x - 1,
                            y: state.rope[i].y + 1,
                        },
                        Point {
                            x: 1..=2,
                            y: -2..=-1,
                        } => Point {
                            x: state.rope[i].x + 1,
                            y: state.rope[i].y - 1,
                        },
                        Point {
                            x: -2..=-1,
                            y: -2..=-1,
                        } => Point {
                            x: state.rope[i].x - 1,
                            y: state.rope[i].y - 1,
                        },
                        _ => {
                            state.print();
                            panic!(
                                "Invalid point: {:?}: {:?} - {:?} = {:?}",
                                i,
                                state.rope[i - 1],
                                state.rope[i],
                                state.rope[i - 1] - state.rope[i]
                            )
                        }
                    }
                }
            }

            state.visited.insert(state.rope.last().unwrap().clone());
        }
    }

    println!("Part 2: {}", state.visited.len());
}
