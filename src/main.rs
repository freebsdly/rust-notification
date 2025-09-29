use crate::cli::run_cli;

mod api;
mod cli;
mod server;
mod conf;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    run_cli().await
}
