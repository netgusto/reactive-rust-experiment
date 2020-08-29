use std::cell::RefCell;

mod component;
mod lib;
use lib::{Element, Node};

use component::{HeaderProps, SettingsControlsProps};

#[derive(Copy, Clone)]
pub struct State {
    percent: u16,
}

fn main() -> Result<(), String> {
    let state = RefCell::new(State { percent: 50 });
    lib::run(&app, &state)
}

fn app<'a>(state: &'a RefCell<State>) -> Element<'a> {
    Element::Node(Node::new(1, 1).set_children(Some(vec![
        component::header(HeaderProps {
            text: "Reactive TUI experiment with Rust",
        }),
        component::settings_controls(SettingsControlsProps { increment: 1 }, state),
        component::footer(),
    ])))
}
