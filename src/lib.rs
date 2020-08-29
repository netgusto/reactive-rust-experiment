use std::{thread::sleep, time::Duration};

use std::io::{stdout, Stdout, Write};
use termion::color;
use termion::cursor;
use termion::cursor::Goto;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{Events, MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, AsyncReader};

use std::cell::RefCell;

pub trait Component<'a, TState> {
    fn render(&self, state: &'a RefCell<TState>) -> Element<'a, TState>;
}

pub enum Element<'a, TState> {
    Node(Node<'a, TState>),
    Component(Box<dyn Component<'a, TState>>),
    None,
}

pub type MouseClickHandler<'a> = Box<dyn FnMut() -> () + 'a>;
pub type Children<'a, TState> = Vec<Element<'a, TState>>;

pub struct Node<'a, TState> {
    text: Option<String>,
    children: Option<Children<'a, TState>>,
    top: u16,
    left: u16,
    width: u16,
    height: u16,
    border: bool,
    on_mouse_click: Option<MouseClickHandler<'a>>,
    on_mouse_down: Option<MouseClickHandler<'a>>,
    disabled: bool,
}

impl<'a, TState> Node<'a, TState> {
    pub fn new(left: u16, top: u16) -> Node<'a, TState> {
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

    pub fn set_children(mut self, children: Option<Children<'a, TState>>) -> Self {
        self.children = children;
        self
    }

    pub fn set_on_mouse_click(mut self, handler: Option<MouseClickHandler<'a>>) -> Self {
        self.on_mouse_click = handler;
        self
    }
}

pub fn run<'a, TState>(
    app_maker: &dyn Fn() -> Element<'a, TState>,
    state: &'a RefCell<TState>,
) -> Result<(), String> {
    let stdin = async_stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}", termion::clear::All, cursor::Hide).unwrap();

    let mut events_it = stdin.events();

    let mut current_app: Option<Element<TState>> = None;

    loop {
        write!(stdout, "{}{}", termion::clear::All, cursor::Hide).unwrap();

        match &mut current_app {
            None => (),
            Some(some) => match process_events(&mut events_it, &mut stdout, some) {
                true => break,
                _ => (),
            },
        }

        let mut rendered = render_element(app_maker(), state);
        draw_node(&mut stdout, &mut rendered, state);
        current_app = Some(rendered);
        stdout.flush().unwrap();

        sleep(Duration::from_millis(16));
    }

    write!(stdout, "{}{}", termion::clear::All, cursor::Show).unwrap();
    stdout.flush().unwrap();
    Ok(())
}

fn process_events<TState>(
    events_it: &mut Events<AsyncReader>,
    stdout: &mut RawTerminal<Stdout>,
    app: &mut Element<TState>,
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

fn track_mouse_down<TState>(el: &mut Element<TState>, left: u16, top: u16) {
    let node = match el {
        Element::Node(node) => node,
        _ => return,
    };

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
                track_mouse_down(&mut children[i], left, top)
            }
        }
    }
}

fn track_mouse_pressed<TState>(el: &mut Element<TState>, left: u16, top: u16) {
    let node = match el {
        Element::Node(node) => node,
        _ => return,
    };

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
                track_mouse_pressed(&mut children[i], left, top)
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

fn render_element<'a, TState>(
    el: Element<'a, TState>,
    state: &'a RefCell<TState>,
) -> Element<'a, TState> {
    match el {
        Element::Node(node) => render_node(node, state),
        Element::Component(component) => render_component(component, state),
        Element::None => Element::None,
    }
}

fn render_node<'a, TState>(n: Node<'a, TState>, state: &'a RefCell<TState>) -> Element<'a, TState> {
    let rendered_node: Node<'a, TState> = Node::new(n.left, n.top)
        .set_text(n.text)
        .set_width(n.width)
        .set_height(n.height)
        .set_border(n.border)
        .set_on_mouse_click(n.on_mouse_click)
        .disable(n.disabled)
        .set_children(match n.children {
            None => None,
            Some(children) => {
                let mut v: Vec<Element<'a, TState>> = Vec::new();
                for c_el in children {
                    v.push(render_element(c_el, state))
                }
                Some(v)
            }
        });

    Element::Node(rendered_node)
}

fn render_component<'a, TState>(
    component: Box<dyn Component<'a, TState>>,
    state: &'a RefCell<TState>,
) -> Element<'a, TState> {
    render_element(component.render(state), state)
}

fn draw_node<'a, TState>(
    stdout: &mut RawTerminal<Stdout>,
    el: &mut Element<'a, TState>,
    state: &'a RefCell<TState>,
) {
    let b = match el {
        Element::Node(node) => node,
        Element::None => return,
        Element::Component(_) => {
            // Should never happen, the graph is rendered before being drawn
            panic!("draw_node called on un-rendered Component Element; this is a programming error")
        }
    };

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

    match &mut b.children {
        Some(children) => {
            for c in children {
                draw_node(stdout, c, state)
            }
        }
        None => (),
    }
}
