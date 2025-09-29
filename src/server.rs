use crate::api::ApiService;
use crate::conf::Settings;
use anyhow::anyhow;
use log::{debug, info};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

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
