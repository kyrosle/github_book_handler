use std::collections::HashMap;

use anyhow::Result;
use serde::{Serialize, Deserialize};

use super::ReadConfig;

/// node server config
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub domains: Option<String>,
    pub ip: String,
    pub port: String,
}

/// load from configs/server-config.yml
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfigs {
    #[serde(flatten)]
    pub configs : HashMap<String, ServerConfig>,
}

impl<'de> ReadConfig<'de> for ServerConfigs{
    fn read_configs_from_yaml() -> Result<Self> {
        todo!()
    }
}