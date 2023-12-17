use std::fs;

use itertools::Itertools;

use cached::proc_macro::cached;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Query {
    map: Vec<char>,
    blocks: Vec<i64>,
}

#[cached]
fn solve(q: Query) -> i64 {
    if q.blocks.len() == 0 {
        if q.map.iter().all(|c| *c == '.' || *c == '?') {
            return 1;
        } else {
            return 0;
        }
    }

    let max_blanks =
        q.map.len() as i64 - q.blocks.iter().sum::<i64>() - (q.blocks.len() as i64 - 1);
    let mut ret = 0;
    for blanks in 0..=max_blanks {
        if q.map
            .iter()
            .take(blanks as usize)
            .all(|c| *c == '.' || *c == '?')
            && q.map
                .iter()
                .skip(blanks as usize)
                .take(*q.blocks.first().unwrap() as usize)
                .all(|c| *c == '#' || *c == '?')
            && q.map
                .iter()
                .skip(blanks as usize + *q.blocks.first().unwrap() as usize)
                .take(1)
                .all(|c| *c == '.' || *c == '?')
        {
            ret += solve(Query {
                map: q
                    .map
                    .iter()
                    .skip(blanks as usize + *q.blocks.first().unwrap() as usize + 1)
                    .map(|x| *x)
                    .collect_vec(),
                blocks: q.blocks.iter().skip(1).map(|x| *x).collect_vec(),
            });
        }
    }

    ret
}

pub fn solve_a() {
    let data = fs::read_to_string("./inputs/day12.txt").unwrap();
    let queries = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            let s = line.split(" ").collect_vec();
            Query {
                map: s[0].chars().collect_vec(),
                blocks: s[1]
                    .split(",")
                    .map(|number| number.parse::<i64>().unwrap())
                    .collect_vec(),
            }
        })
        .collect_vec();

    // let ret = queries
    //     .iter()
    //     .map(|q| (q.map.iter().collect::<String>(), &q.blocks, solve(q)))
    //     .collect_vec();
    let ret = queries.iter().map(|q| solve(q.clone())).sum::<i64>();
    dbg!(ret);
}

pub fn solve_b() {
    let data = fs::read_to_string("./inputs/day12.txt").unwrap();
    let queries = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            let s = line.split(" ").collect_vec();
            let q = Query {
                map: s[0].chars().collect_vec(),
                blocks: s[1]
                    .split(",")
                    .map(|number| number.parse::<i64>().unwrap())
                    .collect_vec(),
            };

            let mut new_map = vec![];
            new_map.append(&mut q.map.clone());
            for _ in 0..4 {
                new_map.push('?');
                new_map.append(&mut q.map.clone());
            }

            Query {
                map: new_map,
                blocks: q
                    .blocks
                    .iter()
                    .cycle()
                    .take(q.blocks.len() * 5)
                    .map(|y| *y)
                    .collect_vec(),
            }
        })
        .collect_vec();

    // let ret = queries
    //     .iter()
    //     .map(|q| (q.map.iter().collect::<String>(), &q.blocks, solve(q)))
    //     .collect_vec();

    // let ret = queries.iter().map(|q| solve(dbg!(q))).sum::<i64>();
    // dbg!(ret);

    let mut ret = 0;
    for (i, q) in queries.iter().enumerate() {
        println!(
            "{}/{}: {}, {:?}",
            i,
            queries.len(),
            q.map.iter().collect::<String>(),
            q.blocks
        );
        let result = solve(q.clone());
        dbg!(result);
        ret += result;
    }
    dbg!(ret);
    // TODO: add DP caching?
}
