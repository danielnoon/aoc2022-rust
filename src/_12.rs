pub fn run() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    let start_y = input.lines().position(|l| l.contains('S')).unwrap();
    let start_x = input.lines().nth(start_y).unwrap().find('S').unwrap();
    let start = (start_x, start_y);

    let end_y = input.lines().position(|l| l.contains('E')).unwrap();
    let end_x = input.lines().nth(end_y).unwrap().find('E').unwrap();
    let end = (end_x, end_y);

    let graph = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let locations = vec![
                        (x, y.saturating_sub(1)),
                        (x, y + 1),
                        (x.saturating_sub(1), y),
                        (x + 1, y),
                    ]
                    .iter()
                    .filter(|(x1, y1)| x != *x1 || y != *y1)
                    .filter(|(x1, y1)| {
                        if let Some(line) = input.lines().nth(*y1) {
                            if let Some(c1) = line.chars().nth(*x1) {
                                println!("{} {}", c, c1);
                                if (c == 'S') && (c1 == 'a') {
                                    true
                                } else if (c1 as i32 - c as i32) < 1 {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .map(|(x1, y1)| (*x1, *y1))
                    .collect::<Vec<_>>();

                    ((x, y), locations)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // println!("Graph: {:?}", graph);
    graph.iter().for_each(|line| {
        line.iter().for_each(|(loc, edges)| {
            println!("{:?}:", loc);
            edges.iter().for_each(|edge| {
                println!("  {:?}", edge);
            });
        });
        println!();
    });
}
