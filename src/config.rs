use crate::keybind::Keybind;
use xcb::x::ModMask;

pub struct Config {
    keybinds: Vec<Keybind>
}

impl Config {
    pub fn new() -> Self {
        Self {
            keybinds: vec![]
        }
    }

    pub fn get_keybind(&self, keycode: u8) -> Option<&Keybind> {
        self.keybinds.iter().find(|k| k.keycode == keycode)
    }

    pub fn keybinds(&self) -> &Vec<Keybind> {
        &self.keybinds
    }
}

impl Default for Config {
    fn default() -> Self {
        use crate::keybind::Action::*;

        let keybinds = vec![
            Keybind {
                action: Command("alacritty"),
                keycode: 11, // enter
                modifier: ModMask::N4,
            },
        ];

        Self {
            keybinds,
        }
    }
}
