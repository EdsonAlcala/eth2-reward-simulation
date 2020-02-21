////////////////////////////////////////////////////////////////////////////////
//
// simple simulator of rewards and penalties for Phase 0
//
////////////////////////////////////////////////////////////////////////////////

mod process_epoch;
mod types;
mod simulator;
mod exporter;

use config::Config;
use types::*;
use simulator::start_simulation;
use std::thread;
use std::sync::mpsc;
use exporter::file_exporter::FileExporter;

fn main() {
    let config: Config = Config::new();
    let mut file_exporter = FileExporter::new();
    let (tx, rx) = mpsc::channel();

    for i in 0..config.number_of_simulations {
        let tx = tx.clone();
        let mut config = config.clone();
        config.total_at_stake_initial = (i + 1 ) * 1_000_000 * 1_000_000_000; // TODO FIX THIS
        let config_copy = config.clone();

        thread::spawn(move || {
            let simulation_result = start_simulation(config);

            if config_copy.report_type == "monthly" { // TODO Refactor      
                if config_copy.output_file_name.is_empty() {
                    simulation_result.print_monthly_report(&config_copy);
                } else {
                    let monthly_report_chunk = simulation_result.get_monthly_report(&config_copy);
                    tx.send(monthly_report_chunk).unwrap();
                }
            } else if config_copy.report_type == "epoch" {
                simulation_result.print_epoch_report(&config_copy);
            }
    
        });
    }  

    for _i in 0..config.number_of_simulations {
        let simulation_result = rx.recv().unwrap();
        file_exporter.add_items(simulation_result);
    }

    if config.report_type == "monthly" { // TODO Refactor
        if !config.output_file_name.is_empty() {
            file_exporter.export_to_file(&config); // I SHOULDN"T PASS THE WHOLE OBJECT
        }
    }
}
