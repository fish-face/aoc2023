use std::collections::{HashMap, HashSet};
use std::iter::zip;
use std::ops::Rem;
use itertools::{Itertools, izip};
use aoc2023::common::read_input_lines;

fn instr(c: char) -> usize {
    match c {
        'L' => 0,
        'R' => 1,
        _ => panic!("Bad instruction {c}"),
    }
}

fn nodeline(s: &String) -> (&str, (&str, &str)) {
    let res = (&s[0..3], (&s[7..10], &s[12..15]));
    // println!("{:?}", res);
    res
}

#[derive(Debug)]
struct PathRecord {
    node: usize,
    start: usize,
    // Apparently each start node gets into a loop containing a target node, disjoint from the other loops
    // hit_target_at: Vec<usize>,
    hit_target_at: usize,
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

    // println!("{}", names_to_idx.len());
    // println!("{}", names_to_idx["ZZZ"]);
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut node = names_to_idx["AAA"];
    for (i, instruction) in instructions.clone().cycle().enumerate() {
        if visited.contains(&(i.rem(instr_count), node)) {
            panic!("Infinite loop at {i}, {node}");
        }
        visited.insert((i, node));
        if i > 1_000_000 {
            panic!("Ran out at {i}");
        }
        // if i.rem(instr_count) < 10 || i.rem(instr_count) > instr_count - 10 {
        // if i > 13250 || i < 13350 {
        //     println!("{i}: {}={} {node}={} --> {:?}", i.rem(instr_count), instruction, &nodes_lines[node][0..3], nodes[node])
        // }
        node = match instruction {
            0 => nodes[node].0,
            1 => nodes[node].1,
            _ => {panic!();},
        };
        if node == names_to_idx["ZZZ"] {
            println!("{}", i+1);
            break;
        }
    }

    let targets = names_to_idx.iter().filter_map(
        |(name, idx)| match name.as_bytes()[2] as char {
            'Z' => Some(idx),
            _ => None,
        }
    ).collect::<Vec<_>>();
    let mut current = names_to_idx.iter().filter_map(
        |(name, idx)| match name.as_bytes()[2] as char {
            'A' => Some(*idx),
            _ => None,
        }
    ).collect::<Vec<_>>();
    let starts = current.clone();
    // let mut cycle_lengths = vec![0; current.len()];
    // let mut hit_target_at = vec![(0, 0); current.len()];
    let mut paths = current.into_iter().map(
        |start| PathRecord{node: start, start, hit_target_at: 0, cycle_length: 0 }
    ).collect::<Vec<_>>();

    for (i, instruction) in instructions.cycle().enumerate() {
        // if visited.contains(&(i.rem(instr_count), node)) {
        //     panic!("Infinite loop at {i}, {node}");
        // }
        // visited.insert((i, node));
        if i > 100_000 {
            panic!("Ran out at {i}");
        }
        // if i.rem(instr_count) < 10 || i.rem(instr_count) > instr_count - 10 {
        // if i > 13250 || i < 13350 {
        //     println!("{i}: {}={} {node}={} --> {:?}", i.rem(instr_count), instruction, &nodes_lines[node][0..3], nodes[node])
        // }

        for pathrecord in paths.iter_mut() {
            pathrecord.node = match instruction {
                0 => nodes[pathrecord.node].0,
                1 => nodes[pathrecord.node].1,
                _ => {panic!();},
            };
            if targets.iter().any(|x| **x == pathrecord.node) {
                // println!("Starting node {} hit {} on {}/{}", pathrecord.start, pathrecord.node, i, i.rem(instr_count));
                if pathrecord.hit_target_at != 0 && pathrecord.cycle_length == 0 {
                    pathrecord.cycle_length = i - pathrecord.hit_target_at;
                } else {
                    pathrecord.hit_target_at = i;
                }
            }
        }
        // TODO optimise
        if paths.iter().all(|pr| pr.cycle_length > 0) {
            // println!("{:?}", paths);
            // println!("{:?}", paths.iter().map(|pr| pr.cycle_length).product::<usize>());
            let largest_cycle_length = paths.iter().map(|pr| pr.cycle_length).max().unwrap();
            let mut ii = i;
            loop {
                if paths.iter().all(|pr| (ii - pr.hit_target_at).rem(pr.cycle_length) == 0) {
                    println!("{ii}");
                    break;
                }
                ii += largest_cycle_length;
            }
            break;
        }

        // if current.iter().all(|node| targets.contains(&node)) {
        //     println!("{}", i+1);
        //     break;
        // }
    }
}