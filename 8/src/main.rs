use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    let nodes: HashMap<_,_> = lines.map(|l| {
        let (key, targets) = l.split_once(" = (").unwrap();
        let (left, right) = targets.trim_end_matches(")").split_once(", ").unwrap();
        (key.to_string(), (left.to_string(), right.to_string()))
    }).collect();
    let mut current = nodes.get("AAA").unwrap();
    let mut count = 0;
    loop {
        for direction in &directions {
            let next = match direction {
                'L' => &current.0,
                'R' => &current.1,
                _ => unreachable!("Invalid direction"),
            };
            count += 1;
            if next == "ZZZ" {
                println!("Found ZZZ after {count} tries");
                return;
            }
            current = &nodes[next];
        }
    }
}
