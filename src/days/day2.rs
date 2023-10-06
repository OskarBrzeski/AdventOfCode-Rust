use std::fs;
use std::path::Path;
use std::str::Split;

pub fn solution() -> (isize, isize) {
    let file_content = fs::read_to_string(Path::new("data/input2.txt"))
        .expect("Should be able to read file");

    let lines = file_content.split("\r\n");
    
    return (part1(&lines), part2(&lines))
}

fn part1(lines: &Split<&str>) -> isize {
    let mut score: isize = 0;

    for line in lines.clone().into_iter() {
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
            _ => 0,
        }
    }

    score
}

fn part2(lines: &Split<&str>) -> isize {
    let mut score: isize = 0;

    for line in lines.clone().into_iter() {
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
            _ => -1,
        }
    }

    score
}