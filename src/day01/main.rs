use std::{fs::File, io, io::BufRead};

fn main() {
    part_one("src/day01/real.txt");
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_slice(direction: &str) -> Self {
        match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Should all be left or right"),
        }
    }
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i16,
}

impl Rotation {
    fn new(raw_text: String) -> Rotation {
        let direction_str = &raw_text[0..1];
        let direction = Direction::from_slice(direction_str);

        let distance_str = &raw_text[1..];
        let distance: i16 = distance_str.parse().unwrap();

        Rotation {
            direction,
            distance,
        }
    }
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut current = 50;
    let mut zeros = 0;
    for line in lines {
        let rotation = Rotation::new(line.unwrap());

        if current == 0 && rotation.direction == Direction::Left {
            zeros -= 1;
        }

        match rotation.direction {
            Direction::Left => {
                current = current - rotation.distance;
                loop {
                    if current < 0 {
                        current = 100 + current;
                        zeros += 1;
                    } else {
                        if current == 0 {
                            zeros += 1;
                        }
                        break;
                    }
                }
            }
            Direction::Right => {
                current = current + rotation.distance;
                loop {
                    if current > 99 {
                        current = current - 100;
                        zeros += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("");
    println!("{}", zeros);
}

// starting at 25
// L150
//
// 125 hit 0
// 25 hit 0
// 75
//
// -125 add to 0
// -25 add to 0
