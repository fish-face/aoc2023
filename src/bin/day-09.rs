use std::iter;
use aoc2023::common::{read_input_lines, strs_to_nums};
// use itertools::Itertools;

fn diffs(seq: &Vec<isize>) -> Vec<isize> {
    seq.iter().zip(seq.iter().skip(1)).map(|(a, b)| b - a).collect::<Vec<_>>()
}

fn is_constant(seq: &Vec<isize>) -> bool {
    seq.iter().skip(1).all(|x| *x == seq[0])
}

fn main() {
    let sequences = read_input_lines().expect("Couldn't read input file");

    let predictions = sequences.map(|seq| {
        let mut nums = strs_to_nums(seq.split_ascii_whitespace()).collect::<Vec<isize>>();
        let mut firsts = vec![nums[0]];
        let mut lasts = vec![*nums.last().unwrap()];
        loop {
            nums = diffs(&nums);
            if is_constant(&nums) {
                // println!("{:?}", firsts);
                return (
                    firsts.into_iter().chain(iter::once(nums[0])).rev().reduce(|b, a| a - b).unwrap(),
                    lasts.into_iter().sum::<isize>() + nums.last().unwrap()
                );
            } else {
                firsts.push(nums[0]);
                lasts.push(*nums.last().unwrap());
            }
        }
    });
    let sums = predictions.reduce(|(x1, y1), (x2, y2)| (x1 + x2, y1 + y2)).unwrap();
    println!("{}\n{}", sums.1, sums.0);
}