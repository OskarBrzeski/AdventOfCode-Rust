use std::fs;
use std::path::Path;
use std::str::Lines;

#[derive(Debug)]
struct Directory {
    files: Vec<i32>,
    dirs: Vec<Directory>,
}

impl Directory {
    fn total_size(&self) -> i32 {
        let mut total = 0;

        for file in self.files.iter() {
            total += file;
        }

        for dir in self.dirs.iter() {
            total += dir.total_size();
        }

        total
    }
}

pub fn solution() -> (i32, i32) {
    let file_content = fs::read_to_string(Path::new("data/input7.txt"))
        .expect("Should be able to read file");

    let lines = file_content.lines();

    (part1(), part2())
}

fn part1() -> i32 {
    0
}

fn part2() -> i32 {
    0
}

fn create_file_tree(lines: Lines) -> Directory {
    let root = Directory {
        files: vec![],
        dirs: vec![],
    };

    root
}