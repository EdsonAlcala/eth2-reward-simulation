////////////////////////////////////////////////////////////////////////////////
//
// - simulates relevant `process_epoch` ops during the state transition
//
//   https://github.com/hermanjunge/eth2-reward-simulation/blob/master/assumptions.md#epoch-processing
//
////////////////////////////////////////////////////////////////////////////////

mod process_rewards_and_penalties;

use std::time::Instant;

use crate::types::*;
use process_rewards_and_penalties::*;

pub fn process_epoch(pre_state: State, epoch_number: i32, output: &mut Output) -> State {
    // start to record
    let mut output_row = OutputRow::new();
    output_row.epoch_number = epoch_number;

    let epoch_processing_start = Instant::now();

    let mut post_state_validators = vec![];

    // pre-compute some values that remain constant throughout the epoch
    let total_active_balance = pre_state.get_total_active_balance();
    let total_active_validators = pre_state.get_total_active_validators();
    let matching_balance = pre_state.get_matching_balance();

    // pick the 32 block proposers
    let mut dice = Dice::new();
    let proposer_indices = dice.pick_epoch_proposers(&pre_state);

    for (validator_index, validator) in pre_state.validators.iter().enumerate() {
        let base_reward = validator.get_base_reward(total_active_balance);

        // SPEC: process_rewards_and_penalties
        let mut deltas = Deltas::new();
        get_attestation_deltas(
            &validator,
            &validator_index,
            base_reward,
            &pre_state.config,
            total_active_balance,
            total_active_validators,
            matching_balance,
            &proposer_indices,
            &mut deltas,
        );
        // apply_deltas() // TODO

        // SPEC: process_registry_updates
        // TODO

        // SPEC: process_final_updates
        // TODO

        // update values in output
        output_row.update(&deltas);

        post_state_validators.push(Validator {
            balance: 0,
            effective_balance: 0,
            is_active: false,
            is_slashed: false,
        });
    }

    // stop the timer, send the values to output
    output_row.time_elapsed = epoch_processing_start.elapsed().as_nanos();
    output.push(output_row);

    State {
        config: pre_state.config,
        validators: post_state_validators,
    }
}
