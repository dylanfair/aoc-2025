use std::collections::HashSet;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let start1 = Instant::now();
    part_one("src/day08/real.txt", 1000);
    let duration1 = start1.elapsed();
    println!("Part one time: {:?}", duration1);

    let start2 = Instant::now();
    part_two("src/day08/real.txt", 10000);
    let duration2 = start2.elapsed();
    println!("Part two time: {:?}", duration2);
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

    fn distance(&self, other: &Box) -> f64 {
        let adds =
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        (adds as f64).sqrt()
    }
}

fn part_one(path: &str, connections: usize) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut boxes = vec![];
    for line in lines {
        let jbox = Box::from_string(line.unwrap());
        boxes.push(jbox);
    }

    let mut calculted_connections = HashSet::new();
    let mut distances = Vec::with_capacity(connections);
    let mut current_max = 0.0;
    for i in 0..boxes.len() {
        let ibox = boxes.get(i).unwrap();
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }
            if calculted_connections.contains(&(i, j)) || calculted_connections.contains(&(j, i)) {
                continue;
            }
            let jbox = boxes.get(j).unwrap();
            let distance = ibox.distance(jbox);
            calculted_connections.insert((i, j));

            if distances.len() < connections {
                distances.push((distance, (i, j)));
                if distances.len() == connections {
                    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    current_max = distances.get(connections - 1).unwrap().0;
                }
                continue;
            }

            if distance >= current_max {
                continue;
            } else {
                for x in 0..connections - 1 {
                    let curr = distances.get(x).unwrap().0;
                    let next = distances.get(x + 1).unwrap().0;

                    if distance <= curr {
                        distances.insert(0, (distance, (i, j)));
                        distances.pop();
                        current_max = distances.get(connections - 1).unwrap().0;
                        break;
                    }

                    if distance >= curr && distance <= next {
                        let mut right_half = distances.split_off(x + 1);
                        distances.push((distance, (i, j)));
                        right_half.pop();
                        distances.extend(right_half);
                        // println!("Distances after fix: {:?}", distances);
                        current_max = distances.get(connections - 1).unwrap().0;
                        break;
                    }
                }
            }
        }
    }

    // Now get our connections
    let mut conns = vec![];
    'outer: for (_, boxes) in distances.iter() {
        if conns.is_empty() {
            conns.push(HashSet::from([boxes.0, boxes.1]));
        } else {
            for conn in conns.iter_mut() {
                if conn.contains(&boxes.0) || conn.contains(&boxes.1) {
                    conn.insert(boxes.0);
                    conn.insert(boxes.1);
                    continue 'outer;
                }
            }
            conns.push(HashSet::from([boxes.0, boxes.1]));
        }
    }

    // Loop through merging overlapping connections together
    let mut prior_length = conns.len();
    loop {
        let mut intermediate_conns = vec![];
        let mut merged = vec![];

        for i in 0..conns.len() {
            let i_conn = conns.get(i).unwrap();
            let mut final_conn = i_conn.clone();
            if merged.contains(&i) {
                continue;
            }
            for j in 0..conns.len() {
                if i == j {
                    continue;
                }
                let j_conn = conns.get(j).unwrap();
                for value in i_conn {
                    if j_conn.contains(value) {
                        final_conn.extend(j_conn);
                        merged.push(j);
                    }
                }
            }
            intermediate_conns.push(final_conn);
        }

        if intermediate_conns.len() == prior_length {
            break;
        }
        prior_length = intermediate_conns.len();
        conns = intermediate_conns;
    }

    let mut sizes: Vec<usize> = conns.iter().map(|conn| conn.len()).collect();
    sizes.sort_by(|a, b| b.cmp(a));
    let answer = sizes.iter().take(3).product::<usize>();
    println!("{}", answer);
}

fn part_two(path: &str, connections: usize) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut boxes = vec![];
    for line in lines {
        let jbox = Box::from_string(line.unwrap());
        boxes.push(jbox);
    }

    // let connections = connections;
    let mut calculted_connections = HashSet::new();
    let mut distances = Vec::with_capacity(connections);
    let mut current_max = 0.0;
    for i in 0..boxes.len() {
        let ibox = boxes.get(i).unwrap();
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }
            if calculted_connections.contains(&(i, j)) || calculted_connections.contains(&(j, i)) {
                continue;
            }
            let jbox = boxes.get(j).unwrap();
            let distance = ibox.distance(jbox);
            calculted_connections.insert((i, j));

            if distances.len() < connections {
                distances.push((distance, (i, j)));
                if distances.len() == connections {
                    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    current_max = distances.get(connections - 1).unwrap().0;
                }
                continue;
            }

            if distance >= current_max {
                continue;
            } else {
                for x in 0..connections - 1 {
                    let curr = distances.get(x).unwrap().0;
                    let next = distances.get(x + 1).unwrap().0;

                    if distance <= curr {
                        distances.insert(0, (distance, (i, j)));
                        distances.pop();
                        current_max = distances.get(connections - 1).unwrap().0;
                        break;
                    }

                    if distance >= curr && distance <= next {
                        let mut right_half = distances.split_off(x + 1);
                        distances.push((distance, (i, j)));
                        right_half.pop();
                        distances.extend(right_half);
                        current_max = distances.get(connections - 1).unwrap().0;
                        break;
                    }
                }
            }
        }
    }

    // Now get our connections
    let mut conns = vec![];
    for (_, pairs) in distances {
        conns.push(HashSet::from([pairs.0, pairs.1]));

        let mut prior_length = conns.len();
        loop {
            let mut intermediate_conns = vec![];
            let mut merged = vec![];

            for i in 0..conns.len() {
                let i_conn = conns.get(i).unwrap();
                let mut final_conn = i_conn.clone();
                if merged.contains(&i) {
                    continue;
                }
                for j in 0..conns.len() {
                    if i == j {
                        continue;
                    }
                    let j_conn = conns.get(j).unwrap();
                    for value in i_conn {
                        if j_conn.contains(value) {
                            final_conn.extend(j_conn);
                            merged.push(j);
                        }
                    }
                }
                intermediate_conns.push(final_conn);
            }

            if intermediate_conns.len() == prior_length {
                break;
            }
            prior_length = intermediate_conns.len();
            conns = intermediate_conns;
        }
        if conns.first().unwrap().len() == boxes.len() {
            println!("{} - {}", pairs.0, pairs.1);
            let pair0_x = boxes.get(pairs.0).unwrap().x;
            let pair1_x = boxes.get(pairs.1).unwrap().x;
            println!("answer: {}", pair0_x * pair1_x);
            return;
        }
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
