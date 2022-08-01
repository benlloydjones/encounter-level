use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;

use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::process;

/// Simple program for working out an encounter level for 5e DnD
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///space or comma separated levels of the adventurers in the encounter
    #[clap(short, long, value_parser)]
    levels: String,
    ///difficulty either (e)asy, (m)edium, (h)ard or (d)eadly
    #[clap(short, long, value_parser)]
    difficulty: Option<String>,
    ///path to encounter table (defaults to encounter_table.json in local folder)
    #[clap(short, long, value_parser)]
    path: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct EncounterTable {
    pub easy: Vec<u32>,
    pub medium: Vec<u32>,
    pub hard: Vec<u32>,
    pub deadly: Vec<u32>,
}

enum Level {
    EASY,
    MEDIUM,
    HARD,
    DEADLY,
}

fn main() {
    let args = Args::parse();
    let levels = get_levels(&args);
    let difficulty = get_difficulty(&args);
    let path_to_encounter_table = get_path(&args);
    let encounter_table = get_encounter_table(&path_to_encounter_table);
    outcome(&encounter_table, &levels, &difficulty);
}

fn get_path(args: &Args) -> String {
    match &args.path {
        Some(path) => String::from(path),
        None => String::from("./encounter_table.json"),
    }
}

fn get_encounter_table(path: &String) -> EncounterTable {
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

fn outcome(encounter_table: &EncounterTable, levels: &Vec<u8>, difficulty: &Option<Level>) {
    let outcome = match difficulty {
        Some(difficulty_level) => xp_for_level(&encounter_table, &levels, &difficulty_level),
        None => format!(
            "{}{}{}{}",
            xp_for_level(&encounter_table, &levels, &Level::EASY),
            xp_for_level(&encounter_table, &levels, &Level::MEDIUM),
            xp_for_level(&encounter_table, &levels, &Level::HARD),
            xp_for_level(&encounter_table, &levels, &Level::DEADLY)
        ),
    };

    println!("{}", outcome);
}

fn xp_for_level(encounter_table: &EncounterTable, levels: &Vec<u8>, difficulty: &Level) -> String {
    let mut total = 0;
    let (encounter_list, diff_str) = match difficulty {
        Level::EASY => (&encounter_table.easy, "Easy"),
        Level::MEDIUM => (&encounter_table.medium, "Medium"),
        Level::HARD => (&encounter_table.hard, "Hard"),
        Level::DEADLY => (&encounter_table.deadly, "Deadly"),
    };

    for level in levels {
        total = encounter_list[usize::from(level - 1)] + total;
    }

    format!("{}: {} xp\n", diff_str, total)
}

fn get_levels(args: &Args) -> Vec<u8> {
    args.levels
        .split(&[',', ' '][..])
        .map(|n| match n.parse::<u8>() {
            Ok(n) if n >= 1 && n <= 20 => n,
            Ok(n) => {
                eprintln!("All provided levels must be between 1 and 20 inclusive, received:\n{}", n);
                process::exit(1);
            },
            Err(_) => {
                eprintln!("Please check that levels contains only integers separated by commas or spaces, received:\n{}", &args.levels);
                process::exit(1);
            }
        })
        .collect()
}

fn get_difficulty(args: &Args) -> Option<Level> {
    match &args.difficulty {
        Some(letter) => match &letter[..] {
            "e" => Some(Level::EASY),
            "m" => Some(Level::MEDIUM),
            "h" => Some(Level::HARD),
            "d" => Some(Level::DEADLY),
            _ => None,
        },
        None => None,
    }
}
