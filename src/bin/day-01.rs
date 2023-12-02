use aoc2023::common::read_input_lines;

fn firstdigit(line: &str) -> char {
    line.chars().find(|&c| c.is_digit(10)).expect("No digit found")
}

fn lastdigit(line: &str) -> char {
    line.chars().rev().find(|&c| c.is_digit(10)).expect("No digit found")
}

fn part1<'a>(lines: impl Iterator<Item = &'a String>) -> usize {
    lines.map(|line| {
        let first = firstdigit(&line);
        let last = lastdigit(&line);
        format!("{first}{last}").parse::<usize>().unwrap()
    }).sum()
}

fn parse_word_digit(line: &str) -> Option<u8> {
    for (n, word) in [
        (0_u8, "zero"),
        (1_u8, "one"),
        (2_u8, "two"),
        (3_u8, "three"),
        (4_u8, "four"),
        (5_u8, "five"),
        (6_u8, "six"),
        (7_u8, "seven"),
        (8_u8, "eight"),
        (9_u8, "nine"),
    ] {
        // ASCII 48 == '0'
        if line.bytes().nth(0).unwrap() == n + 48 || line.starts_with(word) {
            return Some(n);
        }
    }
    None
}

fn find_word_digit<'a>(mut initials: impl Iterator<Item = &'a str>) -> u8 {
    initials.find_map(parse_word_digit).unwrap()
}

fn part2<'a>(lines: impl Iterator<Item = &'a String>) -> usize {
    lines.map(|line| {
        let first = find_word_digit((0..(line.len())).map(
                |i| &line[i..]
        ));
        let last = find_word_digit((0..(line.len())).rev().map(
                |i| &line[i..]
        ));
        first as usize * 10 + last as usize
    }).sum()
}

fn main() {
    let lines: Vec<String> = read_input_lines().expect("Could not read file").collect();

    println!("{}", part1(lines.iter()));
    println!("{}", part2(lines.iter()));
}
