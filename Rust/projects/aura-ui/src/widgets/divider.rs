//! Horizontal rule.

use gpui::{App, IntoElement, RenderOnce, Styled, Window, div, px};

use crate::theme::Theme;

/// Thin horizontal separator.
#[derive(IntoElement, Default)]
pub struct Divider;

impl Divider {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for Divider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::tokens(cx);
        div().w_full().h(px(1.)).bg(t.border)
    }
}
