////////////////////////////////////////////////////////////////////////////////
//
// The state of the simulation
//   - validators
//   - config variables
//
////////////////////////////////////////////////////////////////////////////////

use super::config::Config;
use super::validator::Validator;

pub struct State {
    // we keep the config at hand
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
