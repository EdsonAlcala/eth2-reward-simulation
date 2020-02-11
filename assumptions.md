## Simulation Assumptions

_version 1.0.0_

### Beacon chain state properties

* validators

### Validator properties

* balance
* effective_balance
* is_active
* is_slashed

### Validators start

* All of them active at epoch 0 (no gradually adding)
* All at 32 ETH balance and effective balance

### Fixed probabilities

* Online Probability
  * The probability a validator has to be online during the epoch.
  * For simplicity, we model the "inclusion probability" on this very number.

### Epoch processing

* No real consensus operations (justification and finalization)
  * `State::get_matching_balance`
    * All active validators, less the slashed ones, vote for the right source/target/head.
* By spec functionality
  * process_justification_and_finalization
    * NOT SIMULATED
  * process_rewards_and_penalties
    * head/FFG rewards/penalties
    * proposer and attestator microincentives
    * inactivity penalty
  * process_registry_updates
    * exit due to EJECTION_BALANCE
  * process_slashings
    * NOT IMPLEMENTED
  * NOTE: We are omitting slasher whistleblower/proposal rewards (for now!)
  * process_final_updates
    * update effective balances with hysteresis

