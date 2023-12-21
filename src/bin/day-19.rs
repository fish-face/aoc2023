use std::cmp::{max, min};
use std::ops::{BitAndAssign, Index, IndexMut, Not};
use array_macro::array;
use aoc2023::common::read_input_lines;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Var { X, M, A, S }

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Comp { LT, GT }

type RuleIndex = usize;

fn idx(s: &str) -> usize {
    let s = s.as_bytes();
    let mut result = 0;
    for c in s.iter().take(3) {
        result *= 26;
        result += (*c - b'a') as usize;
    }
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Target {
    Workflow(RuleIndex),
    Reject,
    Accept,
}

impl Target {
    fn from_str(s: &str) -> Self {
        match s.as_bytes()[0] {
            b'A' => Target::Accept,
            b'R' => Target::Reject,
            _ => Target::Workflow(idx(s)),
        }
    }
}

#[derive(Clone, Debug)]
struct Rule {
    var: Var,
    comp: Comp,
    threshold: u16,
    target: Target,
}

impl Rule {
    fn from_str(rule_str: &str) -> Self {
        let (cond, target) = rule_str.split_once(':').unwrap();
        let var = match cond.as_bytes()[0] {
            b'x' => Var::X,
            b'm' => Var::M,
            b'a' => Var::A,
            b's' => Var::S,
            _ => panic!("nope"),
        };
        let comp = if cond.as_bytes()[1] == b'<' {
            Comp::LT
        } else {
            Comp::GT
        };
        let threshold = cond[2..].parse().expect("invalid u16");
        let target = Target::from_str(target);
        Rule { var, comp, threshold, target }
    }
}

impl Not for &Rule {
    type Output = Rule;

    fn not(self) -> Self::Output {
        // negate the rule
        Rule {
            var: self.var,
            comp: if self.comp == Comp::LT { Comp::GT } else { Comp::LT },
            threshold: if self.comp == Comp::LT { self.threshold - 1 } else { self.threshold + 1 },
            // bleh... it doesn't make sense to "negate" the target, but where we need this, we only
            // care about the other bits anyway.
            target: Target::Accept
        }
    }
}

#[derive(Clone, Debug)]
struct WorkFlow {
    rules: Vec<Rule>,
    default: Target,
}

impl WorkFlow {
    fn eval(&self, p: &Part) -> Target {
        for rule in self.rules.iter() {
            if rule.comp == Comp::LT && p[rule.var] < rule.threshold ||
                rule.comp == Comp::GT && p[rule.var] > rule.threshold {
                return rule.target;
            }
        }
        return self.default.clone();
    }
}

type Part = [u16; 4];

impl Index<Var> for Part {
    type Output = u16;

    fn index(&self, index: Var) -> &Self::Output {
        &self[index as usize]
    }
}

// open intervals (.0, .1) --> the range 1 to 4000 inclusive is represented by (0, 4001)
// 1 for each of the four variables
type Restrictions = [(u16, u16); 4];

impl Index<Var> for Restrictions {
    type Output = (u16, u16);

    fn index(&self, index: Var) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Var> for Restrictions {
    fn index_mut(&mut self, index: Var) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

// why can we implement (arbitrary) traits for a type alias but not arbitrary methods?!
trait Restrictiony {
    fn valid(&self, rule: &Rule) -> bool;
    fn intersection_inplace(&mut self, rule: &Rule);
    fn intersection(&self, rule: &Rule) -> Restrictions;
    fn volume(&self) -> usize;
}

impl Restrictiony for Restrictions {
    fn valid(&self, rule: &Rule) -> bool {
        let (low, high) = self[rule.var];

        if rule.comp == Comp::GT {
            rule.threshold <= high
        } else {
            rule.threshold >= low
        }
    }

    fn intersection_inplace(&mut self, rule: &Rule) {
        if rule.comp == Comp::GT {
            self[rule.var].0 = max(self[rule.var].0, rule.threshold);
        } else {
            self[rule.var].1 = min(self[rule.var].1, rule.threshold);
        }
    }

    fn intersection(&self, rule: &Rule) -> Restrictions {
        let mut result = self.clone();
        if rule.comp == Comp::GT {
            result[rule.var].0 = max(result[rule.var].0, rule.threshold);
        } else {
            result[rule.var].1 = min(result[rule.var].1, rule.threshold);
        }
        result
    }

    fn volume(&self) -> usize {
        // subtract 1 from each length because the intervals are open
        self.iter().map(|(low, high)| (high - low - 1) as usize).product()
    }

}

fn count_accepted(workflows: &[Option<WorkFlow>; 26 * 26 * 26], target: &Target, mut restrictions: Restrictions) -> usize {
    // Recurse through the tree, keeping track of the restrictions we've acquired along the way.
    // If we hit "accept" return the remaining possibilities (the volume of the hypercube of restrictions).
    if let Target::Workflow(idx) = target {
        let workflow = workflows[*idx].as_ref().unwrap();
        let mut count = 0;
        for rule in workflow.rules.iter() {
            if !restrictions.valid(&rule) {
                continue;
            }
            count += count_accepted(workflows, &rule.target, restrictions.intersection(rule));
            // If a criterion is not met, we must add in the negation of that criterion to continue
            restrictions.intersection_inplace(&!rule);
        }
        count += count_accepted(workflows, &workflow.default, restrictions);
        count
    } else if *target == Target::Accept {
        restrictions.volume()
    } else {
        0
    }
}

fn main() {
    const NO_WORKFLOW: Option<WorkFlow> = None;
    let mut workflows = [NO_WORKFLOW; 26 * 26 * 26];
    let mut parts: Vec<Part> = vec![];
    let mut done_workflows = false;

    // PARSE
    for line in read_input_lines().unwrap() {
        if !done_workflows {
            if line == "" {
                done_workflows = true;
                continue;
            }

            let (index, line) = line.split_once('{').unwrap();
            let mut rules = vec![];
            for rule_str in line.split(',') {
                if rule_str.ends_with('}') {
                    let default = Target::from_str(&rule_str[..rule_str.len() - 1]);
                    workflows[idx(index)] = Some(WorkFlow { rules, default });
                    break;
                }

                let rule = Rule::from_str(rule_str);
                rules.push(rule);
            }
        } else {
            let assignments = &line.as_bytes()[1..line.len() - 1];
            let x = assignments.split(|b| *b == b',').take(4).map(|assignment| {
                let mut value = 0;
                for digit in assignment[2..].iter() {
                    value *= 10;
                    value += (*digit as char).to_digit(10).unwrap() as u16;
                }
                value
            }).collect::<Vec<_>>().try_into().unwrap();
            parts.push(x);
        }
    }

    // PART1
    let start_index: RuleIndex = idx("in");
    let mut part1 = 0_usize;

    for part in parts.iter() {
        let mut workflow = workflows[start_index].as_ref().unwrap();
        loop {
            let target = workflow.eval(part);
            if let Target::Workflow(idx) = target {
                workflow = workflows[idx].as_ref().unwrap();
            } else if target == Target::Accept {
                part1 += part.iter().map(|x| *x as usize).sum::<usize>();
                break;
            } else {
                break;
            }
        }
    }

    // PART2
    let part2 = count_accepted(&workflows, &Target::Workflow(start_index), array![(0, 4001); 4]);
    println!("{}", part1);
    println!("{}", part2);
}
