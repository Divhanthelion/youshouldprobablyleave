//! Input Components

use leptos::*;

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
    let input_value = value.unwrap_or_else(|| create_rw_signal(String::new()));
    
    view! {
        <div class=format!("form-group {}", if error.is_some() { "has-error" } else { "" })>
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
            {error.map(|e| view! { <span class="form-error">{e}</span> })}
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
    let selected = value.unwrap_or_else(|| create_rw_signal(String::new()));
    
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
                }).collect_view()}
            </select>
        </div>
    }
}

/// Search input with debounce
#[component]
pub fn SearchInput(
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] on_search: Option<Box<dyn Fn(String) + 'static>>,
) -> impl IntoView {
    let query = create_rw_signal(String::new());
    
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
                    query.set(value.clone());
                    if let Some(ref handler) = on_search {
                        handler(value);
                    }
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

