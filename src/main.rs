mod note; // Inclut le module note.rs

use clap::{Arg, Command};
use note::NoteManager;

fn main() {
    let matches = Command::new("Notes Manager")
        .version("1.0")
        .author("Your Name")
        .about("Manages your notes")
        .subcommand(Command::new("add")
            .about("Add a new note")
            .arg(Arg::new("title")
                .help("Title of the note")
                .required(true)
                .index(1))
            .arg(Arg::new("content")
                .help("Content of the note")
                .required(true)
                .index(2)))
        .subcommand(Command::new("list")
            .about("List all notes"))
        .subcommand(Command::new("read")
            .about("Read a specific note by ID")
            .arg(Arg::new("id")
                .help("ID of the note to read")
                .required(true)
                .index(1)))
        .subcommand(Command::new("delete")
            .about("Delete a specific note by ID")
            .arg(Arg::new("id")
                .help("ID of the note to delete")
                .required(true)
                .index(1)))
        .get_matches();

    let file_path = "notes.json";
    let mut note_manager = NoteManager::load_from_file(file_path).unwrap_or_else(|_| NoteManager::new());

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let title = sub_m.get_one::<String>("title").expect("Title is required").clone();
            let content = sub_m.get_one::<String>("content").expect("Content is required").clone();
            note_manager.add_note(&title, &content);
            println!("Note added: {} - {}", title, content);
        }
        Some(("list", _)) => {
            note_manager.list_notes();
        }
        Some(("read", sub_m)) => {
            let id: u32 = sub_m.get_one::<String>("id").expect("ID is required").parse().expect("Invalid ID");
            note_manager.read_note_by_id(id);
        }
        Some(("delete", sub_m)) => {
            let id: u32 = sub_m.get_one::<String>("id").expect("ID is required").parse().expect("Invalid ID");
            note_manager.delete_note_by_id(id);
        }
        _ => {
            println!("No valid subcommand provided.");
        }
    }

    note_manager.save_to_file(file_path).expect("Failed to save notes");
}
