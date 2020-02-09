////////////////////////////////////////////////////////////////////////////////
//
// - config: UI (or command line) params
// - state: simple beacon chain state (with a reference to config)
// - validator: simple validator object
//
////////////////////////////////////////////////////////////////////////////////

pub struct Deltas {
    pub ffg_source_reward: u64,
    pub ffg_source_penalty: u64,
    /*
    ffg_target_reward: u64,
    ffg_target_penalty: u64,
    lmd_head_reward: u64,
    lmd_head_penalty: u64,
    proposer_reward: u64,
    attester_reward: u64,
    inactivity_penalty: u64,
    */
}

impl Deltas {
    pub fn new() -> Deltas {
        Deltas {
            ffg_source_reward: 0,
            ffg_source_penalty: 0,
            /*
            ffg_target_reward: 0,
            ffg_target_penalty: 0,
            lmd_head_reward: 0,
            lmd_head_penalty: 0,
            proposer_reward: 0,
            attester_reward: 0,
            inactivity_penalty: 0,
            */
        }
    }
}
