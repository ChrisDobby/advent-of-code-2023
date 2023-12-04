use std::{fs, vec};

fn find_number(s: &str) -> Option<u32> {
    let numbers = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for n in numbers {
        if s.starts_with(n.0) || s.starts_with(n.1) {
            return Some(n.1.parse::<u32>().unwrap());
        }
    }

    return None;
}

fn get_calibration(s: &str) -> u32 {
    let n = s.chars().enumerate().fold((None, None), |acc, (i, _)| {
        let number_at_pos = find_number(&s[i..]);
        if number_at_pos.is_some() {
            return if acc.0.is_some() {
                (acc.0, number_at_pos)
            } else {
                (number_at_pos, number_at_pos)
            };
        }

        return acc;
    });

    let ret = (n.0.unwrap() * 10) + n.1.unwrap();
    println!("{s}, {ret}");
    return ret;
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let calibration: u32 = content
        .split("\n")
        .filter(|&line| line != "")
        .map(|line| get_calibration(line))
        .sum();

    println!("{calibration}");
}
