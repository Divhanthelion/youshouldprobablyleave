//! Card Component

use leptos::prelude::*;

/// Card container component
#[component]
pub fn Card(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("card {}", class.unwrap_or_default())>
            {title.map(|t| view! {
                <div class="card-header">
                    <h3 class="card-title">{t}</h3>
                </div>
            })}
            <div class="card-body">
                {children()}
            </div>
        </div>
    }
}

/// Stat card for dashboard metrics
#[component]
pub fn StatCard(
    title: &'static str,
    #[prop(into)] value: Signal<String>,
    #[prop(optional, into)] trend: Option<f64>,
    #[prop(optional)] icon: Option<&'static str>,
) -> impl IntoView {
    let icon_str = icon;
    let trend_val = trend;

    view! {
        <div class="stat-card">
            {icon_str.map(|i| view! { <span class="stat-icon">{i}</span> })}
            <div class="stat-content">
                <span class="stat-title">{title}</span>
                <span class="stat-value">{move || value.get()}</span>
                {trend_val.map(|t| {
                    let trend_class = if t >= 0.0 { "positive" } else { "negative" };
                    let trend_icon = if t >= 0.0 { "↑" } else { "↓" };
                    view! {
                        <span class=format!("stat-trend {}", trend_class)>
                            {trend_icon} {format!("{:.1}%", t.abs())}
                        </span>
                    }
                })}
            </div>
        </div>
    }
}
