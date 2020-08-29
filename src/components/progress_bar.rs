use crate::lib::{Element, Node};
use crate::AllStates;

pub struct ProgressBarProps {
    pub percent: i32,
}
pub fn progress_bar<'a>(props: ProgressBarProps) -> Element<'a, AllStates> {
    Element::Node(
        Node::new(10, 20)
            .set_text(Some(format!("{} %", props.percent)))
            .set_border(true)
            .set_width(if props.percent <= 0 { 0 } else { props.percent } as u16)
            .set_height(3),
    )
}
