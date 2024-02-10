use core::panic;
use std::fs;
use std::path::Path;
use std::str::Lines;

pub fn solution(path: &str) -> (i32, i32) {
    let file_content = fs::read_to_string(Path::new(path))
        .expect("Should be able to read file");

    let lines = file_content.lines();
    
    (
        part1(lines.clone()),
        part2(lines.clone()),
    )
}

fn part1(lines: Lines) -> i32 {
    let mut score: i32 = 0;

    for line in lines {
        score += match line {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!("Unknown Line ({})", line),
        }
    }

    score
}

fn part2(lines: Lines) -> i32 {
    let mut score: i32 = 0;

    for line in lines {
        score += match line {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!("Unknown Line ({})", line),
        }
    }

    score
}