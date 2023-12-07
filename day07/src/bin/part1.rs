use std::cmp::Ordering;
use std::fs;

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    value: usize,
    t: Type,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Type {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cards.cmp(&other.cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t > other.t {
            return Ordering::Greater;
        } else if self.t < other.t {
            return Ordering::Less;
        }

        for (i, v) in self.cards.iter().enumerate() {
            if v < &other.cards[i] {
                return Ordering::Less;
            } else if v > &other.cards[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl Hand {
    fn new(cards: &str, value: usize) -> Self {
        let cards: Vec<usize> = cards
            .chars()
            .map(|c| match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => todo!(),
            })
            .collect();

        let t = Self::get_type(&cards);

        Self { cards, value, t }
    }

    fn get_type(cards: &Vec<usize>) -> Type {
        let mut duplicates = vec![0; 15];
        for c in cards {
            duplicates[*c] += 1;
        }

        if duplicates.contains(&5) {
            return Type::Five;
        } else if duplicates.contains(&4) {
            return Type::Four;
        }

        let duplicates: Vec<usize> = duplicates.into_iter().filter(|d| *d != 0).collect();

        if duplicates.contains(&3) {
            if duplicates.contains(&2) {
                return Type::Full;
            } else {
                return Type::Three;
            }
        }

        let num_of_two = duplicates.iter().filter(|&d| *d == 2).count();
        if num_of_two == 2 {
            return Type::Two;
        } else if num_of_two == 1 {
            return Type::One;
        }

        return Type::High;
    }
}

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();
    let r = parse(&input);
    dbg!(&r);
}

fn parse(input: &str) -> usize {
    let mut hands: Vec<Hand> = Vec::new();

    for l in input.lines() {
        let (cards, value) = l.split_once(" ").unwrap();
        let value = value.parse::<usize>().unwrap();
        hands.push(Hand::new(cards, value));
    }
    hands.sort();
    dbg!(&hands);
    let mut points = 0;
    for (i, card) in hands.iter().enumerate() {
        points += (i + 1) * card.value;
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let v = "32T3K";
        assert_eq!(Hand::new(v, 1).get_type(), Type::One);
    }

    #[test]
    fn two() {
        let v = "23432";
        assert_eq!(Hand::new(v, 1).get_type(), Type::Two);
    }

    #[test]
    fn three() {
        let v = "TT98T";
        assert_eq!(Hand::new(v, 1).get_type(), Type::Three);
    }

    #[test]
    fn full() {
        let v = "2AAA2";
        assert_eq!(Hand::new(v, 1).get_type(), Type::Full);
    }

    #[test]
    fn four() {
        let v = "JQJJJ";
        assert_eq!(Hand::new(v, 1).get_type(), Type::Four);
    }

    #[test]
    fn five() {
        let v = "99999";
        assert_eq!(Hand::new(v, 1).get_type(), Type::Five);
    }

    #[test]
    fn five_bigger_four() {
        let a = "99999";
        let b = "99998";
        assert!(Hand::new(a, 1).get_type() > Hand::new(b, 1).get_type());
    }

    #[test]
    fn order() {
        let a = "QQQJA 2\nTJJT 1";
        let result = parse(a);
        assert_eq!(result, 5);
    }
}
