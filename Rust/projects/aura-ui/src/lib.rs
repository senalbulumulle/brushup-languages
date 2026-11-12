//! # Aura UI
//!
//! A high-level desktop UI framework built entirely on [GPUI](https://gpui.rs).
//! Aura layers theming, layout primitives, and styled widgets on top of GPUI's
//! GPU-accelerated element tree — you still get full access to GPUI when you
//! need it.
//!
//! ```ignore
//! use aura_ui::prelude::*;
//!
//! fn main() {
//!     Aura::run(|cx| {
//!         Aura::window("Aura", size(px(720.), px(520.)), cx, |_, cx| {
//!             cx.new(|_| MyView)
//!         });
//!     });
//! }
//! ```

pub mod app;
pub mod layout;
pub mod prelude;
pub mod speak;
pub mod theme;
pub mod widgets;

pub use app::{Aura, WindowConfig};
pub use layout::{center, hstack, spacer, vstack};
pub use theme::{Theme, ThemeMode, ThemeTokens};
pub use widgets::{
    Badge, BadgeTone, Button, ButtonVariant, Card, Divider, Heading, Label, Surface,
};

pub use gpui::AppContext;

/// Re-export GPUI so apps can stay on one import path.
pub use gpui;
