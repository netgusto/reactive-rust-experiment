mod components;
mod lib;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use lib::Element;

use components::{
    footer::footer,
    header::{header, HeaderProps},
    settings_controls::{SettingsControls, SettingsControlsProps},
};

pub enum AllStates {
    ControlsSettingsState(State),
}

// impl Into<State> for AllStates {
//     fn into(self) -> State {
//         match self {
//             AllStates::ControlsSettingsState(u) => u,
//             _ => panic!(),
//         }
//     }
// }

#[derive(Debug)]
pub struct State {
    pub percent: i32,
}

fn main() -> Result<(), String> {
    let mut state_store: HashMap<i32, Rc<RefCell<AllStates>>> = HashMap::new();
    state_store.insert(
        42, // component instance key; here, dummy value (component identification not implemented)
        Rc::new(RefCell::new(AllStates::ControlsSettingsState(State {
            percent: 36,
        }))),
    );
    lib::run(&app, &state_store)
}

fn app<'a>() -> Element<'a, AllStates> {
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
