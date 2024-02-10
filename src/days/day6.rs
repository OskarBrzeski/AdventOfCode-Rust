use std::fs;
use std::path::Path;

pub fn solution(path: &str) -> (i32, i32) {
    let file_content = fs::read_to_string(Path::new(path))
        .expect("Should be able to read file");

    (
        part1(file_content.as_bytes()),
        part2(file_content.as_bytes()),
    )
}

fn part1(line: &[u8]) -> i32 {
    let mut stack: [u8; 4] = [line[0], line[1], line[2], line[3]];

    for (i, c) in line[4..].iter().enumerate() {
        stack[i%4] = *c;
        if all_different(&stack) {
            return (i+5) as i32
        }
    }
    0
}

fn part2(line: &[u8]) -> i32 {
    let mut stack: [u8; 14] = [0; 14];
    for (i, c) in line[..14].iter().enumerate() {
        stack[i] = *c;
    }

    for (i, c) in line[14..].iter().enumerate() {
        stack[i%14] = *c;
        if all_different(&stack) {
            return (i+15) as i32
        }
    }
    0
}

fn all_different(stack: &[u8]) -> bool {
    let mut known = [false; 26];

    for i in stack.iter() {
        let index = (i - 97) as usize;
        if known[index] {
            return false
        }
        known[index] = true;
    }

    true
}