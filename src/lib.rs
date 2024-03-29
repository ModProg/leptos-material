#![allow(unused_braces)]
//! # Static CSS for initial style
//! To have the correct background color before the JS/WASM is executed in a CSR
//! scenario, include the follwoing css in your [`<head>`]:
//! ```html
//! <style>
#![doc = grass::include!("src/style/static.scss")]
//! </style>
//! ```
//! You can also include the full generated css, though keep in mind that you will need to manually
//! update it when a new version of leptos-material changes it (currently very likely), but it
//! actually enables to use the css without leptos (see the [`github-pages-index`] example).
//! ```css
#![doc = grass::include!("src/style/mod.scss")]
//! ```
//! [`<head>`]: https://developer.mozilla.org/docs/Web/HTML/Element/head
//! [`github-pages-index`]: https://github.com/ModProg/leptos-material/tree/main/examples/github-pages-index
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
mod button;
pub use button::*;
mod text_field;
pub use text_field::*;
mod icon;
pub use icon::*;

// type Children = Box<dyn FnOnce(Scope) -> Fragment>;

fn optional(cx: Scope, children: Option<Children>) -> impl IntoView {
    children.map(|c| c(cx))
}

pub enum Font {
    Family(String),
    Path(String),
}

#[derive(Default)]
pub enum IconFonts {
    #[default]
    MaterialSymbols,
    Single(Font),
    Multiple {
        outlined: Font,
        rounded: Font,
        sharp: Font,
    },
}

/// Currently only the defaults i.e. Roboto and Material Icons are supported.
///
/// The path is currently expected to contain:
/// - MaterialSymbolsOutlined[FILL,GRAD,opsz,wght].woff2
/// - MaterialSymbolsRounded[FILL,GRAD,opsz,wght].woff2
/// - MaterialSymbolsSharp[FILL,GRAD,opsz,wght].woff2
/// - RobotoFlex-VariableFont_GRAD,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC,opsz,slnt,
///   wdth,wght.ttf
#[component]
pub fn MaterialFonts(cx: Scope, #[prop(into)] path: String) -> impl IntoView {
    view! {
                                                    cx,
                                                    <style>
                                                        {format!(r"@font-face {{
  font-family: 'Material Symbols Outlined';
  font-style: normal;
  src: url({path}/MaterialSymbolsOutlined[FILL\,GRAD\,opsz\,wght].woff2) format('woff');
}}

@font-face {{
  font-family: 'Material Symbols Rounded';
  font-style: normal;
  src: url({path}/MaterialSymbolsRounded[FILL\,GRAD\,opsz\,wght].woff2) format('woff');
}}

@font-face {{
  font-family: 'Material Symbols Sharp';
  font-style: normal;
  src: url({path}/MaterialSymbolsSharp[FILL\,GRAD\,opsz\,wght].woff2) format('woff');
}}

@font-face {{
  font-family: 'Roboto Flex';
  src: url({path}/RobotoFlex-VariableFont_GRAD\,XTRA\,YOPQ\,YTAS\,YTDE\,YTFI\,YTLC\,YTUC\,opsz\,slnt\,wdth\,wght.ttf);
}}
")}
                                                    </style>
                                                }
}

#[component]
pub fn MaterialStyle(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <style>
            {grass::include!("src/style/mod.scss")}
        </style>
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

#[component]
pub fn MaterialColors(
    cx: Scope,
    /// [`RGB8`]
    #[prop(into)]
    primary: RGB8,
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
        let colors: String = extract_colors![scheme =>
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
        .map(|(name, [_a, r, g, b])| format!("--md-theme-{}: {r} {g} {b};", name.replace('_', "-")))
        .collect();
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
