use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<[Card; 5]> for HandType {
    fn from(cards: [Card; 5]) -> Self {
        let mut card_counts: HashMap<Card, usize> = HashMap::with_capacity(5);

        for card in cards {
            card_counts.entry(card).and_modify(|v| *v += 1).or_insert(1);
        }

        match card_counts.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if card_counts.iter().any(|(_, &count)| count == 4) {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if card_counts.iter().any(|(_, &count)| count == 3) {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            c => bail!("couldn't parse card from {:?}", c),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s
            .chars()
            .map(std::convert::TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?[..5]
            .try_into()?;

        Ok(Hand {
            cards,
            hand_type: cards.into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    games: Vec<(Hand, u64)>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s
            .lines()
            .map(|l| -> Result<(Hand, u64), Self::Err> {
                let (hand, bet) = l
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("couldn't split {:?} at space character", l))?;

                Ok((hand.parse()?, bet.parse()?))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { games })
    }
}

pub fn solve_part_1(p: &mut Problem) -> u64 {
    let Problem { games } = p;

    games.sort_by_key(|c| c.0);

    (1u64..)
        .zip(games)
        .map(|(rank, &mut (_, bid))| rank * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(
            p,
            Problem {
                games: vec![
                    (
                        Hand {
                            cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                            hand_type: HandType::OnePair,
                        },
                        765
                    ),
                    (
                        Hand {
                            cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                            hand_type: HandType::ThreeOfAKind,
                        },
                        684
                    ),
                    (
                        Hand {
                            cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                            hand_type: HandType::TwoPair,
                        },
                        28
                    ),
                    (
                        Hand {
                            cards: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                            hand_type: HandType::TwoPair,
                        },
                        220
                    ),
                    (
                        Hand {
                            cards: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                            hand_type: HandType::ThreeOfAKind,
                        },
                        483
                    )
                ]
            }
        );
    }

    #[test]
    fn test_solve_part_1() {
        let mut p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&mut p), 6440);
    }
}
