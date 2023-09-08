use server::run_server; // Update 'my_project' to your project name
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run_server().await?;
    Ok(())
}