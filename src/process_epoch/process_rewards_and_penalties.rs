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
    config: &Config,
    total_active_balance: u64,
    matching_balance: u64,
    proposer_indices: &Vec<usize>,
    base_reward: u64,
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
    }

    // inclusion reward
    // we don't punish not-online, not-honest, slashed, as we did above if applicable.
    // see https://notes.ethereum.org/@vbuterin/rkhCgQteN?type=view#Break-even-uptime
    //
    // - proposers
    //   we compute the amount, as we need it for the attester reward.
    //   since one validator per slot is chosen as a proposer,
    //   we randomly select 32 validators from the set of active validators.
    //
    //   note: while in this simulation the probability is uniform,
    //         in the specifications validators with low effective balance
    //         get lower chances to be elected.

    /*
    let proposer_reward_amount = base_reward / PROPOSER_REWARD_QUOTIENT;
    if validator.proposer {
        validator.increase_balance(proposer_reward_amount);
        // reset the flag!
        validator.proposer = false;

        println!("Did it!")
    }

    */
}
