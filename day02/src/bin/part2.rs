use std::fs;

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();

    let mut result = 0;
    for line in input.lines() {
        result += power_fewest_num(line);
    }
    dbg!(result);
}

fn power_fewest_num(input: &str) -> usize {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let data = input.split_once(": ").unwrap().1;
    let rounds = data.split("; ");

    for r in rounds {
        let dices = r.split(", ");
        for d in dices {
            let temp: Vec<_> = d.split(" ").collect();
            let num: usize = temp[0].parse().unwrap();

            if d.contains("red") && num > red {
                red = num;
            } else if d.contains("green") && num > green {
                green = num;
            } else if d.contains("blue") && num > blue {
                blue = num;
            }
        }
    }
    red * green * blue
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
