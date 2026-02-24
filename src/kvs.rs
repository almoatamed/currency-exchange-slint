
use core::error::Error;
use serde::Serialize;
use std::collections::HashMap;
use serde_json::Value;

#[cfg(target_arch = "wasm32")]
use web_sys;

#[cfg(not(target_arch = "wasm32"))]
use std::env::home_dir;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::create_dir_all;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::{self, read_to_string};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;


#[cfg(not(target_arch = "wasm32"))]
pub struct KeyValueStore {
    base_dir: PathBuf,
}

#[cfg(target_arch = "wasm32")]
pub struct KeyValueStore {}

#[cfg(target_arch = "wasm32")]
impl KeyValueStore {
    pub fn new() -> Self {
        return Self {};
    }

    fn storage(&self) -> web_sys::Storage {
        web_sys::window()
            .expect("No window available")
            .local_storage()
            .expect("localStorage not available")
            .expect("localStorage not found")
    }
    pub fn get_as<T>(&self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value = self.storage().get_item(key).ok()??;
        return serde_json::from_str(&value).ok();
    }

    pub fn set<T>(&self, key: &str, value: T) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        let ls = self.storage();

        let set_result = ls.set_item(key, &serde_json::to_string(&value)?);
        match set_result {
            Err(_) => return Err("web local storage set value error".into()),
            Ok(()) => return Ok(()),
        }
    }
}

/**
 * uses the file system to allocate a key value
 */
#[cfg(all(
    not(target_arch = "wasm32"),
    any(
        target_os = "linux",
        target_os = "windows",
        target_os = "macos",
        target_os = "android",
        target_os = "ios",
    )
))]
impl KeyValueStore {
    pub fn new(base_dir: PathBuf) -> Self {
        return Self { base_dir: base_dir };
    }
    pub fn get_store_full_path(&self) -> PathBuf {
        let mut home = self.base_dir.clone();

        home.push(".local");
        home.push("saraf-currency-exchange-systems-front");

        create_dir_all(&home).unwrap();

        home.push("kvs.json");

        if !home.exists() {
            fs::write(&home, "{}").unwrap();
        }

        return home;
    }

    pub fn get_hash_map(&self) -> HashMap<String, Value> {
        let content_string = read_to_string(self.get_store_full_path()).unwrap();
        let result = serde_json::from_str::<HashMap<String, Value>>(&content_string).unwrap();
        return result;
    }

    pub fn get_as<T>(&self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value: Option<T> = self.get_hash_map().get(key).and_then(|value| {
            return serde_json::from_value(value.clone()).ok();
        });

        return value;
    }

    pub fn set<T>(&self, key: &str, value: T) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        let mut hashmap = self.get_hash_map();
        hashmap.insert(key.into(), serde_json::to_value(value)?);
        fs::write(
            &self.get_store_full_path(),
            serde_json::to_string_pretty(&hashmap)?,
        )?;
        return Ok(());
    }
}
