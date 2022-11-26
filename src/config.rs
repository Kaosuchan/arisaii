use std::io::Read;
use std::path::Path;
use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "kebab-name")]
pub struct Config {
    title: Option<String>,
    description: Option<String>,
    author: Option<String>,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let mut file = fs::File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let config = toml::from_str(&buf)?;
        Ok(config)
    }
}
