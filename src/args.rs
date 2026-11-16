use crate::{api, config::set_config};
use clap::Parser;
use std::io;

#[derive(Parser)]
struct Cli {
    action: Action,
}

#[derive(Clone, clap::ValueEnum)]
enum Action {
    List,
    Help,
    Version,
    Setup,
}

pub async fn check_args() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.action {
        Action::List => list().await?,
        Action::Help => help(),
        Action::Version => version(),
        Action::Setup => setup(),
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

fn help() {
    println!("Available actions: search, help, version");
}

fn version() {
    println!("muslimcli v0.1.0");
}

fn setup() {
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

    set_config(city, country);
}
