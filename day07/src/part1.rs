use std::collections::HashMap;
use prse::parse;

use std::cmp::Ordering;

#[derive(Eq, Ord, PartialOrd, PartialEq)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfKind = 3,
    FullHouse = 4,
    FourOfKind = 5,
    FiveOfKind = 6,
}

#[derive(Eq, Ord, PartialOrd, PartialEq)]
enum CardValue {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}
#[derive(Eq, Ord)]
struct Hand {
    pub cards: Vec<char>,
    pub cards_value: Vec<CardValue>,
    pub hand_type: HandType,
    pub bid: u64,
}

impl Hand {
    pub fn new(cards: Vec<char>, bid: u64) -> Self {

        let hand_type = get_type(&cards);
        let cards_value: Vec<CardValue> = cards.iter().map(|x| get_value(x)).collect();

        Self { cards, bid, hand_type, cards_value }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
// implement partialOrd for Hand, using enums for comparison
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // if type is equal, we compare cards and return based on that
        if self.hand_type == other.hand_type {
            // go through each card until we have a winner
            for i in 0..5 {
                // if equal continue to next card
                if self.cards.get(i) == other.cards.get(i) {
                    continue;
                } else {
                    // return comparison of value
                    return self.cards_value.get(i).partial_cmp(&other.cards_value.get(i));
                }
            }
        } else { // Not equal, so we can just compare through enum
            return self.hand_type.partial_cmp(&other.hand_type);
        }
        panic!("Missing case... could not order {:?} and {:?}", self.cards, other.cards)
    }
}

fn get_type(cards: &Vec<char>) -> HandType {
    // clone cards
    let cards_cloned = cards.clone();

    // move into hashmap set counting occurrences per card
    let set = cards_cloned.into_iter()
        .fold(HashMap::<char, usize>::new(), |mut m, x| {
            *m.entry(x).or_default() += 1;
            m
    });

    //  1 bin; size 5 == fiveofakind
    //  2 bin; size 4 + 1 == fourofakind
    //  2 bin; size 3 + 2 == fullhouse
    //  3 bin; size 3 + 1 + 1 == threeofakind
    //  3 bin; size 2 + 2 + 1 == twopair
    //  4 bin; == onepair
    //  5 bin; == high card
    // match amount of bins, and maximum value in a bin
    match (set.keys().len(), set.values().max()) {
        (1, Some(5)) => HandType::FiveOfKind,
        (2, Some(4)) => HandType::FourOfKind,
        (2, Some(3)) => HandType::FullHouse,
        (3, Some(3)) => HandType::ThreeOfKind,
        (3, Some(2)) => HandType::TwoPair,
        (4, _) => HandType::OnePair,
        (5, _) => HandType::HighCard,
        _ => {panic!("Could not parse type of hand")}
    }
}

fn get_value(card: &char) -> CardValue {
    // match statement to return what value a card have
    match card {
        'A' => CardValue::Ace,
        'K' => CardValue::King,
        'Q' => CardValue::Queen,
        'J' => CardValue::Jack,
        'T' => CardValue::Ten,
        '9' => CardValue::Nine,
        '8' => CardValue::Eight,
        '7' => CardValue::Seven,
        '6' => CardValue::Six,
        '5' => CardValue::Five,
        '4' => CardValue::Four,
        '3' => CardValue::Three,
        '2' => CardValue::Two,
        _ => {panic!("Could not parse type of card")}
    }
}




pub fn process(input: &str) -> anyhow::Result<String> {
    let mut set: Vec<Hand> = vec![];

    // parse input
    for line in input.lines() {
        let (hand, bid): (String, u64) = parse!(line, "{} {}");
        let hand_obj = Hand::new(hand.chars().collect(), bid);
        set.push(hand_obj);
    }

    // sort custom structure based on traits for eq and partial eq
    set.sort();

    let mut sum: u64 = 0;
    // enumerate set
    for (n, hand) in set.iter().enumerate() {
        sum += hand.bid * (n as u64 + 1);
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
                assert_eq!("6440", process(input)?);
                Ok(())
    }
}