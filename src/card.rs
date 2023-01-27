use leptos::*;

use crate::Children;

#[component]
pub fn Card(
    cx: Scope,
    #[prop(optional)] elevated: bool,
    #[prop(optional)] filled: bool,
    #[prop(optional)] outlined: bool,
    children: Children,
) -> impl IntoView {
    // debug_assert!((0..=5).contains(&elevation), "elevation should be in 0..=5");
    view! {
        cx,
        <div class=classes!["card", elevated, filled, outlined]>
            {children(cx)}
        </div>
    }
}
