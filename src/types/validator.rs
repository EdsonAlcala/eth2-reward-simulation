////////////////////////////////////////////////////////////////////////////////
//
// A simplified Eth2 validator
//
////////////////////////////////////////////////////////////////////////////////

use super::config;
use std::cmp;

pub struct Validator {
    pub balance: u64,
    pub effective_balance: u64,

    // these flags may represent something slightly different
    //   than the actual specification.
    // is_active implies that the validator was considered "active"
    //   during the previous epoch.
    // is_slashed self-explains.
    pub is_active: bool,
    pub is_slashed: bool,
}

impl Validator {
    pub fn get_base_reward(&self, sqrt_total_active_balance: u64) -> u64 {
        self.effective_balance * config::BASE_REWARD_FACTOR
            / sqrt_total_active_balance
            / config::BASE_REWARDS_PER_EPOCH
    }

    pub fn update_effective_balance(&mut self) {
        let half_increment = config::EFFECTIVE_BALANCE_INCREMENT / 2;

        if self.balance < self.effective_balance
            || self.effective_balance + 3 * half_increment < self.balance
        {
            self.effective_balance = cmp::min(
                self.balance - self.balance % config::EFFECTIVE_BALANCE_INCREMENT,
                config::MAX_EFFECTIVE_BALANCE,
            );
        }
    }
}

// We want to do some tests here?
