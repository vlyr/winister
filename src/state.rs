use xcb::x::{ScreenBuf, Window};
use crate::config::Config;
use xcb::Connection;

pub struct State {
    config: Config,
    focused_window: Option<Window>,
    connection: Connection,
    should_exit: bool,
    screen_num: i32,
}

impl State {
    pub fn new(connection: Connection, screen_num: i32) -> Self {
        Self {
            connection,
            screen_num,
            focused_window: None,
            config: Config::default(),
            should_exit: false,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
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

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn screen_wh(&self) -> (u16, u16) {
        let setup = self.connection.get_setup();
        let screen = setup.roots().nth(self.screen_num as usize).unwrap();
        
        (screen.width_in_pixels(), screen.height_in_pixels())
    }
}
