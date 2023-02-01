use derive_more::Display;
use leptos::*;

use crate::Children;

#[component]
pub fn Textfield(
    cx: Scope,
    #[prop(optional)] filled: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let input = view! {cx,
        <input class=classes![filled?] disabled=disabled>
            { children.map(|c|c(cx)) }
        </input>
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
        view! {cx,
            <div class="pseudo-label">
                {input}
            </div>
        }
        .into_any()
    }
}
