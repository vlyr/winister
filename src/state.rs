use xcb::x::{ScreenBuf, Window};
use crate::{core::Workspace, config::Config};
use xcb::Connection;

pub struct State {
    config: Config,
    focused_window: Option<Window>,
    connection: Connection,
    should_exit: bool,
    screen_num: i32,
    workspaces: Vec<Workspace>,
    current_workspace: usize,
}

impl State {
    pub fn new(connection: Connection, screen_num: i32, workspace_count: u32) -> Self {
        Self {
            connection,
            screen_num,
            focused_window: None,
            config: Config::default(),
            should_exit: false,
            workspaces: vec![Workspace::new(); 10],
            current_workspace: 0,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn current_workspace(&self) -> &Workspace {
        self.workspaces.get(self.current_workspace).unwrap()
    }

    pub fn current_workspace_mut(&mut self) -> &mut Workspace {
        self.workspaces.get_mut(self.current_workspace).unwrap()
    }

    pub fn set_current_workspace(&mut self, new_value: usize) {
        self.current_workspace = new_value;
    }
    pub fn screen_num(&self) -> i32 {
        self.screen_num
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn focused_window(&self) -> Option<Window> {
        self.focused_window
    }

    pub fn workspaces(&self) -> &Vec<Workspace> {
        &self.workspaces
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn screen_wh(&self) -> (u16, u16) {
        let setup = self.connection.get_setup();
        let screen = setup.roots().nth(self.screen_num as usize).unwrap();
        
        (screen.width_in_pixels(), screen.height_in_pixels())
    }
}
