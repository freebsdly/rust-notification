mod api;
mod application;
mod conf;
mod repository;

use clap::{Args, Parser, Subcommand};
use std::time::Duration;
use tokio::{select, signal, time};

use crate::api::ApiService;
use crate::conf::Settings;
use anyhow::anyhow;
use log::{debug, info};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

/**
 * ServiceManager
 */
pub struct ServiceManager {
    settings: OnceLock<Settings>,
    parent_token: CancellationToken,
    api_service: Arc<RwLock<ApiService>>,
}

impl ServiceManager {
    pub async fn new(settings: Settings) -> Result<Self, anyhow::Error> {
        debug!("server args: {:?}", settings.clone());
        let parent_token = CancellationToken::new();
        let api_service = ApiService::new(parent_token.clone(), settings.api().clone())?;

        Ok(Self {
            settings: OnceLock::from(settings),
            parent_token,
            api_service: Arc::new(RwLock::new(api_service)),
        })
    }

    pub fn start(&self) -> Result<(), anyhow::Error> {
        self.start_api_service()?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), anyhow::Error> {
        info!("Stopping ServerManager gracefully");
        self.stop_api_service()?;
        Ok(())
    }

    pub fn stop_force(&self) -> Result<(), anyhow::Error> {
        info!("Stopping ServerManager force");
        self.parent_token.cancel();
        Ok(())
    }

    fn start_api_service(&self) -> Result<(), anyhow::Error> {
        let api_service = self.api_service.clone();
        let guard = api_service.try_write();
        match guard {
            Ok(guard) => guard.start(),
            Err(err) => Err(anyhow!(err)),
        }
    }

    fn stop_api_service(&self) -> Result<(), anyhow::Error> {
        let api_service = self.api_service.clone();
        let guard = api_service.try_write();
        match guard {
            Ok(service) => service.stop(),
            Err(err) => Err(anyhow!(err)),
        }
    }
}

/**
 * CLI
 */
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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    run_cli().await
}
