use num::integer::lcm;
use std::{
    collections::{self, HashMap},
    fs, vec,
};

fn get_next_node<'a>(direction: char, node: (String, String)) -> String {
    match direction {
        'R' => node.1,
        _ => node.0,
    }
}

fn find_steps(
    instructions: &str,
    map: &collections::HashMap<String, (String, String)>,
    starting_node: &str,
    test: fn(String) -> bool,
) -> u64 {
    let mut current = String::from(starting_node);
    let mut count = 0;
    while !test(current.clone()) {
        for c in instructions.chars() {
            count += 1;
            current = get_next_node(c, map.get(&current).unwrap().clone());

            if test(current.clone()) {
                break;
            }
        }
    }

    count
}

fn find_lcm(values: &[u64]) -> u64 {
    let mut result = values[0];
    for &value in values.iter().skip(1) {
        result = lcm(result, value);
    }

    result
}

fn parse_content(content: String) -> (String, HashMap<String, (String, String)>, Vec<String>) {
    let mut lines = content.split("\n").filter(|l| !l.is_empty());
    let instructions = lines.next().unwrap().trim().to_string();
    let mut map = collections::HashMap::new();
    let mut starting_nodes = vec![];
    for line in lines {
        let mut parts = line.split("=");
        let key = parts.next().unwrap().trim();
        let value = parts
            .next()
            .map(|p| {
                let items = p
                    .trim()
                    .split(",")
                    .map(|x| x.trim().replace("(", "").replace(")", ""))
                    .collect::<Vec<String>>();
                (items[0].to_owned(), items[1].to_owned())
            })
            .unwrap();
        map.insert(key.to_string(), value);
        if key.ends_with("A") {
            starting_nodes.push(key.to_string());
        }
    }

    (instructions, map, starting_nodes)
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let (instructions, map, starting_nodes) = parse_content(content);

    let steps_to_find_zzz = find_steps(instructions.as_str(), &map, "AAA", |s| s == "ZZZ");
    println!("steps to find zzz {steps_to_find_zzz}");

    let steps_for_each = starting_nodes
        .iter()
        .map(|n| find_steps(instructions.as_str(), &map, n, |s| s.ends_with("Z")))
        .collect::<Vec<u64>>();

    let steps_to_find_all_ending_in_z = find_lcm(&steps_for_each);
    println!("steps to all ending in z {steps_to_find_all_ending_in_z:?}");
}
