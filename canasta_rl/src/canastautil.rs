#![allow(dead_code)]

use canasta_rl::mdp::{Agent, State};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;
use std::slice::Iter;
use std::sync::{Arc, Mutex};
extern crate rand;

const DEBUG: bool = false;

//Game: Canasta
//Util Functions
//Playing without red threes

//TODO: Fix get_num_canastas()

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Play {
    Discard(Card),
    Draw,
    PickupPile,
    PlaceWild(PlayableCardSubset),
    Play(PlayableCardSubset),
    GoOut,
}

impl Play {
    pub fn iterator() -> Iter<'static, Play> {
        static PLAYS: [Play; 39] = [
            Play::Discard(Card::Joker),
            Play::Discard(Card::Two),
            Play::Discard(Card::Three),
            Play::Discard(Card::Four),
            Play::Discard(Card::Five),
            Play::Discard(Card::Six),
            Play::Discard(Card::Seven),
            Play::Discard(Card::Eight),
            Play::Discard(Card::Nine),
            Play::Discard(Card::Ten),
            Play::Discard(Card::Jack),
            Play::Discard(Card::Queen),
            Play::Discard(Card::King),
            Play::Discard(Card::Ace),
            Play::GoOut,
            Play::PickupPile,
            Play::Play(PlayableCardSubset::Four),
            Play::Play(PlayableCardSubset::Five),
            Play::Play(PlayableCardSubset::Six),
            Play::Play(PlayableCardSubset::Seven),
            Play::Play(PlayableCardSubset::Eight),
            Play::Play(PlayableCardSubset::Nine),
            Play::Play(PlayableCardSubset::Ten),
            Play::Play(PlayableCardSubset::Jack),
            Play::Play(PlayableCardSubset::Queen),
            Play::Play(PlayableCardSubset::King),
            Play::Play(PlayableCardSubset::Ace),
            Play::PlaceWild(PlayableCardSubset::Four),
            Play::PlaceWild(PlayableCardSubset::Five),
            Play::PlaceWild(PlayableCardSubset::Six),
            Play::PlaceWild(PlayableCardSubset::Seven),
            Play::PlaceWild(PlayableCardSubset::Eight),
            Play::PlaceWild(PlayableCardSubset::Nine),
            Play::PlaceWild(PlayableCardSubset::Ten),
            Play::PlaceWild(PlayableCardSubset::Jack),
            Play::PlaceWild(PlayableCardSubset::Queen),
            Play::PlaceWild(PlayableCardSubset::King),
            Play::PlaceWild(PlayableCardSubset::Ace),
            Play::Draw,
        ];
        PLAYS.iter()
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum Card {
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
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Jack => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
        }
    }
    fn from_index(i: usize) -> Card {
        match i {
            0 => Card::Joker,
            1 => Card::Two,
            2 => Card::Three,
            3 => Card::Four,
            4 => Card::Five,
            5 => Card::Six,
            6 => Card::Seven,
            7 => Card::Eight,
            8 => Card::Nine,
            9 => Card::Ten,
            10 => Card::Jack,
            11 => Card::Queen,
            12 => Card::King,
            13 => Card::Ace,
            _ => panic!("Invalid card index"),
        }
    }
    fn get_simple_string(&self) -> &str {
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
    fn iterator() -> Iter<'static, Card> {
        static CARDS: [Card; 14] = [
            Card::Joker,
            Card::Two,
            Card::Three,
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
        ];
        CARDS.iter()
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum WildCardSubset {
    Joker,
    Two,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PlayableCardSubset {
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
impl PlayableCardSubset {
    fn iterator() -> Iter<'static, PlayableCardSubset> {
        static CARDS: [PlayableCardSubset; 11] = [
            PlayableCardSubset::Four,
            PlayableCardSubset::Five,
            PlayableCardSubset::Six,
            PlayableCardSubset::Seven,
            PlayableCardSubset::Eight,
            PlayableCardSubset::Nine,
            PlayableCardSubset::Ten,
            PlayableCardSubset::Jack,
            PlayableCardSubset::Queen,
            PlayableCardSubset::King,
            PlayableCardSubset::Ace,
        ];
        CARDS.iter()
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct BoardStack {
    card_type: Card,
    jokers: u8,
    twos: u8,
    card_count: u8,
}

impl BoardStack {
    fn new(card_type: Card, jokers: u8, twos: u8, card_count: u8) -> BoardStack {
        Self {
            card_type: card_type,
            jokers: jokers,
            twos: twos,
            card_count: card_count,
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Board {
    piles: [Option<BoardStack>; 14],
    down: bool,
    went_out: bool,
}

impl Board {
    fn new() -> Board {
        Self {
            piles: [None; 14],
            down: false,
            went_out: false,
        }
    }
    fn get_score(&self) -> u16 {
        let mut score: u16 = {
            if self.went_out {
                100
            } else {
                0
            }
        };
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
            Some(pile) => {
                self.piles[card.get_index()] = Some(BoardStack {
                    card_type: card,
                    jokers: pile.jokers,
                    twos: pile.twos,
                    card_count: pile.card_count + count,
                })
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
            Some(pile) => {
                self.piles[card.get_index()] = Some(BoardStack {
                    card_type: card,
                    jokers: pile.jokers + 1,
                    twos: pile.twos,
                    card_count: pile.card_count,
                })
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
            Some(pile) => {
                self.piles[card.get_index()] = Some(BoardStack {
                    card_type: card,
                    jokers: pile.jokers,
                    twos: pile.twos + 1,
                    card_count: pile.card_count,
                })
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: Vec<(String, u8, u8, u8, bool)> = Vec::new();
        for i in 0..14 {
            if let Some(stack) = self.piles[i] {
                let card = Card::from_index(i);
                output.push((
                    (*card.get_simple_string()).to_string(),
                    stack.card_count,
                    stack.jokers,
                    stack.twos,
                    stack.is_canasta(),
                ));
            }
        }
        write!(f, "{:?}", output)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Hand {
    hand: [u8; 14],
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cards: Vec<String> = Vec::new();
        for i in 0..14 {
            for _ in 0..self.hand[i] {
                let card = Card::from_index(i);
                cards.push((*card.get_simple_string()).to_string());
            }
        }
        write!(f, "{:?}", cards)
    }
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

#[derive(Clone, Debug)]
struct Player {
    hand: Hand,
    board: Arc<Mutex<Board>>,
    knowledge: Vec<[i8; 14]>,
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}
impl Eq for Player {
    fn assert_receiver_is_total_eq(&self) {
        assert_eq!(self, self);
    }
}
impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hand.hash(state);
        self.knowledge.hash(state);
    }
}
impl Player {
    fn new(players_count: u8, board: Arc<Mutex<Board>>) -> Player {
        let mut knowledge: Vec<[i8; 14]> = Vec::new();
        for _ in 0..(players_count - 1) {
            knowledge.push([0; 14]);
        }
        Self {
            hand: Hand::new(),
            board: board,
            knowledge: knowledge,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TurnCounter {
    turn: u8,
    players_count: u8,
    pub total_turns: u16,
}
impl TurnCounter {
    fn new(players_count: u8) -> TurnCounter {
        Self {
            turn: 0,
            players_count: players_count,
            total_turns: 0,
        }
    }
    pub fn get(&self) -> u8 {
        self.turn
    }
    fn add(&mut self) {
        self.turn += 1;
        self.turn %= self.players_count;
        self.total_turns += 1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Game {
    draw_pile: DrawPile,
    discard_pile: Vec<Card>,
    players: Vec<Player>,
    players_per_team: u8,
    teams_count: u8,
    pub finished: bool,
    frozen: bool,
    pub turn: TurnCounter,
    curr_player_drawn: bool,
}

impl Game {
    pub fn new(teams_count: u8, players_per_team: u8, decks: u8, hand_size: u8) -> Game {
        let mut draw_pile = DrawPile::new(decks);
        draw_pile.shuffle();
        let mut players: Vec<Player> = Vec::new();
        let mut boards: Vec<Arc<Mutex<Board>>> = Vec::new();
        for _ in 0..teams_count {
            boards.push(Arc::new(Mutex::new(Board::new())));
        }
        for i in 0..(players_per_team * teams_count) {
            let mut player: Player = Player::new(
                players_per_team * teams_count,
                boards[(i % teams_count) as usize].clone(),
            );
            for _ in 0..hand_size {
                player.hand.add(draw_pile.draw().unwrap(), 1);
            }
            players.push(player);
        }
        let mut discard_pile: Vec<Card> = Vec::new();
        let up_card: Card = draw_pile.draw().unwrap();
        discard_pile.push(up_card);
        Self {
            draw_pile: draw_pile,
            discard_pile: discard_pile,
            players: players,
            teams_count: teams_count,
            players_per_team: players_per_team,
            finished: false,
            frozen: (up_card == Card::Joker || up_card == Card::Two),
            turn: TurnCounter::new(players_per_team * teams_count),
            curr_player_drawn: false,
        }
    }
    pub fn get_total_turns(&self) -> u16 {
        self.turn.total_turns
    }
    fn draw(&mut self) -> Card {
        let card: Option<Card> = self.draw_pile.draw();
        if card.is_none() {
            panic!("Drew a card from an empty deck");
        } else {
            card.unwrap()
        }
    }
    fn get_curr_player(&self) -> &Player {
        &self.players[self.turn.get() as usize]
    }
    fn get_curr_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.turn.get() as usize]
    }
    pub fn get_scores(&self) -> Vec<i16> {
        let mut scores: Vec<i16> = Vec::new();
        for i in 0..self.players_per_team {
            let mut score: i16 = 0;
            for j in 0..self.teams_count {
                score -= self.players[(i + j * self.players_per_team) as usize]
                    .hand
                    .get_score() as i16;
            }
            score += self.players[i as usize].board.lock().unwrap().get_score() as i16;
            scores.push(score);
        }
        for _ in 0..self.teams_count - 1 {
            for i in 0..self.players_per_team {
                scores.push(scores[i as usize]);
            }
        }
        scores
    }
    pub fn check_legal(&self, play: Play) -> bool {
        let player: &Player = self.get_curr_player();
        let hand: &Hand = &player.hand;
        let hand_size: u8 = hand.get_hand_size();
        let board: &Board = &player.board.lock().unwrap();
        match play {
            Play::GoOut => {
                self.curr_player_drawn
                    && board.get_num_canastas() >= 2
                    && hand_size - hand.get(Card::Three) == 1
                    && (hand.get(Card::Three) >= 3 || hand.get(Card::Three) == 0)
            }
            Play::PickupPile => {
                let subset_wild = {
                    if self.get_curr_player().hand.get(Card::Joker) > 0 {
                        Card::Joker
                    } else {
                        Card::Two
                    }
                };
                if self.discard_pile.len() == 0 {
                    return false;
                }
                let top_card = self.discard_pile[self.discard_pile.len() - 1];
                if self.curr_player_drawn {
                    return false;
                }
                if self.discard_pile.len() == 0 {
                    return false;
                }
                if let Some(_stack) = board.get(top_card) {
                    if hand_size + (self.discard_pile.len() as u8) - 1 >= 2 && !self.frozen {
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
            Play::PlaceWild(subset_card) => {
                let wild = {
                    if self.get_curr_player().hand.get(Card::Joker) > 0 {
                        Card::Joker
                    } else {
                        Card::Two
                    }
                };
                let card: Card = Card::from(subset_card);
                if !self.curr_player_drawn {
                    return false;
                }
                if hand_size <= 2 && board.get_num_canastas() < 2 {
                    if board.get_num_canastas() == 1 {
                        if let Some(stack) = board.get(card) {
                            if stack.get_total_count() == 6 {
                                return hand_size == 2
                                    && hand.get(Card::from(wild)) >= 1
                                    && stack.twos + stack.jokers + 1 <= stack.card_count;
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
            Play::Draw => !self.curr_player_drawn,
            Play::Discard(card) => {
                if !self.curr_player_drawn {
                    return false;
                }
                if hand.get(card) == 0 {
                    return false;
                }
                board.get_num_canastas() >= 2 || hand_size > 1
            }
            Play::Play(subset_card) => {
                let card: Card = Card::from(subset_card);
                if hand.get(card) == 0
                    || !self.curr_player_drawn
                    || hand_size == 1
                    || (hand_size == 2
                        && board.get_num_canastas() < 2
                        && !(board.get_num_canastas() == 1
                            && board.get(card).is_some()
                            && board.get(card).unwrap().get_total_count() == 6))
                {
                    return false;
                }
                if board.get(card).is_some() {
                    return true;
                }
                if hand.get(card) >= 3
                    || (hand.get(card) == 2 && hand.get(Card::Joker) + hand.get(Card::Two) >= 1)
                {
                    return hand_size >= 5 || (hand_size >= 4 && board.get_num_canastas() >= 2);
                }
                false
            }
        }
    }
    pub fn execute_play(&mut self, play: Play) {
        let mut knowledge_update: [i8; 14] = [0; 14];
        let current_player_index: u8 = self.turn.get();
        if DEBUG {
            let mut file = OpenOptions::new().append(true).open("debug.txt").unwrap();
            file.write_fmt(format_args!(
                "Turn: {}, Total Turns: {}\n",
                self.turn.get(),
                self.turn.total_turns
            ))
            .unwrap();
            if self.discard_pile.len() > 0 {
                file.write_fmt(format_args!(
                    "Top Discard Pile: {}\n",
                    self.discard_pile[self.discard_pile.len() - 1]
                ))
                .unwrap();
            } else {
                file.write_fmt(format_args!("Top Discard Pile: None\n"))
                    .unwrap();
            }
            let mut i = 0;
            for player in self.players.iter() {
                if i == current_player_index as usize {
                    file.write("> ".as_bytes()).unwrap();
                } else {
                    file.write("  ".as_bytes()).unwrap();
                }
                file.write_fmt(format_args!("Hand: {}\n", player.hand))
                    .unwrap();
                file.write_fmt(format_args!(
                    "  Board: {}\n\n",
                    player.board.lock().unwrap()
                ))
                .unwrap();
                i += 1;
            }
            file.write_fmt(format_args!("Action: {:?} \n\n", play))
                .unwrap();
        }
        match play {
            Play::GoOut => {
                let mut discard_card: Option<Card> = None;
                for card in Card::iterator() {
                    if self.get_curr_player().hand.get(*card) == 1 {
                        discard_card = Some(*card);
                        break;
                    }
                }
                if discard_card.is_none() {
                    panic!("Tried to go out but didn't have a card to discard");
                }
                self.discard_pile.push(discard_card.unwrap());
                self.get_curr_player_mut()
                    .hand
                    .remove(discard_card.unwrap(), 1);
                knowledge_update[discard_card.unwrap().get_index()] -= 1;
                self.turn.add();
                self.curr_player_drawn = false;
                if discard_card.unwrap() == Card::Joker || discard_card.unwrap() == Card::Two {
                    self.frozen = true;
                }
                let num_threes: u8 = self.get_curr_player().hand.get(Card::Three);
                self.get_curr_player_mut()
                    .hand
                    .remove(Card::Three, num_threes);
                self.finished = true;
                self.get_curr_player_mut().board.lock().unwrap().went_out = true;
            }
            Play::PickupPile => {
                let subset_wild = {
                    if self.get_curr_player().hand.get(Card::Joker) > 0 {
                        Card::Joker
                    } else {
                        Card::Two
                    }
                };
                let wild: Card = Card::from(subset_wild);
                self.curr_player_drawn = true;
                let top_card: Card = self.discard_pile[self.discard_pile.len() - 1];
                if self
                    .get_curr_player()
                    .board
                    .lock()
                    .unwrap()
                    .get(top_card)
                    .is_none()
                    || self.frozen
                {
                    if self.get_curr_player().hand.get(top_card) >= 2 {
                        self.get_curr_player_mut().hand.remove(top_card, 2);
                        knowledge_update[top_card.get_index()] -= 2;
                        self.get_curr_player_mut()
                            .board
                            .lock()
                            .unwrap()
                            .place_card(top_card, 3);
                    } else {
                        self.get_curr_player_mut().hand.remove(top_card, 1);
                        self.get_curr_player_mut().hand.remove(wild, 1);
                        knowledge_update[top_card.get_index()] -= 1;
                        knowledge_update[wild.get_index()] -= 1;
                        self.get_curr_player_mut()
                            .board
                            .lock()
                            .unwrap()
                            .place_card(top_card, 2);
                        if wild == Card::Joker {
                            self.get_curr_player_mut()
                                .board
                                .lock()
                                .unwrap()
                                .place_joker(top_card);
                        } else {
                            self.get_curr_player_mut()
                                .board
                                .lock()
                                .unwrap()
                                .place_two(top_card);
                        }
                    }
                } else {
                    self.get_curr_player_mut()
                        .board
                        .lock()
                        .unwrap()
                        .place_card(top_card, 1);
                }
                let mut new_cards: Vec<Card> = Vec::new();
                for card in self.discard_pile.iter() {
                    new_cards.push(*card);
                    knowledge_update[card.get_index()] += 1;
                }
                for card in new_cards.iter() {
                    self.get_curr_player_mut().hand.add(*card, 1);
                }
                self.get_curr_player_mut().hand.remove(top_card, 1);
                knowledge_update[top_card.get_index()] -= 1;
                self.frozen = false;
                self.discard_pile.clear();
            }
            Play::PlaceWild(subset_card) => {
                let subset_wild = {
                    if self.get_curr_player().hand.get(Card::Joker) > 0 {
                        Card::Joker
                    } else {
                        Card::Two
                    }
                };
                let card: Card = Card::from(subset_card);
                let wild: Card = Card::from(subset_wild);
                self.get_curr_player_mut().hand.remove(wild, 1);
                knowledge_update[wild.get_index()] -= 1;
                if wild == Card::Joker {
                    self.get_curr_player_mut()
                        .board
                        .lock()
                        .unwrap()
                        .place_joker(card);
                } else {
                    self.get_curr_player_mut()
                        .board
                        .lock()
                        .unwrap()
                        .place_two(card);
                }
            }
            Play::Draw => {
                self.curr_player_drawn = true;
                let new_card = self.draw();
                self.get_curr_player_mut().hand.add(new_card, 1);
                knowledge_update[new_card.get_index()] += 1;
            }
            Play::Discard(card) => {
                self.get_curr_player_mut().hand.remove(card, 1);
                knowledge_update[card.get_index()] -= 1;
                self.discard_pile.push(card);
                self.curr_player_drawn = false;
                self.turn.add();
                if card == Card::Joker || card == Card::Two {
                    self.frozen = true;
                }
                if self.draw_pile.is_empty() {
                    self.finished = true;
                }
            }
            Play::Play(subset_card) => {
                let card: Card = Card::from(subset_card);
                if self
                    .get_curr_player()
                    .board
                    .lock()
                    .unwrap()
                    .get(card)
                    .is_some()
                {
                    self.get_curr_player_mut()
                        .board
                        .lock()
                        .unwrap()
                        .place_card(card, 1);
                    self.get_curr_player_mut().hand.remove(card, 1);
                    knowledge_update[card.get_index()] -= 1;
                } else {
                    if self.get_curr_player().hand.get(card) >= 3 {
                        self.get_curr_player_mut().hand.remove(card, 3);
                        knowledge_update[card.get_index()] -= 3;
                        self.get_curr_player_mut()
                            .board
                            .lock()
                            .unwrap()
                            .place_card(card, 3);
                    } else {
                        if self.get_curr_player().hand.get(Card::Joker) >= 1 {
                            self.get_curr_player_mut().hand.remove(Card::Joker, 1);
                            self.get_curr_player_mut()
                                .board
                                .lock()
                                .unwrap()
                                .place_joker(card);
                            knowledge_update[Card::Joker.get_index()] -= 1;
                            self.get_curr_player_mut().hand.remove(card, 2);
                            self.get_curr_player_mut()
                                .board
                                .lock()
                                .unwrap()
                                .place_card(card, 2);
                            knowledge_update[card.get_index()] -= 2;
                        } else if self.get_curr_player().hand.get(Card::Two) >= 1 {
                            self.get_curr_player_mut().hand.remove(Card::Two, 1);
                            self.get_curr_player_mut()
                                .board
                                .lock()
                                .unwrap()
                                .place_two(card);
                            knowledge_update[Card::Two.get_index()] -= 1;
                            self.get_curr_player_mut().hand.remove(card, 2);
                            self.get_curr_player_mut()
                                .board
                                .lock()
                                .unwrap()
                                .place_card(card, 2);
                            knowledge_update[card.get_index()] -= 2;
                        } else {
                            panic!("Tried to play a card that a player didn't have");
                        }
                    }
                }
            }
        }
        for i in current_player_index + 1
            ..current_player_index + (self.teams_count * self.players_per_team)
        {
            let player_index = i % (self.teams_count * self.players_per_team);
            let mut knowledge_to_be_updated: [i8; 14] = self.players[player_index as usize]
                .knowledge[(self.teams_count * self.players_per_team
                + current_player_index
                + 1
                - 2
                - i) as usize];
            for j in 0..14 {
                knowledge_to_be_updated[j] += knowledge_update[j];
                self.players[player_index as usize].knowledge[(self.teams_count
                    * self.players_per_team
                    + current_player_index
                    + 1
                    - 2
                    - i) as usize] = knowledge_to_be_updated;
            }
        }
        if self.players[current_player_index as usize].hand.is_empty() {
            self.finished = true;
            self.get_curr_player_mut().board.lock().unwrap().went_out = true;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct GameState<const PLAYERS_PER_TEAM: u8, const TEAMS_COUNT: u8> {
    pub game: Game,
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Action {
    play: Play,
}

impl State for GameState<1, 2> {
    type A = Action;
    fn reward(&self) -> f64 {
        if self.game.finished {
            let mut out = self.game.get_scores()[((self.game.turn.get() + 1) % 2) as usize] as f64;
            for i in 0..self.game.teams_count {
                if (i + 1) % 2 != self.game.turn.get() {
                    out -= self.game.get_scores()[i as usize] as f64;
                }
            }
            return out;
        }
        0.0
    }
    fn actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        for play in Play::iterator() {
            if self.game.check_legal(*play) {
                actions.push(Action { play: *play });
            }
        }
        actions
    }
}

impl State for GameState<2, 2> {
    type A = Action;
    fn reward(&self) -> f64 {
        if self.game.finished {
            let mut out = self.game.get_scores()[((self.game.turn.get() + 3) % 4) as usize] as f64;
            for i in 0..self.game.teams_count {
                if (i + 1) % 4 != self.game.turn.get() {
                    out -= self.game.get_scores()[i as usize] as f64;
                }
            }
            return out;
        }
        0.0
    }
    fn actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();
        for play in Play::iterator() {
            if self.game.check_legal(*play) {
                actions.push(Action { play: *play });
            }
        }
        actions
    }
}

impl From<GameState<1, 2>> for [f32; 160] {
    fn from(state: GameState<1, 2>) -> Self {
        let mut output: [f32; 160] = [0.0; 160];
        //Discard Pile, 14 for each card, 14 for top card
        //TODO: Fix this insane hack
        let mut last_card: f32 = -1.0;
        for card in state.game.discard_pile.clone() {
            output[card.get_index()] += 1.0;
            last_card += 1.0;
            output[14 + card.get_index()] = last_card;
        }
        for i in 14..28 {
            if output[i] == last_card {
                output[i] = 1.0;
            } else {
                output[i] = 0.0;
            }
        }
        //Cards in hand
        for card in Card::iterator() {
            output[28 + card.get_index()] = state.game.get_curr_player().hand.get(*card) as f32;
        }
        // Has drawn
        if state.game.curr_player_drawn {
            output[42] = 1.0;
        } else {
            output[42] = 0.0;
        }
        //Cards in boards + Num canastas
        for i in 0..state.game.teams_count {
            for card in PlayableCardSubset::iterator() {
                output[43 + (i as usize) * 12 + (Card::from(*card).get_index() as usize) - 3] =
                    match state.game.players[i as usize]
                        .board
                        .lock()
                        .unwrap()
                        .get(Card::from(*card))
                    {
                        Some(stack) => stack.get_total_count() as f32,
                        None => 0.0,
                    };
            }
            output[43 + (i as usize) * 12 + 11] = state.game.players[i as usize]
                .board
                .lock()
                .unwrap()
                .get_num_canastas() as f32;
        }
        let mut curr: usize = 55 + ((state.game.teams_count * 12) as usize);
        //Hand Sizes
        for i in 0..state.game.teams_count * state.game.players_per_team {
            output[curr + i as usize] = state.game.players[i as usize].hand.get_hand_size() as f32;
        }
        curr += (state.game.teams_count * state.game.players_per_team) as usize;
        //Knowledge
        let knowledge = state.game.get_curr_player().knowledge.clone();
        for i in 0..state.game.teams_count * state.game.players_per_team - 1 {
            for j in 0..14 {
                output[curr + (i * 14 + j) as usize] = knowledge[i as usize][j as usize] as f32;
            }
        }
        output
    }
}

impl From<GameState<2, 2>> for [f32; 190] {
    fn from(state: GameState<2, 2>) -> Self {
        let mut output: [f32; 190] = [0.0; 190];
        //Discard Pile, 14 for each card, 14 for top card
        //TODO: Fix this insane hack
        let mut last_card: f32 = -1.0;
        for card in state.game.discard_pile.clone() {
            output[card.get_index()] += 1.0;
            last_card += 1.0;
            output[14 + card.get_index()] = last_card;
        }
        for i in 14..28 {
            if output[i] == last_card {
                output[i] = 1.0;
            } else {
                output[i] = 0.0;
            }
        }
        //Cards in hand
        for card in Card::iterator() {
            output[28 + card.get_index()] = state.game.get_curr_player().hand.get(*card) as f32;
        }
        // Has drawn
        if state.game.curr_player_drawn {
            output[42] = 1.0;
        } else {
            output[42] = 0.0;
        }
        //Cards in boards + Num canastas
        for i in 0..state.game.teams_count {
            for card in PlayableCardSubset::iterator() {
                output[43 + (i as usize) * 12 + (Card::from(*card).get_index() as usize) - 3] =
                    match state.game.players[i as usize]
                        .board
                        .lock()
                        .unwrap()
                        .get(Card::from(*card))
                    {
                        Some(stack) => stack.get_total_count() as f32,
                        None => 0.0,
                    };
            }
            output[43 + (i as usize) * 12 + 11] = state.game.players[i as usize]
                .board
                .lock()
                .unwrap()
                .get_num_canastas() as f32;
        }
        let mut curr: usize = 55 + ((state.game.teams_count * 12) as usize);
        //Hand Sizes
        for i in 0..state.game.teams_count * state.game.players_per_team {
            output[curr + i as usize] = state.game.players[i as usize].hand.get_hand_size() as f32;
        }
        curr += (state.game.teams_count * state.game.players_per_team) as usize;
        //Knowledge
        let knowledge = state.game.get_curr_player().knowledge.clone();
        for i in 0..state.game.teams_count * state.game.players_per_team - 1 {
            for j in 0..14 {
                output[curr + (i * 14 + j) as usize] = knowledge[i as usize][j as usize] as f32;
            }
        }
        output
    }
}

pub struct CanastaAgent<const PLAYERS_PER_TEAM: u8, const TEAMS_COUNT: u8> {
    pub state: Arc<Mutex<GameState<PLAYERS_PER_TEAM, TEAMS_COUNT>>>,
    pub player_id: u8,
}
impl Agent<GameState<1, 2>> for CanastaAgent<1, 2> {
    fn current_state(&self) -> GameState<1, 2> {
        let mut i = 0;
        loop {
            let state = self.state.lock().unwrap();
            if state.game.finished {
                return state.clone();
            }
            if state.game.turn.get() == self.player_id {
                //println!("{}", i);
                return state.clone();
            } else {
                i += 1;
                drop(state);
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        }
    }
    fn take_action(&mut self, action: &Action) -> () {
        let mut state = self.state.lock().unwrap();
        if state.game.finished {
            return;
        }
        if DEBUG {
            println!("Turn: {}", state.game.turn.get());
            if state.game.discard_pile.len() > 0 {
                println!(
                    "Top Discard Pile: {}",
                    state.game.discard_pile[state.game.discard_pile.len() - 1]
                );
            } else {
                println!("Top Discard Pile: None");
            }
            for player in state.game.players.iter() {
                println!("Hand: {}", player.hand);
                println!("Board: {}", player.board.lock().unwrap());
            }
            println!("Action: {:?}", (*action).play);
            println!("");
        }
        state.game.execute_play((*action).play);
    }
}

impl Agent<GameState<2, 2>> for CanastaAgent<2, 2> {
    fn current_state(&self) -> GameState<2, 2> {
        let mut i = 0;
        loop {
            let state = self.state.lock().unwrap();
            if state.game.finished {
                return state.clone();
            }
            if state.game.turn.get() == self.player_id {
                //println!("{}", i);
                return state.clone();
            } else {
                i += 1;
                drop(state);
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        }
    }
    fn take_action(&mut self, action: &Action) -> () {
        let mut state = self.state.lock().unwrap();
        if state.game.finished {
            return;
        }
        state.game.execute_play((*action).play);
    }
}

impl From<Action> for [f32; 39] {
    fn from(val: Action) -> Self {
        let mut output: [f32; 39] = [0.0; 39];
        match val.play {
            Play::Discard(Card::Joker) => output[0] = 1.0,
            Play::Discard(Card::Two) => output[1] = 1.0,
            Play::Discard(Card::Three) => output[2] = 1.0,
            Play::Discard(Card::Four) => output[3] = 1.0,
            Play::Discard(Card::Five) => output[4] = 1.0,
            Play::Discard(Card::Six) => output[5] = 1.0,
            Play::Discard(Card::Seven) => output[6] = 1.0,
            Play::Discard(Card::Eight) => output[7] = 1.0,
            Play::Discard(Card::Nine) => output[8] = 1.0,
            Play::Discard(Card::Ten) => output[9] = 1.0,
            Play::Discard(Card::Jack) => output[10] = 1.0,
            Play::Discard(Card::Queen) => output[11] = 1.0,
            Play::Discard(Card::King) => output[12] = 1.0,
            Play::Discard(Card::Ace) => output[13] = 1.0,
            Play::Draw => output[14] = 1.0,
            Play::PickupPile => output[15] = 1.0,
            Play::GoOut => output[16] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Four) => output[17] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Five) => output[18] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Six) => output[19] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Seven) => output[20] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Eight) => output[21] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Nine) => output[22] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Ten) => output[23] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Jack) => output[24] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Queen) => output[25] = 1.0,
            Play::PlaceWild(PlayableCardSubset::King) => output[26] = 1.0,
            Play::PlaceWild(PlayableCardSubset::Ace) => output[27] = 1.0,
            Play::Play(PlayableCardSubset::Four) => output[28] = 1.0,
            Play::Play(PlayableCardSubset::Five) => output[29] = 1.0,
            Play::Play(PlayableCardSubset::Six) => output[30] = 1.0,
            Play::Play(PlayableCardSubset::Seven) => output[31] = 1.0,
            Play::Play(PlayableCardSubset::Eight) => output[32] = 1.0,
            Play::Play(PlayableCardSubset::Nine) => output[33] = 1.0,
            Play::Play(PlayableCardSubset::Ten) => output[34] = 1.0,
            Play::Play(PlayableCardSubset::Jack) => output[35] = 1.0,
            Play::Play(PlayableCardSubset::Queen) => output[36] = 1.0,
            Play::Play(PlayableCardSubset::King) => output[37] = 1.0,
            Play::Play(PlayableCardSubset::Ace) => output[38] = 1.0,
        }
        output
    }
}

impl From<[f32; 39]> for Action {
    fn from(v: [f32; 39]) -> Self {
        // Find the index of the maximum value
        let max_index = v
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        match max_index {
            0 => Action {
                play: Play::Discard(Card::Joker),
            },
            1 => Action {
                play: Play::Discard(Card::Two),
            },
            2 => Action {
                play: Play::Discard(Card::Three),
            },
            3 => Action {
                play: Play::Discard(Card::Four),
            },
            4 => Action {
                play: Play::Discard(Card::Five),
            },
            5 => Action {
                play: Play::Discard(Card::Six),
            },
            6 => Action {
                play: Play::Discard(Card::Seven),
            },
            7 => Action {
                play: Play::Discard(Card::Eight),
            },
            8 => Action {
                play: Play::Discard(Card::Nine),
            },
            9 => Action {
                play: Play::Discard(Card::Ten),
            },
            10 => Action {
                play: Play::Discard(Card::Jack),
            },
            11 => Action {
                play: Play::Discard(Card::Queen),
            },
            12 => Action {
                play: Play::Discard(Card::King),
            },
            13 => Action {
                play: Play::Discard(Card::Ace),
            },
            14 => Action { play: Play::Draw },
            15 => Action {
                play: Play::PickupPile,
            },
            16 => Action { play: Play::GoOut },
            17 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Four),
            },
            18 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Five),
            },
            19 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Six),
            },
            20 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Seven),
            },
            21 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Eight),
            },
            22 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Nine),
            },
            23 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Ten),
            },
            24 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Jack),
            },
            25 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Queen),
            },
            26 => Action {
                play: Play::PlaceWild(PlayableCardSubset::King),
            },
            27 => Action {
                play: Play::PlaceWild(PlayableCardSubset::Ace),
            },
            28 => Action {
                play: Play::Play(PlayableCardSubset::Four),
            },
            29 => Action {
                play: Play::Play(PlayableCardSubset::Five),
            },
            30 => Action {
                play: Play::Play(PlayableCardSubset::Six),
            },
            31 => Action {
                play: Play::Play(PlayableCardSubset::Seven),
            },
            32 => Action {
                play: Play::Play(PlayableCardSubset::Eight),
            },
            33 => Action {
                play: Play::Play(PlayableCardSubset::Nine),
            },
            34 => Action {
                play: Play::Play(PlayableCardSubset::Ten),
            },
            35 => Action {
                play: Play::Play(PlayableCardSubset::Jack),
            },
            36 => Action {
                play: Play::Play(PlayableCardSubset::Queen),
            },
            37 => Action {
                play: Play::Play(PlayableCardSubset::King),
            },
            38 => Action {
                play: Play::Play(PlayableCardSubset::Ace),
            },
            _ => panic!("Invalid action index: {}", max_index),
        }
    }
}
