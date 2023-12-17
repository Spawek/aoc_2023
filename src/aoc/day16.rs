use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

type XY = (i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pos: XY,
    direction: XY,
}

fn dirs(c: char) -> HashMap<XY, Vec<XY>> {
    match c {
        '.' => vec![
            ((0, -1), vec![(0, -1)]),
            ((0, 1), vec![(0, 1)]),
            ((-1, 0), vec![(-1, 0)]),
            ((1, 0), vec![(1, 0)]),
        ],
        '/' => vec![
            ((0, -1), vec![(1, 0)]),
            ((0, 1), vec![(-1, 0)]),
            ((-1, 0), vec![(0, 1)]),
            ((1, 0), vec![(0, -1)]),
        ],
        '\\' => vec![
            ((0, -1), vec![(-1, 0)]),
            ((0, 1), vec![(1, 0)]),
            ((-1, 0), vec![(0, -1)]),
            ((1, 0), vec![(0, 1)]),
        ],
        '|' => vec![
            ((0, -1), vec![(0, -1)]),
            ((0, 1), vec![(0, 1)]),
            ((-1, 0), vec![(0, 1), (0, -1)]),
            ((1, 0), vec![(0, -1), (0, 1)]),
        ],
        '-' => vec![
            ((0, -1), vec![(-1, 0), (1, 0)]),
            ((0, 1), vec![(-1, 0), (1, 0)]),
            ((-1, 0), vec![(-1, 0)]),
            ((1, 0), vec![(1, 0)]),
        ],
        _ => unimplemented!("{}", c),
    }
    .into_iter()
    .collect::<HashMap<XY, Vec<XY>>>()
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day16.txt").unwrap();
    let map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut beams = vec![Beam {
        pos: (-1, 0),
        direction: (1, 0),
    }];

    let mut energized = HashSet::new();

    let mut seen = HashSet::new();

    while let Some(beam) = beams.pop() {
        let new_pos = (beam.pos.0 + beam.direction.0, beam.pos.1 + beam.direction.1);
        if new_pos.0 < 0
            || new_pos.0 >= map[0].len() as i64
            || new_pos.1 < 0
            || new_pos.1 >= map.len() as i64
        {
            continue;
        }
        energized.insert(new_pos);
        let new_directions = &dirs(map[new_pos.1 as usize][new_pos.0 as usize])[&beam.direction];
        for dir in new_directions {
            let b = Beam {
                pos: new_pos,
                direction: *dir,
            };
            if !seen.contains(&b) {
                beams.push(b);
                seen.insert(b);
            }
        }
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if energized.contains(&(x as i64, y as i64)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    dbg!(energized.len());
}

fn solve(map: &Vec<Vec<char>>, start_beam: &Beam) -> usize {
    let mut beams = vec![*start_beam];
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();

    while let Some(beam) = beams.pop() {
        let new_pos = (beam.pos.0 + beam.direction.0, beam.pos.1 + beam.direction.1);
        if new_pos.0 < 0
            || new_pos.0 >= map[0].len() as i64
            || new_pos.1 < 0
            || new_pos.1 >= map.len() as i64
        {
            continue;
        }
        energized.insert(new_pos);
        let new_directions = &dirs(map[new_pos.1 as usize][new_pos.0 as usize])[&beam.direction];
        for dir in new_directions {
            let b = Beam {
                pos: new_pos,
                direction: *dir,
            };
            if !seen.contains(&b) {
                beams.push(b);
                seen.insert(b);
            }
        }
    }

    energized.len()
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day16.txt").unwrap();
    let map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut beams = vec![];
    for y in 0..map.len() {
        beams.push(Beam {
            pos: (-1, y as i64),
            direction: (1, 0),
        });
        beams.push(Beam {
            pos: (map[0].len() as i64, y as i64),
            direction: (-1, 0),
        });
    }
    for x in 0..map[0].len() {
        beams.push(Beam {
            pos: (x as i64, -1),
            direction: (0, 1),
        });
        beams.push(Beam {
            pos: (map.len() as i64, -1),
            direction: (0, -1),
        });
    }

    let ret = beams.iter().map(|beam| solve(&map, beam)).max();
    dbg!(ret);
}
