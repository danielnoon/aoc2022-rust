use crate::utils::read_file;

#[derive(Debug)]
struct Stack {
    stack: Vec<char>,
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Stack>,
}

impl Stack {
    fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, c: char) {
        self.stack.push(c);
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop()
    }
}

impl Ship {
    fn new(size: usize) -> Ship {
        let mut ship = Ship { stacks: Vec::new() };

        for _ in 0..size {
            ship.stacks.push(Stack::new());
        }

        ship
    }

    fn push(&mut self, to: usize, c: char) {
        self.stacks[to].push(c);
    }

    fn move_crates(self, from: usize, to: usize, n: usize) -> Ship {
        let mut ship = self;

        for _ in 0..n {
            if let Some(c) = ship.stacks[from].pop() {
                ship.stacks[to].push(c);
            }
        }

        ship
    }

    fn move_crates_2(self, from: usize, to: usize, n: usize) -> Ship {
        let mut ship = self;

        let mut crates = Vec::new();

        for _ in 0..n {
            if let Some(c) = ship.stacks[from].pop() {
                crates.push(c);
            }
        }

        for c in crates.iter().rev() {
            ship.stacks[to].push(*c);
        }

        ship
    }

    fn get_top_crates(&self) -> String {
        let mut crates = Vec::new();

        for stack in self.stacks.iter() {
            if let Some(c) = stack.stack.last() {
                crates.push(*c);
            }
        }

        return crates.iter().collect();
    }
}

pub fn run() {
    let input = read_file("input/05.in");

    if let [blueprint, instructions] = input.split("\n\n").collect::<Vec<_>>()[..] {
        let mut ship = Ship::new(9);

        for line in blueprint.lines().rev().skip(1) {
            for (i, c) in line
                .chars()
                .enumerate()
                .filter(|(i, _)| *i > 0 && (*i - 1) % 4 == 0)
                .map(|(_, c)| c)
                .enumerate()
            {
                if c != ' ' {
                    ship.push(i, c);
                }
            }
        }

        for line in instructions.lines() {
            let mut words = line.split_whitespace();
            words.next();
            let n = words.next().unwrap().parse::<usize>().unwrap();
            words.next();
            let from = words.next().unwrap().parse::<usize>().unwrap();
            words.next();
            let to = words.next().unwrap().parse::<usize>().unwrap();

            ship = ship.move_crates(from - 1, to - 1, n);
        }

        println!("Part 1: {}", ship.get_top_crates());

        ship = Ship::new(9);

        for line in blueprint.lines().rev().skip(1) {
            for (i, c) in line
                .chars()
                .enumerate()
                .filter(|(i, _)| *i > 0 && (*i - 1) % 4 == 0)
                .map(|(_, c)| c)
                .enumerate()
            {
                if c != ' ' {
                    ship.push(i, c);
                }
            }
        }

        for line in instructions.lines() {
            let mut words = line.split_whitespace();
            words.next();
            let n = words.next().unwrap().parse::<usize>().unwrap();
            words.next();
            let from = words.next().unwrap().parse::<usize>().unwrap();
            words.next();
            let to = words.next().unwrap().parse::<usize>().unwrap();

            ship = ship.move_crates_2(from - 1, to - 1, n);
        }

        println!("Part 2: {}", ship.get_top_crates());
    }
}
