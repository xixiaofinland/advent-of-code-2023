use std::fs;

#[derive(Debug, Clone)]
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

fn main() {
    let input = fs::read_to_string("./i1.txt").unwrap();
    let (direction, nodes) = do_it(&input);
    let steps = execute(&direction, nodes);
}

fn execute(direction: &str, nodes: Vec<Node>) -> usize {
    let mut steps = 0;
    let mut nodes_to_run: Vec<Node>;
    nodes_to_run = nodes
        .clone()
        .into_iter()
        .filter(|n| n.value.ends_with("A"))
        .collect();
    dbg!(nodes_to_run.len());
    let mut next_values;

    loop {
        for d in direction.chars() {
            next_values = match d {
                'L' => shift_left(nodes_to_run),
                'R' => shift_right(nodes_to_run),
                _ => todo!(),
            };
            steps += 1;

            let mut found = true;
            for n in &next_values {
                if !n.ends_with("Z") {
                    found = false;
                    break;
                }
            }
            if found == true {
                return steps;
            }

            nodes_to_run = Vec::new();
            for n in &next_values {
                let new_node = nodes.iter().find(|&node| &node.value == n).unwrap();
                let value = new_node.value.to_string();
                let left = new_node.left.to_string();
                let right = new_node.right.to_string();
                nodes_to_run.push(Node::new(value, left, right));
            }
        }
    }
}

fn shift_left(nodes: Vec<Node>) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    for n in nodes {
        values.push(n.left);
    }
    values
}

fn shift_right(nodes: Vec<Node>) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    for n in nodes {
        values.push(n.right);
    }
    values
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
