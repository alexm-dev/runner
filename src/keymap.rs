use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Action {
    GoParent,
    GoIntoDir,
    GoUp,
    GoDown,
    Open,
    Quit,
}

pub struct Keymap {
    map: HashMap<String, Action>,
}

impl Keymap {
    pub fn from_config(config: &crate::config::Config) -> Self {
        let mut map = HashMap::new();
        let keys = config.keys();
        for key in keys.go_origin() {
            map.insert(key.clone(), Action::GoParent);
        }
        for key in keys.go_into_dir() {
            map.insert(key.clone(), Action::GoIntoDir);
        }
        for key in keys.go_up() {
            map.insert(key.clone(), Action::GoUp);
        }
        for key in keys.go_down() {
            map.insert(key.clone(), Action::GoDown);
        }
        for key in keys.open_file() {
            map.insert(key.clone(), Action::Open);
        }
        for key in keys.quit() {
            map.insert(key.clone(), Action::Quit);
        }
        Keymap { map }
    }

    pub fn lookup(&self, key: &str) -> Option<Action> {
        self.map.get(key).copied()
    }
}
