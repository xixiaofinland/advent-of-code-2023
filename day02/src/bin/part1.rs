use std::fs;

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();

    let mut result = 0;
    for (i, line) in input.lines().enumerate() {
        let b = is_possible_game(line);
        if b {
            result += i + 1;
        }
    }
    dbg!(result);
}

fn is_possible_game(input: &str) -> bool {
    let data = input.split_once(": ").unwrap().1;
    let rounds = data.split("; ");

    for r in rounds {
        let dices = r.split(", ");
        for d in dices {
            let temp: Vec<_> = d.split(" ").collect();
            let num: usize = temp[0].parse().unwrap();

            if d.contains("red") && num > 12 {
                return false;
            } else if d.contains("green") && num > 13 {
                return false;
            } else if d.contains("blue") && num > 14 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let v = is_possible_game(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        assert_eq!(v, false);
    }
}
