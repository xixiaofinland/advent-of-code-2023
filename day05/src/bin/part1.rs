use std::fs;

fn main() {
    let data = "91926764 235794528 3279509610 325625103 2781720183 218217413 1315129829 102999617 3995609239 143268116 358337926 185836835 1543999077 241888600 1795811745 806228439 2616560939 56204204 869828854 224520829";
    let seeds: Vec<usize> = data
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let map_inputs: [&str; 7] = [
        "ss.txt", "sf.txt", "fw.txt", "wl.txt", "lt.txt", "th.txt", "hl.txt",
    ];

    let mut result: Vec<usize> = Vec::new();

    for s in seeds {
        let mut seed = s;

        for map_input in map_inputs {
            let rules = parse(map_input);
            for r in rules {
                if seed >= r.1 && seed <= (r.1 + r.2) {
                    seed = seed + r.0 - r.1;
                    break;
                }
            }
        }
        dbg!(seed);
        result.push(seed);
    }

    dbg!(result.iter().min().unwrap());
}

fn parse(file_path: &str) -> Vec<(usize, usize, usize)> {
    let mut v: Vec<(usize, usize, usize)> = Vec::new();
    let input = fs::read_to_string(file_path).unwrap();
    for line in input.lines() {
        v.push(process_line(line));
    }
    v
}

fn process_line(data: &str) -> (usize, usize, usize) {
    let r: Vec<usize> = data
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    (r[0], r[1], r[2])
}
