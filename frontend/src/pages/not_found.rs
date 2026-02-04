//! 404 Not Found Page

use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="not-found-page">
            <div class="not-found-content">
                <span class="not-found-icon">"📦"</span>
                <h1>"404"</h1>
                <p>"Page not found"</p>
                <p class="not-found-message">
                    "The page you're looking for doesn't exist or has been moved."
                </p>
                <A href="/" attr:class="btn btn-primary">"Back to Dashboard"</A>
            </div>
        </div>
    }
}
