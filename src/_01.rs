use std::fs;

pub fn run() {
    let path = "input/01.in";

    let contents = fs::read_to_string(path).expect("Bruh");

    let lines = contents.split("\r\n");

    let mut elves: Vec<Vec<i32>> = vec![Vec::new()];

    for line in lines {
        if line == "" {
            elves.push(Vec::new());
        } else {
            let n = line.parse::<i32>().unwrap();
            elves.last_mut().unwrap().push(n);
        }
    }

    let sums = elves
        .iter()
        .map(|x| x.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    println!("Part 1: {}", sums.iter().max().unwrap());

    let mut sum = 0;

    for _ in 0..3 {
        let max = sums.iter().max().unwrap();
        sum += max;
        let idx = sums.iter().position(|x| x == max).unwrap();
        elves.remove(idx);
    }

    println!("Part 2: {}", sum);
}
