use directories::ProjectDirs;
use std::fs;
use std::fs::File;

pub fn create_config() {
    if let Some(proj_dirs) = ProjectDirs::from("org", "medthedev", "muslimcli") {
        let config_dir = proj_dirs.config_dir();

        fs::create_dir_all(config_dir)
            .expect("Could not create config directory");

        let config_path = config_dir.join("config.toml");

        if !config_path.exists() {
            File::create(config_path)
                .expect("Could not create config file");

            println!("Config file created successfully.");
        } else {
            println!("Config file already exists.");
        }
    } else {
        println!("Could not determine config directory.");
    }
}