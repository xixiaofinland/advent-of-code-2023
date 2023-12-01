use std::fs;

fn main() {
    let mut result: u32 = 0;
    let input = fs::read_to_string("./input1.txt").unwrap();
    for line in input.lines() {
        result += parse2(&parse(line));
    }
    dbg!(result);
}

fn parse(input: &str) -> String {
    let mut result = input.to_string();
    let elements: [(&str, &str); 17] = [
        ("oneight", "18"),
        ("twone", "21"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("sevenine", "79"),
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("nineight", "98"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for el in &elements {
        result = result.replace(el.0, el.1);
    }

    result
}

fn parse2(input: &str) -> u32 {
    let nums: Vec<_> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    if nums.len() == 1 {
        nums[0] * 11
    } else {
        nums[0] * 10 + nums[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = parse2(&parse("two1nine"));
        assert_eq!(result, 29);
    }

    #[test]
    fn t2() {
        let result = parse2(&parse("eightwothree"));
        assert_eq!(result, 83);
    }

    #[test]
    fn t3() {
        let result = parse2(&parse("abcone2threexyz"));
        assert_eq!(result, 13);
    }

    #[test]
    fn t4() {
        let result = parse2(&parse("xtwone3four"));
        assert_eq!(result, 24);
    }

    #[test]
    fn t5() {
        let result = parse2(&parse("4nineeightseven2"));
        assert_eq!(result, 42);
    }

    #[test]
    fn t6() {
        let result = parse2(&parse("zoneight234"));
        assert_eq!(result, 14);
    }

    #[test]
    fn t7() {
        let result = parse2(&parse("7pqrstsixteen"));
        assert_eq!(result, 76);
    }
}
