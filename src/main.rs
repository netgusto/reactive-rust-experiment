use std::cell::RefCell;

mod lib;
use lib::Node;

#[derive(Copy, Clone)]
struct State {
    percent: u16,
}

fn main() -> Result<(), String> {
    let state = RefCell::new(State { percent: 50 });
    lib::run(&app, &state)
}

fn app<'a>(state: &'a RefCell<State>) -> Node<'a> {
    Node::new(1, 1).set_children(Some(vec![
        component_header(),
        component_settings_controls(state),
        component_footer(),
    ]))
}

fn component_settings_controls<'a>(state: &'a RefCell<State>) -> Option<Node<'a>> {
    let state_copy = state.borrow();
    Some(Node::new(1, 1).set_children(Some(vec![
        component_buttons(state),
        component_warning(state_copy.percent),
        component_progressbar(state_copy.percent),
    ])))
}

fn component_warning<'a>(percent: u16) -> Option<Node<'a>> {
    match percent {
        x if x <= 0 => {
            Some(Node::new(50, 27).set_text(Some("Can't go lower than 0!".to_string())))
        }
        x if x >= 100 => {
            Some(Node::new(50, 27).set_text(Some("You're at the maximum!".to_string())))
        }
        _ => None,
    }
}

fn component_progressbar<'a>(percent: u16) -> Option<Node<'a>> {
    Some(
        Node::new(10, 20)
            .set_text(Some(format!("{} %", percent)))
            .set_border(true)
            .set_width(percent)
            .set_height(3),
    )
}

fn component_buttons<'a>(state: &'a RefCell<State>) -> Option<Node<'a>> {
    let state_copy = state.borrow();
    Some(Node::new(1, 1).set_children(Some(vec![
        Some(
            Node::new(10, 10)
                .set_text(Some("Less".to_string()))
                .set_border(true)
                .set_width(30)
                .set_height(7)
                .disable(state_copy.percent <= 0)
                .set_on_mouse_click(Some(Box::new(move || {
                    let mut mutstate = state.borrow_mut();
                    let new_counter: i32 = (mutstate.percent as i32) - 10;
    
                    mutstate.percent = if new_counter >= 0 { new_counter } else { 0 } as u16;
                }))),
        ),
        Some(
            Node::new(45, 10)
                .set_text(Some("Moar!".to_string()))
                .set_border(true)
                .set_width(30)
                .set_height(7)
                .disable(state_copy.percent >= 100)
                .set_on_mouse_click(Some(Box::new(move || {
                    state.borrow_mut().percent += 10;
                }))),
        )
    ])))
}

fn component_header<'a>() -> Option<Node<'a>> {
    Some(Node::new(1, 1).set_text(Some("# Reactive TUI experiment with Rust".to_string())))
}

fn component_footer<'a>() -> Option<Node<'a>> {
    let dim = termion::terminal_size().unwrap();
    Some(Node::new(0, dim.1).set_text(Some("Quit: q".to_string())))
}
