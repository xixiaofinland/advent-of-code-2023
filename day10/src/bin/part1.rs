use std::fs;

// determine direction from s;
// move; step++; get input direction of next tile;
//

// todo
const WIDTH: usize = 140;
const LENGTH: usize = 140;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (start, map) = parse(&input);
    let result = exec(start, map);
    dbg!(result);
}

fn parse(input: &str) -> ((usize, usize), [[(u8, u8); WIDTH]; LENGTH]) {
    let mut start: (usize, usize) = (0, 0);
    let mut map = [[(0u8, 0u8); WIDTH]; LENGTH];

    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
            }
            map[i][j] = get_tube(c);
        }
    }
    (start, map)
}

fn get_tube(c: char) -> (u8, u8) {
    match c {
        '|' => (2, 4),
        '-' => (1, 3),
        'L' => (2, 3),
        'J' => (1, 2),
        '7' => (1, 4),
        'F' => (3, 4),
        // todo;
        'S' => (1, 2),
        '.' => (6, 6),
        _ => unreachable!(),
    }
}

fn exec(start: (usize, usize), map: [[(u8, u8); WIDTH]; LENGTH]) -> usize {
    dbg!(start);
    let start_value = map[start.0][start.1];
    let mut direction = start_value.0;
    let mut current_node_index = (start.0, start.1);
    let mut steps = 0;

    loop {
        match direction {
            1 => {
                current_node_index = (current_node_index.0, current_node_index.1 - 1);
                if back_to_start(current_node_index, start) {
                    return steps;
                }

                let node_value = map[current_node_index.0][current_node_index.1];
                if let Some(d) = get_next(node_value, 3) {
                    direction = d;
                } else {
                    unreachable!();
                }
            }
            2 => {
                current_node_index = (current_node_index.0 - 1, current_node_index.1);
                if back_to_start(current_node_index, start) {
                    return steps;
                }

                let node_value = map[current_node_index.0][current_node_index.1];
                if let Some(d) = get_next(node_value, 4) {
                    direction = d;
                } else {
                    unreachable!();
                }
            }
            3 => {
                current_node_index = (current_node_index.0, current_node_index.1 + 1);
                if back_to_start(current_node_index, start) {
                    return steps;
                }

                let node_value = map[current_node_index.0][current_node_index.1];
                if let Some(d) = get_next(node_value, 1) {
                    direction = d;
                } else {
                    unreachable!();
                }
            }
            4 => {
                current_node_index = (current_node_index.0 + 1, current_node_index.1);
                if back_to_start(current_node_index, start) {
                    return steps;
                }

                let node_value = map[current_node_index.0][current_node_index.1];
                if let Some(d) = get_next(node_value, 2) {
                    direction = d;
                } else {
                    unreachable!();
                }
            }
            _ => unreachable!(),
        }
        steps += 1;
    }
}

fn get_next(node_value: (u8, u8), direction: u8) -> Option<u8> {
    if node_value.0 == direction {
        return Some(node_value.1);
    } else if node_value.1 == direction {
        return Some(node_value.0);
    }
    return None;
}

fn back_to_start(node: (usize, usize), start: (usize, usize)) -> bool {
    node == start
}
