use std::{collections::HashMap, fs, time::Instant};

#[derive(Debug)]
struct Room {
    connections: Vec<usize>,
    flow: usize,
}

fn walk(
    rooms: &Vec<Room>,
    active_flow: usize,
    minutes_left: usize,
    activated: usize,
    current: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if minutes_left == 0 {
        return 0;
    }
    if let Some(&x) = cache.get(&(current, activated, minutes_left)) {
        return x;
    }
    let mut max_flow = 0;
    for &c in &rooms[current].connections {
        max_flow = max_flow.max(walk(
            rooms,
            active_flow,
            minutes_left - 1,
            activated,
            c,
            cache,
        ));
    }
    if rooms[current].flow != 0 && activated & (1 << current) == 0 {
        max_flow = max_flow.max(walk(
            rooms,
            active_flow + rooms[current].flow,
            minutes_left - 1,
            activated | (1 << current),
            current,
            cache,
        ));
    }
    max_flow += active_flow;
    cache.insert((current, activated, minutes_left), max_flow);
    max_flow
}

fn walk2(
    rooms: &Vec<Room>,
    active_flow: usize,
    minutes_left: usize,
    activated: usize,
    current: (usize, usize),
    cache: &mut HashMap<((usize, usize), usize, usize), usize>,
) -> usize {
    if minutes_left == 0 {
        return 0;
    }
    if let Some(&x) = cache.get(&(current, activated, minutes_left)) {
        return x;
    }
    if let Some(&x) = cache.get(&((current.1, current.0), activated, minutes_left)) {
        return x;
    }
    let mut max_flow = 0;
    for &c0 in &rooms[current.0].connections {
        for &c1 in &rooms[current.1].connections {
            max_flow = max_flow.max(walk2(
                rooms,
                active_flow,
                minutes_left - 1,
                activated,
                (c0, c1),
                cache,
            ));
            if rooms[current.0].flow != 0 && activated & (1 << current.0) == 0 {
                max_flow = max_flow.max(walk2(
                    rooms,
                    active_flow + rooms[current.0].flow,
                    minutes_left - 1,
                    activated | (1 << current.0),
                    (current.0, c1),
                    cache,
                ));
            }
            if (rooms[current.1].flow != 0 && activated & (1 << current.1) == 0) {
                max_flow = max_flow.max(walk2(
                    rooms,
                    active_flow + rooms[current.1].flow,
                    minutes_left - 1,
                    activated | (1 << current.1),
                    (c0, current.1),
                    cache,
                ));
            }
            if current.0 != current.1
                && (rooms[current.0].flow != 0 && activated & (1 << current.0) == 0)
                && (rooms[current.1].flow != 0 && activated & (1 << current.1) == 0)
            {
                max_flow = max_flow.max(walk2(
                    rooms,
                    active_flow + rooms[current.0].flow + rooms[current.1].flow,
                    minutes_left - 1,
                    activated | (1 << current.0) | (1 << current.1),
                    (current.0, current.1),
                    cache,
                ));
            }
        }
    }
    max_flow += active_flow;
    cache.insert((current, activated, minutes_left), max_flow);
    max_flow
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");
    // let input = fs::read_to_string("input2").expect("Failed to read input file");

    let mut str_to_int: HashMap<&str, usize> = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        let mut w = l.split_whitespace();
        let room = w.nth(1).unwrap();
        str_to_int.insert(room, i);
    }

    let mut rooms: Vec<Room> = Vec::new();
    for l in input.lines() {
        let mut w = l.split_whitespace();

        let (_, flow) = w.nth(4).unwrap().split_once('=').unwrap();
        let flow: usize = flow.trim_end_matches(';').parse().unwrap();

        let connections: Vec<usize> = w
            .skip(4)
            .map(|x| x.trim_end_matches(','))
            .map(|x| str_to_int[x])
            .collect();

        rooms.push(Room { connections, flow });
    }

    let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let timer = Instant::now();
    let max_preassure = walk(&rooms, 0, 30, 0, str_to_int["AA"], &mut cache);
    println!(
        "Part 1: {} in {}ms",
        max_preassure,
        timer.elapsed().as_millis()
    );

    let mut cache: HashMap<((usize, usize), usize, usize), usize> = HashMap::new();
    let timer = Instant::now();
    let max_preassure = walk2(
        &rooms,
        0,
        26,
        0,
        (str_to_int["AA"], str_to_int["AA"]),
        &mut cache,
    );
    println!(
        "Part 2: {} in {}ms",
        max_preassure,
        timer.elapsed().as_millis()
    );
}
