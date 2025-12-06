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

    let start2 = Instant::now();
    part_two("src/day05/real.txt");
    let duration2 = start2.elapsed();
    println!("Part two time: {:?}", duration2);

    let start3 = Instant::now();
    part_two_alt("src/day05/real.txt");
    let duration3 = start3.elapsed();
    println!("Part two alt time: {:?}", duration3);
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

fn part_two(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut current_ranges: Vec<(u64, u64)> = vec![];
    for line in lines {
        let line_unwrap = line.unwrap();
        if line_unwrap == "" {
            break;
        }

        let (start, end) = line_unwrap
            .split_once("-")
            .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
            .unwrap();
        current_ranges.push((start, end))
    }

    let mut final_ranges = vec![];
    loop {
        let mut new_ranges: Vec<(u64, u64)> = vec![];

        'outer: for i in 0..current_ranges.len() {
            let range = current_ranges[i];
            if current_ranges.len() == 1 {
                let final_range = (range.0, range.1);
                if !final_ranges.contains(&final_range) {
                    final_ranges.push(final_range);
                }
            }
            if i == current_ranges.len() {
                if !new_ranges.contains(&range) {
                    new_ranges.push(range);
                }
                continue 'outer;
            }

            for j in 0..current_ranges.len() {
                if i == j {
                    continue;
                }
                let alt_range = current_ranges[j];
                let extended_outcome = is_extend_start(alt_range, range);
                match extended_outcome {
                    Extend::Start => {
                        let new_range = (alt_range.0, range.1);
                        if !new_ranges.contains(&new_range) {
                            new_ranges.push(new_range);
                        }
                        continue 'outer;
                    }
                    Extend::End => {
                        let new_range = (range.0, alt_range.1);
                        if !new_ranges.contains(&new_range) {
                            new_ranges.push(new_range);
                        }
                        continue 'outer;
                    }
                    Extend::Both => {
                        let new_range = (alt_range.0, alt_range.1);
                        if !new_ranges.contains(&new_range) {
                            new_ranges.push(new_range);
                        }
                        continue 'outer;
                    }
                    Extend::Inside => {
                        let new_range = (range.0, range.1);
                        if !new_ranges.contains(&new_range) {
                            new_ranges.push(new_range);
                        }
                        continue 'outer;
                    }
                    Extend::Neither => {}
                }
            }
            let final_range = (range.0, range.1);
            if !final_ranges.contains(&final_range) {
                final_ranges.push(final_range);
            }
        }

        if new_ranges.is_empty() {
            break;
        }
        current_ranges = new_ranges
    }

    // println!("{:?}", final_ranges);
    println!("{}", calculate_final_score(final_ranges));
}

#[derive(Debug)]
enum Extend {
    Start,
    End,
    Both,
    Inside,
    Neither,
}

fn is_extend_start(first_range: (u64, u64), second_range: (u64, u64)) -> Extend {
    let is_start_inside = first_range.0 >= second_range.0 && first_range.0 <= second_range.1;
    let is_start_outside = first_range.0 < second_range.0;
    let is_end_inside = first_range.1 >= second_range.0 && first_range.1 <= second_range.1;
    let is_end_outside = first_range.1 > second_range.1;

    if is_start_outside && is_end_outside {
        return Extend::Both;
    }
    if is_start_outside && is_end_inside {
        return Extend::Start;
    }
    if is_start_inside && is_end_outside {
        return Extend::End;
    }
    if is_start_inside && is_end_inside {
        return Extend::Inside;
    }
    return Extend::Neither;
}

fn calculate_final_score(final_ranges: Vec<(u64, u64)>) -> u64 {
    let mut total_score = 0;
    for range in final_ranges {
        total_score += (range.1 - range.0) + 1
    }
    return total_score;
}

fn part_two_alt(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut fresh_ranges: Vec<(u64, u64)> = vec![];

    for line in lines {
        let line_unwrap = line.unwrap();
        if line_unwrap == "" {
            break;
        }

        let (start, end) = line_unwrap
            .split_once("-")
            .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
            .unwrap();
        fresh_ranges.push((start, end))
    }

    fresh_ranges.sort();
    let mut highest = 0;
    let mut total = 0;
    for range in fresh_ranges {
        if range.0 > highest {
            total += range.1 - range.0 + 1;
        } else if range.1 > highest {
            total += range.1 - highest;
        }
        highest = range.1.max(highest);
    }
    println!("{:?}", total);
}
