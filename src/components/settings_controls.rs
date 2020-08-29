use crate::lib::{Element, MouseClickHandler, StatefulComponent};
use crate::AllStates;

use std::collections::HashMap;

use super::control_buttons::{control_buttons, ControlButtonsProps};
use super::progress_bar::{progress_bar, ProgressBarProps};
use super::warning::{warning, WarningProps};

pub struct SettingsControlsProps {
    pub increment: u16,
}

pub struct SettingsControls {
    pub props: SettingsControlsProps,
}

impl<'a> StatefulComponent<'a, AllStates> for SettingsControls {
    fn render(&self, state: Option<&mut AllStates>) -> Element<'a, AllStates> {
        let s = match state {
            Some(AllStates::ControlsSettingsState(s)) => s,
            _ => panic!("No state"),
        };

        use std::time::{SystemTime, UNIX_EPOCH};

        let percent = s.percent;
        if SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("msg: &str")
            .as_micros()
            % 2
            == 0
        {
            s.percent += 1;
        } else {
            s.percent -= 1;
        }
        // let increment = self.props.increment;

        Element::Container(vec![
            control_buttons(ControlButtonsProps {
                percent: percent,
                // on_less: click_less(state, percent, increment),
                // on_more: click_more(state, increment),
            }),
            warning(WarningProps { percent: percent }),
            progress_bar(ProgressBarProps { percent: percent }),
        ])
    }
}

fn click_less<'a>(
    _state: &'a HashMap<i32, AllStates>,
    _percent: u16,
    _increment: u16,
) -> MouseClickHandler {
    Box::new(move || {
        // let mut mutstate = state.borrow_mut();
        // let new_counter: i32 = (percent as i32) - increment as i32;

        // mutstate.percent = if new_counter >= 0 { new_counter } else { 0 } as u16;
    })
}

fn click_more<'a>(_state: &'a HashMap<i32, AllStates>, _increment: u16) -> MouseClickHandler {
    Box::new(move || {
        // state.borrow_mut().percent += increment;
    })
}
