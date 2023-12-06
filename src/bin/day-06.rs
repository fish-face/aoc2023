use std::iter::zip;
use itertools::Itertools;
use aoc2023::common::{read_input_lines, strs_to_nums};

#[inline]
fn n_winning_times(time: f64, best_distance: f64) -> i64 {
    let discriminant = (time * time - 4.0 * best_distance).sqrt();
    let left = (0.5 * (time - discriminant)).floor() + 1.0;
    let right = (0.5 * (time + discriminant)).ceil() - 1.0;
    (right - left + 1.0) as i64
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
        .parse::<f64>()
        .unwrap();
    let d2 = lines[1]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<f64>()
        .unwrap();

    println!("{}", n_winning_times(t2, d2));
}