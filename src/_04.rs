use crate::utils::read_file;

#[derive(Debug, Clone, Copy)]
struct Elf {
    min: i32,
    max: i32,
}

pub fn run() {
    let input = read_file("input/04.in");

    let groups: Vec<Vec<Elf>> = input
        .split("\n")
        .map(|line| {
            let elves = line.split(",").collect::<Vec<&str>>();
            return elves
                .iter()
                .map(|elf| {
                    let range = elf.split("-").collect::<Vec<&str>>();
                    let range = range
                        .iter()
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    return Elf {
                        min: range[0],
                        max: range[1],
                    };
                })
                .collect::<Vec<Elf>>();
        })
        .collect();

    let p1: i32 = groups
        .iter()
        .map(|group| {
            let first = group[0];
            let second = group[1];

            if (first.min <= second.min && first.max >= second.max)
                || (second.min <= first.min && second.max >= first.max)
            {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("Part 1: {}", p1);

    let p2: i32 = groups
        .iter()
        .map(|group| {
            let first = group[0];
            let second = group[1];

            if (first.min <= second.min && first.max >= second.min)
                || (second.min <= first.min && second.max >= first.min)
                || (first.min <= second.max && first.max >= second.max)
                || (second.min <= first.max && second.max >= first.max)
            {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("Part 2: {}", p2);
}
