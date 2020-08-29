use crate::lib::{Element, MouseClickHandler, Node};
use crate::State;
use std::cell::RefCell;

pub struct SettingsControlsProps {
    pub increment: u16,
}

pub fn settings_controls<'a>(props: SettingsControlsProps, state: &'a RefCell<State>) -> Element {
    let percent = state.borrow().percent;
    Element::Node(Node::new(1, 1).set_children(Some(vec![
        control_buttons(ControlButtonsProps {
            percent: percent,
            on_less: click_less(state, percent, props.increment),
            on_more: click_more(state, props.increment),
        }),
        warning(WarningProps { percent: percent }),
        progress_bar(ProgressBarProps { percent: percent }),
    ])))
}

pub struct WarningProps {
    percent: u16,
}

pub fn warning<'a>(props: WarningProps) -> Element<'a> {
    match &props.percent {
        &x if x <= 0 => {
            Element::Node(Node::new(50, 27).set_text(Some("Can't go lower than 0!".to_string())))
        }
        &x if x >= 100 => {
            Element::Node(Node::new(50, 27).set_text(Some("You're at the maximum!".to_string())))
        }
        _ => Element::None,
    }
}

pub struct ProgressBarProps {
    percent: u16,
}
pub fn progress_bar<'a>(props: ProgressBarProps) -> Element<'a> {
    Element::Node(
        Node::new(10, 20)
            .set_text(Some(format!("{} %", props.percent)))
            .set_border(true)
            .set_width(props.percent)
            .set_height(3),
    )
}

pub struct ControlButtonsProps<'a> {
    percent: u16,
    on_less: MouseClickHandler<'a>,
    on_more: MouseClickHandler<'a>,
}

pub fn control_buttons<'a>(props: ControlButtonsProps<'a>) -> Element {
    let percent = props.percent;

    Element::Node(Node::new(1, 1).set_children(Some(vec![
        button(ButtonProps {
            left: 10,
            top: 10,
            title: "Less",
            disable: percent <= 0,
            on_mouse_click: Some(props.on_less),
        }),
        button(ButtonProps {
            left: 45,
            top: 10,
            title: "Moar!",
            disable: percent >= 100,
            on_mouse_click: Some(props.on_more),
        }),
    ])))
}

fn click_less<'a>(state: &'a RefCell<State>, percent: u16, increment: u16) -> MouseClickHandler {
    Box::new(move || {
        let mut mutstate = state.borrow_mut();
        let new_counter: i32 = (percent as i32) - increment as i32;

        mutstate.percent = if new_counter >= 0 { new_counter } else { 0 } as u16;
    })
}

fn click_more<'a>(state: &'a RefCell<State>, increment: u16) -> MouseClickHandler {
    Box::new(move || {
        state.borrow_mut().percent += increment;
    })
}

pub struct ButtonProps<'a> {
    left: u16,
    top: u16,
    title: &'a str,
    disable: bool,
    on_mouse_click: Option<MouseClickHandler<'a>>,
}

impl<'a> Default for ButtonProps<'a> {
    fn default() -> Self {
        ButtonProps {
            left: 1,
            top: 1,
            title: "",
            disable: false,
            on_mouse_click: None,
        }
    }
}

pub fn button<'a>(props: ButtonProps<'a>) -> Element<'a> {
    Element::Node(
        Node::new(props.left, props.top)
            .set_text(Some(props.title.to_string()))
            .set_border(true)
            .set_width(30)
            .set_height(7)
            .disable(props.disable)
            .set_on_mouse_click(props.on_mouse_click),
    )
}

pub struct HeaderProps<'a> {
    pub text: &'a str,
}
pub fn header<'a>(props: HeaderProps) -> Element<'a> {
    Element::Node(Node::new(1, 1).set_text(Some(format!("# {}", props.text))))
}

pub fn footer<'a>() -> Element<'a> {
    let dim = termion::terminal_size().unwrap();
    Element::Node(Node::new(0, dim.1).set_text(Some("Quit: q".to_string())))
}
