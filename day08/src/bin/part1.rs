use std::fs;

#[derive(Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn new(value: String, left: String, right: String) -> Self {
        Self { value, left, right }
    }
}

const RESULT: &str = "ZZZ";

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();
    let (direction, nodes) = do_it(&input);
    let steps = execute(&direction, nodes);
    dbg!(steps);
}

fn execute(direction: &str, nodes: Vec<Node>) -> usize {
    let mut steps = 0;
    let mut node_to_check: &Node = nodes.iter().find(|&node| &node.value == "AAA").unwrap();
    let mut next_value;

    loop {
        for d in direction.chars() {
            next_value = match d {
                'L' => &node_to_check.left,
                'R' => &node_to_check.right,
                _ => todo!(),
            };
            steps += 1;
            dbg!(next_value);

            if next_value == RESULT {
                return steps;
            }

            node_to_check = nodes
                .iter()
                .find(|&node| &node.value == next_value)
                .unwrap();
        }
    }
}

fn do_it(input: &str) -> (String, Vec<Node>) {
    let (directions, node_lines) = input.split_once("\n\n").unwrap();

    let mut nodes: Vec<Node> = Vec::new();
    for node_data in node_lines.lines() {
        let (value, left_right_tube) = node_data.split_once(" = ").unwrap();
        let left_right_tube = left_right_tube.replace("(", "").replace(")", "");
        let (left, right) = left_right_tube.split_once(", ").unwrap();
        nodes.push(Node::new(
            value.to_string(),
            left.to_string(),
            right.to_string(),
        ));
    }
    (directions.to_string(), nodes)
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn t1() {
//         let v = "";
//         assert_eq!(do_it(v), 4361);
//     }
// }
