use std::cmp::{max, Ordering};
use aoc2023::common::read_input_lines;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
enum Hand {
    Nowt,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
enum Card {
    LowJ, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, J, Q, K, A
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
struct Play {
    cards: Vec<Card>,
    cards2: Vec<Card>,
    hand: Hand,
    hand2: Hand,
    bid: usize,
}

impl Play {
    #[inline]
    fn from_str(line: String) -> Play {
        let (cards, bid) = line.split_once(' ').unwrap();
        let (counts, joker_count) = Self::counts(cards);
        let hand = Play::hand(counts);
        let hand2 = Play::hand2(counts, joker_count);
        let cards2 = cards.bytes().map(Play::byte_to_card2).collect();
        let cards = cards.bytes().map(Play::byte_to_card).collect();
        let bid = bid.parse().unwrap();
        Play{cards, cards2, hand, hand2, bid}
    }

    #[inline]
    fn byte_to_card(byte: u8) -> Card {
        let byte = byte as char;
        match byte {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card {byte}")
        }
    }

    #[inline]
    fn byte_to_card2(byte: u8) -> Card {
        let byte = byte as char;
        match byte {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::LowJ,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card {byte}")
        }
    }

    #[inline]
    fn counts(cards: &str) -> ((u8, u8), u8) {
        let mut counts = [0; 5];

        let bytes = &cards.as_bytes()[0..5];
        let mut map = [255_u8; 256];
        for i in 0..5 {
            let c = bytes[i] as usize;
            if map[c] == 255 {
                map[c] = i as u8;
                counts[i] += 1;
            } else {
                counts[map[c] as usize] += 1;
            }
        }
        let mut largest = 0;
        let mut twos = 0;
        for i in 0..5 {
            largest = max(largest, counts[i]);
            if counts[i] == 2 {
                twos += 1;
            }
        }
        let jokers = match map['J' as usize] as usize {
            255 => 0,
            i => counts[i],
        };
        if largest > 2 {
            if twos > 0 {
                ((largest, 2), jokers)
            } else {
                ((largest, 1), jokers)
            }
        } else if twos == 2 {
            ((2, 2), jokers)
        } else if twos == 1 {
            ((2, 1), jokers)
        } else {
            ((1, 1), jokers)
        }
    }

    #[inline]
    fn hand(counts: (u8, u8)) -> Hand {
        match counts.0 {
            5 => Hand::Five,
            4 => Hand::Four,
            3 => if counts.1 == 2 { Hand::FullHouse } else { Hand::Three },
            2 => if counts.1 == 2 { Hand::TwoPair } else { Hand::OnePair },
            1 => Hand::Nowt,
            _ => panic!(),
        }
    }

    #[inline]
    fn hand2(counts: (u8, u8), joker_count: u8) -> Hand {
        return match joker_count {
            5 | 4 => Hand::Five,
            3 => match counts.1 {
                // counts.0 is the three jokers

                // JJJXX --> XXXXX
                2 => Hand::Five,
                // JJJXY --> XXXXY
                1 => Hand::Four,
                _ => panic!(),
            },
            2 => match counts.0 {
                3 => Hand::Five,
                2 => match counts.1 {
                    // XXJJY
                    2 => Hand::Four,
                    // XYZJJ
                    _ => Hand::Three,
                },
                _ => { panic!(); }
            },
            1 => match counts.0 {
                // JXXXX --> XXXXX
                4 => Hand::Five,
                // JXXXY --> XXXXY
                3 => Hand::Four,
                2 => match counts.1 {
                    // JXXYY --> XXXYY
                    2 => Hand::FullHouse,
                    // JXXYZ --> XXXYZ
                    1 => Hand::Three,
                    _ => panic!(),
                },
                1 => Hand::OnePair,
                _ => panic!(),
            },
            0 => match counts.0 {
                5 => Hand::Five,
                4 => Hand::Four,
                3 => if counts.1 == 2 { Hand::FullHouse } else { Hand::Three },
                2 => if counts.1 == 2 { Hand::TwoPair } else { Hand::OnePair },
                1 => Hand::Nowt,
                _ => panic!(),
            },
            _ => panic!(),
        };
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand.partial_cmp(&other.hand) {
            Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            ord => ord,
        }
    }
}

fn main () {
    for _ in 0..1{
        let lines = read_input_lines().expect("Could not read input file");

        let mut plays = lines.map(Play::from_str).collect::<Vec<_>>();
        plays.sort();
        let part1: usize = plays
            .iter()
            .enumerate()
            .map(|(i, play)| (i+1) * play.bid)
            .sum();
        println!("{}", part1);
        plays.sort_by(|play1, play2| match play1.hand2.cmp(&play2.hand2) {
            Ordering::Equal => play1.cards2.cmp(&play2.cards2),
            ord => ord,
        });
        let part2: usize = plays
            .iter()
            .enumerate()
            .map(|(i, play)| (i+1) * play.bid)
            .sum();
        println!("{:?}", part2);
    }
}