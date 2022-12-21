use crate::utils::read_file;
use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let input = read_file("input/06.in");

    let result: Vec<usize> = input
        .chars()
        .into_iter()
        .tuple_windows::<(_, _, _, _)>()
        .enumerate()
        .filter(|(_, (a, b, c, d))| {
            let set = HashSet::from([*a, *b, *c, *d]);
            set.len() == 4
        })
        .map(|(i, _)| i)
        .collect();

    if let Some(i) = result.first() {
        println!("Part 1: {}", *i + 4);
    }

    let som = result.iter().find(|i| {
        let start = **i;
        let end = start + 14;

        let set: HashSet<char> = HashSet::from_iter(input[start..end].chars());

        set.len() == 14
    });

    if let Some(i) = som {
        println!("Part 2: {}", *i + 14);
    }
}
