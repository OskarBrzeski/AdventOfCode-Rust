use std::fs;
use std::path::Path;
use std::str::{Lines, from_utf8};

pub fn solution() -> (String, String) {
    let file_content = fs::read_to_string(Path::new("data/input5.txt"))
        .expect("Should be able to read files");

    let lines = file_content.lines();

    (
        part1(lines.clone()),
        part2(lines.clone()),
    )
}

fn part1(mut lines: Lines) -> String {
    let mut config = starting_config(&mut lines);

    for line in lines {
        let (amount, from, to) = get_instructions(line);
        
        for _ in 0..amount {
            let value = config[from-1].pop().unwrap();
            config[to-1].push(value);
        }
    }
    
    array_to_string(config)
}

fn part2(mut lines: Lines) -> String {
    let mut config = starting_config(&mut lines);

    for line in lines {
        let (amount, from, to) = get_instructions(line);
        
        let mut temp: Vec<u8> = vec![];
        for _ in 0..amount {
            let value = config[from-1].pop().unwrap();
            temp.push(value);
        }
        for _ in 0..amount {
            let value = temp.pop().unwrap();
            config[to-1].push(value);
        }
    }
    
    array_to_string(config)
}

fn starting_config(lines: &mut Lines) -> [Vec<u8>; 9] {
    let mut yeah: Vec<&str> = vec![];

    for _ in 0..8 {
        yeah.push(lines.next().unwrap());
    }

    yeah.reverse();

    let mut config: [Vec<u8>; 9] = [
        vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![],
    ];

    for line in yeah {
        let line_bytes = line.as_bytes();

        for i in 0..9 {
            let c = line_bytes[4*i+1];

            if c != 32 {
                config[i].push(c);
            }
        }
    }

    lines.next();
    lines.next();

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

fn array_to_string(mut config: [Vec<u8>; 9]) -> String {
    let mut answer: [u8; 9] = [0; 9];
    for i in 0..9 {
        let value = config[i].pop().unwrap();
        answer[i] = value;
    }

    String::from(from_utf8(&answer).unwrap())
}