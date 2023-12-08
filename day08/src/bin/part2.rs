use std::fs;

#[derive(Debug)]
struct Num {
    line: usize,
    value: usize,
    start: usize,
    end: usize,
}

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();
    let result = do_it(&input);
    dbg!(result);
}

fn do_it(input: &str) -> usize {
    let mut symbol_result: Vec<(i32, i32)> = vec![];
    let mut num_result: Vec<Num> = vec![];

    for (i, line_data) in input.lines().enumerate() {
        let t = process_line(i, line_data);
        num_result.extend(t.0);
        symbol_result.extend(t.1);
    }

    let mut result = 0;
    for s in symbol_result {
        result += get_gear_result(s, &num_result);
    }
    result
}

fn get_gear_result(s: (i32, i32), num_result: &Vec<Num>) -> usize {
    let mut values: Vec<usize> = vec![];

    // left
    let left_value = get_value((s.0, s.1 - 1), num_result);
    if left_value != 0 {
        values.push(left_value);
    }

    // right
    let right_value = get_value((s.0, s.1 + 1), num_result);
    if right_value != 0 {
        values.push(right_value);
    }

    // up; if no up left; then up right;
    let up_result = get_value((s.0 - 1, s.1), num_result);
    if up_result == 0 {
        let up_left_result = get_value((s.0 - 1, s.1 - 1), num_result);
        if up_left_result != 0 {
            values.push(up_left_result);
        }
        let up_right_result = get_value((s.0 - 1, s.1 + 1), num_result);
        if up_right_result != 0 {
            values.push(up_right_result);
        }
    } else {
        values.push(up_result)
    }

    // down; if no down left; then down right;
    let down_result = get_value((s.0 + 1, s.1), num_result);
    if down_result == 0 {
        let down_left_result = get_value((s.0 + 1, s.1 - 1), num_result);
        if down_left_result != 0 {
            values.push(down_left_result);
        }
        let down_right_result = get_value((s.0 + 1, s.1 + 1), num_result);
        if down_right_result != 0 {
            values.push(down_right_result);
        }
    } else {
        values.push(down_result)
    }

    dbg!(&values);

    if values.len() == 2 {
        return values[0] * values[1];
    }
    0
}

fn get_value(location: (i32, i32), num_result: &Vec<Num>) -> usize {
    for n in num_result {
        if location.0 == n.line.try_into().unwrap()
            && location.1 >= n.start.try_into().unwrap()
            && location.1 <= n.end.try_into().unwrap()
        {
            return n.value;
        }
    }
    0
}

fn process_line(line: usize, line_data: &str) -> (Vec<Num>, Vec<(i32, i32)>) {
    let mut num_result: Vec<Num> = vec![];
    let mut symbol_result: Vec<(i32, i32)> = vec![];
    let mut num_in_string = String::new();
    let mut is_num_start = true;
    let mut start = 0;

    let end = line_data.chars().count() - 1;
    let indices = line_data.char_indices();
    for (i, ch) in indices {
        if ch == '*' {
            symbol_result.push((line.try_into().unwrap(), i.try_into().unwrap()));
        }

        if ch.is_digit(10) {
            if is_num_start {
                is_num_start = false;
                start = i;
            }
            num_in_string.push(ch);

            if i == end {
                num_result.push(Num {
                    line,
                    value: num_in_string.parse::<usize>().unwrap(),
                    start,
                    end: i - 1,
                });
            }
        } else {
            if !num_in_string.is_empty() {
                num_result.push(Num {
                    line,
                    value: num_in_string.parse::<usize>().unwrap(),
                    start,
                    end: i - 1,
                });
            }

            // reset status
            is_num_start = true;
            num_in_string.clear();
        }
    }

    (num_result, symbol_result)
}
