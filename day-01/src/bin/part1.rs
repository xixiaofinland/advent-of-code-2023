use std::fs;

fn main() {
    let mut data: u32 = 0;
    let input = fs::read_to_string("./input1.txt").unwrap();
    for line in input.lines() {
        data += part1(line);
    }
    dbg!(data);
}

fn part1(input: &str) -> u32 {
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
    fn single_num() {
        let result = part1("treb7uchet");
        assert_eq!(result, 77);
    }

    #[test]
    fn double_num() {
        let result = part1("pqr3stu8vwx");
        assert_eq!(result, 38);
    }

    #[test]
    fn many_num() {
        let result = part1("a1b2c3d4e5f");
        assert_eq!(result, 15);
    }
}
