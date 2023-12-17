use std::fs;

use itertools::Itertools;

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day11.txt").unwrap();
    let map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut empty_y = vec![];
    for y in 0..map.len() {
        if map[y].iter().all(|c| *c == '.') {
            empty_y.push(y);
        }
    }

    let mut empty_x = vec![];
    for x in 0..map.len() {
        if map.iter().all(|row| row[x] == '.') {
            empty_x.push(x);
        }
    }

    let mut galaxies = vec![];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
    }

    for galaxy in &mut galaxies {
        for y in empty_y.iter().rev() {
            if galaxy.1 > *y {
                galaxy.1 += 1;
            }
        }

        for x in empty_x.iter().rev() {
            if galaxy.0 > *x {
                galaxy.0 += 1;
            }
        }
    }

    let mut ret = 0;
    for i in 0..galaxies.len() {
        let g1 = galaxies[i];
        for j in i + 1..galaxies.len() {
            let g2 = galaxies[j];
            ret += g1.0.abs_diff(g2.0);
            ret += g1.1.abs_diff(g2.1);
        }
    }

    dbg!(ret);
} // 10490062

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day11.txt").unwrap();
    let map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut empty_y = vec![];
    for y in 0..map.len() {
        if map[y].iter().all(|c| *c == '.') {
            empty_y.push(y);
        }
    }

    let mut empty_x = vec![];
    for x in 0..map.len() {
        if map.iter().all(|row| row[x] == '.') {
            empty_x.push(x);
        }
    }

    let mut galaxies = vec![];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
    }

    for galaxy in &mut galaxies {
        let mut yc = 0;
        for y in &empty_y {
            if galaxy.1 > *y {
                yc += 1;
            }
        }
        galaxy.1 += yc * (1000000 - 1);

        let mut xc = 0;
        for x in &empty_x {
            if galaxy.0 > *x {
                xc += 1;
            }
        }
        galaxy.0 += xc * (1000000 - 1);
    }

    let mut ret = 0;
    for i in 0..galaxies.len() {
        let g1 = galaxies[i];
        for j in i + 1..galaxies.len() {
            let g2 = galaxies[j];
            ret += g1.0.abs_diff(g2.0);
            ret += g1.1.abs_diff(g2.1);
        }
    }

    dbg!(ret);
} // 382979724122
