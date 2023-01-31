use leptos::*;

use crate::{optional, Children};

#[component]
pub fn Card(
    cx: Scope,
    #[prop(optional)] elevated: bool,
    #[prop(optional)] filled: bool,
    #[prop(optional)] outlined: bool,
    #[prop(optional, into)] max_width: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // debug_assert!((0..=5).contains(&elevation), "elevation should be in 0..=5");
    view! {cx,
        <div class=classes!["card", elevated, filled, outlined] style=styles![(max_width?)]>
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
