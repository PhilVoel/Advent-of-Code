use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    htype: HandType,
    bid: u16,
}

impl Hand {
    fn new(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let mut cards_array = [Card::N2; 5];
        for (i, card) in cards.chars().enumerate() {
            cards_array[i] = Card::from(card);
        }
        Hand {
            cards: cards_array,
            htype: HandType::from(&cards_array),
            bid: bid.parse().unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.htype == other.htype {
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].partial_cmp(&other.cards[i]);
                }
            }
            return Some(Ordering::Equal);
        }
        else {
            return self.htype.partial_cmp(&other.htype);
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
enum Card {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => unreachable!("Invalid card"),
        }
    }
}

#[derive(PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; 5]> for HandType {
    fn from(value: &[Card; 5]) -> Self {
        let mut counts = [0; 13];
        let mut jokers = 0;
        for card in value.iter() {
            if *card == Card::J {
                jokers += 1;
            }
            else {
                counts[*card as usize] += 1;
            }
        }
        let mut count2 = false;
        let mut count3 = false;
        for count in counts.iter() {
            if *count == 5 {
                return HandType::FiveOfAKind;
            }
            else if *count == 4 {
                if jokers == 1 {
                    return HandType::FiveOfAKind;
                }
                else {
                    return HandType::FourOfAKind;
                }
            }
            else if *count == 3 {
                if count2 {
                    return HandType::FullHouse;
                }
                else if jokers == 2 {
                    return HandType::FiveOfAKind;
                }
                else if jokers == 1 {
                    return HandType::FourOfAKind;
                }
                else {
                    count3 = true;
                }
            }
            else if *count == 2 {
                if count3 {
                    return HandType::FullHouse;
                }
                else if count2 && jokers == 1{
                    return HandType::FullHouse;
                }
                else if count2 {
                    return HandType::TwoPair;
                }
                else if jokers == 3 {
                    return HandType::FiveOfAKind;
                }
                else if jokers == 2 {
                    return HandType::FourOfAKind;
                }
                else {
                    count2 = true;
                }
            }
        }
        if count3 {
            HandType::ThreeOfAKind
        }
        else if count2 && jokers == 1 {
            HandType::ThreeOfAKind
        }
        else if count2 {
            HandType::OnePair
        }
        else if jokers == 5 {
            HandType::FiveOfAKind
        }
        else if jokers == 4 {
            HandType::FiveOfAKind
        }
        else if jokers == 3 {
            HandType::FourOfAKind
        }
        else if jokers == 2 {
            HandType::ThreeOfAKind
        }
        else if jokers == 1 {
            HandType::OnePair
        }
        else {
            HandType::HighCard
        }
    }
}

fn main() {
    let mut hands = std::fs::read_to_string("input").unwrap()
        .lines()
        .map(|line| Hand::new(line))
        .collect::<Vec<Hand>>();
    hands.sort();
    println!("{}", hands.iter()
             .enumerate()
             .map(|(i, hand)| (i+1)*hand.bid as usize)
             .sum::<usize>()
    );
}
