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

#[derive(Debug,Clone)]
pub struct Config {
    // what kind of reports are we producing here?
    pub report_type: String,

    pub output_file_name: String,
    // what output format (json) // TODO add csv support
    pub output_format: String,
    
    // how many epochs we want to run?
    pub epochs: i32,

    // how much ETH we want to start with?
    pub total_at_stake_initial: u64,
    pub final_stake: u64,
    pub number_of_simulations: u64,

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
                Arg::with_name("initial-stake")
                    .short("i")
                    .long("initial-stake")
                    .value_name("initial_stake")
                    .help("Your initial stake in ETH"),
            )
            .arg(
                Arg::with_name("final-stake")
                    .short("s")
                    .long("final-stake")
                    .value_name("final_stake")
                    .help("Your final stake in ETH"),
            )
            .arg(
                Arg::with_name("epochs")
                    .short("e")
                    .long("epochs")
                    .value_name("t")
                    .help("Epochs to run"),
            )
            .arg(
                Arg::with_name("probability-online")
                    .short("p")
                    .long("probability-online")
                    .value_name("p")
                    .help("A value in [0,1] or multiple values separated by comma"),
            )
            .arg(
                Arg::with_name("report-type")
                    .short("r")
                    .long("report-type")
                    .value_name("type")
                    .help("Type of report (epoch, monthly)"),
            )
            .arg(
                Arg::with_name("output-file-name")
                    .short("o")
                    .long("output-file-name")
                    .value_name("output-file-name")
                    .help("Output results in a file")
            )
            .arg(
                Arg::with_name("output-format")
                    .short("f")
                    .long("output-format")
                    .value_name("output-format")
                    .help("Output results format (json, csv)")
            )
            .get_matches();

        let initial_stake = matches.value_of("initial-stake").unwrap_or("500000"); // TODO MAKE Enum
        let initial_stake: u64 = match initial_stake.trim().parse() {
            Ok(num) => num,
            Err(_) => 500_000,
        };
        
        if initial_stake < 500_000 { // TODO and multiple of 500,000
            panic!("initial_stake should be equal or greater than 500000")
        }

        let final_stake = matches.value_of("final-stake").unwrap_or("10000000"); // TODO MAKE Enum
        let final_stake: u64 = match final_stake.trim().parse() {
            Ok(num) => num,
            Err(_) => 10_000_000, // TODO, here should panic
        };
        
        if final_stake > 10_000_000 {
            panic!("final_stake should be less than 10 000 000")
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

        let probability_online_parameter = matches.value_of("probability-online").unwrap_or("0.99");       
        let probabilities_online = get_probabilities_from_string(probability_online_parameter);

        let number_of_simulations = (final_stake / initial_stake) * probabilities_online.len();
        println!("Number of simulations {}", number_of_simulations);

        let report_type = matches.value_of("report-type").unwrap_or("epoch");
        if report_type != "epoch" && report_type != "monthly" {
            panic!("report type only supports 'epoch' or 'monthly'");
        }

        // stick to 1.0 for now
        let probability_honest: f32 = 1.0;

        // pre-computation
        let exp_value_inclusion_prob = Config::get_exp_value_inclusion_prob(0.99);

        // output format
        let output_format = matches.value_of("output-format").unwrap_or("csv");
        if output_format != "json" && output_format != "csv" {
            panic!("the only available format is JSON and CSV");
        }

        let output_file_name = matches.value_of("output-file-name").unwrap_or("");
        
        Config {
            output_file_name: output_file_name.to_string(),
            output_format: output_format.to_string(),
            report_type: report_type.to_string(),
            epochs: epochs,
            total_at_stake_initial: initial_stake * 1_000_000_000,
            probability_online: probability_online,
            probability_honest: probability_honest,
            exp_value_inclusion_prob: exp_value_inclusion_prob,
            final_stake: final_stake * 1_000_000_000,
            number_of_simulations: number_of_simulations
        }
    }

    fn get_exp_value_inclusion_prob(p: f32) -> f32 {
        if p == 0.0 {
            p
        } else if p == 1.0 {
            p
        } else {
            p * p.ln() / (p - 1.00)
        }
    }

    fn get_probabilities_from_string(value: String) -> Vec<f32> {
        let mut probabilities_splitted = value.split(",");
        let mut probabilities_online: Vec<f32> = vec![];

        for probability in probabilities_splitted {
            let current_probability = match probability.trim().parse() {
                Ok(num) => num,
                Err(_) => 0.99,
            };

            if current_probability < 0.0 || current_probability > 1.0 {
                panic!("probability online should be in the interval [0,1]");
            }
            println!("Current probability {}", current_probability);

            probabilities_online.push(current_probability);
        }
        probabilities_online
    }

    // fn get_number_of_simulations(value: String) -> Vec<> {
    //     10
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_exp_value_inclusion_prob() {
        assert_eq!(
            0.00,
            truncate_two(Config::get_exp_value_inclusion_prob(0.0))
        );
        assert_eq!(
            1.00,
            truncate_two(Config::get_exp_value_inclusion_prob(1.0))
        );
        assert_eq!(
            0.99,
            truncate_two(Config::get_exp_value_inclusion_prob(0.99))
        );
        assert_eq!(
            0.97,
            truncate_two(Config::get_exp_value_inclusion_prob(0.95))
        );
        assert_eq!(
            0.94,
            truncate_two(Config::get_exp_value_inclusion_prob(0.9))
        );
    }

    fn truncate_two(number: f32) -> f32 {
        (number * 100.0).floor() / 100.0
    }

    #[test]
    fn it_should_get_probabilities_from_string(){
        let result = Output::get_probabilities_from_string("8,9,10");

        assert_eq!(result.len(), 3); 
    }
}
