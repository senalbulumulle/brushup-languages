//      _                               _   _   ___ 
//     / \     _   _   _ __    __ _    | | | | |_ _|
//    / _ \   | | | | | '__|  / _` |   | | | |  | | 
//   / ___ \  | |_| | | |    | (_| |   | |_| |  | | 
//  /_/   \_\  \__,_| |_|     \__,_|    \___/  |___|

//! Convenient re-exports for Aura apps (including Aura Speak).

pub use crate::app::{Aura, WindowConfig};
pub use crate::layout::{center, hstack, spacer, vstack};
pub use crate::speak::{Action, Node, SpeakApp, SpeakState};
pub use crate::theme::{Theme, ThemeMode, ThemeTokens};
pub use crate::widgets::{
    Badge, BadgeTone, Button, ButtonVariant, Card, Divider, Heading, Label, Surface,
};
pub use gpui::{
    App, AppContext, Application, Bounds, Context, Entity, InteractiveElement, IntoElement,
    ParentElement, Render, SharedString, StatefulInteractiveElement, Styled, Window, WindowBounds,
    WindowOptions, div, px, rgb, size,
};

/// Aura Speak entry macro (`aura! { ... }`).
pub use crate::aura;
