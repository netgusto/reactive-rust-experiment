use std::collections::HashMap;
use std::io::{stdout, Stdout, Write};
use std::{thread::sleep, time::Duration};

use termion::color;
use termion::cursor;
use termion::cursor::Goto;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{Events, MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, AsyncReader};

use std::cell::RefCell;
use std::rc::Rc;

type StateStore<TState> = HashMap<i32, Rc<RefCell<TState>>>;

pub trait StatefulComponent<'a, TState> {
    fn render(&self, state: Rc<RefCell<TState>>) -> Element<'a, TState>;
}

pub enum Element<'a, TState> {
    Container(Container<'a, TState>),
    Node(Node<'a, TState>),
    StatefulComponent(Box<dyn StatefulComponent<'a, TState>>),
    None,
}

pub type MouseClickHandler<'a> = Box<dyn FnMut() + 'a>;
pub type Children<'a, TState> = Vec<Element<'a, TState>>;

pub type Container<'a, TState> = Children<'a, TState>;

pub struct Node<'a, TState> {
    text: Option<String>,
    children: Option<Children<'a, TState>>,
    top: u16,
    left: u16,
    width: u16,
    height: u16,
    border: bool,
    on_click: Option<MouseClickHandler<'a>>,
    disabled: bool,
}

impl<'a, TState> Node<'a, TState> {
    pub fn new(left: u16, top: u16) -> Node<'a, TState> {
        Node {
            left,
            top,
            width: 1,
            height: 1,
            text: None,
            on_click: None,
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

    pub fn set_on_click(mut self, handler: Option<MouseClickHandler<'a>>) -> Self {
        self.on_click = handler;
        self
    }
}

pub fn run<'a, TState>(
    app_maker: &dyn Fn() -> Element<'a, TState>,
    state_store: &'a StateStore<TState>,
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
            Some(some) => {
                if let true = process_events(&mut events_it, some) {
                    break;
                }
            }
        }

        let mut rendered = render_element(app_maker(), state_store);
        draw_node(&mut stdout, &mut rendered, state_store);
        current_app = Some(rendered);
        stdout.flush().unwrap();

        sleep(Duration::from_millis(16));
    }

    write!(stdout, "{}{}", termion::clear::All, cursor::Show).unwrap();
    stdout.flush().unwrap();
    Ok(())
}

fn process_events<TState>(events_it: &mut Events<AsyncReader>, app: &mut Element<TState>) -> bool {
    loop {
        let event = events_it.next();
        match event {
            None => return false,
            Some(Ok(Event::Key(Key::Char(c)))) => {
                if let 'q' = c {
                    return true;
                }
            }
            Some(Ok(Event::Mouse(me))) => {
                if let MouseEvent::Release(left, top) = me {
                    track_mouse_clicked(app, left, top);
                }
            }
            _ => (),
        }
    }
}

fn track_mouse_clicked<TState>(el: &mut Element<TState>, left: u16, top: u16) {
    let node = match el {
        Element::Node(node) => node,
        Element::Container(container) => {
            return for c in container {
                track_mouse_clicked(c, left, top)
            }
        }
        _ => return,
    };

    if node.disabled {
        return;
    }
    if aabb_contains(node.left, node.top, node.width, node.height, left, top) {
        if let Some(c) = &mut node.on_click {
            return c();
        }
    }

    match &mut node.children {
        None => {}
        Some(children) => {
            for c in children {
                track_mouse_clicked(c, left, top)
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
    state: &'a StateStore<TState>,
) -> Element<'a, TState> {
    match el {
        Element::Container(container) => render_container(container, state),
        Element::Node(node) => render_node(node, state),
        Element::StatefulComponent(component) => render_component(component, state),
        Element::None => Element::None,
    }
}

fn render_container<'a, TState>(
    container: Container<'a, TState>,
    state: &'a StateStore<TState>,
) -> Element<'a, TState> {
    let mut v: Container<'a, TState> = Vec::new();
    for c_el in container {
        v.push(render_element(c_el, state))
    }
    Element::Container(v)
}

fn render_node<'a, TState>(
    n: Node<'a, TState>,
    state: &'a StateStore<TState>,
) -> Element<'a, TState> {
    let rendered_node = Node::new(n.left, n.top)
        .set_text(n.text)
        .set_width(n.width)
        .set_height(n.height)
        .set_border(n.border)
        .set_on_click(n.on_click)
        .disable(n.disabled)
        .set_children(match n.children {
            None => None,
            Some(children) => {
                let mut v = Vec::new();
                for c_el in children {
                    v.push(render_element(c_el, state))
                }
                Some(v)
            }
        });

    Element::Node(rendered_node)
}

fn render_component<'a, TState>(
    component: Box<dyn StatefulComponent<'a, TState>>,
    state: &'a StateStore<TState>,
) -> Element<'a, TState> {
    let b = state.get(&42);
    let s = match b {
        Some(v) => v,
        _ => panic!("hannnn"),
    };

    render_element(component.render(s.clone()), state)
}

fn draw_node<'a, TState>(
    stdout: &mut RawTerminal<Stdout>,
    el: &mut Element<'a, TState>,
    state: &'a StateStore<TState>,
) {
    let b = match el {
        Element::Node(node) => node,
        Element::Container(container) => {
            return for c in container {
                draw_node(stdout, c, state)
            }
        }
        Element::None => return,
        Element::StatefulComponent(_) => {
            // Should never happen, the graph is rendered before being drawn
            panic!("draw_node called on un-rendered StatefulComponent Element; this is a programming error")
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
