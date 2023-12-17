use std::{collections::HashSet, fs};

use itertools::Itertools;

type XY = (i64, i64);

#[derive(Debug)]
struct Cell {
    start: bool,
    letter: char,
    connections: HashSet<XY>,
}

fn find_start(map: &Vec<Vec<Cell>>) -> XY {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x].start {
                return (x as i64, y as i64);
            }
        }
    }

    unimplemented!()
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day10.txt").unwrap();
    let map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| Cell {
                    start: c == 'S',
                    letter: c,
                    connections: (match c {
                        '|' => vec![(0, -1), (0, 1)],
                        '-' => vec![(-1, 0), (1, 0)],
                        'L' => vec![(0, -1), (1, 0)],
                        'J' => vec![(0, -1), (-1, 0)],
                        '7' => vec![(-1, 0), (0, 1)],
                        'F' => vec![(0, 1), (1, 0)],
                        'S' => vec![(-1, 0), (1, 0)], // hardcoded  // is it even needed?
                        '.' => vec![],
                        _ => unimplemented!("{}", c),
                    })
                    .into_iter()
                    .collect::<HashSet<XY>>(),
                })
                .collect_vec()
        })
        .collect_vec();

    let start = find_start(&map);
    let mut curr_pos = start;
    let mut curr_direction = (1, 0); // go right first
    let mut path = vec![curr_direction];
    loop {
        let next_pos = (curr_pos.0 + curr_direction.0, curr_pos.1 + curr_direction.1);
        let next_cell = &map[next_pos.1 as usize][next_pos.0 as usize];
        let mut next_directions = next_cell.connections.clone();
        let removed = next_directions.remove(&(-1 * curr_direction.0, -1 * &curr_direction.1));
        if !removed {
            if !next_cell.start {
                panic!("Should be start");
            }
            break;
        }
        let next_direction = *next_directions.iter().next().unwrap();

        path.push(next_pos);
        curr_pos = next_pos;
        curr_direction = next_direction;
    }
    dbg!(path.len() / 2);
} // 6831

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RayPosition {
    Above,
    At,
    Below,
}

fn ray_position(ray_start: XY, point: XY) -> RayPosition {
    if point.0 >= ray_start.0 && point.1 == ray_start.1 {
        RayPosition::At
    } else if point.1 < ray_start.1 {
        RayPosition::Above
    } else {
        // left = below, but it doesn't matter
        RayPosition::Below
    }
}

fn ray_count(ray_start: XY, cells: &Vec<XY>) -> i64 {
    let mut cells = cells.clone();
    cells.push(*cells.first().unwrap()); // to make a full loop

    let mut ret = 0;
    let mut prev_at_below = None;
    let mut prev = None;
    // dbg!(ray_start);
    for cell in cells {
        let pos = ray_position(ray_start, cell);
        // dbg!((cell, pos));
        if pos != RayPosition::At
            && prev.is_some()
            && prev.unwrap() == RayPosition::At
            && ((pos == RayPosition::Above
                && prev_at_below.is_some()
                && prev_at_below.unwrap() == RayPosition::Below)
                || (pos == RayPosition::Below
                    && prev_at_below.is_some()
                    && prev_at_below.unwrap() == RayPosition::Above))
        {
            ret += 1;
            // dbg!("++");
        }

        prev = Some(pos);
        if pos == RayPosition::Above || pos == RayPosition::Below {
            prev_at_below = Some(pos);
        }
    }

    // if ret % 2 == 1{
    // dbg!((ray_start, ret));
    // }

    ret
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day10.txt").unwrap();
    let map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| Cell {
                    start: c == 'S',
                    letter: c,
                    connections: (match c {
                        '|' => vec![(0, -1), (0, 1)],
                        '-' => vec![(-1, 0), (1, 0)],
                        'L' => vec![(0, -1), (1, 0)],
                        'J' => vec![(0, -1), (-1, 0)],
                        '7' => vec![(-1, 0), (0, 1)],
                        'F' => vec![(0, 1), (1, 0)],
                        'S' => vec![(-1, 0), (1, 0)], // hardcoded  // is it even needed?
                        '.' => vec![],
                        _ => unimplemented!("{}", c),
                    })
                    .into_iter()
                    .collect::<HashSet<XY>>(),
                })
                .collect_vec()
        })
        .collect_vec();

    let start = find_start(&map);
    let mut curr_pos = start;
    let mut curr_direction = (1, 0); // go right first
    let mut path = vec![curr_direction];
    let mut cells = vec![start];
    loop {
        let next_pos = (curr_pos.0 + curr_direction.0, curr_pos.1 + curr_direction.1);
        let next_cell = &map[next_pos.1 as usize][next_pos.0 as usize];
        if next_cell.start {
            break;
        }
        let mut next_directions = next_cell.connections.clone();
        let removed = next_directions.remove(&(-1 * curr_direction.0, -1 * &curr_direction.1));
        if !removed {
            panic!();
        }
        let next_direction = *next_directions.iter().next().unwrap();

        path.push(next_pos);
        cells.push(next_pos);
        curr_pos = next_pos;
        curr_direction = next_direction;
    }

    let mut ret = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !cells.contains(&(x as i64, y as i64)) {
                if (ray_count((x as i64, y as i64), &cells) % 2) == 1 {
                    ret += 1;
                    dbg!((x, y, ray_count((x as i64, y as i64), &cells)));
                }
            }
        }
    }
    dbg!(ret);
}
