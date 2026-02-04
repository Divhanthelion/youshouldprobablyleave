//! Badge Component

use leptos::prelude::*;

/// Badge variant
#[derive(Clone, Copy, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Danger,
    Info,
}

impl BadgeVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "badge-default",
            Self::Success => "badge-success",
            Self::Warning => "badge-warning",
            Self::Danger => "badge-danger",
            Self::Info => "badge-info",
        }
    }
}

/// Badge component for status indicators
#[component]
pub fn Badge(
    children: Children,
    #[prop(optional)] variant: BadgeVariant,
) -> impl IntoView {
    view! {
        <span class=format!("badge {}", variant.class())>
            {children()}
        </span>
    }
}

/// Status badge with predefined states
#[component]
pub fn StatusBadge(
    status: &'static str,
) -> impl IntoView {
    let (variant, label) = match status.to_lowercase().as_str() {
        "active" | "delivered" | "completed" | "approved" => (BadgeVariant::Success, status),
        "pending" | "draft" | "processing" => (BadgeVariant::Default, status),
        "warning" | "low_stock" | "partial" => (BadgeVariant::Warning, status),
        "error" | "failed" | "cancelled" | "rejected" => (BadgeVariant::Danger, status),
        _ => (BadgeVariant::Info, status),
    };
    
    view! {
        <Badge variant=variant>
            {label}
        </Badge>
    }
}

