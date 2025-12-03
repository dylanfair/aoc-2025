use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    part_one("src/day02/test.txt");
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

    for range in ranges {
        println!("{} to {}", range.0, range.1);
        println!("{}", range.0);
        println!("{}", range.1);
        let start = range.0.parse::<u64>().unwrap();
        let end = range.1.parse::<u64>().unwrap();

        for i in start..=end {
            println!("{i}")
        }
    }
}
