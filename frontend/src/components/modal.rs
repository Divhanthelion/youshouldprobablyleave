//! Modal Component

use leptos::prelude::*;

/// Modal dialog component
#[component]
pub fn Modal(
    #[prop(into)] open: Signal<bool>,
    #[prop(optional)] title: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let title_text = title.clone();

    view! {
        <Show when=move || open.get()>
            <div class="modal-overlay">
                <div class="modal" on:click=|e| e.stop_propagation()>
                    <div class="modal-header">
                        {title_text.clone().map(|t| view! { <h2 class="modal-title">{t}</h2> })}
                        <button class="modal-close">"✕"</button>
                    </div>
                    <div class="modal-body">
                        {children()}
                    </div>
                </div>
            </div>
        </Show>
    }
}
