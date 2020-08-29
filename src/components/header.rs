use crate::lib::{Element, Node};
use crate::State;

pub struct HeaderProps<'a> {
    pub text: &'a str,
}
pub fn header<'a>(props: HeaderProps) -> Element<'a, State> {
    Element::Node(Node::new(1, 1).set_text(Some(format!("# {}", props.text))))
}
