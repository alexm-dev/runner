use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Keys {
    open_file: Vec<String>,
    go_up: Vec<String>,
    go_down: Vec<String>,
    go_origin: Vec<String>,
    go_into_dir: Vec<String>,
    quit: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Editor {
    cmd: String,
}

impl Keys {
    pub fn open_file(&self) -> &Vec<String> {
        &self.open_file
    }
    pub fn go_up(&self) -> &Vec<String> {
        &self.go_up
    }
    pub fn go_down(&self) -> &Vec<String> {
        &self.go_down
    }
    pub fn go_origin(&self) -> &Vec<String> {
        &self.go_origin
    }
    pub fn go_into_dir(&self) -> &Vec<String> {
        &self.go_into_dir
    }
    pub fn quit(&self) -> &Vec<String> {
        &self.quit
    }
}

impl Default for Keys {
    fn default() -> Self {
        Keys {
            open_file: vec!["Enter".into()],
            go_up: vec!["k".into(), "Up Arrow".into()],
            go_down: vec!["j".into(), "Down Arrow".into()],
            go_origin: vec!["h".into(), "Left Arrow".into(), "Backspace".into()],
            go_into_dir: vec!["l".into(), "Right Arrow".into()],
            quit: vec!["q".into(), "Esc".into()],
        }
    }
}

impl Editor {
    pub fn cmd(&self) -> &str {
        &self.cmd
    }
}

impl Default for Editor {
    fn default() -> Self {
        Editor { cmd: "nvim".into() }
    }
}
