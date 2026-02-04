//! Input Components

use leptos::prelude::*;
use leptos::ev::Event;
use wasm_bindgen::JsCast;

/// Get target value from event
fn event_target_value(ev: &Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|t| t.value())
        .unwrap_or_default()
}

/// Text input component
#[component]
pub fn TextInput(
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] input_type: Option<&'static str>,
) -> impl IntoView {
    let input_value = value.unwrap_or_else(|| RwSignal::new(String::new()));
    let has_error = error.is_some();
    let error_msg = error;

    view! {
        <div class=format!("form-group {}", if has_error { "has-error" } else { "" })>
            {label.map(|l| view! { <label class="form-label">{l}</label> })}
            <input
                type=input_type.unwrap_or("text")
                class="form-input"
                placeholder=placeholder
                disabled=disabled
                prop:value=move || input_value.get()
                on:input=move |ev| {
                    input_value.set(event_target_value(&ev));
                }
            />
            {error_msg.map(|e| view! { <span class="form-error">{e}</span> })}
        </div>
    }
}

/// Select dropdown
#[component]
pub fn Select(
    #[prop(optional)] label: Option<&'static str>,
    options: Vec<(String, String)>, // (value, label)
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] placeholder: Option<&'static str>,
) -> impl IntoView {
    let selected = value.unwrap_or_else(|| RwSignal::new(String::new()));

    view! {
        <div class="form-group">
            {label.map(|l| view! { <label class="form-label">{l}</label> })}
            <select
                class="form-select"
                on:change=move |ev| {
                    selected.set(event_target_value(&ev));
                }
            >
                {placeholder.map(|p| view! {
                    <option value="" disabled selected>{p}</option>
                })}
                {options.into_iter().map(|(val, label)| {
                    view! {
                        <option value=val.clone()>{label}</option>
                    }
                }).collect::<Vec<_>>()}
            </select>
        </div>
    }
}

/// Search input with debounce
#[component]
pub fn SearchInput(
    #[prop(optional)] placeholder: Option<&'static str>,
) -> impl IntoView {
    let query = RwSignal::new(String::new());

    view! {
        <div class="search-input-wrapper">
            <span class="search-icon">"🔍"</span>
            <input
                type="search"
                class="search-input"
                placeholder=placeholder.unwrap_or("Search...")
                prop:value=move || query.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    query.set(value);
                }
            />
            <Show when=move || !query.get().is_empty()>
                <button
                    class="search-clear"
                    on:click=move |_| query.set(String::new())
                >
                    "✕"
                </button>
            </Show>
        </div>
    }
}
