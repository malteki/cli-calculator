use std::{ fs, path::Path };

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigFile {
    pub print_timing: bool,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self { print_timing: false }
    }
}

pub fn load_config_file<P: AsRef<Path>>(path: P) -> anyhow::Result<ConfigFile> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

pub fn write_config_file<P: AsRef<Path>>(path: P, config: &ConfigFile) -> anyhow::Result<()> {
    Ok(fs::write(path, toml::to_string(config)?)?)
}

pub fn write_config_file_pretty<P: AsRef<Path>>(
    path: P,
    config: &ConfigFile
) -> anyhow::Result<()> {
    Ok(fs::write(path, toml::to_string_pretty(config)?)?)
}
