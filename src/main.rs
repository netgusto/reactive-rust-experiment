mod components;
mod lib;

use lib::{new_state_box, Container, Element};

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
    lib::run(&app, &new_state_box(State { percent: 50 }))
}

fn app<'a>() -> Element<'a, State> {
    Element::Container(Container::new().set_children(Some(vec![
        header(HeaderProps {
            text: "Reactive TUI experiment with Rust",
        }),
        Element::Component(Box::new(SettingsControls {
            props: SettingsControlsProps { increment: 10 },
        })),
        footer(),
    ])))
}
