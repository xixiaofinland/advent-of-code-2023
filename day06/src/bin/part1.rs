use std::fs;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();
    let races = parse_input(&input);
    let result: Vec<_> = races
        .iter()
        .map(|race| {
            let mut beats = 0;
            for t in 1..race.time {
                if do_race(t, race) > race.distance {
                    beats += 1;
                }
            }
            beats
        })
        .collect();
    dbg!(result.iter().fold(1, |acc, x| acc * x));
}

fn parse_input(input: &str) -> Vec<Race> {
    let (time_str, distance_str) = input.split_once('\n').unwrap();

    let times: Vec<usize> = time_str
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let distances: Vec<usize> = distance_str
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut races: Vec<Race> = Vec::new();
    for (i, time) in times.into_iter().enumerate() {
        races.push(Race {
            time,
            distance: distances[i],
        });
    }
    races
}

fn do_race(hold_time: usize, race: &Race) -> usize {
    let travel_time = race.time - hold_time;
    let speed = hold_time;
    speed * travel_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let v = "Time:      7  15   30\n Distance:  9  40  200";
        assert_eq!(parse_input(v).len(), 3);
    }
}
