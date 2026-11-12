//      _                               _   _   ___ 
//     / \     _   _   _ __    __ _    | | | | |_ _|
//    / _ \   | | | | | '__|  / _` |   | | | |  | | 
//   / ___ \  | |_| | | |    | (_| |   | |_| |  | | 
//  /_/   \_\  \__,_| |_|     \__,_|    \___/  |___|


//! Themed button widget.

use std::rc::Rc;

use gpui::{
    App, ClickEvent, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::theme::Theme;

/// Visual treatment for a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

/// Fluent builder for a clickable Aura button.
#[derive(IntoElement)]
pub struct Button {
    id: SharedString,
    label: SharedString,
    variant: ButtonVariant,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Button {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Primary,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn primary(self) -> Self {
        self.variant(ButtonVariant::Primary)
    }

    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }

    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }

    pub fn danger(self) -> Self {
        self.variant(ButtonVariant::Danger)
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::tokens(cx);
        let (bg, fg, border, hover_bg) = match self.variant {
            ButtonVariant::Primary => (t.accent, t.text_inverse, t.accent, t.accent_hover),
            ButtonVariant::Secondary => {
                (t.surface_raised, t.text, t.border, t.accent_muted)
            }
            ButtonVariant::Ghost => (
                gpui::transparent_black(),
                t.text,
                gpui::transparent_black(),
                t.accent_muted,
            ),
            ButtonVariant::Danger => (t.danger, t.text_inverse, t.danger, t.danger),
        };

        let mut el = div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .px(px(t.space_4))
            .py(px(t.space_2))
            .rounded(px(t.radius_md))
            .bg(bg)
            .text_color(fg)
            .border_1()
            .border_color(border)
            .cursor_pointer()
            .hover(move |style| style.bg(hover_bg))
            .active(|style| style.opacity(0.88))
            .child(self.label);

        if let Some(handler) = self.on_click {
            el = el.on_click(move |event, window, cx| handler(event, window, cx));
        }

        el
    }
}
