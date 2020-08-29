use crate::lib::{Component, Element, MouseClickHandler, Node};
use crate::State;
use std::cell::RefCell;

use super::control_buttons::{control_buttons, ControlButtonsProps};
use super::progress_bar::{progress_bar, ProgressBarProps};
use super::warning::{warning, WarningProps};

pub struct SettingsControlsProps {
    pub increment: u16,
}

pub struct SettingsControls {
    pub props: SettingsControlsProps,
}

impl<'a> Component<'a, State> for SettingsControls {
    fn render(&self, state: &'a RefCell<State>) -> Element<'a, State> {
        let percent = state.borrow().percent;
        Element::Node(Node::new(1, 1).set_children(Some(vec![
            control_buttons(ControlButtonsProps {
                percent: percent,
                on_less: click_less(state, percent, self.props.increment),
                on_more: click_more(state, self.props.increment),
            }),
            warning(WarningProps { percent: percent }),
            progress_bar(ProgressBarProps { percent: percent }),
        ])))
    }
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
