use std::{collections::HashMap, fs};

fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap().lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>().into_iter();
    let mut indices = HashMap::new();
    indices.insert(input.next().unwrap().into_iter().position(|c| c == 'S').unwrap(), 1_u64);
    let mut splits = 0;
    while let Some(line) = input.next() {
        let mut next_indices = HashMap::with_capacity(indices.len());
        for (idx, count) in indices {
            if line[idx] == '^' {
                splits += 1;
                match next_indices.insert(idx - 1, count) {
                    None => (),
                    Some(old) => {next_indices.insert(idx - 1, count + old);}
                }
                match next_indices.insert(idx + 1, count) {
                    None => (),
                    Some(old) => {next_indices.insert(idx + 1, count + old);}
                }
            } else {
                match next_indices.insert(idx, count) {
                    None => (),
                    Some(old) => {next_indices.insert(idx, count + old);}
                }
            }
        }
        indices = next_indices;
    }
    println!("Part 1: {splits}");
    println!("Part 2: {}", indices.values().sum::<u64>());
}
