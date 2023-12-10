use std::fs;

// determine direction from s;
// move; step++; get input direction of next tile;
//

const WIDTH: usize = 5;
const LENGTH: usize = 5;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines = parse(&input);
    // let result = exec(lines);
    // dbg!(result);
}

fn parse(input: &str) {
    let mut map = [[(0u8, 0u8); WIDTH]; LENGTH];

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            map[i][j] = getTube(c);
        }
    }
    dbg!(map);
}

fn getTube(c: char) -> (u8, u8) {
    match c {
        '|' => (2, 4),
        '-' => (1, 3),
        'L' => (2, 3),
        'J' => (1, 2),
        '7' => (1, 4),
        'F' => (3, 4),
        'S' => (5, 5),
        '.' => (6, 6),
        _ => unreachable!(),
    }
}

// const WIDTH: usize = input.lines().take(1).next().unwrap().chars().count();
// const LENGTH: u8 = input.lines().count();
