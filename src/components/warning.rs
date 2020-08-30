use crate::lib::{Element, Node};
use crate::AllStates;

pub struct WarningProps {
    pub percent: i32,
}

pub fn warning<'a>(props: WarningProps) -> Element<'a, AllStates> {
    match props.percent {
        x if x <= 0 => {
            Element::Node(Node::new(50, 27).set_text(Some("Can't go lower than 0!".to_string())))
        }
        x if x >= 100 => {
            Element::Node(Node::new(50, 27).set_text(Some("You're at the maximum!".to_string())))
        }
        _ => Element::None,
    }
}
