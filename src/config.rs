use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use simple_config_parser::config;

use crate::VERSION;

#[derive(Debug, Clone)]
pub struct Config {
    pub game_path: PathBuf,

    // Game Settings
    pub volume: u8,
    pub full_screen: bool,
}

#[derive(Debug, Clone)]
pub enum ConfigUpdate {
    Volume(u8),
    FullScreen(bool),
    GamePath(String),
}

impl Config {
    pub fn load(path: PathBuf) -> Option<Config> {
        let mut file = OpenOptions::new().read(true).open(path).ok()?;
        let mut data = String::new();
        file.read_to_string(&mut data).ok()?;

        let mut cfg = config::Config::new(None);
        cfg.parse(&data.replace('\r', "")).ok()?;

        let game_path = cfg.get("game_path")?;

        let game_config = fs::read_to_string(Path::new(&game_path).join("freeways.cfg")).unwrap();

        let mut game_config_data = game_config.split('"');

        Some(Config {
            game_path: Path::new(&game_path).to_path_buf(),
            volume: game_config_data.clone().nth(7).unwrap().parse().ok()?,
            full_screen: game_config_data.nth(1).unwrap() == "true",
            ..Config::default()
        })
    }

    pub fn apply_update(&self, update: ConfigUpdate) -> Config {
        match update {
            ConfigUpdate::Volume(volume) => Config {
                volume,
                ..self.clone()
            },

            ConfigUpdate::FullScreen(full_screen) => Config {
                full_screen,
                ..self.clone()
            },

            ConfigUpdate::GamePath(game_path) => Config {
                game_path: Path::new(&game_path).to_path_buf(),
                ..self.clone()
            },
        }
    }

    pub fn save(&self, path: PathBuf) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();

        fs::write(
            path,
            format!(
                "; Freeways-Launcher V{} Config\ngame_path = {}\n",
                VERSION,
                self.game_path.to_string_lossy()
            ),
        )
        .unwrap();
    }
}

impl Default for Config {
    fn default() -> Config {
        // TODO: Support for other os
        Config {
            game_path: Path::new(r#"C:\Program Files\Steam\steamapps\common\Freeways"#)
                .to_path_buf(),
            volume: 100,
            full_screen: false,
        }
    }
}
