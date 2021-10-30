use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use simple_config_parser::config;

#[derive(Debug, Clone)]
pub struct Config {
    pub game_path: PathBuf,
    pub volume: u8,
    pub full_screen: bool,
}

#[derive(Debug, Clone)]
pub enum ConfigUpdate {
    Volume(u8),
    FullScreen(bool),
}

impl Config {
    pub fn load(path: PathBuf) -> Option<Config> {
        let mut file = match OpenOptions::new().read(true).open(path) {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(_) => (),
            Err(_) => return None,
        };

        let mut cfg = config::Config::new(None);
        match cfg.parse(&data.replace('\r', "")) {
            Ok(_) => {}
            Err(_) => return None,
        }

        let game_path = cfg.get("game_path")?;

        let game_config = fs::read_to_string(Path::new(&game_path).join("freeways.cfg")).unwrap();

        let mut game_config_data = game_config.split('"');

        Some(Config {
            game_path: Path::new(&game_path).to_path_buf(),
            volume: game_config_data.clone().nth(7).unwrap().parse().unwrap(),
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
        }
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
