#![allow(non_snake_case)]

extern crate cocoa;
extern crate core_graphics;
extern crate block;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate log;
#[macro_use]
extern crate objc;
extern crate webkitten;

mod webkit;
mod cocoa_ext;
mod ui;
mod runtime;

use std::env;
use webkitten::ui::ApplicationUI;
use webkitten::Engine;
use ui::CocoaUI;

lazy_static! {
    pub static ref UI: CocoaUI = {
        if let Some(home_dir) = env::home_dir() {
            let default_config_path = &format!("{}/.config/webkitten/config.toml", home_dir.display());
            let ui = webkitten::optparse::parse_opts(default_config_path).and_then(|run_config| {
                Engine::new(&run_config.path)
            }).and_then(|engine| CocoaUI::new(engine));
            if let Some(ui) = ui {
                return ui;
            }
            panic!("Unable to initialize application");
        } else {
            panic!("Unable to locate home directory");
        }
    };
}

fn main() {
    env_logger::init().unwrap();
    runtime::declare_bar_delegates();
    UI.run();
}
