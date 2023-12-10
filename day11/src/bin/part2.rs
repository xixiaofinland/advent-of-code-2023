use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
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

fn exec(lines: Vec<Vec<i64>>) -> i64 {}
