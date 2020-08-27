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

pub type MouseClickHandler<'a> = Box<dyn FnMut() -> () + 'a>;

pub struct Node<'a> {
    top: u16,
    left: u16,
    width: u16,
    height: u16,
    text: Option<String>,
    border: bool,
    on_mouse_click: Option<MouseClickHandler<'a>>,
    on_mouse_down: Option<MouseClickHandler<'a>>,
    disabled: bool,
    children: Option<Vec<Option<Node<'a>>>>,
}

pub trait Component<'a> {
    fn render() -> Node<'a>;
}

// pub trait StatefulComponent<'a> {
//     type State;
//     type Props;

//     fn new(initial_state: &mut Self::State) -> Self;
//     fn render(&self, props: &Self::Props) -> Option<Node<'a>>;
// }

impl<'a> Node<'a> {
    pub fn new(left: u16, top: u16) -> Node<'a> {
        Node {
            left: left,
            top: top,
            width: 1,
            height: 1,
            text: None,
            on_mouse_click: None,
            on_mouse_down: None,
            disabled: false,
            border: false,
            children: None,
        }
    }

    pub fn disable(mut self, dis: bool) -> Self {
        self.disabled = dis;
        self
    }

    pub fn set_border(mut self, b: bool) -> Self {
        self.border = b;
        self
    }

    pub fn set_text(mut self, t: Option<String>) -> Self {
        self.text = t;
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

    pub fn set_children(mut self, children: Option<Vec<Option<Node<'a>>>>) -> Self {
        self.children = children;
        self
    }

    pub fn set_on_mouse_click(mut self, handler: Option<MouseClickHandler<'a>>) -> Self {
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

    write!(stdout, "{}{}", termion::clear::All, cursor::Show).unwrap();
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
                match &mut children[i] {
                    Some(c) => track_mouse_down(c, left, top),
                    _ => (),
                }
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
                match &mut children[i] {
                    Some(c) => track_mouse_pressed(c, left, top),
                    _ => (),
                }
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

    let text = match &b.text {
        Some(t) => t.as_str(),
        None => "",
    };

    if b.border && b.height >= 3 {
        let width = if b.width >= 2 { b.width - 2 } else { 0 } as usize;
        write!(
            stdout,
            "{}{}{}{}{}{}",
            Goto(left + 1, top),
            "▀".repeat(width),
            Goto(left + 1, top + b.height - 1),
            "▄".repeat(width),
            Goto(
                left + (b.width / 2) - (text.len() as u16 / 2),
                top + (b.height / 2)
            ),
            if width == 0 {
                ""
            } else if text.len() > width {
                text.split_at(width).0
            } else {
                text
            }
        )
        .unwrap();
        for line in top..top + b.height {
            write!(
                stdout,
                "{}█{}█",
                Goto(left, line),
                Goto(left + b.width - 1, line),
            )
            .unwrap();
        }
    } else {
        write!(stdout, "{}{}", Goto(left, top), text).unwrap();
    }

    if b.disabled {
        write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
    }

    match &b.children {
        Some(children) => {
            for c in children {
                match c {
                    Some(n) => render_node(stdout, n),
                    _ => (),
                }
            }
        }
        None => (),
    }
}
