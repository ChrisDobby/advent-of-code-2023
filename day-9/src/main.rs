use std::fs;

fn get_diffs(sequence: Vec<i64>) -> Vec<i64> {
    sequence
        .iter()
        .enumerate()
        .skip(1)
        .fold(vec![], |mut acc, (i, n)| {
            acc.push(n - sequence.get(i - 1).unwrap());
            acc
        })
}

fn extrapolate(sequence: Vec<i64>) -> (i64, i64) {
    let mut all_diffs = vec![];
    let mut is_complete = false;
    all_diffs.push(sequence.clone());
    while !is_complete {
        let diffs = get_diffs(all_diffs.get(all_diffs.len() - 1).unwrap().to_vec());
        is_complete = diffs.iter().all(|n| *n == 0);
        all_diffs.push(diffs.clone());
    }

    all_diffs.iter().rev().fold((0, 0), |(first, last), n| {
        (n[0] - first, n[n.len() - 1] + last)
    })
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let (sum_of_first_numbers, sum_of_next_numbers) = content
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(extrapolate)
        .fold((0, 0), |(first_sum, last_sum), (first, last)| {
            (first_sum + first, last_sum + last)
        });

    println!("sum of next numbers {sum_of_next_numbers:?}");
    println!("sum of first numbers {sum_of_first_numbers:?}");
}
