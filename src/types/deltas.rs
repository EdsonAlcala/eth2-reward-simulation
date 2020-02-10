////////////////////////////////////////////////////////////////////////////////
//
// - config: UI (or command line) params
// - state: simple beacon chain state (with a reference to config)
// - validator: simple validator object
//
////////////////////////////////////////////////////////////////////////////////

pub struct Deltas {
    pub head_ffg_reward: u64,
    pub head_ffg_penalty: u64,
    /*

    proposer_reward: u64,
    attester_reward: u64,
    inactivity_penalty: u64,
    */
}

impl Deltas {
    pub fn new() -> Deltas {
        Deltas {
            head_ffg_reward: 0,
            head_ffg_penalty: 0,
            /*

            proposer_reward: 0,
            attester_reward: 0,
            inactivity_penalty: 0,
            */
        }
    }
}
