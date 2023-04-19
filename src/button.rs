use derive_more::Display;
use leptos::*;

use crate::*;

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
    #[prop(optional, into, default=MaybeSignal::Static(false))] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {cx,
        <button class=kind.to_string() disabled=disabled>
            {icon.map(|i|view! {cx, <Icon icon=i/>})}
            { children.map(|c|c(cx)) }
        </button>
    }
}
