use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteManager {
    pub notes: Vec<Note>,
}

impl NoteManager {
    pub fn new() -> Self {
        NoteManager { notes: Vec::new() }
    }

    pub fn load_from_file(file_path: &str) -> Result<Self> {
        let data = fs::read_to_string(file_path).unwrap_or_else(|_| String::from("[]"));
        let notes = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
        Ok(NoteManager { notes })
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.notes)?;
        fs::write(file_path, data)
    }

    pub fn add_note(&mut self, title: &str, content: &str) {
        let new_id = self.notes.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        let new_note = Note {
            id: new_id,
            title: title.to_string(),
            content: content.to_string(),
        };
        self.notes.push(new_note);
    }

    pub fn list_notes(&self) {
        if self.notes.is_empty() {
            println!("No notes available.");
        } else {
            for note in &self.notes {
                println!("ID: {} | Title: {}", note.id, note.title);
            }
        }
    }

    pub fn read_note_by_id(&self, id: u32) {
        if let Some(note) = self.notes.iter().find(|&n| n.id == id) {
            println!("Title: {}\nContent: {}", note.title, note.content);
        } else {
            println!("Note with ID {} not found", id);
        }
    }

    pub fn delete_note_by_id(&mut self, id: u32) {
        if let Some(pos) = self.notes.iter().position(|n| n.id == id) {
            self.notes.remove(pos);
            println!("Note with ID {} deleted", id);
        } else {
            println!("Note with ID {} not found", id);
        }
    }
}
