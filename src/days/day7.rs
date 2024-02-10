use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::Lines;

#[derive(Debug)]
enum Command {
    CD(String),
    LS,
    Dir(String),
    File(i32, String),
}

#[derive(Debug)]
struct Directory {
    files: HashMap<String, i32>,
    dirs: HashMap<String, Directory>,
}

impl Directory {
    fn add_dir(&mut self, name: String, dir: Directory) {
        self.dirs.insert(name, dir);
    }

    fn add_file(&mut self, name: String, size: i32) {
        self.files.insert(name, size);
    }

    fn total_size(&self) -> i32 {
        let mut total = 0;

        for (_, file) in self.files.iter() {
            total += *file;
        }

        for (_, dir) in self.dirs.iter() {
            total += dir.total_size();
        }

        total
    }
}

pub fn solution(path: &str) -> (i32, i32) {
    let file_content =
        fs::read_to_string(Path::new(path)).expect("Should be able to read file");

    let lines = file_content.lines();

    (part1(lines.clone()), part2(lines.clone()))
}

fn part1(lines: Lines) -> i32 {
    let commands = parse_input(lines);

    let (_, file_sizes) = create_file_tree(commands);

    file_sizes.iter().filter(|x| **x <= 100000).sum()
}

fn part2(lines: Lines) -> i32 {
    let commands = parse_input(lines);

    let (file_tree, file_sizes) = create_file_tree(commands);

    let mut big_enough: Vec<_> = file_sizes
        .iter()
        .filter(|x| **x >= file_tree.total_size() - 40000000)
        .collect();
    big_enough.sort();

    *big_enough[0]
}

fn parse_input(lines: Lines) -> Vec<Command> {
    let mut commands = vec![];

    for line in lines {
        if line.starts_with("$ cd") {
            commands.push(Command::CD(String::from(line.split(" ").last().unwrap())));
        } else if line == "$ ls" {
            commands.push(Command::LS);
        } else if line.starts_with("dir") {
            commands.push(Command::Dir(String::from(line.split(" ").last().unwrap())));
        } else {
            let (size, name) = line.split_once(" ").unwrap();
            commands.push(Command::File(size.parse().unwrap(), String::from(name)));
        }
    }

    commands
}

fn create_file_tree(commands: Vec<Command>) -> (Directory, Vec<i32>) {
    let mut file_tree = Directory {
        files: HashMap::new(),
        dirs: HashMap::new(),
    };

    let mut file_sizes: Vec<i32> = vec![];

    let mut current_dir: Vec<&String> = vec![];

    let mut commands_iter = commands.iter();

    commands_iter.next();

    while let Some(cmd) = commands_iter.next() {
        match cmd {
            Command::LS => {
                continue;
            }
            Command::Dir(name) => {
                let mut curr = &mut file_tree;
                for dir_name in current_dir.iter() {
                    curr = curr.dirs.get_mut(*dir_name).unwrap();
                }

                curr.add_dir(
                    name.clone().to_string(),
                    Directory {
                        dirs: HashMap::new(),
                        files: HashMap::new(),
                    },
                );
            }
            Command::File(size, name) => {
                let mut curr = &mut file_tree;
                for dir_name in current_dir.iter() {
                    curr = curr.dirs.get_mut(*dir_name).unwrap();
                }

                curr.add_file(name.clone().to_string(), *size);
            }
            Command::CD(dir) => {
                if dir == ".." {
                    let mut curr = &mut file_tree;
                    for dir_name in current_dir.iter() {
                        curr = curr.dirs.get_mut(*dir_name).unwrap();
                    }

                    file_sizes.push(curr.total_size());
                    current_dir.pop();
                } else {
                    current_dir.push(dir);
                }
            }
        }
    }

    file_sizes.push(file_tree.total_size());

    (file_tree, file_sizes)
}
