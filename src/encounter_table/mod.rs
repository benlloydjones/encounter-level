use serde::{Deserialize, Serialize};
use serde_json;

use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::process;

#[derive(Deserialize, Serialize)]
pub struct EncounterTable {
    pub easy: Vec<u32>,
    pub medium: Vec<u32>,
    pub hard: Vec<u32>,
    pub deadly: Vec<u32>,
}

pub enum Level {
    EASY,
    MEDIUM,
    HARD,
    DEADLY,
}

pub fn get_encounter_table(path: &Option<String>) -> EncounterTable {
    match path {
        Some(path_to_file) => get_encounter_table_from_file(path_to_file),
        None => get_encounter_table_from_binary(),
    }
}

fn get_encounter_table_from_file(path: &String) -> EncounterTable {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            eprintln!(
                "Encounter table json not found at: {}, please check path",
                &path
            );
            process::exit(1);
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(encounter_table) => encounter_table,
        Err(e) => {
            eprintln!(
                "Unable to construct Encounter Table from JSON, error received:\n{}",
                e
            );
            process::exit(1);
        }
    }
}

fn get_encounter_table_from_binary() -> EncounterTable {
    let encounter_table_json = r#"
    {
        "easy": [25, 50, 75, 125, 250, 300, 350, 450, 550, 600, 800, 1000, 1100, 1250, 1400, 1600, 2000, 2100, 2400, 2800],
        "medium": [50, 100, 150, 250, 500, 600, 750, 900, 1100, 1200, 1600, 2000, 2200, 2500, 2800, 3200, 3900, 4200, 4900, 5700],
        "hard": [75, 150, 225, 375, 750, 900, 1100, 1400, 1600, 1900, 2400, 3000, 3400, 3800, 4300, 4800, 5900, 6300, 7300, 8500],
        "deadly": [100, 200, 400, 500, 1100, 1400, 1700, 2100, 2400, 2800, 3600, 4500, 5100, 5700, 6400, 7200, 8800, 9500, 10900, 12700]
    }
    "#;
    match serde_json::from_str(encounter_table_json) {
        Ok(encounter_table) => encounter_table,
        Err(e) => {
            eprintln!(
                "Unable to construct Encounter Table from JSON, error received:\n{}",
                e
            );
            process::exit(1);
        }
    }
}
