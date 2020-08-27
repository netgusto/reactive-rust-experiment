use crate::lib::{MouseClickHandler, Node};
use crate::State;
use std::cell::RefCell;

pub fn settings_controls<'a>(state: &'a RefCell<State>) -> Option<Node<'a>> {
    let percent = state.borrow().percent;
    Some(Node::new(1, 1).set_children(Some(vec![
        control_buttons(ControlButtonsProps { percent: percent }, state),
        warning(WarningProps { percent: percent }),
        progress_bar(ProgressBarProps { percent: percent }),
    ])))
}

pub struct WarningProps {
    percent: u16,
}

pub fn warning<'a>(props: WarningProps) -> Option<Node<'a>> {
    match &props.percent {
        &x if x <= 0 => {
            Some(Node::new(50, 27).set_text(Some("Can't go lower than 0!".to_string())))
        }
        &x if x >= 100 => {
            Some(Node::new(50, 27).set_text(Some("You're at the maximum!".to_string())))
        }
        _ => None,
    }
}

pub struct ProgressBarProps {
    percent: u16,
}
pub fn progress_bar<'a>(props: ProgressBarProps) -> Option<Node<'a>> {
    Some(
        Node::new(10, 20)
            .set_text(Some(format!("{} %", props.percent)))
            .set_border(true)
            .set_width(props.percent)
            .set_height(3),
    )
}

pub struct ControlButtonsProps {
    percent: u16,
}
pub fn control_buttons<'a>(
    props: ControlButtonsProps,
    state: &'a RefCell<State>,
) -> Option<Node<'a>> {
    let percent = props.percent;

    Some(Node::new(1, 1).set_children(Some(vec![
        button(ButtonProps {
            left: 10,
            top: 10,
            title: "Less",
            disable: percent <= 0,
            on_mouse_click: Some(click_less(percent, state)),
        }),
        button(ButtonProps {
            left: 45,
            top: 10,
            title: "Moar!",
            disable: percent >= 100,
            on_mouse_click: Some(click_more(state)),
        }),
    ])))
}

fn click_less<'a>(percent: u16, state: &'a RefCell<State>) -> MouseClickHandler {
    Box::new(move || {
        let mut mutstate = state.borrow_mut();
        let new_counter: i32 = (percent as i32) - 10;

        mutstate.percent = if new_counter >= 0 { new_counter } else { 0 } as u16;
    })
}

fn click_more<'a>(state: &'a RefCell<State>) -> MouseClickHandler {
    Box::new(move || {
        state.borrow_mut().percent += 10;
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

pub fn button<'a>(props: ButtonProps<'a>) -> Option<Node<'a>> {
    Some(
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
pub fn header<'a>(props: HeaderProps) -> Option<Node<'a>> {
    Some(Node::new(1, 1).set_text(Some(format!("# {}", props.text))))
}

pub fn footer<'a>() -> Option<Node<'a>> {
    let dim = termion::terminal_size().unwrap();
    Some(Node::new(0, dim.1).set_text(Some("Quit: q".to_string())))
}
