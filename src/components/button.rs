use crate::lib::{Element, MouseClickHandler, Node};
use crate::State;

pub struct ButtonProps<'a> {
    pub left: u16,
    pub top: u16,
    pub title: &'a str,
    pub disable: bool,
    pub on_click: Option<MouseClickHandler<'a>>,
}

impl<'a> Default for ButtonProps<'a> {
    fn default() -> Self {
        ButtonProps {
            left: 1,
            top: 1,
            title: "",
            disable: false,
            on_click: None,
        }
    }
}

pub fn button<'a>(props: ButtonProps<'a>) -> Element<'a, State> {
    Element::Node(
        Node::new(props.left, props.top)
            .set_text(Some(props.title.to_string()))
            .set_border(true)
            .set_width(12 + props.title.len() as u16)
            .set_height(5)
            .disable(props.disable)
            .set_on_click(props.on_click),
    )
}
