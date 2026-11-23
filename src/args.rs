use crate::{
    api, config::{create_config, reset_config, set_config},
};
use clap::Parser;
use std::io;

/// A small CLI for prayer times and duas.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Action to perform
    action: Action,
}

#[derive(Clone, clap::ValueEnum)]
enum Action {
    /// Show today's prayer times
    List,
    /// Set your city and country in the config
    Setup,
    /// Reset the config to its defaults
    Reset,
    /// Show a random dua
    Dua,
}

pub async fn check_args() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    create_config()?;

    match args.action {
        Action::List => list().await?,
        Action::Setup => setup()?,
        Action::Reset => reset()?,
        Action::Dua => dua().await?,
    }

    Ok(())
}

async fn list() -> Result<(), Box<dyn std::error::Error>> {
    let times = api::get_data().await?;

    println!("Fajr: {}", times.fajr);
    println!("Dhuhr: {}", times.dhuhr);
    println!("Asr: {}", times.asr);
    println!("Maghrib: {}", times.maghrib);
    println!("Isha: {}", times.isha);

    Ok(())
}

async fn dua() -> Result<(), Box<dyn std::error::Error>> {
    let dua = api::get_dua().await?;

    println!("{}", dua);

    Ok(())
}

fn setup() -> io::Result<()> {
    let mut city = String::new();
    let mut country = String::new();

    println!("What is your city name?");
    io::stdin()
        .read_line(&mut city)
        .expect("Failed to read input");

    println!("What is your country name?");
    io::stdin()
        .read_line(&mut country)
        .expect("Failed to read input");

    set_config(city, country)
}

fn reset() -> io::Result<()> {
    reset_config()
}
