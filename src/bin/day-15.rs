use std::collections::HashMap;
use itertools::Itertools;
use aoc2023::common::read_input;

#[derive(Debug)]
struct Box {
    num: usize,
    lenses: HashMap<Vec<u8>, usize>,
    lens_power: Vec<Option<usize>>,
}

impl Box {
    fn insert(&mut self, label: &[u8], power: usize) {
        if self.lenses.contains_key(label) {
            self.lens_power[self.lenses[label]] = Some(power);
        } else {
            self.lenses.insert(label.to_owned(), self.lens_power.len());
            self.lens_power.push(Some(power));
        }
    }

    fn remove(&mut self, label: &[u8]) {
        self.lenses.remove(label).and_then(|i| Some(self.lens_power[i] = None));
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction.operation {
            Op::Ins => self.insert(instruction.label, instruction.value.unwrap()),
            Op::Rem => self.remove(instruction.label),
        }
    }

    fn power(&self) -> usize {
        self.lens_power.iter().filter(|x| x.is_some()).enumerate().map(
            |(i, power)| (self.num + 1) * (i + 1) * power.unwrap()
        ).sum::<usize>()
    }
}

#[derive(Debug)]
enum Op { Ins, Rem }

#[derive(Debug)]
struct Instruction<'a> {
    label: &'a [u8],
    operation: Op,
    value: Option<usize>,
}

fn parse_int(c: u8) -> usize {
    (c - b'0') as usize
}

impl<'a> Instruction<'a> {
    fn from_str(s: &'a [u8]) -> Self {
        let (i_op, op) = s.iter().find_position(|c| **c == b'=' || **c == b'-').unwrap();
        let label = &s[0..i_op];
        let operation = match op {
            b'=' => Op::Ins,
            b'-' => Op::Rem,
            _ => panic!("nope"),
        };
        let value = match operation {
            Op::Ins => Some(parse_int(s[i_op+1])),
            Op::Rem => None,
        };
        Instruction{label, operation, value}
    }
}

fn hash_one(acc: &mut u8, char: u8) {
    (*acc, _) = acc.overflowing_add(char);
    (*acc, _) = acc.overflowing_mul(17);
}

fn hash(s: &[u8]) -> usize {
    let mut h = 0;
    s.iter().for_each(|c| hash_one(&mut h, *c));
    h as usize
}

fn main() {
    let input = read_input().unwrap();
    let input = input.trim().as_bytes();
    let part1 = input.split(|b| *b == b',').map(hash).sum::<usize>();
    println!("{:?}", part1);

    let mut boxes = vec![];
    for num in 0..256 {
        boxes.push(Box{num, lenses: HashMap::new(), lens_power: vec![]});
    }

    for instruction in input.split(|b| *b == b',').map(
        |instruction| Instruction::from_str(instruction)
    ) {
        let bx = hash(instruction.label);
        boxes[bx].execute(&instruction);
    }
    let power = boxes.iter().map(|b| b.power()).sum::<usize>();
    println!("{:?}", power);
}