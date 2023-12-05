use std::fs;

fn get_number_list(s: &str) -> Vec<u32> {
    s.split(" ")
        .map(|s| s.trim().parse::<u32>())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect()
}

fn get_matches(s: &str) -> usize {
    let number_lists: Vec<&str> = s
        .split(":")
        .last()
        .unwrap()
        .split("|")
        .map(|s| s.trim())
        .collect();
    let winning_numbers = get_number_list(number_lists[0]);
    let our_numbers = get_number_list(number_lists[1]);

    return our_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();
}

fn get_points(s: &str) -> u32 {
    let number_lists: Vec<&str> = s
        .split(":")
        .last()
        .unwrap()
        .split("|")
        .map(|s| s.trim())
        .collect();
    let winning_numbers = get_number_list(number_lists[0]);
    let our_numbers = get_number_list(number_lists[1]);

    return our_numbers.iter().fold(0, |acc, n| {
        if !winning_numbers.contains(n) {
            return acc;
        }

        return if acc == 0 { 1 } else { acc * 2 };
    });
}

fn play_card((count, stack): (u32, Vec<usize>), matches: usize) -> (u32, Vec<usize>) {
    let goes = stack.iter().filter(|n| **n > 0).count() + 1;
    let mut new_stack = stack
        .iter()
        .map(|n| n - 1)
        .filter(|n| *n > 0)
        .collect::<Vec<usize>>();

    if matches > 0 {
        for _ in 0..goes {
            new_stack.push(matches);
        }
    }

    return (count + goes as u32, new_stack);
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let lines = content
        .split("\n")
        .filter(|&line| line != "")
        .collect::<Vec<&str>>();

    let points: u32 = lines.iter().map(|l| get_points(l)).sum();

    let card_count = lines
        .iter()
        .map(|l| get_matches(l))
        .fold((0, vec![]), play_card)
        .0;

    println!("Points {points}");
    println!("Total cards {card_count}");
}
