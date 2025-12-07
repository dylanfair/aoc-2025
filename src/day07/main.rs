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
                let mut left = *timelines.get(&col).unwrap();
                let mut right = *timelines.get(&col).unwrap();
                if let Some(existing_l_timelines) = timelines.get(&(col - 1)) {
                    left += existing_l_timelines;
                }
                if let Some(existing_r_timelines) = timelines.get(&(col + 1)) {
                    right += existing_r_timelines;
                }
                timelines.insert(col - 1, left);
                timelines.insert(col + 1, right);
                timelines.remove(&col);
            }
        }
    }

    let total: u64 = timelines.values().sum();
    println!("new: {}", total);
}
