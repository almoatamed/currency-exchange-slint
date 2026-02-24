use core::error::Error;
use serde::Serialize;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)] // optional, suppresses dead code warnings
use serde_json::Value;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

slint::include_modules!();

#[cfg(not(target_arch = "wasm32"))]
struct KeyValueStore {
    base_dir: PathBuf,
}

#[cfg(target_arch = "wasm32")]
struct KeyValueStore {}

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

fn load_theme(kvs: Rc<KeyValueStore>, ui: Rc<MainWindow>) {
    let theme_name = kvs.get_as::<String>("theme-name");

    let ui_clone = ui.clone();
    let kvs_clone: Rc<KeyValueStore> = kvs.clone();

    ui.global::<Theme>().on_setTheme(move |theme_name| {
        ui_clone.global::<Theme>().set_themeName(theme_name);
        let theme_name_string: &str;
        match theme_name {
            ThemeMode::Dark => {
                theme_name_string = "dark";
            }
            ThemeMode::Light => {
                theme_name_string = "light";
            }
        };
        let _ = kvs_clone.set("theme-name", &theme_name_string);
    });

    match theme_name {
        Some(theme) => {
            if theme == "dark" {
                ui.global::<Theme>().set_themeName(ThemeMode::Dark);
                return;
            }
            ui.global::<Theme>().set_themeName(ThemeMode::Light);
        }
        None => {
            ui.global::<Theme>().set_themeName(ThemeMode::Light);
        }
    };
}

#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "linux", target_os = "macos", target_os = "windows",)
))]
fn ui() -> Rc<MainWindow> {
    use std::rc::Rc;
    let ui = Rc::new(MainWindow::new().unwrap());
    let kvs = Rc::new(KeyValueStore::new(home_dir().unwrap()));
    load_theme(kvs, ui.clone());
    return ui;
}

#[cfg(target_arch = "wasm32")]
fn ui() -> MainWindow {
    use std::rc::Rc;
    let ui = Rc::new(MainWindow::new().unwrap());
    let kvs = Rc::new(KeyValueStore::new());
    load_theme(kvs, ui.clone());
    return ui;
}

#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "android", target_os = "ios")
))]
fn ui(base_dir: PathBuf) -> MainWindow {
    use std::rc::Rc;
    let ui = Rc::new(MainWindow::new().unwrap());
    let kvs = Rc::new(KeyValueStore::new(base_dir));
    load_theme(kvs, ui.clone());
    return ui;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn main() {
    let ui = ui();
    ui.run().unwrap();
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    slint::android::init(android_app).unwrap();
    let base_dir = android_app.context().base_dir();
    let ui = ui(base_dir);
    MaterialWindowAdapter::get(&ui).set_disable_hover(true);
    ui.run().unwrap();
}
