use std::iter::zip;
use itertools::Itertools;
use aoc2023::common::{read_input_lines, strs_to_nums};

#[inline]
fn n_winning_times(time: u64, best_distance: u64) -> i64 {
    let discriminant = ((time * time - 4 * best_distance) as f64).sqrt();
    // .floor + 1 because
    let left = (0.5 * (time as f64 - discriminant)).floor() as i64 + 1;
    let right = (0.5 * (time as f64 + discriminant)).ceil() as i64 - 1;
    right - left + 1
}

fn main() {
    let lines = read_input_lines()
        .expect("Could not read input")
        .collect::<Vec<_>>();

    let times = strs_to_nums(lines[0]
        .split_ascii_whitespace()
        .skip(1));
    let distances = strs_to_nums(lines[1]
        .split_ascii_whitespace()
        .skip(1));

    let part1: i64 = zip(times, distances)
        .map(|(t, d)| n_winning_times(t, d))
        .product();
    println!("{}", part1);

    let t2 = lines[0]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<u64>()
        .unwrap();
    let d2 = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<u64>()
        .unwrap();
    println!("{}", n_winning_times(t2, d2));
}
