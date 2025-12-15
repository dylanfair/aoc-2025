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
        for j in i + 1..red_tiles.len() {
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
    let mut minX = 0;
    let mut minY = 0;
    let mut maxX = 0;
    let mut maxY = 0;
    for i in 0..red_tiles.len() {
        let mut j = i + 1;
        if i == red_tiles.len() - 1 {
            j = 0;
        }
        let tile1 = red_tiles.get(i).unwrap();
        let tile2 = red_tiles.get(j).unwrap();

        segments.push((*tile1, *tile2));

        minX = minX.min(tile1.0);
        maxX = maxX.max(tile1.0);
        minY = minY.min(tile1.1);
        maxY = maxY.max(tile1.1);
    }

    let mut max_area = 0;
    let mut max_redtile_i = (0, 0);
    let mut max_redtile_j = (0, 0);
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            let red_tile_i = red_tiles.get(i).unwrap();
            let red_tile_j = red_tiles.get(j).unwrap();
            let red_tile_x = (red_tile_i.0, red_tile_j.1);
            let red_tile_y = (red_tile_j.0, red_tile_i.1);

            let segment1 = (*red_tile_i, red_tile_x);
            let segment2 = (red_tile_x, *red_tile_j);
            let segment3 = (*red_tile_j, red_tile_y);
            let segment4 = (red_tile_y, *red_tile_i);

            if point_is_in_poly(red_tile_i, &segments, minX, minY, maxX, maxY)
                && point_is_in_poly(red_tile_j, &segments, minX, minY, maxX, maxY)
                && point_is_in_poly(&red_tile_x, &segments, minX, minY, maxX, maxY)
                && point_is_in_poly(&red_tile_y, &segments, minX, minY, maxX, maxY)
                && is_segment_inside(&segment1, &segments)
                && is_segment_inside(&segment2, &segments)
                && is_segment_inside(&segment3, &segments)
                && is_segment_inside(&segment4, &segments)
            {
                let height = red_tile_i.1.abs_diff(red_tile_j.1) + 1;
                let width = red_tile_i.0.abs_diff(red_tile_j.0) + 1;
                let area = height * width;
                if area > max_area {
                    println!("{:?} and {:?} are valid", red_tile_i, red_tile_j);
                    println!("New max! {area}");
                    max_area = area;
                    max_redtile_i = *red_tile_i;
                    max_redtile_j = *red_tile_j;
                }
            }
            // if is_point_inside(red_tile_i, &segments)
            //     && is_point_inside(red_tile_j, &segments)
            //     && is_point_inside(&red_tile_x, &segments)
            //     && is_point_inside(&red_tile_y, &segments)
            //     && is_segment_inside(&segment1, &segments)
            //     && is_segment_inside(&segment2, &segments)
            //     && is_segment_inside(&segment3, &segments)
            //     && is_segment_inside(&segment4, &segments)
            // {
            //     let height = red_tile_i.1.abs_diff(red_tile_j.1) + 1;
            //     let width = red_tile_i.0.abs_diff(red_tile_j.0) + 1;
            //     let area = height * width;
            //     if area > max_area {
            //         println!("{:?} and {:?} are valid", red_tile_i, red_tile_j);
            //         println!("New max! {area}");
            //         max_area = area;
            //         max_redtile_i = *red_tile_i;
            //         max_redtile_j = *red_tile_j;
            //     }
            // }
        }
    }
    println!("{max_area}");
    println!("{max_area}");
    println!("tile i: {:?}", max_redtile_i);
    println!("tile j: {:?}", max_redtile_j);
    let red_tile_x = (max_redtile_i.0, max_redtile_j.1);
    let red_tile_y = (max_redtile_j.0, max_redtile_i.1);
    println!("tile x: {:?}", red_tile_x);
    println!("tile y: {:?}", red_tile_y);
    let segment1 = (max_redtile_i, red_tile_x);
    let segment2 = (red_tile_x, max_redtile_j);
    let segment3 = (max_redtile_j, red_tile_y);
    let segment4 = (red_tile_y, max_redtile_i);
    println!("segement1: {:?}", segment1);
    println!("segement2: {:?}", segment2);
    println!("segement3: {:?}", segment3);
    println!("segement4: {:?}", segment4);
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

fn point_is_in_poly(
    point: &(i64, i64),
    polygon: &LineSegment,
    minX: i64,
    minY: i64,
    maxX: i64,
    maxY: i64,
) -> bool {
    if point.0 < minX || point.0 > maxX || point.1 < minY || point.1 > maxY {
        return false;
    }

    let mut count = 0;
    for segment in polygon {
        if is_between(point, segment) {
            if point.0 == 3611 && point.1 == 95543 {
                println!("Count upped - got an is_between {:?}", segment);
            }
            return true;
        }
        if (segment.0.1 > point.1) != (segment.1.1 > point.1)
            && point.0
                < ((segment.1.0 - segment.0.0) * (point.1 - segment.0.1)
                    / (segment.1.1 - segment.0.1)
                    + segment.0.0)
        {
            count += 1;
        }
    }

    if (count & 1) == 1 { true } else { false }
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

    // if oa != oc && ob != od {
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
            if point.0 == 3611 && point.1 == 95543 {
                println!("Count upped - got an is_between {:?}", side);
            }
            return 1;
        }
        // if check_overlap(&(*point, point_segment), side) {
        //     // println!("Got an overlap! {:?} - {:?}", (point, point_segment), side);
        //     if point.0 == 3611 && point.1 == 95543 {
        //         println!("Count upped - got a check_overlap {:?}", side);
        //     }
        //     count += 1;
        //     continue;
        // }
        // Otherwise need to check if inside or outside
        // check if segment crosses
        if intercept(point, &point_segment, &side.0, &side.1) {
            if point.0 == 3611 && point.1 == 95543 {
                println!("Count upped - got an intercept {:?}", side);
            }
            count += 1;
        }
    }

    if point.0 == 3611 && point.1 == 95543 {
        println!("Count returned was {count}");
    }

    count
}

fn check_overlap(segment1: &((i64, i64), (i64, i64)), segment2: &((i64, i64), (i64, i64))) -> bool {
    // Check x
    let segment1_x_check = segment1.0.0 == segment1.1.0;
    let segment2_x_check = segment2.0.0 == segment2.1.0;
    let segment1_to_2_x_check = segment1.0.0 == segment2.0.0;

    if segment1_x_check && segment2_x_check && segment1_to_2_x_check {
        let segment1_height = segment1.0.1.abs_diff(segment1.1.1);
        let segment2_height = segment2.0.1.abs_diff(segment2.1.1);
        if segment1_height > segment2_height {
            return true;
        }
    }

    // Check y
    let segment1_y_check = segment1.0.1 == segment1.1.1;
    let segment2_y_check = segment2.0.1 == segment2.1.1;
    let segment1_to_2_y_check = segment1.0.1 == segment2.0.1;
    if segment1_y_check && segment2_y_check && segment1_to_2_y_check {
        let segment1_len = segment1.0.0.abs_diff(segment1.1.0);
        let segment2_len = segment2.0.0.abs_diff(segment2.1.0);
        if segment1_len > segment2_len {
            return true;
        }
    }

    false
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

    #[test]
    fn alternative_point_test() {
        let polygon = vec![
            ((1, 1), (1, 3)),
            ((1, 3), (3, 3)),
            ((3, 3), (3, 1)),
            ((3, 1), (1, 1)),
        ];

        let minX = 1;
        let minY = 1;
        let maxX = 3;
        let maxY = 3;

        let inside_point = (2, 2);
        assert!(point_is_in_poly(
            &inside_point,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));

        let edge_point = (3, 2);
        assert!(point_is_in_poly(
            &edge_point,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));

        let other_edge_point = (1, 2);
        assert!(point_is_in_poly(
            &other_edge_point,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));

        let outside_point = (0, 2);
        assert!(!point_is_in_poly(
            &outside_point,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));

        let other_outside_point = (4, 2);
        assert!(!point_is_in_poly(
            &other_outside_point,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));

        let vertex_point1 = (1, 1);
        let vertex_point2 = (1, 3);
        let vertex_point3 = (3, 3);
        let vertex_point4 = (3, 1);
        assert!(point_is_in_poly(
            &vertex_point1,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));
        assert!(point_is_in_poly(
            &vertex_point2,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));
        assert!(point_is_in_poly(
            &vertex_point3,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));
        assert!(point_is_in_poly(
            &vertex_point4,
            &polygon,
            minX,
            minY,
            maxX,
            maxY
        ));
    }
}
