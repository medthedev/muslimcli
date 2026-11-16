use crate::config::reset_config;

mod api;
mod args;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::create_config();
    reset_config();

    args::check_args().await?;

    Ok(())
}