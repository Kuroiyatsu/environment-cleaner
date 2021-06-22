use serde::{Serialize, Deserialize}

/// Settings used to handle cleanup. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    drive_locations: Vec<str>,
    log_folder: str
}