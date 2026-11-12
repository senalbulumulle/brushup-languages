//! Design tokens and theme globals for Aura.

use gpui::{App, Global, Hsla, rgb};

/// Light or dark presentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

/// Concrete color / spacing tokens used by Aura widgets.
#[derive(Debug, Clone)]
pub struct ThemeTokens {
    pub mode: ThemeMode,

    pub background: Hsla,
    pub surface: Hsla,
    pub surface_raised: Hsla,
    pub border: Hsla,
    pub border_strong: Hsla,

    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_inverse: Hsla,

    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub accent_muted: Hsla,

    pub success: Hsla,
    pub warning: Hsla,
    pub danger: Hsla,

    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,

    pub space_1: f32,
    pub space_2: f32,
    pub space_3: f32,
    pub space_4: f32,
    pub space_6: f32,
    pub space_8: f32,
}

impl ThemeTokens {
    /// Cool stone surfaces, deep ink type, teal accent — light default.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            background: rgb(0xE9EEF2).into(),
            surface: rgb(0xF7FAFC).into(),
            surface_raised: rgb(0xFFFFFF).into(),
            border: rgb(0xC9D2DA).into(),
            border_strong: rgb(0x8A97A5).into(),
            text: rgb(0x121820).into(),
            text_muted: rgb(0x5A6674).into(),
            text_inverse: rgb(0xF7FAFC).into(),
            accent: rgb(0x0B7A78).into(),
            accent_hover: rgb(0x096462).into(),
            accent_muted: rgb(0xD5EDEC).into(),
            success: rgb(0x1F7A4D).into(),
            warning: rgb(0xB56A12).into(),
            danger: rgb(0xB42318).into(),
            radius_sm: 4.0,
            radius_md: 8.0,
            radius_lg: 14.0,
            space_1: 4.0,
            space_2: 8.0,
            space_3: 12.0,
            space_4: 16.0,
            space_6: 24.0,
            space_8: 32.0,
        }
    }

    /// Deep ink canvas with soft cyan accent.
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            background: rgb(0x0C1218).into(),
            surface: rgb(0x151C24).into(),
            surface_raised: rgb(0x1E2732).into(),
            border: rgb(0x2C3846).into(),
            border_strong: rgb(0x4A5A6C).into(),
            text: rgb(0xE8EEF4).into(),
            text_muted: rgb(0x93A0AE).into(),
            text_inverse: rgb(0x0C1218).into(),
            accent: rgb(0x2EC4B6).into(),
            accent_hover: rgb(0x4AD4C6).into(),
            accent_muted: rgb(0x163A38).into(),
            success: rgb(0x3DCF8E).into(),
            warning: rgb(0xE0A045).into(),
            danger: rgb(0xF07167).into(),
            radius_sm: 4.0,
            radius_md: 8.0,
            radius_lg: 14.0,
            space_1: 4.0,
            space_2: 8.0,
            space_3: 12.0,
            space_4: 16.0,
            space_6: 24.0,
            space_8: 32.0,
        }
    }

    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }
}

/// App-wide theme. Stored as a GPUI [`Global`].
#[derive(Debug, Clone)]
pub struct Theme {
    pub tokens: ThemeTokens,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            tokens: ThemeTokens::light(),
        }
    }
}

impl Global for Theme {}

impl Theme {
    pub fn new(mode: ThemeMode) -> Self {
        Self {
            tokens: ThemeTokens::for_mode(mode),
        }
    }

    pub fn light() -> Self {
        Self::new(ThemeMode::Light)
    }

    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark)
    }

    /// Install (or replace) the global theme.
    pub fn install(self, cx: &mut App) {
        cx.set_global(self);
    }

    /// Borrow the current global theme.
    ///
    /// Panics if [`Theme::install`] / [`crate::Aura::run`] has not run yet.
    pub fn current(cx: &App) -> &Theme {
        cx.try_global::<Theme>()
            .expect("Aura Theme is not installed. Call Aura::run or Theme::install first.")
    }

    pub fn tokens(cx: &App) -> &ThemeTokens {
        &Self::current(cx).tokens
    }

    pub fn set_mode(cx: &mut App, mode: ThemeMode) {
        if !cx.has_global::<Theme>() {
            Theme::new(mode).install(cx);
            return;
        }
        cx.global_mut::<Theme>().tokens = ThemeTokens::for_mode(mode);
    }

    pub fn toggle(cx: &mut App) {
        let next = match Self::tokens(cx).mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        Self::set_mode(cx, next);
    }
}
