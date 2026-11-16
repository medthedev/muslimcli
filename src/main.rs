mod args;
mod config;

fn main() {
    config::create_config();
    args::check_args();
}
