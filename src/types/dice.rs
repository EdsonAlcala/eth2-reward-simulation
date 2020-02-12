////////////////////////////////////////////////////////////////////////////////
//
// - Dice helps with randomness
//
////////////////////////////////////////////////////////////////////////////////

use super::state::State;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Dice {
    rng: ThreadRng,
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            rng: rand::thread_rng(),
        }
    }

    pub fn throw_dice(&mut self, probability: f32) -> bool {
        probability > self.rng.gen()
    }

    pub fn pick_epoch_proposers(&mut self, state: &State) -> Vec<usize> {
        let mut proposer_indices = vec![];

        let n = state.validators.len();
        let proposers_per_epoch = 32;
        let max_effective_balance = 32_000_000_000;
        let max_random_byte = 255;

        if state.get_total_active_validators() < 32 {
            panic!("not enough active validators");
        }

        loop {
            if proposer_indices.len() == proposers_per_epoch {
                break;
            }

            let candidate_index = self.rng.gen_range(0, n);

            if state.validators[candidate_index].is_slashed
                || !state.validators[candidate_index].is_active
                || proposer_indices.contains(&candidate_index)
            {
                continue;
            }

            // effective balance bias on proposer election
            let random_byte = self.rng.gen_range(0, 255);
            if state.validators[candidate_index].effective_balance * max_random_byte
                >= random_byte * max_effective_balance
            {
                proposer_indices.push(candidate_index);
            } else {
                continue;
            }
        }

        proposer_indices
    }
}
