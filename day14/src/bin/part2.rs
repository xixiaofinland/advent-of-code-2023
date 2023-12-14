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
        if i == 0 {
            if one_node_diff_only(&lines[i], &lines[i + 1]) {
                count += 100;
                println!("Lstart-{}-{}", i, count);
                break;
            }
        } else if i == lines.len() - 2 {
            if one_node_diff_only(&lines[i], &lines[i + 1]) {
                count += 100 * (i + 1);
                println!("Lend-{}-{}", i, count);
                break;
            }
        } else {
            if lines[i] == lines[i + 1] {
                let mut found = 0;
                let mut s: i32 = 1;
                let mut is_reflect = true;
                let ii = i32::try_from(i).unwrap();
                while ii - s >= 0 && ii + 1 + s <= i32::try_from(lines.len()).unwrap() - 1 {
                    let ss = usize::try_from(s).unwrap();
                    if lines[i - ss] == lines[i + 1 + ss] {
                        s += 1;
                    } else if one_node_diff_only(&lines[i - ss], &lines[i + 1 + ss]) {
                        s += 1;
                        found += 1;
                        if found > 1 {
                            is_reflect = false;
                            break;
                        }
                    } else {
                        is_reflect = false;
                        break;
                    }
                }
                if is_reflect && found == 1 {
                    count += (i + 1) * 100;
                    println!("!L-perf_Start-{} {}", i, count);
                    break;
                }
            } else if one_node_diff_only(&lines[i], &lines[i + 1]) {
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
                    println!("!L-imperf_Start-{} {}", i, count);
                    count += (i + 1) * 100;
                }
            }
        }
    }

    for i in 0..columns.len() - 1 {
        if i == 0 {
            if one_node_diff_only(&columns[i], &columns[i + 1]) {
                count += 1;
                println!("!Cstart: col {} {}", i, count);
                break;
            }
        } else if i == columns.len() - 2 {
            if one_node_diff_only(&columns[i], &columns[i + 1]) {
                count += i + 1;
                println!("!Cend col {} {}", i, count);
                break;
            }
        } else {
            if columns[i] == columns[i + 1] {
                let mut found = 0;
                let mut s: i32 = 1;
                let mut is_reflect = true;
                let ii = i32::try_from(i).unwrap();
                while ii - s >= 0 && ii + 1 + s <= i32::try_from(columns.len()).unwrap() - 1 {
                    let ss = usize::try_from(s).unwrap();
                    if columns[i - ss] == columns[i + 1 + ss] {
                        s += 1;
                    } else if one_node_diff_only(&columns[i - ss], &columns[i + 1 + ss]) {
                        s += 1;
                        found += 1;
                        if found > 1 {
                            is_reflect = false;
                            break;
                        }
                    } else {
                        is_reflect = false;
                        break;
                    }
                }
                if is_reflect && found == 1 {
                    count += i + 1;
                    println!("!C_perf_ {} {}", i, count);
                    break;
                }
            } else if one_node_diff_only(&columns[i], &columns[i + 1]) {
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
                    println!("!C_imperf_ {} {}", i, count);
                    count += i + 1;
                }
            }
        }
    }
    count
}

fn one_node_diff_only(a: &Vec<char>, b: &Vec<char>) -> bool {
    let mut diff = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            diff += 1;
        }
        if diff > 1 {
            return false;
        }
    }

    if diff == 0 {
        return false;
    } else {
        return true;
    }
}
