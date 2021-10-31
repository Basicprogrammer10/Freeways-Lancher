use std::env::consts;
use std::fs;
use std::panic;
use std::path::Path;
use std::process;

use iced::{window, Application, Settings};
use image::GenericImageView;

mod app;
mod assets;
mod config;
mod resource_pack;
mod style;
use app::App;

pub const VERSION: &str = "α0.1.0";
pub const CFG_PATH: &str = ".freeways_launcher";

pub fn main() -> iced::Result {
    println!("[*] Freeways Launcher [{}]", VERSION);

    // Set Panic Handler
    panic::set_hook(Box::new(|p| {
        let data = &format!(
            "{}\n{}\nCompile Time: {}\nPlatform: {} {}\nVersion: {}",
            p.to_string(),
            env!("GIT_INFO"),
            env!("COMPILE_TIME"),
            consts::OS,
            consts::ARCH,
            VERSION,
        );
        eprintln!("{}", data);
        msgbox::create("Freeways-Launcher Error", data, msgbox::IconType::Error)
            .unwrap_or_default();
        process::exit(-1);
    }));

    // Load Window Icon
    let icon = image::load_from_memory(assets::ICON).unwrap();

    // Run Application
    App::run(Settings {
        window: window::Settings {
            size: (800, 400),
            min_size: Some((800, 400)),
            icon: Some(
                window::Icon::from_rgba(icon.to_rgba8().into_raw(), icon.width(), icon.height())
                    .unwrap(),
            ),
            ..Default::default()
        },
        default_font: Some(assets::MAIN_FONT_RAW),
        ..Settings::default()
    })
}

// TODO:

// ✅ Config Saveing

// Allow picking between diffrent worlds
// Allow makeing new worlds
// Allow deleteing worlds (move to like a /deleted folder)

// Rescorse pack loader
//  * Load from zip / tar files
//  * Load meta file from package
//  * add gui stuff for adding / managing packs
//  * alow resetting to orginal rescorses
