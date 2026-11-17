mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    args::check_args().await?;

    Ok(())
}