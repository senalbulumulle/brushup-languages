//      _                               _   _   ___ 
//     / \     _   _   _ __    __ _    | | | | |_ _|
//    / _ \   | | | | | '__|  / _` |   | | | |  | | 
//   / ___ \  | |_| | | |    | (_| |   | |_| |  | | 
//  /_/   \_\  \__,_| |_|     \__,_|    \___/  |___|


//! Compact status / tag badge.

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::theme::Theme;

/// Semantic color for a badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeTone {
    #[default]
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

/// Small label chip.
#[derive(IntoElement)]
pub struct Badge {
    text: SharedString,
    tone: BadgeTone,
}

impl Badge {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            tone: BadgeTone::Neutral,
        }
    }

    pub fn tone(mut self, tone: BadgeTone) -> Self {
        self.tone = tone;
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = Theme::tokens(cx);
        let (bg, fg) = match self.tone {
            BadgeTone::Neutral => (t.surface, t.text_muted),
            BadgeTone::Accent => (t.accent_muted, t.accent),
            BadgeTone::Success => (t.accent_muted, t.success),
            BadgeTone::Warning => (t.accent_muted, t.warning),
            BadgeTone::Danger => (t.accent_muted, t.danger),
        };

        div()
            .flex()
            .items_center()
            .px(px(t.space_2))
            .py(px(t.space_1))
            .rounded(px(t.radius_sm))
            .bg(bg)
            .text_color(fg)
            .text_size(px(12.))
            .child(self.text)
    }
}
