#[path = "canastautil.rs"] mod canastautil;
#[path = "dqn.rs"] mod dqn;
use canasta_rl::mdp::{Agent, State};
use rand::seq::SliceRandom;

const STATE_SIZE: usize = canastautil::STATE_SIZE;
const ACTION_SIZE: usize = canastautil::ACTION_SIZE;
const INNER_SIZE: usize = canastautil::INNER_SIZE;


const PLAYERS_PER_TEAM : u8 = canastautil::PLAYERS_PER_TEAM;
const TEAMS_COUNT : u8 = canastautil::TEAMS_COUNT;
const DECKS : u8 = canastautil::DECKS;
const HAND_SIZE : u8 = canastautil::HAND_SIZE;

pub fn play_random_game() -> Vec<i16> {
    let mut game = canastautil::Game::new(PLAYERS_PER_TEAM, TEAMS_COUNT, DECKS, HAND_SIZE);
    let mut possible_plays: Vec<canastautil::Play> = Vec::new();
    for play in canastautil::Play::iterator() {
        possible_plays.push(*play);
    }
    
    while !game.finished {
        //Shuffle PLAYS
        possible_plays.shuffle(&mut rand::thread_rng());
        let mut i = 0;
        while i < possible_plays.len() {
            let play = possible_plays[i];
            if game.check_legal(play) {
                game.execute_play(play);
                break;
            }
            i += 1;
        }
        if i == possible_plays.len() {
            panic!("No legal plays");
        }
    }
    game.get_scores()
}

pub fn test_models(models : [dqn::QNetworkDevice<STATE_SIZE, ACTION_SIZE, INNER_SIZE>; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize]) -> Vec<i16> {
    assert!(models.len() == (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize);
    let mut game = canastautil::Game::new(PLAYERS_PER_TEAM, TEAMS_COUNT, DECKS, HAND_SIZE);
    while !game.finished {
        let turn = game.turn.get();
        let model_to_play = models[turn as usize];
         
    }
    game.get_scores()
}