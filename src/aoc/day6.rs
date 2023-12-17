struct Race {
    time: i64,
    distance: i64,
}

fn ways_to_win(race: &Race) -> i64 {
    let mut ret = 0;
    for hold in 0..race.time {
        if hold * (race.time - hold) > race.distance {
            ret += 1;
        }
    }

    ret
}

pub fn solve_a() {
    // let races = vec![
    //     Race {
    //         time: 7,
    //         distance: 9,
    //     },
    //     Race {
    //         time: 15,
    //         distance: 40,
    //     },
    //     Race {
    //         time: 30,
    //         distance: 200,
    //     },
    // ]; // test data
    let races = vec![
        Race {
            time: 58,
            distance: 434,
        },
        Race {
            time: 81,
            distance: 1041,
        },
        Race {
            time: 96,
            distance: 2219,
        },
        Race {
            time: 76,
            distance: 1218,
        },
    ];

    let ret = races
        .iter()
        .map(|race| ways_to_win(race))
        .reduce(|acc, next| acc * next)
        .unwrap();

    dbg!(ret);
}

pub fn solve_b() {
    // let race = Race{time: 71530, distance: 940200};
    let race = Race {
        time: 58819676,
        distance: 434104122191218,
    };

    let ret = ways_to_win(&race);

    dbg!(ret);
}
