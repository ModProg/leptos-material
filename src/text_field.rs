use leptos::ev::Event;
use leptos::*;

#[component]
pub fn TextField(
    cx: Scope,
    #[prop(optional)] filled: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
) -> impl IntoView {
    let input = if let Some(value) = value {
        log!("hi");
        view! {cx,
            <input
                class:filled=filled
                disabled=disabled
                prop:value={value}
                on:input=move |ev: Event| {log!("hi"); value.set(event_target_value(&ev))}
            />
        }
    } else {
        view! {cx,
            <input class:filled=filled disabled=disabled/>
        }
    };
    if let Some(label) = label {
        view! {cx,
            <label>
                {input}
                <span>{label}</span>
            </label>
        }
        .into_any()
    } else {
        input.into_any()
    }
}
