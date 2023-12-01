fn main() {
    println!("{}", std::fs::read_to_string("input")
        .expect("Error reading input")
        .lines()
        .map(|line|
             replace_all(line)
             .chars()
             .filter(|c| c.is_digit(10))
             .map(|c| c.to_digit(10).unwrap())
             .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>()
        .into_iter()
        .map(|line| 10*line.first().unwrap()+line.last().unwrap())
        .sum::<u32>()
    );
}

fn replace_all(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "9")
}
