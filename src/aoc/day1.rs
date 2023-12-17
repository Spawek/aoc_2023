use std::fs;

use itertools::Itertools;

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day1.txt").unwrap();
    let ret = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|c| c.to_digit(10)).flatten().collect_vec())
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum::<u32>();

    dbg!(ret);
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day1.txt").unwrap();
    let ret = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|x| x.chars().map(|c| c.to_digit(10)).flatten().collect_vec())
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum::<u32>();

    dbg!(ret);
}
