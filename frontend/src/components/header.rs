//! Header Component

use leptos::prelude::*;
use leptos::context::use_context;
use crate::state::AppState;

/// Application header
#[component]
pub fn Header() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    
    let toggle_theme = move |_| {
        state.theme.update(|t| {
            *t = match t {
                crate::state::Theme::Light => crate::state::Theme::Dark,
                crate::state::Theme::Dark => crate::state::Theme::Light,
            }
        });
    };
    
    view! {
        <header class="app-header">
            <div class="header-left">
                <div class="search-box">
                    <span class="search-icon">"🔍"</span>
                    <input 
                        type="text" 
                        placeholder="Search inventory, shipments, customers..."
                        class="search-input"
                    />
                    <kbd class="search-shortcut">"⌘K"</kbd>
                </div>
            </div>
            
            <div class="header-right">
                <button class="header-btn" title="Scan barcode">
                    <span>"📷"</span>
                </button>
                
                <button class="header-btn" title="Notifications">
                    <span>"🔔"</span>
                    <span class="notification-badge">"3"</span>
                </button>
                
                <button class="header-btn" on:click=toggle_theme title="Toggle theme">
                    {move || if matches!(state.theme.get(), crate::state::Theme::Dark) { "☀️" } else { "🌙" }}
                </button>
                
                <div class="user-menu">
                    <button class="user-btn">
                        <span class="user-avatar">"👤"</span>
                        <span class="user-name">
                            {move || state.user.get().map(|u| u.full_name).unwrap_or_else(|| "Guest".to_string())}
                        </span>
                    </button>
                </div>
            </div>
        </header>
    }
}

