use std::fs;

fn get_races(s: String) -> Vec<(u64, u64)> {
    let lines = s
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|l| {
            l.split(":")
                .last()
                .unwrap()
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    lines
        .get(0)
        .unwrap()
        .iter()
        .zip(lines.get(1).unwrap().iter())
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<(u64, u64)>>()
}

fn get_lowest_winning_wait(
    wait_time: u64,
    last_checked_time: u64,
    total_time: u64,
    winning_distance: u64,
) -> u64 {
    let calculated_distance = wait_time * (total_time - wait_time);
    if calculated_distance == winning_distance {
        return wait_time;
    }

    if calculated_distance > winning_distance {
        return get_lowest_winning_wait(wait_time / 2, wait_time, total_time, winning_distance);
    }

    let diff = last_checked_time.abs_diff(wait_time);
    if diff == 1 {
        return wait_time + 1;
    }

    get_lowest_winning_wait(
        wait_time + (diff / 2),
        wait_time.max(last_checked_time),
        total_time,
        winning_distance,
    )
}

fn get_number_of_ways_to_win(race: &(u64, u64)) -> u64 {
    let (time, distance) = race;
    let lowest_winning_wait_time =
        get_lowest_winning_wait(time / 2, *time / 2, *time, distance + 1);

    let highest_winning_wait_time = time - lowest_winning_wait_time;
    highest_winning_wait_time - lowest_winning_wait_time + 1
}

fn create_single_race(races: Vec<(u64, u64)>) -> (u64, u64) {
    let single_race = races.iter().fold(
        ("".to_string(), "".to_string()),
        |(total_time, total_distance), (time, distance)| {
            (
                format!("{}{}", total_time, time),
                format!("{}{}", total_distance, distance),
            )
        },
    );

    (
        single_race.0.parse::<u64>().unwrap(),
        single_race.1.parse::<u64>().unwrap(),
    )
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let races = get_races(content);
    let part_1_result: u64 = races.iter().map(get_number_of_ways_to_win).product();

    println!("Part 1 result {part_1_result:?}");

    let single_race = create_single_race(races);
    let part_2_result = get_number_of_ways_to_win(&single_race);

    println!("Part 2 result {part_2_result:?}");
}
