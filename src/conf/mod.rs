use config::{Config, ConfigError, Environment, File};
use getset::Getters;
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize, Clone, Getters)]
#[get = "pub"]
pub struct DevOpsOptions {
    #[serde(default)]
    base_url: String,
    #[serde(default)]
    access_token: String,
    #[serde(default)]
    user_id: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Clone, Getters)]
#[get = "pub"]
pub struct Settings {
    devops: DevOpsOptions,
}

impl Settings {
    pub fn new(path: String) -> Result<Settings, ConfigError> {
        let settings = Config::builder()
            // 支持环境变量覆盖配置文件 (环境变量的优先级更高)
            .add_source(File::with_name(&path))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        settings.try_deserialize()
    }
}
