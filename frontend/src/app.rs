//! Main Application Component

use leptos::prelude::*;
use leptos::context::provide_context;
use leptos_router::components::{Router, Route, Routes};
use leptos_router::path;
use crate::components::*;
use crate::pages::*;
use crate::state::AppState;

/// Main application component with routing
#[component]
pub fn App() -> impl IntoView {
    // Provide global state
    provide_context(AppState::new());

    view! {
        <Router>
            <div class="app-container">
                <Sidebar/>
                <main class="main-content">
                    <Header/>
                    <div class="page-content">
                        <Routes fallback=|| NotFoundPage>
                            <Route path=path!("/") view=Dashboard/>
                            <Route path=path!("/inventory") view=InventoryPage/>
                            <Route path=path!("/inventory/:id") view=InventoryDetailPage/>
                            <Route path=path!("/shipping") view=ShippingPage/>
                            <Route path=path!("/shipping/new") view=NewShipmentPage/>
                            <Route path=path!("/receiving") view=ReceivingPage/>
                            <Route path=path!("/deliveries") view=DeliveriesPage/>
                            <Route path=path!("/deliveries/:id") view=DeliveryDetailPage/>
                            <Route path=path!("/customers") view=CustomersPage/>
                            <Route path=path!("/customers/:id") view=CustomerDetailPage/>
                            <Route path=path!("/timesheets") view=TimesheetsPage/>
                            <Route path=path!("/settings") view=SettingsPage/>
                        </Routes>
                    </div>
                </main>
            </div>
        </Router>
    }
}
