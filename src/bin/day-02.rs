use std::cmp::max;
use std::iter;
use pest_typed::ParsableTypedNode as _;
use pest_typed_derive::TypedParser;
use num_enum::IntoPrimitive;

use aoc2023::common::read_input_lines;

#[derive(TypedParser)]
#[grammar = "day-02.pest"]
#[emit_rule_reference]
pub struct GameParser;

#[derive(IntoPrimitive)]
#[repr(u8)]
enum Colours {
    RED, GREEN, BLUE
}

const CONTENTS: [usize; 3] = [12, 13, 14];

fn parse(line: &String) -> (usize, usize) {
    let mut mins = [0_usize, 0_usize, 0_usize];

    let (_, interesting) = line.split_once(' ').unwrap();
    let (game_id, draws) = interesting.split_once(':').unwrap();
    let mut part1_contribution  = game_id.parse::<usize>().expect("Could not parse game ID");

    let draws = draws.split(';');
    for draw in draws {
        let parts = draw.split(',');
        for draw_part in parts {
            let draw_part = &draw_part[1..];
            let (number, colour) = draw_part.split_once(' ').unwrap();
            let number = number.parse::<usize>().expect("Could not parse number");
            let colour = match colour {
                "red" => Colours::RED,
                "green" => Colours::GREEN,
                "blue" => Colours::BLUE,
                _ => { panic!("Invalid colour"); }
            };
            let colour_idx: u8 = colour.into();
            mins[colour_idx as usize] = max(mins[colour_idx as usize], number);
            if number > CONTENTS[colour_idx as usize] {
                part1_contribution = 0;
            }
        }
    }
    (part1_contribution, mins.iter().product())
}

fn main() {
    let lines = read_input_lines().expect("Could not read file");

    let (part1, part2) = lines
        .map(|line| parse(&line))
        .reduce(|(part1a, part2a), (part1b, part2b)| (part1a + part1b, part2a + part2b))
        .unwrap();
    println!("{part1}\n{part2}");
}
