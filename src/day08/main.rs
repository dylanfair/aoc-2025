use std::collections::HashMap;
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
    let mut nodes: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, line) in lines.enumerate() {
        let jbox = Box::from_string(line.unwrap());
        boxes.push(jbox);
        nodes.insert(index, vec![]);
    }

    let mut distances = Vec::with_capacity(connections);
    let mut current_max = 0.0;
    for i in 0..boxes.len() {
        let ibox = boxes.get(i).unwrap();
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }
            let jbox = boxes.get(j).unwrap();
            let distance = ibox.distance(&jbox);

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
                for x in 0..connections - 2 {
                    let curr = distances.get(x).unwrap().0;
                    let next = distances.get(x + 1).unwrap().0;

                    if distance > curr && distance < next {
                        let mut right_half = distances.split_off(x + 1);
                        distances.push((distance, (i, j)));
                        right_half.pop();
                        distances.extend(right_half);
                    }
                }
            }
        }
    }
    // println!("{:?}", distances);
    println!("Showing {} distances", distances.len());
    let mut nodes = HashMap::new();
    for (_, node_connection) in distances {
        nodes
            .entry(node_connection.0)
            .or_insert(vec![])
            .push(node_connection.1);
        nodes
            .entry(node_connection.1)
            .or_insert(vec![])
            .push(node_connection.0);
    }
    println!("{:?}", nodes);

    // BFS time?
    let mut visited = vec![];
    for (node, to_visit) in nodes {
        visited.push(node);
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
