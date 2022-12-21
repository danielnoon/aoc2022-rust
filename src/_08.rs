use crate::utils::read_file;

fn path(forest: &Vec<Vec<i32>>, start: (i32, i32), velocity: (i32, i32)) -> bool {
    let get = |(x, y): (i32, i32)| forest.get(y as usize).and_then(|row| row.get(x as usize));
    let mv = |(x, y): (i32, i32)| (x + velocity.0, y + velocity.1);

    let start_height = get(start).unwrap();

    let mut current = mv(start);

    while let Some(height) = get(current) {
        if height >= start_height {
            return false;
        }

        current = mv(current);
    }

    true
}

fn scenic(forest: &Vec<Vec<i32>>, start: (i32, i32), velocity: (i32, i32)) -> i32 {
    let get = |(x, y): (i32, i32)| forest.get(y as usize).and_then(|row| row.get(x as usize));
    let mv = |(x, y): (i32, i32)| (x + velocity.0, y + velocity.1);

    let start_height = get(start).unwrap();

    let mut current = mv(start);

    if get(current) == None {
        return 0;
    }

    let mut score = 0;

    while let Some(height) = get(current) {
        score += 1;

        if height >= start_height {
            return score;
        }

        current = mv(current);
    }

    score
}

pub fn run() {
    let input = read_file("input/08.in");

    let trees: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.trim()
                .split("")
                .filter_map(|num| match num.parse::<i32>() {
                    Ok(num) => Some(num),
                    Err(_) => None,
                })
                .collect()
        })
        .collect();

    let res = trees
        .iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .map(|(col, _)| {
                    vec![
                        path(&trees, (col as i32, row as i32), (1, 0)),
                        path(&trees, (col as i32, row as i32), (-1, 0)),
                        path(&trees, (col as i32, row as i32), (0, 1)),
                        path(&trees, (col as i32, row as i32), (0, -1)),
                    ]
                    .iter()
                    .any(|&x| x)
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    println!("Part 1: {:?}", res.iter().flatten().filter(|&&x| x).count());

    let res = trees
        .iter()
        .enumerate()
        .map(|(row, v)| {
            v.iter()
                .enumerate()
                .map(|(col, _)| {
                    let scenery: Vec<i32> = vec![
                        scenic(&trees, (col as i32, row as i32), (1, 0)), // right
                        scenic(&trees, (col as i32, row as i32), (-1, 0)), // left
                        scenic(&trees, (col as i32, row as i32), (0, 1)), // down
                        scenic(&trees, (col as i32, row as i32), (0, -1)), // up
                    ];

                    return scenery.iter().product::<i32>();
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    println!("Part 2: {:?}", res.iter().flatten().max().unwrap());
}
