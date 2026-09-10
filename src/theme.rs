use gpui::{App, Global, Hsla, Rems, Window, WindowAppearance, hsla, rems, rgb, transparent_black};

pub use waku_client::theme::ThemePreference;

/// Scaled pixels: a dimension authored at the default 14px UI font size,
/// expressed in rems so the UI font size setting scales it. The window's rem
/// size *is* the UI font size, so at the default setting this resolves to
/// exactly the authored pixel value.
///
/// Chrome text sizes and their line heights go through here. Content surfaces
/// that already derive from a font-size setting — markdown metrics, the file
/// editor, diff rows, tool-output mono — stay in `px` so they never scale
/// twice.
pub fn sp(value: f32) -> Rems {
    rems(value / waku_client::persistence::DEFAULT_UI_FONT_SIZE)
}

fn resolves_to_dark(preference: ThemePreference, system_appearance: WindowAppearance) -> bool {
    match preference {
        ThemePreference::System => matches!(
            system_appearance,
            WindowAppearance::Dark | WindowAppearance::VibrantDark
        ),
        ThemePreference::Light => false,
        ThemePreference::Dark => true,
    }
}

fn native_override(preference: ThemePreference) -> Option<bool> {
    match preference {
        ThemePreference::System => None,
        ThemePreference::Light => Some(false),
        ThemePreference::Dark => Some(true),
    }
}

/// Waku's visual language uses Codex-like neutral surfaces: near-monochrome
/// graphite in dark mode, warm white in light mode, and a restrained green for
/// the few actions that need emphasis. On macOS the sidebar's semantic tint is
/// installed as a native layer above Sidebar vibrancy; keeping this GPUI
/// surface clear avoids incorrectly accumulating the alpha of nested Metal
/// backgrounds. Selected, hovered, and pressed rows remain a neutral layer.
#[derive(Clone, Copy)]
pub struct Theme {
    pub is_dark: bool,
    pub canvas: Hsla,
    pub sidebar: Hsla,
    pub sidebar_drag_background: Hsla,
    pub sidebar_item_background: Hsla,
    pub surface: Hsla,
    pub raised: Hsla,
    pub composer: Hsla,
    pub inset: Hsla,
    /// Terminal screen surface: paper-white in light mode, near-black in dark.
    pub terminal: Hsla,
    pub overlay: Hsla,
    pub overlay_strong: Hsla,

    pub border: Hsla,
    pub border_strong: Hsla,
    pub sidebar_border: Hsla,

    pub text: Hsla,
    pub text_secondary: Hsla,
    pub text_tertiary: Hsla,
    pub text_ghost: Hsla,

    /// Brand coral. Logo, caret, live-activity pulses — nothing structural.
    pub accent: Hsla,
    pub resize_handle: Hsla,
    /// Meter fills in the usage panel. Quota-meter blue by convention;
    /// warning/danger take over as a lane fills.
    pub gauge: Hsla,

    /// Text-selection wash. Painted *under* the glyphs, so it stays
    /// translucent and deliberately reads as the familiar browser blue rather
    /// than as brand color.
    pub selection: Hsla,
    /// Inline `code` foreground and its rounded wash.
    pub code_text: Hsla,
    pub code_wash: Hsla,

    /// Light fill for primary buttons (send, allow), dark glyph on top.
    pub inverse: Hsla,
    pub on_inverse: Hsla,

    pub warning: Hsla,
    pub success: Hsla,
    pub favorite: Hsla,
    pub danger: Hsla,
    pub danger_soft: Hsla,
}

impl Theme {
    pub fn current(cx: &App) -> Self {
        if cx.has_global::<ActiveWakuTheme>() {
            cx.global::<ActiveWakuTheme>().0
        } else {
            Self::dark()
        }
    }

    pub fn dark() -> Self {
        Self {
            is_dark: true,
            canvas: rgb(0x1F1F1E).into(),
            sidebar: if cfg!(target_os = "macos") {
                transparent_black()
            } else {
                rgb(0x191918).into()
            },
            sidebar_drag_background: rgb(0x191918).into(),
            sidebar_item_background: hsla(0.0, 0.0, 1.0, 0.055),
            surface: rgb(0x1F1F1E).into(),
            raised: rgb(0x292928).into(),
            composer: rgb(0x272726).into(),
            inset: rgb(0x171716).into(),
            terminal: rgb(0x171716).into(),
            overlay: hsla(0.0, 0.0, 1.0, 0.055),
            overlay_strong: hsla(0.0, 0.0, 1.0, 0.10),

            border: hsla(0.0, 0.0, 1.0, 0.075),
            border_strong: hsla(0.0, 0.0, 1.0, 0.15),
            sidebar_border: hsla(0.0, 0.0, 1.0, 0.08),

            text: rgb(0xECECEA).into(),
            text_secondary: rgb(0xB5B5B0).into(),
            text_tertiary: rgb(0x898984).into(),
            text_ghost: rgb(0x62625E).into(),

            accent: rgb(0x10A37F).into(),
            resize_handle: rgb(0x10A37F).into(),
            gauge: rgb(0x4F8DF7).into(),

            selection: hsla(211.0 / 360.0, 1.0, 0.50, 0.55),
            code_text: rgb(0xD7D7D2).into(),
            code_wash: hsla(0.0, 0.0, 1.0, 0.075),

            inverse: rgb(0xECECEA).into(),
            on_inverse: rgb(0x1F1F1E).into(),

            warning: rgb(0xE0B36A).into(),
            success: rgb(0x62C987).into(),
            favorite: rgb(0xEAB308).into(),
            danger: rgb(0xE2726A).into(),
            danger_soft: hsla(4.0 / 360.0, 0.55, 0.63, 0.10),
        }
    }

    pub fn light() -> Self {
        Self {
            is_dark: false,
            canvas: rgb(0xFAFAF9).into(),
            sidebar: if cfg!(target_os = "macos") {
                transparent_black()
            } else {
                rgb(0xF4F4F2).into()
            },
            sidebar_drag_background: rgb(0xF4F4F2).into(),
            sidebar_item_background: hsla(0.0, 0.0, 0.0, 0.045),
            surface: rgb(0xFAFAF9).into(),
            raised: rgb(0xF1F1EF).into(),
            composer: rgb(0xFFFFFF).into(),
            inset: rgb(0xF3F3F1).into(),
            terminal: rgb(0xFFFFFF).into(),
            overlay: hsla(0.0, 0.0, 0.0, 0.045),
            overlay_strong: hsla(0.0, 0.0, 0.0, 0.085),

            border: hsla(0.0, 0.0, 0.0, 0.09),
            border_strong: hsla(0.0, 0.0, 0.0, 0.17),
            sidebar_border: hsla(0.0, 0.0, 0.0, 0.10),

            text: rgb(0x2D2D2A).into(),
            text_secondary: rgb(0x666662).into(),
            text_tertiary: rgb(0x858580).into(),
            text_ghost: rgb(0xA6A6A0).into(),

            accent: rgb(0x0F8A6B).into(),
            resize_handle: rgb(0x0F8A6B).into(),
            gauge: rgb(0x3978E8).into(),

            selection: hsla(211.0 / 360.0, 1.0, 0.50, 0.35),
            code_text: rgb(0x444440).into(),
            code_wash: hsla(0.0, 0.0, 0.0, 0.045),

            inverse: rgb(0x2D2D2A).into(),
            on_inverse: rgb(0xFAFAF9).into(),

            warning: rgb(0xA66B20).into(),
            success: rgb(0x2F8F52).into(),
            favorite: rgb(0xCA8A04).into(),
            danger: rgb(0xC64A42).into(),
            danger_soft: hsla(4.0 / 360.0, 0.55, 0.52, 0.10),
        }
    }
}

#[derive(Clone, Copy)]
struct ActiveWakuTheme(Theme);

impl Global for ActiveWakuTheme {}

/// Publish the resolved palette. [`Theme::current`] reads it back from the
/// global, which is how every view gets its colors.
fn set_active_theme(theme: Theme, cx: &mut App) {
    cx.set_global(ActiveWakuTheme(theme));
}

/// Resolve and publish the startup palette, before any window exists.
pub fn init(cx: &mut App) {
    let system_appearance = cx.window_appearance();
    let theme = if resolves_to_dark(ThemePreference::System, system_appearance) {
        Theme::dark()
    } else {
        Theme::light()
    };
    set_active_theme(theme, cx);
}

pub fn apply_theme_preference(preference: ThemePreference, window: &mut Window, cx: &mut App) {
    crate::platform::set_window_appearance(window, native_override(preference));
    let is_dark = resolves_to_dark(preference, cx.window_appearance());
    set_active_theme(
        if is_dark {
            Theme::dark()
        } else {
            Theme::light()
        },
        cx,
    );
    crate::platform::configure_sidebar_material(window, is_dark);
    window.refresh();
}
