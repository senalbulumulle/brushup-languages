//! Layout primitives built on GPUI `div`.

use gpui::{Div, Styled, div, px};

/// Vertical flex stack with a gap.
pub fn vstack(gap: f32) -> Div {
    div().flex().flex_col().gap(px(gap))
}

/// Horizontal flex stack with a gap.
pub fn hstack(gap: f32) -> Div {
    div().flex().flex_row().items_center().gap(px(gap))
}

/// Full-size flex center.
pub fn center() -> Div {
    div()
        .flex()
        .flex_col()
        .size_full()
        .items_center()
        .justify_center()
}

/// Flexible spacer that expands along the main axis.
pub fn spacer() -> Div {
    div().flex_1()
}
