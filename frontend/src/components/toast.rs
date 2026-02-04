//! Toast Notification Component

use leptos::prelude::*;
use leptos::context::use_context;
use crate::state::{AppState, ToastType};

/// Toast container that shows notifications
#[component]
pub fn ToastContainer() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");

    view! {
        <div class="toast-container">
            <For
                each=move || state.toasts.get()
                key=|toast| toast.id.clone()
                children=move |toast| {
                    let toast_class = match toast.toast_type {
                        ToastType::Success => "toast-success",
                        ToastType::Error => "toast-error",
                        ToastType::Warning => "toast-warning",
                        ToastType::Info => "toast-info",
                    };

                    view! {
                        <div class=format!("toast {}", toast_class)>
                            <span class="toast-message">{toast.message.clone()}</span>
                            <button class="toast-close">"✕"</button>
                        </div>
                    }
                }
            />
        </div>
    }
}
