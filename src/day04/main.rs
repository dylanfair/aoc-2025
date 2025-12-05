use std::collections::HashSet;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let start1 = Instant::now();
    part_one("src/day04/real.txt");
    let duration1 = start1.elapsed();
    println!("Part one time: {:?}", duration1);

    let start2 = Instant::now();
    part_two("src/day04/real.txt");
    let duration2 = start2.elapsed();
    println!("Part two time: {:?}", duration2);
}

fn valid_roll(coord: &(i16, i16), grid: &HashSet<(i16, i16)>) -> bool {
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

        if grid.contains(&(row, col)) {
            surrounding_rolls += 1;
            if surrounding_rolls == 4 {
                return false;
            }
        }
    }

    return true;
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut grid = HashSet::new();
    let mut total = 0;

    for (row, line) in lines.enumerate() {
        let rolls = line.unwrap();
        for (column, roll) in rolls.chars().enumerate() {
            if roll == '@' {
                grid.insert((row as i16, column as i16));
            }
        }
    }

    for coord in grid.iter() {
        if valid_roll(coord, &grid) {
            total += 1;
        }
    }

    println!("{total}");
}

fn part_two(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut grid = HashSet::new();
    let mut total = 0;

    for (row, line) in lines.enumerate() {
        let rolls = line.unwrap();
        for (column, roll) in rolls.chars().enumerate() {
            if roll == '@' {
                grid.insert((row as i16, column as i16));
            }
        }
    }

    let mut prior_total = 0;
    loop {
        let mut coords_to_set_to_false = vec![];
        for coord in grid.iter() {
            if valid_roll(coord, &grid) {
                total += 1;
                coords_to_set_to_false.push(*coord);
            }
        }
        if total == prior_total {
            break;
        }
        prior_total = total;
        for coords in coords_to_set_to_false {
            grid.remove(&coords);
        }
    }

    println!("{total}");
}
