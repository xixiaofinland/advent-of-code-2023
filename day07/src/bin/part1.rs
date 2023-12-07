use std::fs;

#[derive(Debug, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<usize>,
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

// impl PartialEq for Hand {
//     fn eq(&self, other: &Self) -> bool {
//         (self.car == other.car) && (self.customer_details == other.customer_details)
//     }
// }

impl Hand {
    fn new(cards: &str) -> Self {
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

        Self { cards }
    }

    fn get_type(&self) -> Type {
        let mut duplicates = vec![0; 15];
        for c in &self.cards {
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
    // let input = fs::read_to_string("./i1.txt").unwrap();
    // let result = do_it(&input);
    // dbg!(result);
}

// fn do_it(input: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let v = "32T3K";
        assert_eq!(Hand::new(v).get_type(), Type::One);
    }

    #[test]
    fn two() {
        let v = "23432";
        assert_eq!(Hand::new(v).get_type(), Type::Two);
    }

    #[test]
    fn three() {
        let v = "TT98T";
        assert_eq!(Hand::new(v).get_type(), Type::Three);
    }

    #[test]
    fn full() {
        let v = "2AAA2";
        assert_eq!(Hand::new(v).get_type(), Type::Full);
    }

    #[test]
    fn four() {
        let v = "JQJJJ";
        assert_eq!(Hand::new(v).get_type(), Type::Four);
    }

    #[test]
    fn five() {
        let v = "99999";
        assert_eq!(Hand::new(v).get_type(), Type::Five);
    }

    #[test]
    fn five_bigger_four() {
        let a = "99999";
        let b = "99998";
        assert!(Hand::new(a).get_type() > Hand::new(b).get_type());
    }
}
