use directories::ProjectDirs;
use std::{fs, io, path::PathBuf};
use toml_edit::{DocumentMut, value};

fn get_config_path() -> io::Result<PathBuf> {
    let project_dirs = ProjectDirs::from("org", "medthedev", "muslimcli")
        .ok_or_else(|| io::Error::other("Could not determine config directory"))?;

    let config_dir = project_dirs.config_dir();

    fs::create_dir_all(config_dir)?;

    Ok(config_dir.join("config.toml"))
}

fn default_config() -> String {
    let mut config = DocumentMut::new();

    config["info"]["city"] = value("unset");
    config["info"]["country"] = value("unset");

    config.to_string()
}

pub fn create_config() -> io::Result<()> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        fs::write(&config_path, default_config())?;
        println!("Config file created successfully.");
    } else {
        println!("Config file already exists.");
    }

    Ok(())
}

pub fn reset_config() -> io::Result<()> {
    let config_path = get_config_path()?;

    fs::write(&config_path, default_config())?;

    println!("Config reset successfully.");

    Ok(())
}


pub fn set_config(city: String, country: String) -> io::Result<()> {
    let config_path = get_config_path()?;

    let mut config = DocumentMut::new();

    config["info"]["city"] = value(city);
    config["info"]["country"] = value(country);

    fs::write(&config_path, config.to_string())?;

    println!("Config updated successfully.");

    Ok(())
}
