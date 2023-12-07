use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
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
            _ => Err(anyhow!("Invalid card: {}", c)),
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    r#type: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: usize) -> Self {
        let card_counts = cards.iter().counts();
        let mut card_counts = card_counts.values().sorted().rev().take(2);

        let r#type = match card_counts.next().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if *card_counts.next().unwrap() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if *card_counts.next().unwrap() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        };

        Self { cards, bid, r#type }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" ");

        let cards = parts
            .next()
            .ok_or(anyhow!("Missing cards"))?
            .chars()
            .map(|c| c.try_into().context("Invalid card"))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|v: Vec<_>| anyhow!("Invalid number of cards: {}", v.len()))?;

        let bid = parts
            .next()
            .ok_or(anyhow!("Missing bid"))?
            .parse()
            .context("Invalid bid")?;

        Ok(Hand::new(cards, bid))
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut hands = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.parse())
        .collect::<Result<Vec<Hand>>>()?;

    hands.sort();

    let total_winnings: usize = (0..hands.len()).map(|i| (i + 1) * hands[i].bid).sum();

    println!("Total Winnings: {}", total_winnings);

    Ok(())
}
