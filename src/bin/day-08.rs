use std::collections::{HashMap, HashSet};
use std::ops::Rem;
use itertools::Itertools;
use modinverse::egcd;
use aoc2023::common::read_input_lines;

fn instr(c: char) -> usize {
    match c {
        'L' => 0,
        'R' => 1,
        _ => panic!("Bad instruction {c}"),
    }
}

fn nodeline(s: &String) -> (&str, (&str, &str)) {
    (&s[0..3], (&s[7..10], &s[12..15]))
}

#[derive(Debug)]
struct PathRecord {
    node: usize,
    start: usize,
    // Apparently each start node gets into a loop containing a target node, disjoint from the other loops
    // hit_target_at: Vec<usize>,
    cycle_start: usize,
    cycle_length: usize,
}

fn main () {
    let mut lines = read_input_lines().expect("Could not read input file");

    let instructions = lines.next().unwrap();
    let instructions = instructions.chars().map(instr);
    let instr_count = instructions.clone().count();

    let nodes_lines = lines.skip(1).collect::<Vec<_>>();
    let nodes = nodes_lines.iter().map(nodeline);
    let names_to_idx = HashMap::<&str, usize>::from_iter(nodes
        .clone()
        .enumerate()
        .map(|(i, (from, _))| (from, i)));
    let nodes = nodes
        .map(|(_, (left, right))| (names_to_idx[left], names_to_idx[right]))
        .collect::<Vec<_>>();

    let part1_start = names_to_idx["AAA"];
    let targets = names_to_idx.iter().filter_map(
        |(name, idx)| match name.as_bytes()[2] as char {
            'Z' => Some(idx),
            _ => None,
        }
    ).collect::<Vec<_>>();
    let mut paths = names_to_idx.iter().filter_map(
        |(name, idx)| match name.as_bytes()[2] as char {
            'A' => Some(PathRecord{node: *idx, start: *idx, cycle_start: 0, cycle_length: 0}),
            _ => None,
        }
    ).collect::<Vec<_>>();

    for (i, instruction) in instructions.cycle().enumerate() {
        for pathrecord in paths.iter_mut() {
            if pathrecord.cycle_length > 0 {
                continue;
            }
            pathrecord.node = match instruction {
                0 => nodes[pathrecord.node].0,
                1 => nodes[pathrecord.node].1,
                _ => {panic!();},
            };
            if targets.iter().any(|x| **x == pathrecord.node) {
                if pathrecord.cycle_start != 0 && pathrecord.cycle_length == 0 {
                    pathrecord.cycle_length = i - pathrecord.cycle_start;
                } else {
                    pathrecord.cycle_start = i;
                    if pathrecord.start == part1_start {
                        // part1
                        println!("{}", i+1);
                    }
                }
            }
        }
        // Doesn't seem to slow things down to check this every iteration
        if paths.iter().all(|pr| pr.cycle_length > 0) {
            // Apply Chinese Remainder Theorem:
            // Solving x ≡ cycle_start mod cycle_length. The cycle lengths are not coprime but
            // we know there is a solution. To make use of the CRT we have to divide by the HCF at
            // each step
            let mut acc_offset = paths[0].cycle_start;
            let mut acc_cycle = paths[0].cycle_length;
            for pr in paths.iter().skip(1) {
                let (h, a, b) = egcd(acc_cycle, pr.cycle_length);
                // solve this pair of congruences to get:
                // z ≡ a*l1*h1 + b*l2*h2 mod l1*l2

                acc_offset = (a * acc_cycle * acc_offset + b * pr.cycle_length * pr.cycle_start)
                    .rem(acc_cycle * pr.cycle_length / h);
                acc_cycle = acc_cycle * pr.cycle_length / h;
            }
            println!("{}", acc_cycle);
            break;
        }
    }
}