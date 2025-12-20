//! Settings Page

use leptos::*;
use crate::components::Card;
use crate::state::{AppState, Theme};

#[component]
pub fn SettingsPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    
    view! {
        <div class="page settings-page">
            <div class="page-header">
                <h1>"Settings"</h1>
                <p class="subtitle">"Configure application preferences"</p>
            </div>
            
            <div class="settings-grid">
                <Card title=Some("Appearance".to_string())>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Theme"</span>
                            <span class="setting-description">"Choose your preferred color scheme"</span>
                        </div>
                        <select 
                            class="form-select"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                state.theme.set(if value == "dark" { Theme::Dark } else { Theme::Light });
                            }
                        >
                            <option value="dark" selected=move || matches!(state.theme.get(), Theme::Dark)>"Dark"</option>
                            <option value="light" selected=move || matches!(state.theme.get(), Theme::Light)>"Light"</option>
                        </select>
                    </div>
                </Card>
                
                <Card title=Some("Synchronization".to_string())>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Auto Sync"</span>
                            <span class="setting-description">"Automatically sync when online"</span>
                        </div>
                        <label class="toggle">
                            <input type="checkbox" checked />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Sync Interval"</span>
                            <span class="setting-description">"How often to sync in background"</span>
                        </div>
                        <select class="form-select">
                            <option>"Every 5 minutes"</option>
                            <option>"Every 15 minutes"</option>
                            <option>"Every 30 minutes"</option>
                            <option>"Manual only"</option>
                        </select>
                    </div>
                </Card>
                
                <Card title=Some("Notifications".to_string())>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Low Stock Alerts"</span>
                            <span class="setting-description">"Get notified when items are low"</span>
                        </div>
                        <label class="toggle">
                            <input type="checkbox" checked />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                    <div class="setting-item">
                        <div class="setting-info">
                            <span class="setting-label">"Delivery Updates"</span>
                            <span class="setting-description">"Receive delivery status notifications"</span>
                        </div>
                        <label class="toggle">
                            <input type="checkbox" checked />
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                </Card>
                
                <Card title=Some("About".to_string())>
                    <dl class="about-list">
                        <dt>"Version"</dt><dd>"0.1.0"</dd>
                        <dt>"Build"</dt><dd>"2024.01.15"</dd>
                        <dt>"Runtime"</dt><dd>"Tauri v2 + Leptos"</dd>
                        <dt>"Database"</dt><dd>"SQLite + SQLCipher"</dd>
                    </dl>
                </Card>
            </div>
        </div>
    }
}

