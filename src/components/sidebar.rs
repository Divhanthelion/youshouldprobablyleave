//! Sidebar Navigation Component

use leptos::*;
use leptos_router::*;
use crate::state::{AppState, Module};

/// Sidebar navigation component
#[component]
pub fn Sidebar() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not found");
    let collapsed = create_rw_signal(false);
    
    let modules = vec![
        Module::Dashboard,
        Module::Inventory,
        Module::Shipping,
        Module::Receiving,
        Module::Deliveries,
        Module::Customers,
        Module::Timesheets,
        Module::Settings,
    ];
    
    view! {
        <aside class=move || format!("sidebar {}", if collapsed.get() { "collapsed" } else { "" })>
            <div class="sidebar-header">
                <div class="logo">
                    <span class="logo-icon">"📦"</span>
                    <Show when=move || !collapsed.get()>
                        <span class="logo-text">"WMS"</span>
                    </Show>
                </div>
                <button 
                    class="collapse-btn"
                    on:click=move |_| collapsed.update(|c| *c = !*c)
                >
                    {move || if collapsed.get() { "→" } else { "←" }}
                </button>
            </div>
            
            <nav class="sidebar-nav">
                <ul>
                    {modules.into_iter().map(|module| {
                        view! {
                            <li>
                                <A 
                                    href=module.path()
                                    class="nav-link"
                                    active_class="active"
                                >
                                    <span class="nav-icon">{module.icon()}</span>
                                    <Show when=move || !collapsed.get()>
                                        <span class="nav-text">{module.title()}</span>
                                    </Show>
                                </A>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </nav>
            
            <div class="sidebar-footer">
                <div class="sync-status">
                    <span class=move || {
                        let status = state.sync_status.get();
                        format!("status-dot {}", if status.is_online { "online" } else { "offline" })
                    }></span>
                    <Show when=move || !collapsed.get()>
                        <span class="status-text">
                            {move || {
                                let status = state.sync_status.get();
                                if status.is_syncing {
                                    "Syncing...".to_string()
                                } else if status.is_online {
                                    format!("{} pending", status.pending_changes)
                                } else {
                                    "Offline".to_string()
                                }
                            }}
                        </span>
                    </Show>
                </div>
            </div>
        </aside>
    }
}

