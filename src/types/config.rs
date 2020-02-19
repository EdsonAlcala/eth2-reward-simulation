////////////////////////////////////////////////////////////////////////////////
//
// Specs constants and simulation variables
//
////////////////////////////////////////////////////////////////////////////////

extern crate clap;

use clap::{App, Arg};

pub const MAX_EFFECTIVE_BALANCE: u64 = 32_000_000_000;
pub const BASE_REWARD_FACTOR: u64 = 64;
pub const BASE_REWARDS_PER_EPOCH: u64 = 4;
pub const PROPOSER_REWARD_QUOTIENT: u64 = 8;
pub const EFFECTIVE_BALANCE_INCREMENT: u64 = 1_000_000_000;

#[derive(Debug)]
pub struct Config {
    // what kind of reports are we producing here?
    pub printing_output: String,

    // how many epochs we want to run?
    pub epochs: i32,

    // how much ETH we want to start with?
    pub total_at_stake_initial: u64,

    // probabilities of any validator
    pub probability_online: f32,
    pub probability_honest: f32,

    // pre-computation
    pub exp_value_inclusion_prob: f32,
}

impl Config {
    pub fn new() -> Config {
        // parse command line options
        let matches = App::new("Eth2 Reward Simulator")
            .arg(
                Arg::with_name("initial_stake")
                    .short("i")
                    .long("initial_stake")
                    .value_name("ETH")
                    .help("Your initial stake in ETH"),
            )
            .arg(
                Arg::with_name("epochs")
                    .short("e")
                    .long("epochs")
                    .value_name("t")
                    .help("Epochs to run"),
            )
            .arg(
                Arg::with_name("probability_online")
                    .short("p")
                    .long("probability_online")
                    .value_name("p")
                    .help("A value in [0,1]"),
            )
            .arg(
                Arg::with_name("printing_output")
                    .short("r")
                    .long("printing_output")
                    .value_name("option")
                    .help("Type of report (epoch, monthly)"),
            )
            .get_matches();

        let initial_stake = matches.value_of("initial_stake").unwrap_or("500000");
        let initial_stake: u64 = match initial_stake.trim().parse() {
            Ok(num) => num,
            Err(_) => 500_000,
        };
        if initial_stake < 500_000 {
            panic!("initial_stake should be equal or greater than 500000")
        }

        // ideal default: 81_125 = (60 * 60 * 24 * 365)/(12 * 32)
        // current default 10
        let epochs = matches.value_of("epochs").unwrap_or("10");
        let epochs: i32 = match epochs.trim().parse() {
            Ok(num) => num,
            Err(_) => 10,
        };
        if epochs < 1 {
            panic!("epoch should be a positive integer")
        }

        let probability_online = matches.value_of("probability_online").unwrap_or("0.99");
        let probability_online: f32 = match probability_online.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.99,
        };
        if probability_online < 0.0 || probability_online > 1.0 {
            panic!("probability_online should be in the interval [0,1]");
        }

        let printing_output = matches.value_of("printing_output").unwrap_or("epoch");
        if printing_output != "epoch" && printing_output != "monthly" {
            panic!("printing_output only supports 'epoch' or 'monthly'");
        }

        // stick to 1.0 for now
        let probability_honest: f32 = 1.0;

        // pre-computation
        let exp_value_inclusion_prob = Config::get_exp_value_inclusion_prob(probability_online);

        Config {
            printing_output: printing_output.to_string(),
            epochs: epochs,
            total_at_stake_initial: initial_stake * 1_000_000_000,
            probability_online: probability_online,
            probability_honest: probability_honest,
            exp_value_inclusion_prob: exp_value_inclusion_prob,
        }
    }

    fn get_exp_value_inclusion_prob(p: f32) -> f32 {
        p * p.ln() / (p - 1.00)
    }
}

// TODO
// - A control variable for printing output
// - Tests
//   - edge cases for get_exp_value_inclusion_prob() (0, 1, values outside the interval)
//   - Config::new()
