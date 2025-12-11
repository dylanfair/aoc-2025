use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufRead},
};

type LineSegment = Vec<((i64, i64), (i64, i64))>;

fn main() {
    let start1 = Instant::now();
    part_one("src/day09/real.txt");
    let duration1 = start1.elapsed();
    println!("Part one time: {:?}", duration1);

    let start2 = Instant::now();
    part_two("src/day09/real.txt");
    let duration2 = start2.elapsed();
    println!("Part two time: {:?}", duration2);
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

fn part_two(path: &str) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut red_tiles = vec![];

    for line in lines {
        let data = line.unwrap();
        let (x, y) = data.split_once(",").unwrap();
        red_tiles.push((x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()));
    }

    let mut segments = vec![];
    for i in 0..red_tiles.len() {
        let mut j = i + 1;
        if i == red_tiles.len() - 1 {
            j = 0;
        }
        let tile1 = red_tiles.get(i).unwrap();
        let tile2 = red_tiles.get(j).unwrap();

        segments.push((*tile1, *tile2));
    }

    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in i..red_tiles.len() {
            let red_tile_i = red_tiles.get(i).unwrap();
            let red_tile_j = red_tiles.get(j).unwrap();
            let red_tile_x = (red_tile_i.0, red_tile_j.1);
            let red_tile_y = (red_tile_j.0, red_tile_i.1);

            let segment1 = (*red_tile_i, red_tile_x);
            let segment2 = (red_tile_x, *red_tile_j);
            let segment3 = (*red_tile_j, red_tile_y);
            let segment4 = (red_tile_y, *red_tile_i);

            if is_point_inside(red_tile_i, &segments)
                && is_point_inside(red_tile_j, &segments)
                && is_point_inside(&red_tile_x, &segments)
                && is_point_inside(&red_tile_y, &segments)
                && is_segment_inside(&segment1, &segments)
                && is_segment_inside(&segment2, &segments)
                && is_segment_inside(&segment3, &segments)
                && is_segment_inside(&segment4, &segments)
            {
                let height = red_tile_i.1.abs_diff(red_tile_j.1) + 1;
                let width = red_tile_i.0.abs_diff(red_tile_j.0) + 1;
                let area = height * width;
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    println!("{max_area}");
}

fn is_point_inside(point: &(i64, i64), polygon: &LineSegment) -> bool {
    let cast = ray_cast(point, polygon);
    if cast.is_multiple_of(2) {
        return false;
    }
    true
}

fn is_segment_inside(segment: &((i64, i64), (i64, i64)), polygon: &LineSegment) -> bool {
    for side in polygon {
        // check if segment crosses
        if intercept(&segment.0, &segment.1, &side.0, &side.1) {
            return false;
        }
    }
    true
}

fn cross(point_a: &(i64, i64), point_b: &(i64, i64)) -> i64 {
    point_a.0 * point_b.1 - point_a.1 * point_b.0
}

fn orient(point_a: &(i64, i64), point_b: &(i64, i64), point_c: &(i64, i64)) -> i64 {
    // let value = ((point_b.1 - point_a.1) * (point_c.0 - point_b.0))
    //     - ((point_b.0 - point_a.0) - (point_c.1 - point_b.1));
    //
    // if value == 0 {
    //     return 0;
    // }
    // if value > 0 { 1 } else { 2 }
    //
    cross(
        &(point_b.0 - point_a.0, point_b.1 - point_a.1),
        &(point_c.0 - point_a.0, point_c.1 - point_a.1),
    )
}

fn intercept(
    point_a: &(i64, i64),
    point_b: &(i64, i64),
    point_c: &(i64, i64),
    point_d: &(i64, i64),
) -> bool {
    let oa = orient(point_a, point_b, point_c);
    let ob = orient(point_a, point_b, point_d);
    let oc = orient(point_c, point_d, point_a);
    let od = orient(point_c, point_d, point_b);

    oa * ob < 0 && oc * od < 0

    // if oa != ob && oc != od {
    //     return true;
    // }
    //
    // if oa == 0 && on_segment(point_a, point_c, point_b) {
    //     return true;
    // }
    // if ob == 0 && on_segment(point_a, point_d, point_b) {
    //     return true;
    // }
    // if oc == 0 && on_segment(point_c, point_a, point_d) {
    //     return true;
    // }
    // if od == 0 && on_segment(point_c, point_b, point_d) {
    //     return true;
    // }
    //
    // false
}

fn ray_cast(point: &(i64, i64), polygon: &LineSegment) -> u64 {
    let mut count = 0;
    const OFFSET: i64 = 100_000;
    let point_segment = (point.0 + OFFSET, point.1);

    for side in polygon {
        // If on a segment, we can just exit early
        if is_between(point, side) {
            return 1;
        }
        // Otherwise need to check if inside or outside
        // check if segment crosses
        if intercept(point, &point_segment, &side.0, &side.1) {
            count += 1;
        }
    }

    count
}

fn is_between(point: &(i64, i64), side: &((i64, i64), (i64, i64))) -> bool {
    let crossproduct =
        (point.1 - side.0.1) * (side.1.0 - side.0.0) - (point.0 - side.0.0) * (side.1.1 - side.0.1);

    if crossproduct.abs() != 0 {
        return false;
    }

    let dotproduct =
        (point.0 - side.0.0) * (side.1.0 - side.0.0) + (point.1 - side.0.1) * (side.1.1 - side.0.1);
    if dotproduct < 0 {
        return false;
    }

    let squaredlengthba = (side.1.0 - side.0.0).pow(2) + (side.1.1 - side.0.1).pow(2);
    if dotproduct > squaredlengthba {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_between() {
        let line_segment = ((2, 1), (2, 3));

        let point = (2, 2);
        assert!(is_between(&point, &line_segment));

        let point_at_edge = (2, 1);
        assert!(is_between(&point_at_edge, &line_segment));
    }

    #[test]
    fn test_intercept() {
        let segment = ((2, 4), (8, 4));

        let smaller_segment = ((3, 4), (7, 4));
        assert!(!intercept(
            &segment.0,
            &segment.1,
            &smaller_segment.0,
            &smaller_segment.1
        ));

        let bigger_segment = ((1, 4), (9, 4));
        assert!(!intercept(
            &segment.0,
            &segment.1,
            &bigger_segment.0,
            &bigger_segment.1
        ));

        let cross_segment = ((6, 3), (6, 5));
        assert!(intercept(
            &segment.0,
            &segment.1,
            &cross_segment.0,
            &cross_segment.1
        ));
    }

    #[test]
    fn ray_cast_testing() {
        let polygon = vec![
            ((1, 1), (1, 3)),
            ((1, 3), (3, 3)),
            ((3, 3), (3, 1)),
            ((3, 1), (1, 1)),
        ];

        let inside_point = (2, 2);
        assert!(is_point_inside(&inside_point, &polygon));

        let edge_point = (3, 2);
        assert!(is_point_inside(&edge_point, &polygon));

        let other_edge_point = (1, 2);
        assert!(is_point_inside(&other_edge_point, &polygon));

        let outside_point = (0, 2);
        assert!(!is_point_inside(&outside_point, &polygon));

        let other_outside_point = (4, 2);
        assert!(!is_point_inside(&other_outside_point, &polygon));

        let vertex_point1 = (1, 1);
        let vertex_point2 = (1, 3);
        let vertex_point3 = (3, 3);
        let vertex_point4 = (3, 1);
        assert!(is_point_inside(&vertex_point1, &polygon));
        assert!(is_point_inside(&vertex_point2, &polygon));
        assert!(is_point_inside(&vertex_point3, &polygon));
        assert!(is_point_inside(&vertex_point4, &polygon));
    }
}
