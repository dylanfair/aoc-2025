use std::collections::HashMap;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let start1 = Instant::now();
    part_one("src/day06/real.txt");
    let duration1 = start1.elapsed();
    println!("Part one time: {:?}", duration1);

    let start2 = Instant::now();
    part_two("src/day06/real.txt");
    let duration2 = start2.elapsed();
    println!("Part two time: {:?}", duration2);
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut operators = vec![];
    let mut totals: HashMap<usize, u64> = HashMap::new();
    let mut values: Vec<Vec<u64>> = vec![];

    // First pass to get values
    for line in lines {
        let data = line.unwrap();
        if &data[0..1] == "*" || &data[0..1] == "+" {
            operators = data
                .split(" ")
                .filter_map(|v| {
                    if !v.is_empty() {
                        Some(v.to_string())
                    } else {
                        None
                    }
                })
                .collect();
        } else {
            values.push(
                data.split(" ")
                    .filter_map(|v| {
                        if !v.is_empty() {
                            Some(v.parse::<u64>().unwrap())
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        }
    }

    for data in values {
        for (column, value) in data.iter().enumerate() {
            let operator = operators.get(column).unwrap();
            if let Some(curr_total) = totals.get(&column) {
                let new_total = if operator == "*" {
                    *curr_total * value
                } else {
                    *curr_total + value
                };

                totals.insert(column, new_total);
            } else {
                totals.insert(column, *value);
            }
        }
    }

    // println!("{:?}", totals);
    let final_total: u64 = totals.values().sum();
    println!("{final_total}");
}

fn part_two(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut operators = vec![];
    let mut totals: HashMap<usize, u64> = HashMap::new();
    // (pointer) (column, Number as string)
    let mut numbers: HashMap<usize, (u64, String)> = HashMap::new();

    // First pass to get values
    for line in lines {
        let data = line.unwrap();
        if &data[0..1] == "*" || &data[0..1] == "+" {
            operators = data
                .split(" ")
                .filter_map(|v| {
                    if !v.is_empty() {
                        Some(v.to_string())
                    } else {
                        None
                    }
                })
                .collect();
        } else {
            let mut column: i64 = -1;
            let mut start_incrementing = true;
            for (pointer, char) in data.chars().enumerate() {
                if char != ' ' && start_incrementing {
                    column += 1;
                    start_incrementing = false;
                }
                if char == ' ' && !start_incrementing {
                    start_incrementing = true;
                }

                if char == ' ' {
                    continue;
                }

                if let Some((col, number)) = numbers.get(&pointer) {
                    let updated_number = format!("{number}{char}");
                    numbers.insert(pointer, (*col, updated_number));
                } else {
                    numbers.insert(pointer, (column as u64, char.to_string()));
                }
            }
        }
    }

    for (column, value) in numbers.values() {
        let col_usize = *column as usize;
        let operator = operators.get(col_usize).unwrap();
        let value_digit = value.parse::<u64>().unwrap();
        if let Some(curr_total) = totals.get(&col_usize) {
            let new_total = if operator == "*" {
                *curr_total * value_digit
            } else {
                *curr_total + value_digit
            };

            totals.insert(col_usize, new_total);
        } else {
            totals.insert(col_usize, value_digit);
        }
    }
    let final_total: u64 = totals.values().sum();
    println!("{final_total}");
}
