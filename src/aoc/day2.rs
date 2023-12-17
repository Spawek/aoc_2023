use std::fs;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

fn get_regex_int(regex: &str, s: &str) -> Option<i32> {
    let re = Regex::new(regex).unwrap();

    let Some((_, [ret])) = re.captures(s).map(|caps| caps.extract()) else {
        return None;
    };

    Some(ret.parse().unwrap())
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day2.txt").unwrap();
    let ret = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| Game {
            id: get_regex_int(r"Game (\d+): ", line).unwrap(),
            sets: line
                .split(";")
                .map(|s| Set {
                    red: get_regex_int(r"(\d+) red", s).unwrap_or(0),
                    green: get_regex_int(r"(\d+) green", s).unwrap_or(0),
                    blue: get_regex_int(r"(\d+) blue", s).unwrap_or(0),
                })
                .collect_vec(),
        })
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<i32>();

    dbg!(ret);
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day2.txt").unwrap();
    let ret = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| Game {
            id: get_regex_int(r"Game (\d+): ", line).unwrap(),
            sets: line
                .split(";")
                .map(|s| Set {
                    red: get_regex_int(r"(\d+) red", s).unwrap_or(0),
                    green: get_regex_int(r"(\d+) green", s).unwrap_or(0),
                    blue: get_regex_int(r"(\d+) blue", s).unwrap_or(0),
                })
                .collect_vec(),
        })
        .map(|game| {
            game.sets
                .into_iter()
                .reduce(|lhs, rhs| Set {
                    red: lhs.red.max(rhs.red),
                    green: lhs.green.max(rhs.green),
                    blue: lhs.blue.max(rhs.blue),
                })
                .unwrap()
        })
        .map(|set| set.red * set.green * set.blue)
        .sum::<i32>();

    dbg!(ret);
}
