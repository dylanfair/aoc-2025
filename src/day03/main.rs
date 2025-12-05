use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    part_one("src/day03/real.txt");
    part_two("src/day03/real.txt");
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total = 0;
    for line in lines {
        let batteries = line.unwrap();
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

fn part_two(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total = 0;
    for line in lines {
        let batteries = line.unwrap();
        // println!("{}", batteries);
        let batteries_len = batteries.len();
        let mut digits: [u64; 12] = [0; 12];

        for (i, digit) in batteries.chars().enumerate() {
            place_battery(&mut digits, digit, batteries_len, i);
        }
        total = add_battery_to_total(total, digits);
    }

    println!("{}", total);
}

fn place_battery(digits: &mut [u64; 12], curr_battery: char, batteries_len: usize, place: usize) {
    let digit_num = curr_battery.to_string().parse::<u64>().unwrap();
    for i in 0..12 {
        let digit_i = digits[i];
        if digit_num > digit_i && 11 - i < batteries_len - place {
            digits[i] = digit_num;
            for j in i + 1..12 {
                digits[j] = 0;
            }
            return;
        }
    }
}

fn add_battery_to_total(mut total: u64, digits: [u64; 12]) -> u64 {
    let base: u64 = 10;
    for i in 0..12 {
        let digit = digits[i];
        let factor: u64 = base.pow(11 - (i as u32));
        total += digit * factor;
    }
    return total;
}
