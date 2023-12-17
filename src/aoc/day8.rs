use std::{collections::HashMap, fs};

use itertools::Itertools;
use regex::Regex;

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day8.txt").unwrap();
    let sections = data.split("\n\n").collect_vec();

    let commands = sections[0].chars().collect_vec();
    let map = sections[1]
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

            let Some((_, [from, left, right])) = re.captures(line).map(|caps| caps.extract())
            else {
                panic!("no match at: {}", line)
            };

            (from, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut counter = 0;
    let mut curr = "AAA";
    loop {
        for c in &commands {
            let ways = map[curr];
            match c {
                'L' => {
                    curr = ways.0;
                }
                'R' => {
                    curr = ways.1;
                }
                _ => unimplemented!(),
            }
            counter += 1;
            if curr == "ZZZ" {
                dbg!(counter);
                return;
            }
        }
    }
}

fn next_field<'a>(map: &'a HashMap<&str, (&str, &str)>, curr: &str, command: char) -> &'a str {
    let ways = map[curr];
    match command {
        'L' => {
            return ways.0;
        }
        'R' => {
            return ways.1;
        }
        _ => unimplemented!(),
    }
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day8.txt").unwrap();
    let sections = data.split("\n\n").collect_vec();

    let commands = sections[0].chars().collect_vec();
    let map = sections[1]
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            let re = Regex::new(r"([0-Z]{3}) = \(([0-Z]{3}), ([0-Z]{3})\)").unwrap();

            let Some((_, [from, left, right])) = re.captures(line).map(|caps| caps.extract())
            else {
                panic!("no match at: {}", line)
            };

            (from, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let start_nodes = map
        .iter()
        .map(|(x, _)| *x)
        .filter(|x| x.ends_with("A"))
        .collect_vec();

    for start_node in start_nodes {
        dbg!(start_node);
        let mut states = vec![(start_node, 0, 0 as i64)];
        loop {
            let curr = states.last().unwrap();
            let command = commands[curr.1];
            let next_field = next_field(&map, curr.0, command);
            let next_command_index = (curr.1 + 1) % commands.len();

            if states
                .iter()
                .find(|(f, c, _)| *f == next_field && *c == next_command_index)
                .is_some()
            {
                states.push((next_field, next_command_index, curr.2 + 1));
                break;
            }
            states.push((next_field, next_command_index, curr.2 + 1));
        }
        let cycle_end = states.last().unwrap();
        let cycle_start = states
            .iter()
            .find(|(f, c, _)| *f == cycle_end.0 && *c == cycle_end.1)
            .unwrap();
        let steps_to_cycle_start = cycle_start.2;
        let cycle_length = cycle_end.2 - cycle_start.2;
        let steps_to_first_stop = states.iter().find(|(f, _, _)| f.ends_with("Z")).unwrap().2;
        dbg!((steps_to_cycle_start, cycle_length, steps_to_first_stop));
    }
}

// LCM(15871, 14257, 16409, 19637, 18023, 12643) = 11795205644011
