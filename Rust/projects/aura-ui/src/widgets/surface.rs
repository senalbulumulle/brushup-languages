//! Full-bleed themed page background.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
};

use crate::theme::Theme;

/// Root page surface filled with the theme background.
#[derive(IntoElement)]
pub struct Surface {
    children: Vec<AnyElement>,
}

impl Surface {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
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

impl Default for Surface {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Surface {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::tokens(cx);
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(t.background)
            .text_color(t.text)
            .children(self.children)
    }
}
