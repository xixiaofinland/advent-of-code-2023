use std::fs;

// todo
const WIDTH: usize = 140;
const HIGHT: usize = 140;
const TIMES: usize = 1000000;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (map, galaxies) = parse(&input);
    let (empty_column, empty_row) = cal(map);
    let result = exec(galaxies, empty_column, empty_row);
    dbg!(result);
}

fn parse(input: &str) -> ([[char; WIDTH]; HIGHT], Vec<(usize, usize)>) {
    let mut map = [['0'; WIDTH]; HIGHT];
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            map[i][j] = c;
            if c == '#' {
                galaxies.push((i, j));
            }
        }
    }
    (map, galaxies)
}

fn cal(map: [[char; WIDTH]; HIGHT]) -> (Vec<usize>, Vec<usize>) {
    let mut empty_column: Vec<usize> = Vec::new();
    let mut empty_row: Vec<usize> = Vec::new();

    for i in 0..HIGHT {
        for j in 0..WIDTH {
            if map[i][j] == '#' {
                break;
            }
            if j == WIDTH - 1 {
                empty_row.push(i);
            }
        }
    }

    for i in 0..WIDTH {
        for j in 0..HIGHT {
            if map[j][i] == '#' {
                break;
            }
            if j == HIGHT - 1 {
                empty_column.push(i);
            }
        }
    }

    (empty_column, empty_row)
}

fn exec(galaxies: Vec<(usize, usize)>, empty_column: Vec<usize>, empty_row: Vec<usize>) -> usize {
    let mut result = 0;
    let end_element = galaxies.len();
    for i in 0..end_element - 1 {
        for j in i + 1..end_element {
            let (a, b) = galaxies[i];
            let (c, d) = galaxies[j];
            for step in a.min(c) + 1..a.max(c) + 1 {
                if empty_row.contains(&step) {
                    result += TIMES;
                } else {
                    result += 1;
                }
            }
            for step in b.min(d) + 1..b.max(d) + 1 {
                if empty_column.contains(&step) {
                    result += TIMES;
                } else {
                    result += 1;
                }
            }
        }
    }
    result
}
