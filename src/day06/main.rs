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
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut operators = vec![];
    let mut totals: HashMap<usize, u64> = HashMap::new();
    let mut values: Vec<Vec<u64>> = vec![];

    // First pass to get operators
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
