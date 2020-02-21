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
        let simulation_result = rx.recv().unwrap();

        if config.report_type == "monthly" { // TODO Refactor      
            if config.output_file_name.is_empty() {
                simulation_result.print_monthly_report(&config);
            } else {
                let monthly_report_chunk = simulation_result.get_monthly_report(&config);
                file_exporter.add_items(monthly_report_chunk);
            }
        } else if config.report_type == "epoch" {
            simulation_result.print_epoch_report(&config);
        }
    }

    if config.report_type == "monthly" { // TODO Refactor
        if !config.output_file_name.is_empty() {
            file_exporter.export_to_file(&config);
        }
    }
}
