////////////////////////////////////////////////////////////////////////////////
//
// - the state of the simulation
//   - validators
//   - config variables
//
////////////////////////////////////////////////////////////////////////////////

use super::config::Config;
use super::validator::Validator;

pub struct State {
    pub config: Config,
    pub validators: Vec<Validator>,
}

impl State {
    pub fn new() -> State {
        let config = Config::new();

        let number_of_validators = config.total_at_stake_initial / config.max_effective_balance;
        let mut validators = vec![];

        for _ in 0..number_of_validators {
            validators.push(Validator {
                // we just start with each validator active, non-slashed, and at 32 ETH
                balance: config.max_effective_balance,
                effective_balance: config.max_effective_balance,

                is_active: true,
                is_slashed: false,
            });
        }

        State {
            config: config,
            validators: validators,
        }
    }

    pub fn get_total_active_balance(&self) -> u64 {
        self.validators
            .iter()
            .map(
                |v: &Validator| {
                    if v.is_active {
                        v.effective_balance
                    } else {
                        0
                    }
                },
            )
            .sum()
    }

    pub fn get_total_active_validators(&self) -> u64 {
        self.validators
            .iter()
            .map(|v: &Validator| if v.is_active { 1 } else { 0 })
            .sum()
    }

    pub fn get_matching_balance(&self) -> u64 {
        // assumption
        //   https://github.com/hermanjunge/eth2-reward-simulation/blob/master/assumptions.md#epoch-processing
        self.validators
            .iter()
            .map(|v: &Validator| {
                if v.is_active && !v.is_slashed {
                    v.effective_balance
                } else {
                    0
                }
            })
            .sum()
    }
}
