use std::{collections::HashMap, fs};

use itertools::Itertools;
use priority_queue::PriorityQueue;

type XY = (i64, i64);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    pos: XY,
    direction: XY,
    steps_forward: i64,
}

fn possible_dirs(curr_dir: XY) -> Vec<XY> {
    match curr_dir {
        (0, -1) => vec![(0, -1), (1, 0), (-1, 0)],
        (0, 1) => vec![(0, 1), (1, 0), (-1, 0)],
        (1, 0) => vec![(1, 0), (0, -1), (0, 1)],
        (-1, 0) => vec![(-1, 0), (0, -1), (0, 1)],
        _ => unimplemented!(),
    }
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day17.txt").unwrap();
    let map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect_vec()
        })
        .collect_vec();

    let initial_states = vec![
        State {
            pos: (0, 0),
            direction: (1, 0),
            steps_forward: 0,
        },
        State {
            pos: (0, 0),
            direction: (0, 1),
            steps_forward: 0,
        },
    ];

    let mut costs = HashMap::new();
    let mut queue = PriorityQueue::new();

    for initial_state in initial_states {
        costs.insert(initial_state, 0);
        queue.push(initial_state, 0);
    }

    while let Some((curr, minus_cost)) = queue.pop() {
        let curr_cost = -1 * minus_cost;
        for next_dir in possible_dirs(curr.direction) {
            let next_pos = (curr.pos.0 + next_dir.0, curr.pos.1 + next_dir.1);
            if next_pos.0 < 0
                || next_pos.0 >= map[0].len() as i64
                || next_pos.1 < 0
                || next_pos.1 >= map.len() as i64
            {
                continue;
            }

            let next_steps_forward = if next_dir == curr.direction {
                curr.steps_forward + 1
            } else {
                0
            };

            if next_steps_forward > 2 {
                continue;
            }

            let next_cost = curr_cost + map[next_pos.1 as usize][next_pos.0 as usize];
            let next_state = State {
                pos: next_pos,
                direction: next_dir,
                steps_forward: next_steps_forward,
            };

            if let Some(lowest_cost_at_next_state) = costs.get(&next_state) {
                if next_cost < *lowest_cost_at_next_state {
                    *costs.get_mut(&next_state).unwrap() = next_cost;
                    queue.push(next_state, -1 * next_cost);
                }
            } else {
                costs.insert(next_state, next_cost);
                queue.push(next_state, -1 * next_cost);
            }
        }
    }

    dbg!(costs.len());

    let ret = costs
        .iter()
        .filter(|s| s.0.pos == (map[0].len() as i64 - 1, map.len() as i64 - 1))
        .map(|s| s.1)
        .min()
        .unwrap();

    dbg!(ret);
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day17.txt").unwrap();
    let map = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect_vec()
        })
        .collect_vec();

    let initial_states = vec![
        State {
            pos: (0, 0),
            direction: (1, 0),
            steps_forward: 0,
        },
        State {
            pos: (0, 0),
            direction: (0, 1),
            steps_forward: 0,
        },
    ];

    let mut costs = HashMap::new();
    let mut queue = PriorityQueue::new();

    for initial_state in initial_states {
        costs.insert(initial_state, 0);
        queue.push(initial_state, 0);
    }

    while let Some((curr, minus_cost)) = queue.pop() {
        let curr_cost = -1 * minus_cost;
        for next_dir in possible_dirs(curr.direction) {
            let next_pos = (curr.pos.0 + next_dir.0, curr.pos.1 + next_dir.1);
            if next_pos.0 < 0
                || next_pos.0 >= map[0].len() as i64
                || next_pos.1 < 0
                || next_pos.1 >= map.len() as i64
            {
                continue;
            }

            let next_steps_forward = if next_dir == curr.direction {
                curr.steps_forward + 1
            } else {
                1
            };

            if next_dir != curr.direction && curr.steps_forward < 4 {
                continue;
            }

            if next_steps_forward > 10 {
                continue;
            }

            let next_cost = curr_cost + map[next_pos.1 as usize][next_pos.0 as usize];
            let next_state = State {
                pos: next_pos,
                direction: next_dir,
                steps_forward: next_steps_forward,
            };

            if let Some(lowest_cost_at_next_state) = costs.get(&next_state) {
                if next_cost < *lowest_cost_at_next_state {
                    *costs.get_mut(&next_state).unwrap() = next_cost;
                    queue.push(next_state, -1 * next_cost);
                }
            } else {
                costs.insert(next_state, next_cost);
                queue.push(next_state, -1 * next_cost);
            }
        }
    }

    dbg!(costs.len());

    let ret = costs
        .iter()
        .filter(|s| s.0.pos == (map[0].len() as i64 - 1, map.len() as i64 - 1) && s.0.steps_forward >= 4)
        .map(|s| s.1)
        .min()
        .unwrap();

    dbg!(ret);
}
