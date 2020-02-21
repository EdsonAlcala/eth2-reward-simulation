use super::process_epoch::process_epoch;
use super::types::*;
// use config::Config;

pub fn start_simulation(config: Config) -> Output {
    // println!("start_simulation stake value {}", config.total_at_stake_initial);
    let mut state = State::new(config);
    let mut output = Output::new();

    for i in 0..state.config.epochs {
        state = process_epoch(state, i, &mut output);
    }

    output
}