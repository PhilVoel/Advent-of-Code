use std::{fs::read_to_string, process::exit};

fn main() {
    let input = read_to_string("inputs/17.txt").unwrap();
    let mut in_lines = input.lines();
    let mut a: u128 = in_lines.next().unwrap().split_at(12).1.parse().unwrap();
    let mut b: u128 = in_lines.next().unwrap().split_at(12).1.parse().unwrap();
    let mut c: u128 = in_lines.next().unwrap().split_at(12).1.parse().unwrap();
    let mut ctr: usize = 0;
    let mut pre = "";
    let mut out = String::new();
    let program_string = in_lines.next_back().unwrap().split_at(9).1;
    let program: Vec<(&str, u128)> = program_string.split(',')
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|cnk| (cnk[0], cnk[1].parse().unwrap()))
        .collect();
    loop {
        let (opcode, operand) = program.get(ctr).unwrap_or_else(||{
            println!("{out}");
            exit(0);
        });
        let combo = match *operand {
            n@0..=3 => n,
            4 => a,
            5 => b,
            6 => c,
            _ => 0, //never happens, so we supply a default in case a literal 7 occurs
        };
        match *opcode {
            "0" => a = a>>combo,
            "1" => b = b^operand,
            "2" => b = combo%8,
            "3" => (),
            "4" => b = b^c,
            "5" => {
                out.push_str(&format!("{pre}{}", combo%8));
                pre = ",";
            },
            "6" => b = a>>combo,
            "7" => c = a>>combo,
            _ => unreachable!()
        }
        if *opcode == "3" && a != 0 {
            ctr = TryInto::<usize>::try_into(*operand).unwrap()/2;
        }
        else {
            ctr += 1;
        }
    }
}
