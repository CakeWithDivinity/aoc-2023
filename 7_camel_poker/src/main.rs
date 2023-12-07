use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
    str::FromStr,
};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum WinType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
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

    let joker_count = *card_counts.get(&Card::Jack).unwrap_or(&0);
    card_counts.remove_entry(&Card::Jack);

    if card_counts.values().any(|v| (*v + joker_count) == 5) {
        return WinType::FiveOfAKind;
    }

    if card_counts.values().any(|v| (*v + joker_count) == 4) {
        return WinType::FourOfAKind;
    }

    let mut sorted_card_counts: Vec<(&&Card, usize)> = card_counts
        .iter()
        .map(|(card, card_count)| (card, *card_count))
        .collect();
    sorted_card_counts.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (card, card_count) in sorted_card_counts.iter() {
        if *card_count > 3 {
            // already checked
            continue;
        }

        if card_count + joker_count >= 3 {
            let remaining_jokers = joker_count - (3 - card_count);

            for (pair_card, pair_card_count) in card_counts.iter() {
                if pair_card.eq(*card) {
                    continue;
                }

                if pair_card_count + remaining_jokers >= 2 {
                    return WinType::FullHouse;
                }
            }

            return WinType::ThreeOfAKind;
        }

        if card_count + joker_count >= 2 {
            let remaining_jokers = joker_count - (2 - card_count);

            for (pair_card, pair_card_count) in card_counts.iter() {
                if pair_card.eq(*card) {
                    continue;
                }

                if pair_card_count + remaining_jokers >= 2 {
                    return WinType::TwoPair;
                }
            }

            return WinType::OnePair;
        }
    }

    match joker_count {
        5 => WinType::FiveOfAKind,
        0 => WinType::HighCard,
        _ => panic!("not possible?"),
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
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
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

