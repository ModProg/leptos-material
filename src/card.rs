use derive_more::Display;
use leptos::*;

use crate::{optional, Children};

#[derive(Display, Default)]
pub enum CardKind {
    #[default]
    #[display(fmt = "elevated")]
    Elevated,
    #[display(fmt = "filled")]
    Filled,
    #[display(fmt = "outlined")]
    Outlined,
}

#[component]
pub fn Card(
    cx: Scope,
    #[prop(optional)] kind: CardKind,
    #[prop(optional, into)] max_width: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // debug_assert!((0..=5).contains(&elevation), "elevation should be in 0..=5");
    view! {cx,
        <div class=classes!["card", kind] style=styles![(max_width?)]>
            {optional(cx, children)}
        </div>
    }
}

#[component]
pub fn Actions(cx: Scope, #[prop(optional)] children: Option<Children>) -> impl IntoView {
    view! {cx,
        <div class="actions">
            {optional(cx, children)}
        </div>
    }
}
