use std::any::{Any};
use std::array;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug};
use array_macro::array;
use aoc2023::common::read_input_lines;

type Idx = usize;

fn idx(s: &str) -> Idx {
    if s == "broadcaster" {
        0
    } else {
        let s = s.as_bytes();
        (s[0] - b'a' + 1) as usize * 27 + (s[1] - b'a' + 1) as usize
    }
}

#[derive(Debug)]
struct Module {
    // (index of destination, index in destination's input vec)
    dests: Vec<(Idx, usize)>,
    imp: Box<dyn Moduley>,
}

impl Module {
    fn parse(s: &str, out_to_in: &mut [Vec<Idx>]) -> (Idx, Self) {
        let (module, dests) = s.split_once(" -> ").unwrap();
        let (label, imp): (_, Box<dyn Moduley>) = match module.as_bytes()[0] {
            b'%' => (&module[1..], Box::new(FlipFlop{on: false})),
            b'&' => (&module[1..], Box::new(Nand{memory: vec![]})),
            _ => (module, Box::new(Broadcast{})),
        };
        let dests = dests.split(", ").map(
            |s| {
                let id = idx(s);
                let outs = &mut out_to_in[id];
                outs.push(idx(label));
                (id, outs.len() - 1)
            }
        ).collect();
        (idx(label), Module{dests, imp})
    }
}

#[derive(Debug)]
struct Broadcast {}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
}

#[derive(Debug)]
struct Nand {
    memory: Vec<bool>,
}

trait Moduley: Any + Debug {
    fn eval(&mut self, input_idx: usize, input: bool) -> Option<bool>;
    // for some reason you can't blanket implement this on the trait, and numerous other attempts
    // at getting the concrete type don't work
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Moduley for Broadcast {
    fn eval(&mut self, _: usize, input: bool) -> Option<bool> {
        Some(input)
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

impl Moduley for FlipFlop {
    fn eval(&mut self, _: usize, input: bool) -> Option<bool> {
        if input {
            None
        } else {
            self.on = !self.on;
            Some(self.on)
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

impl Moduley for Nand {
    fn eval(&mut self, input_idx: usize, input: bool) -> Option<bool> {
        self.memory[input_idx] = input;
        Some(!self.memory.iter().all(|x| *x))
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

fn part1(modules: &mut [Option<Module>], pulses: &mut VecDeque<(Idx, usize, bool)>, highs: &mut i32, lows: &mut i32) {
    pulses.push_back((idx(&"broadcaster"), 0, false));
    while let Some((idx, input_idx, input)) = pulses.pop_front() {
        if input {
            *highs += 1;
        } else {
            *lows += 1;
        }
        simulate(modules, pulses, idx, input_idx, input);
    }
}

fn part2(modules: &mut [Option<Module>], pulses: &mut VecDeque<(Idx, usize, bool)>, i: usize, goals: &mut HashMap::<Idx, Option<usize>>) {
    pulses.push_back((idx(&"broadcaster"), 0, false));
    while let Some((idx, input_idx, input)) = pulses.pop_front() {
        if let Some(output) = simulate(modules, pulses, idx, input_idx, input) {
            if output {
                goals.entry(idx).and_modify(|period| {period.get_or_insert(i);});
            }
        }
    }
}

fn simulate(modules: &mut [Option<Module>], pulses: &mut VecDeque<(Idx, usize, bool)>, idx: Idx, input_idx: usize, input: bool) -> Option<bool> {
    if let Some(module) = &mut modules[idx] {
        if let Some(result) = module.imp.eval(input_idx, input) {
            for (dest_idx, dest_input_idx) in module.dests.iter() {
                pulses.push_back((dest_idx.clone(), *dest_input_idx, result));
            }
            return Some(result)
        }
    }
    None
}

fn main() {
    let mut modules: [Option<Module>; 27*27] = array::from_fn(|_| None);
    let mut out_to_in: [_; 27*27] = array::from_fn(|_| Vec::<Idx>::new());
    for line in read_input_lines().unwrap() {
        let (idx, module) = Module::parse(&line, &mut out_to_in);
        modules[idx] = Some(module);
    }

    // set up the Nands' memories
    for (idx, module) in modules.iter_mut().enumerate() {
        if let Some(module) = module {
            if let Some(imp) = module.imp.as_any_mut().downcast_mut::<Nand>() {
                imp.memory.resize(out_to_in[idx].len(), false);
            }
        }
    }

    // find input to "rx"
    let rx_input = &out_to_in[idx("rx")];
    assert_eq!(rx_input.len(), 1);
    // find inputs to that. we assume these all go high periodically for one iteration and then go
    // low again, and that the length of this period is prime.
    let mut rx_input_inputs = HashMap::<Idx, Option<usize>>::from_iter(
        out_to_in[rx_input[0]].iter().map(|idx| (idx.clone(), None))
    );

    let mut pulses = VecDeque::<(Idx, usize, bool)>::new();
    let mut highs = 0;
    let mut lows = 0;
    for _ in 0..1000 {
        part1(&mut modules, &mut pulses, &mut highs, &mut lows);
    }
    println!("{}", highs * lows);

    // find the periods for part2. assume they're all between 1000 and 6000 (mine were about 4000)
    for i in 0..5000 {
        part2(&mut modules, &mut pulses, 1000 + i, &mut rx_input_inputs);
        if rx_input_inputs.values().all(|v| v.is_some()) {
            // magic off by one who cares why at this point tbh
            // since we're assuming all these inputs are periodic, the desired final input occurs at
            // their LCM. Since we're assuming the periods are primes, that's their product
            println!("{}", rx_input_inputs.values().map(|v| v.unwrap() + 1).product::<usize>());
            break;
        }
    }
}
