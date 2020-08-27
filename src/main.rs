use std::cell::RefCell;

mod lib;
use lib::Node;

#[derive(Copy, Clone)]
struct State {
    counter: i32,
}

fn main() -> Result<(), String> {
    let state = RefCell::new(State { counter: 0 });
    lib::run(&app, &state)
}

fn app<'a>(state: &'a RefCell<State>) -> Node<'a> {
    let state_copy = state.borrow();
    Node::new(1, 1, "SUPER")
        .set_width(10)
        .set_height(3)
        .set_on_mouse_click(Some(Box::new(move || {
            state.borrow_mut().counter += 100;
        })))
        .set_children(Some(vec![
            Node::new(10, 10, "Less")
                .set_width(30)
                .set_height(7)
                .disable(state_copy.counter <= 0)
                .set_on_mouse_click(Some(Box::new(move || {
                    let mut mutstate = state.borrow_mut();
                    let new_counter = mutstate.counter - 5;

                    mutstate.counter = if new_counter >= 0 { new_counter } else { 0 };
                }))),
            Node::new(45, 10, "Moar!")
                .set_width(30)
                .set_height(7)
                .set_on_mouse_click(Some(Box::new(move || {
                    state.borrow_mut().counter += 5;
                }))),
            Node::new(27, 30, &format!("{}", state_copy.counter))
                .set_width(if state_copy.counter > 0 {
                    state_copy.counter as u16
                } else {
                    0
                })
                .set_height(7),
        ]))
}

// fn component_stuff<'a>(state: &'a RefCell<State>) -> Node<'a> {
//     Node::new(30, 30, "Component stuff").set_children(Some(vec![
//         Node::new(40, 40, "Clickable Children")
//             .set_width(30)
//             .set_height(5)
//             .set_on_mouse_click(Some(Box::new(move || {
//                 state.borrow_mut().counter -= 1;
//             }))),
//         Node::new(50, 50, "50!").set_height(10).set_width(30),
//     ]))
// }
