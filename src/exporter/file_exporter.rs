use super::config::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::types::MonthlyReportRow;

pub struct FileExporter { // TODO To make it generic
    rows: Vec<MonthlyReportRow>
}

impl FileExporter {
    pub fn new() -> FileExporter {
        let rows: Vec<MonthlyReportRow> = vec![];

        FileExporter { rows: rows }
    }

    pub fn add_items(&mut self, rows: Vec<MonthlyReportRow>) {
        self.rows.extend_from_slice(&rows);
    }

    pub fn export_to_file(&self, config: &Config) {
        if config.output_format == "json" {       
            // TODO Move to a method i.e export to json file
            let file_name = format!("{}", config.output_file_name); // TODO if name contains json, don't add the ext, otherwise add it
            let path = Path::new(&file_name);
            let display = path.display();

            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why.description()),
                Ok(file) => file,
            };

            let json_data = serde_json::to_string(&self.rows)
                .expect("Couldn't convert to JSON");

            match file.write_all(json_data.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
                Ok(_) => println!("Successfully wrote to {}", display),
            }

        } else if config.output_format == "csv" {
            panic!("Not implemented method");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_base_reward() {


    }
}