use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
    str::FromStr,
};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum WinType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    win_type: WinType,
}

fn get_win_type(cards: &[Card]) -> WinType {
    let mut card_counts: HashMap<&Card, usize> = HashMap::new();

    for card in cards {
        *card_counts.entry(card).or_insert(0) += 1;
    }

    if card_counts.values().find(|v| **v == 5).is_some() {
        return WinType::FiveOfAKind;
    }

    if card_counts.values().find(|v| **v == 4).is_some() {
        return WinType::FourOfAKind;
    }

    if card_counts.values().find(|v| **v == 3).is_some() {
        if card_counts.values().find(|v| **v == 2).is_some() {
            return WinType::FullHouse;
        }

        return WinType::ThreeOfAKind;
    }

    let pair_count = card_counts.values().filter(|v| **v == 2).count();

    match pair_count {
        2 => WinType::TwoPair,
        1 => WinType::OnePair,
        _ => WinType::HighCard,
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').expect("hand bid");

        let cards: Vec<Card> = cards.chars().map(|char| char.into()).collect();
        let win_type = get_win_type(&cards);

        Ok(Self {
            cards,
            bid: bid.parse().expect("number"),
            win_type,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("invalid char for card {c}"),
        }
    }
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut hands: Vec<Hand> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    hands.sort_by(|a, b| {
        if a.win_type == b.win_type {
            for (a_card, b_card) in a.cards.iter().zip(b.cards.iter()) {
                match a_card.partial_cmp(b_card).unwrap() {
                    Ordering::Equal => continue,
                    ord => return ord,
                }
            }
        }
        a.win_type.partial_cmp(&b.win_type).unwrap()
    });

    let score: usize = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1))
        .sum();

    println!("Score: {score}");

    Ok(())
}
