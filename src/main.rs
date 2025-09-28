mod conf;
mod devops;
mod db;

use crate::conf::Settings;
use crate::devops::DevOpsApiClient;
use once_cell::sync::Lazy;
use pretty_env_logger;
use std::env;
use std::sync::{Arc, RwLock};

// 全局配置实例（单例）
#[allow(unused)]
pub static GLOBAL_CONFIG: Lazy<Arc<RwLock<Settings>>> = Lazy::new(|| {
    let settings = Settings::new("conf/settings.toml".to_string()).expect("配置加载失败");
    Arc::new(RwLock::new(settings))
});

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 当RUST_LOG环境变量不存在或为空时设置它
    if env::var("RUST_LOG").is_err() || env::var("RUST_LOG")?.is_empty() {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }

    // 初始化日志
    pretty_env_logger::init();

    log::info!("配置加载成功");
    db::Repository::new();
    let devops_options = GLOBAL_CONFIG
        .read()
        .map_err(|e| anyhow::anyhow!("读取配置失败: {:?}", e))?
        .devops()
        .clone();
    log::info!("DevOps Base URL: {}", devops_options.base_url());
    let client = DevOpsApiClient::new(devops_options);
    match client.get_project_pipelines("dc62af".to_string()).await {
        Ok(pipelines) => {
            log::info!("项目流水线列表: {:?}", pipelines);
        }
        Err(e) => {
            log::error!("{:?}", e.to_string());
        }
    }
    Ok(())
}

async fn query_db() -> Result<(), sqlx::Error>{
    // let db_url = env::var("DATABASE_URL")
    //     .unwrap_or_else(|_| "sqlite:demo.db".into());
    //
    // let pool = SqlitePool::connect(&db_url).await?;
    Ok(())
}