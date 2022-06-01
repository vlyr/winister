use xcb::x::{self, Event as XEvent, ModMask};
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
        map.insert("button_press", on_button_press);
        map.insert("map_notify", on_map_notify);
        map.insert("destroy_notify", on_destroy_notify);
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
        if let Some(keybind) = state.config().get_keybind(ev.detail(), ev.state()) {              
            let keybind = keybind.clone();
            keybind.exec(state);
        }
    }
}

fn on_button_press(state: &mut State, event: XEvent) {
    if let XEvent::ButtonPress(ev) = event {
        state.connection().send_request(&x::SetInputFocus {
            focus: ev.child(),
            revert_to: x::InputFocus::Parent,
            time: xcb::x::CURRENT_TIME,
        });
    }
}

fn on_map_notify(state: &mut State, event: XEvent) {
    if let XEvent::MapNotify(ev) = event {
        let workspace = state.current_workspace_mut();
        workspace.add_window(ev.window());

        let mut workspace = workspace.clone();

        let (sw, sh) = state.screen_wh();

        workspace.resize(
            winister::layout::Layout::Winister,
            sw as u32,
            sh as u32,
            2,
            state
        );

        state.connection().send_request(&x::ConfigureWindow {
            window: ev.window(),
            value_list: &[
                x::ConfigWindow::Width(sw as u32 - 4),
                x::ConfigWindow::Height(sh as u32 - 4),
                x::ConfigWindow::BorderWidth(2),
            ],
        });

        state.connection().send_request(&x::ChangeWindowAttributes {
            window: ev.window(),
            value_list: &[x::Cw::BorderPixel(0xff0000)],
        });

        state.set_focused_window(Some(ev.window()));
    }
}

fn on_destroy_notify(state: &mut State, event: XEvent) {
    if let XEvent::DestroyNotify(ev) = event {
        let workspace = state.current_workspace_mut();
        workspace.remove_window(ev.window());

        let mut workspace = workspace.clone();

        let (sw, sh) = state.screen_wh();

        workspace.resize(
            winister::layout::Layout::Winister,
            sw as u32,
            sh as u32,
            2,
            state
        );
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| panic_handler(info)));
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();

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

    for idx in [x::ButtonIndex::N1, x::ButtonIndex::N2] {
        state.connection().send_request(&x::GrabButton {
            button: idx,
            owner_events: false,
            cursor: xcb::x::CURSOR_NONE,
            confine_to: root,
            modifiers: ModMask::ANY,
            event_mask: x::EventMask::BUTTON_PRESS | x::EventMask::BUTTON_RELEASE,
            grab_window: root,
            keyboard_mode: x::GrabMode::Async,
            pointer_mode: x::GrabMode::Async,
        });
    }

    state.connection().flush().unwrap();

    loop {
        if state.should_exit() {
            break;
        }

        match state.connection().wait_for_event() {
            Ok(event) => {
                if let XCBEvent::X(ev) = event {
                    debug(format!("{:#?}", ev));
                    let ev_str = event_to_string(&ev);
                    
                    if let Some(f) = HANDLERS.get(&ev_str) {
                        f(&mut state, ev);
                    }
                }
            }
            Err(_) => ()
        }

        state.connection().flush().unwrap();
    }
}
