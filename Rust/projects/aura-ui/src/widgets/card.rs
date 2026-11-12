//! Raised content card.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px,
};

use crate::theme::Theme;

/// Contained surface for grouping related content.
#[derive(IntoElement)]
pub struct Card {
    children: Vec<AnyElement>,
    padded: bool,
}

impl Card {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            padded: true,
        }
    }

    pub fn bare(mut self) -> Self {
        self.padded = false;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::tokens(cx);
        let mut el = div()
            .flex()
            .flex_col()
            .gap(px(t.space_3))
            .bg(t.surface_raised)
            .border_1()
            .border_color(t.border)
            .rounded(px(t.radius_lg))
            .shadow_sm();

        if self.padded {
            el = el.p(px(t.space_6));
        }

        el.children(self.children)
    }
}
