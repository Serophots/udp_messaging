use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::ui::ContactsList;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub contacts: ContactsList,
}
impl Settings {
    fn config_dir() -> anyhow::Result<PathBuf> {
        let mut config_dir =
            dirs::config_dir().ok_or(anyhow::anyhow!("couldn't find config directory"))?;
        config_dir.push("udp_messaging");

        if !config_dir.exists() {
            fs::create_dir(config_dir.clone())?;
        }
        config_dir.push("settings.json");

        Ok(config_dir)
    }

    pub fn load() -> anyhow::Result<Self> {
        let dir = Settings::config_dir()?;

        if dir.exists() {
            Ok(serde_json::from_str::<Settings>(&fs::read_to_string(dir)?)?)
        } else {
            Ok(Settings::default())
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let data = serde_json::to_string(&self)?;
        Ok(fs::write(Settings::config_dir()?, data)?)
    }
}
