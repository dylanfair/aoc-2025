use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let start1 = Instant::now();
    part_one("src/day08/test.txt", 10);
    let duration1 = start1.elapsed();
    println!("Part one time: {:?}", duration1);
}

#[derive(Debug)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Box {
    fn from_string(input: String) -> Box {
        let mut input_split = input.split(',');
        Box {
            x: input_split.next().unwrap().parse::<i64>().unwrap(),
            y: input_split.next().unwrap().parse::<i64>().unwrap(),
            z: input_split.next().unwrap().parse::<i64>().unwrap(),
        }
    }

    fn distance(self, other: &Box) -> f64 {
        let adds =
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        (adds as f64).sqrt()
    }
}

fn part_one(path: &str, connections: u32) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut boxes = vec![];
    for line in lines {
        let jbox = Box::from_string(line.unwrap());
        boxes.push(jbox);
    }

    for jbox in boxes {
        println!("{:?}", jbox);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_distance() {
        let box1 = Box::from_string("162,817,812".to_string());
        let box2 = Box::from_string("425,690,689".to_string());

        let distance = box1.distance(&box2);
        assert_eq!(distance, 316.90219311326956)
    }
}
