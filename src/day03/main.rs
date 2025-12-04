use std::{
    fs::File,
    io::{self, BufRead, Read},
};

fn main() {
    part_one("src/day03/real.txt");
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total = 0;
    for line in lines {
        let batteries = line.unwrap();
        // println!("{}", batteries);
        let batteries_len = batteries.len();
        let mut digit_one = 0;
        let mut digit_two = 0;
        for (i, digit) in batteries.chars().enumerate() {
            let digit_num = digit.to_string().parse::<u32>().unwrap();
            if digit_num > digit_one && i != batteries_len - 1 {
                digit_one = digit_num;
                digit_two = 0;
                continue;
            }
            if digit_num > digit_two && digit_one != 0 {
                digit_two = digit_num;
            }
        }
        total += digit_one * 10;
        total += digit_two;
    }

    println!("{}", total);
}
