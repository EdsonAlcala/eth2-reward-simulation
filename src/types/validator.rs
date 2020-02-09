////////////////////////////////////////////////////////////////////////////////
//
// - a simplified Eth2 validator
//
////////////////////////////////////////////////////////////////////////////////

use integer_sqrt::IntegerSquareRoot;

pub struct Validator {
    pub balance: u64,
    pub effective_balance: u64,
    pub is_active: bool,
    pub is_slashed: bool,
}

impl Validator {
    pub fn get_base_reward(&self, total_active_balance: u64) -> u64 {
        self.effective_balance * crate::types::config::BASE_REWARD_FACTOR
            / total_active_balance.integer_sqrt()
            / crate::types::config::BASE_REWARDS_PER_EPOCH
    }
}
