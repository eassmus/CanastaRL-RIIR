use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

//Util Functions
//Playing without red threes

#[derive(Clone)]
enum Rank {
    Joker,
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank = match self {
            Rank::Ace => "Ace",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Joker => "Joker",
        };
        write!(f, "{}", rank)
    }
}

struct Card {
    rank : Rank
}

impl Card {
    fn to_string(&self) -> &str {
        match self.rank {
            Rank::Joker => "J",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }
    fn get_index(&self) -> usize {
        match self.rank {
            Rank::Joker => 0,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 1,
        }
    }
    fn get_value(&self) -> u16 {
        match self.rank {
            Rank::Joker => 50,
            Rank::Two => 20,
            Rank::Three => 5,
            Rank::Four => 5,
            Rank::Five => 5,
            Rank::Six => 5,
            Rank::Seven => 5,
            Rank::Eight => 10,
            Rank::Nine => 10,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 20,
        }
    }
}

struct DrawPile {
    cards: Vec<Card>,
}

impl DrawPile {
    fn new() -> Self {
        let mut cards = Vec::new();
        for rank in [
            Rank::Two,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ] {
            // Add four of each card
            for _ in 0..4 {
                cards.push(Card { rank: rank.clone() });
            }
        }

        // Add two Jokers
        cards.push(Card { rank: Rank::Joker });
        cards.push(Card { rank: Rank::Joker });
        
        cards.push(Card { rank: Rank::Three });
        cards.push(Card { rank: Rank::Three });

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.as_mut_slice().shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

struct BoardStack {
    card_type : Card,
    jokers : u8,
    twos : u8,
    card_count : u8,
}

impl BoardStack {
    fn new(card_type : Card, jokers : u8, twos : u8, card_count : u8) -> BoardStack {
        Self {card_type, jokers, twos, card_count}
    }

    fn get_score(&self) -> u16 {
        (self.jokers as u16) * 50 + (self.twos as u16) * 20 + (self.card_count as u16) * self.card_type.get_value()
    }

    fn is_dirty(&self) -> bool {
        self.jokers > 0 || self.twos > 0
    }

    fn get_total_count(&self) -> u8 {
        self.jokers + self.twos + self.card_count
    }

    fn to_string(&self) -> String {
        format!("({}, C:{}, J:{}, T:{}", self.card_type.to_string(), self.card_count, self.jokers, self.twos)
    }
}

struct Board {
    piles : [Option<BoardStack>; 14],
}

impl Board {

}

struct Game {

}