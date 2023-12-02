use std::cmp::max;
use std::iter;
use pest_typed::{error::Error, ParsableTypedNode as _};
use pest_typed_derive::TypedParser;

use aoc2023::common::read_input_lines;

#[derive(TypedParser)]
#[grammar = "day-02.pest"]
#[emit_rule_reference]
pub struct GameParser;

const CONTENTS: [usize; 3] = [12, 13, 14];

fn parse(line: &String) -> (usize, usize) {
    let mut mins = [0_usize, 0_usize, 0_usize];

    let game = pairs::game::parse(line).unwrap();
    let game_id = game.game_id().span.as_str().parse::<usize>().expect("Could not parse game ID");
    let mut part1_contribution = game_id;

    let (first_draw, following_draws) = game.draw();
    let draws = iter::once(first_draw).chain(following_draws);
    for draw in draws {
        let (first_part, following_parts) = draw.draw_part();
        let parts = iter::once(first_part).chain(following_parts);
        for draw_part in parts {
            let number = draw_part.number().span.as_str().parse::<usize>().expect("Could not parse number");
            let colour = match draw_part.colour().span.as_str() {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => { panic!("Invalid colour"); }
            };
            mins[colour] = max(mins[colour], number);
            if number > CONTENTS[colour] {
                part1_contribution = 0;
            }
        }
    }
    (part1_contribution, mins.iter().product())
}

fn main() {
    let lines: Vec<String> = read_input_lines().expect("Could not read file").collect();
    // let input = read_input();

    let (part1, part2) = lines.iter().map(
        |line| {
            parse(line)
        }).reduce(|(part1a, part2a), (part1b, part2b)| (part1a + part1b, part2a + part2b)).unwrap();
    println!("{part1}\n{part2}");
}