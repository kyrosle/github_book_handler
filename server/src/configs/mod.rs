use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod read_book_config;
pub mod read_server_config;

pub(crate) trait ReadConfig<'de>
where
    Self: Serialize + Deserialize<'de> + Sized,
{
    fn new_init() -> Result<Self> {
        let configs = Self::read_configs_from_yaml().unwrap();
        Ok(configs)
    }
    fn read_configs_from_yaml() -> Result<Self>;
}
