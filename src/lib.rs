mod kvs;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use std::env::home_dir;

slint::include_modules!();

fn load_theme(kvs: Rc<kvs::KeyValueStore>, ui: Rc<MainWindow>) {
    let theme_name = kvs.get_as::<String>("theme-name");

    let ui_clone = ui.clone();
    let kvs_clone: Rc<kvs::KeyValueStore> = kvs.clone();

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
    let kvs = Rc::new(kvs::KeyValueStore::new(home_dir().unwrap()));
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
