mod components;
mod lib;

use lib::{new_state_box, Element};

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
    Element::Container(vec![
        header(HeaderProps {
            text: "Reactive TUI experiment with Rust",
        }),
        Element::StatefulComponent(Box::new(SettingsControls {
            props: SettingsControlsProps { increment: 10 },
        })),
        footer(),
    ])
}
