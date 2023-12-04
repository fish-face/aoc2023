use std::collections::HashMap;
use std::iter::Iterator;
use aoc2023::common::{read_input_lines};
use aoc2023::coord::Pt;
use aoc2023::grid::Grid;

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

#[derive(Debug)]
struct Span {
    left: usize,
    len: usize,
    y: usize,
}

impl Span {
    fn iter(&self) -> impl Iterator<Item=Pt<usize>> + '_ {
        (self.left..self.left + self.len).map(|x| Pt(x, self.y))
    }

    fn right_limit(&self) -> usize {
        self.left + self.len - 1
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.y == other.y && self.left < other.right_limit() && other.left <= self.right_limit()
    }

    fn map_from_grid<'a, T>(&'a self, grid: &'a Grid<T>) -> impl Iterator<Item=&'a T> {
        self.iter().map(|p| &grid[p])
    }
}

fn fill_digits(start: &Pt<usize>, grid: &Grid<u8>) -> Span {
    let Pt(x, y) = start;
    let mut span = Span { left: *x, len: 1, y: *y };
    // is there a nicer way of doing these loops?!
    // extend left as far as possible
    loop {
        if span.left > 0 && (grid[Pt(span.left - 1, span.y)] as char).is_ascii_digit() {
            span.left -= 1;
            span.len += 1;
        } else {
            break;
        }
    }
    // extend right as far as possible
    loop {
        if span.left + span.len < grid.width && (grid[Pt(span.left + span.len, span.y)] as char).is_ascii_digit() {
            span.len += 1;
        } else {
            break;
        }
    }
    span
}

fn connected_to<'a>(starts: impl Iterator<Item=&'a Pt<usize>>, grid: &Grid<u8>) -> HashMap<&'a Pt<usize>, Vec<Span>> {
    let mut result = HashMap::new();
    for start in starts {
        let mut spans: Vec<Span> = Vec::new();
        for neighbour in start.neighbours8() {
            if !grid.contains(neighbour) || !grid[neighbour].is_ascii_digit() { continue; }

            let span = fill_digits(&neighbour, grid);
            if spans.is_empty() || !spans.last().unwrap().overlaps(&span) {
                spans.push(span);
            }
        }
        result.insert(start, spans);
    }
    result
}

fn main() {
    let data = read_input_lines().expect("Could not read input file");
    let grid = Grid::from_row_data(data.map(|line| line.into_bytes()));
    let symbols: Vec<_> = grid.enumerate().filter_map(
        |(p, c)| if is_symbol(*c as char) {
            Some(p)
        } else {
            None
        }
    ).collect();

    let symbols_to_spans = connected_to(symbols.iter(), &grid);
    println!("{}", symbols_to_spans.iter()
        // [span span...]
        .map(|(_, value)| value.iter()
            // [pt pt ...] -> [value value value ...]
            .map(|span| span.map_from_grid(&grid).map(|c| *c as char)
                // strings of digits
                .collect::<String>()
                // parse into int
                .parse::<u64>()
                .unwrap())
            // sum numbers for one starting point
            .sum::<u64>()
        )
        // sum everything
        .sum::<u64>()
    );

    let asterisks: Vec<_> = grid.enumerate().filter_map(
        |(p, c)| if *c as char == '*' { Some(p) } else { None }
    ).collect();
    let gears_to_spans = connected_to(asterisks.iter(), &grid);
    println!("{}", gears_to_spans.iter()
        .filter(|(_, value)| value.len() == 2)
        // as above
        .map(|(_, value)| value.iter()
            .map(|span| span
                .map_from_grid(&grid)
                .map(|c| *c as char)
                .collect::<String>()
                .parse::<u64>()
                .unwrap())
            // product of numbers around starting point
            .product::<u64>()
        )
        // sum of all products
        .sum::<u64>()
    );
}