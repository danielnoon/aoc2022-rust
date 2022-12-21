#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Op {
    Addx(i32),
    Noop,
    None,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum State {
    Normal,
    AddCycle,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pixel {
    On,
    Off,
}

struct CPU {
    ip: usize,
    cycle: i32,
    x: i32,
    instructions: Vec<Op>,
    state: State,
}

#[derive(Debug)]
struct CPUInfo {
    cycle: i32,
    x: i32,
}

struct CRT {
    pixels: Vec<Pixel>,
    width: u32,
}

impl CPU {
    fn new(instructions: Vec<Op>) -> Self {
        Self {
            ip: 0,
            cycle: 0,
            x: 1,
            instructions,
            state: State::Normal,
        }
    }

    fn current(&self) -> Option<Op> {
        self.instructions.get(self.ip).map(|op| *op)
    }

    fn info(&self) -> CPUInfo {
        CPUInfo {
            cycle: self.cycle,
            x: self.x,
        }
    }
}

impl Iterator for CPU {
    type Item = CPUInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ip >= self.instructions.len() {
            return None;
        }

        let op = self.current().unwrap_or(Op::None);

        let result = self.info();

        match op {
            Op::Addx(x) => match self.state {
                State::Normal => {
                    self.state = State::AddCycle;
                }
                State::AddCycle => {
                    self.x += x;
                    self.state = State::Normal;
                    self.ip += 1;
                }
            },
            Op::Noop => self.ip += 1,
            Op::None => panic!("Unknown op: {:?}", op),
        }

        self.cycle += 1;

        Some(result)
    }
}

impl CRT {
    fn new(width: u32, height: u32) -> Self {
        Self {
            pixels: vec![Pixel::Off; (width * height) as usize],
            width,
        }
    }

    fn set(&mut self, n: i32) {
        self.pixels[n as usize] = Pixel::On;
    }

    fn print(&self) {
        for x in 0..self.pixels.len() {
            let pixel = self.pixels[x];
            match pixel {
                Pixel::On => print!("#"),
                Pixel::Off => print!("."),
            }
            if (x + 1) % self.width as usize == 0 {
                println!();
            }
        }
    }
}

pub fn run() {
    let input = include_str!("../input/10.in");

    let instructions = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let op = parts.next().unwrap();
            let arg = parts.next().and_then(|n| n.parse::<i32>().ok());

            match op {
                "addx" => Op::Addx(arg.unwrap()),
                "noop" => Op::Noop,
                _ => panic!("Unknown op: {}", op),
            }
        })
        .collect::<Vec<_>>();

    let state = CPU::new(instructions.clone());

    let interesting_values: i32 = state
        .into_iter()
        .map(|CPUInfo { cycle, x }| (cycle + 1, x))
        .filter(|(cycle, _)| (cycle + 20) % 40 == 0)
        .map(|(cycle, x)| cycle as i32 * x)
        .sum();

    println!("Part 1: {}", interesting_values);

    let cpu = CPU::new(instructions.clone());
    let mut crt = CRT::new(40, 6);

    for CPUInfo { cycle, x } in cpu {
        let n = (cycle) % 40;
        if (x - n).abs() < 2 {
            crt.set(cycle);
        }
    }

    println!("Part 2:");
    crt.print();
}
