//! # Aura Speak
//!
//! A tiny UI language for kids — English words, almost no Rust noise.
//!
//! ```ignore
//! use aura_ui::prelude::*;
//!
//! fn main() {
//!     aura! {
//!         open "My App" size 640 by 420
//!
//!         remember clicks = 0;
//!
//!         paint {
//!             card {
//!                 title "Hello!"
//!                 whisper "Tap a button"
//!                 row {
//!                     press "Add" => bump clicks
//!                     press "Reset" => set clicks 0
//!                 }
//!                 say "Clicks" with clicks
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Words you can use
//!
//! | Word | Meaning |
//! |------|---------|
//! | `open "Name" size W by H` | Start a window |
//! | `remember name = 0;` | Keep a number in memory |
//! | `paint { ... }` | Draw the screen |
//! | `card { ... }` | A cozy box |
//! | `stack { ... }` | Things one under another |
//! | `row { ... }` | Things side by side |
//! | `title "..."` | Big words |
//! | `say "..."` | Normal words |
//! | `whisper "..."` | Soft gray words |
//! | `say "Label" with name` | Show a remembered number |
//! | `press "Label" => …` | A button |
//! | `bump name` | Add 1 |
//! | `set name N` | Set to a number |
//! | `flip theme` | Light ↔ dark |
//! | `line` | A thin divider |
//! | `tag "..."` | A small chip |

mod runtime;

pub use runtime::{Action, Node, SpeakApp, SpeakState};

/// The Aura Speak language macro. See the module docs for the word list.
#[macro_export]
macro_rules! aura {
    (
        open $title:literal size $width:literal by $height:literal
        $(remember $mem_name:ident = $mem_val:literal ;)*
        paint { $($paint:tt)* }
    ) => {{
        let mut __aura_app = $crate::speak::SpeakApp::new($title, $width as f32, $height as f32);
        $(
            __aura_app.remember(stringify!($mem_name), $mem_val as i64);
        )*
        __aura_app.set_root($crate::aura_nodes! { $($paint)* });
        __aura_app.go();
    }};
}

/// Internal: turn Aura Speak body tokens into a [`Vec<Node>`].
#[macro_export]
#[doc(hidden)]
macro_rules! aura_nodes {
    () => { ::std::vec::Vec::<$crate::speak::Node>::new() };

    (title $text:literal $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::title($text) ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (say $text:literal with $name:ident $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::say_with($text, stringify!($name)) ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (say $text:literal $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::say($text) ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (whisper $text:literal $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::whisper($text) ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (tag $text:literal $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::tag($text) ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (line $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::line() ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (card { $($inner:tt)* } $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::card($crate::aura_nodes! { $($inner)* }) ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (stack { $($inner:tt)* } $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::stack($crate::aura_nodes! { $($inner)* }) ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (row { $($inner:tt)* } $($rest:tt)*) => {{
        let mut __v = ::std::vec![ $crate::speak::Node::row($crate::aura_nodes! { $($inner)* }) ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (press $label:literal => bump $name:ident $($rest:tt)*) => {{
        let mut __v = ::std::vec![
            $crate::speak::Node::press($label, $crate::speak::Action::Bump(stringify!($name).into()))
        ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (press $label:literal => set $name:ident $value:literal $($rest:tt)*) => {{
        let mut __v = ::std::vec![
            $crate::speak::Node::press(
                $label,
                $crate::speak::Action::Set(stringify!($name).into(), $value as i64),
            )
        ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};

    (press $label:literal => flip theme $($rest:tt)*) => {{
        let mut __v = ::std::vec![
            $crate::speak::Node::press($label, $crate::speak::Action::FlipTheme)
        ];
        __v.extend($crate::aura_nodes! { $($rest)* });
        __v
    }};
}
