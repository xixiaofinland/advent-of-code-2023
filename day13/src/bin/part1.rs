use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let r = parse(&input);
    dbg!(r);
}

fn parse(input: &str) -> usize {
    let mut result = 0;
    let maps: Vec<&str> = input.split("\n\n").collect();
    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut columns: Vec<Vec<char>> = Vec::new();
    let mut line: Vec<char> = Vec::new();
    let mut column: Vec<char> = Vec::new();

    for map in maps {
        for l in map.lines() {
            for c in l.chars() {
                line.push(c);
            }
            lines.push(line.clone());
            line.clear();
        }

        for i in 0..lines[0].len() {
            for j in 0..lines.len() {
                column.push(lines[j][i]);
            }
            columns.push(column.clone());
            column.clear();
        }

        let r = cal(&lines, &columns);
        result += r;
        lines.clear();
        columns.clear();
    }
    result
}

fn cal(lines: &Vec<Vec<char>>, columns: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for i in 0..lines.len() - 1 {
        if lines[i] == lines[i + 1] {
            let mut s: i32 = 1;
            let mut is_reflect = true;
            let ii = i32::try_from(i).unwrap();
            while ii - s >= 0 && ii + 1 + s <= i32::try_from(lines.len()).unwrap() - 1 {
                let ss = usize::try_from(s).unwrap();
                if lines[i - ss] == lines[i + 1 + ss] {
                    s += 1;
                } else {
                    is_reflect = false;
                    break;
                }
            }
            if is_reflect {
                dbg!(i);
                count += (i + 1) * 100;
            }
        }
    }

    for i in 0..columns.len() - 1 {
        if columns[i] == columns[i + 1] {
            let mut s: i32 = 1;
            let mut is_reflect = true;
            let ii = i32::try_from(i).unwrap();
            while ii - s >= 0 && ii + 1 + s <= i32::try_from(columns.len()).unwrap() - 1 {
                let ss = usize::try_from(s).unwrap();
                if columns[i - ss] == columns[i + 1 + ss] {
                    s += 1;
                } else {
                    is_reflect = false;
                    break;
                }
            }
            if is_reflect {
                dbg!(i);
                count += i + 1;
            }
        }
    }
    count
}
