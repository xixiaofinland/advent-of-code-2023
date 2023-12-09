use std::fs;

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();
    let lines = parse(&input);
    let result = exec(lines);
    dbg!(result);
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    let mut lines: Vec<Vec<i64>> = Vec::new();

    for l in input.lines() {
        let line: Vec<i64> = l
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        lines.push(line);
    }
    lines
}

fn exec(lines: Vec<Vec<i64>>) -> i64 {
    let mut result: i64 = 0;
    let mut end_nums: Vec<i64> = vec![];

    for line in lines {
        let mut l = line;
        end_nums.push(*l.last().unwrap());

        loop {
            l = cal_delta(l);
            end_nums.push(*l.last().unwrap());
            if l.iter().all(|&item| item == 0) {
                break;
            }
        }
        let t: i64 = end_nums.iter().sum();
        end_nums.clear();
        result += t;
    }
    result
}

fn cal_delta(input: Vec<i64>) -> Vec<i64> {
    let elements = input.iter();
    let elements_skip_first = input.iter().skip(1);

    elements
        .zip(elements_skip_first)
        .map(|(a, b)| b - a)
        .collect()
}
