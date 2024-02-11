use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::str::Lines;

#[derive(Debug)]
enum Movement {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

pub fn solution(path: &str) -> (i32, i32) {
    let file_content = fs::read_to_string(Path::new(path)).expect("Should be able to read file");

    let lines = file_content.lines();

    let movements = parse_input(lines);

    (part1(&movements), part2(&movements))
}

fn part1(movements: &Vec<Movement>) -> i32 {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    let mut head = vec![0, 0];
    let mut tail = vec![0, 0];

    for movement in movements {
        let mut head_dx = 0;
        let mut head_dy = 0;
        let loop_amount;

        match movement {
            Movement::Up(amount) => {
                head_dy = 1;
                loop_amount = *amount;
            }
            Movement::Down(amount) => {
                head_dy = -1;
                loop_amount = *amount;
            }
            Movement::Left(amount) => {
                head_dx = -1;
                loop_amount = *amount;
            }
            Movement::Right(amount) => {
                head_dx = 1;
                loop_amount = *amount;
            }
        }

        for _ in 0..loop_amount {
            head[0] += head_dx;
            head[1] += head_dy;
            update_tail(&head, &mut tail);
            visited.insert((tail[0], tail[1]));
        }
    }

    visited.len() as i32
}

fn part2(movements: &Vec<Movement>) -> i32 {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    let mut rope_parts: Vec<Vec<isize>> = vec![
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
        vec![0, 0],
    ];

    for movement in movements {
        let mut head_dx = 0;
        let mut head_dy = 0;
        let loop_amount;

        match movement {
            Movement::Up(amount) => {
                head_dy = 1;
                loop_amount = *amount;
            }
            Movement::Down(amount) => {
                head_dy = -1;
                loop_amount = *amount;
            }
            Movement::Left(amount) => {
                head_dx = -1;
                loop_amount = *amount;
            }
            Movement::Right(amount) => {
                head_dx = 1;
                loop_amount = *amount;
            }
        };

        for _ in 0..loop_amount {
            rope_parts[0][0] += head_dx;
            rope_parts[0][1] += head_dy;

            for i in 1..10 {
                let mut proxy = vec![rope_parts[i][0], rope_parts[i][1]];
                update_tail(&rope_parts[i - 1], &mut proxy);
                rope_parts[i][0] = proxy[0];
                rope_parts[i][1] = proxy[1];
            }

            let last = rope_parts.last().unwrap();
            visited.insert((last[0], last[1]));
        }
    }

    visited.len() as i32
}

fn parse_input(lines: Lines) -> Vec<Movement> {
    let mut movements = vec![];

    for line in lines {
        let (dir, amount) = line.split_once(" ").unwrap();
        let amount_int: usize = amount.parse().unwrap();

        movements.push(match dir {
            "U" => Movement::Up(amount_int),
            "D" => Movement::Down(amount_int),
            "L" => Movement::Left(amount_int),
            "R" => Movement::Right(amount_int),
            _ => panic!("Unknown glyph"),
        })
    }

    movements
}

fn update_tail(head: &Vec<isize>, tail: &mut Vec<isize>) {
    if (head[0] - tail[0]).pow(2) + (head[1] - tail[1]).pow(2) <= 2 {
        return;
    }

    if head[1] == tail[1] {
        if head[0] - tail[0] > 1 {
            tail[0] += 1;
        } else {
            tail[0] -= 1;
        }

        return;
    }

    if head[0] == tail[0] {
        if head[1] - tail[1] > 1 {
            tail[1] += 1;
        } else {
            tail[1] -= 1;
        }

        return;
    }

    let dx;
    let dy;

    if head[0] > tail[0] {
        dx = 1;
    } else {
        dx = -1;
    }

    if head[1] > tail[1] {
        dy = 1;
    } else {
        dy = -1;
    }

    tail[0] += dx;
    tail[1] += dy;
}
