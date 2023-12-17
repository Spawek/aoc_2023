use std::fs;

use itertools::Itertools;

fn extrapolate(input: &Vec<i64>) -> i64 {
    if input.len() == 1 {
        panic!("size 1 input?");
    }
    if input.iter().all(|x| x == input.first().unwrap()) {
        return *input.first().unwrap();
    }

    let diffs = input.windows(2).map(|arr| arr[1] - arr[0]).collect_vec();
    input.last().unwrap() + extrapolate(&diffs)
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day9.txt").unwrap();
    let inputs = data
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let ret = inputs.iter().map(|input| extrapolate(&input)).sum::<i64>();

    dbg!(ret);
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day9.txt").unwrap();
    let inputs = data
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut v = line
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec();
            v.reverse();
            v
        })
        .collect_vec();

    let ret = inputs.iter().map(|input| extrapolate(&input)).sum::<i64>();

    dbg!(ret);
}
