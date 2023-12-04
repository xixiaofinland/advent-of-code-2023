use std::fs;

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();
    let result = do_it(&input);
    dbg!(result);
}

fn do_it(input: &str) -> usize {
    let mut result = 0;
    for data in input.lines() {
        let (win_cards, have_cards) = process_line(data);
        result += match calculate(win_cards, have_cards) {
            0 => 0,
            n => 2_usize.pow(n - 1),
        }
    }
    result
}

fn process_line(data: &str) -> (Vec<usize>, Vec<usize>) {
    let all_nums = data.split_once(": ").unwrap().1;
    let (win, have) = all_nums.split_once(" | ").unwrap();
    let win_cards: Vec<usize> = win.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let have_cards: Vec<usize> = have
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    (win_cards, have_cards)
}

fn calculate(win_cards: Vec<usize>, have_cards: Vec<usize>) -> u32 {
    let result: Vec<&usize> = win_cards
        .iter()
        .filter(|n| have_cards.contains(n))
        .collect();
    u32::try_from(result.len()).ok().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let v = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(do_it(v), 13);
    }
}
