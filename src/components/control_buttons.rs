use crate::lib::{Element, MouseClickHandler};
use crate::AllStates;

use super::button::{button, ButtonProps};

pub struct ControlButtonsProps<'a> {
    pub percent: i32,
    pub on_less: MouseClickHandler<'a>,
    pub on_more: MouseClickHandler<'a>,
}

pub fn control_buttons(props: ControlButtonsProps) -> Element<AllStates> {
    let percent = props.percent;

    Element::Container(vec![
        button(ButtonProps {
            left: 10,
            top: 10,
            title: "Less",
            disable: percent <= 0,
            on_click: Some(props.on_less),
        }),
        button(ButtonProps {
            left: 45,
            top: 10,
            title: "Moar!",
            disable: percent >= 100,
            on_click: Some(props.on_more),
        }),
    ])
}
