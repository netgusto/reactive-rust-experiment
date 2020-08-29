use crate::lib::{Element, Node};
use crate::AllStates;

pub fn footer<'a>() -> Element<'a, AllStates> {
    let dim = termion::terminal_size().unwrap();
    Element::Node(Node::new(0, dim.1).set_text(Some("Quit: q".to_string())))
}
