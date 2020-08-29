use crate::lib::{Element, Node};
use crate::State;

pub struct ProgressBarProps {
    pub percent: u16,
}
pub fn progress_bar<'a>(props: ProgressBarProps) -> Element<'a, State> {
    Element::Node(
        Node::new(10, 20)
            .set_text(Some(format!("{} %", props.percent)))
            .set_border(true)
            .set_width(props.percent)
            .set_height(3),
    )
}
