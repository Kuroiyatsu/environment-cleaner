use std::fs::{self, File};
use std::io::stdin;
use std::path::*;
use std::*;

pub struct Config {
    config_path: &'static str,
    config_file: &'static str,
}

impl Config {
    pub fn new() -> Config {
        Config {
            config_path: "C:\\ProgramData\\Environment Cleaner\\",
            config_file: "config.yaml",
        }
    }

    /// Ensures configuration file is created, and loads file.
    pub fn setup(&self) {
        if !self.has_config_file() {
            self.create_file();
            self.initialize();
        }
    }

    /// Checks if configuration file exists.
    fn has_config_file(&self) -> bool {
        let full_file_path = [self.config_path, self.config_file].join(""); //format!("{}{}", config_path, config_file);

        Path::new(&full_file_path[..]).exists()
    }

    /// Creates configuration file;
    fn create_file(&self) {
        println!("No configuration file found.");

        let full_file_path = [self.config_path, self.config_file].join("");

        println!("Creating file {}", full_file_path);

        fs::create_dir_all(self.config_path).unwrap();
        File::create(full_file_path).unwrap();

        println!("Configuration file created!");
    }

    pub fn initialize(&self) {
        self.add_virtual_drive_locations();
    }

    fn add_virtual_drive_locations(&self) {
        let mut done = false;
        let mut response = String::new();

        while !done {
            println!("Enter location for virtual disk, including the drive name: ");

            // TODO: Add "escape" functionality to quit and terminate application.
            stdin()
                .read_line(&mut response)
                .expect("Invalid user input");

            if !Path::new(&response).exists() {
                println!("The virtual drive located at {} was not found.", &response);
                continue;
            }
        }
    }
}
