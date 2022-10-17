use anyhow::Result;
use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

use crate::path::{RPath, IntoInner};

use super::ReadConfig;

/// node book config
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BookConfig {
    pub local: bool,
    #[serde(skip_serializing_if = "is_local_path", default)]
    pub path: Option<String>,
}

/// load from configs/book-config.yml
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
pub struct BookConfigs {
    #[serde(flatten)]
    pub configs: HashMap<String, BookConfig>,
}

/// the config .git url
fn is_local_path(v: &Option<String>) -> bool {
    !v.is_none()
}

impl<'de> ReadConfig<'de> for BookConfigs {
    fn read_configs_from_yaml() -> Result<Self> {
        // Build the Book-Config path
        let dir_path = RPath::from_project_path()
            .append("configs")
            .set_file_name("book-config")
            .into_inner()
            .set_extension("yml")
            .unwrap();

        // read the file context
        let yml = fs::read(dir_path.as_path()).unwrap();
        let yml = String::from_utf8(yml).unwrap();

        let configs: BookConfigs = serde_yaml::from_str(&yml).unwrap();
        Ok(configs)
    }
}
