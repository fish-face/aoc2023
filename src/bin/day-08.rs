use std::collections::{HashMap, HashSet};
use std::ops::Rem;
use bit_set::BitSet;
use itertools::Itertools;
use modinverse::egcd;
use aoc2023::common::read_input_lines;
use num::integer::lcm;

fn instr(c: char) -> bool {
    match c {
        'L' => false,
        'R' => true,
        _ => panic!("Bad instruction {c}"),
    }
}

fn nodeline(s: &String) -> (&str, (&str, &str)) {
    (&s[0..3], (&s[7..10], &s[12..15]))
}

fn follow_path(nodes: &Vec<(usize, usize)>, targets: &BitSet<u16>, instructions: &Vec<bool>, start: usize, is_part1: bool) -> usize {
    let mut node = start;
    for (i, instruction) in instructions.iter().cycle().enumerate() {
        node = if *instruction {
            nodes[node].1
        } else {
            nodes[node].0
        };
        if targets.contains(node) {
            if is_part1 {
                println!("{}", i + 1);
            }
            return i + 1;
        }
    }
    0
}

fn main () {
    let mut lines = read_input_lines().expect("Could not read input file");

    let instructions = lines.next().unwrap();
    let instructions = instructions.chars().map(instr).collect::<Vec<_>>();

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
    let targets = BitSet::<u16>::from_iter(names_to_idx.iter().filter_map(
        |(name, idx)| match name.as_bytes()[2] as char {
            'Z' => Some(*idx),
            _ => None,
        }
    ));
    let mut paths = names_to_idx.iter().filter_map(
        |(name, idx)| match name.as_bytes()[2] as char {
            'A' => Some(*idx),
            _ => None,
        }
    );

    let so_called_cycle_lengths = paths.map(|p|
        follow_path(&nodes, &targets, &instructions, p, p == part1_start)
    );

    let lcm = so_called_cycle_lengths.clone().reduce(lcm).unwrap();
    println!("{}", lcm);
}