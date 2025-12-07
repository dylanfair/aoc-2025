use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let start1 = Instant::now();
    part_one("src/day07/real.txt");
    let duration1 = start1.elapsed();
    println!("Part one time: {:?}", duration1);

    let start2 = Instant::now();
    part_two("src/day07/real.txt");
    let duration2 = start2.elapsed();
    println!("Part two time: {:?}", duration2);
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut active_streams = HashSet::new();
    let mut total_splits = 0;

    for line in lines {
        let row = line.unwrap();
        for (col, char) in row.chars().enumerate() {
            if char == 'S' {
                active_streams.insert(col);
            }
            if char == '^' && active_streams.contains(&col) {
                total_splits += 1;
                active_streams.insert(col + 1);
                active_streams.insert(col - 1);
                active_streams.remove(&col);
            }
        }
    }

    println!("{total_splits}");
}

fn part_two(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut timelines: HashMap<usize, u64> = HashMap::new();

    for line in lines {
        let row = line.unwrap();
        for (col, char) in row.chars().enumerate() {
            if char == 'S' {
                timelines.insert(col, 1);
            }
            if char == '^' && timelines.contains_key(&col) {
                let count = timelines.remove(&col).unwrap();
                *timelines.entry(col - 1).or_insert(0) += count;
                *timelines.entry(col + 1).or_insert(0) += count;
            }
        }
    }

    let total: u64 = timelines.values().sum();
    println!("new: {}", total);
}
