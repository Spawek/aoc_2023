use std::fs;

use itertools::Itertools;

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day15.txt").unwrap();
    let ret = data
        .replace("\n", "")
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars()
                .fold(0, |acc, next| ((acc + (next as u8) as usize) * 17) % 256)
        })
        .sum::<usize>();

    dbg!(ret);
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, next| ((acc + (next as u8) as usize) * 17) % 256)
}

#[derive(Debug, Clone, Copy)]
enum Op {
    InsertOrReplace(usize),
    Remove,
}

#[derive(Debug, Clone)]
struct Action {
    label: String,
    op: Op,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    strength: usize,
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day15.txt").unwrap();
    let actions = data
        .replace("\n", "")
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| {
            if x.ends_with("-") {
                return Action {
                    label: x[..x.len() - 1].to_string(),
                    op: Op::Remove,
                };
            }
            Action {
                label: x[..x.len() - 2].to_string(),
                op: Op::InsertOrReplace(x[x.len() - 1..].parse().unwrap()),
            }
        })
        .collect_vec();

    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for a in actions {
        let index = hash(&a.label);
        let b = boxes.get_mut(index).unwrap();

        match a.op {
            Op::InsertOrReplace(strength) => {
                if let Some(lens) = b.iter_mut().find(|l| l.label == a.label) {
                    lens.strength = strength;
                } else {
                    b.push(Lens {
                        label: a.label,
                        strength: strength,
                    });
                }
            }
            Op::Remove => {
                if let Some(index) = b.iter().position(|x| x.label == a.label) {
                    b.remove(index);
                }
            }
        }
    }

    let ret = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, lens)| (i + 1) * (j + 1) * lens.strength)
                .sum::<usize>()
        })
        .sum::<usize>();

    dbg!(ret);
}
