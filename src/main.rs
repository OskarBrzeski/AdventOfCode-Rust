mod days;

fn main() {
    let solutions = vec![
        days::day1::solution(),
        days::day2::solution(),
        days::day3::solution(),
        days::day4::solution(),
    ];

    for (s, i) in solutions.iter().zip(1..=25) {
        println!("Day {}: {:?}", i, s);
    }
}