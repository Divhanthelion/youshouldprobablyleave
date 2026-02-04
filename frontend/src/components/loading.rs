//! Loading Component

use leptos::prelude::*;

/// Loading spinner component
#[component]
pub fn Loading(
    #[prop(optional)] message: Option<&'static str>,
    #[prop(optional)] full_page: bool,
) -> impl IntoView {
    view! {
        <div class=if full_page { "loading-fullpage" } else { "loading" }>
            <div class="loading-spinner"></div>
            {message.map(|m| view! { <p class="loading-message">{m}</p> })}
        </div>
    }
}

/// Skeleton loader for content placeholders
#[component]
pub fn Skeleton(
    #[prop(optional)] width: Option<&'static str>,
    #[prop(optional)] height: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div 
            class="skeleton"
            style=format!(
                "width: {}; height: {};",
                width.unwrap_or("100%"),
                height.unwrap_or("1rem")
            )
        ></div>
    }
}

