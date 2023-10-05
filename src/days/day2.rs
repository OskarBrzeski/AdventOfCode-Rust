use std::fs;
use std::path::Path;

pub fn solution() -> (isize, isize) {
    let file_content = fs::read_to_string(Path::new("data/input2.txt"))
        .expect("Should be able to read file");

    let lines: Vec<&str> = file_content.split("\r\n").collect();

    let mut score: isize = 0;

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
            _ => 0,
        }
    }
    
    return (score, 0)
}