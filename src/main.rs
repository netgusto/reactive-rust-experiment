mod components;
mod lib;

use std::cell::RefCell;

use lib::{Element, Node};

use components::{
    footer::footer,
    header::{header, HeaderProps},
    settings_controls::{SettingsControls, SettingsControlsProps},
};

#[derive(Copy, Clone)]
pub struct State {
    pub percent: u16,
}

fn main() -> Result<(), String> {
    let state = RefCell::new(State { percent: 50 });
    lib::run(&app, &state)
}

fn app<'a>() -> Element<'a, State> {
    Element::Node(Node::new(1, 1).set_children(Some(vec![
        header(HeaderProps {
            text: "Reactive TUI experiment with Rust",
        }),
        Element::Component(Box::new(SettingsControls {
            props: SettingsControlsProps { increment: 10 },
        })),
        footer(),
    ])))
}
