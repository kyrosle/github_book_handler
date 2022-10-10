use anyhow::Result;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

/// load from configs/config.yml
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct Configs {
    #[serde(flatten)]
    pub configs: HashMap<String, Config>,
}
/// sub node config
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Config {
    pub local: bool,
    #[serde(skip_serializing_if = "is_local_path", default)]
    pub path: Option<String>,
}

/// the config .git url 
fn is_local_path(v: &Option<String>) -> bool {
    !v.is_none()
}

impl Configs {
    pub fn new_init() -> Result<Configs> {
        let configs = Self::read_configs_from_yaml()?;
        Ok(configs)
    }
    pub fn read_configs_from_yaml() -> Result<Configs> {
        let mut dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let config_path = Path::new(r"configs");
        dir_path.push(config_path);
        dir_path.push("config");
        dir_path.set_extension("yml");
        let yml = fs::read(dir_path.as_path()).unwrap();
        let yml = String::from_utf8(yml).unwrap();
        let configs: Configs = serde_yaml::from_str(&yml).unwrap();
        Ok(configs)
    }
}
