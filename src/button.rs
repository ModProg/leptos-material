use derive_more::Display;
use leptos::*;

use crate::Children;

#[derive(Display, Default)]
pub enum ButtonKind {
    #[default]
    #[display(fmt = "elevated")]
    Elevated,
    #[display(fmt = "filled")]
    Filled,
    #[display(fmt = "tonal")]
    Tonal,
    #[display(fmt = "outlined")]
    Outlined,
    #[display(fmt = "text")]
    Text,
}

#[component]
pub fn Button(
    cx: Scope,
    kind: ButtonKind,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {cx,
        <button class=kind.to_string() disabled=disabled>
            { children.map(|c|c(cx)) }
        </button>
    }
}
