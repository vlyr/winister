use std::error::Error;
use crate::x::KeyButMask;
use xcb::x::{self, Event as XEvent};
use xcb::Event as XCBEvent;
use std::collections::HashMap;
use lazy_static::lazy_static;

use winister::{
    util::event_to_string,
    state::State,
};

lazy_static! {
    static ref HANDLERS: HashMap<&'static str, fn(&mut State, XEvent) -> ()> = {
        let mut map: HashMap<_, fn(&mut State, XEvent) -> ()> = HashMap::new();

        map.insert("key_press", on_key_press);
        map.insert("other".into(), |_, _| {});

        map
    };
}

fn on_key_press(state: &mut State, event: XEvent) {
    if let XEvent::KeyPress(ev) = event {
        if let Some(keybind) = state.config().get_keybind(ev.detail()) {
            let mod_mask_bits = keybind.modifier.bits();

            if ev.state() == KeyButMask::from_bits(mod_mask_bits).unwrap() {
                keybind.exec();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (conn, screen) = xcb::Connection::connect(None)?;

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen as usize).unwrap();

    let root = screen.root();

    conn.send_request(&x::ChangeWindowAttributes {
        window: root,
        value_list: &[x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_NOTIFY)]
    });

    conn.send_request(&x::GrabKey {
        grab_window: root,
        key: 11,
        keyboard_mode: x::GrabMode::Async,
        modifiers: x::ModMask::N4,
        owner_events: false,
        pointer_mode: x::GrabMode::Async
    });

    conn.flush()?;

    let mut state = State::new(conn);

    loop {
        if state.should_exit() {
            break;
        }

        let event = state.connection().wait_for_event()?;

        if let XCBEvent::X(ev) = event {
            let ev_str = event_to_string(&ev);

            if let Some(f) = HANDLERS.get(&ev_str) {
                f(&mut state, ev);
            }
        }
    }

    Ok(())
}
