mod canastautil;
mod dqn;

use canasta_rl::strategy::terminate::TerminationStrategy;
use canasta_rl::{
    mdp::{Agent, State},
    strategy::learn::QLearning,
};
use canastautil::GameState;
use rand::seq::SliceRandom;
use std::{
    num::IntErrorKind,
    sync::{Arc, Mutex},
    thread,
};

fn play_random_game() -> Vec<i16> {
    let mut game = canastautil::Game::new(2, 2, 2, 13);
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

pub struct CanastaTerminator {}

impl CanastaTerminator {
    pub fn new() -> CanastaTerminator {
        CanastaTerminator {}
    }
}

impl TerminationStrategy<GameState<1, 2>> for CanastaTerminator {
    fn should_stop(&mut self, state: &GameState<1, 2>) -> bool {
        state.game.finished
    }
}

fn play_model_game() -> Vec<i16> {
    let mut game = canastautil::Game::new(2, 2, 2, 13);
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

fn main() {
    use canasta_rl::strategy::explore::RandomExploration;
    const PLAYERS_PER_TEAM: u8 = 1;
    const TEAMS_COUNT: u8 = 2;
    const NUM_EPISODES: u32 = 1000;
    let initial_state = Arc::new(Mutex::new(GameState {
        game: canastautil::Game::new(PLAYERS_PER_TEAM, TEAMS_COUNT, 2, 15),
    }));
    let done: Arc<Mutex<[bool; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize]>> = Arc::new(Mutex::new(
        [false; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize],
    ));
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for handle_id in 0..PLAYERS_PER_TEAM * TEAMS_COUNT {
        let agent_intial_state = Arc::clone(&initial_state);
        let done_clone = Arc::clone(&done);
        let agentthread = thread::spawn(move || {
            let agent_num = handle_id;
            let mut trainer = dqn::DQNAgentTrainer::<
                GameState<PLAYERS_PER_TEAM, TEAMS_COUNT>,
                160,
                51,
                128,
            >::new(1.0, 0.2);
            for ep in 1..NUM_EPISODES + 1 {
                let mut agent = canastautil::CanastaAgent {
                    state: Arc::clone(&agent_intial_state),
                    player_id: handle_id as u8,
                };
                trainer.train(
                    &mut agent,
                    &mut CanastaTerminator::new(),
                    &RandomExploration::new(),
                );
                println!("Agent: {}, Ep: {}", agent_num, ep);
                done_clone.lock().unwrap()[agent_num as usize] = true;
                if agent_num == 0 {
                    while {
                        let done_lock = done_clone.lock().unwrap();
                        let mut out: bool = false;
                        for i in 0..(PLAYERS_PER_TEAM * TEAMS_COUNT) as usize {
                            if done_lock[i] == false {
                                out = true;
                            }
                        }
                        out
                    } {
                        thread::sleep(std::time::Duration::from_nanos(1));
                    }
                    let mut state = agent_intial_state.lock().unwrap();
                    println!("{:?}, {}", state.game.get_scores(), state.game.turn.total_turns);
                    state.game = canastautil::Game::new(1, 2, 2, 15);
                    let mut done_lock = done_clone.lock().unwrap();
                    for i in 0..(PLAYERS_PER_TEAM * TEAMS_COUNT) as usize {
                        done_lock[i] = false;
                    }
                }
                while done_clone.lock().unwrap()[agent_num as usize] == true {
                    std::thread::sleep(std::time::Duration::from_nanos(1));
                }
            }
        });
        handles.push(agentthread);
        println!("Thread Spawned: {}", handle_id);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
