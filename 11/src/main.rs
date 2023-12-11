fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("input").unwrap()
        .lines()
        .map(|l| l.chars()
             .collect()
        ).collect();
    let mut gapsy = Vec::new();
    for (i, l) in input.iter().enumerate() {
        if l.iter().all(|c| *c == '.') {
            gapsy.push(i);
        }
    }
    let mut i = 0;
    let mut gapsx = Vec::new();
    while i<input[0].len() {
        if input.iter().all(|l| l[i] == '.') {
            gapsx.push(i);
        }
        i += 1;
    }
    let galaxies: Vec<(i64, i64)> = input.into_iter()
        .enumerate()
        .map(|(y, l)| l.into_iter()
             .enumerate()
             .filter_map(|(x, c)| if c == '#' {
                 let xoffset = 999_999 * gapsx.iter().filter(|r| x > **r).count() as i64;
                 let yoffset = 999_999 * gapsy.iter().filter(|r| y > **r).count() as i64;
                 Some((x as i64 + xoffset, y as i64 + yoffset))
             } else {
                 None
             }).collect::<Vec<_>>()
        ).flatten()
        .collect();
    let mut accum = 0;
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies.iter().skip(i+1) {
            accum += (galaxy1.0-galaxy2.0).abs() + (galaxy1.1-galaxy2.1).abs();
        } 
    }
    println!("{accum}");
}
