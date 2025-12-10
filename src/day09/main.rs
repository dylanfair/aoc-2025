use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let start1 = Instant::now();
    part_one("src/day09/real.txt");
    let duration1 = start1.elapsed();
    println!("Part one time: {:?}", duration1);
}

fn part_one(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut red_tiles = vec![];

    for line in lines {
        let data = line.unwrap();
        let (x, y) = data.split_once(",").unwrap();
        red_tiles.push((x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()));
    }

    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in i..red_tiles.len() {
            let red_tile_i = red_tiles.get(i).unwrap();
            let red_tile_j = red_tiles.get(j).unwrap();

            let height = red_tile_i.1.abs_diff(red_tile_j.1) + 1;
            let width = red_tile_i.0.abs_diff(red_tile_j.0) + 1;
            let area = height * width;
            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("{max_area}");
}

fn is_point_inside(point: &(u64, u64), polygon: &Vec<((u64, u64), (u64, u64))>) -> bool {
    if ray_cast(point, polygon).is_multiple_of(2) {
        return false;
    }
    true
}

fn ray_cast(point: &(u64, u64), polygon: &Vec<((u64, u64), (u64, u64))>) -> u64 {
    let mut count = 0;
    const OFFSET: u64 = 100_000;
    let point_segment = (point.0 + OFFSET, point.1);

    for side in polygon {
        // check if segment crosses
        if cross {
            count += 1;
        }
    }

    return count;
}
