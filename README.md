
### Rewards and Penalties Simulator for Ethereum 2.0

* [Simulation Assumptions](assumptions.md)

#### Run this simulation

1. [Get rust](https://www.rust-lang.org/learn/get-started)
2. You can just run the simulation in _debug_ mode (slow), or compile and run it (good)

```bash
# Just run the simulation
cargo run

# Compile and run the simulation
cargo build --release && ./target/release/simulation

```

Note: `cargo` is the Rust package manager.

#### Command line flags

We haven't enabled them yet. Sorry.

#### TODO!

* test Validator::get_base_reward() and Validator::update_effective_balance()
--------------------------------
* edit proposer incentives assumptions
* edit attester incenties assumptions
* add the inactivity penalty, edit assumptions
* add the slasher reward/punishment
  * https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#slash_validator
  * https://github.com/ethereum/eth2.0-specs/blob/dev/specs/phase0/beacon-chain.md#slashings
  * document assumptions
* have the mechanism to remove slashed validators over time
  * we are just marking the flag
* validator withdrawal (forceful)
* add command line option parameters for
  * initial stake (check, min is 500,000 ETH)
  * probability online (check, [0,1], excluding)
  * probability honest (check, [0,1], excluding)
--------------------------------
* add ETHUSD as parameter and then return reward in USD
* pick a specific (or random) validator, and follow what's going on with it over a whole simulation.
--------------------------------
* make this README a decent one
--------------------------------
* do the blogpost
  * https://docs.google.com/document/d/10_z2YudaBBWfqgIFAip44TA6PJ049R0NPAyAsGBsMH8/edit
* tests, coverage. and blah blah.

#### License

* Apache 2.0.
