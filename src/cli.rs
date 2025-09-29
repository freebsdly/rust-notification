use crate::server::ServiceManager;
use clap::{Args, Parser, Subcommand};
use log::info;
use std::time::Duration;
use tokio::{select, signal, time};
use crate::conf::Settings;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct AppCli {
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Start the server
    Start(StartServerArgs),
}

#[derive(Args, Debug)]
pub struct StartServerArgs {
    /// Configuration file path
    #[arg(short, long, default_value = "conf/settings.toml")]
    pub path: String,
    #[arg(short, long)]
    pub graceful_shutdown: bool,
}

pub async fn start_server(args: StartServerArgs) -> Result<(), anyhow::Error> {
    let settings = Settings::new(args.path)?;
    let server = ServiceManager::new(settings.clone()).await?;
    server.start()?;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    let with_graceful_shutdown = args.graceful_shutdown.clone();
    select! {
        _ = ctrl_c => {
            info!("receive ctrl_c to shutting down server");
            if with_graceful_shutdown {
                server.stop()?
            } else {
                server.stop_force()?
            }
        },
        _ = terminate => {
            info!("signal handler exited unexpectedly");
            server.stop_force()?
        },
    }

    Ok(time::sleep(Duration::from_secs(1)).await)
}

pub async fn run_cli() -> Result<(), anyhow::Error> {
    let cli = AppCli::parse();

    match cli.command {
        Some(SubCommands::Start(start_server_args)) => start_server(start_server_args).await,
        _ => Err(anyhow::Error::msg("not starting server")),
    }
}
