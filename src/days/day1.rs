use std::fs;
use std::path::Path;

pub fn solution() -> (i32, i32) {
    let file_content = fs::read_to_string(Path::new("data/input1.txt"))
        .expect("Should be able to read file");

    let lines = file_content.lines();

    let mut calories: Vec<i32> = vec![0];
    let mut i = 0;

    for line in lines {
        if calories.len() == i {
            calories.push(0);
        }

        if line == "" {
            i += 1;
            continue;
        }

        let line_num: i32 = line.parse().unwrap();
        calories[i] = calories[i] + line_num;
    }

    calories.sort();
    calories.reverse();

    (
        calories.first().unwrap().clone(),
        calories[0..3].iter().sum(),
    )
}