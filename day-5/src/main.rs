use std::fs;

type Lookup = Vec<(u64, u64, u64)>;

fn add_to_lookup(
    lookup_name: &str,
    lookup_data: (u64, u64, u64),
    lookups: [Lookup; 7],
) -> [Lookup; 7] {
    let mut updated_lookups = lookups.clone();
    match lookup_name {
        "seed-to-soil map:" => updated_lookups[0].push(lookup_data),
        "soil-to-fertilizer map:" => updated_lookups[1].push(lookup_data),
        "fertilizer-to-water map:" => updated_lookups[2].push(lookup_data),
        "water-to-light map:" => updated_lookups[3].push(lookup_data),
        "light-to-temperature map:" => updated_lookups[4].push(lookup_data),
        "temperature-to-humidity map:" => updated_lookups[5].push(lookup_data),
        "humidity-to-location map:" => updated_lookups[6].push(lookup_data),
        _ => {}
    }

    updated_lookups
}

fn create_lookups(lines: Vec<&str>) -> [Lookup; 7] {
    lines
        .iter()
        .fold(
            ("", [vec![], vec![], vec![], vec![], vec![], vec![], vec![]]),
            |(current_lookup, lookups): (&str, [Lookup; 7]), l| {
                if l.contains(":") {
                    return (l, lookups);
                }

                let lookup_line = l
                    .trim()
                    .split(" ")
                    .map(|s| s.trim().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                (
                    current_lookup,
                    add_to_lookup(
                        current_lookup,
                        (lookup_line[0], lookup_line[1], lookup_line[2]),
                        lookups,
                    ),
                )
            },
        )
        .1
}

fn get_seeds(line: &str) -> Vec<u64> {
    line.split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn get_seed_ranges(line: &str) -> Vec<(u64, u64)> {
    line.split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.trim().parse::<u64>().unwrap())
        .fold((0, 0, vec![]), |(start, _, mut seeds), n| {
            if start == 0 {
                return (n, 0, seeds);
            }

            seeds.push((start, n));
            (0, 0, seeds)
        })
        .2
}

fn get_destination(source: u64, lookup: Lookup) -> u64 {
    let source_lookup = lookup
        .iter()
        .find(|l| source >= l.1 && source <= l.1 + l.2 - 1);

    if source_lookup.is_some() {
        source_lookup.unwrap().0 + (source - source_lookup.unwrap().1)
    } else {
        source
    }
}

fn get_source(desination: u64, lookup: Lookup) -> u64 {
    let desination_lookup = lookup
        .iter()
        .find(|l| desination >= l.0 && desination <= l.0 + l.2 - 1);

    if desination_lookup.is_some() {
        desination_lookup.unwrap().1 + (desination - desination_lookup.unwrap().0)
    } else {
        desination
    }
}

fn find_seed_location(seed: u64, lookups: [Lookup; 7]) -> u64 {
    lookups.iter().fold(seed as u64, |source, lookup| {
        get_destination(source, lookup.clone())
    })
}

fn find_seed_for_location(location: u64, lookups: [Lookup; 7]) -> u64 {
    lookups.iter().rev().fold(location, |destination, lookup| {
        get_source(destination, lookup.clone())
    })
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let lines = content
        .split("\n")
        .filter(|&line| line != "")
        .collect::<Vec<&str>>();

    let seeds = get_seeds(lines[0]);
    let lookups = create_lookups(lines[1..].to_vec());
    let seed_ranges = get_seed_ranges(lines[0]);

    let lowest_location = seeds
        .iter()
        .map(|s| find_seed_location(*s, lookups.clone()))
        .min()
        .unwrap();

    println!("Lowest location {lowest_location:?}");

    for location in 0..1000000000 {
        let seed: u64 = find_seed_for_location(location, lookups.clone());
        let range = seed_ranges
            .iter()
            .find(|r| r.0 <= seed && r.0 + r.1 >= seed);
        if range.is_some() {
            println!("Lowest location from range {location}");
            break;
        }
    }
}
