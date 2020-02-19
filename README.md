
### Rewards and Penalties Simulator for Ethereum 2.0

* [Simulation Assumptions](assumptions.md)

#### Run this simulation

1. [Get Rust](https://www.rust-lang.org/learn/get-started)
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

#### Features

##### UX

#### Features
##### Balance

- [x] FFG rewards and penalties
- [x] Proposer and attester incentives

##### Documentation

- [x] [Assumptions](/assumptions.md)

#### TODO!
##### UX
- [ ] Command option parameters
  - [ ] Initial stake
  - [ ] Online probability
  - [ ] Honesty probability
  - [ ] Epochs to run
- [ ] Add ETHUSD as parameter and then return reward in USD
- [ ] One special validator with different initial parameters

##### Balance
- [ ] Inactivity penalty
- [ ] Slashing
  - [ ] Whistleblower reward
  - [ ] Proposer reward
  - [ ] Slashed validator penalty
  - [ ] _Midway penalty_

##### Validator
- [ ] Validator exit
  - [ ] Balance ejection
  - [ ] Slasher ejection
  - [ ] Voluntary exit
- [ ] Validator activation
- [ ] Account _top up_

##### Tests
- [ ] Attester incentives
- [ ] process_epoch()
  - [ ] probability 1.0
  - [ ] probability 0.9
  - [ ] probability 0.5

##### Docs
- [ ] Inactivity penalty: assumptions.md
- [ ] Slashing: assumptions.md

#### License

* Apache 2.0.
