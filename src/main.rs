use std::cell::RefCell;

mod component;
mod lib;
use lib::{Element, Node, State};

use component::{footer, header, HeaderProps, SettingsControls, SettingsControlsProps};

fn main() -> Result<(), String> {
    let state = RefCell::new(State { percent: 50 });
    lib::run(&app, &state)
}

fn app<'a>() -> Element<'a> {
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
