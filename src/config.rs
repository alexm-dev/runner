use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub dirs_first: bool,
    pub show_hidden: bool,
    pub case_insensitive: bool,
    pub display: Display,
    pub theme: Theme,
    pub editor: Editor,
    pub keys: Keys,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Display {
    pub show_dir_marker: bool,
    pub borders: bool,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Theme {
    pub background: String,
    pub accent_color: String,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Keys {
    pub open_file: Vec<String>,
    pub go_up: Vec<String>,
    pub go_down: Vec<String>,
    pub go_parent: Vec<String>,
    pub go_into_dir: Vec<String>,
    pub quit: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Editor {
    pub cmd: String,
}

impl Config {
    pub fn load(path: &str) -> Self {
        match fs::read_to_string(path)
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
        {
            Some(cfg) => cfg,
            None => {
                println!("Config file missing or invalid, using defaults.");
                Config::default()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dirs_first: true,
            show_hidden: false,
            case_insensitive: false,
            display: Display::default(),
            theme: Theme::default(),
            editor: Editor::default(),
            keys: Keys::default(),
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Display {
            show_dir_marker: true,
            borders: true,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: "default".into(),
            accent_color: "default".into(),
        }
    }
}

impl Default for Keys {
    fn default() -> Self {
        Keys {
            open_file: vec!["Enter".into()],
            go_up: vec!["k".into(), "Up Arrow".into()],
            go_down: vec!["j".into(), "Down Arrow".into()],
            go_parent: vec!["h".into(), "Left Arrow".into(), "Backspace".into()],
            go_into_dir: vec!["l".into(), "Right Arrow".into(), "Enter".into()],
            quit: vec!["q".into(), "Esc".into()],
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Editor { cmd: "vim".into() }
    }
}
