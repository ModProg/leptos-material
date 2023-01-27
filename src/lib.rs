#![allow(unused_braces)]
use bytemuck::cast;
use leptos::*;
use material_color_utilities_rs::palettes::core::CorePalette;
use material_color_utilities_rs::palettes::tonal::TonalPalette;
use material_color_utilities_rs::scheme::Scheme;
use rgb::alt::ARGB8;
use rgb::RGB8;

#[macro_use]
mod macros;

mod card;
pub use card::*;

type Children = Box<dyn FnOnce(Scope) -> Fragment>;

#[component]
pub fn MaterialStyle(cx: Scope) -> impl IntoView {
    view! {
        cx,
        // <style>
        //     {include_scss!("src/style.scss")}
        // </style>
    }
}

macro_rules! extract_colors {
    ($scheme:expr => $($color:ident),+ $(,)?) => {
        [$((stringify!($color), $scheme.$color)),*]
    };
}

macro_rules! set_if_some {
    [$palette:ident.$($field:ident = $color:ident),+] => {
        $(if let Some($color) = $color {
            $palette.$field = TonalPalette::from_int(cast(ARGB8::from($color)));
        })*
    };
}

fn tint(a: [u8; 4], b: [u8; 4], opacity: f32) -> [u8; 3] {
    let opacity = opacity / 100.;
    [1, 2, 3].map(|i| (a[i] as f32 * (1. - opacity) + b[i] as f32 * opacity).round() as u8)
}

#[component]
pub fn MaterialColors(
    cx: Scope,
    #[prop(into)] primary: RGB8,
    #[prop(optional, into)] secondary: Option<RGB8>,
    #[prop(optional, into)] tertiary: Option<RGB8>,
    #[prop(optional, into)] error: Option<RGB8>,
    #[prop(optional, into)] neutral: Option<RGB8>,
    #[prop(optional, into)] neutral_variant: Option<RGB8>,
) -> impl IntoView {
    let mut core = CorePalette::new(cast(ARGB8::from(primary)), true);

    set_if_some![
        core.a2 = secondary,
        a3 = tertiary,
        error = error,
        n1 = neutral,
        n2 = neutral_variant
    ];

    let [light, dark] = [
        Scheme::light_from_core_palette(&mut core),
        Scheme::dark_from_core_palette(&mut core),
    ]
    .map(|scheme| {
        let mut colors: String = extract_colors![scheme =>
                        primary, on_primary,
                        primary_container, on_primary_container,
                        secondary, on_secondary,
                        secondary_container, on_secondary_container,
                        tertiary, on_tertiary,
                        tertiary_container, on_tertiary_container,
                        error, on_error,
                        error_container, on_error_container,
                        background, on_background,
                        surface, on_surface,
                        surface_variant, on_surface_variant,
                        outline, outline_variant,
                        shadow, scrim,
                        inverse_surface, inverse_on_surface, inverse_primary,
        ]
        .into_iter()
        .map(|(name, [_a, r, g, b])| format!("--md-{}: rgb({r} {g} {b});", name.replace('_', "-")))
        .collect();
        for opacity in [5., 8., 11., 12., 14.] {
            let [r, g, b] = tint(scheme.surface, scheme.primary, opacity);

            colors.push_str(&format!(
                "--md-overlay-surface-primary-{opacity}: rgb({r} {g} {b});"
            ))
        }
        let [_a, r, g, b] = scheme.shadow;
        colors.push_str(&format!("--md-shadow-25: rgb({r} {g} {b} / 0.25)"));
        colors
    });
    let style = format!(
        ":root {{ {light} }} @media (prefers-color-scheme: dark) {{
:root {{ {dark} }} }}"
    );
    view! {
        cx,
        <style>
            {style}
        </style>
    }
}
