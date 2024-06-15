#![allow(dead_code)]

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

//Game: Canasta
//Util Functions
//Playing without red threes

//TODO: Add card transposition tables
//TODO: Fix get_num_canastas()

enum Play {
    Discard(Card),
    Draw,
    PickupPile(WildCardSubset),
    PlaceWild(PlayableCardSubset, WildCardSubset),
    Play(PlayableCardSubset),
    GoOut,
}

#[derive(Copy, Clone, PartialEq)]
enum Card {
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

impl Card {
    fn to_string(&self) -> &str {
        match self {
            Card::Joker => "J",
            Card::Two => "2",
            Card::Three => "3",
            Card::Four => "4",
            Card::Five => "5",
            Card::Six => "6",
            Card::Seven => "7",
            Card::Eight => "8",
            Card::Nine => "9",
            Card::Ten => "10",
            Card::Jack => "J",
            Card::Queen => "Q",
            Card::King => "K",
            Card::Ace => "A",
        }
    }
    fn get_index(&self) -> usize {
        match self {
            Card::Joker => 0,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 1,
        }
    }
    fn get_value(&self) -> u16 {
        match self {
            Card::Joker => 50,
            Card::Two => 20,
            Card::Three => 5,
            Card::Four => 5,
            Card::Five => 5,
            Card::Six => 5,
            Card::Seven => 5,
            Card::Eight => 10,
            Card::Nine => 10,
            Card::Ten => 10,
            Card::Jack => 10,
            Card::Queen => 10,
            Card::King => 10,
            Card::Ace => 20,
        }
    }
}

fn get_card(index: usize) -> Card {
    match index {
        0 => Card::Joker,
        1 => Card::Ace,
        2 => Card::Two,
        3 => Card::Three,
        4 => Card::Four,
        5 => Card::Five,
        6 => Card::Six,
        7 => Card::Seven,
        8 => Card::Eight,
        9 => Card::Nine,
        10 => Card::Ten,
        11 => Card::Jack,
        12 => Card::Queen,
        13 => Card::King,
        _ => panic!("Asked for card of invalid index"),
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank = match self {
            Card::Ace => "Ace",
            Card::Two => "Two",
            Card::Three => "Three",
            Card::Four => "Four",
            Card::Five => "Five",
            Card::Six => "Six",
            Card::Seven => "Seven",
            Card::Eight => "Eight",
            Card::Nine => "Nine",
            Card::Ten => "Ten",
            Card::Jack => "Jack",
            Card::Queen => "Queen",
            Card::King => "King",
            Card::Joker => "Joker",
        };
        write!(f, "{}", rank)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum WildCardSubset {
    Joker,
    Two,
}

#[derive(Debug, PartialEq, Eq)]
enum PlayableCardSubset {
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

impl From<PlayableCardSubset> for Card {
    fn from(card_subset: PlayableCardSubset) -> Card {
        match card_subset {
            PlayableCardSubset::Four => Card::Four,
            PlayableCardSubset::Five => Card::Five,
            PlayableCardSubset::Six => Card::Six,
            PlayableCardSubset::Seven => Card::Seven,
            PlayableCardSubset::Eight => Card::Eight,
            PlayableCardSubset::Nine => Card::Nine,
            PlayableCardSubset::Ten => Card::Ten,
            PlayableCardSubset::Jack => Card::Jack,
            PlayableCardSubset::Queen => Card::Queen,
            PlayableCardSubset::King => Card::King,
            PlayableCardSubset::Ace => Card::Ace,
        }
    }
}

impl From<WildCardSubset> for Card {
    fn from(wild_card_subset: WildCardSubset) -> Card {
        match wild_card_subset {
            WildCardSubset::Joker => Card::Joker,
            WildCardSubset::Two => Card::Two,
        }
    }
}

struct DrawPile {
    cards: Vec<Card>,
}

impl DrawPile {
    fn new(decks: u8) -> Self {
        let mut cards = Vec::new();
        for _ in 0..decks {
            for card in [
                Card::Two,
                Card::Four,
                Card::Five,
                Card::Six,
                Card::Seven,
                Card::Eight,
                Card::Nine,
                Card::Ten,
                Card::Jack,
                Card::Queen,
                Card::King,
                Card::Ace,
            ] {
                // Add four of each card
                for _ in 0..4 {
                    cards.push(card);
                }
            }

            // Add two Jokers
            cards.push(Card::Joker);
            cards.push(Card::Joker);

            cards.push(Card::Three);
            cards.push(Card::Three);
        }
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

#[derive(Copy, Clone)]
struct BoardStack {
    card_type: Card,
    jokers: u8,
    twos: u8,
    card_count: u8,
}

impl BoardStack {
    fn new(card_type: Card, jokers: u8, twos: u8, card_count: u8) -> BoardStack {
        Self {
            card_type,
            jokers,
            twos,
            card_count,
        }
    }

    fn get_score(&self) -> u16 {
        (self.jokers as u16) * 50
            + (self.twos as u16) * 20
            + (self.card_count as u16) * self.card_type.get_value()
    }

    fn is_canasta(&self) -> bool {
        self.get_total_count() >= 7
    }

    fn is_dirty(&self) -> bool {
        self.jokers > 0 || self.twos > 0
    }

    fn get_total_count(&self) -> u8 {
        self.jokers + self.twos + self.card_count
    }

    fn to_string(&self) -> String {
        format!(
            "({}, C:{}, J:{}, T:{}",
            self.card_type.to_string(),
            self.card_count,
            self.jokers,
            self.twos
        )
    }
}

struct Board {
    piles: [Option<BoardStack>; 14],
    down: bool,
}

impl Board {
    fn new() -> Board {
        Self {
            piles: [None; 14],
            down: false,
        }
    }
    fn get_score(&self) -> u16 {
        let mut score: u16 = 0;
        for stack_opt in self.piles.iter() {
            if let Some(stack) = stack_opt {
                score += stack.get_score();
                if stack.is_canasta() {
                    if stack.is_dirty() {
                        score += 300;
                    } else {
                        score += 500;
                    }
                }
            }
        }
        score
    }
    fn get(&self, card: Card) -> Option<BoardStack> {
        self.piles[card.get_index()]
    }
    fn is_down(&self) -> bool {
        self.down
    }
    fn place_card(&mut self, card: Card, count: u8) {
        self.down = true;
        match self.piles[card.get_index()] {
            Some(mut pile) => {
                pile.card_count += count;
            }
            None => {
                self.piles[card.get_index()] = Some(BoardStack {
                    card_type: card,
                    jokers: 0,
                    twos: 0,
                    card_count: count,
                });
            }
        }
    }
    fn place_joker(&mut self, card: Card) {
        match self.piles[card.get_index()] {
            Some(mut pile) => {
                pile.jokers += 1;
            }
            None => {
                self.piles[card.get_index()] = Some(BoardStack {
                    card_type: card,
                    jokers: 1,
                    twos: 0,
                    card_count: 0,
                });
            }
        }
    }
    fn place_two(&mut self, card: Card) {
        match self.piles[card.get_index()] {
            Some(mut pile) => {
                pile.twos += 1;
            }
            None => {
                self.piles[card.get_index()] = Some(BoardStack {
                    card_type: card,
                    jokers: 0,
                    twos: 1,
                    card_count: 0,
                });
            }
        }
    }
    fn get_num_canastas(&self) -> u8 {
        let mut count = 0;
        for stack_opt in self.piles.iter() {
            if let Some(stack) = stack_opt {
                if stack.is_canasta() {
                    count += 1;
                }
            }
        }
        count
    }
}

struct Hand {
    hand: [u8; 14],
}

impl Hand {
    fn new() -> Hand {
        Self { hand: [0; 14] }
    }
    fn get(&self, card: Card) -> u8 {
        self.hand[card.get_index()]
    }
    fn add(&mut self, card: Card, count: u8) {
        self.hand[card.get_index()] += count;
    }
    fn remove(&mut self, card: Card, count: u8) {
        if self.hand[card.get_index()] < count {
            panic!("Tried to discard a card that a player didn't have");
        }
        self.hand[card.get_index()] -= count;
    }
    fn is_empty(&self) -> bool {
        for i in 0..14 {
            if self.hand[i] > 0 {
                return false;
            }
        }
        true
    }
    fn get_score(&self) -> u16 {
        let mut score = 0;
        for i in 0..14 {
            score += (self.hand[i] as u16) * get_card(i).get_value();
        }
        score
    }
    fn get_hand_size(&self) -> u8 {
        let mut size = 0;
        for i in 0..14 {
            size += self.hand[i]
        }
        size
    }
}

struct Player {
    hand: Hand,
    board: Board,
    knowledge: Vec<[u8; 14]>,
}
impl Player {
    fn new(players_count: u8) -> Player {
        let mut knowledge: Vec<[u8; 14]> = Vec::new();
        for _ in 0..(players_count - 1) {
            knowledge.push([0; 14]);
        }
        Self {
            hand: Hand::new(),
            board: Board::new(),
            knowledge: knowledge,
        }
    }
}

struct TurnCounter {
    turn: u8,
    players_count: u8,
}
impl TurnCounter {
    fn new(players_count: u8) -> TurnCounter {
        Self {
            turn: 0,
            players_count: players_count,
        }
    }
    fn get(&self) -> u8 {
        self.turn
    }
    fn add(&mut self) {
        self.turn += 1;
        self.turn %= self.players_count;
    }
}

struct Game {
    draw_pile: DrawPile,
    discard_pile: Vec<Card>,
    players: Vec<Player>,
    teams_count: u8,
    players_per_team: u8,
    finished: bool,
    frozen: bool,
    turn: TurnCounter,
    drawn: bool,
    curr_player_drawn: bool,
}
impl Game {
    fn new(teams_count: u8, players_per_team: u8, decks: u8) -> Game {
        let mut draw_pile = DrawPile::new(decks);
        draw_pile.shuffle();
        let mut players: Vec<Player> = Vec::new();
        for _ in 0..(players_per_team * teams_count) {
            players.push(Player::new(players_per_team * teams_count))
        }
        Self {
            draw_pile: draw_pile,
            discard_pile: Vec::new(),
            players: players,
            teams_count: teams_count,
            players_per_team: players_per_team,
            finished: false,
            frozen: false,
            turn: TurnCounter::new(players_per_team * teams_count),
            drawn: false,
            curr_player_drawn: false,
        }
    }
    fn draw(&mut self) -> Card {
        let card: Option<Card> = self.draw_pile.draw();
        if card.is_none() {
            panic!("Drew a card from an empty deck");
        } else {
            if self.discard_pile.is_empty() {
                self.finished = true;
            }
            card.unwrap()
        }
    }
    fn get_curr_player(&self) -> &Player {
        &self.players[self.turn.get() as usize]
    }
    fn check_legal(&self, play: Play) -> bool {
        let player: &Player = self.get_curr_player();
        let hand: &Hand = &player.hand;
        let hand_size : u8 = hand.get_hand_size();
        let board: &Board = &player.board;
        match play {
            Play::GoOut => {
                self.drawn
                    && board.get_num_canastas() >= 2
                    && hand_size - hand.get(Card::Three) == 1
                    && (hand.get(Card::Three) >= 3 || hand.get(Card::Three) == 0)
            }
            Play::PickupPile(subset_wild) => {
                let top_card = self.discard_pile[self.discard_pile.len() - 1];
                if self.drawn {
                    return false;
                }
                if self.discard_pile.len() == 0 {
                    return false;
                }
                if let Some(_stack) = board.get(top_card) {
                    if hand_size + (self.discard_pile.len() as u8) - 1 >= 2
                        && !self.frozen
                    {
                        return true;
                    }
                }
                if hand_size <= 3 && self.discard_pile.len() == 1 {
                    return false;
                }
                if hand_size <= 2 && self.discard_pile.len() == 2 {
                    return false;
                }
                if top_card == Card::Joker || top_card == Card::Two || top_card == Card::Three {
                    return false;
                }
                if self.frozen || !board.is_down() {
                    return hand.get(top_card) >= 2;
                }
                !self.frozen && hand.get(Card::from(subset_wild)) >= 1 && hand.get(top_card) >= 2
            }
            Play::PlaceWild(subset_card, wild) => {
                let card: Card = Card::from(subset_card);
                if !self.drawn {
                    return false;
                }
                if hand_size <= 2 && board.get_num_canastas() < 2 {
                    if board.get_num_canastas() == 1 {
                        if let Some(stack) = board.get(card) {
                            if stack.get_total_count() == 6 {
                                return hand_size == 2 && hand.get(Card::from(wild)) >= 1 && stack.twos + stack.jokers + 1 <= stack.card_count;
                            }
                        }
                    }
                    return false;
                }
                if let Some(stack) = board.get(card) { 
                    if hand.get(Card::from(wild)) >= 1 {
                        return stack.twos + stack.jokers + 1 <= stack.card_count;
                    }
                }
                false
            }
            Play::Draw => !self.drawn,
            Play::Discard(card) => {
                if !self.drawn {
                    return false;
                }
                if hand.get(card) == 0 {
                    return false;
                }
                board.get_num_canastas() >= 2 || hand_size > 1
            },
            Play::Play(subset_card) => {
                let card : Card = Card::from(subset_card);
                if hand.get(card) == 0 || !self.drawn || hand_size == 1 || (hand_size == 2 && board.get_num_canastas() < 2 && !(board.get_num_canastas() == 1 && board.get(card).is_some() && board.get(card).unwrap().get_total_count() == 6)) {
                    return false;
                }
                if board.get(card).is_some() {
                    return true;
                }
                if hand.get(card) >= 3 || (hand.get(card) == 2 && hand.get(Card::Joker) + hand.get(Card::Two) + hand.get(Card::Three) >= 1) {
                    return hand_size >= 5 || (hand_size >= 4 && board.get_num_canastas() >= 2);
                }
                false
            },
        }
    }
}
