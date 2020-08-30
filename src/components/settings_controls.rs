use crate::lib::{Element, MouseClickHandler, StatefulComponent};
use crate::{AllStates, State};

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use super::control_buttons::{control_buttons, ControlButtonsProps};
use super::progress_bar::{progress_bar, ProgressBarProps};
use super::warning::{warning, WarningProps};

pub struct SettingsControlsProps {
    pub increment: u16,
}

pub struct SettingsControls {
    pub props: SettingsControlsProps,
}

// pub fn extract<T>

impl<'a> StatefulComponent<'a, AllStates> for SettingsControls {
    fn render(&self, state: Rc<RefCell<AllStates>>) -> Element<'a, AllStates> {
        let bs = state.borrow_mut();
        let s = match &*bs {
            AllStates::ControlsSettingsState(v) => v,
            _ => panic!("NOOOOOO"),
        };

        let percent = s.percent;
        let increment = self.props.increment;

        Element::Container(vec![
            control_buttons(ControlButtonsProps {
                percent: percent,
                on_less: click_less(state.clone(), increment),
                on_more: click_more(state.clone(), increment),
            }),
            warning(WarningProps { percent: percent }),
            progress_bar(ProgressBarProps { percent: percent }),
        ])
    }
}

fn click_less<'a>(state: Rc<RefCell<AllStates>>, increment: u16) -> MouseClickHandler<'a> {
    Box::new(move || {
        let mut bs = state.borrow_mut();
        let s = match &mut *bs {
            AllStates::ControlsSettingsState(v) => v,
            _ => panic!("NOOOOOO"),
        };

        let new_counter: i32 = s.percent - increment as i32;
        s.percent = if new_counter >= 0 { new_counter } else { 0 };
    })
}

fn click_more<'a>(state: Rc<RefCell<AllStates>>, increment: u16) -> MouseClickHandler<'a> {
    Box::new(move || {
        let mut bs = state.borrow_mut();
        let s = match &mut *bs {
            AllStates::ControlsSettingsState(v) => v,
            _ => panic!("NOOOOOO"),
        };

        let new_counter: i32 = s.percent + increment as i32;
        s.percent = if new_counter <= 100 { new_counter } else { 100 };
    })
}
