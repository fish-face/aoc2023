use bit_set::BitSet;
use aoc2023::common::read_input_lines;

/*
// slower PEG solution

#[derive(TypedParser)]
#[grammar = "day-04/cards.pest"]
#[emit_rule_reference]
pub struct GameParser;

fn single_iterator<T>(both: (T, Vec<T>)) -> impl Iterator<Item = T> {
    let (first, rest) = both;
    iter::once(first).chain(rest)
}

fn all_numbers<'a, 'b, R: RuleType, T: RuleStruct<'a, R>>(both: (&'b T, Vec<&'b T>)) -> impl Iterator<Item = usize> + 'b
{
    single_iterator(both).map(|n| n.span().as_str().parse::<usize>().unwrap())
}

fn num_matches(game: &pairs::game) -> usize {
    let game_numbers = all_numbers(game.game_number());
    let have_numbers = all_numbers(game.have_number());

    let mut game_numbers: BitSet<usize> = BitSet::from_iter(game_numbers);
    let have_numbers = BitSet::from_iter(have_numbers);

    game_numbers.intersect_with(&have_numbers);
    game_numbers.len()
}
 */

fn num_matches(line: &String) -> Option<usize> {
    let (_, body) = line.split_once(':')?;
    let (card, have) = body.split_once('|')?;
    let card_nums = card.split_ascii_whitespace().map(|num| num.parse::<usize>().unwrap());
    let have_nums = have.split_ascii_whitespace().map(|num| num.parse::<usize>().unwrap());

    let mut card_nums: BitSet<usize> = BitSet::from_iter(card_nums);
    let have_nums = BitSet::from_iter(have_nums);

    card_nums.intersect_with(&have_nums);
    Some(card_nums.len())
}

fn score(num_matches: usize) -> usize {
    if num_matches == 0 {
        0
    } else {
        2_usize.pow((num_matches - 1) as u32)
    }
}

fn main() {
    let lines = read_input_lines().expect("Could not read input");

    let matches = lines.map(|line| num_matches(&line).expect("Could not parse a line")).collect::<Vec<_>>();
    let num = matches.len();

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