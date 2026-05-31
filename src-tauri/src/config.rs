use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
};

pub type NameDataBase = String;

pub const PATH_CONFIG: &str = "./lab_manager.config";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    current_database: Option<PathBuf>,
    list_db: HashMap<NameDataBase, PathBuf>,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        match std::fs::read_to_string(PATH_CONFIG) {
            Ok(content) => Ok(serde_json::from_str(&content)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let config = Self {
                    current_database: None,
                    list_db: HashMap::new(),
                };
                std::fs::write(PATH_CONFIG, serde_json::to_string(&config)?)
                    .context("Failed to write default config")?;
                Ok(config)
            }
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_current_database(&self) -> Option<&PathBuf> {
        self.current_database.as_ref()
    }

    pub fn get_list_database(&self) -> &HashMap<String, PathBuf> {
        &self.list_db
    }

    pub fn select_database(&mut self, name: &str) -> anyhow::Result<()> {
        let path = self
            .list_db
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Database '{}' not found", name))?;
        self.current_database = Some(path.clone());
        self.save_data()?;
        Ok(())
    }

    pub fn scan_databases(&mut self) -> anyhow::Result<()> {
        self.list_db.clear();

        std::fs::read_dir("./")?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_type = entry.file_type().ok()?;
                if file_type.is_file() && entry.path().extension() == Some("db".as_ref()) {
                    Some((entry.file_name().into_string().ok()?, entry.path()))
                } else {
                    None
                }
            })
            .for_each(|(name, path)| {
                self.list_db.insert(name, path);
            });

        self.save_data()?;
        Ok(())
    }

    fn save_data(&self) -> anyhow::Result<()> {
        std::fs::write(PATH_CONFIG, serde_json::to_string(self)?)?;
        Ok(())
    }
}
