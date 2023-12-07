use itertools::Itertools;
use std::{cmp::Ordering, fs, vec};

fn apply_jokers(jokers_count: usize, grouped: Vec<usize>) -> Vec<usize> {
    if jokers_count == 0 {
        return grouped;
    }

    if jokers_count == 5 {
        return vec![5];
    }

    let with_jokers = grouped.iter().max().unwrap() + jokers_count;
    match with_jokers {
        5 => vec![5],
        4 => vec![4, 1],
        3 if grouped.len() == 2 => vec![3, 2],
        3 => vec![3, 1, 1],
        2 if grouped.len() == 4 => vec![2, 1, 1, 1],
        _ => vec![2, 2, 1],
    }
}

fn get_hand(s: &str, with_joker: bool) -> (&str, u32, Vec<usize>) {
    let mut split = s.split_whitespace();
    let cards = split.next().unwrap();
    let bid = split.next().unwrap().parse::<u32>().unwrap();
    let mut chars = cards
        .chars()
        .filter(|c| !with_joker || *c != 'J')
        .collect::<Vec<char>>();
    chars.sort();
    let mut grouped = vec![];
    for (_, group) in &chars.into_iter().group_by(|c| *c) {
        grouped.push(group.count());
    }

    (
        cards,
        bid,
        if with_joker {
            apply_jokers(cards.chars().filter(|c| *c == 'J').count(), grouped)
        } else {
            grouped
        },
    )
}

fn get_ranking(groups: Vec<usize>) -> u32 {
    match groups.len() {
        1 => 7,
        2 if groups.iter().find(|count| **count == 4).is_some() => 6,
        2 => 5,
        3 if groups.iter().find(|count| **count == 3).is_some() => 4,
        3 => 3,
        4 => 2,
        _ => 1,
    }
}

fn card_value(c: &char, j_value: u32) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => j_value,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

fn cmp_cards(cards1: &str, cards2: &str, j_value: u32) -> Ordering {
    for (i, card1) in cards1.chars().enumerate() {
        let card1_value = card_value(&card1, j_value);
        let card2_value = card_value(&cards2.chars().nth(i).unwrap(), j_value);

        if card1_value != card2_value {
            return card1_value.cmp(&card2_value);
        }
    }

    Ordering::Equal
}

fn compare_hands(
    (cards1, _, grouped1): (&str, u32, Vec<usize>),
    (cards2, _, grouped2): (&str, u32, Vec<usize>),
    j_value: u32,
) -> Ordering {
    let ranking1 = get_ranking(grouped1);
    let ranking2 = get_ranking(grouped2);

    if ranking1 != ranking2 {
        return ranking1.cmp(&ranking2);
    }

    cmp_cards(cards1, cards2, j_value)
}

fn get_winnings(content: String, with_joker: bool) -> u32 {
    let mut hands = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|s| get_hand(s, with_joker))
        .collect::<Vec<(&str, u32, Vec<usize>)>>();

    let j_value = if with_joker { 1 } else { 11 };
    hands.sort_by(|a, b| compare_hands(a.clone(), b.clone(), j_value));

    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid, _))| ((i + 1) as u32) * bid)
        .sum()
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let winnings = get_winnings(content.clone(), false);
    println!("winnings {:?}", winnings);

    let winnings_with_jokers = get_winnings(content.clone(), true);
    println!("winnings with jokers {:?}", winnings_with_jokers);
}
