////////////////////////////////////////////////////////////////////////////////
//
// simple simulator of rewards and penalties for Phase 0
//
////////////////////////////////////////////////////////////////////////////////

mod process_epoch;
mod types;
mod simulator_engine;

use config::Config;
use types::*;
use simulator_engine::start_simulation;
use std::thread;
use std::sync::mpsc;

fn main() {
    let config: Config = Config::new();
    let mut final_output = Output::new();
    let (tx, rx) = mpsc::channel();
    let job_count = 2; // TODO CALCULATE BASED ON PARAMETERS
    
    for i in 0..job_count {
        let tx = tx.clone();
        let config = config.clone();
        thread::spawn(move || {
            let result = start_simulation(config);
            tx.send(result).unwrap();
        });
    }  

    for _i in 0..job_count {
        let output_result = rx.recv().unwrap();

        if config.report_type == "monthly" {        
            if config.output_file_name.is_empty() {
                final_output.print_monthly_report(&config);
            } else {
                let monthly_report_chunk = output_result.get_monthly_report(&config);
                final_output.add_row_items(monthly_report_chunk);
                final_output.write_monthly_report_to_file(&config);
            }
        } else if config.report_type == "epoch" {
            final_output.print_epoch_report(&config);
        }
    }

    println!("Elements in output {}", final_output.get_rows().len());
}
