use xcb::x::Window;
use crate::config::Config;
use xcb::Connection;

pub struct State {
    config: Config,
    focused_window: Option<Window>,
    connection: Connection,
    should_exit: bool
}

impl State {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection,
            focused_window: None,
            config: Config::default(),
            should_exit: false,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
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
}
