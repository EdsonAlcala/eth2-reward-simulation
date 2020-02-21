////////////////////////////////////////////////////////////////////////////////
//
// The state of the simulation
//   - validators
//   - config variables
//
////////////////////////////////////////////////////////////////////////////////

use super::config;
use super::validator::Validator;

pub struct State {
    // we keep the config at hand
    pub config: config::Config,

    pub validators: Vec<Validator>,
}

impl State {
    pub fn new(config: config::Config) -> State {
        let number_of_validators = config.total_at_stake_initial / config::MAX_EFFECTIVE_BALANCE;
        let mut validators = vec![];

        for _ in 0..number_of_validators {
            validators.push(Validator {
                balance: config::MAX_EFFECTIVE_BALANCE,
                effective_balance: config::MAX_EFFECTIVE_BALANCE,

                is_active: true,
                is_slashed: false,
            });
        }

        State {
            config: config,
            validators: validators,
        }
    }

    pub fn get_total_staked_balance(&self) -> u64 {
        self.validators.iter().map(|v: &Validator| v.balance).sum()
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

    pub fn get_max_balance(&self) -> u64 {
        self.validators
            .iter()
            .map(|v: &Validator| v.balance)
            .fold(0, std::cmp::max)
    }

    pub fn get_min_balance(&self) -> u64 {
        self.validators
            .iter()
            .map(|v: &Validator| v.balance)
            .fold(std::u64::MAX, std::cmp::min)
    }
}

// TODO: Test
// - State::new()
// - State::get_total_active_balance()
// - State::get_total_active_balance()
// - State::get_total_active_validators()
// - State::get_matching_balance()
// - State::get_max_balance()
// - State::get_min_balance()
