//! Aura UI showcase — a live playground for the framework primitives.

use aura_ui::prelude::*;
use aura_ui::{BadgeTone, ButtonVariant, ThemeMode};

struct Showcase {
    clicks: u32,
}

impl Showcase {
    fn increment(&mut self, _event: &gpui::ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.clicks += 1;
        cx.notify();
    }

    fn toggle_theme(
        &mut self,
        _event: &gpui::ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        Theme::toggle(cx);
        cx.notify();
    }

    fn reset(&mut self, _event: &gpui::ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.clicks = 0;
        cx.notify();
    }
}

impl Render for Showcase {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mode = Theme::tokens(cx).mode;
        let mode_label = match mode {
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
        };

        Surface::new().child(
            center().child(
                Card::new()
                    .child(
                        hstack(12.)
                            .child(Heading::new("Aura UI").level(1))
                            .child(spacer())
                            .child(Badge::new(mode_label).tone(BadgeTone::Accent)),
                    )
                    .child(Label::new(
                        "A high-level desktop UI framework built entirely on GPUI.",
                    ).muted())
                    .child(Divider::new())
                    .child(
                        vstack(8.)
                            .child(Heading::new("Counter").level(3))
                            .child(Label::new(format!("Clicks: {}", self.clicks)))
                            .child(
                                hstack(8.)
                                    .child(
                                        Button::new("inc", "Increment").on_click(cx.listener(
                                            Self::increment,
                                        )),
                                    )
                                    .child(
                                        Button::new("reset", "Reset")
                                            .variant(ButtonVariant::Secondary)
                                            .on_click(cx.listener(Self::reset)),
                                    )
                                    .child(
                                        Button::new("theme", "Toggle theme")
                                            .ghost()
                                            .on_click(cx.listener(Self::toggle_theme)),
                                    ),
                            ),
                    )
                    .child(Divider::new())
                    .child(
                        vstack(8.)
                            .child(Heading::new("Badges").level(3))
                            .child(
                                hstack(8.)
                                    .child(Badge::new("neutral"))
                                    .child(Badge::new("accent").tone(BadgeTone::Accent))
                                    .child(Badge::new("success").tone(BadgeTone::Success))
                                    .child(Badge::new("warning").tone(BadgeTone::Warning))
                                    .child(Badge::new("danger").tone(BadgeTone::Danger)),
                            ),
                    )
                    .child(
                        hstack(8.)
                            .child(
                                Button::new("danger-demo", "Danger")
                                    .danger()
                                    .on_click(|_, _, _| {}),
                            )
                            .child(
                                Button::new("secondary-demo", "Secondary")
                                    .secondary()
                                    .on_click(|_, _, _| {}),
                            ),
                    ),
            ),
        )
    }
}

fn main() {
    Aura::run(|cx| {
        Aura::window("Aura UI — Showcase", size(px(720.), px(560.)), cx, |_, cx| {
            cx.new(|_| Showcase { clicks: 0 })
        });
    });
}
