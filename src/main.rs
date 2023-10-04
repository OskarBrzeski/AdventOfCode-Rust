mod days;

fn main() {
    let solutions = vec![
        days::day1::solution(),
    ];

    for (i, s) in solutions.iter().enumerate() {
        println!("Day {}: {}", i+1, s);
    }
}
