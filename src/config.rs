use std::fs::{self, File};

/// Ensures configuration file is created, and loads file.
pub fn setup_config() {
    let config_path = "C:\\ProgramData\\Environment Cleaner\\";
    let config_file = "config.yaml";

    if !has_config_file(config_path, config_file) {
        create_file(config_path, config_file);
    }
}

/// Checks if configuration file exists.
fn has_config_file(config_path: &str, config_file: &str) -> bool{
    let full_file_path = [config_path, config_file].join("");//format!("{}{}", config_path, config_file);
    
    std::path::Path::new(&full_file_path[..]).exists()
}

/// Creates configuration file;
fn create_file(config_path: &str, config_file: &str) {
    println!("No configuration file found.");

    let full_file_path = [config_path, config_file].join("");

    println!("Creating file {}", full_file_path);

    fs::create_dir_all(config_path).unwrap();
    File::create(full_file_path).unwrap();

    println!("Configuration file created!");
}