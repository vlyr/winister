use crate::core::Keybind;
use xcb::x::{KeyButMask, ModMask};

pub struct Config {
    keybinds: Vec<Keybind>
}

impl Config {
    pub fn new() -> Self {
        Self {
            keybinds: vec![]
        }
    }

    pub fn get_keybind(&self, keycode: u8, state: KeyButMask) -> Option<&Keybind> {
        self.keybinds
            .iter()
            .find(|k| k.keycode == keycode && k.modifier.bits() == state.bits())
    }

    pub fn keybinds(&self) -> &Vec<Keybind> {
        &self.keybinds
    }
}

impl Default for Config {
    fn default() -> Self {
        use crate::core::KeybindAction::*;

        let mut keybinds = vec![
            Keybind {
                action: Command("alacritty"),
                keycode: 36, // enter
                modifier: ModMask::N4,
            },

            Keybind {
                action: Command("dmenu_run"),
                keycode: 33, // enter
                modifier: ModMask::N4,
            },

            Keybind {
                action: CloseFocused,
                keycode: 54,
                modifier: ModMask::N4 | ModMask::SHIFT
            }
        ];

        for i in 0..10 {
            keybinds.push(Keybind {
                action: MoveToWorkspace(i),
                keycode: i as u8 + 10,
                modifier: ModMask::N4,
            });

            keybinds.push(Keybind {
                action: MoveFocusedToWorkspace(i),
                keycode: i as u8 + 10,
                modifier: ModMask::N4 | ModMask::SHIFT,
            })
        }

        Self {
            keybinds,
        }
    }
}
