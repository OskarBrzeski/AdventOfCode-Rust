use std::fs;
use std::path::Path;
use std::str::{Lines, from_utf8};

pub fn solution() -> (String, i32) {
    let file_content = fs::read_to_string(Path::new("data/input5.txt"))
        .expect("Should be able to read files");

    let lines = file_content.lines();

    (
        part1(&mut lines.clone()),
        0,
    )
}

fn part1(lines: &mut Lines) -> String {
    let mut yeah: Vec<&str> = vec![];

    for _ in 0..8 {
        yeah.push(lines.next().unwrap());
    }

    yeah.reverse();

    let mut config = starting_config(yeah);

    lines.next();
    lines.next();

    for line in lines {
        let (amount, from, to) = get_instructions(line);
        for _ in 0..amount {
            let value = config[from-1].pop().unwrap();
            config[to-1].push(value);
        }
    }
    
    let mut answer: [u8; 9] = [0; 9];
    for i in 0..9 {
        let value = config[i].pop().unwrap();
        answer[i] = *value;
    }

    String::from(from_utf8(&answer).unwrap())
}

fn starting_config(lines: Vec<&str>) -> [Vec<&u8>; 9] {
    let mut config: [Vec<&u8>; 9] = [
        vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![],
    ];

    for line in lines {
        let line_bytes: Vec<&u8> = line.as_bytes().iter().collect();

        for i in 0..9 {
            let c = line_bytes[4*i+1];

            if *c != 32 {
                config[i].push(c);
            }
        }
    }

    config
}

fn get_instructions(line: &str) -> (usize, usize, usize) {
    let parts: Vec<&str> = line.split(" ").collect();
    
    (
        parts[1].parse().expect("Should be a number"),
        parts[3].parse().expect("Should be a number"),
        parts[5].parse().expect("Should be a number"),
    )
}