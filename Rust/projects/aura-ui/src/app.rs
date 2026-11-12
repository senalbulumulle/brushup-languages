//      _                               _   _   ___ 
//     / \     _   _   _ __    __ _    | | | | |_ _|
//    / _ \   | | | | | '__|  / _` |   | | | |  | | 
//   / ___ \  | |_| | | |    | (_| |   | |_| |  | | 
//  /_/   \_\  \__,_| |_|     \__,_|    \___/  |___|


//! Application bootstrap helpers.

use gpui::{
    App, Application, Bounds, Entity, Pixels, Render, Size, WindowBounds, WindowOptions, px, size,
};

use crate::theme::{Theme, ThemeMode};

/// Options for opening an Aura window.
#[derive(Clone)]
pub struct WindowConfig {
    pub title: Option<String>,
    pub width: Pixels,
    pub height: Pixels,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: Some("Aura".into()),
            width: px(800.),
            height: px(600.),
        }
    }
}

impl WindowConfig {
    pub fn new(title: impl Into<String>, width: Pixels, height: Pixels) -> Self {
        Self {
            title: Some(title.into()),
            width,
            height,
        }
    }

    pub fn sized(width: Pixels, height: Pixels) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }
}

/// Entry point for Aura applications.
pub struct Aura;

impl Aura {
    /// Boot GPUI, install the default light theme, then run `f`.
    pub fn run(f: impl FnOnce(&mut App) + 'static) {
        Self::run_with_theme(ThemeMode::Light, f);
    }

    /// Boot GPUI with an explicit theme mode.
    pub fn run_with_theme(mode: ThemeMode, f: impl FnOnce(&mut App) + 'static) {
        Application::new().run(move |cx: &mut App| {
            Theme::new(mode).install(cx);
            f(cx);
            cx.activate(true);
        });
    }

    /// Open a centered window hosting `root`.
    pub fn window<V: Render>(
        title: impl Into<String>,
        window_size: Size<Pixels>,
        cx: &mut App,
        root: impl FnOnce(&mut gpui::Window, &mut App) -> Entity<V> + 'static,
    ) -> gpui::WindowHandle<V> {
        let config = WindowConfig::new(title, window_size.width, window_size.height);
        Self::window_with_config(config, cx, root)
    }

    /// Open a window using [`WindowConfig`].
    pub fn window_with_config<V: Render>(
        config: WindowConfig,
        cx: &mut App,
        root: impl FnOnce(&mut gpui::Window, &mut App) -> Entity<V> + 'static,
    ) -> gpui::WindowHandle<V> {
        let bounds = Bounds::centered(None, size(config.width, config.height), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(gpui::TitlebarOptions {
                title: config.title.map(Into::into),
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(options, root)
            .expect("Aura failed to open window")
    }
}
