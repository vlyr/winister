use std::error::Error;
use xcb::x::{self, Event as XEvent};
use xcb::Event as XCBEvent;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::panic::PanicInfo;

use winister::{
    util::{debug, event_to_string},
    state::State,
};

lazy_static! {
    static ref HANDLERS: HashMap<&'static str, fn(&mut State, XEvent) -> ()> = {
        let mut map: HashMap<_, fn(&mut State, XEvent) -> ()> = HashMap::new();

        map.insert("key_press", on_key_press);
        map.insert("map_notify", on_map_notify);
        map.insert("other".into(), |_, _| {});

        map
    };
}

fn panic_handler(info: &PanicInfo) -> ! {
    debug(format!("{}", info));
    std::process::exit(1);
}

fn on_key_press(state: &mut State, event: XEvent) {
    if let XEvent::KeyPress(ev) = event {
        if let Some(keybind) = state.config().get_keybind(ev.detail()) {
            let keybind = keybind.clone();
            let mod_mask_bits = keybind.modifier.bits();

            if ev.state().bits() == mod_mask_bits {
                keybind.exec(state);
            }
        }
    }
}

fn on_map_notify(state: &mut State, event: XEvent) {
    if let XEvent::MapNotify(ev) = event {
        state.current_workspace_mut().windows_mut().push(ev.window());
        let (screen_width, screen_height) = state.screen_wh();

        state.connection().send_request(&x::ConfigureWindow {
            window: ev.window(),
            value_list: &[
                x::ConfigWindow::Width(screen_width as u32 - 4),
                x::ConfigWindow::Height(screen_height as u32 - 4),
                x::ConfigWindow::BorderWidth(2),
            ],
        });

        state.connection().send_request(&x::ChangeWindowAttributes {
            window: ev.window(),
            value_list: &[x::Cw::BorderPixel(0xff0000)],
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    std::process::Command::new("alacritty").spawn().unwrap();
    std::panic::set_hook(Box::new(|info| panic_handler(info)));
    let (conn, screen_num) = xcb::Connection::connect(None)?;

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let root = screen.root();

    conn.send_request(&x::ChangeWindowAttributes {
        window: root,
        value_list: &[x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_NOTIFY)]
    });

    let mut state = State::new(conn, screen_num, 10);

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
            debug(format!("{:#?}", ev));
            let ev_str = event_to_string(&ev);

            if let Some(f) = HANDLERS.get(&ev_str) {
                f(&mut state, ev);
            }
        }

        state.connection().flush().unwrap();
    }

    Ok(())
}
