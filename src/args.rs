use clap::Parser;

#[derive(Parser)]
struct Cli {
    action: Action,
}

#[derive(Clone, clap::ValueEnum)]
enum Action {
    List,
    Help,
    Version,
}

pub fn check_args() {
    let args = Cli::parse();

    match args.action {
        Action::List => list(),
        Action::Help => help(),
        Action::Version => version(),
    }
}

fn list() {
    println!("( aka 'payers' )");
}

fn help() {
    println!("Available actions: search, help, version");
}

fn version() {
    println!("muslimcli v0.1.0");
}