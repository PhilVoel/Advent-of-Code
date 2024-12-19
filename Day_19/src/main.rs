use std::{collections::HashMap, env::args, fs::read_to_string};
use regex::Regex;

fn main() {
    let input = read_to_string("inputs/19.txt").unwrap();
    let mut in_lines = input.lines();
    let available = in_lines.next().unwrap();
    let available_regex = Regex::new(&format!("^({})*$", available.replace(", ", "|"))).unwrap();
    in_lines.next();
    let possible = in_lines.filter(|l| available_regex.is_match(l));
    if args().any(|a| a == "--part2") {
        part2(possible.collect(), available);
    }
    else {
        println!("{}", possible.count());
    }
}

fn part2(lines: Vec<&str>, available: &str) {
    let mut map: HashMap<&str, u128> = HashMap::new();
    println!("{}", lines.into_iter().map(|line| num_matches(line, &available.split(", ").collect(), &mut map)).sum::<u128>());
}

fn num_matches<'a>(line: &'a str, available: &Vec<&str>, map: &mut HashMap<&'a str, u128>) -> u128 {
    if line.is_empty() {
        return 1;
    }
    match map.get(line) {
        Some(count) => *count,
        None => {
            let mut count = 0;
            for start in available {
                match line.strip_prefix(start) {
                    Some(new) => {
                        count+=num_matches(new, available, map)
                    }
                    None => ()
                }
            }
            map.insert(line, count);
            count
        }
    }
}
