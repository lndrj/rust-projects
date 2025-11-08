use serde::{Deserialize, Serialize};

/// Struct for note
#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: usize,
    pub text: String,
}
