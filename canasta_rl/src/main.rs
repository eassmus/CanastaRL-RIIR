mod canastautil;
mod dqn;

use canasta_rl::strategy::terminate::TerminationStrategy;
use canastautil::GameState;
use dfdx::nn::ToDevice;
use dfdx::prelude::*;
use dqn::QNetworkDevice;
use rand::seq::SliceRandom;
use std::{fs::File, fs::OpenOptions, io::Write};
use std::{
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

impl TerminationStrategy<GameState<2, 2>> for CanastaTerminator {
    fn should_stop(&mut self, state: &GameState<2, 2>) -> bool {
        state.game.finished
    }
}

#[derive(PartialEq)]
enum RunType {
    Training,
    Testing,
}

fn training() {
    use canasta_rl::strategy::explore::RandomExploration;
    const PLAYERS_PER_TEAM: u8 = 2;
    const TEAMS_COUNT: u8 = 2;
    const DECKS: u8 = 2;
    const HAND_SIZE: u8 = 13;
    const NUM_EPISODES: u32 = 5000;
    const ACTION_SIZE: usize = 39;
    const STATE_SIZE: usize = 190;
    const INNER_SIZE: usize = 128;
    const NUM_ENVS: u8 = 2;
    let mut handles = Vec::new();
    let file = File::create("debug.txt").unwrap();
    drop(file);
    for env_num in 1..NUM_ENVS + 1 {
        let initial_state = Arc::new(Mutex::new(GameState::<PLAYERS_PER_TEAM, TEAMS_COUNT> {
            game: canastautil::Game::new(PLAYERS_PER_TEAM, TEAMS_COUNT, 2, HAND_SIZE),
        }));
        let done: Arc<Mutex<[bool; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize]>> = Arc::new(
            Mutex::new([false; (PLAYERS_PER_TEAM * TEAMS_COUNT) as usize]),
        );
        for handle_num in 0..PLAYERS_PER_TEAM * TEAMS_COUNT {
            let agent_intial_state = Arc::clone(&initial_state);
            let done_clone = Arc::clone(&done);
            let agentthread = thread::spawn(move || {
                let mut trainer = dqn::DQNAgentTrainer::<
                    GameState<PLAYERS_PER_TEAM, TEAMS_COUNT>,
                    STATE_SIZE,
                    ACTION_SIZE,
                    INNER_SIZE,
                >::new(1.0, 0.2);
                for ep in 1..NUM_EPISODES + 1 {
                    if handle_num == 0 {
                        let mut file = OpenOptions::new().append(true).open("debug.txt").unwrap();
                        file.write_fmt(format_args!("Ep: {}\n", ep)).unwrap();
                    }
                    let mut agent = canastautil::CanastaAgent {
                        state: Arc::clone(&agent_intial_state),
                        player_id: handle_num as u8,
                    };
                    trainer.train(
                        &mut agent,
                        &mut CanastaTerminator::new(),
                        &RandomExploration::new(),
                    );
                    done_clone.lock().unwrap()[handle_num as usize] = true;
                    if handle_num == 0 {
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
                        println!(
                            "Env: {}, Ep: {}, {:?}, {}",
                            env_num,
                            ep,
                            state.game.get_scores(),
                            state.game.turn.total_turns / 4
                        );
                        state.game =
                            canastautil::Game::new(PLAYERS_PER_TEAM, TEAMS_COUNT, DECKS, HAND_SIZE);
                        let mut done_lock = done_clone.lock().unwrap();
                        for i in 0..(PLAYERS_PER_TEAM * TEAMS_COUNT) as usize {
                            done_lock[i] = false;
                        }
                    }
                    while done_clone.lock().unwrap()[handle_num as usize] == true {
                        std::thread::sleep(std::time::Duration::from_nanos(1));
                    }
                }
                let learned_values = trainer.export_learned_values();
                let dev: Cpu = Default::default();
                (env_num, handle_num, learned_values.to_device(&dev).clone())
            });
            handles.push(agentthread);
            println!("Thread Spawned: {}, {}", env_num, handle_num);
        }
    }
    let dev: Cuda = Default::default();
    let mut models: Vec<(u8, u8, QNetworkDevice<STATE_SIZE, ACTION_SIZE, INNER_SIZE>)> = Vec::new();
    for handle in handles {
        let out = handle.join().unwrap();
        models.push((out.0, out.1, out.2.to_device(&dev)));
    }
}

const RUN_TYPE: RunType = RunType::Training;

fn main() {
    if RUN_TYPE == RunType::Training {
        training();
    } else if RUN_TYPE == RunType::Testing {
        play_random_game();
    }
}
