mod encounter_table;

use clap::Parser;

use std::process;

use encounter_table::{EncounterTable, Level};

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
    ///path to encounter table (defaults to using encounter table details from the DMG)
    #[clap(short, long, value_parser)]
    path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let levels = get_levels(&args);
    let difficulty = get_difficulty(&args);
    let encounter_table = encounter_table::get_encounter_table(&args.path);
    outcome(&encounter_table, &levels, &difficulty);
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
