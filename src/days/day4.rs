use std::fs;
use std::path::Path;
use std::str::Split;

pub fn solution() -> (isize, isize) {
    let file_content = fs::read_to_string(Path::new("data/input4.txt"))
        .expect("Should be able to read file");

    let lines = file_content.split("\r\n");

    (
        part1(lines.clone()),
        part2(lines.clone()),
    )
}

fn part1(lines: Split<&str>) -> isize {
    let mut count = 0;

    for line in lines {
        let values = get_values(line);
        if values[0] <= values[2] && values[1] >= values[3] {
            count += 1;
        } else if values[2] <= values[0] && values[3] >= values[1] {
            count += 1;
        }
    }

    count
}

fn part2(lines: Split<&str>) -> isize {
    let mut count = 0;

    for line in lines {
        let values = get_values(line);
        if !(values[1] < values[2] || values[0] > values[3]) {
            count += 1;
        }
    }

    count
}

fn get_values(line: &str) -> Vec<isize> {
    let (left, right) = line.split_once(",")
        .expect(format!("Not a standard line {}", line).as_str());

    let (left_lower, left_higher) = left.split_once("-")
        .expect(format!("Not a standard line {}", left).as_str());
    let (right_lower, right_higher) = right.split_once("-")
        .expect(format!("Not a standard line {}", right).as_str());

    vec![
        left_lower.parse().expect("Not a number"),
        left_higher.parse().expect("Not a number"),
        right_lower.parse().expect("Not a number"),
        right_higher.parse().expect("Not a number"),
    ]
}