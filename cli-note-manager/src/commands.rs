use clap::{Parser, Subcommand};
use std::error::Error;
use std::{fs, path::Path};

use crate::models::Note;

///Note manager in CLI
#[derive(Parser, Debug)]
pub struct Args {
    /// What to do
    #[command(subcommand)]
    pub command: Command,
}

///Available commands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add new note
    Add { text: String },
    /// List all notes
    List,
    ///List notes in JSON format
    ListJson,
    ///Delete a note
    Delete { id: usize },
}

pub fn add_note(
    path: &Path,
    notes: &mut Vec<Note>,
    text: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_id = notes.len() + 1;
    notes.push(Note { id: new_id, text });
    fs::write(path, serde_json::to_string_pretty(&notes)?)?;
    println!("Note successfully added");
    Ok(())
}

pub fn list_notes(notes: &[Note]) -> Result<(), Box<dyn Error>> {
    for note in notes {
        println!("{} - {}", note.id, note.text);
    }
    Ok(())
}

pub fn list_notes_json(notes: &[Note]) -> Result<(), Box<dyn Error>> {
    println!("{}", serde_json::to_string_pretty(notes)?);
    Ok(())
}

pub fn delete_note(path: &Path, notes: &mut Vec<Note>, id: usize) -> Result<(), Box<dyn Error>> {
    notes.retain(|note| note.id != id);
    fs::write(path, serde_json::to_string_pretty(notes)?)?;
    println!("Note succeessfully deleted");
    println!("Remaining number of notes: {}", notes.len());
    Ok(())
}
