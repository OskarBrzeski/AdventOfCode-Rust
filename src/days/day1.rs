use std::fs;
use std::path::Path;

pub fn solution() -> (isize, isize) {
    let file_content = fs::read_to_string(Path::new("data/input1.txt"))
        .expect("Should be able to read file");

    let lines: Vec<&str> = file_content.split("\r\n").collect();

    let mut calories: Vec<isize> = vec![0];
    let mut i = 0;

    for line in lines {
        if calories.len() == i {
            calories.push(0);
        }

        if line == "" {
            i += 1;
            continue;
        }

        let line_num: isize = line.parse().unwrap();
        calories[i] = calories[i] + line_num;
    }

    calories.sort();
    calories.reverse();

    return (
        calories.first().unwrap().clone(),
        calories[0..3].iter().sum(),
    )
}