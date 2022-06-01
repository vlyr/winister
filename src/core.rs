use xcb::x::{Window, ModMask, self};
use crate::state::State;
use std::process::Command;
use xcb::Xid;
use crate::layout::Layout;
use crate::util::debug;

#[derive(Debug, Clone)]
pub struct Workspace {
    windows: Vec<Window>,
    focused_window: Option<Window>
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            windows: vec![],
            focused_window: None,
        }
    }

    pub fn windows(&self) -> &Vec<Window> {
        &self.windows
    }

    pub fn windows_mut(&mut self) -> &mut Vec<Window> {
        &mut self.windows
    }

    pub fn add_window(&mut self, window: Window) {
        if let None = self.windows.iter().find(|win| *win == &window) {
            self.windows.push(window);
        }
    }

    pub fn remove_window(&mut self, window: Window) {
        if let Some(idx) = self.windows.iter().position(|win| win == &window) {
            self.windows.remove(idx);
        }
    }

    pub fn resize(&mut self, layout: Layout, width: u32, height: u32, border_width: u32, state: &State) {
        let count = self.windows.len();

        let areas = layout.generate_window_sizes(
            count,
            0,
            0, // gap
            0, // gap
            width,
            height,
            1
        );

        for (i, window) in self.windows.iter_mut().enumerate() {
            let area = &areas[i];

            state.connection().send_request(&x::ConfigureWindow {
                window: *window,
                value_list: &[
                    x::ConfigWindow::X(area.x as i32),
                    x::ConfigWindow::Y(area.y as i32),
                    x::ConfigWindow::Width(area.width),
                    x::ConfigWindow::Height(area.height),
                    x::ConfigWindow::BorderWidth(border_width)
                ]
            });

            state.connection().send_request(&x::ChangeWindowAttributes {
                window: *window,
                value_list: &[x::Cw::BorderPixel(0xff0000)],
            });

            if let Some(win) = state.focused_window() {
                if win == *window {
                    state.connection().send_request(&x::SetInputFocus {
                        focus: *window,
                        revert_to: x::InputFocus::Parent,
                        time: 0,
                    });
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum KeybindAction {
    Command(&'static str),
    MoveToWorkspace(usize),
    MoveFocusedToWorkspace(usize),
    CloseFocused,
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
                if *workspace_idx != state.current_workspace_idx() {
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
            }

            KeybindAction::MoveFocusedToWorkspace(workspace_idx) => {
                if let Some(win) = state.focused_window() {
                    state.connection().send_request(&x::UnmapWindow {
                        window: win
                    });

                    state.connection().flush().unwrap();

                    let window_idx = state.current_workspace_mut()
                        .windows_mut()
                        .iter()
                        .position(|w| w == &win)
                        .unwrap();

                    state.current_workspace_mut().windows_mut().remove(window_idx);

                    state.workspaces_mut()
                        .get_mut(*workspace_idx)
                        .unwrap()
                        .windows_mut()
                        .push(win)
                }
            }

            KeybindAction::CloseFocused => {
                if let Some(win) = state.focused_window() {
                    state.connection().send_request(&x::KillClient {
                        resource: win.resource_id(),
                    });

                    let windows = state.current_workspace_mut().windows_mut();
                    windows.retain(|w| w != &win);

                    if !windows.is_empty() {
                        let window = windows.get(windows.len() - 1).unwrap().clone();
                        state.set_focused_window(Some(window));
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
