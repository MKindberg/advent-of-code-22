use std::{fs, time::Instant};

fn dist(sensor: (i64, i64), beacon: (i64, i64)) -> i64 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

fn beacons_in_row(row: i64, sensors: &Vec<((i64, i64), i64)>, ranges: &mut Vec<(i64, i64)>) {
    ranges.clear();
    for (sensor, d) in sensors {
        let width = d - (sensor.1 - row).abs();
        if width >= 0 {
            ranges.push((sensor.0 - width, sensor.0 + width));
        }
    }
    ranges.sort();
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read input file");

    let now = Instant::now();
    let mut sensors: Vec<((i64, i64), i64)> = Vec::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let s_x = words.nth(2).unwrap();
        let s_y = words.next().unwrap();
        let sensor = (
            s_x[2..s_x.len() - 1].parse().unwrap(),
            s_y[2..s_y.len() - 1].parse().unwrap(),
        );

        let b_x = words.nth(4).unwrap();
        let b_y = words.next().unwrap();
        let beacon = (
            b_x[2..b_x.len() - 1].parse().unwrap(),
            b_y[2..].parse().unwrap(),
        );
        sensors.push((sensor, dist(sensor, beacon)));
    }
    let mut ranges: Vec<(i64, i64)> = vec![];
    let setup_time = now.elapsed();

    beacons_in_row(2_000_000, &sensors, &mut ranges);
    let mut no_beacon = 0;
    let mut end = ranges[0].0;
    for r in ranges.iter() {
        if r.1 <= end {
            continue;
        } else if r.0 >= end {
            no_beacon += r.1 - r.0;
        } else if r.0 < end {
            no_beacon += r.1 - end;
        }
        end = r.1;
    }
    println!("Part 1: {}", no_beacon);
    let part1_time = now.elapsed();

    let max: i64 = 4_000_000;
    for row in 0..=max {
        beacons_in_row(row, &sensors, &mut ranges);
        let mut col = 0;
        for r in &ranges {
            if col >= r.0 && col <= r.1 {
                col = r.1 + 1;
            }
        }
        if col <= max + 1 {
            println!("Part 2: {}", col * max + row);
            break;
        }
    }

    let part2_time = now.elapsed();

    println!("Setup time: {}µs", setup_time.as_micros());
    println!("Part 1 time: {}µs", part1_time.as_micros());
    println!("Part 2 time: {}ms", part2_time.as_millis());
}
