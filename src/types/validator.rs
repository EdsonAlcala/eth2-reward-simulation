////////////////////////////////////////////////////////////////////////////////
//
// A simplified Eth2 validator
//
////////////////////////////////////////////////////////////////////////////////

pub struct Validator {
    pub balance: u64,
    pub effective_balance: u64,

    // these flags may represent something slightly different
    //   than the actual specification.
    // is_active implies that the validator was considered "active"
    //   during the previous epoch.
    pub is_active: bool,
    pub is_slashed: bool,
}

impl Validator {
    pub fn get_base_reward(&self, sqrt_total_active_balance: u64) -> u64 {
        self.effective_balance * crate::types::config::BASE_REWARD_FACTOR
            / sqrt_total_active_balance
            / crate::types::config::BASE_REWARDS_PER_EPOCH
    }
}
