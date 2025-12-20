//! Toast Notification Component

use leptos::*;
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
                    let toast_id = toast.id.clone();
                    
                    view! {
                        <div class=format!("toast {}", toast_class)>
                            <span class="toast-message">{toast.message.clone()}</span>
                            <button 
                                class="toast-close"
                                on:click=move |_| state.dismiss_toast(&toast_id)
                            >"✕"</button>
                        </div>
                    }
                }
            />
        </div>
    }
}

