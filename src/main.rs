use std::cell::RefCell;

mod component;
mod lib;
use lib::Node;

#[derive(Copy, Clone)]
pub struct State {
    percent: u16,
}

fn main() -> Result<(), String> {
    let state = RefCell::new(State { percent: 50 });
    lib::run(&app, &state)
}

fn app<'a>(state: &'a RefCell<State>) -> Node<'a> {
    Node::new(1, 1).set_children(Some(vec![
        component::header(),
        component::settings_controls(state),
        component::footer(),
    ]))
}
