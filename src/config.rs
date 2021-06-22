use std::fs::{self, File};
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
        }
    }

    /// Checks if configuration file exists.
    fn has_config_file(&self) -> bool {
        let full_file_path = [self.config_path, self.config_file].join(""); //format!("{}{}", config_path, config_file);

        std::path::Path::new(&full_file_path[..]).exists()
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

    fn initialize() {}
}
