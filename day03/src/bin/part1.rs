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
    do_it(&input);
}

fn do_it(input: &str) -> usize {
    let mut num_result: Vec<Num> = vec![];
    let mut symbol_result: Vec<(i32, i32)> = vec![];

    for (i, line_data) in input.lines().enumerate() {
        let t = process_line(i, line_data);
        num_result.extend(t.0);
        symbol_result.extend(t.1);
    }

    let mut result = 0;
    for n in num_result {
        if is_possible(&n, &symbol_result) {
            result += n.value;
        }
    }
    dbg!(result);
    result
}

fn is_possible(n: &Num, symbol_result: &Vec<(i32, i32)>) -> bool {
    let locations = get_possible_sym_locations(n);
    for l in locations {
        if symbol_result.contains(&l) {
            return true;
        }
    }
    false
}

fn get_possible_sym_locations(n: &Num) -> Vec<(i32, i32)> {
    let mut possibles: Vec<(i32, i32)> = vec![];

    let start: i32 = n.start.try_into().unwrap();
    let end: i32 = n.end.try_into().unwrap();
    let line: i32 = n.line.try_into().unwrap();

    possibles.push((line, start - 1)); //left
    possibles.push((line, end + 1)); //right

    for i in (start - 1)..=(end + 1) {
        // up and down
        possibles.push((line - 1, i));
        possibles.push((line + 1, i));
    }
    possibles
}

fn process_line(line: usize, line_data: &str) -> (Vec<Num>, Vec<(i32, i32)>) {
    let mut num_result: Vec<Num> = vec![];
    let mut symbol_result: Vec<(i32, i32)> = vec![];
    let mut num_in_string = String::new();
    let mut is_num_start = true;
    let mut start = 0;

    let indices = line_data.char_indices();
    for (i, ch) in indices {
        if !ch.is_digit(10) && ch != '.' {
            symbol_result.push((line.try_into().unwrap(), i.try_into().unwrap()));
        }

        if ch.is_digit(10) {
            if is_num_start {
                is_num_start = false;
                start = i;
            }
            num_in_string.push(ch);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let v = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ";
        assert_eq!(do_it(v), 4361);
    }
}
