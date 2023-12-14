use std::fs;

// const WIDTH: usize = 10;
// const LENGTH: usize = 10;
const WIDTH: usize = 100;
const LENGTH: usize = 100;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut map = parse(&input);
    map = exec(map);
    let result = cal(map);
    dbg!(result);
}

fn parse(input: &str) -> [[char; WIDTH]; LENGTH] {
    let mut map = [['.'; WIDTH]; LENGTH];

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            map[j][LENGTH - i - 1] = c;
        }
    }
    map
}

fn exec(mut map: [[char; WIDTH]; LENGTH]) -> [[char; WIDTH]; LENGTH] {
    for i in 0..WIDTH {
        let mut times = LENGTH;
        while times > 0 {
            for j in 0..LENGTH - 1 {
                if map[i][j] == 'O' {
                    if map[i][j + 1] == '.' {
                        map[i][j] = '.';
                        map[i][j + 1] = 'O';
                    }
                } else if map[i][j] == '#' {
                } else if map[i][j] == '.' {
                }
            }
            times -= 1;
        }
    }
    map
}

fn cal(m: [[char; WIDTH]; LENGTH]) -> usize {
    let mut mutiplier = 1;
    let mut result = 0;
    for i in 0..WIDTH {
        let mut num = 0;
        for j in 0..LENGTH {
            if m[j][i] == 'O' {
                num += 1;
            }
            // dbg!(m[j][i]);
        }
        result += num * mutiplier;
        mutiplier += 1;
    }
    result
}
