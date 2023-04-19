use derive_more::Display;
use leptos::*;

#[derive(Display)]
pub enum IconKind {
    #[display(fmt = "outlined")]
    Outlined,
    #[display(fmt = "rounded")]
    Rounded,
    #[display(fmt = "sharp")]
    Sharp,
}

#[component]
pub fn Icon(
    cx: Scope,
    #[prop(optional)] filled: bool,
    #[prop(optional)] kind: Option<IconKind>,
    #[prop(into)] icon: String,
) -> impl IntoView {
    view! {cx,
        <span class:filled=filled class=classes!["icon", kind?]>{icon}</span>
    }
}
