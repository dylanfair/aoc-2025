use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    part_one("src/day02/real.txt");
    part_two("src/day02/real.txt");
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let mut input = String::new();
    io::BufReader::new(file).read_to_string(&mut input).unwrap();

    let ranges: Vec<(&str, &str)> = input
        .split(',')
        .into_iter()
        .map(|range| range.trim_ascii().split_once('-').unwrap())
        .collect();

    let mut total_sum = 0;
    for range in ranges {
        let start = range.0.parse::<u64>().unwrap();
        let end = range.1.parse::<u64>().unwrap();

        'number: for i in start..=end {
            let i_string = i.to_string();
            let i_string_len = i_string.len();
            if i_string_len % 2 == 0 {
                let i_string_len_half = i_string_len / 2;
                let first_half = &i_string[..i_string_len_half];
                let second_half = &i_string[i_string_len_half..];

                if first_half == second_half {
                    total_sum += i;
                }
            }
        }
    }
    println!("{total_sum}");
}

fn part_two(path: &str) {
    let file = File::open(path).unwrap();
    let mut input = String::new();
    io::BufReader::new(file).read_to_string(&mut input).unwrap();

    let ranges: Vec<(&str, &str)> = input
        .split(',')
        .into_iter()
        .map(|range| range.trim_ascii().split_once('-').unwrap())
        .collect();

    let mut total_sum = 0;
    for range in ranges {
        let start = range.0.parse::<u64>().unwrap();
        let end = range.1.parse::<u64>().unwrap();

        'number: for i in start..=end {
            let i_string = i.to_string();
            let i_string_len = i_string.len();

            // Sliding window problem?
            // Lookup how to do those
            'window: for window_size in 1..=(i_string_len / 2) {
                let mut left = 0;
                let mut right = left + window_size;
                let comparison = &i_string[left..right];
                let mut valid = true;

                loop {
                    left += window_size;
                    right += window_size;
                    if right > i_string_len {
                        valid = false;
                        break;
                    }

                    let current_window = &i_string[left..right];
                    if current_window != comparison {
                        continue 'window;
                    }

                    if right == i_string_len {
                        break;
                    }
                }
                if valid == true {
                    // println!("{i}");
                    total_sum += i;
                    continue 'number;
                }
            }
        }
    }
    println!("{total_sum}");
}
