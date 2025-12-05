use std::collections::HashSet;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let start1 = Instant::now();
    part_one("src/day05/real.txt");
    let duration1 = start1.elapsed();
    println!("Part one time: {:?}", duration1);
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut fresh_ranges: Vec<(u64, u64)> = vec![];
    let mut ids = vec![];
    let mut fresh_ids = HashSet::new();

    let mut id_trigger = false;
    for line in lines {
        let line_unwrap = line.unwrap();
        if line_unwrap == "" {
            id_trigger = true;
            continue;
        }

        if id_trigger {
            let id = line_unwrap.parse::<u64>().unwrap();
            ids.push(id);
        } else {
            let (start, end) = line_unwrap
                .split_once("-")
                .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
                .unwrap();
            fresh_ranges.push((start, end))
        }
    }

    for id in ids {
        for range in &fresh_ranges {
            if id >= range.0 && id <= range.1 {
                fresh_ids.insert(id);
            }
        }
    }

    println!("{}", fresh_ids.len());
}
