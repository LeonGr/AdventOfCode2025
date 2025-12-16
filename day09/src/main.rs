use std::collections::HashSet;
use image::{RgbaImage, Rgba};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_rect_mut, draw_polygon_mut};
use imageproc::point::Point;
use imageproc::rect::Rect;
use rayon::prelude::*;
use itertools::Itertools;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}

type Unit = i64;

type Coord = (Unit, Unit);

struct Map {
    tiles: Vec<Coord>,
    max_x: Unit,
    max_y: Unit,
    min_x: Unit,
    min_y: Unit,
}

fn parse(lines: &[String]) -> Map {
    let mut min_x = 100000;
    let mut min_y = 100000;
    let mut max_x = 0;
    let mut max_y = 0;

    let tiles =
        lines.iter()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();

                max_x = Unit::max(x, max_x);
                max_y = Unit::max(y, max_y);
                min_x = Unit::min(x, min_x);
                min_y = Unit::min(y, min_y);


                (x, y)
            })
            .collect();

    Map { tiles, min_x, min_y, max_x, max_y }
}

fn rectangle_area(point_a: &Coord, point_b: &Coord) -> Unit {
    // let side_1 = (point_a.0..=point_b.0);
    // let side_2 = (point_a.1..=point_b.1);
    // let side_1_count = side_1.count();
    // let side_2_count = side_2.count();

    ((point_a.0 - point_b.0).abs() + 1) * ((point_a.1 - point_b.1).abs() + 1)
}

impl Map {
    fn draw(&self, name: &str) {
        let width = 3000;
        let height = 2000;
        let max_x = (self.max_x + 100) as f64;
        let max_y = (self.max_y + 100) as f64;
        let min_x = 0 as f64;
        let min_y = 0 as f64;

        let convert_coord = |(x, y): Coord| {
            let x = x as f64;
            let y = y as f64;

            let x = (x - min_x) / (max_x - min_x);
            let y = (y - min_y) / (max_y - min_y);

            let x = x * (width as f64);
            let y = y * (height as f64);

            (x as i32, y as i32)
        };

        let mut img: RgbaImage = RgbaImage::new(width, height);

        let converted_coords: Vec<(i32, i32)> = self.tiles.iter()
            .map(|coord| {
                let (x, y) = convert_coord(*coord);
                // println!("before: {coord:?} - after: {:?}", (x, y));
                (x, y)
            })
            .collect();

        let poly: Vec<Point<i32>> = converted_coords.iter().map(|&(x, y )| Point::new(x, y)).collect();

        draw_polygon_mut(&mut img, &poly, Rgba([0, 255, 0, 255]));

        converted_coords.iter().for_each(|&(x, y)| {
            draw_filled_circle_mut(&mut img, (x, y), 3, Rgba([255, 0, 0, 255]));
        });

        draw_filled_circle_mut(&mut img, convert_coord((18000, 18000)), 3, Rgba([255, 0, 0, 255]));

        // Save
        img.save(name).expect("save failed");
    }
}


fn part1(map: &Map) -> Unit {
    let mut max_size = 0;

    for point_a in &map.tiles {
        for point_b in &map.tiles {
            let size = rectangle_area(point_a, point_b);
            // println!("{:?} - {:?}, area: {}", point_a, point_b, size);
            if size > max_size {
                max_size = size;
            }
        }
    }

    max_size
}

fn segments_intersect((l_a_x, l_a_y): (Coord, Coord), (l_b_x, l_b_y): (Coord, Coord)) -> bool {
    let orientation = |a: Coord, b: Coord, c: Coord| {
        let value = (b.1 - a.1) * (c.0 - b.0) - (b.0 - a.0) * (c.1 - b.1);
        if value == 0 {
            0
        } else if value > 0 {
            1
        } else {
            2
        }
    };

    let o1 = orientation(l_a_x, l_a_y, l_b_x);
    let o2 = orientation(l_a_x, l_a_y, l_b_y);
    let o3 = orientation(l_b_x, l_b_y, l_a_x);
    let o4 = orientation(l_b_x, l_b_y, l_a_y);

    o1 != o2 && o3 != o4
}

fn get_rectangle_points(point_a: &Coord, point_b: &Coord) -> (Coord, Coord, Coord, Coord) {
    // (0,0)
    // (2, 1)
    // ->
    // (0, 0), (2, 0), (2, 1), (0, 1)
    //
    // (a, b)
    // (c, d)
    let dx = (point_a.0 - point_b.0);
    let dy = (point_a.1 - point_b.1);

    (
        *point_a, (point_a.0, point_a.1 - dy),
        *point_b, (point_a.0 - dx, point_a.1),
    )
}

fn crosses_polygon(map: &Map, (a, b, c, d): (Coord, Coord, Coord, Coord)) -> bool {
    for edge in map.tiles.windows(2) {
        let edge_ab = segments_intersect((edge[0], edge[1]), (a, b));
        let edge_bc = segments_intersect((edge[0], edge[1]), (b, c));
        let edge_cd = segments_intersect((edge[0], edge[1]), (c, d));
        let edge_da = segments_intersect((edge[0], edge[1]), (d, a));
        if edge_ab || edge_bc || edge_cd || edge_da {
            return true;
        }
    }

    false
}

fn hits_edge((x, y): &Coord, edges: &HashSet<(Coord, Coord)>) -> bool {
    for ((s_x, s_y), (e_x, e_y)) in edges {
        if y == s_y && y == e_y {
            if e_x <= x && x <= s_x {
                return true;
            }

            if s_x <= x && x <= e_x {
                return true;
            }
        }

        if x == s_x && x == e_x {
            if e_y <= y && y <= s_y {
                return true;
            }

            if s_y <= y && y <= e_y {
                return true;
            }
        }
    }

    false
}

// fn flood_fill_rec(start @ (x, y): &Coord, seen: &mut HashSet<Coord>, edges: &HashSet<(Coord, Coord)>) {
    // let displacements = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    // seen.insert(*start);

    // if !hits_edge(start, edges) {
        // for (dx, dy) in displacements {
            // let point = (x + dx, y + dy);
            // if seen.contains(&point) {
                // continue;
            // }

            // flood_fill_rec(&point, seen, edges);
        // }
    // }
// }

// fn flood_fill(map: &Map) -> HashSet<Coord> {
    // let start = (18000, 18000);

    // let edges = map.tiles.windows(2)
        // .map(|window| {
            // (window[0], window[1])
        // })
        // .collect();

    // let displacements = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    // let mut seen = HashSet::new();
    // let mut stack = vec![start];

    // while let Some(start @ (x, y)) = stack.pop() {
        // seen.insert(start);

        // if !hits_edge(&start, &edges) {
            // for (dx, dy) in &displacements {
                // let point = (x + dx, y + dy);
                // if seen.contains(&point) {
                    // continue;
                // }

                // stack.push(point);
            // }
        // }
    // }

    // seen
// }

fn is_inside(coord @ (x, y): &Coord, map: &Map) -> bool {
    // println!("Checking if {coord:?} is inside");
    if map.tiles.contains(coord) {
        // println!("result, is vertex: true");
        return true;
    }

    let edges = map.tiles.windows(2)
        .map(|window| {
            (window[0], window[1])
        })
        .collect();

    if hits_edge(coord, &edges) {
        // println!("result, on edge: true");
        return true;
    }

    // let mut edge_hit_count = 0;
    // for nx in *x..=(map.max_x) {
        // if hits_edge(&(nx, *y), &edges) {
            // edge_hit_count += 1;
        // }
    // }

    // edge_hit_count % 2 == 1

    // let mut inside = false;
    // for window in map.tiles.windows(2) {
        // let p1 = window[0];
        // let p2 = window[1];

        // if *y > Unit::min(p1.1, p2.1) {
            // if *y <= Unit::max(p1.1, p2.1) {
                // if *x <= Unit::max(p1.0, p2.0) {
                    // let x_intersection = (y - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1) + p1.0;

                    // if p1.0 == p2.0 || *x <= x_intersection { // Flip the inside flag
                        // inside = !inside;
                    // }
                // }
            // }
        // }
    // }

    // for window in map.tiles.windows(2) {
        // let a = window[0];
        // let b = window[1];
        // if point_on_segment(point, a, b) {
            // return true;
        // }
    // }


    let mut inside = false;
    let n = map.tiles.len();
    for i in 0..n {
    // for window in map.tiles.windows(2) {
        let a = map.tiles[i];
        let b = map.tiles[(i + 1) % n];

        // Check if edge straddles the horizontal line at point.y:
        let cond = ((a.1 > *y) as i32) ^ ((b.1 > *y) as i32); // true if exactly one is above
        if cond != 0 {
            // compute x coordinate of intersection of the edge with horizontal line y = point.y
            let x_intersect = a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1);
            if x_intersect > *x {
                // println!("inside was {inside}, changing to {}", !inside);
                inside = !inside;
            }
            // if x_intersect approx equals point.x, we already handled on-boundary earlier
        }
    }

    // println!("result, else: {inside}");
    inside
}

fn part2(map: &Map) -> Unit {
    // let start = (18000, 18000);
    // let start = (9000, 9000);
    // let inside = is_inside(&start, map);
    // println!("{:?}", inside);
    // todo!();

    // map.draw("test.png");

    // TODO:
    // - create function to check if lines intersect
    // - for each rectangle (any 2 points), check if it's edges intersect with any edges of the
    // Map.
    // let p1 = (0, 0);
    // let p2 = (4, 0);
    // let p3 = (0, 1);
    // let p4 = (4, 1);

    // println!("{}", segments_intersect((p1, p2), (p3, p4)));

    let mut max_size = 0;

    let mut valid_count = 0;

    let point = (2, 1);
    let point_is_inside = is_inside(&point, map);
    assert!(!point_is_inside);

    // let point_a = &(2, 3);
    // let point_b = &(11, 7);
    let point_a = &(2, 5);
    let point_b = &(11, 1);
    // let point_a = &(9, 5);
    // let point_b = &(2, 3);
    // for point_a in &map.tiles {
        // for point_b in &map.tiles {
            // if point_a == point_b {
                // continue;
            // }

            // // let size = rectangle_area(point_a, point_b);
            // let size = rectangle_area(point_a, point_b);
            // let points = get_rectangle_points(point_a, point_b);

            // // println!("{:?} - {:?}, area: {}", point_a, point_b, size);
            // // println!("tiles: {:?}", map.tiles);

            // // println!("Checking if points are inside polygon");
            // let p1 = is_inside(&points.0, map);
            // let p2 = is_inside(&points.1, map);
            // let p3 = is_inside(&points.2, map);
            // let p4 = is_inside(&points.3, map);

            // if p1 && p2 && p3 && p4 {
                // // println!("Checking if edges aren't crossing polygon");
                // if !edges_are_valid(points, map) {
                    // // println!("Edges invalid");
                    // continue;
                // }

                // if size > max_size {
                    // max_size = size;
                // }
            // } else {
                // // println!("invalid");
            // }

            // // println!("{:?} - {:?}, area: {}", point_a, point_b, size);
        // }
    // }

    let permutations: Vec<Vec<&Coord>> = map.tiles.iter().permutations(2).unique().collect();
    permutations.par_iter()
        .map(|points| {
            let point_a = points[0];
            let point_b = points[1];

            if point_a == point_b {
                return 0;
            }

            // let size = rectangle_area(point_a, point_b);
            let size = rectangle_area(point_a, point_b);
            let points = get_rectangle_points(point_a, point_b);

            // println!("{:?} - {:?}, area: {}", point_a, point_b, size);
            // println!("tiles: {:?}", map.tiles);

            // println!("Checking if points are inside polygon");
            let p1 = is_inside(&points.0, map);
            let p2 = is_inside(&points.1, map);
            let p3 = is_inside(&points.2, map);
            let p4 = is_inside(&points.3, map);

            if p1 && p2 && p3 && p4 {
                // println!("Checking if edges aren't crossing polygon");
                if edges_are_valid(points, map) {
                    return size;
                }
            }

            0
        })
        .max().unwrap()


    // println!("valid count: {valid_count}");
    // max_size
}

fn edge_is_valid((p1, p2): (Coord, Coord), map: &Map) -> bool {
    if p1.0 == p2.0 {
        let min_y = Unit::min(p1.1, p2.1);
        let max_y = Unit::max(p1.1, p2.1);

        for y in min_y..=max_y {
            if !is_inside(&(p1.0, y), map) {
                return false;
            }
        }
    } else {
        let min_x = Unit::min(p1.0, p2.0);
        let max_x = Unit::max(p1.0, p2.0);

        for x in min_x..=max_x {
            if !is_inside(&(x, p1.1), map) {
                return false;
            }
        }
    }

    true
}

fn edges_are_valid((p1, p2, p3, p4): (Coord, Coord, Coord, Coord), map: &Map) -> bool {
    let e1 = edge_is_valid((p1, p2), map);
    let e2 = edge_is_valid((p2, p3), map);
    let e3 = edge_is_valid((p3, p4), map);
    let e4 = edge_is_valid((p4, p1), map);

    e1 && e2 && e3 && e4
}
