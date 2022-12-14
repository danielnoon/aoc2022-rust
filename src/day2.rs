use std::fs;

pub fn run() {
    let path = "input/02.in";

    let contents = fs::read_to_string(path).expect("Bruh");

    let mut sum = 0;

    for line in contents.split("\r\n") {
        if let [opponent, me] = line.split(" ").collect::<Vec<&str>>()[..2] {
            let shape_score = match me {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };

            let result_score = match me {
                "X" => match opponent {
                    "A" => 3,
                    "B" => 0,
                    "C" => 6,
                    _ => 0,
                },
                "Y" => match opponent {
                    "A" => 6,
                    "B" => 3,
                    "C" => 0,
                    _ => 0,
                },
                "Z" => match opponent {
                    "A" => 0,
                    "B" => 6,
                    "C" => 3,
                    _ => 0,
                },
                _ => 0,
            };

            sum += shape_score + result_score;
        }
    }

    println!("Part 1: {}", sum);

    let mut sum2 = 0;

    for line in contents.split("\r\n") {
        if let [opponent, goal] = line.split(" ").collect::<Vec<&str>>()[..2] {
            let me = match goal {
                "X" => match opponent {
                    "A" => "Z",
                    "B" => "X",
                    "C" => "Y",
                    _ => "",
                },
                "Y" => match opponent {
                    "A" => "X",
                    "B" => "Y",
                    "C" => "Z",
                    _ => "",
                },
                "Z" => match opponent {
                    "A" => "Y",
                    "B" => "Z",
                    "C" => "X",
                    _ => "",
                },
                _ => "",
            };

            let shape_score = match me {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };

            let result_score = match me {
                "X" => match opponent {
                    "A" => 3,
                    "B" => 0,
                    "C" => 6,
                    _ => 0,
                },
                "Y" => match opponent {
                    "A" => 6,
                    "B" => 3,
                    "C" => 0,
                    _ => 0,
                },
                "Z" => match opponent {
                    "A" => 0,
                    "B" => 6,
                    "C" => 3,
                    _ => 0,
                },
                _ => 0,
            };

            sum2 += shape_score + result_score;
        }
    }

    println!("Part 2: {}", sum2);
}
