
use directories::ProjectDirs;
use std::{fs, io, path::PathBuf};
use toml_edit::{DocumentMut, Item, Table, value};

fn get_config_path() -> io::Result<PathBuf> {
    let project_dirs = ProjectDirs::from("org", "medthedev", "muslimcli")
        .ok_or_else(|| io::Error::other("Could not determine config directory"))?;

    let config_dir = project_dirs.config_dir();

    fs::create_dir_all(config_dir)?;

    Ok(config_dir.join("config.toml"))
}

fn default_config() -> String {
    let mut config = DocumentMut::new();

    let mut info = Table::new();
    info["city"] = value("unset");
    info["country"] = value("unset");

    config["info"] = Item::Table(info);

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

    let mut config = if config_path.exists() {
        fs::read_to_string(&config_path)?
            .parse::<DocumentMut>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
    } else {
        DocumentMut::new()
    };

    if !config["info"].is_table() {
        config["info"] = Item::Table(Table::new());
    }

    config["info"]["city"] = Item::Value(
        toml_edit::Value::from(city.trim())
            .decorated(" ", "")
    );
    
    config["info"]["country"] = Item::Value(
        toml_edit::Value::from(country.trim())
            .decorated(" ", "")
    );

    fs::write(&config_path, config.to_string())?;

    println!("Config updated successfully.");

    Ok(())
}
