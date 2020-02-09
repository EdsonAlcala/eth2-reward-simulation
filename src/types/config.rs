////////////////////////////////////////////////////////////////////////////////
//
// - specs constants and simulation variables
//
////////////////////////////////////////////////////////////////////////////////

pub const MAX_EFFECTIVE_BALANCE: u64 = 32_000_000_000;
pub const BASE_REWARD_FACTOR: u64 = 64;
pub const BASE_REWARDS_PER_EPOCH: u64 = 4;
pub const PROPOSER_REWARD_QUOTIENT: u64 = 8;

pub struct Config {
    // how many epochs we want to run?
    pub epochs: i32,

    // how much ETH we want to start with?
    // TODO: stake to vary over the simulation
    pub total_at_stake_initial: u64,

    // probabilities of any validator
    // TODO: we would want to have something
    //       more like distributions in the future
    pub probability_online: f32,
    pub probability_honest: f32,

    // the constants
    pub max_effective_balance: u64,
    pub base_reward_factor: u64,
    pub base_rewards_per_epoch: u64,
    pub proposer_reward_quotient: u64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            epochs: 1, // We want 82_125 = (60 * 60 * 24 * 365)/(12 * 32)

            total_at_stake_initial: 500_000_000_000_000,

            probability_online: 0.90,
            probability_honest: 1.00,

            max_effective_balance: MAX_EFFECTIVE_BALANCE,
            base_reward_factor: BASE_REWARD_FACTOR,
            base_rewards_per_epoch: BASE_REWARDS_PER_EPOCH,
            proposer_reward_quotient: PROPOSER_REWARD_QUOTIENT,
        }
    }
}
