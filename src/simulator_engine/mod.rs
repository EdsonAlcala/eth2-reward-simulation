use super::process_epoch::process_epoch;
use super::types::*;

pub fn start_simulation(config: config::Config) -> Output {
    let mut state = State::new(config);
    let mut output = Output::new();

    for i in 0..state.config.epochs {
        state = process_epoch(state, i, &mut output);
    }

    output
}