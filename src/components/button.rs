//! Button Components

use leptos::*;

/// Button variant
#[derive(Clone, Copy, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
}

impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            Self::Primary => "btn-primary",
            Self::Secondary => "btn-secondary",
            Self::Danger => "btn-danger",
            Self::Ghost => "btn-ghost",
        }
    }
}

/// Button size
#[derive(Clone, Copy, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    fn class(&self) -> &'static str {
        match self {
            Self::Small => "btn-sm",
            Self::Medium => "btn-md",
            Self::Large => "btn-lg",
        }
    }
}

/// Button component
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,
    #[prop(optional)] icon: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=format!("btn {} {}", variant.class(), size.class())
            disabled=disabled || loading
        >
            {if loading {
                Some(view! { <span class="btn-spinner"></span> })
            } else {
                icon.map(|i| view! { <span class="btn-icon">{i}</span> })
            }}
            <span class="btn-text">{children()}</span>
        </button>
    }
}

/// Icon button
#[component]
pub fn IconButton(
    icon: &'static str,
    #[prop(optional)] title: Option<&'static str>,
    #[prop(optional)] variant: ButtonVariant,
) -> impl IntoView {
    view! {
        <button 
            class=format!("icon-btn {}", variant.class())
            title=title
        >
            {icon}
        </button>
    }
}

