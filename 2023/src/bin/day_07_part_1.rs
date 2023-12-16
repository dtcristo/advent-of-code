use itertools::Itertools;
use winnow::{
    ascii::digit1,
    combinator::{fail, opt, repeat},
    token::any,
    PResult, Parser,
};

fn main() {
    let input = include_str!("../../input/day_07");
    let result = solution(input);
    println!("{result}");
}

fn solution(input: &str) -> u32 {
    // Parse input as a vector of `Hand`.
    let hands: Vec<Hand> = repeat(0.., Hand::parse).parse(input).unwrap();

    // Iterate over each hand.
    hands
        .into_iter()
        // Sort them by strength ascending.
        .sorted_by_key(|hand| hand.strength())
        // Enumerate each hand, add one to form the rank (which starts at 1).
        // Multiply rank by hand's bid to produce winnings.
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        // Sum the winnings.
        .sum()
}

// `Hand` is a struct we parse the each line of input into.
#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    // Parse an input string slice into `Hand`.
    fn parse(input: &mut &str) -> PResult<Hand> {
        let parse_cards: (_, _, _, _, _) = [Card::parse; 5].into();
        let (cards, _, bid, _) = (parse_cards, ' ', digit1, opt('\n')).parse_next(input)?;

        Ok(Hand {
            cards: cards.into(),
            bid: bid.parse().unwrap(),
        })
    }

    // Classify the hand into a `HandType` variant.
    fn classify(&self) -> HandType {
        let counts: Vec<usize> =
            // Iterate over cards in the hand.
            self
                .cards
                .iter()
                // Count card occurences.
                .counts()
                .values()
                .copied()
                // Sort the counts for matching below.
                .sorted()
                .collect();

        // Match every possible compbination of grouped cards into a `HandType`.
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("invalid hand"),
        }
    }

    // Strength is a numeric value used to simplify ordering of relative hands.
    // We build a number in base-13. 13 is chosen because there are 13 possible
    // cards. The numeric representation of the hand type is the most significant
    // digit, followed by first card's numeric representation, second etc. These
    // are combined to produce a single `u32` that can be easily ordered.
    fn strength(&self) -> u32 {
        [
            self.cards[4] as u32,
            self.cards[3] as u32,
            self.cards[2] as u32,
            self.cards[1] as u32,
            self.cards[0] as u32,
            self.classify() as u32,
        ]
        .iter()
        .enumerate()
        .map(|(i, x)| x * 13_u32.pow(i as u32))
        .sum()
    }
}

// Struct representing a hand type. This is `repr(u8)` for use in
// `Hand::strength` calculation. The order here is significant.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// Struct representing a single card. This is `repr(u8)` for use in
// `Hand::strength` calculation (and other places we sort cards).
// The order here is significant.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
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

impl Card {
    // Parse an input string slice into `Card`.
    fn parse(input: &mut &str) -> PResult<Card> {
        Ok(match any.parse_next(input)? {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => return fail(input),
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::*;

    #[test]
    fn test_solution() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = solution(input);
        let expected = 6440;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand_parse() {
        let mut input = "KTJJT 220
QQQJA 483";
        let result = Hand::parse(&mut input).unwrap();
        assert_eq!(
            result,
            Hand {
                cards: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                bid: 220,
            }
        );
        assert_eq!(input, "QQQJA 483");
    }

    #[rstest]
    #[case(
        Hand {
            cards: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
            bid: 220,
        },
        HandType::TwoPair
    )]
    fn test_hand_classify(#[case] hand: Hand, #[case] hand_type: HandType) {
        assert_eq!(hand.classify(), hand_type);
    }

    #[test]
    fn test_card_parse() {
        let mut input = "A2345";
        let result = Card::parse(&mut input).unwrap();
        assert_eq!(result, Card::Ace);
        assert_eq!(input, "2345");
    }
}
