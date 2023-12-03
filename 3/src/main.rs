fn main() {
    let input = std::fs::read_to_string("input").unwrap()
        .lines()
        .map(|l| l.chars()
             .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();
    let mut sum = 0;
    for (ln, l) in input.iter().enumerate() {
        let mut num_start = 0;
        let mut num = false;
        for (cn, c) in l.iter().enumerate() {
            if num && (!c.is_ascii_alphanumeric() || cn == 139) {
                num = false;
                let last = {
                    if c.is_ascii_alphanumeric() {
                        cn
                    }
                    else {
                        cn-1
                    }
                };
                let number = l[num_start..=last].iter().collect::<String>().parse::<u32>().unwrap();
                let lower_line = {
                    if ln == 0 {
                        0
                    }
                    else {
                        ln-1
                    }
                };
                let upper_line = {
                    if ln == 139 {
                        139
                    }
                    else {
                        ln+1
                    }
                };
                'add: for line in lower_line..=upper_line {
                    let lower_col = {
                        if num_start == 0 {
                            0
                        }
                        else {
                            num_start-1
                        }
                    };
                    println!("{}, {}, {}, {}", line, lower_col, cn, number);
                    for col in lower_col..=cn {
                        match input[line as usize][col as usize] {
                            '.' => (),
                            c if c.is_ascii_alphanumeric() => (),
                            _ => {
                                sum += number;
                                break 'add;
                            }
                        }
                    }
                }
            }
            if c.is_ascii_alphanumeric() && !num {
                num_start = cn;
                num = true;
            }
        }
    }
    println!("{sum}");
}
