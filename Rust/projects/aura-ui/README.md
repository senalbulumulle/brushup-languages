# Aura UI

A high-level desktop UI framework built entirely on [GPUI](https://gpui.rs) —
Zed's GPU-accelerated UI engine. Aura adds theming, layout primitives, styled
widgets, and **Aura Speak**: a tiny language inside Rust that a 10-year-old
can read and write.

## Aura Speak (kid language)

```rust
use aura_ui::prelude::*;

fn main() {
    aura! {
        open "My First Aura App" size 640 by 460

        remember clicks = 0;

        paint {
            card {
                title "Hello, friend!"
                whisper "Tap a button"
                say "Clicks" with clicks
                row {
                    press "Add" => bump clicks
                    press "Reset" => set clicks 0
                    press "Night" => flip theme
                }
            }
        }
    }
}
```

### Word list

| Word | Meaning |
|------|---------|
| `open "Name" size W by H` | Open a window |
| `remember name = 0;` | Keep a number |
| `paint { ... }` | Draw the screen |
| `card` / `stack` / `row` | Box, vertical list, horizontal list |
| `title` / `say` / `whisper` | Big / normal / soft text |
| `say "Label" with name` | Show a remembered number |
| `press "Label" => …` | Button |
| `bump name` | Add 1 |
| `set name N` | Set to N |
| `flip theme` | Light ↔ dark |
| `line` / `tag` | Divider / chip |

```shell
cargo run --example thing
```

## Rust API (power users)

```rust
use aura_ui::prelude::*;

struct Hello;

impl Render for Hello {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Surface::new().child(
            center().child(
                Card::new()
                    .child(Heading::new("Hello, Aura").level(1))
                    .child(Label::new("Built on GPUI.").muted())
                    .child(Button::new("ok", "Nice").on_click(|_, _, _| {})),
            ),
        )
    }
}

fn main() {
    Aura::run(|cx| {
        Aura::window("Aura", size(px(640.), px(420.)), cx, |_, cx| {
            cx.new(|_| Hello)
        });
    });
}
```

```shell
cargo run --example showcase
```

## Layout

```
src/
  speak/          # Aura Speak language + runtime
  app.rs          # Aura::run / Aura::window
  theme.rs        # Theme tokens
  layout.rs       # vstack / hstack / center / spacer
  widgets/        # Button, Card, Badge, …
examples/
  thing.rs        # Aura Speak demo
  showcase.rs     # widget playground
```

## Dependencies

- [gpui](https://crates.io/crates/gpui) `0.2.2` (Metal on macOS)

## Notes

Aura is a learning / brush-up framework. APIs will move as GPUI evolves.
