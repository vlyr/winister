use xcb::x::{Window, ModMask, self};
use crate::state::State;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Workspace {
    windows: Vec<Window>
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            windows: vec![],
        }
    }

    pub fn windows(&self) -> &Vec<Window> {
        &self.windows
    }

    pub fn windows_mut(&mut self) -> &mut Vec<Window> {
        &mut self.windows
    }
}

#[derive(Debug, Clone)]
pub enum KeybindAction {
    Command(&'static str),
    MoveToWorkspace(usize),
    MoveFocusedToWorkspace(usize)
}

#[derive(Debug, Clone)]
pub struct Keybind {
    pub action: KeybindAction,
    pub keycode: u8,
    pub modifier: ModMask,
}

impl Keybind {
    pub fn exec(&self, state: &mut State) {
        match &self.action {
            KeybindAction::Command(cmd) => { Command::new(cmd).spawn().unwrap(); }

            KeybindAction::MoveToWorkspace(workspace_idx) => {
                for window in state.current_workspace().windows() {
                    state.connection().send_request(&x::UnmapWindow {
                        window: *window
                    });
                }

                state.set_current_workspace(*workspace_idx);

                for window in state.current_workspace().windows() {
                    state.connection().send_request(&x::MapWindow {
                        window: *window,
                    });
                }
            }

            KeybindAction::MoveFocusedToWorkspace(workspace_idx) => {}
        }
    }
}
