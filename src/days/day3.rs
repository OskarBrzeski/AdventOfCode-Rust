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
    let mut result: i32 = 0;

    for line in lines {
        let line_bytes = line.as_bytes();
        let mut done = [false; 52];

        for i in 0..line.len()/2 {
            let character = line_bytes[i];
            done[convert(character)] = true;
        }

        for i in line.len()/2..line.len() {
            let character = line_bytes[i];
            let character_index = convert(character);

            if done[character_index] {
                result += (character_index + 1) as i32;
                break;
            }
        }
    }

    result
}

fn part2(lines: Lines) -> i32 {
    let mut result: i32 = 0;

    let lines: Vec<&str> = lines.collect();

    for i in 0..lines.len()/3 {
        let mut done = [false; 52*3];

        for j in 0..3 {
            let current_line = lines[i*3+j];
            let line_bytes = current_line.as_bytes();

            for i in 0..current_line.len() {
                let character = line_bytes[i];
                done[convert(character) + 52 * j] = true;
            }
        }

        for j in 0..52 {
            if done[j] && done[j + 52] && done[j + 104] {
                result += (j + 1) as i32;
            }
        }
    }

    result
}

fn convert(character: u8) -> usize {
    if (97 <= character) && (character <= 122) {
        return (character - 97) as usize
    } else if (65 <= character) && (character <= 90) {
        return (character - 65 + 26) as usize
    } else {
        panic!("Invalid byte {character}")
    }
}