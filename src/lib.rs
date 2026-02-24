#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "linux", target_os = "windows")
))]
use serde_json::Value;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use core::error::Error;
use serde::Serialize;
#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "linux", target_os = "windows")
))]
use std::any;
#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "linux", target_os = "windows")
))]
use std::collections::HashMap;
use std::env::home_dir;
use std::fs::create_dir_all;
#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "linux", target_os = "windows")
))]
use std::fs::{self, read, read_to_string};
#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "linux", target_os = "windows")
))]
use std::path::PathBuf;

slint::include_modules!();

fn load_theme(ui: &MainWindow) {
    ui.global::<Theme>().set_themeName(ThemeMode::Dark);
}

struct KeyValueStore {}

#[cfg(target_arch = "wasm32")]
impl KeyValueStore {
    pub fn get(key: &str) -> Option<String> {
        return Some("".into());
    }
    pub fn get_as<T>(key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        return None;
    }

    pub fn set<T>(key: &str) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        return Ok(());
    }
}

/**
 * uses the file system to allocate a
 */
#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "linux", target_os = "windows")
))]
impl KeyValueStore {
    pub fn new() -> Self {
        return Self {};
    }
    pub fn get_store_full_path(&self) -> PathBuf {
        let mut home = home_dir().unwrap();

        home.push(".local");
        home.push("saraf-currency-exchange-systems-front");

        create_dir_all(&home).unwrap();

        home.push("kvs.json");

        if !fs::exists(&home).unwrap() {
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

#[cfg(all(not(target_arch = "wasm32"), target_os = "ios"))]
impl KeyValueStore {
    pub fn get(key: &str) -> Option<String> {
        return Some("".into());
    }
    pub fn get_as<T>(key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        return None;
    }

    pub fn set<T>(key: &str) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        return Ok(());
    }
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "android"))]
impl KeyValueStore {
    pub fn get(key: &str) -> Option<String> {
        return Some("".into());
    }
    pub fn get_as<T>(key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        return None;
    }

    pub fn set<T>(key: &str) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        return Ok(());
    }
}

fn initialize_key_value_store(ui: &MainWindow) {}

fn ui() -> MainWindow {
    let ui = MainWindow::new().unwrap();
    initialize_key_value_store(&ui);

    load_theme(&ui);
    return ui;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let ui = ui();
    ui.run().unwrap();
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    slint::android::init(android_app).unwrap();
    let ui = ui();
    MaterialWindowAdapter::get(&ui).set_disable_hover(true);
    ui.run().unwrap();
}
