//! Modal Component

use leptos::*;

/// Modal dialog component
#[component]
pub fn Modal(
    #[prop(into)] open: Signal<bool>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] on_close: Option<Box<dyn Fn() + 'static>>,
    children: Children,
) -> impl IntoView {
    let close = move |_| {
        if let Some(ref handler) = on_close {
            handler();
        }
    };
    
    view! {
        <Show when=move || open.get()>
            <div class="modal-overlay" on:click=close>
                <div class="modal" on:click=|e| e.stop_propagation()>
                    <div class="modal-header">
                        {title.map(|t| view! { <h2 class="modal-title">{t}</h2> })}
                        <button class="modal-close" on:click=close>"✕"</button>
                    </div>
                    <div class="modal-body">
                        {children()}
                    </div>
                </div>
            </div>
        </Show>
    }
}

