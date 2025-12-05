use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    part_one("src/day04/real.txt");
    part_two("src/day04/real.txt");
}

fn valid_roll(coord: &(i16, i16), grid: &HashMap<(i16, i16), bool>) -> bool {
    const MOVEMENTS: [(i16, i16); 8] = [
        (-1, -1),
        (-1, 1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (0, 1),
    ];

    let mut surrounding_rolls = 0;
    for movement in MOVEMENTS {
        let row = coord.0 + movement.0;
        let col = coord.1 + movement.1;

        if let Some(roll) = grid.get(&(row, col)) {
            if *roll {
                surrounding_rolls += 1;
                if surrounding_rolls == 4 {
                    return false;
                }
            }
        }
    }

    return true;
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut grid = HashMap::new();
    let mut total = 0;

    for (row, line) in lines.enumerate() {
        let rolls = line.unwrap();
        for (column, roll) in rolls.chars().enumerate() {
            if roll == '.' {
                grid.insert((row as i16, column as i16), false);
            } else {
                grid.insert((row as i16, column as i16), true);
            }
        }
    }

    for (coord, roll) in grid.iter() {
        if *roll {
            if valid_roll(coord, &grid) {
                total += 1;
            }
        }
    }

    println!("{total}");
}

fn part_two(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut grid = HashMap::new();
    let mut total = 0;

    for (row, line) in lines.enumerate() {
        let rolls = line.unwrap();
        for (column, roll) in rolls.chars().enumerate() {
            if roll == '.' {
                grid.insert((row as i16, column as i16), false);
            } else {
                grid.insert((row as i16, column as i16), true);
            }
        }
    }

    let mut prior_total = 0;
    loop {
        let mut coords_to_set_to_false = vec![];
        for coord in grid.keys() {
            let roll = grid.get(coord).unwrap();
            if *roll {
                if valid_roll(coord, &grid) {
                    total += 1;
                    coords_to_set_to_false.push(*coord);
                }
            }
        }
        if total == prior_total {
            break;
        }
        prior_total = total;
        for coords in coords_to_set_to_false {
            grid.insert(coords, false);
        }
    }

    println!("{total}");
}
