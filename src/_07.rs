use itertools::Itertools;

use crate::utils::read_file;
use std::collections::HashMap;

#[derive(Debug)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug)]
struct Folder {
    id: usize,
    name: String,
    files: Vec<usize>,
    folders: Vec<usize>,
    parent: Option<usize>,
}

#[derive(Debug)]
struct Filesystem {
    files: HashMap<usize, File>,
    folders: HashMap<usize, Folder>,
}

#[derive(Debug, PartialEq)]
enum Phase {
    Shell,
    List,
}

#[derive(Debug)]
struct State {
    _current: usize,
    current_directory: usize,
    fs: Filesystem,
    phase: Phase,
}

impl File {
    fn new(size: usize) -> Self {
        Self { size, id: 0 }
    }
}

impl Folder {
    fn new(name: String, parent: Option<usize>) -> Self {
        match parent {
            Some(parent) => Self {
                id: 0,
                name,
                files: Vec::new(),
                folders: Vec::new(),
                parent: Some(parent),
            },
            None => Self {
                id: 0,
                name,
                files: Vec::new(),
                folders: Vec::new(),
                parent: None,
            },
        }
    }

    fn add_file(&mut self, file: usize) {
        self.files.push(file);
    }

    fn add_folder(&mut self, folder: usize) {
        self.folders.push(folder);
    }

    fn get_child<'a>(&'a self, name: &str, state: &'a State) -> Option<&Folder> {
        for folder in &self.folders {
            let result = state.fs.folders.get(folder);
            if let Some(result) = result {
                if result.name == name {
                    return Some(result);
                }
            }
        }

        return None;
    }

    fn get_size(&self, state: &State) -> usize {
        let mut size = 0;

        for file in &self.files {
            state
                .fs
                .files
                .get(file)
                .iter()
                .for_each(|file| size += file.size);
        }

        for folder in &self.folders {
            size += state.fs.folders.get(folder).unwrap().get_size(state);
        }

        size
    }
}

impl State {
    fn new() -> Self {
        let mut s = Self {
            current_directory: 0,
            _current: 0,
            fs: Filesystem {
                files: HashMap::new(),
                folders: HashMap::new(),
            },
            phase: Phase::Shell,
        };

        s.mkdir("/");

        s.fs.folders.get_mut(&0).unwrap().parent = None;
        s.fs.folders.get_mut(&0).unwrap().folders = Vec::new();

        s
    }

    fn cd(&mut self, name: &str) {
        if name == ".." {
            let current = self.fs.folders.get(&self.current_directory);
            if let Some(current) = current {
                if let Some(parent) = current.parent {
                    self.current_directory = parent;
                }
            }
        } else if name == "/" {
            self.current_directory = 0;
        } else {
            let current = self.fs.folders.get(&self.current_directory);
            if let Some(current) = current {
                let child = current.get_child(name, self);
                if let Some(child) = child {
                    self.current_directory = child.id;
                }
            }
        }
    }

    fn mkdir(&mut self, name: &str) {
        let mut folder = Folder::new(name.to_string(), Some(self.current_directory));
        let id = self._current;
        folder.id = id;
        self.fs.folders.insert(id, folder);
        self.fs
            .folders
            .get_mut(&self.current_directory)
            .unwrap()
            .add_folder(id);
        self._current += 1;
    }

    fn touch(&mut self, size: usize) {
        let mut file = File::new(size);
        let id = self._current;
        file.id = id;
        self.fs.files.insert(id, file);
        self.fs
            .folders
            .get_mut(&self.current_directory)
            .unwrap()
            .add_file(id);
        self._current += 1;
    }
}

pub fn run() {
    let input = read_file("input/07.in");

    let mut state = State::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        if state.phase == Phase::List {
            match parts.next() {
                Some("dir") => {
                    let name = parts.next().unwrap();
                    state.mkdir(name);
                }
                Some(size) => {
                    if let Ok(size) = size.parse() {
                        state.touch(size);
                    } else {
                        state.phase = Phase::Shell;
                    }
                }
                _ => {
                    state.phase = Phase::Shell;
                }
            }
        } else {
            parts.next();
        }

        if state.phase == Phase::Shell {
            match parts.next() {
                Some("cd") => {
                    let name = parts.next().unwrap();
                    state.cd(name);
                }
                Some("ls") => {
                    state.phase = Phase::List;
                }
                anything => {
                    println!("Unknown command {}", anything.unwrap())
                }
            }
        }
    }

    let sum = &state
        .fs
        .folders
        .iter()
        .map(|(_, folder)| folder.get_size(&state))
        .filter(|size| *size < 100000)
        .sum::<usize>();

    println!("Part 1: {}", sum);

    const CAPACITY: usize = 70000000;
    const NECESSARY: usize = 30000000;

    let current_utilization = state.fs.folders.get(&0).unwrap().get_size(&state);
    let current_capacity = CAPACITY - current_utilization;
    let min_deletion = NECESSARY - current_capacity;

    let folders = &state
        .fs
        .folders
        .iter()
        .map(|(_, folder)| (folder, folder.get_size(&state)))
        .filter(|(_, size)| *size > min_deletion)
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect::<Vec<_>>();

    println!("Part 2: {:?}", folders.last().unwrap());
}
