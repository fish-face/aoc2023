use std::iter;
use bit_set::BitSet;
use pest_typed::ParsableTypedNode as _;
use pest_typed_derive::TypedParser;
use aoc2023::common::read_input_lines;

#[derive(TypedParser)]
#[grammar = "day-04/cards.pest"]
#[emit_rule_reference]
pub struct GameParser;

fn single_iterator<T>(both: (T, Vec<T>)) -> impl Iterator<Item = T> {
    let (first, rest) = both;
    iter::once(first).chain(rest)
}

fn num_matches(game: &pairs::game) -> usize {
    let game_numbers = single_iterator(game.game_number())
        .map(|n| n.span.as_str().parse::<usize>().unwrap());
    let have_numbers = single_iterator(game.have_number())
        .map(|n| n.span.as_str().parse::<usize>().unwrap());

    let mut game_numbers: BitSet<usize> = BitSet::from_iter(game_numbers);
    let have_numbers = BitSet::from_iter(have_numbers);

    game_numbers.intersect_with(&have_numbers);
    game_numbers.len()
}

fn score(num_matches: usize) -> usize {
    if num_matches == 0 {
        0
    } else {
        2_usize.pow((num_matches - 1) as u32)
    }
}

fn main() {
    let lines = read_input_lines().expect("Could not read input").collect::<Vec<_>>();
    let games = lines.iter().map(|line| pairs::game::parse(line).unwrap());
    let num = games.len();

    let matches = games.map(|game| num_matches(&game)).collect::<Vec<_>>();
    let part1 = matches.iter().map(|matches| score(*matches)).sum::<usize>();
    println!("{}", part1);

    let mut copies = vec![1; num];
    for (i, matches) in matches.iter().enumerate() {
        for ii in i+1..=i+matches {
            copies[ii] += copies[i];
        }
    }
    println!("{}", copies.into_iter().sum::<u64>());
}