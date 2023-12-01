fn main() {
    println!("{}", std::fs::read_to_string("input")
        .expect("Error reading input")
        .lines()
        .map(|line|
             line.chars()
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
