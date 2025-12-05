use std::{env::args, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines().into_iter();
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    loop {
        let next = lines.next().unwrap();
        if next.is_empty() {
            break;
        }
        let (start, end) = next.split_once("-").unwrap();
        ranges.push((start.parse().unwrap(), end.parse().unwrap()));
    }
    let count = if args().any(|arg| arg == "--part2") {
        ranges.sort();
        let mut ranges = ranges.into_iter();
        let mut last = ranges.next().unwrap();
        let mut count = 0;
        for (start, end) in ranges {
            if start <= last.1 + 1 {
                if end > last.1 {
                    last.1 = end;
                }
            } else {
                count += 1 + last.1 - last.0;
                last = (start, end);
            }
        }
        count += 1 + last.1 - last.0;
        count
    } else {
        lines.map(|line| line.parse().unwrap()).filter(|i| ranges.iter().any(|(start, end)| start<= i && end >= i)).collect::<Vec<u64>>().len() as u64
    };
    println!("{count}");
}
