use itertools::Itertools;
use rayon::prelude::*;
use std::ops::Bound;
use std::{collections::BTreeMap, fs};

#[derive(Debug, Clone, Copy)]
struct Record {
    from: i64,
    to: i64,
    length: i64,
}

fn map(val: i64, records: &BTreeMap<i64, Record>) -> i64 {
    let cursor = records.upper_bound(Bound::Included(&val));
    if let Some(r) = cursor.value() {
        if val >= r.from && val < r.from + r.length {
            return r.to + (val - r.from);
        }
    }

    val
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day5.txt").unwrap();
    let sections = data.split("\n\n").filter(|x| !x.is_empty()).collect_vec();

    let seeds = sections[0].split(":").collect_vec()[1]
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();

    let maps = sections
        .iter()
        .skip(1)
        .map(|section| {
            section.split(":").collect_vec()[1]
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|line| {
                    let v = line
                        .split(" ")
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_vec();
                    (
                        v[1],
                        Record {
                            from: v[1],
                            to: v[0],
                            length: v[2],
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>()
        })
        .collect_vec();

    let ret = seeds
        .iter()
        .map(|seed| {
            maps.iter()
                .fold(seed.clone(), |curr, m| map(curr, m))
                .clone()
        })
        .min()
        .unwrap();

    dbg!(ret);
}

#[derive(Debug)]
struct SeedRecord {
    from: i64,
    length: i64,
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day5.txt").unwrap();
    let sections = data.split("\n\n").filter(|x| !x.is_empty()).collect_vec();

    let seed_ranges = sections[0].split(":").collect_vec()[1]
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .tuples()
        .map(|(x, y)| SeedRecord { from: x, length: y })
        .collect_vec();

    let maps = sections
        .iter()
        .skip(1)
        .map(|section| {
            section.split(":").collect_vec()[1]
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|line| {
                    let v = line
                        .split(" ")
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_vec();
                    (
                        v[1],
                        Record {
                            from: v[1],
                            to: v[0],
                            length: v[2],
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>()
        })
        .collect_vec();

    dbg!(seed_ranges.iter().map(|x| x.length).sum::<i64>());

    let mut min = i64::MAX;
    for seed_range in seed_ranges {
        println!("doing seed range: {seed_range:?}");
        let ret = (seed_range.from..(seed_range.from + seed_range.length))
            .into_par_iter()
            .map(|seed| {
                maps.iter()
                    .fold(seed.clone(), |curr, m| map(curr, m))
                    .clone()
            })
            .min()
            .unwrap();
        min = min.min(ret);
    }
    dbg!(min);
}
