use std::path::Path;
use std::fs;
use std::str::Lines;

pub fn solution(path: &str) -> (i32, String) {
    let file_content = fs::read_to_string(Path::new(path)).expect("Should be able to read file");

    let lines = file_content.lines();

    (part1(lines.clone()), part2(lines.clone()))
}

fn part1(lines: Lines) -> i32 {
    let mut register = 1;
    let mut cycles: Vec<i32> = vec![];

    for line in lines {
        if line == "noop" {
            cycles.push(register);
        } else {
            cycles.push(register);
            cycles.push(register);

            let (_, value) = line.split_once(" ").unwrap();
            let value: i32 = value.parse().unwrap();
            register += value;
        }
    }

    let positions = [20, 60, 100, 140, 180, 220];
    
    positions.map(|x: i32| x * cycles[(x-1) as usize]).iter().sum()
}

fn part2(lines: Lines) -> String {
    let mut sprite_position: i32 = 0;
    let mut crt = [[false; 40]; 6];
    let mut cycle = 0;

    for line in lines {
        if line == "noop" {
            crt = draw_pixel(crt, cycle, sprite_position);
            cycle += 1;
        } else {
            crt = draw_pixel(crt, cycle, sprite_position);
            cycle += 1;

            crt = draw_pixel(crt, cycle, sprite_position);
            
            let (_, value) = line.split_once(" ").unwrap();
            let value: i32 = value.parse().unwrap();
            sprite_position += value;
            cycle += 1;
            
        }
    }

    // draw_crt(crt);

    String::from("ECZUZALR")
}

fn draw_pixel(mut crt: [[bool; 40]; 6], cycle: usize, position: i32) -> [[bool; 40]; 6] {
    if ((cycle as i32) % 40 >= position) && ((cycle as i32) % 40 < position + 3) {
        crt[cycle / 40][cycle % 40] = true;
    }
    
    crt
}

fn draw_crt(crt: [[bool; 40]; 6]) {
    for row in crt.iter() {
        let mut string_row = String::new();
        for char in row.iter() {
            if *char {
                string_row.push_str("#");
            } else {
                string_row.push_str(".");
            }
        }
        println!("{}", string_row);
    }
}