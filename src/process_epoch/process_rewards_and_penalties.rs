////////////////////////////////////////////////////////////////////////////////
//
// - simulates relevant `process_rewards_and_penalties` ops during the state transition
//
//
//   def process_rewards_and_penalties(state: BeaconState) -> None:
//       if get_current_epoch(state) == GENESIS_EPOCH:
//           return
//       rewards, penalties = get_attestation_deltas(state)
//       for index in range(len(state.validators)):
//           increase_balance(state, ValidatorIndex(index), rewards[index])
//           decrease_balance(state, ValidatorIndex(index), penalties[index])
//
////////////////////////////////////////////////////////////////////////////////

use crate::types::*;

pub fn get_attestation_deltas(
    validator: &Validator,
    validator_index: &usize,
    base_reward: u64,
    config: &Config,
    total_active_balance: u64,
    total_active_validators: u64,
    matching_balance: u64,
    proposer_indices: &Vec<usize>,
    deltas: &mut Deltas,
) {
    // load our random component
    let mut dice = Dice::new();

    // simulate the eligibility check.
    // SPEC:
    //    eligible_validator_indices = [
    //      ValidatorIndex(index) for index, v in enumerate(state.validators)
    //      if is_active_validator(v, previous_epoch) or (v.slashed and previous_epoch + 1 < v.withdrawable_epoch)
    //
    if !validator.is_active {
        return;
    }

    // we will check now, if
    //   - the validator is slashed (property)
    //   - the validator was offline when it had to vote (probability)
    //   - the validator is not behaving honestly (probability)
    // if any of the conditions above is true, we penalize the validators
    // otherwise, we assume it is behaving honestly and error-free,
    //   and reward it for correct FFG source, FFG target and head.
    // note that we simplify and assume that the three matches
    //   happen or not together (hence the 3 coefficient)
    if validator.is_slashed
        || !dice.throw_dice(config.probability_online)
        || !dice.throw_dice(config.probability_honest)
    {
        deltas.head_ffg_penalty = 3 * base_reward;
    } else {
        // HACK
        // avoid integer overflows by "shaving" both balances
        let mb = matching_balance / 1000;
        let tab = total_active_balance / 1000;
        deltas.head_ffg_reward = 3 * base_reward * mb / tab;

        // inclusion rewards
        //
        // - proposer
        //
        //   - our validator is elegible for reward (not slashed, online, and honest)
        //   - our validator has been chosen as a proposer in this epoch
        //   - assume the attesters to this block are evenly distributed in committees
        //     across the epoch, that is, N/32 attesters per slot, hence per block.
        //   - consider only online, and honest attesters from these N/32.

        // we are using this value for the attester reward as well
        let proposer_reward_amount = base_reward / config.proposer_reward_quotient;

        if proposer_indices.contains(validator_index) {
            let number_of_attesters = total_active_validators / 32;
            let number_of_eligible_attesters = (number_of_attesters as f32
                * config.probability_online
                * config.probability_honest)
                .floor() as u64;

            deltas.proposer_reward = proposer_reward_amount * number_of_eligible_attesters;
        }

        // - attester
        //
        //   - our validator is elegible for reward (not slashed, online, and honest)
        //   - our validator cast its attestation
        //   - the reward is inversely proportional to the SLOT DIFFERENCE
        //     (a.k.a INCLUSION DELAY) on which it was included
        //   - for the sake of simplicity, assume the probability of inclusion
        //     to be the same probability of a validator to be online
        //   - the expected value of the probability tree can be modeled as:
        //     E = P + P(1-P)/2 + (P(1-P)^2)/3 + ... = P*ln(P)/(P-1)
        //     - which we can pre-compute, as such probability is given at start-up
        let maximum_attester_reward = base_reward - proposer_reward_amount;

        deltas.attester_reward =
            (maximum_attester_reward as f32 * config.exp_value_inclusion_prob).floor() as u64;
    }
}

//

/*

*/

// (get_attester_inclusion_reward is memoized)
// by the spec in `get_attestation_deltas`, the attestation inclusion reward
// is (B - B/proposer_reward_quotient) / inclusion_delay
// factoring, we have (B(PRQ - 1) / PRQ ) / inclusion_delay.
//
// If the probability to be included is p...

/*
        let aaa = base_reward
            * (proposer_reward_quotient - 1)
            * dice.get_attester_inclusion_reward(config.probability_online)
            / proposer_reward_quotient;
*/
