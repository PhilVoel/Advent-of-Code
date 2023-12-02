fn main() {
    println!("{}", std::fs::read_to_string("input").unwrap()
             .lines()
             .map(|line| {
                 let mut red = Vec::new();
                 let mut blue = Vec::new();
                 let mut green = Vec::new();
                 for set in line.split_once(':').unwrap().1.split(';') {
                     for num_and_color in set.split(',') {
                         match num_and_color.trim().split_once(' ').unwrap() {
                             (n, "red") => red.push(n.parse::<u32>().unwrap()),
                             (n, "green") => green.push(n.parse::<u32>().unwrap()),
                             (n, "blue") => blue.push(n.parse::<u32>().unwrap()),
                             _ => unreachable!()
                         }
                     }
                 }
                 red.into_iter().max().unwrap() * green.into_iter().max().unwrap() * blue.into_iter().max().unwrap()
             })
             .sum::<u32>()
            );
}
