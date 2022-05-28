use std::error::Error;
use crate::x::KeyButMask;
use xcb::x::{self, Event as XEvent};
use xcb::Event as XCBEvent;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::io::Write;
use std::panic::PanicInfo;

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

fn panic_handler(info: &PanicInfo) -> ! {
    std::fs::write("./panic.txt", format!("{}", info));
    std::process::exit(1);
}

fn on_key_press(state: &mut State, event: XEvent) {
    if let XEvent::KeyPress(ev) = event {
        state.debug_file.write(b"balls").unwrap();

        if let Some(keybind) = state.config().get_keybind(ev.detail()) {
            let mod_mask_bits = keybind.modifier.bits();

            std::fs::write("./panic.txt", format!("{:#?}, {}", mod_mask_bits, ev.state().bits()));
            if ev.state().bits() == mod_mask_bits {
                keybind.exec();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    std::process::Command::new("alacritty").spawn();
    std::panic::set_hook(Box::new(|info| panic_handler(info)));
    let (conn, screen) = xcb::Connection::connect(None)?;

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen as usize).unwrap();

    let root = screen.root();

    conn.send_request(&x::ChangeWindowAttributes {
        window: root,
        value_list: &[x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_NOTIFY)]
    });

    let mut state = State::new(conn);

    for keybind in state.config().keybinds().iter() {
        state.connection().send_request(&x::GrabKey {
            grab_window: root,
            key: keybind.keycode,
            keyboard_mode: x::GrabMode::Async,
            modifiers: keybind.modifier,
            owner_events: false,
            pointer_mode: x::GrabMode::Async
        });

    }

    state.connection().flush()?;

    loop {
        if state.should_exit() {
            break;
        }

        let event = state.connection().wait_for_event()?;

        if let XCBEvent::X(ev) = event {
            let ev_str = event_to_string(&ev);
            

            if let Some(f) = HANDLERS.get(&ev_str) {
                //fs::write("./debug.txt", format!("{:#?}", ev)).unwrap();
                f(&mut state, ev);
            }
        }
    }

    Ok(())
}
