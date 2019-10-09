use std::{
    collections::HashMap,
    fs,
    io,
    path::PathBuf,
};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize)]
/// Stores variables and data for later use
pub struct Store {
    pub saved: HashMap<String, char>,
}

pub enum StoreErr {
    Io(io::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
    MissingConfig,
}

type Result<A> = std::result::Result<A, StoreErr>;

impl Store {
    /// Creates a new, empty store
    /// This is the value used when there is no existing store file.
    pub fn new() -> Store {
        Store {
            saved: HashMap::new(),
        }
    }

    fn store_path() -> Result<PathBuf> {
        let mut path = Store::mk_config_dir()?;
        path.push("store.toml");

        Ok(path)
    }

    /// Returns the path to the config directory
    /// If the directory does not exist, this will create it.
    fn mk_config_dir() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or(StoreErr::MissingConfig)?;
        path.push("unicode_util");
        if !path.is_dir() {
            std::fs::create_dir_all(&path)
                .expect("could not create config directory");
        }

        Ok(path)
    }

    /// Loads a store from the path
    pub fn load_file() -> Result<Store> {
        let path = Store::store_path()?;
        if path.is_file() {
            let file_data = fs::read_to_string(path)
                .map_err(|e| StoreErr::Io(e))?;
            toml::from_str(file_data.as_str())
                .map_err(|e| StoreErr::TomlDe(e))
        } else {
            Ok(Store::new())
        }
    }

    /// Saves a store to the path
    pub fn save_file(&self) -> Result<()>{
        let path = Store::store_path()?;
        let file_data = toml::to_string(self)
            .map_err(|e| StoreErr::TomlSer(e))?;
        fs::write(path, file_data)
            .map_err(|e| StoreErr::Io(e))
    }
}
