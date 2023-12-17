use std::{collections::HashMap, fs};

use cached::proc_macro::cached;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Field {
    Round,
    Cube,
    Empty,
}

type Map = Vec<Vec<Field>>;

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day14.txt").unwrap();
    let mut map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Field::Round,
                    '#' => Field::Cube,
                    '.' => Field::Empty,
                    _ => unimplemented!(),
                })
                .collect_vec()
        })
        .collect_vec();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Field::Round {
                let mut cy = y;
                while cy > 0 && map[cy - 1][x] == Field::Empty {
                    map[cy - 1][x] = Field::Round;
                    map[cy][x] = Field::Empty;
                    cy -= 1;
                }
            }
        }
    }

    let mut ret = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Field::Round {
                ret += map.len() - y;
            }
        }
    }

    dbg!(ret);
} // 108792

#[cached]
fn round(mut map: Vec<Vec<Field>>) -> Vec<Vec<Field>> {
    // north
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Field::Round {
                let mut cy = y;
                while cy > 0 && map[cy - 1][x] == Field::Empty {
                    map[cy - 1][x] = Field::Round;
                    map[cy][x] = Field::Empty;
                    cy -= 1;
                }
            }
        }
    }

    // west
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == Field::Round {
                let mut cx = x;
                while cx > 0 && map[y][cx - 1] == Field::Empty {
                    map[y][cx - 1] = Field::Round;
                    map[y][cx] = Field::Empty;
                    cx -= 1;
                }
            }
        }
    }

    // south
    for y in (0..map.len()).rev() {
        for x in 0..map[0].len() {
            if map[y][x] == Field::Round {
                let mut cy = y;
                while cy < map.len() - 1 && map[cy + 1][x] == Field::Empty {
                    map[cy + 1][x] = Field::Round;
                    map[cy][x] = Field::Empty;
                    cy += 1;
                }
            }
        }
    }

    // east
    for x in (0..map[0].len()).rev() {
        for y in 0..map.len() {
            if map[y][x] == Field::Round {
                let mut cx = x;
                while cx < map[0].len() - 1 && map[y][cx + 1] == Field::Empty {
                    map[y][cx + 1] = Field::Round;
                    map[y][cx] = Field::Empty;
                    cx += 1;
                }
            }
        }
    }

    map
}

fn print(map: &Vec<Vec<Field>>) {
    println!();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!(
                "{}",
                match map[y][x] {
                    Field::Round => "O",
                    Field::Cube => "#",
                    Field::Empty => ".",
                }
            )
        }
        println!();
    }
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day14.txt").unwrap();
    let mut map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Field::Round,
                    '#' => Field::Cube,
                    '.' => Field::Empty,
                    _ => unimplemented!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut history = HashMap::new();
    history.insert(map.clone(), vec![0]);
    let mut cycle_begin = 0;
    let mut cycle_length = 0;
    for i in 1..=100000 {
        map = round(map);
        history.entry(map.clone()).or_default().push(i);
        if history[&map].len() > 1 {
            cycle_begin = history[&map][0];
            cycle_length = history[&map][1] - history[&map][0];
            break;
        }
    }
    dbg!(cycle_begin, cycle_length);

    let rounds_needed = (1000000000 - cycle_begin) % cycle_length;
    for _ in 0..rounds_needed {
        map = round(map);
    }

    let mut ret = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Field::Round {
                ret += map.len() - y;
            }
        }
    }

    dbg!(ret);
}
