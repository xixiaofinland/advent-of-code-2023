use std::fs;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();
    let race = parse_input(&input);
    let s = (1..race.time)
        .map(
            |hold_time: usize| match do_race(hold_time, &race) > race.distance {
                true => 1,
                false => 0,
            },
        )
        .fold(0, |acc, x| (acc + x));
    dbg!(s);
}

fn parse_input(input: &str) -> Race {
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    let time = time_str
        .strip_prefix("Time: ")
        .unwrap()
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = distance_str
        .strip_prefix("Distance: ")
        .unwrap()
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    Race { time, distance }
}

fn do_race(hold_time: usize, race: &Race) -> usize {
    let travel_time = race.time - hold_time;
    let speed = hold_time;
    speed * travel_time
}
