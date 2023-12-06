use std::fs;

fn main() {
    // let data = "79 14 55 13";
    let data = "91926764 235794528 3279509610 325625103 2781720183 218217413 1315129829 102999617 3995609239 143268116 358337926 185836835 1543999077 241888600 1795811745 806228439 2616560939 56204204 869828854 224520829";
    let data_vec: Vec<usize> = data
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let nums: Vec<usize> = data_vec
        .clone()
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, e)| e)
        .collect();

    let times: Vec<usize> = data_vec
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0)
        .map(|(_, e)| e)
        .collect();

    dbg!(&nums);
    dbg!(&times);

    let mut seeds: Vec<usize> = Vec::new();

    for (i, start) in nums.into_iter().enumerate() {
        let end = times[i];
        for i in start..(start + end) {
            dbg!(i);
            seeds.push(i);
        }
    }

    let map_inputs: [&str; 7] = [
        "ss1.txt", "sf1.txt", "fw1.txt", "wl1.txt", "lt1.txt", "th1.txt", "hl1.txt",
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
