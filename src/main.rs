mod days;

fn main() {
    println!("Day 1: {:?}", days::day1::solution("data/input1.txt"));
    println!("Day 2: {:?}", days::day2::solution("data/input2.txt"));
    println!("Day 3: {:?}", days::day3::solution("data/input3.txt"));
    println!("Day 4: {:?}", days::day4::solution("data/input4.txt"));
    println!("Day 5: {:?}", days::day5::solution("data/input5.txt"));
    println!("Day 6: {:?}", days::day6::solution("data/input6.txt"));
    println!("Day 7: {:?}", days::day7::solution("data/input7.txt"));
    // println!("Day 8: {:?}", days::day8::solution());
    // println!("Day 9: {:?}", days::day9::solution());
    // println!("Day 10: {:?}", days::day10::solution());
    // println!("Day 11: {:?}", days::day11::solution());
    // println!("Day 12: {:?}", days::day12::solution());
    // println!("Day 13: {:?}", days::day13::solution());
    // println!("Day 14: {:?}", days::day14::solution());
    // println!("Day 15: {:?}", days::day15::solution());
    // println!("Day 16: {:?}", days::day16::solution());
    // println!("Day 17: {:?}", days::day17::solution());
    // println!("Day 18: {:?}", days::day18::solution());
    // println!("Day 19: {:?}", days::day19::solution());
    // println!("Day 20: {:?}", days::day20::solution());
    // println!("Day 21: {:?}", days::day21::solution());
    // println!("Day 22: {:?}", days::day22::solution());
    // println!("Day 23: {:?}", days::day23::solution());
    // println!("Day 24: {:?}", days::day24::solution());
    // println!("Day 25: {:?}", days::day25::solution());
}

#[cfg(test)]
mod tests {
    use super::days;

    #[test]
    fn test_day1() {
        let result = days::day1::solution("test_data/input1.txt");
        assert_eq!(result.0, 24000);
        assert_eq!(result.1, 45000);
    }

    #[test]
    fn test_day2() {
        let result = days::day2::solution("test_data/input2.txt");
        assert_eq!(result.0, 15);
        assert_eq!(result.1, 12);
    }
    
    #[test]
    fn test_day3() {
        let result = days::day3::solution("test_data/input3.txt");
        assert_eq!(result.0, 157);
        assert_eq!(result.1, 70);
    }

    #[test]
    fn test_day4() {
        let result = days::day4::solution("test_data/input4.txt");
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 4);
    }

    #[test]
    fn test_day5() {
        let result = days::day5::solution("test_data/input5.txt");
        assert_eq!(result.0, String::from("CMZ"));
        assert_eq!(result.1, String::from("MCD"));
    }

    #[test]
    fn test_day6() {
        let result = days::day6::solution("test_data/input6.txt");
        assert_eq!(result.0, 7);
        assert_eq!(result.1, 19);
    }

    #[test]
    fn test_day7() {
        let result = days::day7::solution("test_data/input7.txt");
        assert_eq!(result.0, 95437);
        assert_eq!(result.1, 24933642);
    }
}