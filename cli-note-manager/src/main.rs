use std::{error::Error, fs, path::Path};

use crate::models::Note;
use clap::Parser;

mod commands;
mod models;

fn main() -> Result<(), Box<dyn Error>> {
    let args = commands::Args::parse();

    // println!("{:?}", args);

    let path = Path::new("notes.json");

    if !path.exists() {
        fs::write(path, "[]")?;
        println!("Created new empty .json file ");
    }

    let json_data = fs::read_to_string(path)?;
    let mut notes: Vec<Note> = serde_json::from_str(&json_data)?;

    match args.command {
        commands::Command::Add { text } => commands::add_note(path, &mut notes, text)?,
        commands::Command::List => commands::list_notes(&notes)?,
        commands::Command::ListJson => commands::list_notes_json(&notes)?,
        commands::Command::Delete { id } => commands::delete_note(path, &mut notes, id)?,
    }

    Ok(())
}
