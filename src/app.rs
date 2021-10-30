use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use home::home_dir;
use iced::{
    button, executor, slider, text_input, time, Align, Application, Button, Checkbox, Clipboard,
    Color, Column, Command, Container, Element, Length, Radio, Row, Slider, Text, TextInput,
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
}

#[derive(Debug)]
pub enum View {
    Main,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Launch,
    SettingsUpdate(config::ConfigUpdate),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let config_path = home_dir().unwrap().join(Path::new(CFG_PATH));

        print!("[*] Loading Config ({}) ", config_path.to_string_lossy());
        let config = config::Config::load(config_path);

        let app = match config {
            Some(config) => {
                println!("[Success]");
                App {
                    config,
                    ..Default::default()
                }
            }
            None => {
                println!("[Failed]");
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
                println!("[*] Updateing Game Config");
                fs::write(
                    self.config.game_path.join("freeways.cfg"),
                    format!(
                        r#"<Freeways fullScreen="{}" screenWidth="0" useFBO="false" volume="{}"/>\n"#,
                        self.config.full_screen, self.config.volume
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
        let content = match self.view {
            View::Main => Column::new()
                .align_items(Align::Center)
                .padding(20)
                .push(Text::new(format!("Freeways Launcher {}", VERSION)).size(45))
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
                                |x| Message::SettingsUpdate(config::ConfigUpdate::Volume(x as u8)),
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
                    Row::new().push(
                        Button::new(&mut self.launch_button, Text::new("Launch!").size(50))
                            .on_press(Message::Launch)
                            .style(self.theme),
                    ),
                ),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}

impl Default for View {
    fn default() -> View {
        View::Main
    }
}
