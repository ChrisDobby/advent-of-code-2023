use std::{fs, vec};

fn get_coordinates(
    params: (usize, &str),
) -> (Vec<(usize, usize, usize, u32)>, Vec<(usize, usize, char)>) {
    let (y, s) = params;
    let coordinates = s.chars().enumerate().fold(
        (vec![], vec![], "".to_string()),
        |acc: (
            Vec<(usize, usize, usize, u32)>,
            Vec<(usize, usize, char)>,
            String,
        ),
         (x, c)| {
            let (mut numbers, mut symbols, mut current_number) = acc.clone();
            if c.is_digit(10) {
                return (numbers, symbols, format!("{current_number}{c}"));
            }

            if current_number != "" {
                numbers.push((
                    x - current_number.len(),
                    x,
                    y,
                    current_number.parse::<u32>().unwrap(),
                ));
                current_number = "".to_string();
            }

            if c != '.' {
                symbols.push((x, y, c));
            }

            return (numbers, symbols, current_number);
        },
    );

    let (mut numbers, symbols, current_number) = coordinates;
    if current_number != "" {
        numbers.push((
            s.len() - current_number.len(),
            s.len(),
            y,
            current_number.parse::<u32>().unwrap(),
        ));
    }

    return (numbers, symbols);
}

fn is_part_number(
    coordinates: (usize, usize, usize, u32),
    symbols: Vec<(usize, usize, char)>,
) -> bool {
    let (x_start, x_end, y, _) = coordinates;

    let x_range = if x_start > 0 { x_start - 1 } else { x_start }..x_end + 1;
    let y_range = if y > 0 { y - 1 } else { y }..y + 2;

    return symbols
        .iter()
        .find(|(symbol_x, symbol_y, _)| x_range.contains(symbol_x) && y_range.contains(symbol_y))
        .is_some();
}

fn calculate_gear_ratio(
    coordinates: (usize, usize, char),
    numbers: Vec<(usize, usize, usize, u32)>,
) -> u32 {
    let (x, y, _) = coordinates;
    let x_range = if x > 0 { x - 1 } else { x }..x + 2;
    let y_range = if y > 0 { y - 1 } else { y }..y + 2;

    let adjacent_numbers = numbers
        .iter()
        .filter(|(x_start, x_end, y, _)| {
            y_range.contains(y) && (x_range.contains(x_start) || x_range.contains(&(x_end - 1)))
        })
        .map(|(_, _, _, n)| n)
        .collect::<Vec<&u32>>();

    return if adjacent_numbers.len() == 2 {
        **adjacent_numbers.get(0).unwrap() * **adjacent_numbers.get(1).unwrap()
    } else {
        0
    };
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let coordinates = content
        .split("\n")
        .filter(|&line| line != "")
        .enumerate()
        .map(get_coordinates)
        .collect::<Vec<(Vec<(usize, usize, usize, u32)>, Vec<(usize, usize, char)>)>>();

    let numbers = coordinates
        .iter()
        .map(|c| c.0.clone())
        .flatten()
        .collect::<Vec<(usize, usize, usize, u32)>>();
    let symbols = coordinates
        .iter()
        .map(|c| c.1.clone())
        .flatten()
        .collect::<Vec<(usize, usize, char)>>();

    let part_numbers: u32 = numbers
        .iter()
        .filter(|c| is_part_number(**c, symbols.clone()))
        .map(|c| c.3)
        .sum();

    let gear_ratio: u32 = symbols
        .iter()
        .filter(|(_, _, s)| *s == '*')
        .map(|g| calculate_gear_ratio(*g, numbers.clone()))
        .sum();

    println!("Part numbers {part_numbers:?}");
    println!("Gear ratio {gear_ratio:?}");
}
