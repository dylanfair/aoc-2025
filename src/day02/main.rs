use std::{fs::File, io, io::BufRead};

fn main() {
    part_one("src/day02/test.txt");
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
}
