use itertools::Itertools;
use num::BigInt;
use num::integer::sqrt;
use aoc2023::common::{read_input_lines, strs_to_nums};

#[derive(Debug, Clone)]
struct Stone {
    p: (isize, isize, isize),
    v: (isize, isize, isize),
}

#[derive(Debug, Clone)]
struct BigStone {
    p: (BigInt, BigInt, BigInt),
    v: (BigInt, BigInt, BigInt),
}

impl Stone {
    fn parse(s: String) -> Self {
        let (p, v) = s.split_once(" @ ").unwrap();
        let p = strs_to_nums(p.split(',').map(|s| s.trim())).collect_tuple().unwrap();
        let v = strs_to_nums(v.split(',').map(|s| s.trim())).collect_tuple().unwrap();
        Stone{p, v}
    }

    fn test(&self, other: &Self, area: (isize, isize)) -> bool {
        if self.v.0 == 0 || self.v.1 * other.v.0 == self.v.0 * other.v.1 {
            return false;
        }

        let tb = (self.v.0 * (other.p.1 - self.p.1) - self.v.1 * (other.p.0 - self.p.0)) /
            (self.v.1 * other.v.0 - self.v.0 * other.v.1);
        let ta = (other.p.0 + tb * other.v.0 - self.p.0) / self.v.0;
        let xa = self.p.0 + ta * self.v.0;
        let ya = self.p.1 + ta * self.v.1;

        if ta >= 0 && tb >= 0 && xa >= area.0 && xa <= area.1 && ya >= area.0 && ya <= area.1 {
            return true;
        }
        false
    }

    fn big(&self) -> BigStone {
        BigStone{p: (self.p.0.into(), self.p.1.into(), self.p.2.into()), v: (self.v.0.into(), self.v.1.into(), self.v.2.into())}
    }
}

#[inline]
fn pos_at(s: &BigStone, t: &BigInt) -> (BigInt, BigInt, BigInt) {
    (s.p.0.clone() + s.v.0.clone() * t.clone(), s.p.1.clone() + s.v.1.clone() * t.clone(), s.p.2.clone() + s.v.2.clone() * t.clone())
}

fn part2(stones: &[Stone]) -> isize {
    let (a, b, c) = (&stones[0].big(), &stones[1].big(), &stones[2].big());

    // let tc = (-a.v.0.clone()*a.p.1.clone()*b.p.2.clone() + a.v.0.clone()*a.p.1.clone()*c.p.2.clone() + a.v.0.clone()*a.p.2.clone()*b.p.1.clone() - a.v.0.clone()*a.p.2.clone()*c.p.1.clone() - a.v.0.clone()*b.p.1.clone()*c.p.2.clone() + a.v.0.clone()*b.p.2.clone()*c.p.1.clone() + a.v.1.clone()*a.p.0.clone()*b.p.2.clone() - a.v.1.clone()*a.p.0.clone()*c.p.2.clone() - a.v.1.clone()*a.p.2.clone()*b.p.0.clone() + a.v.1.clone()*a.p.2.clone()*c.p.0.clone() + a.v.1.clone()*b.p.0.clone()*c.p.2.clone() - a.v.1.clone()*b.p.2.clone()*c.p.0.clone() - a.v.2.clone()*a.p.0.clone()*b.p.1.clone() + a.v.2.clone()*a.p.0.clone()*c.p.1.clone() + a.v.2.clone()*a.p.1.clone()*b.p.0.clone() - a.v.2.clone()*a.p.1.clone()*c.p.0.clone() - a.v.2.clone()*b.p.0.clone()*c.p.1.clone() + a.v.2.clone()*b.p.1.clone()*c.p.0.clone() - a.p.0.clone()*b.v.1.clone()*b.p.2.clone() + a.p.0.clone()*b.v.1.clone()*c.p.2.clone() + a.p.0.clone()*b.v.2.clone()*b.p.1.clone() - a.p.0.clone()*b.v.2.clone()*c.p.1.clone() + a.p.1.clone()*b.v.0.clone()*b.p.2.clone() - a.p.1.clone()*b.v.0.clone()*c.p.2.clone() - a.p.1.clone()*b.v.2.clone()*b.p.0.clone() + a.p.1.clone()*b.v.2.clone()*c.p.0.clone() - a.p.2.clone()*b.v.0.clone()*b.p.1.clone() + a.p.2.clone()*b.v.0.clone()*c.p.1.clone() + a.p.2.clone()*b.v.1.clone()*b.p.0.clone() - a.p.2.clone()*b.v.1.clone()*c.p.0.clone() + b.v.0.clone()*b.p.1.clone()*c.p.2.clone() - b.v.0.clone()*b.p.2.clone()*c.p.1.clone() - b.v.1.clone()*b.p.0.clone()*c.p.2.clone() + b.v.1.clone()*b.p.2.clone()*c.p.0.clone() + b.v.2.clone()*b.p.0.clone()*c.p.1.clone() - b.v.2.clone()*b.p.1.clone()*c.p.0.clone()) /
    //     (a.v.0.clone()*a.p.1.clone()*b.v.2.clone() - a.v.0.clone()*a.p.1.clone()*c.v.2.clone() - a.v.0.clone()*a.p.2.clone()*b.v.1.clone() + a.v.0.clone()*a.p.2.clone()*c.v.1.clone() + a.v.0.clone()*b.v.1.clone()*b.p.2.clone() - a.v.0.clone()*b.v.2.clone()*b.p.1.clone() + a.v.0.clone()*b.p.1.clone()*c.v.2.clone() - a.v.0.clone()*b.p.2.clone()*c.v.1.clone() - a.v.1.clone()*a.p.0.clone()*b.v.2.clone() + a.v.1.clone()*a.p.0.clone()*c.v.2.clone() + a.v.1.clone()*a.p.2.clone()*b.v.0.clone() - a.v.1.clone()*a.p.2.clone()*c.v.0.clone() - a.v.1.clone()*b.v.0.clone()*b.p.2.clone() + a.v.1.clone()*b.v.2.clone()*b.p.0.clone() - a.v.1.clone()*b.p.0.clone()*c.v.2.clone() + a.v.1.clone()*b.p.2.clone()*c.v.0.clone() + a.v.2.clone()*a.p.0.clone()*b.v.1.clone() - a.v.2.clone()*a.p.0.clone()*c.v.1.clone() - a.v.2.clone()*a.p.1.clone()*b.v.0.clone() + a.v.2.clone()*a.p.1.clone()*c.v.0.clone() + a.v.2.clone()*b.v.0.clone()*b.p.1.clone() - a.v.2.clone()*b.v.1.clone()*b.p.0.clone() + a.v.2.clone()*b.p.0.clone()*c.v.1.clone() - a.v.2.clone()*b.p.1.clone()*c.v.0.clone() - a.p.0.clone()*b.v.1.clone()*c.v.2.clone() + a.p.0.clone()*b.v.2.clone()*c.v.1.clone() + a.p.1.clone()*b.v.0.clone()*c.v.2.clone() - a.p.1.clone()*b.v.2.clone()*c.v.0.clone() - a.p.2.clone()*b.v.0.clone()*c.v.1.clone() + a.p.2.clone()*b.v.1.clone()*c.v.0.clone() - b.v.0.clone()*b.p.1.clone()*c.v.2.clone() + b.v.0.clone()*b.p.2.clone()*c.v.1.clone() + b.v.1.clone()*b.p.0.clone()*c.v.2.clone() - b.v.1.clone()*b.p.2.clone()*c.v.0.clone() - b.v.2.clone()*b.p.0.clone()*c.v.1.clone() + b.v.2.clone()*b.p.1.clone()*c.v.0.clone());

    let mut tc_p = BigInt::from(0);
    tc_p -= &a.v.0 * &a.p.1 * &b.p.2;
    tc_p += &a.v.0 * &a.p.1 * &c.p.2;
    tc_p += &a.v.0 * &a.p.2 * &b.p.1;
    tc_p -= &a.v.0 * &a.p.2 * &c.p.1;
    tc_p -= &a.v.0 * &b.p.1 * &c.p.2;
    tc_p += &a.v.0 * &b.p.2 * &c.p.1;
    tc_p += &a.v.1 * &a.p.0 * &b.p.2;
    tc_p -= &a.v.1 * &a.p.0 * &c.p.2;
    tc_p -= &a.v.1 * &a.p.2 * &b.p.0;
    tc_p += &a.v.1 * &a.p.2 * &c.p.0;
    tc_p += &a.v.1 * &b.p.0 * &c.p.2;
    tc_p -= &a.v.1 * &b.p.2 * &c.p.0;
    tc_p -= &a.v.2 * &a.p.0 * &b.p.1;
    tc_p += &a.v.2 * &a.p.0 * &c.p.1;
    tc_p += &a.v.2 * &a.p.1 * &b.p.0;
    tc_p -= &a.v.2 * &a.p.1 * &c.p.0;
    tc_p -= &a.v.2 * &b.p.0 * &c.p.1;
    tc_p += &a.v.2 * &b.p.1 * &c.p.0;
    tc_p -= &a.p.0 * &b.v.1 * &b.p.2;
    tc_p += &a.p.0 * &b.v.1 * &c.p.2;
    tc_p += &a.p.0 * &b.v.2 * &b.p.1;
    tc_p -= &a.p.0 * &b.v.2 * &c.p.1;
    tc_p += &a.p.1 * &b.v.0 * &b.p.2;
    tc_p -= &a.p.1 * &b.v.0 * &c.p.2;
    tc_p -= &a.p.1 * &b.v.2 * &b.p.0;
    tc_p += &a.p.1 * &b.v.2 * &c.p.0;
    tc_p -= &a.p.2 * &b.v.0 * &b.p.1;
    tc_p += &a.p.2 * &b.v.0 * &c.p.1;
    tc_p += &a.p.2 * &b.v.1 * &b.p.0;
    tc_p -= &a.p.2 * &b.v.1 * &c.p.0;
    tc_p += &b.v.0 * &b.p.1 * &c.p.2;
    tc_p -= &b.v.0 * &b.p.2 * &c.p.1;
    tc_p -= &b.v.1 * &b.p.0 * &c.p.2;
    tc_p += &b.v.1 * &b.p.2 * &c.p.0;
    tc_p += &b.v.2 * &b.p.0 * &c.p.1;
    tc_p -= &b.v.2 * &b.p.1 * &c.p.0;

    let mut tc_q = BigInt::from(0);
    tc_q += &a.v.0 * &a.p.1 * &b.v.2;
    tc_q -= &a.v.0 * &a.p.1 * &c.v.2;
    tc_q -= &a.v.0 * &a.p.2 * &b.v.1;
    tc_q += &a.v.0 * &a.p.2 * &c.v.1;
    tc_q += &a.v.0 * &b.v.1 * &b.p.2;
    tc_q -= &a.v.0 * &b.v.2 * &b.p.1;
    tc_q += &a.v.0 * &b.p.1 * &c.v.2;
    tc_q -= &a.v.0 * &b.p.2 * &c.v.1;
    tc_q -= &a.v.1 * &a.p.0 * &b.v.2;
    tc_q += &a.v.1 * &a.p.0 * &c.v.2;
    tc_q += &a.v.1 * &a.p.2 * &b.v.0;
    tc_q -= &a.v.1 * &a.p.2 * &c.v.0;
    tc_q -= &a.v.1 * &b.v.0 * &b.p.2;
    tc_q += &a.v.1 * &b.v.2 * &b.p.0;
    tc_q -= &a.v.1 * &b.p.0 * &c.v.2;
    tc_q += &a.v.1 * &b.p.2 * &c.v.0;
    tc_q += &a.v.2 * &a.p.0 * &b.v.1;
    tc_q -= &a.v.2 * &a.p.0 * &c.v.1;
    tc_q -= &a.v.2 * &a.p.1 * &b.v.0;
    tc_q += &a.v.2 * &a.p.1 * &c.v.0;
    tc_q += &a.v.2 * &b.v.0 * &b.p.1;
    tc_q -= &a.v.2 * &b.v.1 * &b.p.0;
    tc_q += &a.v.2 * &b.p.0 * &c.v.1;
    tc_q -= &a.v.2 * &b.p.1 * &c.v.0;
    tc_q -= &a.p.0 * &b.v.1 * &c.v.2;
    tc_q += &a.p.0 * &b.v.2 * &c.v.1;
    tc_q += &a.p.1 * &b.v.0 * &c.v.2;
    tc_q -= &a.p.1 * &b.v.2 * &c.v.0;
    tc_q -= &a.p.2 * &b.v.0 * &c.v.1;
    tc_q += &a.p.2 * &b.v.1 * &c.v.0;
    tc_q -= &b.v.0 * &b.p.1 * &c.v.2;
    tc_q += &b.v.0 * &b.p.2 * &c.v.1;
    tc_q += &b.v.1 * &b.p.0 * &c.v.2;
    tc_q -= &b.v.1 * &b.p.2 * &c.v.0;
    tc_q -= &b.v.2 * &b.p.0 * &c.v.1;
    tc_q += &b.v.2 * &b.p.1 * &c.v.0;

    let tc = tc_p / tc_q;

    let tb = (&a.v.0*&a.p.1*&tc - &a.v.0*&b.p.1*&tc - &a.v.1*&a.p.0*&tc + &a.v.1*&b.p.0*&tc - &a.p.0*&b.p.1 + &a.p.0*&c.v.1*&tc + &a.p.0*&c.p.1 + &a.p.1*&b.p.0 - &a.p.1*&c.v.0*&tc - &a.p.1*&c.p.0 - &b.p.0*&c.v.1*&tc - &b.p.0*&c.p.1 + &b.p.1*&c.v.0*&tc + &b.p.1*&c.p.0)/(&a.v.0*&a.p.1 + &a.v.0*&b.v.1*&tc - &a.v.0*&c.v.1*&tc - &a.v.0*&c.p.1 - &a.v.1*&a.p.0 - &a.v.1*&b.v.0*&tc + &a.v.1*&c.v.0*&tc + &a.v.1*&c.p.0 + &a.p.0*&b.v.1 - &a.p.1*&b.v.0 + &b.v.0*&c.v.1*&tc + &b.v.0*&c.p.1 - &b.v.1*&c.v.0*&tc - &b.v.1*&c.p.0);
    let ta = (-&a.p.1*&tb + &a.p.1*&tc - &b.v.1*&tb*&tc - &b.p.1*&tc + &c.v.1*&tb*&tc + &c.p.1*&tb) / (&a.v.1*&tb - &a.v.1*&tc - &b.v.1*&tb - &b.p.1 + &c.v.1*&tc + &c.p.1);

    let (ax, ay, az) = pos_at(a, &ta);
    let (bx, by, bz) = pos_at(b, &tb);
    let v = ((&bx - &ax) / (&tb - &ta), (&by - &ay) / (&tb - &ta), (&bz - &az) / (&tb - &ta));
    let p = (&ax - v.0 * &ta, &ay - v.1 * &ta, &az - v.2 * &ta);

    return [p.0, p.1, p.2].iter().sum::<BigInt>().try_into().unwrap();
}

fn main() {
    let input = read_input_lines().unwrap().map(Stone::parse).collect_vec();
    // println!("{:#?}", input);
    println!("{:?}", input.iter().combinations(2)
        // .cartesian_product(input.iter())
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];
            ((a, b), a.test(&b, (200000000000000, 400000000000000)))
        })
        .filter(|(_, y)| *y)
        .count());

    println!("{:?}", part2(&input));
}