use std::fs;
use std::path::Path;
use std::str::Lines;

pub fn solution(path: &str) -> (i32, i32) {
    let file_content = fs::read_to_string(Path::new(path)).expect("Should be able to read file");

    let lines = file_content.lines();

    let trees = parse_input(lines);

    (part1(&trees), part2(&trees))
}

fn part1(trees: &Vec<Vec<i32>>) -> i32 {
    let mut counter = 0;

    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            if tree_is_visible(trees, x, y) {
                counter += 1;
            }
        }
    }

    counter
}

fn part2(trees: &Vec<Vec<i32>>) -> i32 {
    let mut max_score = 0;
    
    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            let score = scenic_score(trees, x, y);

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn parse_input(lines: Lines) -> Vec<Vec<i32>> {
    let mut output = vec![];

    for line in lines {
        let mut current_row = vec![];

        for c in line.chars() {
            current_row.push(c.to_digit(10).unwrap() as i32);
        }

        output.push(current_row);
    }

    output
}

fn tree_is_visible(trees: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    if x == 0 || x + 1 == trees[y].len() {
        return true;
    }

    if y == 0 || y + 1 == trees.len() {
        return true;
    }

    let mut visible = [true, true, true, true];
    let current_tree = trees[y][x];

    for dx in 0..x {
        if trees[y][dx] >= current_tree {
            visible[0] = false;
            break;
        }
    }

    for dx in x + 1..trees[y].len() {
        if trees[y][dx] >= current_tree {
            visible[1] = false;
            break;
        }
    }

    for dy in 0..y {
        if trees[dy][x] >= current_tree {
            visible[2] = false;
            break;
        }
    }

    for dy in y + 1..trees.len() {
        if trees[dy][x] >= current_tree {
            visible[3] = false;
            break;
        }
    }

    visible[0] || visible[1] || visible[2] || visible[3]
}

fn scenic_score(trees: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut visible_trees = [0, 0, 0, 0];

    let current_tree = trees[y][x];

    for dx in 1..=x {
        visible_trees[0] += 1;
        if trees[y][x - dx] >= current_tree {
            break;
        }
    }

    for dx in x + 1..trees[y].len() {
        visible_trees[1] += 1;
        if trees[y][dx] >= current_tree {
            break;
        }
    }

    for dy in 1..=y {
        visible_trees[2] += 1;
        if trees[y-dy][x] >= current_tree {
            break;
        }
    }

    for dy in y + 1..trees.len() {
        visible_trees[3] += 1;
        if trees[dy][x] >= current_tree {
            break;
        }
    }

    visible_trees[0] * visible_trees[1] * visible_trees[2] * visible_trees[3]
}
