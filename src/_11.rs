use itertools::Itertools;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    n_inspected: i64,

    op_type: String,
    op_value: i64,
    op_old: bool,

    test: i64,
    on_true: i64,
    on_false: i64,
}

struct MonkeyMaker {
    items: Option<Vec<i64>>,

    op_type: Option<String>,
    op_value: Option<i64>,
    op_old: Option<bool>,

    test: Option<i64>,
    on_true: Option<i64>,
    on_false: Option<i64>,
}

#[derive(Debug, Clone)]
struct State {
    monkeys: Vec<Monkey>,
}

impl MonkeyMaker {
    fn new() -> Self {
        Self {
            items: None,
            op_type: None,
            op_value: None,
            op_old: None,
            test: None,
            on_true: None,
            on_false: None,
        }
    }

    fn items(&mut self, items: Vec<i64>) -> &mut Self {
        self.items = Some(items);
        self
    }

    fn op_type(&mut self, operation: String) -> &mut Self {
        self.op_type = Some(operation);
        self
    }

    fn op_value(&mut self, value: i64) -> &mut Self {
        self.op_value = Some(value);
        self
    }

    fn op_old(&mut self, old: bool) -> &mut Self {
        self.op_old = Some(old);
        self
    }

    fn test(&mut self, test: i64) -> &mut Self {
        self.test = Some(test);
        self
    }

    fn on_true(&mut self, on_true: i64) -> &mut Self {
        self.on_true = Some(on_true);
        self
    }

    fn on_false(&mut self, on_false: i64) -> &mut Self {
        self.on_false = Some(on_false);
        self
    }

    fn build(&self) -> Monkey {
        Monkey {
            items: self.items.clone().unwrap(),
            op_type: self.op_type.clone().unwrap(),
            op_value: self.op_value.unwrap(),
            op_old: self.op_old.unwrap(),
            test: self.test.unwrap(),
            on_true: self.on_true.unwrap(),
            on_false: self.on_false.unwrap(),
            n_inspected: 0,
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            monkeys: Vec::new(),
        }
    }
}

fn round(state: &mut State, worry_factor: i64) -> State {
    let mut holder: Vec<Vec<i64>> = vec![vec![]; state.monkeys.len()];

    for (i, monkey) in state.monkeys.iter_mut().enumerate() {
        monkey.items.append(&mut holder[i]);

        for item in monkey.items.drain(0..) {
            let new_item = match monkey.op_type.as_str() {
                "*" => {
                    if monkey.op_old {
                        item.checked_mul(item).unwrap_or_else(|| {
                            panic!("Overflow: {} * {}", item, item);
                        })
                    } else {
                        item * monkey.op_value
                    }
                }
                "+" => {
                    if monkey.op_old {
                        item + item
                    } else {
                        item + monkey.op_value
                    }
                }
                _ => panic!("Not implemented"),
            } % worry_factor;

            if new_item % monkey.test == 0 {
                holder[monkey.on_true as usize].push(new_item);
            } else {
                holder[monkey.on_false as usize].push(new_item);
            }

            monkey.n_inspected += 1;
        }
    }

    state
        .monkeys
        .iter_mut()
        .enumerate()
        .for_each(|(i, monkey)| monkey.items.append(&mut holder[i]));

    state.clone()
}

pub fn run() {
    let input = include_str!("../input/11.in");

    let mut initial_state = State::new();

    let mut current_monkey: Option<MonkeyMaker> = None;

    for line in input.lines() {
        if line.starts_with("Monkey") {
            let m = &current_monkey;
            if let Some(m) = m {
                initial_state.monkeys.push(m.build());
            }
            current_monkey = Some(MonkeyMaker::new());
        }

        if line.starts_with("  Starting items") {
            let items = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            current_monkey.as_mut().unwrap().items(items);
        }

        if line.starts_with("  Operation") {
            let operation = line.split(": ").nth(1).unwrap();
            let op_type = operation.split_whitespace().nth(3).unwrap();
            let op_value = operation
                .split_whitespace()
                .nth(4)
                .unwrap()
                .parse::<i64>()
                .unwrap_or(0);
            let op_old = operation.split_whitespace().nth(4).unwrap() == "old";

            current_monkey
                .as_mut()
                .unwrap()
                .op_type(op_type.to_string())
                .op_value(op_value)
                .op_old(op_old);
        }

        if line.starts_with("  Test") {
            let test = line.split(": ").nth(1).unwrap();
            let n = test.split_whitespace().nth(2).unwrap();
            current_monkey
                .as_mut()
                .unwrap()
                .test(n.parse::<i64>().unwrap());
        }

        if line.starts_with("    If true") {
            let on_true = line.split(": ").nth(1).unwrap();
            let target = on_true
                .split_whitespace()
                .nth(3)
                .unwrap()
                .parse::<i64>()
                .unwrap();
            current_monkey.as_mut().unwrap().on_true(target);
        }

        if line.starts_with("    If false") {
            let on_false = line.split(": ").nth(1).unwrap();
            let target = on_false
                .split_whitespace()
                .nth(3)
                .unwrap()
                .parse::<i64>()
                .unwrap();
            current_monkey.as_mut().unwrap().on_false(target);
        }
    }

    initial_state.monkeys.push(current_monkey.unwrap().build());

    let mut state = initial_state.clone();

    for _ in 0..20 {
        state = round(&mut state, 3);
    }

    println!(
        "Part 1: {:?}",
        state
            .monkeys
            .iter()
            .map(|x| x.n_inspected)
            .sorted_by(|a, b| b.cmp(a))
            .take(2)
            .product::<i64>()
    );

    let mut state = initial_state.clone();

    let worry_factor = state.monkeys.iter().map(|x| x.test).product::<i64>();

    for _ in 0..10000 {
        state = round(&mut state, worry_factor);
    }

    println!(
        "Part 2: {:?}",
        state
            .monkeys
            .iter()
            .map(|x| x.n_inspected)
            .sorted_by(|a, b| b.cmp(a))
            .take(2)
            .product::<i64>()
    );
}
