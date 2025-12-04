use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    part_one("src/day03/test.txt");
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let mut input = String::new();
    io::BufReader::new(file).read_to_string(&mut input).unwrap();
    println!("{input}");
}
