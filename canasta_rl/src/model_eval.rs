#[path = "canastautil.rs"]
mod canastautil;
#[path = "dqn.rs"]
mod dqn;
use canasta_rl::mdp::{Agent, State};
use canastautil::Action;
use dqn::QNetworkDevice;
use rand::seq::SliceRandom;

const STATE_SIZE: usize = canastautil::STATE_SIZE;
const ACTION_SIZE: usize = canastautil::ACTION_SIZE;
const INNER_SIZE: usize = canastautil::INNER_SIZE;

const PLAYERS_PER_TEAM: u8 = canastautil::PLAYERS_PER_TEAM;
const TEAMS_COUNT: u8 = canastautil::TEAMS_COUNT;
const DECKS: u8 = canastautil::DECKS;
const HAND_SIZE: u8 = canastautil::HAND_SIZE;

trait CanastaAgent {
    fn get_action(
        &self,
        state: &canastautil::GameState<PLAYERS_PER_TEAM, TEAMS_COUNT>,
    ) -> canastautil::Play;
}

#[derive(Clone, Copy)]
struct RandomAgent {}

impl CanastaAgent for RandomAgent {
    fn get_action(
        &self,
        state: &canastautil::GameState<PLAYERS_PER_TEAM, TEAMS_COUNT>,
    ) -> canastautil::Play {
        let mut possible_plays: Vec<canastautil::Play> = Vec::new();
        for play in canastautil::Play::iterator() {
            if state.game.check_legal(*play) {
                possible_plays.push(*play);
            }
        }
        //choose randomly from possible_plays without shuffling
        let chosen_play = possible_plays.choose(&mut rand::thread_rng());
        match chosen_play {
            Some(play) => return *play,
            None => panic!("No legal plays"),
        }
    }
}

struct TrainedAgent {
    trainer: dqn::DQNAgentTrainer<
        canastautil::GameState<PLAYERS_PER_TEAM, TEAMS_COUNT>,
        STATE_SIZE,
        ACTION_SIZE,
        INNER_SIZE,
    >,
}

impl TrainedAgent {
    pub fn new(model: QNetworkDevice<STATE_SIZE, ACTION_SIZE, INNER_SIZE>) -> Self {
        let mut trainer = dqn::DQNAgentTrainer::new(0.99, 1e-3);
        trainer.import_model(model);
        Self { trainer: trainer }
    }
}

impl CanastaAgent for TrainedAgent {
    fn get_action(
        &self,
        state: &canastautil::GameState<PLAYERS_PER_TEAM, TEAMS_COUNT>,
    ) -> canastautil::Play {
        let best_action = self.trainer.best_action(state);
        match best_action {
            Some(action) => return Action::from(action).play,
            None => panic!("No legal plays"),
        }
    }
}

fn run_game(agents: [&dyn CanastaAgent; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize]) -> Vec<i16> {
    let mut game = canastautil::Game::new(PLAYERS_PER_TEAM, TEAMS_COUNT, DECKS, HAND_SIZE);
    let mut possible_plays: Vec<canastautil::Play> = Vec::new();
    for play in canastautil::Play::iterator() {
        possible_plays.push(*play);
    }

    while !game.finished {
        let state = canastautil::GameState { game: game.clone() };
        let action = agents[game.turn.get() as usize].get_action(&state);
        game.execute_play(action);
    }
    game.get_scores()
}

pub fn play_random_game() -> Vec<i16> {
    run_game([&RandomAgent {}; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize])
}

pub fn test_model(raw_model: QNetworkDevice<STATE_SIZE, ACTION_SIZE, INNER_SIZE>) -> Vec<i16> {
    let model = TrainedAgent::new(raw_model);
    let mut models: [&dyn CanastaAgent; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize] = [&RandomAgent {}; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize];
    models[0] = &model;    
    models[2] = &model;
    run_game(models)
}
