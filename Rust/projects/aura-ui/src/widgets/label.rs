//! Text primitives.

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::theme::Theme;

/// Body / muted text.
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    muted: bool,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            muted: false,
        }
    }

    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::tokens(cx);
        let color = if self.muted { t.text_muted } else { t.text };
        div().text_color(color).child(self.text)
    }
}

/// Display heading.
#[derive(IntoElement)]
pub struct Heading {
    text: SharedString,
    level: u8,
}

impl Heading {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            level: 1,
        }
    }

    pub fn level(mut self, level: u8) -> Self {
        self.level = level.clamp(1, 4);
        self
    }
}

impl RenderOnce for Heading {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::tokens(cx);
        let size = match self.level {
            1 => 28.0,
            2 => 22.0,
            3 => 18.0,
            _ => 15.0,
        };

        div()
            .text_color(t.text)
            .text_size(px(size))
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .child(self.text)
    }
}
