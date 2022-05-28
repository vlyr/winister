use xcb::x::ModMask;
use std::process::Command;

pub enum Action {
    Command(&'static str),
    MoveToWorkspace(usize),
    MoveFocusedToWorkspace(usize)
}

pub struct Keybind {
    pub action: Action,
    pub keycode: u8,
    pub modifier: ModMask,
}

impl Keybind {
    pub fn exec(&self) {
        match &self.action {
            Action::Command(cmd) => { Command::new(cmd).spawn().unwrap(); }
            _ => ()
        }
    }
}
