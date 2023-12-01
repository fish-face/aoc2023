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

fn parse_word_digit(line: &str) -> Option<char> {
    for (ch, word) in [
        ('0', "zero"),
        ('1', "one"),
        ('2', "two"),
        ('3', "three"),
        ('4', "four"),
        ('5', "five"),
        ('6', "six"),
        ('7', "seven"),
        ('8', "eight"),
        ('9', "nine"),
    ] {
        if (line.len() >= word.len() && &line[..word.len()] == word) || line.chars().nth(0).unwrap() == ch {
            return Some(ch);
        }
    }
    None
}

fn find_word_digit<'a>(mut initials: impl Iterator<Item = &'a str>) -> char {
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
        format!("{first}{last}").parse::<usize>().unwrap()
    }).sum()
}

fn main() {
    let lines: Vec<String> = read_input_lines().expect("Could not read file").collect();

    println!("{}", part1(lines.iter()));
    println!("{}", part2(lines.iter()));
}
