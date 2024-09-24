mod canastautil;
mod dqn;
mod model_eval;

use canasta_rl::strategy::terminate::TerminationStrategy;
use canastautil::GameState;
use dfdx::nn::ToDevice;
use dfdx::prelude::*;
use dqn::QNetworkDevice;
use std::{fs::File, fs::OpenOptions, io::Write};
use std::{
    sync::{Arc, Mutex},
    thread,
};

use canasta_rl::strategy::explore::RandomExploration;

const ACTION_SIZE: usize = canastautil::ACTION_SIZE;
const STATE_SIZE: usize = canastautil::STATE_SIZE;
const INNER_SIZE: usize = canastautil::INNER_SIZE;

const PLAYERS_PER_TEAM: u8 = canastautil::PLAYERS_PER_TEAM;
const TEAMS_COUNT: u8 = canastautil::TEAMS_COUNT;
const DECKS: u8 = canastautil::DECKS;
const HAND_SIZE: u8 = canastautil::HAND_SIZE;

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
    const NUM_EPISODES_PER_EVAL: u32 = 25;
    const NUM_EVAL_EPISODES: u32 = 50;
    const NUM_ENVS: u8 = 6;
    const TESTING_GAMES: u32 = 10;
    const DEBUG_FILE: bool = true;
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
                for eval_ep in 1..NUM_EVAL_EPISODES + 1 {
                    for ep in 1..NUM_EPISODES_PER_EVAL + 1 {
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
                                (eval_ep - 1) * NUM_EPISODES_PER_EVAL + ep,
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
                    //run some testing
                    let mut scores : Vec<i16> = Vec::new();
                    for _ in 0..TESTING_GAMES {
                        let results = model_eval::test_model(trainer.export_learned_values());
                        scores.push(results[0] - results[1]);
                    }
                    println!("TESTING RESULT {} : Env: {}, Agent: {}, Avg: {} \n", eval_ep, env_num, handle_num + 1, scores.iter().sum::<i16>() / TESTING_GAMES as i16);
                    let mut file = OpenOptions::new().append(true).open("debug.txt").unwrap();
                    if DEBUG_FILE {
                        file.write_fmt(format_args!("TESTING RESULT {} : Env: {}, Agent: {}, Avg: {} \n", eval_ep, env_num, handle_num + 1, scores.iter().sum::<i16>() / TESTING_GAMES as i16)).unwrap();
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
        let scores = model_eval::play_random_game();
        println!("{:?}", scores);
    }
}
