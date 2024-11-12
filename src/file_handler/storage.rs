use std::{ fs, path::Path };

use hashbrown::HashMap;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StorageFile {
    pub active_number: Option<String>,
    pub numbers: HashMap<String, f64>,
}

impl Default for StorageFile {
    fn default() -> Self {
        Self { active_number: None, numbers: HashMap::new() }
    }
}

pub fn load_storage_file<P: AsRef<Path>>(path: P) -> anyhow::Result<StorageFile> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

pub fn write_storage_file<P: AsRef<Path>>(path: P, config: &StorageFile) -> anyhow::Result<()> {
    Ok(fs::write(path, serde_json::to_string(config)?)?)
}

pub fn write_storage_file_pretty<P: AsRef<Path>>(
    path: P,
    config: &StorageFile
) -> anyhow::Result<()> {
    Ok(fs::write(path, serde_json::to_string_pretty(config)?)?)
}
