use std::fs;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Field {
    Wall,
    Empty,
}

type Map = Vec<Vec<Field>>;

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day13.txt").unwrap();
    let maps: Vec<Map> = data
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|map| {
            map.split('\n')
                .filter(|x| !x.is_empty())
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Field::Wall,
                            '.' => Field::Empty,
                            _ => unimplemented!(),
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut ret = 0;
    for (i, map) in maps.iter().enumerate() {
        dbg!(i);
        for dy in 1..map.len() {
            if (0..dy.min(map.len() - dy)).all(|distance| {
                let y1 = dy - distance - 1;
                let y2 = dy + distance;
                map[y1] == map[y2]
            }) {
                ret += 100 * dy;
                dbg!(dy);
            }
        }
        for dx in 1..map[0].len() {
            if (0..dx.min(map[0].len() - dx)).all(|distance| {
                let x1 = dx - distance - 1;
                let x2 = dx + distance;
                map.iter().all(|row| row[x1] == row[x2])
            }) {
                ret += dx;
                dbg!(dx);
            }
        }
    }

    dbg!(ret);
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day13.txt").unwrap();
    let maps: Vec<Map> = data
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|map| {
            map.split('\n')
                .filter(|x| !x.is_empty())
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Field::Wall,
                            '.' => Field::Empty,
                            _ => unimplemented!(),
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut ret = 0;
    for (i, map) in maps.iter().enumerate() {
        dbg!(i);
        for dy in 1..map.len() {
            if (0..dy.min(map.len() - dy))
                .map(|distance| {
                    let y1 = dy - distance - 1;
                    let y2 = dy + distance;
                    (0..map[0].len())
                        .filter(|x| map[y1][*x] != map[y2][*x])
                        .count()
                })
                .sum::<usize>()
                == 1
            {
                ret += 100 * dy;
                dbg!(dy);
            }
        }
        for dx in 1..map[0].len() {
            if (0..dx.min(map[0].len() - dx))
                .map(|distance| {
                    let x1 = dx - distance - 1;
                    let x2 = dx + distance;
                    (0..map.len())
                        .filter(|y| map[*y][x1] != map[*y][x2])
                        .count()
                })
                .sum::<usize>()
                == 1
            {
                ret += dx;
                dbg!(dx);
            }
        }
    }

    dbg!(ret);
}
