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
) -> (usize, usize) {
    let mut m_activated = activated;
    if minutes_left == 0 {
        return (0, activated);
    }
    if let Some(&x) = cache.get(&(current, activated, minutes_left)) {
        return (x, activated);
    }
    let mut max_flow = 0;
    for &c in &rooms[current].connections {
        let (f, a) = walk(rooms, active_flow, minutes_left - 1, activated, c, cache);
        if f > max_flow {
            max_flow = f;
            m_activated = a;
        }
    }
    if rooms[current].flow != 0 && activated & (1 << current) == 0 {
        let (f, a) = walk(
            rooms,
            active_flow + rooms[current].flow,
            minutes_left - 1,
            activated | (1 << current),
            current,
            cache,
        );
        if f > max_flow {
            max_flow = f;
            m_activated = a;
        }
    }
    max_flow += active_flow;
    cache.insert((current, activated, minutes_left), max_flow);
    (max_flow, m_activated)
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

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
    let (max_preassure, _) = walk(&rooms, 0, 30, 0, str_to_int["AA"], &mut cache);
    println!(
        "Part 1: {} in {}ms",
        max_preassure,
        timer.elapsed().as_millis()
    );

    let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let timer = Instant::now();
    let (mut max_preassure, activated) = walk(&rooms, 0, 26, 0, str_to_int["AA"], &mut cache);
    max_preassure += walk(&rooms, 0, 26, activated, str_to_int["AA"], &mut cache).0;

    println!(
        "Part 2: {} in {}ms",
        max_preassure,
        timer.elapsed().as_millis()
    );
}
