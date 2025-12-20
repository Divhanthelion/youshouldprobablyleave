//! Main Application Component

use leptos::*;
use leptos_router::*;
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
                        <Routes>
                            <Route path="/" view=Dashboard/>
                            <Route path="/inventory" view=InventoryPage/>
                            <Route path="/inventory/:id" view=InventoryDetailPage/>
                            <Route path="/shipping" view=ShippingPage/>
                            <Route path="/shipping/new" view=NewShipmentPage/>
                            <Route path="/receiving" view=ReceivingPage/>
                            <Route path="/deliveries" view=DeliveriesPage/>
                            <Route path="/deliveries/:id" view=DeliveryDetailPage/>
                            <Route path="/customers" view=CustomersPage/>
                            <Route path="/customers/:id" view=CustomerDetailPage/>
                            <Route path="/timesheets" view=TimesheetsPage/>
                            <Route path="/settings" view=SettingsPage/>
                            <Route path="/*" view=NotFoundPage/>
                        </Routes>
                    </div>
                </main>
            </div>
        </Router>
    }
}

