use xcb::x::Window;
use crate::config::Config;
use xcb::Connection;
use std::fs::File;
use std::fs::OpenOptions;

pub struct State {
    config: Config,
    focused_window: Option<Window>,
    connection: Connection,
    pub debug_file: File,
    should_exit: bool
}

impl State {
    pub fn new(connection: Connection) -> Self {
        let file = OpenOptions::new().append(true).write(true).read(true).create(true).open("./debug.txt").unwrap();

        Self {
            connection,
            focused_window: None,
            config: Config::default(),
            should_exit: false,
            debug_file: file
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
