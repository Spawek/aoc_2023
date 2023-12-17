use std::{collections::HashSet, fs, hash::RandomState};

use itertools::Itertools;

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day4.txt").unwrap();
    let parsed = data
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(":").collect_vec()[1])
        .map(|x| {
            let lists = x
                .split("|")
                .map(|y| {
                    y.split(" ")
                        .filter(|t| !t.is_empty())
                        .map(|number| number.parse::<i32>().unwrap())
                        .collect_vec()
                })
                .collect_vec();
            (lists[0].clone(), lists[1].clone())
        })
        .collect_vec();

    let ret = parsed
        .iter()
        .map(|(v1, v2)| {
            let h1: HashSet<&i32, RandomState> = HashSet::from_iter(v1);
            let h2: HashSet<&i32, RandomState> = HashSet::from_iter(v2);
            h1.intersection(&h2).count() as u32
        })
        .map(|c| if c == 0 { 0 } else { 2_u32.pow(c - 1) })
        .sum::<u32>();

    dbg!(ret);
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day4.txt").unwrap();
    let parsed = data
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(":").collect_vec()[1])
        .map(|x| {
            let lists = x
                .split("|")
                .map(|y| {
                    y.split(" ")
                        .filter(|t| !t.is_empty())
                        .map(|number| number.parse::<i32>().unwrap())
                        .collect_vec()
                })
                .collect_vec();
            (lists[0].clone(), lists[1].clone())
        })
        .collect_vec();

    let winnings = parsed
        .iter()
        .map(|(v1, v2)| {
            let h1: HashSet<&i32, RandomState> = HashSet::from_iter(v1);
            let h2: HashSet<&i32, RandomState> = HashSet::from_iter(v2);
            h1.intersection(&h2).count() as u64
        })
        .collect_vec();

    let mut cards = vec![1; winnings.len()];
    for i in 0..winnings.len() {
        for j in i + 1..i + 1 + (winnings[i] as usize) {
            cards[j] += cards[i];
        }
    }

    let ret = cards.iter().sum::<u64>();

    dbg!(ret);
}
