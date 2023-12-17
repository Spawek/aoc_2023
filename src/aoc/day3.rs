use std::fs;

use itertools::Itertools;

#[derive(Debug)]
enum Cell {
    Empty,
    Digit(i32),
    Symbol(char),
}

#[derive(Debug, Copy, Clone)]
struct Label {
    code: i32,
    y: usize,
    x_first: usize,
    x_last: usize,
}

fn find_labels(map: &Vec<Vec<Cell>>) -> Vec<Label> {
    let mut ret = vec![];
    for y in 0..map.len() {
        let mut curr: Option<Label> = None;
        for x in 0..map[y].len() {
            match map[y][x] {
                Cell::Digit(val) => {
                    if curr.is_some() {
                        curr.as_mut().unwrap().code = curr.unwrap().code * 10 + val;
                        curr.as_mut().unwrap().x_last = x;
                    } else {
                        curr = Some(Label {
                            code: val,
                            x_first: x,
                            x_last: x,
                            y: y,
                        });
                    }
                }
                _ => {
                    if curr.is_some() {
                        ret.push(curr.unwrap());
                        curr = None;
                    }
                }
            }
        }
        if curr.is_some() {
            ret.push(curr.unwrap());
        }
    }

    ret
}

fn has_neighbouring_symbol(label: &Label, map: &Vec<Vec<Cell>>) -> bool {
    for y in (label.y.max(1) - 1)..=(label.y + 1).min(map.len() - 1) {
        for x in (label.x_first.max(1) - 1)..=(label.x_last + 1).min(map[y].len() - 1) {
            match map[y][x] {
                Cell::Symbol(_) => return true,
                _ => (),
            }
        }
    }

    false
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day3.txt").unwrap();
    let map = data
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0'..='9' => Cell::Digit(c.to_digit(10).unwrap() as i32),
                    '.' => Cell::Empty,
                    s => Cell::Symbol(s),
                })
                .collect_vec()
        })
        .collect_vec();

    let labels = find_labels(&map);
    let ret = labels
        .iter()
        .filter(|label| has_neighbouring_symbol(label, &map))
        .map(|label| label.code)
        .sum::<i32>();

    dbg!(ret);
}

fn find_stars(map: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Cell::Symbol('*') = map[y][x] {
                ret.push((x, y));
            }
        }
    }

    ret
}

fn is_neighbour(p: (usize, usize), label: &Label) -> bool {
    for x in label.x_first..=label.x_last {
        if x.abs_diff(p.0) <= 1 && label.y.abs_diff(p.1) <= 1 {
            return true;
        }
    }

    false
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day3.txt").unwrap();
    let map = data
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0'..='9' => Cell::Digit(c.to_digit(10).unwrap() as i32),
                    '.' => Cell::Empty,
                    s => Cell::Symbol(s),
                })
                .collect_vec()
        })
        .collect_vec();

    let labels = find_labels(&map);
    let stars = find_stars(&map);

    let ret = stars
        .iter()
        .map(|star| {
            labels
                .iter()
                .filter(|label| is_neighbour(*star, label))
                .collect_vec()
        })
        .filter(|labels| labels.len() == 2)
        .map(|labels| labels[0].code * labels[1].code)
        .sum::<i32>();

    dbg!(ret);
}
