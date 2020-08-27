use std::thread;

use std::io::{stdout, Write};
use termion::color;
use termion::cursor;
use termion::cursor::Goto;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{Events, MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, AsyncReader};

use std::cell::RefCell;

#[allow(dead_code)]
pub struct Node<'a> {
    top: u16,
    left: u16,
    width: u16,
    height: u16,
    text: String,
    on_mouse_click: Option<Box<dyn FnMut() -> () + 'a>>,
    on_mouse_down: Option<Box<dyn FnMut() -> () + 'a>>,
    disabled: bool,
    children: Option<Vec<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn new(left: u16, top: u16, text: &str) -> Node<'a> {
        Node {
            left: left,
            top: top,
            width: 1,
            height: 1,
            text: String::from(text),
            on_mouse_click: None,
            on_mouse_down: None,
            disabled: false,
            children: None,
        }
    }

    pub fn disable(mut self, dis: bool) -> Self {
        self.disabled = dis;
        self
    }

    pub fn set_width(mut self, w: u16) -> Self {
        self.width = w;
        self
    }

    pub fn set_height(mut self, h: u16) -> Self {
        self.height = h;
        self
    }

    pub fn set_children(mut self, children: Option<Vec<Node<'a>>>) -> Self {
        self.children = children;
        self
    }

    pub fn set_on_mouse_click(mut self, handler: Option<Box<dyn FnMut() -> () + 'a>>) -> Self {
        self.on_mouse_click = handler;
        self
    }
}

pub fn run<'a, T>(
    app_maker: &dyn Fn(&'a RefCell<T>) -> Node<'a>,
    state: &'a RefCell<T>,
) -> Result<(), String> {
    let stdin = async_stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}", termion::clear::All, cursor::Hide).unwrap();

    let mut events_it = stdin.events();

    loop {
        write!(stdout, "{}{}", termion::clear::All, cursor::Hide).unwrap();
        let mut app = app_maker(&state);

        match process_events(&mut events_it, &mut stdout, &mut app) {
            true => break,
            _ => (),
        };

        render_node(&mut stdout, &app);
        stdout.flush().unwrap();

        #[allow(deprecated)]
        thread::sleep_ms(16);
    }

    write!(stdout, "{}{}", termion::clear::All, cursor::Hide).unwrap();
    stdout.flush().unwrap();
    Ok(())
}

fn process_events(
    events_it: &mut Events<AsyncReader>,
    stdout: &mut RawTerminal<std::io::Stdout>,
    app: &mut Node,
) -> bool {
    loop {
        let event = events_it.next();
        match event {
            None => return false,
            Some(Ok(Event::Key(Key::Char(c)))) => match c {
                'q' => return true,
                'c' => write!(stdout, "{}", termion::clear::All).unwrap(),
                _ => (),
            },
            Some(Ok(Event::Mouse(me))) => match me {
                MouseEvent::Press(_, left, top) => {
                    track_mouse_down(app, left, top);
                }
                MouseEvent::Release(left, top) => {
                    track_mouse_pressed(app, left, top);
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn track_mouse_down(node: &mut Node, left: u16, top: u16) {
    if node.disabled {
        return;
    }

    if aabb_contains(node.left, node.top, node.width, node.height, left, top) {
        match &mut node.on_mouse_down {
            Some(c) => return c(),
            _ => (),
        }
    }

    match &mut node.children {
        None => return,
        Some(children) => {
            for i in 0..children.len() {
                track_mouse_down(&mut children[i], left, top);
            }
        }
    }
}

fn track_mouse_pressed(node: &mut Node, left: u16, top: u16) {
    if node.disabled {
        return;
    }
    if aabb_contains(node.left, node.top, node.width, node.height, left, top) {
        match &mut node.on_mouse_click {
            Some(c) => return c(),
            _ => (),
        }
    }

    match &mut node.children {
        None => return,
        Some(children) => {
            for i in 0..children.len() {
                track_mouse_pressed(&mut children[i], left, top);
            }
        }
    }
}

fn aabb_contains(
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    point_left: u16,
    point_top: u16,
) -> bool {
    left <= point_left
        && left + width >= point_left
        && top <= point_top
        && top + height >= point_top
}

fn render_node(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, b: &Node) {
    let left = b.left;
    let top = b.top;

    if b.disabled {
        write!(stdout, "{}", color::Fg(color::Yellow)).unwrap();
    }

    if b.height >= 3 {
        write!(
            stdout,
            "{}{}{}{}{}{}",
            Goto(left, top),
            "▄".repeat(1 + b.width as usize),
            Goto(left, top + b.height),
            "▀".repeat(1 + b.width as usize),
            Goto(
                left + (b.width / 2) - (b.text.len() as u16 / 2),
                top + (b.height / 2)
            ),
            b.text,
        )
        .unwrap();
        for line in top + 1..top + b.height {
            write!(
                stdout,
                "{}█{}█",
                Goto(left, line),
                Goto(left + b.width, line),
            )
            .unwrap();
        }
    } else {
        write!(stdout, "{}{}", Goto(left, top), b.text).unwrap();
    }

    if b.disabled {
        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
    }

    match &b.children {
        Some(children) => {
            for c in children {
                render_node(stdout, c)
            }
        }
        None => (),
    }
}
