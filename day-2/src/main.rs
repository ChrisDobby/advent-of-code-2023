use std::fs;

fn get_turn_data(s: &str) -> (u32, u32, u32) {
    return s.split(",").map(|c| c.trim()).fold((0, 0, 0), |acc, c| {
        let mut parts = c.split(" ");
        let count = parts.next().unwrap().parse::<u32>().unwrap();
        let colour = parts.next().unwrap();

        match colour {
            "red" => (acc.0 + count, acc.1, acc.2),
            "green" => (acc.0, acc.1 + count, acc.2),
            "blue" => (acc.0, acc.1, acc.2 + count),
            _ => acc,
        }
    });
}

fn get_game_data(s: &str) -> (u32, Vec<(u32, u32, u32)>) {
    let mut parts = s.split(":");
    let id = parts
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let turns = parts
        .next()
        .unwrap()
        .split(";")
        .map(get_turn_data)
        .collect();

    return (id, turns);
}

const CUBES: (u32, u32, u32) = (12, 13, 14);

fn is_game_valid(g: &(u32, Vec<(u32, u32, u32)>)) -> bool {
    let (_, turns) = g;

    return turns
        .iter()
        .all(|t| t.0 <= CUBES.0 && t.1 <= CUBES.1 && t.2 <= CUBES.2);
}

fn minimum_cubes(g: &(u32, Vec<(u32, u32, u32)>)) -> (u32, u32, u32) {
    let (_, turns) = g;

    return turns.iter().fold((0, 0, 0), |acc, t| {
        (
            if t.0 > acc.0 { t.0 } else { acc.0 },
            if t.1 > acc.1 { t.1 } else { acc.1 },
            if t.2 > acc.2 { t.2 } else { acc.2 },
        )
    });
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let games: Vec<_> = content
        .split("\n")
        .filter(|&line| line != "")
        .map(get_game_data)
        .collect();

    let possible_games: u32 = games.iter().filter(|g| is_game_valid(g)).map(|g| g.0).sum();

    let power: u32 = games
        .iter()
        .map(minimum_cubes)
        .map(|(r, g, b)| r * g * b)
        .sum();

    println!("Possible games: {possible_games}");
    println!("Power: {power}");
}
