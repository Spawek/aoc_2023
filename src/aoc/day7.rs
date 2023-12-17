use std::{collections::HashMap, fs};

use itertools::Itertools;

fn max_char_occurences(s: &str) -> (char, i64) {
    let mut m: HashMap<char, i64> = HashMap::new();
    for c in s.chars() {
        *m.entry(c).or_default() += 1;
    }

    m.into_iter().max_by_key(|(_, x)| *x).unwrap()
}

fn char_value(c: char) -> i64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => unimplemented!(),
    }
}

fn reminder(s: &str) -> i64 {
    let mut ret = 0;
    for c in s.chars() {
        ret = ret * 100 + char_value(c);
    }

    ret
}

fn score(s: &str) -> i64 {
    let (c, max) = max_char_occurences(s);
    let combo = match max {
        5 => 900000000000,
        4 => 800000000000,
        3 => match max_char_occurences(&s.replace(c, &"")).1 {
            2 => 700000000000,
            1 => 600000000000,
            _ => unimplemented!(),
        },
        2 => match max_char_occurences(&s.replace(c, &"")).1 {
            2 => 400000000000,
            1 => 300000000000,
            _ => unimplemented!(),
        },
        1 => 0,
        _ => unimplemented!(),
    };

    combo + reminder(s)
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day7.txt").unwrap();
    let ret = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| line.split(" ").collect_vec())
        .map(|v| (score(v[0]), v[0], v[1].parse::<i64>().unwrap()))
        .sorted_by_key(|(score, _, _)| *score)
        .enumerate()
        .map(|(e, rest)| ((e + 1) as i64, rest))
        .map(|(rank, (_score, _cards, bid))| rank * bid)
        .sum::<i64>();
    // .collect_vec();

    dbg!(ret);
}

fn char_value2(c: char) -> i64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0, // joker
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => unimplemented!(),
    }
}

fn reminder2(s: &str) -> i64 {
    let mut ret = 0;
    for c in s.chars() {
        ret = ret * 100 + char_value2(c);
    }

    ret
}

fn score2(s: &str) -> i64 {
    if s == "JJJJJ" {
        return 900000000000;
    }
    let joker_count = *s.chars().counts().get(&'J').unwrap_or(&0) as i64;
    let s2 = s.replace('J', "");
    let (c, max) = max_char_occurences(&s2);
    let combo = match max + joker_count {
        5 => 900000000000,
        4 => 800000000000,
        3 => match max_char_occurences(&s2.replace(c, &"")).1 {
            2 => 700000000000,
            1 => 600000000000,
            0 => 600000000000,
            _ => unimplemented!(),
        },
        2 => match max_char_occurences(&s2.replace(c, &"")).1 {
            2 => 400000000000,
            1 => 300000000000,
            0 => 300000000000,
            _ => unimplemented!(),
        },
        1 => 0,
        _ => panic!("{}", &s),
    };

    combo + reminder2(&s)
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day7.txt").unwrap();
    let ret = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| line.split(" ").collect_vec())
        .map(|v| (score2(v[0]), v[0], v[1].parse::<i64>().unwrap()))
        .sorted_by_key(|(score, _, _)| *score)
        .enumerate()
        .map(|(e, rest)| ((e + 1) as i64, rest))
        .map(|(rank, (_score, _cards, bid))| rank * bid)
        .sum::<i64>();
    // .collect_vec();

    dbg!(ret);
}
