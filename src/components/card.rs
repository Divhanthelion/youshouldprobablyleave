//! Card Component

use leptos::*;

/// Card container component
#[component]
pub fn Card(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] class: Option<String>,
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
    value: Signal<String>,
    #[prop(optional)] trend: Option<f64>,
    #[prop(optional)] icon: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="stat-card">
            {icon.map(|i| view! { <span class="stat-icon">{i}</span> })}
            <div class="stat-content">
                <span class="stat-title">{title}</span>
                <span class="stat-value">{move || value.get()}</span>
                {trend.map(|t| {
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

