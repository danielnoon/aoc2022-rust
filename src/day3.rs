use std::collections::HashSet;
use std::fs;

pub fn run() {
    let path = "input/03.in";

    let contents = fs::read_to_string(path).expect("Bruh");

    let lines = contents.split("\r\n");

    let mut sum = 0;

    for line in lines.clone() {
        let mut set: HashSet<char> = HashSet::new();

        let n = line.len() / 2;

        let chars = line.chars().into_iter();

        for item in chars.clone().take(n) {
            set.insert(item);
        }

        for item in chars.clone().skip(n) {
            if set.contains(&item) {
                let i = item as i32;

                if i >= 'a' as i32 {
                    sum += i + 1 - 'a' as i32;
                } else {
                    sum += i + 1 - 'A' as i32 + 26;
                }

                break;
            }
        }
    }

    println!("Part 1: {}", sum);

    sum = 0;

    for group in lines
        .collect::<Vec<&str>>()
        .windows(3)
        .enumerate()
        .filter(|(i, _)| i % 3 == 0)
        .map(|(_, group)| group)
    {
        let mut first: HashSet<char> = HashSet::new();

        for item in group[0].chars() {
            first.insert(item);
        }

        let mut second: HashSet<char> = HashSet::new();

        for item in group[1].chars() {
            second.insert(item);
        }

        for item in group[2].chars() {
            if first.contains(&item) && second.contains(&item) {
                let i = item as i32;

                if i >= 'a' as i32 {
                    sum += i + 1 - 'a' as i32;
                } else {
                    sum += i + 1 - 'A' as i32 + 26;
                }

                break;
            }
        }
    }

    println!("Part 2: {}", sum);
}
