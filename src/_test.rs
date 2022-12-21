#[derive(Clone, Debug)]
struct Item {
    value: i32,
    destination: i32,
}

#[derive(Clone, Debug)]
struct Container {
    values: Vec<Item>,
}

impl Item {
    fn new(value: i32, destination: i32) -> Self {
        Self { value, destination }
    }
}

impl Container {
    fn new(values: Vec<(i32, i32)>) -> Self {
        Self {
            values: values
                .into_iter()
                .map(|(value, destination)| Item::new(value, destination))
                .collect(),
        }
    }
}

macro_rules! c {
    ($($x:expr),*) => {
        Container::new(vec![$($x),*])
    };
}

pub fn run() {
    let mut containers = vec![
        c!((1, 2), (2, 4), (3, 1)),
        c!((4, 1), (5, 2), (6, 3)),
        c!((7, 2), (8, 2), (9, 1)),
    ];

    let mut holder = vec![Container::new(vec![]); containers.len()];

    let length = containers.len();

    for (i, container) in containers.iter_mut().enumerate() {
        container.values.append(&mut holder[i].values);
        for item in container.values.drain(0..) {
            holder[(i + item.destination as usize) % length]
                .values
                .push(item);
        }
    }

    for (i, container) in containers.iter_mut().enumerate() {
        container.values.append(&mut holder[i].values);
    }

    for (i, container) in containers.iter().enumerate() {
        print!("{} => ", i);
        for item in container.values.iter() {
            print!("({}, {}) ", item.value, item.destination);
        }
        println!();
    }
}
