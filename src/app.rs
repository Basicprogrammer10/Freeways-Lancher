use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use home::home_dir;
use iced::{
    button, executor, image::Handle, slider, text_input, time, Align, Application, Button,
    Checkbox, Clipboard, Color, Column, Command, Container, Element, Image, Length, Radio, Row,
    Slider, Space, Text, TextInput,
};

use crate::config;
use crate::style;
use crate::CFG_PATH;
use crate::VERSION;

#[derive(Debug, Default)]
pub struct App {
    view: View,
    config: config::Config,
    theme: style::Theme,

    // Ui Elements
    launch_button: button::State,
    volume_slider: slider::State,
    settings_button: button::State,

    // Settings
    save_button: button::State,
    exit_button: button::State,
    reset_button: button::State,

    game_path_text: text_input::State,
}

#[derive(Debug)]
pub enum View {
    Main,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Launch,
    SettingsUpdate(config::ConfigUpdate),
    OpenSettings,
    ConfigSave,
    ConfigExit,
    ConfigReset,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let config_path = home_dir().unwrap().join(Path::new(CFG_PATH));

        print!("[*] Checking Data Dir ({}) ", config_path.to_string_lossy());
        match config::check_data_dir(config_path.clone()) {
            Some(_) => println!("[✅]"),
            None => println!("[❌]"),
        }

        print!(
            "[*] Loading Config ({}) ",
            config_path.join("config.cfg").to_string_lossy()
        );
        let config = config::Config::load(config_path.join("config.cfg"));

        let app = match config {
            Some(config) => {
                println!("[✅]");
                App {
                    config,
                    ..Default::default()
                }
            }
            None => {
                println!("[❌]");
                App::default()
            }
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        format!("Freeways Launcher {}", VERSION)
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        #[allow(unreachable_patterns)]
        match message {
            Message::Tick => {}
            Message::Launch => {
                println!(
                    "[*] Updateing Game Config (FullScreen: {}, Volume: {})",
                    self.config.full_screen, self.config.volume
                );

                fs::write(
                    self.config.game_path.join("freeways.cfg"),
                    format!(
                        r#"<Freeways fullScreen="{}" screenWidth="0" useFBO="false" volume="{}"/>{}"#,
                        self.config.full_screen, self.config.volume, "\n"
                    ),
                )
                .unwrap();

                let path = self.config.game_path.join("Freeways.exe");
                println!("[*] Launching Game ({})", path.to_string_lossy());
                process::Command::new(path).output().unwrap();
            }

            Message::SettingsUpdate(config_update) => {
                self.config = self.config.apply_update(config_update);
            }

            Message::OpenSettings => {
                self.view = View::Settings;
            }

            Message::ConfigSave => {
                println!(
                    "[*] Saveing Config (GamePath: '{}')",
                    self.config.game_path.to_string_lossy()
                );
                self.config.save(
                    home_dir()
                        .unwrap()
                        .join(Path::new(CFG_PATH))
                        .join("config.cfg"),
                );
                self.view = View::Main;
            }

            Message::ConfigExit => {
                self.view = View::Main;
            }

            Message::ConfigReset => {
                self.config = config::Config::default();
            }

            _ => {
                panic!("Unhandled Event: {:?}", message);
            }
        };
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(std::time::Duration::from_millis(500)).map(|_| Message::Tick)
    }

    fn view(&mut self) -> Element<Message> {
        match self.view {
            View::Main => Container::new(
                Column::new()
                    .align_items(Align::Center)
                    .padding(25)
                    .push(Text::new(format!("Freeways Launcher {}", VERSION)).size(45))
                    .push(Space::with_height(Length::Units(25)))
                    .push(
                        Row::new()
                            .spacing(20)
                            .push(
                                Text::new("Music Volume")
                                    .size(25)
                                    .width(Length::FillPortion(1)),
                            )
                            .push(
                                Slider::new(
                                    &mut self.volume_slider,
                                    0.0..=100.0,
                                    self.config.volume as f64,
                                    |x| {
                                        Message::SettingsUpdate(config::ConfigUpdate::Volume(
                                            x as u8,
                                        ))
                                    },
                                )
                                .width(Length::FillPortion(4))
                                .style(self.theme),
                            )
                            .push(Text::new(format!("[ {:0>3} ]", self.config.volume))),
                    )
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(Text::new("Options").size(25).width(Length::FillPortion(1)))
                            .push(
                                Checkbox::new(self.config.full_screen, "Full Screen", |x| {
                                    Message::SettingsUpdate(config::ConfigUpdate::FullScreen(x))
                                })
                                .width(Length::FillPortion(4))
                                .style(self.theme),
                            ),
                    )
                    .push(
                        Row::new().spacing(10).push(
                            Text::new("Resource Pack")
                                .size(25)
                                .width(Length::FillPortion(1)),
                        ),
                    )
                    .push(Space::new(Length::Fill, Length::Fill))
                    .push(
                        Row::new()
                            .height(Length::Shrink)
                            .spacing(10)
                            .push(Space::with_width(Length::Fill))
                            .push(
                                Button::new(
                                    &mut self.settings_button,
                                    Image::new(Handle::from_memory(
                                        crate::assets::SETTINGS_BUTTON.to_vec(),
                                    ))
                                    .height(Length::Units(50)),
                                )
                                .style(self.theme)
                                .on_press(Message::OpenSettings),
                            )
                            .push(
                                Button::new(&mut self.launch_button, Text::new("Launch!").size(50))
                                    .on_press(Message::Launch)
                                    .style(self.theme),
                            ),
                    ),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(self.theme)
            .into(),

            View::Settings => Container::new(
                Column::new()
                    .padding(20)
                    .spacing(17)
                    .push(Text::new("Settings").size(40))
                    .push(
                        Row::new()
                            .spacing(20)
                            .push(
                                Text::new("Game Path")
                                    .size(25)
                                    .width(Length::FillPortion(1)),
                            )
                            .push(
                                TextInput::new(
                                    &mut self.game_path_text,
                                    "",
                                    &self.config.game_path.to_string_lossy(),
                                    |x| Message::SettingsUpdate(config::ConfigUpdate::GamePath(x)),
                                )
                                .width(Length::FillPortion(4))
                                .style(self.theme),
                            ),
                    )
                    .push(Space::new(Length::Fill, Length::Fill))
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(
                                Button::new(&mut self.save_button, Text::new("Save").size(25))
                                    .on_press(Message::ConfigSave)
                                    .style(self.theme),
                            )
                            .push(
                                Button::new(&mut self.reset_button, Text::new("Reset").size(25))
                                    .on_press(Message::ConfigReset)
                                    .style(self.theme),
                            )
                            .push(
                                Button::new(&mut self.exit_button, Text::new("Cancel").size(25))
                                    .on_press(Message::ConfigExit)
                                    .style(self.theme),
                            ),
                    ),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(self.theme)
            .into(),
        }
    }
}

impl Default for View {
    fn default() -> View {
        View::Main
    }
}
