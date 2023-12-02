fn main() {
    println!("{}", std::fs::read_to_string("input").unwrap()
             .lines()
             .filter(|line| line.split_once(':').unwrap()
                     .1
                     .split(';')
                     .all(|set|
                            set.split(',')
                            .all(|num_and_color|
                                match num_and_color.trim().split_once(' ').unwrap() {
                                    (n, "red") => n.parse::<u8>().unwrap() <= 12,
                                    (n, "green") => n.parse::<u8>().unwrap() <= 13,
                                    (n, "blue") => n.parse::<u8>().unwrap() <= 14,
                                    _ => unreachable!()
                                }
                                )
                          )
                     )
             .map(|line| line.split_once(':').unwrap()
                  .0
                  .split_once(' ').unwrap()
                  .1
                  .parse::<u16>().unwrap())
             .sum::<u16>()
             );
}
