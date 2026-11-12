//! Runtime that turns Aura Speak into Aura widgets + GPUI.

use std::collections::HashMap;

use gpui::{
    AppContext, Context, IntoElement, ParentElement, Render, SharedString, Window, px, size,
};

use crate::app::Aura;
use crate::layout::{center, hstack, vstack};
use crate::theme::Theme;
use crate::widgets::{Badge, Button, Card, Divider, Heading, Label, Surface};

/// A remembered number box (`remember score = 0`).
#[derive(Debug, Clone, Default)]
pub struct SpeakState {
    numbers: HashMap<SharedString, i64>,
}

impl SpeakState {
    pub fn remember(&mut self, name: impl Into<SharedString>, value: i64) {
        self.numbers.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> i64 {
        self.numbers.get(name).copied().unwrap_or(0)
    }

    pub fn bump(&mut self, name: &str) {
        let n = self.get(name);
        self.remember(SharedString::from(name.to_string()), n + 1);
    }

    pub fn set(&mut self, name: &str, value: i64) {
        self.remember(SharedString::from(name.to_string()), value);
    }
}

/// What a `press` button does.
#[derive(Debug, Clone)]
pub enum Action {
    Bump(SharedString),
    Set(SharedString, i64),
    FlipTheme,
}

/// One piece of the screen tree.
#[derive(Debug, Clone)]
pub enum Node {
    Title(SharedString),
    Say(SharedString),
    Whisper(SharedString),
    SayWith {
        label: SharedString,
        name: SharedString,
    },
    Tag(SharedString),
    Line,
    Card(Vec<Node>),
    Stack(Vec<Node>),
    Row(Vec<Node>),
    Press {
        id: SharedString,
        label: SharedString,
        action: Action,
    },
}

impl Node {
    pub fn title(text: impl Into<SharedString>) -> Self {
        Self::Title(text.into())
    }

    pub fn say(text: impl Into<SharedString>) -> Self {
        Self::Say(text.into())
    }

    pub fn whisper(text: impl Into<SharedString>) -> Self {
        Self::Whisper(text.into())
    }

    pub fn say_with(label: impl Into<SharedString>, name: impl Into<SharedString>) -> Self {
        Self::SayWith {
            label: label.into(),
            name: name.into(),
        }
    }

    pub fn tag(text: impl Into<SharedString>) -> Self {
        Self::Tag(text.into())
    }

    pub fn line() -> Self {
        Self::Line
    }

    pub fn card(children: Vec<Node>) -> Self {
        Self::Card(children)
    }

    pub fn stack(children: Vec<Node>) -> Self {
        Self::Stack(children)
    }

    pub fn row(children: Vec<Node>) -> Self {
        Self::Row(children)
    }

    pub fn press(label: impl Into<SharedString>, action: Action) -> Self {
        let label = label.into();
        Self::Press {
            id: label.clone(),
            label,
            action,
        }
    }
}

/// A complete Aura Speak program ready to run.
pub struct SpeakApp {
    title: SharedString,
    width: f32,
    height: f32,
    state: SpeakState,
    root: Vec<Node>,
}

impl SpeakApp {
    pub fn new(title: impl Into<SharedString>, width: f32, height: f32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
            state: SpeakState::default(),
            root: Vec::new(),
        }
    }

    pub fn remember(&mut self, name: impl Into<SharedString>, value: i64) {
        self.state.remember(name, value);
    }

    pub fn set_root(&mut self, root: Vec<Node>) {
        self.root = root;
    }

    /// Boot GPUI and show the program.
    pub fn go(self) {
        let title = self.title.clone();
        let width = self.width;
        let height = self.height;
        let view = SpeakView {
            state: self.state,
            root: self.root,
            button_seq: 0,
        };

        Aura::run(move |cx| {
            Aura::window(title.to_string(), size(px(width), px(height)), cx, |_, cx| {
                cx.new(|_| view)
            });
        });
    }
}

struct SpeakView {
    state: SpeakState,
    root: Vec<Node>,
    button_seq: u64,
}

impl SpeakView {
    fn run_action(&mut self, action: &Action, cx: &mut Context<Self>) {
        match action {
            Action::Bump(name) => self.state.bump(name),
            Action::Set(name, value) => self.state.set(name, *value),
            Action::FlipTheme => Theme::toggle(cx),
        }
        cx.notify();
    }

    fn render_nodes(
        &mut self,
        nodes: &[Node],
        cx: &mut Context<Self>,
    ) -> Vec<gpui::AnyElement> {
        nodes
            .iter()
            .map(|node| self.render_node(node, cx))
            .collect()
    }

    fn render_node(&mut self, node: &Node, cx: &mut Context<Self>) -> gpui::AnyElement {
        match node {
            Node::Title(text) => Heading::new(text.clone()).level(1).into_any_element(),
            Node::Say(text) => Label::new(text.clone()).into_any_element(),
            Node::Whisper(text) => Label::new(text.clone()).muted().into_any_element(),
            Node::SayWith { label, name } => {
                let value = self.state.get(name);
                Label::new(format!("{label}: {value}")).into_any_element()
            }
            Node::Tag(text) => Badge::new(text.clone()).into_any_element(),
            Node::Line => Divider::new().into_any_element(),
            Node::Card(children) => {
                let kids = self.render_nodes(children, cx);
                Card::new().children(kids).into_any_element()
            }
            Node::Stack(children) => {
                let kids = self.render_nodes(children, cx);
                vstack(12.).children(kids).into_any_element()
            }
            Node::Row(children) => {
                let kids = self.render_nodes(children, cx);
                hstack(8.).children(kids).into_any_element()
            }
            Node::Press { id, label, action } => {
                self.button_seq += 1;
                let seq = self.button_seq;
                let button_id: SharedString = format!("{id}-{seq}").into();
                let action = action.clone();
                Button::new(button_id, label.clone())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.run_action(&action, cx);
                    }))
                    .into_any_element()
            }
        }
    }
}

impl Render for SpeakView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Reset ids each frame so press buttons stay stable-ish per tree walk.
        self.button_seq = 0;
        let root = self.root.clone();
        let kids = self.render_nodes(&root, cx);

        Surface::new().child(center().child(vstack(16.).children(kids)))
    }
}
